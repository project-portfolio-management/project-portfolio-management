//! Attachment request tests (LT-5): refused before storing (type, size,
//! empty), the SHA-256 matches, a download carries safe headers and a UTF-8
//! name, tampering is detected, and removal hides the file.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axum::body::Bytes;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::artifacts::sha256_hex;
use serde_json::{Value, json};
use serial_test::serial;

fn stored_files(dir: &std::path::Path) -> usize {
    let mut n = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        if let Ok(rd) = std::fs::read_dir(&d) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else {
                    n += 1;
                }
            }
        }
    }
    n
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn attachments_are_refused_before_storing_verified_and_served_safely() {
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("tempdir");
    // SAFETY: `#[serial]` — nothing else runs while these are set.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES", "100");
    }
    let root = dir.path().to_path_buf();
    request::<App, _, _>(move |request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let upload = |name: &str, ctype: &str, body: &[u8]| {
            let url = format!("/api/attachments?subject_kind=plan&subject_pid={p}&filename={name}");
            let (request, ctype, body) = (&request, ctype.to_string(), Bytes::copy_from_slice(body));
            async move { request.post(&url).content_type(&ctype).bytes(body).await }
        };

        // Refused BEFORE anything is stored.
        assert_eq!(upload("x.html", "text/html", b"<script>1</script>").await.status_code(), 422, "markup");
        assert_eq!(upload("x.svg", "image/svg+xml", b"<svg/>").await.status_code(), 422, "svg");
        assert_eq!(upload("x.zip", "application/zip", b"PK").await.status_code(), 422, "archive");
        assert_eq!(upload("x.pdf", "application/pdf", b"").await.status_code(), 422, "empty");
        assert_eq!(upload("x.pdf", "application/pdf", &[b'a'; 101]).await.status_code(), 422, "over the cap");
        assert_eq!(stored_files(&root), 0, "no refused upload left a byte on disk");
        let none: Value = request.get(&format!("/api/attachments?subject_kind=plan&subject_pid={p}")).await.json();
        assert!(none.as_array().unwrap().is_empty());

        // An unknown subject and an unknown kind.
        assert_eq!(
            request
                .post("/api/attachments?subject_kind=plan&subject_pid=3f2c6b7e-0000-4000-8000-000000000bad&filename=a.pdf")
                .content_type("application/pdf")
                .bytes(Bytes::from_static(b"%PDF-1.7"))
                .await
                .status_code(),
            404
        );
        assert_eq!(
            request
                .post(&format!("/api/attachments?subject_kind=nope&subject_pid={p}&filename=a.pdf"))
                .content_type("application/pdf")
                .bytes(Bytes::from_static(b"%PDF-1.7"))
                .await
                .status_code(),
            422
        );

        // A good upload, under a non-Latin name (percent-encoded in the query).
        let content = b"%PDF-1.7 hello";
        let ok = upload("%D0%BE%D1%82%D1%87%D1%91%D1%82.pdf", "application/pdf; charset=binary", content).await;
        ok.assert_status_ok();
        let up: Value = ok.json();
        assert_eq!(up["filename"], "отчёт.pdf", "UTF-8 survives upload");
        assert_eq!(up["sha256"], sha256_hex(content), "the stored hash is the hash of the bytes");
        assert_eq!(up["content_type"], "application/pdf");
        assert_eq!(stored_files(&root), 1);
        let att = up["pid"].as_str().unwrap().to_string();

        let listed: Value = request.get(&format!("/api/attachments?subject_kind=plan&subject_pid={p}")).await.json();
        assert_eq!(listed[0]["size_bytes"], content.len());

        // Download: same bytes, safe headers, the real name.
        let dl = request.get(&format!("/api/attachments/{att}/download")).await;
        dl.assert_status_ok();
        assert_eq!(dl.as_bytes().as_ref(), content);
        let h = dl.headers();
        assert_eq!(h.get("x-content-type-options").unwrap(), "nosniff");
        assert_eq!(h.get("cache-control").unwrap(), "private, no-store");
        let cd = h.get("content-disposition").unwrap().to_str().unwrap().to_string();
        assert!(cd.starts_with("attachment;"), "{cd}");
        assert!(cd.contains("filename*=UTF-8''%D0%BE%D1%82%D1%87%D1%91%D1%82.pdf"), "{cd}");
        assert_eq!(cd.matches(".pdf").count(), 2, "never a double extension: {cd}");

        // Tamper with the file on disk: the download refuses rather than serve it.
        let file = walk(&root);
        std::fs::write(&file, b"%PDF-1.7 EVIL!!").unwrap();
        assert_eq!(request.get(&format!("/api/attachments/{att}/download")).await.status_code(), 500);
        std::fs::write(&file, content).unwrap();
        request.get(&format!("/api/attachments/{att}/download")).await.assert_status_ok();

        // Remove (POST twin): hidden from the list and the download.
        request.post(&format!("/api/attachments/{att}/delete")).await.assert_status_ok();
        let after: Value = request.get(&format!("/api/attachments?subject_kind=plan&subject_pid={p}")).await.json();
        assert!(after.as_array().unwrap().is_empty());
        assert_eq!(request.get(&format!("/api/attachments/{att}/download")).await.status_code(), 404);
    })
    .await;
    unsafe {
        std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR");
        std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES");
    }
}

