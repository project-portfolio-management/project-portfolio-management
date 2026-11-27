// German (de-001 and de-de) for the page and component catalogue (WW-1.1). Typed as a complete record of
// the English keys, so a missing string is a compile error.
//
// STATUS: MACHINE DRAFT. Written by Claude (the AI assistant) on 2026-10-31 and NOT reviewed by a
// speaker or a professional translator. The review is recorded in
// `governance/translation-reviews/de-001.md`; update the status there, not here.

import type { PageKey } from "./i18n-pages";

export const PAGES_DE: Record<PageKey, string> = {
  "c.CalendarMonth.next_month": "Nächster Monat",
  "p.calendar.day": "Tag",
  "p.calendar.agenda": "Agenda",
  "p.calendar.status": "Status",
  "p.calendar.status_done": "erledigt",
  "p.calendar.status_open": "offen",
  "p.calendar.item": "Arbeitselement",
  "p.calendar.showing": "Anzeige: {view}, {start} bis {end}",
  "p.calendar.more": "+{n} weitere",
  "p.calendar.milestone": "Meilenstein",
  "p.calendar.due": "Fällig",
  "p.plans.pid.board.assignee": "Zuständig",
  "p.plans.pid.board.moving": "{title} wird nach {column} verschoben…",
  "p.plans.pid.board.move_failed":
    "{title} konnte nicht nach {column} verschoben werden; die Karte ist wieder an ihrem alten Platz.",
  "p.plans.pid.board.empty_column": "Keine Aufgaben",
  "p.plans.pid.board.search": "Board durchsuchen",
  "p.plans.pid.board.task": "Aufgabe",
  "p.plans.pid.board.status": "Status",
  "c.GanttView.holiday": "Feiertag",
  "c.GanttView.shaded":
    "Schattierte Spalten sind freie Tage: Wochenenden und Feiertage im Arbeitskalender dieses Plans.",
  "c.GanttView.kind": "Art",
  "c.GanttView.stage": "Phase",
  "c.GanttView.task": "Aufgabe",
  "c.GanttView.start": "Beginn",
  "c.GanttView.end": "Ende",
  "c.GanttView.predecessors": "Hängt ab von",
  "c.GanttView.depends_on": "Hängt ab von: {names}",
  "c.GanttView.critical": "{name} (kritischer Pfad)",
  "c.GanttView.zoom_in": "Feinere Zeitskala",
  "c.GanttView.zoom_out": "Gröbere Zeitskala",
  "c.GanttView.zoomed": "Zeitskala: {unit}",
  "c.GanttView.unit_day": "Tag",
  "c.GanttView.unit_week": "Woche",
  "c.GanttView.unit_month": "Monat",
  "c.GanttView.export": "Zeitplan exportieren",
  "c.GanttView.csv": "CSV",
  "c.GanttView.excel": "Excel",
  "c.GanttView.exported": "{n} Aufgaben nach {file} exportiert",
  "c.CalendarMonth.week": "Woche",
  "c.CalendarMonth.four_weeks": "4 Wochen",
  "c.CalendarMonth.month": "Monat",
  "c.CalendarMonth.today": "Heute",
  "c.CalendarMonth.previous": "Vorherige(r) {view}",
  "c.CalendarMonth.next": "Nächste(r) {view}",
  "c.CalendarMonth.working_calendar": "Arbeitskalender",
  "c.CalendarMonth.previous_month": "Vorheriger Monat",
  "c.CapacityMeter.days_over_capacity": "Tage über Kapazität",
  "c.CostRiskView.adds": "{risk}: erhöht um {money} — {eventHitText}",
  "c.CostRiskView.already_spent": "Bereits ausgegeben",
  "c.CostRiskView.assumptions": "Annahmen",
  "c.CostRiskView.budget": "Budget",
  "c.CostRiskView.budget_2": "Budget",
  "c.CostRiskView.budget_sum_of_planned_amounts":
    "Budget (Summe der geplanten Beträge)",
  "c.CostRiskView.by_plan": "Nach Plan",
  "c.CostRiskView.cost": "Kosten",
  "c.CostRiskView.cost_at_each_percentile": "Kosten in {currency} je Perzentil",
  "c.CostRiskView.cost_from": "Kosten aus",
  "c.CostRiskView.each_bar_is_a_cost":
    "Jeder Balken ist ein Kostenbereich; je höher, desto mehr Simulationen endeten dort. Durchgezogene Linien sind P50, P80 und P90 (beschriftet); die gestrichelte Linie ist das Budget.",
  "c.CostRiskView.each_plan_s_budget_and":
    "Budget und mittlere simulierte Kosten je Plan in {currency}",
  "c.CostRiskView.estimate": "Schätzung",
  "c.CostRiskView.independence": "Unabhängigkeit.",
  "c.CostRiskView.left_out": "Nicht berücksichtigt:",
  "c.CostRiskView.line": "Position",
  "c.CostRiskView.low_planned_high": "Niedrig / geplant / hoch",
  "c.CostRiskView.mean_simulated_cost": "Mittlere simulierte Kosten",
  "c.CostRiskView.no_register_risk_was_used":
    "Kein Risiko aus dem Register wurde als Kostenereignis verwendet.",
  "c.CostRiskView.outcome": "Ergebnis",
  "c.CostRiskView.per_line_estimate_and_sensitivity":
    "Schätzung und Sensitivität je Position in {currency}",
  "c.CostRiskView.plan": "Plan",
  "c.CostRiskView.risks_that_added_cost":
    "Risiken, die Kosten verursacht haben",
  "c.CostRiskView.seed_the_same_seed_gives":
    "Startwert {seed}: Derselbe Startwert liefert dasselbe Ergebnis. Bis zu {max_iterations} Iterationen.",
  "c.CostRiskView.sensitivity": "Sensitivität",
  "c.CostRiskView.simulations": "Simulationen",
  "c.CostRiskView.spent": "Ausgegeben",
  "c.CostRiskView.the_same_figures_as_a": "Dieselben Zahlen als Tabelle",
  "c.CostRiskView.to": "bis",
  "c.CostRiskView.to_of": "{money} bis {money2}: {count} von {iterations}",
  "c.CostRiskView.what_drives_it": "Was sie treibt",
  "c.CpmGantt.cannot_meet_a_constraint_or":
    "kann eine Einschränkung oder Frist nicht einhalten (die Linie markiert die Grenze)",
  "c.CpmGantt.critical_any_slip_moves_the":
    "kritisch: jede Verzögerung verschiebt das Ende",
  "c.CpmGantt.has_float_the_hatched_tail": "hat Puffer (das schraffierte Ende)",
  "c.CpmGantt.legend": "Legende",
  "c.CpmGantt.near_critical": "beinahe kritisch",
  "c.CpmGantt.no_scheduled_tasks_to_draw":
    "Noch keine terminierten Aufgaben zum Zeichnen. Geben Sie einer Aufgabe eine Dauer (und ein Startdatum oder einen Vorgänger).",
  "c.CpmGantt.the_outline_under_a_bar":
    "der Umriss unter einem Balken zeigt die Lage im Basisplan",
  "c.CpmGantt.to_float_is_counted_in":
    "{from} bis {to}. Der Puffer wird in Arbeitstagen des Plans gezählt; die Balken werden in Kalendertagen gezeichnet.",
  "c.CpmTable.calendar": "Kalender",
  "c.CpmTable.constraint": "Einschränkung",
  "c.CpmTable.deadline": "Frist",
  "c.CpmTable.finishes": "Endet",
  "c.CpmTable.free_float": "Freier Puffer",
  "c.CpmTable.overrides_its_predecessors":
    "(hat Vorrang vor seinen Vorgängern)",
  "c.CpmTable.plan": "Plan",
  "c.CpmTable.plan_assignee": "Plan + Zuständige(r)",
  "c.CpmTable.starts": "Beginnt",
  "c.CpmTable.state": "Zustand",
  "c.CpmTable.task": "Aufgabe",
  "c.CpmTable.task_schedule_with_float_in":
    "Aufgabenzeitplan mit Puffer, in Arbeitstagen des Plans",
  "c.CpmTable.total_float": "Gesamtpuffer",
  "c.CpmTable.untitled": "(ohne Titel)",
  "c.CumulativeFlow.cumulative_flow": "Kumulativer Fluss",
  "c.CumulativeFlow.day": "Tag",
  "c.CumulativeFlow.every_value_in_the_chart":
    "Alle Werte des Diagramms. {note}",
  "c.CumulativeFlow.no_samples_in_this_window":
    "Keine Messpunkte in diesem Zeitfenster.",
  "c.CumulativeFlow.the_vertical_gap_above_done":
    "der senkrechte Abstand über „Erledigt“ ist die laufende Arbeit; der waagerechte Abstand dazu entspricht ungefähr der Durchlaufzeit",
  "c.CumulativeFlow.total": "Gesamt",
  "c.CumulativeFlow.total_in_progress_done":
    "{total} gesamt · {work_in_progress} in Arbeit · {done} erledigt",
  "c.EvBars.the_double_line_is_the":
    "Die Doppellinie ist das Budget bei Fertigstellung: {money}.",
  "c.EvBars.unknown": "unbekannt",
  "c.KpiCard.add_a_reading": "Messwert hinzufügen",
  "c.KpiCard.cadence": "Rhythmus",
  "c.KpiCard.daily": "Täglich",
  "c.KpiCard.delete": "Löschen",
  "c.KpiCard.edit_or_delete": "Bearbeiten oder löschen",
  "c.KpiCard.history": "Verlauf ({readings})",
  "c.KpiCard.loading": "Wird geladen…",
  "c.KpiCard.monthly": "Monatlich",
  "c.KpiCard.name": "Name",
  "c.KpiCard.no_reading_yet": "Noch kein Messwert.",
  "c.KpiCard.no_readings": "Keine Messwerte.",
  "c.KpiCard.none": "Keiner",
  "c.KpiCard.note": "Notiz",
  "c.KpiCard.observed": "Beobachtet",
  "c.KpiCard.observed_on": "Beobachtet am",
  "c.KpiCard.on": "{valueText} am {observed_on}",
  "c.KpiCard.quarterly": "Vierteljährlich",
  "c.KpiCard.read": "· Messung {cadence}",
  "c.KpiCard.record": "Erfassen",
  "c.KpiCard.red_line": "Rote Linie",
  "c.KpiCard.save": "Speichern",
  "c.KpiCard.target": "Zielwert",
  "c.KpiCard.unit": "Einheit",
  "c.KpiCard.value": "Wert",
  "c.KpiCard.weekly": "Wöchentlich",
  "c.KpiCard.which_way_is_good_cannot":
    "Die günstige Richtung ({v}) kann nicht geändert werden: Sonst würden alle bisherigen Messwerte neu eingefärbt.",
  "c.LilyGanttTable.critical_path": "kritischer Pfad",
  "c.LilyGanttTable.day": "Tag",
  "c.LilyGanttTable.month": "Monat",
  "c.LilyGanttTable.no_dated_tasks_to_show":
    "Keine terminierten Aufgaben anzuzeigen.",
  "c.LilyGanttTable.no_usable_dates": "Keine verwendbaren Daten:",
  "c.LilyGanttTable.task": "Aufgabe",
  "c.LilyGanttTable.that_range_is_too_long":
    "Dieser Zeitraum ist zu lang für eine Spalte je {unit}; es wird eine je {unit2} angezeigt.",
  "c.LilyGanttTable.time_scale": "Zeitskala",
  "c.LilyGanttTable.week": "Woche",
  "c.LilyKanbanTable.move_to": "{title} verschieben nach",
  "c.LilyKanbanTable.no_tasks_yet": "Noch keine Aufgaben.",
  "c.LilyKanbanTable.unknown_status": "Unbekannter Status:",
  "c.LineRange.high": "Hoch",
  "c.LineRange.low": "Niedrig",
  "c.LineRange.planned_is_the_likely_cost":
    "Geplant sind {money}, die wahrscheinlichen Kosten. Lassen Sie ein Feld leer, um es zu löschen.",
  "c.LineRange.save_range": "Bereich speichern",
  "c.MilestoneCard.by": "Durch",
  "c.MilestoneCard.by_hand_only": "Nur manuell",
  "c.MilestoneCard.complete": "Abschließen",
  "c.MilestoneCard.completes": "Wird abgeschlossen",
  "c.MilestoneCard.done": "Erledigt",
  "c.MilestoneCard.due": "Fällig {due} · {kind}",
  "c.MilestoneCard.every_move_is_kept_in":
    "Jede Verschiebung bleibt im Verlauf erhalten: altes Datum, neues Datum, Grund und wer.",
  "c.MilestoneCard.forecast": "· Prognose {early_finish}",
  "c.MilestoneCard.from": "Von",
  "c.MilestoneCard.loading": "Wird geladen…",
  "c.MilestoneCard.mark_done": "Als erledigt markieren",
  "c.MilestoneCard.move": "Verschieben",
  "c.MilestoneCard.never_rescheduled": "Nie verschoben.",
  "c.MilestoneCard.new_due_date": "Neues Fälligkeitsdatum",
  "c.MilestoneCard.no_reason_given": "kein Grund angegeben",
  "c.MilestoneCard.open": "Offen",
  "c.MilestoneCard.pick_the_tasks_that_must":
    "Wählen Sie die Aufgaben, die erledigt sein müssen",
  "c.MilestoneCard.reason_required": "Grund (erforderlich)",
  "c.MilestoneCard.reopen": "Wieder öffnen",
  "c.MilestoneCard.reschedule": "Verschieben",
  "c.MilestoneCard.save": "Speichern",
  "c.MilestoneCard.slip_history": "Verschiebungsverlauf ({reschedules})",
  "c.MilestoneCard.tasks_it_waits_for": "Aufgaben, auf die er wartet",
  "c.MilestoneCard.tasks_this_milestone_waits_for":
    "Aufgaben, auf die dieser Meilenstein wartet",
  "c.MilestoneCard.this_plan_has_no_tasks":
    "Dieser Plan hat noch keine Aufgaben.",
  "c.MilestoneCard.to": "Nach",
  "c.MilestoneCard.view_plan": "Plan ansehen",
  "c.MilestoneCard.when_all_those_tasks_are":
    "Wenn alle diese Aufgaben erledigt sind",
  "c.MilestoneCard.why": "Grund",
  "c.NetworkDiagram.float": "{slice} → {slice2} · Puffer {total_float}",
  "c.NetworkDiagram.no_dependencies": "Keine Abhängigkeiten.",
  "c.NetworkDiagram.no_scheduled_tasks_to_draw":
    "Noch keine terminierten Aufgaben zum Zeichnen.",
  "c.NetworkDiagram.task_network_boxes_are_tasks":
    "Aufgabennetz: Kästen sind Aufgaben, Pfeile sind Abhängigkeiten",
  "c.NetworkDiagram.the_same_network_as_a": "Dasselbe Netz als Liste",
  "c.ObjectiveCard.add_a_key_result": "Schlüsselergebnis hinzufügen",
  "c.ObjectiveCard.add_key_result": "Schlüsselergebnis hinzufügen",
  "c.ObjectiveCard.check_in": "Check-in",
  "c.ObjectiveCard.confidence_0_100": "Zuversicht (0–100)",
  "c.ObjectiveCard.currency": "Währung",
  "c.ObjectiveCard.currency_e_g_gbp":
    "Währung (zum Beispiel EUR, JPY oder GBP)",
  "c.ObjectiveCard.decrease": "Senken",
  "c.CommandPalette.cmd_dashboard": "Dashboard",
  "c.CommandPalette.cmd_escalations": "Eskalationen",
  "c.CommandPalette.cmd_my_work": "Meine Arbeit",
  "c.CommandPalette.cmd_new_plan": "Neuer Plan",
  "c.CommandPalette.count_one": "{n} Ergebnis",
  "c.CommandPalette.count_other": "{n} Ergebnisse",
  "c.CommandPalette.failed":
    "Die Plansuche ist gerade nicht verfügbar. Befehle funktionieren weiterhin.",
  "c.CommandPalette.kind_command": "Befehl",
  "c.CommandPalette.kind_plan": "Plan",
  "c.CommandPalette.kind_recent": "Zuletzt verwendet",
  "c.CommandPalette.label": "Pläne und Befehle suchen",
  "c.CommandPalette.none": "Keine Treffer",
  "c.CommandPalette.open": "Suche und Befehle",
  "c.CommandPalette.placeholder": "Planname oder Befehl eingeben",
  "c.CommandPalette.title": "Suche und Befehle",
  "c.NavItems.portfolios": "Portfolios",
  "c.NavItems.projects": "Projekte",
  "c.NavItems.products": "Produkte",
  "c.NavItems.programs": "Programme",
  "c.NavItems.practices": "Praktiken",
  "c.NavItems.processes": "Prozesse",
  "c.NavItems.purposes": "Zwecke",
  "c.NavItems.pathways": "Pfade",
  "c.PasteTasks.check": "Mit dem Dienst prüfen",
  "c.PasteTasks.clear": "Leeren",
  "c.PasteTasks.col_due": "Frist",
  "c.PasteTasks.col_line": "Zeile",
  "c.PasteTasks.col_notes": "Notizen",
  "c.PasteTasks.col_status": "Status",
  "c.PasteTasks.col_title": "Titel",
  "c.PasteTasks.create_one": "1 Aufgabe erstellen",
  "c.PasteTasks.create_other": "{n} Aufgaben erstellen",
  "c.PasteTasks.done": "Fertig",
  "c.PasteTasks.failed": "Das Einfügen hat nicht funktioniert.",
  "c.PasteTasks.help":
    "Kopieren Sie Zellen aus einer Tabellenkalkulation und fügen Sie sie hier ein: eine Aufgabe pro Zeile, mit den Spalten Titel, Frist (jjjj-mm-tt) und Notizen in dieser Reihenfolge. Das Datum wird zur Frist der Aufgabe. Eine erste Zeile mit Spaltennamen wird übersprungen. Eine fehlerhafte Zeile wird mit dem Grund angezeigt und ausgelassen.",
  "c.PasteTasks.label": "Eingefügte Zeilen",
  "c.PasteTasks.left_out": "Ausgelassen: {why}",
  "c.PasteTasks.problem_beyond_limit": "mehr als {max} Zeilen",
  "c.PasteTasks.problem_date_bad":
    "„{value}“ ist kein Datum; schreiben Sie es als jjjj-mm-tt",
  "c.PasteTasks.problem_no_title": "kein Titel",
  "c.PasteTasks.problem_notes_long":
    "die Notizen sind länger als {max} Zeichen",
  "c.PasteTasks.problem_same_as": "identisch mit Zeile {line}",
  "c.PasteTasks.problem_title_long": "der Titel ist länger als {max} Zeichen",
  "c.PasteTasks.ready": "Bereit",
  "c.PasteTasks.tally": "{good} bereit, {left} ausgelassen",
  "c.PasteTasks.title": "Zeilen aus einer Tabellenkalkulation einfügen",
  "c.PasteTasks.would": "Was passieren würde",
  "c.ConfirmHost.cancel": "Abbrechen",
  "c.ConfirmHost.ok": "OK",
  "c.ConfirmHost.title": "Bitte bestätigen",
  "c.ObjectiveCard.delete": "Löschen",
  "c.ObjectiveCard.direction": "Richtung",
  "c.ObjectiveCard.edit_or_delete": "Bearbeiten oder löschen",
  "c.ObjectiveCard.edit_or_delete_this_objective":
    "Dieses Ziel bearbeiten oder löschen",
  "c.ObjectiveCard.from": "{valueText} (ausgehend von {start_value})",
  "c.ObjectiveCard.increase": "Steigern",
  "c.ObjectiveCard.it_has_no_measurable_key":
    "— es hat noch kein messbares Schlüsselergebnis und daher keine Bewertung (nicht null).",
  "c.ObjectiveCard.key_results_in_more_than":
    "Schlüsselergebnisse in mehr als einer Währung werden getrennt bewertet; Währungen werden nie umgerechnet.",
  "c.ObjectiveCard.last_check_in": "· letzter Check-in {slice}",
  "c.ObjectiveCard.maintain": "Halten",
  "c.ObjectiveCard.metric": "Kennzahl",
  "c.ObjectiveCard.no_key_results_yet": "Noch keine Schlüsselergebnisse.",
  "c.ObjectiveCard.note": "Notiz",
  "c.ObjectiveCard.number": "Zahl",
  "c.ObjectiveCard.percent": "Prozent",
  "c.ObjectiveCard.period": "Zeitraum",
  "c.ObjectiveCard.record": "Erfassen",
  "c.ObjectiveCard.save": "Speichern",
  "c.ObjectiveCard.start": "Ausgangswert",
  "c.ObjectiveCard.target": "Zielwert",
  "c.ObjectiveCard.the_baseline_metric_and_direction":
    "Ausgangswert, Kennzahl und Richtung können nicht geändert werden: Fortschritt gegenüber einem beweglichen Ausgangswert ist kein Fortschritt.",
  "c.ObjectiveCard.title": "Titel",
  "c.ObjectiveCard.tolerance": "· Toleranz ±{tolerance}",
  "c.ObjectiveCard.tolerance_2": "Toleranz",
  "c.ObjectiveCard.unit": "Einheit",
  "c.ObjectiveCard.value": "Wert",
  "c.ObjectiveCard.yes_no": "Ja / nein",
  "c.PlanForm.organization": "organization:…",
  "c.PlanForm.parent_plan_pid_uuid": "pid des übergeordneten Plans (UUID)",
  "c.PlanForm.proj_2026": "PROJ-2026",
  "c.RaciMatrix.complete": "vollständig",
  "c.RaciMatrix.gaps": "Lücken",
  "c.RaciMatrix.task": "Aufgabe",
  "c.RaciMatrix.who_does_what_raci_matrix":
    "Wer macht was (RACI-Matrix): wer für jede Aufgabe verantwortlich (Responsible), rechenschaftspflichtig (Accountable), konsultiert (Consulted) und informiert (Informed) ist",
  "c.RiskInputs.adds_cost": "Verursacht Kosten",
  "c.RiskInputs.by_business_days": "um (Arbeitstage)",
  "c.RiskInputs.currency": "Währung",
  "c.RiskInputs.delays_task": "Verzögert Aufgabe",
  "c.RiskInputs.gbp": "EUR",
  "c.RiskInputs.none": "Keine",
  "c.RiskInputs.one_probability_covers_both_effects":
    "Eine Wahrscheinlichkeit gilt für beide Auswirkungen: Ein Risiko, das eine Aufgabe verzögert und Kosten verursacht, tritt einmal ein und bewirkt beides. Lassen Sie einen Teil leer, um ihn auszulassen.",
  "c.RiskInputs.probability": "Wahrscheinlichkeit (%)",
  "c.RiskInputs.save": "Speichern",
  "c.RiskInputs.saving": "Wird gespeichert…",
  "c.RiskInputs.simulation_inputs": "Simulationseingaben",
  "c.SCurve.actual": "Ist",
  "c.SCurve.date": "Datum",
  "c.SCurve.earned": "Erarbeitet",
  "c.SCurve.line_planned_value_to_earned":
    "Linie: Planwert ({from} bis {to}). ● Fertigstellungswert und ■ Istkosten, nur an erfassten Tagen (bisher {snapshots}). Gestrichelt: Budget bei Fertigstellung, {money}.",
  "c.SCurve.no_curve_to_draw": "Keine Kurve zum Zeichnen.",
  "c.SCurve.not_recorded": "nicht erfasst",
  "c.SCurve.planned": "Geplant",
  "c.SCurve.s_curve_planned_value_over":
    "S-Kurve: Planwert im Zeitverlauf, mit erfasstem Fertigstellungswert und Istkosten",
  "c.SCurve.the_same_figures_as_a": "Dieselben Zahlen als Tabelle",
  "c.VarianceTable.baselined": "Basisplan",
  "c.VarianceTable.duration": "Dauer Δ",
  "c.VarianceTable.finish": "Ende Δ",
  "c.VarianceTable.now": "Jetzt",
  "c.VarianceTable.schedule_variance_against_the_baseli":
    "Terminabweichung gegenüber dem Basisplan, in Arbeitstagen",
  "c.VarianceTable.standing": "Stand",
  "c.VarianceTable.start": "Beginn Δ",
  "c.VarianceTable.task": "Aufgabe",
  "c.VarianceTable.the_plan_s_finish_is": "Das Ende des Plans ist unverändert.",
  "c.VarianceTable.the_plan_s_own_finish":
    "Das Ende des Plans selbst lässt sich nicht vergleichen.",
  "p.announcements.announcements": "Ankündigungen",
  "p.announcements.announcements_ppm": "Ankündigungen — PPM",
  "p.announcements.expires_optional": "Läuft ab (optional)",
  "p.announcements.important": "Wichtig",
  "p.announcements.info": "Hinweis",
  "p.announcements.loading": "Wird geladen…",
  "p.announcements.message": "Nachricht",
  "p.announcements.nothing_has_been_announced": "Es wurde nichts angekündigt.",
  "p.announcements.pin_to_the_top": "Oben anheften",
  "p.announcements.pinned": "· Angeheftet",
  "p.announcements.publish": "Veröffentlichen",
  "p.announcements.publish_an_announcement": "Ankündigung veröffentlichen",
  "p.announcements.publishing_needs_the_announcements_p":
    "Zum Veröffentlichen ist die Berechtigung für Ankündigungen erforderlich.",
  "p.announcements.retract": "Zurückziehen",
  "p.announcements.severity": "Dringlichkeit",
  "p.announcements.title": "Titel",
  "p.announcements.urgent": "Dringend",
  "p.auditor.action": "Aktion",
  "p.auditor.actor": "Akteur",
  "p.auditor.actor_less_recorded_actions": "Erfasste Aktionen ohne Akteur:",
  "p.auditor.audit_trail": "Audit-Protokoll",
  "p.auditor.bearer_sub": "Bearer-sub",
  "p.auditor.detail": "Detail",
  "p.auditor.e_g_merged": "z. B. merged",
  "p.auditor.entity": "Entität",
  "p.auditor.evidence_pack_csv": "Nachweispaket (CSV)",
  "p.auditor.filter": "Filtern",
  "p.auditor.findings": "Feststellungen",
  "p.auditor.no_audit_rows_match": "Keine passenden Audit-Einträge.",
  "p.auditor.no_findings": "Keine Feststellungen.",
  "p.auditor.ppm": "{t} — PPM",
  "p.auditor.rows_distinct_actors_actor_less":
    "{returned} Zeilen · {distinct_actors} verschiedene Akteure · {actorless} ohne Akteur",
  "p.auditor.rule": "Regel",
  "p.auditor.submitted_without_a_verified_token":
    "(ohne verifiziertes Token übermittelt)",
  "p.auditor.when": "Wann",
  "p.automations.action": "Aktion",
  "p.automations.action_2": "Aktion",
  "p.automations.add_label": "Label hinzufügen",
  "p.automations.any": "beliebig",
  "p.automations.assign": "zuweisen",
  "p.automations.assignee": "Zuständige(r)",
  "p.automations.automation": "Automatisierung",
  "p.automations.automations_ppm": "Automatisierungen — PPM",
  "p.automations.blank_every_plan": "leer = jeder Plan",
  "p.automations.cancel": "Abbrechen",
  "p.automations.claim_based_so_a_deadline":
    "Beanspruchungsbasiert, sodass eine Frist genau einmal auslöst, auch wenn Taktgeber und manueller Durchlauf gleichzeitig laufen.",
  "p.automations.configure_a_rule_once_as":
    "Richten Sie eine Regel einmal ein; wenn Elemente über das Board wandern, wendet die Plattform sie an. Regeln blockieren nie die Verschiebung, die sie ausgelöst hat — ein Fehler wird als fehlgeschlagener Lauf erfasst, nicht rückgängig gemacht.",
  "p.automations.delete": "Löschen",
  "p.automations.detail": "Detail",
  "p.automations.disable": "Deaktivieren",
  "p.automations.due": "Fällig",
  "p.automations.enable": "Aktivieren",
  "p.automations.every_plan": "jeder Plan",
  "p.automations.expire_review": "Prüfung ablaufen lassen",
  "p.automations.from": "von",
  "p.automations.in_calendar_days": "in Kalendertagen",
  "p.automations.label": "Label",
  "p.automations.message": "Nachricht",
  "p.automations.name": "Name",
  "p.automations.name_2": "Name",
  "p.automations.no_rules_configured": "Keine Regeln eingerichtet.",
  "p.automations.nothing_has_fired_yet": "Noch nichts ausgelöst.",
  "p.automations.nothing_pending": "Nichts ausstehend.",
  "p.automations.notify": "benachrichtigen",
  "p.automations.off": "aus",
  "p.automations.on": "an",
  "p.automations.operator": "Operator",
  "p.automations.outcome": "Ergebnis",
  "p.automations.person_uuid": "person:<uuid>",
  "p.automations.plan_pid": "pid des Plans",
  "p.automations.plan_stage_changed": "Planphase geändert",
  "p.automations.recipient": "Empfänger",
  "p.automations.review_submitted": "Prüfung eingereicht",
  "p.automations.rules": "Regeln",
  "p.automations.runs": "Läufe",
  "p.automations.save_rule": "Regel speichern",
  "p.automations.schedule_action": "Aktion planen",
  "p.automations.scope": "Geltungsbereich",
  "p.automations.set_and_forget": "Einrichten und vergessen",
  "p.automations.set_task_status": "Aufgabenstatus setzen",
  "p.automations.source": "Quelle",
  "p.automations.state": "Zustand",
  "p.automations.status": "Status",
  "p.automations.subject": "Gegenstand",
  "p.automations.sweep_due_actions": "Fällige Aktionen ausführen",
  "p.automations.task_moved": "Aufgabe verschoben",
  "p.automations.then": "dann",
  "p.automations.to": "nach",
  "p.automations.trigger": "Auslöser",
  "p.automations.trigger_2": "Auslöser",
  "p.automations.when": "Wann",
  "p.automations.workflow_automation": "Workflow-Automatisierung",
  "p.board.amber": "gelb",
  "p.board.amount": "Betrag",
  "p.board.benefit_realizations": "realisierte Nutzen",
  "p.board.board_pack": "Vorstandsunterlagen",
  "p.board.decision": "Entscheidung",
  "p.board.decisions_in_window": "Entscheidungen im Zeitraum",
  "p.board.green": "grün",
  "p.board.investment_decisions": "Investitionsentscheidungen",
  "p.board.kind": "Art",
  "p.board.milestones_completed": "abgeschlossene Meilensteine",
  "p.board.no_decisions_in_the_window": "Keine Entscheidungen im Zeitraum.",
  "p.board.no_investment_decisions_recorded":
    "Keine Investitionsentscheidungen erfasst.",
  "p.board.no_snapshots_captured_yet": "Noch keine Momentaufnahmen erstellt.",
  "p.board.open_exposure": "Offene Exposition",
  "p.board.portfolios": "Portfolios",
  "p.board.ppm": "{t} — PPM",
  "p.board.realized_in_window": "Im Zeitraum realisiert:",
  "p.board.red_portfolios": "rote Portfolios",
  "p.board.subject": "Gegenstand",
  "p.board.take_snapshot_now": "Jetzt Momentaufnahme erstellen",
  "p.board.taken": "Erstellt",
  "p.board.tranches_released": "freigegebene Tranchen",
  "p.board.trends": "Trends",
  "p.board.unattributed_events":
    "({unattributed_events} nicht zugeordnete Ereignisse)",
  "p.board.what": "Was",
  "p.board.when": "Wann",
  "p.board.window": "Zeitraum {from} → {to}",
  "p.board.work_items": "Arbeitselemente",
  "p.calendar.all": "(alle)",
  "p.calendar.done": "· erledigt",
  "p.calendar.kind": "Art",
  "p.calendar.no_milestones_match": "Keine passenden Meilensteine.",
  "p.calendar.ppm": "{t} — PPM",
  "p.capacity.capacity_ppm": "Kapazität — PPM",
  "p.capacity.from": "von",
  "p.capacity.hatched_extension_proposed_not_count":
    "schraffierte Verlängerung: vorgeschlagen, nicht gezählt",
  "p.capacity.legend": "Legende",
  "p.capacity.over": "über",
  "p.capacity.over_capacity_solid_bar_firm":
    "über Kapazität (durchgezogener Balken: feste Buchungen; Linie: geltende Kapazität)",
  "p.capacity.proposed": "Vorgeschlagen",
  "p.capacity.to": "bis",
  "p.capacity.window": "Zeitraum: {from} → {to}",
  "p.capacity.within_capacity": "innerhalb der Kapazität",
  "p.capacity.within_capacity_but_committing_the":
    "innerhalb der Kapazität, aber das Festschreiben der vorgeschlagenen Buchungen würde sie überschreiten",
  "p.ceo.all_announcements": "Alle Ankündigungen",
  "p.ceo.all_key_measures": "Alle Kennzahlen",
  "p.ceo.all_milestones": "Alle Meilensteine",
  "p.ceo.and_more": "und {more} weitere",
  "p.ceo.announcements": "Ankündigungen",
  "p.ceo.ceo_dashboard": "CEO-Dashboard",
  "p.ceo.ceo_dashboard_ppm": "CEO-Dashboard — PPM",
  "p.ceo.decision_log": "Entscheidungsprotokoll",
  "p.ceo.escalated_risks": "eskalierte Risiken",
  "p.ceo.every_kpi_is_on_target": "● Jeder KPI liegt im Ziel.",
  "p.ceo.executive_detail": "Details für die Führung",
  "p.ceo.key_measures_needing_a_look":
    "Kennzahlen, die Aufmerksamkeit brauchen",
  "p.ceo.loading": "Wird geladen…",
  "p.ceo.milestones_to_watch": "Zu beobachtende Meilensteine",
  "p.ceo.no_data_yet": "Noch keine Daten",
  "p.ceo.no_decisions_recorded_yet": "Noch keine Entscheidungen erfasst.",
  "p.ceo.no_kpis_defined_yet": "Noch keine KPIs definiert.",
  "p.ceo.no_open_milestones": "Keine offenen Meilensteine.",
  "p.ceo.no_portfolio_is_over_budget": "● Kein Portfolio liegt über dem Budget",
  "p.ceo.nothing_has_been_announced": "Es wurde nichts angekündigt.",
  "p.ceo.over_budget": "▲ Über dem Budget: {join}",
  "p.ceo.overdue_milestones": "{overdueMilestones} überfällige Meilensteine",
  "p.ceo.portfolio_health": "Portfolio-Gesundheit",
  "p.ceo.recent_decisions": "Letzte Entscheidungen",
  "p.ceo.red_amber_green": "■ {red} rot · ▲ {amber} gelb · ● {green} grün",
  "p.ceo.risk_and_money": "Risiko und Geld",
  "p.ceo.risk_register": "Risikoregister",
  "p.compliance.compliance_risk_register": "Compliance-Risikoregister",
  "p.compliance.conformance_findings": "Konformitätsfeststellungen",
  "p.compliance.detail": "Detail",
  "p.compliance.exposure": "Exposition",
  "p.compliance.gate_review_recency_threshold_calend":
    "Aktualitätsschwelle für Gate-Prüfungen: {review_days} Kalendertage",
  "p.compliance.item": "Element",
  "p.compliance.no_compliance_risks_recorded":
    "Keine Compliance-Risiken erfasst.",
  "p.compliance.no_conformance_findings": "Keine Konformitätsfeststellungen.",
  "p.compliance.open_exposure": "{note} · offene Exposition {open_exposure}",
  "p.compliance.owner": "Verantwortlich",
  "p.compliance.ppm": "{t} — PPM",
  "p.compliance.risk": "Risiko",
  "p.compliance.rule": "Regel",
  "p.compliance.status": "Status",
  "p.cost_rates.a_person": "Eine Person",
  "p.cost_rates.a_role": "Eine Rolle",
  "p.cost_rates.add_a_rate_table": "Satztabelle hinzufügen",
  "p.cost_rates.add_rate": "Satz hinzufügen",
  "p.cost_rates.add_table": "Tabelle hinzufügen",
  "p.cost_rates.cost_rates": "Kostensätze",
  "p.cost_rates.cost_rates_are_finance_data":
    "Kostensätze sind Finanzdaten. Sie haben keinen Zugriff darauf.",
  "p.cost_rates.cost_rates_ppm": "Kostensätze — PPM",
  "p.cost_rates.currency": "Währung",
  "p.cost_rates.effective_from": "Gültig ab",
  "p.cost_rates.for": "Für",
  "p.cost_rates.from": "Ab",
  "p.cost_rates.name": "Name",
  "p.cost_rates.no_rate_tables_yet": "Noch keine Satztabellen.",
  "p.cost_rates.no_rate_yet_work_by":
    "Noch kein Satz: Arbeit dieser {v} wird nicht bewertet.",
  "p.cost_rates.per_hour": "Pro Stunde",
  "p.cost_rates.person_person_reference": "Person (Verweis person:…)",
  "p.cost_rates.role": "Rolle",
  "p.cost_risk.band_for_lines_with_no":
    "Spanne für Positionen ohne Schätzung (± %)",
  "p.cost_risk.distribution": "Verteilung",
  "p.cost_risk.iterations": "Iterationen",
  "p.cost_risk.pert": "PERT",
  "p.cost_risk.portfolio_cost_risk": "Kostenrisiko des Portfolios",
  "p.cost_risk.portfolio_cost_risk_ppm": "Kostenrisiko des Portfolios — PPM",
  "p.cost_risk.resources": "Ressourcen",
  "p.cost_risk.run": "Ausführen",
  "p.cost_risk.running": "Wird ausgeführt…",
  "p.cost_risk.seed": "Startwert",
  "p.cost_risk.shows_people_instead": "zeigt stattdessen Personen.",
  "p.cost_risk.triangular": "Dreiecksverteilung",
  "p.cost_risk.what_all_your_plans_together":
    "Was alle Ihre Pläne zusammen kosten könnten, ausgehend von der Spanne jeder Budgetposition und den Risiken, die Kosten verursachen. Jede Währung steht für sich; sie werden nie addiert.",
  "p.dashboard.dashboard_ppm": "Dashboard — PPM",
  "p.display.display": "Anzeige",
  "p.display.display_ppm": "Anzeige — PPM",
  "p.display.dyslexia_friendly_font": "Legasthenikerfreundliche Schrift",
  "p.display.go_back_to_the_default":
    "Zur Standardeinstellung meiner Rolle zurückkehren",
  "p.display.how_the_app_looks_and":
    "Wie die Anwendung aussieht und sich liest. Diese Einstellungen ändern nie, was Sie tun dürfen.",
  "p.display.keeps_the_everyday_pages_in":
    "Hält die alltäglichen Seiten im Vordergrund (Meine Arbeit, Pläne, Board, Kalender, Rundgang) und legt den Rest hinter einen Link „Mehr“. Jede Seite ist weiterhin vorhanden und funktioniert.",
  "p.display.sign_in_to_keep_these":
    "Melden Sie sich an, um diese Einstellungen in Ihrem Konto und auf diesem Gerät zu speichern.",
  "p.display.simple_view": "Einfache Ansicht",
  "p.display.this_is_the_default_for":
    "Dies ist die Standardeinstellung für Ihre Rolle: {v}.",
  "p.display.use_simple_view": "Einfache Ansicht verwenden",
  "p.display.sections_title": "Bereiche in der oberen Leiste",
  "p.display.sections_help":
    "Entfernen Sie das Häkchen bei einem Bereich, um ihn in der oberen Leiste auszublenden. Er bleibt über seine Adresse erreichbar, und Sie können ihn hier wieder einblenden. Gespeichert wird erst, wenn Sie einen ausblenden, sodass später hinzugefügte Bereiche bei Ihnen ankommen.",
  "p.display.sections_legend": "Diese Bereiche anzeigen",
  "p.display.sections_hidden_count_one": "{n} Bereich ausgeblendet",
  "p.display.sections_hidden_count_other": "{n} Bereiche ausgeblendet",
  "p.display.sections_none_hidden": "Kein Bereich ist ausgeblendet.",
  "p.display.sections_show_all": "Alle Bereiche anzeigen",
  "p.display.week_start_title": "Erster Tag der Woche",
  "p.display.week_start_help":
    "Womit Kalender jede Woche beginnen. Standardmäßig folgt er der gewählten Sprache; Wochennummern bleiben ISO-Wochen, die am Montag beginnen.",
  "p.display.week_start_label": "Wochen beginnen am",
  "p.display.week_start_locale": "Üblicher Tag der Sprache",
  "p.display.week_start_mon": "Montag",
  "p.display.week_start_sun": "Sonntag",
  "p.display.week_start_sat": "Samstag",
  "p.display.use_the_dyslexia_friendly_font":
    "Legasthenikerfreundliche Schrift verwenden",
  "p.display.uses_the_opendyslexic_typeface_with":
    "Verwendet die Schrift OpenDyslexic mit größeren Abständen zwischen Buchstaben, Wörtern und Zeilen.",
  "p.display.you_chose_this": "Von Ihnen gewählt.",
  "p.documents.controlled_documents": "Gelenkte Dokumente",
  "p.documents.create_document": "Dokument erstellen",
  "p.documents.document": "Dokument",
  "p.documents.documents": "Dokumente",
  "p.documents.documents_ppm": "Dokumente — PPM",
  "p.documents.first_draft": "Erster Entwurf",
  "p.documents.in_force": "In Kraft",
  "p.documents.loading": "Wird geladen…",
  "p.documents.new_document": "Neues Dokument",
  "p.documents.no_documents_yet": "Noch keine Dokumente.",
  "p.documents.pid.approver_s_id": "ID der genehmigenden Person",
  "p.documents.pid.date": "Datum",
  "p.documents.pid.document": "Dokument",
  "p.documents.pid.documents": "← Dokumente",
  "p.documents.pid.look_up": "Nachschlagen",
  "p.documents.pid.nothing_was_in_force_on": "Am {on} war nichts in Kraft.",
  "p.documents.pid.person_id": "Personen-ID",
  "p.documents.pid.save_draft": "Entwurf speichern",
  "p.documents.pid.send_back": "Zurücksenden",
  "p.documents.pid.text": "Text",
  "p.documents.pid.this_version_s_text_is":
    "Der Text dieser Version ist endgültig. Eine Änderung gehört in den nächsten Entwurf.",
  "p.documents.pid.version": "Version {version}",
  "p.documents.pid.version_was_in_force_on":
    "Version {version} war am {on} in Kraft.",
  "p.documents.pid.versions": "Versionen",
  "p.documents.pid.what_changed": "Was sich geändert hat",
  "p.documents.pid.what_was_in_force_on": "Was an einem Datum in Kraft war",
  "p.documents.policies_and_procedures_a_new":
    "Richtlinien und Verfahren. Eine neue Version wird geprüft und genehmigt, bevor sie in Kraft tritt, und eine geltende Version ändert sich nie.",
  "p.documents.title": "Titel",
  "p.documents.versions": "Versionen",
  "p.engineering.assignee": "Zuständig",
  "p.engineering.blocked_calendar_days": "Blockiert (Kalendertage)",
  "p.engineering.blocked_work": "Blockierte Arbeit",
  "p.engineering.declared_cause_failure_rate":
    "Fehlerrate nach angegebener Ursache",
  "p.engineering.delivery_links": "Lieferverknüpfungen",
  "p.engineering.deploys_mo": "Deployments ({window_months} Mon.)",
  "p.engineering.devops_metrics": "DevOps-Kennzahlen",
  "p.engineering.environment": "Umgebung",
  "p.engineering.incidents": "Vorfälle",
  "p.engineering.incidents_unresolved_counted_never_t":
    "{unresolved_incidents} Vorfälle ungelöst (gezählt, nie zeitlich gemessen).",
  "p.engineering.item": "Element",
  "p.engineering.items_carry_no_moscow_tag":
    "{untagged} Elemente haben kein MoSCoW-Tag.",
  "p.engineering.median_recovery_h": "mittlere Wiederherstellung (Median, h)",
  "p.engineering.moscow_scope": "MoSCoW-Umfang",
  "p.engineering.no_deploy_events_ingested_yet":
    "Noch keine Deployment-Ereignisse übernommen.",
  "p.engineering.no_externally_tracked_items":
    "Keine extern verfolgten Elemente.",
  "p.engineering.nothing_blocked_ship_it": "Nichts blockiert. Ausliefern.",
  "p.engineering.ppm": "{t} — PPM",
  "p.engineering.releases": "Releases",
  "p.engineering.task": "Aufgabe",
  "p.engineering.trackers": "Tracker",
  "p.engineering.untracked": "Nicht verfolgt:",
  "p.engineering.version": "Version",
  "p.engineering.when": "Wann",
  "p.executive.actor": "Akteur",
  "p.executive.aligned": "Ausgerichtet",
  "p.executive.as_of": "Stand {as_of} · {derivation}",
  "p.executive.benefits": "Nutzen",
  "p.executive.benefits_realization": "Nutzenrealisierung",
  "p.executive.calendar_days_since_update": "Kalendertage seit Aktualisierung",
  "p.executive.collection": "Sammlung",
  "p.executive.currency": "Währung",
  "p.executive.decision": "Entscheidung",
  "p.executive.decision_log": "Entscheidungsprotokoll",
  "p.executive.escalated_risks": "Eskalierte Risiken",
  "p.executive.kind": "Art",
  "p.executive.largest_unaligned_items": "Größte nicht ausgerichtete Elemente",
  "p.executive.members": "Mitglieder",
  "p.executive.no_decisions_recorded_yet": "Noch keine Entscheidungen erfasst.",
  "p.executive.no_target": "kein Zielwert",
  "p.executive.non_financial_only": "nur nicht finanziell ({non_financial})",
  "p.executive.open_exposure": "Offene Exposition",
  "p.executive.overdue_milestones": "Überfällige Meilensteine",
  "p.executive.overrun_currencies": "Währungen mit Überschreitung",
  "p.executive.planned_with_no_objective": "geplant ohne Ziel",
  "p.executive.portfolio": "Portfolio",
  "p.executive.portfolio_health": "Portfolio-Gesundheit",
  "p.executive.ppm": "{t} — PPM",
  "p.executive.realization": "Realisierung",
  "p.executive.realized": "Realisiert",
  "p.executive.status": "Status",
  "p.executive.strategic_alignment": "Strategische Ausrichtung",
  "p.executive.subject": "Gegenstand",
  "p.executive.target": "Zielwert",
  "p.executive.total": "Gesamt",
  "p.executive.unaligned": "Nicht ausgerichtet",
  "p.executive.unaligned_spend": "Nicht ausgerichtete Ausgaben",
  "p.executive.when": "Wann",
  "p.financials.actual": "Ist",
  "p.financials.as_of": "Stand {as_of} · {note}",
  "p.financials.currency": "Währung",
  "p.financials.currency_exposure": "Währungsexposition",
  "p.financials.lines": "Positionen",
  "p.financials.no_budget_lines_recorded_yet":
    "Noch keine Budgetpositionen erfasst.",
  "p.financials.planned": "Geplant",
  "p.financials.ppm": "{t} — PPM",
  "p.financials.remaining": "Verbleibend",
  "p.financials.work_items": "Arbeitselemente",
  "p.gantt.main_x": "{t} — Main X",
  "p.ideas.convert_target": "Umwandlungsziel",
  "p.ideas.ideas_ppm": "Ideen — PPM",
  "p.kpis.all": "Alle",
  "p.kpis.blank_for_the_whole_portfolio": "leer für das gesamte Portfolio",
  "p.kpis.daily": "Täglich",
  "p.kpis.define": "Definieren",
  "p.kpis.define_a_key_measure_kpi": "Kennzahl (KPI) definieren",
  "p.kpis.higher_is_better": "Höher ist besser",
  "p.kpis.key_measures_kpis": "Kennzahlen (KPIs)",
  "p.kpis.kpis_by_status": "KPIs nach Status",
  "p.kpis.kpis_ppm": "KPIs — PPM",
  "p.kpis.loading": "Wird geladen…",
  "p.kpis.lower_is_better": "Niedriger ist besser",
  "p.kpis.monthly": "Monatlich",
  "p.kpis.name": "Name",
  "p.kpis.no_data_yet": "Noch keine Daten",
  "p.kpis.not_set": "Nicht festgelegt",
  "p.kpis.off_target": "Nicht im Ziel",
  "p.kpis.on_target": "Im Ziel",
  "p.kpis.percent_defects_gbp": "Prozent, Fehler, EUR…",
  "p.kpis.plan_optional_pid": "Plan (optionale pid)",
  "p.kpis.quarterly": "Vierteljährlich",
  "p.kpis.read_how_often": "Wie oft messen",
  "p.kpis.red_line": "Rote Linie",
  "p.kpis.show": "Anzeigen",
  "p.kpis.something_you_track_over_time":
    "Etwas, das Sie im Zeitverlauf gegen einen Zielwert verfolgen, damit Sie früh sehen, ob Sie auf Kurs sind.",
  "p.kpis.standing_measures_green_is_at":
    "Dauerhafte Kennzahlen. Grün bedeutet am oder besser als der Zielwert; Rot am oder schlechter als die rote Linie; Gelb liegt dazwischen. Ein KPI, den noch niemand gemessen hat, zeigt „Noch keine Daten“.",
  "p.kpis.target": "Zielwert",
  "p.kpis.the_red_line_must_be":
    "Die rote Linie muss auf der schlechteren Seite des Zielwerts liegen. Skalieren Sie Werte auf ganze Zahlen (Basispunkte, Cent).",
  "p.kpis.unit": "Einheit",
  "p.kpis.watch": "Beobachten",
  "p.kpis.weekly": "Wöchentlich",
  "p.kpis.which_way_is_good": "Welche Richtung ist günstig",
  "p.layout.display": "Anzeige",
  "p.layout.more": "Mehr",
  "p.layout.simple_view": "Einfache Ansicht: {v}",
  "p.lifecycle.as_of": "Stand {as_of}",
  "p.lifecycle.in_an_unrecognized_phase": "in einer unbekannten Phase",
  "p.lifecycle.lifecycle": "Lebenszyklus",
  "p.lifecycle.lifecycle_ppm": "Lebenszyklus — PPM",
  "p.lifecycle.live": "Aktiv",
  "p.lifecycle.live_items_one":
    "{total} aktives Element · Stillstandsschwelle {stall}",
  "p.lifecycle.live_items_other":
    "{total} aktive Elemente · Stillstandsschwelle {stall}",
  "p.lifecycle.phase": "Phase",
  "p.lifecycle.share": "Anteil",
  "p.lifecycle.stalled": "Stillstand",
  "p.lifecycle.where_everything_sits_across_the":
    "Wo sich alles im Lebenszyklus der Vorhaben befindet, damit die nächste Phase nie überrascht. „Stillstand“ zählt Elemente, die länger als die Schwelle in einer Phase verharren.",
  "p.me.calendar.add": "Hinzufügen",
  "p.me.calendar.at_least_one_weekday_must":
    "Mindestens ein Wochentag muss ein Arbeitstag sein.",
  "p.me.calendar.from": "Von",
  "p.me.calendar.loading": "Wird geladen…",
  "p.me.calendar.most_i_can_commit": "Maximal zusagbar (%)",
  "p.me.calendar.my_calendar": "Mein Kalender",
  "p.me.calendar.my_calendar_ppm": "Mein Kalender — PPM",
  "p.me.calendar.my_days_off": "Meine freien Tage",
  "p.me.calendar.my_work": "← Meine Arbeit",
  "p.me.calendar.none_set": "Keine festgelegt.",
  "p.me.calendar.note": "Notiz",
  "p.me.calendar.part_time_spells_and_leave": "Teilzeitphasen und Abwesenheit",
  "p.me.calendar.remove": "Entfernen",
  "p.me.calendar.save": "Speichern",
  "p.me.calendar.saved": "Gespeichert.",
  "p.me.calendar.the_days_i_work": "Meine Arbeitstage",
  "p.me.calendar.the_most_of_your_time":
    "Der größte Anteil Ihrer Zeit, den Sie zwischen zwei Daten zusagen können: 60 für eine Teilzeitphase, 0 für Abwesenheit. Außerhalb dieser Daten sind es 100. Die Kapazität wird tageweise gelesen, sodass eine kurze Phase nie weggemittelt wird.",
  "p.me.calendar.to": "{from} bis {to}: {max_percent} %{v}",
  "p.me.calendar.to_2": "Bis",
  "p.me.due": "· fällig {due_date}",
  "p.me.in_plans_you_cannot_read":
    "· {omitted_unreadable} in Plänen, die Sie nicht lesen dürfen",
  "p.me.loading": "Wird geladen…",
  "p.me.my_calendar": "Mein Kalender",
  "p.me.my_work": "Meine Arbeit",
  "p.me.my_work_ppm": "Meine Arbeit — PPM",
  "p.me.nothing_assigned_to_you_enjoy":
    "Ihnen ist nichts zugewiesen. Genießen Sie es.",
  "p.me.open": "{today} ({tz}) · {total} offen",
  "p.me.showing_the_first_500": "· die ersten 500 werden angezeigt",
  "p.me.the_days_you_work_so":
    "— die Tage, an denen Sie arbeiten, damit Ihre Termine ehrlich sind.",
  "p.me.your_sign_in_identity_cannot":
    "Ihre Anmeldeidentität lässt sich keinen Aufgabenzuweisungen zuordnen, daher kann hier noch nichts aufgelistet werden.",
  "p.milestones.a_milestone_is_a_dated":
    "Ein Meilenstein ist ein datiertes Versprechen. Verknüpfen Sie ihn mit den Aufgaben, auf die er wartet, dann ist er erreicht, wenn diese erledigt sind, mit einer Prognose aus dem Zeitplan; ohne Verknüpfung markiert eine Person ihn als erledigt.",
  "p.milestones.achieved": "Erreicht",
  "p.milestones.all": "Alle",
  "p.milestones.due_soon": "Bald fällig",
  "p.milestones.forecast": "Prognose:",
  "p.milestones.late": "Verspätet",
  "p.milestones.loading": "Wird geladen…",
  "p.milestones.milestones": "Meilensteine",
  "p.milestones.milestones_by_status": "Meilensteine nach Status",
  "p.milestones.milestones_ppm": "Meilensteine — PPM",
  "p.milestones.show": "Anzeigen",
  "p.milestones.upcoming": "Anstehend",
  "p.objectives.objectives_ppm": "Ziele — PPM",
  "p.objectives.period_e_g_2026_h2": "Zeitraum (z. B. 2026-H2)",
  "p.okrs.2026_q4_blank_for_all": "2026-Q4 (leer für alle)",
  "p.okrs.a_goal_with_a_few":
    "Ein Ziel mit einigen messbaren Ergebnissen, die zeigen, ob Sie es erreicht haben.",
  "p.okrs.create": "Erstellen",
  "p.okrs.goals_and_key_results_okrs": "Ziele und Schlüsselergebnisse (OKRs)",
  "p.okrs.loading": "Wird geladen…",
  "p.okrs.new_objective": "Neues Ziel",
  "p.okrs.objectives_and_key_results_scores":
    "Ziele und Schlüsselergebnisse. Bewertungen werden bei jedem Aufruf aus den Check-ins berechnet; nichts wird gespeichert, das veralten könnte.",
  "p.okrs.okrs_ppm": "OKRs — PPM",
  "p.okrs.period": "Zeitraum",
  "p.okrs.show": "Anzeigen",
  "p.okrs.title": "Titel",
  "p.onboarding.a_role_by_role_first":
    "Ein Rundgang für die erste Stunde, Rolle für Rolle. Jeder Schritt nennt die Seite, auf der er landet. Dies ist eine vorgeschlagene Reihenfolge, keine gemessene Aussage wie „Sie sind in N Minuten produktiv“ — niemand hat die Zeit gemessen.",
  "p.onboarding.onboarding_ppm": "Einarbeitung — PPM",
  "p.onboarding.operator_onboarding": "Einarbeitung für Bedienende",
  "p.plans.main_x": "{t} — Main X",
  "p.plans.merge.ppm": "{t} — PPM",
  "p.plans.new.main_x": "{t} — Main X",
  "p.plans.pid.board.action": "Aktion",
  "p.plans.pid.board.add_note": "Notiz hinzufügen",
  "p.plans.pid.board.add_sprint": "Sprint hinzufügen",
  "p.plans.pid.board.add_task": "Aufgabe hinzufügen",
  "p.plans.pid.board.board": "Board",
  "p.plans.pid.board.burndown": "Burndown — {name}",
  "p.plans.pid.board.convert_to_task": "In Aufgabe umwandeln",
  "p.plans.pid.board.created_moves_blocked_now":
    "{length} erstellt · {length2} Verschiebungen · {length3} derzeit blockiert",
  "p.plans.pid.board.date": "Datum",
  "p.plans.pid.board.feedback": "Feedback",
  "p.plans.pid.board.improve": "verbessern",
  "p.plans.pid.board.new_task_title": "Titel der neuen Aufgabe",
  "p.plans.pid.board.no_notes_for_this_sprint":
    "Noch keine Notizen für diesen Sprint.",
  "p.plans.pid.board.none": "(keine)",
  "p.plans.pid.board.note": "Notiz",
  "p.plans.pid.board.note_body": "Notiztext",
  "p.plans.pid.board.note_category": "Notizkategorie",
  "p.plans.pid.board.points_done": "Erledigte Punkte",
  "p.plans.pid.board.ppm": "{t} — PPM",
  "p.plans.pid.board.pts": "Pkt.",
  "p.plans.pid.board.remaining_of": "Verbleibend von {total_tasks}",
  "p.plans.pid.board.retro_feedback_notes": "Retro- / Feedback-Notizen",
  "p.plans.pid.board.sprint": "Sprint",
  "p.plans.pid.board.sprint_end": "Sprintende",
  "p.plans.pid.board.sprint_name": "Sprintname",
  "p.plans.pid.board.sprint_start": "Sprintbeginn",
  "p.plans.pid.board.sprints": "Sprints",
  "p.plans.pid.board.standup_last_24h": "Standup (letzte 24 h)",
  "p.plans.pid.board.story_points": "Story Points",
  "p.plans.pid.board.task_created": "→ Aufgabe erstellt",
  "p.plans.pid.board.tasks_done": "Erledigte Aufgaben",
  "p.plans.pid.board.unpointed": "Ohne Punkte",
  "p.plans.pid.board.velocity": "Velocity",
  "p.plans.pid.board.went_well": "lief gut",
  "p.plans.pid.calendar.at_least_one_weekday_must":
    "Mindestens ein Wochentag muss ein Arbeitstag sein.",
  "p.plans.pid.calendar.critical_path": "← Kritischer Pfad",
  "p.plans.pid.calendar.holidays": "Feiertage",
  "p.plans.pid.calendar.loading": "Wird geladen…",
  "p.plans.pid.calendar.my_calendar": "Mein Kalender",
  "p.plans.pid.calendar.save_calendar": "Kalender speichern",
  "p.plans.pid.calendar.saved": "Gespeichert.",
  "p.plans.pid.calendar.their_work_then_happens_only":
    "); ihre Arbeit findet dann nur an Tagen statt, die in beiden Kalendern Arbeitstage sind.",
  "p.plans.pid.calendar.working_calendar": "Arbeitskalender",
  "p.plans.pid.calendar.working_calendar_ppm": "Arbeitskalender — PPM",
  "p.plans.pid.calendar.working_weekdays": "Arbeitswochentage",
  "p.plans.pid.cost_risk.band_for_lines_with_no":
    "Spanne für Positionen ohne Schätzung (± %)",
  "p.plans.pid.cost_risk.cost_and_schedule_together":
    "Kosten und Zeitplan zusammen",
  "p.plans.pid.cost_risk.cost_risk": "Kostenrisiko",
  "p.plans.pid.cost_risk.cost_risk_ppm": "Kostenrisiko — PPM",
  "p.plans.pid.cost_risk.distribution": "Verteilung",
  "p.plans.pid.cost_risk.iterations": "Iterationen",
  "p.plans.pid.cost_risk.many_what_if_runs_of":
    "Viele Was-wäre-wenn-Läufe des Budgets zeigen die Spanne der Endkosten und wie wahrscheinlich jeder Wert ist.",
  "p.plans.pid.cost_risk.pert": "PERT",
  "p.plans.pid.cost_risk.plan": "← Plan",
  "p.plans.pid.cost_risk.run": "Ausführen",
  "p.plans.pid.cost_risk.running": "Wird ausgeführt…",
  "p.plans.pid.cost_risk.seed": "Startwert",
  "p.plans.pid.cost_risk.triangular": "Dreiecksverteilung",
  "p.plans.pid.cost_risk.whole_portfolio": "Gesamtes Portfolio",
  "p.plans.pid.costs.a_recorded_cost_keeps_the":
    "Erfasste Kosten behalten den Satz, mit dem sie bewertet wurden; eine spätere Satzänderung schreibt sie nie um.",
  "p.plans.pid.costs.add_an_expense": "Ausgabe hinzufügen",
  "p.plans.pid.costs.add_expense": "Ausgabe hinzufügen",
  "p.plans.pid.costs.all_at_the_end": "Alles am Ende",
  "p.plans.pid.costs.all_at_the_start": "Alles am Anfang",
  "p.plans.pid.costs.amount": "Betrag",
  "p.plans.pid.costs.an_expense_of_so_it":
    "▲ Eine Ausgabe von {money} {reason}, daher ist sie nicht in den Perioden enthalten.",
  "p.plans.pid.costs.choose": "Auswählen…",
  "p.plans.pid.costs.cost_rates": "Kostensätze",
  "p.plans.pid.costs.costs": "Kosten",
  "p.plans.pid.costs.costs_and_rates_are_finance":
    "Kosten und Sätze sind Finanzdaten. Sie haben keinen Zugriff darauf.",
  "p.plans.pid.costs.costs_ppm": "Kosten — PPM",
  "p.plans.pid.costs.currency": "Währung",
  "p.plans.pid.costs.description": "Beschreibung",
  "p.plans.pid.costs.evenly_across_the_task": "Gleichmäßig über die Aufgabe",
  "p.plans.pid.costs.falls": "Anfall",
  "p.plans.pid.costs.fixed_cost": "Fixkosten",
  "p.plans.pid.costs.kind": "Art",
  "p.plans.pid.costs.material": "Material",
  "p.plans.pid.costs.no_time_has_been_costed":
    "Es wurde noch keine Zeit bewertet.",
  "p.plans.pid.costs.nothing_could_be_costed_yet":
    "Noch nichts ließ sich bewerten: Fügen Sie Sätze für die ausführenden Personen oder eine Ausgabe hinzu.",
  "p.plans.pid.costs.per_period": "{currency}, je Periode",
  "p.plans.pid.costs.plan": "← Plan",
  "p.plans.pid.costs.planned_cost": "Geplante Kosten",
  "p.plans.pid.costs.record_labour_costs": "Personalkosten erfassen",
  "p.plans.pid.costs.recorded_labour_cost": "Erfasste Personalkosten",
  "p.plans.pid.costs.so_its_labour_is_not":
    "▲ {title} ({hoursText}): {reason}, daher wird die Arbeitszeit nicht bewertet.",
  "p.plans.pid.costs.task": "Aufgabe",
  "p.plans.pid.costs.time_entries_are_not_costed":
    "▲ {entries} Zeiteinträge ({hoursText}) sind nicht bewertet: noch nicht erfasst oder kein Satz. Ihre Kosten sind unbekannt, nicht null.",
  "p.plans.pid.costs.total": "Gesamt",
  "p.plans.pid.costs.work": "Arbeit",
  "p.plans.pid.critical_path.a_baseline_is_a_numbered":
    "Ein Basisplan ist eine nummerierte, dauerhaft aufbewahrte Momentaufnahme des Zeitplans: Ein neuer Basisplan erzeugt die nächste Version und ersetzt nie eine bestehende.",
  "p.plans.pid.critical_path.baselines": "Basispläne",
  "p.plans.pid.critical_path.business_days": "Arbeitstage",
  "p.plans.pid.critical_path.compare_with": "Vergleichen mit",
  "p.plans.pid.critical_path.constraint": "Einschränkung",
  "p.plans.pid.critical_path.constraint_date": "Datum der Einschränkung",
  "p.plans.pid.critical_path.critical_chains":
    "{count} kritische Ketten{more}.",
  "p.plans.pid.critical_path.critical_path": "Kritischer Pfad",
  "p.plans.pid.critical_path.critical_path_ppm": "Kritischer Pfad — PPM",
  "p.plans.pid.critical_path.deadline_a_goal_it_shows":
    "Frist (ein Ziel: erscheint als negativer Puffer, verschiebt die Aufgabe nie)",
  "p.plans.pid.critical_path.duration_business_days_0_is":
    "Dauer (Arbeitstage; 0 ist ein Meilenstein)",
  "p.plans.pid.critical_path.finish_no_earlier_than": "Ende nicht früher als",
  "p.plans.pid.critical_path.finish_no_later_than": "Ende nicht später als",
  "p.plans.pid.critical_path.gantt": "Gantt",
  "p.plans.pid.critical_path.loading": "Wird geladen…",
  "p.plans.pid.critical_path.must_finish_on_overrides_logic":
    "Muss enden am (hat Vorrang vor der Logik)",
  "p.plans.pid.critical_path.must_start_on_overrides_logic":
    "Muss beginnen am (hat Vorrang vor der Logik)",
  "p.plans.pid.critical_path.near_critical_within":
    "Beinahe kritisch innerhalb von",
  "p.plans.pid.critical_path.network": "Netzplan",
  "p.plans.pid.critical_path.no_baseline_yet_so_nothing":
    "Noch kein Basisplan, daher lässt sich nichts über Verzögerungen sagen.",
  "p.plans.pid.critical_path.none_as_soon_as_possible":
    "Keine (so früh wie möglich)",
  "p.plans.pid.critical_path.not_scheduled": "Nicht terminiert",
  "p.plans.pid.critical_path.optimistic_business_days":
    "Optimistisch (Arbeitstage)",
  "p.plans.pid.critical_path.pessimistic_business_days":
    "Pessimistisch (Arbeitstage)",
  "p.plans.pid.critical_path.plan": "← Plan",
  "p.plans.pid.critical_path.planned_start": "Geplanter Beginn",
  "p.plans.pid.critical_path.range_for_schedule_risk":
    "Spanne für das Terminrisiko",
  "p.plans.pid.critical_path.range_saved": "Spanne gespeichert.",
  "p.plans.pid.critical_path.reason_required_this_overrides_the":
    "Grund (erforderlich: dies hat Vorrang vor der Logik des Zeitplans)",
  "p.plans.pid.critical_path.save_range": "Spanne speichern",
  "p.plans.pid.critical_path.save_schedule": "Zeitplan speichern",
  "p.plans.pid.critical_path.saved_moved": "Gespeichert. Verschoben:",
  "p.plans.pid.critical_path.saved_nothing_else_moved":
    "Gespeichert. Sonst wurde nichts verschoben.",
  "p.plans.pid.critical_path.saving": "Wird gespeichert…",
  "p.plans.pid.critical_path.show_the_baseline_under_each":
    "Basisplan unter jedem Balken anzeigen",
  "p.plans.pid.critical_path.start_no_earlier_than": "Beginn nicht früher als",
  "p.plans.pid.critical_path.start_no_later_than": "Beginn nicht später als",
  "p.plans.pid.critical_path.table": "Tabelle",
  "p.plans.pid.critical_path.take_baseline": "Basisplan erstellen",
  "p.plans.pid.critical_path.task": "Aufgabe",
  "p.plans.pid.critical_path.the_duration_above_is_the":
    "{taskRangeText}. Die obige Dauer ist der wahrscheinliche Wert.",
  "p.plans.pid.critical_path.the_longest_chain_of_tasks":
    "Die längste Kette von Aufgaben, die das Enddatum bestimmt. Eine Verzögerung einer Aufgabe darauf verzögert den ganzen Plan.",
  "p.plans.pid.critical_path.these_assignees_share_no_business":
    "Diese Zuständigen haben keinen gemeinsamen Arbeitstag mit dem Kalender des Plans, daher wurde für sie der Kalender des Plans verwendet: {join}.",
  "p.plans.pid.critical_path.this_network_has_tasks_too":
    "Dieses Netz hat {length} Aufgaben, zu viele für eine sinnvolle Darstellung. Die Tabelle unten sagt dasselbe aus.",
  "p.plans.pid.critical_path.untitled": "(ohne Titel)",
  "p.plans.pid.critical_path.version": "Version {version}: {reason} ({slice})",
  "p.plans.pid.critical_path.view": "Ansicht",
  "p.plans.pid.critical_path.why_a_baseline_now": "Warum jetzt ein Basisplan?",
  "p.plans.pid.critical_path.working_calendar": "Arbeitskalender",
  "p.plans.pid.earned_value.a_percent_i_enter_flagged":
    "Ein von mir eingegebener Prozentwert (markiert)",
  "p.plans.pid.earned_value.actual_cost": "Istkosten",
  "p.plans.pid.earned_value.against_the_budget_vac":
    "Gegenüber dem Budget (VAC)",
  "p.plans.pid.earned_value.baseline_version":
    "Basisplan-Version {baseline_version}",
  "p.plans.pid.earned_value.budget_at_completion": "Budget bei Fertigstellung",
  "p.plans.pid.earned_value.budget_lines_in_have_no":
    "Budgetpositionen in {join} haben keinen Basisplan, daher sind sie in keiner der obigen Zahlen enthalten und werden nie zu einer anderen Währung addiert.",
  "p.plans.pid.earned_value.by_story_points_when_finished":
    "nach Story Points, bei Abschluss",
  "p.plans.pid.earned_value.cancel": "Abbrechen",
  "p.plans.pid.earned_value.change": "Ändern",
  "p.plans.pid.earned_value.complete": "{round} % abgeschlossen",
  "p.plans.pid.earned_value.cost_index": "Kostenindex (",
  "p.plans.pid.earned_value.cost_is": "Die Kosten sind {word}.",
  "p.plans.pid.earned_value.cost_variance_ev_ac": "Kostenabweichung (EV − AC)",
  "p.plans.pid.earned_value.count_a_task_another_way":
    "Eine Aufgabe anders zählen",
  "p.plans.pid.earned_value.count_tasks": "Aufgaben zählen",
  "p.plans.pid.earned_value.critical_path": "Kritischer Pfad",
  "p.plans.pid.earned_value.earned_value": "Fertigstellungswert",
  "p.plans.pid.earned_value.earned_value_figures":
    "Fertigstellungswerte in {currency}",
  "p.plans.pid.earned_value.earned_value_ppm": "Fertigstellungswert — PPM",
  "p.plans.pid.earned_value.final_cost_eac": "Endkosten (EAC)",
  "p.plans.pid.earned_value.finished_only": "nur abgeschlossene",
  "p.plans.pid.earned_value.forecast": "Prognose",
  "p.plans.pid.earned_value.forecast_final_cost":
    "· prognostizierte Endkosten {money}",
  "p.plans.pid.earned_value.forecasts_of_the_final_cost":
    "Prognosen der Endkosten in {currency}, jeweils nach eigener Formel",
  "p.plans.pid.earned_value.governance": "← Governance",
  "p.plans.pid.earned_value.half_for_started": "die Hälfte für begonnene",
  "p.plans.pid.earned_value.is_the_work_finished_worth":
    "Ist die erledigte Arbeit so viel wert wie bis jetzt geplant, und was hat sie gekostet? Ein Index über 1,00 ist besser als geplant.",
  "p.plans.pid.earned_value.leading_with_that_one_because":
    "Dieser Wert steht vorn, weil {why}. Die anderen werden daneben gezeigt: Welcher zutrifft, hängt davon ab, ob sich bisherige Überschreitungen fortsetzen.",
  "p.plans.pid.earned_value.loading": "Wird geladen…",
  "p.plans.pid.earned_value.manual": "manuell",
  "p.plans.pid.earned_value.measured_as_of_percent_complete":
    "Gemessen zum {as_of}. Der Fertigstellungsgrad ist erledigte Arbeit, nicht verstrichene Zeit.",
  "p.plans.pid.earned_value.method": "Methode",
  "p.plans.pid.earned_value.only_when_finished": "nur bei Abschluss",
  "p.plans.pid.earned_value.only_when_finished_2": "Nur bei Abschluss",
  "p.plans.pid.earned_value.over_time": "Im Zeitverlauf",
  "p.plans.pid.earned_value.percent_complete": "Fertigstellungsgrad",
  "p.plans.pid.earned_value.planned_value": "Planwert",
  "p.plans.pid.earned_value.plans": "{plans} Pläne",
  "p.plans.pid.earned_value.progress_against_budget_earned_value":
    "Fortschritt gegenüber dem Budget (Fertigstellungswert)",
  "p.plans.pid.earned_value.record_today_s_figures_now":
    "Heutige Zahlen jetzt erfassen",
  "p.plans.pid.earned_value.recorded": "Erfasst.",
  "p.plans.pid.earned_value.save": "Speichern",
  "p.plans.pid.earned_value.schedule_index": "Terminindex (",
  "p.plans.pid.earned_value.schedule_index_by_time": "Terminindex nach Zeit (",
  "p.plans.pid.earned_value.schedule_is": "Der Zeitplan ist {word}.",
  "p.plans.pid.earned_value.schedule_variance_ev_pv":
    "Terminabweichung (EV − PV)",
  "p.plans.pid.earned_value.spi": "SPI {ratioOrReason} ·",
  "p.plans.pid.earned_value.started_work_counts_half":
    "begonnene Arbeit zählt zur Hälfte",
  "p.plans.pid.earned_value.started_work_counts_half_2":
    "Begonnene Arbeit zählt zur Hälfte",
  "p.plans.pid.earned_value.still_to_spend_etc": "Noch auszugeben (ETC)",
  "p.plans.pid.earned_value.task": "(Aufgabe)",
  "p.plans.pid.earned_value.task_2": "Aufgabe",
  "p.plans.pid.earned_value.the_one_to_lead_with": "— der maßgebliche Wert",
  "p.plans.pid.earned_value.the_scheduler_also_records_them":
    "Der Planer erfasst sie außerdem täglich.",
  "p.plans.pid.earned_value.this_plan_and_everything_beneath":
    "Dieser Plan und alles darunter",
  "p.plans.pid.earned_value.unknown": "unbekannt",
  "p.plans.pid.earned_value.use_the_plan_s_method":
    "Methode des Plans verwenden",
  "p.plans.pid.earned_value.where_it_is_heading": "Wohin es geht",
  "p.plans.pid.earned_value.why_required": "Grund (erforderlich)",
  "p.plans.pid.edit.main_x": "{t} — Main X",
  "p.plans.pid.flow.180_calendar_days": "180 Kalendertage",
  "p.plans.pid.flow.30_calendar_days": "30 Kalendertage",
  "p.plans.pid.flow.60_calendar_days": "60 Kalendertage",
  "p.plans.pid.flow.90_calendar_days": "90 Kalendertage",
  "p.plans.pid.flow.age": "Alter",
  "p.plans.pid.flow.aging_work_in_progress": "Alternde laufende Arbeit",
  "p.plans.pid.flow.arrival_rate": "Ankunftsrate (λ)",
  "p.plans.pid.flow.assignee": "Zuständig",
  "p.plans.pid.flow.blocked": "Blockiert",
  "p.plans.pid.flow.calendar_day": "{v} / Kalendertag",
  "p.plans.pid.flow.column": "Spalte",
  "p.plans.pid.flow.columns": "Spalten",
  "p.plans.pid.flow.concentrated": "konzentriert",
  "p.plans.pid.flow.constraints": "Engpässe",
  "p.plans.pid.flow.count": "Anzahl",
  "p.plans.pid.flow.cumulative_flow": "Kumulativer Fluss",
  "p.plans.pid.flow.cycle_time_is_work_in":
    "Die Durchlaufzeit ist laufende Arbeit geteilt durch Durchsatz; eine niedrigere Obergrenze verkürzt sie also, ohne dass jemand schneller arbeitet.",
  "p.plans.pid.flow.detail": "Detail",
  "p.plans.pid.flow.distributions": "Verteilungen",
  "p.plans.pid.flow.first_pass_yield": "Erstdurchlaufquote",
  "p.plans.pid.flow.flow": "Fluss",
  "p.plans.pid.flow.flow_efficiency": "Flusseffizienz",
  "p.plans.pid.flow.flow_ppm": "Fluss — PPM",
  "p.plans.pid.flow.how_long_work_takes": "Wie lange Arbeit dauert",
  "p.plans.pid.flow.how_long_work_takes_to":
    "Wie lange es dauert, bis Arbeit erledigt ist, und wie viel gleichzeitig in Arbeit ist.",
  "p.plans.pid.flow.implied_cycle_time": "Abgeleitete Durchlaufzeit",
  "p.plans.pid.flow.item": "Element",
  "p.plans.pid.flow.limit": "Grenze",
  "p.plans.pid.flow.max": "max.",
  "p.plans.pid.flow.mean": "Mittelwert",
  "p.plans.pid.flow.measure": "Kennzahl",
  "p.plans.pid.flow.min": "min.",
  "p.plans.pid.flow.no_cap": "keine Obergrenze",
  "p.plans.pid.flow.no_expectation_yet": "noch keine Erwartung",
  "p.plans.pid.flow.no_findings_yet": "Noch keine Feststellungen.",
  "p.plans.pid.flow.no_finished_work_to_measure":
    "Noch keine erledigte Arbeit zum Messen.",
  "p.plans.pid.flow.not_enough_finished_items_yet":
    "Noch nicht genügend erledigte Elemente.",
  "p.plans.pid.flow.not_started_finished_of":
    "{not_started} nicht begonnen · {finished} erledigt von {tasks}",
  "p.plans.pid.flow.nothing_in_progress": "Nichts in Arbeit.",
  "p.plans.pid.flow.observed_median": "Beobachteter Median",
  "p.plans.pid.flow.of_the_calendar_time_these":
    "Wie viel der Kalenderzeit, die diese Elemente gebraucht haben, hat tatsächlich jemand daran gearbeitet? Die Flusseffizienz liegt in der Wissensarbeit typischerweise bei 5–15 %, ein niedriger Wert ist also normal — die Warteschlange ist der Befund, nicht der Einsatz.",
  "p.plans.pid.flow.of_the_transitions_behind_these":
    "{percent} der Übergänge hinter diesen Zahlen wurden bei Einführung des Protokolls nachträglich erzeugt, nicht beobachtet. Sie werden belastbarer, je mehr echte Board-Bewegungen hinzukommen.",
  "p.plans.pid.flow.percentiles_are_nearest_rank_so":
    "Perzentile werden nach dem Nächster-Rang-Verfahren bestimmt, daher ist jedes ein reales Element. Die Durchlaufzeit steuert das Team; die Vorlaufzeit ist die Wartezeit der anfragenden Person, und der Unterschied zwischen beiden ist die Zeit, die Arbeit im Backlog lag.",
  "p.plans.pid.flow.recoverable": "Vermeidbar",
  "p.plans.pid.flow.rework": "Nacharbeit",
  "p.plans.pid.flow.rule": "Regel",
  "p.plans.pid.flow.service_level_expectation": "Service-Level-Erwartung",
  "p.plans.pid.flow.status": "Status",
  "p.plans.pid.flow.strong_against_the_5_15":
    "Stark gemessen an der Norm von 5–15 %.",
  "p.plans.pid.flow.subject": "Gegenstand",
  "p.plans.pid.flow.the_mean_is_shown_for":
    "Der Mittelwert wird der Vollständigkeit halber gezeigt und beschreibt kein tatsächliches Element — nennen Sie das p85.",
  "p.plans.pid.flow.the_plan_s_overall_ratio":
    ": Das Gesamtverhältnis des Plans ({percent}) weicht deutlich vom typischen Element ({percent2}) ab. Die meisten Elemente fließen; eine Minderheit hängt fest. Das erfordert eine andere Lösung als eine gleichmäßig langsame Lieferung — finden Sie diese Elemente, statt den Prozess umzubauen.",
  "p.plans.pid.flow.the_waste_is": "Die Verschwendung ist",
  "p.plans.pid.flow.throughput": "Durchsatz (μ)",
  "p.plans.pid.flow.typical_median_item": "Typisch. Median-Element: {percent}.",
  "p.plans.pid.flow.unassigned": "nicht zugewiesen",
  "p.plans.pid.flow.unusually_high_this_usually_means":
    "Ungewöhnlich hoch — das bedeutet meist, dass das Board nicht aktuell gehalten wird, nicht dass die Warteschlange verschwunden ist.",
  "p.plans.pid.flow.utilization": "Auslastung (ρ)",
  "p.plans.pid.flow.vs_expectation": "ggü. Erwartung",
  "p.plans.pid.flow.window": "Zeitfenster",
  "p.plans.pid.flow.work_in_progress": "Laufende Arbeit",
  "p.plans.pid.flow.work_in_progress_2": "Laufende Arbeit (κ)",
  "p.plans.pid.governance.actual": "Ist",
  "p.plans.pid.governance.approved": "genehmigt",
  "p.plans.pid.governance.approved_with_conditions": "mit Auflagen genehmigt",
  "p.plans.pid.governance.benefit": "Nutzen",
  "p.plans.pid.governance.benefit_category": "Nutzenkategorie",
  "p.plans.pid.governance.capex": "Investitionskosten",
  "p.plans.pid.governance.category": "Kategorie",
  "p.plans.pid.governance.choose_objective": "Ziel auswählen…",
  "p.plans.pid.governance.commit": "Festschreiben",
  "p.plans.pid.governance.commit_anyway": "Trotzdem festschreiben",
  "p.plans.pid.governance.committing_this_puts_them_over":
    "Durch das Festschreiben wird ihre Kapazität überschritten.",
  "p.plans.pid.governance.compliance": "Compliance",
  "p.plans.pid.governance.cost_saving": "Kosteneinsparung",
  "p.plans.pid.governance.currency": "Währung",
  "p.plans.pid.governance.decision": "Entscheidung",
  "p.plans.pid.governance.governance_ppm": "Governance — PPM",
  "p.plans.pid.governance.hold": "zurückgestellt",
  "p.plans.pid.governance.line": "Position",
  "p.plans.pid.governance.materialized": "eingetreten: {materialized}",
  "p.plans.pid.governance.max_exposure": "max. Exposition: {max_exposure}",
  "p.plans.pid.governance.new_benefit": "Neuer Nutzen",
  "p.plans.pid.governance.new_budget_line": "Neue Budgetposition",
  "p.plans.pid.governance.new_milestone": "Neuer Meilenstein",
  "p.plans.pid.governance.new_milestone_due":
    "Fälligkeitsdatum des neuen Meilensteins",
  "p.plans.pid.governance.new_risk": "Neues Risiko",
  "p.plans.pid.governance.next": "nächstes: {next_gate}",
  "p.plans.pid.governance.objective": "Ziel",
  "p.plans.pid.governance.open_risks": "offene Risiken: {open}",
  "p.plans.pid.governance.opex": "Betriebskosten",
  "p.plans.pid.governance.other": "sonstige",
  "p.plans.pid.governance.planned": "Geplant",
  "p.plans.pid.governance.planned_actual_variance":
    ": geplant {money} · Ist {money2} · Abweichung {money3}",
  "p.plans.pid.governance.planned_major": "Geplant (Haupteinheiten)",
  "p.plans.pid.governance.proposed_does_not_count_until":
    "vorgeschlagen (zählt erst nach dem Festschreiben)",
  "p.plans.pid.governance.quality": "Qualität",
  "p.plans.pid.governance.range_for_cost_risk": "Spanne für das Kostenrisiko",
  "p.plans.pid.governance.realized": "Realisiert",
  "p.plans.pid.governance.record_actual": "Ist erfassen",
  "p.plans.pid.governance.rejected": "abgelehnt",
  "p.plans.pid.governance.revenue": "Umsatz",
  "p.plans.pid.governance.review": "Prüfung",
  "p.plans.pid.governance.risk": "Risiko",
  "p.plans.pid.governance.risk_reduction": "Risikominderung",
  "p.plans.pid.governance.roi": "· ROI {toFixed} %",
  "p.plans.pid.governance.stage": "Phase: {v}",
  "p.plans.pid.governance.status": "Status",
  "p.plans.pid.governance.target": "Soll",
  "p.plans.pid.governance.target_major": "Soll (Haupteinheiten)",
  "p.plans.pid.governance.target_realized_spend":
    ": Soll {money} · realisiert {money2} · Ausgaben {money3}",
  "p.plans.pid.governance.weight": "Gewicht",
  "p.plans.pid.governance.worker_uuid": "worker:<uuid>",
  "p.plans.pid.handover.discard": "Verwerfen",
  "p.plans.pid.handover.from": "Von",
  "p.plans.pid.handover.hand_over": "Übergeben",
  "p.plans.pid.handover.handover": "Übergabe",
  "p.plans.pid.handover.handover_ppm": "Übergabe — PPM",
  "p.plans.pid.handover.kind": "Art",
  "p.plans.pid.handover.onboarding_and_offboarding":
    "Onboarding und Offboarding",
  "p.plans.pid.handover.person": "person:…",
  "p.plans.pid.handover.person_empty_releases_the_work":
    "person:… (leer gibt die Arbeit frei)",
  "p.plans.pid.handover.plan": "← Plan",
  "p.plans.pid.handover.preview": "Vorschau",
  "p.plans.pid.handover.reason_recorded_with_each_item":
    "Grund (wird bei jedem Eintrag festgehalten)",
  "p.plans.pid.handover.release": "Freigeben",
  "p.plans.pid.handover.the_new_person_s_capacity":
    "Die Kapazität der neuen Person wird hier nicht geprüft.",
  "p.plans.pid.handover.to": "An",
  "p.plans.pid.handover.what": "Was",
  "p.plans.pid.handover.what_holds_that_is_still":
    "Was {from_person} noch an laufender Arbeit hält",
  "p.plans.pid.handover.who_is_handing_over": "Wer übergibt?",
  "p.plans.pid.import.bring_tasks_in_from_jira":
    "Übernehmen Sie Aufgaben aus Jira, Asana oder Microsoft Project. Sie sehen zuerst, was passieren würde; nichts wird geschrieben, bevor Sie bestätigen.",
  "p.plans.pid.import.choose_another_file": "Andere Datei auswählen",
  "p.plans.pid.import.file": "Datei",
  "p.plans.pid.import.import": "Importieren",
  "p.plans.pid.import.import_ppm": "Import — PPM",
  "p.plans.pid.import.imported": "Importiert",
  "p.plans.pid.import.plan": "← Plan",
  "p.plans.pid.import.preview": "Vorschau",
  "p.plans.pid.import.tracker": "Tracker",
  "p.plans.pid.import.where_from": "Herkunft",
  "p.plans.pid.joint_risk.a_late_project_usually_costs":
    "Ein verspätetes Projekt kostet meist auch mehr, daher werden hier Kosten und Termine gemeinsam simuliert.",
  "p.plans.pid.joint_risk.assumptions": "Annahmen",
  "p.plans.pid.joint_risk.band_when_there_is_no": "Spanne ohne Schätzung (± %)",
  "p.plans.pid.joint_risk.both_at_once": "Beides zugleich",
  "p.plans.pid.joint_risk.budget": "Budget",
  "p.plans.pid.joint_risk.budget_2": "Budget",
  "p.plans.pid.joint_risk.budget_lines_in_are_not":
    "Budgetpositionen in {join} sind nicht Teil dieses Laufs; Währungen werden nie addiert.",
  "p.plans.pid.joint_risk.business_days_late":
    "{days} Arbeitstage verspätet, {money}",
  "p.plans.pid.joint_risk.business_days_late_2": "Arbeitstage verspätet →",
  "p.plans.pid.joint_risk.cost": "Kosten",
  "p.plans.pid.joint_risk.cost_2": "Kosten — {title}: {costSkippedText}",
  "p.plans.pid.joint_risk.cost_and_schedule_risk_ppm":
    "Kosten- und Terminrisiko — PPM",
  "p.plans.pid.joint_risk.cost_and_schedule_together":
    "Kosten und Termine gemeinsam",
  "p.plans.pid.joint_risk.cost_lines": "Kostenpositionen",
  "p.plans.pid.joint_risk.cost_of_each_business_day":
    "Kosten je Arbeitstag Verspätung (Haupteinheiten)",
  "p.plans.pid.joint_risk.cost_risk": "Kostenrisiko",
  "p.plans.pid.joint_risk.critical_path_finish_the_plan":
    "Ende laut kritischem Pfad (das eigene Datum des Plans)",
  "p.plans.pid.joint_risk.currency_if_the_plan_has":
    "Währung (falls der Plan mehrere hat)",
  "p.plans.pid.joint_risk.distribution": "Verteilung",
  "p.plans.pid.joint_risk.each_dot_is_one_simulation":
    "Jeder Punkt ist eine Simulation (ausgedünnte Stichprobe). Rechts der Termin-Linie ist verspätet, oberhalb der Budgetlinie ist über Budget. Die Tabelle oben enthält die vollständigen Zahlen.",
  "p.plans.pid.joint_risk.estimate": "Schätzung",
  "p.plans.pid.joint_risk.finish": "Ende",
  "p.plans.pid.joint_risk.finish_date_at_each_percentile":
    "Enddatum je Perzentil",
  "p.plans.pid.joint_risk.how_the_simulated_outcomes_fall":
    "Wie die simulierten Ergebnisse um das Budget und das Ende laut kritischem Pfad verteilt sind",
  "p.plans.pid.joint_risk.independence": "Unabhängigkeit.",
  "p.plans.pid.joint_risk.iterations": "Iterationen",
  "p.plans.pid.joint_risk.line": "Position",
  "p.plans.pid.joint_risk.low_planned_high": "Niedrig / geplant / hoch",
  "p.plans.pid.joint_risk.mean_delay_cost_included":
    "Mittelwert (inkl. Verzögerungskosten)",
  "p.plans.pid.joint_risk.no_register_risk_was_used":
    "In diesem Lauf wurde kein Risiko aus dem Register verwendet.",
  "p.plans.pid.joint_risk.not_enough_data": "nicht genügend Daten",
  "p.plans.pid.joint_risk.of_which_delay_cost_on":
    "Davon Verzögerungskosten im Durchschnitt",
  "p.plans.pid.joint_risk.on_time": "pünktlich",
  "p.plans.pid.joint_risk.outcome": "Ergebnis",
  "p.plans.pid.joint_risk.per_line_estimate_in":
    "Schätzung je Position in {currency}",
  "p.plans.pid.joint_risk.pert": "PERT",
  "p.plans.pid.joint_risk.plan": "← Plan",
  "p.plans.pid.joint_risk.risks": "Risiken",
  "p.plans.pid.joint_risk.run": "Ausführen",
  "p.plans.pid.joint_risk.running": "Wird ausgeführt…",
  "p.plans.pid.joint_risk.scatter_of_simulated_total_cost":
    "Streudiagramm der simulierten Gesamtkosten gegenüber den Arbeitstagen Verspätung, mit Budget- und Termin-Linie",
  "p.plans.pid.joint_risk.schedule": "Zeitplan — {title}: {skippedReasonText}",
  "p.plans.pid.joint_risk.schedule_risk": "Terminrisiko",
  "p.plans.pid.joint_risk.seed": "Startwert",
  "p.plans.pid.joint_risk.seed_the_same_seed_gives":
    "Startwert {seed}: Derselbe Startwert liefert dasselbe Ergebnis.",
  "p.plans.pid.joint_risk.share": "Anteil",
  "p.plans.pid.joint_risk.simulations": "Simulationen",
  "p.plans.pid.joint_risk.total_cost_delay_cost_included":
    "Gesamtkosten inkl. Verzögerungskosten",
  "p.plans.pid.joint_risk.triangular": "Dreiecksverteilung",
  "p.plans.pid.leveling.allow_critical_tasks_to_move":
    "Kritische Aufgaben dürfen verschoben werden (dies kann das Ende verzögern)",
  "p.plans.pid.leveling.apply": "Anwenden",
  "p.plans.pid.leveling.apply_the_ticked_moves":
    "Ausgewählte Verschiebungen anwenden",
  "p.plans.pid.leveling.balance_workloads_resource_leveling":
    "Arbeitslast ausgleichen (Ressourcenabgleich)",
  "p.plans.pid.leveling.could_not_be_fixed": "Konnte nicht behoben werden",
  "p.plans.pid.leveling.critical": "kritisch",
  "p.plans.pid.leveling.each_start_date_the_proposal":
    "Jedes Startdatum, das der Vorschlag ändern würde",
  "p.plans.pid.leveling.finish": "Ende",
  "p.plans.pid.leveling.float": "Puffer",
  "p.plans.pid.leveling.i_have_read_the_proposal":
    "Ich habe den Vorschlag gelesen und möchte diese Startdaten ändern",
  "p.plans.pid.leveling.later_by": "Später um",
  "p.plans.pid.leveling.load_per_task_of_a":
    "Last je Aufgabe (% eines Arbeitstags einer Person)",
  "p.plans.pid.leveling.loading": "Wird geladen…",
  "p.plans.pid.leveling.no_start_date_would_change":
    "Kein Startdatum würde sich ändern.",
  "p.plans.pid.leveling.nobody_is_over_allocated_so":
    "Niemand ist überlastet, daher gibt es nichts auszugleichen.",
  "p.plans.pid.leveling.capacity_undeclared_h": "Keine Kapazität angegeben",
  "p.plans.pid.leveling.capacity_undeclared_p":
    "Diese Personen haben Arbeit an Tagen ohne angegebene Verfügbarkeit; an diesen Tagen sind sie weder über noch innerhalb der Kapazität:",
  "p.plans.pid.leveling.over_allocated_now": "Derzeit überlastet",
  "p.plans.pid.leveling.person": "Person",
  "p.plans.pid.leveling.plan": "← Plan",
  "p.plans.pid.leveling.propose_again": "Erneut vorschlagen",
  "p.plans.pid.leveling.proposed_start_dates": "Vorgeschlagene Startdaten",
  "p.plans.pid.leveling.refused": "Abgelehnt: {note}",
  "p.plans.pid.leveling.resource_leveling_ppm": "Ressourcenabgleich — PPM",
  "p.plans.pid.leveling.start": "Start",
  "p.plans.pid.leveling.start_finish_business_days":
    "{pid}: Start {v}{start_days}, Ende {v2}{finish_days} Arbeitstage",
  "p.plans.pid.leveling.suggests_moving_tasks_so_that":
    "Schlägt Verschiebungen von Aufgaben vor, sodass niemand über seine Möglichkeiten hinaus eingeplant ist. Nichts ändert sich, bevor Sie bestätigen.",
  "p.plans.pid.leveling.task": "Aufgabe",
  "p.plans.pid.leveling.the_search_stopped_at_its":
    "die Suche hat ihr Limit erreicht",
  "p.plans.pid.leveling.this_is_advice_nothing_changes":
    "Dies ist eine Empfehlung. Nichts ändert sich, bis Sie unten Verschiebungen auswählen und anwenden.",
  "p.plans.pid.leveling.why_kept_with_each_audit":
    "Begründung (wird bei jedem Audit-Eintrag gespeichert)",
  "p.plans.pid.leveling.working_calendar": "Arbeitskalender",
  "p.plans.pid.more_tools": "Weitere Werkzeuge",
  "p.plans.pid.onboarding.access": "Zugriff",
  "p.plans.pid.onboarding.access_record": "Zugriffseintrag",
  "p.plans.pid.onboarding.action": "Aktion",
  "p.plans.pid.onboarding.cancel_checklist": "Checkliste abbrechen",
  "p.plans.pid.onboarding.checklists": "Checklisten",
  "p.plans.pid.onboarding.complete_checklist": "Checkliste abschließen",
  "p.plans.pid.onboarding.done": "Erledigt",
  "p.plans.pid.onboarding.effective":
    "{status}{v} · gültig ab {effective_date} · {progressText}",
  "p.plans.pid.onboarding.from":
    "{status} · ab {effective_date} · {progressText}",
  "p.plans.pid.onboarding.granted": "Erteilt",
  "p.plans.pid.onboarding.hand_over_someone_s_work":
    "Arbeit einer Person übergeben",
  "p.plans.pid.onboarding.no_access_is_outstanding":
    "● Kein Zugriff ist offen.",
  "p.plans.pid.onboarding.none_yet": "Noch keine.",
  "p.plans.pid.onboarding.onboarding_and_offboarding":
    "Onboarding und Offboarding",
  "p.plans.pid.onboarding.onboarding_and_offboarding_ppm":
    "Onboarding und Offboarding — PPM",
  "p.plans.pid.onboarding.person": "person:…",
  "p.plans.pid.onboarding.person_2": "Person",
  "p.plans.pid.onboarding.plan": "← Plan",
  "p.plans.pid.onboarding.ready_to_close": "Bereit zum Abschließen?",
  "p.plans.pid.onboarding.reason": "Grund",
  "p.plans.pid.onboarding.reason_for_cancelling": "Grund für den Abbruch",
  "p.plans.pid.onboarding.record": "Erfassen",
  "p.plans.pid.onboarding.record_access": "Zugriff erfassen",
  "p.plans.pid.onboarding.reopen": "Wieder öffnen",
  "p.plans.pid.onboarding.repo_write": "Schreibzugriff auf Repository",
  "p.plans.pid.onboarding.revoked": "Entzogen",
  "p.plans.pid.onboarding.save": "Speichern",
  "p.plans.pid.onboarding.since":
    "▲ {person_ref}: {access} (seit {granted_on})",
  "p.plans.pid.onboarding.skip": "Überspringen",
  "p.plans.pid.onboarding.start": "Starten",
  "p.plans.pid.onboarding.start_a_checklist": "Checkliste starten",
  "p.plans.pid.onboarding.takes_effect": "Wirksam ab",
  "p.plans.pid.onboarding.what": "Was",
  "p.plans.pid.onboarding.who_holds_what_access_is":
    "Wer welchen Zugriff hat, betrifft Personen; daher ist die Berechtigung zur Personenverwaltung erforderlich.",
  "p.plans.pid.onboarding.why_skip": "Warum überspringen?",
  "p.plans.pid.raci.add_a_person_person_or":
    "Person hinzufügen (person:… oder worker:…)",
  "p.plans.pid.raci.add_a_person_to_start":
    "Fügen Sie eine Person hinzu, um Buchstaben zuzuweisen.",
  "p.plans.pid.raci.add_column": "Spalte hinzufügen",
  "p.plans.pid.raci.export_to_a_spreadsheet":
    "In eine Tabellenkalkulation exportieren",
  "p.plans.pid.raci.for_each_task_who_does":
    "Für jede Aufgabe: wer sie ausführt, wer sie abnimmt, wer gefragt und wer informiert wird.",
  "p.plans.pid.raci.have_nobody_accountable":
    "{missing_accountable} haben niemanden, der rechenschaftspflichtig ist.",
  "p.plans.pid.raci.have_nobody_responsible":
    "{missing_responsible} haben niemanden, der verantwortlich ist.",
  "p.plans.pid.raci.how_many_tasks_each_person":
    "Bei wie vielen Aufgaben jede Person welchen Buchstaben hat",
  "p.plans.pid.raci.legend": "Legende",
  "p.plans.pid.raci.load_by_person": "Last je Person",
  "p.plans.pid.raci.loading": "Wird geladen…",
  "p.plans.pid.raci.of_tasks_are_complete_one":
    "{complete} von {tasks} Aufgaben sind vollständig (eine rechenschaftspflichtige, mindestens eine verantwortliche Person).",
  "p.plans.pid.raci.person": "Person",
  "p.plans.pid.raci.plan": "← Plan",
  "p.plans.pid.raci.raci_ppm": "RACI — PPM",
  "p.plans.pid.raci.this_plan_has_no_tasks":
    "Dieser Plan hat noch keine Aufgaben.",
  "p.plans.pid.raci.who_does_what_raci_matrix": "Wer macht was (RACI-Matrix)",
  "p.plans.pid.schedule.dependency_violated_successor_starts":
    "Abhängigkeit verletzt: Der Nachfolger beginnt am {actual_start}, darf aber frühestens am {earliest_start} beginnen.",
  "p.plans.pid.schedule.schedule_ppm": "Zeitplan — PPM",
  "p.plans.pid.schedule_health.as_of": "Stand {as_of} ·",
  "p.plans.pid.schedule_health.baseline_version": "Baseline-Version {version}",
  "p.plans.pid.schedule_health.check": "Prüfen",
  "p.plans.pid.schedule_health.checks_the_schedule_for_common":
    "Prüft den Zeitplan auf häufige Probleme, etwa Aufgaben ohne Verknüpfungen oder ohne Schätzung.",
  "p.plans.pid.schedule_health.critical_path": "Kritischer Pfad",
  "p.plans.pid.schedule_health.held_to": "Eingehalten bis",
  "p.plans.pid.schedule_health.loading": "Wird geladen…",
  "p.plans.pid.schedule_health.no_baseline_taken": "keine Baseline erstellt",
  "p.plans.pid.schedule_health.plan": "← Plan",
  "p.plans.pid.schedule_health.re_run": "Erneut ausführen",
  "p.plans.pid.schedule_health.result": "Ergebnis",
  "p.plans.pid.schedule_health.schedule_health": "Zeitplanqualität",
  "p.plans.pid.schedule_health.schedule_health_ppm": "Zeitplanqualität — PPM",
  "p.plans.pid.schedule_health.see_the_note": "Siehe Hinweis",
  "p.plans.pid.schedule_health.status_date": "Statusdatum",
  "p.plans.pid.schedule_health.the_fourteen_schedule_health_checks":
    "Die vierzehn Prüfungen der Zeitplanqualität",
  "p.plans.pid.schedule_health.to_fix": "Zu beheben",
  "p.plans.pid.schedule_health.what_was_found": "Was gefunden wurde",
  "p.plans.pid.schedule_risk.assumptions": "Annahmen",
  "p.plans.pid.schedule_risk.band_for_tasks_with_no":
    "Spanne für Aufgaben ohne Schätzung (± %)",
  "p.plans.pid.schedule_risk.critical_in": "Kritisch in",
  "p.plans.pid.schedule_risk.critical_path": "Kritischer Pfad",
  "p.plans.pid.schedule_risk.critical_path_finish_the_plan":
    "Ende laut kritischem Pfad (das eigene Datum des Plans)",
  "p.plans.pid.schedule_risk.distribution": "Verteilung",
  "p.plans.pid.schedule_risk.each_bar_is_a_finish":
    "Jeder Balken ist ein Enddatum; je höher, desto mehr Simulationen endeten dort. Durchgezogene Linien sind P50, P80 und P90 (beschriftet); die gestrichelte Linie ist das eigene Datum des Plans.",
  "p.plans.pid.schedule_risk.earliest_latest":
    "{join}. Frühestens {earliest}, spätestens {latest}.",
  "p.plans.pid.schedule_risk.estimate": "Schätzung",
  "p.plans.pid.schedule_risk.finish": "Ende",
  "p.plans.pid.schedule_risk.finish_date": "Enddatum",
  "p.plans.pid.schedule_risk.finish_date_at_each_percentile":
    "Enddatum je Perzentil",
  "p.plans.pid.schedule_risk.histogram_of_simulated_finish_dates":
    "Histogramm der simulierten Enddaten mit Markierung von P50, P80 und P90",
  "p.plans.pid.schedule_risk.independence": "Unabhängigkeit.",
  "p.plans.pid.schedule_risk.iterations": "Iterationen",
  "p.plans.pid.schedule_risk.left_out": "Ausgelassen:",
  "p.plans.pid.schedule_risk.many_what_if_runs_of":
    "Viele Was-wäre-wenn-Läufe des Zeitplans zeigen die Spanne der Enddaten und wie wahrscheinlich jedes davon ist.",
  "p.plans.pid.schedule_risk.no_register_risk_was_used":
    "Kein Risiko aus dem Register wurde als Terminereignis verwendet.",
  "p.plans.pid.schedule_risk.not_enough_data": "nicht genügend Daten",
  "p.plans.pid.schedule_risk.not_simulated_because_the_schedule":
    "Nicht simuliert, da der Zeitplan sie nicht einordnen kann: {join}.",
  "p.plans.pid.schedule_risk.of": "{date}: {count} von {iterations}",
  "p.plans.pid.schedule_risk.of_runs": "{criticalityText} der Läufe",
  "p.plans.pid.schedule_risk.optimistic_likely_pessimistic_busine":
    "Optimistisch / wahrscheinlich / pessimistisch (Arbeitstage)",
  "p.plans.pid.schedule_risk.outcome": "Ergebnis",
  "p.plans.pid.schedule_risk.per_task_estimate_criticality_and":
    "Schätzung, Kritikalität und Sensitivität je Aufgabe",
  "p.plans.pid.schedule_risk.plan": "← Plan",
  "p.plans.pid.schedule_risk.plan_2": "Plan",
  "p.plans.pid.schedule_risk.risks_used": "Verwendete Risiken",
  "p.plans.pid.schedule_risk.run": "Ausführen",
  "p.plans.pid.schedule_risk.run_on_against_the_plan":
    "Ausgeführt am {ranOn} gegen den aktuellen Plan · Startwert {seed} · {iterations} Iterationen · {distributionText} · Kalender {calendar}",
  "p.plans.pid.schedule_risk.running": "Wird ausgeführt…",
  "p.plans.pid.schedule_risk.schedule_risk": "Terminrisiko",
  "p.plans.pid.schedule_risk.schedule_risk_ppm": "Terminrisiko — PPM",
  "p.plans.pid.schedule_risk.seed": "Startwert",
  "p.plans.pid.schedule_risk.seed_the_same_seed_gives":
    "Startwert {seed}: Derselbe Startwert liefert dasselbe Ergebnis. Bis zu {max_iterations} Iterationen.",
  "p.plans.pid.schedule_risk.sensitivity": "Sensitivität",
  "p.plans.pid.schedule_risk.share": "Anteil",
  "p.plans.pid.schedule_risk.simulations": "Simulationen",
  "p.plans.pid.schedule_risk.task": "Aufgabe",
  "p.plans.pid.schedule_risk.the_same_figures_as_a":
    "Dieselben Zahlen als Tabelle",
  "p.plans.pid.schedule_risk.triangular": "Dreiecksverteilung",
  "p.plans.pid.schedule_risk.weighted_towards_the_likely_value":
    "Zum wahrscheinlichen Wert hin gewichtet (PERT)",
  "p.plans.pid.schedule_risk.what_drives_it": "Was es beeinflusst",
  "p.plans.pid.schedule_risk.when_it_finishes": "Wann es endet",
  "p.plans.pid.wbs.code": "Code",
  "p.plans.pid.wbs.level": "Ebene",
  "p.plans.pid.wbs.no_tasks_yet": "Noch keine Aufgaben.",
  "p.plans.pid.wbs.plan": "← Plan",
  "p.plans.pid.wbs.sub_tasks": "Teilaufgaben",
  "p.plans.pid.wbs.task": "Aufgabe",
  "p.plans.pid.wbs.tasks_is_too_many_to":
    "{length} Aufgaben sind zu viele für eine sinnvolle Verschachtelung; die Tabelle zeigt alle.",
  "p.plans.pid.wbs.the_plan_s_tasks_each":
    "Die Aufgaben des Plans, jeweils in ihre Teilaufgaben gegliedert. Der Code in jeder Zeile dient nur der Orientierung: Er ändert sich, wenn Arbeit hinzugefügt oder verschoben wird.",
  "p.plans.pid.wbs.work_breakdown": "Projektstrukturplan",
  "p.plans.pid.wbs.work_breakdown_as_a_table":
    "Projektstrukturplan als Tabelle",
  "p.plans.pid.wbs.work_breakdown_ppm": "Projektstrukturplan — PPM",
  "p.plans.pid.wbs.work_breakdown_tree": "Projektstrukturbaum",
  "p.plans.pid.what_if.after": "Nachher",
  "p.plans.pid.what_if.before": "Vorher",
  "p.plans.pid.what_if.business_days": "Arbeitstage",
  "p.plans.pid.what_if.choose": "Auswählen…",
  "p.plans.pid.what_if.commit_these_changes": "Diese Änderungen festschreiben",
  "p.plans.pid.what_if.constraint": "Einschränkung",
  "p.plans.pid.what_if.critical": "■ kritisch",
  "p.plans.pid.what_if.critical_path": "Kritischer Pfad",
  "p.plans.pid.what_if.date": "Datum",
  "p.plans.pid.what_if.discard": "Verwerfen",
  "p.plans.pid.what_if.edit": "Bearbeiten",
  "p.plans.pid.what_if.finish_to_finish": "Ende-Ende",
  "p.plans.pid.what_if.finish_to_start": "Ende-Anfang",
  "p.plans.pid.what_if.float": "Puffer",
  "p.plans.pid.what_if.follows": "{name} folgt auf {name2} ({toUpperCase})",
  "p.plans.pid.what_if.joins_the_critical_path":
    "Kommt auf den kritischen Pfad: {join}.",
  "p.plans.pid.what_if.lag_business_days": "Verzögerung (Arbeitstage)",
  "p.plans.pid.what_if.leaves_it": "Verlässt ihn: {join}.",
  "p.plans.pid.what_if.link": "Verknüpfung",
  "p.plans.pid.what_if.new_start": "Neuer Start",
  "p.plans.pid.what_if.no_task_s_dates_or":
    "Keine Daten oder Puffer einer Aufgabe ändern sich.",
  "p.plans.pid.what_if.none_clear_it": "Keine (zurücksetzen)",
  "p.plans.pid.what_if.none_yet": "Noch keine.",
  "p.plans.pid.what_if.now_takes_of_the_day":
    "„{name}“ beansprucht jetzt {units_percent} % des Tages für {hoursText} Arbeit (aus ihrer Planungsregel berechnet).",
  "p.plans.pid.what_if.of_work_has_no_rate":
    "▲ {hoursText} Arbeit haben keinen Satz und werden daher nicht bewertet.",
  "p.plans.pid.what_if.plan": "← Plan",
  "p.plans.pid.what_if.planned_labour_cost":
    "Geplante Personalkosten: {deltaText} ({deltaText2} → {deltaText3}).",
  "p.plans.pid.what_if.predecessor": "Vorgänger",
  "p.plans.pid.what_if.preview": "Vorschau",
  "p.plans.pid.what_if.reason": "Grund",
  "p.plans.pid.what_if.reason_recorded_with_each_change":
    "Grund (wird bei jeder Änderung festgehalten)",
  "p.plans.pid.what_if.remove": "Entfernen",
  "p.plans.pid.what_if.stage_an_edit": "Eine Änderung vormerken",
  "p.plans.pid.what_if.stage_edit": "Änderung vormerken",
  "p.plans.pid.what_if.staged_edits": "Vorgemerkte Änderungen",
  "p.plans.pid.what_if.start_to_finish": "Anfang-Ende",
  "p.plans.pid.what_if.start_to_start": "Anfang-Anfang",
  "p.plans.pid.what_if.successor": "Nachfolger",
  "p.plans.pid.what_if.task": "Aufgabe",
  "p.plans.pid.what_if.tasks_whose_dates_float_or":
    "Aufgaben, deren Daten, Puffer oder kritischer Status sich ändern",
  "p.plans.pid.what_if.try_changes_on_a_copy":
    "Probieren Sie Änderungen an einer Kopie aus. Nichts wird gespeichert, bevor Sie festschreiben.",
  "p.plans.pid.what_if.type": "Typ",
  "p.plans.pid.what_if.what_if": "Was-wäre-wenn",
  "p.plans.pid.what_if.what_if_ppm": "Was-wäre-wenn — PPM",
  "p.plans.pid.work.business_days_of_a_day":
    "{duration_days} Arbeitstage · {units_percent} % eines Tages · {hoursText} Arbeit · {v}",
  "p.plans.pid.work.change": "Ändern",
  "p.plans.pid.work.costs": "Kosten",
  "p.plans.pid.work.day": "Tag",
  "p.plans.pid.work.duration_business_days": "Dauer (Arbeitstage)",
  "p.plans.pid.work.edit_assignment": "Zuweisung bearbeiten",
  "p.plans.pid.work.falls_outside_this_window":
    "{hoursText} liegen außerhalb dieses Zeitfensters.",
  "p.plans.pid.work.hours_per_each_row_adds":
    "Stunden je {grain}; jede Zeile summiert sich zur Arbeit der Aufgabe, die letzte Zeile zur Arbeit des Plans.",
  "p.plans.pid.work.how_long_each_task_takes":
    "Wie lange jede Aufgabe dauert, wie viel vom Tag einer Person sie beansprucht und wie viel Arbeit sich daraus ergibt.",
  "p.plans.pid.work.month": "Monat",
  "p.plans.pid.work.no_assignment_yet_give_a":
    "Noch keine Zuweisung. Geben Sie Dauer und Einheiten an, dann wird die Arbeit berechnet.",
  "p.plans.pid.work.no_task_carries_work_yet":
    "Noch keine Aufgabe enthält Arbeit, daher gibt es nichts zu verteilen.",
  "p.plans.pid.work.not_placed": "(nicht eingeplant)",
  "p.plans.pid.work.percentages": "Prozentwerte",
  "p.plans.pid.work.period": "Zeitraum",
  "p.plans.pid.work.plan": "← Plan",
  "p.plans.pid.work.plan_2": "Plan",
  "p.plans.pid.work.rule": "Regel",
  "p.plans.pid.work.save_assignment": "Zuweisung speichern",
  "p.plans.pid.work.save_contour": "Verlauf speichern",
  "p.plans.pid.work.so_its_work_is_not":
    "▲ {title} ({hoursText}) {reason}, daher ist ihre Arbeit nicht in den Zeiträumen enthalten.",
  "p.plans.pid.work.spread": "Verteilung",
  "p.plans.pid.work.task": "Aufgabe",
  "p.plans.pid.work.tasks": "Aufgaben",
  "p.plans.pid.work.this_plan_has_no_scheduled":
    "Dieser Plan hat noch keine terminierten Aufgaben.",
  "p.plans.pid.work.total": "Gesamt",
  "p.plans.pid.work.units": "Einheiten (%)",
  "p.plans.pid.work.units_of_the_day": "Einheiten (% des Tages)",
  "p.plans.pid.work.week": "Woche",
  "p.plans.pid.work.work_and_assignments": "Arbeit und Zuweisungen",
  "p.plans.pid.work.work_and_assignments_ppm": "Arbeit und Zuweisungen — PPM",
  "p.plans.pid.work.work_hours": "Arbeit (Stunden)",
  "p.plans.pid.work.work_over_time": "Arbeit im Zeitverlauf",
  "p.prioritization.all": "alle",
  "p.prioritization.apply": "Anwenden",
  "p.prioritization.band": "Band",
  "p.prioritization.band_2": "Band",
  "p.prioritization.component": "Komponente",
  "p.prioritization.coverage": "Abdeckung",
  "p.prioritization.coverage_2": "Abdeckung",
  "p.prioritization.explain": "Erklären",
  "p.prioritization.hide": "Ausblenden",
  "p.prioritization.high": "hoch",
  "p.prioritization.loading_the_breakdown": "Aufschlüsselung wird geladen…",
  "p.prioritization.low": "niedrig",
  "p.prioritization.medium": "mittel",
  "p.prioritization.missing_evidence": "Fehlende Belege",
  "p.prioritization.no_evidence_for_any_component":
    "Keine Belege für irgendeine Komponente — dieser Plan ist nicht bewertet.",
  "p.prioritization.no_plans_match_that_filter":
    "Kein Plan entspricht diesem Filter.",
  "p.prioritization.none": "keine",
  "p.prioritization.of_plans_one": "{length} von {scored_of} Plan",
  "p.prioritization.of_plans_other": "{length} von {scored_of} Plänen",
  "p.prioritization.plan": "Plan",
  "p.prioritization.points": "Punkte",
  "p.prioritization.prioritization": "Priorisierung",
  "p.prioritization.prioritization_ppm": "Priorisierung — PPM",
  "p.prioritization.score": "Bewertung",
  "p.prioritization.shows_how_much_of_the":
    "zeigt, wie viel der Bewertung tatsächlich durch Daten gestützt war.",
  "p.prioritization.smart_score_over_roi_strategic":
    "Smart Score über ROI, strategische Ausrichtung, Expertenprüfung, Risiko, Nachfrage, MoSCoW-Priorität und Dynamik. Komponenten ohne Belege werden weggelassen und die übrigen neu normiert —",
  "p.prioritization.unscored": "nicht bewertet",
  "p.prioritization.value": "Wert",
  "p.prioritization.weight": "Gewicht",
  "p.proposals.actions": "Aktionen",
  "p.proposals.currency": "Währung",
  "p.proposals.live": "aktiv:",
  "p.proposals.promote": "befördern",
  "p.proposals.proposal": "Vorschlag:",
  "p.proposals.proposals_ppm": "Vorschläge — PPM",
  "p.proposals.requested": "Beantragt",
  "p.proposals.status": "Status",
  "p.proposals.status_filter": "Statusfilter",
  "p.proposals.target": "Ziel",
  "p.proposals.target_collection": "Zielsammlung",
  "p.proposals.title": "Titel",
  "p.regulator.actual": "Ist",
  "p.regulator.benefits": "Nutzen:",
  "p.regulator.currency": "Währung",
  "p.regulator.gate_decisions": "· Gate-Entscheidungen:",
  "p.regulator.members": "Mitglieder:",
  "p.regulator.names_withheld_abac_mask_obligation":
    "Namen zurückgehalten (ABAC-Maskierungspflicht).",
  "p.regulator.no_budget_lines": "Keine Budgetpositionen.",
  "p.regulator.no_portfolios_recorded": "Keine Portfolios erfasst.",
  "p.regulator.none": "keine",
  "p.regulator.planned": "Geplant",
  "p.regulator.ppm": "{t} — PPM",
  "p.regulator.stage": "Phase: {v}",
  "p.regulator.target_realized": "Soll {money} / realisiert {money2}",
  "p.reports.collection": "Sammlung",
  "p.reports.csv": "CSV",
  "p.reports.fields_comma_separated": "Felder (durch Komma getrennt)",
  "p.reports.name_contains": "Name enthält…",
  "p.reports.reports_ppm": "Berichte — PPM",
  "p.resources.a_multi_skill_booking_counts":
    "Eine Buchung mit mehreren Fähigkeiten zählt voll für jede ihrer Fähigkeiten, daher überschneiden sich die Fähigkeitszeilen und ergeben nicht die Gesamtsumme.",
  "p.resources.by": "Nach",
  "p.resources.demand_fte": "Nachfrage FTE",
  "p.resources.demand_supply_and_gap_by":
    "Nachfrage, Angebot und Lücke nach Zeitraum und {group_by}",
  "p.resources.every_figure_as_a_table": "Alle Zahlen als Tabelle",
  "p.resources.from": "Von",
  "p.resources.gap_fte": "Lücke FTE",
  "p.resources.gap_per_in_fte_supply":
    "Lücke je {period}, in FTE (Angebot minus Nachfrage)",
  "p.resources.loading": "Wird geladen…",
  "p.resources.month": "Monat",
  "p.resources.no_demand_or_supply_in":
    "Keine Nachfrage und kein Angebot in diesem Zeitfenster.",
  "p.resources.not_known": "unbekannt",
  "p.resources.of_which_unassigned": "Davon nicht zugewiesen",
  "p.resources.people": "Personen",
  "p.resources.per": "Je",
  "p.resources.per_by_scope":
    "{periodLabel} · je {period} · nach {group_by} · Bereich: {kind}",
  "p.resources.period": "Zeitraum",
  "p.resources.proposed_fte": "Vorgeschlagene FTE",
  "l.processes.name_problem":
    "Geben Sie dem Prozess einen Namen mit 1 bis 120 Zeichen.",
  "l.processes.steps_problem": "Ein Prozess hat 1 bis {max} Schritte.",
  "l.processes.title_problem":
    "Schritt {n}: Geben Sie einen Titel mit 1 bis 200 Zeichen an.",
  "l.processes.person_problem":
    "Schritt {n}: Die Person muss eine person:- oder worker:-Referenz sein.",
  "l.processes.kind_approve": "Genehmigen",
  "l.processes.kind_todo": "Zu erledigen",
  "l.processes.kind_notify": "Benachrichtigen",
  "l.processes.state_pending": "nicht erreicht",
  "l.processes.state_waiting": "wartend",
  "l.processes.state_completed": "erledigt",
  "l.processes.state_skipped": "übersprungen",
  "l.processes.state_rejected": "abgelehnt",
  "l.processes.state_failed": "fehlgeschlagen",
  "l.processes.run_running": "läuft",
  "l.processes.run_completed": "abgeschlossen",
  "l.processes.run_rejected": "abgelehnt",
  "l.processes.run_cancelled": "abgebrochen",
  "l.processes.run_failed": "fehlgeschlagen",
  "p.processes.processes": "Genehmigungsprozesse",
  "p.processes.processes_ppm": "Genehmigungsprozesse — PPM",
  "p.processes.intro":
    "Ein Prozess ist eine Liste von Schritten, die nacheinander ausgeführt werden: jemanden um Genehmigung bitten, jemandem eine Aufgabe geben oder jemanden informieren. Eine Ablehnung beendet den Lauf und überspringt die folgenden Schritte. Das Bearbeiten eines Prozesses ändert nie einen bereits laufenden Lauf.",
  "p.processes.definitions": "Prozesse",
  "p.processes.none_defined": "Es sind noch keine Prozesse definiert.",
  "p.processes.version": "Version {v}",
  "p.processes.new_process": "Prozess definieren",
  "p.processes.name": "Name",
  "p.processes.step_n": "Schritt {n}",
  "p.processes.kind": "Was er bewirkt",
  "p.processes.person": "Person (person:… oder worker:…)",
  "p.processes.step_title": "Was er besagt",
  "p.processes.add_step": "Schritt hinzufügen",
  "p.processes.remove_step": "Schritt {n} entfernen",
  "p.processes.create": "Prozess definieren",
  "p.processes.defined": "{name} definiert.",
  "p.processes.start_heading": "Lauf von {name} starten",
  "p.processes.subject_kind": "Worum es geht",
  "p.processes.subject_pid": "Seine Kennung",
  "p.processes.start": "Lauf starten",
  "p.processes.started": "Lauf von {name} gestartet.",
  "p.processes.runs": "Ihre Läufe",
  "p.processes.none_runs": "Sie haben noch keine Läufe gestartet.",
  "p.processes.run_of": "{name} für {kind} {pid}",
  "p.processes.status": "Status",
  "p.processes.show_run": "{name} anzeigen",
  "p.processes.hide_run": "{name} ausblenden",
  "p.processes.steps": "Schritte",
  "p.processes.log": "Was geschah",
  "p.processes.complete_step": "Schritt {n} als erledigt markieren",
  "p.processes.cancel": "Lauf abbrechen",
  "p.processes.resume": "Genehmigung erneut prüfen",
  "p.processes.done": "Erledigt.",
  "p.processes.could_not": "Das hat nicht funktioniert: {reason}",
  "p.processes.could_not_load": "Die Prozesse konnten nicht geladen werden",
  "l.tolerance.green": "innerhalb der Toleranz",
  "l.tolerance.amber": "nahe am Limit",
  "l.tolerance.red": "über der Toleranz",
  "l.tolerance.unknown": "nicht messbar",
  "l.tolerance.not_set": "kein Limit festgelegt",
  "l.tolerance.time": "Zeit",
  "l.tolerance.schedule": "Zeitplan gegenüber Baseline",
  "l.tolerance.cost": "Kosten",
  "l.tolerance.risk": "Risiko",
  "l.tolerance.resources": "Ressourcen",
  "l.tolerance.day_one": "{n} Tag",
  "l.tolerance.day_other": "{n} Tage",
  "l.tolerance.score": "Bewertung {n}",
  "l.tolerance.points": "{n} Punkte über 100 %",
  "l.tolerance.field_time": "Limit (Kalendertage Verspätung)",
  "l.tolerance.field_schedule": "Limit (Kalendertage später als die Baseline)",
  "l.tolerance.field_cost": "Limit (Prozent über dem genehmigten Budget)",
  "l.tolerance.field_risk":
    "Limit (höchste akzeptable Risikobewertung, 1 bis 25)",
  "l.tolerance.field_resources": "Limit (Prozentpunkte Überbuchung)",
  "l.tolerance.no_limit": "Weder hier noch darüber ist ein Limit festgelegt.",
  "l.tolerance.could_not_measure": "Konnte nicht gemessen werden.",
  "l.tolerance.zero_tolerance":
    "Null-Toleranz: Jede Abweichung liegt über dem Limit.",
  "l.tolerance.used": "{percent} % des Limits ausgeschöpft",
  "l.tolerance.own": "Für diesen Plan festgelegt",
  "l.tolerance.inherited": "Von einem übergeordneten Plan geerbt",
  "p.issue_settings.title": "Vorgangstypen, Status und Rollen",
  "p.issue_settings.title_ppm": "Vorgangstypen, Status und Rollen — PPM",
  "p.issue_settings.intro":
    "Das eigene Vorgangsvokabular der Organisation. Nur ein Administrator kann es ändern. Ohne Konfiguration gelten die integrierten Typen und Status.",
  "p.issue_settings.types_heading": "Vorgangstypen",
  "p.issue_settings.types_intro":
    "Ein ausgemusterter Typ kann nicht mehr gewählt werden. Vorgänge, die ihn tragen, behalten ihn.",
  "p.issue_settings.key": "Schlüssel",
  "p.issue_settings.label": "Bezeichnung",
  "p.issue_settings.state": "Zustand",
  "p.issue_settings.in_use": "In Verwendung",
  "p.issue_settings.retired": "Ausgemustert",
  "p.issue_settings.retire": "Ausmustern",
  "p.issue_settings.restore": "Wiederherstellen",
  "p.issue_settings.actions": "Aktionen",
  "p.issue_settings.add_type": "Typ hinzufügen",
  "p.issue_settings.roles_heading": "Rollen",
  "p.issue_settings.roles_intro":
    "Die Rollen, die diese Organisation anerkennt, und was jede auf einem Plan mit Rollenmitgliedern erlaubt. Eine Rolle kann nur einschränken, was die Anmeldung einer Person bereits erlaubt. Ein Workflow kann nur eine aufgeführte Rolle nennen, damit ein Übergang nicht auf eine falsch geschriebene beschränkt wird.",
  "p.issue_settings.description": "Beschreibung",
  "p.issue_settings.add_role": "Rolle hinzufügen",
  "p.issue_settings.workflows_heading": "Status und Übergänge",
  "p.issue_settings.workflows_intro":
    "Ein Workflow legt die Status eines Vorgangstyps fest, wer welchen Übergang ausführen darf und welche Felder jeder Status verlangt oder sperrt. Ein Workflow für einen Typ hat Vorrang vor dem für alle Typen. Vorgänge in einem entfernten Status behalten ihn.",
  "p.issue_settings.none_workflows":
    "Es ist kein Vorgangs-Workflow registriert, daher gelten die integrierten Status.",
  "p.issue_settings.wf_name": "Name des Workflows",
  "p.issue_settings.wf_type": "Gilt für",
  "p.issue_settings.wf_all_types": "Alle Typen ohne eigenen Workflow",
  "p.issue_settings.wf_plan": "Ein Plan",
  "p.issue_settings.statuses": "Status",
  "p.issue_settings.category": "Bedeutung",
  "p.issue_settings.initial": "Beginnt hier",
  "p.issue_settings.terminal": "Erfordert eine Lösung",
  "p.issue_settings.add_status": "Status hinzufügen",
  "p.issue_settings.moves": "Erlaubte Übergänge",
  "p.issue_settings.moves_intro":
    "Ohne aufgeführte Übergänge ist jeder erlaubt. Sobald einer aufgeführt ist, gelten nur die aufgeführten.",
  "p.issue_settings.from": "Von",
  "p.issue_settings.to": "Nach",
  "p.issue_settings.move_roles": "Rollen (kommagetrennt; leer für alle)",
  "p.issue_settings.add_move": "Übergang hinzufügen",
  "p.issue_settings.remove": "Entfernen",
  "p.issue_settings.register": "Workflow registrieren",
  "p.issue_settings.withdraw": "Zurückziehen",
  "p.issue_settings.saved": "Gespeichert.",
  "p.issue_settings.could_not_load":
    "Die Vorgangseinstellungen konnten nicht geladen werden.",
  "p.issue_settings.could_not_save":
    "Speichern nicht möglich. Nur ein Administrator kann diese Einstellungen ändern, und jeder Schlüssel muss neu und korrekt gebildet sein.",
  "p.issue_settings.cat_todo": "Nicht begonnen",
  "p.issue_settings.cat_active": "In Arbeit",
  "p.issue_settings.cat_waiting": "Wartend",
  "p.issue_settings.cat_done": "Erledigt",
  "p.issue_settings.translations":
    "Übersetzungen (Sprache=Name, durch Semikolons getrennt)",
  "p.issue_settings.bad_translations":
    "Schreiben Sie jede Übersetzung als Sprache=Name, zum Beispiel cy=Risg.",
  "p.issue_settings.save": "Speichern",
  "p.issue_settings.permissions": "Berechtigungen",
  "p.issue_settings.perm_issue_add": "Vorgänge erfassen",
  "p.issue_settings.perm_issue_edit": "Vorgänge bearbeiten",
  "p.issue_settings.perm_issue_move": "Vorgänge verschieben",
  "p.issue_settings.perm_issue_delete": "Vorgänge entfernen",
  "p.issue_settings.edit": "Bearbeiten",
  "p.issue_settings.save_workflow": "Workflow speichern",
  "p.issue_settings.cancel": "Abbrechen",
  "p.issue_settings.editing": "{name} wird geändert",
  "p.issue_settings.rule_field": "Feld",
  "p.issue_settings.rule_kind": "Regel",
  "p.issue_settings.rule_required": "Pflichtfeld",
  "p.issue_settings.rule_read_only": "Schreibgeschützt",
  "p.issue_settings.rule_roles":
    "Rollen (durch Kommas getrennt; leer für alle)",
  "p.issue_settings.add_rule": "Feldregel hinzufügen",
  "p.issue_settings.field_title": "Titel",
  "p.issue_settings.field_description": "Beschreibung",
  "p.issue_settings.field_severity": "Schweregrad",
  "p.issue_settings.field_priority": "Priorität",
  "p.issue_settings.field_owner_ref": "Verantwortlich",
  "p.issue_settings.field_due_date": "Fälligkeitsdatum",
  "p.issue_settings.field_resolution": "Lösung",
  "p.plan_settings.title": "Vorgangseinstellungen dieses Plans",
  "p.plan_settings.title_ppm": "Vorgangseinstellungen dieses Plans — PPM",
  "p.plan_settings.intro":
    "Wer hier welche Rolle hat, welche Vorgangstypen dieser Plan anbietet und seine eigenen Workflows. Änderungen nimmt eine Administratorin oder ein Administrator vor.",
  "p.plan_settings.back": "Zurück zu den Vorgängen dieses Plans",
  "p.plan_settings.all_settings": "Einstellungen der gesamten Organisation",
  "p.plan_settings.members_heading": "Personen und Rollen",
  "p.plan_settings.members_intro":
    "Sobald hier mindestens eine Person eingetragen ist, brauchen Erfassen, Bearbeiten, Verschieben und Entfernen von Vorgängen auf diesem Plan eine Rolle, die das erlaubt. Eine Rolle kann niemandem mehr geben, als die Anmeldung erlaubt, und sie öffnet keinen privaten Plan.",
  "p.plan_settings.enforced":
    "Vorgangsaktionen auf diesem Plan erfordern eine Rolle.",
  "p.plan_settings.not_enforced":
    "Alle mit Schreibrecht können hier Vorgänge bearbeiten.",
  "p.plan_settings.person": "Benutzer-ID der Person",
  "p.plan_settings.role": "Rolle",
  "p.plan_settings.choose_role": "Rolle wählen",
  "p.plan_settings.add_member": "Person hinzufügen",
  "p.plan_settings.types_heading": "Vorgangstypen auf diesem Plan",
  "p.plan_settings.types_intro":
    "Wählen Sie die Typen, die dieser Plan anbietet. Ohne Auswahl bietet er alle Typen an. Ein Vorgang behält einen nicht gewählten Typ.",
  "p.plan_settings.save_types": "Typen speichern",
  "p.plan_settings.workflows_intro":
    "Ein Workflow hier gilt nur für diesen Plan und hat Vorrang vor dem der Organisation. Ein Workflow für einen Typ hat Vorrang vor dem für alle Typen. Vorgänge in einem entfernten Status behalten ihn.",
  "p.plan_issues.settings_link": "Vorgangseinstellungen dieses Plans",
  "p.plan_issues.problem_fields_required": "Dieser Status verlangt: {fields}.",
  "p.tolerances.tolerances": "Toleranzen",
  "p.tolerances.tolerances_ppm": "Toleranzen — PPM",
  "p.tolerances.plan_link": "Toleranzen",
  "p.tolerances.intro":
    "Wie weit jedes Projekt bei Zeit, Kosten, Risiko und Ressourcen abweichen darf, bevor es nach oben eskaliert werden muss, und wie viel jedes Limits ausgeschöpft ist. Innerhalb des Limits, aber nahe daran, ist gelb; darüber ist rot; ein nicht messbares Limit wird als nicht messbar angezeigt, nie als grün.",
  "p.tolerances.board_intro":
    "Jeder Plan, den Sie lesen dürfen und der eine Toleranz hat, die schlechtesten zuerst.",
  "p.tolerances.none": "Noch kein Plan hat eine Toleranz.",
  "p.tolerances.none_match": "Kein Plan entspricht.",
  "p.tolerances.show": "Anzeigen",
  "p.tolerances.all_statuses": "Alle Farben",
  "p.tolerances.plan": "Plan",
  "p.tolerances.overall": "Gesamt",
  "p.tolerances.truncated":
    "Nur die ersten {n} Pläne wurden gemessen, daher ist diese Liste möglicherweise unvollständig.",
  "p.tolerances.loading": "Wird geladen…",
  "p.tolerances.could_not_load": "Die Toleranzen konnten nicht geladen werden",
  "p.tolerances.back": "← Plan",
  "p.tolerances.board_link": "Alle Toleranzen",
  "p.tolerances.limit": "Limit",
  "p.tolerances.measured": "Gemessen",
  "p.tolerances.how": "Wie gemessen wird",
  "p.tolerances.amber_from": "Gelb ab (Prozent des Limits)",
  "p.tolerances.note": "Hinweis",
  "p.tolerances.save": "Limit festlegen",
  "p.tolerances.remove": "Eigenes Limit dieses Plans entfernen",
  "p.tolerances.saved": "Limit für {dimension} gespeichert.",
  "p.tolerances.removed": "Limit für {dimension} entfernt.",
  "p.tolerances.could_not": "Das hat nicht funktioniert: {reason}",
  "p.tolerances.limit_problem": "Geben Sie das Limit als Zahl an.",
  "l.escalation.open": "offen",
  "l.escalation.acknowledged": "bestätigt",
  "l.escalation.resolved": "gelöst",
  "l.escalation.withdrawn": "zurückgezogen",
  "l.escalation.reason_tolerance": "Eine Toleranz ist überschritten",
  "l.escalation.reason_risk": "Ein Risiko",
  "l.escalation.reason_issue": "Ein Problem",
  "l.escalation.reason_dependency": "Eine Abhängigkeit",
  "l.escalation.reason_decision": "Eine Entscheidung ist nötig",
  "l.escalation.reason_resource": "Ressourcen",
  "l.escalation.reason_other": "Etwas anderes",
  "l.escalation.outcome_decided": "Entschieden",
  "l.escalation.outcome_guidance_given": "Empfehlung gegeben",
  "l.escalation.outcome_rejected": "Abgelehnt",
  "l.escalation.event_raised": "Gemeldet",
  "l.escalation.event_acknowledged": "Bestätigt",
  "l.escalation.event_level_raised": "Eine Ebene höher gebracht",
  "l.escalation.event_resolved": "Gelöst",
  "l.escalation.event_withdrawn": "Zurückgezogen",
  "l.escalation.title_problem":
    "Geben Sie einen Titel mit 1 bis 200 Zeichen an.",
  "l.escalation.dimension_problem":
    "Geben Sie an, welche Toleranz betroffen ist.",
  "l.escalation.level_problem": "Die Ebene ist eine ganze Zahl von 1 bis 9.",
  "l.escalation.owner_problem":
    "Die Person muss eine person:- oder worker:-Referenz sein.",
  "l.escalation.level_to": "Ebene {level}: {to}",
  "l.escalation.level": "Ebene {level}",
  "l.escalation.open_for": "Seit {n} Tagen offen",
  "l.escalation.took": "Dauerte {n} Tage",
  "l.escalation.overdue": "überfällig",
  "l.escalation.automatic": "Automatisch gemeldet",
  "p.escalations.escalations": "Eskalationen",
  "p.escalations.escalations_ppm": "Eskalationen — PPM",
  "p.escalations.plan_link": "Eskalationen",
  "p.task.task": "Aufgabe",
  "p.task.task_ppm": "Aufgabe — PPM",
  "p.task.back": "Zurück zum Projektstrukturplan",
  "p.task.could_not_load": "Die Aufgabe konnte nicht geladen werden.",
  "p.task.loading": "Wird geladen…",
  "p.task.status": "Status",
  "p.task.due": "Fällig",
  "p.task.assignee": "Zugewiesen an",
  "p.task.description": "Beschreibung",
  "p.task.s_todo": "Zu erledigen",
  "p.task.s_in_progress": "In Arbeit",
  "p.task.s_in_review": "In Prüfung",
  "p.task.s_done": "Erledigt",
  "p.task.s_blocked": "Blockiert",
  "p.task.links_intro":
    "Wo diese Aufgabe in anderen Werkzeugen geführt wird, etwa als Jira- oder Linear-Issue. Die Adresse wird gespeichert und angezeigt; sie wird nie abgerufen und nichts wird synchronisiert.",
  "p.task.back_board": "Zurück zum Board",
  "p.task.could_not_load_links":
    "Die Verknüpfungen der Aufgabe mit anderen Aufgaben konnten nicht geladen werden.",
  "p.task.could_not_load_tasks":
    "Die Aufgaben des Plans zum Verknüpfen konnten nicht geladen werden.",
  "p.task.edit_heading": "Aufgabe bearbeiten",
  "p.task.edit_note":
    "Der Status ändert sich auf dem Board. Sprint, Komponente und übergeordnete Aufgabe bleiben, wie sie sind.",
  "p.task.f_title": "Titel",
  "p.task.f_assignee":
    "Zugewiesen an (eine person:- oder worker:-Referenz; leer lassen für niemanden)",
  "p.task.f_points":
    "Story Points (eine ganze Zahl von 0 bis 100; leer lassen für keine)",
  "p.task.f_due": "Fälligkeitsdatum (leer lassen für keines)",
  "p.task.save": "Aufgabe speichern",
  "p.task.saved": "Die Aufgabe wurde gespeichert.",
  "p.task.problem_title_required": "Geben Sie der Aufgabe einen Titel.",
  "p.task.problem_title_too_long": "Der Titel ist zu lang.",
  "p.task.problem_assignee":
    "Geben Sie die zugewiesene Person als person:- oder worker:-Referenz an oder lassen Sie das Feld leer.",
  "p.task.problem_points_whole": "Story Points sind eine ganze Zahl.",
  "p.task.problem_points_range":
    "Story Points müssen zwischen 0 und 100 liegen.",
  "p.task.problem_due": "Das Fälligkeitsdatum ist kein Datum.",
  "p.task.relations_heading": "Verknüpfungen mit anderen Aufgaben",
  "p.task.relations_intro":
    "„Blockiert“ verhindert, dass die andere Aufgabe abgeschlossen wird, solange diese offen ist. „Kommt vor“ verlangt, dass die andere Aufgabe nach dieser fällig ist. Wird eine Aufgabe abgeschlossen, wird auch jede Aufgabe abgeschlossen, die ein Duplikat davon ist. „Verwandt mit“ und „Kopiert von“ dienen nur der Information.",
  "p.task.no_relations":
    "Diese Aufgabe hat keine Verknüpfungen mit anderen Aufgaben.",
  "p.task.rel_related": "Verwandt mit",
  "p.task.rel_blocks": "Blockiert",
  "p.task.rel_blocked_by": "Blockiert durch",
  "p.task.rel_precedes": "Kommt vor",
  "p.task.rel_follows": "Kommt nach",
  "p.task.rel_duplicates": "Duplikat von",
  "p.task.rel_duplicated_by": "Dupliziert durch",
  "p.task.rel_copied_from": "Kopiert von",
  "p.task.rel_copied_to": "Kopiert nach",
  "p.task.other_state": "(jetzt {status})",
  "p.task.other_state_due": "(jetzt {status}, fällig {date})",
  "p.task.remove_link": "Entfernen",
  "p.task.remove_link_named": "Verknüpfung entfernen: {link} {title}",
  "p.task.hidden_links_one":
    "{n} weitere Verknüpfung führt zu einer Aufgabe in einem Plan, den Sie nicht sehen können.",
  "p.task.hidden_links_other":
    "{n} weitere Verknüpfungen führen zu Aufgaben in Plänen, die Sie nicht sehen können.",
  "p.task.add_link_heading": "Mit einer anderen Aufgabe verknüpfen",
  "p.task.f_kind": "Diese Aufgabe",
  "p.task.f_other": "Andere Aufgabe",
  "p.task.choose_task": "Aufgabe wählen",
  "p.task.add_link": "Aufgaben verknüpfen",
  "p.task.link_added": "Die Verknüpfung wurde hinzugefügt.",
  "p.task.link_removed": "Die Verknüpfung wurde entfernt.",
  "p.task.problem_choose_task": "Wählen Sie die andere Aufgabe.",
  "p.task.problem_self":
    "Eine Aufgabe kann nicht mit sich selbst verknüpft werden.",
  "p.task.problem_exists": "Diese Aufgabe hat diese Verknüpfung bereits.",
  "p.task.problem_cycle":
    "Die andere Aufgabe hat diese Verknüpfung zu dieser bereits; die beiden würden aufeinander warten.",
  "p.task.problem_precedes_dates":
    "„Kommt vor“ verlangt, dass die andere Aufgabe nach dieser fällig ist: Ändern Sie zuerst ein Fälligkeitsdatum.",
  "p.task.refused_forbidden":
    "Sie dürfen die Aufgaben dieses Plans nicht ändern.",
  "p.task.refused_gone":
    "Diese Aufgabe existiert nicht mehr, oder Sie können sie nicht sehen; sie wurde neu gelesen.",
  "p.task.refused_link_gone":
    "Diese Verknüpfung war bereits entfernt; die Verknüpfungen wurden neu gelesen.",
  "p.task.refused_link_exists":
    "Diese Verknüpfung besteht bereits; die Verknüpfungen wurden neu gelesen.",
  "p.task.refused_edit_invalid":
    "Der Dienst hat die Änderung nicht angenommen: Prüfen Sie die Felder. Ein neues Fälligkeitsdatum muss außerdem jede „Kommt vor“-Verknüpfung in der richtigen Reihenfolge halten.",
  "p.task.refused_link_invalid":
    "Der Dienst hat die Verknüpfung nicht angenommen: Die andere Aufgabe ist vielleicht entfernt, eine „Blockiert“- oder „Kommt vor“-Verknüpfung darf keinen Kreis bilden, und „Kommt vor“ verlangt, dass die andere Aufgabe nach dieser fällig ist.",
  "p.me.notifications_link": "Meine Benachrichtigungen",
  "p.me.notifications_note":
    "— Ihr Posteingang, worüber Sie informiert werden und wohin Kopien gehen.",
  "p.notifications.title_ppm": "Meine Benachrichtigungen — PPM",
  "p.notifications.heading": "Meine Benachrichtigungen",
  "p.notifications.back": "← Meine Arbeit",
  "p.notifications.intro":
    "Alles hier gehört nur Ihnen: Niemand sonst kann Ihren Posteingang, Ihre Einstellungen oder Ihre Ziele lesen.",
  "p.notifications.loading": "Wird geladen…",
  "p.notifications.inbox_heading": "Posteingang",
  "p.notifications.unread_only": "Nur ungelesene zeigen",
  "p.notifications.unread_count_one": "{n} ungelesene Benachrichtigung.",
  "p.notifications.unread_count_other": "{n} ungelesene Benachrichtigungen.",
  "p.notifications.inbox_tz": "Tage werden in Ihrer Zeitzone gezählt, {tz}.",
  "p.notifications.inbox_empty": "Ihr Posteingang ist leer.",
  "p.notifications.inbox_unread_empty": "Nichts Ungelesenes.",
  "p.notifications.status_unread": "Ungelesen",
  "p.notifications.status_read": "Gelesen",
  "p.notifications.digest_count_one":
    "Tageszusammenfassung von {n} Benachrichtigung",
  "p.notifications.digest_count_other":
    "Tageszusammenfassung von {n} Benachrichtigungen",
  "p.notifications.open_subject": "Den Plan öffnen",
  "p.notifications.mark_read": "Als gelesen markieren",
  "p.notifications.mark_read_named": "Als gelesen markieren: {message}",
  "p.notifications.marked_read": "Als gelesen markiert.",
  "p.notifications.could_not_load_inbox":
    "Ihr Posteingang konnte nicht geladen werden.",
  "p.notifications.could_not_mark":
    "Die Benachrichtigung konnte nicht als gelesen markiert werden.",
  "p.notifications.prefs_heading": "Worüber Sie informiert werden",
  "p.notifications.prefs_intro":
    "Schalten Sie jede Art von Benachrichtigung für den Posteingang und für jede Art von Ziel ein oder aus. Eine Kopie geht nur dann an einen Webhook, einen Chat oder eine E-Mail, wenn Sie unten ein Ziel dieser Art hinzugefügt haben. Eine Tageszusammenfassung fasst die Benachrichtigungen eines Tages zu einer Sache in einer Zeile des Posteingangs zusammen; nichts wird zurückgehalten oder verworfen.",
  "p.notifications.pref_daily": "Tageszusammenfassung im Posteingang",
  "p.notifications.pref_saved": "Ihre Einstellung wurde gespeichert.",
  "p.notifications.pref_reset_done":
    "Zurück zur Voreinstellung: an, sobald sie eintreffen.",
  "p.notifications.reset": "Zurück zur Voreinstellung",
  "p.notifications.reset_named": "Zurück zur Voreinstellung: {kind}",
  "p.notifications.could_not_load_prefs":
    "Ihre Einstellungen konnten nicht geladen werden.",
  "p.notifications.could_not_save_pref":
    "Die Einstellung konnte nicht gespeichert werden.",
  "p.notifications.sinks_heading": "Wohin Kopien gehen",
  "p.notifications.sinks_intro":
    "Ein Ziel erhält eine Kopie jeder Ihrer Benachrichtigungen, nur mit dem Text der Benachrichtigung selbst. Die Adresse ist geheim: Einmal gespeichert, wird sie nur noch gekürzt angezeigt. Ein langsames oder defektes Ziel hält den Posteingang nie auf; ein fehlgeschlagener Versand wird noch viermal über etwa zweieinhalb Stunden wiederholt und dann aufgegeben.",
  "p.notifications.no_sinks":
    "Sie haben keine Ziele; Ihre Benachrichtigungen gehen nur in den Posteingang.",
  "p.notifications.chat_named": "{name}-Chat",
  "p.notifications.sink_sending": "Sendet",
  "p.notifications.sink_paused": "Pausiert",
  "p.notifications.show_deliveries": "Letzte Zustellungen",
  "p.notifications.deliveries_named": "Letzte Zustellungen: {destination}",
  "p.notifications.remove_sink": "Entfernen",
  "p.notifications.remove_sink_named": "Entfernen: {destination}",
  "p.notifications.sink_removed": "Das Ziel wurde entfernt.",
  "p.notifications.deliveries_heading": "Letzte Zustellungen an {destination}",
  "p.notifications.deliveries_note":
    "Warum ein Versand fehlschlug, wird hier nicht gezeigt, da der Grund die geheime Adresse enthalten kann.",
  "p.notifications.no_deliveries": "An dieses Ziel wurde noch nichts gesendet.",
  "p.notifications.attempts_one": "{n} Versuch",
  "p.notifications.attempts_other": "{n} Versuche",
  "p.notifications.delivered_at": "gesendet {date}",
  "p.notifications.next_attempt": "nächster Versuch {date}",
  "p.notifications.close_deliveries": "Zustellungen schließen",
  "p.notifications.add_heading": "Ein Ziel hinzufügen",
  "p.notifications.f_kind": "Art",
  "p.notifications.f_destination_email": "E-Mail-Adresse",
  "p.notifications.f_destination_url":
    "Webadresse (https, zu einem öffentlichen Host)",
  "p.notifications.f_chat_id":
    "Telegram-Chat (seine Nummer oder @ und sein Name)",
  "p.notifications.add_sink": "Das Ziel hinzufügen",
  "p.notifications.sink_added": "Das Ziel wurde hinzugefügt: {destination}.",
  "p.notifications.could_not_load_sinks":
    "Ihre Ziele konnten nicht geladen werden.",
  "p.notifications.could_not_load_deliveries":
    "Die Zustellungen konnten nicht geladen werden.",
  "p.notifications.could_not_save_sink":
    "Das Ziel konnte nicht hinzugefügt werden.",
  "p.notifications.could_not_remove_sink":
    "Das Ziel konnte nicht entfernt werden.",
  "p.notifications.problem_too_many":
    "Sie haben bereits zehn Ziele, das Höchste, was möglich ist: Entfernen Sie zuerst eines.",
  "p.notifications.problem_destination_required":
    "Geben Sie die Adresse an, an die gesendet werden soll.",
  "p.notifications.problem_email_invalid": "Das ist keine E-Mail-Adresse.",
  "p.notifications.problem_url_invalid": "Das ist keine Webadresse.",
  "p.notifications.problem_url_not_https":
    "Die Webadresse muss mit https:// beginnen.",
  "p.notifications.problem_url_credentials":
    "Die Webadresse darf keinen Benutzernamen und kein Passwort enthalten.",
  "p.notifications.problem_url_private":
    "Die Webadresse muss ein öffentlicher Host sein, kein lokaler, privater oder interner.",
  "p.notifications.problem_url_too_long": "Die Webadresse ist zu lang.",
  "p.notifications.problem_chat_id_required":
    "Ein Telegram-Ziel braucht den Chat, in den es schreibt.",
  "p.notifications.refused_forbidden":
    "Sie dürfen diese Benachrichtigungseinstellungen nicht ändern.",
  "p.notifications.refused_note_gone":
    "Diese Benachrichtigung gibt es nicht mehr, oder sie gehört nicht Ihnen; der Posteingang wurde neu gelesen.",
  "p.notifications.refused_sink_gone":
    "Dieses Ziel war bereits entfernt; Ihre Ziele wurden neu gelesen.",
  "p.notifications.refused_not_addressable":
    "Ihre Anmeldeidentität kann keine persönlich adressierten Benachrichtigungen empfangen, daher gibt es hier nichts zu zeigen.",
  "p.notifications.refused_pref_invalid":
    "Der Dienst hat diese Einstellung nicht angenommen; Ihre Einstellungen wurden neu gelesen.",
  "p.notifications.refused_sink_invalid":
    "Der Dienst hat das Ziel nicht angenommen: Es muss eine E-Mail-Adresse oder eine https-Webadresse zu einem öffentlichen Host sein; ein Telegram-Ziel braucht seinen Chat; und es dürfen höchstens zehn sein.",
  "l.enum.notify_kind.approval_decided":
    "Über eine von Ihnen erbetene Genehmigung wurde entschieden",
  "l.enum.notify_kind.approval_requested":
    "Sie werden um eine Genehmigung gebeten",
  "l.enum.notify_kind.auto_watch": "Automatisch verfolgen, woran Sie arbeiten",
  "l.enum.notify_kind.automation": "Eine Regel informiert Sie",
  "l.enum.notify_kind.comment": "Kommentare",
  "l.enum.notify_kind.escalation": "Eskalationen",
  "l.enum.notify_kind.exceptions_digest": "Die wöchentliche Ausnahmenübersicht",
  "l.enum.notify_kind.reschedule_proposed":
    "Ein neuer Termin wird vorgeschlagen",
  "l.enum.notify_kind.review_invited": "Sie werden zu einer Prüfung eingeladen",
  "l.enum.notify_kind.review_submitted": "Eine Prüfung wurde eingereicht",
  "l.enum.notify_kind.scheduled": "Eine geplante Erinnerung",
  "l.enum.notify_kind.status_update_due": "Ein Statusbericht ist fällig",
  "l.enum.notify_kind.task_assigned": "Ihnen wurde eine Aufgabe zugewiesen",
  "l.enum.notify_kind.watch_update": "Etwas, dem Sie folgen, hat sich geändert",
  "l.enum.notify_kind.process_todo":
    "Ein Prozessschritt ist von Ihnen zu erledigen",
  "l.enum.notify_kind.process_notice":
    "Ein Prozessschritt, über den Sie informiert werden",
  "l.enum.notify_kind.break_glass":
    "Ein privater Plan wurde im Notfall geöffnet",
  "l.enum.notify_channel.in_app": "Im Posteingang",
  "l.enum.notify_channel.webhook": "Per signiertem Webhook",
  "l.enum.notify_channel.chat": "Im Chat",
  "l.enum.notify_channel.email": "Per E-Mail",
  "l.enum.sink_kind.email": "E-Mail",
  "l.enum.sink_kind.webhook": "Signierter Webhook",
  "l.enum.delivery_status.pending": "Wartet auf Versand",
  "l.enum.delivery_status.sent": "Gesendet",
  "l.enum.delivery_status.failed": "Aufgegeben",
  "l.enum.delivery_status.skipped":
    "Nicht gesendet: Das Ziel oder die Benachrichtigung gibt es nicht mehr",
  "l.link.kind_docs": "Dokumente",
  "l.link.kind_repo": "Code-Repository",
  "l.link.kind_ticket": "Ticket",
  "l.link.kind_design": "Design",
  "l.link.kind_chat": "Chat",
  "l.link.kind_tracker": "Arbeitstracker",
  "l.link.kind_other": "Sonstiges",
  "l.link.system_other": "Ein anderes Werkzeug",
  "l.fw.method_waterfall": "Wasserfall",
  "l.fw.method_hybrid": "Hybrid",
  "l.fw.method_other": "Sonstiges",
  "l.fw.model_sponsor": "Sponsor entscheidet",
  "l.fw.model_steering_committee": "Lenkungsausschuss",
  "l.fw.model_product_owner": "Product Owner",
  "l.fw.model_delegated": "Delegiert",
  "l.fw.model_consensus": "Konsens",
  "l.fw.model_other": "Sonstiges",
  "p.frameworks.wow": "Arbeitsweise",
  "p.frameworks.wow_ppm": "Arbeitsweise — PPM",
  "p.frameworks.gov": "Governance-Rahmen",
  "p.frameworks.gov_ppm": "Governance-Rahmen — PPM",
  "p.frameworks.intro_wow":
    "Wie das Team arbeitet: Methode, Iterationslänge, WIP-Limit. Ein Plan folgt einer Arbeitsweise, geteilt oder eigen; ohne eine erbt er die des nächsten übergeordneten Plans.",
  "p.frameworks.intro_gov":
    "Wie die Arbeit gesteuert wird: wer entscheidet, wie oft geprüft wird und welchem Eskalationspfad Eskalationen folgen. Ein Plan folgt einem Rahmen, geteilt oder eigen; ohne einen erbt er den des nächsten übergeordneten Plans.",
  "p.frameworks.back": "Zurück zum Plan",
  "p.frameworks.loading": "Wird geladen…",
  "p.frameworks.could_not_load": "Dies konnte nicht geladen werden.",
  "p.frameworks.could_not": "Das war nicht möglich: {reason}",
  "p.frameworks.in_force": "Für diesen Plan gültig",
  "p.frameworks.source_own": "für diesen Plan festgelegt",
  "p.frameworks.source_inherited": "von einem übergeordneten Plan geerbt",
  "p.frameworks.source_none": "keine",
  "p.frameworks.from_plan": "siehe diesen Plan",
  "p.frameworks.scope_plan": "für einen Plan geschrieben",
  "p.frameworks.scope_org": "von der Organisation geteilt",
  "p.frameworks.none": "Nichts ist in Kraft.",
  "p.frameworks.edit": "Bearbeiten",
  "p.frameworks.unfollow": "Nicht mehr folgen (wieder erben)",
  "p.frameworks.follow_heading": "Einer anderen folgen",
  "p.frameworks.no_others": "Es gibt keine andere geteilte zum Folgen.",
  "p.frameworks.choose": "Geteilt oder vorhanden",
  "p.frameworks.follow": "Dieser folgen",
  "p.frameworks.write_new": "Eine neue für diesen Plan schreiben",
  "p.frameworks.followed": "Sie wird jetzt befolgt.",
  "p.frameworks.unfollowed": "Nicht mehr gefolgt; der Plan erbt wieder.",
  "p.frameworks.saved": "Gespeichert.",
  "p.frameworks.new_heading": "Eine neue für diesen Plan",
  "p.frameworks.edit_heading": "Diese bearbeiten",
  "p.frameworks.f_name": "Name",
  "p.frameworks.f_description": "Beschreibung",
  "p.frameworks.save": "Speichern",
  "p.frameworks.cancel": "Abbrechen",
  "p.frameworks.methodology": "Methode",
  "p.frameworks.cadence_days": "Iterationslänge (Tage)",
  "p.frameworks.wip_limit": "WIP-Limit",
  "p.frameworks.ready": "Definition of Ready",
  "p.frameworks.done": "Definition of Done",
  "p.frameworks.ceremonies": "Zeremonien",
  "p.frameworks.decision_model": "Entscheidungsmodell",
  "p.frameworks.review_cadence": "Prüfung alle (Tage)",
  "p.frameworks.reporting_cadence": "Bericht nach oben alle (Tage)",
  "p.frameworks.path": "Eskalationspfad",
  "p.frameworks.rights": "Entscheidungsrechte",
  "p.frameworks.forums": "Gremien",
  "p.frameworks.days": "{n} Tage",
  "p.frameworks.path_level": "Ebene",
  "p.frameworks.path_body": "Wer es erfährt",
  "p.frameworks.path_trigger": "Was es dorthin bringt",
  "p.frameworks.path_add": "Ebene hinzufügen",
  "p.frameworks.path_remove": "Ebene {n} entfernen",
  "p.frameworks.name_problem":
    "Geben Sie einen Namen mit 1 bis 120 Zeichen an.",
  "p.frameworks.number_problem":
    "Tage sind ganze Zahlen von 1 bis 366, das Limit von 1 bis 1000.",
  "p.frameworks.path_problem":
    "Jede Eskalationsebene ist eine andere ganze Zahl von 1 bis 9, mit Angabe, wer es erfährt und was es dorthin bringt.",
  "p.frameworks.confirm_shared_one":
    "Dies ist geteilt: Speichern ändert es auch für {n} anderen Plan.",
  "p.frameworks.confirm_shared_other":
    "Dies ist geteilt: Speichern ändert es auch für {n} andere Pläne.",
  "p.frameworks.confirm_problem_one":
    "Setzen Sie das Häkchen, um die Änderung für {n} anderen Plan zu bestätigen.",
  "p.frameworks.confirm_problem_other":
    "Setzen Sie das Häkchen, um die Änderung für {n} andere Pläne zu bestätigen.",
  "p.frameworks.deleted": "{name} gelöscht.",
  "p.frameworks.delete": "{name} löschen (niemand folgt ihr)",
  "p.frameworks.text_problem": "Eine Definition hat höchstens 10000 Zeichen.",
  "p.frameworks.practice_problem":
    "Jede Zeremonie und jedes Gremium braucht einen Namen und eine Häufigkeit; es sind höchstens 20.",
  "p.frameworks.rights_problem":
    "Jedes Entscheidungsrecht braucht die Entscheidung und wer sie trifft; es sind höchstens 20.",
  "p.frameworks.practice_name": "Name",
  "p.frameworks.practice_cadence": "Wie oft",
  "p.frameworks.practice_purpose": "Wofür",
  "p.frameworks.row_remove": "Zeile {n} entfernen",
  "p.frameworks.ceremony_add": "Zeremonie hinzufügen",
  "p.frameworks.forum_add": "Gremium hinzufügen",
  "p.frameworks.right_decision": "Entscheidung",
  "p.frameworks.right_decides": "Wer entscheidet",
  "p.frameworks.right_consulted": "Wer zuerst gefragt wird",
  "p.frameworks.right_add": "Entscheidungsrecht hinzufügen",
  "p.links.links": "Links",
  "p.links.links_ppm": "Links — PPM",
  "p.links.intro":
    "Wo die Dokumente und die Arbeit dieses Plans liegen, einschließlich seines Jira-, Trello-, Asana-, Linear- oder Monday-Boards. Die Adresse wird gespeichert und angezeigt; sie wird nie abgerufen.",
  "p.links.back": "Zurück zum Plan",
  "p.links.loading": "Wird geladen…",
  "p.links.could_not_load": "Die Links konnten nicht geladen werden.",
  "p.links.could_not": "Das war nicht möglich: {reason}",
  "p.links.none": "Noch keine Links.",
  "p.links.opens_new": "öffnet in einem neuen Tab",
  "p.links.remove": "{label} entfernen",
  "p.links.added": "{label} hinzugefügt.",
  "p.links.removed": "{label} entfernt.",
  "p.links.add_heading": "Link hinzufügen",
  "p.links.add": "Link hinzufügen",
  "p.links.f_label": "Bezeichnung",
  "p.links.f_url": "Adresse (http oder https)",
  "p.links.f_kind": "Art",
  "p.links.f_system": "Externes Werkzeug",
  "p.links.f_id": "Seine Kennung dort",
  "p.links.url_problem":
    "Die Adresse muss mit http:// oder https:// beginnen und einen Host nennen.",
  "p.links.label_problem": "Geben Sie eine Bezeichnung an.",
  "p.links.system_problem": "Ein Tracker-Link muss sein Werkzeug nennen.",
  "p.links.system_kind_problem":
    "Nur ein Tracker- oder Ticket-Link nennt ein Werkzeug.",
  "p.links.id_problem":
    "Eine Kennung braucht ein Werkzeug und enthält keine Leerzeichen.",
  "p.links.domain_problem": "Die Links dieses Werkzeugs liegen auf {domains}.",
  "p.escalations.intro":
    "Eskalieren Sie etwas nach oben, wenn das Projekt es nicht selbst klären kann oder soll. Es wird auf einer Ebene des Eskalationspfads im Governance-Rahmen des Projekts gemeldet, bestätigt, eine Ebene höher gebracht, wenn niemand antwortet, und endet gelöst oder zurückgezogen. Die als verantwortlich genannte Person und die meldende Person werden in ihrem Posteingang informiert (es wird keine E-Mail gesendet).",
  "p.escalations.register_intro":
    "Jede Eskalation in den Plänen, die Sie lesen dürfen.",
  "p.escalations.none": "Noch keine Eskalationen.",
  "p.escalations.none_match": "Keine Eskalation entspricht.",
  "p.escalations.raise_heading": "Eskalation melden",
  "p.escalations.f_title": "Titel",
  "p.escalations.f_reason": "Warum",
  "p.escalations.f_dimension": "Welche Toleranz",
  "p.escalations.f_level": "Ebene (leer für die erste Ebene des Pfads)",
  "p.escalations.f_to": "Eskalieren an (leer, um den Pfad zu verwenden)",
  "p.escalations.f_owner": "Zuständige Person (person:… oder worker:…)",
  "p.escalations.f_respond": "Antwort erbeten bis",
  "p.escalations.f_description": "Was und warum",
  "p.escalations.raise": "Melden",
  "p.escalations.raised": "{title} gemeldet.",
  "p.escalations.acknowledge": "Bestätigen",
  "p.escalations.take_up": "Eine Ebene höher bringen",
  "p.escalations.take_up_note": "Warum es nach oben geht",
  "p.escalations.resolve": "Lösen",
  "p.escalations.outcome": "Ergebnis",
  "p.escalations.response": "Antwort",
  "p.escalations.withdraw": "Zurückziehen",
  "p.escalations.done": "Erledigt.",
  "p.escalations.could_not": "Das hat nicht funktioniert: {reason}",
  "p.error.status_401":
    "Sie sind nicht angemeldet, oder Ihre Sitzung ist abgelaufen.",
  "p.error.status_403": "Dazu sind Sie nicht berechtigt.",
  "p.error.status_404":
    "Das wurde nicht gefunden, oder Sie dürfen es nicht sehen.",
  "p.error.status_409":
    "Das steht im Konflikt mit dem aktuellen Stand: Jemand anderes hat es geändert oder entschieden, oder es ist bereits erledigt.",
  "p.error.status_412":
    "Es hat sich seit dem Laden geändert: Laden Sie neu und versuchen Sie es erneut.",
  "p.error.status_413": "Das ist zu groß.",
  "p.error.status_422":
    "Etwas, das Sie eingegeben haben, wird nicht akzeptiert.",
  "p.error.status_429":
    "Zu viele Anfragen: Warten Sie einen Moment und versuchen Sie es erneut.",
  "p.bridge.open_workforce": "{name} in der Personalplanung öffnen",
  "p.bridge.open_contacts": "{name} in den Kontakten öffnen",
  "p.error.client": "Die Anfrage wurde abgelehnt.",
  "p.error.server":
    "Beim Dienst ist ein Problem aufgetreten: Versuchen Sie es erneut.",
  "p.escalations.note_problem": "Schreiben Sie mindestens 10 Zeichen.",
  "p.escalations.show": "{title} anzeigen",
  "p.escalations.hide": "{title} ausblenden",
  "p.escalations.history": "Verlauf",
  "p.escalations.status": "Status",
  "p.escalations.plan": "Plan",
  "p.escalations.level_col": "Ebene",
  "p.escalations.age": "Alter",
  "p.escalations.respond_by": "Antwort erbeten bis",
  "p.escalations.filter": "Anzeigen",
  "p.escalations.all": "Alle",
  "p.escalations.live": "Noch offen",
  "p.escalations.overdue_count": "{n} überfällig",
  "p.escalations.loading": "Wird geladen…",
  "p.escalations.could_not_load":
    "Die Eskalationen konnten nicht geladen werden",
  "p.escalations.back": "← Plan",
  "p.escalations.register_link": "Alle Eskalationen",
  "p.tolerances.needs_escalation":
    "Dies liegt über der Toleranz, und niemand hat es eskaliert.",
  "p.tolerances.raise_one": "Eskalation melden",
  "p.tolerances.escalate_all": "Jede rote Toleranz eskalieren",
  "p.tolerances.escalate_all_intro":
    "Jede Dimension, die über der Toleranz liegt und noch nicht eskaliert ist, erhält eine Eskalation auf der ersten Ebene des Eskalationspfads. Nichts wird von selbst eskaliert.",
  "p.tolerances.escalated_one": "{n} Eskalation gemeldet.",
  "p.tolerances.escalated_other": "{n} Eskalationen gemeldet.",
  "p.tolerances.escalations_one": "{n} offene Eskalation",
  "p.tolerances.escalations_other": "{n} offene Eskalationen",
  "p.resources.rbs.by_role": "Nach Rolle",
  "p.resources.rbs.show_tasks": "Offene Aufgaben von {role} anzeigen",
  "p.resources.rbs.hide_tasks": "Offene Aufgaben von {role} ausblenden",
  "p.resources.rbs.no_open_tasks": "keine offenen Aufgaben",
  "p.resources.rbs.tasks_in_plans_you_cannot_read":
    "{count} in Plänen, die Sie nicht lesen dürfen, werden nicht angezeigt.",
  "p.resources.rbs.only_the_first_500_tasks":
    "Nur die ersten 500 Aufgaben wurden gelesen, daher ist diese Liste möglicherweise unvollständig.",
  "p.resources.rbs.could_not_load_tasks":
    "Die Aufgaben konnten nicht geladen werden: {reason}",
  "p.resources.rbs.by_skill": "Nach Fähigkeit",
  "p.resources.rbs.demand_by_skill": "Nachfrage nach Fähigkeit",
  "p.resources.rbs.demand_fte": "Nachfrage FTE",
  "p.resources.rbs.no_roles_in_this_window":
    "Keine Rollen in diesem Zeitfenster.",
  "p.resources.rbs.no_skill_demand_in_this":
    "Keine Nachfrage nach Fähigkeiten in diesem Zeitfenster.",
  "p.resources.rbs.not_known": "unbekannt",
  "p.resources.rbs.people": "Personen",
  "p.resources.rbs.resource_breakdown": "Ressourcenstrukturplan",
  "p.resources.rbs.resource_breakdown_ppm": "Ressourcenstrukturplan — PPM",
  "p.resources.rbs.role": "Rolle",
  "p.resources.rbs.roles_with_people_demand_and":
    "Rollen mit Personen, Nachfrage und Angebot",
  "p.resources.rbs.skill": "Fähigkeit",
  "p.resources.rbs.supply_fte": "Angebot FTE",
  "p.resources.rbs.you_can_see_headcounts_only":
    "Sie sehen nur Kopfzahlen, keine Namen.",
  "p.resources.resources": "Ressourcen",
  "p.resources.resources_ppm": "Ressourcen — PPM",
  "p.resources.role": "Rolle",
  "p.resources.roles_and_totals_only": "nur Rollen und Summen",
  "p.resources.roles_asked_for_with_no":
    "Angefragte Rollen ohne bisher zugeordnete Person",
  "p.resources.short_spare_balanced_supply_not":
    "▼ Mangel · ▲ Überschuss · ● ausgeglichen · ? Angebot unbekannt. Die Schattierung zeigt nur, wie groß die Lücke ist.",
  "p.resources.show": "Anzeigen",
  "p.resources.skill": "Fähigkeit",
  "p.resources.skill_demand_only": "Fähigkeit (nur Nachfrage)",
  "p.resources.supply_fte": "Angebot FTE",
  "p.resources.to": "Bis",
  "p.resources.unclassified": "nicht zugeordnet",
  "p.resources.week": "Woche",
  "p.resources.what_the_plans_ask_for":
    "Was die Pläne anfordern im Vergleich zu dem, was die Personen in jeder Rolle leisten können, in Vollzeitäquivalenten. Vorgeschlagene Buchungen werden getrennt geführt und nie mitgezählt.",
  "p.reviews.accept": "Annehmen",
  "p.reviews.accepted": "angenommen",
  "p.reviews.actions": "Aktionen",
  "p.reviews.advance": "Weiterführen",
  "p.reviews.all": "alle",
  "p.reviews.collaborative_review": "Gemeinsame Prüfung",
  "p.reviews.consensus": "Konsens",
  "p.reviews.consensus_for_this_subject": "Konsens zu diesem Gegenstand",
  "p.reviews.decline": "Ablehnen",
  "p.reviews.declined": "abgelehnt",
  "p.reviews.delegate": "Delegieren",
  "p.reviews.delegate_a_specific_item_to":
    "Delegieren Sie einen bestimmten Punkt an die passende Fachperson. Nur wer angenommen hat, kann ein Urteil abgeben, sodass eine unbeantwortete Einladung nie zum Beleg wird.",
  "p.reviews.due": "fällig",
  "p.reviews.e_g_clinical_safety": "z. B. klinische Sicherheit",
  "p.reviews.expertise": "Fachgebiet",
  "p.reviews.expertise_2": "Fachgebiet",
  "p.reviews.expired": "abgelaufen",
  "p.reviews.external": "extern",
  "p.reviews.hold": "Zurückstellen",
  "p.reviews.idea": "Idee",
  "p.reviews.internal": "intern",
  "p.reviews.invited": "eingeladen",
  "p.reviews.no_review_invitations_yet": "Noch keine Prüfungseinladungen.",
  "p.reviews.no_score": "keine Bewertung",
  "p.reviews.person_uuid": "person:<uuid>",
  "p.reviews.plan": "Plan",
  "p.reviews.proposal": "Vorschlag",
  "p.reviews.reject": "Ablehnen",
  "p.reviews.reviewer": "Prüfer",
  "p.reviews.reviewer_2": "Prüfer",
  "p.reviews.reviews_ppm": "Prüfungen — PPM",
  "p.reviews.scope": "Umfang",
  "p.reviews.scope_2": "Umfang",
  "p.reviews.status": "Status",
  "p.reviews.status_2": "Status",
  "p.reviews.subject": "Gegenstand",
  "p.reviews.subject_2": "Gegenstand",
  "p.reviews.subject_pid": "pid des Gegenstands",
  "p.reviews.submitted": "eingereicht",
  "p.reviews.submitted_outstanding_declined_exter":
    "{submitted} eingereicht · {outstanding} ausstehend · {declined} abgelehnt · {external_submitted} extern",
  "p.reviews.uuid": "UUID",
  "p.reviews.verdict": "Urteil",
  "p.reviews.withdraw": "Zurückziehen",
  "p.risk.category": "Kategorie",
  "p.risk.concentration": "Konzentration",
  "p.risk.escalated": "Eskaliert",
  "p.risk.exposure": "Exposition",
  "p.risk.exposure_25_of_estate":
    "— Exposition {exposure} (≥ 25 % des Bestands)",
  "p.risk.heatmap": "Heatmap",
  "p.risk.item": "Element",
  "p.risk.materialized": "Eingetreten",
  "p.risk.max_estate_max_single_risk":
    "— max. Bestand {v}, max. Einzelrisiko {v2}",
  "p.risk.no_open_risks": "Keine offenen Risiken.",
  "p.risk.open_exposure": "Offene Exposition",
  "p.risk.open_risks_estate_open_exposure":
    "offene Risiken · offene Exposition des Bestands",
  "p.risk.overdue_reviews": "Überfällige Prüfungen",
  "p.risk.portfolio": "Portfolio",
  "p.risk.posture_by_portfolio": "Lage nach Portfolio",
  "p.risk.ppm": "{t} — PPM",
  "p.risk.probability_impact": "Wahrscheinlichkeit ↓ / Auswirkung →",
  "p.risk.review_due": "(Prüfung fällig am {review_date})",
  "p.risk.risk": "Risiko",
  "p.risk.top_risks": "Top-Risiken",
  "p.risk.unassigned": "(nicht zugewiesen)",
  "p.scenarios.alignment": "Ausrichtung",
  "p.scenarios.cap": "Obergrenze",
  "p.scenarios.compare": "Vergleichen",
  "p.scenarios.currency": "Währung",
  "p.scenarios.delta_b_a": "Differenz (b − a)",
  "p.scenarios.evaluation": "Bewertung",
  "p.scenarios.feasible": "Machbar",
  "p.scenarios.no": "nein",
  "p.scenarios.open_exposure": "Offene Exposition",
  "p.scenarios.pick": "auswählen…",
  "p.scenarios.planned": "Geplant {currency}",
  "p.scenarios.scenario": "Szenario",
  "p.scenarios.scenarios_ppm": "Szenarien — PPM",
  "p.scenarios.status": "Status",
  "p.scenarios.work_item_pids_space_comma":
    "pids der Arbeitselemente (durch Leerzeichen/Komma getrennt)",
  "p.scenarios.yes": "ja",
  "p.security.exposure": "Exposition",
  "p.security.item": "Element",
  "p.security.late_stage_items_with_no":
    "Elemente in späten Phasen ohne erfasstes Sicherheitsrisiko",
  "p.security.no_security_risks_recorded": "Keine Sicherheitsrisiken erfasst.",
  "p.security.none_every_late_stage_item":
    "Keine — jedes Element in später Phase hat ein erfasstes Sicherheitsrisiko.",
  "p.security.open_exposure": "{note} · offene Exposition {open_exposure}",
  "p.security.owner": "Verantwortlich",
  "p.security.ppm": "{t} — PPM",
  "p.security.risk": "Risiko",
  "p.security.security_risk_register": "Sicherheitsrisikoregister",
  "p.security.status": "Status",
  "p.signin.check_your_email_for_a":
    "Prüfen Sie Ihre E-Mails auf einen Anmeldelink.",
  "p.signin.could_not_send_the_sign":
    "Der Anmeldelink konnte nicht gesendet werden. Bitte versuchen Sie es erneut.",
  "p.signin.email": "E-Mail",
  "p.signin.send_magic_link": "Magic Link senden",
  "p.signin.sign_in": "Anmelden",
  "p.technology.as_of": "Stand {as_of} · {convention}",
  "p.technology.as_of_edges": "Stand {as_of} · {edges} Kanten",
  "p.technology.calendar_days": "{median_lead_days} Kalendertage",
  "p.technology.completed_before_timing_existed_unti":
    "· {undated_completions} vor Einführung der Zeiterfassung abgeschlossen (ohne Zeitangabe)",
  "p.technology.cross_portfolio_edges": "Portfolioübergreifende Kanten",
  "p.technology.delivery_flow": "Lieferfluss",
  "p.technology.dependency_risk": "Abhängigkeitsrisiko",
  "p.technology.dependents": "Abhängige",
  "p.technology.edges_importing_risk_from_a":
    "Kanten, die Risiko von einem roten Vorgänger übernehmen",
  "p.technology.exposure": "Exposition",
  "p.technology.item": "Element",
  "p.technology.kind": "Art",
  "p.technology.median_lead": "Mittlere Durchlaufzeit:",
  "p.technology.milestones_completed": "Abgeschlossene Meilensteine",
  "p.technology.month": "Monat",
  "p.technology.most_depended_on_single_point":
    "Am häufigsten benötigt (Kandidaten für Single Points of Failure)",
  "p.technology.no_dependencies_recorded_yet":
    "Noch keine Abhängigkeiten erfasst.",
  "p.technology.no_tech_debt_risks_recorded":
    "Keine tech_debt-Risiken erfasst.",
  "p.technology.no_tech_tags_recorded_yet": "Noch keine `tech:*`-Tags erfasst.",
  "p.technology.no_timed_completions": "keine zeitlich erfassten Abschlüsse",
  "p.technology.no_timed_completions_in_the":
    "Keine zeitlich erfassten Abschlüsse im Zeitfenster.",
  "p.technology.none_all_edges_stay_within":
    "Keine — alle Kanten bleiben innerhalb eines Portfolios.",
  "p.technology.none_no_red_predecessors": "Keine — keine roten Vorgänger.",
  "p.technology.open_exposure": "{note} · offene Exposition {open_exposure}",
  "p.technology.over_timed_completions":
    "über {timed_completions} zeitlich erfasste Abschlüsse",
  "p.technology.owner": "Verantwortlich",
  "p.technology.ppm": "{t} — PPM",
  "p.technology.predecessor": "Vorgänger",
  "p.technology.predecessor_red": "Vorgänger (rot)",
  "p.technology.ring_votes": "({ring_votes} Ring-Stimmen)",
  "p.technology.risk": "Risiko",
  "p.technology.status": "Status (",
  "p.technology.status_2": "Status",
  "p.technology.successor": "Nachfolger",
  "p.technology.successor_at_risk": "Gefährdeter Nachfolger",
  "p.technology.technical_debt_register": "Register technischer Schulden",
  "p.technology.technology_radar": "Technologie-Radar",
  "p.verify.request_a_new_link": "Neuen Link anfordern",
  "p.verify.sign_in_link": "Anmeldelink",
  "p.milestones.milestone_by_due_date_one":
    "{length} Meilenstein, nach Fälligkeitsdatum",
  "p.milestones.milestone_by_due_date_other":
    "{length} Meilensteine, nach Fälligkeitsdatum",
  "p.okrs.objective_least_progressed_first_one":
    "{length} Ziel, am wenigsten fortgeschrittenes zuerst",
  "p.okrs.objective_least_progressed_first_other":
    "{length} Ziele, am wenigsten fortgeschrittene zuerst",
  "p.plans.pid.board.blocked_calendar_day_one":
    "{blocked_days} Kalendertag blockiert",
  "p.plans.pid.board.blocked_calendar_day_other":
    "{blocked_days} Kalendertage blockiert",
  "p.plans.pid.calendar.holiday_one": "{length} Feiertag: {join}",
  "p.plans.pid.calendar.holiday_other": "{length} Feiertage: {join}",
  "p.plans.pid.costs.for_across_one":
    "{money} für {hoursText} in {entries} Eintrag.",
  "p.plans.pid.costs.for_across_other":
    "{money} für {hoursText} in {entries} Einträgen.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_one":
    "{plans_without_figures} Plan unten hat noch keine Zahlen (keine Baseline oder nichts zu messen) und wird nicht geschätzt.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_other":
    "{plans_without_figures} Pläne unten haben noch keine Zahlen (keine Baseline oder nichts zu messen) und werden nicht geschätzt.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_one":
    "ohne je zurückzugehen abgeschlossen ({rework_count} Rückwärtsschritt). Lesen Sie dies zusammen mit dem Durchsatz: Steigender Durchsatz bei sinkender Ausbeute bedeutet, dass Arbeit an sich selbst zurückgeliefert wird.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_other":
    "ohne je zurückzugehen abgeschlossen ({rework_count} Rückwärtsschritte). Lesen Sie dies zusammen mit dem Durchsatz: Steigender Durchsatz bei sinkender Ausbeute bedeutet, dass Arbeit an sich selbst zurückgeliefert wird.",
  "p.plans.pid.flow.of_items_finish_within_this_one":
    "{percent} der Elemente werden innerhalb dieser Zeit fertig, basierend auf {sample} abgeschlossenem Element",
  "p.plans.pid.flow.of_items_finish_within_this_other":
    "{percent} der Elemente werden innerhalb dieser Zeit fertig, basierend auf {sample} abgeschlossenen Elementen",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_one":
    "{length} Risiko betrifft sowohl Zeitplan als auch Kosten: Jedes tritt einmal ein und wirkt auf beides.",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_other":
    "{length} Risiken betreffen sowohl Zeitplan als auch Kosten: Jedes tritt einmal ein und wirkt auf beides.",
  "p.plans.pid.leveling.apply_move_one": "{length} Verschiebung anwenden",
  "p.plans.pid.leveling.apply_move_other": "{length} Verschiebungen anwenden",
  "p.plans.pid.leveling.business_day_one": "{delay_days} Arbeitstag",
  "p.plans.pid.leveling.business_day_other": "{delay_days} Arbeitstage",
  "p.plans.pid.leveling.other_task_would_be_pushed_one":
    "{length} weitere Aufgabe würde dadurch verschoben",
  "p.plans.pid.leveling.other_task_would_be_pushed_other":
    "{length} weitere Aufgaben würden dadurch verschoben",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_one":
    "{hidden} Checkliste zu Personen wird Ihnen nicht angezeigt.",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_other":
    "{hidden} Checklisten zu Personen werden Ihnen nicht angezeigt.",
  "p.plans.pid.schedule_health.preset_thresholds_task_one":
    "· Voreinstellung {preset} (Schwellenwerte: {source}) · {tasks} Aufgabe",
  "p.plans.pid.schedule_health.preset_thresholds_task_other":
    "· Voreinstellung {preset} (Schwellenwerte: {source}) · {tasks} Aufgaben",
  "p.plans.pid.schedule_health.task_one": "{length} Aufgabe",
  "p.plans.pid.schedule_health.task_other": "{length} Aufgaben",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_one":
    "{title}: {probability_percent} % Wahrscheinlichkeit, {impact_days} Arbeitstag hinzuzufügen",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_other":
    "{title}: {probability_percent} % Wahrscheinlichkeit, {impact_days} Arbeitstage hinzuzufügen",
  "p.plans.pid.wbs.move_up": "{title} nach oben verschieben",
  "p.plans.pid.wbs.move_down": "{title} nach unten verschieben",
  "p.plans.pid.wbs.moved": "{title} verschoben.",
  "p.plans.pid.wbs.truncated":
    "{shown} von {total} Aufgaben werden angezeigt: Dieser Plan ist zu groß, um ihn vollständig aufzulisten, daher ist die Gliederung unten unvollständig.",
  "p.plans.pid.critical_path.too_large":
    "Dieser Plan ist zu groß, um ihn in einer Antwort vollständig zu terminieren.",
  "p.plans.pid.critical_path.show_critical_only":
    "Nur den kritischen Teil anzeigen",
  "p.plans.pid.critical_path.critical_only_notice":
    "Es werden nur die kritischen und fast kritischen Aufgaben angezeigt; der Rest des Plans ist ausgelassen.",
  "p.plans.pid.earned_value.names_truncated":
    "Dieser Plan ist sehr groß: Die Zahlen umfassen jede Aufgabe, aber einige Aufgabennamen unten fehlen möglicherweise.",
  "p.plans.pid.wbs.could_not_move":
    "Die Aufgabe konnte nicht verschoben werden: {reason}",
  "p.plans.pid.wbs.sub_task_one": "({children} Teilaufgabe)",
  "p.plans.pid.wbs.sub_task_other": "({children} Teilaufgaben)",
  "p.plans.pid.wbs.links": "Links",
  "p.plans.pid.wbs.link_one": "· {links} Link",
  "p.plans.pid.wbs.link_other": "· {links} Links",
  "p.common.on": "ein",
  "p.common.off": "aus",
  "p.common.pre_gate": "vor dem Gate",
  "p.common.role_word": "Rolle",
  "p.common.person_word": "Person",
  "c.NetworkDiagram.task_fallback": "Aufgabe",
  "c.NetworkDiagram.task_summary":
    "{title}, {early_start} bis {early_finish}, {total_float} Arbeitstage Puffer",
  "c.NetworkDiagram.task_summary_critical":
    "{title}, {early_start} bis {early_finish}, {total_float} Arbeitstage Puffer, auf dem kritischen Pfad",
  "c.NetworkDiagram.lag_one": ", Verzögerung {lag_days} Arbeitstag",
  "c.NetworkDiagram.lag_other": ", Verzögerung {lag_days} Arbeitstage",
  "c.KpiCard.direction_higher": "höher",
  "c.KpiCard.direction_lower": "niedriger",
  "c.CalendarMonth.legend":
    "– kein Arbeitstag   ✕ Feiertag   A erfasste Abwesenheit",
  "c.CalendarMonth.legend_editable":
    " · wählen Sie einen Arbeitstag, um einen Feiertag zu markieren oder zu entfernen",
  "c.CostRiskView.run_summary":
    "Startwert {seed} · {iterations} Iterationen · {distribution}",
  "c.CostRiskView.dist_pert": "PERT",
  "c.CostRiskView.dist_triangular": "Dreiecksverteilung",
  "c.CostRiskView.risks_without_cost_one":
    "{count} Risiko im Register hat keine Kostenfelder und ist hier nicht enthalten.",
  "c.CostRiskView.risks_without_cost_other":
    "{count} Risiken im Register haben keine Kostenfelder und sind hier nicht enthalten.",
  "c.CumulativeFlow.hide_the_numbers": "Zahlen ausblenden",
  "c.CumulativeFlow.show_the_numbers": "Zahlen anzeigen",
  "c.CostRiskView.scope_all_one": "Alle {plans} Plan, den Sie lesen können",
  "c.CostRiskView.scope_all_other": "Alle {plans} Pläne, die Sie lesen können",
  "c.CostRiskView.scope_this_plan": "Dieser Plan",
  "c.MilestoneCard.reason": "Grund",
  "c.MilestoneCard.reason_required_open":
    " (erforderlich: Aufgaben, auf die er wartet, sind noch offen)",
  "c.MilestoneCard.reason_optional": " (optional)",
  "c.VarianceTable.matched_of_baselined":
    "{matched} von {baseline_tasks} Aufgaben aus dem Basisplan sind noch im Plan:",
  "c.VarianceTable.slipped_pulled_on":
    "{slipped} verschoben, {pulled_in} vorgezogen, {on_baseline} im Basisplan{unmeasurable}.",
  "c.VarianceTable.cannot_be_measured": ", {unmeasurable} nicht messbar",
  "c.VarianceTable.added_removed":
    "Seitdem {added} hinzugefügt und {removed} entfernt.",
  "c.VarianceTable.finish_later_one":
    "Das Planende liegt {days} Arbeitstag später als im Basisplan{note}.",
  "c.VarianceTable.finish_later_other":
    "Das Planende liegt {days} Arbeitstage später als im Basisplan{note}.",
  "c.VarianceTable.finish_earlier_one":
    "Das Planende liegt {days} Arbeitstag früher als im Basisplan{note}.",
  "c.VarianceTable.finish_earlier_other":
    "Das Planende liegt {days} Arbeitstage früher als im Basisplan{note}.",
  "c.VarianceTable.check_removed":
    " (prüfen Sie die entfernten Aufgaben: Arbeit zu löschen ist keine Verbesserung)",
  "p.plans.pid.critical_path.finishes": "Endet am {date}.",
  "p.plans.pid.critical_path.nothing_scheduled": "Noch nichts terminiert.",
  "p.plans.pid.critical_path.calendar_label": "Kalender: {calendar}.",
  "p.plans.pid.critical_path.calendar_plan": "der dieses Plans",
  "p.plans.pid.critical_path.calendar_default":
    "Standard (jeder Tag ist Arbeitstag)",
  "p.plans.pid.critical_path.more_not_shown": " (weitere nicht angezeigt)",
  "p.plans.pid.critical_path.pushed_later":
    "Seine Vorgänger verschieben ihn um {days} Arbeitstage gegenüber der Planung.",
  "p.plans.pid.earned_value.manual_percent_banner":
    "Ein Teil dieses Werts beruht auf einem von Personen eingegebenen Prozentsatz, nicht auf erledigter Arbeit: {tasks}.",
  "p.plans.pid.earned_value.a_task": "eine Aufgabe",
  "p.plans.pid.earned_value.overrun": " (Überschreitung)",
  "p.plans.pid.earned_value.target_budget": "das Budget",
  "p.plans.pid.earned_value.target_forecast": "die Prognose",
  "p.plans.pid.earned_value.lights_use":
    "Die Ampeln verwenden {which}: {values} (Basispunkte).",
  "p.plans.pid.earned_value.thresholds_default": "die Standardschwellenwerte",
  "p.plans.pid.earned_value.thresholds_deployment":
    "die Schwellenwerte dieser Installation",
  "p.plans.pid.earned_value.rollup_money":
    "Budget {bac}; geplant {pv}; erwirtschaftet {ev}; ausgegeben {ac}{unknown}.",
  "p.plans.pid.earned_value.cost_unknown_for":
    " (Kosten unbekannt für {plans} Pläne)",
  "p.plans.pid.earned_value.cpi_value": "CPI {value}",
  "p.plans.pid.earned_value.cpi_not_stated":
    "nicht angegeben: Kosten sind für einige Pläne unbekannt",
  "p.plans.pid.earned_value.weight_points": ", {weight} Punkte",
  "p.plans.pid.critical_path.moved_line":
    "{title}: {moved}{finish}{start} Arbeitstage",
  "p.plans.pid.critical_path.moved_both": "Start und Ende",
  "p.plans.pid.critical_path.moved_start": "Start",
  "p.plans.pid.critical_path.moved_finish": "Ende",
  "p.plans.pid.critical_path.moved_finish_by": " Ende {days}",
  "p.plans.pid.critical_path.moved_start_by": " Start {days}",
  "p.plans.pid.flow.overdue_one":
    "{count} Element hat die Erwartung bereits überschritten. Ein alterndes Element ist das Einzige auf dieser Seite, dessen Ergebnis Sie noch ändern können.",
  "p.plans.pid.flow.overdue_other":
    "{count} Elemente haben die Erwartung bereits überschritten. Ein alterndes Element ist das Einzige auf dieser Seite, dessen Ergebnis Sie noch ändern können.",
  "p.plans.pid.flow.classified_with": "Klassifiziert mit {map}:",
  "p.plans.pid.flow.map_override": "einer Überschreibung dieser Installation",
  "p.plans.pid.flow.map_default": "der Standardzuordnung",
  "p.plans.pid.governance.proposed_not_counted":
    "vorgeschlagen{requested} — nicht gezählt",
  "p.plans.pid.governance.requested": ", beantragt",
  "p.plans.pid.joint_risk.run_summary":
    "{currency} · Startwert {seed} · {iterations} Iterationen · {distribution} · Kalender {calendar}",
  "p.plans.pid.joint_risk.dist_pert": "PERT",
  "p.plans.pid.joint_risk.dist_triangular": "Dreiecksverteilung",
  "p.plans.pid.leveling.applied_one":
    "{ok} von {requested} Verschiebung angewendet{refused}.",
  "p.plans.pid.leveling.applied_other":
    "{ok} von {requested} Verschiebungen angewendet{refused}.",
  "p.plans.pid.leveling.applied_refused": "; {refused} abgelehnt",
  "p.plans.pid.leveling.proposal_summary":
    "Kalender: {calendar} · Auslastung {load} % je Aufgabe · kritische Aufgaben {critical} verschoben werden",
  "p.plans.pid.leveling.may": "dürfen",
  "p.plans.pid.leveling.may_not": "dürfen nicht",
  "p.plans.pid.onboarding.due": ", fällig am {date}",
  "p.plans.pid.schedule_risk.risks_without_schedule_one":
    "{count} Risiko im Register hat keine Terminfelder und ist hier nicht enthalten.",
  "p.plans.pid.schedule_risk.risks_without_schedule_other":
    "{count} Risiken im Register haben keine Terminfelder und sind hier nicht enthalten.",
  "p.resources.from_date": "ab {date}",
  "p.resources.to_date": "bis {date}",
  "p.kpis.none": "Noch keine KPIs.",
  "p.kpis.none_with_status": "Noch keine KPIs mit diesem Status.",
  "p.milestones.none":
    "Noch keine Meilensteine. Fügen Sie sie auf der Governance-Seite eines Plans hinzu.",
  "p.milestones.none_with_status":
    "Noch keine Meilensteine mit diesem Status. Fügen Sie sie auf der Governance-Seite eines Plans hinzu.",
  "p.okrs.none": "Noch keine Ziele.",
  "p.okrs.none_for_period": "Noch keine Ziele für {period}.",
  "p.me.calendar.own_set": "Ihr eigener Kalender ist festgelegt.",
  "p.me.calendar.plan_applies":
    "Kein persönlicher Kalender: Für Sie gilt der Kalender des Plans.",
  "p.me.calendar.leave_counts":
    "Für Sie erfasster Urlaub zählt bereits als freie Tage.",
  "p.documents.pid.page_title": "{title} — PPM",
  "p.documents.pid.document_fallback": "Dokument",
  "p.plans.pid.page_title": "{name} — Main X",
  "p.plans.pid.plan_fallback": "Plan",
  "p.plans.pid.custom_scheme": "Benutzerdefiniert({name})",
  "p.plans.pid.calendar.own_calendar":
    "Dieser Plan hat einen eigenen Kalender.",
  "p.plans.pid.calendar.no_calendar":
    "Kein Kalender festgelegt: Jeder Tag ist Arbeitstag.",
  "p.plans.pid.calendar.each_person_can_set":
    "Jede Person kann auch einen eigenen festlegen (",
  "p.reviews.consensus_line":
    "Durchschnittswert: {mean} · Mehrheit: {majority} · {state}",
  "p.reviews.no_majority": "keine Mehrheit",
  "p.reviews.everyone_answered": "alle haben geantwortet",
  "p.reviews.still_waiting": "Antworten ausstehend",
  "l.announcement_view.just_now": "gerade eben",
  "l.announcement_view.min_ago": "vor {minutes} Min.",
  "l.announcement_view.h_ago": "vor {hours} Std.",
  "l.announcement_view.calendar_days_ago": "vor {days} Kalendertagen",
  "l.announcement_view.scheduled": "geplant",
  "l.announcement_view.expired": "Abgelaufen",
  "l.api_ppm.task": "Aufgabe {uid}",
  "l.api_ppm.the_project_calendar_comes_across":
    " Der Projektkalender wird übernommen.",
  "l.api_tba.calendar_days": "{toFixed} Kalendertage",
  "l.api_tba.work_in_progress_is_growing": "Laufende Arbeit nimmt zu",
  "l.api_tba.steady_state": "Stabiler Zustand",
  "l.api_tba.queue_draining": "Warteschlange leert sich",
  "l.api_tba.not_enough_data": "Nicht genügend Daten",
  "l.capacity_view.committed_against_a_capacity_as":
    "{committed} % zugesagt bei einer Kapazität von nur {capacity} %",
  "l.capacity_view.committed": "{committed} % zugesagt",
  "l.capacity_view.to": " bis {to}",
  "l.capacity_view.over_capacity_from_asked_available":
    "über der Kapazität ab {from}{v} ({demand} % angefragt, {capacity} % verfügbar)",
  "l.capacity_view.capacity_undeclared":
    "an einigen Tagen mit Arbeit ist keine Kapazität angegeben: weder über noch innerhalb der Kapazität",
  "l.capacity_view.over_capacity_from_asked_undeclared":
    "über der Kapazität ab {from}{v} ({demand} angefragt, mehr als jede Kapazität sein kann)",
  "l.capacity_view.asked_undeclared":
    "{from}{v}: {demand} angefragt, mehr als jede Kapazität sein kann",
  "l.capacity_view.over_capacity": "über der Kapazität",
  "l.capacity_view.committing_it_would_put_them":
    "; eine Zusage würde die Kapazität überschreiten",
  "l.capacity_view.proposed_not_counted":
    "{percent} % vorgeschlagen, nicht gezählt{v}",
  "l.capacity_view.asked_available":
    "{from}{v}: {demand} % angefragt, {capacity} % verfügbar",
  "l.ceo_view.no_portfolios_to_report_on":
    "Noch keine Portfolios für einen Bericht.",
  "l.ceo_view.portfolios_exist_but_hold_no":
    "Es gibt Portfolios, aber sie enthalten noch keine Pläne.",
  "l.ceo_view.of_plans_are_red_amber":
    "{red} von {n} Plänen sind rot, {amber} gelb, {green} grün.",
  "c.CalendarMonth.not_a_business_day": "kein Arbeitstag",
  "c.CalendarMonth.business_day": "Arbeitstag",
  "c.KpiCard.that_did_not_work": "Das hat nicht funktioniert",
  "c.KpiCard.could_not_load_the_history": "Verlauf konnte nicht geladen werden",
  "c.KpiCard.delete_the_kpi_its_readings":
    "KPI „{name}“ löschen? Die Messwerte bleiben erhalten.",
  "c.LineRange.could_not_save": "Speichern nicht möglich",
  "c.MilestoneCard.that_did_not_work": "Das hat nicht funktioniert",
  "c.MilestoneCard.could_not_load_the_plan":
    "Aufgaben des Plans konnten nicht geladen werden",
  "c.MilestoneCard.could_not_load_the_history":
    "Verlauf konnte nicht geladen werden",
  "c.ObjectiveCard.that_did_not_work": "Das hat nicht funktioniert",
  "c.ObjectiveCard.delete_the_objective_and_its":
    "Ziel „{title}“ und seine Schlüsselergebnisse löschen?",
  "c.ObjectiveCard.delete_the_key_result":
    "Schlüsselergebnis „{title}“ löschen?",
  "c.RaciMatrix.accountable_moved_from_to_a":
    "Rechenschaftspflicht von {shortRef} auf {shortRef2} übertragen: Eine Aufgabe hat genau eine verantwortliche Person.",
  "c.RiskInputs.could_not_load_the_plan":
    "Aufgaben des Plans konnten nicht geladen werden",
  "c.RiskInputs.could_not_save": "Speichern nicht möglich",
  "c.RiskInputs.chance": "{probability_percent} % Wahrscheinlichkeit",
  "c.RiskInputs.adds_business_days_to_a":
    "verlängert eine Aufgabe um {schedule_impact_days} Arbeitstage",
  "c.RiskInputs.adds": "erhöht um {toFixed} {cost_currency}",
  "c.RiskInputs.not_used_in_the_simulations":
    "Wird in den Simulationen nicht verwendet",
  "l.cpm_view.cannot_meet_its_constraint_or":
    "Kann Einschränkung oder Frist nicht einhalten: {plural} zu wenig",
  "l.cpm_view.on_the_critical_path_any":
    "Auf dem kritischen Pfad: Jede Verzögerung verschiebt das Ende",
  "l.cpm_view.of_float_near_critical": "{d} Puffer (fast kritisch)",
  "l.cpm_view.of_float": "{d} Puffer",
  "l.cpm_view.baselined_to": ". Basisplan {start} bis {finish}",
  "l.cpm_view.to": "{v}: {early_start} bis {early_finish}. {floatLabel}{v2}",
  "l.cpm_view.has_no_duration_yet": "hat noch keine Dauer",
  "l.cpm_view.has_no_start_date_and":
    "hat kein Startdatum, und nichts davor ist terminiert",
  "l.cpm_view.has_a_date_outside_the":
    "hat ein Datum außerhalb des unterstützten Bereichs",
  "l.cpm_view.slipped": "Verschoben",
  "l.cpm_view.finishes_later_than_baselined":
    "endet {d} später als im Basisplan",
  "l.cpm_view.pulled_in": "Vorgezogen",
  "l.cpm_view.finishes_earlier_than_baselined":
    "endet {d} früher als im Basisplan",
  "l.cpm_view.on_baseline": "Im Basisplan",
  "l.cpm_view.finishes_when_it_was_baselined":
    "endet wie im Basisplan vorgesehen am",
  "l.cpm_view.added": "Hinzugefügt",
  "l.cpm_view.not_in_the_baseline_so":
    "nicht im Basisplan, daher keine Verschiebung",
  "l.cpm_view.removed": "Entfernt",
  "l.cpm_view.was_in_the_baseline_and":
    "war im Basisplan und ist entfallen; die Termine aus dem Basisplan bleiben erhalten",
  "l.cpm_view.cannot_say": "Nicht bestimmbar",
  "l.cpm_view.it_had_no_schedule_when":
    "hatte beim Erstellen des Basisplans keinen Termin",
  "l.cpm_view.it_has_no_schedule_now": "hat derzeit keinen Termin",
  "l.document_view.version": "Version {version} — {v}",
  "l.document_view.since": "{base} seit {effective_from}",
  "l.document_view.by_version": " durch Version {version}",
  "l.document_view.on": "{base} am {superseded_on}{v}",
  "l.document_view.nothing_in_force_yet": "Noch nichts in Kraft",
  "l.document_view.version_in_force": "Version {in_force_today} in Kraft",
  "l.ev_view.nothing_was_planned_to_be":
    "bisher war nichts zur Erledigung geplant",
  "l.ev_view.no_cost_has_been_recorded": "bisher wurden keine Kosten erfasst",
  "l.ev_view.the_baseline_plans_no_value":
    "der Basisplan sieht keinen Wert vor",
  "l.ev_view.not_available": "nicht verfügbar",
  "l.ev_view.schedule_no_ratio": "Termin: kein Verhältnis, {absentText}.",
  "l.ev_view.ahead_of_schedule": "Vor dem Zeitplan",
  "l.ev_view.behind_schedule": "Hinter dem Zeitplan",
  "l.ev_view.on_schedule": "Im Zeitplan",
  "l.ev_view.spi": "{word}: SPI {ratioText}, {gap}.",
  "l.ev_view.cost_no_ratio": "Kosten: kein Verhältnis, {absentText}.",
  "l.ev_view.under_budget": "Unter Budget",
  "l.ev_view.over_budget": "Über Budget",
  "l.ev_view.on_budget": "Im Budget",
  "l.ev_view.earned_schedule_no_ratio":
    "Earned Schedule: kein Verhältnis, {absentText}.",
  "l.ev_view.by_earned_schedule_the_work":
    "Nach Earned Schedule ist die Arbeit {word}: Sie hat in {days} erreicht, was der Plan nach {days2} erwartete (SPI(t) {ratioText}).",
  "l.ev_view.finished_work_counts_for_its":
    "erledigte Arbeit zählt mit ihren Story Points",
  "l.ev_view.only_finished_work_counts": "nur erledigte Arbeit zählt",
  "l.ev_view.by_story_points": "nach Story Points",
  "l.ev_view.counted_by_a_person_entered":
    "{length} nach von Personen eingegebenem Prozentsatz gezählt (manuell)",
  "l.ev_view.with_no_points_carry_no":
    "{length} ohne Punkte haben kein Gewicht",
  "l.ev_view.no_actual_cost_is_recorded": "keine Ist-Kosten erfasst",
  "l.ev_view.the_ratio_it_divides_by":
    "das Verhältnis, durch das geteilt wird, ist noch nicht definiert",
  "l.ev_view.no_estimate_to_complete_has":
    "es wurde keine Restschätzung eingegeben",
  "l.ev_view.the_budget_is_already_spent": "das Budget ist bereits ausgegeben",
  "l.ev_view.no_cost_remains": "keine Kosten verbleiben",
  "l.ev_view.against_the_money_is_already":
    "Gegenüber {target}: Das Geld ist bereits ausgegeben, daher kann keine Effizienz die verbleibende Arbeit darin liefern.",
  "l.ev_view.against": "Gegenüber {target}: {amountAbsentText}.",
  "l.ev_view.against_the_remaining_work_must":
    "Gegenüber {target}: Die verbleibende Arbeit muss {ratioText}-mal so effizient erledigt werden wie die bisherige (TCPI {ratioText2}), was eine höhere Effizienz erfordert.",
  "l.ev_view.against_the_remaining_work_can":
    "Gegenüber {target}: Die verbleibende Arbeit kann mit {ratioText} der bisherigen Kosteneffizienz erledigt werden (TCPI {ratioText2}).",
  "l.ev_view.earned": "{date}: erwirtschaftet {ev_minor}",
  "l.ev_view.spent": "{date}: ausgegeben {ac_minor}",
  "l.import_view.would_create": "Würde erstellen",
  "l.import_view.created": "Erstellt",
  "l.import_view.and": "{verb} {plural} und {upd} {tasks_updated}{tail}.",
  "l.import_view.preview_the_import": "Vorschau des Imports",
  "l.import_view.import": "{plural} importieren",
  "l.kpi_view.target_or_more_red_at":
    "Ziel {target_value} oder mehr; rot bei {red_value} oder weniger{u}",
  "l.kpi_view.target_or_less_red_at":
    "Ziel {target_value} oder weniger; rot bei {red_value} oder mehr{u}",
  "l.kpi_view.no_reading_within_twice_its":
    "Kein Messwert innerhalb des doppelten Intervalls ({cadence})",
  "l.kpi_view.no_reading_yet": "noch kein Messwert",
  "l.milestone_view.there_is_no_forecast": "Es gibt keine Prognose.",
  "l.milestone_view.some_open_tasks_cannot_be":
    " (einige offene Aufgaben lassen sich nicht einplanen, daher ist dies evtl. zu früh)",
  "l.milestone_view.forecast_to_finish_after_it":
    "Voraussichtliches Ende {forecast}, {businessDays} nach der Fälligkeit ({due}){partial}.",
  "l.milestone_view.forecast_to_finish_on_the":
    "Voraussichtliches Ende zum Fälligkeitsdatum, {due}{partial}.",
  "l.milestone_view.forecast_to_finish_before_it":
    "Voraussichtliches Ende {forecast}, {businessDays} vor der Fälligkeit ({due}){partial}.",
  "l.milestone_view.of_them_later": ", davon {later_moves} später",
  "l.milestone_view.back_on_its_original_date":
    "wieder am ursprünglichen Datum",
  "l.milestone_view.later_than_the_original":
    "{calendarDays} später als ursprünglich {original_due}",
  "l.milestone_view.earlier_than_the_original":
    "{calendarDays} früher als ursprünglich {original_due}",
  "l.milestone_view.originally_due_now":
    "Ursprünglich fällig {original_due}; {times}{later}; jetzt {net}.",
  "l.milestone_view.on": " am {achieved_on}",
  "l.milestone_view.achieved_every_task_it_waits":
    "Erreicht{on}: Jede Aufgabe, auf die er wartet, ist erledigt.",
  "l.milestone_view.reason": " Grund: {done_reason}",
  "l.milestone_view.marked_done": "Als erledigt markiert{on}.{why}",
  "l.milestone_view.due": "{name}: {toLowerCase}, fällig {due}{state}",
  "l.my_work_view.this_is_already_late_for":
    "Dies ist für den Plan bereits verspätet ({days} im Rückstand).",
  "l.my_work_view.on_the_critical_path": "Auf dem kritischen Pfad",
  "l.my_work_view.of_float": "{days} Puffer",
  "l.okr_view.not_measured": "Nicht gemessen",
  "l.okr_view.not_measurable": "Nicht messbar ({reason}).",
  "l.okr_view.of": "{current} von {target}{u}",
  "l.okr_view.confidence_not_part_of_the":
    "Zuversicht {confidence} % (nicht Teil der Bewertung)",
  "l.onboarding_view.give_the_date_it_takes":
    "Geben Sie das Datum des Inkrafttretens an.",
  "l.onboarding_view.give_the_person_as_person":
    "Geben Sie die Person als person:… (oder worker:…) an.",
  "l.onboarding_view.say_what_access_in_words":
    "Beschreiben Sie den Zugang in Worten.",
  "l.onboarding_view.choose_granted_or_revoked":
    "Wählen Sie „erteilt“ oder „entzogen“.",
  "l.onboarding_view.give_who_is_handing_over":
    "Geben Sie an, wer übergibt, als person:…",
  "l.onboarding_view.give_who_takes_over_as":
    "Geben Sie an, wer übernimmt, als person:… (oder lassen Sie es leer, um die Arbeit freizugeben).",
  "l.onboarding_view.hand_over_to_someone_else":
    "An eine andere Person übergeben.",
  "l.onboarding_view.no_items": "Keine Einträge",
  "l.onboarding_view.of_done": "{done} von {total} erledigt",
  "l.onboarding_view.skipped": "{skipped} übersprungen",
  "l.onboarding_view.overdue": "{overdue} überfällig",
  "l.onboarding_view.skipped_2": "Übersprungen",
  "l.onboarding_view.overdue_2": "Überfällig",
  "l.onboarding_view.nothing_is_left_open_this":
    "Nichts ist mehr offen. Dieser Plan kann abgeschlossen werden.",
  "l.onboarding_view.not_ready_to_close": "Nicht bereit zum Abschluss: {join}.",
  "l.onboarding_view.this_person_holds_nothing_live":
    "Diese Person hält in diesem Plan nichts Aktives.",
  "l.raci_view.nobody_is_accountable": "Niemand ist rechenschaftspflichtig",
  "l.raci_view.nobody_is_responsible":
    "Niemand ist durchführungsverantwortlich",
  "l.raci_view.more_than_one_person_is":
    "Mehr als eine Person ist rechenschaftspflichtig",
  "l.raci_view.has_no_role": "{task}: {who} hat keine Rolle",
  "l.raci_view.and": " und ",
  "l.raci_view.is": "{task}: {who} ist {join}",
  "l.rbs.a_booking_that_needs_several":
    "Eine Buchung, die mehrere Fähigkeiten benötigt, zählt bei jeder voll, daher überschneiden sich Fähigkeiten und ergeben keine Summe. Angebot und Personen sind für Fähigkeiten nicht bekannt.",
  "l.rbs.named": "{length} benannt",
  "l.rbs.supply_not_known": "Angebot unbekannt",
  "l.rbs.supply_fte": "Angebot {supply_fte} FTE",
  "l.rbs.demand_fte": "{role}: {people}, Bedarf {demand_fte} FTE, {supply}",
  "l.scheduling_view.pass": "Bestanden",
  "l.scheduling_view.fail": "Nicht bestanden",
  "l.scheduling_view.not_computable": "Nicht berechenbar",
  "l.scheduling_view.not_enough_data": "Nicht genügend Daten.",
  "l.scheduling_view.not_enough_data_2": "Nicht genügend Daten: {reason}",
  "l.scheduling_view.must_pass_a_business_day":
    "Muss bestehen: Eine Verzögerung von {v} Arbeitstagen muss das Ende verschieben",
  "l.scheduling_view.at_least_business_days": "Mindestens {limit} Arbeitstage",
  "l.scheduling_view.at_most": "höchstens",
  "l.scheduling_view.at_least": "mindestens",
  "l.scheduling_view.no_figure": "Kein Wert",
  "l.scheduling_view.finish_moved_of_business_days":
    "Ende um {count} von {of} Arbeitstagen verschoben",
  "l.scheduling_view.of": "{toFixed} ({count} von {of})",
  "l.scheduling_view.of_2": "{value} % ({count} von {of} {of_what})",
  "l.scheduling_view.passed_failed_not_computable":
    "{passed} bestanden, {failed} nicht bestanden, {not_computable} nicht berechenbar",
  "l.scheduling_view.not_resolved": "Nicht aufgelöst.",
  "l.scheduling_view.it_is_on_the_critical":
    " Sie liegt auf dem kritischen Pfad, daher verschiebt sich das Projektende.",
  "l.scheduling_view.starts_later_finishes_instead_of":
    "{pid} ({who}) beginnt {from_start} → {scheduled_start}, {days} später; endet {to_finish} statt {from_finish}.{crit}",
  "l.scheduling_view.to": "{from} bis {to}",
  "l.scheduling_view.is_over_capacity_on":
    "{who} ist im Zeitraum {span} über der Kapazität.",
  "l.scheduling_view.is_asked_for_on_and":
    "{who} wird im Zeitraum {span} zu {demand} % angefragt und kann {capacity} % leisten.",
  "l.scheduling_view.no_project_finish_can_be":
    "Es lässt sich kein Projektende berechnen.",
  "l.scheduling_view.the_project_finish_stays":
    "Das Projektende bleibt {before}.",
  "l.scheduling_view.the_project_finish_moves_from":
    "Das Projektende verschiebt sich von {before} auf {after} ({days} später).",
  "l.scheduling_view.no_demand_or_supply": "Kein Bedarf und kein Angebot",
  "l.scheduling_view.demand_fte_supply_not_known":
    "Bedarf {demand_fte} FTE; Angebot unbekannt",
  "l.scheduling_view.short_fte": "Fehlend {abs} FTE",
  "l.scheduling_view.spare_fte": "Frei {gap_fte} FTE",
  "l.scheduling_view.balanced": "Ausgeglichen",
  "l.scheduling_view.the_critical_path_finish": "das Ende des kritischen Pfads",
  "l.scheduling_view.of_the_simulated_outcomes_finish":
    "{pct} % der simulierten Ergebnisse enden am oder vor dem {date}.",
  "l.scheduling_view.no_variation": "Keine Schwankung",
  "l.scheduling_view.no_estimate_using_the_fallback":
    "Keine Schätzung: Ersatzbandbreite wird verwendet",
  "l.scheduling_view.milestone_no_spread": "Meilenstein: keine Streuung",
  "l.scheduling_view.estimated_widened_to_include_the":
    "Geschätzt (erweitert, um den wahrscheinlichen Wert einzuschließen)",
  "l.scheduling_view.estimated": "Geschätzt",
  "l.scheduling_view.pert_favours_the_likely_value":
    "PERT (gewichtet den wahrscheinlichen Wert)",
  "l.scheduling_view.of_the_simulated_outcomes_cost":
    "{round} % der simulierten Ergebnisse kosten nicht mehr als das Budget ({budgetText}).",
  "l.scheduling_view.estimated_widened_to_include_the_2":
    "Geschätzt (erweitert, um den geplanten Betrag einzuschließen)",
  "l.scheduling_view.stated_struck_in_of_runs":
    "angegeben {round} %, eingetreten in {toFixed} % der Läufe",
  "l.scheduling_view.within_budget_and_on_time": "Im Budget und pünktlich",
  "l.scheduling_view.within_budget_but_late": "Im Budget, aber verspätet",
  "l.scheduling_view.on_time_but_over_budget": "Pünktlich, aber über Budget",
  "l.scheduling_view.over_budget_and_late": "Über Budget und verspätet",
  "l.scheduling_view.both_within_budget_and_on":
    "Im Budget und pünktlich zugleich: {pct} (nur im Budget {pct2}; nur pünktlich {pct3}).",
  "l.scheduling_view.a_late_finish_costs_nothing":
    "Ein verspätetes Ende kostet in diesem Lauf nichts zusätzlich (Verzögerungskosten je Arbeitstag sind 0).",
  "l.scheduling_view.each_business_day_late_adds":
    "Jeder Arbeitstag Verspätung kostet {format}; ein früheres Ende bringt nichts zurück.",
  "l.simulation_inputs.the_probability_must_be_a":
    "Die Wahrscheinlichkeit muss eine ganze Zahl von 0 bis 100 sein.",
  "l.simulation_inputs.name_the_task_and_the":
    "Geben Sie die Aufgabe und die zusätzlichen Arbeitstage gemeinsam an oder lassen Sie beides leer.",
  "l.simulation_inputs.the_business_days_a_risk":
    "Die Arbeitstage, die ein Risiko hinzufügt, müssen eine ganze Zahl von 1 bis 365 sein.",
  "l.simulation_inputs.give_the_cost_and_its":
    "Geben Sie die Kosten und ihre Währung gemeinsam an oder lassen Sie beides leer.",
  "l.simulation_inputs.the_cost_must_be_an":
    "Die Kosten müssen ein Betrag über null sein.",
  "l.simulation_inputs.the_currency_must_be_a":
    "Die Währung muss ein dreistelliger Code sein, zum Beispiel EUR, JPY oder GBP.",
  "l.simulation_inputs.a_risk_that_changes_the":
    "Ein Risiko, das Termin oder Kosten ändert, benötigt eine Wahrscheinlichkeit.",
  "l.simulation_inputs.amounts_must_be_numbers_zero":
    "Beträge müssen Zahlen von null oder mehr sein.",
  "l.simulation_inputs.the_low_amount_cannot_be":
    "Der niedrige Betrag darf nicht über dem geplanten Betrag liegen.",
  "l.simulation_inputs.the_high_amount_cannot_be":
    "Der hohe Betrag darf nicht unter dem geplanten Betrag liegen.",
  "l.simulation_inputs.business_days_must_be_whole":
    "Arbeitstage müssen ganze Zahlen von 0 bis 3650 sein.",
  "l.simulation_inputs.give_the_task_a_duration":
    "Geben Sie der Aufgabe zuerst eine Dauer: Das ist ihr wahrscheinlicher Wert.",
  "l.simulation_inputs.optimistic_cannot_be_above_the":
    "Der optimistische Wert darf nicht über der wahrscheinlichen Dauer ({likely}) liegen.",
  "l.simulation_inputs.pessimistic_cannot_be_below_the":
    "Der pessimistische Wert darf nicht unter der wahrscheinlichen Dauer ({likely}) liegen.",
  "l.simulation_inputs.no_range_the_simulation_uses":
    "Keine Bandbreite: Die Simulation verwendet ihre Ersatzbandbreite",
  "l.simulation_inputs.to": "{fmt} bis {fmt2}",
  "l.wbs.restricted_item": "Eingeschränktes Element",
  "l.whatif_view.choose_a_task": "Wählen Sie eine Aufgabe.",
  "l.whatif_view.give_the_new_start_date": "Geben Sie das neue Startdatum an.",
  "l.whatif_view.duration_must_be_a_whole":
    "Die Dauer muss eine ganze Zahl von Arbeitstagen von 0 bis 3650 sein.",
  "l.whatif_view.choose_a_constraint": "Wählen Sie eine Einschränkung.",
  "l.whatif_view.a_constraint_needs_a_date":
    "Eine Einschränkung benötigt ein Datum.",
  "l.whatif_view.a_hard_constraint_needs_a":
    "Eine harte Einschränkung benötigt einen Grund.",
  "l.whatif_view.choose_both_tasks": "Wählen Sie beide Aufgaben.",
  "l.whatif_view.a_task_cannot_depend_on":
    "Eine Aufgabe kann nicht von sich selbst abhängen.",
  "l.whatif_view.lag_must_be_a_whole":
    "Der Verzug muss eine ganze Zahl von Arbeitstagen sein (negativ bedeutet Vorlauf).",
  "l.whatif_view.choose_a_link": "Wählen Sie eine Verknüpfung.",
  "l.whatif_view.start_on": "„{name}“ am {start_date} beginnen",
  "l.whatif_view.make_a_milestone": "„{name}“ zum Meilenstein machen",
  "l.whatif_view.for": "{constraint_kind} {constraint_date} für „{name}“",
  "l.whatif_view.clear_the_constraint_on":
    "Einschränkung für „{name}“ aufheben",
  "l.whatif_view.lag": ", Verzug {lag_days}",
  "l.whatif_view.follows": "„{name}“ folgt auf „{name2}“ ({toUpperCase}{v})",
  "l.whatif_view.remove_the_link": "Verknüpfung {slice} entfernen",
  "l.whatif_view.the_plan_has_no_finish": "Der Plan hat noch kein Enddatum.",
  "l.whatif_view.the_finish_stays": "Das Ende bleibt {after}.",
  "l.whatif_view.the_finish_becomes": "Das Ende wird {after}.",
  "l.whatif_view.the_plan_no_longer_has":
    "Der Plan hat kein Enddatum mehr (war {before}).",
  "l.whatif_view.the_finish_moves_from_to":
    "Das Ende verschiebt sich von {before} auf {after}.",
  "l.whatif_view.cost_impact_is_finance_data":
    "Die Kostenauswirkung gehört zu den Finanzdaten und wird Ihnen daher nicht angezeigt.",
  "l.whatif_view.no_task_with_work_has":
    "Keine Aufgabe mit Arbeit hat einen Satz, daher gibt es keine Kosten zum Vergleichen.",
  "l.work_view.work_must_be_a_number":
    "Die Arbeit muss eine Stundenzahl über null sein.",
  "l.work_view.duration_must_be_a_whole":
    "Die Dauer muss eine ganze Zahl von Arbeitstagen von 1 bis 3650 sein.",
  "l.work_view.units_must_be_a_whole":
    "Einheiten müssen ein ganzer Prozentsatz eines Tages von 1 bis 1000 sein.",
  "l.work_view.give_the_units_too": "Geben Sie auch die Einheiten an.",
  "l.work_view.business_days_at_is":
    "{duration_days} Arbeitstage zu {units_percent} % ergeben {hoursText}",
  "l.work_view.the_duration_was_worked_out": "Die Dauer wurde berechnet.",
  "l.work_view.the_share_of_the_day": "Der Tagesanteil wurde berechnet.",
  "l.work_view.the_work_was_worked_out": "Die Arbeit wurde berechnet.",
  "l.work_view.rounded_the_three_do_not":
    " Gerundet: Die drei Werte stimmen nicht auf die Minute überein.",
  "l.work_view.capacity_not_checked_the_task":
    "Kapazität nicht geprüft: Die Aufgabe hat noch keine Termine.",
  "l.work_view.capacity_undeclared":
    "Für einige dieser Tage ist keine Kapazität angegeben, daher liegt dies weder über noch innerhalb der Kapazität der Person.",
  "l.work_view.within_the_person_s_capacity":
    "Innerhalb der Kapazität der Person.",
  "l.work_view.give_1_to_40_whole":
    "Geben Sie 1 bis 40 ganze Prozentsätze an, durch Kommas getrennt.",
  "l.work_view.the_percentages_add_up_to":
    "Die Prozentsätze ergeben zusammen {sum}, nicht 100.",
  "p.home.main_x_plans": "Main X · Pläne",
  "p.announcements.could_not_load_the_announcements":
    "Ankündigungen konnten nicht geladen werden",
  "p.announcements.could_not_publish": "Veröffentlichen nicht möglich",
  "p.announcements.could_not_retract": "Zurückziehen nicht möglich",
  "p.auditor.auditor_ppm": "Prüfer — PPM",
  "p.automations.could_not_load_automations":
    "Automatisierungen konnten nicht geladen werden",
  "p.automations.rule_saved_it_fires_from":
    "Regel gespeichert — sie greift ab der nächsten passenden Verschiebung",
  "p.automations.could_not_save_the_rule":
    "Regel konnte nicht gespeichert werden",
  "p.automations.that_action_was_refused": "Diese Aktion wurde abgelehnt",
  "p.automations.capped_at_run_again_for":
    " (begrenzt auf {cap} — für den Rest erneut ausführen)",
  "p.automations.swept_fired_already_claimed":
    "Durchlauf: {fired} ausgelöst, {skipped_already_claimed} bereits beansprucht{v}",
  "p.automations.the_sweep_failed": "Der Durchlauf ist fehlgeschlagen",
  "p.board.board_ppm": "Board — PPM",
  "p.calendar.calendar_ppm": "Kalender — PPM",
  "p.ceo.could_not_load": "Laden nicht möglich",
  "p.compliance.compliance_ppm": "Compliance — PPM",
  "p.cost_rates.could_not_load_the_rate":
    "Satztabellen konnten nicht geladen werden",
  "p.cost_rates.rate_table_added": "Satztabelle hinzugefügt.",
  "p.cost_rates.could_not_add_the_table":
    "Tabelle konnte nicht hinzugefügt werden",
  "p.cost_rates.give_a_date_and_an":
    "Geben Sie ein Datum und einen Stundensatz von null oder mehr an.",
  "p.cost_rates.rate_added": "Satz hinzugefügt.",
  "p.cost_rates.could_not_add_the_rate": "Satz konnte nicht hinzugefügt werden",
  "p.cost_risk.could_not_run_the_simulation":
    "Simulation konnte nicht ausgeführt werden",
  "p.documents.could_not_load_the_documents":
    "Dokumente konnten nicht geladen werden",
  "p.documents.could_not_create_the_document":
    "Dokument konnte nicht erstellt werden",
  "p.documents.pid.could_not_load_the_document":
    "Dokument konnte nicht geladen werden",
  "p.documents.pid.that_did_not_work": "Das hat nicht funktioniert",
  "p.documents.pid.enter_the_approver_s_id":
    "Geben Sie die ID der genehmigenden Person ein (eine Personen-ID, nicht Ihre eigene).",
  "p.documents.pid.sent_for_review": "Zur Prüfung gesendet.",
  "p.documents.pid.needs_changes": "Änderungen erforderlich",
  "p.documents.pid.approved": "Genehmigt.",
  "p.documents.pid.sent_back_to_draft": "Zurück in den Entwurf gesetzt.",
  "p.documents.pid.in_force_from_today": "Ab heute in Kraft.",
  "p.documents.pid.next_draft_started": "Nächster Entwurf begonnen.",
  "p.documents.pid.could_not_look_that_up": "Nachschlagen nicht möglich",
  "p.engineering.engineering_ppm": "Engineering — PPM",
  "p.executive.executive_ppm": "Führung — PPM",
  "p.financials.financials_ppm": "Finanzen — PPM",
  "p.gantt.gantt_main_x": "Gantt — Main X",
  "p.gantt.calendar_failed":
    "Der Arbeitskalender konnte nicht geladen werden, daher sind arbeitsfreie Tage nicht schattiert.",
  "p.kpis.could_not_load_the_kpis": "KPIs konnten nicht geladen werden",
  "p.kpis.could_not_define_the_kpi": "KPI konnte nicht definiert werden",
  "p.lifecycle.could_not_load_the_funnel":
    "Trichter konnte nicht geladen werden",
  "p.me.could_not_load_your_tasks":
    "Ihre Aufgaben konnten nicht geladen werden",
  "p.me.calendar.could_not_load_your_calendar":
    "Ihr Kalender konnte nicht geladen werden",
  "p.me.calendar.could_not_save": "Speichern nicht möglich",
  "p.me.calendar.could_not_remove": "Entfernen nicht möglich",
  "p.milestones.could_not_load_the_milestones":
    "Meilensteine konnten nicht geladen werden",
  "p.objectives.create_failed": "Erstellen fehlgeschlagen",
  "p.objectives.alignment_failed": "Ausrichtung fehlgeschlagen",
  "p.okrs.could_not_load_the_okrs": "OKRs konnten nicht geladen werden",
  "p.okrs.could_not_create_the_objective": "Ziel konnte nicht erstellt werden",
  "p.plans.plans_main_x": "Pläne — Main X",
  "p.plans.pid.main_x": "{v} — Main X",
  "p.plans.pid.board.pt": " · {points} Pkt.",
  "p.plans.pid.board.board_ppm": "Board — PPM",
  "p.plans.pid.calendar.could_not_load_the_calendar":
    "Kalender konnte nicht geladen werden",
  "p.plans.pid.calendar.could_not_save": "Speichern nicht möglich",
  "p.plans.pid.cost_risk.could_not_run_the_simulation":
    "Simulation konnte nicht ausgeführt werden",
  "p.plans.pid.costs.could_not_load_the_costs":
    "Kosten konnten nicht geladen werden",
  "p.plans.pid.costs.choose_a_task": "Wählen Sie eine Aufgabe.",
  "p.plans.pid.costs.the_amount_must_be_above":
    "Der Betrag muss über null liegen.",
  "p.plans.pid.costs.the_currency_must_be_a":
    "Die Währung muss ein dreistelliger Code sein, zum Beispiel EUR, JPY oder GBP.",
  "p.money.currency_hint": "zum Beispiel EUR, JPY oder GBP",
  "p.money.amount_or_currency":
    "Geben Sie den Betrag als Zahl und die Währung als dreistelligen Code an, zum Beispiel EUR, JPY oder GBP.",
  "p.plans.pid.costs.expense_added": "Ausgabe hinzugefügt.",
  "p.plans.pid.costs.could_not_add_the_expense":
    "Ausgabe konnte nicht hinzugefügt werden",
  "p.plans.pid.costs.could_not_record": "Erfassen nicht möglich",
  "p.plans.pid.critical_path.could_not_load_the_baselines":
    "Basispläne konnten nicht geladen werden",
  "p.plans.pid.critical_path.could_not_take_the_baseline":
    "Basisplan konnte nicht erstellt werden",
  "p.plans.pid.critical_path.could_not_load_the_schedule":
    "Zeitplan konnte nicht geladen werden",
  "p.plans.pid.critical_path.could_not_save_the_estimate":
    "Schätzung konnte nicht gespeichert werden",
  "p.plans.pid.critical_path.could_not_save": "Speichern nicht möglich",
  "p.plans.pid.earned_value.could_not_load_earned_value":
    "Earned Value konnte nicht geladen werden",
  "p.plans.pid.earned_value.could_not_save": "Speichern nicht möglich",
  "p.plans.pid.earned_value.could_not_record": "Erfassen nicht möglich",
  "p.plans.pid.edit.edit_case_main_x": "Fall bearbeiten — Main X",
  "p.plans.pid.flow.could_not_load_flow_data":
    "Flussdaten konnten nicht geladen werden.",
  "p.plans.pid.handover.handover_is_about_people_so":
    "Bei der Übergabe geht es um Personen, daher ist die Berechtigung zur Personenverwaltung erforderlich.",
  "p.plans.pid.handover.could_not_preview": "Vorschau nicht möglich",
  "p.plans.pid.handover.what_this_person_holds_changed":
    "Was diese Person hält, hat sich seit der Vorschau geändert, daher wurde nichts geändert. Erneut Vorschau anzeigen, dann festschreiben.",
  "p.plans.pid.handover.could_not_commit": "Festschreiben nicht möglich",
  "p.plans.pid.handover.discarded_nothing_was_changed":
    "Verworfen. Es wurde nichts geändert.",
  "p.plans.pid.import.choose_the_file_first": "Wählen Sie zuerst die Datei.",
  "p.plans.pid.import.the_import_did_not_work":
    "Der Import hat nicht funktioniert",
  "p.plans.pid.joint_risk.could_not_run_the_simulation":
    "Simulation konnte nicht ausgeführt werden",
  "p.plans.pid.leveling.could_not_propose_a_leveling":
    "Kein Kapazitätsabgleich vorschlagbar",
  "p.plans.pid.leveling.could_not_apply_the_moves":
    "Verschiebungen konnten nicht angewendet werden",
  "p.plans.pid.onboarding.could_not_load": "Laden nicht möglich",
  "p.plans.pid.onboarding.could_not_open_the_checklist":
    "Checkliste konnte nicht geöffnet werden",
  "p.plans.pid.onboarding.checklist_started": "Checkliste begonnen.",
  "p.plans.pid.onboarding.could_not_start": "Starten nicht möglich",
  "p.plans.pid.onboarding.could_not_save": "Speichern nicht möglich",
  "p.plans.pid.onboarding.checklist_completed": "Checkliste abgeschlossen.",
  "p.plans.pid.onboarding.could_not_complete": "Abschließen nicht möglich",
  "p.plans.pid.onboarding.say_why_you_are_cancelling":
    "Geben Sie an, warum Sie abbrechen.",
  "p.plans.pid.onboarding.checklist_cancelled": "Checkliste abgebrochen.",
  "p.plans.pid.onboarding.could_not_cancel": "Abbrechen nicht möglich",
  "p.plans.pid.onboarding.could_not_record": "Erfassen nicht möglich",
  "p.plans.pid.raci.could_not_load_who_does":
    "Zuständigkeiten konnten nicht geladen werden",
  "p.plans.pid.raci.could_not_save": "Speichern nicht möglich",
  "p.plans.pid.raci.use_a_person_or_worker":
    "Verwenden Sie einen person:- oder worker:-Verweis mit einer UUID.",
  "p.plans.pid.schedule_health.could_not_run_the_schedule":
    "Zeitplanprüfungen konnten nicht ausgeführt werden",
  "p.plans.pid.schedule_risk.could_not_run_the_simulation":
    "Simulation konnte nicht ausgeführt werden",
  "p.plans.pid.wbs.could_not_load_the_tasks":
    "Aufgaben konnten nicht geladen werden",
  "p.plans.pid.wbs.link_counts_failed":
    "Die Anzahl der Verknüpfungen je Aufgabe konnte nicht geladen werden, daher werden keine Zahlen angezeigt.",
  "p.plans.pid.what_if.could_not_load_the_plan":
    "Plan konnte nicht geladen werden",
  "p.plans.pid.what_if.at_most_edits_at_a":
    "Höchstens {MAX_EDITS} Änderungen gleichzeitig.",
  "p.plans.pid.what_if.edit_staged_preview_to_see":
    "Änderung vorgemerkt. Zeigen Sie die Vorschau an, um die Wirkung zu sehen.",
  "p.plans.pid.what_if.discarded_nothing_was_changed":
    "Verworfen. Es wurde nichts geändert.",
  "p.plans.pid.what_if.could_not_preview": "Vorschau nicht möglich",
  "p.plans.pid.what_if.the_plan_changed_since_this":
    "Der Plan hat sich seit dieser Vorschau geändert, daher wurde nichts angewendet. Erneut Vorschau anzeigen, dann festschreiben.",
  "p.plans.pid.what_if.could_not_commit": "Festschreiben nicht möglich",
  "p.plans.pid.what_if.to": "{early_start} bis {early_finish}",
  "p.plans.pid.what_if.not_scheduled": "nicht terminiert",
  "p.plans.pid.work.could_not_load_the_plan":
    "Plan konnte nicht geladen werden",
  "p.plans.pid.work.could_not_load_the_work":
    "Arbeitsprofil konnte nicht geladen werden",
  "p.plans.pid.work.could_not_save": "Speichern nicht möglich",
  "p.plans.pid.work.contour_saved": "Verlaufsprofil gespeichert.",
  "p.plans.merge.merge_plans_ppm": "Pläne zusammenführen — PPM",
  "p.plans.new.new_case_main_x": "Neuer Fall — Main X",
  "p.prioritization.could_not_load_the_queue":
    "Warteschlange konnte nicht geladen werden",
  "p.prioritization.could_not_load_the_score":
    "Bewertung konnte nicht geladen werden",
  "p.proposals.duplicate_check_failed": "Duplikatprüfung fehlgeschlagen",
  "p.regulator.regulator_ppm": "Regulierer — PPM",
  "p.reports.save_failed": "Speichern fehlgeschlagen",
  "p.reports.run_failed": "Ausführen fehlgeschlagen",
  "p.resources.could_not_load_the_resource":
    "Ressourcenplan konnte nicht geladen werden",
  "p.resources.rbs.could_not_load_resources":
    "Ressourcen konnten nicht geladen werden",
  "p.reviews.could_not_load_reviews":
    "Bewertungen konnten nicht geladen werden",
  "p.reviews.invitation_sent": "Einladung gesendet",
  "p.reviews.could_not_delegate": "Delegieren nicht möglich",
  "p.reviews.score_0_100_leave_blank":
    "Bewertung 0–100 (leer lassen für keine)",
  "p.reviews.score_must_be_between_0":
    "Die Bewertung muss zwischen 0 und 100 liegen",
  "p.reviews.that_action_was_refused": "Diese Aktion wurde abgelehnt",
  "p.reviews.could_not_load_consensus": "Konsens konnte nicht geladen werden",
  "p.risk.risk_ppm": "Risiko — PPM",
  "p.scenarios.evaluate_failed": "Bewerten fehlgeschlagen",
  "p.security.security_ppm": "Sicherheit — PPM",
  "p.technology.technology_ppm": "Technologie — PPM",
  "p.tour.tour_main_x_plans": "Rundgang · Main X Pläne",
  "p.verify.this_sign_in_link_is": "Diesem Anmeldelink fehlt das Token.",
  "p.verify.we_could_not_reach_the":
    "Der Anmeldedienst ist nicht erreichbar. Bitte versuchen Sie es gleich noch einmal.",
  "p.verify.this_sign_in_link_is_2":
    "Dieser Anmeldelink ist ungültig oder abgelaufen.",
  "l.ceo_view.gate_review": "Gate-Prüfung",
  "l.ceo_view.scenario_committed": "Szenario festgeschrieben",
  "l.ceo_view.proposal": "Vorschlag",
  "l.ceo_view.plans_merged": "Pläne zusammengeführt",
  "c.CumulativeFlow.in_review": "In Prüfung",
  "c.CumulativeFlow.in_progress": "In Arbeit",
  "c.CumulativeFlow.todo": "Offen",
  "l.document_view.draft": "Entwurf",
  "l.document_view.in_review": "In Prüfung",
  "l.document_view.approved_not_yet_in_force": "Genehmigt, noch nicht in Kraft",
  "l.document_view.replaced": "Ersetzt",
  "l.document_view.send_for_review": "Zur Prüfung senden",
  "l.document_view.approve": "Genehmigen",
  "l.document_view.bring_into_force_today": "Heute in Kraft setzen",
  "l.document_view.start_the_next_draft": "Nächsten Entwurf beginnen",
  "l.document_view.waiting_for_the_approver":
    "Wartet auf die genehmigende Person",
  "l.document_view.approved": "Genehmigt",
  "l.document_view.change_this_document": "Dieses Dokument ändern",
  "l.document_view.someone_else_approves_it_you":
    "Eine andere Person genehmigt es: Sie können Ihr eigenes Dokument nicht genehmigen.",
  "l.document_view.only_the_person_asked_can":
    "Nur die angefragte Person kann entscheiden.",
  "l.document_view.it_comes_into_force_today":
    "Es tritt heute in Kraft und ersetzt die derzeit gültige Version.",
  "l.document_view.the_next_draft_starts_as":
    "Der nächste Entwurf beginnt als Kopie des neuesten Texts.",
  "l.ev_view.on_track": "im Plan",
  "l.ev_view.off_plan": "außerhalb des Plans",
  "l.ev_view.not_yet_known": "noch nicht bekannt",
  "l.glossary.schedule_performance_index_work_done":
    "Termin-Leistungsindex: erledigte Arbeit im Vergleich zur bis jetzt geplanten Arbeit. Über 1,00 heißt vor dem Plan.",
  "l.glossary.cost_performance_index_value_of":
    "Kosten-Leistungsindex: Wert der erledigten Arbeit im Vergleich zu ihren Kosten. Über 1,00 heißt unter Budget.",
  "l.glossary.schedule_index_by_time_how":
    "Zeitbasierter Terminindex: wie weit der Plan zeitlich fortgeschritten ist; bleibt auch dann aussagekräftig, wenn ein Projekt verspätet endet.",
  "l.glossary.earned_value_the_budget_for":
    "Fertigstellungswert: das Budget für die tatsächlich erledigte Arbeit.",
  "l.glossary.planned_value_the_budget_for":
    "Planwert: das Budget für die Arbeit, die bis jetzt erledigt sein sollte.",
  "l.glossary.actual_cost_what_has_really":
    "Ist-Kosten: was tatsächlich ausgegeben wurde.",
  "l.glossary.estimate_at_completion_the_expected":
    "Schätzung bei Fertigstellung: die erwarteten Endkosten.",
  "l.glossary.estimate_to_complete_what_is":
    "Restschätzung: was voraussichtlich noch ausgegeben wird.",
  "l.glossary.variance_at_completion_the_expected":
    "Abweichung bei Fertigstellung: die erwarteten Endkosten im Vergleich zum Budget.",
  "l.glossary.red_amber_or_green_a": "Rot, Gelb oder Grün: eine Statusampel.",
  "l.glossary.responsible_accountable_consulted_in":
    "Durchführungsverantwortlich, Rechenschaftspflichtig, Konsultiert, Informiert: wer die Arbeit macht, wer sie abnimmt, wer gefragt und wer informiert wird.",
  "l.glossary.a_way_of_weighting_the":
    "Ein Verfahren, das die wahrscheinliche Schätzung stärker gewichtet als den besten und schlechtesten Fall.",
  "l.glossary.key_performance_indicator_a_measure":
    "Leistungskennzahl: eine Messgröße, die über die Zeit gegen ein Ziel verfolgt wird.",
  "l.glossary.key_performance_indicators_measures_":
    "Leistungskennzahlen: Messgrößen, die über die Zeit gegen ein Ziel verfolgt werden.",
  "l.glossary.objective_and_key_results_a":
    "Ziel und Schlüsselergebnisse: ein Ziel mit einigen messbaren Ergebnissen.",
  "l.glossary.objectives_and_key_results_goals":
    "Ziele und Schlüsselergebnisse: Ziele mit einigen messbaren Ergebnissen.",
  "l.import_view.jira_csv_export": "Jira (CSV-Export)",
  "l.import_view.asana_csv_export": "Asana (CSV-Export)",
  "l.import_view.microsoft_project_xml_file": "Microsoft Project (XML-Datei)",
  "l.import_view.columns_not_imported": "Nicht importierte Spalten",
  "l.import_view.rows_skipped": "Übersprungene Zeilen",
  "l.import_view.links_not_imported": "Nicht importierte Verknüpfungen",
  "l.import_view.parents_not_used": "Nicht verwendete Elternelemente",
  "l.import_view.dates_not_understood": "Nicht erkannte Datumsangaben",
  "l.import_view.fields_shortened": "Gekürzte Felder",
  "l.import_view.status_notes": "Statushinweise",
  "l.import_view.resources_not_matched_to_people":
    "Nicht Personen zugeordnete Ressourcen",
  "l.import_view.costs_not_imported": "Nicht importierte Kosten",
  "l.import_view.schedules_not_imported": "Nicht importierte Termine",
  "l.import_view.constraints_not_imported": "Nicht importierte Einschränkungen",
  "l.import_view.calendar_notes": "Kalenderhinweise",
  "l.kpi_view.improving": "Verbessert sich",
  "l.kpi_view.getting_worse": "Verschlechtert sich",
  "l.kpi_view.no_change": "Keine Änderung",
  "l.milestone_view.on_track": "Im Plan",
  "l.milestone_view.at_risk": "Gefährdet",
  "l.milestone_view.forecast_late": "Voraussichtlich verspätet",
  "l.milestone_view.no_forecast": "Keine Prognose",
  "l.milestone_view.it_is_not_linked_to":
    "Er ist mit keiner Aufgabe verknüpft, daher gibt es keine Grundlage für eine Prognose.",
  "l.milestone_view.its_open_tasks_have_no":
    "Seine offenen Aufgaben haben keine Termine, die der Zeitplan einordnen kann (geben Sie ihnen eine Dauer und einen Start oder einen Vorgänger).",
  "l.milestone_view.the_plan_s_dependencies_contain":
    "Die Abhängigkeiten des Plans enthalten einen Zyklus, daher ist keine Prognose möglich.",
  "l.okr_view.not_measurable_the_start_and":
    "Nicht messbar: Start und Ziel sind gleich, daher gibt es keine Strecke zurückzulegen.",
  "l.okr_view.not_measurable_a_maintain_key":
    "Nicht messbar: Ein Halte-Schlüsselergebnis benötigt ein Toleranzband.",
  "l.okr_view.not_measurable_the_numbers_are":
    "Nicht messbar: Die Zahlen sind zu groß für eine Bewertung.",
  "l.onboarding_view.someone_joins_this_plan":
    "Jemand kommt zu diesem Plan hinzu",
  "l.onboarding_view.someone_leaves_this_plan": "Jemand verlässt diesen Plan",
  "l.onboarding_view.take_this_plan_in": "Diesen Plan übernehmen",
  "l.onboarding_view.close_this_plan_out": "Diesen Plan abschließen",
  "l.onboarding_view.booking": "Buchung",
  "l.onboarding.executive": "Führung",
  "l.onboarding.outcomes_and_exposure_across_the":
    "Ergebnisse und Exposition über den gesamten Bestand, nicht nur für einen Plan.",
  "l.onboarding.read_the_executive_summary": "Management-Zusammenfassung lesen",
  "l.onboarding.health_decisions_benefits_realized_a":
    "Zustand, Entscheidungen, realisierter Nutzen und strategische Ausrichtung in einer Ansicht.",
  "l.onboarding.open_the_at_a_glance": "Übersichts-Dashboard öffnen",
  "l.onboarding.the_whole_estate_s_rag":
    "RAG-Status und Zeitplanverstöße des gesamten Bestands auf einen Blick.",
  "l.onboarding.review_the_board_pack": "Vorstandsunterlagen prüfen",
  "l.onboarding.the_pack_a_board_meeting":
    "Die Unterlagen, die eine Vorstandssitzung tatsächlich erhalten würde: Investitionen und Trends.",
  "l.onboarding.check_budget_variance_and_exposure":
    "Budgetabweichung und Exposition prüfen",
  "l.onboarding.where_spend_is_diverging_from":
    "Wo die Ausgaben vom Plan abweichen, je Währung — Währungen werden nie zusammengerechnet.",
  "l.onboarding.compare_two_candidate_portfolios":
    "Zwei Portfolio-Varianten vergleichen",
  "l.onboarding.what_if_scenarios_side_by":
    "Was-wäre-wenn-Szenarien nebeneinander, bevor Mittel für eines davon zugesagt werden.",
  "l.onboarding.see_what_s_ranked_highest":
    "Sehen, was am höchsten eingestuft ist, und warum",
  "l.onboarding.the_smart_score_queue_every":
    "Die Smart-Score-Warteschlange — jede Bewertung zeigt die Belege dahinter oder sagt, dass es keine gibt.",
  "l.onboarding.pmo_portfolio_manager": "PMO / Portfoliomanagement",
  "l.onboarding.intake_governance_and_the_schedule":
    "Aufnahme, Governance und der Zeitplan über alle Pläne.",
  "l.onboarding.open_the_intake_pipeline": "Aufnahme-Pipeline öffnen",
  "l.onboarding.every_proposal_s_stage_from":
    "Die Phase jedes Vorschlags, vom Entwurf bis zur Genehmigung.",
  "l.onboarding.register_a_plan": "Einen Plan registrieren",
  "l.onboarding.the_first_plan_you_create":
    "Mit dem ersten Plan, den Sie anlegen, sehen Sie gut die vollständige Datenstruktur.",
  "l.onboarding.browse_the_plan_registry": "Planregister durchsuchen",
  "l.onboarding.one_recursive_collection_kind_is":
    "Eine rekursive Sammlung — `kind` ist eine Bezeichnung, keine Schranke.",
  "l.onboarding.open_a_plan_s_governance":
    "Governance-Reiter eines Plans öffnen",
  "l.onboarding.gate_reviews_risks_and_budget":
    "Gate-Prüfungen, Risiken und Budgetpositionen für diesen einen Plan.",
  "l.onboarding.look_at_the_cross_plan": "Planübergreifenden Zeitplan ansehen",
  "l.onboarding.dependencies_the_critical_path_and":
    "Abhängigkeiten, der kritische Pfad und jeder sich verschiebende Nachfolger.",
  "l.onboarding.check_the_risk_register": "Risikoregister prüfen",
  "l.onboarding.everything_open_or_mitigating_ranked":
    "Alles Offene oder in Minderung, nach Exposition geordnet.",
  "l.onboarding.review_the_compliance_register": "Compliance-Register prüfen",
  "l.onboarding.standing_findings_across_the_estate":
    "Bestehende Feststellungen über den gesamten Bestand, nicht nur für einen Plan.",
  "l.onboarding.run_a_saved_report": "Einen gespeicherten Bericht ausführen",
  "l.onboarding.a_filter_field_projection_saved":
    "Ein Filter mit Feldauswahl, zur Wiederverwendung gespeichert und bei Bedarf ausgeführt.",
  "l.onboarding.resource_manager": "Ressourcenmanagement",
  "l.onboarding.who_is_allocated_where_and":
    "Wer wo zugewiesen ist und wie die Lieferung tatsächlich fließt.",
  "l.onboarding.check_the_capacity_rollup": "Kapazitätsübersicht prüfen",
  "l.onboarding.who_is_over_allocated_this":
    "Wer in diesem Zeitraum überlastet ist, summiert über alle Pläne der Person.",
  "l.onboarding.open_a_plan_s_task": "Aufgabenboard eines Plans öffnen",
  "l.onboarding.the_kanban_board_that_task":
    "Das Kanban-Board, über das die Arbeit auf Aufgabenebene tatsächlich läuft.",
  "l.onboarding.read_that_plan_s_flow": "Flusskennzahlen dieses Plans lesen",
  "l.onboarding.cycle_time_lead_time_and":
    "Zykluszeit, Durchlaufzeit und Flusseffizienz — abgeleitet aus dem eigenen Verlauf des Boards.",
  "l.onboarding.scan_blocked_items_and_the":
    "Blockierte Elemente und den Liefermix überblicken",
  "l.onboarding.what_s_stuck_the_moscow":
    "Was feststeckt, der MoSCoW-Mix und der Meilensteinkalender in einer Ansicht.",
  "l.onboarding.look_at_the_milestone_calendar": "Meilensteinkalender ansehen",
  "l.onboarding.every_milestone_across_every_plan":
    "Jeder Meilenstein aus jedem Plan, einschließlich Fälligkeitsdaten.",
  "l.onboarding.see_what_automations_are_configured":
    "Konfigurierte Automatisierungen ansehen",
  "l.onboarding.what_fires_on_a_field":
    "Was bei einer Feldänderung ausgelöst wird und was dann geschieht.",
  "l.onboarding.open_collaborative_reviews": "Gemeinsame Bewertungen öffnen",
  "l.onboarding.where_a_plan_gets_a":
    "Wo ein Plan eine zweite Meinung erhält, bevor darüber entschieden wird.",
  "l.raci_view.responsible": "Durchführungsverantwortlich",
  "l.raci_view.accountable": "Rechenschaftspflichtig",
  "l.raci_view.consulted": "Konsultiert",
  "l.raci_view.informed": "Informiert",
  "l.raci_view.does_the_work": "erledigt die Arbeit",
  "l.raci_view.answers_for_it_and_signs":
    "steht dafür ein und nimmt sie ab (eine Person je Aufgabe)",
  "l.raci_view.is_asked_before": "wird vorher gefragt",
  "l.raci_view.is_told_after": "wird danach informiert",
  "l.scheduling_view.no_schedule_baseline_has_been":
    "Für diesen Plan wurde kein Termin-Basisplan erstellt.",
  "l.scheduling_view.nothing_was_baselined_to_finish":
    "Laut Basisplan sollte bis zum Stichtag nichts abgeschlossen sein.",
  "l.scheduling_view.the_plan_s_dependencies_contain":
    "Die Abhängigkeiten des Plans enthalten einen Zyklus.",
  "l.scheduling_view.there_are_no_incomplete_tasks":
    "Es gibt keine unerledigten Aufgaben zum Messen.",
  "l.scheduling_view.there_are_no_relationships_between":
    "Es gibt keine Beziehungen zwischen Aufgaben zum Messen.",
  "l.scheduling_view.no_incomplete_task_has_dates":
    "Keine unerledigte Aufgabe hat Termine, die der Zeitplan einordnen kann.",
  "l.scheduling_view.no_incomplete_task_has_a":
    "Keine unerledigte Aufgabe hat eine Dauer.",
  "l.scheduling_view.the_plan_has_no_tasks": "Der Plan hat keine Aufgaben.",
  "l.scheduling_view.there_is_no_critical_task":
    "Es gibt keine kritische Aufgabe zum Testen.",
  "l.scheduling_view.no_path_remains_to_measure":
    "Es bleibt kein Pfad zum Messen.",
  "l.scheduling_view.every_task_involved_has_already":
    "Jede beteiligte Aufgabe hat bereits begonnen, daher darf keine verschoben werden.",
  "l.scheduling_view.the_tasks_involved_are_critical":
    "Die beteiligten Aufgaben sind kritisch; eine Verschiebung würde das Ende verschieben. Lassen Sie kritische Aufgaben zu, um diese Kosten zu sehen.",
  "l.scheduling_view.delaying_a_task_enough_would":
    "Eine ausreichende Verzögerung einer Aufgabe würde den Zeitplan sprengen (negativer Puffer oder späteres Ende).",
  "l.scheduling_view.a_task_involved_has_a":
    "Eine beteiligte Aufgabe hat eine harte Einschränkung und kann nicht verschoben werden.",
  "l.scheduling_view.the_schedule_would_not_let":
    "Der Zeitplan lässt keinen späteren Start der Aufgabe zu.",
  "l.scheduling_view.the_search_stopped_at_its":
    "Die Suche hat ihr Limit erreicht, bevor dies aufgelöst wurde.",
  "l.scheduling_view.no_probability": "keine Wahrscheinlichkeit",
  "l.scheduling_view.no_task_named": "keine Aufgabe angegeben",
  "l.scheduling_view.no_impact_in_business_days":
    "keine Auswirkung in Arbeitstagen",
  "l.scheduling_view.the_risk_is_not_open": "das Risiko ist nicht offen",
  "l.scheduling_view.its_task_is_not_on": "seine Aufgabe ist nicht im Zeitplan",
  "l.scheduling_view.the_plan_has_no_budget":
    "der Plan hat keine Budgetposition in dieser Währung",
  "l.whatif_view.move_a_task_set_its": "Aufgabe verschieben (Start festlegen)",
  "l.whatif_view.change_a_task_s_duration": "Dauer einer Aufgabe ändern",
  "l.whatif_view.change_a_task_s_constraint":
    "Einschränkung einer Aufgabe ändern",
  "l.whatif_view.add_a_link": "Verknüpfung hinzufügen",
  "l.whatif_view.remove_a_link": "Verknüpfung entfernen",
  "l.whatif_view.must_start_on_hard": "Muss beginnen am (hart)",
  "l.whatif_view.must_finish_on_hard": "Muss enden am (hart)",
  "l.work_view.fixed_units": "Feste Einheiten",
  "l.work_view.fixed_duration": "Feste Dauer",
  "l.work_view.fixed_work": "Feste Arbeit",
  "l.work_view.changing_the_work_changes_how":
    "Eine Änderung der Arbeit ändert die Dauer; der Tagesanteil bleibt.",
  "l.work_view.changing_the_work_changes_the":
    "Eine Änderung der Arbeit ändert den Tagesanteil; die Dauer bleibt.",
  "l.work_view.changing_the_duration_or_the":
    "Eine Änderung der Dauer oder des Tagesanteils ändert den jeweils anderen Wert; die Arbeit bleibt.",
  "l.work_view.flat_the_same_each_day": "Gleichmäßig: jeden Tag gleich",
  "l.work_view.front_loaded_more_at_the": "Frontlastig: mehr am Anfang",
  "l.work_view.back_loaded_more_at_the": "Endlastig: mehr am Ende",
  "l.work_view.bell_most_in_the_middle": "Glocke: das meiste in der Mitte",
  "l.work_view.turtle_ramps_up_holds_ramps":
    "Schildkröte: steigt an, hält, fällt ab",
  "l.work_view.custom_percentages": "Benutzerdefinierte Prozentsätze",
  "l.work_view.has_no_dates_yet": "hat noch keine Termine",
  "l.work_view.has_no_business_day_to": "fällt auf keinen Arbeitstag",
  "l.work_view.has_a_contour_that_gives":
    "hat ein Verlaufsprofil, das ihr kein Gewicht gibt",
  "l.work_view.no_rate_applies": "kein Satz anwendbar",
  "l.work_view.no_rate_applied_on_some":
    "an einigen Tagen kein Satz angewendet",
  "l.work_view.has_nobody_assigned": "hat niemanden zugewiesen",
  "p._layout_svelte.reviews": "Bewertungen",
  "p._layout_svelte.automations": "Automatisierungen",
  "p._layout_svelte.goals_okrs": "Ziele (OKRs)",
  "p._layout_svelte.onboarding": "Einarbeitung",
  "p.ceo.red": "Rot",
  "p.ceo.amber": "Gelb",
  "p.ceo.green": "Grün",
  "p.display.saved_on_this_device": "Auf diesem Gerät gespeichert.",
  "p.display.saved_to_your_account_so":
    "In Ihrem Konto gespeichert, sodass es Ihnen in andere Browser folgt.",
  "p.display.could_not_save_to_your":
    "Speichern in Ihrem Konto gerade nicht möglich; die Einstellung bleibt auf diesem Gerät erhalten.",
  "p.me.today": "Heute",
  "p.me.this_week": "Diese Woche",
  "p.me.later": "Später",
  "p.me.no_due_date": "Kein Fälligkeitsdatum",
  "p.plans.pid.leveling": "Kapazitätsabgleich",
  "p.plans.pid.cost_and_schedule_risk": "Kosten- und Terminrisiko",
  "p.plans.pid.board.todo": "Offen",
  "p.plans.pid.board.in_progress": "In Arbeit",
  "p.plans.pid.board.in_review": "In Prüfung",
  "p.common.business_day_one": "{n} Arbeitstag",
  "p.common.business_day_other": "{n} Arbeitstage",
  "p.common.calendar_day_one": "{n} Kalendertag",
  "p.common.calendar_day_other": "{n} Kalendertage",
  "l.milestone_view.rescheduled_one": "{count}-mal verschoben",
  "l.milestone_view.rescheduled_other": "{count}-mal verschoben",
  "l.milestone_view.open_of_linked_one":
    "{open_tasks} von {linked_tasks} Aufgabe noch offen",
  "l.milestone_view.open_of_linked_other":
    "{open_tasks} von {linked_tasks} Aufgaben noch offen",
  "l.cpm_view.restricted_one":
    "{n} Abhängigkeit zu einem Plan, den Sie nicht sehen können, ist in diesen Terminen nicht berücksichtigt.",
  "l.cpm_view.restricted_other":
    "{n} Abhängigkeiten zu einem Plan, den Sie nicht sehen können, sind in diesen Terminen nicht berücksichtigt.",
  "l.cpm_view.monday": "Montag",
  "l.cpm_view.tuesday": "Dienstag",
  "l.cpm_view.wednesday": "Mittwoch",
  "l.cpm_view.thursday": "Donnerstag",
  "l.cpm_view.friday": "Freitag",
  "l.cpm_view.saturday": "Samstag",
  "l.cpm_view.sunday": "Sonntag",
  "l.ev_view.gap_more":
    "{money} mehr Wert erwirtschaftet als bis jetzt geplant",
  "l.ev_view.gap_less":
    "{money} weniger Wert erwirtschaftet als bis jetzt geplant",
  "l.ev_view.cost_line_less":
    "{word}: CPI {ratio}, {money} weniger ausgegeben, als die Arbeit erwirtschaftet hat.",
  "l.ev_view.cost_line_more":
    "{word}: CPI {ratio}, {money} mehr ausgegeben, als die Arbeit erwirtschaftet hat.",
  "l.ev_view.tasks_counted_one":
    "{count} Aufgabe gezählt, gewichtet {weighting}; {how}",
  "l.ev_view.tasks_counted_other":
    "{count} Aufgaben gezählt, gewichtet {weighting}; {how}",
  "l.ev_view.equally": "gleichmäßig",
  "l.onboarding_view.would_move_one": "{n} Element würde an {to} übergehen.",
  "l.onboarding_view.would_move_other":
    "{n} Elemente würden an {to} übergehen.",
  "l.onboarding_view.would_release_one":
    "{n} Element würde freigegeben: Aufgaben und Risiken ohne Zuweisung, Buchungen und Rollen entfernt.",
  "l.onboarding_view.would_release_other":
    "{n} Elemente würden freigegeben: Aufgaben und Risiken ohne Zuweisung, Buchungen und Rollen entfernt.",
  "l.whatif_view.set_duration_one":
    "„{name}“ auf {duration_days} Arbeitstag setzen",
  "l.whatif_view.set_duration_other":
    "„{name}“ auf {duration_days} Arbeitstage setzen",
  "l.work_view.over_capacity_one":
    "Über der Kapazität an {days_over} Kalendertag ({from} bis {to}): Dies verlangt mehr, als die Person leisten kann.",
  "l.work_view.over_capacity_other":
    "Über der Kapazität an {days_over} Kalendertagen ({from} bis {to}): Dies verlangt mehr, als die Person leisten kann.",
  "p.plans.pid.handover.done_one":
    "Erledigt: {count} Element übergeben und erfasst.",
  "p.plans.pid.handover.done_other":
    "Erledigt: {count} Elemente übergeben und erfasst.",
  "p.plans.pid.what_if.committed_one":
    "Festgeschrieben: {count} Änderung erfasst.",
  "p.plans.pid.what_if.committed_other":
    "Festgeschrieben: {count} Änderungen erfasst.",
  "l.import_view.mspdi_hint":
    "Verwenden Sie in Microsoft Project Datei → Speichern unter → XML-Format. Eine .mpp-Datei kann nicht gelesen werden.",
  "c.CalendarMonth.january": "Januar",
  "c.CalendarMonth.february": "Februar",
  "c.CalendarMonth.march": "März",
  "c.CalendarMonth.april": "April",
  "c.CalendarMonth.may": "Mai",
  "c.CalendarMonth.june": "Juni",
  "c.CalendarMonth.july": "Juli",
  "c.CalendarMonth.august": "August",
  "c.CalendarMonth.september": "September",
  "c.CalendarMonth.october": "Oktober",
  "c.CalendarMonth.november": "November",
  "c.CalendarMonth.december": "Dezember",
  "c.LilyGanttTable.schedule": "Zeitplan",
  "c.LilyKanbanTable.board": "Board",
  "l.announcement_view.yesterday": "gestern",
  "c.CalendarMonth.holiday_state": "Feiertag",
  "c.CalendarMonth.absent_state": "abwesend",
  "l.ev_view.word_ahead": "voraus",
  "l.ev_view.word_behind": "im Rückstand",
  "l.ev_view.slipping": "verzögert sich",
  "l.import_view.task_one": "{n} Aufgabe",
  "l.import_view.task_other": "{n} Aufgaben",
  "l.import_view.dependency_one": "{n} Abhängigkeit",
  "l.import_view.dependency_other": "{n} Abhängigkeiten",
  "l.import_view.word_update": "aktualisieren",
  "l.import_view.word_updated": "aktualisiert",
  "l.rbs.headcount_one": "{headcount} Person",
  "l.rbs.headcount_other": "{headcount} Personen",
  "l.scheduling_view.exactly": "genau",
  "l.scheduling_view.strength_strong": "stark",
  "l.scheduling_view.strength_moderate": "mittel",
  "l.scheduling_view.strength_weak": "schwach",
  "l.simulation_inputs.planned_word": "geplant",
  "p.plans.pid.board.unassigned": "nicht zugewiesen",
  "p.plans.pid.costs.recorded_one":
    "{recorded} Zeiteintrag erfasst; {already_recorded} waren bereits erfasst; {length} haben keinen Satz.",
  "p.plans.pid.costs.recorded_other":
    "{recorded} Zeiteinträge erfasst; {already_recorded} waren bereits erfasst; {length} haben keinen Satz.",
  "c.CostRiskView.histogram_label":
    "Histogramm der simulierten Kosten in {currency}, mit P50, P80, P90 und dem Budget markiert",
  "c.CumulativeFlow.chart_label":
    "Kumulativer Fluss über {days} Kalendertage, mit einem Höchststand von {peak} Aufgaben",
  "p.plans.pid.onboarding.owner_of": "Verantwortliche Person für {title}",
  "p.plans.pid.onboarding.due_date_of": "Fälligkeitsdatum von {title}",
  "p.plans.pid.onboarding.why_skip_title": "Warum {title} überspringen",
  "p.plans.pid.what_if.remove_edit": "Änderung {n} entfernen",
  "c.LilyGanttTable.cell_active": "{name}: {start} bis {end}{ends}",
  "c.LilyGanttTable.ends_here": " (endet hier)",
  "c.LilyGanttTable.cell_not_scheduled": "{name}: nicht terminiert",
  "l.simulation_inputs.range_business_days":
    "{optimistic} / {likely} / {pessimistic} Arbeitstage (optimistisch / wahrscheinlich / pessimistisch)",
  "l.enum.task_status.todo": "offen",
  "l.enum.task_status.in_progress": "in Arbeit",
  "l.enum.task_status.in_review": "in Prüfung",
  "l.enum.task_status.done": "erledigt",
  "l.enum.task_status.blocked": "blockiert",
  "l.enum.kind.Portfolio": "Portfolio",
  "l.enum.kind.Project": "Projekt",
  "l.enum.kind.Product": "Produkt",
  "l.enum.kind.Program": "Programm",
  "l.enum.kind.Practice": "Praxis",
  "l.enum.kind.Process": "Prozess",
  "l.enum.kind.Purpose": "Zweck",
  "l.enum.kind.Pathway": "Pfad",
  "l.enum.kind.Proposal": "Vorschlag",
  "l.enum.plan_status.Proposed": "Vorgeschlagen",
  "l.enum.plan_status.Active": "Aktiv",
  "l.enum.plan_status.OnHold": "Pausiert",
  "l.enum.plan_status.Completed": "Abgeschlossen",
  "l.enum.plan_status.Cancelled": "Abgebrochen",
  "l.enum.collection.portfolios": "Portfolios",
  "l.enum.collection.projects": "Projekte",
  "l.enum.collection.products": "Produkte",
  "l.enum.collection.programs": "Programme",
  "l.enum.collection.practices": "Praktiken",
  "l.enum.collection.processes": "Prozesse",
  "l.enum.collection.purposes": "Zwecke",
  "l.enum.collection.pathways": "Pfade",
  "l.enum.proposal_status.draft": "Entwurf",
  "l.enum.proposal_status.submitted": "eingereicht",
  "l.enum.proposal_status.in_review": "in Prüfung",
  "l.enum.proposal_status.approved": "genehmigt",
  "l.enum.proposal_status.rejected": "abgelehnt",
  "l.enum.proposal_status.promoted": "übernommen",
  "l.enum.band.high": "hoch",
  "l.enum.band.medium": "mittel",
  "l.enum.band.low": "niedrig",
  "l.enum.band.unscored": "unbewertet",
  "l.enum.api.unauthorized": "nicht angemeldet",
  "l.enum.api.forbidden": "nicht erlaubt",
  "l.enum.api.not_found": "nicht gefunden",
  "l.enum.api.internal_error": "Interner Fehler",
  "l.enum.api.rate_limited": "Anfragelimit erreicht",
  "l.enum.api.http_status": "HTTP {status}",
  "p.error.not_found": "Diese Seite existiert nicht.",
  "p.error.not_allowed": "Sie dürfen diese Seite nicht sehen.",
  "p.error.went_wrong":
    "Etwas ist schiefgelaufen. Bitte versuchen Sie es erneut.",
  "p.error.title": "Fehler {status} — PPM",
  "p.error.back": "Zurück zum Anfang",
  "p.issues.issues_ppm": "Problemregister — PPM",
  "p.issues.issues": "Problemregister",
  "p.issues.intro":
    "Ein Problem ist etwas, das bereits eingetreten ist und bearbeitet werden muss; ein Risiko ist etwas, das eintreten könnte. Schwerwiegendste zuerst, dann älteste. Das Alter wird in Kalendertagen seit der Meldung angegeben.",
  "p.issues.could_not_load": "Probleme konnten nicht geladen werden",
  "p.issues.by_severity": "Probleme nach Schweregrad",
  "p.issues.overdue_count": "Überfällig",
  "p.issues.oldest_open": "Ältestes offenes: {title} ({days} Kalendertage)",
  "p.issues.show": "Anzeigen",
  "p.issues.all_statuses": "Alle Status",
  "p.issues.open_only": "Nur offene und in Arbeit",
  "p.issues.severity_filter": "Schweregrad",
  "p.issues.all_severities": "Alle Schweregrade",
  "p.issues.older_than": "Mindestens so viele Kalendertage alt",
  "p.issues.none": "Keine Probleme erfasst.",
  "p.issues.none_match": "Keine passenden Probleme.",
  "p.issues.loading": "Wird geladen…",
  "p.issues.count_one": "{n} Problem, schwerwiegendste zuerst",
  "p.issues.count_other": "{n} Probleme, schwerwiegendste zuerst",
  "p.issues.truncated":
    "Die ersten {n} werden angezeigt; grenzen Sie die Filter ein, um den Rest zu sehen.",
  "p.issues.title_col": "Problem",
  "p.issues.kind_col": "Art",
  "p.issues.severity_col": "Schweregrad",
  "p.issues.status_col": "Status",
  "p.issues.owner_col": "Verantwortlich",
  "p.issues.age_col": "Alter (Kalendertage)",
  "p.issues.due_col": "Fällig",
  "p.issues.overdue_flag": "Überfällig",
  "p.issues.no_owner": "Niemand verantwortlich",
  "p.issues.no_due": "Kein Fälligkeitsdatum",
  "p.issues.plan_link": "Probleme des Plans öffnen",
  "p.issues.plan_issues_link": "Probleme",
  "p.plan_issues.title_ppm": "Planprobleme — PPM",
  "p.plan_issues.title": "Planprobleme",
  "p.plan_issues.intro":
    "Die zu diesem Plan gemeldeten Probleme, schwerwiegendste zuerst. Ein Problem ist etwas, das bereits eingetreten ist; ein Risiko ist etwas, das eintreten könnte. Melden Sie ein Problem, bearbeiten Sie eines oder melden Sie eines aus einem eingetretenen Risiko.",
  "p.plan_issues.back": "Zurück zum Plan",
  "p.plan_issues.register_link": "Probleme über alle Pläne",
  "p.plan_issues.could_not_load":
    "Probleme des Plans konnten nicht geladen werden",
  "p.plan_issues.could_not_save": "Problem konnte nicht gespeichert werden",
  "p.plan_issues.counts":
    "{total} insgesamt, {open} offen oder in Arbeit, {overdue} überfällig",
  "p.plan_issues.actions_col": "Aktionen",
  "p.plan_issues.edit": "Bearbeiten",
  "p.plan_issues.raise_heading": "Problem melden",
  "p.plan_issues.edit_heading": "Problem bearbeiten",
  "p.plan_issues.f_title": "Titel",
  "p.plan_issues.f_description": "Beschreibung",
  "p.plan_issues.f_owner": "Verantwortlich (ein person:- oder worker:-Verweis)",
  "p.plan_issues.f_raised_on": "Gemeldet am (leer lassen für heute)",
  "p.plan_issues.f_resolution":
    "Lösung (zum Schließen des Problems erforderlich)",
  "p.plan_issues.raise": "Problem melden",
  "p.plan_issues.save": "Änderungen speichern",
  "p.plan_issues.cancel_edit": "Bearbeitung abbrechen",
  "p.plan_issues.raised": "Das Problem wurde gemeldet.",
  "p.plan_issues.saved": "Das Problem wurde gespeichert.",
  "p.plan_issues.problem_title_required": "Geben Sie dem Problem einen Titel.",
  "p.plan_issues.problem_resolution_required":
    "Ein geschlossenes Problem benötigt eine Lösung: Geben Sie an, wie es erledigt wurde.",
  "p.plan_issues.problem_due_before_raised":
    "Das Fälligkeitsdatum darf nicht vor dem Tag der Meldung liegen.",
  "p.plan_issues.from_risk_heading": "Problem aus einem Risiko melden",
  "p.plan_issues.from_risk_intro":
    "Für ein eingetretenes Risiko. Das Problem übernimmt Titel, Beschreibung und Verantwortliche des Risikos sowie einen Schweregrad aus seiner Exposition; das Risiko selbst bleibt unverändert. Ein Risiko kann jeweils nur ein offenes Problem haben.",
  "p.plan_issues.f_risk": "Risiko",
  "p.plan_issues.choose_risk": "Risiko auswählen",
  "p.plan_issues.raise_from_risk": "Problem aus diesem Risiko melden",
  "p.plan_issues.raised_from_risk":
    "Das Problem wurde aus dem Risiko gemeldet.",
  "l.enum.severity.low": "niedrig",
  "l.enum.severity.med": "mittel",
  "l.enum.severity.high": "hoch",
  "l.enum.severity.critical": "kritisch",
  "l.enum.issue_status.open": "offen",
  "l.enum.issue_status.in_progress": "in Arbeit",
  "l.enum.issue_status.resolved": "gelöst",
  "l.enum.issue_status.closed": "geschlossen",
  "l.enum.issue_kind.bug": "Fehler",
  "l.enum.issue_kind.risk": "Risiko",
  "l.enum.issue_kind.blocker": "Blocker",
  "l.enum.issue_kind.question": "Frage",
  "l.enum.issue_kind.improvement": "Verbesserung",
  "l.enum.issue_kind.concern": "Bedenken",
  "p.signin.oidc_idle":
    "Sie wurden abgemeldet, weil Sie eine Weile inaktiv waren. Bitte melden Sie sich erneut an.",
  "p.plans.pid.board.only_first_tasks":
    "Dieser Plan hat {total} Aufgaben; das Board zeigt die ersten {shown}. Grenzen Sie es nach Komponente ein oder lesen Sie den Rest über die API.",
  "p.plan_modules.title": "Werkzeuge dieses Plans",
  "p.plan_modules.title_ppm": "Werkzeuge dieses Plans — PPM",
  "p.plan_modules.back": "← Zurück zum Plan",
  "p.plan_modules.intro":
    "Wählen Sie, welche Werkzeuge dieser Plan zeigt. Ein ausgeschaltetes Werkzeug ist auf der Planseite ausgeblendet, und seine Seiten antworten mit „nicht gefunden“. Nichts wird gelöscht: Wird ein Werkzeug wieder eingeschaltet, zeigt es, was es enthielt.",
  "p.plan_modules.source_plan":
    "Dieser Plan hat eine eigene Auswahl an Werkzeugen.",
  "p.plan_modules.source_default":
    "Dieser Plan nutzt die Standardwerkzeuge der Installation.",
  "p.plan_modules.legend": "Werkzeuge, die dieser Plan zeigt",
  "p.plan_modules.module_tasks": "Aufgaben, das Board und die Terminanalyse",
  "p.plan_modules.module_time": "Zeiterfassung",
  "p.plan_modules.module_risks": "Risiken und Probleme",
  "p.plan_modules.module_financials": "Budgets, Kosten und Fertigstellungswert",
  "p.plan_modules.module_milestones":
    "Meilensteine, der Terminplan und der Kalender",
  "p.plan_modules.module_governance": "Governance, Gates und Änderungsanträge",
  "p.plan_modules.module_roadmap": "Öffentliche Produkt-Roadmap",
  "p.plan_modules.none_chosen":
    "Ohne gewähltes Werkzeug zeigt die Planseite ihren Datensatz und keine Werkzeuge.",
  "p.plan_modules.save": "Werkzeuge speichern",
  "p.plan_modules.presets_heading": "Fertige Auswahlen",
  "p.plan_modules.personal_is":
    "Die persönliche Auswahl, für die eigene Arbeit einer Person: {names}.",
  "p.plan_modules.use_personal": "Persönliche Auswahl verwenden",
  "p.plan_modules.default_is": "Der Standard der Installation: {names}.",
  "p.plan_modules.use_default": "Dem Standard der Installation folgen",
  "p.plan_modules.no_tools": "keine Werkzeuge",
  "p.plan_modules.unchanged":
    "Nichts zu speichern: Dieser Plan folgt bereits dem Standard, der dieser Auswahl entspricht.",
  "p.plan_modules.saved": "Gespeichert.",
  "p.plan_modules.could_not_load":
    "Die Werkzeuge dieses Plans konnten nicht geladen werden.",
  "p.plan_modules.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.canvas.title": "Strategie-Canvases",
  "p.canvas.title_ppm": "Strategie-Canvases — PPM",
  "p.canvas.intro":
    "Ein Canvas ist eine einseitige Tafel aus Feldern, etwa ein Lean-Canvas oder eine SWOT-Analyse. Ein Eintrag darauf kann zu einer Idee, einem Ziel oder, auf dem Canvas eines Plans, zu einem Risiko werden.",
  "p.canvas.plan_title": "Canvases dieses Plans",
  "p.canvas.plan_title_ppm": "Canvases dieses Plans — PPM",
  "p.canvas.plan_intro":
    "Die Strategie-Canvases dieses Plans. Ein Eintrag auf einem davon kann zu einem Risiko im Register dieses Plans werden.",
  "p.canvas.one_title": "Strategie-Canvas",
  "p.canvas.one_title_ppm": "Strategie-Canvas — PPM",
  "p.canvas.back": "← Zurück zu den Canvases",
  "p.canvas.list_heading": "Alle Canvases",
  "p.canvas.filter": "Anzeigen",
  "p.canvas.filter_all": "Alle Arten",
  "p.canvas.none_yet": "Es gibt noch keine Canvases.",
  "p.canvas.none_of_kind": "Es gibt keine Canvases dieser Art.",
  "p.canvas.free_standing": "zu keinem Plan",
  "p.canvas.on_plan": "zu einem Plan",
  "p.canvas.new_heading": "Canvas beginnen",
  "p.canvas.new_on_plan": "Der neue Canvas gehört zu diesem Plan.",
  "p.canvas.new_free_standing":
    "Der neue Canvas gehört zu keinem Plan; seine Einträge können also zu Ideen oder Zielen werden, aber nicht zu Risiken.",
  "p.canvas.f_template": "Art des Canvas",
  "p.canvas.f_title": "Titel",
  "p.canvas.f_description": "Beschreibung (optional)",
  "p.canvas.create": "Canvas beginnen",
  "p.canvas.could_not_load_list": "Die Canvases konnten nicht geladen werden.",
  "p.canvas.could_not_load_templates":
    "Die Arten von Canvas konnten nicht geladen werden, daher kann kein neuer Canvas begonnen werden.",
  "p.canvas.could_not_create": "Der Canvas wurde nicht begonnen.",
  "p.canvas.problem_title_blank": "Geben Sie dem Canvas einen Titel.",
  "p.canvas.problem_title_long":
    "Ein Titel darf höchstens 200 Zeichen lang sein.",
  "p.canvas.problem_description_long":
    "Eine Beschreibung darf höchstens 2.000 Zeichen lang sein.",
  "p.canvas.template_lean": "Lean-Canvas",
  "p.canvas.template_business_model": "Geschäftsmodell-Canvas",
  "p.canvas.template_swot": "SWOT-Analyse",
  "p.canvas.template_empathy": "Empathiekarte",
  "p.canvas.template_logic_model": "Wirkungsmodell",
  "p.canvas.template_risk": "Risiko-Canvas",
  "p.canvas.template_goal": "Ziel-Canvas",
  "p.canvas.box_problem": "Problemstellung",
  "p.canvas.box_solution": "Lösung",
  "p.canvas.box_key_metrics": "Schlüsselkennzahlen",
  "p.canvas.box_unique_value_proposition": "Alleinstellungsmerkmal",
  "p.canvas.box_unfair_advantage": "Unfairer Vorteil",
  "p.canvas.box_channels": "Kanäle",
  "p.canvas.box_customer_segments": "Kundensegmente",
  "p.canvas.box_cost_structure": "Kostenstruktur",
  "p.canvas.box_revenue_streams": "Einnahmequellen",
  "p.canvas.box_key_partners": "Schlüsselpartner",
  "p.canvas.box_key_activities": "Schlüsselaktivitäten",
  "p.canvas.box_key_resources": "Schlüsselressourcen",
  "p.canvas.box_value_propositions": "Wertangebote",
  "p.canvas.box_customer_relationships": "Kundenbeziehungen",
  "p.canvas.box_strengths": "Stärken",
  "p.canvas.box_weaknesses": "Schwächen",
  "p.canvas.box_opportunities": "Chancen",
  "p.canvas.box_threats": "Bedrohungen",
  "p.canvas.box_says": "Sagt",
  "p.canvas.box_thinks": "Denkt",
  "p.canvas.box_does": "Tut",
  "p.canvas.box_feels": "Fühlt",
  "p.canvas.box_pains": "Schmerzpunkte",
  "p.canvas.box_gains": "Gewinne",
  "p.canvas.box_inputs": "Ressourceneinsatz",
  "p.canvas.box_activities": "Aktivitäten",
  "p.canvas.box_outputs": "Leistungen",
  "p.canvas.box_outcomes": "Wirkungen",
  "p.canvas.box_impact": "Langfristige Wirkung",
  "p.canvas.box_vulnerabilities": "Schwachstellen",
  "p.canvas.box_existing_controls": "Bestehende Kontrollen",
  "p.canvas.box_residual_risks": "Restrisiken",
  "p.canvas.box_goal": "Ziel",
  "p.canvas.box_success_measures": "Erfolgsmaße",
  "p.canvas.box_actions": "Maßnahmen",
  "p.canvas.box_obstacles": "Hindernisse",
  "p.canvas.box_resources": "Ressourcen",
  "p.canvas.box_empty": "Hier steht noch nichts.",
  "p.canvas.f_new_item": "Neuer Eintrag in {box}",
  "p.canvas.add": "Hinzufügen",
  "p.canvas.added": "Hinzugefügt.",
  "p.canvas.f_text": "Wortlaut",
  "p.canvas.f_box": "Feld",
  "p.canvas.save": "Speichern",
  "p.canvas.cancel": "Abbrechen",
  "p.canvas.saved": "Gespeichert.",
  "p.canvas.unchanged": "Nichts wurde geändert, daher wurde nichts gesendet.",
  "p.canvas.edit": "Bearbeiten",
  "p.canvas.remove": "Entfernen",
  "p.canvas.removed": "Entfernt.",
  "p.canvas.confirm_remove_item": "„{text}“ vom Canvas entfernen?",
  "p.canvas.promote": "Umwandeln in…",
  "p.canvas.promote_legend": "Diesen Eintrag umwandeln in",
  "p.canvas.target_idea": "eine Idee",
  "p.canvas.target_objective": "ein Ziel",
  "p.canvas.target_risk": "ein Risiko dieses Plans",
  "p.canvas.promoted": "Umgewandelt in {kind}.",
  "p.canvas.promote_once": "Ein Eintrag kann nur einmal umgewandelt werden.",
  "p.canvas.promote_go": "Darin umwandeln",
  "p.canvas.no_risk_free_standing":
    "Dieser Canvas gehört zu keinem Plan, daher kann ein Eintrag kein Risiko werden.",
  "p.canvas.f_probability": "Wahrscheinlichkeit (1 bis 5)",
  "p.canvas.f_impact": "Auswirkung (1 bis 5)",
  "p.canvas.problem_risk_scores":
    "Ein Risiko braucht eine Wahrscheinlichkeit und eine Auswirkung, jeweils eine ganze Zahl von 1 bis 5.",
  "p.canvas.problem_text_blank": "Schreiben Sie zuerst den Text des Eintrags.",
  "p.canvas.problem_text_long":
    "Ein Eintrag darf höchstens 2.000 Zeichen lang sein.",
  "p.canvas.became_idea": "Wurde zu einer Idee",
  "p.canvas.became_risk": "Wurde zu einem Risiko dieses Plans",
  "p.canvas.became_objective": "Wurde zu einem Ziel",
  "p.canvas.owner": "Verantwortlich: {owner}",
  "p.canvas.full":
    "Dieser Canvas enthält die größtmögliche Zahl an Einträgen ({max}); entfernen Sie einen, um einen weiteren hinzuzufügen.",
  "p.canvas.could_not_load": "Dieser Canvas konnte nicht geladen werden.",
  "p.canvas.could_not_add": "Der Eintrag wurde nicht hinzugefügt.",
  "p.canvas.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.canvas.could_not_remove": "Der Eintrag wurde nicht entfernt.",
  "p.canvas.could_not_promote": "Der Eintrag wurde nicht umgewandelt.",
  "p.canvas.remove_canvas_heading": "Canvas entfernen",
  "p.canvas.remove_canvas_says":
    "Wer den Canvas entfernt, entfernt seine Einträge. Die Ideen, Ziele und Risiken, zu denen seine Einträge wurden, bleiben.",
  "p.canvas.remove_canvas": "Diesen Canvas entfernen",
  "p.canvas.confirm_remove_canvas":
    "Den Canvas „{title}“ und seine Einträge entfernen?",
  "p.canvas.could_not_remove_canvas": "Der Canvas wurde nicht entfernt.",
  "p.plan_templates.title": "Planvorlagen",
  "p.plan_templates.title_ppm": "Planvorlagen — PPM",
  "p.plan_templates.intro":
    "Eine Vorlage ist ein Plan mit den Plänen darunter, gespeichert ohne Personen und mit jedem Datum als Anzahl Tage nach dem Start, damit neue Pläne von ihr ausgehen können.",
  "p.plan_templates.saved_heading": "Gespeicherte Vorlagen",
  "p.plan_templates.starters_heading": "Mitgelieferte Startvorlagen",
  "p.plan_templates.none_saved":
    "Noch keine Vorlage gespeichert. Speichern Sie eine von der Seite eines Plans aus.",
  "p.plan_templates.none_starters":
    "Es gibt keine mitgelieferten Startvorlagen.",
  "p.plan_templates.plans_count_one": "{count} Plan",
  "p.plan_templates.plans_count_other": "{count} Pläne",
  "p.plan_templates.needs_start": "braucht ein Startdatum",
  "p.plan_templates.could_not_load_list":
    "Die Vorlagen konnten nicht geladen werden.",
  "p.plan_templates.one_title": "Planvorlage",
  "p.plan_templates.one_title_ppm": "Planvorlage — PPM",
  "p.plan_templates.back": "← Zurück zu den Vorlagen",
  "p.plan_templates.is_starter":
    "Eine mitgelieferte Startvorlage. Sie kann nicht geändert oder entfernt werden; beginnen Sie einen Plan daraus und speichern Sie diesen unter eigenem Namen.",
  "p.plan_templates.is_saved": "Eine gespeicherte Vorlage, Version {version}.",
  "p.plan_templates.holds_heading": "Was sie enthält",
  "p.plan_templates.holds_says":
    "Jeder Plan, die Zahl seiner Aufgaben und seine Meilensteine als Tage nach dem Start. Keine Person wird gespeichert.",
  "p.plan_templates.tasks_count_one": "{count} Aufgabe",
  "p.plan_templates.tasks_count_other": "{count} Aufgaben",
  "p.plan_templates.milestone_at": "{name}: Tag {days}",
  "p.plan_templates.start_heading": "Pläne daraus beginnen",
  "p.plan_templates.start_says":
    "Neue Pläne werden aus der Vorlage erstellt, die Daten ab dem Startdatum gezählt. Aufgaben beginnen in ihrem ersten Schritt.",
  "p.plan_templates.f_plan_name": "Name des obersten Plans (optional)",
  "p.plan_templates.f_start": "Startdatum",
  "p.plan_templates.no_dates":
    "Diese Vorlage enthält keine Daten, daher ist kein Startdatum nötig.",
  "p.plan_templates.f_parent": "Darunter einordnen",
  "p.plan_templates.no_parent": "Kein Plan: ein neuer Plan auf oberster Ebene",
  "p.plan_templates.start": "Pläne beginnen",
  "p.plan_templates.started":
    "Erstellt. Pläne: {plans}; Aufgaben: {tasks}; Meilensteine: {milestones}.",
  "p.plan_templates.open_new": "Den neuen Plan öffnen",
  "p.plan_templates.could_not_load":
    "Diese Vorlage konnte nicht geladen werden.",
  "p.plan_templates.could_not_start": "Die Pläne wurden nicht begonnen.",
  "p.plan_templates.problem_plan_name_long":
    "Der Name eines Plans darf höchstens 1.024 Zeichen lang sein.",
  "p.plan_templates.problem_start_needed":
    "Wählen Sie ein Startdatum: Diese Vorlage enthält Daten.",
  "p.plan_templates.problem_start_bad":
    "Schreiben Sie das Startdatum als Tag, etwa 2027-01-04.",
  "p.plan_templates.remove_heading": "Vorlage entfernen",
  "p.plan_templates.remove_says":
    "Das Entfernen der Vorlage ändert die bereits daraus erstellten Pläne nicht.",
  "p.plan_templates.remove": "Diese Vorlage entfernen",
  "p.plan_templates.confirm_remove": "Die Vorlage „{name}“ entfernen?",
  "p.plan_templates.could_not_remove": "Die Vorlage wurde nicht entfernt.",
  "p.plan_templates.save_title": "Als Vorlage speichern",
  "p.plan_templates.save_title_ppm": "Als Vorlage speichern — PPM",
  "p.plan_templates.save_says":
    "Speichert diesen Plan und die Pläne darunter, die Sie lesen dürfen, mit ihren Aufgaben, Komponenten, Meilensteinen und Abläufen. Keine Person wird gespeichert, und jedes Datum wird zu einer Anzahl Tage nach dem Start. Spätere Änderungen am Plan ändern die Vorlage nicht.",
  "p.plan_templates.f_name": "Name der Vorlage",
  "p.plan_templates.f_description": "Beschreibung (optional)",
  "p.plan_templates.save": "Vorlage speichern",
  "p.plan_templates.saved":
    "Gespeichert. Pläne: {plans}; Aufgaben: {tasks}; Meilensteine: {milestones}.",
  "p.plan_templates.omitted_one":
    "{count} Plan darunter wurde ausgelassen, weil Sie ihn nicht lesen dürfen.",
  "p.plan_templates.omitted_other":
    "{count} Pläne darunter wurden ausgelassen, weil Sie sie nicht lesen dürfen.",
  "p.plan_templates.open_template": "Die Vorlage öffnen",
  "p.plan_templates.name_taken":
    "Eine Vorlage mit diesem Namen gibt es schon. Wählen Sie einen anderen Namen.",
  "p.plan_templates.could_not_save": "Die Vorlage wurde nicht gespeichert.",
  "p.plan_templates.problem_name_blank": "Geben Sie der Vorlage einen Namen.",
  "p.plan_templates.problem_name_long":
    "Der Name einer Vorlage darf höchstens 120 Zeichen lang sein.",
  "p.plan_export.title": "Nach Microsoft Project exportieren",
  "p.plan_export.title_ppm": "Nach Microsoft Project exportieren — PPM",
  "p.plan_export.intro":
    "Speichert die Aufgaben dieses Plans als MSPDI-XML-Datei, das Format, das Microsoft Project und andere Planungswerkzeuge öffnen.",
  "p.plan_export.carried_heading": "Was die Datei enthält",
  "p.plan_export.carried":
    "Jede Aufgabe mit ihrer Gliederung unter ihrer übergeordneten Aufgabe, Startdatum, Dauer in Arbeitstagen, Meilensteinen, Einschränkungen, Stichtag und Notizen; ob sie erledigt ist; die Verknüpfungen zwischen Aufgaben mit Art und Verzögerung; und den Arbeitskalender des Plans.",
  "p.plan_export.not_carried_heading": "Was fehlt",
  "p.plan_export.not_carried":
    "Zugewiesene Personen und Ressourcen; jeder Status außer erledigt (eine Aufgabe ist zu 0 oder 100 Prozent abgeschlossen); Sprints und Komponenten; Aufwand; benutzerdefinierte Felder; Kommentare; Basispläne; Verknüpfungen außer „geht voraus“; und Verzögerungen in verstrichener Zeit (jede Verzögerung zählt in Arbeitstagen). Die Datei sagt das auch.",
  "p.plan_export.limit":
    "Ein Plan mit mehr als 5.000 Aufgaben kann nicht exportiert werden.",
  "p.plan_export.not_checked":
    "Die Datei wurde vom eigenen Import dieses Dienstes zurückgelesen, aber noch nicht in Microsoft Project selbst geöffnet.",
  "p.plan_export.download": "MSPDI-Datei herunterladen",
  "p.plan_export.to_import":
    "Stattdessen eine MSPDI-Datei in diesen Plan holen",
  "p.confidence.assess_heading": "Einschätzung hinzufügen",
  "p.confidence.assess_says":
    "Ein genannter Wert ist die Sicht einer Person darauf, ob der Plan liefern wird. Er wird dem Verlauf hinzugefügt und ersetzt nie einen früheren.",
  "p.confidence.choose_value": "Wert wählen",
  "p.confidence.could_not_load":
    "Die Liefersicherheit dieses Plans konnte nicht geladen werden.",
  "p.confidence.could_not_save": "Die Einschätzung wurde nicht hinzugefügt.",
  "p.confidence.f_assessed_on": "Eingeschätzt am (optional; leer heißt heute)",
  "p.confidence.f_kind": "Wer sagt es",
  "p.confidence.f_rationale": "Warum (mindestens {min} Zeichen)",
  "p.confidence.f_value": "Liefersicherheit",
  "p.confidence.history_heading": "Verlauf, neueste zuerst",
  "p.confidence.intro":
    "Die genannte Sicht einer Person darauf, ob dieser Plan liefern wird, neben der aus den Plandaten berechneten Ampel gehalten und nie mit ihr vermischt.",
  "p.confidence.kind_independent": "Eine unabhängige Einschätzung",
  "p.confidence.kind_self": "Das eigene Team des Plans (Selbsteinschätzung)",
  "p.confidence.no_history": "Noch wurde keine Einschätzung genannt.",
  "p.confidence.none_yet":
    "Nicht eingeschätzt: Noch niemand hat eine Sicht genannt. Das ist nicht dasselbe wie Grün.",
  "p.confidence.opens_exception":
    "Gelb/Rot oder Rot eröffnet eine Ausnahme für diesen Plan oder verschärft die schon offene. Danach muss jemand sie vervollständigen.",
  "p.confidence.problem_future":
    "Das Datum der Einschätzung darf nicht in der Zukunft liegen.",
  "p.confidence.problem_kind": "Wählen Sie, wer es sagt.",
  "p.confidence.problem_rationale":
    "Begründen Sie es mit mindestens {min} Zeichen.",
  "p.confidence.problem_value": "Wählen Sie einen Wert.",
  "p.confidence.refused_independent":
    "Eine unabhängige Einschätzung erfordert das Recht als unabhängiger Prüfer. Sie können eine Selbsteinschätzung hinzufügen.",
  "p.confidence.save": "Einschätzung hinzufügen",
  "p.confidence.saved_exception_one":
    "Hinzugefügt. Dieser Plan hat jetzt eine offene Ausnahme: Sie ist innerhalb von {hours} Stunde zu vervollständigen.",
  "p.confidence.saved_exception_other":
    "Hinzugefügt. Dieser Plan hat jetzt eine offene Ausnahme: Sie ist innerhalb von {hours} Stunden zu vervollständigen.",
  "p.confidence.saved": "Hinzugefügt.",
  "p.confidence.shown_heading": "Was die Portfoliokarte zeigt",
  "p.confidence.shown_independent": "Unabhängige Einschätzung vom {when}.",
  "p.confidence.shown_self":
    "Selbsteinschätzung vom {when}; eine unabhängige Einschätzung würde stattdessen gezeigt.",
  "p.confidence.stale": "veraltet: älter als dieser Monat",
  "p.confidence.title_ppm": "Liefersicherheit — PPM",
  "p.confidence.title": "Liefersicherheit",
  "p.exceptions.a_plan": "Ein Plan",
  "p.exceptions.breached_one":
    "Verletzt: nicht innerhalb von {hours} Stunde nach der Eröffnung vervollständigt.",
  "p.exceptions.breached_other":
    "Verletzt: nicht innerhalb von {hours} Stunden nach der Eröffnung vervollständigt.",
  "p.exceptions.cancel": "Abbrechen",
  "p.thresholds.could_not_load":
    "Triage und Schwellen konnten nicht geladen werden.",
  "p.thresholds.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.thresholds.could_not_sweep": "Die Schwellen konnten nicht geprüft werden.",
  "p.thresholds.decide_says":
    "Die Entscheidung zu erfassen schließt die Ausnahme. Sie hält fest, was entschieden wurde; sie pausiert, setzt nicht zurück und stoppt das Programm nicht, das tut eine Person über den Programmstatus.",
  "p.thresholds.decide": "Entscheidung erfassen",
  "p.thresholds.decided_done": "Entscheidung erfasst: {action}.",
  "p.thresholds.f_action": "Entscheidung",
  "p.thresholds.f_amount": "Betrag der Obergrenze",
  "p.thresholds.f_count":
    "Veröffentlichte Pakete in Folge auf Rot ({min} bis {max})",
  "p.thresholds.f_currency": "Währung (drei Buchstaben)",
  "p.thresholds.f_date":
    "Prognostiziertes Enddatum, das nicht überschritten werden darf",
  "p.thresholds.f_expiry":
    "Übersteuerung bis (in der Zukunft, höchstens {days} Tage voraus)",
  "p.thresholds.f_outcome": "Ergebnis",
  "p.thresholds.f_reason": "Begründung (mindestens {min} Zeichen)",
  "p.thresholds.history_item": "Am {date}: {outcome}. {reason}",
  "p.thresholds.history": "Frühere Triagen",
  "p.thresholds.not_set": "Nicht festgelegt",
  "p.thresholds.now_cap":
    "Überschritten, wenn die P80-Kosten über {amount} liegen.",
  "p.thresholds.now_forecast":
    "Überschritten, wenn das prognostizierte Ende nach dem {date} liegt.",
  "p.thresholds.now_red_packs_one":
    "Überschritten, wenn die Zuversicht in {n} veröffentlichtem Paket in Folge rot ist.",
  "p.thresholds.now_red_packs_other":
    "Überschritten, wenn die Zuversicht in {n} veröffentlichten Paketen in Folge rot ist.",
  "p.thresholds.opened_by_threshold":
    "Geöffnet, weil eine Schwelle des Programms überschritten wurde.",
  "p.thresholds.problem_action":
    "Wählen Sie Pause, Zurücksetzen, Stopp oder Übersteuerung.",
  "p.thresholds.problem_cap": "Die Obergrenze muss ein Betrag über null sein.",
  "p.thresholds.problem_count":
    "Die Zahl der Pakete muss eine ganze Zahl von {min} bis {max} sein.",
  "p.thresholds.problem_currency":
    "Die Währung muss ein dreistelliger ISO-4217-Code sein.",
  "p.thresholds.problem_date": "Wählen Sie ein Datum.",
  "p.thresholds.problem_expiry_long":
    "Eine Übersteuerung darf höchstens {days} Tage gelten.",
  "p.thresholds.problem_expiry_needed":
    "Eine Übersteuerung braucht ein Datum, bis zu dem sie gilt.",
  "p.thresholds.problem_expiry_past":
    "Das Datum einer Übersteuerung muss in der Zukunft liegen.",
  "p.thresholds.problem_outcome":
    "Wählen Sie Abschließen, Beheben, Pausieren oder Stoppen.",
  "p.thresholds.problem_reason":
    "Die Begründung muss mindestens {min} Zeichen haben.",
  "p.thresholds.remove_threshold": "Schwelle entfernen",
  "p.thresholds.save_threshold": "Schwelle speichern",
  "p.thresholds.save_triage": "Triage erfassen",
  "p.thresholds.sweep_done_one":
    "{checked} Schwellen geprüft; {n} Ausnahme geöffnet.",
  "p.thresholds.sweep_done_other":
    "{checked} Schwellen geprüft; {n} Ausnahmen geöffnet.",
  "p.thresholds.sweep_item": "{programme} — {kind}: {outcome}",
  "p.thresholds.sweep_list": "Überschrittene Schwellen",
  "p.thresholds.sweep_says":
    "Die Prüfung öffnet für jede überschrittene Schwelle eine Lieferausnahme oder ergänzt die bereits offene; sie pausiert, setzt nicht zurück und stoppt nichts.",
  "p.thresholds.sweep": "Schwellen jetzt prüfen",
  "p.thresholds.threshold_removed": "Schwelle entfernt: {kind}.",
  "p.thresholds.threshold_saved": "Schwelle gespeichert: {kind}.",
  "p.thresholds.thresholds_heading": "Schwellen",
  "p.thresholds.thresholds_says":
    "Eine überschrittene Schwelle öffnet eine Lieferausnahme, die eine Person mit Pause, Zurücksetzen, Stopp oder einer befristeten Übersteuerung beantwortet. Nichts wird automatisch pausiert, zurückgesetzt oder gestoppt.",
  "p.thresholds.triage_heading": "Triage des Programms",
  "p.thresholds.triage_latest": "Letzte Triage, {date}: {outcome}. {reason}",
  "p.thresholds.triage_none": "Noch keine Triage erfasst.",
  "p.thresholds.triage_saved": "Triage erfasst.",
  "p.thresholds.triage_says":
    "Eine Triage ist eine datierte Einschätzung, wohin das Programm gehen soll, mit Begründung. Sie hält die Einschätzung fest; am Programm ändert sie nichts.",
  "p.thresholds.unchanged":
    "Nichts wurde geändert, daher wurde nichts gesendet.",
  "p.thresholds.action_pause": "Pausieren",
  "p.thresholds.action_reset": "Zurücksetzen",
  "p.thresholds.action_stop": "Stoppen",
  "p.thresholds.action_override": "Übersteuern",
  "p.thresholds.kind_confidence_red_packs":
    "Zuversicht über mehrere Pakete rot",
  "p.thresholds.kind_forecast_past": "Prognostiziertes Ende nach einem Datum",
  "p.thresholds.kind_p80_cap": "P80-Kosten über einer Obergrenze",
  "p.thresholds.outcome_finish": "Abschließen",
  "p.thresholds.outcome_fix": "Beheben",
  "p.thresholds.outcome_pause": "Pausieren",
  "p.thresholds.outcome_stop": "Stoppen",
  "p.thresholds.sweep_opened": "Ausnahme geöffnet",
  "p.thresholds.sweep_already_open": "Ausnahme bereits offen",
  "p.thresholds.sweep_attached_to_open_exception":
    "zur offenen Ausnahme hinzugefügt",
  "p.thresholds.sweep_suppressed_by_override":
    "nicht geöffnet: eine Übersteuerung gilt",
  "p.exceptions.closed_done":
    "Entscheidung festgehalten; die Ausnahme ist geschlossen.",
  "p.exceptions.complete_it": "Vervollständigen",
  "p.exceptions.complete_says":
    "Eine Ausnahme ist vollständig, wenn Ursache, nötige Entscheidung, Verantwortliche Person und Entscheidungsdatum vorliegen. Ein leer gelassenes Feld behält seinen Wert.",
  "p.exceptions.complete":
    "Vollständig: Ursache, nötige Entscheidung, Verantwortliche Person und Entscheidungsdatum liegen vor.",
  "p.exceptions.conflict_closed": "Diese Ausnahme ist schon geschlossen.",
  "p.exceptions.could_not_load": "Die Ausnahmen konnten nicht geladen werden.",
  "p.exceptions.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.exceptions.decide_it": "Entscheidung festhalten",
  "p.exceptions.decide_says":
    "Das Festhalten der Entscheidung schließt die Ausnahme. Eine bessere Liefersicherheit schließt sie nie von selbst.",
  "p.exceptions.decide": "Entscheidung festhalten und schließen",
  "p.exceptions.f_breached": "Nur verletzte",
  "p.exceptions.f_decision": "Entscheidung (mindestens {min} Zeichen)",
  "p.exceptions.f_owner_ref": "Verantwortlich (Personenverweis)",
  "p.exceptions.f_owner_role":
    "Rolle der verantwortlichen Person (was ein Paket zeigt)",
  "p.exceptions.f_status": "Anzeigen",
  "p.exceptions.intro_one":
    "Eine Ausnahme wird eröffnet, wenn die Liefersicherheit eines Plans Gelb/Rot oder Rot erreicht. Sie ist innerhalb von {hours} Stunde zu vervollständigen und wird nur durch eine festgehaltene Entscheidung geschlossen.",
  "p.exceptions.intro_other":
    "Eine Ausnahme wird eröffnet, wenn die Liefersicherheit eines Plans Gelb/Rot oder Rot erreicht. Sie ist innerhalb von {hours} Stunden zu vervollständigen und wird nur durch eine festgehaltene Entscheidung geschlossen.",
  "p.exceptions.is_closed": "geschlossen {when}",
  "p.exceptions.is_open": "offen",
  "p.exceptions.missing": "Es fehlt noch: {parts}.",
  "p.exceptions.none": "Keine Ausnahme passt.",
  "p.exceptions.opened": "Eröffnet {when}",
  "p.exceptions.part_cause": "Ursache",
  "p.exceptions.part_decide_by": "Entscheiden bis",
  "p.exceptions.part_decision_needed": "Nötige Entscheidung",
  "p.exceptions.part_decision": "Entscheidung",
  "p.exceptions.part_owner": "Verantwortlich",
  "p.exceptions.problem_decision":
    "Schreiben Sie die Entscheidung mit mindestens {min} Zeichen.",
  "p.exceptions.problem_part_short":
    "Ursache, nötige Entscheidung oder Verantwortlich brauchen mindestens {min} Zeichen, um zu zählen.",
  "p.exceptions.refused_threshold":
    "Diese Ausnahme wurde durch einen Schwellenwert eröffnet; sie wird durch Pause, Neustart, Stopp oder festgehaltene Übersteuerung geschlossen, nicht durch eine Entscheidung hier.",
  "p.exceptions.save": "Speichern",
  "p.exceptions.saved": "Gespeichert.",
  "p.exceptions.see_register": "Ausnahmenregister öffnen",
  "p.exceptions.status_all": "Alle Ausnahmen",
  "p.exceptions.status_closed": "Geschlossen",
  "p.exceptions.status_open": "Offen",
  "p.exceptions.title_ppm": "Ausnahmen — PPM",
  "p.exceptions.title": "Ausnahmen",
  "p.exceptions.unchanged":
    "Nichts wurde geändert, daher wurde nichts gesendet.",
  "p.exceptions.worsened_one": "Seit der Eröffnung {count}-mal verschlechtert",
  "p.exceptions.worsened_other":
    "Seit der Eröffnung {count}-mal verschlechtert",
  "p.exceptions.worst": "Schlechteste Liefersicherheit: {value}",
  "p.aims.aims_heading": "Strategische Ziele",
  "p.aims.aims_legend": "Ziele, die strategische Ziele sind",
  "p.aims.aims_says":
    "Ein strategisches Ziel ist ein Ziel, um das das Portfolio geordnet ist. Wird eines abgewählt, ist es auch für kein Programm mehr das Hauptziel.",
  "p.aims.choose_aim": "Ziel wählen",
  "p.aims.choose_programme": "Programm wählen",
  "p.aims.could_not_load": "Die Portfoliokarte konnte nicht geladen werden.",
  "p.aims.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.aims.f_aim": "Hauptziel",
  "p.aims.f_period": "Monat (leer für diesen Monat)",
  "p.aims.f_programme": "Programm",
  "p.aims.intro":
    "Die Programme, die Sie lesen dürfen, jedes einmal unter seinem strategischen Hauptziel. Die Liefersicherheit ist der genannte Wert, neben der aus den Daten berechneten Ampel gezeigt und nie mit ihr vermischt; nicht eingeschätzt ist nicht grün.",
  "p.aims.map_heading": "Portfoliokarte, {period}",
  "p.aims.marked": "Als strategisches Ziel markiert.",
  "p.aims.no_aims":
    "Noch kein Ziel ist ein strategisches Ziel; markieren Sie zuerst oben eines.",
  "p.aims.no_objectives": "Es gibt noch keine Ziele.",
  "p.aims.no_programmes":
    "Es gibt keine Programme auf der Karte: Ein Programm ist ein Plan der Art Programm.",
  "p.aims.primary_heading": "Hauptziel eines Programms",
  "p.aims.primary_says":
    "Ein Programm erscheint auf der Karte unter seinem einen Hauptziel; weitere Ziele, denen es dient, werden daneben genannt. Ein neues Hauptziel macht das frühere zu einer gewöhnlichen Verknüpfung.",
  "p.aims.primary_set": "Hauptziel festgelegt.",
  "p.aims.problem_already_primary":
    "Das ist schon das Hauptziel dieses Programms.",
  "p.aims.problem_choose": "Wählen Sie ein Programm und ein Ziel.",
  "p.aims.problem_period":
    "Wählen Sie den Monat als JJJJ-MM oder lassen Sie ihn leer für diesen Monat.",
  "p.aims.set_primary": "Hauptziel festlegen",
  "p.aims.show": "Anzeigen",
  "p.aims.title_ppm": "Portfoliokarte — PPM",
  "p.aims.title": "Portfoliokarte",
  "p.aims.to_objectives": "Ziele öffnen",
  "p.aims.unmarked": "Kein strategisches Ziel mehr.",
  "p.programme.cited_exception":
    "Sie folgte aus einer Lieferausnahme dieses Programms.",
  "p.programme.cited_review":
    "Sie folgte aus einer Gate-Prüfung dieses Programms.",
  "p.programme.cites_none": "Keines",
  "p.programme.conflict_status":
    "Das Programm hat diesen Status schon, oder es ist gestoppt und bleibt es.",
  "p.programme.could_not_decide": "Die Entscheidung wurde nicht festgehalten.",
  "p.programme.could_not_load":
    "Gates und Status dieses Plans konnten nicht geladen werden.",
  "p.programme.could_not_save_gates":
    "Nicht jedes Gate-Datum wurde gespeichert; gezeigt werden die Daten, die der Dienst hat.",
  "p.programme.decide": "Entscheidung festhalten",
  "p.programme.decided": "Festgehalten: Das Programm ist jetzt {status}.",
  "p.programme.do_pause": "Programm pausieren",
  "p.programme.do_resume": "Programm fortsetzen",
  "p.programme.do_stop": "Programm stoppen",
  "p.programme.f_change": "Programm ändern auf",
  "p.programme.f_exception": "Folgt aus einer Ausnahme (optional)",
  "p.programme.f_reason": "Begründung (mindestens {min} Zeichen)",
  "p.programme.f_review": "Folgt aus einer Gate-Prüfung (optional)",
  "p.programme.gates_heading": "Geplante Gate-Termine",
  "p.programme.gates_says":
    "Das Datum, an dem dieser Plan seine Prüfung an jedem Gate erwartet. Ein gesetztes Datum lässt sich nicht löschen, nur verschieben.",
  "p.programme.gates_saved": "Gate-Termine gespeichert.",
  "p.programme.gates_unchanged":
    "Kein Gate-Datum wurde geändert, daher wurde nichts gesendet.",
  "p.programme.held_allocations_one":
    "{count} Zuweisung ist markiert, nicht gelöscht.",
  "p.programme.held_allocations_other":
    "{count} Zuweisungen sind markiert, nicht gelöscht.",
  "p.programme.held_benefits_one": "{count} Nutzen bleibt erfasst.",
  "p.programme.held_benefits_other": "{count} Nutzen bleiben erfasst.",
  "p.programme.held_heading": "Was das Programm noch hält",
  "p.programme.held_money": "Geld: geplant {planned}, tatsächlich {actual}.",
  "p.programme.history_heading": "Entscheidungen, neueste zuerst",
  "p.programme.no_history": "Noch keine Entscheidung: Das Programm ist aktiv.",
  "p.programme.not_a_programme":
    "Nur ein Plan der Art Programm hat einen Programmstatus. Seine Gate-Termine lassen sich oben trotzdem planen.",
  "p.programme.problem_reason": "Begründen Sie mit mindestens {min} Zeichen.",
  "p.programme.problem_status": "Wählen Sie die Änderung.",
  "p.programme.refused_decider":
    "Programme pausieren, fortsetzen oder stoppen erfordert das Recht als Programmentscheider.",
  "p.programme.refused_not_programme":
    "Nur ein Plan der Art Programm hat einen Status, und eine genannte Prüfung oder Ausnahme muss zu diesem Programm gehören.",
  "p.programme.save_gates": "Gate-Termine speichern",
  "p.programme.status_heading": "Programmstatus",
  "p.programme.status_is": "Aktueller Stand: {status}",
  "p.programme.status_says":
    "Ein Programm ist aktiv, bis eine festgehaltene Entscheidung es pausiert oder stoppt. Eine Pause markiert seine Zuweisungen und löscht keine.",
  "p.programme.stop_warns":
    "Stoppen ist endgültig: Das Programm bleibt gestoppt und behält seine Aufzeichnungen.",
  "p.programme.stopped_final":
    "Dieses Programm ist gestoppt. Das ist endgültig; seine Aufzeichnungen bleiben.",
  "p.programme.title_ppm": "Gates und Programmstatus — PPM",
  "p.programme.title": "Gates und Programmstatus",
  "p.programme.buffer_by_days": "Eine Zahl von Arbeitstagen",
  "p.programme.buffer_by_rule":
    "Die Hälfte der kritischen Arbeit im Basisplan, aufgerundet",
  "p.programme.buffer_declared_one":
    "Festgelegt: ein Puffer von {days} Arbeitstag.",
  "p.programme.buffer_declared_other":
    "Festgelegt: ein Puffer von {days} Arbeitstagen.",
  "p.programme.buffer_heading": "Projektpuffer",
  "p.programme.buffer_says":
    "Die am Ende des Programms zurückgehaltene Zeit, um Verzug aufzufangen. Die Grafik setzt den verbrauchten Anteil ins Verhältnis zum erledigten Anteil der kritischen Arbeit. Eine Festlegung ersetzt nie eine frühere; die neueste ist der Puffer. Ein Pufferplan der kritischen Kette wird, wo vorhanden, stattdessen verwendet.",
  "p.programme.buffer_history": "Festlegungen, neueste zuerst",
  "p.programme.buffer_was_days_one": "{days} Arbeitstag, festgelegt",
  "p.programme.buffer_was_days_other": "{days} Arbeitstage, festgelegt",
  "p.programme.buffer_was_rule_one": "{days} Arbeitstag, nach der Regel",
  "p.programme.buffer_was_rule_other": "{days} Arbeitstage, nach der Regel",
  "p.programme.could_not_declare_buffer": "Der Puffer wurde nicht festgelegt.",
  "p.programme.could_not_load_buffer":
    "Der Projektpuffer konnte nicht gelesen werden.",
  "p.programme.declare_buffer": "Puffer festlegen",
  "p.programme.f_buffer_days": "Arbeitstage (1 bis {max})",
  "p.programme.f_buffer_how": "Wie lang der Puffer ist",
  "p.programme.f_buffer_note": "Notiz (optional)",
  "p.programme.problem_buffer_days":
    "Geben Sie den Puffer als ganze Zahl von Arbeitstagen von 1 bis 2000 an.",
  "p.programme.refused_buffer_rule":
    "Die Regel braucht einen Terminbasisplan mit kritischer Arbeit, die eine Dauer hat; legen Sie zuerst einen Basisplan fest oder geben Sie eine Zahl von Tagen an.",
  "p.programme.refused_not_programme_buffer":
    "Nur ein Plan der Art Programm hat einen Projektpuffer.",
  "p.programme.profile_heading": "Liefermodell und Berichterstattung",
  "p.programme.f_model": "Liefermodell",
  "p.programme.f_owner": "Verantwortliche Organisation",
  "p.programme.no_owner": "Keine verantwortliche Organisation",
  "p.programme.model_says":
    "Das Liefermodell sagt, wer baut und wer ausrollt; das Direktorenpaket zeigt es neben dem Programm. Modell und Organisation werden zusammen gespeichert.",
  "p.programme.save_model": "Liefermodell speichern",
  "p.programme.model_saved": "Das Liefermodell ist gespeichert.",
  "p.programme.f_method": "Wie das Programm über den Fortschritt berichtet",
  "p.programme.method_evm": "Earned Value (Kosten- und Terminleistung)",
  "p.programme.method_product": "Burn-up und Fluss (ein Produktprogramm)",
  "p.programme.save_method": "Berichtsmethode speichern",
  "p.programme.method_saved_evm":
    "Das Programm berichtet jetzt nach Earned Value.",
  "p.programme.method_saved_product":
    "Das Programm berichtet jetzt nach Burn-up und Fluss.",
  "p.programme.profile_unchanged":
    "Nichts wurde geändert, daher wurde nichts gesendet.",
  "p.programme.could_not_load_profile":
    "Das Liefermodell konnte nicht geladen werden.",
  "p.programme.could_not_save_profile": "Die Änderung wurde nicht gespeichert.",
  "p.benefits.classified": "Einordnung gespeichert.",
  "p.benefits.classify_says":
    "Beim Speichern wird die ganze Einordnung gesendet: Eine als nicht angegeben belassene Wahl wird gelöscht.",
  "p.benefits.could_not_load":
    "Die Nutzen dieses Plans konnten nicht geladen werden.",
  "p.benefits.could_not_save": "Die Einordnung wurde nicht gespeichert.",
  "p.benefits.could_not_validate": "Die Validierung wurde nicht erfasst.",
  "p.benefits.f_annual": "Jährlicher Betrag ({currency})",
  "p.benefits.f_classification": "Einordnung",
  "p.benefits.f_kpi": "Messende Kennzahl",
  "p.benefits.f_note": "Notiz (optional)",
  "p.benefits.f_recurrence": "Wiederkehrend",
  "p.benefits.f_validated": "Bisher validierter Betrag ({currency})",
  "p.benefits.intro":
    "Ein realisierter Betrag ist eine Behauptung. Er gilt nur durch eine erfasste Validierung von jemandem außerhalb des Programms als validiert; jede nennt den bisher validierten Betrag, und die neueste zählt.",
  "p.benefits.measured_by_kpi":
    "Ein nichtfinanzieller Nutzen wird über seine Kennzahl gemessen, nicht als Betrag validiert.",
  "p.benefits.money_line": "Ziel {target}; realisiert (behauptet) {realized}.",
  "p.benefits.no_kpi": "Keine Kennzahl",
  "p.benefits.non_financial": "Ein nichtfinanzieller Nutzen.",
  "p.benefits.none": "Dieser Plan hat noch keine Nutzen.",
  "p.benefits.problem_annual_amount":
    "Geben Sie den jährlichen Betrag als Zahl an, nicht unter 0.",
  "p.benefits.problem_annual_financial":
    "Nur ein finanzieller Nutzen, einer mit Währung, hat einen jährlichen Betrag.",
  "p.benefits.problem_annual_recurring":
    "Nur ein wiederkehrender Nutzen hat einen jährlichen Betrag: Wählen Sie zuerst wiederkehrend.",
  "p.benefits.problem_validated_amount":
    "Geben Sie den bisher validierten Betrag als Zahl an, nicht unter 0.",
  "p.benefits.refused_member":
    "Wer validiert, darf nicht zum Programm gehören: Bitten Sie jemanden außerhalb, zu validieren.",
  "p.benefits.save": "Einordnung speichern",
  "p.benefits.standing_claimed": "Behauptet, noch nicht validiert",
  "p.benefits.standing_not_claimed": "Noch nichts behauptet",
  "p.benefits.standing_partly_validated": "Teilweise validiert",
  "p.benefits.standing_validated": "Validiert",
  "p.benefits.title_ppm": "Nutzen — PPM",
  "p.benefits.title": "Nutzen: Einordnung und Validierung",
  "p.benefits.validate": "Validierung erfassen",
  "p.benefits.validated_done": "Validierung erfasst.",
  "p.benefits.validated_to_date": "{amount} bisher validiert",
  "p.benefits.validation_heading": "Validierung",
  "p.benefits.validation_says":
    "Validierungen bleiben wie erfasst; eine neue nennt den bisher validierten Betrag und ersetzt keine.",
  "p.funding.add": "Quelle hinzufügen",
  "p.funding.added": "Finanzierungsquelle hinzugefügt.",
  "p.funding.choose_plan": "Plan wählen",
  "p.funding.could_not_load_lines":
    "Die Budgetposten dieses Plans konnten nicht geladen werden.",
  "p.funding.could_not_load_money":
    "Das Geld nach Finanzierungsquelle konnte nicht geladen werden.",
  "p.funding.could_not_load":
    "Die Finanzierungsquellen konnten nicht geladen werden.",
  "p.funding.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.funding.f_combine": "Investiv und konsumtiv zusammenfassen",
  "p.funding.f_fy":
    "Geschäftsjahr (das Jahr, in dem es beginnt; leer für das aktuelle)",
  "p.funding.f_name": "Neue Finanzierungsquelle",
  "p.funding.f_plan": "Gewählter Plan",
  "p.funding.f_source": "Finanzierungsquelle",
  "p.funding.intro":
    "Woher das Geld jedes Programms kommt. Ein Budgetposten ohne Finanzierungsquelle wird gezählt und aufgeführt, nie versteckt.",
  "p.funding.is_active": "aktiv",
  "p.funding.is_retired": "stillgelegt",
  "p.funding.line_cleared": "Der Posten hat keine Finanzierungsquelle mehr.",
  "p.funding.line_set": "Die Finanzierungsquelle des Postens ist gesetzt.",
  "p.funding.lines_heading": "Budgetposten und ihre Finanzierungsquelle",
  "p.funding.lines_says":
    "Wählen Sie einen Plan, um seine Budgetposten zu sehen und jedem eine Finanzierungsquelle zu geben.",
  "p.funding.money_heading": "Geld nach Finanzierungsquelle",
  "p.funding.no_lines": "Dieser Plan hat keine Budgetposten.",
  "p.funding.no_source": "Keine Finanzierungsquelle",
  "p.funding.no_sources": "Noch wurde keine Finanzierungsquelle hinzugefügt.",
  "p.funding.problem_fy":
    "Geben Sie das Geschäftsjahr als das Jahr an, in dem es beginnt, von 2000 bis 2100.",
  "p.funding.problem_name_taken":
    "Eine Finanzierungsquelle mit diesem Namen gibt es schon.",
  "p.funding.problem_name":
    "Der Name einer Finanzierungsquelle hat 2 bis 120 Zeichen.",
  "p.funding.refused_inactive":
    "Diese Finanzierungsquelle ist stillgelegt und kann keinem Posten gegeben werden.",
  "p.funding.restore": "Wiederherstellen",
  "p.funding.restored": "Finanzierungsquelle wiederhergestellt.",
  "p.funding.retire_says":
    "Eine stillgelegte Quelle lässt sich keinem Posten mehr geben, aber Posten, die sie haben, behalten sie.",
  "p.funding.retire": "Stilllegen",
  "p.funding.retired_option": "{name} (stillgelegt)",
  "p.funding.retired": "Finanzierungsquelle stillgelegt.",
  "p.funding.show": "Anzeigen",
  "p.funding.sources_heading": "Finanzierungsquellen",
  "p.funding.title_ppm": "Finanzierung — PPM",
  "p.funding.title": "Finanzierung",
  "p.director_pack.actions_heading": "Prüfen, herausgeben oder neu herausgeben",
  "p.director_pack.aud_board": "Vorstand",
  "p.director_pack.aud_escalation_board": "Eskalationsgremium",
  "p.director_pack.aud_executive": "Geschäftsleitung",
  "p.director_pack.aud_org": "Organisation: {name}",
  "p.director_pack.aud_partner": "Partner",
  "p.director_pack.choose_audience": "Zielgruppe wählen",
  "p.director_pack.conflict_draft":
    "Für diesen Zeitraum gibt es schon ein Paket für diese Zielgruppe. Öffnen Sie es: Geben Sie den Entwurf heraus oder geben Sie es mit einer Änderungsnotiz neu heraus.",
  "p.director_pack.conflict_issue":
    "Dieses Paket kann jetzt nicht herausgegeben werden: Es ist schon herausgegeben, oder es ist ein geplanter Entwurf, den das Portfoliobüro noch nicht geprüft hat.",
  "p.director_pack.conflict_reissue":
    "Nur die neueste herausgegebene Version wird neu herausgegeben, und es gibt schon eine spätere Version.",
  "p.director_pack.conflict_review":
    "Dieser Entwurf wurde schon geprüft, oder er ist kein Entwurf mehr.",
  "p.director_pack.could_not_act": "Die Änderung wurde nicht vorgenommen.",
  "p.director_pack.could_not_draft": "Das Paket wurde nicht entworfen.",
  "p.director_pack.cut_off_says":
    "Leer lassen für jetzt. Das Paket liest die Pläne so, wie sie zum Stichzeitpunkt waren; er darf nicht in der Zukunft und nicht mehr als 400 Tage zurück liegen.",
  "p.director_pack.draft_heading": "Paket entwerfen",
  "p.director_pack.draft_says":
    "Der Entwurf liest die Pläne, die Sie teilen dürfen, zum Stichzeitpunkt und speichert das Ergebnis als Entwurf. Die Zielgruppe erhält nichts, bis jemand es herausgibt.",
  "p.director_pack.draft": "Paket entwerfen",
  "p.director_pack.f_audience": "Zielgruppe",
  "p.director_pack.f_change_note": "Änderungsnotiz",
  "p.director_pack.f_cut_off": "Stichzeitpunkt (optional)",
  "p.director_pack.f_period": "Zeitraum (Monat)",
  "p.director_pack.from_schedule": "vom Monatsplan erstellt",
  "p.director_pack.is_reviewed": "Vom Portfoliobüro geprüft.",
  "p.director_pack.is_stale":
    "veraltet: sein Stichzeitpunkt liegt vor dem Ende seines Zeitraums",
  "p.director_pack.issue_says":
    "Die Herausgabe friert das Paket ein und benachrichtigt seine Zielgruppe. Sie lässt sich nicht zurücknehmen; eine spätere Änderung ist eine Neuherausgabe.",
  "p.director_pack.issue_waits":
    "Der Monatsplan hat diesen Entwurf erstellt; das Portfoliobüro prüft ihn, bevor er herausgegeben werden kann.",
  "p.director_pack.issue": "Paket herausgeben",
  "p.director_pack.issued_done": "Herausgegeben.",
  "p.director_pack.not_reviewed": "Noch nicht geprüft.",
  "p.director_pack.problem_audience": "Wählen Sie eine Zielgruppe.",
  "p.director_pack.problem_cut_off_bad":
    "Geben Sie den Stichzeitpunkt als Datum und Uhrzeit an.",
  "p.director_pack.problem_cut_off_future":
    "Der Stichzeitpunkt darf nicht in der Zukunft liegen.",
  "p.director_pack.problem_cut_off_old":
    "Der Stichzeitpunkt darf nicht mehr als 400 Tage zurückliegen.",
  "p.director_pack.problem_note":
    "Begründen Sie die neue Version mit mindestens {min} Zeichen.",
  "p.director_pack.problem_period": "Wählen Sie den Zeitraum als Monat.",
  "p.director_pack.refused_edit":
    "Pakete entwerfen, prüfen und neu herausgeben erfordert Bearbeitungsrechte für Direktorenpakete.",
  "p.director_pack.refused_issue":
    "Pakete herausgeben erfordert Herausgaberechte für Direktorenpakete.",
  "p.director_pack.reissue_says":
    "Die Neuherausgabe entwirft die nächste Version mit Ihrer Notiz; diese Version bleibt unverändert lesbar. Schreiben Sie mindestens {min} Zeichen.",
  "p.director_pack.reissue": "Mit Änderungsnotiz neu herausgeben",
  "p.director_pack.review": "Als geprüft markieren",
  "p.director_pack.reviewed_done": "Als geprüft markiert.",
  "p.director_pack.add_schedule_heading": "Zeitplan hinzufügen",
  "p.director_pack.add_schedule": "Zeitplan hinzufügen",
  "p.director_pack.conflict_schedule":
    "Diese Zielgruppe hat schon einen Zeitplan; ändern Sie stattdessen seinen Tag.",
  "p.director_pack.could_not_change_schedule":
    "Der Zeitplan wurde nicht geändert.",
  "p.director_pack.could_not_load_schedules":
    "Die Zeitpläne konnten nicht geladen werden.",
  "p.director_pack.could_not_load_subscriptions":
    "Über welche Pakete Sie benachrichtigt werden, konnte nicht geladen werden.",
  "p.director_pack.could_not_subscribe":
    "Die Änderung wurde nicht gespeichert.",
  "p.director_pack.day_says":
    "Ein Tag von 1 bis 28, damit jeder Monat ihn hat. Der erste Lauf ist der nächste solche Tag.",
  "p.director_pack.f_day": "Tag des Monats",
  "p.director_pack.last_drafted":
    "Letzter Lauf {when}: Paket für {period} entworfen.",
  "p.director_pack.last_run": "Letzter Lauf {when}.",
  "p.director_pack.last_skipped":
    "Letzter Lauf {when}: {period} übersprungen, dafür gab es schon ein Paket.",
  "p.director_pack.move_day": "Auf diesen Tag legen",
  "p.director_pack.no_schedules":
    "Noch wird kein Paket nach Zeitplan entworfen.",
  "p.director_pack.problem_day":
    "Wählen Sie einen Tag des Monats von 1 bis 28.",
  "p.director_pack.refused_subscribe":
    "Sie können nur über Zielgruppen benachrichtigt werden, deren Pakete Sie lesen dürfen.",
  "p.director_pack.schedule_added": "Zeitplan hinzugefügt.",
  "p.director_pack.schedule_off":
    "Ausgeschaltet. Er würde am {day}. jedes Monats entwerfen.",
  "p.director_pack.schedule_on":
    "Entwirft am {day}. jedes Monats; nächster Lauf am {next}.",
  "p.director_pack.schedule_saved": "Gespeichert.",
  "p.director_pack.schedule_switched_off": "Ausgeschaltet.",
  "p.director_pack.schedule_switched_on": "Eingeschaltet.",
  "p.director_pack.schedule_unchanged":
    "Das ist schon sein Tag, daher wurde nichts gesendet.",
  "p.director_pack.schedules_intro":
    "Jeder Zeitplan entwirft das Paket einer Zielgruppe an einem Tag jedes Monats. Er gibt nie eines heraus: Das Portfoliobüro prüft den Entwurf, und eine Person gibt ihn heraus.",
  "p.director_pack.schedules_link": "Monatliche Zeitpläne",
  "p.director_pack.schedules_title_ppm": "Monatliche Zeitpläne — PPM",
  "p.director_pack.schedules_title": "Monatliche Zeitpläne",
  "p.director_pack.subs_heading":
    "Benachrichtigen, wenn ein Paket herausgegeben wird",
  "p.director_pack.subs_legend": "Pakete, über die ich benachrichtigt werde",
  "p.director_pack.subs_says":
    "Sie werden im Dienst mit einem Link benachrichtigt, wenn ein Paket für eine gewählte Zielgruppe herausgegeben wird. Die Nachricht enthält nie das Paket selbst.",
  "p.director_pack.subscribed": "Sie werden benachrichtigt.",
  "p.director_pack.sweep_heading": "Fälliges jetzt ausführen",
  "p.director_pack.sweep_result":
    "Entworfen: {drafted}; übersprungen, weil es schon ein Paket gab: {skipped}.",
  "p.director_pack.sweep_says":
    "Entwirft jedes Paket, dessen Tag gekommen ist und das noch keines hat. Nichts wird herausgegeben.",
  "p.director_pack.sweep": "Fällige Zeitpläne ausführen",
  "p.director_pack.switch_off": "Ausschalten",
  "p.director_pack.switch_on": "Einschalten",
  "p.director_pack.unsubscribed": "Sie werden nicht mehr benachrichtigt.",
  "p.plans.pid.modules_failed":
    "Welche Werkzeuge dieser Plan verwendet, konnte nicht geprüft werden, daher werden alle Werkzeuge aufgeführt.",
  "p._layout_svelte.handling_mark": "Kennzeichnung: {mark}",
  "p._layout_svelte.handling_mark_unknown":
    "Kennzeichnung: konnte nicht geprüft werden. Behandeln Sie diesen Plan als gekennzeichnet, bis dies geschehen ist.",
  "l.enum.cr_status.draft": "Entwurf",
  "l.enum.cr_status.submitted": "eingereicht",
  "l.enum.cr_status.under_review": "in Prüfung",
  "l.enum.cr_status.approved": "genehmigt",
  "l.enum.cr_status.rejected": "abgelehnt",
  "l.enum.cr_status.implemented": "umgesetzt",
  "l.enum.cr_status.withdrawn": "zurückgezogen",
  "l.enum.cr_kind.scope": "Umfang",
  "l.enum.cr_kind.schedule": "Termin",
  "l.enum.cr_kind.cost": "Kosten",
  "l.enum.cr_kind.quality": "Qualität",
  "l.enum.cr_kind.other": "Sonstiges",
  "p.plan_change_requests.title_ppm": "Änderungsanträge des Plans — PPM",
  "p.plan_change_requests.title": "Änderungsanträge des Plans",
  "p.plan_change_requests.intro":
    "Die Änderungsanträge dieses Plans, die neuesten zuerst. Entwerfen Sie einen und bearbeiten Sie ihn, solange er ein Entwurf ist; reichen Sie ihn bei einer benannten genehmigenden Person ein, die nicht Sie sind; diese prüft ihn und hält die Entscheidung mit einer Begründung fest. Ein offener Antrag kann zurückgezogen werden.",
  "p.plan_change_requests.register_link": "Änderungsanträge aller Pläne",
  "p.plan_change_requests.could_not_load":
    "Die Änderungsanträge des Plans konnten nicht geladen werden",
  "p.plan_change_requests.could_not_save":
    "Der Änderungsantrag konnte nicht gespeichert werden",
  "p.plan_change_requests.steps": "Schritte",
  "p.plan_change_requests.draft_heading": "Änderungsantrag entwerfen",
  "p.plan_change_requests.edit_heading": "Entwurf bearbeiten",
  "p.plan_change_requests.f_description": "Beschreibung (was sich ändern soll)",
  "p.plan_change_requests.f_reason":
    "Begründung (warum die Änderung nötig ist)",
  "p.plan_change_requests.f_days":
    "Auswirkung auf den Zeitplan in Arbeitstagen (negativ zieht das Ende vor; leer lassen, wenn nicht bewertet)",
  "p.plan_change_requests.f_cost":
    "Auswirkung auf die Kosten (leer lassen, wenn nicht bewertet)",
  "p.plan_change_requests.f_currency":
    "Währung der Kosten (drei Buchstaben, zum Beispiel EUR)",
  "p.plan_change_requests.f_summary": "Zusammenfassung der Auswirkung",
  "p.plan_change_requests.save_draft": "Entwurf speichern",
  "p.plan_change_requests.drafted": "Der Änderungsantrag wurde entworfen.",
  "p.plan_change_requests.saved": "Der Entwurf wurde gespeichert.",
  "p.plan_change_requests.problem_title_required":
    "Geben Sie dem Änderungsantrag einen Titel.",
  "p.plan_change_requests.problem_description_required":
    "Geben Sie an, was sich ändern soll.",
  "p.plan_change_requests.problem_reason_required":
    "Geben Sie an, warum die Änderung nötig ist.",
  "p.plan_change_requests.problem_days_not_whole":
    "Die Auswirkung auf den Zeitplan ist eine ganze Zahl von Arbeitstagen.",
  "p.plan_change_requests.problem_days_out_of_range":
    "Die Auswirkung auf den Zeitplan muss zwischen -3650 und 3650 Arbeitstagen liegen.",
  "p.plan_change_requests.problem_cost_not_amount":
    "Die Auswirkung auf die Kosten ist kein Betrag.",
  "p.plan_change_requests.problem_cost_needs_currency":
    "Eine Auswirkung auf die Kosten braucht ihre Währung.",
  "p.plan_change_requests.problem_currency_needs_cost":
    "Eine Währung braucht eine Auswirkung auf die Kosten.",
  "p.plan_change_requests.problem_currency_invalid":
    "Die Währung ist ein Code aus drei Buchstaben, zum Beispiel EUR, JPY oder GBP.",
  "p.plan_change_requests.steps_heading": "Schritte für {title}",
  "p.plan_change_requests.status_now": "Aktueller Stand: {status}.",
  "p.plan_change_requests.decided": "Entschieden von {who} am {on}: {reason}",
  "p.plan_change_requests.no_steps":
    "Für diesen Änderungsantrag ist kein weiterer Schritt möglich.",
  "p.plan_change_requests.who_note":
    "Der Dienst entscheidet, wer welchen Schritt gehen darf: Nur die Person, die ihn entworfen hat, bearbeitet, reicht ein oder zieht zurück (eine Administratorin oder ein Administrator darf ihn ebenfalls zurückziehen), und nur die benannte genehmigende Person prüft oder entscheidet.",
  "p.plan_change_requests.submit_heading": "Zur Entscheidung einreichen",
  "p.plan_change_requests.f_approver":
    "Genehmigende Person (eine person:- oder worker:-Referenz, nicht die Person, die ihn entworfen hat)",
  "p.plan_change_requests.submit": "Einreichen",
  "p.plan_change_requests.submitted": "Der Änderungsantrag wurde eingereicht.",
  "p.plan_change_requests.problem_approver_not_reference":
    "Nennen Sie die genehmigende Person als person:- oder worker:-Referenz.",
  "p.plan_change_requests.problem_approver_is_requester":
    "Die genehmigende Person kann nicht die Person sein, die die Änderung beantragt hat.",
  "p.plan_change_requests.review": "Prüfung beginnen",
  "p.plan_change_requests.review_started": "Die Prüfung hat begonnen.",
  "p.plan_change_requests.decide_heading": "Entscheidung festhalten",
  "p.plan_change_requests.f_decision": "Entscheidung",
  "p.plan_change_requests.approve": "Genehmigen",
  "p.plan_change_requests.reject": "Ablehnen",
  "p.plan_change_requests.f_decision_reason":
    "Begründung der Entscheidung (mindestens 10 Zeichen)",
  "p.plan_change_requests.decide": "Entscheidung speichern",
  "p.plan_change_requests.decision_recorded":
    "Die Entscheidung wurde festgehalten.",
  "p.plan_change_requests.problem_decision_reason":
    "Eine Entscheidung braucht eine Begründung von mindestens 10 Zeichen.",
  "p.plan_change_requests.withdraw": "Änderungsantrag zurückziehen",
  "p.plan_change_requests.withdrawn":
    "Der Änderungsantrag wurde zurückgezogen.",
  "p.plan_change_requests.close_steps": "Schritte schließen",
  "p.plan_change_requests.refused_requester":
    "Nur die Person, die diesen Änderungsantrag entworfen hat, kann das tun.",
  "p.plan_change_requests.refused_withdraw":
    "Nur die Person, die diesen Änderungsantrag entworfen hat, oder eine Administratorin bzw. ein Administrator kann ihn zurückziehen.",
  "p.plan_change_requests.refused_approver":
    "Nur die benannte genehmigende Person kann diesen Änderungsantrag prüfen oder entscheiden.",
  "p.plan_change_requests.refused_state":
    "Dieser Änderungsantrag ist weitergegangen, und dieser Schritt ist für ihn nicht mehr möglich; die Liste wurde neu gelesen.",
  "p.plan_change_requests.refused_invalid":
    "Der Dienst hat die Eingabe nicht angenommen: Prüfen Sie die Felder und versuchen Sie es erneut.",
  "p.change_requests.change_requests_ppm": "Änderungsanträge — PPM",
  "p.change_requests.change_requests": "Änderungsanträge",
  "p.change_requests.intro":
    "Ein Änderungsantrag ist ein kontrollierter Vorschlag, einen genehmigten Basisplan zu ändern. Eine benannte genehmigende Person, die nicht die antragstellende ist, entscheidet und begründet. Auswirkungen sind die Angaben der antragstellenden Person; eine Prognose wird nur angezeigt, wenn Was-wäre-wenn-Änderungen angehängt sind.",
  "p.change_requests.could_not_load":
    "Änderungsanträge konnten nicht geladen werden",
  "p.change_requests.by_status": "Änderungsanträge nach Status",
  "p.change_requests.awaiting": "Wartet auf Entscheidung",
  "p.change_requests.oldest_open":
    "Ältester offener: {title} ({days} Kalendertage)",
  "p.change_requests.show": "Anzeigen",
  "p.change_requests.all_statuses": "Alle Status",
  "p.change_requests.open_only": "Nur offene",
  "p.change_requests.none": "Keine Änderungsanträge erfasst.",
  "p.change_requests.none_match": "Keine passenden Änderungsanträge.",
  "p.change_requests.loading": "Wird geladen…",
  "p.change_requests.count_one": "{n} Änderungsantrag, neueste zuerst",
  "p.change_requests.count_other": "{n} Änderungsanträge, neueste zuerst",
  "p.change_requests.title_col": "Änderungsantrag",
  "p.change_requests.kind_col": "Art",
  "p.change_requests.status_col": "Status",
  "p.change_requests.impact_col": "Angegebene Auswirkung",
  "p.change_requests.age_col": "Alter (Kalendertage)",
  "p.change_requests.approver_col": "Genehmigende Person",
  "p.change_requests.no_approver": "Noch nicht benannt",
  "p.change_requests.no_impact": "Keine Auswirkung angegeben",
  "p.change_requests.impact_days": "{days} Arbeitstage",
  "p.change_requests.plan_link": "Änderungsanträge des Plans öffnen",
  "p.director_pack.director_packs_ppm": "Vorstandsberichte — PPM",
  "p.director_pack.director_packs": "Vorstandsberichte",
  "p.director_pack.intro":
    "Ein Vorstandsbericht ist der Monatsbericht an die Geschäftsleitung, einmal entworfen, von einer benannten Person herausgegeben und dann eingefroren: Ein herausgegebener Bericht ändert sich nie, eine Korrektur ist eine neue Version mit Vermerk. Jeder Wert unten stammt aus dem herausgegebenen Bericht; hier wird nichts neu berechnet.",
  "p.director_pack.could_not_load":
    "Vorstandsberichte konnten nicht geladen werden",
  "p.director_pack.could_not_load_one":
    "Dieser Vorstandsbericht konnte nicht geladen werden",
  "p.director_pack.loading": "Wird geladen…",
  "p.director_pack.none": "Für Sie ist kein Vorstandsbericht verfügbar.",
  "p.director_pack.col_period": "Zeitraum",
  "p.director_pack.col_audience": "Zielgruppe",
  "p.director_pack.col_version": "Version",
  "p.director_pack.col_status": "Status",
  "p.director_pack.col_issued": "Herausgegeben",
  "p.director_pack.status_draft": "Entwurf",
  "p.director_pack.status_issued": "herausgegeben",
  "p.director_pack.open_pack":
    "Bericht für {period}, {audience}, Version {version} öffnen",
  "p.director_pack.all_packs": "Alle Vorstandsberichte",
  "p.director_pack.pack_heading": "Vorstandsbericht {period} — {audience}",
  "p.director_pack.version_line":
    "Version {version} · {status} · Stichtag {cutoff}",
  "p.director_pack.issued_on": "Herausgegeben {when}",
  "p.director_pack.draft_note":
    "Dies ist ein Entwurf. Er ist noch nicht herausgegeben und daher nicht eingefroren.",
  "p.director_pack.change_note": "Änderungsvermerk: {note}",
  "p.director_pack.previous_pack":
    "Verglichen mit dem Bericht für {period}, Version {version}.",
  "p.director_pack.download_pdf": "Als PDF herunterladen",
  "p.director_pack.pdf_note":
    "Das PDF enthält dieselben Werte, aber seine festen Texte sind auf Englisch. Um eine andere Sprache zu behalten, drucken Sie diese Seite.",
  "p.director_pack.basis":
    "Die Datensätze entsprechen dem Stand beim Entwurf des Berichts, datiert am oder vor dem Stichtag. Probleme und Änderungsanträge haben keinen Statusverlauf, daher erscheint einer, der nach dem Stichtag, aber vor dem Entwurf geschlossen wurde, als geschlossen. Ein Abschnitt ohne Inhalt sagt dies; Fehlendes wird nie als null oder grün dargestellt.",
  "p.director_pack.s_quality": "Datenqualität",
  "p.director_pack.s_changed": "Was sich geändert hat",
  "p.director_pack.s_map": "Portfolio-Übersicht",
  "p.director_pack.s_exceptions": "Ausnahmen",
  "p.director_pack.s_gates": "Gate-Entscheidungen",
  "p.director_pack.s_funding": "Finanzierung",
  "p.director_pack.s_fever": "Puffer-Fieberkurve",
  "p.director_pack.s_evm": "Earned Value",
  "p.director_pack.s_benefits": "Nutzen",
  "p.director_pack.s_capacity": "Kapazität",
  "p.director_pack.s_issues": "Probleme",
  "p.director_pack.s_changes": "Änderungsanträge",
  "p.director_pack.plans_included": "Enthaltene Pläne: {n}",
  "p.director_pack.plans_left_out":
    "Ausgelassene Pläne, da privat oder gekennzeichnet: {n}",
  "p.director_pack.sections_missing": "Noch nicht erstellte Abschnitte: {list}",
  "p.director_pack.all_sections":
    "Alle Abschnitte des Berichts sind vorhanden.",
  "p.director_pack.nothing_here":
    "In diesem Abschnitt gibt es nichts anzuzeigen.",
  "p.director_pack.changed_issues": "Offene Probleme: {now}, {change}",
  "p.director_pack.changed_changes":
    "Änderungsanträge mit ausstehender Entscheidung: {now}, {change}",
  "p.director_pack.changed_was": "vorher {before}",
  "p.director_pack.changed_unchanged": "unverändert",
  "p.director_pack.changed_first": "kein früherer Bericht zum Vergleich",
  "p.director_pack.c_programme": "Programm",
  "p.director_pack.c_aim": "Strategisches Ziel",
  "p.director_pack.c_status": "Status",
  "p.director_pack.c_stage": "Stufe",
  "p.director_pack.c_phase": "Phase",
  "p.director_pack.c_confidence": "Lieferzuversicht",
  "p.director_pack.c_trend": "Trend",
  "p.director_pack.self_assessed": "selbst eingeschätzt",
  "p.director_pack.stale": "veraltet",
  "p.director_pack.no_aim": "Kein strategisches Ziel",
  "p.director_pack.c_plan": "Plan",
  "p.director_pack.c_worst": "Schlechteste Bewertung",
  "p.director_pack.c_need": "Erforderliche Entscheidung",
  "p.director_pack.c_owner_role": "Verantwortlich (Rolle)",
  "p.director_pack.c_decide_by": "Entscheiden bis",
  "p.director_pack.c_hours_open": "Stunden offen",
  "p.director_pack.ex_window":
    "Eine Ausnahme, die nicht innerhalb von {hours} Stunden nach Eröffnung abgeschlossen ist, gilt als verletzt.",
  "p.director_pack.ex_open": "Offen: {n}",
  "p.director_pack.ex_breached": "Verletzt: {n}",
  "p.director_pack.ex_closed": "Im Zeitraum geschlossen: {n}",
  "p.director_pack.g_passed": "Bestanden: {n}",
  "p.director_pack.g_failed": "Nicht bestanden: {n}",
  "p.director_pack.g_deferred": "Zurückgestellt: {n}",
  "p.director_pack.g_paused": "Pausierte Programme: {n}",
  "p.director_pack.g_stopped": "Gestoppte Programme: {n}",
  "p.director_pack.c_gate": "Gate",
  "p.director_pack.c_decided": "Entschieden",
  "p.director_pack.c_planned": "Geplant",
  "p.director_pack.c_reason": "Grund",
  "p.director_pack.c_days_late": "Tage verspätet",
  "p.director_pack.c_since": "Seit",
  "p.director_pack.c_allocs": "Markierte Zuweisungen",
  "p.director_pack.reason_hold": "als „angehalten“ geprüft",
  "p.director_pack.reason_no_review":
    "geplantes Datum ohne Prüfung verstrichen",
  "p.director_pack.fy_line":
    "Geschäftsjahr {label}, {start} bis {end}. Investitions- und laufende Ausgaben werden getrennt dargestellt, und Währungen werden nie addiert.",
  "p.director_pack.c_source": "Finanzierungsquelle",
  "p.director_pack.c_class": "Klasse",
  "p.director_pack.c_budget": "Budget",
  "p.director_pack.c_spend": "Bisherige Ausgaben",
  "p.director_pack.c_variance": "Abweichung",
  "p.director_pack.no_funding_source": "keine Finanzierungsquelle",
  "p.director_pack.forecast_line":
    "Lebenszyklusprognose in {currency}: Ist {actual}, Schätzung bei Fertigstellung {eac}",
  "p.director_pack.unfunded_lines":
    "Positionen ohne Finanzierungsquelle: {n}. Positionen ohne Datum: {dated}.",
  "p.director_pack.c_chain": "Kette abgeschlossen (%)",
  "p.director_pack.c_buffer": "Puffer verbraucht (%)",
  "p.director_pack.c_zone": "Zone",
  "p.director_pack.c_slip": "Verzug (Tage)",
  "p.director_pack.c_buffer_days": "Puffer (Tage)",
  "p.director_pack.c_previous": "Vorheriger Bericht",
  "p.director_pack.edge_below": "Erholung: am unteren Rand gezeichnet",
  "p.director_pack.edge_above": "Überschreitung: am oberen Rand gezeichnet",
  "p.director_pack.fever_none_no_buffer":
    "kein Punkt in der Fieberkurve: kein Projektpuffer deklariert",
  "p.director_pack.fever_none_no_baseline":
    "kein Punkt in der Fieberkurve: keine Terminplan-Baseline",
  "p.director_pack.fever_none_no_chain":
    "kein Punkt in der Fieberkurve: keine kritische Arbeit mit messbarer Dauer",
  "p.director_pack.fever_none_no_forecast":
    "kein Punkt in der Fieberkurve: der Terminplan lässt sich derzeit nicht prognostizieren",
  "p.director_pack.fever_lines":
    "Zonenlinien (verbrauchter Puffer in % bei 0 % und bei 100 % abgeschlossener Kette): Grün zu Gelb {ga0} bis {ga1}; Gelb zu Rot {ar0} bis {ar1}.",
  "p.director_pack.c_bac": "Budget bei Fertigstellung",
  "p.director_pack.c_ev": "Fertigstellungswert",
  "p.director_pack.c_ac": "Istkosten",
  "p.director_pack.c_spi": "SPI",
  "p.director_pack.c_cpi": "CPI",
  "p.director_pack.below_threshold":
    "Unter dem Earned-Value-Schwellenwert: Budget bei Fertigstellung {bac}",
  "p.director_pack.no_baseline": "keine Baseline",
  "p.director_pack.evm_ratio_note":
    "SPI und CPI werden mit zwei Nachkommastellen angezeigt; leer bedeutet nicht definiert, nicht null.",
  "p.director_pack.c_recurrence": "Wiederkehr",
  "p.director_pack.c_classification": "Klassifizierung",
  "p.director_pack.c_claimed": "Beansprucht",
  "p.director_pack.c_validated": "Validiert",
  "p.director_pack.c_unvalidated": "Beansprucht, nicht validiert",
  "p.director_pack.benefits_note":
    "Realisiert, aber nicht validiert ist eine Behauptung, kein validierter Nutzen. Wiederkehrende Beträge sind jährlich und werden nie mit einmaligen Beträgen addiert.",
  "p.director_pack.c_pool": "Kompetenzpool",
  "p.director_pack.c_demand": "Bedarf (FTE)",
  "p.director_pack.c_supply": "Angebot (FTE)",
  "p.director_pack.c_gap": "Lücke (FTE)",
  "p.director_pack.c_month": "Monat",
  "p.director_pack.c_reading": "Bewertung",
  "p.director_pack.no_pool": "kein Pool zugewiesen",
  "p.director_pack.capacity_note":
    "Jede Zelle hat neben einer Farbe auch ein Wort und ein Symbol. Kein erfasstes Angebot bedeutet nicht null Kapazität.",
  "p.director_pack.partners_short": "Partnerlieferung unter der Zusage",
  "p.director_pack.open_issues": "Offene Probleme: {n}",
  "p.director_pack.awaiting":
    "Änderungsanträge, die auf eine Entscheidung warten: {n}",
  "p.director_pack.dash": "nicht erfasst",
  "l.enum.pk_conf.green": "grün",
  "l.enum.pk_conf.amber_green": "gelb/grün",
  "l.enum.pk_conf.amber": "gelb",
  "l.enum.pk_conf.amber_red": "gelb/rot",
  "l.enum.pk_conf.red": "rot",
  "l.enum.pk_conf.not_assessed": "nicht bewertet",
  "l.enum.pk_trend.up": "verbessert sich",
  "l.enum.pk_trend.down": "verschlechtert sich",
  "l.enum.pk_trend.same": "unverändert",
  "l.enum.pk_trend.new": "neu bewertet",
  "l.enum.pk_trend.not_assessed": "nicht bewertet",
  "l.enum.pk_trend.no_earlier_pack": "kein früherer Bericht",
  "l.enum.pk_status.active": "aktiv",
  "l.enum.pk_status.paused": "pausiert",
  "l.enum.pk_status.stopped": "gestoppt",
  "l.enum.pk_zone.green": "grün",
  "l.enum.pk_zone.amber": "gelb",
  "l.enum.pk_zone.red": "rot",
  "l.enum.pk_heat.shortfall": "Unterdeckung",
  "l.enum.pk_heat.tight": "knapp",
  "l.enum.pk_heat.enough": "ausreichend",
  "l.enum.pk_heat.no_supply_recorded": "kein Angebot erfasst",
  "l.enum.pk_heat.no_demand": "kein Bedarf",
  "l.enum.pk_heat.nothing_recorded": "nichts erfasst",
  "l.enum.pk_class.capex": "Investition",
  "l.enum.pk_class.opex": "laufende Ausgaben",
  "l.enum.pk_class.all": "Investition und laufende Ausgaben zusammen",
  "l.enum.pk_recur.recurring": "wiederkehrend",
  "l.enum.pk_recur.one_off": "einmalig",
  "l.enum.pk_recur.not_stated": "nicht angegeben",
  "l.enum.pk_cls.cash_releasing": "liquiditätswirksam",
  "l.enum.pk_cls.non_cash_releasing": "nicht liquiditätswirksam",
  "l.enum.pk_cls.societal": "gesellschaftlich",
  "l.enum.pk_cls.not_stated": "nicht angegeben",
  "l.enum.pk_gate.g0_concept": "G0 Konzept",
  "l.enum.pk_gate.g1_feasibility": "G1 Machbarkeit",
  "l.enum.pk_gate.g2_definition": "G2 Definition",
  "l.enum.pk_gate.g3_delivery": "G3 Lieferung",
  "l.enum.pk_gate.g4_launch": "G4 Einführung",
  "l.enum.pk_gate.g5_benefits": "G5 Nutzen",
  "p.director_pack.c_currency": "Währung",
  "p.critical_chain.chain_ppm": "Kritische Kette und Puffer — PPM",
  "p.critical_chain.buffers_ppm": "Puffermanagement — PPM",
  "p.critical_chain.pipeline_ppm": "Trommel und Pipeline — PPM",
  "p.critical_chain.loading": "Wird geladen…",
  "p.critical_chain.yes": "ja",
  "p.critical_chain.no": "nein",
  "p.critical_chain.zone_green": "grün",
  "p.critical_chain.zone_amber": "gelb",
  "p.critical_chain.zone_red": "rot",
  "p.critical_chain.zone_none": "keine Bewertung",
  "p.critical_chain.outcome_as_planned": "behält seine Termine",
  "p.critical_chain.outcome_staggered": "später beginnen",
  "p.critical_chain.outcome_exceeds_drum": "benötigt mehr, als die Trommel hat",
  "p.critical_chain.outcome_no_slot": "kein passender Start",
  "p.critical_chain.advice":
    "Dies ist eine Empfehlung. Es wurde nichts geändert, und eine Person entscheidet, was zu tun ist.",
  "p.critical_chain.link_plan": "Plan",
  "p.critical_chain.link_buffers": "Puffermanagement",
  "p.critical_chain.link_chain": "Kritische Kette",
  "p.critical_chain.link_critical_path": "Kritischer Pfad",
  "p.critical_chain.nav_pipeline": "Trommel und Pipeline",
  "p.critical_chain.chain_h1": "Kritische Kette und Puffer",
  "p.critical_chain.chain_lede":
    "Die kritische Kette ist die längste Abfolge von Aufgaben, wenn die Zeit der Menschen berücksichtigt wird: Sie folgt den Abhängigkeiten und den Wartezeiten auf eine Person, die mit einer anderen Aufgabe beschäftigt ist. Sie wird neben dem kritischen Pfad gezeigt, der nur den Abhängigkeiten folgt; beide werden nie vermischt.",
  "p.critical_chain.chain_h2": "Die kritische Kette",
  "p.critical_chain.no_chain": "Es gibt keine kritische Kette: {reason}",
  "p.critical_chain.reason_no_assignments":
    "niemand ist einer unerledigten Aufgabe zugewiesen, daher gibt es keine Wartezeiten auf Personen",
  "p.critical_chain.reason_no_scheduled_tasks":
    "keine Aufgabe hat einen Termin",
  "p.critical_chain.chain_work": "Arbeitstage Arbeit auf der Kette: {days}",
  "p.critical_chain.chain_tasks_count": "Aufgaben auf der Kette: {tasks}",
  "p.critical_chain.chain_finish":
    "Unter Berücksichtigung der Zeit der Menschen endet der Plan am {finish}.",
  "p.critical_chain.chain_delay_none":
    "Das ist nicht später als das Ende des kritischen Pfads.",
  "p.critical_chain.chain_delay":
    "Arbeitstage nach dem Ende des kritischen Pfads: {days}",
  "p.critical_chain.crosses_yes":
    "Die Kette wechselt zwischen Pfaden: Eine Person wird geteilt.",
  "p.critical_chain.crosses_no":
    "Die Kette bleibt auf einem Pfad: Keine Person wird geteilt.",
  "p.critical_chain.tied":
    "Eine andere Kette ist genauso lang; diese wird gezeigt, weil sie im Plan zuerst kommt.",
  "p.critical_chain.timeline_label": "Die Kette auf einer Zeitachse",
  "p.critical_chain.timeline_caption":
    "Jeder durchgezogene Balken ist eine Aufgabe auf der Kette. Ein gestrichelter Balken ist der Projektpuffer. Die Tabelle unten enthält dieselben Zahlen.",
  "p.critical_chain.col_task": "Aufgabe",
  "p.critical_chain.col_days": "Arbeitstage",
  "p.critical_chain.col_start": "Beginn",
  "p.critical_chain.col_finish": "Ende",
  "p.critical_chain.col_held_by": "Aufgehalten durch",
  "p.critical_chain.col_on_path": "Auf dem kritischen Pfad",
  "p.critical_chain.held_first": "erste auf der Kette",
  "p.critical_chain.held_dep": "eine Abhängigkeit",
  "p.critical_chain.held_person": "eine Person: {person}",
  "p.critical_chain.held_person_hidden": "eine Person (verborgen)",
  "p.critical_chain.cp_h2": "Der kritische Pfad daneben",
  "p.critical_chain.cp_finish": "Der kritische Pfad endet am {finish}.",
  "p.critical_chain.cp_none": "Es gibt keinen kritischen Pfad anzuzeigen.",
  "p.critical_chain.buffers_h2": "Puffer",
  "p.critical_chain.buffers_lede":
    "Aufgaben werden mit ihren ambitionierten Schätzungen eingeplant; die aus jeder Aufgabe entnommene Sicherheit wird in einem Puffer gebündelt. Größen in Arbeitstagen. Die Regel: {rule}.",
  "p.critical_chain.rule_cut_and_paste":
    "die Hälfte der entnommenen Sicherheit (Cut and Paste)",
  "p.critical_chain.rule_root_sum_squares":
    "die Quadratwurzel aus der Summe der Quadrate der entnommenen Sicherheit",
  "p.critical_chain.no_buffers": "Es gibt keine Puffer anzuzeigen: {reason}",
  "p.critical_chain.reason_estimates_missing":
    "einige Aufgaben auf der Kette haben noch keine ambitionierte und sichere Schätzung. Erfassen Sie sie über den Schätzungsdienst; dafür gibt es noch keine Maske.",
  "p.critical_chain.missing_h3":
    "Aufgaben auf der Kette ohne beide Schätzungen",
  "p.critical_chain.project_buffer":
    "Projektpuffer: {days} Arbeitstage zum Schutz einer Kette, die am {finish} endet. Mit Puffer ist das Datum {with}.",
  "p.critical_chain.project_safety":
    "Aus den Aufgaben der Kette entnommene Sicherheit: {days} Arbeitstage.",
  "p.critical_chain.project_buffer_bar": "Projektpuffer",
  "p.critical_chain.feeding_h3": "Zuführungspuffer",
  "p.critical_chain.feeding_none":
    "Keine andere Aufgabenkette mündet in die kritische Kette.",
  "p.critical_chain.col_feeding_tasks": "Zuführende Aufgaben",
  "p.critical_chain.col_joins": "Mündet in",
  "p.critical_chain.col_buffer": "Puffer (Arbeitstage)",
  "p.critical_chain.col_room": "Spielraum vor der Einmündung (Arbeitstage)",
  "p.critical_chain.col_push": "Würde die Kette verschieben",
  "p.critical_chain.push_yes": "ja, um {days} Arbeitstage",
  "p.critical_chain.unsized": "nicht bemessen: eine Schätzung fehlt",
  "p.critical_chain.resource_h3": "Ressourcenpuffer (Bereitschaftswarnungen)",
  "p.critical_chain.resource_lede":
    "Ein Ressourcenpuffer ist keine Zeit. Er erinnert daran, dass eine Person bereit sein muss, bevor ihre Aufgabe auf der Kette beginnt, {lead} Arbeitstage im Voraus.",
  "p.critical_chain.col_person": "Person",
  "p.critical_chain.col_alert_on": "Bereit bis",
  "p.critical_chain.col_task_starts": "Aufgabe beginnt",
  "p.critical_chain.hidden": "(verborgen)",
  "p.critical_chain.likely_h3":
    "Mit wahrscheinlicher Dauer eingeplant (keine ambitionierte Schätzung)",
  "p.critical_chain.could_not_load_chain":
    "Die kritische Kette konnte nicht geladen werden",
  "p.critical_chain.buffers_h1": "Puffermanagement",
  "p.critical_chain.buffers_lede2":
    "Jeder Puffer wird am aktuellen Terminplan gemessen: wie viel der Kette erledigt ist und wie viel des Puffers der prognostizierte Verzug verbraucht hat. Ein Puffer im roten Bereich erfordert eine Korrekturmaßnahme, die eine Person vereinbart und erfasst.",
  "p.critical_chain.no_plan":
    "Für diesen Plan wurde noch kein Pufferplan eingefroren. Frieren Sie einen ein, sobald Kette und Puffer vereinbart sind; spätere Messungen beziehen sich darauf.",
  "p.critical_chain.freeze_confirm":
    "Ich habe Kette und Puffer gelesen und möchte sie als Plan einfrieren",
  "p.critical_chain.freeze_button": "Pufferplan einfrieren",
  "p.critical_chain.frozen_on":
    "Gemessen am Pufferplan, eingefroren am {date}.",
  "p.critical_chain.alerts_h2": "Warnungen",
  "p.critical_chain.alerts_none": "Kein Puffer ist im roten Bereich.",
  "p.critical_chain.alert_project":
    "Der Projektpuffer ist im roten Bereich: {consumed} % verbraucht. Kette: {chain}.",
  "p.critical_chain.alert_feeding":
    "Ein Zuführungspuffer ist im roten Bereich: {consumed} % verbraucht. Kette: {chain}, mündet in {joins}.",
  "p.critical_chain.needs_action":
    "Keine offene Korrekturmaßnahme erfasst für: {buffers}.",
  "p.critical_chain.charts_h2": "Fieberkurven",
  "p.critical_chain.project_label": "Projektpuffer",
  "p.critical_chain.feeding_label": "Zuführungspuffer: {chain}",
  "p.critical_chain.axis_x": "Kette abgeschlossen (%)",
  "p.critical_chain.axis_y": "Puffer verbraucht (%)",
  "p.critical_chain.chart_caption":
    "{buffer}: {zone}; {consumed} % des Puffers verbraucht; {done} % der Kette erledigt.",
  "p.critical_chain.beyond_edge":
    "Der tatsächliche Wert liegt außerhalb des Diagramms, daher wird der Punkt am Rand gezeichnet.",
  "p.critical_chain.table_h2": "Dieselben Zahlen als Tabelle",
  "p.critical_chain.col_size": "Puffer (Arbeitstage)",
  "p.critical_chain.col_done": "Kette erledigt (%)",
  "p.critical_chain.col_used": "Puffer verbraucht (%)",
  "p.critical_chain.col_slip": "Prognostizierter Verzug (Arbeitstage)",
  "p.critical_chain.col_zone": "Zone",
  "p.critical_chain.no_point": "Keine Bewertung: {reason}",
  "p.critical_chain.actions_h2": "Korrekturmaßnahmen",
  "p.critical_chain.actions_none": "Es wurde keine Korrekturmaßnahme erfasst.",
  "p.critical_chain.col_buffer_name": "Puffer",
  "p.critical_chain.col_action": "Maßnahme",
  "p.critical_chain.col_owner": "Verantwortlich",
  "p.critical_chain.col_due": "Fällig",
  "p.critical_chain.col_status": "Status",
  "p.critical_chain.status_open": "offen",
  "p.critical_chain.status_done": "erledigt",
  "p.critical_chain.status_dropped": "verworfen",
  "p.critical_chain.mark_done": "Als erledigt markieren",
  "p.critical_chain.drop_action": "Verwerfen",
  "p.critical_chain.add_action_h3": "Korrekturmaßnahme erfassen",
  "p.critical_chain.f_buffer": "Puffer",
  "p.critical_chain.f_action": "Was getan wird",
  "p.critical_chain.f_owner": "Verantwortlich (Personenreferenz)",
  "p.critical_chain.f_due": "Fällig (optional)",
  "p.critical_chain.add_button": "Maßnahme erfassen",
  "p.critical_chain.snap_h2": "Verlauf",
  "p.critical_chain.snap_button": "Jetzt einen Snapshot erfassen",
  "p.critical_chain.snap_none": "Es wurde kein Snapshot erfasst.",
  "p.critical_chain.col_snap_date": "Erfasst am",
  "p.critical_chain.could_not_load_buffers":
    "Das Puffermanagement konnte nicht geladen werden",
  "p.critical_chain.could_not_save": "Das konnte nicht gespeichert werden",
  "p.critical_chain.pipeline_h1": "Trommel und Pipeline",
  "p.critical_chain.pipeline_lede":
    "Die Trommel ist der Kompetenzpool, von dem das Portfolio am stärksten abhängt. Projekte, die ihn gleichzeitig benötigen, werden gestaffelt: Der Dienst schlägt spätere Starts mit einem Kapazitätspuffer zwischen den Projekten vor und begründet jeden. Er verschiebt nie ein Datum.",
  "p.critical_chain.drum_h2": "Die Trommel",
  "p.critical_chain.drum_named":
    "Die Trommel ist {name}: vom Portfoliobüro benannt.",
  "p.critical_chain.drum_overloaded":
    "Die Trommel ist {name}: Es ist kein Pool benannt, und dieser hat die größte Unterdeckung.",
  "p.critical_chain.no_drum": "Es gibt keine Trommel: {why}",
  "p.critical_chain.set_h3": "Die Trommel benennen",
  "p.critical_chain.pick_pool": "Kompetenzpool",
  "p.critical_chain.set_button": "Als Trommel benennen",
  "p.critical_chain.clear_button": "Benennung aufheben",
  "p.critical_chain.capacity":
    "Kapazität der Trommel: {fte} FTE. Kapazitätspuffer zwischen Projekten: {days} Tage.",
  "p.critical_chain.no_capacity":
    "Für die Trommel ist im Zeitfenster kein Angebot erfasst, daher gibt es nichts, wonach gestaffelt werden kann.",
  "p.critical_chain.projects_h2": "Vorgeschlagene Starts",
  "p.critical_chain.projects_none":
    "Kein Projekt benötigt die Trommel in diesem Zeitfenster.",
  "p.critical_chain.pipeline_timeline":
    "Nutzung der Trommel jetzt und wie vorgeschlagen",
  "p.critical_chain.pipeline_timeline_caption":
    "Für jedes Projekt zeigt der obere Balken, wann es die Trommel jetzt nutzt, und der untere, wann es sie laut Vorschlag nutzen würde. Die Tabelle unten enthält dieselben Daten.",
  "p.critical_chain.bar_now": "jetzt",
  "p.critical_chain.bar_proposed": "vorgeschlagen",
  "p.critical_chain.col_project": "Projekt",
  "p.critical_chain.col_needs": "Benötigt die Trommel",
  "p.critical_chain.col_peak": "Spitze (FTE)",
  "p.critical_chain.col_proposal": "Vorschlag",
  "p.critical_chain.col_proposed": "Vorgeschlagene erste Nutzung",
  "p.critical_chain.col_delay": "Später um (Tage)",
  "p.critical_chain.col_why": "Begründung",
  "p.critical_chain.could_not_load_pipeline":
    "Die Pipeline konnte nicht geladen werden",
  "p.start_queue.nav": "Startwarteschlange",
  "p.start_queue.title": "Startwarteschlange — PPM",
  "p.start_queue.h1": "Startwarteschlange",
  "p.start_queue.lede":
    "Programme, die den Start der Lieferung beantragt haben und abgelehnt wurden, in der Reihenfolge ihrer Anträge. Ein Programm wird abgelehnt, wenn das Limit für laufende Arbeit erreicht ist oder wenn dem Engpass-Pool in den kommenden Monaten Angebot fehlt. Die Reihenfolge ändert sich nie, und ein Start kann dennoch mit einer erfassten Übersteuerung und einer Begründung erfolgen.",
  "p.start_queue.loading": "Warteschlange wird geladen…",
  "p.start_queue.none": "Kein Programm wartet auf den Start.",
  "p.start_queue.col_pos": "Platz",
  "p.start_queue.col_prog": "Programm",
  "p.start_queue.col_asked": "Beantragt am",
  "p.start_queue.col_why": "Ablehnungsgrund",
  "p.start_queue.col_act": "Aktion",
  "p.start_queue.limit":
    "Alle {limit} Plätze sind belegt: {in_delivery} Programme sind in der Lieferung.",
  "p.start_queue.pool": "Dem Engpass-Pool {pool} fehlt Angebot in: {months}.",
  "p.start_queue.withdraw": "{name} aus der Warteschlange zurückziehen",
  "p.start_queue.withdraw_short": "Zurückziehen",
  "p.start_queue.could_not_load":
    "Die Startwarteschlange konnte nicht geladen werden",
  "p.start_queue.could_not_withdraw":
    "Rückzug aus der Warteschlange nicht möglich",
  "p.table.search": "Zeilen filtern",
  "p.table.export": "Exportieren",
  "p.table.find": "Suchen",
  "p.table.moved": "{header} an Position {position} von {count} verschoben",
  "p.table.copy": "Kopieren",
  "p.table.copied": "{rows} Zeilen in die Zwischenablage kopiert",
  "p.table.copy_failed": "Kopieren in die Zwischenablage nicht möglich",
  "p.table.matches": "{matches} von {total} angezeigt",
  "p.table.search_in": "Suchen in",
  "p.table.all_columns": "Alle Spalten",
  "p.table.pages": "Seiten",
  "p.table.export_csv": "CSV",
  "p.table.export_tsv": "TSV",
  "p.table.export_excel": "Excel",
  "p.table.exported": "{rows} Zeilen nach {file} exportiert",
  "p.table.columns": "Spalten",
  "p.table.show_column": "{header} anzeigen",
  "p.table.resize": "Breite der Spalte {header} ändern",
  "p.table.select_all": "Alle Zeilen auswählen",
  "p.table.select_col": "Auswählen",
  "p.table.select_row": "{row} auswählen",
  "p.table.prev": "Vorherige Seite",
  "p.table.next": "Nächste Seite",
  "p.table.page_status": "Seite {page} von {pages} ({rows} Zeilen)",
  "p.table.page_announce": "Seite {page} von {pages}",
  "p.table.sorted": "{header} {direction} sortiert",
  "p.table.asc": "aufsteigend",
  "p.table.desc": "absteigend",
  "p.table.none": "nicht sortiert",
  "p.table.filtered": "{matches} von {total} Zeilen stimmen überein",
  "p.table.selected": "{n} Zeilen ausgewählt",
  "p.table.empty": "Nichts anzuzeigen.",
  "p.demo.tour_title": "Demo-Rundgang — PPM",
  "p.demo.tour_h1": "Demo-Rundgang",
  "p.demo.tour_lede":
    "Sie sind als Example Person angemeldet. Alles hier ist fiktiv: Die Northgate Transport Authority, ihre Mitarbeitenden, ihr Geld und ihre Termine wurden für diese Demonstration erfunden. Wählen Sie unten eine beliebige Funktion und nutzen Sie sie: Sie können Dinge ändern, und nichts ist echt.",
  "p.demo.tour_story_h2": "Die Geschichte",
  "p.demo.tour_story":
    "Die Northgate Transport Authority modernisiert ihr Schienennetz: neue Signaltechnik, barrierefreie Bahnhöfe und ein digitales Depot. Zehn Pläne, acht Personen, vier Organisationen und einige absichtlich schiefgehende Dinge, damit die Ansichten etwas zu zeigen haben.",
  "p.demo.tour_open": "Öffnen",
  "p.demo.tour_no_plan":
    "Der Plan North Line wurde nicht gefunden, daher sind die Links zu seinen Seiten ausgeblendet. Führen Sie das Seeding erneut aus.",
  "p.demo.tour_toc": "Funktionen",
  "p.demo.g_overview": "Das Portfolio auf einen Blick",
  "p.demo.g_delivery": "Lieferung eines Projekts (Stellwerk North Line)",
  "p.demo.g_risk": "Risiken, Probleme und Änderungen",
  "p.demo.g_money": "Geld",
  "p.demo.g_people": "Menschen und Kapazität",
  "p.demo.g_strategy": "Strategie und Aufnahme",
  "p.demo.g_escalation": "Governance und Eskalation",
  "p.demo.g_chain": "Kritische Kette",
  "p.demo.g_tools": "Werkzeuge",
  "p.demo.f_kanban_t": "Mein Kanban-Board",
  "p.demo.f_kanban_d":
    "Ihre eigenen offenen Aufgaben aus allen Projekten, in Spalten nach Status. Verschieben Sie eine mit der Maus oder über ihr Menü; der Status der Aufgabe ändert sich für alle.",
  "p.demo.f_me_t": "Meine Arbeit",
  "p.demo.f_me_d":
    "Dieselben Aufgaben nach Fälligkeitsdatum in Ihrer Zeitzone: überfällig, heute, diese Woche, später.",
  "p.demo.f_plans_t": "Pläne",
  "p.demo.f_plans_d":
    "Zehn fiktive Pläne: ein Portfolio, drei Programme und sechs Projekte für die Northgate Transport Authority. Öffnen Sie einen beliebigen.",
  "p.demo.f_dashboard_t": "Portfolio-Dashboard",
  "p.demo.f_dashboard_d":
    "Status nach Art und Phase, mit Anzahlen für Rot, Gelb und Grün.",
  "p.demo.f_gantt_t": "Gantt",
  "p.demo.f_gantt_d": "Der Terminplan jedes Projekts auf einer Zeitachse.",
  "p.demo.f_milestones_t": "Meilensteine",
  "p.demo.f_milestones_d":
    "Entwurf genehmigt, Ausrüstung geliefert und Inbetriebnahme für jedes Projekt.",
  "p.demo.f_calendar_t": "Kalender",
  "p.demo.f_calendar_d": "Meilensteine und Fälligkeitsdaten nach Monat.",
  "p.demo.f_board_t": "Aufgabenboard",
  "p.demo.f_board_d":
    "Acht Aufgaben von der Vermessung bis zur Übergabe, in Spalten nach Status. Eine ist durch den Sicherheitsnachweis blockiert.",
  "p.demo.f_critical_path_t": "Kritischer Pfad",
  "p.demo.f_critical_path_d":
    "Die Kette abhängiger Aufgaben, die das Enddatum bestimmt.",
  "p.demo.f_earned_value_t": "Fertigstellungswert",
  "p.demo.f_earned_value_d":
    "Planwert, Fertigstellungswert und Kosten, mit Termin- und Kostenindizes.",
  "p.demo.f_schedule_health_t": "Terminplanqualität",
  "p.demo.f_schedule_health_d":
    "Prüfungen der Planlogik: fehlende Verknüpfungen, lange Aufgaben, Einschränkungen.",
  "p.demo.f_wbs_t": "Projektstrukturplan",
  "p.demo.f_wbs_d": "Das Projekt, aufgeteilt in Arbeitspakete.",
  "p.demo.f_risk_t": "Risikoregister",
  "p.demo.f_risk_d":
    "Drei Risiken pro Projekt, bewertet nach Wahrscheinlichkeit und Auswirkung.",
  "p.demo.f_issues_t": "Probleme",
  "p.demo.f_issues_d":
    "Eine blockierte Kabeltrasse, ein doppelt gebuchter Prüfstand und eine verzögerte Genehmigung.",
  "p.demo.f_change_requests_t": "Änderungsanträge",
  "p.demo.f_change_requests_d":
    "Ein Antrag, die Inbetriebnahme um zwei Wochen zu verschieben, mit seinen Termin- und Kostenauswirkungen, wartet auf die Programmleitung.",
  "p.demo.f_joint_risk_t": "Kosten- und Terminrisiko gemeinsam",
  "p.demo.f_joint_risk_d":
    "Was die Risiken gemeinsam für Enddatum und Kosten bedeuten.",
  "p.demo.f_financials_t": "Finanzen",
  "p.demo.f_financials_d":
    "Budget, Istausgaben und Abweichung über das Portfolio.",
  "p.demo.f_cost_risk_t": "Kostenrisiko",
  "p.demo.f_cost_risk_d":
    "Die Streuung der wahrscheinlichen Kosten, mit P50 und P80.",
  "p.demo.f_ledger_t": "Finanzierungsbuch",
  "p.demo.f_ledger_d":
    "Ein Zuschuss und eine Reserve, zurückverfolgt zu den Budgetpositionen, die darauf zugreifen. Noch nicht auf einer Position eingeplantes Geld wird als nicht zugeordnet angezeigt.",
  "p.demo.f_costs_t": "Kosten eines Projekts",
  "p.demo.f_costs_d":
    "Personalkosten aus den Satztabellen und sonstige Kosten nach Aufgabe.",
  "p.demo.f_resources_t": "Ressourcen",
  "p.demo.f_resources_d":
    "Acht fiktive Personen, ihre Rollen und wofür sie eingeplant sind.",
  "p.demo.f_capacity_t": "Kapazitäts-Heatmap",
  "p.demo.f_capacity_d":
    "Angebot und Bedarf nach Kompetenzpool und Monat. Rote Monate zeigen, wo Signaltechnik-Ingenieure fehlen.",
  "p.demo.f_key_persons_t": "Schlüsselpersonen",
  "p.demo.f_key_persons_d":
    "Einzelne Ausfallpunkte: ein Ingenieur, den noch niemand ersetzen kann, und eine Rolle mit einer bereitstehenden Nachfolge.",
  "p.demo.f_objectives_t": "Ziele",
  "p.demo.f_objectives_d":
    "Drei Ziele, jeweils mit den Plänen verknüpft, die ihnen dienen.",
  "p.demo.f_okrs_t": "Ziele und Schlüsselergebnisse",
  "p.demo.f_okrs_d": "Schlüsselergebnisse mit Check-ins und Zuversicht.",
  "p.demo.f_kpis_t": "Kennzahlen",
  "p.demo.f_kpis_d":
    "Verspätungsminuten, Fahrgastzufriedenheit und Ausgaben gegenüber Plan, mit einem Messwert pro Monat.",
  "p.demo.f_proposals_t": "Arbeitsaufnahme",
  "p.demo.f_proposals_d":
    "Fünf Vorschläge in verschiedenen Phasen, vom Entwurf bis genehmigt und abgelehnt.",
  "p.demo.f_ideas_t": "Ideen",
  "p.demo.f_ideas_d":
    "Vier Ideen mit Stimmen. Wandeln Sie eine in einen Vorschlag um.",
  "p.demo.f_scenarios_t": "Szenarien",
  "p.demo.f_scenarios_d":
    "Was unter eine Budgetobergrenze passt, wenn die Signaltechnik zuerst kommt.",
  "p.demo.f_director_packs_t": "Führungsberichte",
  "p.demo.f_director_packs_d":
    "Berichte für Geschäftsführung und Vorstand für September, aus den aktuellen Daten zusammengestellt. Öffnen Sie einen, um alle Abschnitte zu sehen.",
  "p.demo.f_organizations_t": "Partnerorganisationen",
  "p.demo.f_organizations_d":
    "Die Behörde, ihr Lieferunternehmen, ein Partner und ein Lieferant.",
  "p.demo.f_milestone_signoffs_t": "Meilenstein-Abnahme",
  "p.demo.f_milestone_signoffs_d":
    "Für die Inbetriebnahme müssen Partner und Lieferant unterzeichnen. Der Lieferant hat es getan, der Partner nicht, daher ist die Inbetriebnahme geliefert, aber nicht erreicht.",
  "p.demo.f_cutover_t": "Umstellung: Go/No-Go",
  "p.demo.f_cutover_d":
    "Drei Stoppkriterien, eines erfüllt, eine Mitzeichnung liegt vor. Ein Go würde abgelehnt, und die Ansicht sagt, warum.",
  "p.demo.f_deployment_moves_t": "Verschiebungen der Bereitstellung",
  "p.demo.f_deployment_moves_d":
    "Ein Antrag, den Installationsbeginn um zwei Wochen zu verschieben, mit den Folgen für andere Aufgaben und den Personalkosten, wartet auf Genehmigung.",
  "p.demo.f_start_queue_t": "Startwarteschlange",
  "p.demo.f_start_queue_d":
    "Zwei Programme sind in der Lieferung und das Limit ist zwei, daher wartet das dritte der Reihe nach in der Warteschlange.",
  "p.demo.f_critical_chain_t": "Kritische Kette",
  "p.demo.f_critical_chain_d":
    "Die Kette durch die abhängigen Aufgaben und die Personen, die sie teilen, mit ambitionierten und sicheren Schätzungen.",
  "p.demo.f_buffers_t": "Puffermanagement",
  "p.demo.f_buffers_d":
    "Der eingefrorene Pufferplan, Fieberkurven und eine Korrekturmaßnahme.",
  "p.demo.f_pipeline_t": "Trommel und Pipeline",
  "p.demo.f_pipeline_d":
    "Der Kompetenzpool, von dem das Portfolio am stärksten abhängt, und vorgeschlagene gestaffelte Starts.",
  "p.demo.f_search_t": "Suche",
  "p.demo.f_search_d":
    "Tippen Sie in die Lupe in der Kopfzeile. Dieser Link sucht nach „signal“.",
  "p.demo.f_email_deliveries_t": "E-Mail-Zustellungen",
  "p.demo.f_email_deliveries_d":
    "Jede E-Mail, die der Dienst zu senden versucht hat. Aus der Demonstration wird nichts gesendet, daher ist diese Liste leer.",
  "p.demo.f_display_t": "Anzeigeeinstellungen",
  "p.demo.f_display_d": "Textgröße, Themen, einfache Ansicht und Sprache.",
  "p.kanban.nav": "Mein Kanban",
  "p.kanban.title": "Mein Kanban-Board — PPM",
  "p.kanban.h1": "Mein Kanban-Board",
  "p.kanban.lede":
    "Ihre offenen Aufgaben aus allen Plänen, in Spalten nach Status. Verschieben Sie eine Karte mit der Maus oder drücken Sie auf einer Karte die Eingabetaste, um eine Spalte aus einem Menü zu wählen. Eine Verschiebung ändert den Status der Aufgabe für alle.",
  "p.kanban.board_label": "Ihre Aufgaben nach Status",
  "p.kanban.loading": "Ihr Board wird geladen…",
  "p.kanban.none": "Sie haben keine offenen Aufgaben.",
  "p.kanban.todo": "Zu erledigen",
  "p.kanban.in_progress": "In Arbeit",
  "p.kanban.in_review": "In Prüfung",
  "p.kanban.blocked": "Blockiert",
  "p.kanban.done": "Erledigt",
  "p.kanban.card_count": "{n} Karten",
  "p.kanban.move_button": "{title} verschieben",
  "p.kanban.move_menu": "In Spalte verschieben",
  "p.kanban.moved": "{title} nach {column} verschoben",
  "p.kanban.due": "fällig am {date}",
  "p.kanban.points": "{n} Punkte",
  "p.kanban.sign_in": "Melden Sie sich an, um Ihr Board zu sehen.",
  "p.kanban.could_not_load": "Ihr Board konnte nicht geladen werden",
  "p.kanban.could_not_move":
    "Die Aufgabe konnte nicht verschoben werden; das Board ist unverändert.",
  "p.kanban.hidden":
    "{n} Aufgaben werden nicht angezeigt, da Sie ihre Pläne nicht lesen dürfen.",
  "p.dashboard.my_home": "Meine Startseite",
  "p.demo.banner":
    "Dies ist eine Demonstration mit fiktiven Daten. Nichts hier ist echt.",
  "p.demo.banner_tour": "Rundgang machen",
  "p.demo.signin_h2": "Demonstration",
  "p.demo.signin_lede":
    "Diese Website ist eine Demonstration. Melden Sie sich mit example@example.com an, um alle Funktionen mit fiktiven Daten zu sehen. Es wird keine E-Mail gesendet.",
  "p.demo.signin_button": "Bei der Demonstration anmelden",
  "p.demo.signin_wrong":
    "Die Demonstration akzeptiert nur example@example.com.",
  "p.search.title": "Suche — PPM",
  "p.search.h1": "Pläne durchsuchen",
  "p.search.lede":
    "Pläne, deren Name, Bezeichner, Schlüsselwörter oder Tags zu Ihrer Suche passen. Pläne, die Sie nicht lesen dürfen, werden nicht aufgeführt.",
  "p.search.none_asked":
    "Geben Sie etwas in das Suchfeld in der Kopfzeile ein.",
  "p.search.searching": "Wird gesucht…",
  "p.search.results": "{n} Pläne passen zu „{q}“.",
  "p.search.no_results": "Kein Plan passt zu „{q}“.",
  "p.search.could_not_search": "Suche nicht möglich",
  "p.layout.link_picker": "Seiten",
  "p.layout.search_picker": "Pläne durchsuchen",
  "p.layout.search_input": "Suchbegriffe",
  "p.layout.search_submit": "Suchen",
  "p.layout.lp_home": "Start",
  "p.layout.lp_plans": "Pläne",
  "p.layout.lp_dashboard": "Dashboard",
  "p.layout.lp_milestones": "Meilensteine",
  "p.layout.lp_proposals": "Vorschläge",
  "p.cutover.link": "Umstellung: Go/No-Go",
  "p.cutover.title": "Umstellung: Go/No-Go — PPM",
  "p.cutover.h1": "Umstellung: Go/No-Go",
  "p.cutover.lede":
    "Die Entscheidung, ein sicherheitskritisches System umzustellen. Die Stoppkriterien werden zuerst verfasst und veröffentlicht und dann eingefroren: Eine Änderung erzeugt eine neue Version. Ein Go erfordert, dass jedes Kriterium erfüllt ist und jede erforderliche Mitzeichnung vorliegt. Ein No-Go oder eine Verzögerung aus Sicherheitsgründen ist eine auf Basis der Kriterien getroffene Entscheidung, kein Verzug.",
  "p.cutover.loading": "Entscheidungen werden geladen…",
  "p.cutover.none": "Es wurde noch keine Umstellungsentscheidung entworfen.",
  "p.cutover.new_h2": "Umstellungsentscheidung entwerfen",
  "p.cutover.new_title": "Titel",
  "p.cutover.new_criteria":
    "Stoppkriterien, eines pro Zeile (jeweils 5 bis 500 Zeichen)",
  "p.cutover.draft": "Entscheidung entwerfen",
  "p.cutover.version": "Version {n}",
  "p.cutover.status_draft": "Entwurf",
  "p.cutover.status_published": "veröffentlicht, Kriterien eingefroren",
  "p.cutover.status_open": "zur Entscheidung offen",
  "p.cutover.status_decided": "entschieden",
  "p.cutover.status_superseded": "ersetzt",
  "p.cutover.outcome_go": "Go",
  "p.cutover.outcome_no_go": "No-Go",
  "p.cutover.outcome_delay_for_safety": "Verzögerung aus Sicherheitsgründen",
  "p.cutover.decided_line": "Entschieden am {on}: {outcome}. {why}",
  "p.cutover.criteria_h3": "Stoppkriterien",
  "p.cutover.col_n": "Nr.",
  "p.cutover.col_criterion": "Kriterium",
  "p.cutover.col_met": "Erfüllt",
  "p.cutover.col_evidence": "Nachweis",
  "p.cutover.col_mark": "Markieren",
  "p.cutover.met": "erfüllt",
  "p.cutover.not_met": "nicht erfüllt",
  "p.cutover.evidence": "Nachweis",
  "p.cutover.mark_met": "Als erfüllt markieren: {name}",
  "p.cutover.mark_unmet": "Als nicht erfüllt markieren: {name}",
  "p.cutover.mark_met_short": "Erfüllt",
  "p.cutover.mark_unmet_short": "Nicht erfüllt",
  "p.cutover.cosign_h3": "Mitzeichnungen",
  "p.cutover.cosign_signed": "Unterzeichnet: {roles}.",
  "p.cutover.cosign_missing": "Noch erforderlich: {roles}.",
  "p.cutover.cosign_stale":
    "Für frühere Nachweise unterzeichnet, daher erneut zu unterzeichnen: {roles}.",
  "p.cutover.cosign_none": "keine",
  "p.cutover.cosign_role":
    "Rolle, in der Sie zeichnen (Ihr Token muss sie enthalten)",
  "p.cutover.cosign": "Mitzeichnen",
  "p.cutover.refused_h3": "Ein Go würde abgelehnt, weil",
  "p.cutover.publish": "Veröffentlichen und Kriterien einfrieren",
  "p.cutover.open": "Entscheidung eröffnen",
  "p.cutover.new_version": "Neue Version beginnen, um die Kriterien zu ändern",
  "p.cutover.decide_h3": "Entscheiden",
  "p.cutover.outcome": "Ergebnis",
  "p.cutover.reason": "Grund",
  "p.cutover.decide": "Entscheidung erfassen",
  "p.cutover.could_not_load": "Die Entscheidungen konnten nicht geladen werden",
  "p.cutover.could_not_save": "Speichern nicht möglich",
  "p.cutover.cancel_edit": "Bearbeiten beenden",
  "p.cutover.criteria_frozen":
    "Diese Entscheidung ist veröffentlicht, ihre Kriterien sind daher eingefroren: Legen Sie eine neue Version an, um sie zu ändern.",
  "p.cutover.criteria_rules":
    "Ein Abbruchkriterium pro Zeile: 1 bis 30, jedes 5 bis 500 Zeichen. Sie lassen sich bis zur Veröffentlichung der Entscheidung ändern.",
  "p.cutover.criteria_saved": "Die Abbruchkriterien sind gespeichert.",
  "p.cutover.criteria_unchanged":
    "Nichts wurde geändert, daher wurde nichts gesendet.",
  "p.cutover.edit_criteria_label": "Abbruchkriterien, eines pro Zeile",
  "p.cutover.edit_criteria": "Abbruchkriterien ändern",
  "p.cutover.problem_count":
    "Eine Entscheidung braucht 1 bis 30 Abbruchkriterien.",
  "p.cutover.problem_length":
    "Abbruchkriterium {line} muss 5 bis 500 Zeichen haben.",
  "p.cutover.save_criteria": "Abbruchkriterien speichern",
  "p.moves.link": "Verschiebungen der Bereitstellung",
  "p.moves.title": "Verschiebungen der Bereitstellung — PPM",
  "p.moves.h1": "Verschiebungen der Bereitstellung",
  "p.moves.lede":
    "Ein Antrag, den Beginn einer Aufgabe zu verschieben. Nichts ändert sich, bis die genehmigende Person zustimmt. Die Folgen für andere Aufgaben und deren Personalkosten wurden bei Antragstellung ermittelt, indem der Terminplan mit dem verschobenen Datum berechnet wurde.",
  "p.moves.loading": "Verschiebungen werden geladen…",
  "p.moves.none": "Es wurde keine Verschiebung beantragt.",
  "p.moves.request_h2": "Verschiebung beantragen",
  "p.moves.task": "Aufgabe",
  "p.moves.new_start": "Neuer Beginn",
  "p.moves.reason": "Grund",
  "p.moves.approver":
    "Genehmigende Person (person:<id> oder role:<name>, optional)",
  "p.moves.request": "Verschiebung beantragen",
  "p.moves.col_task": "Aufgabe",
  "p.moves.col_dates": "Beginn",
  "p.moves.col_reason": "Grund",
  "p.moves.col_by": "Beantragt von",
  "p.moves.col_approver": "Genehmigende Person",
  "p.moves.col_status": "Status",
  "p.moves.col_knock": "Folgen",
  "p.moves.col_act": "Entscheiden",
  "p.moves.status_requested": "wartet auf Genehmigung",
  "p.moves.status_approved": "genehmigt",
  "p.moves.status_refused": "abgelehnt",
  "p.moves.knock":
    "{n} andere Aufgaben verschieben sich; Personalkosten {cost}",
  "p.moves.no_cost": "keine Änderung der Personalkosten",
  "p.moves.decision_reason": "Grund für die Entscheidung",
  "p.moves.approve": "Genehmigen",
  "p.moves.refuse": "Ablehnen",
  "p.moves.approve_for": "Verschiebung von {name} genehmigen",
  "p.moves.refuse_for": "Verschiebung von {name} ablehnen",
  "p.moves.decided": "{by} am {on}: {why}",
  "p.moves.by_requester_h2": "Folgekosten nach antragstellender Person",
  "p.moves.requester_row": "{who}: {n} Anträge; Personal {cost}",
  "p.moves.could_not_load": "Die Verschiebungen konnten nicht geladen werden",
  "p.moves.could_not_save": "Speichern nicht möglich",
  "p.ledger.nav": "Finanzierungsbuch",
  "p.ledger.title": "Finanzierungsbuch — PPM",
  "p.ledger.h1": "Finanzierungsbuch",
  "p.ledger.lede":
    "Jede Mittelzuweisung, zurückverfolgt zu den Budgetpositionen, die darauf zugreifen, ihren Gates, den erreichten Meilensteinen und den validierten Nutzen. Zugewiesenes Geld, das auf keiner für Sie lesbaren Position eingeplant ist, gilt als nicht zugeordnet und wird mit seinem Betrag angezeigt.",
  "p.ledger.scope":
    "Zuweisungen anlegen, Kostenstellen zuordnen und einen Finanzexport abstimmen erfolgt über die API; diese Ansicht liest das Buch.",
  "p.ledger.loading": "Buch wird geladen…",
  "p.ledger.none": "Es wurde keine Mittelzuweisung erfasst.",
  "p.ledger.col_source": "Finanzierungsquelle",
  "p.ledger.col_ref": "Referenz",
  "p.ledger.col_on": "Zugewiesen am",
  "p.ledger.col_amount": "Zugewiesen",
  "p.ledger.col_planned": "Auf Positionen eingeplant",
  "p.ledger.col_untraced": "Nicht zugeordnet",
  "p.ledger.col_lines": "Positionen",
  "p.ledger.col_act": "Verfolgen",
  "p.ledger.trace": "{name} verfolgen",
  "p.ledger.trace_short": "Verfolgen",
  "p.ledger.trace_h2": "Wofür {name} verwendet wurde",
  "p.ledger.money":
    "Zugewiesen {allocated}, auf Positionen eingeplant {planned}, ausgegeben {spent}, nicht zugeordnet {untraced} ({currency}).",
  "p.ledger.gates": "Positionen mit Gate: {gated}. Freigegeben: {released}.",
  "p.ledger.milestones_met": "Erreichte Meilensteine: {met} von {total}.",
  "p.ledger.benefits": "Erfasste Nutzen: {n}.",
  "p.ledger.validated": "Validiert in {currency}: {amount}",
  "p.ledger.lines_h3": "Budgetpositionen",
  "p.ledger.line_plan": "Programm",
  "p.ledger.line_desc": "Beschreibung",
  "p.ledger.line_planned": "Geplant",
  "p.ledger.line_actual": "Ausgegeben",
  "p.ledger.line_gate": "Gate",
  "p.ledger.released": "freigegeben",
  "p.ledger.held": "zurückgehalten",
  "p.ledger.no_gate": "kein Gate",
  "p.ledger.could_not_load": "Das Buch konnte nicht geladen werden",
  "p.ledger.could_not_trace": "Die Zuweisung konnte nicht verfolgt werden",
  "p.ledger.a_line":
    "ein Budgetposten (wählen Sie oben seinen Plan, um ihn zu sehen)",
  "p.ledger.choose_line": "Posten wählen",
  "p.ledger.choose_plan_first": "Wählen Sie zuerst oben einen Plan",
  "p.ledger.choose_plan": "Plan wählen",
  "p.ledger.choose_source": "Finanzierungsquelle wählen",
  "p.ledger.code_mapped": "Kostencode zugeordnet.",
  "p.ledger.codes_heading": "Kostencodes",
  "p.ledger.codes_says":
    "Jeder Kostencode der Finanzbuchhaltung gehört zu einem Budgetposten, damit ein Finanzexport neben das Register gelegt werden kann. Ein schon zugeordneter Code wird verschoben.",
  "p.ledger.could_not_load_lines":
    "Die Budgetposten dieses Plans konnten nicht geladen werden.",
  "p.ledger.could_not_reconcile": "Der Export konnte nicht abgeglichen werden.",
  "p.ledger.could_not_record": "Die Zuweisung wurde nicht erfasst.",
  "p.ledger.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.ledger.f_allocation": "Zuweisung, aus der er schöpft",
  "p.ledger.f_amount": "Betrag",
  "p.ledger.f_code_line": "Budgetposten (aus dem Plan oben)",
  "p.ledger.f_code": "Kostencode",
  "p.ledger.f_csv": "Oder den Export einfügen (CSV)",
  "p.ledger.f_currency": "Währung",
  "p.ledger.f_file": "Datei des Finanzexports (CSV)",
  "p.ledger.f_on": "Zugewiesen am",
  "p.ledger.f_plan": "Gewählter Plan",
  "p.ledger.f_reference": "Referenz (optional)",
  "p.ledger.f_source": "Finanzierungsquelle",
  "p.ledger.line_detached": "Der Posten schöpft aus keiner Zuweisung mehr.",
  "p.ledger.line_drawn": "Der Posten schöpft jetzt aus dieser Zuweisung.",
  "p.ledger.lines_heading":
    "Budgetposten und die Zuweisungen, aus denen sie schöpfen",
  "p.ledger.lines_says":
    "Ein Posten schöpft aus einer Zuweisung seiner eigenen Finanzierungsquelle und Währung; legen Sie seine Quelle zuerst auf der Seite Finanzierung fest.",
  "p.ledger.map_code": "Kostencode zuordnen",
  "p.ledger.matched_heading": "Zugeordnete Kostencodes",
  "p.ledger.matched_line":
    "im Export {ledger}, im Register {register}, Differenz {difference}",
  "p.ledger.mismatch_heading": "Zeilen in einer anderen Währung als ihr Posten",
  "p.ledger.mismatch_line":
    "Zeile in {row}, Posten in {line}: {amount}, nicht mitgezählt",
  "p.ledger.needs_source": "braucht zuerst eine Finanzierungsquelle",
  "p.ledger.no_allocation": "Schöpft aus keiner Zuweisung",
  "p.ledger.no_codes": "Noch ist kein Kostencode zugeordnet.",
  "p.ledger.no_lines": "Dieser Plan hat keine Budgetposten.",
  "p.ledger.no_matching_allocation":
    "Noch keine Zuweisung dieser Quelle und Währung",
  "p.ledger.no_source": "keine Finanzierungsquelle",
  "p.ledger.not_reconciled":
    "Nicht abgeglichen: Etwas stimmt nicht überein. Nichts wurde geschrieben.",
  "p.ledger.problem_amount": "Geben Sie den Betrag als Zahl über 0 an.",
  "p.ledger.problem_code_line": "Wählen Sie den Budgetposten für den Code.",
  "p.ledger.problem_code": "Ein Kostencode hat 1 bis 60 Zeichen.",
  "p.ledger.problem_currency":
    "Geben Sie die Währung mit drei Buchstaben an, etwa GBP.",
  "p.ledger.problem_date_future":
    "Das Zuweisungsdatum darf nicht in der Zukunft liegen.",
  "p.ledger.problem_date": "Wählen Sie das Datum der Zuweisung.",
  "p.ledger.problem_export_empty":
    "Wählen Sie zuerst eine Datei oder fügen Sie den Export ein.",
  "p.ledger.problem_export_header":
    "Die erste Zeile des Exports muss cost_code, amount_minor und currency nennen.",
  "p.ledger.problem_export_large":
    "Der Export ist größer als 1 MB: Teilen Sie ihn.",
  "p.ledger.problem_reference":
    "Eine Referenz darf höchstens 120 Zeichen lang sein.",
  "p.ledger.problem_source": "Wählen Sie eine Finanzierungsquelle.",
  "p.ledger.reconcile_heading": "Finanzexport abgleichen",
  "p.ledger.reconcile_says":
    "Der Kostencode jeder Zeile wird in der Zuordnung gesucht und die Beträge werden neben das gelegt, was das Register als ausgegeben für den Posten führt. Nichts wird geschrieben.",
  "p.ledger.reconcile": "Abgleichen",
  "p.ledger.reconciled":
    "Abgeglichen: Jede Zeile passte zu einem Posten, in seiner Währung, ohne Differenz.",
  "p.ledger.record_heading": "Zuweisung erfassen",
  "p.ledger.record_says":
    "Geld, das eine Finanzierungsquelle zugewiesen hat. Zuweisungen bleiben wie erfasst und werden nie geändert.",
  "p.ledger.record": "Zuweisung erfassen",
  "p.ledger.recorded": "Zuweisung erfasst.",
  "p.ledger.refused_export":
    "Der Export konnte nicht gelesen werden: Jede Zeile braucht einen Kostencode, einen ganzen Betrag in kleinsten Einheiten und eine Währung, und nicht mehr, als der Dienst annimmt.",
  "p.ledger.refused_line":
    "Der Posten kann nicht aus dieser Zuweisung schöpfen: Beide brauchen dieselbe Finanzierungsquelle und Währung.",
  "p.ledger.rows_read_one": "{n} Zeile gelesen.",
  "p.ledger.rows_read_other": "{n} Zeilen gelesen.",
  "p.ledger.to_funding":
    "Finanzierungsquellen auf der Seite Finanzierung hinzufügen",
  "p.ledger.unmatched_heading": "Kostencodes ohne Posten, den Sie lesen dürfen",
  "p.signoffs.nav": "Meilenstein-Abnahme",
  "p.signoffs.title": "Meilenstein-Abnahme — PPM",
  "p.signoffs.h1": "Meilenstein-Abnahme",
  "p.signoffs.lede":
    "Manche Meilensteine sind erst erreicht, wenn die darin genannten Partnerorganisationen sie abgenommen haben. Geliefert bedeutet, dass die liefernde Organisation sie für fertig erklärt; erreicht bedeutet geliefert mit allen gültigen Abnahmen. Eine zurückgezogene Abnahme gilt wieder als ausstehend.",
  "p.signoffs.loading": "Abnahmen werden geladen…",
  "p.signoffs.none": "Kein Meilenstein benötigt eine Partnerabnahme.",
  "p.signoffs.totals":
    "Abnahme erforderlich: {n}. Geliefert: {delivered}. Erreicht: {met}. Geliefert, aber nicht erreicht: {gap}. Erreicht, als Anteil der gelieferten: {share}.",
  "p.signoffs.no_share": "nicht gemessen",
  "p.signoffs.col_milestone": "Meilenstein",
  "p.signoffs.col_due": "Fällig",
  "p.signoffs.col_state": "Zustand",
  "p.signoffs.col_delivered": "Geliefert am",
  "p.signoffs.col_met": "Erreicht am",
  "p.signoffs.col_outstanding": "Wartet auf",
  "p.signoffs.col_act": "Abnehmen oder zurückziehen",
  "p.signoffs.state_met": "erreicht",
  "p.signoffs.state_delivered_not_met": "geliefert, nicht erreicht",
  "p.signoffs.state_not_delivered": "nicht geliefert",
  "p.signoffs.organization": "Organisation",
  "p.signoffs.reason": "Grund (bei Rückzug)",
  "p.signoffs.sign": "Abnehmen",
  "p.signoffs.withdraw": "Zurückziehen",
  "p.signoffs.sign_for": "{name} als gewählte Organisation abnehmen",
  "p.signoffs.withdraw_for": "Abnahme für {name} zurückziehen",
  "p.signoffs.not_built":
    "Welche Organisationen ein Meilenstein braucht, wird auf der Governance-Seite jedes Plans gewählt; diese Seite zeigt und erfasst die Abzeichnungen.",
  "p.signoffs.problem_inactive":
    "{name} ist nicht mehr aktiv und kann daher nicht verlangt werden: entfernen Sie das Häkchen, um zu speichern.",
  "p.signoffs.problem_too_many":
    "Ein Meilenstein kann höchstens {max} Organisationen verlangen.",
  "p.signoffs.required_cancel": "Abbrechen",
  "p.signoffs.required_change_for": "Ändern, wer {name} abzeichnen muss",
  "p.signoffs.required_change": "Ändern, wer abzeichnen muss",
  "p.signoffs.required_cleared":
    "{name} braucht keine Partnerabzeichnung mehr.",
  "p.signoffs.required_heading": "Partnerabzeichnung",
  "p.signoffs.required_inactive": "{name} (nicht mehr aktiv)",
  "p.signoffs.required_legend":
    "Organisationen, die {name} abzeichnen müssen (höchstens {max})",
  "p.signoffs.required_link": "Abzeichnung erfassen",
  "p.signoffs.required_no_orgs":
    "Es gibt keine aktive Organisation zur Auswahl.",
  "p.signoffs.required_none": "keine Partnerabzeichnung nötig",
  "p.signoffs.required_save": "Speichern, wer abzeichnen muss",
  "p.signoffs.required_saved": "Gespeichert, wer {name} abzeichnen muss.",
  "p.signoffs.required_says":
    "Ein Meilenstein mit Partnerabzeichnung ist erst erreicht, wenn jede Organisation unten abgezeichnet hat; geliefert und erreicht sind getrennte Daten.",
  "p.signoffs.required_signed": "{name} hat am {date} abgezeichnet",
  "p.signoffs.required_unchanged":
    "Nichts wurde geändert, daher wurde nichts gesendet.",
  "p.signoffs.required_waiting": "{name} hat noch nicht abgezeichnet",
  "p.signoffs.could_not_load": "Die Abnahmen konnten nicht geladen werden",
  "p.signoffs.could_not_save": "Die Abnahme konnte nicht erfasst werden",
  "p.key_persons.nav": "Schlüsselpersonen",
  "p.key_persons.title": "Schlüsselpersonen — PPM",
  "p.key_persons.h1": "Schlüsselpersonen und einzelne Ausfallpunkte",
  "p.key_persons.lede":
    "Die Personen und Rollen, deren Verlust dem Portfolio schaden würde, wer übernehmen würde und wie bereit diese sind. Nur Ressourcenmanager können dies lesen, da es benannte Personen betrifft.",
  "p.key_persons.loading": "Wird geladen…",
  "p.key_persons.none": "Es wurde keine Schlüsselperson oder -rolle erfasst.",
  "p.key_persons.not_allowed":
    "Nur Ressourcenmanager können die Schlüsselpersonen lesen.",
  "p.key_persons.add_h2": "Schlüsselperson oder -rolle erfassen",
  "p.key_persons.subject_kind": "Person oder Rolle",
  "p.key_persons.kind_person": "Person",
  "p.key_persons.kind_role": "Rolle",
  "p.key_persons.subject": "Name oder Rolle",
  "p.key_persons.successor": "Nachfolge",
  "p.key_persons.readiness": "Bereitschaft der Nachfolge",
  "p.key_persons.ready_ready": "bereit",
  "p.key_persons.ready_partly": "teilweise bereit",
  "p.key_persons.ready_none": "nicht bereit",
  "p.key_persons.note": "Notiz",
  "p.key_persons.add": "Erfassen",
  "p.key_persons.col_subject": "Person oder Rolle",
  "p.key_persons.col_successor": "Nachfolge",
  "p.key_persons.col_ready": "Bereitschaft",
  "p.key_persons.col_note": "Notiz",
  "p.key_persons.col_act": "Aktion",
  "p.key_persons.remove": "{name} entfernen",
  "p.key_persons.remove_short": "Entfernen",
  "p.key_persons.no_successor": "keine Nachfolge benannt",
  "p.key_persons.contingent_h2": "Befristete Arbeitskräfte",
  "p.key_persons.contingent_note":
    "Als befristete Arbeitskräfte markierte Personen, die kurzfristig gehen können.",
  "p.key_persons.contingent_none": "Niemand ist als befristet markiert.",
  "p.key_persons.contingent_person": "Personenreferenz",
  "p.key_persons.contingent_unmark": "Markierung für {name} aufheben",
  "p.key_persons.mark": "Markieren",
  "p.key_persons.unmark": "Markierung aufheben",
  "p.key_persons.could_not_load":
    "Die Schlüsselpersonen konnten nicht geladen werden",
  "p.key_persons.could_not_save": "Speichern nicht möglich",
  "p.organizations.nav": "Organisationen",
  "p.organizations.title": "Organisationen — PPM",
  "p.organizations.h1": "Partnerorganisationen",
  "p.organizations.lede":
    "Die Organisationen, die im Portfolio liefern, fördern, zuliefern oder partnerschaftlich mitwirken. Ein Bericht kann an eine Organisation gerichtet sein, ein Meilenstein kann ihre Abnahme benötigen, und sie kann Kapazität zusagen. Wird eine Organisation stillgelegt, bleibt ihr Verlauf erhalten.",
  "p.organizations.loading": "Organisationen werden geladen…",
  "p.organizations.none": "Es wurde noch keine Organisation hinzugefügt.",
  "p.organizations.add_h2": "Organisation hinzufügen",
  "p.organizations.name": "Name",
  "p.organizations.kind": "Art",
  "p.organizations.add": "Hinzufügen",
  "p.organizations.col_name": "Name",
  "p.organizations.col_kind": "Art",
  "p.organizations.col_state": "Zustand",
  "p.organizations.col_audience": "Berichtsempfänger",
  "p.organizations.col_act": "Aktion",
  "p.organizations.kind_delivery": "Lieferung",
  "p.organizations.kind_partner": "Partner",
  "p.organizations.kind_sponsor": "Sponsor",
  "p.organizations.kind_supplier": "Lieferant",
  "p.organizations.active": "aktiv",
  "p.organizations.retired": "stillgelegt",
  "p.organizations.retire": "{name} stilllegen",
  "p.organizations.restore": "{name} wiederherstellen",
  "p.organizations.retire_short": "Stilllegen",
  "p.organizations.restore_short": "Wiederherstellen",
  "p.organizations.could_not_load":
    "Die Organisationen konnten nicht geladen werden",
  "p.organizations.could_not_save":
    "Die Organisation konnte nicht gespeichert werden",
  "p.email_deliveries.nav": "E-Mail-Zustellungen",
  "p.email_deliveries.title": "E-Mail-Zustellungen — PPM",
  "p.email_deliveries.h1": "E-Mail-Zustellungen",
  "p.email_deliveries.lede":
    "Jede E-Mail, die der Dienst zu senden versucht hat, eine Zeile pro Empfänger: ob sie gesendet wurde, auf einen erneuten Versuch wartet oder endgültig fehlgeschlagen ist, wie viele Versuche unternommen wurden und der letzte Fehler. Nur Administratoren können dies lesen, da es E-Mail-Adressen enthält.",
  "p.email_deliveries.loading": "Zustellungen werden geladen…",
  "p.email_deliveries.none": "Es wurde keine E-Mail gesendet oder versucht.",
  "p.email_deliveries.counts":
    "Gesendet: {sent}. Wartet auf erneuten Versuch: {retry}. Endgültig fehlgeschlagen: {failed}.",
  "p.email_deliveries.filter": "Anzeigen",
  "p.email_deliveries.all": "Alle",
  "p.email_deliveries.col_recipient": "Empfänger",
  "p.email_deliveries.col_subject": "Betreff",
  "p.email_deliveries.col_status": "Status",
  "p.email_deliveries.col_attempts": "Versuche",
  "p.email_deliveries.col_error": "Letzter Fehler",
  "p.email_deliveries.col_next": "Nächster Versuch",
  "p.email_deliveries.col_sent": "Gesendet um",
  "p.email_deliveries.status_queued": "in Warteschlange",
  "p.email_deliveries.status_retry": "wird erneut versucht",
  "p.email_deliveries.status_sent": "gesendet",
  "p.email_deliveries.status_failed": "fehlgeschlagen",
  "p.email_deliveries.not_allowed":
    "Nur Administratoren können E-Mail-Zustellungen lesen.",
  "p.email_deliveries.could_not_load":
    "Die E-Mail-Zustellungen konnten nicht geladen werden",
  "p.director_pack.s_signoffs": "Meilenstein-Abnahme",
  "p.director_pack.signoffs_note":
    "Manche Meilensteine sind erst erreicht, wenn die darin genannten Partnerorganisationen sie abgenommen haben. Geliefert bedeutet, dass die liefernde Organisation sie für fertig erklärt; erreicht bedeutet geliefert mit allen gültigen Abnahmen. Eine zurückgezogene Abnahme gilt als ausstehend.",
  "p.director_pack.signoffs_totals":
    "Abnahme erforderlich: {n}. Geliefert: {delivered}. Erreicht: {met}. Geliefert, aber nicht erreicht: {gap}. Erreicht, als Anteil der gelieferten: {share}.",
  "p.director_pack.so_met": "erreicht",
  "p.director_pack.so_delivered_not_met": "geliefert, nicht erreicht",
  "p.director_pack.so_not_delivered": "nicht geliefert",
  "p.director_pack.so_no_share": "noch nichts geliefert",
  "p.director_pack.c_milestone": "Meilenstein",
  "p.director_pack.c_due": "Fällig",
  "p.director_pack.c_delivered_on": "Geliefert am",
  "p.director_pack.c_met_on": "Erreicht am",
  "p.director_pack.c_outstanding": "Abnahme ausstehend von",
  "p.director_pack.c_delivery": "Lieferung",
  "p.director_pack.dm_not_stated": "nicht angegeben",
  "p.director_pack.dm_central_product": "zentrales Produkt",
  "p.director_pack.dm_central_platform_local_deployment":
    "zentrale Plattform, lokale Bereitstellung",
  "p.director_pack.dm_local_programme_central_standards":
    "lokales Programm, zentrale Standards",
  "p.director_pack.dm_owner": "Verantwortlich: {name}",
  "p.director_pack.dm_not_ours": "nicht von der Organisation geliefert",
  "p.director_pack.s_flow": "Burn-up und Fluss (Produktprogramme)",
  "p.director_pack.c_scope": "Erstellte Aufgaben",
  "p.director_pack.c_finished": "Abgeschlossene Aufgaben",
  "p.director_pack.c_open": "Offene Aufgaben",
  "p.director_pack.c_throughput":
    "Abgeschlossen pro Woche (letzte vier Wochen)",
  "p.director_pack.c_burnup": "Abgeschlossen, Woche für Woche (älteste zuerst)",
  "p.director_pack.no_flow":
    "{name}: ein Produktprogramm noch ohne Aufgaben, daher kein Burn-up und kein Fluss.",
  "p.director_pack.s_asks": "Externe Anfragen und ihre Zielkonflikte",
  "p.director_pack.asks_note":
    "Eine Anfrage des Sponsors kann erst genehmigt werden, wenn sie benennt, was sie verdrängt. Hier werden die im Zeitraum gestellten Anfragen aufgeführt.",
  "p.director_pack.asks_count":
    "Im Zeitraum gestellte Anfragen: {n}. Noch ohne benannten Zielkonflikt: {without}.",
  "p.director_pack.ask_none": "noch kein Zielkonflikt benannt",
  "p.director_pack.c_ask": "Anfrage",
  "p.director_pack.c_tradeoff": "Was sie verdrängt",
  "p.director_pack.register_line":
    "Programme mit vollständigem Register: {complete} von {n}.",
  "p.director_pack.register_enforced":
    "Bei einem Programm mit unvollständigem Register wird keine Budgetposition freigegeben und keine feste Zuweisung gebucht.",
  "p.director_pack.register_not_enforced":
    "Ein unvollständiges Register ist eine Warnung, keine Sperre: Geld kann weiterhin fließen, und die Warnung wird protokolliert.",
  "p.director_pack.s_register": "Programme mit unvollständigem Register",
  "p.director_pack.c_score": "Vollständig",
  "p.director_pack.c_missing": "Fehlend",
  "p.director_pack.rf_sro": "verantwortliche Führungskraft (SRO)",
  "p.director_pack.rf_partners": "Partner",
  "p.director_pack.rf_stage": "Phase",
  "p.director_pack.rf_baseline": "Terminplan-Baseline",
  "p.director_pack.rf_target_date": "Zieldatum",
  "p.director_pack.rf_budget": "Budget",
  "p.director_pack.rf_funding_source": "Finanzierungsquelle",
  "p.director_pack.rf_confidence": "Lieferzuversicht",
  "p.director_pack.rf_top_risks": "Top-Risiken",
  "p.director_pack.rf_dependencies": "Abhängigkeiten",
  "p.director_pack.s_watermelon": "Wassermelonen-Kennzahlen",
  "p.director_pack.wm_note":
    "Außen grün, innen rot. Diese Werte stammen aus diesem Bericht und dem vorherigen herausgegebenen Bericht wie gespeichert, nicht neu berechnet. Eine Kennzahl ohne Messgrundlage nennt den Grund; sie ist nicht null.",
  "p.director_pack.wm_gap":
    "Selbsteinschätzung und unabhängige Zuversicht weichen voneinander ab",
  "p.director_pack.wm_late":
    "Späte Entdeckung: jetzt rot oder gelb/rot, im vorherigen Bericht grün",
  "p.director_pack.wm_red": "Rot ohne erfasste Entscheidung im Zeitraum",
  "p.director_pack.wm_signoff":
    "Mit Abnahme erreichte Meilensteine, gemessen an den gelieferten",
  "p.director_pack.wm_signoff_of": "{met} erreicht von {delivered} geliefert",
  "p.director_pack.wm_not_measured": "nicht gemessen",
  "p.director_pack.c_measure": "Kennzahl",
  "p.director_pack.c_programmes": "Programme oder Grund für fehlende Messung",
  "p.director_pack.wm_count": "Programme: {n}",
  "p.director_pack.s_keypeople":
    "Schlüsselpersonen und befristete Arbeitskräfte",
  "p.director_pack.kp_note":
    "Einzelne Ausfallpunkte werden hier gezählt und nie namentlich genannt; das Register selbst ist für Ressourcenmanager bestimmt.",
  "p.director_pack.kp_counts":
    "Einzelne Ausfallpunkte: {n} ({people} Personen, {roles} Rollen). Ohne Nachfolge: {nosucc}. Nachfolge bereit: {ready}; teilweise bereit: {partly}; nicht bereit: {none}.",
  "p.director_pack.kp_no_people": "niemand in diesem Pool",
  "p.director_pack.c_headcount": "Personen",
  "p.director_pack.c_contingent": "Befristet",
  "p.director_pack.c_contingent_share": "Anteil befristet",
  "p.director_pack.s_triage": "Triage und Schwellenwerte",
  "p.director_pack.tr_note":
    "Die jeweils letzte Triage jedes Programms wird angezeigt. Eine Schwellenwertüberschreitung eröffnet eine Ausnahme, die eine Pause, einen Neustart, einen Stopp oder eine befristete Übersteuerung erfordert; nichts wird automatisch pausiert oder gestoppt.",
  "p.director_pack.tr_decisions":
    "Entscheidungen in diesem Zeitraum: Pause {pause}, Neustart {reset}, Stopp {stop}, Übersteuerung {override}.",
  "p.director_pack.tr_open":
    "Schwellenwertausnahmen, die auf eine Entscheidung warten: {n}.",
  "p.director_pack.tr_override_line":
    "Übersteuerung für {programme} gültig bis {expiry}.",
  "p.director_pack.tr_finish": "abschließen",
  "p.director_pack.tr_fix": "korrigieren",
  "p.director_pack.tr_pause": "pausieren",
  "p.director_pack.tr_stop": "stoppen",
  "p.director_pack.c_outcome": "Triage",
  "p.director_pack.c_triaged_on": "Triagiert am",
  "l.enum.pk_conf.threshold": "Schwellenwert überschritten",
  "p.director_pack.s_cutovers": "Umstellungsentscheidungen",
  "p.director_pack.co_note":
    "Über eine Umstellung wird anhand von Stoppkriterien entschieden, die vor der Entscheidung veröffentlicht wurden. Ein No-Go oder eine Verzögerung aus Sicherheitsgründen ist eine auf Basis dieser Kriterien getroffene Entscheidung und zählt nicht als Verzug.",
  "p.director_pack.co_counts":
    "Im Zeitraum entschieden: Go {go}, No-Go {nogo}, Verzögerung aus Sicherheitsgründen {delay}.",
  "p.director_pack.co_go": "Go",
  "p.director_pack.co_no_go": "No-Go",
  "p.director_pack.co_delay_for_safety": "Verzögerung aus Sicherheitsgründen",
  "p.director_pack.c_cutover": "Umstellung",
  "p.director_pack.c_decision": "Entscheidung",
  "p.director_pack.c_decided_on": "Entschieden am",
  "p.director_pack.s_outcomes": "Ergebnisse und Deeskalation",
  "p.director_pack.oc_note":
    "Die Eskalationsstufe ist die vom Sponsor festgelegte und wird hier nie berechnet. Die Folge unten zählt abgeschlossene Quartale, in denen jede Scorecard-Kennzahl und jedes Kriterium erfüllt ist; sie wird der entscheidenden Person angeboten.",
  "p.director_pack.oc_level": "Eskalationsstufe: {level}, festgelegt am {on}.",
  "p.director_pack.oc_no_level": "Es wurde keine Eskalationsstufe erfasst.",
  "p.director_pack.oc_run":
    "Aufeinanderfolgende erfüllte Quartale: {n}. Die Regel verlangt {need}.",
  "p.director_pack.oc_rule_yes":
    "Die Regel ist erfüllt. Ob eine Stufe zurückgegangen wird, entscheidet der Sponsor.",
  "p.director_pack.oc_rule_no": "Die Regel ist noch nicht erfüllt.",
  "p.director_pack.oc_met": "erfüllt",
  "p.director_pack.oc_not_met": "nicht erfüllt",
  "p.director_pack.oc_missing": "fehlt",
  "p.director_pack.c_quarter": "Quartal",
  "p.director_pack.c_why": "Begründung",
  "p.balance.balance": "Portfolio-Balance",
  "p.balance.balance_ppm": "Portfolio-Balance — PPM",
  "p.balance.intro":
    "Wie sich geplante Kosten, angestrebter Nutzwert und offenes Risiko der Portfolios zueinander verhalten, und die Ungleichgewichte, die sich aus den Zahlen ergeben.",
  "p.balance.loading": "Wird geladen…",
  "p.balance.could_not_load":
    "Die Portfolio-Balance konnte nicht geladen werden",
  "p.balance.none":
    "Für noch kein Portfolio sind Kosten, Nutzwert oder Risiko erfasst.",
  "p.balance.currency": "Währung",
  "p.balance.unassigned": "Kein Portfolio",
  "p.balance.caption":
    "Jede Blase ist ein Portfolio. Waagerecht: geplante Kosten in {currency}. Senkrecht: angestrebter Nutzwert in {currency}. Fläche: offene Risikoexposition.",
  "p.balance.axis_cost": "Geplante Kosten ({currency})",
  "p.balance.axis_value": "Angestrebter Nutzwert ({currency})",
  "p.balance.imbalances": "Ungleichgewichte",
  "p.balance.no_imbalance":
    "Keine der folgenden Regeln meldet in diesen Zahlen ein Ungleichgewicht.",
  "p.balance.rules":
    "Eine Markierung wird gesetzt, wenn ein Portfolio {concentration} % oder mehr der geplanten Kosten oder des offenen Risikos hält (bei mindestens zwei Portfolios), wenn sein Anteil an geplanten Kosten oder Risiko um {gap} Prozentpunkte oder mehr über seinem Anteil am angestrebten Wert liegt oder wenn es geplante Kosten, aber keinen angestrebten Wert in der gewählten Währung hat. Eine Markierung ist ein Anlass zum Hinsehen, kein Urteil.",
  "p.balance.flag_cost_concentration":
    "{name} hält {percent} % der geplanten Kosten.",
  "p.balance.flag_risk_concentration":
    "{name} hält {percent} % der offenen Risikoexposition.",
  "p.balance.flag_cost_ahead_of_value":
    "{name} hat einen Anteil an den geplanten Kosten, der deutlich über seinem Anteil von {percent} % am angestrebten Wert liegt.",
  "p.balance.flag_risk_ahead_of_value":
    "{name} hat einen Anteil am offenen Risiko, der deutlich über seinem Anteil von {percent} % am angestrebten Wert liegt.",
  "p.balance.flag_no_value":
    "{name} hat geplante Kosten, aber keinen angestrebten Wert in {currency} erfasst.",
  "p.balance.table": "Dieselben Zahlen als Tabelle",
  "p.balance.portfolio": "Portfolio",
  "p.balance.cost": "Geplante Kosten ({currency})",
  "p.balance.value": "Angestrebter Wert ({currency})",
  "p.balance.risk": "Offene Risikoexposition",
  "p.balance.cost_share": "Kostenanteil",
  "p.balance.value_share": "Wertanteil",
  "p.balance.risk_share": "Risikoanteil",
  "p.balance.flags": "Markierungen",
  "p.balance.no_flag": "keine",
  "p.balance.notes":
    "Gruppiert nach Portfolio, nicht nach Kategorie oder strategischem Ziel: Das Portfolio ist die einzige Gruppierung, nach der Kosten, Nutzwert und Risiko alle berichtet werden. Kosten sind der geplante Betrag auf Budgetpositionen, Wert ist das Ziel der erfassten Nutzen, und Risiko ist der Wert der offenen Exposition; Geld in verschiedenen Währungen wird nie addiert.",
  "p.roadmap.title":
    "Roadmap — Wirkung, Aufwand und Jetzt / Als Nächstes / Später",
  "p.roadmap.h1": "Roadmap: Wirkung, Aufwand und Jetzt / Als Nächstes / Später",
  "p.roadmap.lede":
    "Offene Ideen und noch nicht entschiedene Vorschläge, eingeordnet danach, wie wichtig sie sind (Wirkung) und wie viel Arbeit sie erfordern (Aufwand). Der Dienst speichert für Ideen und Vorschläge weder Wirkung noch Aufwand, daher bewerten Sie jeden Eintrag hier; Ihre Bewertungen werden nur in diesem Browser gespeichert und weder geteilt noch an den Dienst gesendet. Ein Eintrag ohne beide Bewertungen wird nicht eingeordnet.",
  "p.roadmap.loading": "Ideen und Vorschläge werden geladen…",
  "p.roadmap.matrix_h2": "Wirkungs-Aufwands-Matrix",
  "p.roadmap.matrix_caption":
    "Wirkung in Zeilen, Aufwand in Spalten. Jede Zelle listet die so bewerteten Einträge.",
  "p.roadmap.impact": "Wirkung",
  "p.roadmap.effort": "Aufwand",
  "p.roadmap.impact_level": "Wirkung: {level}",
  "p.roadmap.effort_level": "Aufwand: {level}",
  "p.roadmap.level_low": "Niedrig",
  "p.roadmap.level_medium": "Mittel",
  "p.roadmap.level_high": "Hoch",
  "p.roadmap.rate_h2": "Einträge bewerten",
  "p.roadmap.rate_hint":
    "Diese Tabelle ist der Tastaturweg zur Matrix: Wählen Sie für jeden Eintrag Wirkung und Aufwand, und er wandert in die Matrix und in die Listen Jetzt / Als Nächstes / Später.",
  "p.roadmap.col_item": "Eintrag",
  "p.roadmap.col_type": "Typ",
  "p.roadmap.col_horizon": "Wann",
  "p.roadmap.kind_idea": "Idee",
  "p.roadmap.kind_proposal": "Vorschlag",
  "p.roadmap.not_rated": "Nicht bewertet",
  "p.roadmap.rate_impact": "Wirkung von {title}",
  "p.roadmap.rate_effort": "Aufwand von {title}",
  "p.roadmap.no_items":
    "Es gibt keine offenen Ideen oder Vorschläge zu bewerten.",
  "p.roadmap.horizons_h2": "Jetzt / Als Nächstes / Später",
  "p.roadmap.horizon_rule":
    "Jetzt: Wirkung liegt über dem Aufwand. Als Nächstes: Wirkung gleich Aufwand und mindestens mittel. Später: alles andere Bewertete. Noch nicht bewertet: keine Wirkung oder kein Aufwand gewählt.",
  "p.roadmap.h_now": "Jetzt",
  "p.roadmap.h_next": "Als Nächstes",
  "p.roadmap.h_later": "Später",
  "p.roadmap.h_unrated": "Noch nicht bewertet",
  "p.roadmap.nothing_here": "Nichts vorhanden.",
  "p.roadmap.see_ideas": "Ideen und Vorschläge auf der Roadmap ansehen",
  "p.people_impact.title": "Auswirkungen auf Personen (aus dem Personaldienst)",
  "p.people_impact.intro":
    "Nur Anzahlen, aus dem Personaldienst (WPM), abgefragt mit Ihrer eigenen Anmeldung. Keine Person wird genannt, und nichts wird aufsummiert.",
  "p.people_impact.ask": "Personaldienst abfragen",
  "p.people_impact.asking": "Wird abgefragt…",
  "p.people_impact.failed":
    "Die Auswirkungen auf Personen konnten nicht geladen werden.",
  "p.people_impact.not_known": "Nicht bekannt.",
  "p.people_impact.source": "Aus dem Personaldienst, Stand {date}.",
  "p.people_impact.unknown": "Nicht bekannt: {reason}",
  "p.people_impact.end_sent":
    "Gesendetes Enddatum: {date} (Zieldatum dieses Plans).",
  "p.people_impact.committed": "Zugesagte Personen nach Pool",
  "p.people_impact.ending":
    "Befristete und Vertragsverhältnisse, die vor dem Ende auslaufen",
  "p.people_impact.in_no_pool": "In keinem Pool",
  "p.people_impact.end_not_recorded":
    "Kein Ende erfasst (nicht als auslaufend gezählt)",
  "p.people_impact.hidden": "verborgen (weniger als {floor})",
  "p.people_impact.no_pools": "Kein Pool anzuzeigen.",
  "p.people_impact.part_unknown": "Nicht bekannt: {reason}",
  "p.people_impact.over": "Überbuchte Pool-Monate",
  "p.people_impact.over_counts":
    "{over} von {considered} Pool-Monaten sind überbucht; für {unknown} ist keine Aussage möglich.",
  "p.people_impact.no_total":
    "Es wird keine Summe angegeben: Eine Person kann in mehr als einem Pool zählen.",
  "l.enum.pk_heat.supply_undeclared": "Verfügbarkeit nicht angegeben",
  "l.enum.pk_heat.supply_unknown": "Angebot unbekannt",
  "p.capacity_heatmap.nav": "Kapazitäts-Heatmap",
  "p.capacity_heatmap.title": "Kapazitäts-Heatmap — PPM",
  "p.capacity_heatmap.h1": "Kapazität nach Kompetenzpool und Monat",
  "p.capacity_heatmap.lede":
    "Jedes Feld vergleicht die von einem Kompetenzpool in einem Monat verlangte Arbeit mit den dafür verfügbaren Personen, in Vollzeitäquivalenten (FTE).",
  "p.capacity_heatmap.bridge_note":
    "Ein mit dem Personaldienst verknüpfter Pool bezieht sein Angebot von dort. Was hier für diesen Pool eingegeben wurde, wird daneben angezeigt und nie hinzugerechnet. Ein Monat ohne Wert aus dem Personaldienst wird als unbekannt angezeigt, nie als null.",
  "p.capacity_heatmap.from": "Erster Monat",
  "p.capacity_heatmap.months": "Anzahl der Monate",
  "p.capacity_heatmap.show": "Anzeigen",
  "p.capacity_heatmap.loading": "Heatmap wird geladen…",
  "p.capacity_heatmap.could_not_load":
    "Die Kapazitäts-Heatmap konnte nicht geladen werden.",
  "p.capacity_heatmap.none": "Kein Kompetenzpool und kein Bedarf anzuzeigen.",
  "p.capacity_heatmap.caption":
    "Bedarf und Angebot in Vollzeitäquivalenten, nach Kompetenzpool und Monat",
  "p.capacity_heatmap.col_pool": "Kompetenzpool",
  "p.capacity_heatmap.linked_to": "Mit dem Personaldienst verknüpft",
  "p.capacity_heatmap.demand": "Bedarf: {fte}",
  "p.capacity_heatmap.supply": "Angebot: {fte}",
  "p.capacity_heatmap.gap": "Lücke: {fte}",
  "p.capacity_heatmap.unknown": "unbekannt",
  "p.capacity_heatmap.not_declared": "nicht angegeben",
  "p.capacity_heatmap.none_recorded": "nichts erfasst",
  "p.capacity_heatmap.source_wpm": "Angebot aus dem Personaldienst",
  "p.capacity_heatmap.source_here": "Hier eingegebenes Angebot",
  "p.capacity_heatmap.peer_live": "Aktuell, abgerufen {when}.",
  "p.capacity_heatmap.peer_live_no_time": "Aktuell.",
  "p.capacity_heatmap.peer_last":
    "Zuletzt empfangen {age} (abgerufen {when}); derzeit nicht aktuell: {reason}",
  "p.capacity_heatmap.peer_hidden":
    "Verborgen: Der Personaldienst verbirgt einen Monat mit weniger Beschäftigten als seiner Untergrenze.",
  "p.capacity_heatmap.peer_unknown": "Unbekannt: {reason}",
  "p.capacity_heatmap.no_reason": "kein Wert vom Personaldienst empfangen",
  "p.capacity_heatmap.entered_here":
    "Hier eingegeben: {fte} (daneben angezeigt, nicht hinzugerechnet)",
  "p.capacity_heatmap.entered_here_none": "Hier eingegeben: nichts",
  "p.capacity_heatmap.partners_short":
    "Partner haben weniger geliefert als zugesagt",
  "p.capacity_heatmap.no_pool": "Kein Pool zugewiesen",
  "p.capacity_heatmap.key":
    "▲ Unterdeckung · ◆ knapp · ● ausreichend · ○ kein Bedarf · – kein Angebot erfasst · ? Angebot unbekannt. Das Wort wird immer zusätzlich angezeigt.",
  "p.capacity_heatmap.every_figure": "Alle Werte als Tabelle",
  "p.capacity_heatmap.table_label": "Kapazität nach Kompetenzpool und Monat",
  "p.capacity_heatmap.c_pool": "Kompetenzpool",
  "p.capacity_heatmap.c_month": "Monat",
  "p.capacity_heatmap.c_reading": "Bewertung",
  "p.capacity_heatmap.c_demand": "Bedarf (Vollzeitäquivalent)",
  "p.capacity_heatmap.c_supply": "Angebot (Vollzeitäquivalent)",
  "p.capacity_heatmap.c_source": "Angebot aus",
  "p.capacity_heatmap.c_status": "Wert des Personaldienstes",
  "p.capacity_heatmap.c_entered_here": "Hier eingegeben",
  "p.capacity_heatmap.see_pools":
    "Kompetenzpools mit dem Personaldienst verknüpfen",
  "p.skill_pools.nav": "Kompetenzpools",
  "p.skill_pools.title": "Kompetenzpools — PPM",
  "p.skill_pools.h1": "Kompetenzpools",
  "p.skill_pools.lede":
    "Ein Kompetenzpool bündelt die Rollen und Fähigkeiten, die dieselbe Arbeit erledigen können. Hier können Sie einen Pool mit dem passenden Pool im Personaldienst verknüpfen.",
  "p.skill_pools.rule":
    "Die Verknüpfung muss auf beiden Seiten gesetzt werden, hier und im Personaldienst, von einer Person, die beide sehen kann. Sie wird nie über den Namen abgeglichen. Solange ein Pool verknüpft ist, stammt sein Angebot in der Kapazitäts-Heatmap aus dem Personaldienst, und sein Bedarf wird dorthin gesendet.",
  "p.skill_pools.loading": "Kompetenzpools werden geladen…",
  "p.skill_pools.could_not_load":
    "Die Kompetenzpools konnten nicht geladen werden.",
  "p.skill_pools.none": "Es wurde noch kein Kompetenzpool angelegt.",
  "p.skill_pools.col_name": "Name",
  "p.skill_pools.col_roles": "Rollen",
  "p.skill_pools.col_skills": "Fähigkeiten",
  "p.skill_pools.col_state": "Zustand",
  "p.skill_pools.col_link": "Pool im Personaldienst",
  "p.skill_pools.col_act": "Ändern",
  "p.skill_pools.active": "aktiv",
  "p.skill_pools.retired": "stillgelegt",
  "p.skill_pools.not_linked": "nicht verknüpft",
  "p.skill_pools.change_short": "Verknüpfung ändern",
  "p.skill_pools.change": "Verknüpfung für {name} ändern",
  "p.skill_pools.edit_h2": "{name} mit dem Personaldienst verknüpfen",
  "p.skill_pools.ref_label": "Pool-ID im Personaldienst",
  "p.skill_pools.ref_hint":
    "Kopieren Sie sie vom Pool im Personaldienst: 36 Zeichen, Kleinbuchstaben, mit Bindestrichen, etwa 3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b.",
  "p.skill_pools.ref_invalid":
    "Das ist keine Pool-ID des Personaldienstes: Sie muss 36 Zeichen lang sein, in Kleinbuchstaben, mit Bindestrichen.",
  "p.skill_pools.save": "Verknüpfung speichern",
  "p.skill_pools.clear": "Verknüpfung entfernen",
  "p.skill_pools.cancel": "Abbrechen",
  "p.skill_pools.confirm_clear":
    "Verknüpfung zwischen {name} und dem Personaldienst entfernen? Sein Angebot in der Heatmap ist dann wieder der hier eingegebene Wert. Jeder für diesen Pool bereits an den Personaldienst gesendete Bedarfswert wird dort zurückgezogen (als null gesendet).",
  "p.skill_pools.saved":
    "{name} ist jetzt mit dem Pool {ref} im Personaldienst verknüpft.",
  "p.skill_pools.cleared":
    "{name} ist nicht mehr mit dem Personaldienst verknüpft.",
  "p.skill_pools.taken":
    "Ein anderer Pool hier ist bereits mit diesem Pool im Personaldienst verknüpft. Entfernen Sie zuerst diese Verknüpfung: ein Pool hier pro Pool dort.",
  "p.skill_pools.invalid":
    "Der Dienst hat diese ID nicht akzeptiert. Es muss die Pool-ID des Personaldienstes sein: 36 Zeichen, Kleinbuchstaben, mit Bindestrichen.",
  "p.skill_pools.missing":
    "Dieser Pool existiert nicht mehr. Die Liste wurde neu geladen.",
  "p.skill_pools.could_not_save":
    "Die Verknüpfung konnte nicht geändert werden.",
  "p.skill_pools.see_heatmap": "Kapazitäts-Heatmap ansehen",
  "p.skill_pools.withdrawing":
    "Bedarfswerte, die aus dem zuvor verknüpften Pool im Personaldienst zurückgezogen werden: {n}.",
  "p.skill_pools.add_heading": "Pool hinzufügen",
  "p.skill_pools.add_says":
    "Ein Pool fasst Ressourcenrollen und Fähigkeiten zusammen, damit die Kapazitäts-Heatmap ihren Bedarf und ihr Angebot einmal zählt. Eine Rolle oder Fähigkeit gehört zu höchstens einem Pool.",
  "p.skill_pools.add": "Pool hinzufügen",
  "p.skill_pools.added": "Pool {name} hinzugefügt.",
  "p.skill_pools.choose_pool": "Pool wählen",
  "p.skill_pools.clash_item": "{value} (im Pool {pool})",
  "p.skill_pools.conflict_members":
    "Ein Pool mit diesem Namen existiert, oder eine dieser Rollen oder Fähigkeiten gehört schon zu einem anderen Pool.",
  "p.skill_pools.could_not_add": "Der Pool wurde nicht hinzugefügt.",
  "p.skill_pools.f_name": "Name des Pools",
  "p.skill_pools.f_pool": "Gewählter Pool",
  "p.skill_pools.f_roles": "Rollen",
  "p.skill_pools.f_skills": "Fähigkeiten",
  "p.skill_pools.list_says":
    "Trennen Sie Rollen oder Fähigkeiten mit Kommas oder Zeilenumbrüchen; eine Wiederholung zählt einmal.",
  "p.skill_pools.members_heading": "Pool ändern",
  "p.skill_pools.members_saved":
    "Rollen und Fähigkeiten von {name} sind gespeichert.",
  "p.skill_pools.problem_clash": "Schon in einem anderen Pool: {list}.",
  "p.skill_pools.problem_name_taken":
    "Ein Pool mit diesem Namen existiert schon.",
  "p.skill_pools.problem_name": "Der Name eines Pools hat 2 bis 120 Zeichen.",
  "p.skill_pools.restore": "Pool wiederherstellen",
  "p.skill_pools.restored": "Pool {name} wiederhergestellt.",
  "p.skill_pools.retire_says":
    "Ein stillgelegter Pool bleibt erfasst und behält seine Rollen und Fähigkeiten; wiederhergestellt zählt er wieder.",
  "p.skill_pools.retire": "Pool stilllegen",
  "p.skill_pools.retired_done": "Pool {name} stillgelegt.",
  "p.skill_pools.save_members": "Rollen und Fähigkeiten speichern",
  "p.skill_pools.unchanged":
    "Nichts wurde geändert, daher wurde nichts gesendet.",
  "p.capacity_heatmap.set_commitment": "Kapazitätszusage festlegen",
  "p.commitments.title": "Kapazitätszusagen — PPM",
  "p.commitments.h1": "Kapazitätszusagen",
  "p.commitments.lede":
    "Erfassen Sie einem Programm zugesagte Kapazität (Angebot) oder Arbeit, die ein Programm von einem Kompetenzpool benötigt (Bedarf), für einen Monat, in Vollzeitäquivalenten.",
  "p.commitments.bridge_note":
    "Bei einem mit dem Personaldienst verknüpften Pool sendet eine Bedarfszusage auch den Gesamtbedarf des Programms für diesen Pool und Monat dorthin. Eine Summe von null zieht den Wert dort zurück. Nichts wird wiederholt: Speichern Sie erneut, um erneut zu senden.",
  "p.commitments.see_heatmap": "Kapazitäts-Heatmap ansehen",
  "p.commitments.from_plan": "Kapazitätszusagen für dieses Programm",
  "p.commitments.form_h2": "Zusage festlegen",
  "p.commitments.choose": "Auswählen…",
  "p.commitments.pool": "Kompetenzpool",
  "p.commitments.pool_linked": "{name} (mit dem Personaldienst verknüpft)",
  "p.commitments.pool_hint": "Nur aktive Pools werden aufgeführt.",
  "p.commitments.pool_missing": "Wählen Sie einen Kompetenzpool.",
  "p.commitments.plan": "Programm",
  "p.commitments.plan_hint":
    "Der Plan muss ein Programm sein (ein Plan der Art Program).",
  "p.commitments.plan_missing": "Wählen Sie ein Programm.",
  "p.commitments.kind": "Was zugesagt wird",
  "p.commitments.kind_demand":
    "Bedarf: Arbeit, die das Programm vom Pool benötigt",
  "p.commitments.kind_supply": "Angebot: dem Programm zugesagte Kapazität",
  "p.commitments.organization": "Partnerorganisation (optional)",
  "p.commitments.organization_hint":
    "Für Ihre eigene Organisation leer lassen.",
  "p.commitments.month": "Monat",
  "p.commitments.month_hint": "Ein Monat: Jahr und Monat.",
  "p.commitments.month_invalid": "Geben Sie einen Monat als Jahr und Monat an.",
  "p.commitments.committed": "Zugesagt (Vollzeitäquivalent)",
  "p.commitments.committed_hint":
    "Eine Zahl von 0 bis 1000, so geschrieben, wie Sie Zahlen schreiben. Null zieht einen an den Personaldienst gesendeten Bedarfswert zurück.",
  "p.commitments.delivered": "Geliefert (Vollzeitäquivalent, optional)",
  "p.commitments.delivered_hint":
    "Was tatsächlich geliefert wurde, falls bekannt.",
  "p.commitments.fte_invalid":
    "Eine Zahl von 0 bis 1000, so geschrieben, wie Sie Zahlen schreiben.",
  "p.commitments.note": "Notiz (optional)",
  "p.commitments.save": "Zusage speichern",
  "p.commitments.saved": "Gespeichert: {pool}, {plan}, {month}.",
  "p.commitments.not_found":
    "Dieses Programm wurde nicht gefunden, oder Sie dürfen es nicht lesen.",
  "p.commitments.refused":
    "Der Dienst hat sie nicht akzeptiert. Der Plan muss ein Programm sein, der Pool aktiv und jeder Wert zwischen 0 und 1000.",
  "p.commitments.could_not_save": "Die Zusage konnte nicht gespeichert werden.",
  "p.commitments.could_not_load":
    "Die Kapazitätszusagen konnten nicht geladen werden.",
  "p.commitments.loading": "Letzte Sendungen werden geladen…",
  "p.commitments.sends_h2": "Letzte Sendungen an den Personaldienst",
  "p.commitments.sends_note":
    "Die jeweils letzte Sendung pro Programm, Pool und Monat, neueste zuerst. Bei einer begonnenen Sendung erscheint die Antwort hier, sobald sie eintrifft: Aktualisieren Sie, um sie zu sehen.",
  "p.commitments.refresh": "Aktualisieren",
  "p.commitments.no_sends":
    "Es wurde noch nichts an den Personaldienst gesendet.",
  "p.commitments.c_pool": "Kompetenzpool",
  "p.commitments.c_programme": "Programm",
  "p.commitments.c_month": "Monat",
  "p.commitments.c_sent": "Gesendet",
  "p.commitments.c_outcome": "Ergebnis",
  "p.commitments.c_at": "Wann",
  "p.commitments.sent_fte": "{fte} Vollzeitäquivalent",
  "p.commitments.sent_withdrawal": "ein Rückzug (null)",
  "p.commitments.sent_nothing": "nichts",
  "p.commitments.out_not_sent_at_all":
    "Diese Zusage wird nicht an den Personaldienst gesendet: Nur Bedarf für einen damit verknüpften Pool wird gesendet.",
  "p.commitments.out_none_yet":
    "Für dieses Programm, diesen Pool und Monat wurde noch nichts an den Personaldienst gesendet.",
  "p.commitments.out_started":
    "{fte} Vollzeitäquivalent (die Summe des Programms für diesen Pool und Monat) wird an den Personaldienst gesendet. Die Antwort erscheint unter „Letzte Sendungen“.",
  "p.commitments.out_started_zero":
    "Der Wert dieses Programms für diesen Pool und Monat wird aus dem Personaldienst zurückgezogen, da die Summe jetzt null ist. Die Antwort erscheint unter „Letzte Sendungen“.",
  "p.commitments.out_sent":
    "Der Personaldienst hat {fte} Vollzeitäquivalent übernommen.",
  "p.commitments.out_sent_zero":
    "Der Personaldienst hat seinen Wert für dieses Programm, diesen Pool und Monat zurückgezogen.",
  "p.commitments.out_duplicate":
    "Der Personaldienst hatte diesen Wert bereits; dort hat sich nichts geändert.",
  "p.commitments.out_refused":
    "Der Personaldienst hat ihn abgelehnt. Nichts wird wiederholt; speichern Sie erneut, um erneut zu senden.",
  "p.commitments.out_unreachable":
    "Der Personaldienst war nicht erreichbar oder hat nicht rechtzeitig geantwortet. Nichts wird wiederholt; speichern Sie erneut, um erneut zu senden.",
  "p.commitments.out_not_configured":
    "Nicht gesendet: Dieser Dienst ist nicht für den Zugriff auf den Personaldienst eingerichtet.",
  "p.commitments.out_no_token":
    "Nicht gesendet: Es gab keine Anmeldung, die an den Personaldienst weitergegeben werden konnte.",
  "p.commitments.out_too_large":
    "Nicht gesendet: Der Personaldienst akzeptiert höchstens 10 000 Vollzeitäquivalente.",
  "p.commitments.out_old_zero":
    "Nicht gesendet: eine Null, die erfasst wurde, bevor Nullwerte gesendet wurden, daher hat der Personaldienst seinen letzten Wert behalten. Speichern Sie erneut, um ihn zurückzuziehen.",
  "p.commitments.out_other": "Der Dienst hat erfasst: {outcome}.",
  "p.discussion.heading": "Diskussion",
  "p.discussion.loading": "Wird geladen…",
  "p.discussion.could_not_load_comments":
    "Die Kommentare konnten nicht geladen werden.",
  "p.discussion.could_not_load_reactions":
    "Die Reaktionen konnten nicht geladen werden.",
  "p.discussion.could_not_load_files":
    "Die Dateien konnten nicht geladen werden.",
  "p.discussion.could_not_load_watch":
    "Ob Sie dem folgen, konnte nicht gelesen werden.",
  "p.discussion.sign_in_to_follow":
    "Melden Sie sich an, um dem zu folgen und über neue Kommentare und Änderungen informiert zu werden.",
  "p.discussion.you_follow":
    "Sie folgen dem: Sie werden über neue Kommentare und Änderungen informiert.",
  "p.discussion.you_do_not_follow": "Sie folgen dem nicht.",
  "p.discussion.followers_one": "{n} Person folgt dem.",
  "p.discussion.followers_other": "{n} Personen folgen dem.",
  "p.discussion.follow": "Folgen",
  "p.discussion.unfollow": "Nicht mehr folgen",
  "p.discussion.followed": "Sie folgen dem jetzt.",
  "p.discussion.unfollowed": "Sie folgen dem nicht mehr.",
  "p.discussion.reactions_label": "Reaktionen",
  "p.discussion.reaction_named_one": "{reaction}: {n} Person",
  "p.discussion.reaction_named_other": "{reaction}: {n} Personen",
  "p.discussion.r_thumbs_up": "Daumen hoch",
  "p.discussion.r_heart": "Herz",
  "p.discussion.r_celebrate": "Feiern",
  "p.discussion.r_eyes": "Schaue es mir an",
  "p.discussion.r_question": "Frage",
  "p.discussion.comments_heading": "Kommentare",
  "p.discussion.no_comments": "Noch keine Kommentare.",
  "p.discussion.no_author": "jemand ohne Anmeldung",
  "p.discussion.by_at": "Von {who}, {when}",
  "p.discussion.edited_at": "bearbeitet {when}",
  "p.discussion.edit": "Bearbeiten",
  "p.discussion.edit_named": "Kommentar von {who}, {when} bearbeiten",
  "p.discussion.delete": "Löschen",
  "p.discussion.delete_named": "Kommentar von {who}, {when} löschen",
  "p.discussion.confirm_delete_comment":
    "Diesen Kommentar löschen? Frühere Fassungen bleiben erhalten, aber niemand sieht ihn mehr auf der Seite.",
  "p.discussion.earlier": "Frühere Fassungen",
  "p.discussion.no_earlier": "Es ist keine frühere Fassung gespeichert.",
  "p.discussion.replaced_at": "Ersetzt {when}",
  "p.discussion.f_edit": "Kommentar ändern",
  "p.discussion.save_edit": "Kommentar speichern",
  "p.discussion.cancel": "Abbrechen",
  "p.discussion.f_comment": "Ihr Kommentar",
  "p.discussion.chars_left": "Noch {n} von {max} Zeichen.",
  "p.discussion.chars_over": "{n} Zeichen über der Grenze von {max}.",
  "p.discussion.post": "Kommentar veröffentlichen",
  "p.discussion.posted": "Der Kommentar wurde veröffentlicht.",
  "p.discussion.edited":
    "Der Kommentar wurde geändert; die frühere Fassung bleibt erhalten.",
  "p.discussion.deleted": "Der Kommentar wurde gelöscht.",
  "p.discussion.problem_blank": "Schreiben Sie zuerst den Kommentar.",
  "p.discussion.problem_too_long":
    "Ein Kommentar darf höchstens 4000 Zeichen lang sein.",
  "p.discussion.files_heading": "Dateien",
  "p.discussion.files_intro":
    "Hier angehängte Dateien werden beim Öffnen auf Ihrem Computer gespeichert; sie werden nie innerhalb dieser Seite angezeigt.",
  "p.discussion.no_files": "Keine Dateien angehängt.",
  "p.discussion.file_facts": "{type}, {size}, hinzugefügt von {who}, {when}",
  "p.discussion.remove_file": "Entfernen",
  "p.discussion.remove_file_named": "Datei {name} entfernen",
  "p.discussion.confirm_remove_file":
    "Die Datei {name} entfernen? Niemand kann sie danach hier herunterladen.",
  "p.discussion.removed_file": "Die Datei {name} wurde entfernt.",
  "p.discussion.f_file": "Anzuhängende Datei",
  "p.discussion.files_rules":
    "Ein PDF, ein Bild (PNG, JPEG, GIF oder WebP), reiner Text, CSV, Markdown, JSON oder eine Word-, Excel- oder PowerPoint-Datei, höchstens 10 MB. Webseiten, SVG-Zeichnungen, Archive und Programme werden abgelehnt.",
  "p.discussion.attach": "Datei anhängen",
  "p.discussion.attached": "Die Datei {name} wurde angehängt.",
  "p.discussion.problem_choose_file": "Wählen Sie zuerst eine Datei.",
  "p.discussion.problem_empty": "Diese Datei ist leer.",
  "p.discussion.problem_too_large":
    "Diese Datei ist größer als 10 MB, die Grenze für eine Datei.",
  "p.discussion.problem_type":
    "Diese Art von Datei kann nicht angehängt werden. Die Liste unten nennt die erlaubten Arten.",
  "p.discussion.type_pdf": "PDF-Dokument",
  "p.discussion.type_png": "PNG-Bild",
  "p.discussion.type_jpeg": "JPEG-Bild",
  "p.discussion.type_gif": "GIF-Bild",
  "p.discussion.type_webp": "WebP-Bild",
  "p.discussion.type_text": "reiner Text",
  "p.discussion.type_csv": "CSV-Tabelle",
  "p.discussion.type_markdown": "Markdown-Text",
  "p.discussion.type_json": "JSON-Daten",
  "p.discussion.type_word": "Word-Dokument",
  "p.discussion.type_excel": "Excel-Arbeitsmappe",
  "p.discussion.type_powerpoint": "PowerPoint-Folien",
  "p.discussion.size_b": "{n} Byte",
  "p.discussion.size_kb": "{n} KB",
  "p.discussion.size_mb": "{n} MB",
  "p.discussion.refused_sign_in":
    "Melden Sie sich zuerst an: Der Dienst weiß nicht, wer Sie sind.",
  "p.discussion.refused_not_author":
    "Nur wer einen Kommentar geschrieben hat, darf ihn ändern oder löschen.",
  "p.discussion.refused_not_uploader":
    "Nur wer eine Datei angehängt hat, oder ein Administrator, darf sie entfernen.",
  "p.discussion.refused_forbidden": "Sie dürfen dies weder lesen noch ändern.",
  "p.discussion.refused_comment_gone":
    "Dieser Kommentar ist nicht mehr da; die Kommentare wurden neu gelesen.",
  "p.discussion.refused_file_gone":
    "Diese Datei war bereits entfernt; die Dateien wurden neu gelesen.",
  "p.discussion.refused_gone":
    "Dies wurde nicht gefunden: Es wurde vielleicht entfernt, oder Sie dürfen es nicht sehen.",
  "p.discussion.refused_comment_invalid":
    "Der Dienst hat den Kommentar nicht angenommen: Er darf nicht leer sein und höchstens 4000 Zeichen haben.",
  "p.discussion.refused_file_invalid":
    "Der Dienst hat die Datei nicht angenommen: Art, Größe oder Inhalt haben die Prüfung nicht bestanden (eine Datei muss sein, was ihre Art angibt, und Programme werden nicht angenommen).",
  "p.discussion.refused_too_large": "Die Datei ist zu groß zum Senden.",
  "p.discussion.refused_scan_unavailable":
    "Die Datei wurde nicht angehängt: Der Schadsoftware-Scanner hat nicht geantwortet. Versuchen Sie es später erneut.",
  "p.me.timesheet_link": "Mein Stundenzettel",
  "p.me.timesheet_note":
    "— Ihre Zeit nach Tag und Plan, Woche für Woche, und eine Woche zur Genehmigung senden.",
  "p.timesheet.title_ppm": "Mein Stundenzettel — PPM",
  "p.timesheet.heading": "Mein Stundenzettel",
  "p.timesheet.back": "← Meine Arbeit",
  "p.timesheet.intro":
    "Ihre erfasste Zeit für eine Woche, nach Tag und Plan, in Stunden und Minuten, abrechenbare und nicht abrechenbare Zeit immer getrennt. Senden Sie eine fertige Woche zur Genehmigung; sobald sie genehmigt ist, ist sie endgültig, und es kann keine weitere Zeit darin erfasst werden.",
  "p.timesheet.loading": "Wird geladen…",
  "p.timesheet.week_nav": "Woche wählen",
  "p.timesheet.previous_week": "Vorige Woche",
  "p.timesheet.this_week": "Diese Woche",
  "p.timesheet.next_week": "Nächste Woche",
  "p.timesheet.f_week": "Woche, etwa 2026-W46",
  "p.timesheet.show_week": "Woche anzeigen",
  "p.timesheet.week_hint":
    "Wochen laufen von Montag bis Sonntag und sind nach der internationalen Norm nummeriert, gleich welcher Tag Ihr erster Wochentag ist. Heute ist der {today} in Ihrer Zeitzone, {tz}.",
  "p.timesheet.problem_week":
    "Das ist keine Woche: Schreiben Sie das Jahr, ‘-W’ und die Nummer der Woche, etwa 2026-W46.",
  "p.timesheet.mine_heading": "Ihre Woche {week}",
  "p.timesheet.mine_caption": "Ihre Zeit in Woche {week}, nach Plan und Tag",
  "p.timesheet.status_open": "Offen: noch nicht zur Genehmigung gesendet",
  "p.timesheet.status_submitted":
    "Zur Genehmigung gesendet: wartet auf eine Entscheidung",
  "p.timesheet.status_approved":
    "Genehmigt: endgültig und für neue Zeit gesperrt",
  "p.timesheet.status_rejected": "Zurückgeschickt: ändern und erneut senden",
  "p.timesheet.rejected_because": "die genehmigende Person schrieb: {comment}",
  "p.timesheet.decision_comment": "mit dem Vermerk: {comment}",
  "p.timesheet.no_entries":
    "In dieser Woche ist keine Zeit erfasst, daher gibt es keine Summe.",
  "p.timesheet.col_plan": "Name des Plans",
  "p.timesheet.col_total": "Summe",
  "p.timesheet.col_billable": "Abrechenbar",
  "p.timesheet.col_non_billable": "Nicht abrechenbar",
  "p.timesheet.row_day_totals": "Je Tag",
  "p.timesheet.plan_unnamed": "Plan {id}… (Name nicht angezeigt)",
  "p.timesheet.split_line":
    "Abrechenbar {billable}, nicht abrechenbar {non_billable}: insgesamt {total}.",
  "p.timesheet.omitted":
    "Ausgelassene Einträge, weil sie in Plänen stehen, die Sie nicht lesen dürfen: {n}.",
  "p.timesheet.hours_one": "{n} Std.",
  "p.timesheet.hours_other": "{n} Std.",
  "p.timesheet.minutes_one": "{n} Min.",
  "p.timesheet.minutes_other": "{n} Min.",
  "p.timesheet.duration_hm": "{h} {m}",
  "p.timesheet.submit": "Diese Woche zur Genehmigung senden",
  "p.timesheet.submit_again": "Diese Woche erneut zur Genehmigung senden",
  "p.timesheet.submitted_notice":
    "Woche {week} wurde zur Genehmigung gesendet.",
  "p.timesheet.nothing_to_submit":
    "Eine Woche ohne erfasste Zeit kann nicht zur Genehmigung gesendet werden.",
  "p.timesheet.waiting":
    "Diese Woche wartet auf die Entscheidung einer genehmigenden Person.",
  "p.timesheet.locked":
    "Eine genehmigte Woche ist endgültig: Es kann keine weitere Zeit darin erfasst werden.",
  "p.timesheet.could_not_load": "Ihre Woche konnte nicht geladen werden.",
  "p.timesheet.could_not_submit":
    "Die Woche konnte nicht zur Genehmigung gesendet werden.",
  "p.timesheet.other_heading":
    "Die Woche einer anderen Person, für Genehmigende",
  "p.timesheet.other_intro":
    "Wenn Sie Stundenzettel genehmigen dürfen, geben Sie die Referenz der Person an, um ihre Woche für die oben gewählte Woche zu lesen, und genehmigen Sie sie oder schicken Sie sie zurück. Niemand entscheidet über die eigene Woche, das Zurückschicken braucht einen Grund, und eine Genehmigung ist endgültig.",
  "p.timesheet.f_person": "Person: ‘person:’ oder ‘worker:’ und ihre Kennung",
  "p.timesheet.show_person": "Ihre Woche anzeigen",
  "p.timesheet.problem_person":
    "Geben Sie die Person als ‘person:’ oder ‘worker:’ gefolgt von ihrer Kennung an, ohne weiteren Text.",
  "p.timesheet.other_week_heading": "Woche {week} von {person}",
  "p.timesheet.other_caption":
    "Zeit von {person} in Woche {week}, nach Plan und Tag",
  "p.timesheet.decision_legend": "Ihre Entscheidung",
  "p.timesheet.approve": "Genehmigen",
  "p.timesheet.reject": "Zurückschicken",
  "p.timesheet.f_comment": "Vermerk, falls gewünscht",
  "p.timesheet.f_comment_required":
    "Warum sie zurückgeschickt wird (erforderlich)",
  "p.timesheet.approve_final":
    "Eine Genehmigung ist endgültig: Die Woche wird dann gesperrt, und niemand kann sie zurückschicken.",
  "p.timesheet.record_decision": "Entscheidung festhalten",
  "p.timesheet.approved_notice": "Woche {week} von {person} ist genehmigt.",
  "p.timesheet.rejected_notice":
    "Woche {week} von {person} wurde zurückgeschickt.",
  "p.timesheet.other_not_submitted":
    "Diese Woche wurde nicht zur Genehmigung gesendet, daher gibt es noch nichts zu entscheiden.",
  "p.timesheet.other_rejected":
    "Diese Woche wurde zurückgeschickt; sie kann erneut entschieden werden, sobald die Person sie wieder sendet.",
  "p.timesheet.problem_own_week":
    "Das ist Ihre eigene Woche: Niemand entscheidet über den eigenen Stundenzettel.",
  "p.timesheet.problem_comment":
    "Sagen Sie, warum die Woche zurückgeschickt wird.",
  "p.timesheet.could_not_load_other":
    "Die Woche dieser Person konnte nicht geladen werden.",
  "p.timesheet.could_not_decide":
    "Die Entscheidung konnte nicht festgehalten werden.",
  "p.timesheet.refused_read_forbidden":
    "Nur die Person selbst oder eine Person mit dem Recht, Stundenzettel zu genehmigen, darf diese Woche lesen.",
  "p.timesheet.refused_submit_forbidden":
    "Nur die Person, der eine Woche gehört, darf sie zur Genehmigung senden.",
  "p.timesheet.refused_decide_forbidden":
    "Sie dürfen über diese Woche nicht entscheiden: Dafür braucht es das Recht, Stundenzettel zu genehmigen, und niemand entscheidet über die eigene Woche.",
  "p.timesheet.refused_submit_conflict":
    "Diese Woche wurde bereits zur Genehmigung gesendet oder genehmigt; sie wurde neu gelesen.",
  "p.timesheet.refused_decide_conflict":
    "Diese Woche wartet nicht mehr auf eine Entscheidung; sie wurde neu gelesen.",
  "p.timesheet.refused_not_addressable":
    "Ihre Anmeldeidentität hat keine Personenreferenz, daher gibt es dafür keinen Stundenzettel.",
  "p.timesheet.refused_read_invalid":
    "Der Dienst hat diese Woche oder Person nicht angenommen.",
  "p.timesheet.refused_submit_invalid":
    "Die Woche wurde nicht gesendet: Es ist keine Zeit darin erfasst.",
  "p.timesheet.refused_decide_invalid":
    "Der Dienst hat die Entscheidung nicht angenommen: Das Zurückschicken einer Woche braucht einen Grund.",
  "p.security.access_link": "Zugriffsprüfung und Notfallzugriff",
  "p.security.access_note":
    "— wer welchen Zugriff hat, zur Rezertifizierung, und zeitlich begrenzter Notfallzugriff auf einen privaten Plan, der aufgezeichnet und geprüft wird.",
  "p.access.title_ppm": "Zugriffsprüfung und Notfallzugriff — PPM",
  "p.access.back": "← Sicherheit",
  "p.access.heading": "Zugriffsprüfung und Notfallzugriff",
  "p.access.intro":
    "Die Zugriffsprüfung listet die Zugriffe auf, die als gewährt erfasst wurden, zur Rezertifizierung. Der Notfallzugriff öffnet Ihnen einen privaten Plan für begrenzte Zeit, wenn es keinen anderen Weg gibt; jede Nutzung wird aufgezeichnet und geprüft.",
  "p.access.review_heading": "Zugriffsprüfung",
  "p.access.review_intro":
    "Die im Zugriffsbuch als gewährt erfassten und seitdem nicht widerrufenen Zugriffe, die ältesten zuerst, mit wer sie wann gewährt hat. Sie zeigt, was erfasst wurde, nicht was der Anmeldedienst durchsetzt: Vergleichen Sie beides. Nur für Administratoren; jedes Lesen wird im Prüfprotokoll aufgezeichnet, daher wird die Prüfung nur gelesen, wenn Sie es verlangen.",
  "p.access.f_review_plan":
    "Plan-Kennung (leer lassen für jeden Plan, den Sie lesen können)",
  "p.access.f_review_days": "Zur Prüfung fällig nach so vielen Kalendertagen",
  "p.access.show_review": "Prüfung anzeigen",
  "p.access.problem_review_plan":
    "Das ist keine Plan-Kennung: Fügen Sie die Kennung des Plans ein oder lassen Sie das Feld für alle Pläne leer.",
  "p.access.problem_review_days":
    "Geben Sie die Zahl der Kalendertage als ganze Zahl ab 0 an.",
  "p.access.could_not_load_review":
    "Die Zugriffsprüfung konnte nicht geladen werden.",
  "p.access.review_totals":
    "Stand {as_of}. Bestehende Zugriffe: {standing}. Zur Prüfung fällig, ab {days} Kalendertagen alt: {due}.",
  "p.access.review_note":
    "Das Alter wird in Kalendertagen vom Tag der Erfassung bis zum heutigen Tag der Installation gezählt.",
  "p.access.download_review":
    "Prüfung als Tabelle herunterladen, mit einer Spalte für Ihre Entscheidungen „behalten“ oder „widerrufen“",
  "p.access.review_caption": "Heute bestehende Zugriffe, die ältesten zuerst",
  "p.access.review_empty":
    "In den Plänen, die Sie lesen können, besteht kein als gewährt erfasster Zugriff.",
  "p.access.col_person": "Person",
  "p.access.col_access": "Zugriff",
  "p.access.col_plan": "Planname",
  "p.access.col_granted_on": "Gewährt am",
  "p.access.col_granted_by": "Gewährt von",
  "p.access.col_age": "Alter in Kalendertagen",
  "p.access.col_review": "Prüfung",
  "p.access.col_note": "Notiz",
  "p.access.due": "Zur Prüfung fällig",
  "p.access.not_due": "Noch nicht fällig",
  "p.access.plan_unnamed": "Plan {id}… (Name nicht angezeigt)",
  "p.access.bg_heading": "Notfallzugriff auf einen privaten Plan (Break-Glass)",
  "p.access.bg_audited_heading":
    "Jede Nutzung des Notfallzugriffs wird aufgezeichnet und geprüft.",
  "p.access.bg_audited_open":
    "Das Öffnen eines Plans zeichnet im Prüfprotokoll auf, wer Sie sind, den Plan, Ihren Grund, wie lange und die von Ihnen genannte Person.",
  "p.access.bg_audited_told":
    "Die von Ihnen genannte Person wird sofort benachrichtigt. Kann sie nicht benachrichtigt werden, wird der Plan nicht geöffnet.",
  "p.access.bg_audited_each":
    "Jede Anfrage, die nur dieser Zugriff durchlässt, wird im Prüfprotokoll als Notfallzugriff aufgezeichnet.",
  "p.access.bg_audited_visibility":
    "Sie können den Plan damit sehen. Was Sie ändern dürfen, bestimmen weiterhin Ihre eigenen Rechte.",
  "p.access.bg_audited_ends":
    "Er endet von selbst zur angezeigten Zeit oder früher, wenn Sie ihn beenden. Höchstens fünf am Tag und jeweils einer pro Plan.",
  "p.access.bg_intro":
    "Nutzen Sie ihn nur, wenn die Mitglieder des Plans nicht erreichbar sind und die Arbeit nicht warten kann. Er erfordert die Break-Glass-Berechtigung oder Administratorzugriff; der Dienst entscheidet.",
  "p.access.problems_heading": "Der Plan wurde nicht geöffnet:",
  "p.access.problem_plan": "Geben Sie die Kennung des privaten Plans an.",
  "p.access.problem_reason_short":
    "Sagen Sie in mindestens {reason_min} Zeichen, warum Sie Zugriff brauchen.",
  "p.access.problem_reason_long":
    "Der Grund ist länger als {reason_max} Zeichen.",
  "p.access.problem_minutes":
    "Geben Sie die Dauer in ganzen Minuten an, von {minutes_min} bis {minutes_max}.",
  "p.access.problem_notify":
    "Nennen Sie die zu benachrichtigende Person als ‘person:’ oder ‘worker:’ gefolgt von ihrer Kennung, ohne weiteren Text.",
  "p.access.problem_acknowledged":
    "Kreuzen Sie das Feld an, um zu bestätigen, dass Sie verstehen, dass dies aufgezeichnet und geprüft wird und die Person benachrichtigt wird.",
  "p.access.could_not_open": "Der Plan konnte nicht geöffnet werden.",
  "p.access.opened_notice":
    "Der Plan ist für Sie bis {until} geöffnet (Ihre Zeitzone, {tz}). {person} wurde benachrichtigt.",
  "p.access.go_to_plan": "Zum Plan",
  "p.access.f_plan": "Kennung des privaten Plans",
  "p.access.f_reason":
    "Warum Sie Zugriff brauchen (erforderlich, mindestens {min} Zeichen)",
  "p.access.reason_count": "{n} Zeichen; mindestens {min} und höchstens {max}.",
  "p.access.f_minutes": "Wie lange, in Minuten ({min} bis {max})",
  "p.access.f_notify":
    "Zu benachrichtigende Person: ‘person:’ oder ‘worker:’ und ihre Kennung (nicht Sie)",
  "p.access.f_acknowledge":
    "Ich verstehe, dass dies aufgezeichnet und geprüft wird und dass die von mir genannte Person benachrichtigt wird.",
  "p.access.open": "Plan für Notfallzugriff öffnen",
  "p.access.grants_heading": "Notfallzugriffe",
  "p.access.grants_intro":
    "Ihre, die neuesten zuerst; ein Administrator sieht die aller Personen. Ein Zugriff wird nie gelöscht. Zeiten sind in Ihrer Zeitzone, {tz}.",
  "p.access.grants_caption": "Notfallzugriffe, die neuesten zuerst",
  "p.access.grants_empty": "Es wurde kein Notfallzugriff geöffnet.",
  "p.access.loading": "Wird geladen…",
  "p.access.could_not_load_grants":
    "Die Notfallzugriffe konnten nicht geladen werden.",
  "p.access.could_not_revoke":
    "Der Notfallzugriff konnte nicht beendet werden.",
  "p.access.revoked_notice": "Der Notfallzugriff auf {plan} ist beendet.",
  "p.access.revoke": "Jetzt beenden",
  "p.access.revoke_for": "Jetzt beenden: Notfallzugriff auf {plan}",
  "p.access.state_live": "In Kraft",
  "p.access.state_revoked": "Vorzeitig beendet",
  "p.access.state_expired": "Abgelaufen",
  "p.access.ended_by": "{when}, beendet von {who}",
  "p.access.col_state": "Zustand",
  "p.access.col_holder": "Inhaber",
  "p.access.col_opened": "Geöffnet",
  "p.access.col_ends": "Endet oder endete",
  "p.access.col_reason": "Angegebener Grund",
  "p.access.col_told": "Benachrichtigte Person",
  "p.access.col_act": "Aktion",
  "p.access.refused_sign_in": "Melden Sie sich zuerst an.",
  "p.access.refused_review_forbidden":
    "Die Zugriffsprüfung ist nur für Administratoren.",
  "p.access.refused_review_invalid":
    "Der Dienst hat den Plan oder die Zahl der Tage nicht angenommen.",
  "p.access.refused_open_forbidden":
    "Sie dürfen den Notfallzugriff nicht nutzen: Er erfordert die Break-Glass-Berechtigung oder Administratorzugriff.",
  "p.access.refused_open_not_found": "Es gibt keinen Plan mit dieser Kennung.",
  "p.access.refused_open_conflict":
    "Sie haben bereits Notfallzugriff auf diesen Plan; die Liste unten wurde neu gelesen.",
  "p.access.refused_open_invalid":
    "Der Plan wurde nicht geöffnet. Entweder ist er nicht privat, es gibt also nichts zu öffnen, oder die zu benachrichtigende Person sind Sie, oder diese Person kann nicht benachrichtigt werden. Es wurde kein Zugriff gewährt.",
  "p.access.refused_open_limit":
    "Sie haben die an einem Tag erlaubte Höchstzahl an Notfallzugriffen geöffnet (fünf, gezählt über die letzten 24 Stunden).",
  "p.access.refused_revoke_not_found":
    "Dieser Zugriff wurde nicht gefunden, oder Sie dürfen ihn nicht beenden.",
  "p.deescalation.nav": "Scorecards und Deeskalation",
  "p.deescalation.title": "Scorecards und Deeskalation — PPM",
  "p.deescalation.h1": "Scorecards und Deeskalation",
  "p.deescalation.lede":
    "Die Eskalationsstufe, wie der Sponsor sie festgelegt hat, die Kriterien für die Rückstufung mit ihren Nachweisen je Quartal und die Scorecards, deren Kennzahlen je abgeschlossenem Quartal gelesen werden.",
  "p.deescalation.tracker_heading": "Der Deeskalations-Tracker",
  "p.deescalation.tracker_says":
    "Die Stufe wird so erfasst, wie der Sponsor sie festgelegt hat. Sie wird hier nie berechnet, und nichts auf dieser Seite verändert sie: Die Reihe erfüllter Quartale wird der entscheidenden Person angeboten.",
  "p.deescalation.loading": "Wird geladen…",
  "p.deescalation.could_not_load":
    "Der Deeskalations-Tracker konnte nicht geladen werden.",
  "p.deescalation.could_not_save": "Die Änderung wurde nicht gespeichert.",
  "p.deescalation.level_heading": "Eskalationsstufe",
  "p.deescalation.level_now": "Stufe {level}, festgelegt am {date}.",
  "p.deescalation.level_note": "Notiz des Sponsors: {note}",
  "p.deescalation.level_none": "Es wurde noch keine Eskalationsstufe erfasst.",
  "p.deescalation.level_history": "Frühere Stufen",
  "p.deescalation.level_history_item": "{level}, festgelegt am {date}",
  "p.deescalation.run_heading": "Erfüllte Quartale in Folge",
  "p.deescalation.run_one":
    "{n} abgeschlossenes Quartal in Folge erfüllt, von den {need}, die die Regel verlangt.",
  "p.deescalation.run_other":
    "{n} abgeschlossene Quartale in Folge erfüllt, von den {need}, die die Regel verlangt.",
  "p.deescalation.rule_satisfied":
    "Die Regel ist erfüllt. Das abzuwägen ist Sache des Sponsors; die Stufe ändert sich nicht von selbst.",
  "p.deescalation.rule_not_satisfied": "Die Regel ist noch nicht erfüllt.",
  "p.deescalation.setting_unusable":
    "Die Einstellung der Installation, wie viele Quartale die Regel verlangt, war nicht verwendbar; daher gilt der Standardwert {n}.",
  "p.deescalation.reads_scorecard": "Der Tracker liest die Scorecard {name}.",
  "p.deescalation.reads_no_scorecard":
    "Keine Scorecard ist die, die der Tracker liest; daher zählen nur die Kriterien. Markieren Sie eine beim Hinzufügen unten entsprechend.",
  "p.deescalation.hidden_kpis_one":
    "{n} Kennzahl auf dieser Scorecard gehört zu einem Plan, den Sie nicht lesen dürfen: Sie wird hier weder gezeigt noch gezählt.",
  "p.deescalation.hidden_kpis_other":
    "{n} Kennzahlen auf dieser Scorecard gehören zu Plänen, die Sie nicht lesen dürfen: Sie werden hier weder gezeigt noch gezählt.",
  "p.deescalation.quarters_label": "Jedes abgeschlossene Quartal",
  "p.deescalation.col_quarter": "Quartal",
  "p.deescalation.col_state": "Stand",
  "p.deescalation.col_reasons": "Warum",
  "p.deescalation.state_met": "Erfüllt",
  "p.deescalation.state_not_met": "Nicht erfüllt",
  "p.deescalation.state_missing": "Nichts erfasst",
  "p.deescalation.state_with_value": "{state} (Wert {value})",
  "p.deescalation.reason_all_met":
    "Jede Kennzahl und jedes Kriterium wurde erfüllt.",
  "p.deescalation.reason_nothing":
    "Es gibt nichts zu messen: keine Kennzahl und kein Kriterium.",
  "p.deescalation.reason_no_kpis":
    "Die Scorecard hat keine Kennzahl, die Sie lesen können.",
  "p.deescalation.reason_not_met": "Nicht erfüllt: {list}.",
  "p.deescalation.reason_missing": "Nichts erfasst für: {list}.",
  "p.deescalation.quarter": "{q}. Quartal {year}",
  "p.deescalation.quarter_standing": "{quarter} – {state}. {reasons}",
  "p.deescalation.tracker_kpis_label": "Die Scorecard des Trackers, je Quartal",
  "p.deescalation.set_level_heading": "Die Eskalationsstufe erfassen",
  "p.deescalation.set_level_says":
    "Nur der Sponsor (jemand mit der Rolle sponsor) oder ein Administrator kann die Stufe erfassen, so wie sie festgelegt wurde. Eine Stufe hat {min} bis {max} Zeichen; die Notiz ist optional.",
  "p.deescalation.f_level": "Stufe",
  "p.deescalation.f_level_note": "Notiz (optional)",
  "p.deescalation.save_level": "Stufe erfassen",
  "p.deescalation.level_saved": "Stufe {level} erfasst.",
  "p.deescalation.unchanged":
    "Nichts hat sich geändert, daher wurde nichts gesendet.",
  "p.deescalation.problem_level": "Eine Stufe hat {min} bis {max} Zeichen.",
  "p.deescalation.refused_level_forbidden":
    "Abgelehnt: Nur der Sponsor oder ein Administrator kann die Eskalationsstufe erfassen.",
  "p.deescalation.criteria_heading": "Kriterien für die Rückstufung",
  "p.deescalation.criteria_says":
    "Ein Kriterium wird hinzugefügt und nie bearbeitet oder stillgelegt. Sein Nachweis wird für ein abgeschlossenes Quartal erfasst, und es zählt der neueste Eintrag für dieses Quartal.",
  "p.deescalation.criteria_none": "Noch keine Kriterien.",
  "p.deescalation.evidence_of": "Nachweise für: {criterion}",
  "p.deescalation.evidence_link": "Nachweis",
  "p.deescalation.add_criterion_heading": "Ein Kriterium hinzufügen",
  "p.deescalation.f_criterion": "Kriterium ({min} bis {max} Zeichen)",
  "p.deescalation.add_criterion": "Kriterium hinzufügen",
  "p.deescalation.criterion_added": "Kriterium hinzugefügt.",
  "p.deescalation.problem_criterion":
    "Ein Kriterium hat {min} bis {max} Zeichen.",
  "p.deescalation.evidence_heading": "Nachweis erfassen",
  "p.deescalation.evidence_says":
    "Wählen Sie ein Kriterium und ein abgeschlossenes Quartal (ein laufendes Quartal kann nicht erfasst werden), geben Sie an, ob es erfüllt wurde, und schreiben Sie eine Notiz von mindestens {min} Zeichen. Ein Link ist optional: eine http- oder https-Adresse ohne Leerzeichen, bis zu {max} Zeichen.",
  "p.deescalation.f_ev_criterion": "Kriterium",
  "p.deescalation.f_ev_quarter": "Abgeschlossenes Quartal",
  "p.deescalation.f_ev_met": "Wurde es erfüllt?",
  "p.deescalation.f_ev_note": "Notiz (mindestens {min} Zeichen)",
  "p.deescalation.f_ev_link": "Link zum Nachweis (optional)",
  "p.deescalation.save_evidence": "Nachweis erfassen",
  "p.deescalation.evidence_saved": "Nachweis für {quarter} erfasst.",
  "p.deescalation.problem_criterion_choose":
    "Fügen Sie zuerst ein Kriterium hinzu und erfassen Sie dann seinen Nachweis.",
  "p.deescalation.problem_quarter": "Wählen Sie ein abgeschlossenes Quartal.",
  "p.deescalation.problem_quarter_unfinished":
    "Dieses Quartal ist noch nicht abgeschlossen und kann daher noch nicht erfasst werden.",
  "p.deescalation.problem_met": "Geben Sie an, ob das Kriterium erfüllt wurde.",
  "p.deescalation.problem_note":
    "Eine Notiz von mindestens {min} Zeichen ist nötig.",
  "p.deescalation.problem_link":
    "Ein Link ist eine http- oder https-Adresse ohne Leerzeichen, bis zu {max} Zeichen.",
  "p.deescalation.refused_evidence":
    "Abgelehnt: Das Quartal muss abgeschlossen sein, die Notiz mindestens fünf Zeichen haben und ein Link eine http- oder https-Adresse ohne Leerzeichen sein.",
  "p.deescalation.refused_gone":
    "Dieser Eintrag ist nicht mehr vorhanden. Laden Sie die Seite neu.",
  "p.deescalation.scorecards_heading": "Alle Scorecards",
  "p.deescalation.scorecards_says":
    "Eine Scorecard ist eine benannte Gruppe von Kennzahlen, die je abgeschlossenem Quartal gelesen wird. Eine Kennzahl ist in einem Quartal erfüllt, wenn ihr neuester Messwert in diesem Quartal ihr Ziel erreicht oder übertrifft; ein Quartal ohne Messwert fehlt, was nicht dasselbe ist wie nicht erfüllt.",
  "p.deescalation.scorecards_none": "Noch keine Scorecards.",
  "p.deescalation.could_not_load_scorecards":
    "Die Scorecards konnten nicht geladen werden.",
  "p.deescalation.col_scorecard": "Bezeichnung der Scorecard",
  "p.deescalation.col_tracker": "Vom Tracker gelesen",
  "p.deescalation.is_tracker": "Ja, die der Tracker liest",
  "p.deescalation.not_tracker": "Nein",
  "p.deescalation.add_scorecard_heading": "Eine Scorecard hinzufügen",
  "p.deescalation.add_scorecard_says":
    "Ein Name mit {min} bis {max} Zeichen, der noch nicht vergeben ist (ohne Rücksicht auf Groß- und Kleinschreibung). Höchstens eine Scorecard ist die, die der Tracker liest.",
  "p.deescalation.f_name": "Bezeichnung",
  "p.deescalation.f_for_tracker": "Die, die der Tracker liest",
  "p.deescalation.add_scorecard": "Scorecard hinzufügen",
  "p.deescalation.scorecard_added": "Scorecard {name} hinzugefügt.",
  "p.deescalation.problem_name":
    "Der Name einer Scorecard hat {min} bis {max} Zeichen.",
  "p.deescalation.problem_name_taken":
    "Eine Scorecard hat bereits diesen Namen.",
  "p.deescalation.problem_tracker_taken":
    "Die Scorecard {name} ist bereits die, die der Tracker liest.",
  "p.deescalation.scorecard_heading": "Eine Scorecard je Quartal",
  "p.deescalation.f_scorecard": "Welche Scorecard",
  "p.deescalation.choose_scorecard": "Scorecard wählen",
  "p.deescalation.could_not_load_scorecard":
    "Die Scorecard konnte nicht geladen werden.",
  "p.deescalation.scorecard_says":
    "Jede Kennzahl je abgeschlossenem Quartal: erfüllt, nicht erfüllt oder nichts erfasst.",
  "p.deescalation.scorecard_kpis_label": "Kennzahlen von {name}, je Quartal",
  "p.deescalation.scorecard_quarters_label": "Jedes Quartal der Scorecard",
  "p.deescalation.col_kpi": "Kennzahl",
  "p.deescalation.col_target": "Ziel",
  "p.deescalation.target_at_least": "mindestens {value} {unit}",
  "p.deescalation.target_at_most": "höchstens {value} {unit}",
  "p.deescalation.no_kpis":
    "Diese Scorecard hat keine Kennzahlen, die Sie lesen können.",
  "p.deescalation.unit_percent": "Prozent",
  "p.deescalation.unit_count": "eine Anzahl",
  "p.deescalation.kpis_heading": "Ihre Kennzahlen (KPIs) wählen",
  "p.deescalation.kpis_says":
    "Kreuzen Sie die Kennzahlen an, die diese Scorecard enthält, höchstens {max}. Speichern ersetzt die Gruppe; eine Kennzahl auf einem Plan, den Sie nicht lesen dürfen, bleibt auf der Karte.",
  "p.deescalation.f_kpis": "Kennzahlen auf der Scorecard",
  "p.deescalation.kpis_none":
    "Es gibt keine Kennzahlen zur Auswahl. Fügen Sie sie auf der Kennzahlenseite hinzu oder unten aus der Vorlage.",
  "p.deescalation.save_kpis": "Kennzahlen speichern",
  "p.deescalation.kpis_saved_one": "Die Scorecard enthält jetzt {n} Kennzahl.",
  "p.deescalation.kpis_saved_other":
    "Die Scorecard enthält jetzt {n} Kennzahlen.",
  "p.deescalation.problem_kpis_max":
    "Eine Scorecard enthält höchstens {max} Kennzahlen.",
  "p.deescalation.refused_kpis":
    "Abgelehnt: Jede muss eine Kennzahl sein, die Sie lesen können, und eine Scorecard enthält höchstens 60.",
  "p.deescalation.template_heading": "Messgrößen aus der Vorlage hinzufügen",
  "p.deescalation.template_says":
    "Ein Startsatz, angeboten und nicht angewendet. Er hat keine Ziele: Sie legen jedes Ziel und jede rote Linie fest, und die gewählten Messgrößen werden Kennzahlen auf dieser Scorecard.",
  "p.deescalation.template_rule":
    "Wo höher besser ist, muss die rote Linie unter dem Ziel liegen; wo niedriger besser ist, darüber. Ziele und rote Linien sind ganze Zahlen.",
  "p.deescalation.could_not_load_template":
    "Die Vorlage konnte nicht geladen werden.",
  "p.deescalation.higher_better": "Höher ist besser",
  "p.deescalation.lower_better": "Niedriger ist besser",
  "p.deescalation.unit": "gemessen als {unit}",
  "p.deescalation.f_pick": "{measure} hinzufügen",
  "p.deescalation.f_target": "Ziel",
  "p.deescalation.f_red": "Rote Linie",
  "p.deescalation.add_measures": "Messgrößen hinzufügen",
  "p.deescalation.measures_added_one":
    "{n} Messgröße zur Scorecard hinzugefügt.",
  "p.deescalation.measures_added_other":
    "{n} Messgrößen zur Scorecard hinzugefügt.",
  "p.deescalation.problem_pick_one": "Wählen Sie mindestens eine Messgröße.",
  "p.deescalation.problem_whole":
    "{measure}: Ziel und rote Linie sind ganze Zahlen.",
  "p.deescalation.problem_red_higher":
    "{measure}: Höher ist besser, daher muss die rote Linie unter dem Ziel liegen.",
  "p.deescalation.problem_red_lower":
    "{measure}: Niedriger ist besser, daher muss die rote Linie über dem Ziel liegen.",
  "p.deescalation.problem_not_in_template":
    "{measure} ist nicht in der Vorlage.",
  "p.deescalation.refused_template":
    "Abgelehnt: Prüfen Sie jedes Ziel und jede rote Linie anhand der Richtungsregel.",
  "p.deescalation.measure_programmes_with_independent_assurance":
    "Programme mit unabhängiger Prüfung im letzten Quartal",
  "p.deescalation.measure_milestones_met_on_time":
    "Meilensteine, die zu oder vor ihrem Datum erreicht wurden",
  "p.deescalation.measure_benefits_validated":
    "Geplante Nutzen, die von jemandem außerhalb des Programms bestätigt wurden",
  "p.deescalation.measure_red_without_decision":
    "Programme auf Rot ohne erfasste Entscheidung",
  "p.deescalation.measure_open_critical_issues":
    "Offene Probleme mit kritischem Schweregrad",
  "p.deescalation.measure_change_requests_waiting":
    "Änderungsanträge, die mehr als 30 Tage auf eine Entscheidung warten",
  "p.phase.name_initiating": "Initiierung",
  "p.phase.name_planning": "Planung",
  "p.phase.name_executing": "Durchführung",
  "p.phase.name_controlling": "Steuerung",
  "p.phase.name_closing": "Abschluss",
  "p.phase.heading": "Phase des Plans",
  "p.phase.heading_programme": "Phase und Start",
  "p.phase.says":
    "Ein Plan rückt jeweils um eine Phase vor; er kann mit einer Begründung, die mit dem Wechsel erfasst wird, in eine frühere Phase zurückgehen. Die Phase verhindert nie, dass Arbeit erfasst wird.",
  "p.phase.could_not_load": "Die Phase des Plans konnte nicht geladen werden",
  "p.phase.now": "Der Plan ist in der Phase {phase}.",
  "p.phase.none":
    "Noch ist keine Phase erfasst: Der erste Wechsel führt in die Initiierung.",
  "p.phase.time_heading": "Zeit in jeder Phase",
  "p.phase.days_in_one": "{phase}: {days} Tag",
  "p.phase.days_in_other": "{phase}: {days} Tage",
  "p.phase.visits_one": "{visits}-mal betreten",
  "p.phase.visits_other": "{visits}-mal betreten",
  "p.phase.current": "aktuell",
  "p.phase.check_heading": "Startprüfung",
  "p.phase.check_says":
    "Ein Programm startet, wenn seine Phase zu Durchführung wechselt. Der Start wird abgelehnt, wenn alle Plätze für Programme in aktiver Umsetzung belegt sind (pausierte und gestoppte Programme belegen keinen) oder wenn der Engpass-Pool in einem der nächsten sechs Monate zu wenig Angebot hat. Ein abgelehntes Programm kommt in die Startwarteschlange und behält seinen Platz; es kann trotzdem mit einer Übersteuerungsbegründung von {min} oder mehr Zeichen starten, die erfasst und protokolliert wird. Das Lesen der Prüfung bewegt nichts.",
  "p.phase.could_not_check": "Die Startprüfung konnte nicht gelesen werden",
  "p.phase.places":
    "{in_delivery} von {limit} Plätzen für Programme in aktiver Umsetzung sind belegt.",
  "p.phase.no_limit":
    "Für Programme in aktiver Umsetzung ist keine Grenze gesetzt.",
  "p.phase.limit_unusable":
    "Die Grenze der Installation für Programme in Umsetzung war nicht verwendbar, daher gilt keine Grenze.",
  "p.phase.pool_is": "Der Engpass-Pool ist {pool}.",
  "p.phase.no_pool": "Es ist kein Engpass-Pool bestimmt.",
  "p.phase.would_refuse": "Ein Start wäre jetzt abgelehnt:",
  "p.phase.would_start": "Ein Start wäre jetzt möglich.",
  "p.phase.in_queue":
    "Dieses Programm ist Nummer {n} in der Startwarteschlange.",
  "p.phase.see_queue": "Startwarteschlange ansehen",
  "p.phase.refusal_limit":
    "Alle {limit} Plätze für Programme in aktiver Umsetzung sind belegt ({in_delivery} in Umsetzung).",
  "p.phase.refusal_pool":
    "Der Engpass-Pool {pool} hat zu wenig Angebot in {months}.",
  "p.phase.demand_heading": "Feste Nachfrage dieses Programms",
  "p.phase.demand_caption":
    "Vollzeitäquivalente nach Kompetenzpool und Monat, aus festen Zuordnungen auf diesem Programm und den Plänen darunter",
  "p.phase.no_demand":
    "Keine feste Nachfrage dieses Programms in einem Kompetenzpool in den nächsten sechs Monaten.",
  "p.phase.col_pool": "Kompetenzpool",
  "p.phase.check_again": "Erneut prüfen",
  "p.phase.move_heading": "Phase ändern",
  "p.phase.f_to": "Wechseln zu",
  "p.phase.forward_to": "Weiter zu {phase}",
  "p.phase.back_to": "Zurück zu {phase}",
  "p.phase.back_needs_reason": "Ein Wechsel zurück braucht eine Begründung.",
  "p.phase.f_reason":
    "Begründung (nötig für einen Wechsel zurück; wird mit dem Wechsel erfasst)",
  "p.phase.start_may_queue":
    "Der Wechsel zu Durchführung ist der Start dieses Programms: Findet die Prüfung beim Senden keinen Platz, wird der Start abgelehnt und das Programm kommt in die Startwarteschlange.",
  "p.phase.start_would_queue":
    "Dieser Start würde abgelehnt. Ohne Übersteuerungsbegründung gesendet, kommt das Programm in die Startwarteschlange (oder behält dort seinen Platz).",
  "p.phase.f_override":
    "Übersteuerungsbegründung, um trotzdem zu starten ({min} oder mehr Zeichen; wird erfasst und protokolliert)",
  "p.phase.refused_why": "Warum der Start abgelehnt wurde",
  "p.phase.move": "Phase wechseln",
  "p.phase.moved": "Der Plan ist jetzt in der Phase {phase}.",
  "p.phase.started_override":
    "Das Programm wurde per Übersteuerung gestartet: Es ist jetzt in der Phase {phase}, und die Übersteuerung ist erfasst.",
  "p.phase.start_refused_place":
    "Der Start wurde abgelehnt, daher ist das Programm Nummer {n} in der Startwarteschlange. Um es trotzdem zu starten, geben Sie eine Übersteuerungsbegründung an.",
  "p.phase.start_refused":
    "Der Start wurde abgelehnt, und das Programm kam in die Startwarteschlange. Um es trotzdem zu starten, geben Sie eine Übersteuerungsbegründung an.",
  "p.phase.could_not_move": "Die Phase konnte nicht gewechselt werden",
  "p.phase.problem_choose":
    "Wählen Sie die Phase, zu der gewechselt werden soll.",
  "p.phase.problem_same":
    "Der Plan ist bereits in dieser Phase, daher wurde nichts gesendet.",
  "p.phase.problem_skip": "Vorwärts geht es jeweils nur um eine Phase.",
  "p.phase.problem_reason":
    "Für einen Wechsel zurück ist eine Begründung nötig.",
  "p.phase.problem_override":
    "Eine Übersteuerungsbegründung hat {min} oder mehr Zeichen.",
  "p.phase.refused_move":
    "Abgelehnt: Vorwärts geht es um eine Phase auf einmal, zurück nur mit Begründung, und der Plan darf nicht schon in dieser Phase sein.",
  "p.phase.refused_gone": "Diesen Plan gibt es nicht mehr.",
  "p.proposals.act_submit": "Einreichen",
  "p.proposals.act_review": "Prüfung beginnen",
  "p.proposals.act_approve": "Genehmigen",
  "p.proposals.act_reject": "Ablehnen",
  "p.proposals.act_submit_named": "{title} einreichen",
  "p.proposals.act_review_named": "Prüfung von {title} beginnen",
  "p.proposals.act_approve_named": "{title} genehmigen",
  "p.proposals.act_reject_named": "{title} ablehnen",
  "p.proposals.ask_rule":
    "Eine externe Anfrage des Sponsors muss benennen, was sie verdrängt (ein Zielkonflikt von {min} oder mehr Zeichen), bevor sie genehmigt werden kann.",
  "p.proposals.new_form": "Neuer Vorschlag",
  "p.proposals.f_external_ask": "Eine externe Anfrage des Sponsors",
  "p.proposals.f_new_tradeoff":
    "Was sie verdrängt (der Zielkonflikt, {min} bis {max} Zeichen; er kann später, vor der Genehmigung, benannt werden)",
  "p.proposals.col_tradeoff": "Externe Anfrage und Zielkonflikt",
  "p.proposals.not_ask": "Keine externe Anfrage",
  "p.proposals.ask_with": "Externe Anfrage. Zielkonflikt: {tradeoff}",
  "p.proposals.ask_without": "Externe Anfrage. Noch kein Zielkonflikt benannt.",
  "p.proposals.tradeoff_name": "Zielkonflikt benennen",
  "p.proposals.tradeoff_change": "Zielkonflikt ändern",
  "p.proposals.tradeoff_name_named": "Zielkonflikt benennen für {title}",
  "p.proposals.tradeoff_change_named": "Zielkonflikt ändern für {title}",
  "p.proposals.tradeoff_heading": "Zielkonflikt von {title}",
  "p.proposals.tradeoff_says":
    "Benennen Sie, was diese Anfrage verdrängt, in {min} bis {max} Zeichen. Das geht, bis der Vorschlag genehmigt oder abgelehnt ist, und jede Änderung wird erfasst.",
  "p.proposals.f_tradeoff": "Was sie verdrängt ({min} bis {max} Zeichen)",
  "p.proposals.tradeoff_save": "Zielkonflikt speichern",
  "p.proposals.tradeoff_close": "Formular zum Zielkonflikt schließen",
  "p.proposals.tradeoff_unchanged":
    "Nichts geändert, daher wurde nichts gesendet.",
  "p.proposals.tradeoff_saved": "Zielkonflikt für {title} gespeichert.",
  "p.proposals.problem_title": "Ein Vorschlag braucht einen Titel.",
  "p.proposals.problem_tradeoff":
    "Ein Zielkonflikt hat {min} bis {max} Zeichen.",
  "p.proposals.problem_not_ask":
    "Nur eine externe Anfrage hat einen Zielkonflikt.",
  "p.proposals.problem_decided":
    "Über den Vorschlag ist entschieden, daher kann sich sein Zielkonflikt nicht mehr ändern.",
  "p.proposals.problem_approve":
    "{title} ist eine externe Anfrage: Benennen Sie ihren Zielkonflikt ({min} oder mehr Zeichen), bevor Sie sie genehmigen.",
  "p.proposals.refused_create":
    "Abgelehnt: Ein Vorschlag braucht einen Titel, ein Betrag eine Währung, und ein Zielkonflikt hat höchstens {max} Zeichen.",
  "p.proposals.refused_tradeoff":
    "Abgelehnt: Ein Zielkonflikt hat {min} bis {max} Zeichen und gehört zu einer externen Anfrage, über die noch nicht entschieden ist.",
  "p.proposals.refused_step":
    "Abgelehnt: Dieser Schritt folgt nicht aus dem Status des Vorschlags.",
  "p.proposals.refused_gone": "Diesen Vorschlag gibt es nicht mehr.",
  "p.thresholds.opened_by_kind":
    "Geöffnet, weil das Programm seine Schwelle überschritten hat: {kind}.",
  "p.thresholds.joined_by_kind":
    "Während dies offen war, hat das Programm auch eine Schwelle überschritten: {kind}.",
  "p.deescalation.refused_tracker_taken":
    "Abgelehnt: Eine andere Scorecard ist bereits die, die der Tracker liest.",
  "p.deescalation.level_now_by":
    "Stufe {level}, festgelegt am {date} von {who}.",
  "p.deescalation.level_history_item_by":
    "{level}, festgelegt am {date} von {who}",
  "p.deescalation.evidence_recorded_by": "erfasst von {who} am {date}",
  "p.deescalation.evidence_recorded_on": "erfasst am {date}",
  "p.proposals.refused_approve_status":
    "Abgelehnt: Nur ein Vorschlag in Prüfung kann genehmigt werden.",
  "p.proposals.refused_tradeoff_required":
    "Abgelehnt: Eine externe Anfrage braucht einen benannten Zielkonflikt von {min} oder mehr Zeichen, bevor sie genehmigt werden kann.",
  "p.ledger.untraced_no_readable_line":
    "Keine Budgetzeile, die Sie lesen dürfen, greift auf diese Zuweisung zu, daher lässt sich hier nicht verfolgen, wohin ihr Geld geht.",
  "p.ledger.untraced_not_on_a_line":
    "Ein Teil dieser Zuweisung ist noch auf keiner Budgetzeile geplant.",
  "p.ledger.untraced_lines_over_plan":
    "Die Budgetzeilen planen mehr, als diese Zuweisung enthält.",
  "p.ledger.untraced_other":
    "Ein Teil des Geldes dieser Zuweisung lässt sich nicht verfolgen.",
  "p.status.title_ppm": "Statusberichte — PPM",
  "p.status.title": "Statusberichte",
  "p.status.link": "Statusberichte und ihre Belege",
  "p.status.intro":
    "Ein Statusbericht ist die Einschätzung einer Person, wie der Plan läuft (im Plan, gefährdet oder nicht im Plan), mit dem Geschehen in Worten. Hier wird kein Wert berechnet.",
  "p.status.could_not_load": "Der Status des Plans konnte nicht geladen werden",
  "p.status.now_heading": "Wo der Plan steht",
  "p.status.no_update": "Es wurde noch kein Statusbericht veröffentlicht.",
  "p.status.cadence_is_one": "Ein Bericht wird jeden Tag erwartet.",
  "p.status.cadence_is_other": "Ein Bericht wird alle {n} Tage erwartet.",
  "p.status.standing_overdue_one":
    "Der nächste Bericht war am {date} fällig: er ist {n} Tag überfällig.",
  "p.status.standing_overdue_other":
    "Der nächste Bericht war am {date} fällig: er ist {n} Tage überfällig.",
  "p.status.standing_due": "Heute, am {date}, ist ein Bericht fällig.",
  "p.status.standing_next": "Der nächste Bericht ist am {date} fällig.",
  "p.status.no_cadence":
    "Es ist kein Rhythmus festgelegt, daher ist nie ein Bericht fällig oder überfällig.",
  "p.status.as_of": "Stand {date}, in der Zeitzone der Installation.",
  "p.status.loading": "Wird geladen…",
  "p.status.posted_on_by": "veröffentlicht am {date} von {who}",
  "p.status.posted_on": "veröffentlicht am {date}",
  "p.status.claims_of": "Aussagen im Bericht vom {date}",
  "p.status.source_metric": "{metric}: derzeit {value}",
  "p.status.source_metric_unknown": "{metric}: unbekannt (nicht null)",
  "p.status.source_unavailable": "Eine Quelle, die nicht mehr verfügbar ist",
  "p.status.kind_task": "Aufgabe: {title}",
  "p.status.kind_risk": "Risiko: {title}",
  "p.status.kind_milestone": "Meilenstein: {title}",
  "p.status.kind_issue": "Problem: {title}",
  "p.status.kind_other": "Eintrag: {title}",
  "p.status.metric_overdue": "Überfällige Aufgaben",
  "p.status.metric_due_soon": "Bald fällige Aufgaben",
  "p.status.metric_blocked": "Blockierte Aufgaben",
  "p.status.metric_no_due_date": "Aufgaben ohne Fälligkeitsdatum",
  "p.status.metric_def_overdue":
    "Offene Aufgaben, deren Fälligkeitsdatum vor heute liegt. Eine heute fällige Aufgabe ist nicht überfällig. Unbekannt, wenn es offene Aufgaben gibt und keine ein Fälligkeitsdatum hat.",
  "p.status.metric_def_due_soon":
    "Offene Aufgaben, die heute oder in den nächsten {days} Tagen fällig sind. Unbekannt, wenn es offene Aufgaben gibt und keine ein Fälligkeitsdatum hat.",
  "p.status.metric_def_blocked": "Offene Aufgaben mit dem Status blockiert.",
  "p.status.metric_def_no_due_date":
    "Offene Aufgaben ohne Fälligkeitsdatum: die Aufgaben, die die anderen Zahlen nicht sehen.",
  "p.status.metrics_heading": "Wie die Zahlen gezählt werden",
  "p.status.metrics_says":
    "Über die offenen Aufgaben des Plans (eine Aufgabe, deren Zustand nicht erledigt ist), gezählt, wenn ein Bericht gelesen wird, nicht als er veröffentlicht wurde.",
  "p.status.cadence_heading": "Wie oft ein Bericht erwartet wird",
  "p.status.cadence_says":
    "Alle {min} bis {max} Tage. Das Festlegen startet die Uhr ab heute neu, daher wird derselbe Rhythmus nicht erneut gesendet. Ein einmal festgelegter Rhythmus kann nicht entfernt werden.",
  "p.status.cadence_set_by": "Festgelegt am {date} von {who}.",
  "p.status.cadence_set_on": "Festgelegt am {date}.",
  "p.status.f_cadence": "Tage zwischen Berichten ({min} bis {max})",
  "p.status.save_cadence": "Rhythmus speichern",
  "p.status.unchanged":
    "Nichts hat sich geändert, daher wurde nichts gesendet.",
  "p.status.problem_cadence":
    "Ein Rhythmus ist eine ganze Zahl von Tagen von {min} bis {max}.",
  "p.status.cadence_saved_one":
    "Jetzt wird jeden Tag ein Bericht erwartet; die Uhr beginnt heute.",
  "p.status.cadence_saved_other":
    "Jetzt wird alle {n} Tage ein Bericht erwartet; die Uhr beginnt heute.",
  "p.status.could_not_save": "Der Rhythmus konnte nicht gespeichert werden",
  "p.status.refused_gone": "Diesen Plan gibt es nicht mehr.",
  "p.status.history_heading": "Alle Berichte, die neuesten zuerst",
  "p.status.history_says":
    "Berichte werden nie geändert oder entfernt. Jede Quelle wird so gezeigt, wie sie jetzt ist: Ein verschwundener Eintrag ist nicht mehr verfügbar, und jede Zahl hat ihren heutigen Wert.",
  "p.status.history_none": "Noch keine Berichte.",
  "p.status.post_heading": "Einen Bericht veröffentlichen",
  "p.status.post_says":
    "Wählen Sie die Einschätzung und sagen Sie, was geschieht (bis zu {max} Zeichen). Ein Bericht kann nach dem Veröffentlichen nicht geändert oder entfernt werden, und jeder neue startet die Uhr des Rhythmus neu.",
  "p.status.claims_rule":
    "Sie können bis zu {claims} Aussagen hinzufügen, jede ein Satz von bis zu {chars} Zeichen mit bis zu {sources} Quellen: den Zahlen unten oder den Aufgaben, Risiken, Meilensteinen und Problemen des Plans. Jede Quelle wird beim Veröffentlichen geprüft.",
  "p.status.f_call": "Die Einschätzung",
  "p.status.call_on_track": "Im Plan",
  "p.status.call_at_risk": "Gefährdet",
  "p.status.call_off_track": "Nicht im Plan",
  "p.status.f_narrative": "Was geschieht (bis zu {max} Zeichen)",
  "p.status.claim_legend": "Aussage {n}",
  "p.status.f_claim_text": "Die Aussage (bis zu {chars} Zeichen)",
  "p.status.f_metrics": "Zahlen, die sie anführt",
  "p.status.f_record": "Ein Eintrag des Plans, den Aussage {n} anführt",
  "p.status.choose_record": "Einen Eintrag wählen",
  "p.status.add_record": "Den Eintrag zu Aussage {n} hinzufügen",
  "p.status.claim_records": "Einträge, die Aussage {n} anführt",
  "p.status.remove": "Entfernen",
  "p.status.remove_source": "{source} aus Aussage {n} entfernen",
  "p.status.remove_claim": "Aussage {n} entfernen",
  "p.status.add_claim": "Eine Aussage hinzufügen",
  "p.status.post": "Den Bericht veröffentlichen",
  "p.status.posted": "Bericht veröffentlicht: {call}.",
  "p.status.could_not_post": "Der Bericht konnte nicht veröffentlicht werden",
  "p.status.problem_call": "Wählen Sie im Plan, gefährdet oder nicht im Plan.",
  "p.status.problem_narrative":
    "Sagen Sie, was geschieht, in bis zu {max} Zeichen.",
  "p.status.problem_claims": "Ein Bericht enthält höchstens {claims} Aussagen.",
  "p.status.problem_claim_text":
    "Eine Aussage ist ein Satz von bis zu {chars} Zeichen.",
  "p.status.problem_sources":
    "Eine Aussage führt höchstens {sources} Quellen an.",
  "p.status.problem_in_claim": "Aussage {n}: {problem}",
  "p.status.refused_update":
    "Abgelehnt: Eine Quelle ist möglicherweise kein Eintrag dieses Plans mehr. Nichts wurde veröffentlicht; prüfen Sie die Aussagen und veröffentlichen Sie erneut.",
  "p.status.due_heading": "Fällige Statusberichte",
  "p.status.due_none":
    "Kein Plan, den Sie lesen können, hat einen fälligen Statusbericht.",
  "p.status.due_could_not_load":
    "Die fälligen Statusberichte konnten nicht geladen werden",
  "p.status.a_plan": "Ein Plan",
  "p.status.due_last": "Letzter Bericht: {call}, am {date}.",
  "p.status.due_never": "Noch kein Bericht.",
  "p.status.group_task": "Aufgaben",
  "p.status.group_risk": "Risiken",
  "p.status.group_milestone": "Meilensteine",
  "p.status.group_issue": "Probleme",
  "l.enum.risk_status.open": "Offen",
  "l.enum.risk_status.mitigating": "In Minderung",
  "l.enum.risk_status.closed": "Geschlossen",
  "l.enum.risk_status.materialized": "Eingetreten",
  "p.risk_edit.change": "Ändern",
  "p.risk_edit.change_named": "Ändern: {title}",
  "p.risk_edit.heading": "Risiko „{title}“ ändern",
  "p.risk_edit.says":
    "Wahrscheinlichkeit und Auswirkung sind ganze Zahlen von {min} bis {max}; die Exposition ist ihr Produkt. Die Minderung umfasst höchstens {max_bytes} Byte (in manchen Schriften weniger Zeichen). Ein Überprüfungsdatum kann geändert, aber nicht entfernt werden. Gesendet wird nur, was Sie ändern.",
  "p.risk_edit.says_escalate":
    "Um festzuhalten, dass ein Risiko eingetreten ist, verwenden Sie „Eskalieren“ in der Tabelle; das hält auch den Zeitpunkt fest.",
  "p.risk_edit.problem_status": "Wählen Sie einen Status.",
  "p.risk_edit.problem_score":
    "Wahrscheinlichkeit und Auswirkung müssen jeweils eine ganze Zahl von {min} bis {max} sein.",
  "p.risk_edit.problem_mitigation":
    "Die Minderung ist länger als {max_bytes} Byte.",
  "p.risk_edit.problem_review_cleared":
    "Ein gesetztes Überprüfungsdatum kann nicht entfernt werden; wählen Sie ein anderes Datum.",
  "p.risk_edit.problem_review_date": "Das Überprüfungsdatum ist kein Datum.",
  "p.risk_edit.unchanged": "Nichts geändert, daher wurde nichts gesendet.",
  "p.risk_edit.saved": "Das Risiko „{title}“ wurde gespeichert.",
  "p.risk_edit.could_not_save": "Das Risiko konnte nicht gespeichert werden",
  "p.risk_edit.refused_gone":
    "Dieses Risiko gehört nicht mehr zu diesem Plan; die Liste wurde neu gelesen.",
  "p.risk_edit.refused_invalid":
    "Abgelehnt: Der Dienst hat Status, Bewertungen oder Minderung nicht angenommen. Nichts wurde geändert.",
  "p.risk_edit.f_status": "Status des Risikos",
  "p.risk_edit.f_probability": "Wahrscheinlichkeit ({min} bis {max})",
  "p.risk_edit.f_impact": "Auswirkung ({min} bis {max})",
  "p.risk_edit.f_mitigation": "Minderung",
  "p.risk_edit.f_review_date": "Überprüfungsdatum",
  "p.risk_edit.save": "Risiko speichern",
  "p.risk_edit.close": "Formular schließen",
  "p.risk_edit.f_reason": "Warum sich die Bewertung geändert hat (optional)",
  "p.risk_edit.history_h": "Bewertungsverlauf",
  "p.risk_edit.history_row":
    "{date}: Wahrscheinlichkeit {probability} × Auswirkung {impact} = {score}.",
  "p.risk_edit.history_reason": "Grund: {reason}",
  "p.risk_edit.history_first":
    "Erste bekannte Bewertung; der Verlauf beginnt hier.",
  "p.plan_issues.refused_gone":
    "Dieses Problem gehört nicht mehr zu diesem Plan; die Liste wurde neu gelesen.",
  "p.plan_issues.refused_remove":
    "Abgelehnt: Sie dürfen auf diesem Plan keine Probleme entfernen. Nichts wurde entfernt.",
  "p.plan_issues.refused_remove_admin":
    "Abgelehnt: Nur eine Administratorin oder ein Administrator kann ein Problem unter dem Workflow dieses Typs entfernen. Nichts wurde entfernt.",
  "p.plan_issues.remove_admin_only":
    "Nur eine Administratorin oder ein Administrator kann „{title}“ entfernen: Der Workflow seines Typs regelt seine Übergänge oder Felder. Nichts wurde gesendet.",
  "p.plan_issues.confirm_remove":
    "Das Problem „{title}“ entfernen? Es verschwindet aus allen Listen; der Eintrag bleibt für die Prüfung erhalten.",
  "p.plan_issues.removed": "Das Problem „{title}“ wurde entfernt.",
  "p.plan_issues.remove": "Entfernen",
  "p.plan_issues.remove_named": "Entfernen: {title}",
  "p.plan_issues.remove_says":
    "Ein entferntes Problem verschwindet aus allen Listen und lässt sich hier nicht wiederherstellen; die Prüfung behält einen Eintrag. Unter einem Workflow, der Übergänge oder Felder regelt, kann nur eine Administratorin oder ein Administrator eines entfernen.",
  "p.plan_issues.raised_on_by": "Am {date} erfasst von {who}.",
  "p.plan_issues.raised_on": "Am {date} erfasst.",
  "p.plan_issues.from_risk_named": "Aus dem Risiko „{title}“ erfasst.",
  "p.plan_issues.from_a_risk":
    "Aus einem nicht mehr aufgeführten Risiko erfasst.",
  "p.plan_issues.from_a_task":
    "Aus einer blockierten Aufgabe erfasst (Aufgabe öffnen)",
  "p.plan_issues.answered_by_change":
    "Durch einen Änderungsantrag beantwortet (Änderungsanträge des Plans ansehen)",
  "p.plan_issues.resolved_on": "Am {date} gelöst.",
  "p.task.issue_heading": "Ein Problem aus dieser Aufgabe",
  "p.task.issue_says":
    "Nur eine blockierte Aufgabe kann als Problem erfasst werden, und nur einmal, solange dieses Problem offen ist. Das Problem ist ein Blocker mittlerer Schwere, zuständig ist die der Aufgabe zugewiesene Person; die Aufgabe selbst bleibt unverändert.",
  "p.task.issue_raised": "Das Problem wurde erfasst.",
  "p.task.issue_open_list": "Probleme des Plans ansehen",
  "p.task.issue_exists":
    "Aus dieser Aufgabe wurde ein offenes Problem erfasst: „{title}“.",
  "p.task.issue_not_blocked":
    "Diese Aufgabe ist nicht blockiert und kann daher nicht als Problem erfasst werden. Markieren Sie sie zuerst auf dem Board als blockiert.",
  "p.task.issue_raise": "Problem aus dieser Aufgabe erfassen",
  "p.task.refused_from_task_not_blocked":
    "Abgelehnt: Nur eine blockierte Aufgabe kann als Problem erfasst werden, und diese ist nicht mehr blockiert. Die Aufgabe wurde neu gelesen.",
  "p.task.refused_from_task_exists":
    "Abgelehnt: Aus dieser Aufgabe wurde bereits ein offenes Problem erfasst. Die Probleme wurden neu gelesen.",
  "p.task.refused_from_task_forbidden":
    "Abgelehnt: Sie dürfen auf diesem Plan keine Probleme erfassen.",
  "p.task.refused_from_task_gone":
    "Diese Aufgabe gehört nicht mehr zu diesem Plan.",
  "p.plan_change_requests.refused_gone":
    "Dieser Änderungsantrag gehört nicht mehr zu diesem Plan; die Liste wurde neu gelesen.",
  "p.plan_change_requests.refused_links_conflict":
    "Abgelehnt: Der Antrag ist geschlossen, oder eines der Probleme gehört inzwischen zu einem anderen Änderungsantrag. Der Antrag und die Probleme des Plans wurden neu gelesen.",
  "p.plan_change_requests.refused_links_invalid":
    "Abgelehnt: Ein gewähltes Problem oder Risiko gehört nicht mehr zu diesem Plan. Probleme und Risiken des Plans wurden neu gelesen.",
  "p.plan_change_requests.refused_preview":
    "Abgelehnt: Dieser Antrag enthält keine für die Terminplanung lesbaren Was-wäre-wenn-Änderungen, daher gibt es keine Prognose.",
  "p.plan_change_requests.refused_baseline":
    "Abgelehnt: Für diese Änderung wurde bereits eine Basislinie erstellt, oder sie ist nicht mehr genehmigt. Der Antrag wurde neu gelesen.",
  "p.plan_change_requests.problem_links_too_many":
    "Wählen Sie höchstens {max} Probleme und {max} Risiken.",
  "p.plan_change_requests.problem_links_elsewhere":
    "„{title}“ gehört bereits zu einem anderen Änderungsantrag; entfernen Sie das Häkchen.",
  "p.plan_change_requests.links_unchanged":
    "Nichts geändert, daher wurde nichts gesendet.",
  "p.plan_change_requests.links_saved":
    "Die Probleme und Risiken, auf die diese Änderung antwortet, wurden gespeichert.",
  "p.plan_change_requests.confirm_baseline":
    "Wegen „{title}“ eine Terminbasislinie erstellen? Für diese Änderung ist nur eine möglich, und sie lässt sich nicht rückgängig machen.",
  "p.plan_change_requests.baseline_taken":
    "Basislinie Version {version} wurde wegen dieser Änderung erstellt.",
  "p.plan_change_requests.stated_impact":
    "Von der antragstellenden Person angegebene Auswirkung: {impact}.",
  "p.plan_change_requests.implemented_on": "Am {date} umgesetzt.",
  "p.plan_change_requests.implement": "Als umgesetzt markieren",
  "p.plan_change_requests.implemented":
    "Die Änderung wurde als umgesetzt markiert.",
  "p.plan_change_requests.links_heading":
    "Probleme und Risiken, auf die diese Änderung antwortet",
  "p.plan_change_requests.links_none":
    "Sie antwortet noch auf kein Problem und kein Risiko.",
  "p.plan_change_requests.answers_issue": "Problem: {title}",
  "p.plan_change_requests.answers_risk": "Risiko: {title}",
  "p.plan_change_requests.an_issue": "ein nicht mehr aufgeführtes Problem",
  "p.plan_change_requests.a_risk": "ein nicht mehr aufgeführtes Risiko",
  "p.plan_change_requests.links_says":
    "Auswählbar, solange der Antrag offen ist (Entwurf, eingereicht, in Prüfung oder genehmigt). Ein Problem gehört zu höchstens einem Änderungsantrag; bis zu {max} Probleme und {max} Risiken. Speichern ersetzt die gesamte Auswahl.",
  "p.plan_change_requests.links_issues": "Probleme dieses Plans",
  "p.plan_change_requests.issue_elsewhere":
    "{title} (gehört zu einem anderen Änderungsantrag)",
  "p.plan_change_requests.links_no_issues": "Dieser Plan hat keine Probleme.",
  "p.plan_change_requests.links_risks": "Risiken dieses Plans",
  "p.plan_change_requests.links_no_risks": "Dieser Plan hat keine Risiken.",
  "p.plan_change_requests.links_save": "Zuordnung speichern",
  "p.plan_change_requests.links_closed":
    "Ein geschlossener Antrag kann nicht neu verknüpft werden.",
  "p.plan_change_requests.forecast_heading": "Was-wäre-wenn-Prognose",
  "p.plan_change_requests.forecast_says":
    "Eine Prognose aus dem aktuellen Terminplan mit den Was-wäre-wenn-Änderungen dieses Antrags. Das Lesen ändert nichts; die oben angegebene Auswirkung ist die eigene Zahl der antragstellenden Person.",
  "p.plan_change_requests.forecast_show": "Prognose anzeigen",
  "p.plan_change_requests.forecast_label": "Prognose – nichts wurde geändert",
  "p.plan_change_requests.forecast_finish":
    "Planende: jetzt {before}, mit der Änderung {after}.",
  "p.plan_change_requests.forecast_finish_unknown":
    "Das Planende lässt sich nicht prognostizieren.",
  "p.plan_change_requests.forecast_moved_one":
    "1 Aufgabe würde an einem anderen Tag enden.",
  "p.plan_change_requests.forecast_moved_other":
    "{n} Aufgaben würden an einem anderen Tag enden.",
  "p.plan_change_requests.forecast_later_one":
    "{title}: endet 1 Arbeitstag später",
  "p.plan_change_requests.forecast_later_other":
    "{title}: endet {n} Arbeitstage später",
  "p.plan_change_requests.forecast_earlier_one":
    "{title}: endet 1 Arbeitstag früher",
  "p.plan_change_requests.forecast_earlier_other":
    "{title}: endet {n} Arbeitstage früher",
  "p.plan_change_requests.a_task": "Eine Aufgabe",
  "p.plan_change_requests.forecast_labour":
    "Die geplanten Arbeitskosten würden sich um {amount} ändern.",
  "p.plan_change_requests.forecast_labour_hidden":
    "Die Arbeitskostenprognose sehen nur Personen, die Kostensätze sehen dürfen.",
  "p.plan_change_requests.forecast_labour_none":
    "Keine bewertete Arbeit, daher keine Arbeitskostenprognose.",
  "p.plan_change_requests.forecast_labour_unknown":
    "Keine Arbeitskostenprognose.",
  "p.plan_change_requests.forecast_no_edits":
    "Dieser Antrag enthält keine Was-wäre-wenn-Änderungen, daher gibt es keine Prognose; die angegebene Auswirkung ist die eigene Zahl der antragstellenden Person.",
  "p.plan_change_requests.baseline_heading":
    "Eine Basislinie wegen dieser Änderung",
  "p.plan_change_requests.baseline_done":
    "Wegen dieser Änderung wurde eine Terminbasislinie erstellt.",
  "p.plan_change_requests.baseline_says":
    "Eine zu erstellen ist die Entscheidung einer Person; nichts erhält automatisch eine neue Basislinie. Eine Basislinie pro Änderung, ihr Grund nennt den Antrag, und sie lässt sich nicht rückgängig machen.",
  "p.plan_change_requests.baseline_take": "Basislinie erstellen",
  "p.plan_change_requests.baseline_not_yet":
    "Eine Basislinie kann erstellt werden, sobald die Änderung genehmigt ist.",
  "p.escalations.problem_outcome":
    "Wählen Sie ein Ergebnis: entschieden, Orientierung gegeben oder abgelehnt.",
  "p.escalations.problem_response":
    "Schreiben Sie eine Antwort mit {min} bis {max} Zeichen.",
  "p.escalations.refused_ack_right":
    "Abgelehnt: Das Bestätigen einer Eskalation erfordert das Recht, Eskalationen zu bestätigen (eine Administratorin oder ein Administrator hat es). Es wurde nichts erfasst.",
  "p.escalations.refused_resolve_right":
    "Abgelehnt: Das Lösen einer Eskalation erfordert das Recht, Eskalationen zu lösen (eine Administratorin oder ein Administrator hat es). Es wurde nichts erfasst.",
  "p.escalations.refused_ack_state":
    "Abgelehnt: Diese Eskalation wurde inzwischen gelöst oder zurückgezogen und kann daher nicht bestätigt werden. Sie wurde neu gelesen.",
  "p.escalations.refused_resolve_state":
    "Abgelehnt: Diese Eskalation wurde zurückgezogen oder ist bereits mit einem anderen Ergebnis oder einer anderen Antwort gelöst; eine Lösung wird nie überschrieben. Sie wurde neu gelesen.",
  "p.escalations.refused_resolve_invalid":
    "Abgelehnt: Das Ergebnis oder die Antwort wurde nicht angenommen. Prüfen Sie die Regeln oben und versuchen Sie es erneut.",
  "p.escalations.refused_gone":
    "Diese Eskalation gibt es nicht mehr, oder Sie können sie nicht mehr sehen. Die Liste wurde neu gelesen.",
  "p.escalations.could_not_save": "Das wurde nicht erfasst.",
  "p.escalations.by_unrecorded": "eine nicht erfasste Person",
  "p.escalations.ack_record": "Bestätigt auf Stufe {level} von {by}, {at}.",
  "p.escalations.resolution_record": "Gelöst {at} von {by}: {outcome}.",
  "p.escalations.outcome_unrecorded": "kein Ergebnis erfasst",
  "p.escalations.records_heading": "Bestätigung und Lösung",
  "p.escalations.records_failed":
    "Die Bestätigung oder die Lösung konnte nicht gelesen werden; der Verlauf unten zeigt dennoch, was geschehen ist.",
  "p.escalations.ack_note_line": "Notiz: {note}",
  "p.escalations.ack_none": "Auf der aktuellen Stufe nicht bestätigt.",
  "p.escalations.resolution_kept":
    "Eine Lösung ist ein Nachweis: Sie wird weder geändert noch überschrieben.",
  "p.escalations.records_rules":
    "Das Bestätigen erfordert das Recht, Eskalationen zu bestätigen, das Lösen das Recht, sie zu lösen; Administratoren haben beide, und diese Seite kann vorher nicht erkennen, ob Sie sie haben. Eine Bestätigung gilt auf der aktuellen Stufe: Wird die Eskalation eine Stufe höher gegeben, entfällt sie. Eine Lösung ist ein Nachweis und wird nie überschrieben.",
  "p.escalations.ack_heading": "Diese Eskalation bestätigen",
  "p.escalations.ack_note": "Notiz (optional)",
  "p.escalations.ack_done": "Bestätigt; der Nachweis steht unten.",
  "p.escalations.resolve_heading": "Diese Eskalation lösen",
  "p.escalations.resolve_says":
    "Die Antwort hat {min} bis {max} Zeichen. Sie werden vor dem Erfassen gefragt, und danach kann sie nicht mehr geändert werden.",
  "p.escalations.confirm_resolve":
    "Diese Lösung von „{title}“ erfassen? Sie kann danach weder geändert noch überschrieben werden.",
  "p.escalations.resolution_done": "Gelöst; der Nachweis steht unten.",
  "p.register_completeness.link": "Vollständigkeit des Registers",
  "p.register_completeness.title": "Vollständigkeit des Registers",
  "p.register_completeness.title_ppm": "Vollständigkeit des Registers — PPM",
  "p.register_completeness.intro":
    "Wie vollständig das Register jedes Programms ist: von den Angaben, die ein Programm erfasst haben sollte, der Anteil, den es hat, und jede fehlende Angabe mit Namen. Aufgeführt werden nur die Programme (Pläne der Art Programm), die Sie lesen dürfen.",
  "p.register_completeness.rules":
    "Die zehn: eine verantwortliche Führungskraft (die Leitung des Programms steht dafür), Partner, eine Phase, ein Terminplan-Basisplan, ein Zieldatum, ein Budget, eine Finanzierungsquelle für jede Budgetzeile, eine Bewertung der Liefersicherheit, Risiken und Abhängigkeiten. Partner werden nur verlangt, wo das Liefermodell Partner hat, und eine Finanzierungsquelle nur, wo es eine Budgetzeile gibt. Das Programm und die Pläne darunter zählen zusammen.",
  "p.register_completeness.switch_on":
    "„Kein Register, kein Geld“ ist eingeschaltet: Das Freigeben einer Budgetzeile oder das Buchen einer festen Zuteilung für ein unvollständiges Programm wird abgelehnt, mit Nennung der fehlenden Angaben.",
  "p.register_completeness.switch_off":
    "„Kein Register, kein Geld“ ist ausgeschaltet: Das Freigeben einer Budgetzeile oder das Buchen einer festen Zuteilung für ein unvollständiges Programm wird ausgeführt, und die Warnung wird im Prüfprotokoll festgehalten.",
  "p.register_completeness.count_one":
    "{complete} von {n} Programm ist vollständig.",
  "p.register_completeness.count_other":
    "{complete} von {n} Programmen sind vollständig.",
  "p.register_completeness.none":
    "Es gibt kein Programm, das Sie lesen dürfen.",
  "p.register_completeness.loading": "Wird geladen…",
  "p.register_completeness.could_not_load":
    "Die Vollständigkeit des Registers konnte nicht geladen werden.",
  "p.register_completeness.table": "Programme und ihre Register",
  "p.register_completeness.col_programme": "Programm",
  "p.register_completeness.col_state": "Zustand",
  "p.register_completeness.col_score": "Anteil erfasst",
  "p.register_completeness.col_on_record": "Erfasste Angaben",
  "p.register_completeness.col_missing": "Fehlt",
  "p.register_completeness.col_not_asked": "Nicht verlangt",
  "p.register_completeness.of": "{present} von {applicable}",
  "p.register_completeness.state_complete": "vollständig",
  "p.register_completeness.state_incomplete": "unvollständig",
  "p.register_completeness.field_other": "eine weitere Angabe",
  "p.register_completeness.none_listed": "keine",
  "p.meetings.title": "Besprechungen",
  "p.meetings.title_ppm": "Besprechungen — PPM",
  "p.meetings.link": "Besprechungen, Protokolle und Aktionen",
  "p.meetings.intro":
    "Eine Besprechung wird mit Titel, Datum und Tagesordnung geplant. Danach wird sie einmalig als abgehalten erfasst, mit ihrem Protokoll und ihren Ergebnissen (Entscheidungen, Aktionen und Notizen), oder sie wird abgesagt. Eine abgehaltene oder abgesagte Besprechung ist ein Datensatz: Ein abgehaltenes Protokoll kann nicht korrigiert und eine abgesagte Besprechung nicht wiederhergestellt werden. Teilnehmende werden nicht erfasst.",
  "p.meetings.plan_heading": "Besprechung planen",
  "p.meetings.plan_says":
    "Ein Titel mit bis zu {title} Zeichen und ein Datum. Die Tagesordnung enthält bis zu {items} Punkte mit je bis zu {chars} Zeichen; ein Punkt kann auf eine Aufgabe, ein Risiko, einen Meilenstein oder einen Vorgang dieses Plans verweisen.",
  "p.meetings.f_title": "Titel (bis zu {max} Zeichen)",
  "p.meetings.f_date": "Geplantes Datum",
  "p.meetings.agenda_item": "Tagesordnungspunkt {n}",
  "p.meetings.f_item_text":
    "Worum es im Tagesordnungspunkt {n} geht (bis zu {max} Zeichen)",
  "p.meetings.f_item_link":
    "Der Datensatz, den Tagesordnungspunkt {n} betrifft",
  "p.meetings.no_record": "Kein Datensatz",
  "p.meetings.records_loading": "Die Datensätze des Plans werden geladen…",
  "p.meetings.remove_item": "Tagesordnungspunkt {n} entfernen",
  "p.meetings.add_item": "Tagesordnungspunkt hinzufügen",
  "p.meetings.plan": "Besprechung planen",
  "p.meetings.planned": "Geplant: {title}, am {date}.",
  "p.meetings.list_heading": "Die Besprechungen des Plans, die neuesten zuerst",
  "p.meetings.pdf_says":
    "Jede Besprechung wird als PDF-Dokument mit dem heruntergeladen, was diese Seite zeigt; die festen Wörter darin (die Überschriften und der Status) sind englisch.",
  "p.meetings.loading": "Die Besprechungen werden geladen…",
  "p.meetings.none": "Noch keine Besprechungen.",
  "p.meetings.status_planned": "geplant",
  "p.meetings.status_held": "abgehalten",
  "p.meetings.status_cancelled": "abgesagt",
  "p.meetings.held_on": "abgehalten am {date}",
  "p.meetings.scheduled_on": "geplant für {date}",
  "p.meetings.show": "Besprechung {title} öffnen",
  "p.meetings.hide": "Besprechung {title} schließen",
  "p.meetings.pdf": "Als PDF herunterladen",
  "p.meetings.pdf_named": "Besprechung {title} als PDF herunterladen",
  "p.meetings.agenda_heading": "Tagesordnung",
  "p.meetings.agenda_none": "Keine Tagesordnungspunkte.",
  "p.meetings.minutes_heading": "Protokoll",
  "p.meetings.outcomes_heading": "Ergebnisse",
  "p.meetings.outcomes_none": "Es wurden keine Ergebnisse erfasst.",
  "p.meetings.outcome_decision": "Entscheidung",
  "p.meetings.outcome_action": "Aktion",
  "p.meetings.outcome_note": "Notiz",
  "p.meetings.about": "zu {record}",
  "p.meetings.link_gone": "zu einem Datensatz, den es nicht mehr gibt",
  "p.meetings.task_made": "die aus Ergebnis {n} erstellte Aufgabe",
  "p.meetings.held_kept":
    "Diese Besprechung ist ein Datensatz: Ihr Protokoll und ihre Ergebnisse werden nicht geändert.",
  "p.meetings.cancelled_kept":
    "Diese Besprechung wurde abgesagt. Sie ist ein Datensatz und kann nicht wiederhergestellt werden.",
  "p.meetings.held_heading": "{title} als abgehalten erfassen",
  "p.meetings.held_says":
    "Ein Protokoll mit bis zu {minutes} Zeichen und bis zu {items} Ergebnisse mit je bis zu {chars} Zeichen: jeweils eine Entscheidung, eine Aktion oder eine Notiz, und jedes kann auf einen Datensatz dieses Plans verweisen. Als Datum der Besprechung gilt das geplante Datum, sofern Sie kein anderes angeben. Das geschieht nur einmal: Das Protokoll kann danach nicht korrigiert werden.",
  "p.meetings.f_minutes": "Protokoll (bis zu {max} Zeichen)",
  "p.meetings.f_held_on":
    "Datum, an dem sie abgehalten wurde (leer für das geplante Datum)",
  "p.meetings.outcome_n": "Ergebnis {n}",
  "p.meetings.f_outcome_kind": "Was Ergebnis {n} ist",
  "p.meetings.choose_kind": "Auswählen…",
  "p.meetings.f_outcome_text": "Ergebnis {n} in Worten (bis zu {max} Zeichen)",
  "p.meetings.f_outcome_link": "Der Datensatz, den Ergebnis {n} betrifft",
  "p.meetings.remove_outcome": "Ergebnis {n} entfernen",
  "p.meetings.add_outcome": "Ergebnis hinzufügen",
  "p.meetings.record_held": "Als abgehalten erfassen",
  "p.meetings.cancel_meeting": "Besprechung {title} absagen",
  "p.meetings.confirm_held":
    "{title} als abgehalten erfassen? Protokoll und Ergebnisse können danach nicht korrigiert werden.",
  "p.meetings.confirm_cancel":
    "{title} absagen? Eine abgesagte Besprechung kann nicht wiederhergestellt werden.",
  "p.meetings.held_done": "Als abgehalten erfasst: {title}.",
  "p.meetings.cancelled_done": "Abgesagt: {title}.",
  "p.meetings.problem_title":
    "Geben Sie einen Titel mit bis zu {title} Zeichen an.",
  "p.meetings.problem_date": "Geben Sie ein Datum an.",
  "p.meetings.problem_agenda_count":
    "Eine Tagesordnung enthält höchstens {items} Punkte.",
  "p.meetings.problem_outcome_count":
    "Eine Besprechung enthält höchstens {items} Ergebnisse.",
  "p.meetings.problem_item_text":
    "Schreiben Sie es in höchstens {chars} Zeichen.",
  "p.meetings.problem_minutes":
    "Schreiben Sie das Protokoll in höchstens {minutes} Zeichen.",
  "p.meetings.problem_outcome_kind":
    "Wählen Sie eine Entscheidung, eine Aktion oder eine Notiz.",
  "p.meetings.problem_on_line": "Bei {line}: {problem}",
  "p.meetings.could_not_load":
    "Die Besprechungen konnten nicht geladen werden.",
  "p.meetings.could_not_save": "Die Besprechung wurde nicht geändert.",
  "p.meetings.refused_gone":
    "Abgelehnt: Diese Besprechung oder der Plan existiert nicht mehr. Die Liste wurde neu gelesen.",
  "p.meetings.refused_settled":
    "Abgelehnt: Diese Besprechung wurde bereits abgehalten oder abgesagt, ist also ein Datensatz und wird nicht geändert. Sie wurde neu gelesen.",
  "p.meetings.refused_link":
    "Abgelehnt: Ein Datensatz, auf den ein Punkt verweist, gehört nicht mehr zu diesem Plan. Es wurde nichts gespeichert; wählen Sie die Datensätze erneut aus.",
  "p.meetings.refused_not_held":
    "Abgelehnt: Aufgaben werden nur aus einer Besprechung vorgeschlagen, die abgehalten wurde und ein Protokoll hat.",
  "p.meetings.refused_commit_state":
    "Abgelehnt: Die Besprechung wurde nicht abgehalten, oder zu einer dieser Aktionen gibt es inzwischen eine Aufgabe. Es wurde nichts erstellt; die Besprechung wurde neu gelesen, prüfen Sie also erneut.",
  "p.meetings.refused_commit_invalid":
    "Abgelehnt: Ein Vorschlag ist nicht mehr bereit. Es wurde nichts erstellt; prüfen Sie die Vorschläge erneut.",
  "p.meetings.refused_proposals":
    "Abgelehnt: Senden Sie zwischen 1 und {count} Vorschläge.",
  "p.meetings.tasks_heading": "Aufgaben aus den Aktionen",
  "p.meetings.tasks_says":
    "Für jede Aktion ohne Aufgabe können Sie eine vorschlagen: einen Titel mit bis zu {max} Zeichen und eine Textstelle aus dem Protokoll, die ihn stützt, mit mindestens {min} Zeichen (Groß- und Kleinschreibung sowie Abstände werden ignoriert). Eine zuständige Person ist optional und muss bereits zu den Aufgaben dieses Plans oder seiner Verantwortungsmatrix gehören, als Verweis wie person:…; ein Fälligkeitsdatum ist optional, von {first} bis {last}; die Punkte reichen von 0 bis {points}. Bis zu {count} auf einmal. Beim Prüfen wird nichts geschrieben; die Aufgaben werden danach ganz oder gar nicht erstellt, und eine Aktion erhält nur eine Aufgabe.",
  "p.meetings.proposal_legend": "Aktion {n}: {text}",
  "p.meetings.f_propose": "Aufgabe für Aktion {n} vorschlagen",
  "p.meetings.f_task_title":
    "Aufgabentitel für Aktion {n} (bis zu {max} Zeichen)",
  "p.meetings.f_excerpt":
    "Textstelle aus dem Protokoll für Aktion {n} (mindestens {min} Zeichen)",
  "p.meetings.f_assignee":
    "Zuständige Person für Aktion {n} (optionaler Verweis)",
  "p.meetings.f_due": "Fälligkeitsdatum für Aktion {n} (optional)",
  "p.meetings.f_points": "Punkte für Aktion {n} (optional, 0 bis {max})",
  "p.meetings.check": "Vorschläge prüfen",
  "p.meetings.create_tasks": "Aufgaben erstellen",
  "p.meetings.none_chosen":
    "Es ist kein Vorschlag ausgewählt, daher wurde nichts gesendet.",
  "p.meetings.checked_one":
    "Geprüft, nichts geschrieben: {n} Vorschlag ist bereit, {dropped} nicht.",
  "p.meetings.checked_other":
    "Geprüft, nichts geschrieben: {n} Vorschläge sind bereit, {dropped} nicht.",
  "p.meetings.ready": "Bereit, ohne zuständige Person.",
  "p.meetings.ready_for": "Bereit, für {who}.",
  "p.meetings.not_all_ready":
    "Jeder ausgewählte Vorschlag muss bereit sein, bevor die Aufgaben erstellt werden können: Ändern Sie die nicht bereiten oder entfernen Sie deren Häkchen, und prüfen Sie erneut.",
  "p.meetings.check_again":
    "Die Vorschläge haben sich seit der Prüfung geändert: Prüfen Sie sie erneut.",
  "p.meetings.confirm_tasks_one":
    "{n} Aufgabe aus den Aktionen von {title} erstellen? Eine Aktion erhält nur eine Aufgabe.",
  "p.meetings.confirm_tasks_other":
    "{n} Aufgaben aus den Aktionen von {title} erstellen? Eine Aktion erhält nur eine Aufgabe.",
  "p.meetings.tasks_made_one":
    "{n} Aufgabe erstellt; sie ist unter ihrer Aktion verknüpft.",
  "p.meetings.tasks_made_other":
    "{n} Aufgaben erstellt; jede ist unter ihrer Aktion verknüpft.",
  "p.meetings.could_not_check": "Die Vorschläge konnten nicht geprüft werden.",
  "p.meetings.could_not_create": "Die Aufgaben wurden nicht erstellt.",
  "p.meetings.dropped_title_invalid":
    "Nicht bereit: Der Titel ist leer oder zu lang.",
  "p.meetings.dropped_excerpt_not_in_minutes":
    "Nicht bereit: Die Textstelle ist im Protokoll nicht zu finden oder zu kurz.",
  "p.meetings.dropped_outcome_not_found":
    "Nicht bereit: Die Besprechung hat kein solches Ergebnis.",
  "p.meetings.dropped_outcome_not_an_action":
    "Nicht bereit: Dieses Ergebnis ist keine Aktion.",
  "p.meetings.dropped_outcome_already_has_a_task":
    "Nicht bereit: Zu dieser Aktion gibt es bereits eine Aufgabe.",
  "p.meetings.dropped_due_outside_window":
    "Nicht bereit: Das Fälligkeitsdatum liegt außerhalb der zulässigen Daten.",
  "p.meetings.dropped_points_invalid":
    "Nicht bereit: Die Punkte liegen außerhalb des zulässigen Bereichs.",
  "p.meetings.dropped_assignee_not_in_plan":
    "Nicht bereit: Die zuständige Person gehört nicht zu den Personen dieses Plans (in seinen Aufgaben oder seiner Verantwortungsmatrix).",
  "p.meetings.dropped_outcome_proposed_twice":
    "Nicht bereit: Dieselbe Aktion wird zweimal vorgeschlagen.",
  "p.meetings.dropped_other":
    "Nicht bereit, aus einem Grund, den diese Seite nicht kennt.",
  "p.meetings.problem_proposals_count":
    "Schlagen Sie höchstens {count} Aufgaben auf einmal vor.",
  "p.meetings.problem_task_title":
    "Geben Sie der Aufgabe einen Titel mit bis zu {max} Zeichen.",
  "p.meetings.problem_excerpt":
    "Zitieren Sie eine Textstelle mit mindestens {min} Zeichen, die im Protokoll steht.",
  "p.meetings.problem_due":
    "Das Fälligkeitsdatum muss zwischen einer Woche vor der Besprechung und einem Jahr danach liegen.",
  "p.meetings.problem_points":
    "Punkte sind eine ganze Zahl von 0 bis {points}.",
  "p.lessons.title": "Gelernte Lektionen",
  "p.lessons.title_ppm": "Gelernte Lektionen — PPM",
  "p.lessons.link": "Gelernte Lektionen",
  "p.lessons.intro":
    "Eine Lektion wird an einem Phasen-Gate oder beim Abschluss des Plans erfasst. Sie wird nie geändert: Sie kann nur zurückgezogen werden, und eine zurückgezogene Lektion wird nicht mehr angezeigt (sie bleibt im Audit-Protokoll). Bei einem neuen Vorschlag, dessen Titel dem Titel dieses Plans ähnelt, werden dessen Lektionen angezeigt.",
  "p.lessons.record_heading": "Lektion erfassen",
  "p.lessons.record_says":
    "Wann (ein Gate oder der Abschluss), was es ist, eine Zusammenfassung mit bis zu {max} Zeichen und, wenn Sie möchten, ein Detail mit bis zu {bytes} Bytes (ein Buchstabe mit Akzent oder eine andere Schrift braucht mehr als eines) sowie die Aufgabe, das Risiko, den Meilenstein oder den Vorgang dieses Plans, um den es geht.",
  "p.lessons.f_occasion": "Wann",
  "p.lessons.choose_occasion": "Gate oder Abschluss auswählen…",
  "p.lessons.closure": "Abschluss",
  "p.lessons.occasion_other": "Ein anderer Anlass",
  "p.lessons.f_category": "Was es ist",
  "p.lessons.category_went_well": "Lief gut",
  "p.lessons.category_went_badly": "Lief schlecht",
  "p.lessons.category_recommendation": "Empfehlung",
  "p.lessons.category_other": "Eine andere Art",
  "p.lessons.f_summary": "Zusammenfassung (bis zu {max} Zeichen)",
  "p.lessons.f_detail": "Detail (optional, bis zu {bytes} Bytes)",
  "p.lessons.f_link": "Der Datensatz, um den es geht (optional)",
  "p.lessons.record": "Lektion erfassen",
  "p.lessons.recorded": "Erfasst: {summary}",
  "p.lessons.problem_occasion": "Wählen Sie ein Gate oder den Abschluss.",
  "p.lessons.problem_category":
    "Wählen Sie lief gut, lief schlecht oder Empfehlung.",
  "p.lessons.problem_summary":
    "Schreiben Sie eine Zusammenfassung mit bis zu {max} Zeichen.",
  "p.lessons.problem_detail": "Das Detail ist länger als {bytes} Bytes.",
  "p.lessons.list_heading": "Die Lektionen des Plans, die ältesten zuerst",
  "p.lessons.loading": "Die Lektionen werden geladen…",
  "p.lessons.none": "Noch keine Lektionen.",
  "p.lessons.count_one": "{n} Lektion.",
  "p.lessons.count_other": "{n} Lektionen.",
  "p.lessons.recorded_on": "erfasst am {date}",
  "p.lessons.recorded_on_by": "erfasst am {date} von {who}",
  "p.lessons.link_unlisted":
    "zu einem Datensatz, der nicht zu den hier aufgeführten Aufgaben, Risiken, Meilensteinen und Vorgängen des Plans gehört",
  "p.lessons.withdraw": "Lektion {summary} zurückziehen",
  "p.lessons.withdraw_short": "Zurückziehen",
  "p.lessons.confirm_withdraw":
    "Lektion „{summary}“ zurückziehen? Sie wird weder hier noch bei neuen Vorschlägen mehr angezeigt; sie bleibt im Audit-Protokoll.",
  "p.lessons.withdrawn": "Zurückgezogen: {summary}",
  "p.lessons.could_not_load": "Die Lektionen konnten nicht geladen werden.",
  "p.lessons.could_not_save": "Die Lektion wurde nicht geändert.",
  "p.lessons.refused_gone": "Abgelehnt: Der Plan existiert nicht mehr.",
  "p.lessons.refused_withdrawn":
    "Abgelehnt: Diese Lektion wurde bereits zurückgezogen. Die Liste wurde neu gelesen.",
  "p.lessons.refused_link":
    "Abgelehnt: Der gewählte Datensatz gehört nicht mehr zu diesem Plan. Es wurde nichts erfasst; wählen Sie ihn erneut aus.",
  "p.lessons.refs_heading": "Was auf einen Datensatz verweist",
  "p.lessons.refs_says":
    "Wählen Sie eine Aufgabe, ein Risiko, einen Meilenstein oder einen Vorgang des Plans, um die Besprechungspunkte und Lektionen aufzulisten, die darauf verweisen (jeweils bis zu 100).",
  "p.lessons.f_target": "Der Datensatz",
  "p.lessons.choose_target": "Datensatz auswählen…",
  "p.lessons.show_refs": "Anzeigen, was darauf verweist",
  "p.lessons.problem_target": "Wählen Sie zuerst einen Datensatz.",
  "p.lessons.refs_count_one": "{n} Eintrag verweist darauf.",
  "p.lessons.refs_count_other": "{n} Einträge verweisen darauf.",
  "p.lessons.refs_meetings": "Besprechungspunkte",
  "p.lessons.refs_lessons": "Lektionen, die darauf verweisen",
  "p.lessons.refs_agenda": "Tagesordnung",
  "p.lessons.refused_no_record":
    "Dieser Datensatz gehört nicht mehr zu diesem Plan, oder Sie können ihn nicht lesen.",
  "p.lessons.could_not_read_refs":
    "Es konnte nicht gelesen werden, was darauf verweist.",
  "p.lessons.similar": "Lektionen aus ähnlichen Plänen",
  "p.lessons.similar_named": "Lektionen aus Plänen wie {title}",
  "p.lessons.similar_heading": "Lektionen aus Plänen wie {title}",
  "p.lessons.similar_says":
    "Abgeglichen wird nur der Titel, gegen jeden Plan mit Lektionen, abgeschlossen oder nicht, die beste Übereinstimmung zuerst. Die Bewertung und die Zuverlässigkeit stammen vom Abgleich.",
  "p.lessons.similar_none": "Kein ähnlicher Plan hat Lektionen.",
  "p.lessons.match": "Übereinstimmung {score}, {confidence} Zuverlässigkeit.",
  "p.lessons.match_title": "Titel zu {score} ähnlich.",
  "p.lessons.confidence_high": "hohe",
  "p.lessons.confidence_medium": "mittlere",
  "p.lessons.confidence_low": "geringe",
  "p.lessons.refused_proposal_gone":
    "Dieser Vorschlag existiert nicht mehr, oder Sie können ihn nicht lesen.",
  "p.lessons.could_not_read_similar":
    "Die Lektionen ähnlicher Pläne konnten nicht gelesen werden.",
  "p.director_pack.open_by_severity": "Offen nach Schweregrad: {list}",
  "p.director_pack.issues_overdue": "Überfällige Probleme: {n}",
  "p.director_pack.issues_in_window":
    "Im Zeitraum: {raised} gemeldet, {resolved} gelöst",
  "p.director_pack.oldest_issue":
    "Ältestes offenes Problem: {title} (Kalendertage offen: {n})",
  "p.director_pack.approved_not_implemented":
    "Genehmigt, aber noch nicht umgesetzt: {n}",
  "p.director_pack.decided_in_window": "Im Zeitraum entschieden: {n}",
  "p.director_pack.changes_by_status": "Nach Status: {list}",
  "p.director_pack.oldest_change":
    "Älteste offene Änderungsanforderung: {title} (Kalendertage offen: {n})",
  "p.live_pack.heading": "Programmwerte, jetzt",
  "p.live_pack.says":
    "Jetzt aus den Plänen gelesen, die Sie lesen dürfen, dargestellt wie im Direktorenbericht. Ein herausgegebener Bericht behält seine Zahlen; der vorherige Punkt im Fieberdiagramm stammt nur aus einem herausgegebenen Bericht.",
  "p.live_pack.f_ac": "Istkosten für den Fertigstellungswert",
  "p.live_pack.ac_budget_lines": "Budgetposten (Standard)",
  "p.live_pack.ac_labour": "Erfasste Arbeitskosten",
  "p.live_pack.ac_both": "Budgetposten und Arbeitskosten",
  "p.live_pack.f_window":
    "Probleme und Änderungsanforderungen der letzten Tage ({min} bis {max}; leer für 30)",
  "p.live_pack.read_again": "Erneut lesen",
  "p.live_pack.problem_window":
    "Die Anzahl der Tage muss eine ganze Zahl von {min} bis {max} sein, oder leer für 30.",
  "p.live_pack.could_not_read_fever":
    "Das Puffer-Fieberdiagramm konnte nicht gelesen werden.",
  "p.live_pack.could_not_read_evm":
    "Der Fertigstellungswert konnte nicht gelesen werden.",
  "p.live_pack.could_not_read_benefits":
    "Die Nutzen konnten nicht gelesen werden.",
  "p.live_pack.could_not_read_issues":
    "Die Probleme und Änderungsanforderungen konnten nicht gelesen werden.",
  "p.live_pack.issues_heading": "Probleme und Änderungsanforderungen",
  "p.live_pack.issues_none":
    "Sie können keinen Plan mit Problemen oder Änderungsanforderungen lesen.",
  "p.live_pack.nothing": "Nichts anzuzeigen.",
  "p.forecast.heading": "Prognose bei Fertigstellung",
  "p.forecast.says":
    "Die Schätzung bei Fertigstellung ist die bisherige Ist-Kosten plus die Restschätzung. Die Restschätzung ist der neueste Wert der Gesamtprojektsteuerung in der Währung der Prognose, sonst das, was die neueste Basislinie noch plant. Jetzt gelesen; nichts wird gespeichert.",
  "p.forecast.absent_no_currency":
    "Keine Prognose: Der Plan hat keine Budget-Basislinie und keinen Wert der Gesamtprojektsteuerung, also keine Währung für eine Prognose.",
  "p.forecast.c_actual": "Bisherige Ist-Kosten",
  "p.forecast.c_etc": "Restschätzung",
  "p.forecast.c_eac": "Schätzung bei Fertigstellung",
  "p.forecast.c_plans": "Pläne",
  "p.forecast.not_known": "nicht bekannt",
  "p.forecast.etc_tpc": "aus dem neuesten Wert der Gesamtprojektsteuerung",
  "p.forecast.etc_baseline": "aus dem, was die Basislinie noch plant",
  "p.forecast.no_baseline_used": "Es wurde keine Budget-Basislinie verwendet.",
  "p.forecast.baseline_used": "Verwendet Budget-Basislinie Version {version}.",
  "p.forecast.as_of": "Gelesen am {date}.",
  "p.forecast.other_currency":
    "In einer anderen Währung als {currency} erfasste Ist-Kosten sind nicht eingerechnet.",
  "p.forecast.baselines_heading": "Budget-Basislinien",
  "p.forecast.baselines_says":
    "Eine Basislinie ist das genehmigte Budget nach Zeitraum, in einer Währung. Nach der Genehmigung ist sie eingefroren und wird nie bearbeitet; eine Neufestlegung ist eine neue Version, und jede Version nach der ersten braucht eine Begründung. Eine Basislinie enthält 1 bis {max} Zeiträume, jeder mit einem Ende nicht vor dem Beginn und einem geplanten Betrag nicht unter null.",
  "p.forecast.no_baselines": "Es wurde noch keine Budget-Basislinie genehmigt.",
  "p.forecast.version_line":
    "Version {version} · {currency} · genehmigt am {date}",
  "p.forecast.period_line": "{start} bis {end}: {amount}",
  "p.forecast.first_baseline_heading": "Erste Basislinie genehmigen",
  "p.forecast.rebaseline_heading": "Version {version} genehmigen",
  "p.forecast.f_currency": "Währung (drei Buchstaben)",
  "p.forecast.periods_legend": "Zeiträume",
  "p.forecast.f_start": "Zeitraum {n} beginnt",
  "p.forecast.f_end": "Zeitraum {n} endet",
  "p.forecast.f_amount": "Geplanter Betrag Zeitraum {n}",
  "p.forecast.remove_period": "Zeitraum {n} entfernen",
  "p.forecast.add_period": "Zeitraum hinzufügen",
  "p.forecast.f_reason": "Begründung (für die erste optional)",
  "p.forecast.f_reason_needed":
    "Warum das Budget neu festgelegt wird (erforderlich)",
  "p.forecast.approve": "Basislinie genehmigen",
  "p.forecast.confirm_baseline":
    "Budget-Basislinie Version {version} genehmigen? Nach der Genehmigung ist sie eingefroren und kann weder geändert noch entfernt werden.",
  "p.forecast.baseline_approved": "Version {version} genehmigt.",
  "p.forecast.problem_currency":
    "Die Währung muss ein Code aus drei Buchstaben sein, etwa GBP oder EUR.",
  "p.forecast.problem_no_periods": "Geben Sie mindestens einen Zeitraum an.",
  "p.forecast.problem_too_many":
    "Eine Basislinie enthält höchstens {max} Zeiträume.",
  "p.forecast.problem_dates":
    "Jeder Zeitraum braucht ein Start- und ein Enddatum.",
  "p.forecast.problem_order":
    "Ein Zeitraum kann nicht vor seinem Beginn enden.",
  "p.forecast.problem_amount":
    "Jeder geplante Betrag muss eine Zahl sein, nicht unter null.",
  "p.forecast.problem_reason": "Eine Neufestlegung braucht eine Begründung.",
  "p.forecast.problem_reason_long":
    "Die Begründung ist zu lang: höchstens {bytes} Byte.",
  "p.forecast.problem_same":
    "Das entspricht der neuesten Version; es wurde nichts gesendet.",
  "p.forecast.refused_gone":
    "Dieser Plan existiert nicht mehr, oder Sie können ihn nicht lesen.",
  "p.forecast.refused_baseline":
    "Die Basislinie wurde abgelehnt: Inzwischen wurde möglicherweise eine andere Version genehmigt, sodass eine Begründung nötig ist. Die Liste wurde neu gelesen.",
  "p.forecast.could_not_approve":
    "Die Basislinie konnte nicht genehmigt werden.",
  "p.forecast.could_not_read": "Die Prognose konnte nicht gelesen werden.",
  "p.forecast.could_not_read_baselines":
    "Die Budget-Basislinien konnten nicht gelesen werden.",
  "p.forecast.could_not_read_lines":
    "Die Budgetposten konnten nicht gelesen werden.",
  "p.forecast.tranches_heading": "Bis zu einem Gate zurückgehaltene Mittel",
  "p.forecast.tranches_says":
    "Ein Budgetposten mit Finanzierungs-Gate wird zurückgehalten, bis die Phase des Plans dieses Gate erreicht; dann kann er einmal freigegeben werden, und erst dann lassen sich Ist-Kosten darauf buchen. Eine Freigabe lässt sich nicht rückgängig machen.",
  "p.forecast.stage_now": "Phase des Plans: {stage}.",
  "p.forecast.stage_none": "Der Plan hat noch kein Gate passiert.",
  "p.forecast.no_tranches":
    "Kein Budgetposten wird bis zu einem Gate zurückgehalten.",
  "p.forecast.gate_line": "zurückgehalten bis {gate}",
  "p.forecast.state_released": "freigegeben am {date}",
  "p.forecast.state_held": "noch zurückgehalten: das Gate ist nicht passiert",
  "p.forecast.release": "{description} freigeben",
  "p.forecast.confirm_release":
    "{description} freigeben, zurückgehalten bis {gate}? Eine Freigabe lässt sich nicht rückgängig machen.",
  "p.forecast.released": "{description} freigegeben.",
  "p.forecast.refused_line_gone":
    "Dieser Budgetposten oder Plan existiert nicht mehr.",
  "p.forecast.refused_register":
    "Abgelehnt: Das Register des Programms ist unvollständig. Vervollständigen Sie es zuerst auf der Seite zur Registervollständigkeit.",
  "p.forecast.refused_release":
    "Abgelehnt: Der Posten wird nicht mehr zurückgehalten, oder sein Gate ist nicht passiert. Die Posten wurden neu gelesen.",
  "p.forecast.could_not_release": "Der Posten konnte nicht freigegeben werden.",
  "p.forecast.rollup_heading": "Prognose bei Fertigstellung, zusammengefasst",
  "p.forecast.rollup_says":
    "Von einem Plan über die Pläne darunter, bis zu {max} Ebenen tief, eine Zeile je Währung: Währungen werden nie addiert. Pläne ohne Währung für eine Prognose werden gezählt, nicht addiert.",
  "p.forecast.f_plan": "Ausgangsplan",
  "p.forecast.choose_plan": "Plan wählen",
  "p.forecast.f_depth": "Ebenen tief (1 bis {max}; leer für alle)",
  "p.forecast.show_rollup": "Prognose anzeigen",
  "p.forecast.problem_plan": "Wählen Sie einen Plan.",
  "p.forecast.problem_depth":
    "Die Ebenentiefe muss eine ganze Zahl von 1 bis {max} sein, oder leer.",
  "p.forecast.refused_plan_gone":
    "Dieser Plan existiert nicht mehr, oder Sie dürfen ihn nicht lesen.",
  "p.forecast.could_not_read_rollup":
    "Die zusammengefasste Prognose konnte nicht gelesen werden.",
  "p.forecast.rollup_line_one": "Ab {name}: {n} Plan gelesen, {date}.",
  "p.forecast.rollup_line_other": "Ab {name}: {n} Pläne gelesen, {date}.",
  "p.forecast.rollup_truncated":
    "Der Baum ist größer, als der Dienst durchläuft; die Zahlen decken einen Teil ab.",
  "p.forecast.rollup_revisits_one":
    "{n} Plan wurde zweimal erreicht und einmal gezählt.",
  "p.forecast.rollup_revisits_other":
    "{n} Pläne wurden zweimal erreicht und einmal gezählt.",
  "p.forecast.rollup_no_currency_one":
    "{n} Plan hat keine Währung für eine Prognose und ist nicht eingerechnet.",
  "p.forecast.rollup_no_currency_other":
    "{n} Pläne haben keine Währung für eine Prognose und sind nicht eingerechnet.",
  "p.forecast.rollup_empty": "Kein Plan hier hat eine Prognose.",
  "p.value.heading": "Realisierter Wert",
  "p.value.says":
    "Jetzt aus dem erfassten Wert gelesen; nichts wird gespeichert. Die Kapitalrendite stellt jeden Wertpunkt den erfassten Ist-Kosten gegenüber und entfällt, wenn die Budgetposten mehr als eine Währung verwenden, da keine Währung umgerechnet wird. Eine Zahl, die sich nicht angeben lässt, nennt den Grund, nie null.",
  "p.value.roi": "Kapitalrendite: {pct}",
  "p.value.roi_absent": "Kapitalrendite nicht angegeben: {reason}",
  "p.value.measured_share": "gemessene Belege: {pct} der Wertpunkte",
  "p.value.ttv_one":
    "Zeit bis zum Wert: Median {p50} Kalendertage, 85. Perzentil {p85} ({n} Beobachtung)",
  "p.value.ttv_other":
    "Zeit bis zum Wert: Median {p50} Kalendertage, 85. Perzentil {p85} ({n} Beobachtungen)",
  "p.value.ttv_absent": "Zeit bis zum Wert nicht angegeben: {reason}",
  "p.value.adoption":
    "Nutzung: {pct} ({active} von {target} Nutzern aktiv über {days} Tage)",
  "p.value.adoption_definition": "aktiver Nutzer bedeutet: {definition}",
  "p.value.adoption_absent": "Nutzung nicht angegeben: {reason}",
  "p.value.adoption_none": "Es ist kein Nutzungswert erfasst.",
  "p.value.absent_unrealized": "es ist kein Wert erfasst",
  "p.value.absent_no_denominator":
    "es gibt nichts zum Vergleichen (keine Ist-Kosten oder kein Ziel)",
  "p.value.absent_mixed_currency":
    "die Budgetposten verwenden mehr als eine Währung",
  "p.value.absent_no_baseline": "es gibt keine Budget-Basislinie",
  "p.value.absent_no_responses": "niemand hat geantwortet",
  "p.value.absent_overflow": "die Zahl ist zu groß zum Berechnen",
  "p.value.absent_other": "der Dienst hat keinen Grund genannt",
  "p.value.case_heading": "Ziele des Business Case",
  "p.value.no_targets": "Es ist kein Ziel des Business Case erfasst.",
  "p.value.target_line":
    "{metric}: von {baseline} auf {target} · {source} · genehmigt am {date}",
  "p.value.source_charter": "die Projektcharta",
  "p.value.source_gate_review": "eine Gate-Prüfung",
  "p.value.source_other": "eine andere Quelle",
  "p.value.to_costs": "Ein Ziel auf der Kostenseite erfassen",
  "p.value.point_heading": "Beobachteten Wert erfassen",
  "p.value.point_says":
    "Ein Wert ist gemessen, geschätzt oder behauptet. Mit Währung ist er ein Geldbetrag; ohne, eine ganze Zahl in der Einheit der Kennzahl. Ein erfasster Wertpunkt lässt sich hier nicht ändern. Ein Wertpunkt je Plan kann der erste messbare Wert sein: Er stoppt die Uhr bis zum Wert, die das erste Ziel des Business Case gestartet hat, und lässt sich nicht verschieben.",
  "p.value.f_value": "Wert",
  "p.value.f_currency": "Währung (drei Buchstaben, oder leer)",
  "p.value.f_method": "Wie er ermittelt wurde",
  "p.value.method_measured": "gemessen",
  "p.value.method_estimated": "geschätzt",
  "p.value.method_asserted": "behauptet",
  "p.value.f_evidence": "Beleg (Link oder Verweis, optional)",
  "p.value.f_first": "Dies ist der erste messbare Wert",
  "p.value.record_point": "Wert erfassen",
  "p.value.confirm_first":
    "Dies als ersten messbaren Wert des Plans erfassen? Er stoppt die Uhr bis zum Wert und lässt sich nicht verschieben.",
  "p.value.point_recorded": "Wert erfasst.",
  "p.value.problem_currency":
    "Die Währung muss ein Code aus drei Buchstaben sein, oder leer.",
  "p.value.problem_amount": "Der Wert muss ein Geldbetrag sein.",
  "p.value.problem_whole": "Ohne Währung muss der Wert eine ganze Zahl sein.",
  "p.value.problem_method": "Wählen Sie, wie der Wert ermittelt wurde.",
  "p.value.refused_first_taken":
    "Abgelehnt: Dieser Plan hat bereits einen ersten messbaren Wert; die Uhr stoppt nur einmal.",
  "p.value.refused_value": "Der Wert wurde abgelehnt.",
  "p.value.refused_gone":
    "Dieser Plan existiert nicht mehr, oder Sie haben keinen Lesezugriff.",
  "p.value.could_not_read": "Der realisierte Wert konnte nicht gelesen werden.",
  "p.value.could_not_record_point": "Der Wert konnte nicht erfasst werden.",
  "p.value.case_says":
    "Ein Ziel ist eine in der Projektcharta oder bei einer Gate-Prüfung versprochene Kennzahl, mit ihrem Wert bei Genehmigung und dem versprochenen Wert, als ganze Zahlen in der Einheit der Kennzahl. Das Genehmigungsdatum startet die Uhr bis zum Wert und lässt sich nicht ändern; ein Ziel lässt sich hier weder ändern noch entfernen.",
  "p.value.to_benefits": "Der realisierte Wert steht auf der Nutzenseite.",
  "p.value.f_metric": "Kennzahl",
  "p.value.f_baseline": "Wert bei Genehmigung",
  "p.value.f_target": "Versprochener Wert",
  "p.value.f_unit": "Einheit (optional)",
  "p.value.f_source": "Versprochen in",
  "p.value.record_target": "Ziel erfassen",
  "p.value.confirm_target":
    "Ziel {metric} erfassen? Es lässt sich hier weder ändern noch entfernen.",
  "p.value.target_recorded": "Ziel {metric} erfasst.",
  "p.value.problem_metric": "Nennen Sie die Kennzahl.",
  "p.value.problem_target_values":
    "Der Wert bei Genehmigung und der versprochene Wert müssen ganze Zahlen sein.",
  "p.value.problem_source": "Wählen Sie, wo das Ziel versprochen wurde.",
  "p.value.refused_target": "Das Ziel wurde abgelehnt.",
  "p.value.could_not_record_target": "Das Ziel konnte nicht erfasst werden.",
  "p.scoring.rank_by": "Reihen nach",
  "p.scoring.model_smart": "Smart-Score (aus Belegen)",
  "p.scoring.model_wsjf": "WSJF (gewichtete kürzeste Aufgabe zuerst)",
  "p.scoring.model_rice": "RICE (Reichweite, Wirkung, Zuversicht, Aufwand)",
  "p.scoring.models_say":
    "Der Smart-Score stammt aus den Belegen, die der Dienst hält; WSJF und RICE werden aus den Schätzungen einer Person berechnet, die für jeden Plan unter seiner Schaltfläche „Schätzungen“ gesetzt werden. Ein Plan ohne jede Eingabe, die ein Modell braucht, bleibt ohne Wert und steht zuletzt, nie mit null bewertet. Das Band gehört zum Smart-Score.",
  "p.scoring.of_plans_one":
    "{length} von {scored_of} Plan, gereiht nach {model}",
  "p.scoring.of_plans_other":
    "{length} von {scored_of} Plänen, gereiht nach {model}",
  "p.scoring.ranked_by": "Pläne gereiht nach {model}",
  "p.scoring.waiting_for": "Wartet auf",
  "p.scoring.open": "Schätzungen",
  "p.scoring.open_named": "Die Schätzungen von {name} öffnen",
  "p.scoring.heading": "Schätzungen für {name}",
  "p.scoring.form": "Die Schätzungen von {name}",
  "p.scoring.says":
    "WSJF und RICE werden aus diesen Schätzungen berechnet. Fehlt eine Eingabe, gibt es keinen Wert, nie eine Vermutung; eine angegebene Null ist keine fehlende Eingabe. Speichern ersetzt jede Schätzung: ein leeres Feld löscht sie.",
  "p.scoring.last_set": "Zuletzt gesetzt {when}.",
  "p.scoring.none_set": "Es sind keine Schätzungen gesetzt.",
  "p.scoring.wsjf_inputs":
    "WSJF-Eingaben, jede eine ganze Zahl von {min} bis {max}",
  "p.scoring.rice_inputs": "RICE-Eingaben",
  "p.scoring.input_business_value": "Geschäftswert",
  "p.scoring.input_time_criticality": "Zeitkritikalität",
  "p.scoring.input_risk_reduction": "Risikominderung oder eröffnete Chance",
  "p.scoring.input_job_size": "Umfang der Arbeit",
  "p.scoring.input_reach":
    "Reichweite (Personen oder Ereignisse in einem Zeitraum)",
  "p.scoring.input_impact": "Wirkung",
  "p.scoring.input_confidence": "Zuversicht (Prozent)",
  "p.scoring.input_effort": "Aufwand (Personenmonate)",
  "p.scoring.input_other": "eine andere Eingabe",
  "p.scoring.component_cost_of_delay": "Verzögerungskosten",
  "p.scoring.impact_minimal": "sehr gering",
  "p.scoring.impact_low": "gering",
  "p.scoring.impact_medium": "mittel",
  "p.scoring.impact_high": "hoch",
  "p.scoring.impact_massive": "gewaltig",
  "p.scoring.impact_value": "{value} (Wirkung {word})",
  "p.scoring.not_estimated": "nicht geschätzt",
  "p.scoring.save": "Schätzungen speichern",
  "p.scoring.close": "Schätzungen schließen",
  "p.scoring.unchanged":
    "Die Schätzungen sind wie gespeichert; es wurde nichts gesendet.",
  "p.scoring.saved": "Schätzungen für {name} gespeichert.",
  "p.scoring.could_not_read": "Die Schätzungen konnten nicht gelesen werden.",
  "p.scoring.could_not_save":
    "Die Schätzungen konnten nicht gespeichert werden.",
  "p.scoring.refused_gone":
    "Dieser Plan existiert nicht mehr, oder Sie können ihn nicht lesen.",
  "p.scoring.refused_inputs":
    "Die Schätzungen wurden abgelehnt: eine Eingabe liegt außerhalb ihres Bereichs.",
  "p.scoring.problem_wsjf":
    "{input} muss eine ganze Zahl von {min} bis {max} sein.",
  "p.scoring.problem_reach":
    "{input} muss eine ganze Zahl sein, nicht unter null.",
  "p.scoring.problem_impact": "{input} muss 0,25, 0,5, 1, 2 oder 3 sein.",
  "p.scoring.problem_confidence":
    "{input} muss eine ganze Zahl von 0 bis 100 sein.",
  "p.scoring.problem_effort": "{input} muss über null liegen.",
  "p.scoring.problem_number": "{input} muss eine Zahl sein.",
  "p.scoring.formula_wsjf":
    "WSJF ist (Geschäftswert + Zeitkritikalität + Risikominderung oder eröffnete Chance) ÷ Umfang der Arbeit.",
  "p.scoring.formula_rice":
    "RICE ist Reichweite × Wirkung × Zuversicht ÷ Aufwand.",
  "p.scoring.score_is": "Wert: {score}",
  "p.scoring.no_score": "Kein Wert: er wartet auf diese Eingaben.",
  "p.scoring.needs_delay_part": "sie ist Teil der Verzögerungskosten",
  "p.scoring.needs_divisor":
    "sie ist der Divisor: ohne sie gibt es keinen Wert je Arbeitseinheit",
  "p.scoring.needs_reach":
    "sie sagt, wie viele Personen die Arbeit in einem Zeitraum erreicht",
  "p.scoring.needs_impact":
    "sie sagt, wie viel die Arbeit jeder erreichten Person bedeutet",
  "p.scoring.needs_confidence":
    "sie mindert die Schätzungen danach, wie sicher sie sind",
  "p.scoring.needs_other": "der Wert braucht sie",
  "p.scoring.means_business_value":
    "relativer Wert für die Nutzer oder das Geschäft, 1 bis 20",
  "p.scoring.means_time_criticality":
    "wie schnell der Wert verfällt, wenn es spät wird, 1 bis 20",
  "p.scoring.means_risk_reduction":
    "das Risiko, das es beseitigt, oder die Chance, die es eröffnet, 1 bis 20",
  "p.scoring.means_cost_of_delay":
    "die Summe der drei oben: was eine Verzögerung kostet",
  "p.scoring.means_job_size":
    "der relative Umfang der Arbeit, 1 bis 20; der Divisor",
  "p.scoring.means_reach":
    "in einem Zeitraum erreichte Personen oder Ereignisse",
  "p.scoring.means_impact":
    "0,25 sehr gering, 0,5 gering, 1 mittel, 2 hoch, 3 gewaltig",
  "p.scoring.means_confidence": "wie sicher die Schätzungen sind",
  "p.scoring.means_effort": "Personenmonate Arbeit; der Divisor",
  "p.scoring.means_other": "ein Bestandteil, den diese Seite nicht kennt",
  "p.frontier.heading": "Ein Portfolio wählen",
  "p.frontier.says":
    "Der Generator bewertet jeden Plan, den Sie lesen können, nach seinem Smart-Score und bepreist ihn nach seinen Budgetzeilen; dann nimmt er Pläne nach Wert je Kosteneinheit, bis die Obergrenze erreicht ist. Pläne, die enthalten sein müssen, werden zuerst genommen, was sie auch kosten. Ein Plan ohne Smart-Score bleibt draußen; ein Plan ohne Budgetzeilen kostet nichts. Die Wahl ist nicht immer die bestmögliche, daher wird eine Obergrenze, die nicht mehr Wert kauft als eine günstigere, markiert, nicht versteckt.",
  "p.frontier.must_include":
    "Pläne, die jede Wahl enthalten muss (bis zu {max})",
  "p.frontier.plans_loading": "Die Pläne werden gelesen…",
  "p.frontier.plans_none": "Es gibt keine Pläne, die Sie lesen können.",
  "p.frontier.frontier_heading": "Effizienzgrenze",
  "p.frontier.frontier_says":
    "Der Generator läuft einmal je Obergrenze, und nichts wird gespeichert. Verteilen Sie {min} bis {max} Obergrenzen gleichmäßig von null bis zu den Kosten aller bepreisbaren Pläne (leer für {fallback}), oder nennen Sie selbst bis zu {list} Obergrenzen, jede ein Betrag nicht unter null.",
  "p.frontier.f_currency": "Währung der Kosten (drei Buchstaben)",
  "p.frontier.f_how": "Welche Budgetobergrenzen",
  "p.frontier.how_steps": "Gleichmäßig verteilt",
  "p.frontier.how_caps": "Selbst genannte Obergrenzen",
  "p.frontier.f_steps": "Wie viele Obergrenzen (leer für {fallback})",
  "p.frontier.f_caps": "Budgetobergrenzen, eine je Zeile (bis zu {list})",
  "p.frontier.run": "Die Grenze zeichnen",
  "p.frontier.value_is":
    "Der Wert ist die Summe der Smart-Scores der gewählten Pläne.",
  "p.frontier.table": "Die Grenze, Punkt für Punkt",
  "p.frontier.col_cap": "Budgetobergrenze",
  "p.frontier.col_cost": "Kosten der Wahl",
  "p.frontier.col_value": "Wert",
  "p.frontier.col_included": "Gewählte Pläne",
  "p.frontier.col_more": "Kauft mehr Wert",
  "p.frontier.more_yes": "Ja",
  "p.frontier.more_no": "Nein: eine günstigere Obergrenze erreicht ebenso viel",
  "p.frontier.show_choice": "Die Wahl zeigen",
  "p.frontier.show_choice_named": "Die Wahl bei {cap} zeigen",
  "p.frontier.save_point": "Als Entwurf speichern",
  "p.frontier.save_point_named": "Die Wahl bei {cap} als Entwurf speichern",
  "p.frontier.point_name": "Grenze bei {cap}",
  "p.frontier.axis_cost": "Kosten der gewählten Pläne",
  "p.frontier.axis_value": "Wert (summierter Smart-Score)",
  "p.frontier.chart_points_one": "{n} Budgetobergrenze.",
  "p.frontier.chart_points_other": "{n} Budgetobergrenzen.",
  "p.frontier.chart_best": "Der höchste Wert, {value}, kostet {cost}.",
  "p.frontier.chart_dominated_one":
    "{n} Obergrenze kauft nicht mehr Wert als eine günstigere.",
  "p.frontier.chart_dominated_other":
    "{n} Obergrenzen kaufen nicht mehr Wert als eine günstigere.",
  "p.frontier.legend_efficient": "kauft mehr Wert",
  "p.frontier.legend_dominated":
    "kauft nicht mehr Wert als eine günstigere Obergrenze",
  "p.frontier.no_points": "Die Grenze hat keine Punkte.",
  "p.frontier.generate_heading": "Einen Entwurf erzeugen",
  "p.frontier.generate_says":
    "Als Entwurfsszenario gespeichert, zum Bewerten, Vergleichen und Festlegen wie jedes andere. Ein Name ist nötig, höchstens {bytes} Bytes. Mit einer Währung wird jeder Plan in ihr bepreist; eine Obergrenze braucht ihre Währung.",
  "p.frontier.g_name": "Name des Entwurfs",
  "p.frontier.g_cap": "Budgetobergrenze (optional)",
  "p.frontier.g_currency": "Währung (nötig mit einer Obergrenze)",
  "p.frontier.generate": "Den Entwurf erzeugen",
  "p.frontier.draft_saved":
    "Entwurf {name} gespeichert; er steht in der Liste oben.",
  "p.frontier.choice_at": "Was eine Obergrenze von {cap} wählt",
  "p.frontier.choice_of": "Was {name} gewählt hat",
  "p.frontier.chosen_count_one": "{n} Plan von {of} gewählt.",
  "p.frontier.chosen_count_other": "{n} Pläne von {of} gewählt.",
  "p.frontier.col_plan": "Plan in der Wahl",
  "p.frontier.col_chosen": "Gewählt oder nicht",
  "p.frontier.col_score": "Sein Smart-Score",
  "p.frontier.col_plan_cost": "Seine Kosten",
  "p.frontier.col_why": "Warum draußen",
  "p.frontier.chosen": "gewählt",
  "p.frontier.left_out": "draußen gelassen",
  "p.frontier.no_score": "kein Smart-Score",
  "p.frontier.plan_unknown":
    "Ein Plan, den Sie nicht lesen können oder der nicht mehr existiert",
  "p.frontier.reason_no_score":
    "er hat keinen Smart-Score, was kein Wert von null ist",
  "p.frontier.reason_foreign_currency":
    "seine Budgetzeilen sind alle in einer anderen Währung, daher kann er nicht bepreist werden",
  "p.frontier.reason_over_cap": "die Obergrenze war vor ihm erreicht",
  "p.frontier.reason_must_include_conflict":
    "er muss enthalten sein, ist aber kein Plan, den Sie lesen können",
  "p.frontier.reason_other":
    "aus einem Grund draußen, den diese Seite nicht kennt",
  "p.frontier.problem_currency":
    "Geben Sie die Währung mit drei Buchstaben an.",
  "p.frontier.problem_steps":
    "Die Zahl der Obergrenzen muss eine ganze Zahl von {min} bis {max} sein.",
  "p.frontier.problem_caps_none": "Nennen Sie mindestens eine Obergrenze.",
  "p.frontier.problem_caps_many": "Nennen Sie höchstens {list} Obergrenzen.",
  "p.frontier.problem_caps_amount":
    "Jede Obergrenze muss ein Betrag nicht unter null sein.",
  "p.frontier.problem_must_include":
    "Wählen Sie höchstens {list} Pläne zum Einschließen.",
  "p.frontier.problem_name": "Geben Sie dem Entwurf einen Namen.",
  "p.frontier.problem_name_long": "Der Name ist länger als {bytes} Bytes.",
  "p.frontier.problem_cap_currency": "Eine Obergrenze braucht ihre Währung.",
  "p.frontier.problem_cap_amount":
    "Die Obergrenze muss ein Betrag nicht unter null sein.",
  "p.frontier.refused_frontier": "Die Grenze wurde abgelehnt.",
  "p.frontier.refused_generate": "Der Entwurf wurde abgelehnt.",
  "p.frontier.could_not_read": "Die Grenze konnte nicht gezeichnet werden.",
  "p.frontier.could_not_generate": "Der Entwurf konnte nicht erzeugt werden.",
  "p.frontier.rollback": "Zurücknehmen",
  "p.frontier.rollback_named": "{name} zurücknehmen",
  "p.frontier.rollback_says":
    "Ein festgelegtes Szenario kann nach einer Rückfrage wieder zum Entwurf werden; es ändert nichts außer dem Szenario.",
  "p.frontier.confirm_rollback":
    "{name} wieder zum Entwurf machen? Es ist dann nicht mehr das festgelegte Portfolio; die Festlegung bleibt im Prüfprotokoll.",
  "p.frontier.rolled_back": "{name} ist wieder ein Entwurf.",
  "p.frontier.refused_rollback_status":
    "{name} ist nicht mehr festgelegt, daher gibt es nichts zurückzunehmen. Die Liste wurde neu gelesen.",
  "p.frontier.refused_rollback_gone":
    "{name} existiert nicht mehr, oder Sie können es nicht lesen. Die Liste wurde neu gelesen.",
  "p.frontier.could_not_roll_back":
    "Das Szenario konnte nicht zurückgenommen werden.",
  "p.frontier.status_draft": "Entwurf",
  "p.frontier.status_committed": "festgelegt",
  "p.frontier.violations_one": "{n} Bedingung verletzt",
  "p.frontier.violations_other": "{n} Bedingungen verletzt",
  "p.frontier.compare_note":
    "B minus A, je eine Währung; Währungen werden nie zusammengezählt.",
  "p.intake.edit": "Bearbeiten",
  "p.intake.edit_named": "Den Entwurf {title} bearbeiten",
  "p.intake.edit_heading": "{title} bearbeiten",
  "p.intake.edit_heading_loading": "Der Entwurf wird gelesen…",
  "p.intake.edit_form": "Die Felder des Entwurfs",
  "p.intake.edit_says":
    "Nur ein Entwurf kann bearbeitet werden; dort wird ein Vorschlag auch als externe Anfrage markiert. Ein Titel ist nötig; Titel und Zusammenfassung haben je höchstens {bytes} Bytes; ein Betrag braucht seine Währung; ein genannter Zielkonflikt hat {min} bis {max} Zeichen. Speichern ersetzt jedes Feld.",
  "p.intake.save": "Den Entwurf speichern",
  "p.intake.saved": "{title} gespeichert.",
  "p.intake.unchanged": "Nichts hat sich geändert; es wurde nichts gesendet.",
  "p.intake.could_not_read_draft": "Der Entwurf konnte nicht gelesen werden.",
  "p.intake.problem_not_draft": "Nur ein Entwurf kann bearbeitet werden.",
  "p.intake.problem_title_long": "Der Titel ist länger als {bytes} Bytes.",
  "p.intake.problem_summary_long":
    "Die Zusammenfassung ist länger als {bytes} Bytes.",
  "p.intake.refused_not_draft":
    "Die Bearbeitung wurde abgelehnt: der Vorschlag ist kein Entwurf mehr. Die Liste wurde neu gelesen.",
  "p.intake.refused_edit":
    "Die Bearbeitung wurde abgelehnt: ein Feld liegt außerhalb dessen, was der Dienst annimmt.",
  "p.intake.forecast_heading": "Wie viele genehmigt werden",
  "p.intake.forecast_says":
    "Durch Simulation daraus gezogen, wie viele Vorschläge in jedem vergangenen Zeitraum genehmigt wurden; eingehende Vorschläge fließen nicht ein. Eine Prognose braucht mindestens {needed} Zeiträume Verlauf.",
  "p.intake.f_periods": "Zeiträume voraus (1 bis {max}; leer für {fallback})",
  "p.intake.f_period_days":
    "Tage je Zeitraum (1 bis {max}; leer für {fallback})",
  "p.intake.f_history": "Zeiträume Verlauf (1 bis {max}; leer für {fallback})",
  "p.intake.forecast_run": "Prognostizieren",
  "p.intake.problem_periods":
    "Die Zeiträume voraus müssen eine ganze Zahl von 1 bis {periods} sein.",
  "p.intake.problem_period_days":
    "Ein Zeitraum muss eine ganze Zahl von Tagen von 1 bis {days} sein.",
  "p.intake.problem_history":
    "Der Verlauf muss eine ganze Zahl von Zeiträumen von 1 bis {history} sein.",
  "p.intake.refused_forecast": "Die Prognose wurde abgelehnt.",
  "p.intake.could_not_forecast": "Die Prognose konnte nicht erstellt werden.",
  "p.intake.window":
    "Verlauf vom {from} bis {to}, in Zeiträumen von {days} Tagen.",
  "p.intake.at_least_label_one":
    "Im nächsten {n} Zeitraum ({days} Tage), mindestens",
  "p.intake.at_least_label_other":
    "In den nächsten {n} Zeiträumen ({days} Tage), mindestens",
  "p.intake.at_least_means": "mit 85 % Zuversicht: die Zahl, mit der man plant",
  "p.intake.median_label": "Üblicherweise",
  "p.intake.median_means":
    "die Hälfte der Durchläufe genehmigt mehr, die Hälfte weniger: ein Münzwurf, keine Zusage",
  "p.intake.at_most_label": "Höchstens",
  "p.intake.at_most_means":
    "das hoffnungsvolle Ende: 85 % der Durchläufe genehmigen nicht mehr",
  "p.intake.proposals_n_one": "{n} Vorschlag",
  "p.intake.proposals_n_other": "{n} Vorschläge",
  "p.intake.absent_history":
    "Keine Prognose: sie braucht mindestens {needed} Zeiträume Verlauf, und es gibt {n}.",
  "p.intake.absent_other":
    "Aus diesem Verlauf ließ sich keine Prognose ziehen.",
  "p.intake.approvals_history":
    "Genehmigt je Zeitraum, die ältesten zuerst: {list}",
  "p.intake.arrivals_history":
    "Eingehende Vorschläge je Zeitraum, die ältesten zuerst (daneben gezeigt; sie fließen nicht in die Prognose ein): {list}",
  "p.planner.heading": "Geplante Zeit",
  "p.planner.says":
    "Blocken Sie an einem Tag Zeit für eine Ihrer offenen Aufgaben. Ein Zeitblock gehört nur Ihnen und ändert nie das Fälligkeitsdatum der Aufgabe; er endet nach seinem Beginn und dauert höchstens 12 Stunden. Zeitblöcke dürfen sich überschneiden: Eine Überschneidung wird als Konflikt gezeigt, und ein Block, der endet, wenn der nächste beginnt, ist keiner. Die Zeiten gelten in Ihrer Zeitzone, {tz}.",
  "p.planner.day_nav": "Tag wählen",
  "p.planner.previous_day": "Vorheriger Tag",
  "p.planner.back_to_today": "Zurück zu heute",
  "p.planner.next_day": "Nächster Tag",
  "p.planner.f_day": "Tag",
  "p.planner.show_day": "Tag zeigen",
  "p.planner.problem_day": "Geben Sie den Tag als Datum an.",
  "p.planner.day_heading": "Geplant am {day}",
  "p.planner.day_hours":
    "Dieser Tag hat {hours} Stunden, weil die Uhren umgestellt werden.",
  "p.planner.nothing_planned": "An diesem Tag ist nichts geplant.",
  "p.planner.clashes_one": "Kollidiert mit {n} anderen Zeitblock",
  "p.planner.clashes_other": "Kollidiert mit {n} anderen Zeitblöcken",
  "p.planner.remove": "Entfernen",
  "p.planner.remove_named": "Zeitblock für {title} entfernen, {time}",
  "p.planner.confirm_remove":
    "Den Zeitblock für {title} entfernen, {time}? Die Aufgabe selbst ändert sich nicht.",
  "p.planner.removed": "Zeitblock für {title} entfernt, {time}.",
  "p.planner.omitted_one":
    "{n} Zeitblock zu einer Aufgabe, die Sie nicht mehr lesen dürfen, wird nicht gezeigt.",
  "p.planner.omitted_other":
    "{n} Zeitblöcke zu Aufgaben, die Sie nicht mehr lesen dürfen, werden nicht gezeigt.",
  "p.planner.due_heading": "An diesem Tag fällig",
  "p.planner.nothing_due":
    "Nichts, was Ihnen zugewiesen ist, ist an diesem Tag fällig.",
  "p.planner.loading": "Der Tag wird gelesen …",
  "p.planner.form": "Zeit für eine Aufgabe planen",
  "p.planner.form_says":
    "Der Zeitblock liegt am {day}, in Ihrer Zeitzone. Einer über Mitternacht hinaus wird als zwei geplant.",
  "p.planner.f_task": "Aufgabe",
  "p.planner.choose_task": "Eine Ihrer offenen Aufgaben wählen",
  "p.planner.loading_tasks": "Ihre Aufgaben werden gelesen …",
  "p.planner.task_option": "{title}, im Plan {plan}",
  "p.planner.no_tasks":
    "Ihnen ist nichts zugewiesen, also gibt es keine Aufgabe, für die Zeit geplant werden kann.",
  "p.planner.f_start": "Beginnt",
  "p.planner.f_end": "Endet",
  "p.planner.plan_time": "Diese Zeit planen",
  "p.planner.planned": "{title} geplant, {time}.",
  "p.planner.problem_task": "Wählen Sie eine Aufgabe.",
  "p.planner.problem_start": "Geben Sie die Anfangszeit an.",
  "p.planner.problem_end": "Geben Sie die Endzeit an.",
  "p.planner.problem_start_skipped":
    "An diesem Tag überspringen die Uhren die Anfangszeit; wählen Sie eine andere.",
  "p.planner.problem_end_skipped":
    "An diesem Tag überspringen die Uhren die Endzeit; wählen Sie eine andere.",
  "p.planner.problem_order": "Ein Zeitblock muss nach seinem Beginn enden.",
  "p.planner.problem_too_long": "Ein Zeitblock dauert höchstens 12 Stunden.",
  "p.planner.refused_zone":
    "Die Zeitzone Ihres Browsers wurde nicht angenommen, daher kann der Tag nicht gezeigt werden.",
  "p.planner.refused_slot":
    "Der Zeitblock wurde nicht angenommen: Er muss nach seinem Beginn enden und höchstens 12 Stunden dauern, und eine Person hat höchstens {max} Zeitblöcke.",
  "p.planner.refused_task":
    "Diese Aufgabe gibt es nicht mehr, oder sie liegt in einem Plan, den Sie nicht lesen dürfen.",
  "p.planner.refused_gone":
    "Dieser Zeitblock ist bereits weg. Der Tag wurde neu gelesen.",
  "p.planner.could_not_load": "Der Tag konnte nicht gelesen werden.",
  "p.planner.could_not_plan": "Die Zeit konnte nicht geplant werden.",
  "p.planner.could_not_remove": "Der Zeitblock konnte nicht entfernt werden.",
  "p.feed.heading": "Kalender-Feed",
  "p.feed.says":
    "Eine private Adresse, die eine Kalender-App abonnieren kann: die Ihnen zugewiesenen offenen Aufgaben mit Fälligkeitsdatum, die Meilensteine und Sprints ihrer Pläne und die von Ihnen geplante Zeit. Eine zweite Adresse ist ein Feed Ihrer Benachrichtigungen für einen Feedreader. Wer eine Adresse hat, kann lesen, was sie zeigt; behalten Sie sie also für sich.",
  "p.feed.rules":
    "Eine Adresse wird einmal gezeigt, hier, und diese Seite speichert sie nicht; sie gilt 365 Tage. Eine neue stoppt jede frühere sofort.",
  "p.feed.issue": "Feed-Adresse erstellen",
  "p.feed.confirm_issue":
    "Eine neue Feed-Adresse erstellen? Jede früher erstellte Adresse funktioniert sofort nicht mehr.",
  "p.feed.issued":
    "Ihre neuen Adressen stehen unten. Kopieren Sie sie jetzt: Sie werden nicht noch einmal gezeigt.",
  "p.feed.calendar_address": "Kalenderadresse (iCalendar)",
  "p.feed.atom_address": "Adresse des Benachrichtigungs-Feeds (Atom)",
  "p.feed.revoke": "Alle Feed-Adressen stoppen",
  "p.feed.confirm_revoke":
    "Alle von Ihnen erstellten Feed-Adressen stoppen? Eine Kalender-App, die eine nutzt, wird sofort nicht mehr aktualisiert.",
  "p.feed.revoked":
    "Alle von Ihnen erstellten Feed-Adressen funktionieren nicht mehr.",
  "p.feed.no_address":
    "Die Antwort enthielt keine Adresse, die diese Seite zeigen kann, daher wird keine gezeigt. Jede frühere Adresse ist gestoppt; erstellen Sie bei Bedarf eine neue.",
  "p.feed.refused_forbidden": "Sie dürfen keine Feed-Adresse verwalten.",
  "p.feed.could_not_issue": "Die Feed-Adresse konnte nicht erstellt werden.",
  "p.feed.could_not_revoke": "Die Feed-Adressen konnten nicht gestoppt werden.",
  "p.watching.heading": "Was Sie verfolgen",
  "p.watching.says":
    "Sie werden benachrichtigt, wenn sich etwas ändert, das Sie verfolgen. Sie verfolgen eine Aufgabe, wenn Sie sie anlegen, kommentieren oder zugewiesen bekommen, sofern Sie das in Ihren Benachrichtigungseinstellungen nicht abgeschaltet haben. Niemandem wird gesagt, wer etwas verfolgt. Ein Plan wird benannt; alles andere wird mit seiner Art und dem Anfang seiner Referenz gezeigt.",
  "p.watching.count_one": "Sie verfolgen {n} Eintrag.",
  "p.watching.count_other": "Sie verfolgen {n} Einträge.",
  "p.watching.capped": "Nur die neuesten {max} werden gezeigt.",
  "p.watching.none": "Sie verfolgen derzeit nichts.",
  "p.watching.named_plan": "Vorhaben {name}",
  "p.watching.unnamed": "{kind}, Referenz {ref}",
  "p.watching.kind_plan": "Vorhaben",
  "p.watching.kind_task": "Aufgabe",
  "p.watching.kind_risk": "Risiko",
  "p.watching.kind_proposal": "Vorschlag",
  "p.watching.kind_idea": "Idee",
  "p.watching.kind_canvas_item": "Canvas-Eintrag",
  "p.watching.kind_budget_line": "Budgetposten",
  "p.watching.kind_scenario": "Szenario",
  "p.watching.kind_document_version": "Dokumentversion",
  "p.watching.kind_other": "Etwas anderes",
  "p.watching.stop": "Nicht mehr verfolgen",
  "p.watching.stop_named": "{what} nicht mehr verfolgen",
  "p.watching.confirm_stop":
    "{what} nicht mehr verfolgen? Sie werden bei Änderungen nicht mehr benachrichtigt.",
  "p.watching.stopped": "Sie verfolgen {what} nicht mehr.",
  "p.watching.refused": "Das Beenden dieses Verfolgens wurde nicht angenommen.",
  "p.watching.loading": "Ihre verfolgten Einträge werden gelesen …",
  "p.watching.could_not_load":
    "Ihre verfolgten Einträge konnten nicht gelesen werden.",
  "p.watching.could_not_stop": "Das Verfolgen konnte nicht beendet werden.",
  "p.workprofile.heading": "Arbeit im Zeitverlauf",
  "p.workprofile.says":
    "Die Arbeit der Aufgaben in jedem Plan, den Sie lesen dürfen, nach dem Verlauf jeder Aufgabe auf ihre Arbeitstage verteilt, in ganzen Minuten. Arbeit, die sich nicht verteilen lässt (eine noch nicht terminierte Aufgabe oder eine ohne Arbeitstag), wird getrennt gezählt und nie weggelassen.",
  "p.workprofile.rules":
    "Wochen laufen von Montag bis Sonntag. Lassen Sie die Daten leer, um alle verteilte Arbeit abzudecken. Ein Zeitfenster umfasst höchstens zehn Jahre und {max} Zeiträume.",
  "p.workprofile.form": "Arbeit im Zeitverlauf zeigen",
  "p.workprofile.f_period": "Zeiträume",
  "p.workprofile.period_week": "Wochen",
  "p.workprofile.period_day": "Tage",
  "p.workprofile.period_month": "Monate",
  "p.workprofile.f_from": "Von (leer für den ersten verteilten Tag)",
  "p.workprofile.f_to": "Bis (leer für den letzten verteilten Tag)",
  "p.workprofile.show": "Arbeit zeigen",
  "p.workprofile.problem_from": "Geben Sie den ersten Tag als Datum an.",
  "p.workprofile.problem_to": "Geben Sie den letzten Tag als Datum an.",
  "p.workprofile.problem_order":
    "Der erste Tag darf nicht nach dem letzten liegen.",
  "p.workprofile.problem_too_long":
    "Ein Zeitfenster umfasst höchstens zehn Jahre.",
  "p.workprofile.problem_periods":
    "Dieses Zeitfenster hat mehr als {max} Zeiträume; wählen Sie längere Zeiträume oder ein kürzeres Fenster.",
  "p.workprofile.refused":
    "Dieses Zeitfenster wurde nicht angenommen: Ist noch keine Arbeit verteilt, geben Sie beide Daten an, und bleiben Sie innerhalb der oben genannten Zeiträume.",
  "p.workprofile.none":
    "Kein Plan, den Sie lesen dürfen, hat eine Aufgabe mit Arbeit.",
  "p.workprofile.summary":
    "{range}: {work} Arbeit, davon {unplaced} nicht verteilt.",
  "p.workprofile.plans_table": "Arbeit je Plan",
  "p.workprofile.periods_table": "Arbeit je Zeitraum",
  "p.workprofile.col_plan": "Planname",
  "p.workprofile.col_work": "Arbeit",
  "p.workprofile.col_unplaced": "Nicht verteilt",
  "p.workprofile.col_period": "Zeitraum",
  "p.workprofile.plan_unnamed": "Ein Plan ohne Namen",
  "p.workprofile.could_not_load":
    "Die Arbeit im Zeitverlauf konnte nicht gelesen werden.",
  "p.timeentry.heading": "Zeit erfassen",
  "p.timeentry.says":
    "Zeit wird für Sie erfasst, in ganzen Minuten, von 1 Minute bis {max} an einem Datum, in einem Plan und auf Wunsch einer seiner Aufgaben. Eine genehmigte Woche nimmt keine Zeit mehr an. Ein Eintrag kann nach dem Erfassen weder geändert noch entfernt werden.",
  "p.timeentry.open": "Zeit eintragen",
  "p.timeentry.form": "Zeit in einem Plan erfassen",
  "p.timeentry.f_plan": "In welchem Plan",
  "p.timeentry.choose_plan": "Plan wählen",
  "p.timeentry.loading": "Pläne werden gelesen …",
  "p.timeentry.f_task": "Aufgabe (optional)",
  "p.timeentry.no_task": "Keine Aufgabe: der Plan selbst",
  "p.timeentry.task_unnamed": "Eine Aufgabe, die nicht mehr in diesem Plan ist",
  "p.timeentry.f_date": "Datum",
  "p.timeentry.f_duration": "Aufgewendete Zeit",
  "p.timeentry.f_hours": "Stunden",
  "p.timeentry.f_minutes": "Minuten",
  "p.timeentry.typed": "Das sind {time}.",
  "p.timeentry.f_category": "Kategorie",
  "p.timeentry.category_unclassified": "Nicht eingeordnet",
  "p.timeentry.category_capex": "Investitionsausgaben (Capex)",
  "p.timeentry.category_opex": "Betriebsausgaben (Opex)",
  "p.timeentry.f_billable": "Abrechenbar",
  "p.timeentry.f_note": "Notiz (optional)",
  "p.timeentry.record": "Diese Zeit erfassen",
  "p.timeentry.confirm":
    "{time} am {day} in {plan} erfassen? Ein Eintrag kann nach dem Erfassen weder geändert noch entfernt werden.",
  "p.timeentry.recorded": "{time} am {day} in {plan} erfasst.",
  "p.timeentry.problem_plan": "Wählen Sie einen Plan.",
  "p.timeentry.problem_date":
    "Geben Sie das Datum an, an dem die Zeit aufgewendet wurde.",
  "p.timeentry.problem_locked":
    "Diese Woche ist genehmigt und gesperrt und nimmt keine Zeit mehr an.",
  "p.timeentry.problem_duration":
    "Geben Sie die Zeit in ganzen Stunden (0 bis 24) und Minuten (0 bis 59) an, insgesamt 1 bis {max} Minuten.",
  "p.timeentry.refused_locked":
    "Diese Woche ist genehmigt und gesperrt und nimmt keine Zeit mehr an. Es wurde nichts erfasst.",
  "p.timeentry.refused_invalid":
    "Der Eintrag wurde nicht angenommen: Seine Zeit oder seine Aufgabe passt nicht zum Plan. Es wurde nichts erfasst.",
  "p.timeentry.refused_plan":
    "Diesen Plan gibt es nicht mehr, oder Sie dürfen ihn nicht lesen.",
  "p.timeentry.refused_forbidden":
    "Sie dürfen in diesem Plan keine Zeit erfassen.",
  "p.timeentry.could_not_load_plans": "Die Pläne konnten nicht gelesen werden.",
  "p.timeentry.could_not_load_plan":
    "Aufgaben und Zeit dieses Plans konnten nicht gelesen werden.",
  "p.timeentry.could_not_record": "Die Zeit konnte nicht erfasst werden.",
  "p.timeentry.yours_heading": "Ihre Zeit in {plan}",
  "p.timeentry.yours_none": "Sie haben in diesem Plan keine Zeit erfasst.",
  "p.timeentry.yours_table": "Ihre Zeiteinträge in {plan}, neueste zuerst",
  "p.timeentry.col_date": "Datum",
  "p.timeentry.col_time": "Zeit",
  "p.timeentry.col_task": "Aufgabe",
  "p.timeentry.col_category": "Kategorie",
  "p.timeentry.col_billable": "Abrechenbar",
  "p.timeentry.col_note": "Notiz",
  "p.timeentry.billable_yes": "Abrechenbar",
  "p.timeentry.billable_no": "Nicht abrechenbar",
  "p.timereport.heading": "Zeitbericht",
  "p.timereport.says":
    "Erfasste Zeit in den Plänen, die Sie lesen dürfen, gruppiert nach Ihrer Wahl, abrechenbar und nicht abrechenbar immer getrennt. Lassen Sie die Daten leer für die letzten 90 Tage bis heute. Eine Person ist eine person:- oder worker:-Referenz; leer lassen für alle.",
  "p.timereport.form": "Zeitbericht anfordern",
  "p.timereport.f_group_by": "Gruppieren nach",
  "p.timereport.by_plan": "Nach Plan",
  "p.timereport.by_person": "Nach Person",
  "p.timereport.by_category": "Nach Kategorie",
  "p.timereport.by_billable": "Abrechenbar oder nicht",
  "p.timereport.by_month": "Nach Monat",
  "p.timereport.f_from": "Von",
  "p.timereport.f_to": "Bis",
  "p.timereport.f_plan": "Nur dieser Plan",
  "p.timereport.every_plan": "Jeder Plan, den Sie lesen dürfen",
  "p.timereport.loading_plans": "Pläne werden gelesen …",
  "p.timereport.f_person": "Person (eine Referenz)",
  "p.timereport.show": "Zeitbericht zeigen",
  "p.timereport.problem_from": "Geben Sie den ersten Tag als Datum an.",
  "p.timereport.problem_to": "Geben Sie den letzten Tag als Datum an.",
  "p.timereport.problem_order":
    "Der erste Tag darf nicht nach dem letzten liegen.",
  "p.timereport.problem_person":
    "Eine Person ist person: oder worker: und eine Kennung, etwa person:6f1c2a3b-0000-4000-8000-000000000009.",
  "p.timereport.refused":
    "Der Zeitbericht wurde für diese Auswahl nicht angenommen.",
  "p.timereport.window": "Erfasste Zeit {range}.",
  "p.timereport.no_entries":
    "In diesen Plänen wurde in diesem Zeitraum keine Zeit erfasst.",
  "p.timereport.table": "Zeit je Gruppe",
  "p.timereport.col_entries": "Einträge",
  "p.timereport.col_billable": "Abrechenbar",
  "p.timereport.col_not_billable": "Nicht abrechenbar",
  "p.timereport.col_total": "Gesamt",
  "p.timereport.total":
    "Insgesamt: {billable} abrechenbar, {not_billable} nicht abrechenbar, zusammen {total}.",
  "p.timereport.omitted_one":
    "{n} Eintrag in einem Plan, den Sie nicht lesen dürfen, ist ausgelassen.",
  "p.timereport.omitted_other":
    "{n} Einträge in Plänen, die Sie nicht lesen dürfen, sind ausgelassen.",
  "p.timereport.truncated":
    "Nur die ersten {max} Einträge wurden gelesen; der Bericht ist daher unvollständig.",
  "p.timereport.csv": "Diesen Zeitbericht als CSV herunterladen",
  "p.timereport.billable": "Abrechenbar",
  "p.timereport.not_billable": "Nicht abrechenbar",
  "p.timereport.plan_unnamed": "Ein hier nicht benannter Plan ({ref})",
  "p.timereport.could_not_load": "Der Zeitbericht konnte nicht gelesen werden.",
  "p.timereport.could_not_load_plans":
    "Die Pläne zur Auswahl konnten nicht gelesen werden.",
  "p.hammock.heading": "Sammelvorgänge: Übersichtsaufgaben über andere hinweg",
  "p.hammock.says":
    "Ein Sammelvorgang ist eine Übersicht, deren Termine aus den verknüpften Aufgaben dieses Plans stammen, vom frühesten Anfang bis zum spätesten Ende. Er gehört nicht zum Abhängigkeitsnetz und ändert daher nie den Puffer einer Aufgabe oder das Ende. Eine Aufgabe wird nur dann zum Sammelvorgang, wenn sie keine Dauer, keine Arbeit, keine Einschränkung und keine Abhängigkeit hat; ihr geplanter Anfang wird gelöscht. Wird sie wieder zur Aufgabe, entfallen alle ihre Verknüpfungen. Ein Sammelvorgang verknüpft eine Aufgabe oder einen anderen Sammelvorgang dieses Plans, nicht sich selbst, nicht doppelt und nicht im Kreis.",
  "p.hammock.none": "Dieser Plan hat keine Sammelvorgänge.",
  "p.hammock.spans": "Umfasst {range}.",
  "p.hammock.no_links":
    "Noch keine Termine: Er ist mit keiner Aufgabe verknüpft.",
  "p.hammock.members_unscheduled":
    "Noch keine Termine: Keine der verknüpften Aufgaben hat Termine.",
  "p.hammock.untitled": "Eine Aufgabe ohne Titel",
  "p.hammock.member_hammock": "{title} (ein Sammelvorgang)",
  "p.hammock.unmake_named": "{title} wieder zur Aufgabe machen",
  "p.hammock.unlink": "Verknüpfung entfernen",
  "p.hammock.unlink_named": "Verknüpfung von {hammock} zu {title} entfernen",
  "p.hammock.make_form": "Eine Aufgabe zum Sammelvorgang machen",
  "p.hammock.f_task": "Aufgabe",
  "p.hammock.choose_task": "Aufgabe wählen",
  "p.hammock.make": "Zum Sammelvorgang machen",
  "p.hammock.link_form": "Einen Sammelvorgang mit einer Aufgabe verknüpfen",
  "p.hammock.f_hammock": "Sammelvorgang",
  "p.hammock.choose_hammock": "Sammelvorgang wählen",
  "p.hammock.f_member": "Umfasste Aufgabe",
  "p.hammock.choose_member": "Zu umfassende Aufgabe wählen",
  "p.hammock.link": "Verknüpfung hinzufügen",
  "p.hammock.confirm_make":
    "{title} zum Sammelvorgang machen? Der geplante Anfang wird gelöscht, und die Termine kommen dann aus den verknüpften Aufgaben.",
  "p.hammock.confirm_unmake_one":
    "{title} wieder zur Aufgabe machen? Ihre {n} Verknüpfung entfällt.",
  "p.hammock.confirm_unmake_other":
    "{title} wieder zur Aufgabe machen? Ihre {n} Verknüpfungen entfallen.",
  "p.hammock.confirm_unlink":
    "Verknüpfung von {hammock} zu {title} entfernen? Die Aufgabe selbst bleibt unverändert.",
  "p.hammock.made": "{title} ist jetzt ein Sammelvorgang.",
  "p.hammock.unmade": "{title} ist wieder eine Aufgabe.",
  "p.hammock.linked": "{hammock} umfasst jetzt {title}.",
  "p.hammock.unlinked": "{hammock} umfasst {title} nicht mehr.",
  "p.hammock.problem_choose_task":
    "Wählen Sie eine Aufgabe, die zum Sammelvorgang werden soll.",
  "p.hammock.problem_carries_work":
    "Diese Aufgabe hat eine Dauer, Arbeit oder eine Einschränkung: Löschen Sie diese zuerst im Terminplan.",
  "p.hammock.problem_has_dependencies":
    "Diese Aufgabe hat Abhängigkeiten: Entfernen Sie sie zuerst, denn ein Sammelvorgang hat keine.",
  "p.hammock.problem_choose_hammock": "Wählen Sie einen Sammelvorgang.",
  "p.hammock.problem_choose_member":
    "Wählen Sie eine Aufgabe, die der Sammelvorgang umfassen soll.",
  "p.hammock.problem_self":
    "Ein Sammelvorgang kann sich nicht selbst umfassen.",
  "p.hammock.problem_linked":
    "Dieser Sammelvorgang umfasst diese Aufgabe bereits.",
  "p.hammock.problem_loop":
    "Jener Sammelvorgang umfasst diesen bereits, die Verknüpfung ergäbe einen Kreis.",
  "p.hammock.refused_make":
    "Der Dienst hat die Aufgabe nicht zum Sammelvorgang gemacht: Sie hat noch eine Dauer, Arbeit, eine Einschränkung oder eine Abhängigkeit.",
  "p.hammock.refused_link":
    "Der Dienst hat die Verknüpfung nicht hinzugefügt: Die Aufgabe ist vielleicht nicht mehr im Plan, schon umfasst oder Teil eines Kreises.",
  "p.hammock.refused_gone":
    "Die Aufgabe oder der Sammelvorgang ist weg, oder Sie können ihn nicht mehr lesen.",
  "p.hammock.refused_link_gone": "Diese Verknüpfung ist bereits weg.",
  "p.hammock.could_not_load":
    "Die Sammelvorgänge konnten nicht gelesen werden.",
  "p.hammock.could_not_save": "Der Sammelvorgang konnte nicht geändert werden.",
  "p.crossplan.heading": "Aufgaben verknüpfter Pläne",
  "p.crossplan.says":
    "Das sind die Aufgaben anderer Pläne, die Sie lesen können und mit denen Aufgaben dieses Plans verknüpft sind, terminiert nach dem Kalender ihres eigenen Plans. Ein gestrichelter Balken ist eine Aufgabe abseits des kritischen Pfads; ein voller Balken und eine Raute kennzeichnen eine darauf.",
  "p.crossplan.network_finish":
    "Mit den verknüpften Plänen endet das Netz am {date}.",
  "p.crossplan.truncated":
    "Nicht alle verknüpften Pläne werden gezeigt: Das Netz hat seine Grenze an Plänen oder Aufgaben erreicht.",
  "p.crossplan.links_heading": "Verknüpfungen in einen anderen Plan",
  "p.crossplan.lag": "Verzögerung von {days} Arbeitstagen",
  "p.crossplan.timeline": "Aufgaben von {plan} auf einer Zeitachse",
  "p.crossplan.table": "Termine der Aufgaben verknüpfter Pläne",
  "p.crossplan.col_task": "Aufgabe",
  "p.crossplan.col_plan": "Projektplan",
  "p.crossplan.col_start": "Frühester Anfang",
  "p.crossplan.col_finish": "Frühestes Ende",
  "p.crossplan.col_critical": "Auf dem kritischen Pfad",
  "p.crossplan.yes": "Ja",
  "p.crossplan.no": "Nein",
  "p.crossplan.untitled": "Eine Aufgabe ohne Titel",
  "p.crossplan.kind_fs": "Ende zu Anfang",
  "p.crossplan.kind_ss": "Anfang zu Anfang",
  "p.crossplan.kind_ff": "Ende zu Ende",
  "p.crossplan.kind_sf": "Anfang zu Ende",
  "p.crossplan.task_heading":
    "Abhängigkeiten, in diesem oder einem anderen Plan",
  "p.crossplan.task_says":
    "Eine Abhängigkeit verbindet diese Aufgabe mit einer in einem anderen Plan, den Sie lesen können. Ihre Art ist Ende zu Anfang, sofern nicht anders gewählt; ihre Verzögerung ist eine ganze Zahl von Arbeitstagen von minus {max} bis {max}, eine negative ist ein Vorlauf. Eine Abhängigkeit wird nicht zweimal in derselben Richtung angelegt, nicht im Kreis über Pläne hinweg und nicht mit einem Sammelvorgang. Eine, deren anderes Ende Sie nicht lesen können, wird gezählt, nie benannt, und bleibt außerhalb des Terminplans.",
  "p.crossplan.no_links": "Diese Aufgabe hat noch keine Abhängigkeiten.",
  "p.crossplan.this_before": "Diese Aufgabe kommt vor {other}",
  "p.crossplan.this_after": "Diese Aufgabe kommt nach {other}",
  "p.crossplan.end_own": "{title}, in diesem Plan",
  "p.crossplan.end_external": "{title}, im Plan {plan}",
  "p.crossplan.end_unnamed": "eine hier nicht benannte Aufgabe ({ref})",
  "p.crossplan.remove": "Entfernen",
  "p.crossplan.remove_named": "Abhängigkeit von {other} entfernen",
  "p.crossplan.restricted_one":
    "{n} weitere Abhängigkeit verbindet mit einer Aufgabe, die Sie nicht lesen können.",
  "p.crossplan.restricted_other":
    "{n} weitere Abhängigkeiten verbinden mit Aufgaben, die Sie nicht lesen können.",
  "p.crossplan.see_network": "Den Terminplan über Pläne hinweg ansehen",
  "p.crossplan.form":
    "Eine Abhängigkeit von einer Aufgabe eines anderen Plans hinzufügen",
  "p.crossplan.f_order": "Reihenfolge",
  "p.crossplan.order_before": "Diese Aufgabe kommt zuerst",
  "p.crossplan.order_after": "Die andere Aufgabe kommt zuerst",
  "p.crossplan.f_plan": "Anderer Plan",
  "p.crossplan.choose_plan": "Plan wählen",
  "p.crossplan.f_task": "Aufgabe in diesem Plan",
  "p.crossplan.choose_task": "Aufgabe wählen",
  "p.crossplan.f_kind": "Art der Abhängigkeit",
  "p.crossplan.f_lag": "Verzögerung in Arbeitstagen (0, wenn leer)",
  "p.crossplan.add": "Abhängigkeit hinzufügen",
  "p.crossplan.added": "Abhängigkeit von {title} hinzugefügt.",
  "p.crossplan.removed": "Abhängigkeit entfernt.",
  "p.crossplan.confirm_remove_before":
    "Abhängigkeit entfernen: {task} kommt vor {other}? Keine der beiden Aufgaben wird geändert.",
  "p.crossplan.confirm_remove_after":
    "Abhängigkeit entfernen: {task} kommt nach {other}? Keine der beiden Aufgaben wird geändert.",
  "p.crossplan.problem_plan": "Wählen Sie den anderen Plan.",
  "p.crossplan.problem_task": "Wählen Sie eine Aufgabe in diesem Plan.",
  "p.crossplan.problem_lag":
    "Eine Verzögerung ist eine ganze Zahl von Arbeitstagen von minus {max} bis {max}.",
  "p.crossplan.problem_self":
    "Eine Aufgabe kann nicht von sich selbst abhängen.",
  "p.crossplan.problem_exists": "Diese Abhängigkeit besteht bereits.",
  "p.crossplan.refused_unreadable":
    "Eine der beiden Aufgaben ist weg, oder Sie können sie nicht lesen.",
  "p.crossplan.refused_exists": "Diese Abhängigkeit besteht bereits.",
  "p.crossplan.refused_add":
    "Der Dienst hat die Abhängigkeit nicht hinzugefügt: Sie besteht vielleicht schon, ergäbe einen Kreis oder verbindet einen Sammelvorgang.",
  "p.crossplan.refused_gone":
    "Diese Abhängigkeit ist bereits weg, oder Sie können nicht mehr beide Aufgaben lesen.",
  "p.crossplan.could_not_load":
    "Die Abhängigkeiten dieser Aufgabe konnten nicht gelesen werden.",
  "p.crossplan.could_not_load_plans":
    "Die Pläne zur Auswahl konnten nicht gelesen werden.",
  "p.crossplan.could_not_load_tasks":
    "Die Aufgaben dieses Plans konnten nicht gelesen werden.",
  "p.crossplan.could_not_save":
    "Die Abhängigkeit konnte nicht geändert werden.",
  "p.taskrollup.heading": "Unteraufgaben zusammengefasst",
  "p.taskrollup.says":
    "Jedes Mal aus den Unteraufgaben dieser Aufgabe berechnet, nie gespeichert: das späteste Fälligkeitsdatum unter ihnen, ihre Punkte addiert, der Fortschritt nur nach den geschätzten gewichtet, und die über sie erfasste Zeit.",
  "p.taskrollup.subtasks": "Unteraufgaben",
  "p.taskrollup.done_of_one": "{done} von {n} erledigt",
  "p.taskrollup.done_of_other": "{done} von {n} erledigt",
  "p.taskrollup.due": "Spätestes Fälligkeitsdatum",
  "p.taskrollup.points": "Punkte addiert",
  "p.taskrollup.progress": "Fortschritt",
  "p.taskrollup.spent": "Erfasste Zeit",
  "p.taskrollup.no_children": "Diese Aufgabe hat keine Unteraufgaben.",
  "p.taskrollup.no_dated_children":
    "Keine ihrer Unteraufgaben hat ein Fälligkeitsdatum.",
  "p.taskrollup.no_estimates": "Keine ihrer Unteraufgaben ist geschätzt.",
  "p.taskrollup.absent": "Nicht bekannt.",
  "p.taskrollup.unestimated_one":
    "{n} Unteraufgabe hat keine Schätzung und bleibt daher beim Fortschritt außen vor (nicht als null gezählt):",
  "p.taskrollup.unestimated_other":
    "{n} Unteraufgaben haben keine Schätzung und bleiben daher beim Fortschritt außen vor (nicht als null gezählt):",
  "p.taskrollup.unnamed": "Eine hier nicht benannte Unteraufgabe ({ref})",
  "p.taskrollup.refused_gone":
    "Diese Aufgabe ist weg, oder Sie können sie nicht mehr lesen.",
  "p.taskrollup.could_not_load":
    "Die zusammengefassten Unteraufgaben konnten nicht gelesen werden.",
  "p.readings.heading": "Mehr zu diesem Plan",
  "p.readings.says":
    "Jede dieser Angaben wird erst gelesen, wenn Sie danach fragen.",
  "p.readings.read": "Lesen",
  "p.readings.could_not_read": "Das konnte nicht gelesen werden.",
  "p.readings.not_measured": "nicht gemessen",
  "p.readings.days": "{n} Tage",
  "p.readings.rollup_heading": "Fluss über die enthaltenen Pläne",
  "p.readings.rollup_says":
    "Die Aufgaben dieses Plans und aller Pläne darunter, zusammen gezählt, nicht gemittelt; bis zu {max} Ebenen tief, wenn leer gelassen.",
  "p.readings.f_depth": "Ebenen tief",
  "p.readings.rollup_plans_one": "{n} Plan.",
  "p.readings.rollup_plans_other": "{n} Pläne.",
  "p.readings.rollup_tasks_one":
    "{n} Aufgabe: {finished} beendet, {wip} in Arbeit, {not_started} nicht begonnen.",
  "p.readings.rollup_tasks_other":
    "{n} Aufgaben: {finished} beendet, {wip} in Arbeit, {not_started} nicht begonnen.",
  "p.readings.rollup_times":
    "Die meiste Arbeit (85 von 100) war binnen {cycle} nach Beginn und binnen {lead} nach der Anfrage fertig; Zeit mit Arbeiten statt Warten: {efficiency}.",
  "p.readings.rollup_truncated":
    "Nicht jeder Plan darunter wurde gezählt: Der Durchlauf hat seine Grenze erreicht.",
  "p.readings.rollup_revisits_one":
    "{n} Plan wird zweimal erreicht und einmal gezählt.",
  "p.readings.rollup_revisits_other":
    "{n} Pläne werden zweimal erreicht und einmal gezählt.",
  "p.readings.rollup_table": "Fluss nach Plan",
  "p.readings.col_plan": "Projektplan",
  "p.readings.col_depth": "Ebene",
  "p.readings.col_tasks": "Aufgaben",
  "p.readings.col_finished": "Beendet",
  "p.readings.col_wip": "In Arbeit",
  "p.readings.col_efficiency": "Zeit mit Arbeiten statt Warten",
  "p.readings.effort_heading": "Erfasste Zeit",
  "p.readings.effort_says":
    "Die Zeit, die Personen für diesen Plan erfasst haben, wie angegeben; nichts prüft sie.",
  "p.readings.read_effort": "Erfasste Zeit lesen",
  "p.readings.effort_none": "Für diesen Plan ist keine Zeit erfasst.",
  "p.readings.effort_total": "{total} erfasst, davon {billable} abrechenbar.",
  "p.readings.effort_split":
    "Investitionsausgaben {capex}, Betriebsausgaben {opex}, nicht zugeordnet {unclassified}.",
  "p.readings.effort_by_person": "Zeit nach Person",
  "p.readings.effort_by_task": "Zeit nach Aufgabe",
  "p.readings.col_person": "Mitarbeiter",
  "p.readings.col_time": "Zeit",
  "p.readings.col_billable": "Abrechenbar",
  "p.readings.col_task": "Aufgabe",
  "p.readings.task_unnamed": "Eine hier nicht benannte Aufgabe ({ref})",
  "p.readings.forecast_heading": "Lieferprognose",
  "p.readings.forecast_says":
    "Zehntausend Durchläufe, gezogen aus dem, was dieser Plan in jedem vergangenen Zeitraum beendet hat: wie lange die offene Arbeit dauern kann und wie viel in einer Zahl von Zeiträumen fertig werden kann. Ein leeres Feld ist der Standard; mindestens sechs Zeiträume Verlauf sind nötig.",
  "p.readings.f_items": "Zu beendende Einträge (die offenen, wenn leer)",
  "p.readings.f_periods": "Zeiträume voraus ({n}, wenn leer)",
  "p.readings.f_period_days": "Tage pro Zeitraum ({n}, wenn leer)",
  "p.readings.f_history": "Zeiträume Verlauf ({n}, wenn leer)",
  "p.readings.read_forecast": "Prognose lesen",
  "p.readings.forecast_history_one":
    "Gezogen aus {n} Zeitraum von {days} Tagen.",
  "p.readings.forecast_history_other":
    "Gezogen aus {n} Zeiträumen von {days} Tagen.",
  "p.readings.forecast_batch_one":
    "{n} Eintrag: Die Hälfte der Durchläufe endet binnen {p50} Tagen, 85 von 100 binnen {p85} Tagen und 95 von 100 binnen {p95} Tagen.",
  "p.readings.forecast_batch_other":
    "{n} Einträge: Die Hälfte der Durchläufe endet binnen {p50} Tagen, 85 von 100 binnen {p85} Tagen und 95 von 100 binnen {p95} Tagen.",
  "p.readings.forecast_no_batch":
    "Wie lange die Arbeit dauern kann, lässt sich nicht prognostizieren: Es gibt zu wenig Verlauf, oder darin wurde nichts beendet.",
  "p.readings.forecast_horizon":
    "In den nächsten {days} Tagen können etwa {median} Einträge fertig werden: mindestens {least} in 85 von 100 Durchläufen und höchstens {most}.",
  "p.readings.forecast_no_horizon":
    "Wie viel fertig werden kann, lässt sich nicht prognostizieren: Es gibt zu wenig Verlauf, oder darin wurde nichts beendet.",
  "p.readings.performance_heading": "Termine, Kosten und Beteiligte",
  "p.readings.performance_says":
    "Die Terminleistung ist die geleistete Arbeit gegenüber der bisher geplanten, die Kostenleistung die geleistete Arbeit gegenüber ihren Kosten; über 100 % ist besser als geplant. Der Wert der Beteiligten reicht von minus 100 bis 100.",
  "p.readings.read_performance": "Leistung lesen",
  "p.readings.schedule": "Terminleistung",
  "p.readings.cost": "Kostenleistung",
  "p.readings.stakeholders": "Wert der Beteiligten",
  "p.readings.nps_one":
    "{score}, aus {n} Antwort ({promoters} fördernd, {detractors} ablehnend)",
  "p.readings.nps_other":
    "{score}, aus {n} Antworten ({promoters} fördernd, {detractors} ablehnend)",
  "p.readings.nps_none": "Noch keine Antworten.",
  "p.readings.earned_value_link": "Den Fertigstellungswert vollständig ansehen",
  "p.readings.absent_no_baseline":
    "Nicht gemessen: Der Plan hat keine Budget-Basislinie.",
  "p.readings.absent_no_weighted_tasks":
    "Nicht gemessen: Keine Aufgabe trägt Gewicht.",
  "p.readings.absent_not_yet_defined":
    "Noch nicht gemessen: Bisher wurde nichts geplant oder ausgegeben.",
  "p.readings.absent_several_currencies":
    "Kein einzelner Wert: Der Plan hat Budgets in mehreren Währungen; siehe den Fertigstellungswert.",
  "p.readings.absent_other": "Nicht gemessen.",
  "p.readings.workflow_heading": "Arbeitsablauf der Aufgaben",
  "p.readings.read_workflow": "Arbeitsablauf der Aufgaben lesen",
  "p.readings.workflow_plan":
    "Dieser Plan hat einen eigenen Arbeitsablauf für Aufgaben.",
  "p.readings.workflow_built_in":
    "Dieser Plan nutzt den eingebauten Arbeitsablauf für Aufgaben.",
  "p.readings.workflow_constrained":
    "Nur die aufgeführten Wechsel sind erlaubt.",
  "p.readings.workflow_free":
    "Eine Aufgabe kann von jedem Zustand in jeden anderen wechseln.",
  "p.readings.cat_todo": "nicht begonnen",
  "p.readings.cat_active": "in Arbeit",
  "p.readings.cat_waiting": "wartend",
  "p.readings.cat_done": "beendet",
  "p.readings.cat_other": "eine andere Art",
  "p.readings.initial": "hier beginnt eine Aufgabe",
  "p.readings.terminal": "hier endet eine Aufgabe",
  "p.readings.wip_limit": "höchstens {n} gleichzeitig",
  "p.readings.anyone": "jeder",
  "p.readings.roles": "Rollen: {roles}",
  "p.readings.problem_depth":
    "Ebenen tief ist eine ganze Zahl von 1 bis {max}.",
  "p.readings.problem_depth_zero":
    "Ebenen tief ist eine ganze Zahl von 0 bis {max}.",
  "p.readings.problem_items": "Zu beendende Einträge ist eine ganze Zahl ab 1.",
  "p.readings.problem_periods":
    "Zeiträume voraus ist eine ganze Zahl von 1 bis 520.",
  "p.readings.problem_period_days":
    "Tage pro Zeitraum ist eine ganze Zahl von 1 bis 90.",
  "p.readings.problem_history":
    "Zeiträume Verlauf ist eine ganze Zahl von 6 bis 260.",
  "p.readings.problem_window":
    "Das Fenster ist eine ganze Zahl von Tagen von 1 bis 3650.",
  "p.readings.refused_query": "Der Dienst hat diese Abfrage nicht angenommen.",
  "p.readings.refused_module": "Dieser Teil des Plans ist ausgeschaltet.",
  "p.readings.refused_gone":
    "Dieser Plan ist weg, oder Sie können ihn nicht mehr lesen.",
  "p.readings.flow_feature": "Neue Funktionen",
  "p.readings.flow_defect": "Fehler",
  "p.readings.flow_risk": "Risikoarbeit",
  "p.readings.flow_debt": "Technische Schulden",
  "p.readings.flow_unclassified": "Nicht zugeordnet",
  "p.readings.flow_other": "Eine andere Art",
  "p.flowmix.heading": "Die Mischung der beendeten Arbeit",
  "p.flowmix.says":
    "Wie sich die in einem Zeitfenster beendete Arbeit auf neue Funktionen, Fehler, Risikoarbeit und technische Schulden verteilt, verglichen mit der vom Plan beabsichtigten Mischung, wo er eine angegeben hat. Das Fenster ist {n} Tage, wenn leer gelassen.",
  "p.flowmix.f_window": "Tage zurück ({n}, wenn leer)",
  "p.flowmix.f_rollup": "Die enthaltenen Pläne einbeziehen",
  "p.flowmix.f_depth": "Ebenen tief ({max}, wenn leer)",
  "p.flowmix.read": "Mischung lesen",
  "p.flowmix.window_one": "Arbeit, beendet am letzten {n} Tag.",
  "p.flowmix.window_other": "Arbeit, beendet in den letzten {n} Tagen.",
  "p.flowmix.plans_one": "{n} Plan gezählt.",
  "p.flowmix.plans_other": "{n} Pläne gezählt.",
  "p.flowmix.truncated":
    "Nicht jeder Plan darunter wurde gezählt: Der Durchlauf hat seine Grenze erreicht.",
  "p.flowmix.none": "In diesem Fenster wurde keine Arbeit beendet.",
  "p.flowmix.no_intent":
    "Der Plan gibt keine beabsichtigte Mischung an, es gibt also nichts zu vergleichen.",
  "p.flowmix.table": "Beendete Arbeit nach Art",
  "p.flowmix.col_kind": "Art der Arbeit",
  "p.flowmix.col_count": "Beendet",
  "p.flowmix.col_share": "Anteil",
  "p.flowmix.col_intended": "Beabsichtigter Anteil",
  "p.flowmix.col_gap": "Abweichung",
  "p.ccpm.heading": "Aggressive und sichere Schätzungen",
  "p.ccpm.says":
    "Für die kritische Kette braucht jede Aufgabe eine aggressive Schätzung (etwa gleiche Chancen) und eine sichere, in ganzen Arbeitstagen von 1 bis {max}, die aggressive nicht über der sicheren. Ein gezeigter Wert ist der erfasste, sonst der aus der Risikospanne, sonst einer, den die Regel der Installation ableitet, und jeder sagt, welcher. Ein Meilenstein und eine Aufgabe ohne Dauer werden nicht geschätzt. Beide zu leeren entfernt das Erfasste.",
  "p.ccpm.rule_explicit_only":
    "Regel: Nur angegebene Schätzungen zählen; nichts wird abgeleitet.",
  "p.ccpm.rule_half_of_safe":
    "Regel: Eine fehlende sichere Schätzung ist die Dauer der Aufgabe, eine fehlende aggressive die Hälfte der sicheren.",
  "p.ccpm.rule_other": "Regel: eine, die diese Seite nicht kennt.",
  "p.ccpm.rule_unusable":
    "Die Regeleinstellung der Installation war nicht lesbar, daher zählen nur angegebene Schätzungen.",
  "p.ccpm.attention_one": "{n} Aufgabe braucht ihre Schätzungen.",
  "p.ccpm.attention_other": "{n} Aufgaben brauchen ihre Schätzungen.",
  "p.ccpm.more": "Die ersten {shown} von {total} Aufgaben werden gezeigt.",
  "p.ccpm.no_tasks": "Dieser Plan hat keine zu schätzenden Aufgaben.",
  "p.ccpm.table": "Schätzungen nach Aufgabe",
  "p.ccpm.col_task": "Aufgabe",
  "p.ccpm.col_likely": "Dauer in Arbeitstagen",
  "p.ccpm.col_aggressive": "Aggressiv",
  "p.ccpm.col_safe": "Sicher",
  "p.ccpm.col_state": "Stand",
  "p.ccpm.figure": "{days} Tage, {source}",
  "p.ccpm.none": "keine",
  "p.ccpm.source_recorded": "erfasst",
  "p.ccpm.source_risk_range": "aus der Risikospanne",
  "p.ccpm.source_rule": "nach der Regel",
  "p.ccpm.state_complete": "Beide angegeben",
  "p.ccpm.state_derived": "Teilweise abgeleitet",
  "p.ccpm.state_missing": "Eine Schätzung fehlt",
  "p.ccpm.state_inconsistent": "Aggressiv über sicher",
  "p.ccpm.state_milestone": "Ein Meilenstein, nicht geschätzt",
  "p.ccpm.state_no_duration": "Keine Dauer, nicht geschätzt",
  "p.ccpm.state_other": "Ein anderer Stand",
  "p.ccpm.form": "Schätzungen einer Aufgabe erfassen",
  "p.ccpm.f_task": "Aufgabe",
  "p.ccpm.choose_task": "Aufgabe wählen",
  "p.ccpm.f_aggressive": "Aggressiv, in Arbeitstagen",
  "p.ccpm.f_safe": "Sicher, in Arbeitstagen",
  "p.ccpm.save": "Schätzungen speichern",
  "p.ccpm.unchanged": "Nichts geändert, daher nichts gesendet.",
  "p.ccpm.saved": "Schätzungen für {title} gespeichert.",
  "p.ccpm.cleared": "Erfasste Schätzungen für {title} geleert.",
  "p.ccpm.confirm_clear":
    "Die erfassten Schätzungen für {title} leeren? Ein Wert aus der Risikospanne oder der Regel kann weiter angezeigt werden.",
  "p.ccpm.problem_task": "Wählen Sie eine Aufgabe.",
  "p.ccpm.problem_aggressive":
    "Die aggressive Schätzung ist eine ganze Zahl von Arbeitstagen von 1 bis {max}.",
  "p.ccpm.problem_safe":
    "Die sichere Schätzung ist eine ganze Zahl von Arbeitstagen von 1 bis {max}.",
  "p.ccpm.problem_order":
    "Die aggressive Schätzung darf nicht über der sicheren liegen.",
  "p.ccpm.refused_invalid":
    "Der Dienst hat diese Schätzungen nicht angenommen: Die Aufgabe hat vielleicht ihre Dauer verloren oder ist ein Meilenstein geworden.",
  "p.ccpm.refused_gone":
    "Diese Aufgabe ist weg, oder Sie können sie nicht mehr lesen.",
  "p.ccpm.could_not_load": "Die Schätzungen konnten nicht gelesen werden.",
  "p.ccpm.could_not_save": "Die Schätzungen konnten nicht gespeichert werden.",
  "p.frozen.heading": "Der eingefrorene Pufferplan",
  "p.frozen.none": "Es wurde noch kein Pufferplan eingefroren.",
  "p.frozen.freeze_link": "Einen in der Pufferverwaltung einfrieren",
  "p.frozen.taken": "Eingefroren am {date}; {rule}",
  "p.frozen.table": "Puffer wie eingefroren",
  "p.frozen.col_buffer": "Puffer",
  "p.frozen.col_days": "Arbeitstage",
  "p.frozen.col_chain": "Kette",
  "p.frozen.col_finish": "Geplantes Ende",
  "p.frozen.project": "Projektpuffer",
  "p.frozen.feeding": "Zubringerpuffer in {joins}",
  "p.frozen.unsized": "nicht bemessen",
  "p.frozen.untitled": "Eine Aufgabe ohne Titel",
  "p.frozen.history_one": "{n} Pufferplan eingefroren: {dates}.",
  "p.frozen.history_other":
    "{n} Pufferpläne eingefroren, neueste zuerst: {dates}.",
  "p.frozen.could_not_load":
    "Der eingefrorene Pufferplan konnte nicht gelesen werden.",
  "p.workingtime.heading": "Arbeitstag und Arbeitswoche",
  "p.workingtime.says":
    "Wie lang ein Arbeitstag ist (1 Minute bis 24 Stunden) und an wie vielen Tagen pro Woche gearbeitet wird (1 bis 7), für diesen Plan oder als Standard der Organisation. Jeder Eintrag ist neu und lässt sich hier nicht ändern oder entfernen, daher wird zuerst gefragt; ein zweiter für diesen Plan wird abgelehnt.",
  "p.workingtime.form": "Arbeitstag und Arbeitswoche festlegen",
  "p.workingtime.f_scope": "Für",
  "p.workingtime.scope_plan": "Diesen Plan",
  "p.workingtime.scope_default": "Den Standard der Organisation",
  "p.workingtime.f_hours": "Stunden pro Tag",
  "p.workingtime.f_minutes": "Und Minuten",
  "p.workingtime.f_days": "Tage pro Woche",
  "p.workingtime.save": "Arbeitstag festlegen",
  "p.workingtime.confirm_plan_one":
    "Den Arbeitstag dieses Plans auf {day} festlegen, {days} Tag pro Woche? Danach lässt er sich hier nicht ändern.",
  "p.workingtime.confirm_plan_other":
    "Den Arbeitstag dieses Plans auf {day} festlegen, {days} Tage pro Woche? Danach lässt er sich hier nicht ändern.",
  "p.workingtime.confirm_default_one":
    "Den Standard-Arbeitstag der Organisation auf {day} festlegen, {days} Tag pro Woche? Danach lässt er sich hier nicht ändern.",
  "p.workingtime.confirm_default_other":
    "Den Standard-Arbeitstag der Organisation auf {day} festlegen, {days} Tage pro Woche? Danach lässt er sich hier nicht ändern.",
  "p.workingtime.saved_one":
    "Arbeitstag von {day} festgelegt, {days} Tag pro Woche.",
  "p.workingtime.saved_other":
    "Arbeitstag von {day} festgelegt, {days} Tage pro Woche.",
  "p.workingtime.away_heading": "Abwesenheit von der Projektarbeit",
  "p.workingtime.away_says":
    "Urlaub, ein Feiertag, Bildungsurlaub oder eine Pflicht außerhalb des Projekts, für eine Person von einem ersten bis zu einem letzten Tag, damit diese Zeit nicht als Leerlauf gilt. Die Person ist eine Referenz wie person: und eine Kennung. Einmal erfasst, lässt sie sich nicht ändern oder entfernen, daher wird zuerst gefragt; eine leere Notiz wird nicht gesendet.",
  "p.workingtime.f_person": "Referenz der Person",
  "p.workingtime.f_starts": "Erster Tag",
  "p.workingtime.f_ends": "Letzter Tag",
  "p.workingtime.f_kind": "Art",
  "p.workingtime.f_note": "Notiz (optional)",
  "p.workingtime.record": "Abwesenheit erfassen",
  "p.workingtime.confirm_away":
    "{kind} für {person} erfassen, {range}? Danach lässt sich das nicht ändern oder entfernen.",
  "p.workingtime.recorded": "{kind} für {person} erfasst, {range}.",
  "p.workingtime.kind_leave": "Urlaub",
  "p.workingtime.kind_holiday": "einen Feiertag",
  "p.workingtime.kind_study_leave": "Bildungsurlaub",
  "p.workingtime.kind_non_project_duty": "eine Pflicht außerhalb des Projekts",
  "p.workingtime.problem_hours": "Stunden sind eine ganze Zahl von 0 bis 24.",
  "p.workingtime.problem_minutes": "Minuten sind eine ganze Zahl von 0 bis 59.",
  "p.workingtime.problem_day": "Ein Arbeitstag dauert 1 Minute bis 24 Stunden.",
  "p.workingtime.problem_week":
    "Tage pro Woche sind eine ganze Zahl von 1 bis 7.",
  "p.workingtime.problem_person":
    "Geben Sie die Person als person: oder worker: und eine Kennung an.",
  "p.workingtime.problem_starts": "Geben Sie den ersten Tag an.",
  "p.workingtime.problem_ends": "Geben Sie den letzten Tag an.",
  "p.workingtime.problem_order":
    "Der letzte Tag darf nicht vor dem ersten liegen.",
  "p.workingtime.refused_exists":
    "Für diesen Plan ist bereits ein Arbeitstag festgelegt, und er lässt sich hier nicht ersetzen.",
  "p.workingtime.refused_working":
    "Der Dienst hat diesen Arbeitstag nicht angenommen.",
  "p.workingtime.refused_non_working":
    "Der Dienst hat diese Abwesenheit nicht angenommen.",
  "p.workingtime.refused_forbidden": "Sie dürfen das nicht erfassen.",
  "p.workingtime.could_not_save": "Das konnte nicht erfasst werden.",
  "p.task.is_hammock":
    "Diese Aufgabe ist ein Sammelvorgang: Ihre Termine kommen aus den Aufgaben, die sie umfasst.",
  "p.task.hammock_link": "Ihre Spanne auf dem kritischen Pfad ansehen",
  "p.adoption.could_not": "Die Nutzung konnte nicht erfasst werden.",
  "p.adoption.f_active": "Aktive Nutzer",
  "p.adoption.f_definition": "Was als aktiver Nutzer zählte",
  "p.adoption.f_target": "Vorgesehene Nutzer",
  "p.adoption.f_window": "Über wie viele Tage",
  "p.adoption.form": "Nutzung erfassen",
  "p.adoption.heading": "Wie viele Menschen es nutzen",
  "p.adoption.problem_active":
    "Geben Sie die aktiven Nutzer als ganze Zahl ab 0 an.",
  "p.adoption.problem_definition":
    "Geben Sie an, was als aktiver Nutzer zählte, damit spätere Werte vergleichbar sind.",
  "p.adoption.problem_target":
    "Geben Sie die vorgesehenen Nutzer als ganze Zahl über 0 an.",
  "p.adoption.problem_window":
    "Geben Sie den Zeitraum als ganze Zahl von Tagen ab 1 an.",
  "p.adoption.record": "Nutzung erfassen",
  "p.adoption.refused": "Der Dienst hat diese Zahlen nicht angenommen.",
  "p.adoption.refused_forbidden":
    "Sie dürfen für diesen Plan keine Nutzung erfassen.",
  "p.adoption.refused_gone": "Dieser Plan existiert nicht mehr.",
  "p.adoption.saved": "Nutzung erfasst.",
  "p.adoption.says":
    "Aktive Nutzer im Verhältnis zu den vorgesehenen Nutzern über einen Zeitraum von Tagen. Was als aktiv zählte, wird neben der Zahl gespeichert, damit ein späterer Wert vergleichbar ist.",
  "p.bulk.apply": "Für die gewählten Aufgaben setzen",
  "p.bulk.choose_all": "Alle Aufgaben wählen",
  "p.bulk.choose_none": "Keine wählen",
  "p.bulk.choose_plan": "Plan wählen",
  "p.bulk.chosen_one": "{n} Aufgabe gewählt",
  "p.bulk.chosen_other": "{n} Aufgaben gewählt",
  "p.bulk.confirm_delete_one":
    "{n} Aufgabe löschen? Sie verschwindet vom Board und aus allen Planansichten.",
  "p.bulk.confirm_delete_other":
    "{n} Aufgaben löschen? Sie verschwinden vom Board und aus allen Planansichten.",
  "p.bulk.confirm_move_one":
    "{n} Aufgabe nach {plan} verschieben? Sprint, Komponente und übergeordnete Aufgabe werden geleert, und ein Status, den jener Plan nicht kennt, wird zu dessen erstem.",
  "p.bulk.confirm_move_other":
    "{n} Aufgaben nach {plan} verschieben? Sprint, Komponente und übergeordnete Aufgabe werden geleert, und ein Status, den jener Plan nicht kennt, wird zu dessen erstem.",
  "p.bulk.copy": "In den Plan kopieren",
  "p.bulk.could_not": "Die Aufgaben konnten nicht geändert werden.",
  "p.bulk.could_not_load_plans": "Die Pläne konnten nicht gelesen werden.",
  "p.bulk.delete": "Gewählte Aufgaben löschen",
  "p.bulk.delete_failed": "Nicht gelöscht: {titles}.",
  "p.bulk.deleted_gone_one": "{n} Aufgabe war schon weg.",
  "p.bulk.deleted_gone_other": "{n} Aufgaben waren schon weg.",
  "p.bulk.deleted_one": "{n} Aufgabe gelöscht.",
  "p.bulk.deleted_other": "{n} Aufgaben gelöscht.",
  "p.bulk.done_one": "Erledigt für {done} von {requested} Aufgabe.",
  "p.bulk.done_other": "Erledigt für {done} von {requested} Aufgaben.",
  "p.bulk.edit_form": "Felder der gewählten Aufgaben setzen",
  "p.bulk.f_assignee": "Zuständig",
  "p.bulk.f_due": "Fälligkeitsdatum",
  "p.bulk.f_plan": "Ziel-Plan",
  "p.bulk.f_start": "Startdatum",
  "p.bulk.f_value": "Neu: {field}",
  "p.bulk.heading": "Viele Aufgaben auf einmal ändern",
  "p.bulk.keep_assignee": "Beim Kopieren die Zuständigkeit behalten",
  "p.bulk.mapped_one":
    "{n} Aufgabe hatte einen Status, den jener Plan nicht kennt, und beginnt in dessen erstem Status.",
  "p.bulk.mapped_other":
    "{n} Aufgaben hatten einen Status, den jener Plan nicht kennt, und beginnen in dessen erstem Status.",
  "p.bulk.mode_clear": "Leeren",
  "p.bulk.mode_keep": "Unverändert lassen",
  "p.bulk.mode_set": "Setzen auf",
  "p.bulk.move": "In den Plan verschieben",
  "p.bulk.no_tasks": "Dieser Plan hat keine Aufgaben.",
  "p.bulk.problem_assignee":
    "Geben Sie die Zuständigkeit als person: oder worker: mit einer ID an.",
  "p.bulk.problem_due": "Geben Sie ein gültiges Fälligkeitsdatum an.",
  "p.bulk.problem_no_field":
    "Wählen Sie mindestens ein Feld zum Setzen oder Leeren.",
  "p.bulk.problem_none": "Wählen Sie mindestens eine Aufgabe.",
  "p.bulk.problem_plan":
    "Wählen Sie den Plan, in den kopiert oder verschoben wird.",
  "p.bulk.problem_same_plan":
    "Die Aufgaben sind schon in diesem Plan: Wählen Sie einen anderen als Ziel.",
  "p.bulk.problem_start":
    "Geben Sie ein gültiges Startdatum von {min} bis {latest} an.",
  "p.bulk.problem_too_many": "Wählen Sie höchstens {max} Aufgaben auf einmal.",
  "p.bulk.refused_edit": "Der Dienst hat diese Werte nicht angenommen.",
  "p.bulk.refused_forbidden": "Sie dürfen diese Aufgaben nicht ändern.",
  "p.bulk.refused_plan_gone":
    "Jener Plan existiert nicht mehr oder ist für Sie nicht sichtbar.",
  "p.bulk.refused_titles":
    "Abgelehnt, aus einem Grund, den der Dienst nur auf Englisch nennt: {titles}.",
  "p.bulk.refused_transfer":
    "Jener Plan ist für Sie nicht sichtbar, oder der Dienst hat die Liste nicht angenommen.",
  "p.bulk.says":
    "Wählen Sie Aufgaben und setzen Sie für alle dieselbe Zuständigkeit, Fälligkeit oder denselben Start, kopieren oder verschieben Sie sie in einen Plan, oder löschen Sie sie. Höchstens {max} auf einmal; der Start liegt zwischen {min} und {latest}. Der Status wird auf dem Board geändert, Karte für Karte. Eine Kopie beginnt im ersten Status jenes Plans und behält die Zuständigkeit nur auf Wunsch; beim Verschieben werden Sprint, Komponente und übergeordnete Aufgabe geleert, und eine Aufgabe mit Unteraufgaben wird nicht verschoben.",
  "p.bulk.this_plan": "{name} (dieser Plan)",
  "p.bulk.transfer_form": "Gewählte Aufgaben kopieren oder verschieben",
  "p.bulk.unchanged":
    "Die gewählten Aufgaben haben diese Werte schon, daher wurde nichts gesendet.",
  "p.ceremony.choose_kind": "Zeremonie wählen",
  "p.ceremony.could_not_load":
    "Die Zeremonien des Sprints konnten nicht gelesen werden.",
  "p.ceremony.could_not_save": "Die Zeremonie konnte nicht erfasst werden.",
  "p.ceremony.counts": "Abgehaltene Zeremonien",
  "p.ceremony.f_facilitator":
    "Moderation (person: oder worker: mit ID, freiwillig)",
  "p.ceremony.f_kind": "Zeremonie",
  "p.ceremony.f_summary": "Was entschieden wurde (freiwillig)",
  "p.ceremony.facilitated_by": "moderiert von {ref}",
  "p.ceremony.form": "Zeremonie erfassen",
  "p.ceremony.heading": "Zeremonien von {name}",
  "p.ceremony.held_count_one": "{kind}: {n}-mal abgehalten",
  "p.ceremony.held_count_other": "{kind}: {n}-mal abgehalten",
  "p.ceremony.held_saved": "Erfasst: {kind}.",
  "p.ceremony.kind_daily": "Tägliches Stand-up",
  "p.ceremony.kind_other": "Andere Zeremonie",
  "p.ceremony.kind_planning": "Planung",
  "p.ceremony.kind_retrospective": "Retrospektive",
  "p.ceremony.kind_review": "Sprint-Review",
  "p.ceremony.none_held": "{kind}: nicht abgehalten",
  "p.ceremony.problem_facilitator":
    "Geben Sie die Moderation als person: oder worker: mit ID an, oder lassen Sie das Feld leer.",
  "p.ceremony.problem_kind": "Wählen Sie die Zeremonie.",
  "p.ceremony.problem_once":
    "Dieser Sprint hat {kind} schon abgehalten: ein zweites Mal ist eine Neuplanung, also ein neuer Sprint.",
  "p.ceremony.record": "Zeremonie eintragen",
  "p.ceremony.refused_ceremony":
    "Der Dienst hat abgelehnt: Dieser Sprint hat diese Zeremonie vielleicht schon abgehalten.",
  "p.ceremony.refused_forbidden":
    "Sie dürfen dies für den Sprint nicht erfassen.",
  "p.ceremony.refused_gone": "Dieser Sprint existiert nicht mehr.",
  "p.ceremony.says":
    "Ein Sprint hat eine Planung und ein Review sowie beliebig viele tägliche Stand-ups und Retrospektiven. Jede Art wird gezählt, auch wenn keine stattfand.",
  "p.checkins.heading": "Stände zu {title}",
  "p.checkins.none": "Noch keine Stände.",
  "p.checkins.row": "{date}: Wert {value}",
  "p.commitment.added_one": "{n} Aufgabe seither hinzugekommen: {titles}.",
  "p.commitment.added_other": "{n} Aufgaben seither hinzugekommen: {titles}.",
  "p.commitment.commit": "Aufgaben des Sprints zusagen",
  "p.commitment.confirm_one":
    "Die {n} Aufgabe in {name} jetzt zusagen? Das geschieht einmal und lässt sich nicht ändern; später hinzugefügte oder entfernte Aufgaben erscheinen als Änderung.",
  "p.commitment.confirm_other":
    "Die {n} Aufgaben in {name} jetzt zusagen? Das geschieht einmal und lässt sich nicht ändern; später hinzugefügte oder entfernte Aufgaben erscheinen als Änderung.",
  "p.commitment.could_not": "Der Sprint konnte nicht zugesagt werden.",
  "p.commitment.heading": "Wozu sich der Sprint verpflichtet hat",
  "p.commitment.not_yet_one":
    "Noch nicht zugesagt; jetzt ist {n} Aufgabe im Sprint.",
  "p.commitment.not_yet_other":
    "Noch nicht zugesagt; jetzt sind {n} Aufgaben im Sprint.",
  "p.commitment.problem_done":
    "Dieser Sprint ist schon zugesagt: Eine Zusage wird nie neu geschrieben.",
  "p.commitment.problem_empty": "Dieser Sprint hat keine Aufgaben zum Zusagen.",
  "p.commitment.problem_unread":
    "Die Zusage des Sprints wurde noch nicht gelesen.",
  "p.commitment.refused_commit":
    "Der Dienst hat abgelehnt: Dieser Sprint ist schon zugesagt.",
  "p.commitment.removed_one": "{n} Aufgabe seither entfernt: {titles}.",
  "p.commitment.removed_other": "{n} Aufgaben seither entfernt: {titles}.",
  "p.commitment.saved_one": "{n} Aufgabe zugesagt.",
  "p.commitment.saved_other": "{n} Aufgaben zugesagt.",
  "p.commitment.says":
    "Die Aufgaben des Sprints zum Zeitpunkt der Zusage, einmal festgehalten, damit eine später hinzugefügte oder entfernte Aufgabe als Änderung erscheint und nicht als von Anfang an geplant.",
  "p.commitment.state": "{committed} zugesagt; jetzt {current} im Sprint.",
  "p.commitment.unchanged": "Seither nichts hinzugefügt oder entfernt.",
  "p.hours.col_actual": "Erfasst",
  "p.hours.col_placeholder": "Davon ohne Person",
  "p.hours.col_plan": "Plan des Projekts",
  "p.hours.col_planned": "Geplant",
  "p.hours.col_variance": "Erfasst minus geplant",
  "p.hours.could_not": "Die Stunden konnten nicht gelesen werden.",
  "p.hours.f_from": "Ab",
  "p.hours.f_to": "Bis",
  "p.hours.form": "Geplante gegenüber erfassten Stunden",
  "p.hours.fte_says":
    "Vollzeitäquivalente sind Prozent-Tage geteilt durch 100 mal die Tage des Zeitraums; eine Lücke ist Angebot minus Bedarf (unter null fehlt etwas). Vorgeschlagene Buchungen sind eine eigene Reihe.",
  "p.hours.h": "{h} Std.",
  "p.hours.heading": "Geplante und erfasste Stunden",
  "p.hours.masked":
    "Personen werden nicht gezeigt: Sie dürfen sie nicht verwalten.",
  "p.hours.none":
    "Kein Plan hatte in diesem Zeitraum geplante oder erfasste Stunden.",
  "p.hours.nothing_recorded": "nichts erfasst",
  "p.hours.problem_from":
    "Geben Sie den ersten Tag als gültiges Datum an oder lassen Sie ihn leer.",
  "p.hours.problem_order": "Der letzte Tag kann nicht vor dem ersten liegen.",
  "p.hours.problem_span": "Der Zeitraum darf höchstens {max} Tage umfassen.",
  "p.hours.problem_to":
    "Geben Sie den letzten Tag als gültiges Datum an oder lassen Sie ihn leer.",
  "p.hours.read": "Stunden lesen",
  "p.hours.refused": "Der Dienst hat diesen Zeitraum nicht angenommen.",
  "p.hours.says":
    "Geplant ist der Anteil einer benannten, festen Buchung an jedem Arbeitstag; erfasst ist die eingetragene Zeit, genehmigt oder nicht. Lassen Sie die Tage leer für die vier Wochen bis heute; höchstens {max} Tage.",
  "p.hours.table": "Stunden je Plan",
  "p.hours.totals": "Geplant {planned}; erfasst {actual}.",
  "p.hours.truncated":
    "Nur die ersten Einträge wurden gelesen; die Summen können zu niedrig sein.",
  "p.hours.unplanned": "{plan}: {hours} erfasst",
  "p.hours.unplanned_heading": "Zeit von Personen ohne Buchung im Plan",
  "p.hours.unplanned_people_one": "({n} Person)",
  "p.hours.unplanned_people_other": "({n} Personen)",
  "p.hours.window": "Vom {from} bis {to}.",
  "p.kpireadings.col_standing": "Stellung",
  "p.kpireadings.counted": "gezählt",
  "p.kpireadings.replaced": "durch einen späteren Wert am selben Tag ersetzt",
  "p.kpireadings.says":
    "Alle Werte, die neuesten zuerst. Ein späterer Wert am selben Tag ersetzt einen früheren, der erhalten bleibt.",
  "p.live.burndown_says":
    "Verbleibende Aufgaben je Tag, aus den erfassten Abschlüssen; keine Ideallinie, nichts wird ergänzt.",
  "p.live.event_comment_added": "ein Kommentar hinzugefügt",
  "p.live.event_other": "eine Änderung",
  "p.live.event_raci_updated": "Zuständigkeiten geändert",
  "p.live.event_resync": "das Board vollständig neu gelesen",
  "p.live.event_task_created": "eine Aufgabe hinzugefügt",
  "p.live.event_task_moved": "eine Aufgabe verschoben",
  "p.live.event_task_updated": "eine Aufgabe geändert",
  "p.live.follow": "Änderungen live verfolgen",
  "p.live.heading": "Änderungen in Echtzeit",
  "p.live.last_change": "Zuletzt: {change} um {time}.",
  "p.live.says":
    "Solange verfolgt wird, wird das Board neu gelesen, sobald jemand hier eine Aufgabe hinzufügt, verschiebt oder ändert. Es endet, wenn Sie die Seite verlassen.",
  "p.live.state_connecting": "Verbindung wird aufgebaut…",
  "p.live.state_off": "Nicht verfolgt.",
  "p.live.state_on": "Wird verfolgt.",
  "p.live.state_retrying": "Verbindung verloren; neuer Versuch.",
  "p.live.state_stopped": "Beendet: Der Dienst sendet keine Änderungen.",
  "p.live.stop": "Verfolgen beenden",
  "p.live.velocity_says":
    "In jedem Sprint erledigte Aufgaben und Punkte, vom Board gezählt; dies ist kein Flussmaß.",
  "p.placeholder.choose": "Platzhalter wählen",
  "p.placeholder.confirm_over":
    "Die Zuweisung an {person} würde an manchen Tagen die Kapazität überschreiten. Trotzdem?",
  "p.placeholder.confirm_remove":
    "Diesen Platzhalter entfernen: {what}? Der Bedarf, für den er stand, entfällt mit.",
  "p.placeholder.could_not": "Der Platzhalter konnte nicht geändert werden.",
  "p.placeholder.f_person": "Person (person: oder worker: mit ID)",
  "p.placeholder.f_placeholder": "Platzhalter",
  "p.placeholder.form": "Platzhalter einer Person geben",
  "p.placeholder.give": "Der Person geben",
  "p.placeholder.given": "An {person} gegeben.",
  "p.placeholder.heading": "Platzhalter einer Person geben oder entfernen",
  "p.placeholder.not_given": "Nicht vergeben: nichts wurde geändert.",
  "p.placeholder.open": "offen",
  "p.placeholder.option": "{role} in {plan}, {percent}, {from} bis {to}",
  "p.placeholder.problem_choose": "Wählen Sie einen Platzhalter.",
  "p.placeholder.problem_person":
    "Geben Sie die Person als person: oder worker: mit ID an.",
  "p.placeholder.refused_forbidden":
    "Nur eine Administration oder Ressourcenleitung kann das.",
  "p.placeholder.refused_gone": "Diese Buchung existiert nicht mehr.",
  "p.placeholder.refused_named":
    "Diese Buchung nennt schon eine Person; die Liste wurde neu gelesen.",
  "p.placeholder.refused_person":
    "Der Dienst hat diese Person nicht angenommen.",
  "p.placeholder.remove": "Platzhalter entfernen",
  "p.placeholder.removed": "Platzhalter entfernt.",
  "p.placeholder.says":
    "Ein Platzhalter ist eine angefragte Rolle ohne Person. Ihn zu vergeben entscheidet die Ressourcenleitung; würde das die Kapazität überschreiten, werden Sie zuerst gefragt.",
  "p.planokr.as_of": "berechnet {at}",
  "p.planokr.choose_plan": "Plan wählen",
  "p.planokr.could_not": "Die Ziele des Plans konnten nicht gelesen werden.",
  "p.planokr.could_not_load_plans": "Die Pläne konnten nicht gelesen werden.",
  "p.planokr.currencies": "Schlüsselergebnisse in mehr als einer Währung",
  "p.planokr.f_plan": "Plan für Ziele",
  "p.planokr.form": "Zielwert eines Plans",
  "p.planokr.heading": "Zielwert eines Plans",
  "p.planokr.its_key_results": "Schlüsselergebnisse von {title}",
  "p.planokr.key_results_one": "{n} Schlüsselergebnis",
  "p.planokr.key_results_other": "{n} Schlüsselergebnisse",
  "p.planokr.left_out_one":
    "{n} Ziel ist nicht messbar und fließt nicht in den Wert ein.",
  "p.planokr.left_out_other":
    "{n} Ziele sind nicht messbar und fließen nicht in den Wert ein.",
  "p.planokr.no_objectives": "Kein Ziel ist mit diesem Plan verknüpft.",
  "p.planokr.not_measured":
    "Nicht gemessen: Kein verknüpftes Ziel ist bisher messbar.",
  "p.planokr.objective_not_measured": "nicht gemessen",
  "p.planokr.problem_plan": "Wählen Sie einen Plan.",
  "p.planokr.read": "Wert lesen",
  "p.planokr.refused_gone":
    "Dieser Plan oder dieses Ziel existiert nicht mehr.",
  "p.planokr.says":
    "Der Wert jedes mit dem Plan verknüpften Ziels, gewichtet nach der Stärke der Verknüpfung. Ein nicht messbares Ziel fließt nicht ein und zählt nie als 0 %.",
  "p.planokr.score": "Wert {score}",
  "p.planokr.unmeasurable_no_range":
    "kein Fortschritt: Start und Ziel sind gleich",
  "p.planokr.unmeasurable_no_tolerance":
    "kein Fortschritt: zu halten, aber ohne Toleranz",
  "p.planokr.unmeasurable_other": "kein Fortschritt berechenbar",
  "p.planokr.unmeasurable_overflow":
    "kein Fortschritt: die Zahlen sind zu groß",
  "p.planokr.unnamed": "Ein Ziel ({id})",
  "p.planokr.weight": "Gewicht {weight}",
  "p.rate.answer_person":
    "{rate} je Stunde am {date}, aus der eigenen Tabelle der Person, gültig seit {from}.",
  "p.rate.answer_role":
    "{rate} je Stunde am {date}, aus der Tabelle ihrer Rolle, gültig seit {from}.",
  "p.rate.could_not": "Der Satz konnte nicht gelesen werden.",
  "p.rate.f_date": "Am",
  "p.rate.f_person": "Person (Referenz)",
  "p.rate.f_table": "Tabelle",
  "p.rate.form": "Der geltende Kostensatz",
  "p.rate.heading": "Der geltende Kostensatz",
  "p.rate.none": "Für diese Person gilt am {date} kein Satz.",
  "p.rate.problem_date": "Geben Sie den Tag als gültiges Datum an.",
  "p.rate.problem_person": "Geben Sie die Referenz der Person an.",
  "p.rate.read": "Satz lesen",
  "p.rate.refused_hidden":
    "Kein für Sie sichtbarer Satz: Sätze sieht nur die Finanzabteilung.",
  "p.rate.refused_query": "Der Dienst hat diese Abfrage nicht angenommen.",
  "p.rate.says":
    "Der an einem Tag geltende Stundensatz einer Person: aus ihrer eigenen Tabelle, sonst aus der ihrer Rolle. Die Standardtabelle, wenn Sie keine andere nennen.",
  "p.skillgap.choose_plan": "Plan wählen",
  "p.skillgap.col_person": "Person (Referenz)",
  "p.skillgap.col_skill": "Fähigkeit",
  "p.skillgap.col_status": "Stand",
  "p.skillgap.could_not": "Die Kompetenzlücken konnten nicht gelesen werden.",
  "p.skillgap.count_covered_one": "{n} Fähigkeit vorhanden;",
  "p.skillgap.count_covered_other": "{n} Fähigkeiten vorhanden;",
  "p.skillgap.count_missing_one": "{n} Fähigkeit fehlt;",
  "p.skillgap.count_missing_other": "{n} Fähigkeiten fehlen;",
  "p.skillgap.count_unknown_one": "{n} unbekannt.",
  "p.skillgap.count_unknown_other": "{n} unbekannt.",
  "p.skillgap.f_plan": "Plan prüfen",
  "p.skillgap.form": "Kompetenzlücken eines Plans",
  "p.skillgap.heading": "Von den Buchungen verlangte Fähigkeiten",
  "p.skillgap.none": "Keine Buchung in diesem Plan verlangt eine Fähigkeit.",
  "p.skillgap.problem_plan": "Wählen Sie einen Plan.",
  "p.skillgap.read": "Kompetenzlücken lesen",
  "p.skillgap.says":
    "Jede von einer Buchung im Plan verlangte Fähigkeit im Vergleich zu dem, was die Person im Personaldienst hat. Unbekannt heißt, ihre Fähigkeiten waren nicht lesbar; ein Platzhalter hat noch keine Person und fehlt hier.",
  "p.skillgap.status_covered": "vorhanden",
  "p.skillgap.status_missing": "fehlt",
  "p.skillgap.status_unknown": "unbekannt",
  "p.skillgap.table": "Fähigkeiten je Buchung",
  "p.util.by_person": "Je Person",
  "p.util.by_team": "Das Team",
  "p.util.col_available": "Verfügbar",
  "p.util.col_away": "Abwesend",
  "p.util.col_effort": "Aufwand",
  "p.util.col_figure": "Auslastung",
  "p.util.col_person": "Person (Referenz)",
  "p.util.could_not": "Die Auslastung konnte nicht gelesen werden.",
  "p.util.every_plan": "Alle Pläne",
  "p.util.f_by": "Anzeigen",
  "p.util.f_plan": "Nur Plan",
  "p.util.f_window": "Tage zurück ab heute",
  "p.util.form": "Aufwand gegenüber Kapazität",
  "p.util.heading": "Aufwand gegenüber Kapazität",
  "p.util.none":
    "In diesem Zeitraum hat niemand Aufwand oder Abwesenheit erfasst.",
  "p.util.over": "an oder über der Kapazität: eine Warnung, keine Leistung",
  "p.util.people_counted_one": "{n} Person gezählt.",
  "p.util.people_counted_other": "{n} Personen gezählt.",
  "p.util.problem_window":
    "Geben Sie den Zeitraum als ganze Zahl von Tagen von 1 bis {max} an.",
  "p.util.read": "Auslastung lesen",
  "p.util.refused": "Der Dienst hat diese Abfrage nicht angenommen.",
  "p.util.says":
    "Erfasster Aufwand im Verhältnis zur erklärten Kapazität abzüglich Abwesenheit, über die {days} Tage bis heute, sofern Sie nicht 1 bis {max} wählen. An oder über der Kapazität ist eine Warnung vor dem Stau. Kleine Gruppen und kleine Werte werden zurückgehalten.",
  "p.util.suppression":
    "Zurückgehalten: eine Gruppe unter {people} Personen oder ein Wert unter {hours}.",
  "p.util.table": "Auslastung je Person",
  "p.util.team_detail": "{effort} von {available} verfügbar, {away} abwesend",
  "p.util.unavailable_all_away": "den ganzen Zeitraum abwesend, nicht 0 %",
  "p.util.unavailable_below_floor": "zu wenig für eine Angabe",
  "p.util.unavailable_no_capacity": "keine Kapazität erklärt",
  "p.util.unavailable_other": "nicht berichtbar",
  "p.util.unavailable_small_group": "zu wenige Personen für eine Anzeige",
  "p.util.window_one": "Über den letzten {n} Tag.",
  "p.util.window_other": "Über die letzten {n} Tage.",
  "p.jobs.title": "Importe und Exporte",
  "p.jobs.title_ppm": "Importe und Exporte — PPM",
  "p.jobs.intro":
    "Massenimporte und -exporte von Plänen, die neuesten zuerst. Ein Auftrag läuft im Hintergrund: Lesen Sie die Liste erneut, um den Fortschritt zu sehen.",
  "p.jobs.loading": "Wird geladen…",
  "p.jobs.could_not_load": "Die Aufträge konnten nicht geladen werden.",
  "p.jobs.none": "Es gibt keinen Auftrag, den Sie sehen können.",
  "p.jobs.refresh": "Erneut lesen",
  "p.jobs.filter_kind": "Art",
  "p.jobs.filter_status": "Zustand",
  "p.jobs.any": "beliebig",
  "p.jobs.kind_import": "Import",
  "p.jobs.kind_export": "Export",
  "p.jobs.fmt_jsonl": "JSON-Zeilen",
  "p.jobs.status_queued": "in der Warteschlange",
  "p.jobs.status_running": "läuft",
  "p.jobs.status_completed": "abgeschlossen",
  "p.jobs.status_completed_with_errors": "mit Fehlern abgeschlossen",
  "p.jobs.status_failed": "fehlgeschlagen",
  "p.jobs.status_cancelled": "abgebrochen",
  "p.jobs.status_unknown": "unbekannter Zustand",
  "p.jobs.started": "Begonnen {at}",
  "p.jobs.not_started": "Noch nicht begonnen",
  "p.jobs.finished": "beendet {at}",
  "p.jobs.progress": "{percent} erledigt",
  "p.jobs.rows":
    "{total} Zeilen: {created} angelegt, {updated} aktualisiert, {unchanged} unverändert, {review} zur Prüfung zurückgehalten, {failed} fehlgeschlagen.",
  "p.jobs.not_reconciled": "Diese Zahlen ergeben nicht die Gesamtzahl.",
  "p.jobs.held":
    "Zur Prüfung zurückgehaltene Zeilen stehen unten unter „Zur Prüfung zurückgehalten“.",
  "p.jobs.stopping":
    "Zum Anhalten aufgefordert: Er beendet die aktuelle Zeile und behält, was verarbeitet wurde.",
  "p.jobs.errors": "Fehlerbericht (CSV)",
  "p.jobs.download": "Export herunterladen",
  "p.jobs.cancel": "Abbrechen",
  "p.jobs.cancel_named": "Den {kind}-Auftrag abbrechen",
  "p.jobs.cancel_ask":
    "Diesen Auftrag abbrechen? Ein wartender Auftrag beginnt nicht; ein laufender beendet die aktuelle Zeile und behält, was verarbeitet wurde.",
  "p.jobs.nothing_to_cancel":
    "Nichts abzubrechen: Der Auftrag war bereits beendet oder bereits zum Anhalten aufgefordert.",
  "p.jobs.could_not_cancel": "Der Auftrag konnte nicht abgebrochen werden.",
  "p.jobs.gone": "Dieser Auftrag ist nicht mehr vorhanden.",
  "p.jobs.export_h": "Pläne exportieren",
  "p.jobs.export_rules":
    "Nur Pläne, die Sie lesen können, höchstens 100.000. Personenbezüge werden maskiert, es sei denn, Sie fordern einen vollständigen Export an, der das destruktive Recht erfordert.",
  "p.jobs.format": "Dateiformat",
  "p.jobs.full": "Vollständiger Export mit Personenbezügen",
  "p.jobs.kind_filter": "Planart (optional)",
  "p.jobs.name_filter": "Name enthält (optional)",
  "p.jobs.start_export": "Export starten",
  "p.jobs.export_forbidden":
    "Ein vollständiger Export erfordert das destruktive Recht.",
  "p.jobs.export_refused":
    "Der Dienst hat den Export abgelehnt: Grenzen Sie den Filter ein (höchstens 100.000 Pläne).",
  "p.jobs.could_not_export": "Der Export konnte nicht gestartet werden.",
  "p.jobs.import_h": "Pläne importieren",
  "p.jobs.import_rules":
    "Ein Plan pro Zeile (JSON-Zeilen) oder pro Reihe (CSV), höchstens 100.000 Zeilen; die Installation begrenzt die Dateigröße. Dieselbe Datei erneut zu senden startet nichts Neues.",
  "p.jobs.file": "Datei",
  "p.jobs.keyless":
    "Eine Zeile ohne Kennung, die wie ein vorhandener Plan aussieht,",
  "p.jobs.keyless_review": "wird zur Prüfung zurückgehalten",
  "p.jobs.keyless_create": "wird trotzdem angelegt",
  "p.jobs.start_import": "Import starten",
  "p.jobs.no_file": "Wählen Sie zuerst eine Datei.",
  "p.jobs.too_big":
    "Die Datei überschreitet die Größengrenze der Installation.",
  "p.jobs.import_refused":
    "Der Dienst hat die Datei abgelehnt: Sie ist leer, zu groß oder nicht im gewählten Format.",
  "p.jobs.could_not_import": "Der Import konnte nicht gestartet werden.",
  "p.jobs.queued_ok": "Gestartet: Der Auftrag steht oben in der Liste.",
  "p.jobs.replayed":
    "Diese Datei wurde bereits gesendet; ihr Auftrag steht in der Liste.",
  "p.jobs.review_h": "Zur Prüfung zurückgehalten",
  "p.jobs.review_intro":
    "Zeilen, die wie ein vorhandener Plan aussahen, werden zurückgehalten, nicht angelegt. Entscheiden Sie jede: als neuen Plan anlegen oder als Duplikat verwerfen. Das Zusammenführen mit dem ähnlichen Plan wird hier nicht angeboten: Verwerfen Sie sie und führen Sie die Pläne dann aus der Planliste zusammen.",
  "p.jobs.review_none": "Nichts ist zur Prüfung zurückgehalten.",
  "p.jobs.could_not_load_review":
    "Die zurückgehaltenen Zeilen konnten nicht geladen werden.",
  "p.jobs.review_line": "Zeile {line} der Datei",
  "p.jobs.review_matches": "Sie ähnelt:",
  "p.jobs.review_match": "{score} ähnlich, Zuverlässigkeit {confidence}",
  "p.jobs.confidence_high": "hoch",
  "p.jobs.confidence_medium": "mittel",
  "p.jobs.confidence_low": "niedrig",
  "p.jobs.confidence_unknown": "unbekannt",
  "p.jobs.review_unnamed": "(ohne Namen)",
  "p.jobs.review_create": "Sie ist neu: anlegen",
  "p.jobs.review_discard": "Sie ist ein Duplikat: verwerfen",
  "p.jobs.review_create_ask": "„{name}“ als neuen Plan anlegen?",
  "p.jobs.review_discard_ask":
    "„{name}“ verwerfen? Daraus wurde nichts angelegt.",
  "p.jobs.review_created": "„{name}“ angelegt.",
  "p.jobs.review_discarded": "„{name}“ verworfen.",
  "p.jobs.review_already": "Über diese Zeile wurde bereits entschieden.",
  "p.jobs.review_invalid":
    "Die zurückgehaltene Zeile ist kein gültiger Plan mehr und kann nicht angelegt werden.",
  "p.jobs.review_gone": "Diese Zeile ist nicht mehr vorhanden.",
  "p.jobs.could_not_decide":
    "Die Entscheidung konnte nicht gespeichert werden.",
  "p.jobs.review_show": "Ihre zurückgehaltenen Zeilen zeigen",
  "p.jobs.review_all": "Alle zurückgehaltenen Zeilen zeigen",
  "p.jobs.review_one_job": "Es werden die Zeilen eines Imports gezeigt.",
  "p.jobs.review_more": "Nur die ersten 500 werden aufgeführt.",
  "p.plan_export.record_heading": "Der Datensatz des Plans (JSON)",
  "p.plan_export.record_intro":
    "Alles, was über den Plan gespeichert ist, als JSON, für eine Akten- oder Auskunftsanfrage. Wo Ihr Zugriff Personen maskiert, sind Leitung und Eigentümer maskiert, und die Datei sagt es. Jeder solche Export wird im Prüfprotokoll des Plans vermerkt.",
  "p.plan_export.record_download": "Datensatz des Plans herunterladen",
  "p.template_sync.link": "Vorlagenänderungen",
  "p.template_sync.title": "Vorlagenänderungen",
  "p.template_sync.title_ppm": "Vorlagenänderungen — PPM",
  "p.template_sync.loading": "Wird geladen…",
  "p.template_sync.could_not_load":
    "Die Änderungen der Vorlage konnten nicht geladen werden.",
  "p.template_sync.not_from_template":
    "Dieser Plan wurde nicht aus einer gespeicherten Vorlage erstellt; es gibt nichts zu aktualisieren.",
  "p.template_sync.made_from": "Erstellt aus der Vorlage „{name}“.",
  "p.template_sync.versions":
    "Auf Version {synced} gebracht; die Vorlage ist bei Version {version}.",
  "p.template_sync.intro":
    "Jede Änderung unten stammt aus der Vorlage seit der letzten Aktualisierung dieses Plans. Haken Sie an, was übernommen werden soll; der Rest wartet. Eine Änderung an einer Aufgabe, die die Leute dieses Plans selbst bearbeitet oder entfernt haben, wird nur übernommen, wenn Sie auch „Die Bearbeitung dieses Plans überschreiben“ anhaken.",
  "p.template_sync.up_to_date":
    "Aktuell: Es gibt keine Änderung, die dieser Plan nicht übernommen hat.",
  "p.template_sync.kind_add": "hinzugefügt",
  "p.template_sync.kind_change": "geändert",
  "p.template_sync.kind_remove": "entfernt",
  "p.template_sync.kind_unknown": "unbekannte Änderung",
  "p.template_sync.take": "Diese Änderung übernehmen",
  "p.template_sync.edited":
    "Die Leute dieses Plans haben diese Aufgabe selbst bearbeitet oder entfernt.",
  "p.template_sync.overwrite": "Die Bearbeitung dieses Plans überschreiben",
  "p.template_sync.diff_title": "Titel: {from} → {to}",
  "p.template_sync.diff_description": "Beschreibung: {from} → {to}",
  "p.template_sync.diff_points": "Punkte: {from} → {to}",
  "p.template_sync.none_value": "keine",
  "p.template_sync.apply": "Die gewählten Änderungen übernehmen",
  "p.template_sync.apply_ask":
    "Die gewählten Änderungen in diesen Plan übernehmen? Aufgaben werden wie aufgeführt hinzugefügt, geändert oder entfernt, und die Änderung wird protokolliert.",
  "p.template_sync.choose_first":
    "Haken Sie zuerst mindestens eine Änderung an.",
  "p.template_sync.applied": "Übernommen: {n}.",
  "p.template_sync.overwritten":
    "Davon mit Überschreiben eigener Bearbeitungen dieses Plans: {n}.",
  "p.template_sync.remaining": "Noch wartend: {n}.",
  "p.template_sync.refused_edited":
    "„{title}“ wurde nicht übernommen: Dieser Plan hat sie bearbeitet; haken Sie „Die Bearbeitung dieses Plans überschreiben“ an, um sie trotzdem zu übernehmen.",
  "p.template_sync.refused_removed":
    "„{title}“ wurde nicht übernommen: Dieser Plan hat die Aufgabe entfernt; es gibt nichts zu ändern.",
  "p.template_sync.could_not_apply":
    "Die Änderungen konnten nicht übernommen werden.",
  "p.template_sync.apply_refused":
    "Der Dienst hat die Änderungen abgelehnt: Die Vorlage hat sich vielleicht inzwischen geändert. Lesen Sie die Änderungen erneut.",
  "p.template_sync.plans_h": "Daraus erstellte Pläne",
  "p.template_sync.plans_none":
    "Kein Plan, den Sie lesen können, wurde daraus erstellt.",
  "p.template_sync.plans_row": "Version {synced} von {version}",
  "p.template_sync.waiting": "wartende Änderungen: {n}",
  "p.template_sync.plan_up_to_date": "aktuell",
  "p.template_sync.open_changes": "Änderungen ansehen",
  "p.template_sync.could_not_load_plans":
    "Die daraus erstellten Pläne konnten nicht geladen werden.",
  "p.template_sync.revise_h": "Aus einem Plan überarbeiten",
  "p.template_sync.revise_says":
    "Die Aufgaben des gewählten Planbaums werden zur nächsten Version der Vorlage. Der Baum muss die Form der Vorlage behalten: dieselben Pläne an denselben Stellen, neue Pläne nur am Ende. Aus der Vorlage erstellte Pläne werden nicht geändert; jeder sieht den Unterschied auf seiner Seite Vorlagenänderungen.",
  "p.template_sync.revise_plan": "Planbaum",
  "p.template_sync.revise_choose": "Plan wählen",
  "p.template_sync.revise": "Vorlage überarbeiten",
  "p.template_sync.revise_ask":
    "Die Aufgaben von „{name}“ zur nächsten Version von „{template}“ machen?",
  "p.template_sync.revised": "Jetzt Version {version}.",
  "p.template_sync.revised_omitted":
    "Pläne im Baum, die Sie nicht lesen können, wurden ausgelassen: {n}.",
  "p.template_sync.revise_shape":
    "Der Planbaum hat nicht mehr die Form der Vorlage: Eine Überarbeitung kann Aufgaben ändern und Pläne am Ende anfügen, aber keine Pläne verschieben oder umbenennen.",
  "p.template_sync.could_not_revise":
    "Die Vorlage konnte nicht überarbeitet werden.",
  "p.template_sync.revise_pick": "Wählen Sie zuerst den Planbaum.",
  "p.mspdi_batch.title": "Mehrere MS-Project-Dateien importieren",
  "p.mspdi_batch.title_ppm": "Mehrere MS-Project-Dateien importieren — PPM",
  "p.mspdi_batch.intro":
    "Bis zu zehn MS-Project-Dateien (MSPDI-XML) auf einmal übernehmen. Jede kommt in einen Plan Ihrer Wahl oder in einen neuen Plan. Der erste Schritt ist immer eine Vorschau, die nichts schreibt; dieselbe Schaltfläche importiert dann.",
  "p.mspdi_batch.files": "Dateien (MS-Project-XML, höchstens zehn)",
  "p.mspdi_batch.too_many": "Wählen Sie höchstens zehn Dateien.",
  "p.mspdi_batch.none_chosen": "Wählen Sie zuerst mindestens eine Datei.",
  "p.mspdi_batch.target": "In",
  "p.mspdi_batch.target_new": "einen neuen Plan",
  "p.mspdi_batch.target_existing": "einen vorhandenen Plan",
  "p.mspdi_batch.new_name":
    "Name des neuen Plans (optional: der Projektname in der Datei)",
  "p.mspdi_batch.existing_plan": "Zielplan",
  "p.mspdi_batch.choose_plan": "Plan wählen",
  "p.mspdi_batch.choose_plan_first": "Wählen Sie zuerst den Plan für „{file}“.",
  "p.mspdi_batch.parent": "Neue Pläne einordnen unter (optional)",
  "p.mspdi_batch.no_parent": "kein Plan",
  "p.mspdi_batch.preview": "Import in der Vorschau ansehen",
  "p.mspdi_batch.import": "Importieren",
  "p.mspdi_batch.import_ask":
    "Die Dateien jetzt importieren? Pläne werden angelegt und Aufgaben wie in der Vorschau geschrieben. Das ist keine einzelne Transaktion: Ein fehlschlagendes Projekt stoppt den Rest, und bereits angelegte Pläne bleiben.",
  "p.mspdi_batch.start_over": "Andere Dateien wählen",
  "p.mspdi_batch.result_preview_h": "Vorschau",
  "p.mspdi_batch.result_done_h": "Importiert",
  "p.mspdi_batch.outcome_would_import": "würde importiert",
  "p.mspdi_batch.outcome_imported": "importiert",
  "p.mspdi_batch.outcome_failed": "fehlgeschlagen",
  "p.mspdi_batch.outcome_not_attempted": "nicht versucht",
  "p.mspdi_batch.outcome_unknown": "unbekanntes Ergebnis",
  "p.mspdi_batch.new_plan_note": "ein neuer Plan würde angelegt",
  "p.mspdi_batch.plan_created": "ein neuer Plan wurde angelegt",
  "p.mspdi_batch.failed_note":
    "Dieses Projekt ist fehlgeschlagen, daher wurden die folgenden nicht versucht. Korrigieren Sie die Datei und importieren Sie erneut: Vorhandene Pläne werden aktualisiert, nicht verdoppelt.",
  "p.mspdi_batch.pool_h": "Personen und Ressourcen in den Dateien",
  "p.mspdi_batch.pool_note":
    "Ressourcen werden über alle Dateien einmal aufgeführt, mit der Zahl ihrer Zuordnungen. Sie werden keinen Personen zugeordnet: Ein Name ist keine Identität.",
  "p.mspdi_batch.pool_row": "{name} — Zuordnungen: {assignments}",
  "p.mspdi_batch.links_h": "Verknüpfungen zwischen den Dateien",
  "p.mspdi_batch.links_would":
    "Verknüpfungen zwischen den Dateien, die angelegt würden: {n}.",
  "p.mspdi_batch.links_made":
    "Angelegte Verknüpfungen: {n}; schon vorhanden: {m}.",
  "p.mspdi_batch.links_unresolved":
    "Verknüpfungen nach außerhalb der Dateien oder zu einer fehlenden Aufgabe, nicht angelegt: {n}.",
  "p.mspdi_batch.links_after_failure":
    "Es wurden keine Verknüpfungen zwischen den Dateien angelegt, weil ein Projekt fehlgeschlagen ist.",
  "p.mspdi_batch.not_offered":
    "Hier nicht angeboten: die benutzerdefinierten Felder der Dateien zuordnen, Ressourcen Personen zuordnen und ihre Sätze importieren.",
  "p.mspdi_batch.could_not": "Der Import hat nicht funktioniert.",
  "p.mspdi_batch.refused":
    "Der Dienst hat die Dateien abgelehnt: Eine ist kein MS-Project-XML, ist zu groß oder nennt einen Plan, den Sie nicht sehen können.",
  "p.mspdi_batch.link_from_jobs": "Mehrere MS-Project-Dateien importieren",
  "p.mspdi_batch.read_failed": "„{file}“ konnte nicht gelesen werden.",
  "p.components.link": "Komponenten",
  "p.components.title": "Komponenten",
  "p.components.title_ppm": "Komponenten — PPM",
  "p.components.intro":
    "Komponenten gruppieren die Aufgaben eines Plans nach Teilen der Arbeit, zum Beispiel die API oder die mobile App. Eine Aufgabe gehört zu höchstens einer. Jede Aufgabe wird gezählt: Aufgaben ohne Komponente erscheinen als nicht zugeordnet.",
  "p.components.loading": "Wird geladen…",
  "p.components.could_not_load":
    "Die Komponenten konnten nicht geladen werden.",
  "p.components.none": "Dieser Plan hat noch keine Komponenten.",
  "p.components.unassigned": "Nicht zugeordnet",
  "p.components.counts": "{open} offen, {done} erledigt, {total} insgesamt",
  "p.components.lead": "Leitung: {lead}",
  "p.components.no_lead": "keine Leitung",
  "p.components.edit": "Ändern",
  "p.components.save": "Speichern",
  "p.components.cancel": "Abbrechen",
  "p.components.remove": "Entfernen",
  "p.components.remove_ask":
    "Die Komponente „{name}“ entfernen? Keine Aufgabe gehört zu ihr, daher ändert sich sonst nichts.",
  "p.components.in_use":
    "Noch gehörende Aufgaben: {n}. Verschieben Sie sie in eine andere Komponente, bevor Sie diese entfernen.",
  "p.components.in_use_refused":
    "Zu dieser Komponente gehören noch Aufgaben. Verschieben Sie sie zuerst in eine andere.",
  "p.components.add_h": "Komponente hinzufügen",
  "p.components.name": "Name (bis zu 120 Zeichen)",
  "p.components.lead_field":
    "Leitung (optional: ein person:- oder worker:-Bezug)",
  "p.components.add": "Komponente hinzufügen",
  "p.components.name_required": "Ein Name ist erforderlich.",
  "p.components.name_too_long": "Der Name ist länger als 120 Zeichen.",
  "p.components.lead_bad":
    "Die Leitung muss ein person:- oder worker:-Bezug sein.",
  "p.components.taken":
    "Eine Komponente mit diesem Namen gibt es im Plan schon.",
  "p.components.too_many": "Ein Plan hat höchstens 200 Komponenten.",
  "p.components.could_not_save":
    "Die Komponente konnte nicht gespeichert werden.",
  "p.components.could_not_remove":
    "Die Komponente konnte nicht entfernt werden.",
  "p.components.removed": "„{name}“ entfernt.",
  "p.components.saved": "„{name}“ gespeichert.",
  "p.components.added": "„{name}“ hinzugefügt.",
  "p.resource_roles.heading": "Hauptrollen der Personen",
  "p.resource_roles.says":
    "Die Hauptrolle einer Person entscheidet, in welcher Zeile oben ihre angegebene Kapazität als Angebot zählt. Eine festzulegen erfordert die Rolle Ressourcenmanager oder einen Administrator; alle anderen sehen nur ihre eigene.",
  "p.resource_roles.none": "Noch hat niemand eine Hauptrolle.",
  "p.resource_roles.person": "Person (ein person:- oder worker:-Bezug)",
  "p.resource_roles.role": "Rolle (bis zu 60 Zeichen; leer löscht sie)",
  "p.resource_roles.set": "Rolle festlegen",
  "p.resource_roles.form": "Hauptrolle einer Person festlegen",
  "p.resource_roles.person_bad":
    "Die Person muss ein person:- oder worker:-Bezug sein.",
  "p.resource_roles.role_too_long": "Die Rolle ist länger als 60 Zeichen.",
  "p.resource_roles.forbidden":
    "Eine Rolle festzulegen erfordert die Rolle Ressourcenmanager oder einen Administrator.",
  "p.resource_roles.could_not_load": "Die Rollen konnten nicht geladen werden.",
  "p.resource_roles.could_not_set": "Die Rolle konnte nicht festgelegt werden.",
  "p.resource_roles.set_ok": "{person} hat jetzt die Hauptrolle „{role}“.",
  "p.resource_roles.cleared_ok": "{person} hat keine Hauptrolle mehr.",
  "p.goals.link": "Ziele",
  "p.goals.title": "Ziele",
  "p.goals.title_ppm": "Ziele — PPM",
  "p.goals.intro":
    "Was dieser Plan erreichen will, der Reihe nach. Ein Ziel wird an seiner Stelle in der Liste erkannt, daher wird eine Änderung abgelehnt, wenn jemand anderes die Ziele geändert hat, nachdem diese Seite sie gelesen hat.",
  "p.goals.loading": "Ziele werden geladen…",
  "p.goals.could_not_load": "Die Ziele konnten nicht geladen werden.",
  "p.goals.none": "Dieser Plan hat noch keine Ziele.",
  "p.goals.target": "Zieldatum: {date}",
  "p.goals.status": "Stand: {status}",
  "p.goals.st.NotStarted": "Nicht begonnen",
  "p.goals.st.InProgress": "In Arbeit",
  "p.goals.st.Achieved": "Erreicht",
  "p.goals.st.Missed": "Verfehlt",
  "p.goals.edit": "Ändern",
  "p.goals.save": "Ziel speichern",
  "p.goals.cancel": "Abbrechen",
  "p.goals.remove": "Entfernen",
  "p.goals.remove_ask":
    "Das Ziel „{title}“ entfernen? Die Ziele danach rücken um eine Stelle nach oben.",
  "p.goals.add_h": "Ziel hinzufügen",
  "p.goals.title_field": "Titel (bis zu 512 Zeichen)",
  "p.goals.description_field": "Beschreibung (optional)",
  "p.goals.target_field": "Zieldatum (optional)",
  "p.goals.status_field": "Stand",
  "p.goals.status_none": "Kein Stand",
  "p.goals.status_custom": "Ein anderer Stand, unten benannt",
  "p.goals.custom_field": "Name des Stands",
  "p.goals.add": "Ziel hinzufügen",
  "p.goals.title_required": "Ein Titel ist erforderlich.",
  "p.goals.title_too_long": "Der Titel ist länger als 512 Zeichen.",
  "p.goals.date_bad": "Das Zieldatum muss ein echter Tag sein.",
  "p.goals.custom_required":
    "Benennen Sie den Stand oder wählen Sie einen anderen.",
  "p.goals.full": "Ein Plan enthält höchstens 256 Ziele.",
  "p.goals.changed":
    "Die Ziele haben sich geändert, nachdem diese Seite sie gelesen hat, daher wurde nichts gespeichert. Sie wurden neu gelesen: Prüfen Sie sie und versuchen Sie es erneut.",
  "p.goals.gone":
    "Dieses Ziel ist nicht mehr vorhanden. Die Ziele wurden neu gelesen.",
  "p.goals.could_not_save": "Das Ziel konnte nicht gespeichert werden.",
  "p.goals.could_not_remove": "Das Ziel konnte nicht entfernt werden.",
  "p.goals.added": "„{title}“ hinzugefügt.",
  "p.goals.saved": "„{title}“ gespeichert.",
  "p.goals.removed": "„{title}“ entfernt.",
  "p.custom_fields.title": "Benutzerdefinierte Felder",
  "p.custom_fields.title_ppm": "Benutzerdefinierte Felder — PPM",
  "p.custom_fields.intro":
    "Felder, die eine Administratorin oder ein Administrator jedem Plan oder jeder Aufgabe hinzufügt, jedes mit einem Typ, gegen den seine Werte geprüft werden. Die Werte liegen getrennt vom Plan und beeinflussen den Abgleich daher nie.",
  "p.custom_fields.loading": "Benutzerdefinierte Felder werden geladen…",
  "p.custom_fields.could_not_load":
    "Die benutzerdefinierten Felder konnten nicht geladen werden.",
  "p.custom_fields.for.plan": "Felder an Plänen",
  "p.custom_fields.for.task": "Felder an Aufgaben",
  "p.custom_fields.none": "Noch keine festgelegt.",
  "p.custom_fields.type.text": "Freitext",
  "p.custom_fields.type.date": "Datum",
  "p.custom_fields.type.bool": "Ja oder nein",
  "p.custom_fields.type.integer": "Ganze Zahl",
  "p.custom_fields.type.list": "Eine Auswahl aus einer Liste",
  "p.custom_fields.type.multi_list": "Mehrere Auswahlen aus einer Liste",
  "p.custom_fields.is_required": "erforderlich",
  "p.custom_fields.kinds_only": "nur für Pläne der Art: {kinds}",
  "p.custom_fields.choices": "Auswahlen: {options}",
  "p.custom_fields.withdraw": "Zurückziehen",
  "p.custom_fields.withdraw_ask":
    "Das Feld „{label}“ zurückziehen? Bereits gespeicherte Werte bleiben lesbar, als zurückgezogen markiert, und neue können nicht gesetzt werden.",
  "p.custom_fields.withdrawn": "„{label}“ zurückgezogen.",
  "p.custom_fields.could_not_withdraw":
    "Das Feld konnte nicht zurückgezogen werden.",
  "p.custom_fields.declare_h": "Ein Feld festlegen",
  "p.custom_fields.key_field":
    "Schlüssel (Kleinbuchstaben, Ziffern und Unterstriche, beginnend mit einem Buchstaben)",
  "p.custom_fields.label_field": "Bezeichnung (bis zu 100 Zeichen)",
  "p.custom_fields.type_field": "Art des Werts",
  "p.custom_fields.applies_field": "Gilt für",
  "p.custom_fields.applies.plan": "Pläne",
  "p.custom_fields.applies.task": "Aufgaben",
  "p.custom_fields.kinds_field":
    "Nur für diese Planarten (optional, durch Kommas getrennt)",
  "p.custom_fields.options_field":
    "Auswahlen, eine pro Zeile (bis zu 100, jede bis zu 100 Zeichen)",
  "p.custom_fields.required_field":
    "Jeder Plan bzw. jede Aufgabe muss einen Wert haben",
  "p.custom_fields.declare": "Feld festlegen",
  "p.custom_fields.key_bad":
    "Der Schlüssel muss aus Kleinbuchstaben, Ziffern und Unterstrichen bestehen, mit einem Buchstaben beginnen und höchstens 100 Zeichen lang sein.",
  "p.custom_fields.label_bad":
    "Eine Bezeichnung ist erforderlich, höchstens 100 Zeichen.",
  "p.custom_fields.options_none":
    "Ein Listenfeld braucht mindestens eine Auswahl.",
  "p.custom_fields.options_long":
    "Eine Liste enthält höchstens 100 Auswahlen, jede höchstens 100 Zeichen.",
  "p.custom_fields.options_twice": "Jede Auswahl darf nur einmal vorkommen.",
  "p.custom_fields.key_taken":
    "Für diese Art von Gegenstand gibt es bereits ein Feld mit diesem Schlüssel.",
  "p.custom_fields.could_not_declare":
    "Das Feld konnte nicht festgelegt werden.",
  "p.custom_fields.declared": "„{label}“ festgelegt.",
  "p.custom_fields.search_h": "Nach dem Wert eines Felds suchen",
  "p.custom_fields.field_field": "Feldname",
  "p.custom_fields.choose": "Auswählen…",
  "p.custom_fields.value_field": "Wert",
  "p.custom_fields.yes": "Ja",
  "p.custom_fields.no": "Nein",
  "p.custom_fields.search": "Suchen",
  "p.custom_fields.found.plan": "Passende Pläne, die Sie sehen können: {n}",
  "p.custom_fields.found.task": "Passende Aufgaben, die Sie sehen können: {n}",
  "p.custom_fields.truncated":
    "Für dieses Feld sind mehr als 5.000 Werte gespeichert; nur die ersten 5.000 wurden betrachtet.",
  "p.custom_fields.field_required": "Wählen Sie ein Feld.",
  "p.custom_fields.value_required": "Geben Sie einen Wert zum Suchen an.",
  "p.custom_fields.value_integer": "Der Wert muss eine ganze Zahl sein.",
  "p.custom_fields.value_date": "Der Wert muss ein echter Tag sein.",
  "p.custom_fields.could_not_search":
    "Das Feld konnte nicht durchsucht werden.",
  "p.custom_fields.link": "Benutzerdefinierte Felder",
  "p.custom_fields.values_title": "Benutzerdefinierte Felder dieses Plans",
  "p.custom_fields.values_title_ppm":
    "Benutzerdefinierte Felder des Plans — PPM",
  "p.custom_fields.values_intro":
    "Die Werte der Felder, die eine Administratorin oder ein Administrator für Pläne dieser Art festgelegt hat. Beim Speichern wird nur Geändertes gesendet; ist ein Wert falsch, wird nichts gespeichert.",
  "p.custom_fields.values_could_not_load":
    "Die benutzerdefinierten Felder dieses Plans konnten nicht geladen werden.",
  "p.custom_fields.values_missing": "Erforderliche Felder ohne Wert: {fields}",
  "p.custom_fields.values_none":
    "Für die Art dieses Plans gilt noch kein benutzerdefiniertes Feld. Felder werden festgelegt auf der Seite",
  "p.custom_fields.values_h": "Werte",
  "p.custom_fields.values_required_label": "{label} (erforderlich)",
  "p.custom_fields.values_unset": "Kein Wert",
  "p.custom_fields.values_save": "Werte speichern",
  "p.custom_fields.values_unchanged":
    "Nichts geändert, daher wurde nichts gesendet.",
  "p.custom_fields.values_saved": "Werte gespeichert.",
  "p.custom_fields.values_could_not_save":
    "Die Werte konnten nicht gespeichert werden.",
  "p.custom_fields.values_withdrawn_h":
    "Werte zurückgezogener Felder (nur lesbar)",
  "p.custom_fields.v_integer": "{label}: Der Wert muss eine ganze Zahl sein.",
  "p.custom_fields.v_date": "{label}: Der Wert muss ein echter Tag sein.",
  "p.custom_fields.v_choice":
    "{label}: Wählen Sie eine der Auswahlen des Felds.",
  "p.custom_fields.v_too_long":
    "{label}: Der Text ist länger als 1.000 Zeichen.",
  "p.custom_fields.v_required":
    "{label} ist erforderlich und darf nicht leer bleiben.",
  "p.onboarding_templates.title": "Checklisten-Vorlagen",
  "p.onboarding_templates.title_ppm": "Checklisten-Vorlagen — PPM",
  "p.onboarding_templates.intro":
    "Die Checkliste, mit der jeder Ein- oder Austrittsfall beginnt, mit der Fälligkeit jedes Punkts ab dem Tag des Inkrafttretens. Ein bereits begonnener Fall behält seine Punkte.",
  "p.onboarding_templates.loading": "Checklisten werden geladen…",
  "p.onboarding_templates.could_not_load":
    "Die Checklisten konnten nicht geladen werden.",
  "p.onboarding_templates.is_custom": "Diese Art hat eine eigene Checkliste.",
  "p.onboarding_templates.is_builtin":
    "Diese Art nutzt die eingebaute Checkliste; ihre Punkte stammen vom Dienst, auf Englisch.",
  "p.onboarding_templates.before": "Tage vor dem Datum: {n}",
  "p.onboarding_templates.after": "Tage nach dem Datum: {n}",
  "p.onboarding_templates.on_the_day": "am Datum",
  "p.onboarding_templates.edit": "Ändern",
  "p.onboarding_templates.reset": "Zur eingebauten Checkliste zurückkehren",
  "p.onboarding_templates.reset_ask":
    "Für „{kind}“ zur eingebauten Checkliste zurückkehren? Die eigene wird entfernt; bereits begonnene Fälle behalten ihre Punkte.",
  "p.onboarding_templates.reset_done":
    "„{kind}“ nutzt wieder die eingebaute Checkliste.",
  "p.onboarding_templates.could_not_reset":
    "Die Rückkehr zur eingebauten Checkliste ist fehlgeschlagen.",
  "p.onboarding_templates.item_title":
    "Punkt {n}: was zu tun ist (bis zu 200 Zeichen)",
  "p.onboarding_templates.item_offset":
    "Punkt {n}: Tage ab dem Datum (−365 bis 365; negativ heißt vorher)",
  "p.onboarding_templates.up": "Punkt {n} nach oben",
  "p.onboarding_templates.down": "Punkt {n} nach unten",
  "p.onboarding_templates.remove": "Punkt {n} entfernen",
  "p.onboarding_templates.add": "Punkt hinzufügen",
  "p.onboarding_templates.save": "Checkliste speichern",
  "p.onboarding_templates.cancel": "Abbrechen",
  "p.onboarding_templates.count_bad": "Eine Checkliste hat 1 bis {max} Punkte.",
  "p.onboarding_templates.title_bad":
    "Punkt {n}: Ein Titel ist erforderlich, bis zu 200 Zeichen.",
  "p.onboarding_templates.offset_bad":
    "Punkt {n}: Die Tage müssen eine ganze Zahl von −365 bis 365 sein.",
  "p.onboarding_templates.saved": "Checkliste für „{kind}“ gespeichert.",
  "p.onboarding_templates.could_not_save":
    "Die Checkliste konnte nicht gespeichert werden.",
  "p.onboarding_templates.forbidden":
    "Eine Checkliste zu ändern erfordert die Berechtigung, Personen zu verwalten.",
  "p.frameworks.version_n": "Fassung {n}",
  "p.frameworks.behind":
    "Dieser Plan folgt Fassung {from}; die neueste ist Fassung {to}.",
  "p.frameworks.move": "Zu Fassung {n} wechseln",
  "p.frameworks.move_ask":
    "Diesen Plan auf Fassung {to} von „{name}“ umstellen? Er folgt jetzt Fassung {from}; danach gilt für ihn der neueste Text.",
  "p.frameworks.moved": "Dieser Plan folgt jetzt Fassung {n}.",
  "p.frameworks.behind_inherited":
    "Ein übergeordneter Plan bestimmt, welche Fassung gilt; stellen Sie ihn dort um.",
  "p.frameworks.show_versions": "Fassungen anzeigen",
  "p.frameworks.could_not_load_versions":
    "Die Fassungen konnten nicht geladen werden.",
  "p.frameworks.versions_heading": "Fassungen von „{name}“, neueste zuerst",
  "p.frameworks.plans_following": "folgende Pläne: {n}",
  "p.frameworks.newest": "neueste",
  "p.frameworks.this_plan": "dieser Plan",
  "p.frameworks.pin": "Fassung {n} folgen",
  "p.frameworks.pin_ask":
    "Diesen Plan Fassung {n} von „{name}“ folgen lassen? Er behält diese Fassung, bis er umgestellt wird.",
  "p.frameworks.pinned": "Dieser Plan folgt jetzt Fassung {n}.",
  "p.processes.not_yours":
    "Nur wer diesen Prozess festgelegt hat oder eine Administratorin bzw. ein Administrator darf ihn ändern oder außer Dienst stellen.",
  "p.processes.unchanged":
    "Nichts geändert, daher wurde keine Fassung angelegt.",
  "p.processes.changed":
    "„{name}“ als Fassung {v} gespeichert; laufende Durchläufe behalten ihre Fassung.",
  "p.processes.retire_ask":
    "„{name}“ außer Dienst stellen? Kein neuer Durchlauf kann starten; laufende Durchläufe gehen weiter.",
  "p.processes.retired": "„{name}“ außer Dienst gestellt.",
  "p.processes.edit": "„{name}“ ändern",
  "p.processes.retire": "„{name}“ außer Dienst stellen",
  "p.processes.edit_heading": "„{name}“ ändern (jetzt Fassung {v})",
  "p.processes.edit_note":
    "Speichern legt eine neue Fassung an; laufende Durchläufe behalten die Fassung, mit der sie begonnen haben.",
  "p.processes.save_version": "Als neue Fassung speichern",
  "p.processes.cancel_edit": "Abbrechen",
  "p.processes.versions": "Fassungen, neueste zuerst",
  "p.processes.current": "aktuell",
  "p.announcements.title_bad":
    "Ein Titel ist erforderlich, bis zu 120 Zeichen.",
  "p.announcements.body_bad":
    "Eine Nachricht ist erforderlich, bis zu 4.000 Zeichen.",
  "p.announcements.time_bad":
    "Eine Zeit konnte nicht gelesen werden; wählen Sie sie erneut.",
  "p.announcements.expires_bad":
    "Das Ablaufdatum muss nach dem Veröffentlichungszeitpunkt liegen.",
  "p.announcements.saved": "„{title}“ gespeichert.",
  "p.announcements.could_not_save":
    "Die Mitteilung konnte nicht gespeichert werden.",
  "p.announcements.change_h": "„{title}“ ändern",
  "p.announcements.change": "Ändern",
  "p.announcements.publish_at": "Veröffentlicht ab",
  "p.announcements.save": "Änderungen speichern",
  "p.announcements.cancel": "Abbrechen",
  "p.approvals.title": "Freigaben",
  "p.approvals.title_ppm": "Freigaben — PPM",
  "p.approvals.intro":
    "Eine Person wird gebeten, einen Plan, eine Aufgabe oder einen anderen Gegenstand freizugeben, außerhalb jedes Freigabeprozesses. Eine Entscheidung ist endgültig, und eine Ablehnung nennt den Grund.",
  "p.approvals.loading": "Freigaben werden geladen…",
  "p.approvals.could_not_load": "Die Freigaben konnten nicht geladen werden.",
  "p.approvals.waiting_h": "Wartet auf Sie",
  "p.approvals.waiting_none": "Nichts wartet auf Sie.",
  "p.approvals.asked_by": "Angefragt von {who} am {at}",
  "p.approvals.comment_field":
    "Kommentar (für eine Ablehnung erforderlich; bis zu 2.000 Zeichen)",
  "p.approvals.approve": "Freigeben",
  "p.approvals.reject": "Ablehnen",
  "p.approvals.approved_done": "Ihre Freigabe ist erfasst.",
  "p.approvals.rejected_done": "Ihre Ablehnung ist erfasst.",
  "p.approvals.asked_h": "Was Sie angefragt haben",
  "p.approvals.asked_none": "Sie haben noch niemanden gefragt.",
  "p.approvals.asked_of": "Angefragt bei {approver}",
  "p.approvals.their_comment": "Deren Kommentar: {comment}",
  "p.approvals.from_process":
    "Von einem Freigabeprozess ausgelöst: brechen Sie dessen Durchlauf ab, um sie zu beenden.",
  "p.approvals.withdraw": "Zurückziehen",
  "p.approvals.withdraw_ask":
    "Diese Anfrage zurückziehen? Die gefragte Person kann dann nicht mehr entscheiden.",
  "p.approvals.withdrawn_done": "Anfrage zurückgezogen.",
  "p.approvals.ask_h": "Um eine Freigabe bitten",
  "p.approvals.subject_kind_field": "Worum es geht",
  "p.approvals.subject_field": "Seine Kennung (eine UUID)",
  "p.approvals.approver_field":
    "Wer gefragt wird (person: oder worker: und eine UUID)",
  "p.approvals.message_field": "Eine Notiz an sie (optional)",
  "p.approvals.ask": "Um die Freigabe bitten",
  "p.approvals.asked_done": "{approver} um Freigabe gebeten.",
  "p.approvals.subject_bad": "Die Kennung des Gegenstands muss eine UUID sein.",
  "p.approvals.approver_bad":
    "Die gefragte Person muss person: oder worker: gefolgt von einer UUID sein.",
  "p.approvals.text_long":
    "Eine Notiz oder ein Kommentar hat höchstens 2.000 Zeichen.",
  "p.approvals.why_required":
    "Eine Ablehnung muss den Grund nennen: Schreiben Sie einen Kommentar.",
  "p.approvals.already_settled":
    "Diese Anfrage wurde bereits entschieden oder zurückgezogen, und eine Entscheidung ist endgültig. Die Listen wurden neu gelesen.",
  "p.approvals.not_found":
    "Dieser Gegenstand oder diese Anfrage ist nicht vorhanden, oder Sie dürfen ihn nicht lesen.",
  "p.approvals.not_approver":
    "Nur die gefragte Person darf über diese Anfrage entscheiden.",
  "p.approvals.not_requester":
    "Nur wer gefragt hat, darf diese Anfrage zurückziehen.",
  "p.approvals.could_not_request":
    "Die Freigabe konnte nicht angefragt werden.",
  "p.approvals.could_not_decide":
    "Die Entscheidung konnte nicht erfasst werden.",
  "p.approvals.could_not_withdraw":
    "Die Anfrage konnte nicht zurückgezogen werden.",
  "p.approvals.status.pending": "Wartet",
  "p.approvals.status.approved": "Freigegeben",
  "p.approvals.status.rejected": "Abgelehnt",
  "p.approvals.status.withdrawn": "Zurückgezogen",
  "p.approvals.kind.plan": "Vorhaben (Plan)",
  "p.approvals.kind.task": "Aufgabe",
  "p.approvals.kind.risk": "Risiko",
  "p.approvals.kind.proposal": "Vorschlag",
  "p.approvals.kind.idea": "Idee",
  "p.approvals.kind.canvas_item": "Canvas-Eintrag",
  "p.approvals.kind.budget_line": "Budgetposten",
  "p.approvals.kind.scenario": "Szenario",
  "p.approvals.kind.document_version": "Dokumentfassung",
  "p.custom_fields.task_values_none":
    "Für Aufgaben ist noch kein benutzerdefiniertes Feld festgelegt. Felder werden festgelegt auf der Seite",
  "p.custom_fields.task_values_could_not_load":
    "Die benutzerdefinierten Felder dieser Aufgabe konnten nicht geladen werden.",
  "p.product_roadmap.title": "Produkt-Roadmap",
  "p.product_roadmap.title_ppm": "Produkt-Roadmap — PPM",
  "p.product_roadmap.intro":
    "Was dieses Produkt jetzt, als Nächstes und später tut, wie seine öffentliche Roadmap es zeigt. Eine Roadmap gehört zu einem Plan der Art Product.",
  "p.product_roadmap.loading": "Roadmap wird geladen…",
  "p.product_roadmap.could_not_load":
    "Die Roadmap konnte nicht geladen werden.",
  "p.product_roadmap.not_product":
    "Nur ein Plan der Art Product hat eine öffentliche Roadmap. Ändern Sie die Art des Plans, um eine anzulegen.",
  "p.product_roadmap.state_h": "Ihr Zustand",
  "p.product_roadmap.published": "Veröffentlicht",
  "p.product_roadmap.not_published": "Nicht veröffentlicht",
  "p.product_roadmap.fresh.fresh": "Aktuell",
  "p.product_roadmap.fresh.stale": "Veraltet: Überprüfung fällig",
  "p.product_roadmap.fresh.unknown": "Nie überprüft",
  "p.product_roadmap.reviewed_on": "zuletzt überprüft am {date}",
  "p.product_roadmap.stale_rule":
    "Eine Roadmap gilt {days} Tage nach ihrer letzten Überprüfung als veraltet.",
  "p.product_roadmap.mark_reviewed": "Heute als überprüft markieren",
  "p.product_roadmap.reviewed_done": "Heute als überprüft markiert.",
  "p.product_roadmap.own_h": "Die Roadmap",
  "p.product_roadmap.make_h": "Roadmap anlegen",
  "p.product_roadmap.f_title": "Titel (bis zu 200 Zeichen)",
  "p.product_roadmap.f_slug":
    "Adresse (Kleinbuchstaben, Ziffern und einzelne Bindestriche, bis zu 64)",
  "p.product_roadmap.f_slug_new":
    "Adresse (optional; leer wird sie aus dem Titel gebildet)",
  "p.product_roadmap.f_vision": "Vision (optional, bis zu 2.000 Zeichen)",
  "p.product_roadmap.f_service":
    "Was das Produkt ist, für jemanden, der es nicht kennt (optional, bis zu 2.000 Zeichen)",
  "p.product_roadmap.save": "Roadmap speichern",
  "p.product_roadmap.make": "Roadmap jetzt anlegen",
  "p.product_roadmap.made": "Roadmap angelegt.",
  "p.product_roadmap.saved": "Roadmap gespeichert.",
  "p.product_roadmap.unchanged":
    "Nichts geändert, daher wurde nichts gesendet.",
  "p.product_roadmap.items_h": "Einträge",
  "p.product_roadmap.no_items": "Noch keine Einträge.",
  "p.product_roadmap.placement.recently_delivered": "Kürzlich geliefert",
  "p.product_roadmap.placement.now": "Jetzt",
  "p.product_roadmap.placement.next": "Als Nächstes",
  "p.product_roadmap.placement.later": "Später",
  "p.product_roadmap.placement.delivered_this_year": "Dieses Jahr geliefert",
  "p.product_roadmap.placement.not_doing": "Wird nicht gemacht",
  "p.product_roadmap.placement.none": "Nicht eingeordnet",
  "p.product_roadmap.phase_is": "In der Phase: {phase}",
  "p.product_roadmap.tags_are": "Schlagwörter: {tags}",
  "p.product_roadmap.outcome_is": "Erwartetes Ergebnis: {outcome}",
  "p.product_roadmap.change_item": "„{title}“ ändern",
  "p.product_roadmap.save_item": "Eintrag speichern",
  "p.product_roadmap.cancel": "Abbrechen",
  "p.product_roadmap.change": "Ändern",
  "p.product_roadmap.up": "Nach oben",
  "p.product_roadmap.down": "Nach unten",
  "p.product_roadmap.move_to": "Verschieben nach",
  "p.product_roadmap.move": "Verschieben",
  "p.product_roadmap.remove": "Entfernen",
  "p.product_roadmap.remove_ask":
    "„{title}“ ({id}) entfernen? Seine öffentliche Kennung wird nie einem anderen Eintrag gegeben.",
  "p.product_roadmap.add_h": "Einen Eintrag hinzufügen",
  "p.product_roadmap.f_placement": "Wohin er gehört",
  "p.product_roadmap.f_description":
    "Was die Arbeit ist (optional, bis zu 2.000 Zeichen)",
  "p.product_roadmap.f_outcome":
    "Was sich ändert und für wen (optional, bis zu 2.000 Zeichen)",
  "p.product_roadmap.f_phase":
    "Phase, in den Worten des Produkts (optional, bis zu 100 Zeichen)",
  "p.product_roadmap.f_tags":
    "Schlagwörter, durch Kommas getrennt (optional; bis zu 20, je bis zu 50 Zeichen)",
  "p.product_roadmap.add": "Eintrag hinzufügen",
  "p.product_roadmap.item_added": "„{title}“ hinzugefügt.",
  "p.product_roadmap.item_saved": "„{title}“ gespeichert.",
  "p.product_roadmap.item_moved": "„{title}“ nach {placement} verschoben.",
  "p.product_roadmap.item_removed": "„{title}“ entfernt.",
  "p.product_roadmap.title_bad":
    "Ein Titel ist erforderlich, bis zu 200 Zeichen.",
  "p.product_roadmap.slug_bad":
    "Die Adresse muss aus Kleinbuchstaben, Ziffern und einzelnen Bindestrichen bestehen, bis zu 64 Zeichen.",
  "p.product_roadmap.text_long": "Ein Text hat höchstens 2.000 Zeichen.",
  "p.product_roadmap.phase_long": "Die Phase hat höchstens 100 Zeichen.",
  "p.product_roadmap.tags_many": "Höchstens 20 Schlagwörter.",
  "p.product_roadmap.tag_long": "Ein Schlagwort hat höchstens 50 Zeichen.",
  "p.product_roadmap.tag_twice": "Jedes Schlagwort darf nur einmal vorkommen.",
  "p.product_roadmap.slug_taken":
    "Eine andere Roadmap hat oder hatte diese Adresse; wählen Sie eine andere.",
  "p.product_roadmap.gone":
    "Die Roadmap oder der Eintrag ist nicht mehr vorhanden; die Seite wurde neu gelesen.",
  "p.product_roadmap.could_not_save":
    "Die Roadmap konnte nicht gespeichert werden.",
  "p.product_roadmap.could_not_change_item":
    "Der Eintrag konnte nicht geändert werden.",
};
