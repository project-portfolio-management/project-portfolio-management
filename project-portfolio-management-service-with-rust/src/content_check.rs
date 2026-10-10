//! Attachment content checks (RG-8): the bytes must be what the declared type says, executables are
//! refused whatever they claim to be, and an optional virus scanner (`ClamAV`'s `clamd`) gets the last word.
//!
//! The upload path already refuses a type outside the allow-list ([`crate::artifacts::ALLOWED_TYPES`])
//! and a file over the size cap, and serves every download with `nosniff` and
//! `Content-Disposition: attachment`. What it did not do is look at the **content**: a program
//! renamed to `report.pdf` and declared `application/pdf` was stored as sent. This module closes that.
//!
//! Limits, stated: the checks look at the file's signature and, for text, its encoding; they do not
//! parse a PDF or an Office file for macros or exploits (that is what a scanner is for), and a polyglot
//! file that is valid as two formats passes as the declared one.

use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// The environment variable naming a `clamd` TCP address (`host:port`). Unset: no scan.
pub const SCAN_ADDR_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_SCAN_ADDR";

/// How long the scanner has to answer.
const SCAN_TIMEOUT: Duration = Duration::from_secs(30);

/// Signatures of programs and scripts, refused whatever the declared type.
const EXECUTABLE_SIGNATURES: &[(&[u8], &str)] = &[
    (b"MZ", "a Windows program"),
    (b"\x7fELF", "a Linux program"),
    (b"\xfe\xed\xfa\xce", "a macOS program"),
    (b"\xfe\xed\xfa\xcf", "a macOS program"),
    (b"\xce\xfa\xed\xfe", "a macOS program"),
    (b"\xcf\xfa\xed\xfe", "a macOS program"),
    (b"\xca\xfe\xba\xbe", "a Java class or universal binary"),
    (b"#!", "a script"),
];

fn starts(bytes: &[u8], magic: &[u8]) -> bool {
    bytes.len() >= magic.len() && &bytes[..magic.len()] == magic
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

/// What is wrong with these bytes given the declared media type (bare, lowercase). Empty means they
/// look like what they claim.
#[must_use]
pub fn content_problems(media_type: &str, bytes: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    for (magic, what) in EXECUTABLE_SIGNATURES {
        if starts(bytes, magic) {
            out.push(format!(
                "the file is {what}, which is not accepted as an attachment"
            ));
            return out;
        }
    }
    let matches_declared = match media_type {
        "image/png" => starts(bytes, b"\x89PNG\r\n\x1a\n"),
        "image/jpeg" => starts(bytes, b"\xff\xd8\xff"),
        "image/gif" => starts(bytes, b"GIF87a") || starts(bytes, b"GIF89a"),
        "image/webp" => bytes.len() >= 12 && starts(bytes, b"RIFF") && &bytes[8..12] == b"WEBP",
        "application/pdf" => contains(&bytes[..bytes.len().min(1024)], b"%PDF-"),
        t if t.starts_with("application/vnd.openxmlformats-officedocument.") => {
            starts(bytes, b"PK\x03\x04")
                && contains(&bytes[..bytes.len().min(4096)], b"[Content_Types].xml")
        }
        "application/json" => {
            std::str::from_utf8(bytes).is_ok()
                && serde_json::from_slice::<serde_json::Value>(bytes).is_ok()
        }
        "text/plain" | "text/csv" | "text/markdown" => {
            std::str::from_utf8(bytes).is_ok() && !bytes.contains(&0)
        }
        _ => true,
    };
    if !matches_declared {
        out.push(format!(
            "the file's content does not match its declared type {media_type:?}"
        ));
    }
    out
}

/// What the scanner said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// No scanner is configured.
    NotConfigured,
    /// The scanner found nothing.
    Clean,
    /// The scanner found something (its name for it).
    Infected(String),
    /// A scanner is configured but did not give an answer; the upload is refused (fail closed).
    Unavailable(String),
}

/// Parse a `clamd` reply such as `stream: OK` or `stream: Eicar-Test-Signature FOUND`.
#[must_use]
pub fn parse_reply(reply: &str) -> Verdict {
    let r = reply.trim_matches(['\0', '\n', ' ']).trim();
    if r.ends_with("OK") {
        Verdict::Clean
    } else if let Some(found) = r.strip_suffix("FOUND") {
        let name = found.trim().trim_start_matches("stream:").trim();
        Verdict::Infected(if name.is_empty() {
            "unnamed".to_string()
        } else {
            name.to_string()
        })
    } else {
        Verdict::Unavailable(format!("unexpected scanner reply: {r}"))
    }
}

