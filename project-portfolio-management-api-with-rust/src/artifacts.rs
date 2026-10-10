//! The artifact store (LT-5, and the base T-8 builds on).
//!
//! Files live on local disk under `PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR`
//! (default: a `ppm-artifacts` directory under the system temp dir), at a
//! sharded path derived from a random id — never from anything the uploader
//! chose. The database row ([`crate::models::_entities::artifacts`]) carries
//! the name, declared content type, size and **SHA-256 of the bytes**.
//!
//! Everything here that can be decided without I/O is pure and tested: the
//! content-type allow-list, the size cap, filename sanitizing, and the
//! RFC 6266 / 5987 `Content-Disposition` that keeps UTF-8 names intact.
//!
//! A single node is assumed: the directory must be shared storage (or the
//! store replaced by an object store) before running several replicas.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Environment variable naming the storage directory.
pub const DIR_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR";
/// Environment variable overriding the per-file size cap.
pub const MAX_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES";
/// Default per-file cap: 10 MiB.
pub const DEFAULT_MAX_BYTES: usize = 10 * 1024 * 1024;

/// Content types an upload may declare. **A short allow-list on purpose**:
/// nothing that a browser would execute or render as markup (`text/html`,
/// `image/svg+xml`, scripts) and no archive or executable.
pub const ALLOWED_TYPES: &[&str] = &[
    "application/pdf",
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
    "text/plain",
    "text/csv",
    "text/markdown",
    "application/json",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
];

/// The per-file cap currently in force.
#[must_use]
pub fn max_bytes() -> usize {
    std::env::var(MAX_ENV)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(DEFAULT_MAX_BYTES)
}

/// Normalize a declared `Content-Type` header to its bare media type
/// (lowercase, parameters dropped).
#[must_use]
pub fn media_type(declared: &str) -> String {
    declared
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
}

/// Every problem with an upload, decided **before** anything is stored.
#[must_use]
pub fn upload_problems(declared_type: &str, size: usize, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    if size == 0 {
        out.push("the file is empty".to_string());
    }
    if size > max {
        out.push(format!("the file is {size} bytes; the limit is {max}"));
    }
    let t = media_type(declared_type);
    if !ALLOWED_TYPES.contains(&t.as_str()) {
        out.push(format!(
            "content type {t:?} is not allowed (allowed: {ALLOWED_TYPES:?})"
        ));
    }
    out
}

/// A safe display filename: no path components, no control characters, at
/// most 200 characters, never empty. **UTF-8 is preserved** — non-Latin names
/// are legitimate.
#[must_use]
pub fn sanitize_filename(raw: &str) -> String {
    let base = raw.rsplit(['/', '\\']).next().unwrap_or_default();
    let cleaned: String = base
        .chars()
        .filter(|c| !c.is_control())
        .take(200)
        .collect::<String>()
        .trim()
        .trim_matches('.')
        .trim()
        .to_string();
    if cleaned.is_empty() {
        "file".to_string()
    } else {
        cleaned
    }
}

/// An RFC 6266 `Content-Disposition: attachment` value carrying both an ASCII
/// fallback and the RFC 5987 `filename*` UTF-8 form, so a non-Latin name
/// downloads under its real name (and never as a double extension).
#[must_use]
pub fn content_disposition(filename: &str) -> String {
    use std::fmt::Write;
    let ascii: String = filename
        .chars()
        .map(|c| {
            if c.is_ascii_graphic() && !matches!(c, '"' | '\\' | ';') || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let mut encoded = String::new();
    for b in filename.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(b));
        } else {
            let _ = write!(encoded, "%{b:02X}");
        }
    }
    format!("attachment; filename=\"{ascii}\"; filename*=UTF-8''{encoded}")
}