/// The one stored file under `dir`.
fn walk(dir: &std::path::Path) -> std::path::PathBuf {
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                return p;
            }
        }
    }
    panic!("no stored file");
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn attachment_text_is_searchable_and_honest_about_what_it_cannot_read() {
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("tempdir");
    // SAFETY: `#[serial]` — nothing else runs while these are set.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path());
        std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES");
    }
    request::<App, _, _>(move |request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let upload = |name: &str, ctype: &str, body: &[u8]| {
            let url = format!("/api/attachments?subject_kind=plan&subject_pid={p}&filename={name}");
            let (request, ctype, body) =
                (&request, ctype.to_string(), Bytes::copy_from_slice(body));
            async move { request.post(&url).content_type(&ctype).bytes(body).await }
        };
        let search = |q: &str| {
            let (request, q) = (&request, q.to_string());
            async move { request.get(&format!("/api/attachments/search?q={q}")).await }
        };

        let note: Value = upload(
            "risk.txt",
            "text/plain",
            b"The vendor contract renews in March; negotiate the penalty clause.",
        )
        .await
        .json();
        let pdf = upload("scan.pdf", "application/pdf", b"%PDF-1.7 penalty").await;
        assert_eq!(pdf.status_code(), 200, "an unreadable type still uploads");

        let hit: Value = search("penalty").await.json();
        let results = hit["results"].as_array().unwrap();
        assert_eq!(results.len(), 1, "{hit}");
        assert_eq!(results[0]["attachment_pid"], note["pid"]);
        assert_eq!(results[0]["filename"], "risk.txt");
        assert!(
            results[0]["snippet"]
                .as_str()
                .unwrap()
                .to_lowercase()
                .contains("penalty")
        );
        assert_eq!(
            hit["unindexed_attachments"], 1,
            "the PDF is reported, not silently skipped"
        );

        // A miss, a too-short query, and a removed attachment.
        let miss: Value = search("zebra").await.json();
        assert!(miss["results"].as_array().unwrap().is_empty());
        assert_eq!(search("a").await.status_code(), 422);
        let pid = note["pid"].as_str().unwrap();
        request
            .post(&format!("/api/attachments/{pid}/delete"))
            .await
            .assert_status_ok();
        let gone: Value = search("penalty").await.json();
        assert!(
            gone["results"].as_array().unwrap().is_empty(),
            "a removed file is not found"
        );
    })
    .await;
}

/// A stand-in for `clamd`: answers FOUND when the payload contains `EICAR`, else OK.
async fn fake_clamd() -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    tokio::spawn(async move {
        loop {
            let Ok((mut s, _)) = listener.accept().await else {
                return;
            };
            let mut command = [0_u8; 10];
            if s.read_exact(&mut command).await.is_err() {
                continue;
            }
            let mut payload = Vec::new();
            loop {
                let mut len = [0_u8; 4];
                if s.read_exact(&mut len).await.is_err() {
                    break;
                }
                let n = u32::from_be_bytes(len) as usize;
                if n == 0 {
                    break;
                }
                let mut chunk = vec![0_u8; n];
                if s.read_exact(&mut chunk).await.is_err() {
                    break;
                }
                payload.extend_from_slice(&chunk);
            }
            let found = payload.windows(5).any(|w| w == b"EICAR");
            let _ = s
                .write_all(if found {
                    b"stream: Test.EICAR FOUND\0"
                } else {
                    b"stream: OK\0"
                })
                .await;
        }
    });
    addr
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn content_that_lies_is_refused_and_a_scanner_has_the_last_word() {
    super::isolate_search_index();
    let dir = tempfile::tempdir().expect("temp dir");
    let addr = fake_clamd().await;
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path());
        std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES");
    }
    let root = dir.path().to_path_buf();
    request::<App, _, _>(move |request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("pid").to_string();
        let upload = |name: &str, ctype: &str, body: &[u8]| {
            let url = format!("/api/attachments?subject_kind=plan&subject_pid={p}&filename={name}");
            let (request, ctype, body) =
                (&request, ctype.to_string(), Bytes::copy_from_slice(body));
            async move { request.post(&url).content_type(&ctype).bytes(body).await }
        };

        // No scanner configured: the content checks alone.
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_SCAN_ADDR") };
        let program = upload("report.pdf", "application/pdf", b"MZ\x90\0 a program").await;
        assert_eq!(program.status_code(), 422, "a program renamed report.pdf");
        assert!(program.text().contains("not accepted"));
        assert_eq!(
            upload("a.png", "image/png", b"GIF89a....")
                .await
                .status_code(),
            422,
            "wrong type"
        );
        assert_eq!(
            upload("a.txt", "text/plain", b"abc\0def")
                .await
                .status_code(),
            422,
            "binary as text"
        );
        assert_eq!(stored_files(&root), 0, "nothing refused was stored");
        upload("ok.pdf", "application/pdf", b"%PDF-1.7 fine")
            .await
            .assert_status_ok();
        assert_eq!(stored_files(&root), 1);

        // A scanner that says FOUND quarantines (nothing stored); one that says OK lets it through.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_SCAN_ADDR", &addr) };
        let infected = upload("eicar.txt", "text/plain", b"X5O!P%@AP EICAR test").await;
        assert_eq!(infected.status_code(), 422);
        assert!(infected.text().contains("malware scan"));
        assert_eq!(stored_files(&root), 1, "the infected file was not stored");
        upload("clean.txt", "text/plain", b"nothing to see")
            .await
            .assert_status_ok();
        assert_eq!(stored_files(&root), 2);

        // A configured scanner that cannot answer fails closed.
        unsafe {
            std::env::set_var(
                "PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_SCAN_ADDR",
                "127.0.0.1:1",
            )
        };
        let down = upload("later.txt", "text/plain", b"another file").await;
        assert_eq!(down.status_code(), 503);
        assert_eq!(stored_files(&root), 2);
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_SCAN_ADDR") };
    })
    .await;
}
