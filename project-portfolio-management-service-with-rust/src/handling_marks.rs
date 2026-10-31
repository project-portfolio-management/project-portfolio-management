//! Handling marks (RG-7): a security classification or sensitivity label on a plan, and the clearance
//! a caller needs to see it. The pure rules.
//!
//! A deployment that needs them configures an ordered **scheme**, lowest to highest, in
//! `PROJECT_PORTFOLIO_MANAGEMENT_HANDLING_MARKS` (for example `["OFFICIAL","OFFICIAL-SENSITIVE"]`, or
//! `["Public","Internal","Confidential","Restricted"]`). With no scheme the feature does nothing.
//!
//! * A plan has a mark, or none; **none means the lowest level** of the scheme.
//! * A caller's clearance is the `clearance` attribute the identity provider issues; none, or a value
//!   not in the scheme, means the lowest level.
//! * A caller sees a plan only if their level is **at least the plan's**. This applies to everyone,
//!   administrators included: a label that an administrator role ignores is not a control.
//! * A mark that is not in the scheme (the scheme was changed) counts as the **highest** level, so a
//!   configuration mistake hides rather than shows.
//! * You can set a mark only up to your own clearance, and only on a plan you can already see.
//!
//! This hides and labels; it does not encrypt, and it does not stop a person with clearance from
//! copying what they read. The mark is sent as `X-Handling-Mark` on every response about a plan.
//!
//! **Any script (WW-1.11).** A code is letters and digits of any script (`char::is_alphanumeric`), so a
//! scheme can be written in the language it is used in (`["公开","内部","秘密"]`, `["عام","سري"]`); before
//! 2026-10-31 only ASCII letters were allowed. Codes are compared exactly, as stored: a clearance must be
//! written in the same Unicode form (NFC is usual) or it does not match and counts as the lowest level. An
//! HTTP header carries ASCII only, so a mark with any other character is sent as an RFC 8187 extended value,
//! `UTF-8''` followed by its percent-encoded UTF-8 ([`header_value`]).

use std::fmt::Write as _;
use std::sync::OnceLock;

/// The environment variable holding the scheme as a JSON array of strings, lowest first.
pub const SCHEME_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_HANDLING_MARKS";

/// The longest a mark code may be.
pub const MAX_CODE: usize = 40;
/// The most levels a scheme may have.
pub const MAX_LEVELS: usize = 12;

/// The `X-Handling-Mark` header value for a mark: the mark itself when it is ASCII, else the RFC 8187
/// extended value `UTF-8''<percent-encoded UTF-8>` (a code never holds `'`, so the two cannot be confused).
#[must_use]
pub fn header_value(mark: &str) -> String {
    if mark.is_ascii() {
        return mark.to_string();
    }
    let mut out = String::from("UTF-8''");
    for b in mark.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(char::from(b));
        } else {
            let _ = write!(out, "%{b:02X}");
        }
    }
    out
}

/// An ordered scheme of marks, lowest first.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scheme {
    levels: Vec<String>,
}

