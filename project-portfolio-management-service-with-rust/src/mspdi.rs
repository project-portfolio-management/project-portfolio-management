//! MS Project interchange, **MSPDI XML only** (PJ-10, T-28n): the pure reader.
//! DB-free, unit-tested. Binary `.mpp` is not read (owner decision 2026-10-03).
//!
//! It turns an MSPDI document into plain structs and says **what it could not
//! carry over**: nothing is silently dropped. The two numeric tables that a
//! guess would get wrong are pinned and tested (finding F-11):
//!
//! - dependency type: **FF 0, FS 1, SF 2, SS 3**
//! - constraint type: ASAP 0, ALAP 1, MSO 2, MFO 3, SNET 4, SNLT 5, FNET 6, FNLT 7
//!
//! The XML is parsed with `roxmltree`: no external entities, DTDs refused, a node
//! cap, so a hostile file is a bounded error rather than a resource attack.
//! Dates are zone-free (the time part is dropped, never converted).

use chrono::NaiveDate;

use crate::cpm::Kind;

/// Largest document accepted.
pub const MAX_BYTES: usize = 5 * 1024 * 1024;
/// Most tasks read.
pub const MAX_TASKS: usize = 5_000;
/// Most XML nodes parsed.
pub const MAX_NODES: u32 = 400_000;

/// A constraint as MS Project numbers it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsConstraint {
    /// 0: as soon as possible (no constraint).
    Asap,
    /// 1: as late as possible (not supported here).
    Alap,
    /// 2.
    Mso,
    /// 3.
    Mfo,
    /// 4.
    Snet,
    /// 5.
    Snlt,
    /// 6.
    Fnet,
    /// 7.
    Fnlt,
}

/// MS Project's constraint id.
#[must_use]
pub fn constraint_of(id: i64) -> Option<MsConstraint> {
    Some(match id {
        0 => MsConstraint::Asap,
        1 => MsConstraint::Alap,
        2 => MsConstraint::Mso,
        3 => MsConstraint::Mfo,
        4 => MsConstraint::Snet,
        5 => MsConstraint::Snlt,
        6 => MsConstraint::Fnet,
        7 => MsConstraint::Fnlt,
        _ => return None,
    })
}

/// Our dependency kind for MS Project's link type id: **FF 0, FS 1, SF 2, SS 3**.
#[must_use]
pub fn link_kind_of(id: i64) -> Option<Kind> {
    Some(match id {
        0 => Kind::Ff,
        1 => Kind::Fs,
        2 => Kind::Sf,
        3 => Kind::Ss,
        _ => return None,
    })
}

/// A task as read.
#[derive(Debug, Clone, PartialEq)]
pub struct MsTask {
    /// `UID` (stable within the file).
    pub uid: i64,
    /// `Name`.
    pub name: String,
    /// `OutlineLevel` (1 = top).
    pub outline_level: i64,
    /// `Start` date (time dropped).
    pub start: Option<NaiveDate>,
    /// Business days (`Duration` over minutes per day, rounded up).
    pub duration_days: Option<i64>,
    /// `Summary`: a roll-up row.
    pub summary: bool,
    /// `Milestone`.
    pub milestone: bool,
    /// `PercentComplete`, 0–100.
    pub percent_complete: i64,
    /// `ConstraintType` and `ConstraintDate`.
    pub constraint: Option<(MsConstraint, NaiveDate)>,
    /// `Deadline`.
    pub deadline: Option<NaiveDate>,
    /// `Notes`.
    pub notes: Option<String>,
}

/// A predecessor link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsLink {
    /// The successor's UID (the task carrying the `PredecessorLink`).
    pub successor: i64,
    /// `PredecessorUID`.
    pub predecessor: i64,
    /// Our kind.
    pub kind: Kind,
    /// Business days of lag (negative is a lead).
    pub lag_days: i64,
}

/// What could not be carried over, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unmapped {
    /// What it was (`task`, `link`, `constraint`, `resource`, `baseline` …).
    pub what: &'static str,
    /// The task UID it concerns, if any.
    pub uid: Option<i64>,
    /// Why.
    pub reason: String,
}

/// The base calendar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsCalendar {
    /// Seven `0`/`1`, Monday first.
    pub weekdays: String,
    /// Non-working exception dates.
    pub holidays: Vec<NaiveDate>,
}

/// A custom field the project declares (`ExtendedAttributes`), including enterprise fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsField {
    /// `FieldID`.
    pub field_id: String,
    /// `FieldName`, e.g. `Text1` or `Enterprise Text1`.
    pub name: String,
    /// `Alias`: the label people gave it.
    pub alias: Option<String>,
}

/// One task's value for a custom field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsExtended {
    /// The task's `UID`.
    pub uid: i64,
    /// `FieldID`.
    pub field_id: String,
    /// `Value`.
    pub value: String,
}

/// The most custom-field values one file may carry.
pub const MAX_EXTENDED: usize = 100_000;

