//! A director pack as lines of plain text, for the PDF (DP-11). Pure; built **only from the stored
//! JSON**, so the PDF can never show a figure the JSON does not. Fixed words are English: the PDF
//! writer (`crate::pdf`) is Latin-1 only, so it cannot carry Welsh, Chinese or Hindi text; the HTML view in
//! the interface is translated, and a reader of another language prints that. Free text is shown as
//! entered (anything outside Latin-1 becomes `?` in the PDF, as everywhere else).

use serde_json::Value;

fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(Value::as_str).unwrap_or("")
}
fn arr<'a>(v: &'a Value, k: &str) -> &'a [Value] {
    v.get(k)
        .and_then(Value::as_array)
        .map_or(&[], Vec::as_slice)
}
fn num(v: &Value, k: &str) -> String {
    match v.get(k) {
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::String(t)) => t.clone(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => "-".to_string(),
    }
}

/// Decimal places a currency's minor unit has.
#[must_use]
pub fn exponent(currency: &str) -> u32 {
    match currency {
        "JPY" | "KRW" | "VND" | "CLP" | "ISK" | "UGX" | "XAF" | "XOF" | "XPF" | "PYG" | "RWF" => 0,
        "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
        _ => 2,
    }
}

/// `1234567` minor units of `GBP` as `GBP 12,345.67`. Never mixes currencies: the code is always shown.
#[must_use]
pub fn money(minor: i64, currency: &str) -> String {
    let e = exponent(currency);
    let neg = minor < 0;
    let abs = minor.unsigned_abs();
    let scale = 10_u64.pow(e);
    let (whole, frac) = (abs / scale, abs % scale);
    let digits = whole.to_string();
    let mut grouped = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    let body = if e == 0 {
        grouped
    } else {
        format!("{grouped}.{frac:0width$}", width = e as usize)
    };
    format!("{}{currency} {body}", if neg { "-" } else { "" })
}

fn money_of(v: &Value, k: &str, currency: &str) -> String {
    v.get(k)
        .and_then(Value::as_i64)
        .map_or_else(|| "-".to_string(), |m| money(m, currency))
}

fn fte(v: &Value, k: &str) -> String {
    v.get(k)
        .and_then(Value::as_f64)
        .map_or_else(|| "-".to_string(), |f| format!("{f:.3}"))
}

