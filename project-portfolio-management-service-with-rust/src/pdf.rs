//! A tiny, dependency-free PDF writer (PL-10): plain text on A4 pages in
//! Helvetica. DB-free and unit-tested.
//!
//! It exists to give a board a *document* — not to typeset. That keeps it
//! small and **deterministic**: there is no creation date, no random id and no
//! compression, so the same lines always produce the same bytes. Text is
//! written in `WinAnsi` (Latin-1): a character outside it is written as `?`
//! rather than guessed, and `(`, `)` and `\` are escaped, so a plan name can
//! never break out of its string or inject PDF syntax.

/// Characters per wrapped line at 10 pt Helvetica on A4 with 50 pt margins.
const WRAP: usize = 96;
/// Lines per page.
const PER_PAGE: usize = 60;

/// One WinAnsi-safe, escaped PDF string body for `s`.
fn escape(s: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    for c in s.chars() {
        let b = match c {
            '(' | ')' | '\\' => {
                out.push(b'\\');
                c as u8
            }
            c if (' '..='~').contains(&c) => c as u8,
            // Latin-1 supplement is the same code point in WinAnsi.
            c if ('\u{A0}'..='\u{FF}').contains(&c) => c as u8,
            _ => b'?',
        };
        out.push(b);
    }
    out
}

/// Wrap `text` to `width` characters, breaking at spaces where it can and
/// hard-breaking an over-long word. Never produces an empty trailing line.
#[must_use]
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let mut line = String::new();
        for word in raw.split(' ') {
            let mut w = word.to_string();
            while w.chars().count() > width {
                if !line.is_empty() {
                    out.push(std::mem::take(&mut line));
                }
                out.push(w.chars().take(width).collect());
                w = w.chars().skip(width).collect();
            }
            let sep = usize::from(!line.is_empty());
            if line.chars().count() + sep + w.chars().count() > width {
                out.push(std::mem::take(&mut line));
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(&w);
        }
        out.push(line);
    }
    out
}

/// Render `lines` (already wrapped or not) as a PDF document.
#[must_use]
pub fn render(title: &str, lines: &[String]) -> Vec<u8> {
    let mut wrapped: Vec<String> = Vec::new();
    for l in lines {
        wrapped.extend(wrap(l, WRAP));
    }
    let pages: Vec<&[String]> = if wrapped.is_empty() {
        vec![&[][..]]
    } else {
        wrapped.chunks(PER_PAGE).collect()
    };
    let n = pages.len();
    // Object numbering: 1 catalog, 2 pages, 3 font, 4 info, then per page a
    // page object and its content stream.
    let mut objects: Vec<Vec<u8>> = Vec::new();
    objects.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());
    let kids: String = (0..n)
        .map(|i| format!("{} 0 R", 5 + 2 * i))
        .collect::<Vec<_>>()
        .join(" ");
    objects.push(format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>").into_bytes());
    objects.push(
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
            .to_vec(),
    );
    let mut info = b"<< /Title (".to_vec();
    info.extend(escape(title));
    info.extend(b") /Producer (Project Portfolio Management) >>");
    objects.push(info);
    for (i, page) in pages.iter().enumerate() {
        objects.push(
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] \
                 /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>",
                6 + 2 * i
            )
            .into_bytes(),
        );
        let mut content = b"BT\n/F1 10 Tf\n12 TL\n50 792 Td\n".to_vec();
        for l in *page {
            content.push(b'(');
            content.extend(escape(l));
            content.extend(b") '\n");
        }
        content.extend(b"ET\n");
        let mut stream = format!("<< /Length {} >>\nstream\n", content.len()).into_bytes();
        stream.extend(content);
        stream.extend(b"endstream");
        objects.push(stream);
    }
    let mut out = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend(format!("{} 0 obj\n", i + 1).into_bytes());
        out.extend(o);
        out.extend(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).into_bytes());
    for off in offsets {
        out.extend(format!("{off:010} 00000 n \n").into_bytes());
    }
    out.extend(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R /Info 4 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .into_bytes(),
    );
    out
}

/// The visible text of a rendered PDF's content streams (for tests and for
/// asserting that something is *absent* from the text layer).
#[must_use]
pub fn text_of(pdf: &[u8]) -> String {
    let s = String::from_utf8_lossy(pdf);
    s.lines()
        .filter_map(|l| l.strip_suffix(") '").and_then(|l| l.strip_prefix('(')))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn the_file_is_structurally_a_pdf_with_correct_xref_offsets() {
        let pdf = render("T", &lines(&["hello", "world"]));
        assert!(pdf.starts_with(b"%PDF-1.4\n") && pdf.ends_with(b"%%EOF\n"));
        let text = String::from_utf8_lossy(&pdf).to_string();
        let start: usize = text
            .rsplit("startxref\n")
            .next()
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        assert!(
            text[start..].starts_with("xref\n"),
            "startxref points at the xref table"
        );
        // Every xref entry points at "<n> 0 obj".
        let table: Vec<&str> = text[start..]
            .lines()
            .skip(3)
            .take_while(|l| l.ends_with(" n "))
            .collect();
        assert!(!table.is_empty());
        for (i, entry) in table.iter().enumerate() {
            let off: usize = entry[..10].parse().unwrap();
            assert!(
                text[off..].starts_with(&format!("{} 0 obj", i + 1)),
                "object {}",
                i + 1
            );
        }
    }

    #[test]
    fn rendering_is_deterministic() {
        let l = lines(&["Board pack", "as_of 2026-10-03T00:00:00Z", "Group (a) \\ b"]);
        assert_eq!(render("Board pack", &l), render("Board pack", &l));
        assert_ne!(
            render("Board pack", &l),
            render("Board pack", &lines(&["different"]))
        );
    }

    #[test]
    fn text_cannot_break_out_of_its_string_or_inject_syntax() {
        let pdf = render("x", &lines(&["a) Tj 0 0 Td (b \\ )"]));
        let text = text_of(&pdf);
        assert_eq!(
            text, "a\\) Tj 0 0 Td \\(b \\\\ \\)",
            "every metacharacter is escaped: {text}"
        );
    }

    #[test]
    fn characters_outside_winansi_become_question_marks_not_garbage() {
        let text = text_of(&render("x", &lines(&["café Артемида 日本"])));
        assert!(text.contains("caf\u{e9}") || text.contains("caf"), "{text}");
        assert!(text.contains("????????") && !text.contains('А'), "{text}");
    }

    #[test]
    fn long_text_wraps_and_long_documents_paginate() {
        let w = wrap(&"word ".repeat(60), 96);
        assert!(w.len() > 1 && w.iter().all(|l| l.chars().count() <= 96));
        assert_eq!(
            wrap(&"x".repeat(200), 96).len(),
            3,
            "an over-long word is hard-broken"
        );
        let many: Vec<String> = (0..130).map(|i| format!("line {i}")).collect();
        let pdf = render("x", &many);
        let s = String::from_utf8_lossy(&pdf);
        assert!(
            s.contains("/Count 3"),
            "130 lines at 60 per page is three pages"
        );
    }
}
