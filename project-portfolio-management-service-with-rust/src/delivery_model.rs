//! The delivery-model typology (EX-2). Pure.
//!
//! A programme is one of three kinds of delivery. The kind decides what the organization is accountable
//! for, and later rules read it (a deployment date of a central platform with local deployment is under
//! change control, EX-5).

/// The three delivery models.
pub const MODELS: [&str; 3] = [
    "central_product",
    "central_platform_local_deployment",
    "local_programme_central_standards",
];

/// Whether the organization itself delivers a programme of this model. A local programme with central
/// standards is delivered by someone else, to the organization's standards, and must not be shown as the
/// organization's own delivery.
#[must_use]
pub fn delivered_by_organization(model: &str) -> bool {
    model != "local_programme_central_standards"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_local_programme_is_not_delivered_by_the_organization() {
        assert!(delivered_by_organization("central_product"));
        assert!(delivered_by_organization("central_platform_local_deployment"));
        assert!(!delivered_by_organization("local_programme_central_standards"));
        assert_eq!(MODELS.len(), 3);
    }
}
