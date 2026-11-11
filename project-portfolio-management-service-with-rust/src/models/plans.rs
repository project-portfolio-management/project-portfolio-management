//! `plans` model — CRUD over the stored `project_portfolio_management_matcher::Plan`
//! payload. All plans live in one recursive collection (one table); the
//! `kind` column is an optional descriptive label, and `parent_pid` is
//! the containment link (any plan may contain any other).

use crate::prelude::*;
use project_portfolio_management_matcher::Plan as MatchPlan;
use uuid::Uuid;

pub use super::_entities::plans::{self, Model};

/// Parse a `parent_ref` payload string into the denormalized
/// `parent_pid` column value: `Some(uuid)` when it is a valid UUID,
/// `None` otherwise (or when absent — e.g. a root plan).
fn parent_pid_of(pl: &MatchPlan) -> Option<Uuid> {
    pl.parent_ref
        .as_deref()
        .and_then(|s| Uuid::parse_str(s.trim()).ok())
}

/// The optional `kind` column value: the serde name of the payload's
/// (optional) `kind` label, or `None`.
fn kind_of(pl: &MatchPlan) -> Option<String> {
    pl.kind.map(|k| {
        // `PlanKind` serializes as its variant name (e.g. "Project").
        serde_json::to_value(k)
            .ok()
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_default()
    })
}

impl Model {
    /// Deserialize the stored payload into a matcher `Plan`.
    ///
    /// # Errors
    ///
    /// When the stored JSON cannot be parsed.
    pub fn to_plan(&self) -> ModelResult<MatchPlan> {
        serde_json::from_value(self.data.clone()).map_err(|e| ModelError::Any(e.into()))
    }

    /// Replace the payload of an existing plan (and the denormalized `name`, `kind` and `parent_pid`), in a
    /// transaction or on the pool ([`crate::db::Session`]), so the update can share a transaction with the outbox
    /// insert (see [`Model::create`]). The update names only the columns it changes; one that finds no row is an
    /// error.
    ///
    /// # Errors
    ///
    /// When serialization or the update fails.
    pub async fn update_data_on(
        &self,
        db: &mut impl crate::db::Session,
        pl: &MatchPlan,
    ) -> Result<Model> {
        let data = serde_json::to_value(pl).map_err(|e| Error::Model(ModelError::Any(e.into())))?;
        let kind = kind_of(pl);
        let parent_pid = parent_pid_of(pl);
        // Re-digest over the *new* content: leaving the old digests would report every legitimate edit as tampering.
        let d = crate::compliance::record_integrity::digests(
            &crate::compliance::record_integrity::RecordInput {
                pid: self.pid,
                name: &pl.name,
                kind: kind.as_deref(),
                parent_pid,
                stage: self.stage.as_deref(),
                data: &data,
                active: self.active,
                deleted_at_micros: self
                    .deleted_at
                    .as_ref()
                    .map(chrono::DateTime::timestamp_micros),
            },
        );
        Ok(sqlx::query_as(
            "UPDATE plans
             SET name = $1, kind = $2, data = $3, parent_pid = $4,
                 content_hash = $5, content_hash_sha3 = $6, content_mac = $7
             WHERE id = $8
             RETURNING *",
        )
        .bind(&pl.name)
        .bind(kind)
        .bind(data)
        .bind(parent_pid)
        .bind(Some(d.sha256))
        .bind(Some(d.sha3))
        .bind(d.mac)
        .bind(self.id)
        .fetch_one(db.ex())
        .await?)
    }

    /// Soft-delete: mark inactive, stamp `deleted_at` and re-digest, in a transaction or on the pool. `active` and
    /// `deleted_at` are both in the pre-image, so a soft delete changes the digest; not re-digesting would make every
    /// soft-deleted row look tampered with.
    ///
    /// # Errors
    ///
    /// When the update fails, or finds no row.
    pub async fn soft_delete_on(&self, db: &mut impl crate::db::Session) -> Result<Model> {
        let stamp: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
        let d = crate::compliance::record_integrity::digests(
            &crate::compliance::record_integrity::RecordInput {
                pid: self.pid,
                name: &self.name,
                kind: self.kind.as_deref(),
                parent_pid: self.parent_pid,
                stage: self.stage.as_deref(),
                data: &self.data,
                active: false,
                deleted_at_micros: Some(stamp.timestamp_micros()),
            },
        );
        Ok(sqlx::query_as(
            "UPDATE plans
             SET active = $1, deleted_at = $2, content_hash = $3, content_hash_sha3 = $4, content_mac = $5
             WHERE id = $6
             RETURNING *",
        )
        .bind(false)
        .bind(Some(stamp))
        .bind(Some(d.sha256))
        .bind(Some(d.sha3))
        .bind(d.mac)
        .bind(self.id)
        .fetch_one(db.ex())
        .await?)
    }

