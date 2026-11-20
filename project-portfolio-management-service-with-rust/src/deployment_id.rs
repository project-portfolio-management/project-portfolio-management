//! The deployment's name, `PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_ID` (supplier assurance SA-R13, SA-21).
//!
//! A supplier hosting several customers runs one deployment per customer, each with its own database. The name says
//! which customer a running service belongs to. Production, and any environment not set up for development, refuses to
//! start without a valid one. Development, test and `container-dev` start without it, with a warning.
//!
//! **Not built yet (SA-21):** storing the name in the database on first start and refusing a later start whose name
//! differs. Until then the name is required and checked for form, but it does not yet stop a service from opening
//! another customer's database.

use axa::Environment;

/// The setting that names the deployment.
pub const DEPLOYMENT_ID_ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_ID";

/// The longest name allowed (a DNS label's length).
pub const MAX_LEN: usize = 63;

/// Whether `name` is a deployment name: 1 to 63 lowercase ASCII letters, digits and hyphens, starting with a letter.
#[must_use]
pub fn is_valid(name: &str) -> bool {
    let mut chars = name.chars();
    name.len() <= MAX_LEN
        && chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// The deployment's name from the setting's value `raw`, for `environment`.
///
/// `Ok(Some(name))` for a valid name in any environment. `Ok(None)` when it is unset or blank in an environment that
/// opts out of the production guards ([`crate::app::opts_out_of_guards`]).
///
/// # Errors
///
/// A message naming the setting when it is set but not a valid name (in every environment), or unset or blank where
/// the production guards apply.
pub fn resolve(environment: &Environment, raw: Option<&str>) -> Result<Option<String>, String> {
    match raw.map(str::trim).filter(|v| !v.is_empty()) {
        Some(name) if is_valid(name) => Ok(Some(name.to_string())),
        Some(_) => Err(format!(
            "refusing to start: {DEPLOYMENT_ID_ENV} must be 1 to {MAX_LEN} lowercase letters, digits and hyphens, \
             starting with a letter (for example acme-example)"
        )),
        None if crate::app::opts_out_of_guards(environment) => Ok(None),
        None => Err(format!(
            "refusing to start: {DEPLOYMENT_ID_ENV} is required in this environment; name the deployment, for \
             example acme-example (one name per customer)"
        )),
    }
}

/// [`resolve`] from the process environment, warning when a development environment runs without a name.
///
/// # Errors
///
/// As [`resolve`].
pub fn from_env(environment: &Environment) -> Result<Option<String>, String> {
    let name = resolve(
        environment,
        std::env::var(DEPLOYMENT_ID_ENV).ok().as_deref(),
    )?;
    if name.is_none() {
        tracing::warn!("{DEPLOYMENT_ID_ENV} is not set; production refuses to start without it");
    }
    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_is_lowercase_letters_digits_and_hyphens_starting_with_a_letter() {
        for good in ["acme-example", "a", "nhs-wales-2", &"a".repeat(MAX_LEN)] {
            assert!(is_valid(good), "{good}");
        }
        for bad in [
            "",
            "Acme",
            "1acme",
            "-acme",
            "acme_example",
            "acme example",
            "acmé",
            &"a".repeat(MAX_LEN + 1),
        ] {
            assert!(!is_valid(bad), "{bad}");
        }
    }

    #[test]
    fn production_requires_a_valid_name_and_development_does_not() {
        let production = Environment::Production;
        let staging = Environment::Other("staging".to_string());
        for guarded in [&production, &staging] {
            for unset in [None, Some(""), Some("   ")] {
                let refusal = resolve(guarded, unset).unwrap_err();
                assert!(refusal.contains(DEPLOYMENT_ID_ENV) && refusal.contains("required"));
            }
            assert_eq!(
                resolve(guarded, Some(" acme-example ")),
                Ok(Some("acme-example".to_string()))
            );
        }
        for open in [
            Environment::Development,
            Environment::Test,
            Environment::Other("container-dev".to_string()),
        ] {
            assert_eq!(resolve(&open, None), Ok(None));
            assert_eq!(
                resolve(&open, Some("dev-box")),
                Ok(Some("dev-box".to_string()))
            );
            // A bad name is refused everywhere, so a typo is caught before it reaches production.
            assert!(resolve(&open, Some("Not Valid")).is_err());
        }
        assert!(
            resolve(&production, Some("Not Valid"))
                .unwrap_err()
                .contains("lowercase")
        );
    }
}
