//! Row — `audit_logs`. One row per CRUD action on a
//! plan (who / what / when + a snapshot).

// The row's field-level shape is documented by the
// migration and the `audit_logs` table, not by per-field rustdoc.
#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub entity_pid: Uuid,
    pub action: String,
    pub actor: Option<String>,
    pub snapshot: Option<Json>,
    /// SHA-256 (FIPS 180-4) over this audit row's pre-image.
    ///
    /// Unkeyed, so anyone holding the database can recompute it — what it
    /// catches is careless or unaware modification. Written
    /// unconditionally, unlike the MAC, which needs a key: with no key
    /// configured these two digests are the row's only integrity.
    pub hash: Option<String>,
    /// SHA3-256 (FIPS 202) over the same pre-image. A sponge, unrelated
    /// to SHA-256's Merkle-Damgard chaining, so a cryptanalytic advance
    /// against one design family does not transfer.
    pub hash_sha3: Option<String>,
    /// HMAC-SHA256 over this audit row's pre-image, as
    /// `"<scheme>.<key id>:<hex>"`.
    ///
    /// Detects an audit row whose *content* was edited. It does not
    /// detect a row deleted wholesale — that needs the hash chain this
    /// service does not yet have (see `crate::compliance`).
    pub mac: Option<String>,
}
