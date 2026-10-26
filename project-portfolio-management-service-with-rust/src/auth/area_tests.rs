use super::*;

fn claims(attrs: &[(&str, &str)]) -> Claims {
    Claims {
        sub: "s".into(),
        email: "e@example.com".into(),
        name: "n".into(),
        iss: "i".into(),
        aud: "a".into(),
        exp: 2_000_000_000,
        iat: 1_900_000_000,
        nbf: None,
        sid: "x".into(),
        scope: vec![],
        roles: vec![],
        attrs: attrs
            .iter()
            .map(|(k, v)| ((*k).to_string(), vec![(*v).to_string()]))
            .collect(),
    }
}
/// CQ-2: no claims is a pass only with enforcement off; with it on, a token that expired
/// between the guard and the handler is refused, never treated as an administrator.
#[test]
fn no_claims_passes_only_when_enforcement_is_off() {
    let admin = |c: &Claims| has_attr(c, "access", "admin");
    assert!(holds_or_open(false, None, admin));
    assert!(!holds_or_open(true, None, admin));
    let a = claims(&[("access", "admin")]);
    let w = claims(&[("access", "write")]);
    for on in [true, false] {
        assert!(holds_or_open(on, Some(&a), admin));
        assert!(
            !holds_or_open(on, Some(&w), admin),
            "a token is judged by its claims"
        );
    }
}

/// CQ-2: a route check never treats missing claims as a pass by hand; it goes through
/// `holds_or_open`, which knows the enforcement posture.
#[test]
fn no_controller_treats_missing_claims_as_a_pass() {
    let needle = format!("claims().{}(", "is_none_or");
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/controllers");
    let offenders: Vec<String> = std::fs::read_dir(dir)
        .expect("read controllers")
        .map(|e| e.expect("entry").path())
        .filter(|p| std::fs::read_to_string(p).is_ok_and(|text| text.contains(&needle)))
        .map(|p| p.display().to_string())
        .collect();
    assert!(
        offenders.is_empty(),
        "missing claims treated as a pass: {offenders:?}"
    );
}

fn allowed(p: &Policy, c: &Claims, a: Action, area: &str) -> bool {
    let mut r = std::collections::BTreeMap::new();
    r.insert("area".to_string(), vec![area.to_string()]);
    p.evaluate_with_resource(c, a, ENTITY, &r).allowed
}

#[test]
fn finance_needs_admin_or_the_finance_attribute_for_every_action() {
    let p = with_service_rules(Policy::default_policy());
    for a in [Action::Read, Action::Write, Action::Delete] {
        assert!(
            allowed(&p, &claims(&[("access", "admin")]), a, "finance"),
            "{a:?}"
        );
        assert!(
            allowed(&p, &claims(&[("finance", "cost_rates")]), a, "finance"),
            "{a:?}"
        );
        assert!(
            !allowed(&p, &claims(&[("access", "write")]), a, "finance"),
            "a writer is not finance: {a:?}"
        );
        assert!(!allowed(&p, &claims(&[]), a, "finance"), "{a:?}");
    }
}

#[test]
fn break_glass_is_for_the_eligible_and_administrators_only() {
    let open_policy = Policy {
        rules: vec![Rule {
            effect: Effect::Allow,
            actions: vec![ActionPattern::Any],
            when: std::collections::BTreeMap::default(),
            obligations: vec![],
        }],
    };
    for p in [
        with_service_rules(Policy::default_policy()),
        with_service_rules(open_policy),
    ] {
        for a in [Action::Read, Action::Write, Action::Delete] {
            assert!(
                allowed(&p, &claims(&[("breakglass", "eligible")]), a, "break_glass"),
                "{a:?}"
            );
            assert!(
                allowed(&p, &claims(&[("access", "admin")]), a, "break_glass"),
                "{a:?}"
            );
            assert!(
                !allowed(&p, &claims(&[("access", "write")]), a, "break_glass"),
                "{a:?}"
            );
            assert!(
                !allowed(&p, &claims(&[("breakglass", "no")]), a, "break_glass"),
                "{a:?}"
            );
            assert!(
                !allowed(&p, &claims(&[("breakglass", "Eligible")]), a, "break_glass"),
                "case matters: {a:?}"
            );
            assert!(!allowed(&p, &claims(&[]), a, "break_glass"), "{a:?}");
        }
    }
}

