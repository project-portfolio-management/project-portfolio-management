//! Currency codes: the one validator and the ISO 4217 minor-unit table (spec `02-scope` §2.0, WW-1.3).
//!
//! No currency is built in. A code is three ASCII letters; money is an integer count of the currency's
//! minor unit, and the number of decimal places of that unit (the exponent) comes from the table below,
//! never from a fixed `100`.

/// Whether `code` is shaped like an ISO 4217 code: three ASCII upper-case letters. A format check, not a
/// registry lookup (a deployment may hold a code the table does not list, for example a local scrip).
#[must_use]
pub fn is_code(code: &str) -> bool {
    code.len() == 3 && code.bytes().all(|b| b.is_ascii_uppercase())
}

/// A code as a caller typed it: trimmed and upper-cased, or `None` when it is not three ASCII letters.
/// The lenient entry points (a body or query that may be typed by hand) use this; every stored code
/// is [`is_code`].
#[must_use]
pub fn normalize(raw: &str) -> Option<String> {
    let code = raw.trim().to_ascii_uppercase();
    is_code(&code).then_some(code)
}

/// ISO 4217 minor-unit exponents: the active codes, sorted, with the number of decimal places of the
/// minor unit. Funds and metals that have no minor unit (`XAU`, `XXX`, ...) are left out; they use the
/// default of [`exponent`].
pub const MINOR_UNITS: &[(&str, u32)] = &[
    ("AED", 2),
    ("AFN", 2),
    ("ALL", 2),
    ("AMD", 2),
    ("ANG", 2),
    ("AOA", 2),
    ("ARS", 2),
    ("AUD", 2),
    ("AWG", 2),
    ("AZN", 2),
    ("BAM", 2),
    ("BBD", 2),
    ("BDT", 2),
    ("BGN", 2),
    ("BHD", 3),
    ("BIF", 0),
    ("BMD", 2),
    ("BND", 2),
    ("BOB", 2),
    ("BRL", 2),
    ("BSD", 2),
    ("BTN", 2),
    ("BWP", 2),
    ("BYN", 2),
    ("BZD", 2),
    ("CAD", 2),
    ("CDF", 2),
    ("CHF", 2),
    ("CLF", 4),
    ("CLP", 0),
    ("CNY", 2),
    ("COP", 2),
    ("CRC", 2),
    ("CUP", 2),
    ("CVE", 2),
    ("CZK", 2),
    ("DJF", 0),
    ("DKK", 2),
    ("DOP", 2),
    ("DZD", 2),
    ("EGP", 2),
    ("ERN", 2),
    ("ETB", 2),
    ("EUR", 2),
    ("FJD", 2),
    ("FKP", 2),
    ("GBP", 2),
    ("GEL", 2),
    ("GHS", 2),
    ("GIP", 2),
    ("GMD", 2),
    ("GNF", 0),
    ("GTQ", 2),
    ("GYD", 2),
    ("HKD", 2),
    ("HNL", 2),
    ("HTG", 2),
    ("HUF", 2),
    ("IDR", 2),
    ("ILS", 2),
    ("INR", 2),
    ("IQD", 3),
    ("IRR", 2),
    ("ISK", 0),
    ("JMD", 2),
    ("JOD", 3),
    ("JPY", 0),
    ("KES", 2),
    ("KGS", 2),
    ("KHR", 2),
    ("KMF", 0),
    ("KPW", 2),
    ("KRW", 0),
    ("KWD", 3),
    ("KYD", 2),
    ("KZT", 2),
    ("LAK", 2),
    ("LBP", 2),
    ("LKR", 2),
    ("LRD", 2),
    ("LSL", 2),
    ("LYD", 3),
    ("MAD", 2),
    ("MDL", 2),
    ("MGA", 2),
    ("MKD", 2),
    ("MMK", 2),
    ("MNT", 2),
    ("MOP", 2),
    ("MRU", 2),
    ("MUR", 2),
    ("MVR", 2),
    ("MWK", 2),
    ("MXN", 2),
    ("MYR", 2),
    ("MZN", 2),
    ("NAD", 2),
    ("NGN", 2),
    ("NIO", 2),
    ("NOK", 2),
    ("NPR", 2),
    ("NZD", 2),
    ("OMR", 3),
    ("PAB", 2),
    ("PEN", 2),
    ("PGK", 2),
    ("PHP", 2),
    ("PKR", 2),
    ("PLN", 2),
    ("PYG", 0),
    ("QAR", 2),
    ("RON", 2),
    ("RSD", 2),
    ("RUB", 2),
    ("RWF", 0),
    ("SAR", 2),
    ("SBD", 2),
    ("SCR", 2),
    ("SDG", 2),
    ("SEK", 2),
    ("SGD", 2),
    ("SHP", 2),
    ("SLE", 2),
    ("SOS", 2),
    ("SRD", 2),
    ("SSP", 2),
    ("STN", 2),
    ("SVC", 2),
    ("SYP", 2),
    ("SZL", 2),
    ("THB", 2),
    ("TJS", 2),
    ("TMT", 2),
    ("TND", 3),
    ("TOP", 2),
    ("TRY", 2),
    ("TTD", 2),
    ("TWD", 2),
    ("TZS", 2),
    ("UAH", 2),
    ("UGX", 0),
    ("USD", 2),
    ("UYI", 0),
    ("UYU", 2),
    ("UYW", 4),
    ("UZS", 2),
    ("VED", 2),
    ("VES", 2),
    ("VND", 0),
    ("VUV", 0),
    ("WST", 2),
    ("XAF", 0),
    ("XCD", 2),
    ("XCG", 2),
    ("XOF", 0),
    ("XPF", 0),
    ("YER", 2),
    ("ZAR", 2),
    ("ZMW", 2),
    ("ZWG", 2),
];