/// A parsed document.
#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    /// The custom fields the project declares.
    pub fields: Vec<MsField>,
    /// Each task's custom-field values.
    pub extended: Vec<MsExtended>,
    /// Project `Name` or `Title`.
    pub name: Option<String>,
    /// Minutes in a business day.
    pub minutes_per_day: i64,
    /// Tasks in file order (the project summary task, UID 0, is excluded).
    pub tasks: Vec<MsTask>,
    /// Links between tasks.
    pub links: Vec<MsLink>,
    /// `(uid, name)` of resources.
    pub resources: Vec<(i64, String)>,
    /// `(task uid, resource uid)`.
    pub assignments: Vec<(i64, i64)>,
    /// The project's base calendar, if present.
    pub calendar: Option<MsCalendar>,
    /// Everything not carried over.
    pub unmapped: Vec<Unmapped>,
}

/// Why a document was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Larger than [`MAX_BYTES`].
    TooLarge,
    /// Not well-formed XML, or it carries a DTD.
    Xml(String),
    /// Well-formed but not an MSPDI `Project`.
    NotMspdi,
    /// More than [`MAX_TASKS`] tasks.
    TooManyTasks,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLarge => write!(f, "the file is larger than {MAX_BYTES} bytes"),
            Self::Xml(e) => write!(f, "not valid XML (DTDs are refused): {e}"),
            Self::NotMspdi => write!(
                f,
                "this is not an MS Project XML (MSPDI) file; binary .mpp is not read, export as XML"
            ),
            Self::TooManyTasks => write!(f, "more than {MAX_TASKS} tasks"),
        }
    }
}

fn child<'a>(n: roxmltree::Node<'a, 'a>, name: &str) -> Option<roxmltree::Node<'a, 'a>> {
    n.children()
        .find(|c| c.is_element() && c.tag_name().name() == name)
}

fn text<'a>(n: roxmltree::Node<'a, 'a>, name: &str) -> Option<&'a str> {
    child(n, name)
        .and_then(|c| c.text())
        .map(str::trim)
        .filter(|t| !t.is_empty())
}

fn int(n: roxmltree::Node<'_, '_>, name: &str) -> Option<i64> {
    text(n, name).and_then(|t| t.parse().ok())
}

fn date(n: roxmltree::Node<'_, '_>, name: &str) -> Option<NaiveDate> {
    text(n, name).and_then(|t| NaiveDate::parse_from_str(t.get(..10)?, "%Y-%m-%d").ok())
}

/// Minutes in an ISO 8601 duration as MSPDI writes it (`PT40H0M0S`, `P1DT2H`).
/// Fractional seconds are ignored; months and years are not accepted.
#[must_use]
pub fn duration_minutes(s: &str) -> Option<i64> {
    let s = s.trim().strip_prefix('P')?;
    let (date_part, time_part) = s.split_once('T').map_or((s, ""), |(d, t)| (d, t));
    let mut minutes = 0_i64;
    let mut num = String::new();
    for c in date_part.chars() {
        match c {
            '0'..='9' => num.push(c),
            'D' => {
                minutes += num.parse::<i64>().ok()? * 24 * 60;
                num.clear();
            }
            _ => return None,
        }
    }
    if !num.is_empty() {
        return None;
    }
    for c in time_part.chars() {
        match c {
            '0'..='9' | '.' => num.push(c),
            'H' => {
                minutes += num.parse::<i64>().ok()? * 60;
                num.clear();
            }
            'M' => {
                minutes += num.parse::<i64>().ok()?;
                num.clear();
            }
            'S' => num.clear(),
            _ => return None,
        }
    }
    Some(minutes)
}

/// Whole business days for `minutes`, rounded **up**; and whether it was rounded.
#[must_use]
pub fn days_of(minutes: i64, minutes_per_day: i64) -> (i64, bool) {
    let per = minutes_per_day.max(1);
    let days = (minutes.abs() + per - 1) / per * minutes.signum();
    (days, minutes % per != 0)
}

