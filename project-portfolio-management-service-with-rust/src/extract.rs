//! Text extraction from attachments (PL-8): pure where it can be, bounded
//! everywhere. Unit-tested.
//!
//! What is extracted: plain text, CSV, Markdown and JSON (as UTF-8), and the
//! Office formats (`.docx`, `.xlsx`, `.pptx`), which are zip containers of XML.
//! What is **not**: PDF (no parser is bundled) and images (no OCR) — those are
//! recorded `unindexed` with the reason, never silently skipped.
//!
//! Extraction reads **untrusted files**, so it is bounded on every axis: the
//! number of zip entries, the decompressed size of each entry and of the whole
//! archive (a zip bomb stops at the cap), and the length of the text kept.

use std::io::Read;

/// Characters of text kept per attachment.
pub const MAX_TEXT_CHARS: usize = 1_000_000;
/// Zip entries read.
pub const MAX_ENTRIES: usize = 500;
/// Decompressed bytes read from one entry.
pub const MAX_ENTRY_BYTES: u64 = 5 * 1024 * 1024;
/// Decompressed bytes read from the whole archive.
pub const MAX_TOTAL_BYTES: u64 = 20 * 1024 * 1024;

/// The outcome of trying to extract text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extraction {
    /// The text found.
    Text(String),
    /// No text, and why.
    Unindexed(&'static str),
}

/// Strip XML tags and decode the five predefined entities and numeric
/// character references, turning each tag boundary into a space so adjacent
/// words do not run together.
#[must_use]
pub fn strip_xml(xml: &str) -> String {
    let mut out = String::with_capacity(xml.len() / 2);
    let mut in_tag = false;
    let mut chars = xml.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '<' => {
                in_tag = true;
                out.push(' ');
            }
            '>' => in_tag = false,
            '&' if !in_tag => {
                let mut ent = String::new();
                while let Some(&n) = chars.peek() {
                    chars.next();
                    if n == ';' {
                        break;
                    }
                    ent.push(n);
                    if ent.len() > 8 {
                        break;
                    }
                }
                out.push_str(&decode_entity(&ent));
            }
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_entity(e: &str) -> String {
    match e {
        "amp" => "&".into(),
        "lt" => "<".into(),
        "gt" => ">".into(),
        "quot" => "\"".into(),
        "apos" => "'".into(),
        _ => e
            .strip_prefix('#')
            .and_then(|n| {
                n.strip_prefix(['x', 'X']).map_or_else(
                    || n.parse::<u32>().ok(),
                    |h| u32::from_str_radix(h, 16).ok(),
                )
            })
            .and_then(char::from_u32)
            .map_or_else(String::new, |c| c.to_string()),
    }
}

fn office_text(bytes: &[u8]) -> Result<String, &'static str> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|_| "unreadable_archive")?;
    if zip.len() > MAX_ENTRIES {
        return Err("too_many_entries");
    }
    let mut text = String::new();
    let mut total = 0u64;
    for i in 0..zip.len() {
        let Ok(entry) = zip.by_index(i) else { continue };
        let name = entry.name().to_string();
        // The parts that carry prose in .docx / .xlsx / .pptx. OOXML part names
        // are case-sensitive, so the exact comparison is intended.
        #[allow(clippy::case_sensitive_file_extension_comparisons)]
        let wanted = (name.starts_with("word/") && name.ends_with(".xml"))
            || (name.starts_with("ppt/slides/") && name.ends_with(".xml"))
            || name == "xl/sharedStrings.xml";
        if !wanted {
            continue;
        }
        let mut xml = String::new();
        let mut limited = entry.take(MAX_ENTRY_BYTES);
        if limited.read_to_string(&mut xml).is_err() {
            continue; // not UTF-8: skip the part, keep the rest
        }
        total += xml.len() as u64;
        if total > MAX_TOTAL_BYTES {
            return Err("archive_too_large");
        }
        text.push_str(&strip_xml(&xml));
        text.push(' ');
        if text.chars().count() > MAX_TEXT_CHARS {
            break;
        }
    }
    Ok(text)
}