/// Send the bytes to `clamd` at `addr` (`INSTREAM`) and read its verdict.
pub async fn scan_with(addr: &str, bytes: &[u8]) -> Verdict {
    let run = async {
        let mut stream = TcpStream::connect(addr).await.map_err(|e| e.to_string())?;
        stream
            .write_all(b"zINSTREAM\0")
            .await
            .map_err(|e| e.to_string())?;
        for chunk in bytes.chunks(64 * 1024) {
            let len = u32::try_from(chunk.len()).map_err(|e| e.to_string())?;
            stream
                .write_all(&len.to_be_bytes())
                .await
                .map_err(|e| e.to_string())?;
            stream.write_all(chunk).await.map_err(|e| e.to_string())?;
        }
        stream
            .write_all(&0_u32.to_be_bytes())
            .await
            .map_err(|e| e.to_string())?;
        let mut reply = Vec::new();
        stream
            .read_to_end(&mut reply)
            .await
            .map_err(|e| e.to_string())?;
        Ok::<String, String>(String::from_utf8_lossy(&reply).to_string())
    };
    match tokio::time::timeout(SCAN_TIMEOUT, run).await {
        Ok(Ok(reply)) => parse_reply(&reply),
        Ok(Err(e)) => Verdict::Unavailable(e),
        Err(_) => Verdict::Unavailable("the scanner timed out".to_string()),
    }
}