#[test]
fn data_subject_requests_are_for_administrators_only() {
    let p = with_service_rules(Policy::default_policy());
    for a in [Action::Read, Action::Write, Action::Delete] {
        assert!(
            allowed(&p, &claims(&[("access", "admin")]), a, "subjects"),
            "{a:?}"
        );
        assert!(
            !allowed(&p, &claims(&[("access", "write")]), a, "subjects"),
            "{a:?}"
        );
        assert!(
            !allowed(&p, &claims(&[("finance", "cost_rates")]), a, "subjects"),
            "{a:?}"
        );
        assert!(!allowed(&p, &claims(&[]), a, "subjects"), "{a:?}");
        assert!(
            allowed(&p, &claims(&[("access", "admin")]), a, "access_review"),
            "{a:?}"
        );
        assert!(
            !allowed(&p, &claims(&[("access", "write")]), a, "access_review"),
            "{a:?}"
        );
    }
}

#[test]
fn announcements_are_readable_by_all_and_writable_by_publishers() {
    let p = with_service_rules(Policy::default_policy());
    assert!(allowed(&p, &claims(&[]), Action::Read, "comms"));
    assert!(!allowed(
        &p,
        &claims(&[("access", "write")]),
        Action::Write,
        "comms"
    ));
    assert!(allowed(
        &p,
        &claims(&[("comms", "publish")]),
        Action::Write,
        "comms"
    ));
    assert!(allowed(
        &p,
        &claims(&[("access", "admin")]),
        Action::Delete,
        "comms"
    ));
    assert!(!allowed(
        &p,
        &claims(&[("access", "write")]),
        Action::Delete,
        "comms"
    ));
}

#[test]
fn acknowledging_and_resolving_an_escalation_are_separate_rights() {
    let p = with_service_rules(Policy::default_policy());
    let writer = claims(&[("access", "write")]);
    let acker = claims(&[("access", "write"), ("escalation", "acknowledge")]);
    let resolver = claims(&[("access", "write"), ("escalation", "resolve")]);
    let admin = claims(&[("access", "admin")]);
    for (who, ack, res) in [
        (&writer, false, false),
        (&acker, true, false),
        (&resolver, false, true),
        (&admin, true, true),
    ] {
        assert_eq!(
            allowed(&p, who, Action::Write, ESCALATION_ACKNOWLEDGEMENT),
            ack
        );
        assert_eq!(allowed(&p, who, Action::Write, ESCALATION_RESOLUTION), res);
    }
    // A permissive deployment policy cannot widen them.
    let open = with_service_rules(Policy {
        rules: vec![Rule {
            effect: Effect::Allow,
            actions: vec![ActionPattern::Any],
            when: std::collections::BTreeMap::default(),
            obligations: vec![],
        }],
    });
    assert!(!allowed(
        &open,
        &writer,
        Action::Write,
        ESCALATION_ACKNOWLEDGEMENT
    ));
    assert!(!allowed(
        &open,
        &writer,
        Action::Write,
        ESCALATION_RESOLUTION
    ));
}

#[test]
fn a_deployments_own_policy_cannot_open_these_areas_and_other_areas_are_untouched() {
    // A permissive operator policy that allows everything to everyone.
    let open = Policy {
        rules: vec![Rule {
            effect: Effect::Allow,
            actions: vec![ActionPattern::Any],
            when: std::collections::BTreeMap::default(),
            obligations: vec![],
        }],
    };
    let p = with_service_rules(open);
    let nobody = claims(&[]);
    assert!(!allowed(&p, &nobody, Action::Read, "finance"));
    assert!(!allowed(&p, &nobody, Action::Write, "comms"));
    assert!(
        allowed(&p, &nobody, Action::Write, "plan"),
        "other areas follow the deployment's policy"
    );
    // With the default policy, ordinary writes and reads are as they were.
    let d = with_service_rules(Policy::default_policy());
    assert!(allowed(
        &d,
        &claims(&[("access", "write")]),
        Action::Write,
        "plan"
    ));
    assert!(allowed(&d, &nobody, Action::Read, "plan"));
    assert!(!allowed(&d, &nobody, Action::Write, "plan"));
}