fn calendar_of(root: roxmltree::Node<'_, '_>) -> Option<MsCalendar> {
    let uid = int(root, "CalendarUID");
    let cals = child(root, "Calendars")?;
    let cal = cals
        .children()
        .filter(|c| c.is_element() && c.tag_name().name() == "Calendar")
        .find(|c| uid.is_some_and(|u| int(*c, "UID") == Some(u)))
        .or_else(|| {
            cals.children().find(|c| {
                c.is_element()
                    && c.tag_name().name() == "Calendar"
                    && text(*c, "IsBaseCalendar") == Some("1")
            })
        })?;
    let mut mask = ['1', '1', '1', '1', '1', '0', '0'];
    let mut holidays = Vec::new();
    if let Some(days) = child(cal, "WeekDays") {
        for d in days
            .children()
            .filter(|c| c.is_element() && c.tag_name().name() == "WeekDay")
        {
            let (Some(ty), working) = (int(d, "DayType"), text(d, "DayWorking") != Some("0"))
            else {
                continue;
            };
            // DayType 1 = Sunday … 7 = Saturday; 0 is an exception entry.
            if (1..=7).contains(&ty) {
                let monday_first = usize::try_from((ty + 5) % 7).unwrap_or(0);
                mask[monday_first] = if working { '1' } else { '0' };
            } else if ty == 0
                && !working
                && let Some(tp) = child(d, "TimePeriod")
                && let (Some(from), Some(to)) = (date(tp, "FromDate"), date(tp, "ToDate"))
            {
                holidays.extend(span(from, to));
            }
        }
    }
    if let Some(ex) = child(cal, "Exceptions") {
        for e in ex
            .children()
            .filter(|c| c.is_element() && c.tag_name().name() == "Exception")
        {
            if text(e, "DayWorking") == Some("0")
                && let Some(tp) = child(e, "TimePeriod")
                && let (Some(from), Some(to)) = (date(tp, "FromDate"), date(tp, "ToDate"))
            {
                holidays.extend(span(from, to));
            }
        }
    }
    holidays.sort_unstable();
    holidays.dedup();
    Some(MsCalendar {
        weekdays: mask.iter().collect(),
        holidays,
    })
}

fn span(from: NaiveDate, to: NaiveDate) -> Vec<NaiveDate> {
    if to < from {
        return Vec::new();
    }
    let n = (to - from).num_days().min(366);
    (0..=n).map(|i| from + chrono::Duration::days(i)).collect()
}