/// Decimal places of `currency`'s minor unit: from [`MINOR_UNITS`], and 2 for a code the table does not list
/// (the common case; a deployment that holds such a code should say so in its own documentation).
#[must_use]
pub fn exponent(currency: &str) -> u32 {
    MINOR_UNITS
        .binary_search_by(|(code, _)| (*code).cmp(currency))
        .map_or(2, |i| MINOR_UNITS[i].1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_code_is_three_upper_case_ascii_letters() {
        assert!(is_code("JPY"));
        assert!(is_code("KWD"));
        assert!(!is_code("jpy"));
        assert!(!is_code("JP"));
        assert!(!is_code("JPYY"));
        assert!(!is_code("J1Y"));
        assert!(!is_code("ÉUR"));
    }

    #[test]
    fn normalising_trims_and_upper_cases_then_applies_the_same_rule() {
        assert_eq!(normalize(" eur ").as_deref(), Some("EUR"));
        assert_eq!(normalize("jpy").as_deref(), Some("JPY"));
        assert_eq!(normalize("EU"), None);
        assert_eq!(normalize("E1R"), None);
        assert_eq!(normalize(""), None);
    }

    #[test]
    fn the_table_is_sorted_unique_and_well_formed() {
        for pair in MINOR_UNITS.windows(2) {
            assert!(pair[0].0 < pair[1].0, "{} before {}", pair[0].0, pair[1].0);
        }
        for (code, e) in MINOR_UNITS {
            assert!(is_code(code), "{code}");
            assert!(matches!(e, 0 | 2 | 3 | 4), "{code}");
        }
    }

    #[test]
    fn the_exponents_that_are_not_two_are_exactly_the_iso_list() {
        let of = |e: u32| {
            MINOR_UNITS
                .iter()
                .filter(|(_, x)| *x == e)
                .map(|(c, _)| *c)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            of(0),
            [
                "BIF", "CLP", "DJF", "GNF", "ISK", "JPY", "KMF", "KRW", "PYG", "RWF", "UGX", "UYI",
                "VND", "VUV", "XAF", "XOF", "XPF"
            ]
        );
        assert_eq!(of(3), ["BHD", "IQD", "JOD", "KWD", "LYD", "OMR", "TND"]);
        assert_eq!(of(4), ["CLF", "UYW"]);
    }

    #[test]
    fn exponent_looks_the_code_up_and_defaults_to_two() {
        assert_eq!(exponent("JPY"), 0);
        assert_eq!(exponent("KWD"), 3);
        assert_eq!(exponent("CLF"), 4);
        assert_eq!(exponent("UYW"), 4);
        assert_eq!(exponent("EUR"), 2);
        assert_eq!(exponent("GBP"), 2);
        assert_eq!(exponent("ZZZ"), 2);
    }
}