/// NC-15: a token carrying `scope=readonly` reads and changes nothing, however much else it holds; one without it is
/// judged as before. Nothing in the policy file can lift it.
#[test]
fn a_read_only_token_is_refused_every_write_whatever_else_it_holds() {
    let permissive = with_service_rules(Policy {
        rules: vec![Rule {
            effect: Effect::Allow,
            actions: vec![ActionPattern::Any],
            when: std::collections::BTreeMap::new(),
            obligations: Vec::new(),
        }],
    });
    for holds in [
        &[("access", "write"), ("scope", "readonly")][..],
        &[("access", "admin"), ("scope", "readonly")][..],
        &[("scope", "readonly")][..],
    ] {
        let c = claims(holds);
        assert!(
            permissive.evaluate(&c, Action::Read, ENTITY).allowed,
            "{holds:?} reads"
        );
        for a in [Action::Write, Action::Delete, Action::Destructive] {
            assert!(
                !permissive.evaluate(&c, a, ENTITY).allowed,
                "{holds:?} {a:?}"
            );
        }
        // The service-owned areas too: an administrator who is read-only cannot use an admin-only area to write.
        assert!(!allowed(&permissive, &c, Action::Write, "subjects"));
    }
    let writer = claims(&[("access", "write")]);
    assert!(permissive.evaluate(&writer, Action::Write, ENTITY).allowed);
}

/// BR-12: a capture token reads and posts a status update, a comment or a lesson, and nothing else, whatever else it
/// holds; the same identity without the scope is unchanged.
#[test]
fn a_capture_token_posts_three_things_and_nothing_else() {
    use axum::http::Method;
    let plan = "/api/plans/3f2c6b7e-0000-4000-8000-0000000000ad";
    for ok in [
        (Method::GET, "/api/plans".to_string()),
        (Method::GET, format!("{plan}/status")),
        (Method::POST, format!("{plan}/status-updates")),
        (Method::POST, format!("{plan}/status-updates/")),
        (Method::POST, format!("{plan}/lessons")),
        (Method::POST, "/api/comments".to_string()),
    ] {
        assert!(capture_allows(&ok.0, &ok.1), "{ok:?}");
    }
    for no in [
        (Method::PUT, plan.to_string()),
        (Method::PATCH, plan.to_string()),
        (Method::DELETE, plan.to_string()),
        (Method::POST, "/api/plans".to_string()),
        (Method::POST, format!("{plan}/lessons/x/withdraw")),
        (Method::POST, format!("{plan}/tasks")),
        (Method::POST, format!("{plan}/instantiate")),
        (
            Method::POST,
            "/api/plan-templates/x/instantiate".to_string(),
        ),
        (Method::PUT, "/api/comments/x".to_string()),
        (Method::DELETE, "/api/comments/x".to_string()),
        (Method::POST, "/api/plans//status-updates".to_string()),
    ] {
        assert!(!capture_allows(&no.0, &no.1), "{no:?}");
    }
    let permissive = with_service_rules(Policy {
        rules: vec![Rule {
            effect: Effect::Allow,
            actions: vec![ActionPattern::Any],
            when: std::collections::BTreeMap::new(),
            obligations: Vec::new(),
        }],
    });
    let admin_capture = claims(&[("access", "admin"), ("scope", "capture")]);
    let post = |c: &Claims, p: &str| decide(c, &Method::POST, p, &permissive).is_ok();
    assert!(post(&admin_capture, &format!("{plan}/status-updates")));
    assert!(!post(&admin_capture, &format!("{plan}/tasks")));
    let admin = claims(&[("access", "admin")]);
    assert!(post(&admin, &format!("{plan}/tasks")));
}