impl Scheme {
    /// Parse a JSON array of codes.
    ///
    /// # Errors
    ///
    /// A message naming the first thing wrong: not an array of strings, empty, too many, a bad code,
    /// or a duplicate.
    pub fn parse(json: &str) -> Result<Self, String> {
        let raw: Vec<String> = serde_json::from_str(json).map_err(|_| {
            "the handling marks must be a JSON array of strings, lowest first".to_string()
        })?;
        if raw.is_empty() {
            return Err("the handling marks list is empty".to_string());
        }
        if raw.len() > MAX_LEVELS {
            return Err(format!("at most {MAX_LEVELS} levels"));
        }
        let mut levels: Vec<String> = Vec::new();
        for code in raw {
            let ok = !code.is_empty()
                && code.chars().count() <= MAX_CODE
                && code == code.trim()
                && code
                    .chars()
                    .all(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '/'));
            if !ok {
                return Err(format!(
                    "{code:?} is not a valid mark: 1 to {MAX_CODE} letters, digits, spaces, '-', '_' or '/', with no leading or trailing space"
                ));
            }
            if levels
                .iter()
                .any(|l| l.to_lowercase() == code.to_lowercase())
            {
                return Err(format!("{code:?} appears twice"));
            }
            levels.push(code);
        }
        Ok(Self { levels })
    }

    /// Whether any scheme is configured.
    #[must_use]
    pub fn is_on(&self) -> bool {
        !self.levels.is_empty()
    }

    /// The codes, lowest first.
    #[must_use]
    pub fn levels(&self) -> &[String] {
        &self.levels
    }

    /// The level (0 is the lowest) of an exact code, if it is in the scheme.
    #[must_use]
    pub fn level_of(&self, code: &str) -> Option<usize> {
        self.levels.iter().position(|l| l == code)
    }

    /// The level a plan's stored mark means: none is the lowest, a code not in the scheme is the
    /// highest (fail closed).
    #[must_use]
    pub fn plan_level(&self, mark: Option<&str>) -> usize {
        match mark {
            None => 0,
            Some(m) => self
                .level_of(m)
                .unwrap_or(self.levels.len().saturating_sub(1)),
        }
    }

    /// The level a caller's clearance means: none or unknown is the lowest.
    #[must_use]
    pub fn caller_level(&self, clearance: Option<&str>) -> usize {
        clearance.and_then(|c| self.level_of(c)).unwrap_or(0)
    }

    /// The mark to show for a plan: its own, or the lowest level when it has none.
    #[must_use]
    pub fn effective<'a>(&'a self, mark: Option<&'a str>) -> Option<&'a str> {
        if !self.is_on() {
            return None;
        }
        match mark {
            Some(m) if self.level_of(m).is_some() => Some(m),
            Some(_) => self.levels.last().map(String::as_str),
            None => self.levels.first().map(String::as_str),
        }
    }

    /// Whether a caller with `clearance` may see a plan marked `mark`. Always true with no scheme.
    #[must_use]
    pub fn can_see(&self, mark: Option<&str>, clearance: Option<&str>) -> bool {
        !self.is_on() || self.caller_level(clearance) >= self.plan_level(mark)
    }

    /// Whether the caller may change a plan from `current` to `new` (`None` resets to the lowest).
    ///
    /// # Errors
    ///
    /// A message when there is no scheme, the new mark is not in it, or the caller's clearance is below
    /// the old or the new mark.
    pub fn can_set(
        &self,
        current: Option<&str>,
        new: Option<&str>,
        clearance: Option<&str>,
    ) -> Result<(), String> {
        if !self.is_on() {
            return Err("no handling marks are configured for this deployment".to_string());
        }
        if let Some(n) = new
            && self.level_of(n).is_none()
        {
            return Err(format!("{n:?} is not one of {:?}", self.levels));
        }
        let me = self.caller_level(clearance);
        if me < self.plan_level(current) {
            return Err("you cannot change the mark of a plan above your clearance".to_string());
        }
        if me < self.plan_level(new) {
            return Err("you cannot mark a plan above your own clearance".to_string());
        }
        Ok(())
    }
}