    /// Insert a new plan, returning the created row, in a transaction or on the pool ([`crate::db::Session`]): the
    /// outbox transport passes its own transaction, so the row and its `axa_outbox` row share one commit.
    ///
    /// # Errors
    ///
    /// When serialization or the insert fails.
    pub async fn create(db: &mut impl crate::db::Session, pl: &MatchPlan) -> Result<Self> {
        let data = serde_json::to_value(pl).map_err(|e| Error::Model(ModelError::Any(e.into())))?;
        // Mint the pid before the insert so it is known to the digests.
        let pid = Uuid::new_v4();
        let kind = kind_of(pl);
        let parent_pid = parent_pid_of(pl);
        // All three digests from one call, computed inline rather than
        // stamped afterwards: one statement writes the row and its
        // integrity values, so no row ever exists unhashed.
        let digests = crate::compliance::record_integrity::digests(
            &crate::compliance::record_integrity::RecordInput {
                pid,
                name: &pl.name,
                kind: kind.as_deref(),
                parent_pid,
                stage: None,
                data: &data,
                active: true,
                deleted_at_micros: None,
            },
        );
        Ok(sqlx::query_as(
            "INSERT INTO plans
                 (pid, kind, name, content_hash, content_hash_sha3, content_mac, data, parent_pid, active, deleted_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NULL)
             RETURNING *",
        )
        .bind(pid)
        .bind(kind)
        .bind(&pl.name)
        .bind(Some(digests.sha256))
        .bind(Some(digests.sha3))
        .bind(digests.mac)
        .bind(data)
        .bind(parent_pid)
        .bind(true)
        .fetch_one(db.ex())
        .await?)
    }

    /// Find an active plan by its public id.
    ///
    /// # Errors
    ///
    /// [`Error::NotFound`] when there is none (an id that is not a UUID among them), or when the query fails.
    pub async fn find_by_pid(ctx: &AppContext, pid: &str) -> Result<Self> {
        // An id that is not a UUID names nothing: 404, not a 500.
        let uuid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
        let row: Option<Self> =
            sqlx::query_as("SELECT * FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
                .bind(uuid)
                .fetch_optional(crate::db::pool(ctx))
                .await?;
        row.ok_or(Error::NotFound)
    }

    /// Fetch the active rows for a list of public ids, **preserving the
    /// order of `pids`**.
    ///
    /// The order matters: the caller is a search or blocking query whose
    /// hits are already ranked by relevance, and re-sorting them by row
    /// id would throw that ranking away. Ids that do not resolve (unknown
    /// or soft-deleted — a stale index entry) are simply absent, which is
    /// what keeps a drifted index from resurrecting a deleted record.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn find_by_pids(ctx: &AppContext, pids: &[Uuid]) -> Result<Vec<Self>> {
        if pids.is_empty() {
            return Ok(Vec::new());
        }
        let rows: Vec<Self> =
            sqlx::query_as("SELECT * FROM plans WHERE pid = ANY($1) AND deleted_at IS NULL")
                .bind(pids)
                .fetch_all(crate::db::pool(ctx))
                .await?;
        // Re-order to match `pids` (the SQL `ANY` result order is
        // unspecified). Linear in `pids` × `rows`, both bounded by the
        // caller's result limit.
        let mut ordered = Vec::with_capacity(rows.len());
        for pid in pids {
            if let Some(row) = rows.iter().find(|r| r.pid == *pid) {
                ordered.push(row.clone());
            }
        }
        Ok(ordered)
    }