/// The lines of the pack. `meta` is the pack's row (period, audience, version, status, dates).
#[must_use]
#[allow(clippy::too_many_lines, clippy::many_single_char_names)]
pub fn lines(meta: &Value, body: &Value) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let h = |o: &mut Vec<String>, t: &str| {
        o.push(String::new());
        o.push(t.to_uppercase());
    };
    o.push(format!(
        "Director pack {} for {}",
        s(meta, "period"),
        s(meta, "audience")
    ));
    o.push(format!(
        "Version {} - {} - cut-off {}",
        num(meta, "version"),
        s(meta, "status").to_uppercase(),
        s(meta, "cut_off")
    ));
    if s(meta, "status") == "issued" {
        o.push(format!("Issued {}", s(meta, "issued_at")));
    } else {
        o.push(
            "DRAFT - not issued; it can still be discarded or issued, and is not yet frozen."
                .to_string(),
        );
    }
    if let Some(n) = meta
        .get("change_note")
        .and_then(Value::as_str)
        .filter(|n| !n.is_empty())
    {
        o.push(format!("Change note: {n}"));
    }
    if let Some(p) = body["cover"]["previous_issued_pack"].as_object() {
        o.push(format!(
            "Compared with the previous issued pack: {} version {}",
            p.get("period").and_then(Value::as_str).unwrap_or(""),
            p.get("version")
                .map_or_else(|| "-".to_string(), ToString::to_string)
        ));
    }
    o.push(s(body, "basis").to_string());

    let dq = &body["data_quality"];
    h(&mut o, "Data quality");
    o.push(format!(
        "Plans included: {}. Plans left out (private or marked): {}.",
        num(dq, "plans_included"),
        num(dq, "plans_left_out_private_or_marked")
    ));
    o.push(format!(
        "Sections included: {}.",
        arr(dq, "sections_included")
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(", ")
    ));
    let missing: Vec<&str> = arr(dq, "sections_not_yet_built")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    o.push(if missing.is_empty() {
        "Every section of the pack is present.".to_string()
    } else {
        format!("Sections not yet built: {}.", missing.join(", "))
    });

    h(&mut o, "What changed");
    for c in arr(body, "what_changed") {
        if let Some(line) = c.as_str() {
            o.push(line.to_string());
        }
    }

    h(&mut o, "Portfolio map");
    let map = &body["portfolio_map"];
    o.push(format!("Programmes: {}.", num(map, "programmes_total")));
    for g in arr(map, "groups") {
        o.push(format!("Strategic aim: {}", s(g, "aim")));
        for p in arr(g, "programmes") {
            let c = &p["confidence"];
            o.push(format!(
                "  {} | status {} | stage {} | phase {} | confidence {}{} | trend {}",
                s(p, "name"),
                s(p, "programme_status"),
                p["stage"].as_str().unwrap_or("-"),
                p["phase"].as_str().unwrap_or("-"),
                s(c, "value"),
                if c["marked_self"] == true {
                    " (self-assessed)"
                } else {
                    ""
                },
                s(p, "trend"),
            ));
        }
    }

    h(&mut o, "Exceptions");
    let ex = &body["exceptions"];
    o.push(format!("Window: {} hours.", num(ex, "window_hours")));
    for (label, key) in [
        ("Open", "open"),
        ("Breached", "breached"),
        ("Closed in the period", "closed_in_period"),
    ] {
        o.push(format!("{label}: {}", arr(ex, key).len()));
        for e in arr(ex, key) {
            o.push(format!(
                "  {} | {} | decision needed: {} | owner role: {} | decide by {}",
                s(e, "plan"),
                s(e, "worst_value"),
                s(e, "decision_needed"),
                s(e, "owner_role"),
                s(e, "decide_by")
            ));
        }
    }

    h(&mut o, "Gate decisions");
    let g = &body["gate_decisions"];
    for (label, key) in [
        ("Passed", "passed"),
        ("Failed", "failed"),
        ("Deferred", "deferred"),
    ] {
        o.push(format!("{label}: {}", arr(g, key).len()));
        for l in arr(g, key) {
            o.push(format!(
                "  {} | {} | decided {} | planned {} | {}",
                s(l, "plan"),
                s(l, "gate"),
                l["decided_on"].as_str().unwrap_or("-"),
                l["planned_on"].as_str().unwrap_or("-"),
                l["reason"].as_str().unwrap_or("")
            ));
        }
    }
    for (label, key) in [
        ("Paused programmes", "paused"),
        ("Stopped programmes", "stopped"),
    ] {
        o.push(format!("{label}: {}", arr(g, key).len()));
        for p in arr(g, key) {
            o.push(format!(
                "  {} | since {} | {} | allocations flagged {}",
                s(p, "plan"),
                p["since"].as_str().unwrap_or("-"),
                p["reason"].as_str().unwrap_or(""),
                num(&p["still_held"], "allocations_flagged")
            ));
        }
    }

    h(&mut o, "Funding");
    let f = &body["funding"];
    o.push(format!(
        "Financial year {} ({} to {}). Capital (capex) and revenue (opex) are not summed.",
        s(&f["financial_year"], "label"),
        s(&f["financial_year"], "start"),
        s(&f["financial_year"], "end")
    ));
    for p in arr(f, "programmes") {
        o.push(format!("Programme {}", s(p, "name")));
        for r in arr(p, "by_funding_source") {
            let c = s(r, "currency");
            o.push(format!(
                "  {} | {} | budget {} | spend {} | variance {}",
                s(r, "funding_source"),
                s(r, "class"),
                money_of(r, "budget_minor", c),
                money_of(r, "spend_minor", c),
                money_of(r, "variance_minor", c)
            ));
        }
        for fr in arr(&p["whole_life_forecast"], "rows") {
            let c = s(fr, "currency");
            o.push(format!(
                "  forecast ({c}): actual {} | estimate at completion {}",
                money_of(fr, "actual_minor", c),
                money_of(fr, "estimate_at_completion_minor", c)
            ));
        }
    }
    let dqf = &f["data_quality"];
    o.push(format!(
        "Lines with no funding source: {}. Lines with no dates: {}.",
        num(dqf, "lines_with_no_funding_source"),
        num(dqf, "lines_with_no_dates")
    ));

    h(&mut o, "Buffer fever chart");
    let bf = &body["buffer_fever"];
    for p in arr(bf, "programmes") {
        if p["point"].is_null() {
            o.push(format!(
                "{}: no fever-chart point - {}",
                s(p, "plan"),
                s(&p["no_fever_chart_point"], "reason")
            ));
        } else {
            let pt = &p["point"];
            o.push(format!(
                "{}: chain completed {}% | buffer consumed {}% | zone {} | slip {} days of {}",
                s(p, "plan"),
                num(pt, "chain_completed_pct"),
                num(pt, "buffer_consumed_pct"),
                s(pt, "zone"),
                num(pt, "slip_days"),
                num(pt, "buffer_days")
            ));
        }
    }

    h(&mut o, "Earned value");
    let ev = &body["earned_value"];
    for p in arr(ev, "programmes") {
        if p["state"] == "no_baseline" {
            o.push(format!("{}: no baseline", s(p, "name")));
        }
        for c in arr(p, "currencies") {
            let cur = s(c, "currency");
            if c["state"] == "included" {
                o.push(format!(
                    "{} ({cur}): BAC {} | EV {} | AC {} | SPI {} | CPI {}",
                    s(p, "name"),
                    money_of(&c["totals"], "bac_minor", cur),
                    money_of(&c["totals"], "ev_minor", cur),
                    money_of(&c["totals"], "ac_minor", cur),
                    num(&c["spi"], "bp"),
                    num(&c["cpi"], "bp")
                ));
            } else {
                o.push(format!(
                    "{} ({cur}): below EVM threshold (BAC {})",
                    s(p, "name"),
                    money_of(c, "bac_minor", cur)
                ));
            }
        }
    }

    h(&mut o, "Benefits");
    for r in arr(&body["benefits"], "totals") {
        let c = s(r, "currency");
        o.push(format!(
            "Total {} {} {} | planned {} | claimed {} | validated {} | unvalidated claim {}",
            c,
            s(r, "recurrence"),
            s(r, "classification"),
            money_of(r, "planned_minor", c),
            money_of(r, "claimed_minor", c),
            money_of(r, "validated_minor", c),
            money_of(r, "unvalidated_claim_minor", c)
        ));
    }

    h(&mut o, "Capacity");
    for p in arr(&body["capacity_heatmap"], "pools") {
        o.push(format!("Pool {}", s(p, "name")));
        for c in arr(p, "cells") {
            o.push(format!(
                "  {} | {} ({}) | demand {} | supply {} | gap {}",
                s(c, "month"),
                s(c, "word"),
                s(c, "shape"),
                fte(c, "demand_fte"),
                fte(c, "supply_fte"),
                fte(c, "gap_fte")
            ));
        }
    }

    h(&mut o, "Issues");
    let i = &body["issues"];
    o.push(format!("Open issues: {}.", num(i, "open")));
    h(&mut o, "Change requests");
    let c = &body["change_requests"];
    o.push(format!(
        "Awaiting a decision: {}.",
        num(c, "awaiting_decision")
    ));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn money_uses_the_currencys_own_decimals_and_always_names_it() {
        assert_eq!(money(1_234_567, "GBP"), "GBP 12,345.67");
        assert_eq!(money(5, "GBP"), "GBP 0.05");
        assert_eq!(money(-250, "EUR"), "-EUR 2.50");
        assert_eq!(money(1_234_567, "JPY"), "JPY 1,234,567");
        assert_eq!(money(1_234_567, "KWD"), "KWD 1,234.567");
        assert_eq!(money(0, "USD"), "USD 0.00");
        assert!(money(i64::MIN, "GBP").starts_with("-GBP 92,233,720,368,547,758."));
    }

    #[test]
    fn every_figure_in_the_json_is_in_the_text() {
        let body = json!({
            "cover": { "previous_issued_pack": { "period": "2026-09", "version": 2 } },
            "data_quality": { "plans_included": 17, "plans_left_out_private_or_marked": 3, "sections_included": ["issues"], "sections_not_yet_built": [] },
            "what_changed": ["Open issues: 9, was 4"],
            "funding": { "financial_year": { "label": "2026/27", "start": "2026-04-01", "end": "2027-03-31" },
                "programmes": [{ "name": "Care", "by_funding_source": [{ "funding_source": "Grant", "class": "capex", "currency": "GBP", "budget_minor": 100_000, "spend_minor": 25_000, "variance_minor": 75_000 }], "whole_life_forecast": { "rows": [] } }],
                "data_quality": { "lines_with_no_funding_source": 2, "lines_with_no_dates": 1 } },
            "issues": { "open": 9 }, "change_requests": { "awaiting_decision": 5 },
        });
        let meta = json!({ "period": "2026-10", "audience": "board", "version": 3, "status": "issued", "cut_off": "2026-10-31T00:00:00Z", "issued_at": "2026-11-01T09:00:00Z" });
        let text = lines(&meta, &body).join("\n");
        for needle in [
            "17",
            "left out (private or marked): 3",
            "Open issues: 9, was 4",
            "GBP 1,000.00",
            "GBP 250.00",
            "GBP 750.00",
            "2026/27",
            "Open issues: 9",
            "Awaiting a decision: 5",
            "version 2",
            "Version 3",
            "ISSUED",
        ] {
            assert!(text.contains(needle), "missing {needle:?} in:\n{text}");
        }
    }

    #[test]
    fn a_draft_says_so_and_a_pack_with_every_section_says_that() {
        let meta = json!({ "period": "2026-10", "audience": "board", "version": 1, "status": "draft", "cut_off": "x" });
        let body =
            json!({ "data_quality": { "sections_included": [], "sections_not_yet_built": [] } });
        let t = lines(&meta, &body).join("\n");
        assert!(t.contains("DRAFT") && t.contains("Every section of the pack is present."));
    }

    #[test]
    fn an_empty_body_does_not_panic_and_free_text_cannot_inject_pdf_syntax() {
        let meta =
            json!({ "period": "2026-10", "audience": "a)b", "version": 1, "status": "issued" });
        let l = lines(&meta, &json!({}));
        let pdf = crate::pdf::render("Pack", &l);
        assert!(pdf.starts_with(b"%PDF"));
        assert!(
            crate::pdf::text_of(&pdf).contains("a\\)b"),
            "the bracket is escaped, so it cannot close the string"
        );
    }
}