/// Extract text from `bytes` of the declared (bare) media type.
#[must_use]
pub fn extract(media_type: &str, bytes: &[u8]) -> Extraction {
    let text = match media_type {
        "text/plain" | "text/csv" | "text/markdown" | "application/json" => {
            String::from_utf8_lossy(bytes).into_owned()
        }
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
            match office_text(bytes) {
                Ok(t) => t,
                Err(why) => return Extraction::Unindexed(why),
            }
        }
        "application/pdf" => return Extraction::Unindexed("no_extractor_for_pdf"),
        t if t.starts_with("image/") => return Extraction::Unindexed("no_ocr"),
        _ => return Extraction::Unindexed("unsupported_type"),
    };
    let kept: String = text.chars().take(MAX_TEXT_CHARS).collect();
    if kept.trim().is_empty() {
        Extraction::Unindexed("no_text_found")
    } else {
        Extraction::Text(kept)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn zip_of(parts: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut w = zip::ZipWriter::new(&mut buf);
            let opts = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            for (name, body) in parts {
                w.start_file(*name, opts).unwrap();
                w.write_all(body.as_bytes()).unwrap();
            }
            w.finish().unwrap();
        }
        buf.into_inner()
    }

    const DOCX: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document";

    #[test]
    fn xml_becomes_words_with_entities_decoded() {
        assert_eq!(
            strip_xml(
                "<w:p><w:r><w:t>Hello</w:t></w:r><w:r><w:t>&amp; world &#233;&#x41;</w:t></w:r></w:p>"
            ),
            "Hello & world \u{e9}A"
        );
        assert_eq!(strip_xml("a<b>c</b>d"), "a c d", "tags separate words");
    }

    #[test]
    fn plain_types_are_read_as_text() {
        assert_eq!(
            extract("text/plain", "naïve café".as_bytes()),
            Extraction::Text("naïve café".into())
        );
        assert_eq!(
            extract("application/json", br#"{"k":"v"}"#),
            Extraction::Text(r#"{"k":"v"}"#.into())
        );
        assert_eq!(
            extract("text/plain", b"   \n"),
            Extraction::Unindexed("no_text_found")
        );
    }

    #[test]
    fn a_docx_yields_its_prose_and_ignores_the_rest() {
        let docx = zip_of(&[
            (
                "word/document.xml",
                "<w:document><w:body><w:p><w:t>Quarterly risk review</w:t></w:p></w:body></w:document>",
            ),
            ("docProps/core.xml", "<cp>SECRET-METADATA</cp>"),
            ("word/media/image1.png", "not xml"),
        ]);
        let Extraction::Text(t) = extract(DOCX, &docx) else {
            panic!("text")
        };
        assert!(t.contains("Quarterly risk review"), "{t}");
        assert!(
            !t.contains("SECRET-METADATA"),
            "only the prose parts are read"
        );
    }

    #[test]
    fn what_cannot_be_read_is_recorded_with_a_reason_not_dropped() {
        assert_eq!(
            extract("application/pdf", b"%PDF-1.7"),
            Extraction::Unindexed("no_extractor_for_pdf")
        );
        assert_eq!(
            extract("image/png", b"\x89PNG"),
            Extraction::Unindexed("no_ocr")
        );
        assert_eq!(
            extract(DOCX, b"not a zip"),
            Extraction::Unindexed("unreadable_archive")
        );
        assert_eq!(
            extract("application/x-weird", b"x"),
            Extraction::Unindexed("unsupported_type")
        );
    }

    #[test]
    fn a_zip_bomb_stops_at_the_cap_and_text_is_truncated() {
        // One highly compressible entry larger than the per-entry cap.
        let big = "<w:t>".to_string()
            + &"A".repeat(usize::try_from(MAX_ENTRY_BYTES).unwrap() + 1000)
            + "</w:t>";
        let docx = zip_of(&[("word/document.xml", &big)]);
        let Extraction::Text(t) = extract(DOCX, &docx) else {
            panic!("text")
        };
        assert!(
            t.chars().count() <= MAX_TEXT_CHARS,
            "kept text is bounded: {}",
            t.len()
        );
        // Too many entries is refused outright.
        let names: Vec<String> = (0..=MAX_ENTRIES)
            .map(|i| format!("word/p{i}.xml"))
            .collect();
        let parts: Vec<(&str, &str)> = names.iter().map(|n| (n.as_str(), "<a>x</a>")).collect();
        assert_eq!(
            extract(DOCX, &zip_of(&parts)),
            Extraction::Unindexed("too_many_entries")
        );
    }
}
