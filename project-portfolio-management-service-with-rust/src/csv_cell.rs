//! One CSV cell, written so a spreadsheet cannot run it (CQ-1). Every CSV this service writes goes
//! through [`field`]; the plan import reads cells back through [`read`], so an exported plan
//! re-imports unchanged.
//!
//! A cell that starts with `=`, `+`, `-`, `@`, a tab or a carriage return is read by spreadsheet
//! programs as the start of a formula, so it is prefixed with `'`, which they
//! show as text. Two exceptions keep the escape reversible and the numbers useful:
//!
//! - a plain number (`-12`, `+3.5`) is left alone: it cannot be a formula, and a report's negative
//!   variance stays a number in the spreadsheet;
//! - a cell that already starts with `'` followed by one of those characters, or by another `'`, is
//!   prefixed too, so [`read`] can strip exactly one `'` without guessing.

/// The characters that make a spreadsheet read a cell as a formula.
const TRIGGERS: [char; 6] = ['=', '+', '-', '@', '\t', '\r'];

/// Whether the cell is a plain decimal number, optionally signed: `-12`, `+3.5`, `0.25`.
fn plain_number(v: &str) -> bool {
    let digits = v.strip_prefix(['+', '-']).unwrap_or(v);
    let (whole, frac) = digits.split_once('.').unwrap_or((digits, "0"));
    !whole.is_empty()
        && !frac.is_empty()
        && whole.bytes().all(|b| b.is_ascii_digit())
        && frac.bytes().all(|b| b.is_ascii_digit())
}

/// Whether the cell needs the `'` prefix.
fn needs_prefix(v: &str) -> bool {
    let mut chars = v.chars();
    match chars.next() {
        Some(c) if TRIGGERS.contains(&c) => !plain_number(v),
        Some('\'') => chars
            .next()
            .is_some_and(|c| c == '\'' || TRIGGERS.contains(&c)),
        _ => false,
    }
}

/// One CSV field: a formula is defused as above, then the field is quoted when it holds a comma,
/// quote or line break, with any quote doubled (RFC 4180).
#[must_use]
pub fn field(value: &str) -> String {
    let mut v = value.to_string();
    if needs_prefix(&v) {
        v.insert(0, '\'');
    }
    if v.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", v.replace('"', "\"\""))
    } else {
        v
    }
}

/// A parsed cell back to its value: the inverse of the prefix [`field`] adds.
#[must_use]
pub fn read(cell: &str) -> &str {
    match cell.strip_prefix('\'') {
        Some(rest) if needs_prefix(rest) => rest,
        _ => cell,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_commas_quotes_and_line_breaks() {
        assert_eq!(field("plain"), "plain");
        assert_eq!(field("a,b"), "\"a,b\"");
        assert_eq!(field("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(field("line\nbreak"), "\"line\nbreak\"");
    }

    #[test]
    fn defuses_every_formula_start() {
        assert_eq!(field("=HYPERLINK(\"x\")"), "\"'=HYPERLINK(\"\"x\"\")\"");
        assert_eq!(field("=cmd()"), "'=cmd()");
        assert_eq!(field("@SUM(A1)"), "'@SUM(A1)");
        assert_eq!(field("+1+1"), "'+1+1");
        assert_eq!(field("-2+3"), "'-2+3");
        assert_eq!(field("-cmd|' /C calc'!A0"), "'-cmd|' /C calc'!A0");
        assert_eq!(field("\t=1"), "'\t=1");
        assert_eq!(field("\r=1"), "\"'\r=1\"");
        // A formula behind a quote already starts with the quote, so it is text.
        assert_eq!(field("\"=1\""), "\"\"\"=1\"\"\"");
    }

    #[test]
    fn a_plain_number_stays_a_number() {
        for n in ["-1", "+1", "-12.5", "0.25", "42"] {
            assert_eq!(field(n), n);
        }
        // Not plain numbers: still defused.
        for v in ["-", "-.5", "-1.", "-1e3", "+-1"] {
            assert_eq!(field(v), format!("'{v}"), "{v}");
        }
    }

    #[test]
    fn read_is_the_inverse_of_the_prefix() {
        for v in [
            "plain", "=x", "-1", "-2+3", "'=x", "''=x", "'", "''", "'a", "'-1", "@a", "\t=1",
        ] {
            let written = field(v);
            let unquoted = written
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .map_or(written.clone(), |s| s.replace("\"\"", "\""));
            assert_eq!(read(&unquoted), v, "{v:?} written as {written:?}");
        }
    }

    /// CQ-1: every CSV field goes through [`field`]. Four hand-written escapers had drifted, and
    /// three of them let a formula through; a new one fails here.
    #[test]
    fn no_other_file_escapes_csv_by_hand() {
        let needle = r#"replace('"', "\"\"")"#;
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![root];
        let mut offenders = Vec::new();
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read src") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs")
                    && path.file_name().is_some_and(|n| n != "csv_cell.rs")
                    && std::fs::read_to_string(&path)
                        .expect("read file")
                        .contains(needle)
                {
                    offenders.push(path.display().to_string());
                }
            }
        }
        assert!(offenders.is_empty(), "escapes CSV by hand: {offenders:?}");
    }
}