/// Scan with the configured scanner, if any.
pub async fn scan(bytes: &[u8]) -> Verdict {
    match std::env::var(SCAN_ADDR_ENV) {
        Ok(addr) if !addr.trim().is_empty() => scan_with(addr.trim(), bytes).await,
        _ => Verdict::NotConfigured,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    const PNG: &[u8] = b"\x89PNG\r\n\x1a\n....";

    #[test]
    fn a_file_that_matches_its_declared_type_passes() {
        assert!(content_problems("image/png", PNG).is_empty());
        assert!(content_problems("image/jpeg", b"\xff\xd8\xff\xe0..").is_empty());
        assert!(content_problems("image/gif", b"GIF89a....").is_empty());
        assert!(content_problems("image/webp", b"RIFF\0\0\0\0WEBPVP8 ").is_empty());
        assert!(content_problems("application/pdf", b"%PDF-1.7\n...").is_empty());
        assert!(content_problems("text/plain", "héllo\n".as_bytes()).is_empty());
        assert!(content_problems("text/csv", b"a,b\n1,2\n").is_empty());
        assert!(content_problems("application/json", br#"{"a":1}"#).is_empty());
        let docx = [
            b"PK\x03\x04".as_slice(),
            b"\x14\0\0\0\0\0",
            b"[Content_Types].xml",
        ]
        .concat();
        assert!(
            content_problems(
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                &docx
            )
            .is_empty()
        );
    }

    #[test]
    fn a_program_is_refused_whatever_it_claims_to_be() {
        for (bytes, declared) in [
            (b"MZ\x90\0\x03".as_slice(), "application/pdf"),
            (b"\x7fELF\x02\x01", "image/png"),
            (b"\xcf\xfa\xed\xfe\x07", "text/plain"),
            (b"#!/bin/sh\nrm -rf /\n", "text/plain"),
            (b"\xca\xfe\xba\xbe\0\0", "application/json"),
        ] {
            let p = content_problems(declared, bytes);
            assert_eq!(p.len(), 1, "{declared}: {p:?}");
            assert!(p[0].contains("not accepted"), "{p:?}");
        }
    }

    #[test]
    fn content_that_does_not_match_the_declared_type_is_refused() {
        assert_eq!(content_problems("image/png", b"GIF89a....").len(), 1);
        assert_eq!(content_problems("application/pdf", b"just text").len(), 1);
        assert_eq!(content_problems("text/plain", b"abc\0def").len(), 1);
        assert_eq!(content_problems("text/plain", &[0xff, 0xfe, 0xfd]).len(), 1);
        assert_eq!(content_problems("application/json", b"{not json").len(), 1);
        assert_eq!(
            content_problems(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                b"PK\x03\x04 a plain zip"
            )
            .len(),
            1
        );
        // A PDF may have a short preamble before the signature.
        let mut pdf = vec![b' '; 100];
        pdf.extend_from_slice(b"%PDF-1.4");
        assert!(content_problems("application/pdf", &pdf).is_empty());
    }

    #[test]
    fn scanner_replies_are_understood() {
        assert_eq!(parse_reply("stream: OK\0"), Verdict::Clean);
        assert_eq!(
            parse_reply("stream: Eicar-Test-Signature FOUND\0"),
            Verdict::Infected("Eicar-Test-Signature".to_string())
        );
        assert!(matches!(
            parse_reply("INSTREAM size limit exceeded. ERROR"),
            Verdict::Unavailable(_)
        ));
        assert!(matches!(parse_reply(""), Verdict::Unavailable(_)));
    }

    /// A fake `clamd`: reads the INSTREAM framing, checks the payload, answers.
    async fn fake_clamd(reply: &'static str) -> (String, tokio::task::JoinHandle<Vec<u8>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        let handle = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await.unwrap();
            let mut command = [0_u8; 10];
            s.read_exact(&mut command).await.unwrap();
            assert_eq!(&command, b"zINSTREAM\0");
            let mut payload = Vec::new();
            loop {
                let mut len = [0_u8; 4];
                s.read_exact(&mut len).await.unwrap();
                let n = u32::from_be_bytes(len) as usize;
                if n == 0 {
                    break;
                }
                let mut chunk = vec![0_u8; n];
                s.read_exact(&mut chunk).await.unwrap();
                payload.extend_from_slice(&chunk);
            }
            s.write_all(reply.as_bytes()).await.unwrap();
            payload
        });
        (addr, handle)
    }

    #[tokio::test]
    async fn the_scanner_is_sent_the_whole_file_in_frames_and_its_verdict_is_used() {
        let big = vec![7_u8; 150_000]; // three frames
        let (addr, server) = fake_clamd("stream: OK\0").await;
        assert_eq!(scan_with(&addr, &big).await, Verdict::Clean);
        assert_eq!(server.await.unwrap(), big);

        let (addr, server) = fake_clamd("stream: Win.Test.EICAR_HDB-1 FOUND\0").await;
        assert_eq!(
            scan_with(&addr, b"X5O!P%@AP").await,
            Verdict::Infected("Win.Test.EICAR_HDB-1".to_string())
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn an_unreachable_scanner_fails_closed() {
        // A port nothing listens on.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        drop(listener);
        assert!(matches!(
            scan_with(&addr, b"data").await,
            Verdict::Unavailable(_)
        ));
    }

    /// Hostile input must be refused or accepted, never a panic or a hang: every declared type, many
    /// short byte strings that start like the real signatures and then go wrong.
    #[test]
    fn arbitrary_bytes_never_panic() {
        let types = [
            "image/png",
            "image/jpeg",
            "image/gif",
            "image/webp",
            "application/pdf",
            "text/plain",
            "text/csv",
            "text/markdown",
            "application/json",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "application/zip",
            "",
            "garbage/\u{1F600}",
        ];
        let prefixes: [&[u8]; 14] = [
            b"",
            b"M",
            b"MZ",
            b"\x7fEL",
            b"\x89PNG\r\n\x1a",
            b"\xff\xd8",
            b"GIF8",
            b"RIFF",
            b"RIFF\0\0\0\0WEB",
            b"%PDF",
            b"PK\x03",
            b"#",
            b"{",
            b"\xca\xfe\xba",
        ];
        let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..20_000 {
            let mut bytes = prefixes[(next() % 14) as usize].to_vec();
            for _ in 0..(next() % 120) {
                bytes.push((next() & 0xff) as u8);
            }
            for t in &types {
                let _ = content_problems(t, &bytes);
            }
        }
        // And one big input, to be sure nothing is quadratic in the size.
        let big = vec![b'a'; 20 * 1024 * 1024];
        let started = std::time::Instant::now();
        for t in &types {
            let _ = content_problems(t, &big);
        }
        assert!(
            started.elapsed() < std::time::Duration::from_secs(20),
            "{:?}",
            started.elapsed()
        );
    }
}