/// Lowercase hex SHA-256 of `bytes`.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    Sha256::digest(bytes)
        .iter()
        .fold(String::new(), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

/// The on-disk store.
#[derive(Debug, Clone)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    /// A store rooted at `root` (created on first write).
    #[must_use]
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The store configured by the environment.
    #[must_use]
    pub fn from_env() -> Self {
        let root = std::env::var(DIR_ENV).map_or_else(
            |_| std::env::temp_dir().join("ppm-artifacts"),
            PathBuf::from,
        );
        Self::at(root)
    }

    fn path_of(&self, key: &str) -> Option<PathBuf> {
        // A key is `ab/cd/<uuid>`: refuse anything else so a stored key can
        // never escape the root.
        let ok = key.split('/').count() == 3
            && key
                .split('/')
                .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_hexdigit() || b == b'-'));
        ok.then(|| self.root.join(key))
    }

    /// Write `bytes` under a fresh key and return it.
    ///
    /// # Errors
    ///
    /// Any I/O error creating the directory or writing the file.
    pub fn put(&self, bytes: &[u8]) -> std::io::Result<String> {
        let id = Uuid::new_v4().simple().to_string();
        let key = format!("{}/{}/{id}", &id[0..2], &id[2..4]);
        let path = self.root.join(&key);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&path, bytes)?;
        Ok(key)
    }

    /// Read the bytes stored under `key`.
    ///
    /// # Errors
    ///
    /// `NotFound` for an unknown or malformed key; any other I/O error.
    pub fn get(&self, key: &str) -> std::io::Result<Vec<u8>> {
        let path = self
            .path_of(key)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
        std::fs::read(path)
    }

    /// Delete the file under `key`; absent is fine.
    ///
    /// # Errors
    ///
    /// Any I/O error other than the file already being gone.
    pub fn remove(&self, key: &str) -> std::io::Result<()> {
        let Some(path) = self.path_of(key) else {
            return Ok(());
        };
        match std::fs::remove_file(path) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    }

    /// The root directory (for diagnostics and tests).
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_types_normalize_and_unsafe_ones_are_refused() {
        assert_eq!(
            media_type("Application/PDF; charset=binary"),
            "application/pdf"
        );
        assert!(upload_problems("application/pdf", 10, 100).is_empty());
        for bad in [
            "text/html",
            "image/svg+xml",
            "application/x-msdownload",
            "application/zip",
            "",
            "image/png2",
        ] {
            assert!(
                !upload_problems(bad, 10, 100).is_empty(),
                "{bad:?} must be refused"
            );
        }
    }

    #[test]
    fn size_is_checked_before_anything_is_stored() {
        assert!(
            !upload_problems("application/pdf", 0, 100).is_empty(),
            "empty"
        );
        assert!(
            !upload_problems("application/pdf", 101, 100).is_empty(),
            "over the cap"
        );
        assert!(
            upload_problems("application/pdf", 100, 100).is_empty(),
            "exactly at the cap"
        );
    }

    #[test]
    fn filenames_lose_paths_and_control_characters_but_keep_utf8() {
        assert_eq!(sanitize_filename("../../etc/passwd"), "passwd");
        assert_eq!(sanitize_filename("C:\\Users\\x\\report.pdf"), "report.pdf");
        assert_eq!(sanitize_filename("отчёт 2026.pdf"), "отчёт 2026.pdf");
        assert_eq!(sanitize_filename("a\u{0}b\nc.txt"), "abc.txt");
        assert_eq!(sanitize_filename("  ...  "), "file");
        assert_eq!(sanitize_filename(&"x".repeat(500)).chars().count(), 200);
    }

    #[test]
    fn content_disposition_keeps_a_non_latin_name_intact_without_a_double_extension() {
        let cd = content_disposition("отчёт.pdf");
        assert!(cd.starts_with("attachment; filename=\""));
        assert!(
            cd.contains("filename*=UTF-8''%D0%BE%D1%82%D1%87%D1%91%D1%82.pdf"),
            "{cd}"
        );
        assert_eq!(
            cd.matches(".pdf").count(),
            2,
            "once per form, never doubled: {cd}"
        );
        let tricky = content_disposition("a\"b;c.txt");
        assert!(
            !tricky.contains("a\"b"),
            "a quote cannot break out of the header: {tricky}"
        );
    }

    #[test]
    fn the_store_round_trips_and_refuses_a_key_that_escapes_the_root() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path());
        let key = store.put(b"hello").unwrap();
        assert_eq!(store.get(&key).unwrap(), b"hello");
        assert_eq!(
            sha256_hex(b"hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        assert!(store.get("../../etc/passwd").is_err());
        assert!(store.get("aa/bb/../../x").is_err());
        store.remove(&key).unwrap();
        assert!(store.get(&key).is_err(), "removed");
        store.remove(&key).unwrap();
        store.remove("../escape").unwrap();
    }
}