/// The scheme this process uses, read once from the environment. A bad value is logged and means **no
/// scheme** would be the dangerous reading (everything visible), so it panics at first use instead: a
/// deployment that asked for marks and got a typo must not start without them.
///
/// # Panics
///
/// When the variable is set to something that is not a valid scheme.
#[must_use]
pub fn scheme() -> &'static Scheme {
    static SCHEME: OnceLock<Scheme> = OnceLock::new();
    SCHEME.get_or_init(|| match std::env::var(SCHEME_ENV) {
        Ok(v) if !v.trim().is_empty() => {
            Scheme::parse(&v).unwrap_or_else(|e| panic!("{SCHEME_ENV} is invalid: {e}"))
        }
        _ => Scheme::default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uk() -> Scheme {
        Scheme::parse(r#"["OFFICIAL","OFFICIAL-SENSITIVE"]"#).unwrap()
    }
    fn four() -> Scheme {
        Scheme::parse(r#"["Public","Internal","Confidential","Restricted"]"#).unwrap()
    }

    #[test]
    fn a_scheme_is_an_ordered_list_of_distinct_codes() {
        assert_eq!(uk().levels(), ["OFFICIAL", "OFFICIAL-SENSITIVE"]);
        for bad in [
            "",
            "[]",
            "{}",
            r#"["a",1]"#,
            r#"[" a"]"#,
            r#"["a "]"#,
            r#"[""]"#,
            r#"["a","A"]"#,
            r#"["a;b"]"#,
            r#"["<script>"]"#,
        ] {
            assert!(Scheme::parse(bad).is_err(), "{bad}");
        }
        let long = format!(r#"["{}"]"#, "x".repeat(MAX_CODE + 1));
        assert!(Scheme::parse(&long).is_err());
        let many = format!(
            "[{}]",
            (0..=MAX_LEVELS)
                .map(|i| format!("\"L{i}\""))
                .collect::<Vec<_>>()
                .join(",")
        );
        assert!(Scheme::parse(&many).is_err());
    }

    /// WW-1.11: a scheme in another script is accepted, case-insensitive duplicates are still refused,
    /// and the header carries it as an extended value.
    #[test]
    fn a_scheme_can_be_written_in_any_script() {
        let zh = Scheme::parse(r#"["公开","内部","秘密"]"#).unwrap();
        assert_eq!(zh.level_of("秘密"), Some(2));
        assert!(zh.can_see(Some("内部"), Some("秘密")));
        assert!(!zh.can_see(Some("秘密"), Some("内部")));
        assert!(Scheme::parse(r#"["عام","سري"]"#).is_ok());
        assert!(Scheme::parse(r#"["Öffentlich","VS-NfD"]"#).is_ok());
        assert!(
            Scheme::parse(r#"["ГРИФ","гриф"]"#).is_err(),
            "a duplicate in another case"
        );
        assert!(Scheme::parse(r#"["秘密;"]"#).is_err());
        assert_eq!(header_value("OFFICIAL-SENSITIVE"), "OFFICIAL-SENSITIVE");
        assert_eq!(header_value("秘密"), "UTF-8''%E7%A7%98%E5%AF%86");
        assert!(axum::http::HeaderValue::from_str(&header_value("سري 2")).is_ok());
    }

    #[test]
    fn with_no_scheme_nothing_is_hidden_and_nothing_can_be_set() {
        let off = Scheme::default();
        assert!(!off.is_on());
        assert!(off.can_see(Some("SECRET"), None));
        assert_eq!(off.effective(None), None);
        assert!(off.can_set(None, Some("X"), None).is_err());
    }

    #[test]
    fn a_caller_sees_a_plan_only_at_or_above_its_level() {
        let s = four();
        assert!(
            s.can_see(None, None),
            "an unmarked plan is the lowest level"
        );
        assert!(s.can_see(Some("Public"), None));
        assert!(
            !s.can_see(Some("Internal"), None),
            "no clearance is the lowest level"
        );
        assert!(s.can_see(Some("Internal"), Some("Confidential")));
        assert!(s.can_see(Some("Confidential"), Some("Confidential")));
        assert!(!s.can_see(Some("Restricted"), Some("Confidential")));
        assert!(
            !s.can_see(Some("Internal"), Some("internal")),
            "codes are exact"
        );
        assert!(
            !s.can_see(Some("Internal"), Some("Nonsense")),
            "an unknown clearance is the lowest"
        );
    }

    #[test]
    fn a_mark_the_scheme_no_longer_knows_hides_rather_than_shows() {
        let s = four();
        assert!(!s.can_see(Some("TOP SECRET"), Some("Confidential")));
        assert!(
            s.can_see(Some("TOP SECRET"), Some("Restricted")),
            "only the highest clearance reads it"
        );
        assert_eq!(s.effective(Some("TOP SECRET")), Some("Restricted"));
    }

    #[test]
    fn the_effective_mark_is_always_something_when_a_scheme_is_on() {
        assert_eq!(uk().effective(None), Some("OFFICIAL"));
        assert_eq!(
            uk().effective(Some("OFFICIAL-SENSITIVE")),
            Some("OFFICIAL-SENSITIVE")
        );
    }

    #[test]
    fn you_can_mark_only_up_to_your_clearance_and_only_what_you_can_see() {
        let s = four();
        assert!(
            s.can_set(None, Some("Internal"), Some("Confidential"))
                .is_ok()
        );
        assert!(
            s.can_set(Some("Internal"), None, Some("Internal")).is_ok(),
            "reset to the lowest"
        );
        assert!(
            s.can_set(None, Some("Restricted"), Some("Confidential"))
                .is_err(),
            "above your clearance"
        );
        assert!(
            s.can_set(Some("Restricted"), Some("Public"), Some("Confidential"))
                .is_err(),
            "cannot lower what you cannot see"
        );
        assert!(
            s.can_set(None, Some("Secret"), Some("Restricted")).is_err(),
            "not in the scheme"
        );
        assert!(
            s.can_set(None, Some("internal"), Some("Restricted"))
                .is_err(),
            "codes are exact"
        );
    }
}