/// Parse an MSPDI document.
///
/// # Errors
///
/// [`ParseError`] for an oversized, malformed (or DTD-carrying), non-MSPDI or
/// too-large document.
#[expect(clippy::too_many_lines)]
pub fn parse(xml: &str) -> Result<Parsed, ParseError> {
    if xml.len() > MAX_BYTES {
        return Err(ParseError::TooLarge);
    }
    let doc = roxmltree::Document::parse_with_options(
        xml,
        roxmltree::ParsingOptions {
            allow_dtd: false,
            nodes_limit: MAX_NODES,
            // No other option is set: a DTD is refused outright, so there is no entity to resolve.
            ..roxmltree::ParsingOptions::default()
        },
    )
    .map_err(|e| ParseError::Xml(e.to_string()))?;
    let root = doc.root_element();
    if root.tag_name().name() != "Project" {
        return Err(ParseError::NotMspdi);
    }
    let minutes_per_day = int(root, "MinutesPerDay").filter(|m| *m > 0).unwrap_or(480);
    let mut unmapped = Vec::new();
    let mut tasks = Vec::new();
    let mut links = Vec::new();
    let mut extended: Vec<MsExtended> = Vec::new();
    let fields: Vec<MsField> = child(root, "ExtendedAttributes")
        .map(|fs| {
            fs.children()
                .filter(|c| c.is_element() && c.tag_name().name() == "ExtendedAttribute")
                .filter_map(|f| {
                    Some(MsField {
                        field_id: text(f, "FieldID")?.to_string(),
                        name: text(f, "FieldName").unwrap_or_default().to_string(),
                        alias: text(f, "Alias").map(str::to_string),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    if let Some(ts) = child(root, "Tasks") {
        for t in ts
            .children()
            .filter(|c| c.is_element() && c.tag_name().name() == "Task")
        {
            let Some(uid) = int(t, "UID") else { continue };
            if uid == 0 {
                continue; // the project summary task
            }
            if tasks.len() >= MAX_TASKS {
                return Err(ParseError::TooManyTasks);
            }
            if text(t, "Active") == Some("0") {
                unmapped.push(Unmapped {
                    what: "task",
                    uid: Some(uid),
                    reason: "inactive task skipped".to_string(),
                });
                continue;
            }
            let name = text(t, "Name").unwrap_or("(unnamed)").to_string();
            let milestone = text(t, "Milestone") == Some("1");
            let summary = text(t, "Summary") == Some("1");
            let duration_days = text(t, "Duration").and_then(duration_minutes).map(|m| {
                let (d, rounded) = days_of(m, minutes_per_day);
                if rounded {
                    unmapped.push(Unmapped {
                        what: "duration",
                        uid: Some(uid),
                        reason: format!(
                            "a part-day duration was rounded up to {d} business day(s)"
                        ),
                    });
                }
                d
            });
            let kind = int(t, "ConstraintType").and_then(constraint_of);
            let constraint = match (kind, date(t, "ConstraintDate")) {
                (Some(MsConstraint::Asap) | None, _) => None,
                (Some(MsConstraint::Alap), _) => {
                    // ALAP carries no date, so it must be reported before the date check.
                    unmapped.push(Unmapped {
                        what: "constraint",
                        uid: Some(uid),
                        reason: "as-late-as-possible is not supported yet; the task is scheduled as soon as possible".to_string(),
                    });
                    None
                }
                (Some(c), Some(d)) => Some((c, d)),
                (Some(_), None) => {
                    unmapped.push(Unmapped {
                        what: "constraint",
                        uid: Some(uid),
                        reason: "a constraint without a date was ignored".to_string(),
                    });
                    None
                }
            };
            if text(t, "Baseline").is_some() || child(t, "Baseline").is_some() {
                unmapped.push(Unmapped {
                    what: "baseline",
                    uid: Some(uid),
                    reason:
                        "task baselines are not imported (schedule baselines are not built yet)"
                            .to_string(),
                });
            }
            for l in t
                .children()
                .filter(|c| c.is_element() && c.tag_name().name() == "PredecessorLink")
            {
                let Some(pred) = int(l, "PredecessorUID") else {
                    continue;
                };
                if text(l, "CrossProject") == Some("1") {
                    unmapped.push(Unmapped {
                        what: "link",
                        uid: Some(uid),
                        reason: "a link to another project file was skipped".to_string(),
                    });
                    continue;
                }
                let ty = int(l, "Type").unwrap_or(1);
                let Some(kind) = link_kind_of(ty) else {
                    unmapped.push(Unmapped {
                        what: "link",
                        uid: Some(uid),
                        reason: format!("unknown link type {ty}; skipped"),
                    });
                    continue;
                };
                // LinkLag is tenths of a minute.
                let lag_minutes = int(l, "LinkLag").unwrap_or(0) / 10;
                let (lag_days, rounded) = days_of(lag_minutes, minutes_per_day);
                if rounded {
                    unmapped.push(Unmapped {
                        what: "link",
                        uid: Some(uid),
                        reason: format!("a part-day lag was rounded to {lag_days} business day(s)"),
                    });
                }
                if int(l, "LagFormat")
                    .is_some_and(|f| matches!(f, 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22))
                {
                    unmapped.push(Unmapped {
                        what: "link",
                        uid: Some(uid),
                        reason: "an elapsed-time lag was treated as working time".to_string(),
                    });
                }
                links.push(MsLink {
                    successor: uid,
                    predecessor: pred,
                    kind,
                    lag_days,
                });
            }
            for e in t
                .children()
                .filter(|c| c.is_element() && c.tag_name().name() == "ExtendedAttribute")
            {
                if let (Some(field_id), Some(value)) = (text(e, "FieldID"), text(e, "Value")) {
                    if extended.len() >= MAX_EXTENDED {
                        return Err(ParseError::TooManyTasks);
                    }
                    extended.push(MsExtended {
                        uid,
                        field_id: field_id.to_string(),
                        value: value.to_string(),
                    });
                }
            }
            tasks.push(MsTask {
                uid,
                name,
                outline_level: int(t, "OutlineLevel").unwrap_or(1).max(1),
                start: date(t, "Start"),
                duration_days: if milestone { Some(0) } else { duration_days },
                summary,
                milestone,
                percent_complete: int(t, "PercentComplete").unwrap_or(0).clamp(0, 100),
                constraint,
                deadline: date(t, "Deadline"),
                notes: text(t, "Notes").map(str::to_string),
            });
        }
    }
    let resources: Vec<(i64, String)> = child(root, "Resources")
        .map(|rs| {
            rs.children()
                .filter(|c| c.is_element() && c.tag_name().name() == "Resource")
                .filter_map(|r| {
                    Some((
                        int(r, "UID").filter(|u| *u != 0)?,
                        text(r, "Name")?.to_string(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    let assignments: Vec<(i64, i64)> = child(root, "Assignments")
        .map(|asg| {
            asg.children()
                .filter(|c| c.is_element() && c.tag_name().name() == "Assignment")
                .filter_map(|a| {
                    Some((
                        int(a, "TaskUID")?,
                        int(a, "ResourceUID").filter(|r| *r >= 0)?,
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(Parsed {
        fields,
        extended,
        name: text(root, "Name")
            .or_else(|| text(root, "Title"))
            .map(str::to_string),
        minutes_per_day,
        tasks,
        links,
        resources,
        assignments,
        calendar: calendar_of(root),
        unmapped,
    })
}

// ---- the writer ----------------------------------------------------------------------------------

/// MS Project's constraint id for ours (the inverse of [`constraint_of`]).
#[must_use]
pub fn constraint_id(c: MsConstraint) -> i64 {
    match c {
        MsConstraint::Asap => 0,
        MsConstraint::Alap => 1,
        MsConstraint::Mso => 2,
        MsConstraint::Mfo => 3,
        MsConstraint::Snet => 4,
        MsConstraint::Snlt => 5,
        MsConstraint::Fnet => 6,
        MsConstraint::Fnlt => 7,
    }
}

/// MS Project's link type id for our kind (the inverse of [`link_kind_of`]).
#[must_use]
pub fn link_type_id(k: Kind) -> i64 {
    match k {
        Kind::Ff => 0,
        Kind::Fs => 1,
        Kind::Sf => 2,
        Kind::Ss => 3,
    }
}

/// Escape text for XML 1.0 element content, dropping the control characters it cannot carry.
fn xml_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\t' | '\n' | '\r' => out.push(c),
            c if (c as u32) < 0x20 || matches!(c as u32, 0xFFFE | 0xFFFF) => {}
            c => out.push(c),
        }
    }
    out
}

/// `PT{hours}H{minutes}M0S` for a number of minutes.
fn iso_duration(minutes: i64) -> String {
    format!("PT{}H{}M0S", minutes / 60, minutes % 60)
}

/// The `CalendarUID`, `Calendars` and opening `Tasks` elements.
fn write_calendar(x: &mut String, calendar: Option<&MsCalendar>) {
    use std::fmt::Write as _;
    x.push_str("  <CalendarUID>1</CalendarUID>\n  <Calendars>\n    <Calendar>\n");
    x.push_str("      <UID>1</UID>\n      <Name>Standard</Name>\n      <IsBaseCalendar>1</IsBaseCalendar>\n      <WeekDays>\n");
    let mask: Vec<char> = calendar
        .map_or("1111100", |c| c.weekdays.as_str())
        .chars()
        .collect();
    // DayType 1 = Sunday … 7 = Saturday; our mask is Monday first.
    for day_type in 1..=7_i64 {
        let monday_first = usize::try_from((day_type + 5) % 7).unwrap_or(0);
        let working = mask.get(monday_first) == Some(&'1');
        let _ = write!(
            x,
            "        <WeekDay><DayType>{day_type}</DayType><DayWorking>{}</DayWorking>",
            i32::from(working)
        );
        if working {
            x.push_str("<WorkingTimes><WorkingTime><FromTime>08:00:00</FromTime><ToTime>12:00:00</ToTime></WorkingTime><WorkingTime><FromTime>13:00:00</FromTime><ToTime>17:00:00</ToTime></WorkingTime></WorkingTimes>");
        }
        x.push_str("</WeekDay>\n");
    }
    x.push_str("      </WeekDays>\n");
    if let Some(cal) = calendar.filter(|c| !c.holidays.is_empty()) {
        x.push_str("      <Exceptions>\n");
        for d in &cal.holidays {
            let _ = writeln!(
                x,
                "        <Exception><EnteredByOccurrences>0</EnteredByOccurrences><TimePeriod><FromDate>{d}T00:00:00</FromDate><ToDate>{d}T23:59:00</ToDate></TimePeriod><DayWorking>0</DayWorking></Exception>"
            );
        }
        x.push_str("      </Exceptions>\n");
    }
    x.push_str("    </Calendar>\n  </Calendars>\n  <Tasks>\n");
}

/// Write an MSPDI document ([`parse`] reads it back). Pure.
///
/// `tasks` must be in outline order with `outline_level` set; UIDs must be unique and non-zero (UID 0 is
/// the project summary row written here). A summary task is written without dates or duration, as MS Project
/// computes those. Working time is 08:00-12:00 and 13:00-17:00 on working days, so `minutes_per_day`
/// should be 480. `notice` goes into an XML comment at the top (what the file does not carry).
#[must_use]
pub fn write(
    name: &str,
    notice: &str,
    minutes_per_day: i64,
    tasks: &[MsTask],
    links: &[MsLink],
    calendar: Option<&MsCalendar>,
) -> String {
    use std::fmt::Write as _;
    let mut x = String::new();
    x.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
    let _ = writeln!(x, "<!-- {} -->", xml_text(notice).replace("--", "- -"));
    x.push_str("<Project xmlns=\"http://schemas.microsoft.com/project\">\n");
    let _ = writeln!(
        x,
        "  <Name>{0}</Name>\n  <Title>{0}</Title>",
        xml_text(name)
    );
    let _ = writeln!(x, "  <MinutesPerDay>{minutes_per_day}</MinutesPerDay>");
    write_calendar(&mut x, calendar);
    let _ = writeln!(
        x,
        "    <Task><UID>0</UID><ID>0</ID><Name>{}</Name><Type>0</Type><OutlineLevel>0</OutlineLevel><Summary>1</Summary></Task>",
        xml_text(name)
    );
    for (index, t) in tasks.iter().enumerate() {
        let _ = write!(
            x,
            "    <Task><UID>{}</UID><ID>{}</ID><Name>{}</Name><Type>0</Type><Active>1</Active><OutlineLevel>{}</OutlineLevel><Summary>{}</Summary><Milestone>{}</Milestone>",
            t.uid,
            index + 1,
            xml_text(&t.name),
            t.outline_level.max(1),
            i32::from(t.summary),
            i32::from(t.milestone)
        );
        if !t.summary {
            if let Some(start) = t.start {
                let _ = write!(x, "<Start>{start}T08:00:00</Start>");
            }
            let days = if t.milestone {
                Some(0)
            } else {
                t.duration_days
            };
            if let Some(days) = days {
                let _ = write!(
                    x,
                    "<Duration>{}</Duration>",
                    iso_duration(days * minutes_per_day)
                );
            }
        }
        let _ = write!(
            x,
            "<PercentComplete>{}</PercentComplete>",
            t.percent_complete.clamp(0, 100)
        );
        if let Some((kind, on)) = t.constraint {
            let _ = write!(
                x,
                "<ConstraintType>{}</ConstraintType><ConstraintDate>{on}T08:00:00</ConstraintDate>",
                constraint_id(kind)
            );
        }
        if let Some(d) = t.deadline {
            let _ = write!(x, "<Deadline>{d}T17:00:00</Deadline>");
        }
        if let Some(n) = t.notes.as_deref().filter(|n| !n.is_empty()) {
            let _ = write!(x, "<Notes>{}</Notes>", xml_text(n));
        }
        for l in links.iter().filter(|l| l.successor == t.uid) {
            let _ = write!(
                x,
                "<PredecessorLink><PredecessorUID>{}</PredecessorUID><Type>{}</Type><CrossProject>0</CrossProject><LinkLag>{}</LinkLag><LagFormat>7</LagFormat></PredecessorLink>",
                l.predecessor,
                link_type_id(l.kind),
                l.lag_days * minutes_per_day * 10
            );
        }
        x.push_str("</Task>\n");
    }
    x.push_str("  </Tasks>\n</Project>\n");
    x
}

#[cfg(test)]
mod tests {
    #[test]
    fn what_is_written_reads_back_the_same() {
        let tasks = vec![
            MsTask {
                uid: 7,
                name: "Phase <1> & \"two\"".into(),
                outline_level: 1,
                start: None,
                duration_days: None,
                summary: true,
                milestone: false,
                percent_complete: 0,
                constraint: None,
                deadline: None,
                notes: None,
            },
            MsTask {
                uid: 9,
                name: "Build".into(),
                outline_level: 2,
                start: Some(d(2026, 11, 2)),
                duration_days: Some(5),
                summary: false,
                milestone: false,
                percent_complete: 100,
                constraint: Some((MsConstraint::Snet, d(2026, 11, 2))),
                deadline: Some(d(2026, 12, 1)),
                notes: Some("line one\nline two".into()),
            },
            MsTask {
                uid: 10,
                name: "Ship".into(),
                outline_level: 2,
                start: None,
                duration_days: Some(0),
                summary: false,
                milestone: true,
                percent_complete: 0,
                constraint: None,
                deadline: None,
                notes: None,
            },
        ];
        let links = vec![
            MsLink {
                successor: 10,
                predecessor: 9,
                kind: Kind::Fs,
                lag_days: 2,
            },
            MsLink {
                successor: 10,
                predecessor: 7,
                kind: Kind::Ss,
                lag_days: -1,
            },
        ];
        let calendar = MsCalendar {
            weekdays: "1111010".into(),
            holidays: vec![d(2026, 12, 25), d(2026, 12, 28)],
        };
        let xml = write("Plan & Co", "a -- b", 480, &tasks, &links, Some(&calendar));
        let back = parse(&xml).expect("our own file parses");
        assert_eq!(back.name.as_deref(), Some("Plan & Co"));
        assert_eq!(back.tasks, tasks);
        assert_eq!(back.links, links);
        assert_eq!(back.calendar, Some(calendar));
        assert!(back.unmapped.is_empty(), "{:?}", back.unmapped);
    }

    #[test]
    fn text_that_xml_cannot_carry_is_escaped_or_dropped() {
        assert_eq!(
            xml_text("a<b>&\"c\"\u{1}\u{b}"),
            "a&lt;b&gt;&amp;&quot;c&quot;"
        );
        let t = MsTask {
            uid: 1,
            name: "x\u{0}y".into(),
            outline_level: 1,
            start: None,
            duration_days: None,
            summary: false,
            milestone: false,
            percent_complete: 0,
            constraint: None,
            deadline: None,
            notes: None,
        };
        assert!(parse(&write("p", "n", 480, &[t], &[], None)).is_ok());
    }

    #[test]
    fn the_id_tables_are_the_inverse_of_the_pinned_ones() {
        for id in 0..=7 {
            assert_eq!(constraint_of(id).map(constraint_id), Some(id));
        }
        for id in 0..=3 {
            assert_eq!(link_kind_of(id).map(link_type_id), Some(id));
        }
    }

    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Project xmlns="http://schemas.microsoft.com/project">
  <Name>Apollo</Name>
  <MinutesPerDay>480</MinutesPerDay>
  <CalendarUID>1</CalendarUID>
  <Calendars>
    <Calendar>
      <UID>1</UID><IsBaseCalendar>1</IsBaseCalendar>
      <WeekDays>
        <WeekDay><DayType>1</DayType><DayWorking>0</DayWorking></WeekDay>
        <WeekDay><DayType>2</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>3</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>4</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>5</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>6</DayType><DayWorking>1</DayWorking></WeekDay>
        <WeekDay><DayType>7</DayType><DayWorking>0</DayWorking></WeekDay>
      </WeekDays>
      <Exceptions>
        <Exception><DayWorking>0</DayWorking>
          <TimePeriod><FromDate>2026-12-24T00:00:00</FromDate><ToDate>2026-12-25T23:59:00</ToDate></TimePeriod></Exception>
      </Exceptions>
    </Calendar>
  </Calendars>
  <Tasks>
    <Task><UID>0</UID><Name>Apollo</Name><Summary>1</Summary></Task>
    <Task><UID>1</UID><Name>Design</Name><OutlineLevel>1</OutlineLevel><Summary>1</Summary>
      <Start>2026-10-05T08:00:00</Start><Duration>PT64H0M0S</Duration></Task>
    <Task><UID>2</UID><Name>Draft</Name><OutlineLevel>2</OutlineLevel><Start>2026-10-05T08:00:00</Start>
      <Duration>PT24H0M0S</Duration><PercentComplete>100</PercentComplete><Notes>Use the template</Notes></Task>
    <Task><UID>3</UID><Name>Review</Name><OutlineLevel>2</OutlineLevel><Duration>PT32H0M0S</Duration>
      <ConstraintType>6</ConstraintType><ConstraintDate>2026-10-12T08:00:00</ConstraintDate>
      <Deadline>2026-10-30T17:00:00</Deadline><PercentComplete>40</PercentComplete>
      <PredecessorLink><PredecessorUID>2</PredecessorUID><Type>1</Type><LinkLag>4800</LinkLag><LagFormat>7</LagFormat></PredecessorLink>
    </Task>
    <Task><UID>4</UID><Name>Sign-off</Name><OutlineLevel>1</OutlineLevel><Milestone>1</Milestone><Duration>PT0H0M0S</Duration>
      <ConstraintType>2</ConstraintType><ConstraintDate>2026-11-02T08:00:00</ConstraintDate>
      <PredecessorLink><PredecessorUID>3</PredecessorUID><Type>3</Type></PredecessorLink>
      <PredecessorLink><PredecessorUID>2</PredecessorUID><Type>0</Type><LinkLag>-4800</LinkLag></PredecessorLink>
      <PredecessorLink><PredecessorUID>9</PredecessorUID><Type>2</Type><CrossProject>1</CrossProject></PredecessorLink>
    </Task>
    <Task><UID>5</UID><Name>Old</Name><Active>0</Active></Task>
    <Task><UID>6</UID><Name>Half day</Name><Duration>PT4H0M0S</Duration><ConstraintType>1</ConstraintType></Task>
  </Tasks>
  <Resources>
    <Resource><UID>0</UID><Name>Unassigned</Name></Resource>
    <Resource><UID>1</UID><Name>Alex</Name></Resource>
  </Resources>
  <Assignments>
    <Assignment><TaskUID>2</TaskUID><ResourceUID>1</ResourceUID></Assignment>
    <Assignment><TaskUID>3</TaskUID><ResourceUID>-65535</ResourceUID></Assignment>
  </Assignments>
</Project>"#;

    #[test]
    fn the_numeric_id_tables_are_pinned() {
        // Finding F-11: a codec that guessed would swap relationship types silently.
        assert_eq!(link_kind_of(0), Some(Kind::Ff));
        assert_eq!(link_kind_of(1), Some(Kind::Fs));
        assert_eq!(link_kind_of(2), Some(Kind::Sf));
        assert_eq!(link_kind_of(3), Some(Kind::Ss));
        assert_eq!(link_kind_of(4), None);
        let c = |i| constraint_of(i);
        assert_eq!(
            [c(0), c(1), c(2), c(3), c(4), c(5), c(6), c(7)],
            [
                Some(MsConstraint::Asap),
                Some(MsConstraint::Alap),
                Some(MsConstraint::Mso),
                Some(MsConstraint::Mfo),
                Some(MsConstraint::Snet),
                Some(MsConstraint::Snlt),
                Some(MsConstraint::Fnet),
                Some(MsConstraint::Fnlt)
            ]
        );
        assert_eq!(
            constraint_of(100),
            None,
            "the hammock id is not accepted yet"
        );
    }

    #[test]
    fn durations_parse_and_round_up_to_whole_working_days() {
        assert_eq!(duration_minutes("PT64H0M0S"), Some(3840));
        assert_eq!(duration_minutes("P1DT2H30M"), Some(1440 + 150));
        assert_eq!(duration_minutes("PT0S"), Some(0));
        assert_eq!(duration_minutes("P1M"), None, "months are not accepted");
        assert_eq!(duration_minutes("nope"), None);
        assert_eq!(days_of(3840, 480), (8, false));
        assert_eq!(
            days_of(240, 480),
            (1, true),
            "a half day rounds up and says so"
        );
        assert_eq!(days_of(-4800, 480), (-10, false), "a lead keeps its sign");
        assert_eq!(days_of(0, 480), (0, false));
    }

    #[test]
    fn a_document_is_read_into_tasks_links_calendar_and_resources() {
        let p = parse(SAMPLE).unwrap();
        assert_eq!(p.name.as_deref(), Some("Apollo"));
        assert_eq!(
            p.tasks.iter().map(|t| t.uid).collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 6],
            "UID 0 and the inactive task are excluded"
        );
        let t = |u: i64| p.tasks.iter().find(|t| t.uid == u).unwrap();
        assert!(t(1).summary && t(2).outline_level == 2);
        assert_eq!(
            (t(2).start, t(2).duration_days, t(2).percent_complete),
            (Some(d(2026, 10, 5)), Some(3), 100)
        );
        assert_eq!(t(2).notes.as_deref(), Some("Use the template"));
        assert_eq!(t(3).constraint, Some((MsConstraint::Fnet, d(2026, 10, 12))));
        assert_eq!(t(3).deadline, Some(d(2026, 10, 30)));
        assert!(t(4).milestone && t(4).duration_days == Some(0));
        assert_eq!(t(4).constraint, Some((MsConstraint::Mso, d(2026, 11, 2))));
        // Links; LinkLag is tenths of a minute (4800 = 480 minutes = one business day).
        let link = |s: i64, pr: i64| {
            p.links
                .iter()
                .find(|l| l.successor == s && l.predecessor == pr)
                .cloned()
        };
        assert_eq!(
            link(3, 2).map(|l| (l.kind, l.lag_days)),
            Some((Kind::Fs, 1)),
            "LinkLag 4800 tenths = 480 min = 1 business day"
        );
        assert_eq!(link(4, 3).map(|l| l.kind), Some(Kind::Ss));
        assert_eq!(
            link(4, 2).map(|l| (l.kind, l.lag_days)),
            Some((Kind::Ff, -1)),
            "a lead"
        );
        assert!(link(4, 9).is_none(), "a cross-project link is skipped");
        // Calendar: MS Project numbers weekdays from Sunday = 1, ours start on Monday.
        let cal = p.calendar.unwrap();
        assert_eq!(
            cal.weekdays, "1111100",
            "DayType 1 = Sunday off, 2-6 = Monday to Friday on, 7 = Saturday off"
        );
        assert_eq!(cal.holidays, vec![d(2026, 12, 24), d(2026, 12, 25)]);
        assert_eq!(
            p.resources,
            vec![(1, "Alex".to_string())],
            "the Unassigned placeholder (UID 0) is not a resource"
        );
        assert_eq!(
            p.assignments,
            vec![(2, 1)],
            "an unassigned marker (negative UID) is dropped"
        );
    }

    #[test]
    fn everything_not_carried_over_is_reported() {
        let p = parse(SAMPLE).unwrap();
        let has = |what: &str, needle: &str| {
            p.unmapped
                .iter()
                .any(|u| u.what == what && u.reason.contains(needle))
        };
        assert!(has("task", "inactive"), "{:?}", p.unmapped);
        assert!(has("link", "another project"));
        assert!(has("duration", "rounded up"), "the half-day task");
        assert!(
            has("constraint", "as-late-as-possible"),
            "ALAP has no date and is still reported"
        );
    }

    #[test]
    fn hostile_or_wrong_files_are_a_bounded_error() {
        assert!(matches!(parse("not xml"), Err(ParseError::Xml(_))));
        assert!(matches!(parse("<Other/>"), Err(ParseError::NotMspdi)));
        // A DTD (and so an entity bomb or external entity) is refused outright.
        let bomb = r#"<?xml version="1.0"?><!DOCTYPE p [<!ENTITY a "aaaa"><!ENTITY b "&a;&a;&a;&a;">]><Project><Name>&b;</Name></Project>"#;
        assert!(matches!(parse(bomb), Err(ParseError::Xml(_))));
        let huge = format!("<Project>{}</Project>", "x".repeat(MAX_BYTES));
        assert!(matches!(parse(&huge), Err(ParseError::TooLarge)));
        let many = "<Task><UID>1</UID></Task>".repeat(MAX_TASKS + 1);
        assert!(matches!(
            parse(&format!("<Project><Tasks>{many}</Tasks></Project>")),
            Err(ParseError::TooManyTasks)
        ));
        assert!(
            parse("<Project/>").unwrap().tasks.is_empty(),
            "an empty project is fine"
        );
    }

    #[test]
    fn a_namespace_prefix_does_not_matter() {
        let p = parse(r#"<ns:Project xmlns:ns="http://schemas.microsoft.com/project"><ns:Tasks><ns:Task><ns:UID>1</ns:UID><ns:Name>X</ns:Name></ns:Task></ns:Tasks></ns:Project>"#).unwrap();
        assert_eq!(p.tasks.len(), 1);
    }
}