    /// List active plans (most-recent first), capped at `limit`.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn list(ctx: &AppContext, limit: u64) -> Result<Vec<Self>> {
        Self::list_paged(ctx, None, limit, 0).await
    }

    /// One page of active plans (most-recent first), optionally scoped to
    /// a parent's direct children.
    ///
    /// # Errors
    ///
    /// When `parent` is not a UUID, or the query fails.
    pub async fn list_paged(
        ctx: &AppContext,
        parent: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Self>> {
        let parent = parent
            .map(|p| Uuid::parse_str(p).map_err(|e| Error::Model(ModelError::Any(e.into()))))
            .transpose()?;
        let mut q = sqlx::QueryBuilder::new("SELECT * FROM plans WHERE deleted_at IS NULL");
        if let Some(parent) = parent {
            q.push(" AND parent_pid = ").push_bind(parent);
        }
        q.push(" ORDER BY id DESC LIMIT ")
            .push_bind(i64::try_from(limit).unwrap_or(i64::MAX))
            .push(" OFFSET ")
            .push_bind(i64::try_from(offset).unwrap_or(i64::MAX));
        Ok(q.build_query_as().fetch_all(crate::db::pool(ctx)).await?)
    }

    /// List active child plans whose denormalized `parent_pid` equals
    /// `parent` — the containment roll-up.
    ///
    /// # Errors
    ///
    /// When `parent` is not a UUID, or the query fails.
    pub async fn list_children(ctx: &AppContext, parent: &str, limit: u64) -> Result<Vec<Self>> {
        let parent_uuid =
            Uuid::parse_str(parent).map_err(|e| Error::Model(ModelError::Any(e.into())))?;
        Ok(sqlx::query_as(
            "SELECT * FROM plans WHERE parent_pid = $1 AND deleted_at IS NULL ORDER BY id DESC LIMIT $2",
        )
        .bind(parent_uuid)
        .bind(i64::try_from(limit).unwrap_or(i64::MAX))
        .fetch_all(crate::db::pool(ctx))
        .await?)
    }

    /// Walk `parent_pid` from `start` toward the root, returning the chain
    /// of ancestor pids (nearest first). Bounded by `MAX_DEPTH` so a
    /// pre-existing cycle in the data can never loop forever.
    ///
    /// # Errors
    ///
    /// When a query fails.
    pub async fn ancestor_pids(ctx: &AppContext, start: Uuid) -> Result<Vec<Uuid>> {
        /// Hard cap on how far the containment chain is walked.
        const MAX_DEPTH: usize = 64;
        let mut chain = Vec::new();
        let mut cursor = Some(start);
        while let Some(pid) = cursor {
            if chain.len() >= MAX_DEPTH {
                break;
            }
            // `None` when there is no live row; `Some(None)` when it is a root.
            let row: Option<Option<Uuid>> = sqlx::query_scalar(
                "SELECT parent_pid FROM plans WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
            )
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?;
            match row.flatten() {
                Some(parent) => {
                    chain.push(parent);
                    cursor = Some(parent);
                }
                None => cursor = None,
            }
        }
        Ok(chain)
    }

    /// Case-insensitive substring search on the denormalized `name`, over
    /// active rows (Postgres `ILIKE '%q%'`, wildcards escaped). Full-text
    /// / fuzzy search over the JSONB payload via Tantivy is deferred
    /// (spec §13).
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn search(ctx: &AppContext, q: &str, limit: u64) -> Result<Vec<Self>> {
        Self::search_paged(ctx, q, limit, 0).await
    }

    /// One page of name-search hits.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn search_paged(
        ctx: &AppContext,
        q: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Self>> {
        let pattern = format!("%{}%", escape_like(q));
        Ok(sqlx::query_as(
            "SELECT * FROM plans WHERE deleted_at IS NULL AND name ILIKE $1
             ORDER BY id DESC LIMIT $2 OFFSET $3",
        )
        .bind(pattern)
        .bind(i64::try_from(limit).unwrap_or(i64::MAX))
        .bind(i64::try_from(offset).unwrap_or(i64::MAX))
        .fetch_all(crate::db::pool(ctx))
        .await?)
    }

    /// How many active plans match `q`, ignoring any window.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn search_count(ctx: &AppContext, q: &str) -> Result<u64> {
        let pattern = format!("%{}%", escape_like(q));
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM plans WHERE deleted_at IS NULL AND name ILIKE $1",
        )
        .bind(pattern)
        .fetch_one(crate::db::pool(ctx))
        .await?;
        Ok(u64::try_from(total).unwrap_or(0))
    }
}

/// Escape `LIKE`/`ILIKE` wildcards so a user query matches literally.
fn escape_like(q: &str) -> String {
    q.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::{escape_like, kind_of, parent_pid_of};
    use project_portfolio_management_matcher::{Plan, PlanKind};

    /// Pins the `LIKE` escaping: literal text is unchanged, `%` and `_`
    /// are escaped, and the backslash is escaped first.
    #[test]
    fn escape_like_neutralizes_wildcards() {
        assert_eq!(escape_like("apollo"), "apollo");
        assert_eq!(escape_like("100%"), "100\\%");
        assert_eq!(escape_like("a_b"), "a\\_b");
        assert_eq!(escape_like("a\\%"), "a\\\\\\%");
    }

    /// Pins `parent_pid_of`: a valid UUID `parent_ref` projects to
    /// `Some`, a non-UUID or absent ref to `None`.
    #[test]
    fn parent_pid_projection() {
        let mut pl = Plan::new("X");
        assert_eq!(parent_pid_of(&pl), None);
        pl.parent_ref = Some("not-a-uuid".into());
        assert_eq!(parent_pid_of(&pl), None);
        let u = uuid::Uuid::new_v4();
        pl.parent_ref = Some(u.to_string());
        assert_eq!(parent_pid_of(&pl), Some(u));
    }

    /// Pins `kind_of`: `None` projects to `None`, a set kind to its serde
    /// variant name.
    #[test]
    fn kind_projection() {
        let mut pl = Plan::new("X");
        assert_eq!(kind_of(&pl), None);
        pl.kind = Some(PlanKind::Project);
        assert_eq!(kind_of(&pl), Some("Project".to_string()));
    }
}
