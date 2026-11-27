// French (international) (fr-001) for the page and component catalogue (WW-1.1). Typed as a complete record of
// the English keys, so a missing string is a compile error.
//
// STATUS: MACHINE DRAFT. Written by Claude (the AI assistant) on 2026-10-31 and NOT reviewed by a
// speaker or a professional translator. The review is recorded in
// `governance/translation-reviews/fr-001.md`; update the status there, not here.

import type { PageKey } from "./i18n-pages";

export const PAGES_FR: Record<PageKey, string> = {
  "c.CalendarMonth.next_month": "Mois suivant",
  "p.calendar.day": "Jour",
  "p.calendar.agenda": "Agenda",
  "p.calendar.status": "Statut",
  "p.calendar.status_done": "fait",
  "p.calendar.status_open": "ouvert",
  "p.calendar.item": "Élément de travail",
  "p.calendar.showing": "Affichage {view}, du {start} au {end}",
  "p.calendar.more": "+{n} de plus",
  "p.calendar.milestone": "Étape clé",
  "p.calendar.due": "Échéance",
  "p.plans.pid.board.assignee": "Responsable",
  "p.plans.pid.board.moving": "Déplacement de {title} vers {column}…",
  "p.plans.pid.board.move_failed":
    "Impossible de déplacer {title} vers {column} ; la tâche est revenue à sa place.",
  "p.plans.pid.board.empty_column": "Aucune tâche",
  "p.plans.pid.board.search": "Rechercher dans le tableau",
  "p.plans.pid.board.task": "Tâche",
  "p.plans.pid.board.status": "Statut",
  "c.GanttView.holiday": "Jour férié",
  "c.GanttView.shaded":
    "Les colonnes grisées sont des jours chômés : week-ends et jours fériés du calendrier de travail de ce plan.",
  "c.GanttView.kind": "Nature",
  "c.GanttView.stage": "Étape",
  "c.GanttView.task": "Tâche",
  "c.GanttView.start": "Début",
  "c.GanttView.end": "Fin",
  "c.GanttView.predecessors": "Dépend de",
  "c.GanttView.depends_on": "Dépend de : {names}",
  "c.GanttView.critical": "{name} (chemin critique)",
  "c.GanttView.zoom_in": "Échelle de temps plus fine",
  "c.GanttView.zoom_out": "Échelle de temps plus large",
  "c.GanttView.zoomed": "Échelle de temps : {unit}",
  "c.GanttView.unit_day": "jour",
  "c.GanttView.unit_week": "semaine",
  "c.GanttView.unit_month": "mois",
  "c.GanttView.export": "Exporter le calendrier",
  "c.GanttView.csv": "CSV",
  "c.GanttView.excel": "Excel",
  "c.GanttView.exported": "{n} tâches exportées vers {file}",
  "c.CalendarMonth.week": "Semaine",
  "c.CalendarMonth.four_weeks": "4 semaines",
  "c.CalendarMonth.month": "Mois",
  "c.CalendarMonth.today": "Aujourd'hui",
  "c.CalendarMonth.previous": "{view} précédent(e)",
  "c.CalendarMonth.next": "{view} suivant(e)",
  "c.CalendarMonth.working_calendar": "Calendrier de travail",
  "c.CalendarMonth.previous_month": "Mois précédent",
  "c.CapacityMeter.days_over_capacity": "Jours au-delà de la capacité",
  "c.CostRiskView.adds": "{risk} : ajoute {money} — {eventHitText}",
  "c.CostRiskView.already_spent": "Déjà dépensé",
  "c.CostRiskView.assumptions": "Hypothèses",
  "c.CostRiskView.budget": "budget",
  "c.CostRiskView.budget_2": "Budget",
  "c.CostRiskView.budget_sum_of_planned_amounts":
    "Budget (somme des montants prévus)",
  "c.CostRiskView.by_plan": "Par plan",
  "c.CostRiskView.cost": "Coût",
  "c.CostRiskView.cost_at_each_percentile":
    "Coût en {currency} à chaque percentile",
  "c.CostRiskView.cost_from": "Coût issu de",
  "c.CostRiskView.each_bar_is_a_cost":
    "Chaque barre est une tranche de coût ; plus elle est haute, plus de simulations y ont abouti. Les lignes pleines sont P50, P80 et P90 (étiquetées) ; la ligne pointillée est le budget.",
  "c.CostRiskView.each_plan_s_budget_and":
    "Budget de chaque plan et coût simulé moyen en {currency}",
  "c.CostRiskView.estimate": "Estimation",
  "c.CostRiskView.independence": "Indépendance.",
  "c.CostRiskView.left_out": "Exclus :",
  "c.CostRiskView.line": "Ligne",
  "c.CostRiskView.low_planned_high": "Bas / prévu / haut",
  "c.CostRiskView.mean_simulated_cost": "Coût simulé moyen",
  "c.CostRiskView.no_register_risk_was_used":
    "Aucun risque du registre n'a servi d'événement de coût.",
  "c.CostRiskView.outcome": "Résultat",
  "c.CostRiskView.per_line_estimate_and_sensitivity":
    "Estimation et sensibilité par ligne en {currency}",
  "c.CostRiskView.plan": "Plan",
  "c.CostRiskView.risks_that_added_cost": "Risques ayant ajouté du coût",
  "c.CostRiskView.seed_the_same_seed_gives":
    "Graine {seed} : la même graine donne le même résultat. Jusqu'à {max_iterations} itérations.",
  "c.CostRiskView.sensitivity": "Sensibilité",
  "c.CostRiskView.simulations": "Simulations",
  "c.CostRiskView.spent": "Dépensé",
  "c.CostRiskView.the_same_figures_as_a":
    "Les mêmes chiffres sous forme de tableau",
  "c.CostRiskView.to": "à",
  "c.CostRiskView.to_of": "De {money} à {money2} : {count} sur {iterations}",
  "c.CostRiskView.what_drives_it": "Ce qui l'influence",
  "c.CpmGantt.cannot_meet_a_constraint_or":
    "ne peut pas respecter une contrainte ou une date limite (la ligne marque la limite)",
  "c.CpmGantt.critical_any_slip_moves_the":
    "critique : tout glissement décale la fin",
  "c.CpmGantt.has_float_the_hatched_tail":
    "dispose d'une marge (la traîne hachurée)",
  "c.CpmGantt.legend": "Légende",
  "c.CpmGantt.near_critical": "quasi critique",
  "c.CpmGantt.no_scheduled_tasks_to_draw":
    "Aucune tâche planifiée à tracer pour l'instant. Donnez une durée à une tâche (et une date de début ou un prédécesseur).",
  "c.CpmGantt.the_outline_under_a_bar":
    "le contour sous une barre indique sa position de référence",
  "c.CpmGantt.to_float_is_counted_in":
    "Du {from} au {to}. La marge est comptée en jours ouvrés du plan ; les barres sont tracées en jours calendaires.",
  "c.CpmTable.calendar": "Calendrier",
  "c.CpmTable.constraint": "Contrainte",
  "c.CpmTable.deadline": "Date limite",
  "c.CpmTable.finishes": "Fin",
  "c.CpmTable.free_float": "Marge libre",
  "c.CpmTable.overrides_its_predecessors": "(prime sur ses prédécesseurs)",
  "c.CpmTable.plan": "plan",
  "c.CpmTable.plan_assignee": "plan + responsable",
  "c.CpmTable.starts": "Début",
  "c.CpmTable.state": "État",
  "c.CpmTable.task": "Tâche",
  "c.CpmTable.task_schedule_with_float_in":
    "Calendrier des tâches avec marge, en jours ouvrés du plan",
  "c.CpmTable.total_float": "Marge totale",
  "c.CpmTable.untitled": "(sans titre)",
  "c.CumulativeFlow.cumulative_flow": "Flux cumulé",
  "c.CumulativeFlow.day": "Jour",
  "c.CumulativeFlow.every_value_in_the_chart":
    "Toutes les valeurs du graphique. {note}",
  "c.CumulativeFlow.no_samples_in_this_window":
    "Aucun échantillon dans cette fenêtre.",
  "c.CumulativeFlow.the_vertical_gap_above_done":
    "l'écart vertical au-dessus de « Terminé » est le travail en cours ; l'écart horizontal jusqu'à lui correspond à peu près au temps de cycle",
  "c.CumulativeFlow.total": "Total",
  "c.CumulativeFlow.total_in_progress_done":
    "{total} au total · {work_in_progress} en cours · {done} terminés",
  "c.EvBars.the_double_line_is_the":
    "La double ligne est le budget à l'achèvement : {money}.",
  "c.EvBars.unknown": "inconnu",
  "c.KpiCard.add_a_reading": "Ajouter une mesure",
  "c.KpiCard.cadence": "Fréquence",
  "c.KpiCard.daily": "Quotidienne",
  "c.KpiCard.delete": "Supprimer",
  "c.KpiCard.edit_or_delete": "Modifier ou supprimer",
  "c.KpiCard.history": "Historique ({readings})",
  "c.KpiCard.loading": "Chargement…",
  "c.KpiCard.monthly": "Mensuelle",
  "c.KpiCard.name": "Nom",
  "c.KpiCard.no_reading_yet": "Aucune mesure pour l'instant.",
  "c.KpiCard.no_readings": "Aucune mesure.",
  "c.KpiCard.none": "Aucune",
  "c.KpiCard.note": "Note",
  "c.KpiCard.observed": "Observé",
  "c.KpiCard.observed_on": "Observé le",
  "c.KpiCard.on": "{valueText} le {observed_on}",
  "c.KpiCard.quarterly": "Trimestrielle",
  "c.KpiCard.read": "· mesure {cadence}",
  "c.KpiCard.record": "Enregistrer",
  "c.KpiCard.red_line": "Ligne rouge",
  "c.KpiCard.save": "Enregistrer",
  "c.KpiCard.target": "Cible",
  "c.KpiCard.unit": "Unité",
  "c.KpiCard.value": "Valeur",
  "c.KpiCard.weekly": "Hebdomadaire",
  "c.KpiCard.which_way_is_good_cannot":
    "Le sens favorable ({v}) ne peut pas être modifié : cela recolorerait toutes les mesures passées.",
  "c.LilyGanttTable.critical_path": "chemin critique",
  "c.LilyGanttTable.day": "Jour",
  "c.LilyGanttTable.month": "Mois",
  "c.LilyGanttTable.no_dated_tasks_to_show": "Aucune tâche datée à afficher.",
  "c.LilyGanttTable.no_usable_dates": "Aucune date exploitable :",
  "c.LilyGanttTable.task": "Tâche",
  "c.LilyGanttTable.that_range_is_too_long":
    "Cette période est trop longue pour une colonne par {unit} ; affichage d'une colonne par {unit2}.",
  "c.LilyGanttTable.time_scale": "Échelle de temps",
  "c.LilyGanttTable.week": "Semaine",
  "c.LilyKanbanTable.move_to": "Déplacer {title} vers",
  "c.LilyKanbanTable.no_tasks_yet": "Aucune tâche pour l'instant.",
  "c.LilyKanbanTable.unknown_status": "Statut inconnu :",
  "c.LineRange.high": "Haut",
  "c.LineRange.low": "Bas",
  "c.LineRange.planned_is_the_likely_cost":
    "Le montant prévu est {money}, le coût probable. Laissez un champ vide pour l'effacer.",
  "c.LineRange.save_range": "Enregistrer la fourchette",
  "c.MilestoneCard.by": "Par",
  "c.MilestoneCard.by_hand_only": "Manuellement uniquement",
  "c.MilestoneCard.complete": "Terminer",
  "c.MilestoneCard.completes": "Se termine",
  "c.MilestoneCard.done": "Fait",
  "c.MilestoneCard.due": "Échéance {due} · {kind}",
  "c.MilestoneCard.every_move_is_kept_in":
    "Chaque report est conservé dans l'historique : l'ancienne date, la nouvelle, la raison et l'auteur.",
  "c.MilestoneCard.forecast": "· prévision {early_finish}",
  "c.MilestoneCard.from": "De",
  "c.MilestoneCard.loading": "Chargement…",
  "c.MilestoneCard.mark_done": "Marquer comme fait",
  "c.MilestoneCard.move": "Déplacer",
  "c.MilestoneCard.never_rescheduled": "Jamais reprogrammée.",
  "c.MilestoneCard.new_due_date": "Nouvelle échéance",
  "c.MilestoneCard.no_reason_given": "aucune raison indiquée",
  "c.MilestoneCard.open": "Ouverte",
  "c.MilestoneCard.pick_the_tasks_that_must":
    "Choisissez les tâches qui doivent être terminées",
  "c.MilestoneCard.reason_required": "Raison (obligatoire)",
  "c.MilestoneCard.reopen": "Rouvrir",
  "c.MilestoneCard.reschedule": "Reprogrammer",
  "c.MilestoneCard.save": "Enregistrer",
  "c.MilestoneCard.slip_history": "Historique des glissements ({reschedules})",
  "c.MilestoneCard.tasks_it_waits_for": "Tâches attendues",
  "c.MilestoneCard.tasks_this_milestone_waits_for":
    "Tâches attendues par cette étape clé",
  "c.MilestoneCard.this_plan_has_no_tasks": "Ce plan n'a encore aucune tâche.",
  "c.MilestoneCard.to": "À",
  "c.MilestoneCard.view_plan": "Voir le plan",
  "c.MilestoneCard.when_all_those_tasks_are":
    "Quand toutes ces tâches sont terminées",
  "c.MilestoneCard.why": "Pourquoi",
  "c.NetworkDiagram.float": "{slice} → {slice2} · marge {total_float}",
  "c.NetworkDiagram.no_dependencies": "Aucune dépendance.",
  "c.NetworkDiagram.no_scheduled_tasks_to_draw":
    "Aucune tâche planifiée à tracer pour l'instant.",
  "c.NetworkDiagram.task_network_boxes_are_tasks":
    "Réseau des tâches : les cases sont des tâches, les flèches des dépendances",
  "c.NetworkDiagram.the_same_network_as_a":
    "Le même réseau sous forme de liste",
  "c.ObjectiveCard.add_a_key_result": "Ajouter un résultat clé",
  "c.ObjectiveCard.add_key_result": "Ajouter le résultat clé",
  "c.ObjectiveCard.check_in": "Point d'étape",
  "c.ObjectiveCard.confidence_0_100": "Confiance (0–100)",
  "c.ObjectiveCard.currency": "Devise",
  "c.ObjectiveCard.currency_e_g_gbp": "Devise (par exemple EUR, JPY ou GBP)",
  "c.ObjectiveCard.decrease": "Diminuer",
  "c.CommandPalette.cmd_dashboard": "Tableau de bord",
  "c.CommandPalette.cmd_escalations": "Escalades",
  "c.CommandPalette.cmd_my_work": "Mon travail",
  "c.CommandPalette.cmd_new_plan": "Nouveau plan",
  "c.CommandPalette.count_one": "{n} résultat",
  "c.CommandPalette.count_other": "{n} résultats",
  "c.CommandPalette.failed":
    "La recherche de plans est indisponible pour le moment. Les commandes fonctionnent toujours.",
  "c.CommandPalette.kind_command": "Commande",
  "c.CommandPalette.kind_plan": "Plan",
  "c.CommandPalette.kind_recent": "Récent",
  "c.CommandPalette.label": "Rechercher des plans et des commandes",
  "c.CommandPalette.none": "Aucun résultat",
  "c.CommandPalette.open": "Recherche et commandes",
  "c.CommandPalette.placeholder": "Saisissez un nom de plan ou une commande",
  "c.CommandPalette.title": "Recherche et commandes",
  "c.NavItems.portfolios": "Portefeuilles",
  "c.NavItems.projects": "Projets",
  "c.NavItems.products": "Produits",
  "c.NavItems.programs": "Programmes",
  "c.NavItems.practices": "Pratiques",
  "c.NavItems.processes": "Processus",
  "c.NavItems.purposes": "Finalités",
  "c.NavItems.pathways": "Parcours",
  "c.PasteTasks.check": "Vérifier auprès du service",
  "c.PasteTasks.clear": "Effacer",
  "c.PasteTasks.col_due": "Date limite",
  "c.PasteTasks.col_line": "Ligne",
  "c.PasteTasks.col_notes": "Notes",
  "c.PasteTasks.col_status": "Statut",
  "c.PasteTasks.col_title": "Titre",
  "c.PasteTasks.create_one": "Créer 1 tâche",
  "c.PasteTasks.create_other": "Créer {n} tâches",
  "c.PasteTasks.done": "Terminé",
  "c.PasteTasks.failed": "Le collage n'a pas fonctionné.",
  "c.PasteTasks.help":
    "Copiez des cellules depuis un tableur et collez-les ici : une tâche par ligne, avec les colonnes Titre, Date limite (aaaa-mm-jj) et Notes, dans cet ordre. La date devient la date limite de la tâche. Une première ligne qui nomme les colonnes est ignorée. Une ligne posant problème est affichée avec la raison et exclue.",
  "c.PasteTasks.label": "Lignes collées",
  "c.PasteTasks.left_out": "Exclue : {why}",
  "c.PasteTasks.problem_beyond_limit": "plus de {max} lignes",
  "c.PasteTasks.problem_date_bad":
    "« {value} » n'est pas une date ; écrivez-la au format aaaa-mm-jj",
  "c.PasteTasks.problem_no_title": "aucun titre",
  "c.PasteTasks.problem_notes_long": "les notes dépassent {max} caractères",
  "c.PasteTasks.problem_same_as": "identique à la ligne {line}",
  "c.PasteTasks.problem_title_long": "le titre dépasse {max} caractères",
  "c.PasteTasks.ready": "Prête",
  "c.PasteTasks.tally": "{good} prêtes, {left} exclues",
  "c.PasteTasks.title": "Coller des lignes depuis un tableur",
  "c.PasteTasks.would": "Ce qui se passerait",
  "c.ConfirmHost.cancel": "Annuler",
  "c.ConfirmHost.ok": "OK",
  "c.ConfirmHost.title": "Veuillez confirmer",
  "c.ObjectiveCard.delete": "Supprimer",
  "c.ObjectiveCard.direction": "Sens",
  "c.ObjectiveCard.edit_or_delete": "Modifier ou supprimer",
  "c.ObjectiveCard.edit_or_delete_this_objective":
    "Modifier ou supprimer cet objectif",
  "c.ObjectiveCard.from": "{valueText} (depuis {start_value})",
  "c.ObjectiveCard.increase": "Augmenter",
  "c.ObjectiveCard.it_has_no_measurable_key":
    "— il n'a pas encore de résultat clé mesurable, donc pas de score (et non zéro).",
  "c.ObjectiveCard.key_results_in_more_than":
    "Les résultats clés en plusieurs devises sont notés séparément ; les devises ne sont jamais converties.",
  "c.ObjectiveCard.last_check_in": "· dernier point d'étape {slice}",
  "c.ObjectiveCard.maintain": "Maintenir",
  "c.ObjectiveCard.metric": "Indicateur",
  "c.ObjectiveCard.no_key_results_yet": "Aucun résultat clé pour l'instant.",
  "c.ObjectiveCard.note": "Note",
  "c.ObjectiveCard.number": "Nombre",
  "c.ObjectiveCard.percent": "Pourcentage",
  "c.ObjectiveCard.period": "Période",
  "c.ObjectiveCard.record": "Enregistrer",
  "c.ObjectiveCard.save": "Enregistrer",
  "c.ObjectiveCard.start": "Départ",
  "c.ObjectiveCard.target": "Cible",
  "c.ObjectiveCard.the_baseline_metric_and_direction":
    "La valeur de référence, l'indicateur et le sens ne peuvent pas être modifiés : un progrès mesuré depuis une référence mouvante n'est pas un progrès.",
  "c.ObjectiveCard.title": "Titre",
  "c.ObjectiveCard.tolerance": "· tolérance ±{tolerance}",
  "c.ObjectiveCard.tolerance_2": "Tolérance",
  "c.ObjectiveCard.unit": "Unité",
  "c.ObjectiveCard.value": "Valeur",
  "c.ObjectiveCard.yes_no": "Oui / non",
  "c.PlanForm.organization": "organization:…",
  "c.PlanForm.parent_plan_pid_uuid": "pid du plan parent (UUID)",
  "c.PlanForm.proj_2026": "PROJ-2026",
  "c.RaciMatrix.complete": "complète",
  "c.RaciMatrix.gaps": "Lacunes",
  "c.RaciMatrix.task": "Tâche",
  "c.RaciMatrix.who_does_what_raci_matrix":
    "Qui fait quoi (matrice RACI) : qui est Réalisateur, Approbateur, Consulté et Informé pour chaque tâche",
  "c.RiskInputs.adds_cost": "Ajoute un coût",
  "c.RiskInputs.by_business_days": "de (jours ouvrés)",
  "c.RiskInputs.currency": "Devise",
  "c.RiskInputs.delays_task": "Retarde la tâche",
  "c.RiskInputs.gbp": "EUR",
  "c.RiskInputs.none": "Aucune",
  "c.RiskInputs.one_probability_covers_both_effects":
    "Une seule probabilité couvre les deux effets : un risque qui retarde une tâche et ajoute un coût survient une fois et produit les deux. Laissez une partie vide pour l'exclure.",
  "c.RiskInputs.probability": "Probabilité (%)",
  "c.RiskInputs.save": "Enregistrer",
  "c.RiskInputs.saving": "Enregistrement…",
  "c.RiskInputs.simulation_inputs": "Données de simulation",
  "c.SCurve.actual": "Réel",
  "c.SCurve.date": "Date",
  "c.SCurve.earned": "Acquis",
  "c.SCurve.line_planned_value_to_earned":
    "Ligne : valeur planifiée (du {from} au {to}). ● valeur acquise et ■ coût réel, uniquement les jours enregistrés ({snapshots} à ce jour). Pointillés : budget à l'achèvement, {money}.",
  "c.SCurve.no_curve_to_draw": "Aucune courbe à tracer.",
  "c.SCurve.not_recorded": "non enregistré",
  "c.SCurve.planned": "Planifié",
  "c.SCurve.s_curve_planned_value_over":
    "Courbe en S : valeur planifiée dans le temps, avec la valeur acquise et le coût réel enregistrés",
  "c.SCurve.the_same_figures_as_a": "Les mêmes chiffres sous forme de tableau",
  "c.VarianceTable.baselined": "Référence",
  "c.VarianceTable.duration": "Durée Δ",
  "c.VarianceTable.finish": "Fin Δ",
  "c.VarianceTable.now": "Actuel",
  "c.VarianceTable.schedule_variance_against_the_baseli":
    "Écart de calendrier par rapport à la référence, en jours ouvrés",
  "c.VarianceTable.standing": "Situation",
  "c.VarianceTable.start": "Début Δ",
  "c.VarianceTable.task": "Tâche",
  "c.VarianceTable.the_plan_s_finish_is": "La fin du plan est inchangée.",
  "c.VarianceTable.the_plan_s_own_finish":
    "La fin propre du plan ne peut pas être comparée.",
  "p.announcements.announcements": "Annonces",
  "p.announcements.announcements_ppm": "Annonces — PPM",
  "p.announcements.expires_optional": "Expire le (facultatif)",
  "p.announcements.important": "Important",
  "p.announcements.info": "Information",
  "p.announcements.loading": "Chargement…",
  "p.announcements.message": "Message",
  "p.announcements.nothing_has_been_announced":
    "Aucune annonce pour l'instant.",
  "p.announcements.pin_to_the_top": "Épingler en haut",
  "p.announcements.pinned": "· Épinglée",
  "p.announcements.publish": "Publier",
  "p.announcements.publish_an_announcement": "Publier une annonce",
  "p.announcements.publishing_needs_the_announcements_p":
    "La publication nécessite l'autorisation « annonces ».",
  "p.announcements.retract": "Retirer",
  "p.announcements.severity": "Gravité",
  "p.announcements.title": "Titre",
  "p.announcements.urgent": "Urgent",
  "p.auditor.action": "Action",
  "p.auditor.actor": "Auteur",
  "p.auditor.actor_less_recorded_actions": "Actions enregistrées sans auteur :",
  "p.auditor.audit_trail": "Piste d'audit",
  "p.auditor.bearer_sub": "sub du porteur",
  "p.auditor.detail": "Détail",
  "p.auditor.e_g_merged": "p. ex. fusionné",
  "p.auditor.entity": "Entité",
  "p.auditor.evidence_pack_csv": "Dossier de preuves (CSV)",
  "p.auditor.filter": "Filtrer",
  "p.auditor.findings": "Constats",
  "p.auditor.no_audit_rows_match": "Aucune ligne d'audit ne correspond.",
  "p.auditor.no_findings": "Aucun constat.",
  "p.auditor.ppm": "{t} — PPM",
  "p.auditor.rows_distinct_actors_actor_less":
    "{returned} lignes · {distinct_actors} auteurs distincts · {actorless} sans auteur",
  "p.auditor.rule": "Règle",
  "p.auditor.submitted_without_a_verified_token": "(soumis sans jeton vérifié)",
  "p.auditor.when": "Quand",
  "p.automations.action": "action",
  "p.automations.action_2": "Action",
  "p.automations.add_label": "ajouter une étiquette",
  "p.automations.any": "tous",
  "p.automations.assign": "affecter",
  "p.automations.assignee": "responsable",
  "p.automations.automation": "automatisation",
  "p.automations.automations_ppm": "Automatisations — PPM",
  "p.automations.blank_every_plan": "vide = tous les plans",
  "p.automations.cancel": "Annuler",
  "p.automations.claim_based_so_a_deadline":
    "Fondé sur la réservation : une échéance ne se déclenche qu'une seule fois, même si le minuteur et un balayage manuel entrent en concurrence.",
  "p.automations.configure_a_rule_once_as":
    "Configurez une règle une fois ; la plateforme l'applique à mesure que les éléments se déplacent sur le tableau. Les règles ne bloquent jamais le déplacement qui les a déclenchées — un échec est consigné comme exécution échouée, pas comme une annulation.",
  "p.automations.delete": "Supprimer",
  "p.automations.detail": "Détail",
  "p.automations.disable": "Désactiver",
  "p.automations.due": "Échéance",
  "p.automations.enable": "Activer",
  "p.automations.every_plan": "tous les plans",
  "p.automations.expire_review": "faire expirer la revue",
  "p.automations.from": "de",
  "p.automations.in_calendar_days": "en jours calendaires",
  "p.automations.label": "étiquette",
  "p.automations.message": "message",
  "p.automations.name": "nom",
  "p.automations.name_2": "Nom",
  "p.automations.no_rules_configured": "Aucune règle configurée.",
  "p.automations.nothing_has_fired_yet": "Rien ne s'est encore déclenché.",
  "p.automations.nothing_pending": "Rien en attente.",
  "p.automations.notify": "notifier",
  "p.automations.off": "désactivée",
  "p.automations.on": "activée",
  "p.automations.operator": "opérateur",
  "p.automations.outcome": "Résultat",
  "p.automations.person_uuid": "person:<uuid>",
  "p.automations.plan_pid": "pid du plan",
  "p.automations.plan_stage_changed": "étape du plan modifiée",
  "p.automations.recipient": "destinataire",
  "p.automations.review_submitted": "revue soumise",
  "p.automations.rules": "Règles",
  "p.automations.runs": "Exécutions",
  "p.automations.save_rule": "Enregistrer la règle",
  "p.automations.schedule_action": "planifier une action",
  "p.automations.scope": "Portée",
  "p.automations.set_and_forget": "Configurer et oublier",
  "p.automations.set_task_status": "définir le statut de la tâche",
  "p.automations.source": "Source",
  "p.automations.state": "État",
  "p.automations.status": "statut",
  "p.automations.subject": "Objet",
  "p.automations.sweep_due_actions": "Exécuter les actions échues",
  "p.automations.task_moved": "tâche déplacée",
  "p.automations.then": "alors",
  "p.automations.to": "à",
  "p.automations.trigger": "déclencheur",
  "p.automations.trigger_2": "Déclencheur",
  "p.automations.when": "Quand",
  "p.automations.workflow_automation": "Automatisation des flux de travail",
  "p.board.amber": "ambre",
  "p.board.amount": "Montant",
  "p.board.benefit_realizations": "réalisations de bénéfices",
  "p.board.board_pack": "Dossier du conseil",
  "p.board.decision": "Décision",
  "p.board.decisions_in_window": "Décisions sur la période",
  "p.board.green": "vert",
  "p.board.investment_decisions": "Décisions d'investissement",
  "p.board.kind": "Nature",
  "p.board.milestones_completed": "étapes clés terminées",
  "p.board.no_decisions_in_the_window": "Aucune décision sur la période.",
  "p.board.no_investment_decisions_recorded":
    "Aucune décision d'investissement enregistrée.",
  "p.board.no_snapshots_captured_yet": "Aucun instantané pris pour l'instant.",
  "p.board.open_exposure": "Exposition ouverte",
  "p.board.portfolios": "Portefeuilles",
  "p.board.ppm": "{t} — PPM",
  "p.board.realized_in_window": "Réalisé sur la période :",
  "p.board.red_portfolios": "portefeuilles au rouge",
  "p.board.subject": "Objet",
  "p.board.take_snapshot_now": "Prendre un instantané maintenant",
  "p.board.taken": "Pris le",
  "p.board.tranches_released": "tranches débloquées",
  "p.board.trends": "Tendances",
  "p.board.unattributed_events":
    "({unattributed_events} événements non attribués)",
  "p.board.what": "Quoi",
  "p.board.when": "Quand",
  "p.board.window": "période {from} → {to}",
  "p.board.work_items": "Éléments de travail",
  "p.calendar.all": "(tous)",
  "p.calendar.done": "· fait",
  "p.calendar.kind": "Nature",
  "p.calendar.no_milestones_match": "Aucune étape clé ne correspond.",
  "p.calendar.ppm": "{t} — PPM",
  "p.capacity.capacity_ppm": "Capacité — PPM",
  "p.capacity.from": "du",
  "p.capacity.hatched_extension_proposed_not_count":
    "prolongement hachuré : proposé, non comptabilisé",
  "p.capacity.legend": "Légende",
  "p.capacity.over": "dépassement",
  "p.capacity.over_capacity_solid_bar_firm":
    "au-delà de la capacité (barre pleine : réservations fermes ; ligne : capacité en vigueur)",
  "p.capacity.proposed": "Proposé",
  "p.capacity.to": "au",
  "p.capacity.window": "période : {from} → {to}",
  "p.capacity.within_capacity": "dans la capacité",
  "p.capacity.within_capacity_but_committing_the":
    "dans la capacité, mais engager les réservations proposées la dépasserait",
  "p.ceo.all_announcements": "Toutes les annonces",
  "p.ceo.all_key_measures": "Tous les indicateurs clés",
  "p.ceo.all_milestones": "Toutes les étapes clés",
  "p.ceo.and_more": "et {more} de plus",
  "p.ceo.announcements": "Annonces",
  "p.ceo.ceo_dashboard": "Tableau de bord du directeur général",
  "p.ceo.ceo_dashboard_ppm": "Tableau de bord du directeur général — PPM",
  "p.ceo.decision_log": "Journal des décisions",
  "p.ceo.escalated_risks": "risques escaladés",
  "p.ceo.every_kpi_is_on_target": "● Tous les KPI sont dans la cible.",
  "p.ceo.executive_detail": "Détail pour la direction",
  "p.ceo.key_measures_needing_a_look": "Indicateurs clés à examiner",
  "p.ceo.loading": "Chargement…",
  "p.ceo.milestones_to_watch": "Étapes clés à surveiller",
  "p.ceo.no_data_yet": "Aucune donnée pour l'instant",
  "p.ceo.no_decisions_recorded_yet":
    "Aucune décision enregistrée pour l'instant.",
  "p.ceo.no_kpis_defined_yet": "Aucun KPI défini pour l'instant.",
  "p.ceo.no_open_milestones": "Aucune étape clé ouverte.",
  "p.ceo.no_portfolio_is_over_budget":
    "● Aucun portefeuille ne dépasse son budget",
  "p.ceo.nothing_has_been_announced": "Aucune annonce pour l'instant.",
  "p.ceo.over_budget": "▲ Budget dépassé : {join}",
  "p.ceo.overdue_milestones": "{overdueMilestones} étapes clés en retard",
  "p.ceo.portfolio_health": "Santé du portefeuille",
  "p.ceo.recent_decisions": "Décisions récentes",
  "p.ceo.red_amber_green": "■ {red} rouge · ▲ {amber} ambre · ● {green} vert",
  "p.ceo.risk_and_money": "Risques et finances",
  "p.ceo.risk_register": "Registre des risques",
  "p.compliance.compliance_risk_register": "Registre des risques de conformité",
  "p.compliance.conformance_findings": "Constats de conformité",
  "p.compliance.detail": "Détail",
  "p.compliance.exposure": "Exposition",
  "p.compliance.gate_review_recency_threshold_calend":
    "seuil d'ancienneté des revues de jalon : {review_days} jours calendaires",
  "p.compliance.item": "Élément",
  "p.compliance.no_compliance_risks_recorded":
    "Aucun risque de conformité enregistré.",
  "p.compliance.no_conformance_findings": "Aucun constat de conformité.",
  "p.compliance.open_exposure": "{note} · exposition ouverte {open_exposure}",
  "p.compliance.owner": "Propriétaire",
  "p.compliance.ppm": "{t} — PPM",
  "p.compliance.risk": "Risque",
  "p.compliance.rule": "Règle",
  "p.compliance.status": "Statut",
  "p.cost_rates.a_person": "Une personne",
  "p.cost_rates.a_role": "Un rôle",
  "p.cost_rates.add_a_rate_table": "Ajouter une grille tarifaire",
  "p.cost_rates.add_rate": "Ajouter un taux",
  "p.cost_rates.add_table": "Ajouter la grille",
  "p.cost_rates.cost_rates": "Taux de coût",
  "p.cost_rates.cost_rates_are_finance_data":
    "Les taux de coût sont des données financières. Vous n'y avez pas accès.",
  "p.cost_rates.cost_rates_ppm": "Taux de coût — PPM",
  "p.cost_rates.currency": "Devise",
  "p.cost_rates.effective_from": "En vigueur à partir du",
  "p.cost_rates.for": "Pour",
  "p.cost_rates.from": "À partir du",
  "p.cost_rates.name": "Nom",
  "p.cost_rates.no_rate_tables_yet": "Aucune grille tarifaire pour l'instant.",
  "p.cost_rates.no_rate_yet_work_by":
    "Aucun taux pour l'instant : le travail de ce {v} n'est pas chiffré.",
  "p.cost_rates.per_hour": "Par heure",
  "p.cost_rates.person_person_reference": "Personne (référence person:…)",
  "p.cost_rates.role": "Rôle",
  "p.cost_risk.band_for_lines_with_no":
    "Marge pour les lignes sans estimation (± %)",
  "p.cost_risk.distribution": "Distribution",
  "p.cost_risk.iterations": "Itérations",
  "p.cost_risk.pert": "PERT",
  "p.cost_risk.portfolio_cost_risk": "Risque de coût du portefeuille",
  "p.cost_risk.portfolio_cost_risk_ppm": "Risque de coût du portefeuille — PPM",
  "p.cost_risk.resources": "Ressources",
  "p.cost_risk.run": "Exécuter",
  "p.cost_risk.running": "Exécution…",
  "p.cost_risk.seed": "Graine",
  "p.cost_risk.shows_people_instead": "affiche plutôt les personnes.",
  "p.cost_risk.triangular": "Triangulaire",
  "p.cost_risk.what_all_your_plans_together":
    "Ce que tous vos plans réunis pourraient coûter, compte tenu de la fourchette de chaque ligne budgétaire et des risques qui ajoutent un coût. Chaque devise est traitée séparément ; elles ne sont jamais additionnées.",
  "p.dashboard.dashboard_ppm": "Tableau de bord — PPM",
  "p.display.display": "Affichage",
  "p.display.display_ppm": "Affichage — PPM",
  "p.display.dyslexia_friendly_font": "Police adaptée à la dyslexie",
  "p.display.go_back_to_the_default":
    "Revenir à la valeur par défaut de mon rôle",
  "p.display.how_the_app_looks_and":
    "L'apparence et la lisibilité de l'application. Ces réglages ne changent jamais ce que vous pouvez faire.",
  "p.display.keeps_the_everyday_pages_in":
    "Garde les pages courantes au premier plan (Mon travail, Plans, Tableau, Calendrier, Visite guidée) et place le reste derrière un lien « Plus ». Toutes les pages restent disponibles et fonctionnelles.",
  "p.display.sign_in_to_keep_these":
    "Connectez-vous pour conserver ces réglages dans votre compte ainsi que sur cet appareil.",
  "p.display.simple_view": "Vue simplifiée",
  "p.display.this_is_the_default_for":
    "C'est la valeur par défaut de votre rôle : {v}.",
  "p.display.use_simple_view": "Utiliser la vue simplifiée",
  "p.display.sections_title": "Sections de la barre supérieure",
  "p.display.sections_help":
    "Décochez une section pour la masquer de la barre supérieure. Elle reste accessible à son adresse, et vous pouvez la réafficher ici. Rien n'est enregistré tant que vous n'en masquez aucune, afin que les sections ajoutées plus tard vous parviennent.",
  "p.display.sections_legend": "Afficher ces sections",
  "p.display.sections_hidden_count_one": "{n} section masquée",
  "p.display.sections_hidden_count_other": "{n} sections masquées",
  "p.display.sections_none_hidden": "Aucune section n'est masquée.",
  "p.display.sections_show_all": "Afficher toutes les sections",
  "p.display.week_start_title": "Premier jour de la semaine",
  "p.display.week_start_help":
    "Le jour où les calendriers commencent chaque semaine. Par défaut, il suit la langue choisie ; les numéros de semaine restent des semaines ISO, qui commencent le lundi.",
  "p.display.week_start_label": "Les semaines commencent le",
  "p.display.week_start_locale": "Le jour habituel de la langue",
  "p.display.week_start_mon": "Lundi",
  "p.display.week_start_sun": "Dimanche",
  "p.display.week_start_sat": "Samedi",
  "p.display.use_the_dyslexia_friendly_font":
    "Utiliser la police adaptée à la dyslexie",
  "p.display.uses_the_opendyslexic_typeface_with":
    "Utilise la police OpenDyslexic, avec un espacement plus large entre les lettres, les mots et les lignes.",
  "p.display.you_chose_this": "Vous avez choisi ceci.",
  "p.documents.controlled_documents": "Documents maîtrisés",
  "p.documents.create_document": "Créer le document",
  "p.documents.document": "Document",
  "p.documents.documents": "Documents",
  "p.documents.documents_ppm": "Documents — PPM",
  "p.documents.first_draft": "Premier brouillon",
  "p.documents.in_force": "En vigueur",
  "p.documents.loading": "Chargement…",
  "p.documents.new_document": "Nouveau document",
  "p.documents.no_documents_yet": "Aucun document pour l'instant.",
  "p.documents.pid.approver_s_id": "Id de l'approbateur",
  "p.documents.pid.date": "Date",
  "p.documents.pid.document": "Document",
  "p.documents.pid.documents": "← Documents",
  "p.documents.pid.look_up": "Rechercher",
  "p.documents.pid.nothing_was_in_force_on": "Rien n'était en vigueur le {on}.",
  "p.documents.pid.person_id": "id de la personne",
  "p.documents.pid.save_draft": "Enregistrer le brouillon",
  "p.documents.pid.send_back": "Renvoyer",
  "p.documents.pid.text": "Texte",
  "p.documents.pid.this_version_s_text_is":
    "Le texte de cette version est définitif. Toute modification va dans le brouillon suivant.",
  "p.documents.pid.version": "Version {version}",
  "p.documents.pid.version_was_in_force_on":
    "La version {version} était en vigueur le {on}.",
  "p.documents.pid.versions": "Versions",
  "p.documents.pid.what_changed": "Modifications",
  "p.documents.pid.what_was_in_force_on": "Ce qui était en vigueur à une date",
  "p.documents.policies_and_procedures_a_new":
    "Politiques et procédures. Une nouvelle version est examinée et approuvée avant d'entrer en vigueur, et une version en vigueur ne change jamais.",
  "p.documents.title": "Titre",
  "p.documents.versions": "Versions",
  "p.engineering.assignee": "Responsable",
  "p.engineering.blocked_calendar_days": "Bloqué (jours calendaires)",
  "p.engineering.blocked_work": "Travail bloqué",
  "p.engineering.declared_cause_failure_rate": "taux d'échec à cause déclarée",
  "p.engineering.delivery_links": "Liens de livraison",
  "p.engineering.deploys_mo": "déploiements ({window_months} mois)",
  "p.engineering.devops_metrics": "Indicateurs DevOps",
  "p.engineering.environment": "Environnement",
  "p.engineering.incidents": "incidents",
  "p.engineering.incidents_unresolved_counted_never_t":
    "{unresolved_incidents} incidents non résolus (comptés, jamais chronométrés).",
  "p.engineering.item": "Élément",
  "p.engineering.items_carry_no_moscow_tag":
    "{untagged} éléments n'ont pas d'étiquette MoSCoW.",
  "p.engineering.median_recovery_h": "rétablissement médian (h)",
  "p.engineering.moscow_scope": "Périmètre MoSCoW",
  "p.engineering.no_deploy_events_ingested_yet":
    "Aucun événement de déploiement ingéré pour l'instant.",
  "p.engineering.no_externally_tracked_items":
    "Aucun élément suivi en externe.",
  "p.engineering.nothing_blocked_ship_it": "Rien n'est bloqué. Livrez.",
  "p.engineering.ppm": "{t} — PPM",
  "p.engineering.releases": "Versions publiées",
  "p.engineering.task": "Tâche",
  "p.engineering.trackers": "Outils de suivi",
  "p.engineering.untracked": "Non suivis :",
  "p.engineering.version": "Version",
  "p.engineering.when": "Quand",
  "p.executive.actor": "Auteur",
  "p.executive.aligned": "Aligné",
  "p.executive.as_of": "au {as_of} · {derivation}",
  "p.executive.benefits": "Bénéfices",
  "p.executive.benefits_realization": "Réalisation des bénéfices",
  "p.executive.calendar_days_since_update":
    "Jours calendaires depuis la mise à jour",
  "p.executive.collection": "Collection",
  "p.executive.currency": "Devise",
  "p.executive.decision": "Décision",
  "p.executive.decision_log": "Journal des décisions",
  "p.executive.escalated_risks": "Risques escaladés",
  "p.executive.kind": "Nature",
  "p.executive.largest_unaligned_items": "Plus grands éléments non alignés",
  "p.executive.members": "Membres",
  "p.executive.no_decisions_recorded_yet":
    "Aucune décision enregistrée pour l'instant.",
  "p.executive.no_target": "aucune cible",
  "p.executive.non_financial_only":
    "non financiers uniquement ({non_financial})",
  "p.executive.open_exposure": "Exposition ouverte",
  "p.executive.overdue_milestones": "Étapes clés en retard",
  "p.executive.overrun_currencies": "Devises en dépassement",
  "p.executive.planned_with_no_objective": "prévu sans objectif",
  "p.executive.portfolio": "Portefeuille",
  "p.executive.portfolio_health": "Santé du portefeuille",
  "p.executive.ppm": "{t} — PPM",
  "p.executive.realization": "Réalisation",
  "p.executive.realized": "Réalisé",
  "p.executive.status": "Statut",
  "p.executive.strategic_alignment": "Alignement stratégique",
  "p.executive.subject": "Objet",
  "p.executive.target": "Cible",
  "p.executive.total": "Total",
  "p.executive.unaligned": "Non aligné",
  "p.executive.unaligned_spend": "Dépenses non alignées",
  "p.executive.when": "Quand",
  "p.financials.actual": "Réel",
  "p.financials.as_of": "au {as_of} · {note}",
  "p.financials.currency": "Devise",
  "p.financials.currency_exposure": "Exposition par devise",
  "p.financials.lines": "Lignes",
  "p.financials.no_budget_lines_recorded_yet":
    "Aucune ligne budgétaire enregistrée pour l'instant.",
  "p.financials.planned": "Prévu",
  "p.financials.ppm": "{t} — PPM",
  "p.financials.remaining": "Restant",
  "p.financials.work_items": "Éléments de travail",
  "p.gantt.main_x": "{t} — Main X",
  "p.ideas.convert_target": "Cible de conversion",
  "p.ideas.ideas_ppm": "Idées — PPM",
  "p.kpis.all": "Tous",
  "p.kpis.blank_for_the_whole_portfolio": "vide pour tout le portefeuille",
  "p.kpis.daily": "Quotidienne",
  "p.kpis.define": "Définir",
  "p.kpis.define_a_key_measure_kpi": "Définir un indicateur clé (KPI)",
  "p.kpis.higher_is_better": "Plus c'est élevé, mieux c'est",
  "p.kpis.key_measures_kpis": "Indicateurs clés (KPI)",
  "p.kpis.kpis_by_status": "KPI par statut",
  "p.kpis.kpis_ppm": "KPI — PPM",
  "p.kpis.loading": "Chargement…",
  "p.kpis.lower_is_better": "Plus c'est bas, mieux c'est",
  "p.kpis.monthly": "Mensuelle",
  "p.kpis.name": "Nom",
  "p.kpis.no_data_yet": "Aucune donnée pour l'instant",
  "p.kpis.not_set": "Non défini",
  "p.kpis.off_target": "Hors cible",
  "p.kpis.on_target": "Dans la cible",
  "p.kpis.percent_defects_gbp": "pourcentage, défauts, EUR…",
  "p.kpis.plan_optional_pid": "Plan (pid facultatif)",
  "p.kpis.quarterly": "Trimestrielle",
  "p.kpis.read_how_often": "Fréquence de mesure",
  "p.kpis.red_line": "Ligne rouge",
  "p.kpis.show": "Afficher",
  "p.kpis.something_you_track_over_time":
    "Une valeur que vous suivez dans le temps par rapport à une cible, pour voir tôt si vous êtes sur la bonne voie.",
  "p.kpis.standing_measures_green_is_at":
    "Indicateurs permanents. Vert : égal ou meilleur que la cible ; rouge : égal ou pire que la ligne rouge ; ambre : entre les deux. Un KPI encore jamais mesuré affiche « Aucune donnée pour l'instant ».",
  "p.kpis.target": "Cible",
  "p.kpis.the_red_line_must_be":
    "La ligne rouge doit se situer du côté défavorable de la cible. Exprimez les valeurs en nombres entiers (points de base, centimes).",
  "p.kpis.unit": "Unité",
  "p.kpis.watch": "À surveiller",
  "p.kpis.weekly": "Hebdomadaire",
  "p.kpis.which_way_is_good": "Sens favorable",
  "p.layout.display": "Affichage",
  "p.layout.more": "Plus",
  "p.layout.simple_view": "Vue simplifiée : {v}",
  "p.lifecycle.as_of": "au {as_of}",
  "p.lifecycle.in_an_unrecognized_phase": "dans une phase non reconnue",
  "p.lifecycle.lifecycle": "Cycle de vie",
  "p.lifecycle.lifecycle_ppm": "Cycle de vie — PPM",
  "p.lifecycle.live": "Actifs",
  "p.lifecycle.live_items_one":
    "{total} élément actif · seuil de stagnation {stall}",
  "p.lifecycle.live_items_other":
    "{total} éléments actifs · seuil de stagnation {stall}",
  "p.lifecycle.phase": "Phase",
  "p.lifecycle.share": "Part",
  "p.lifecycle.stalled": "Bloqués",
  "p.lifecycle.where_everything_sits_across_the":
    "La position de chaque élément dans le cycle de vie, pour que la phase suivante ne soit jamais une surprise. « Bloqués » compte les éléments restés dans une phase au-delà du seuil.",
  "p.me.calendar.add": "Ajouter",
  "p.me.calendar.at_least_one_weekday_must":
    "Au moins un jour de semaine doit être travaillé.",
  "p.me.calendar.from": "Du",
  "p.me.calendar.loading": "Chargement…",
  "p.me.calendar.most_i_can_commit": "Engagement maximal (%)",
  "p.me.calendar.my_calendar": "Mon calendrier",
  "p.me.calendar.my_calendar_ppm": "Mon calendrier — PPM",
  "p.me.calendar.my_days_off": "Mes jours de congé",
  "p.me.calendar.my_work": "← Mon travail",
  "p.me.calendar.none_set": "Aucun défini.",
  "p.me.calendar.note": "Note",
  "p.me.calendar.part_time_spells_and_leave":
    "Périodes à temps partiel et congés",
  "p.me.calendar.remove": "Supprimer",
  "p.me.calendar.save": "Enregistrer",
  "p.me.calendar.saved": "Enregistré.",
  "p.me.calendar.the_days_i_work": "Mes jours travaillés",
  "p.me.calendar.the_most_of_your_time":
    "La part maximale de votre temps que vous pouvez engager entre deux dates : 60 pour une période à temps partiel, 0 pour un congé. En dehors de ces dates, elle est de 100. La capacité est lue jour par jour : une courte période n'est jamais noyée dans une moyenne.",
  "p.me.calendar.to": "Du {from} au {to} : {max_percent} %{v}",
  "p.me.calendar.to_2": "Au",
  "p.me.due": "· échéance {due_date}",
  "p.me.in_plans_you_cannot_read":
    "· {omitted_unreadable} dans des plans que vous ne pouvez pas lire",
  "p.me.loading": "Chargement…",
  "p.me.my_calendar": "Mon calendrier",
  "p.me.my_work": "Mon travail",
  "p.me.my_work_ppm": "Mon travail — PPM",
  "p.me.nothing_assigned_to_you_enjoy":
    "Rien ne vous est attribué. Profitez-en.",
  "p.me.open": "{today} ({tz}) · {total} ouverts",
  "p.me.showing_the_first_500": "· affichage des 500 premiers",
  "p.me.the_days_you_work_so":
    "— vos jours travaillés, pour que vos dates soient fiables.",
  "p.me.your_sign_in_identity_cannot":
    "Votre identité de connexion ne peut pas être associée aux affectations de tâches ; rien ne peut donc encore être listé ici.",
  "p.milestones.a_milestone_is_a_dated":
    "Une étape clé est un engagement daté. Reliez-la aux tâches qu'elle attend : elle est atteinte quand elles sont terminées, avec une prévision tirée du calendrier. Sans lien, une personne la marque comme faite.",
  "p.milestones.achieved": "Atteinte",
  "p.milestones.all": "Toutes",
  "p.milestones.due_soon": "Échéance proche",
  "p.milestones.forecast": "Prévision :",
  "p.milestones.late": "En retard",
  "p.milestones.loading": "Chargement…",
  "p.milestones.milestones": "Étapes clés",
  "p.milestones.milestones_by_status": "Étapes clés par statut",
  "p.milestones.milestones_ppm": "Étapes clés — PPM",
  "p.milestones.show": "Afficher",
  "p.milestones.upcoming": "À venir",
  "p.objectives.objectives_ppm": "Objectifs — PPM",
  "p.objectives.period_e_g_2026_h2": "Période (p. ex. 2026-H2)",
  "p.okrs.2026_q4_blank_for_all": "2026-Q4 (vide pour tout)",
  "p.okrs.a_goal_with_a_few":
    "Un but, avec quelques résultats mesurables qui montrent s'il est atteint.",
  "p.okrs.create": "Créer",
  "p.okrs.goals_and_key_results_okrs": "Objectifs et résultats clés (OKR)",
  "p.okrs.loading": "Chargement…",
  "p.okrs.new_objective": "Nouvel objectif",
  "p.okrs.objectives_and_key_results_scores":
    "Objectifs et résultats clés. Les scores sont recalculés à partir des points d'étape à chaque consultation ; rien n'est stocké qui pourrait devenir obsolète.",
  "p.okrs.okrs_ppm": "OKR — PPM",
  "p.okrs.period": "Période",
  "p.okrs.show": "Afficher",
  "p.okrs.title": "Titre",
  "p.onboarding.a_role_by_role_first":
    "Un parcours de première heure, rôle par rôle. Chaque étape indique la page où elle mène. C'est un ordre suggéré, pas une promesse mesurée du type « vous serez productif en N minutes » — personne ne l'a chronométré.",
  "p.onboarding.onboarding_ppm": "Prise en main — PPM",
  "p.onboarding.operator_onboarding": "Prise en main pour les opérateurs",
  "p.plans.main_x": "{t} — Main X",
  "p.plans.merge.ppm": "{t} — PPM",
  "p.plans.new.main_x": "{t} — Main X",
  "p.plans.pid.board.action": "action",
  "p.plans.pid.board.add_note": "Ajouter une note",
  "p.plans.pid.board.add_sprint": "Ajouter un sprint",
  "p.plans.pid.board.add_task": "Ajouter une tâche",
  "p.plans.pid.board.board": "Tableau",
  "p.plans.pid.board.burndown": "Burndown — {name}",
  "p.plans.pid.board.convert_to_task": "Convertir en tâche",
  "p.plans.pid.board.created_moves_blocked_now":
    "{length} créées · {length2} déplacements · {length3} bloquées actuellement",
  "p.plans.pid.board.date": "Date",
  "p.plans.pid.board.feedback": "retour",
  "p.plans.pid.board.improve": "à améliorer",
  "p.plans.pid.board.new_task_title": "Titre de la nouvelle tâche",
  "p.plans.pid.board.no_notes_for_this_sprint":
    "Aucune note pour ce sprint pour l'instant.",
  "p.plans.pid.board.none": "(aucun)",
  "p.plans.pid.board.note": "Note",
  "p.plans.pid.board.note_body": "Texte de la note",
  "p.plans.pid.board.note_category": "Catégorie de la note",
  "p.plans.pid.board.points_done": "Points réalisés",
  "p.plans.pid.board.ppm": "{t} — PPM",
  "p.plans.pid.board.pts": "pts",
  "p.plans.pid.board.remaining_of": "Restant sur {total_tasks}",
  "p.plans.pid.board.retro_feedback_notes": "Notes de rétrospective / retours",
  "p.plans.pid.board.sprint": "Sprint",
  "p.plans.pid.board.sprint_end": "Fin du sprint",
  "p.plans.pid.board.sprint_name": "Nom du sprint",
  "p.plans.pid.board.sprint_start": "Début du sprint",
  "p.plans.pid.board.sprints": "Sprints",
  "p.plans.pid.board.standup_last_24h": "Standup (dernières 24 h)",
  "p.plans.pid.board.story_points": "Points d'histoire",
  "p.plans.pid.board.task_created": "→ tâche créée",
  "p.plans.pid.board.tasks_done": "Tâches terminées",
  "p.plans.pid.board.unpointed": "Non estimées",
  "p.plans.pid.board.velocity": "Vélocité",
  "p.plans.pid.board.went_well": "s'est bien passé",
  "p.plans.pid.calendar.at_least_one_weekday_must":
    "Au moins un jour de semaine doit être travaillé.",
  "p.plans.pid.calendar.critical_path": "← Chemin critique",
  "p.plans.pid.calendar.holidays": "Jours fériés",
  "p.plans.pid.calendar.loading": "Chargement…",
  "p.plans.pid.calendar.my_calendar": "Mon calendrier",
  "p.plans.pid.calendar.save_calendar": "Enregistrer le calendrier",
  "p.plans.pid.calendar.saved": "Enregistré.",
  "p.plans.pid.calendar.their_work_then_happens_only":
    ") ; leur travail n'a alors lieu que les jours travaillés dans les deux calendriers.",
  "p.plans.pid.calendar.working_calendar": "Calendrier de travail",
  "p.plans.pid.calendar.working_calendar_ppm": "Calendrier de travail — PPM",
  "p.plans.pid.calendar.working_weekdays": "Jours de semaine travaillés",
  "p.plans.pid.cost_risk.band_for_lines_with_no":
    "Marge pour les lignes sans estimation (± %)",
  "p.plans.pid.cost_risk.cost_and_schedule_together":
    "Coût et calendrier ensemble",
  "p.plans.pid.cost_risk.cost_risk": "Risque de coût",
  "p.plans.pid.cost_risk.cost_risk_ppm": "Risque de coût — PPM",
  "p.plans.pid.cost_risk.distribution": "Distribution",
  "p.plans.pid.cost_risk.iterations": "Itérations",
  "p.plans.pid.cost_risk.many_what_if_runs_of":
    "De nombreuses simulations du budget montrent la fourchette des coûts finaux et la probabilité de chacun.",
  "p.plans.pid.cost_risk.pert": "PERT",
  "p.plans.pid.cost_risk.plan": "← Plan",
  "p.plans.pid.cost_risk.run": "Exécuter",
  "p.plans.pid.cost_risk.running": "Exécution…",
  "p.plans.pid.cost_risk.seed": "Graine",
  "p.plans.pid.cost_risk.triangular": "Triangulaire",
  "p.plans.pid.cost_risk.whole_portfolio": "Tout le portefeuille",
  "p.plans.pid.costs.a_recorded_cost_keeps_the":
    "Un coût enregistré conserve le taux auquel il a été valorisé ; un changement de taux ultérieur ne le modifie jamais.",
  "p.plans.pid.costs.add_an_expense": "Ajouter une dépense",
  "p.plans.pid.costs.add_expense": "Ajouter la dépense",
  "p.plans.pid.costs.all_at_the_end": "Tout à la fin",
  "p.plans.pid.costs.all_at_the_start": "Tout au début",
  "p.plans.pid.costs.amount": "Montant",
  "p.plans.pid.costs.an_expense_of_so_it":
    "▲ Une dépense de {money} {reason}, elle n'apparaît donc pas dans les périodes.",
  "p.plans.pid.costs.choose": "Choisir…",
  "p.plans.pid.costs.cost_rates": "Taux de coût",
  "p.plans.pid.costs.costs": "Coûts",
  "p.plans.pid.costs.costs_and_rates_are_finance":
    "Les coûts et les taux sont des données financières. Vous n'y avez pas accès.",
  "p.plans.pid.costs.costs_ppm": "Coûts — PPM",
  "p.plans.pid.costs.currency": "Devise",
  "p.plans.pid.costs.description": "Description",
  "p.plans.pid.costs.evenly_across_the_task": "Réparti sur toute la tâche",
  "p.plans.pid.costs.falls": "Imputation",
  "p.plans.pid.costs.fixed_cost": "Coût fixe",
  "p.plans.pid.costs.kind": "Nature",
  "p.plans.pid.costs.material": "Matériel",
  "p.plans.pid.costs.no_time_has_been_costed":
    "Aucun temps n'a encore été chiffré.",
  "p.plans.pid.costs.nothing_could_be_costed_yet":
    "Rien n'a encore pu être chiffré : ajoutez des taux pour les personnes qui réalisent le travail, ou ajoutez une dépense.",
  "p.plans.pid.costs.per_period": "{currency}, par période",
  "p.plans.pid.costs.plan": "← Plan",
  "p.plans.pid.costs.planned_cost": "Coût prévu",
  "p.plans.pid.costs.record_labour_costs":
    "Enregistrer les coûts de main-d'œuvre",
  "p.plans.pid.costs.recorded_labour_cost": "Coût de main-d'œuvre enregistré",
  "p.plans.pid.costs.so_its_labour_is_not":
    "▲ {title} ({hoursText}) : {reason}, sa main-d'œuvre n'est donc pas chiffrée.",
  "p.plans.pid.costs.task": "Tâche",
  "p.plans.pid.costs.time_entries_are_not_costed":
    "▲ {entries} saisies de temps ({hoursText}) ne sont pas chiffrées : pas encore enregistrées, ou sans taux. Leur coût est inconnu, et non nul.",
  "p.plans.pid.costs.total": "Total",
  "p.plans.pid.costs.work": "Travail",
  "p.plans.pid.critical_path.a_baseline_is_a_numbered":
    "Une référence est un instantané numéroté du calendrier, conservé définitivement : une nouvelle référence crée la version suivante et n'en remplace jamais une.",
  "p.plans.pid.critical_path.baselines": "Références",
  "p.plans.pid.critical_path.business_days": "jours ouvrés",
  "p.plans.pid.critical_path.compare_with": "Comparer avec",
  "p.plans.pid.critical_path.constraint": "Contrainte",
  "p.plans.pid.critical_path.constraint_date": "Date de contrainte",
  "p.plans.pid.critical_path.critical_chains":
    "{count} chaînes critiques{more}.",
  "p.plans.pid.critical_path.critical_path": "Chemin critique",
  "p.plans.pid.critical_path.critical_path_ppm": "Chemin critique — PPM",
  "p.plans.pid.critical_path.deadline_a_goal_it_shows":
    "Date limite (un objectif : elle apparaît comme une marge négative, elle ne déplace jamais la tâche)",
  "p.plans.pid.critical_path.duration_business_days_0_is":
    "Durée (jours ouvrés ; 0 correspond à une étape clé)",
  "p.plans.pid.critical_path.finish_no_earlier_than": "Fin au plus tôt le",
  "p.plans.pid.critical_path.finish_no_later_than": "Fin au plus tard le",
  "p.plans.pid.critical_path.gantt": "Gantt",
  "p.plans.pid.critical_path.loading": "Chargement…",
  "p.plans.pid.critical_path.must_finish_on_overrides_logic":
    "Doit se terminer le (prime sur la logique)",
  "p.plans.pid.critical_path.must_start_on_overrides_logic":
    "Doit commencer le (prime sur la logique)",
  "p.plans.pid.critical_path.near_critical_within": "Quasi critique à moins de",
  "p.plans.pid.critical_path.network": "Réseau",
  "p.plans.pid.critical_path.no_baseline_yet_so_nothing":
    "Aucune référence pour l'instant : impossible de juger d'un glissement.",
  "p.plans.pid.critical_path.none_as_soon_as_possible":
    "Aucune (dès que possible)",
  "p.plans.pid.critical_path.not_scheduled": "Non planifiée",
  "p.plans.pid.critical_path.optimistic_business_days":
    "Optimiste (jours ouvrés)",
  "p.plans.pid.critical_path.pessimistic_business_days":
    "Pessimiste (jours ouvrés)",
  "p.plans.pid.critical_path.plan": "← Plan",
  "p.plans.pid.critical_path.planned_start": "Début prévu",
  "p.plans.pid.critical_path.range_for_schedule_risk":
    "Fourchette pour le risque de calendrier",
  "p.plans.pid.critical_path.range_saved": "Fourchette enregistrée.",
  "p.plans.pid.critical_path.reason_required_this_overrides_the":
    "Raison (obligatoire : cela prime sur la logique du calendrier)",
  "p.plans.pid.critical_path.save_range": "Enregistrer la fourchette",
  "p.plans.pid.critical_path.save_schedule": "Enregistrer le calendrier",
  "p.plans.pid.critical_path.saved_moved": "Enregistré. Déplacées :",
  "p.plans.pid.critical_path.saved_nothing_else_moved":
    "Enregistré. Rien d'autre n'a bougé.",
  "p.plans.pid.critical_path.saving": "Enregistrement…",
  "p.plans.pid.critical_path.show_the_baseline_under_each":
    "Afficher la référence sous chaque barre",
  "p.plans.pid.critical_path.start_no_earlier_than": "Début au plus tôt le",
  "p.plans.pid.critical_path.start_no_later_than": "Début au plus tard le",
  "p.plans.pid.critical_path.table": "Tableau",
  "p.plans.pid.critical_path.take_baseline": "Prendre une référence",
  "p.plans.pid.critical_path.task": "Tâche",
  "p.plans.pid.critical_path.the_duration_above_is_the":
    "{taskRangeText}. La durée ci-dessus est la valeur probable.",
  "p.plans.pid.critical_path.the_longest_chain_of_tasks":
    "La plus longue chaîne de tâches, qui fixe la date de fin. Tout retard sur l'une d'elles retarde l'ensemble du plan.",
  "p.plans.pid.critical_path.these_assignees_share_no_business":
    "Ces responsables n'ont aucun jour ouvré en commun avec le calendrier du plan ; celui-ci a donc été utilisé pour eux : {join}.",
  "p.plans.pid.critical_path.this_network_has_tasks_too":
    "Ce réseau compte {length} tâches, trop pour un tracé utile. Le tableau ci-dessous dit la même chose.",
  "p.plans.pid.critical_path.untitled": "(sans titre)",
  "p.plans.pid.critical_path.version": "Version {version} : {reason} ({slice})",
  "p.plans.pid.critical_path.view": "Vue",
  "p.plans.pid.critical_path.why_a_baseline_now":
    "Pourquoi une référence maintenant ?",
  "p.plans.pid.critical_path.working_calendar": "Calendrier de travail",
  "p.plans.pid.earned_value.a_percent_i_enter_flagged":
    "Un pourcentage que je saisis (signalé)",
  "p.plans.pid.earned_value.actual_cost": "Coût réel",
  "p.plans.pid.earned_value.against_the_budget_vac":
    "Par rapport au budget (VAC)",
  "p.plans.pid.earned_value.baseline_version":
    "version de référence {baseline_version}",
  "p.plans.pid.earned_value.budget_at_completion": "Budget à l'achèvement",
  "p.plans.pid.earned_value.budget_lines_in_have_no":
    "Les lignes budgétaires en {join} n'ont pas de référence : elles ne figurent dans aucun chiffre ci-dessus et ne sont jamais additionnées à une autre devise.",
  "p.plans.pid.earned_value.by_story_points_when_finished":
    "en points d'histoire, une fois terminée",
  "p.plans.pid.earned_value.cancel": "Annuler",
  "p.plans.pid.earned_value.change": "Modifier",
  "p.plans.pid.earned_value.complete": "{round} % achevé",
  "p.plans.pid.earned_value.cost_index": "Indice de coût (",
  "p.plans.pid.earned_value.cost_is": "Le coût est {word}.",
  "p.plans.pid.earned_value.cost_variance_ev_ac": "Écart de coût (EV − AC)",
  "p.plans.pid.earned_value.count_a_task_another_way":
    "Compter une tâche autrement",
  "p.plans.pid.earned_value.count_tasks": "Compter les tâches",
  "p.plans.pid.earned_value.critical_path": "Chemin critique",
  "p.plans.pid.earned_value.earned_value": "Valeur acquise",
  "p.plans.pid.earned_value.earned_value_figures":
    "Chiffres de valeur acquise en {currency}",
  "p.plans.pid.earned_value.earned_value_ppm": "Valeur acquise — PPM",
  "p.plans.pid.earned_value.final_cost_eac": "Coût final (EAC)",
  "p.plans.pid.earned_value.finished_only": "terminées uniquement",
  "p.plans.pid.earned_value.forecast": "Prévision",
  "p.plans.pid.earned_value.forecast_final_cost": "· coût final prévu {money}",
  "p.plans.pid.earned_value.forecasts_of_the_final_cost":
    "Prévisions du coût final en {currency}, chacune selon sa propre formule",
  "p.plans.pid.earned_value.governance": "← Gouvernance",
  "p.plans.pid.earned_value.half_for_started": "moitié si commencée",
  "p.plans.pid.earned_value.is_the_work_finished_worth":
    "Le travail achevé vaut-il ce qui était prévu à ce stade, et combien a-t-il coûté ? Un indice supérieur à 1,00 est meilleur que prévu.",
  "p.plans.pid.earned_value.leading_with_that_one_because":
    "Celle-ci est mise en avant car {why}. Les autres sont affichées à côté : la bonne dépend de la poursuite ou non des dépassements passés.",
  "p.plans.pid.earned_value.loading": "Chargement…",
  "p.plans.pid.earned_value.manual": "manuel",
  "p.plans.pid.earned_value.measured_as_of_percent_complete":
    "Mesuré au {as_of}. Le pourcentage d'achèvement correspond au travail terminé, pas au temps écoulé.",
  "p.plans.pid.earned_value.method": "Méthode",
  "p.plans.pid.earned_value.only_when_finished": "uniquement une fois terminée",
  "p.plans.pid.earned_value.only_when_finished_2":
    "Uniquement une fois terminée",
  "p.plans.pid.earned_value.over_time": "Dans le temps",
  "p.plans.pid.earned_value.percent_complete": "Pourcentage d'achèvement",
  "p.plans.pid.earned_value.planned_value": "Valeur planifiée",
  "p.plans.pid.earned_value.plans": "{plans} plans",
  "p.plans.pid.earned_value.progress_against_budget_earned_value":
    "Avancement par rapport au budget (valeur acquise)",
  "p.plans.pid.earned_value.record_today_s_figures_now":
    "Enregistrer les chiffres du jour maintenant",
  "p.plans.pid.earned_value.recorded": "Enregistré.",
  "p.plans.pid.earned_value.save": "Enregistrer",
  "p.plans.pid.earned_value.schedule_index": "Indice de calendrier (",
  "p.plans.pid.earned_value.schedule_index_by_time":
    "Indice de calendrier en temps (",
  "p.plans.pid.earned_value.schedule_is": "Le calendrier est {word}.",
  "p.plans.pid.earned_value.schedule_variance_ev_pv":
    "Écart de calendrier (EV − PV)",
  "p.plans.pid.earned_value.spi": "SPI {ratioOrReason} ·",
  "p.plans.pid.earned_value.started_work_counts_half":
    "le travail commencé compte pour moitié",
  "p.plans.pid.earned_value.started_work_counts_half_2":
    "Le travail commencé compte pour moitié",
  "p.plans.pid.earned_value.still_to_spend_etc": "Reste à dépenser (ETC)",
  "p.plans.pid.earned_value.task": "(tâche)",
  "p.plans.pid.earned_value.task_2": "Tâche",
  "p.plans.pid.earned_value.the_one_to_lead_with": "— celle à mettre en avant",
  "p.plans.pid.earned_value.the_scheduler_also_records_them":
    "Le planificateur les enregistre aussi chaque jour.",
  "p.plans.pid.earned_value.this_plan_and_everything_beneath":
    "Ce plan et tout ce qui en dépend",
  "p.plans.pid.earned_value.unknown": "inconnu",
  "p.plans.pid.earned_value.use_the_plan_s_method":
    "Utiliser la méthode du plan",
  "p.plans.pid.earned_value.where_it_is_heading": "Où cela mène",
  "p.plans.pid.earned_value.why_required": "Pourquoi (obligatoire)",
  "p.plans.pid.edit.main_x": "{t} — Main X",
  "p.plans.pid.flow.180_calendar_days": "180 jours calendaires",
  "p.plans.pid.flow.30_calendar_days": "30 jours calendaires",
  "p.plans.pid.flow.60_calendar_days": "60 jours calendaires",
  "p.plans.pid.flow.90_calendar_days": "90 jours calendaires",
  "p.plans.pid.flow.age": "Âge",
  "p.plans.pid.flow.aging_work_in_progress": "Ancienneté du travail en cours",
  "p.plans.pid.flow.arrival_rate": "Taux d'arrivée (λ)",
  "p.plans.pid.flow.assignee": "Responsable",
  "p.plans.pid.flow.blocked": "Bloqués",
  "p.plans.pid.flow.calendar_day": "{v} / jour calendaire",
  "p.plans.pid.flow.column": "Colonne",
  "p.plans.pid.flow.columns": "Colonnes",
  "p.plans.pid.flow.concentrated": "concentré",
  "p.plans.pid.flow.constraints": "Contraintes",
  "p.plans.pid.flow.count": "Nombre",
  "p.plans.pid.flow.cumulative_flow": "Flux cumulé",
  "p.plans.pid.flow.cycle_time_is_work_in":
    "Le temps de cycle est le travail en cours divisé par le débit : abaisser une limite le raccourcit sans que personne ne travaille plus vite.",
  "p.plans.pid.flow.detail": "Détail",
  "p.plans.pid.flow.distributions": "Distributions",
  "p.plans.pid.flow.first_pass_yield": "Rendement au premier passage",
  "p.plans.pid.flow.flow": "Flux",
  "p.plans.pid.flow.flow_efficiency": "Efficacité du flux",
  "p.plans.pid.flow.flow_ppm": "Flux — PPM",
  "p.plans.pid.flow.how_long_work_takes": "Durée du travail",
  "p.plans.pid.flow.how_long_work_takes_to":
    "Le temps nécessaire pour terminer le travail, et la quantité en cours simultanément.",
  "p.plans.pid.flow.implied_cycle_time": "Temps de cycle implicite",
  "p.plans.pid.flow.item": "Élément",
  "p.plans.pid.flow.limit": "Limite",
  "p.plans.pid.flow.max": "max.",
  "p.plans.pid.flow.mean": "moyenne",
  "p.plans.pid.flow.measure": "Mesure",
  "p.plans.pid.flow.min": "min.",
  "p.plans.pid.flow.no_cap": "sans limite",
  "p.plans.pid.flow.no_expectation_yet": "aucune attente pour l'instant",
  "p.plans.pid.flow.no_findings_yet": "Aucun constat pour l'instant.",
  "p.plans.pid.flow.no_finished_work_to_measure":
    "Aucun travail terminé à mesurer pour l'instant.",
  "p.plans.pid.flow.not_enough_finished_items_yet":
    "Pas encore assez d'éléments terminés.",
  "p.plans.pid.flow.not_started_finished_of":
    "{not_started} non commencées · {finished} terminées sur {tasks}",
  "p.plans.pid.flow.nothing_in_progress": "Rien en cours.",
  "p.plans.pid.flow.observed_median": "Médiane observée",
  "p.plans.pid.flow.of_the_calendar_time_these":
    "Sur le temps calendaire passé par ces éléments, quelle part a réellement été travaillée ? Dans le travail intellectuel, l'efficacité du flux se situe généralement entre 5 et 15 % : un chiffre bas est normal — le constat porte sur la file d'attente, pas sur l'effort.",
  "p.plans.pid.flow.of_the_transitions_behind_these":
    "{percent} des transitions à l'origine de ces chiffres ont été synthétisées lors de la mise en place du journal, et non observées. Elles se consolident à mesure que de vrais déplacements sur le tableau s'accumulent.",
  "p.plans.pid.flow.percentiles_are_nearest_rank_so":
    "Les percentiles sont calculés au rang le plus proche : chacun correspond à un élément réel. Le temps de cycle est ce que l'équipe maîtrise ; le délai de livraison est ce qu'attend le demandeur, et leur différence est le temps passé par le travail dans le backlog.",
  "p.plans.pid.flow.recoverable": "Récupérable",
  "p.plans.pid.flow.rework": "Reprise",
  "p.plans.pid.flow.rule": "Règle",
  "p.plans.pid.flow.service_level_expectation": "Niveau de service attendu",
  "p.plans.pid.flow.status": "Statut",
  "p.plans.pid.flow.strong_against_the_5_15":
    "Élevée au regard de la norme de 5 à 15 %.",
  "p.plans.pid.flow.subject": "Objet",
  "p.plans.pid.flow.the_mean_is_shown_for":
    "La moyenne est affichée par souci d'exhaustivité et ne décrit aucun élément réel — citez le p85.",
  "p.plans.pid.flow.the_plan_s_overall_ratio":
    ": le ratio global du plan ({percent}) est très éloigné de celui de l'élément type ({percent2}). La plupart des éléments avancent ; une minorité est bloquée. Le remède diffère d'une livraison uniformément lente — trouvez ces éléments plutôt que de remanier le processus.",
  "p.plans.pid.flow.the_waste_is": "Le gaspillage est",
  "p.plans.pid.flow.throughput": "Débit (μ)",
  "p.plans.pid.flow.typical_median_item":
    "Typique. Élément médian : {percent}.",
  "p.plans.pid.flow.unassigned": "non affecté",
  "p.plans.pid.flow.unusually_high_this_usually_means":
    "Inhabituellement élevé — cela signifie généralement que le tableau n'est pas tenu à jour, plutôt que la file d'attente a disparu.",
  "p.plans.pid.flow.utilization": "Taux d'utilisation (ρ)",
  "p.plans.pid.flow.vs_expectation": "par rapport à l'attente",
  "p.plans.pid.flow.window": "Fenêtre",
  "p.plans.pid.flow.work_in_progress": "Travail en cours",
  "p.plans.pid.flow.work_in_progress_2": "Travail en cours (κ)",
  "p.plans.pid.governance.actual": "Réel",
  "p.plans.pid.governance.approved": "approuvé",
  "p.plans.pid.governance.approved_with_conditions": "approuvé sous conditions",
  "p.plans.pid.governance.benefit": "Bénéfice",
  "p.plans.pid.governance.benefit_category": "Catégorie de bénéfice",
  "p.plans.pid.governance.capex": "investissement",
  "p.plans.pid.governance.category": "Catégorie",
  "p.plans.pid.governance.choose_objective": "choisir un objectif…",
  "p.plans.pid.governance.commit": "Engager",
  "p.plans.pid.governance.commit_anyway": "Engager quand même",
  "p.plans.pid.governance.committing_this_puts_them_over":
    "Cet engagement dépasse leur capacité.",
  "p.plans.pid.governance.compliance": "conformité",
  "p.plans.pid.governance.cost_saving": "économie de coûts",
  "p.plans.pid.governance.currency": "Devise",
  "p.plans.pid.governance.decision": "Décision",
  "p.plans.pid.governance.governance_ppm": "Gouvernance — PPM",
  "p.plans.pid.governance.hold": "en attente",
  "p.plans.pid.governance.line": "Ligne",
  "p.plans.pid.governance.materialized": "matérialisés : {materialized}",
  "p.plans.pid.governance.max_exposure": "exposition max. : {max_exposure}",
  "p.plans.pid.governance.new_benefit": "Nouveau bénéfice",
  "p.plans.pid.governance.new_budget_line": "Nouvelle ligne budgétaire",
  "p.plans.pid.governance.new_milestone": "Nouvelle étape clé",
  "p.plans.pid.governance.new_milestone_due":
    "Date d'échéance du nouveau jalon",
  "p.plans.pid.governance.new_risk": "Nouveau risque",
  "p.plans.pid.governance.next": "suivant : {next_gate}",
  "p.plans.pid.governance.objective": "Objectif",
  "p.plans.pid.governance.open_risks": "risques ouverts : {open}",
  "p.plans.pid.governance.opex": "fonctionnement",
  "p.plans.pid.governance.other": "autre",
  "p.plans.pid.governance.planned": "Prévu",
  "p.plans.pid.governance.planned_actual_variance":
    " : prévu {money} · réel {money2} · écart {money3}",
  "p.plans.pid.governance.planned_major": "Prévu (unités majeures)",
  "p.plans.pid.governance.proposed_does_not_count_until":
    "proposé (non comptabilisé avant engagement)",
  "p.plans.pid.governance.quality": "qualité",
  "p.plans.pid.governance.range_for_cost_risk":
    "Fourchette pour le risque de coût",
  "p.plans.pid.governance.realized": "Réalisé",
  "p.plans.pid.governance.record_actual": "Saisir le réel",
  "p.plans.pid.governance.rejected": "rejeté",
  "p.plans.pid.governance.revenue": "chiffre d'affaires",
  "p.plans.pid.governance.review": "Revue",
  "p.plans.pid.governance.risk": "Risque",
  "p.plans.pid.governance.risk_reduction": "réduction des risques",
  "p.plans.pid.governance.roi": "· ROI {toFixed} %",
  "p.plans.pid.governance.stage": "étape : {v}",
  "p.plans.pid.governance.status": "Statut",
  "p.plans.pid.governance.target": "Cible",
  "p.plans.pid.governance.target_major": "Cible (unités majeures)",
  "p.plans.pid.governance.target_realized_spend":
    " : cible {money} · réalisé {money2} · dépense {money3}",
  "p.plans.pid.governance.weight": "poids",
  "p.plans.pid.governance.worker_uuid": "worker:<uuid>",
  "p.plans.pid.handover.discard": "Abandonner",
  "p.plans.pid.handover.from": "De",
  "p.plans.pid.handover.hand_over": "Transférer",
  "p.plans.pid.handover.handover": "Passation",
  "p.plans.pid.handover.handover_ppm": "Passation — PPM",
  "p.plans.pid.handover.kind": "Type",
  "p.plans.pid.handover.onboarding_and_offboarding": "Arrivées et départs",
  "p.plans.pid.handover.person": "person:…",
  "p.plans.pid.handover.person_empty_releases_the_work":
    "person:… (vide pour libérer le travail)",
  "p.plans.pid.handover.plan": "← Plan",
  "p.plans.pid.handover.preview": "Aperçu",
  "p.plans.pid.handover.reason_recorded_with_each_item":
    "Motif (consigné avec chaque élément)",
  "p.plans.pid.handover.release": "Libérer",
  "p.plans.pid.handover.the_new_person_s_capacity":
    "La capacité de la nouvelle personne n'est pas vérifiée ici.",
  "p.plans.pid.handover.to": "À",
  "p.plans.pid.handover.what": "Quoi",
  "p.plans.pid.handover.what_holds_that_is_still":
    "Ce que {from_person} détient encore en cours",
  "p.plans.pid.handover.who_is_handing_over": "Qui transfère ?",
  "p.plans.pid.import.bring_tasks_in_from_jira":
    "Importez des tâches depuis Jira, Asana ou Microsoft Project. Vous voyez d'abord ce qui se passerait ; rien n'est écrit avant votre confirmation.",
  "p.plans.pid.import.choose_another_file": "Choisir un autre fichier",
  "p.plans.pid.import.file": "Fichier",
  "p.plans.pid.import.import": "Importer",
  "p.plans.pid.import.import_ppm": "Importation — PPM",
  "p.plans.pid.import.imported": "Importé",
  "p.plans.pid.import.plan": "← Plan",
  "p.plans.pid.import.preview": "Aperçu",
  "p.plans.pid.import.tracker": "Outil de suivi",
  "p.plans.pid.import.where_from": "Provenance",
  "p.plans.pid.joint_risk.a_late_project_usually_costs":
    "Un projet en retard coûte généralement plus cher aussi : cette simulation traite donc coût et délais ensemble.",
  "p.plans.pid.joint_risk.assumptions": "Hypothèses",
  "p.plans.pid.joint_risk.band_when_there_is_no":
    "Marge en l'absence d'estimation (± %)",
  "p.plans.pid.joint_risk.both_at_once": "Les deux à la fois",
  "p.plans.pid.joint_risk.budget": "budget",
  "p.plans.pid.joint_risk.budget_2": "Budget",
  "p.plans.pid.joint_risk.budget_lines_in_are_not":
    "Les lignes budgétaires en {join} ne font pas partie de cette simulation ; les devises ne sont jamais additionnées.",
  "p.plans.pid.joint_risk.business_days_late":
    "{days} jours ouvrés de retard, {money}",
  "p.plans.pid.joint_risk.business_days_late_2": "jours ouvrés de retard →",
  "p.plans.pid.joint_risk.cost": "Coût",
  "p.plans.pid.joint_risk.cost_2": "Coût — {title} : {costSkippedText}",
  "p.plans.pid.joint_risk.cost_and_schedule_risk_ppm":
    "Risque de coût et de délais — PPM",
  "p.plans.pid.joint_risk.cost_and_schedule_together":
    "Coût et délais ensemble",
  "p.plans.pid.joint_risk.cost_lines": "Lignes de coût",
  "p.plans.pid.joint_risk.cost_of_each_business_day":
    "Coût de chaque jour ouvré de retard (unités majeures)",
  "p.plans.pid.joint_risk.cost_risk": "Risque de coût",
  "p.plans.pid.joint_risk.critical_path_finish_the_plan":
    "Fin selon le chemin critique (date propre du plan)",
  "p.plans.pid.joint_risk.currency_if_the_plan_has":
    "Devise (si le plan en a plusieurs)",
  "p.plans.pid.joint_risk.distribution": "Distribution",
  "p.plans.pid.joint_risk.each_dot_is_one_simulation":
    "Chaque point est une simulation (échantillon réduit). À droite de la ligne « à temps » : en retard ; au-dessus de la ligne budgétaire : hors budget. Le tableau ci-dessus donne les décomptes complets.",
  "p.plans.pid.joint_risk.estimate": "Estimation",
  "p.plans.pid.joint_risk.finish": "Fin",
  "p.plans.pid.joint_risk.finish_date_at_each_percentile":
    "Date de fin à chaque percentile",
  "p.plans.pid.joint_risk.how_the_simulated_outcomes_fall":
    "Répartition des résultats simulés autour du budget et de la fin selon le chemin critique",
  "p.plans.pid.joint_risk.independence": "Indépendance.",
  "p.plans.pid.joint_risk.iterations": "Itérations",
  "p.plans.pid.joint_risk.line": "Ligne",
  "p.plans.pid.joint_risk.low_planned_high": "Bas / prévu / haut",
  "p.plans.pid.joint_risk.mean_delay_cost_included":
    "Moyenne (coût du retard inclus)",
  "p.plans.pid.joint_risk.no_register_risk_was_used":
    "Aucun risque du registre n'a été utilisé dans cette simulation.",
  "p.plans.pid.joint_risk.not_enough_data": "données insuffisantes",
  "p.plans.pid.joint_risk.of_which_delay_cost_on":
    "Dont coût du retard, en moyenne",
  "p.plans.pid.joint_risk.on_time": "à temps",
  "p.plans.pid.joint_risk.outcome": "Résultat",
  "p.plans.pid.joint_risk.per_line_estimate_in":
    "Estimation par ligne en {currency}",
  "p.plans.pid.joint_risk.pert": "PERT",
  "p.plans.pid.joint_risk.plan": "← Plan",
  "p.plans.pid.joint_risk.risks": "Risques",
  "p.plans.pid.joint_risk.run": "Lancer",
  "p.plans.pid.joint_risk.running": "Exécution…",
  "p.plans.pid.joint_risk.scatter_of_simulated_total_cost":
    "Nuage de points du coût total simulé en fonction des jours ouvrés de retard, avec la ligne budgétaire et la ligne « à temps »",
  "p.plans.pid.joint_risk.schedule":
    "Calendrier — {title} : {skippedReasonText}",
  "p.plans.pid.joint_risk.schedule_risk": "Risque de délais",
  "p.plans.pid.joint_risk.seed": "Graine",
  "p.plans.pid.joint_risk.seed_the_same_seed_gives":
    "Graine {seed} : la même graine donne le même résultat.",
  "p.plans.pid.joint_risk.share": "Part",
  "p.plans.pid.joint_risk.simulations": "Simulations",
  "p.plans.pid.joint_risk.total_cost_delay_cost_included":
    "Coût total, coût du retard inclus",
  "p.plans.pid.joint_risk.triangular": "Triangulaire",
  "p.plans.pid.leveling.allow_critical_tasks_to_move":
    "Autoriser le déplacement des tâches critiques (peut retarder la fin)",
  "p.plans.pid.leveling.apply": "Appliquer",
  "p.plans.pid.leveling.apply_the_ticked_moves":
    "Appliquer les déplacements cochés",
  "p.plans.pid.leveling.balance_workloads_resource_leveling":
    "Équilibrer les charges (nivellement des ressources)",
  "p.plans.pid.leveling.could_not_be_fixed": "Impossible à résoudre",
  "p.plans.pid.leveling.critical": "critique",
  "p.plans.pid.leveling.each_start_date_the_proposal":
    "Chaque date de début que la proposition modifierait",
  "p.plans.pid.leveling.finish": "Fin",
  "p.plans.pid.leveling.float": "Marge",
  "p.plans.pid.leveling.i_have_read_the_proposal":
    "J'ai lu la proposition et je veux modifier ces dates de début",
  "p.plans.pid.leveling.later_by": "Décalé de",
  "p.plans.pid.leveling.load_per_task_of_a":
    "Charge par tâche (% de la journée d'une personne)",
  "p.plans.pid.leveling.loading": "Chargement…",
  "p.plans.pid.leveling.no_start_date_would_change":
    "Aucune date de début ne changerait.",
  "p.plans.pid.leveling.nobody_is_over_allocated_so":
    "Personne n'est surallouée : il n'y a rien à niveler.",
  "p.plans.pid.leveling.capacity_undeclared_h": "Aucune capacité déclarée",
  "p.plans.pid.leveling.capacity_undeclared_p":
    "Ces personnes ont du travail des jours sans disponibilité déclarée ; ces jours-là, elles ne sont ni au-delà ni dans leur capacité :",
  "p.plans.pid.leveling.over_allocated_now": "Surallouées actuellement",
  "p.plans.pid.leveling.person": "Personne",
  "p.plans.pid.leveling.plan": "← Plan",
  "p.plans.pid.leveling.propose_again": "Proposer à nouveau",
  "p.plans.pid.leveling.proposed_start_dates": "Dates de début proposées",
  "p.plans.pid.leveling.refused": "Refusé : {note}",
  "p.plans.pid.leveling.resource_leveling_ppm":
    "Nivellement des ressources — PPM",
  "p.plans.pid.leveling.start": "Début",
  "p.plans.pid.leveling.start_finish_business_days":
    "{pid} : début {v}{start_days}, fin {v2}{finish_days} jours ouvrés",
  "p.plans.pid.leveling.suggests_moving_tasks_so_that":
    "Propose de déplacer des tâches pour que personne ne soit planifié au-delà de ses possibilités. Rien ne change avant votre confirmation.",
  "p.plans.pid.leveling.task": "Tâche",
  "p.plans.pid.leveling.the_search_stopped_at_its":
    "la recherche s'est arrêtée à sa limite",
  "p.plans.pid.leveling.this_is_advice_nothing_changes":
    "Ceci est un conseil. Rien ne change tant que vous n'avez pas choisi et appliqué des déplacements ci-dessous.",
  "p.plans.pid.leveling.why_kept_with_each_audit":
    "Motif (conservé avec chaque entrée d'audit)",
  "p.plans.pid.leveling.working_calendar": "Calendrier ouvré",
  "p.plans.pid.more_tools": "Autres outils",
  "p.plans.pid.onboarding.access": "Accès",
  "p.plans.pid.onboarding.access_record": "Registre des accès",
  "p.plans.pid.onboarding.action": "Action",
  "p.plans.pid.onboarding.cancel_checklist": "Annuler la liste de contrôle",
  "p.plans.pid.onboarding.checklists": "Listes de contrôle",
  "p.plans.pid.onboarding.complete_checklist": "Terminer la liste de contrôle",
  "p.plans.pid.onboarding.done": "Fait",
  "p.plans.pid.onboarding.effective":
    "{status}{v} · effet au {effective_date} · {progressText}",
  "p.plans.pid.onboarding.from":
    "{status} · à partir du {effective_date} · {progressText}",
  "p.plans.pid.onboarding.granted": "Accordé",
  "p.plans.pid.onboarding.hand_over_someone_s_work":
    "Transférer le travail de quelqu'un",
  "p.plans.pid.onboarding.no_access_is_outstanding":
    "● Aucun accès en suspens.",
  "p.plans.pid.onboarding.none_yet": "Aucun pour l'instant.",
  "p.plans.pid.onboarding.onboarding_and_offboarding": "Arrivées et départs",
  "p.plans.pid.onboarding.onboarding_and_offboarding_ppm":
    "Arrivées et départs — PPM",
  "p.plans.pid.onboarding.person": "person:…",
  "p.plans.pid.onboarding.person_2": "Personne",
  "p.plans.pid.onboarding.plan": "← Plan",
  "p.plans.pid.onboarding.ready_to_close": "Prêt à clôturer ?",
  "p.plans.pid.onboarding.reason": "Motif",
  "p.plans.pid.onboarding.reason_for_cancelling": "Motif de l'annulation",
  "p.plans.pid.onboarding.record": "Enregistrer",
  "p.plans.pid.onboarding.record_access": "Enregistrer un accès",
  "p.plans.pid.onboarding.reopen": "Rouvrir",
  "p.plans.pid.onboarding.repo_write": "écriture sur le dépôt",
  "p.plans.pid.onboarding.revoked": "Révoqué",
  "p.plans.pid.onboarding.save": "Enregistrer",
  "p.plans.pid.onboarding.since":
    "▲ {person_ref} : {access} (depuis le {granted_on})",
  "p.plans.pid.onboarding.skip": "Ignorer",
  "p.plans.pid.onboarding.start": "Démarrer",
  "p.plans.pid.onboarding.start_a_checklist": "Démarrer une liste de contrôle",
  "p.plans.pid.onboarding.takes_effect": "Prend effet",
  "p.plans.pid.onboarding.what": "Quoi",
  "p.plans.pid.onboarding.who_holds_what_access_is":
    "Savoir qui détient quel accès concerne des personnes : cela exige l'autorisation de gérer les personnes.",
  "p.plans.pid.onboarding.why_skip": "Pourquoi ignorer ?",
  "p.plans.pid.raci.add_a_person_person_or":
    "Ajouter une personne (person:… ou worker:…)",
  "p.plans.pid.raci.add_a_person_to_start":
    "Ajoutez une personne pour commencer à attribuer des lettres.",
  "p.plans.pid.raci.add_column": "Ajouter une colonne",
  "p.plans.pid.raci.export_to_a_spreadsheet": "Exporter vers un tableur",
  "p.plans.pid.raci.for_each_task_who_does":
    "Pour chaque tâche : qui la réalise, qui la valide, qui est consulté et qui est informé.",
  "p.plans.pid.raci.have_nobody_accountable":
    "{missing_accountable} n'ont aucun approbateur (A).",
  "p.plans.pid.raci.have_nobody_responsible":
    "{missing_responsible} n'ont aucun réalisateur (R).",
  "p.plans.pid.raci.how_many_tasks_each_person":
    "Nombre de tâches pour lesquelles chaque personne détient chaque lettre",
  "p.plans.pid.raci.legend": "Légende",
  "p.plans.pid.raci.load_by_person": "Charge par personne",
  "p.plans.pid.raci.loading": "Chargement…",
  "p.plans.pid.raci.of_tasks_are_complete_one":
    "{complete} tâches sur {tasks} sont complètes (un approbateur, au moins un réalisateur).",
  "p.plans.pid.raci.person": "Personne",
  "p.plans.pid.raci.plan": "← Plan",
  "p.plans.pid.raci.raci_ppm": "RACI — PPM",
  "p.plans.pid.raci.this_plan_has_no_tasks":
    "Ce plan n'a pas encore de tâches.",
  "p.plans.pid.raci.who_does_what_raci_matrix": "Qui fait quoi (matrice RACI)",
  "p.plans.pid.schedule.dependency_violated_successor_starts":
    "Dépendance non respectée : le successeur commence le {actual_start} mais ne peut pas commencer avant le {earliest_start}.",
  "p.plans.pid.schedule.schedule_ppm": "Calendrier — PPM",
  "p.plans.pid.schedule_health.as_of": "Au {as_of} ·",
  "p.plans.pid.schedule_health.baseline_version":
    "version de référence {version}",
  "p.plans.pid.schedule_health.check": "Vérifier",
  "p.plans.pid.schedule_health.checks_the_schedule_for_common":
    "Recherche dans le calendrier les problèmes courants, comme des tâches sans liens ou sans estimation.",
  "p.plans.pid.schedule_health.critical_path": "Chemin critique",
  "p.plans.pid.schedule_health.held_to": "Comparé à",
  "p.plans.pid.schedule_health.loading": "Chargement…",
  "p.plans.pid.schedule_health.no_baseline_taken":
    "aucune référence enregistrée",
  "p.plans.pid.schedule_health.plan": "← Plan",
  "p.plans.pid.schedule_health.re_run": "Relancer",
  "p.plans.pid.schedule_health.result": "Résultat",
  "p.plans.pid.schedule_health.schedule_health": "Santé du calendrier",
  "p.plans.pid.schedule_health.schedule_health_ppm":
    "Santé du calendrier — PPM",
  "p.plans.pid.schedule_health.see_the_note": "Voir la note",
  "p.plans.pid.schedule_health.status_date": "Date d'état",
  "p.plans.pid.schedule_health.the_fourteen_schedule_health_checks":
    "Les quatorze contrôles de santé du calendrier",
  "p.plans.pid.schedule_health.to_fix": "À corriger",
  "p.plans.pid.schedule_health.what_was_found": "Constats",
  "p.plans.pid.schedule_risk.assumptions": "Hypothèses",
  "p.plans.pid.schedule_risk.band_for_tasks_with_no":
    "Marge pour les tâches sans estimation (± %)",
  "p.plans.pid.schedule_risk.critical_in": "Critique dans",
  "p.plans.pid.schedule_risk.critical_path": "Chemin critique",
  "p.plans.pid.schedule_risk.critical_path_finish_the_plan":
    "Fin selon le chemin critique (date propre du plan)",
  "p.plans.pid.schedule_risk.distribution": "Distribution",
  "p.plans.pid.schedule_risk.each_bar_is_a_finish":
    "Chaque barre est une date de fin ; plus elle est haute, plus de simulations s'y sont terminées. Les lignes pleines sont P50, P80 et P90 (étiquetées) ; la ligne pointillée est la date propre du plan.",
  "p.plans.pid.schedule_risk.earliest_latest":
    "{join}. Au plus tôt {earliest}, au plus tard {latest}.",
  "p.plans.pid.schedule_risk.estimate": "Estimation",
  "p.plans.pid.schedule_risk.finish": "Fin",
  "p.plans.pid.schedule_risk.finish_date": "Date de fin",
  "p.plans.pid.schedule_risk.finish_date_at_each_percentile":
    "Date de fin à chaque percentile",
  "p.plans.pid.schedule_risk.histogram_of_simulated_finish_dates":
    "Histogramme des dates de fin simulées, avec P50, P80 et P90 marqués",
  "p.plans.pid.schedule_risk.independence": "Indépendance.",
  "p.plans.pid.schedule_risk.iterations": "Itérations",
  "p.plans.pid.schedule_risk.left_out": "Exclues :",
  "p.plans.pid.schedule_risk.many_what_if_runs_of":
    "De nombreuses simulations du calendrier montrent l'éventail des dates de fin et la probabilité de chacune.",
  "p.plans.pid.schedule_risk.no_register_risk_was_used":
    "Aucun risque du registre n'a été utilisé comme événement de calendrier.",
  "p.plans.pid.schedule_risk.not_enough_data": "données insuffisantes",
  "p.plans.pid.schedule_risk.not_simulated_because_the_schedule":
    "Non simulées, car le calendrier ne peut pas les placer : {join}.",
  "p.plans.pid.schedule_risk.of": "{date} : {count} sur {iterations}",
  "p.plans.pid.schedule_risk.of_runs": "{criticalityText} des simulations",
  "p.plans.pid.schedule_risk.optimistic_likely_pessimistic_busine":
    "Optimiste / probable / pessimiste (jours ouvrés)",
  "p.plans.pid.schedule_risk.outcome": "Résultat",
  "p.plans.pid.schedule_risk.per_task_estimate_criticality_and":
    "Estimation, criticité et sensibilité par tâche",
  "p.plans.pid.schedule_risk.plan": "← Plan",
  "p.plans.pid.schedule_risk.plan_2": "plan",
  "p.plans.pid.schedule_risk.risks_used": "Risques utilisés",
  "p.plans.pid.schedule_risk.run": "Lancer",
  "p.plans.pid.schedule_risk.run_on_against_the_plan":
    "Exécuté le {ranOn} sur le plan dans son état actuel · graine {seed} · {iterations} itérations · {distributionText} · calendrier {calendar}",
  "p.plans.pid.schedule_risk.running": "Exécution…",
  "p.plans.pid.schedule_risk.schedule_risk": "Risque de délais",
  "p.plans.pid.schedule_risk.schedule_risk_ppm": "Risque de délais — PPM",
  "p.plans.pid.schedule_risk.seed": "Graine",
  "p.plans.pid.schedule_risk.seed_the_same_seed_gives":
    "Graine {seed} : la même graine donne le même résultat. Jusqu'à {max_iterations} itérations.",
  "p.plans.pid.schedule_risk.sensitivity": "Sensibilité",
  "p.plans.pid.schedule_risk.share": "Part",
  "p.plans.pid.schedule_risk.simulations": "Simulations",
  "p.plans.pid.schedule_risk.task": "Tâche",
  "p.plans.pid.schedule_risk.the_same_figures_as_a":
    "Les mêmes chiffres sous forme de tableau",
  "p.plans.pid.schedule_risk.triangular": "Triangulaire",
  "p.plans.pid.schedule_risk.weighted_towards_the_likely_value":
    "Pondérée vers la valeur probable (PERT)",
  "p.plans.pid.schedule_risk.what_drives_it": "Facteurs déterminants",
  "p.plans.pid.schedule_risk.when_it_finishes": "Date d'achèvement",
  "p.plans.pid.wbs.code": "Code",
  "p.plans.pid.wbs.level": "Niveau",
  "p.plans.pid.wbs.no_tasks_yet": "Aucune tâche pour l'instant.",
  "p.plans.pid.wbs.plan": "← Plan",
  "p.plans.pid.wbs.sub_tasks": "Sous-tâches",
  "p.plans.pid.wbs.task": "Tâche",
  "p.plans.pid.wbs.tasks_is_too_many_to":
    "{length} tâches, c'est trop pour une imbrication utile ; le tableau les affiche toutes.",
  "p.plans.pid.wbs.the_plan_s_tasks_each":
    "Les tâches du plan, chacune décomposée en sous-tâches. Le code de chaque ligne sert à la lecture : il change quand du travail est ajouté ou déplacé.",
  "p.plans.pid.wbs.work_breakdown": "Structure de découpage du travail",
  "p.plans.pid.wbs.work_breakdown_as_a_table":
    "Découpage du travail sous forme de tableau",
  "p.plans.pid.wbs.work_breakdown_ppm": "Découpage du travail — PPM",
  "p.plans.pid.wbs.work_breakdown_tree": "Arborescence du découpage du travail",
  "p.plans.pid.what_if.after": "Après",
  "p.plans.pid.what_if.before": "Avant",
  "p.plans.pid.what_if.business_days": "Jours ouvrés",
  "p.plans.pid.what_if.choose": "Choisir…",
  "p.plans.pid.what_if.commit_these_changes": "Valider ces modifications",
  "p.plans.pid.what_if.constraint": "Contrainte",
  "p.plans.pid.what_if.critical": "■ critique",
  "p.plans.pid.what_if.critical_path": "Chemin critique",
  "p.plans.pid.what_if.date": "Date",
  "p.plans.pid.what_if.discard": "Abandonner",
  "p.plans.pid.what_if.edit": "Modifier",
  "p.plans.pid.what_if.finish_to_finish": "Fin à fin",
  "p.plans.pid.what_if.finish_to_start": "Fin à début",
  "p.plans.pid.what_if.float": "Marge",
  "p.plans.pid.what_if.follows": "{name} suit {name2} ({toUpperCase})",
  "p.plans.pid.what_if.joins_the_critical_path":
    "Entrent dans le chemin critique : {join}.",
  "p.plans.pid.what_if.lag_business_days": "Décalage (jours ouvrés)",
  "p.plans.pid.what_if.leaves_it": "En sortent : {join}.",
  "p.plans.pid.what_if.link": "Lien",
  "p.plans.pid.what_if.new_start": "Nouveau début",
  "p.plans.pid.what_if.no_task_s_dates_or":
    "Aucune date ni marge de tâche ne change.",
  "p.plans.pid.what_if.none_clear_it": "Aucune (l'effacer)",
  "p.plans.pid.what_if.none_yet": "Aucune pour l'instant.",
  "p.plans.pid.what_if.now_takes_of_the_day":
    "« {name} » occupe désormais {units_percent} % de la journée pour {hoursText} de travail (calculé selon sa règle de planification).",
  "p.plans.pid.what_if.of_work_has_no_rate":
    "▲ {hoursText} de travail n'ont pas de taux et ne sont donc pas chiffrées.",
  "p.plans.pid.what_if.plan": "← Plan",
  "p.plans.pid.what_if.planned_labour_cost":
    "Coût de main-d'œuvre prévu : {deltaText} ({deltaText2} → {deltaText3}).",
  "p.plans.pid.what_if.predecessor": "Prédécesseur",
  "p.plans.pid.what_if.preview": "Aperçu",
  "p.plans.pid.what_if.reason": "Motif",
  "p.plans.pid.what_if.reason_recorded_with_each_change":
    "Motif (consigné avec chaque modification)",
  "p.plans.pid.what_if.remove": "Supprimer",
  "p.plans.pid.what_if.stage_an_edit": "Préparer une modification",
  "p.plans.pid.what_if.stage_edit": "Préparer la modification",
  "p.plans.pid.what_if.staged_edits": "Modifications préparées",
  "p.plans.pid.what_if.start_to_finish": "Début à fin",
  "p.plans.pid.what_if.start_to_start": "Début à début",
  "p.plans.pid.what_if.successor": "Successeur",
  "p.plans.pid.what_if.task": "Tâche",
  "p.plans.pid.what_if.tasks_whose_dates_float_or":
    "Tâches dont les dates, la marge ou le statut critique changent",
  "p.plans.pid.what_if.try_changes_on_a_copy":
    "Essayez des modifications sur une copie. Rien n'est enregistré avant validation.",
  "p.plans.pid.what_if.type": "Type",
  "p.plans.pid.what_if.what_if": "Simulation",
  "p.plans.pid.what_if.what_if_ppm": "Simulation — PPM",
  "p.plans.pid.work.business_days_of_a_day":
    "{duration_days} jours ouvrés · {units_percent} % d'une journée · {hoursText} de travail · {v}",
  "p.plans.pid.work.change": "Modifier",
  "p.plans.pid.work.costs": "Coûts",
  "p.plans.pid.work.day": "Jour",
  "p.plans.pid.work.duration_business_days": "Durée (jours ouvrés)",
  "p.plans.pid.work.edit_assignment": "Modifier l'affectation",
  "p.plans.pid.work.falls_outside_this_window":
    "{hoursText} tombent hors de cette période.",
  "p.plans.pid.work.hours_per_each_row_adds":
    "Heures par {grain} ; chaque ligne totalise le travail de la tâche, la dernière celui du plan.",
  "p.plans.pid.work.how_long_each_task_takes":
    "Durée de chaque tâche, part de la journée d'une personne qu'elle occupe et quantité de travail correspondante.",
  "p.plans.pid.work.month": "Mois",
  "p.plans.pid.work.no_assignment_yet_give_a":
    "Aucune affectation pour l'instant. Indiquez une durée et des unités, et le travail est calculé.",
  "p.plans.pid.work.no_task_carries_work_yet":
    "Aucune tâche ne porte encore de travail : il n'y a rien à répartir.",
  "p.plans.pid.work.not_placed": "(non placé)",
  "p.plans.pid.work.percentages": "Pourcentages",
  "p.plans.pid.work.period": "Période",
  "p.plans.pid.work.plan": "← Plan",
  "p.plans.pid.work.plan_2": "Plan",
  "p.plans.pid.work.rule": "Règle",
  "p.plans.pid.work.save_assignment": "Enregistrer l'affectation",
  "p.plans.pid.work.save_contour": "Enregistrer le profil",
  "p.plans.pid.work.so_its_work_is_not":
    "▲ {title} ({hoursText}) {reason} : son travail n'est donc pas dans les tranches.",
  "p.plans.pid.work.spread": "Répartition",
  "p.plans.pid.work.task": "Tâche",
  "p.plans.pid.work.tasks": "Tâches",
  "p.plans.pid.work.this_plan_has_no_scheduled":
    "Ce plan n'a pas encore de tâches planifiées.",
  "p.plans.pid.work.total": "Total",
  "p.plans.pid.work.units": "Unités (%)",
  "p.plans.pid.work.units_of_the_day": "Unités (% de la journée)",
  "p.plans.pid.work.week": "Semaine",
  "p.plans.pid.work.work_and_assignments": "Travail et affectations",
  "p.plans.pid.work.work_and_assignments_ppm": "Travail et affectations — PPM",
  "p.plans.pid.work.work_hours": "Travail (heures)",
  "p.plans.pid.work.work_over_time": "Travail dans le temps",
  "p.prioritization.all": "tous",
  "p.prioritization.apply": "Appliquer",
  "p.prioritization.band": "tranche",
  "p.prioritization.band_2": "Tranche",
  "p.prioritization.component": "Composante",
  "p.prioritization.coverage": "couverture",
  "p.prioritization.coverage_2": "Couverture",
  "p.prioritization.explain": "Expliquer",
  "p.prioritization.hide": "Masquer",
  "p.prioritization.high": "haute",
  "p.prioritization.loading_the_breakdown": "Chargement du détail…",
  "p.prioritization.low": "basse",
  "p.prioritization.medium": "moyenne",
  "p.prioritization.missing_evidence": "Éléments manquants",
  "p.prioritization.no_evidence_for_any_component":
    "Aucun élément pour aucune composante — ce plan n'est pas noté.",
  "p.prioritization.no_plans_match_that_filter":
    "Aucun plan ne correspond à ce filtre.",
  "p.prioritization.none": "aucune",
  "p.prioritization.of_plans_one": "{length} plan sur {scored_of}",
  "p.prioritization.of_plans_other": "{length} plans sur {scored_of}",
  "p.prioritization.plan": "Plan",
  "p.prioritization.points": "Points",
  "p.prioritization.prioritization": "Priorisation",
  "p.prioritization.prioritization_ppm": "Priorisation — PPM",
  "p.prioritization.score": "Score",
  "p.prioritization.shows_how_much_of_the":
    "indique quelle part du score repose réellement sur des données.",
  "p.prioritization.smart_score_over_roi_strategic":
    "Score intelligent fondé sur le ROI, l'alignement stratégique, l'avis d'experts, le risque, la demande, la priorité MoSCoW et la dynamique. Les composantes sans éléments sont écartées et les autres renormalisées —",
  "p.prioritization.unscored": "non noté",
  "p.prioritization.value": "Valeur",
  "p.prioritization.weight": "Poids",
  "p.proposals.actions": "Actions",
  "p.proposals.currency": "Devise",
  "p.proposals.live": "actif :",
  "p.proposals.promote": "promouvoir",
  "p.proposals.proposal": "proposition :",
  "p.proposals.proposals_ppm": "Propositions — PPM",
  "p.proposals.requested": "Demandé",
  "p.proposals.status": "Statut",
  "p.proposals.status_filter": "Filtre de statut",
  "p.proposals.target": "Cible",
  "p.proposals.target_collection": "Collection cible",
  "p.proposals.title": "Titre",
  "p.regulator.actual": "Réel",
  "p.regulator.benefits": "Bénéfices :",
  "p.regulator.currency": "Devise",
  "p.regulator.gate_decisions": "· Décisions de jalon :",
  "p.regulator.members": "Membres :",
  "p.regulator.names_withheld_abac_mask_obligation":
    "Noms masqués (obligation de masquage ABAC).",
  "p.regulator.no_budget_lines": "Aucune ligne budgétaire.",
  "p.regulator.no_portfolios_recorded": "Aucun portefeuille enregistré.",
  "p.regulator.none": "aucun",
  "p.regulator.planned": "Prévu",
  "p.regulator.ppm": "{t} — PPM",
  "p.regulator.stage": "étape : {v}",
  "p.regulator.target_realized": "cible {money} / réalisé {money2}",
  "p.reports.collection": "Collection",
  "p.reports.csv": "CSV",
  "p.reports.fields_comma_separated": "champs (séparés par des virgules)",
  "p.reports.name_contains": "le nom contient…",
  "p.reports.reports_ppm": "Rapports — PPM",
  "p.resources.a_multi_skill_booking_counts":
    "Une réservation multicompétence compte entièrement pour chacune de ses compétences : les lignes de compétences se chevauchent et leur somme ne donne pas le total.",
  "p.resources.by": "Par",
  "p.resources.demand_fte": "Demande en FTE",
  "p.resources.demand_supply_and_gap_by":
    "Demande, offre et écart par période et par {group_by}",
  "p.resources.every_figure_as_a_table":
    "Tous les chiffres sous forme de tableau",
  "p.resources.from": "Du",
  "p.resources.gap_fte": "Écart en FTE",
  "p.resources.gap_per_in_fte_supply":
    "Écart par {period}, en FTE (offre moins demande)",
  "p.resources.loading": "Chargement…",
  "p.resources.month": "Mois",
  "p.resources.no_demand_or_supply_in":
    "Aucune demande ni offre sur cette période.",
  "p.resources.not_known": "inconnu",
  "p.resources.of_which_unassigned": "Dont non affecté",
  "p.resources.people": "Personnes",
  "p.resources.per": "Par",
  "p.resources.per_by_scope":
    "{periodLabel} · par {period} · par {group_by} · périmètre : {kind}",
  "p.resources.period": "Période",
  "p.resources.proposed_fte": "FTE proposés",
  "l.processes.name_problem":
    "Donnez au processus un nom de 1 à 120 caractères.",
  "l.processes.steps_problem": "Un processus comporte de 1 à {max} étapes.",
  "l.processes.title_problem":
    "Étape {n} : donnez-lui un titre de 1 à 200 caractères.",
  "l.processes.person_problem":
    "Étape {n} : la personne doit être une référence person: ou worker:.",
  "l.processes.kind_approve": "Approuver",
  "l.processes.kind_todo": "À faire",
  "l.processes.kind_notify": "Notifier",
  "l.processes.state_pending": "non atteinte",
  "l.processes.state_waiting": "en attente",
  "l.processes.state_completed": "faite",
  "l.processes.state_skipped": "ignorée",
  "l.processes.state_rejected": "rejetée",
  "l.processes.state_failed": "échouée",
  "l.processes.run_running": "en cours",
  "l.processes.run_completed": "terminée",
  "l.processes.run_rejected": "rejetée",
  "l.processes.run_cancelled": "annulée",
  "l.processes.run_failed": "échouée",
  "p.processes.processes": "Processus d'approbation",
  "p.processes.processes_ppm": "Processus d'approbation — PPM",
  "p.processes.intro":
    "Un processus est une liste d'étapes exécutées l'une après l'autre : demander une approbation, confier une tâche à faire ou informer quelqu'un. Un rejet met fin à l'exécution et ignore les étapes suivantes. Modifier un processus ne change jamais une exécution déjà en cours.",
  "p.processes.definitions": "Processus",
  "p.processes.none_defined": "Aucun processus n'est encore défini.",
  "p.processes.version": "version {v}",
  "p.processes.new_process": "Définir un processus",
  "p.processes.name": "Nom",
  "p.processes.step_n": "Étape {n}",
  "p.processes.kind": "Ce qu'elle fait",
  "p.processes.person": "Personne (person:… ou worker:…)",
  "p.processes.step_title": "Ce qu'elle indique",
  "p.processes.add_step": "Ajouter une étape",
  "p.processes.remove_step": "Supprimer l'étape {n}",
  "p.processes.create": "Définir le processus",
  "p.processes.defined": "{name} défini.",
  "p.processes.start_heading": "Lancer une exécution de {name}",
  "p.processes.subject_kind": "Objet concerné",
  "p.processes.subject_pid": "Son identifiant",
  "p.processes.start": "Lancer l'exécution",
  "p.processes.started": "Exécution de {name} lancée.",
  "p.processes.runs": "Vos exécutions",
  "p.processes.none_runs": "Vous n'avez lancé aucune exécution.",
  "p.processes.run_of": "{name} sur {kind} {pid}",
  "p.processes.status": "Statut",
  "p.processes.show_run": "Afficher {name}",
  "p.processes.hide_run": "Masquer {name}",
  "p.processes.steps": "Étapes",
  "p.processes.log": "Historique",
  "p.processes.complete_step": "Marquer l'étape {n} comme faite",
  "p.processes.cancel": "Annuler l'exécution",
  "p.processes.resume": "Revérifier l'approbation",
  "p.processes.done": "Terminé.",
  "p.processes.could_not": "Cela n'a pas fonctionné : {reason}",
  "p.processes.could_not_load": "Impossible de charger les processus",
  "l.tolerance.green": "dans la tolérance",
  "l.tolerance.amber": "proche de la limite",
  "l.tolerance.red": "hors tolérance",
  "l.tolerance.unknown": "non mesurable",
  "l.tolerance.not_set": "aucune limite définie",
  "l.tolerance.time": "Délai",
  "l.tolerance.schedule": "Calendrier par rapport à la référence",
  "l.tolerance.cost": "Coût",
  "l.tolerance.risk": "Risque",
  "l.tolerance.resources": "Ressources",
  "l.tolerance.day_one": "{n} jour",
  "l.tolerance.day_other": "{n} jours",
  "l.tolerance.score": "score {n}",
  "l.tolerance.points": "{n} points au-delà de 100 %",
  "l.tolerance.field_time": "Limite (jours calendaires de retard)",
  "l.tolerance.field_schedule": "Limite (jours calendaires après la référence)",
  "l.tolerance.field_cost": "Limite (pourcentage au-delà du budget approuvé)",
  "l.tolerance.field_risk":
    "Limite (score de risque maximal acceptable, de 1 à 25)",
  "l.tolerance.field_resources":
    "Limite (points de pourcentage de surréservation)",
  "l.tolerance.no_limit": "Aucune limite n'est définie ici ni au-dessus.",
  "l.tolerance.could_not_measure": "Mesure impossible.",
  "l.tolerance.zero_tolerance":
    "Tolérance zéro : toute dérive dépasse la limite.",
  "l.tolerance.used": "{percent} % de la limite utilisés",
  "l.tolerance.own": "Définie sur ce plan",
  "l.tolerance.inherited": "Héritée d'un plan supérieur",
  "p.issue_settings.title": "Types, statuts et rôles des problèmes",
  "p.issue_settings.title_ppm": "Types, statuts et rôles des problèmes — PPM",
  "p.issue_settings.intro":
    "Le vocabulaire des problèmes propre à l'organisation. Seul un administrateur peut le modifier. Sans configuration, les problèmes utilisent les types et statuts intégrés.",
  "p.issue_settings.types_heading": "Types de problème",
  "p.issue_settings.types_intro":
    "Un type retiré ne peut plus être choisi. Les problèmes qui le portent le conservent.",
  "p.issue_settings.key": "Clé",
  "p.issue_settings.label": "Nom",
  "p.issue_settings.state": "État",
  "p.issue_settings.in_use": "En service",
  "p.issue_settings.retired": "Retiré",
  "p.issue_settings.retire": "Retirer",
  "p.issue_settings.restore": "Restaurer",
  "p.issue_settings.actions": "Opérations",
  "p.issue_settings.add_type": "Ajouter un type",
  "p.issue_settings.roles_heading": "Rôles",
  "p.issue_settings.roles_intro":
    "Les rôles que cette organisation reconnaît et ce que chacun permet sur un plan ayant des membres de rôle. Un rôle ne peut que restreindre ce que la connexion d'une personne permet déjà. Un flux de travail ne peut nommer qu'un rôle de la liste, afin qu'un déplacement ne soit pas limité à un rôle mal orthographié.",
  "p.issue_settings.description": "Descriptif",
  "p.issue_settings.add_role": "Ajouter un rôle",
  "p.issue_settings.workflows_heading": "Statuts et transitions",
  "p.issue_settings.workflows_intro":
    "Un flux de travail fixe les statuts d'un type de problème, qui peut faire chaque déplacement et quels champs chaque statut exige ou verrouille. Un flux pour un type l'emporte sur celui de tous les types. Les problèmes dans un statut supprimé le conservent.",
  "p.issue_settings.none_workflows":
    "Aucun flux de problèmes n'est enregistré : les statuts intégrés s'appliquent.",
  "p.issue_settings.wf_name": "Nom du flux",
  "p.issue_settings.wf_type": "S'applique à",
  "p.issue_settings.wf_all_types": "Tous les types sans flux propre",
  "p.issue_settings.wf_plan": "Un plan",
  "p.issue_settings.statuses": "Statuts",
  "p.issue_settings.category": "Signification",
  "p.issue_settings.initial": "Commence ici",
  "p.issue_settings.terminal": "Exige une résolution",
  "p.issue_settings.add_status": "Ajouter un statut",
  "p.issue_settings.moves": "Transitions autorisées",
  "p.issue_settings.moves_intro":
    "Sans transition listée, toute transition est permise. Dès qu'une est listée, seules les listées le sont.",
  "p.issue_settings.from": "De",
  "p.issue_settings.to": "Vers",
  "p.issue_settings.move_roles":
    "Rôles (séparés par des virgules ; vide pour tous)",
  "p.issue_settings.add_move": "Ajouter une transition",
  "p.issue_settings.remove": "Supprimer",
  "p.issue_settings.register": "Enregistrer le flux",
  "p.issue_settings.withdraw": "Retirer",
  "p.issue_settings.saved": "Enregistré.",
  "p.issue_settings.could_not_load":
    "Impossible de charger les paramètres des problèmes.",
  "p.issue_settings.could_not_save":
    "Enregistrement impossible. Seul un administrateur peut modifier ces paramètres, et chaque clé doit être nouvelle et bien formée.",
  "p.issue_settings.cat_todo": "Non commencé",
  "p.issue_settings.cat_active": "En cours",
  "p.issue_settings.cat_waiting": "En attente",
  "p.issue_settings.cat_done": "Terminé",
  "p.issue_settings.translations":
    "Traductions (langue=nom, séparées par des points-virgules)",
  "p.issue_settings.bad_translations":
    "Écrivez chaque traduction sous la forme langue=nom, par exemple cy=Risg.",
  "p.issue_settings.save": "Enregistrer",
  "p.issue_settings.permissions": "Autorisations",
  "p.issue_settings.perm_issue_add": "Signaler des problèmes",
  "p.issue_settings.perm_issue_edit": "Modifier des problèmes",
  "p.issue_settings.perm_issue_move": "Déplacer des problèmes",
  "p.issue_settings.perm_issue_delete": "Retirer des problèmes",
  "p.issue_settings.edit": "Modifier",
  "p.issue_settings.save_workflow": "Enregistrer le flux de travail",
  "p.issue_settings.cancel": "Annuler",
  "p.issue_settings.editing": "Modification de {name}",
  "p.issue_settings.rule_field": "Champ",
  "p.issue_settings.rule_kind": "Règle",
  "p.issue_settings.rule_required": "Obligatoire",
  "p.issue_settings.rule_read_only": "Lecture seule",
  "p.issue_settings.rule_roles":
    "Rôles (séparés par des virgules ; vide pour tout le monde)",
  "p.issue_settings.add_rule": "Ajouter une règle de champ",
  "p.issue_settings.field_title": "Titre",
  "p.issue_settings.field_description": "Descriptif",
  "p.issue_settings.field_severity": "Gravité",
  "p.issue_settings.field_priority": "Priorité",
  "p.issue_settings.field_owner_ref": "Responsable",
  "p.issue_settings.field_due_date": "Date d'échéance",
  "p.issue_settings.field_resolution": "Résolution",
  "p.plan_settings.title": "Paramètres des problèmes de ce plan",
  "p.plan_settings.title_ppm": "Paramètres des problèmes de ce plan — PPM",
  "p.plan_settings.intro":
    "Qui détient quel rôle ici, quels types de problèmes ce plan propose, et ses propres flux de travail. Un administrateur effectue ces changements.",
  "p.plan_settings.back": "Retour aux problèmes de ce plan",
  "p.plan_settings.all_settings": "Paramètres de toute l'organisation",
  "p.plan_settings.members_heading": "Personnes et rôles",
  "p.plan_settings.members_intro":
    "Dès qu'un plan compte au moins une personne ici, signaler, modifier, déplacer et retirer des problèmes y requiert un rôle qui le permet. Un rôle ne peut pas donner plus que ce que permet la connexion, et il n'ouvre pas un plan privé.",
  "p.plan_settings.enforced":
    "Les actions sur les problèmes de ce plan requièrent un rôle.",
  "p.plan_settings.not_enforced":
    "Toute personne pouvant écrire peut agir sur les problèmes ici.",
  "p.plan_settings.person": "Identifiant d'utilisateur de la personne",
  "p.plan_settings.role": "Rôle",
  "p.plan_settings.choose_role": "Choisir un rôle",
  "p.plan_settings.add_member": "Ajouter une personne",
  "p.plan_settings.types_heading": "Types de problèmes de ce plan",
  "p.plan_settings.types_intro":
    "Cochez les types que ce plan propose. Sans coche, il les propose tous. Un problème conserve un type laissé de côté.",
  "p.plan_settings.save_types": "Enregistrer les types",
  "p.plan_settings.workflows_intro":
    "Un flux de travail ici s'applique à ce plan seulement et l'emporte sur celui de l'organisation. Un flux pour un type l'emporte sur celui de tous les types. Les problèmes dans un statut supprimé le conservent.",
  "p.plan_issues.settings_link": "Paramètres des problèmes de ce plan",
  "p.plan_issues.problem_fields_required": "Ce statut exige : {fields}.",
  "p.tolerances.tolerances": "Tolérances",
  "p.tolerances.tolerances_ppm": "Tolérances — PPM",
  "p.tolerances.plan_link": "Tolérances",
  "p.tolerances.intro":
    "Jusqu'où chaque projet peut dériver en délai, coût, risque et ressources avant de devoir être remonté, et quelle part de chaque limite est utilisée. Dans la limite mais proche : ambre ; au-delà : rouge ; une limite non mesurable est indiquée comme telle, jamais en vert.",
  "p.tolerances.board_intro":
    "Tous les plans lisibles par vous ayant une tolérance, les pires en premier.",
  "p.tolerances.none": "Aucun plan n'a encore de tolérance.",
  "p.tolerances.none_match": "Aucun plan ne correspond.",
  "p.tolerances.show": "Afficher",
  "p.tolerances.all_statuses": "Toutes les couleurs",
  "p.tolerances.plan": "Plan",
  "p.tolerances.overall": "Global",
  "p.tolerances.truncated":
    "Seuls les {n} premiers plans ont été mesurés : cette liste peut être incomplète.",
  "p.tolerances.loading": "Chargement…",
  "p.tolerances.could_not_load": "Impossible de charger les tolérances",
  "p.tolerances.back": "← Plan",
  "p.tolerances.board_link": "Toutes les tolérances",
  "p.tolerances.limit": "Limite",
  "p.tolerances.measured": "Mesuré",
  "p.tolerances.how": "Méthode de mesure",
  "p.tolerances.amber_from": "Ambre à partir de (pourcentage de la limite)",
  "p.tolerances.note": "Note",
  "p.tolerances.save": "Définir la limite",
  "p.tolerances.remove": "Supprimer la limite propre à ce plan",
  "p.tolerances.saved": "Limite {dimension} enregistrée.",
  "p.tolerances.removed": "Limite {dimension} supprimée.",
  "p.tolerances.could_not": "Cela n'a pas fonctionné : {reason}",
  "p.tolerances.limit_problem": "Indiquez la limite sous forme de nombre.",
  "l.escalation.open": "ouverte",
  "l.escalation.acknowledged": "prise en compte",
  "l.escalation.resolved": "résolue",
  "l.escalation.withdrawn": "retirée",
  "l.escalation.reason_tolerance": "Une tolérance est dépassée",
  "l.escalation.reason_risk": "Un risque",
  "l.escalation.reason_issue": "Un problème",
  "l.escalation.reason_dependency": "Une dépendance",
  "l.escalation.reason_decision": "Une décision est nécessaire",
  "l.escalation.reason_resource": "Ressources",
  "l.escalation.reason_other": "Autre chose",
  "l.escalation.outcome_decided": "Décidé",
  "l.escalation.outcome_guidance_given": "Orientations données",
  "l.escalation.outcome_rejected": "Rejeté",
  "l.escalation.event_raised": "Soulevée",
  "l.escalation.event_acknowledged": "Prise en compte",
  "l.escalation.event_level_raised": "Remontée d'un niveau",
  "l.escalation.event_resolved": "Résolue",
  "l.escalation.event_withdrawn": "Retirée",
  "l.escalation.title_problem": "Donnez-lui un titre de 1 à 200 caractères.",
  "l.escalation.dimension_problem": "Indiquez la tolérance concernée.",
  "l.escalation.level_problem": "Le niveau est un nombre entier de 1 à 9.",
  "l.escalation.owner_problem":
    "La personne doit être une référence person: ou worker:.",
  "l.escalation.level_to": "Niveau {level} : {to}",
  "l.escalation.level": "Niveau {level}",
  "l.escalation.open_for": "Ouverte depuis {n} jours",
  "l.escalation.took": "A pris {n} jours",
  "l.escalation.overdue": "en retard",
  "l.escalation.automatic": "Soulevée automatiquement",
  "p.escalations.escalations": "Escalades",
  "p.escalations.escalations_ppm": "Escalades — PPM",
  "p.escalations.plan_link": "Escalades",
  "p.task.task": "Tâche",
  "p.task.task_ppm": "Tâche — PPM",
  "p.task.back": "Retour au découpage du travail",
  "p.task.could_not_load": "Impossible de charger la tâche.",
  "p.task.loading": "Chargement…",
  "p.task.status": "Statut",
  "p.task.due": "Échéance",
  "p.task.assignee": "Affectée à",
  "p.task.description": "Description",
  "p.task.s_todo": "À faire",
  "p.task.s_in_progress": "En cours",
  "p.task.s_in_review": "En revue",
  "p.task.s_done": "Terminée",
  "p.task.s_blocked": "Bloquée",
  "p.task.links_intro":
    "Où cette tâche figure dans d'autres outils, comme le ticket Jira ou Linear qui la suit. L'adresse est enregistrée et affichée ; elle n'est jamais consultée et rien n'est synchronisé.",
  "p.task.back_board": "Retour au tableau",
  "p.task.could_not_load_links":
    "Impossible de charger les liens de la tâche vers d'autres tâches.",
  "p.task.could_not_load_tasks":
    "Impossible de charger les tâches du plan à lier.",
  "p.task.edit_heading": "Modifier la tâche",
  "p.task.edit_note":
    "Le statut change sur le tableau. Le sprint, le composant et la tâche parente restent tels quels.",
  "p.task.f_title": "Intitulé",
  "p.task.f_assignee":
    "Attribuée à (une référence person: ou worker: ; laisser vide pour personne)",
  "p.task.f_points":
    "Points d'effort (un nombre entier de 0 à 100 ; laisser vide pour aucun)",
  "p.task.f_due": "Date d'échéance (laisser vide pour aucune)",
  "p.task.save": "Enregistrer la tâche",
  "p.task.saved": "La tâche a été enregistrée.",
  "p.task.problem_title_required": "Donnez un intitulé à la tâche.",
  "p.task.problem_title_too_long": "L'intitulé est trop long.",
  "p.task.problem_assignee":
    "Indiquez la personne attribuée par une référence person: ou worker:, ou laissez vide.",
  "p.task.problem_points_whole": "Les points d'effort sont un nombre entier.",
  "p.task.problem_points_range":
    "Les points d'effort doivent être compris entre 0 et 100.",
  "p.task.problem_due": "La date d'échéance n'est pas une date.",
  "p.task.relations_heading": "Liens vers d'autres tâches",
  "p.task.relations_intro":
    "« Bloque » empêche de terminer l'autre tâche tant que celle-ci est ouverte. « Précède » exige que l'autre tâche soit due après celle-ci. Quand une tâche est terminée, toute tâche qui en est un doublon l'est aussi. « Liée à » et « Copiée de » sont purement informatifs.",
  "p.task.no_relations": "Cette tâche n'a aucun lien vers d'autres tâches.",
  "p.task.rel_related": "Liée à",
  "p.task.rel_blocks": "Bloque",
  "p.task.rel_blocked_by": "Bloquée par",
  "p.task.rel_precedes": "Précède",
  "p.task.rel_follows": "Suit",
  "p.task.rel_duplicates": "Doublon de",
  "p.task.rel_duplicated_by": "Dupliquée par",
  "p.task.rel_copied_from": "Copiée de",
  "p.task.rel_copied_to": "Copiée vers",
  "p.task.other_state": "(maintenant {status})",
  "p.task.other_state_due": "(maintenant {status}, échéance {date})",
  "p.task.remove_link": "Retirer",
  "p.task.remove_link_named": "Retirer le lien : {link} {title}",
  "p.task.hidden_links_one":
    "{n} autre lien mène à une tâche d'un plan que vous ne pouvez pas voir.",
  "p.task.hidden_links_other":
    "{n} autres liens mènent à des tâches de plans que vous ne pouvez pas voir.",
  "p.task.add_link_heading": "Lier à une autre tâche",
  "p.task.f_kind": "Cette tâche",
  "p.task.f_other": "Autre tâche",
  "p.task.choose_task": "Choisissez une tâche",
  "p.task.add_link": "Lier les tâches",
  "p.task.link_added": "Le lien a été ajouté.",
  "p.task.link_removed": "Le lien a été retiré.",
  "p.task.problem_choose_task": "Choisissez l'autre tâche.",
  "p.task.problem_self": "Une tâche ne peut pas être liée à elle-même.",
  "p.task.problem_exists": "Cette tâche a déjà ce lien.",
  "p.task.problem_cycle":
    "L'autre tâche a déjà ce lien vers celle-ci ; les deux s'attendraient l'une l'autre.",
  "p.task.problem_precedes_dates":
    "« Précède » exige que l'autre tâche soit due après celle-ci : modifiez d'abord une date d'échéance.",
  "p.task.refused_forbidden":
    "Vous ne pouvez pas modifier les tâches de ce plan.",
  "p.task.refused_gone":
    "Cette tâche n'existe plus, ou vous ne pouvez pas la voir ; elle a été relue.",
  "p.task.refused_link_gone":
    "Ce lien avait déjà été retiré ; les liens ont été relus.",
  "p.task.refused_link_exists":
    "Ce lien existe déjà ; les liens ont été relus.",
  "p.task.refused_edit_invalid":
    "Le service n'a pas accepté la modification : vérifiez les champs. Une nouvelle date d'échéance doit aussi garder chaque lien « Précède » dans l'ordre.",
  "p.task.refused_link_invalid":
    "Le service n'a pas accepté le lien : l'autre tâche a peut-être disparu, un lien « Bloque » ou « Précède » ne peut pas former de boucle, et « Précède » exige que l'autre tâche soit due après celle-ci.",
  "p.me.notifications_link": "Mes notifications",
  "p.me.notifications_note":
    "— votre boîte de réception, ce dont vous êtes averti et où vont les copies.",
  "p.notifications.title_ppm": "Mes notifications — PPM",
  "p.notifications.heading": "Mes notifications",
  "p.notifications.back": "← Mon travail",
  "p.notifications.intro":
    "Tout ce qui est ici n'appartient qu'à vous : personne d'autre ne peut lire votre boîte de réception, vos choix ni vos destinations.",
  "p.notifications.loading": "Chargement…",
  "p.notifications.inbox_heading": "Boîte de réception",
  "p.notifications.unread_only": "N'afficher que les non lues",
  "p.notifications.unread_count_one": "{n} notification non lue.",
  "p.notifications.unread_count_other": "{n} notifications non lues.",
  "p.notifications.inbox_tz":
    "Les jours sont comptés dans votre fuseau horaire, {tz}.",
  "p.notifications.inbox_empty": "Votre boîte de réception est vide.",
  "p.notifications.inbox_unread_empty": "Rien de non lu.",
  "p.notifications.status_unread": "Non lue",
  "p.notifications.status_read": "Lue",
  "p.notifications.digest_count_one": "résumé quotidien de {n} notification",
  "p.notifications.digest_count_other": "résumé quotidien de {n} notifications",
  "p.notifications.open_subject": "Ouvrir le plan",
  "p.notifications.mark_read": "Marquer comme lue",
  "p.notifications.mark_read_named": "Marquer comme lue : {message}",
  "p.notifications.marked_read": "Marquée comme lue.",
  "p.notifications.could_not_load_inbox":
    "Impossible de charger votre boîte de réception.",
  "p.notifications.could_not_mark":
    "Impossible de marquer la notification comme lue.",
  "p.notifications.prefs_heading": "Ce dont vous êtes averti",
  "p.notifications.prefs_intro":
    "Activez ou désactivez chaque type de notification pour la boîte de réception et pour chaque type de destination. Une copie part vers un webhook, une messagerie instantanée ou un courriel seulement si vous avez ajouté ci-dessous une destination de ce type. Un résumé quotidien regroupe les notifications d'une journée sur un même sujet en une seule ligne de la boîte ; rien n'est retenu ni supprimé.",
  "p.notifications.pref_daily": "Résumé quotidien dans la boîte de réception",
  "p.notifications.pref_saved": "Votre choix a été enregistré.",
  "p.notifications.pref_reset_done":
    "Retour à la valeur par défaut : activées, au fil de l'eau.",
  "p.notifications.reset": "Revenir à la valeur par défaut",
  "p.notifications.reset_named": "Revenir à la valeur par défaut : {kind}",
  "p.notifications.could_not_load_prefs": "Impossible de charger vos choix.",
  "p.notifications.could_not_save_pref": "Impossible d'enregistrer le choix.",
  "p.notifications.sinks_heading": "Où vont les copies",
  "p.notifications.sinks_intro":
    "Une destination reçoit une copie de chacune de vos notifications, avec seulement le texte de la notification elle-même. L'adresse est secrète : une fois enregistrée, elle n'est plus affichée qu'en abrégé. Une destination lente ou en panne ne retarde jamais la boîte de réception ; un envoi qui a échoué est retenté encore quatre fois sur environ deux heures et demie, puis abandonné.",
  "p.notifications.no_sinks":
    "Vous n'avez aucune destination ; vos notifications vont seulement dans la boîte de réception.",
  "p.notifications.chat_named": "Messagerie {name}",
  "p.notifications.sink_sending": "Envoi actif",
  "p.notifications.sink_paused": "En pause",
  "p.notifications.show_deliveries": "Envois récents",
  "p.notifications.deliveries_named": "Envois récents : {destination}",
  "p.notifications.remove_sink": "Retirer",
  "p.notifications.remove_sink_named": "Retirer : {destination}",
  "p.notifications.sink_removed": "La destination a été retirée.",
  "p.notifications.deliveries_heading": "Envois récents vers {destination}",
  "p.notifications.deliveries_note":
    "La raison d'un envoi échoué n'est pas affichée ici, car elle peut citer l'adresse secrète.",
  "p.notifications.no_deliveries":
    "Rien n'a encore été envoyé à cette destination.",
  "p.notifications.attempts_one": "{n} tentative",
  "p.notifications.attempts_other": "{n} tentatives",
  "p.notifications.delivered_at": "envoyé le {date}",
  "p.notifications.next_attempt": "prochain essai le {date}",
  "p.notifications.close_deliveries": "Fermer les envois",
  "p.notifications.add_heading": "Ajouter une destination",
  "p.notifications.f_kind": "Type",
  "p.notifications.f_destination_email": "Adresse électronique",
  "p.notifications.f_destination_url":
    "Adresse web (https, vers un hôte public)",
  "p.notifications.f_chat_id":
    "Discussion Telegram (son numéro, ou @ et son nom)",
  "p.notifications.add_sink": "Ajouter la destination",
  "p.notifications.sink_added": "La destination a été ajoutée : {destination}.",
  "p.notifications.could_not_load_sinks":
    "Impossible de charger vos destinations.",
  "p.notifications.could_not_load_deliveries":
    "Impossible de charger les envois.",
  "p.notifications.could_not_save_sink": "Impossible d'ajouter la destination.",
  "p.notifications.could_not_remove_sink":
    "Impossible de retirer la destination.",
  "p.notifications.problem_too_many":
    "Vous avez déjà dix destinations, le maximum possible : retirez-en une d'abord.",
  "p.notifications.problem_destination_required":
    "Indiquez l'adresse à laquelle envoyer.",
  "p.notifications.problem_email_invalid":
    "Ce n'est pas une adresse électronique.",
  "p.notifications.problem_url_invalid": "Ce n'est pas une adresse web.",
  "p.notifications.problem_url_not_https":
    "L'adresse web doit commencer par https://.",
  "p.notifications.problem_url_credentials":
    "L'adresse web ne doit contenir ni nom d'utilisateur ni mot de passe.",
  "p.notifications.problem_url_private":
    "L'adresse web doit désigner un hôte public, pas un hôte local, privé ou interne.",
  "p.notifications.problem_url_too_long": "L'adresse web est trop longue.",
  "p.notifications.problem_chat_id_required":
    "Une destination Telegram a besoin de la discussion où elle publie.",
  "p.notifications.refused_forbidden":
    "Vous ne pouvez pas modifier ces réglages de notification.",
  "p.notifications.refused_note_gone":
    "Cette notification n'existe plus ou n'est pas la vôtre ; la boîte de réception a été relue.",
  "p.notifications.refused_sink_gone":
    "Cette destination avait déjà été retirée ; vos destinations ont été relues.",
  "p.notifications.refused_not_addressable":
    "Votre identité de connexion ne peut pas recevoir de notifications adressées à une personne ; il n'y a donc rien à afficher ici.",
  "p.notifications.refused_pref_invalid":
    "Le service n'a pas accepté ce choix ; vos choix ont été relus.",
  "p.notifications.refused_sink_invalid":
    "Le service n'a pas accepté la destination : ce doit être une adresse électronique ou une adresse web https vers un hôte public ; une destination Telegram a besoin de sa discussion ; et il peut y en avoir dix au plus.",
  "l.enum.notify_kind.approval_decided":
    "Une approbation que vous avez demandée a été tranchée",
  "l.enum.notify_kind.approval_requested":
    "On vous demande d'approuver quelque chose",
  "l.enum.notify_kind.auto_watch":
    "Suivre automatiquement ce sur quoi vous travaillez",
  "l.enum.notify_kind.automation": "Une règle vous avertit",
  "l.enum.notify_kind.comment": "Commentaires",
  "l.enum.notify_kind.escalation": "Remontées",
  "l.enum.notify_kind.exceptions_digest":
    "Le résumé hebdomadaire des exceptions",
  "l.enum.notify_kind.reschedule_proposed": "Une nouvelle date est proposée",
  "l.enum.notify_kind.review_invited": "Vous êtes invité à relire",
  "l.enum.notify_kind.review_submitted": "Une relecture a été remise",
  "l.enum.notify_kind.scheduled": "Un rappel programmé",
  "l.enum.notify_kind.status_update_due": "Un point d'avancement est attendu",
  "l.enum.notify_kind.task_assigned": "Une tâche vous a été confiée",
  "l.enum.notify_kind.watch_update": "Quelque chose que vous suivez a changé",
  "l.enum.notify_kind.process_todo": "Une étape de processus vous revient",
  "l.enum.notify_kind.process_notice":
    "Une étape de processus dont vous êtes averti",
  "l.enum.notify_kind.break_glass": "Un plan privé a été ouvert en urgence",
  "l.enum.notify_channel.in_app": "Dans la boîte de réception",
  "l.enum.notify_channel.webhook": "Par webhook signé",
  "l.enum.notify_channel.chat": "Par messagerie instantanée",
  "l.enum.notify_channel.email": "Par courriel",
  "l.enum.sink_kind.email": "Courriel",
  "l.enum.sink_kind.webhook": "Webhook signé",
  "l.enum.delivery_status.pending": "En attente d'envoi",
  "l.enum.delivery_status.sent": "Envoyé",
  "l.enum.delivery_status.failed": "Abandonné",
  "l.enum.delivery_status.skipped":
    "Non envoyé : la destination ou la notification n'existe plus",
  "l.link.kind_docs": "Documents",
  "l.link.kind_repo": "Dépôt de code",
  "l.link.kind_ticket": "Ticket",
  "l.link.kind_design": "Conception",
  "l.link.kind_chat": "Messagerie",
  "l.link.kind_tracker": "Outil de suivi du travail",
  "l.link.kind_other": "Autre",
  "l.link.system_other": "Un autre outil",
  "l.fw.method_waterfall": "Cycle en V",
  "l.fw.method_hybrid": "Hybride",
  "l.fw.method_other": "Autre",
  "l.fw.model_sponsor": "Le commanditaire décide",
  "l.fw.model_steering_committee": "Comité de pilotage",
  "l.fw.model_product_owner": "Responsable produit",
  "l.fw.model_delegated": "Délégué",
  "l.fw.model_consensus": "Consensus",
  "l.fw.model_other": "Autre",
  "p.frameworks.wow": "Méthodes de travail",
  "p.frameworks.wow_ppm": "Méthodes de travail — PPM",
  "p.frameworks.gov": "Cadre de gouvernance",
  "p.frameworks.gov_ppm": "Cadre de gouvernance — PPM",
  "p.frameworks.intro_wow":
    "Comment l'équipe travaille : méthode, durée d'itération, limite de travail en cours. Un plan en suit une, partagée ou propre ; sans elle, il hérite de celle du plan supérieur le plus proche.",
  "p.frameworks.intro_gov":
    "Comment le travail est gouverné : qui décide, à quelle fréquence il est revu et le circuit d'escalade suivi par les escalades. Un plan en suit un, partagé ou propre ; sans lui, il hérite de celui du plan supérieur le plus proche.",
  "p.frameworks.back": "Retour au plan",
  "p.frameworks.loading": "Chargement…",
  "p.frameworks.could_not_load": "Impossible de charger cet élément.",
  "p.frameworks.could_not": "Action impossible : {reason}",
  "p.frameworks.in_force": "En vigueur pour ce plan",
  "p.frameworks.source_own": "défini sur ce plan",
  "p.frameworks.source_inherited": "hérité d'un plan supérieur",
  "p.frameworks.source_none": "aucun",
  "p.frameworks.from_plan": "voir ce plan",
  "p.frameworks.scope_plan": "rédigé pour un seul plan",
  "p.frameworks.scope_org": "partagé par l'organisation",
  "p.frameworks.none": "Rien n'est en vigueur.",
  "p.frameworks.edit": "Modifier",
  "p.frameworks.unfollow": "Ne plus suivre (hériter à nouveau)",
  "p.frameworks.follow_heading": "En suivre un autre",
  "p.frameworks.no_others": "Aucun autre élément partagé à suivre.",
  "p.frameworks.choose": "Partagé ou existant",
  "p.frameworks.follow": "Suivre celui-ci",
  "p.frameworks.write_new": "En rédiger un nouveau pour ce plan",
  "p.frameworks.followed": "Désormais suivi.",
  "p.frameworks.unfollowed": "N'est plus suivi ; le plan hérite à nouveau.",
  "p.frameworks.saved": "Enregistré.",
  "p.frameworks.new_heading": "Un nouveau pour ce plan",
  "p.frameworks.edit_heading": "Modifier celui-ci",
  "p.frameworks.f_name": "Nom",
  "p.frameworks.f_description": "Description",
  "p.frameworks.save": "Enregistrer",
  "p.frameworks.cancel": "Annuler",
  "p.frameworks.methodology": "Méthode",
  "p.frameworks.cadence_days": "Durée d'itération (jours)",
  "p.frameworks.wip_limit": "Limite de travail en cours",
  "p.frameworks.ready": "Définition de prêt",
  "p.frameworks.done": "Définition de terminé",
  "p.frameworks.ceremonies": "Cérémonies",
  "p.frameworks.decision_model": "Modèle de décision",
  "p.frameworks.review_cadence": "Revue tous les (jours)",
  "p.frameworks.reporting_cadence": "Rapport à la hiérarchie tous les (jours)",
  "p.frameworks.path": "Circuit d'escalade",
  "p.frameworks.rights": "Droits de décision",
  "p.frameworks.forums": "Instances",
  "p.frameworks.days": "{n} jours",
  "p.frameworks.path_level": "Niveau",
  "p.frameworks.path_body": "Instance saisie",
  "p.frameworks.path_trigger": "Déclencheur",
  "p.frameworks.path_add": "Ajouter un niveau",
  "p.frameworks.path_remove": "Supprimer le niveau {n}",
  "p.frameworks.name_problem": "Donnez-lui un nom de 1 à 120 caractères.",
  "p.frameworks.number_problem":
    "Les jours sont des nombres entiers de 1 à 366, et la limite de 1 à 1000.",
  "p.frameworks.path_problem":
    "Chaque niveau d'escalade est un nombre entier distinct de 1 à 9, avec l'instance saisie et le déclencheur.",
  "p.frameworks.confirm_shared_one":
    "Cet élément est partagé : l'enregistrer le modifie aussi pour {n} autre plan.",
  "p.frameworks.confirm_shared_other":
    "Cet élément est partagé : l'enregistrer le modifie aussi pour {n} autres plans.",
  "p.frameworks.confirm_problem_one":
    "Cochez la case pour confirmer la modification pour {n} autre plan.",
  "p.frameworks.confirm_problem_other":
    "Cochez la case pour confirmer la modification pour {n} autres plans.",
  "p.frameworks.deleted": "{name} supprimé.",
  "p.frameworks.delete": "Supprimer {name} (personne ne le suit)",
  "p.frameworks.text_problem":
    "Une définition comporte au plus 10000 caractères.",
  "p.frameworks.practice_problem":
    "Chaque cérémonie ou instance doit avoir un nom et une fréquence, et il y en a au plus 20.",
  "p.frameworks.rights_problem":
    "Chaque droit de décision doit indiquer la décision et qui la prend, et il y en a au plus 20.",
  "p.frameworks.practice_name": "Nom",
  "p.frameworks.practice_cadence": "Fréquence",
  "p.frameworks.practice_purpose": "Objet",
  "p.frameworks.row_remove": "Supprimer la ligne {n}",
  "p.frameworks.ceremony_add": "Ajouter une cérémonie",
  "p.frameworks.forum_add": "Ajouter une instance",
  "p.frameworks.right_decision": "Décision",
  "p.frameworks.right_decides": "Qui décide",
  "p.frameworks.right_consulted": "Qui est consulté d'abord",
  "p.frameworks.right_add": "Ajouter un droit de décision",
  "p.links.links": "Liens",
  "p.links.links_ppm": "Liens — PPM",
  "p.links.intro":
    "Où se trouvent les documents et le travail de ce plan, y compris son tableau Jira, Trello, Asana, Linear ou Monday. L'adresse est enregistrée et affichée ; elle n'est jamais consultée.",
  "p.links.back": "Retour au plan",
  "p.links.loading": "Chargement…",
  "p.links.could_not_load": "Impossible de charger les liens.",
  "p.links.could_not": "Action impossible : {reason}",
  "p.links.none": "Aucun lien pour l'instant.",
  "p.links.opens_new": "s'ouvre dans un nouvel onglet",
  "p.links.remove": "Supprimer {label}",
  "p.links.added": "{label} ajouté.",
  "p.links.removed": "{label} supprimé.",
  "p.links.add_heading": "Ajouter un lien",
  "p.links.add": "Ajouter le lien",
  "p.links.f_label": "Libellé",
  "p.links.f_url": "Adresse (http ou https)",
  "p.links.f_kind": "Type",
  "p.links.f_system": "Outil externe",
  "p.links.f_id": "Son identifiant dans cet outil",
  "p.links.url_problem":
    "L'adresse doit commencer par http:// ou https:// et indiquer un hôte.",
  "p.links.label_problem": "Donnez-lui un libellé.",
  "p.links.system_problem": "Un lien d'outil de suivi doit indiquer son outil.",
  "p.links.system_kind_problem":
    "Seul un lien d'outil de suivi ou de ticket indique un outil.",
  "p.links.id_problem":
    "Un identifiant nécessite un outil et ne contient pas d'espaces.",
  "p.links.domain_problem": "Les liens de cet outil sont sur {domains}.",
  "p.escalations.intro":
    "Remontez un sujet quand le projet ne peut pas ou ne doit pas le régler lui-même. Il est soulevé à un niveau du circuit d'escalade défini dans le cadre de gouvernance du projet, pris en compte, remonté d'un niveau si personne ne répond, puis se termine résolu ou retiré. La personne désignée comme responsable et celle qui l'a soulevé en sont informées dans leur boîte de réception (aucun e-mail n'est envoyé).",
  "p.escalations.register_intro":
    "Toutes les escalades des plans que vous pouvez lire.",
  "p.escalations.none": "Aucune escalade pour l'instant.",
  "p.escalations.none_match": "Aucune escalade ne correspond.",
  "p.escalations.raise_heading": "Soulever une escalade",
  "p.escalations.f_title": "Titre",
  "p.escalations.f_reason": "Pourquoi",
  "p.escalations.f_dimension": "Quelle tolérance",
  "p.escalations.f_level": "Niveau (vide pour le premier niveau du circuit)",
  "p.escalations.f_to": "Escalader vers (vide pour suivre le circuit)",
  "p.escalations.f_owner": "Personne destinataire (person:… ou worker:…)",
  "p.escalations.f_respond": "Réponse attendue pour le",
  "p.escalations.f_description": "Quoi et pourquoi",
  "p.escalations.raise": "La soulever",
  "p.escalations.raised": "{title} soulevée.",
  "p.escalations.acknowledge": "Prendre en compte",
  "p.escalations.take_up": "La remonter d'un niveau",
  "p.escalations.take_up_note": "Motif de la remontée",
  "p.escalations.resolve": "Résoudre",
  "p.escalations.outcome": "Issue",
  "p.escalations.response": "Réponse",
  "p.escalations.withdraw": "Retirer",
  "p.escalations.done": "Terminé.",
  "p.escalations.could_not": "Cela n'a pas fonctionné : {reason}",
  "p.error.status_401": "Vous n'êtes pas connecté, ou votre session a expiré.",
  "p.error.status_403": "Vous n'êtes pas autorisé à faire cela.",
  "p.error.status_404": "Élément introuvable, ou vous ne pouvez pas le voir.",
  "p.error.status_409":
    "Conflit avec l'état actuel : quelqu'un d'autre l'a modifié ou tranché, ou c'est déjà fait.",
  "p.error.status_412":
    "L'élément a changé depuis son chargement : rechargez et réessayez.",
  "p.error.status_413": "C'est trop volumineux.",
  "p.error.status_422": "Une valeur saisie n'est pas acceptée.",
  "p.error.status_429": "Trop de requêtes : patientez un instant et réessayez.",
  "p.bridge.open_workforce":
    "Ouvrir {name} dans la planification des effectifs",
  "p.bridge.open_contacts": "Ouvrir {name} dans les contacts",
  "p.error.client": "La requête a été refusée.",
  "p.error.server": "Le service a rencontré un problème : réessayez.",
  "p.escalations.note_problem": "Saisissez au moins 10 caractères.",
  "p.escalations.show": "Afficher {title}",
  "p.escalations.hide": "Masquer {title}",
  "p.escalations.history": "Historique",
  "p.escalations.status": "Statut",
  "p.escalations.plan": "Plan",
  "p.escalations.level_col": "Niveau",
  "p.escalations.age": "Ancienneté",
  "p.escalations.respond_by": "Réponse attendue pour le",
  "p.escalations.filter": "Afficher",
  "p.escalations.all": "Toutes",
  "p.escalations.live": "Encore ouvertes",
  "p.escalations.overdue_count": "{n} en retard",
  "p.escalations.loading": "Chargement…",
  "p.escalations.could_not_load": "Impossible de charger les escalades",
  "p.escalations.back": "← Plan",
  "p.escalations.register_link": "Toutes les escalades",
  "p.tolerances.needs_escalation":
    "Ce point est hors tolérance et personne ne l'a escaladé.",
  "p.tolerances.raise_one": "Soulever une escalade",
  "p.tolerances.escalate_all": "Escalader toutes les tolérances rouges",
  "p.tolerances.escalate_all_intro":
    "Chaque dimension hors tolérance et pas encore escaladée reçoit une escalade au premier niveau du circuit d'escalade. Rien n'est escaladé automatiquement.",
  "p.tolerances.escalated_one": "{n} escalade soulevée.",
  "p.tolerances.escalated_other": "{n} escalades soulevées.",
  "p.tolerances.escalations_one": "{n} escalade ouverte",
  "p.tolerances.escalations_other": "{n} escalades ouvertes",
  "p.resources.rbs.by_role": "Par rôle",
  "p.resources.rbs.show_tasks": "Afficher les tâches ouvertes de {role}",
  "p.resources.rbs.hide_tasks": "Masquer les tâches ouvertes de {role}",
  "p.resources.rbs.no_open_tasks": "aucune tâche ouverte",
  "p.resources.rbs.tasks_in_plans_you_cannot_read":
    "{count} dans des plans que vous ne pouvez pas lire ne sont pas affichées.",
  "p.resources.rbs.only_the_first_500_tasks":
    "Seules les 500 premières tâches ont été lues : cette liste peut être incomplète.",
  "p.resources.rbs.could_not_load_tasks":
    "Impossible de charger les tâches : {reason}",
  "p.resources.rbs.by_skill": "Par compétence",
  "p.resources.rbs.demand_by_skill": "Demande par compétence",
  "p.resources.rbs.demand_fte": "Demande en FTE",
  "p.resources.rbs.no_roles_in_this_window": "Aucun rôle sur cette période.",
  "p.resources.rbs.no_skill_demand_in_this":
    "Aucune demande de compétence sur cette période.",
  "p.resources.rbs.not_known": "inconnu",
  "p.resources.rbs.people": "Personnes",
  "p.resources.rbs.resource_breakdown": "Structure des ressources",
  "p.resources.rbs.resource_breakdown_ppm": "Structure des ressources — PPM",
  "p.resources.rbs.role": "Rôle",
  "p.resources.rbs.roles_with_people_demand_and":
    "Rôles avec personnes, demande et offre",
  "p.resources.rbs.skill": "Compétence",
  "p.resources.rbs.supply_fte": "Offre en FTE",
  "p.resources.rbs.you_can_see_headcounts_only":
    "Vous ne voyez que les effectifs, pas les noms.",
  "p.resources.resources": "Ressources",
  "p.resources.resources_ppm": "Ressources — PPM",
  "p.resources.role": "Rôle",
  "p.resources.roles_and_totals_only": "rôles et totaux uniquement",
  "p.resources.roles_asked_for_with_no":
    "Rôles demandés sans personne pour l'instant",
  "p.resources.short_spare_balanced_supply_not":
    "▼ déficit · ▲ excédent · ● équilibré · ? offre inconnue. L'ombrage indique seulement l'ampleur de l'écart.",
  "p.resources.show": "Afficher",
  "p.resources.skill": "Compétence",
  "p.resources.skill_demand_only": "Compétence (demande uniquement)",
  "p.resources.supply_fte": "Offre en FTE",
  "p.resources.to": "Au",
  "p.resources.unclassified": "non classé",
  "p.resources.week": "Semaine",
  "p.resources.what_the_plans_ask_for":
    "Ce que demandent les plans face à ce que peuvent fournir les personnes de chaque rôle, en équivalents temps plein. Les réservations proposées sont tenues à part et jamais comptées.",
  "p.reviews.accept": "Accepter",
  "p.reviews.accepted": "acceptée",
  "p.reviews.actions": "Actions",
  "p.reviews.advance": "Faire avancer",
  "p.reviews.all": "toutes",
  "p.reviews.collaborative_review": "Revue collaborative",
  "p.reviews.consensus": "Consensus",
  "p.reviews.consensus_for_this_subject": "Consensus pour ce sujet",
  "p.reviews.decline": "Décliner",
  "p.reviews.declined": "déclinée",
  "p.reviews.delegate": "Déléguer",
  "p.reviews.delegate_a_specific_item_to":
    "Déléguez un élément précis à l'expert approprié. Seul un relecteur ayant accepté peut rendre un avis : une invitation sans réponse ne devient donc jamais un élément probant.",
  "p.reviews.due": "échéance",
  "p.reviews.e_g_clinical_safety": "p. ex. sécurité clinique",
  "p.reviews.expertise": "expertise",
  "p.reviews.expertise_2": "Expertise",
  "p.reviews.expired": "expirée",
  "p.reviews.external": "externe",
  "p.reviews.hold": "Suspendre",
  "p.reviews.idea": "idée",
  "p.reviews.internal": "interne",
  "p.reviews.invited": "invité",
  "p.reviews.no_review_invitations_yet":
    "Aucune invitation à une revue pour l'instant.",
  "p.reviews.no_score": "aucun score",
  "p.reviews.person_uuid": "person:<uuid>",
  "p.reviews.plan": "plan",
  "p.reviews.proposal": "proposition",
  "p.reviews.reject": "Rejeter",
  "p.reviews.reviewer": "relecteur",
  "p.reviews.reviewer_2": "Relecteur",
  "p.reviews.reviews_ppm": "Revues — PPM",
  "p.reviews.scope": "périmètre",
  "p.reviews.scope_2": "Périmètre",
  "p.reviews.status": "statut",
  "p.reviews.status_2": "Statut",
  "p.reviews.subject": "sujet",
  "p.reviews.subject_2": "Sujet",
  "p.reviews.subject_pid": "pid du sujet",
  "p.reviews.submitted": "soumis",
  "p.reviews.submitted_outstanding_declined_exter":
    "{submitted} soumis · {outstanding} en attente · {declined} déclinés · {external_submitted} externes",
  "p.reviews.uuid": "uuid",
  "p.reviews.verdict": "Avis",
  "p.reviews.withdraw": "Retirer",
  "p.risk.category": "Catégorie",
  "p.risk.concentration": "Concentration",
  "p.risk.escalated": "Escaladé",
  "p.risk.exposure": "Exposition",
  "p.risk.exposure_25_of_estate":
    "— exposition {exposure} (≥ 25 % de l'ensemble)",
  "p.risk.heatmap": "Carte de chaleur",
  "p.risk.item": "Élément",
  "p.risk.materialized": "Matérialisé",
  "p.risk.max_estate_max_single_risk":
    "— max. ensemble {v}, max. risque unique {v2}",
  "p.risk.no_open_risks": "Aucun risque ouvert.",
  "p.risk.open_exposure": "Exposition ouverte",
  "p.risk.open_risks_estate_open_exposure":
    "risques ouverts · exposition ouverte de l'ensemble",
  "p.risk.overdue_reviews": "Revues en retard",
  "p.risk.portfolio": "Portefeuille",
  "p.risk.posture_by_portfolio": "Situation par portefeuille",
  "p.risk.ppm": "{t} — PPM",
  "p.risk.probability_impact": "probabilité ↓ / impact →",
  "p.risk.review_due": "(revue prévue le {review_date})",
  "p.risk.risk": "Risque",
  "p.risk.top_risks": "Principaux risques",
  "p.risk.unassigned": "(non affecté)",
  "p.scenarios.alignment": "Alignement",
  "p.scenarios.cap": "Plafond",
  "p.scenarios.compare": "Comparer",
  "p.scenarios.currency": "Devise",
  "p.scenarios.delta_b_a": "Écart (b − a)",
  "p.scenarios.evaluation": "Évaluation",
  "p.scenarios.feasible": "Réalisable",
  "p.scenarios.no": "non",
  "p.scenarios.open_exposure": "Exposition ouverte",
  "p.scenarios.pick": "choisir…",
  "p.scenarios.planned": "Prévu {currency}",
  "p.scenarios.scenario": "Scénario",
  "p.scenarios.scenarios_ppm": "Scénarios — PPM",
  "p.scenarios.status": "Statut",
  "p.scenarios.work_item_pids_space_comma":
    "pid des éléments de travail (séparés par espaces ou virgules)",
  "p.scenarios.yes": "oui",
  "p.security.exposure": "Exposition",
  "p.security.item": "Élément",
  "p.security.late_stage_items_with_no":
    "Éléments en phase avancée sans risque de sécurité enregistré",
  "p.security.no_security_risks_recorded":
    "Aucun risque de sécurité enregistré.",
  "p.security.none_every_late_stage_item":
    "Aucun — chaque élément en phase avancée a un risque de sécurité enregistré.",
  "p.security.open_exposure": "{note} · exposition ouverte {open_exposure}",
  "p.security.owner": "Responsable",
  "p.security.ppm": "{t} — PPM",
  "p.security.risk": "Risque",
  "p.security.security_risk_register": "Registre des risques de sécurité",
  "p.security.status": "Statut",
  "p.signin.check_your_email_for_a":
    "Consultez vos e-mails : un lien de connexion vous a été envoyé.",
  "p.signin.could_not_send_the_sign":
    "Impossible d'envoyer le lien de connexion. Veuillez réessayer.",
  "p.signin.email": "E-mail",
  "p.signin.send_magic_link": "Envoyer le lien magique",
  "p.signin.sign_in": "Se connecter",
  "p.technology.as_of": "au {as_of} · {convention}",
  "p.technology.as_of_edges": "au {as_of} · {edges} liens",
  "p.technology.calendar_days": "{median_lead_days} jours calendaires",
  "p.technology.completed_before_timing_existed_unti":
    "· {undated_completions} terminés avant l'existence du chronométrage (non chronométrés)",
  "p.technology.cross_portfolio_edges": "Liens entre portefeuilles",
  "p.technology.delivery_flow": "Flux de livraison",
  "p.technology.dependency_risk": "Risque de dépendance",
  "p.technology.dependents": "Dépendants",
  "p.technology.edges_importing_risk_from_a":
    "Liens important un risque d'un prédécesseur rouge",
  "p.technology.exposure": "Exposition",
  "p.technology.item": "Élément",
  "p.technology.kind": "Type",
  "p.technology.median_lead": "Délai médian :",
  "p.technology.milestones_completed": "Étapes clés terminées",
  "p.technology.month": "Mois",
  "p.technology.most_depended_on_single_point":
    "Les plus sollicités (points de défaillance unique potentiels)",
  "p.technology.no_dependencies_recorded_yet":
    "Aucune dépendance enregistrée pour l'instant.",
  "p.technology.no_tech_debt_risks_recorded":
    "Aucun risque tech_debt enregistré.",
  "p.technology.no_tech_tags_recorded_yet":
    "Aucune étiquette `tech:*` enregistrée pour l'instant.",
  "p.technology.no_timed_completions": "aucun achèvement chronométré",
  "p.technology.no_timed_completions_in_the":
    "Aucun achèvement chronométré sur cette période.",
  "p.technology.none_all_edges_stay_within":
    "Aucun — tous les liens restent dans un même portefeuille.",
  "p.technology.none_no_red_predecessors": "Aucun — pas de prédécesseur rouge.",
  "p.technology.open_exposure": "{note} · exposition ouverte {open_exposure}",
  "p.technology.over_timed_completions":
    "sur {timed_completions} achèvements chronométrés",
  "p.technology.owner": "Responsable",
  "p.technology.ppm": "{t} — PPM",
  "p.technology.predecessor": "Prédécesseur",
  "p.technology.predecessor_red": "Prédécesseur (rouge)",
  "p.technology.ring_votes": "({ring_votes} votes d'anneau)",
  "p.technology.risk": "Risque",
  "p.technology.status": "Statut (",
  "p.technology.status_2": "Statut",
  "p.technology.successor": "Successeur",
  "p.technology.successor_at_risk": "Successeur à risque",
  "p.technology.technical_debt_register": "Registre de la dette technique",
  "p.technology.technology_radar": "Radar technologique",
  "p.verify.request_a_new_link": "Demander un nouveau lien",
  "p.verify.sign_in_link": "Lien de connexion",
  "p.milestones.milestone_by_due_date_one":
    "{length} étape clé, par date d'échéance",
  "p.milestones.milestone_by_due_date_other":
    "{length} étapes clés, par date d'échéance",
  "p.okrs.objective_least_progressed_first_one":
    "{length} objectif, le moins avancé en premier",
  "p.okrs.objective_least_progressed_first_other":
    "{length} objectifs, les moins avancés en premier",
  "p.plans.pid.board.blocked_calendar_day_one":
    "bloqué {blocked_days} jour calendaire",
  "p.plans.pid.board.blocked_calendar_day_other":
    "bloqué {blocked_days} jours calendaires",
  "p.plans.pid.calendar.holiday_one": "{length} jour férié : {join}",
  "p.plans.pid.calendar.holiday_other": "{length} jours fériés : {join}",
  "p.plans.pid.costs.for_across_one":
    "{money} pour {hoursText} sur {entries} saisie.",
  "p.plans.pid.costs.for_across_other":
    "{money} pour {hoursText} sur {entries} saisies.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_one":
    "{plans_without_figures} plan ci-dessous n'a pas encore de chiffres (pas de référence ou rien à mesurer) et n'est pas estimé au jugé.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_other":
    "{plans_without_figures} plans ci-dessous n'ont pas encore de chiffres (pas de référence ou rien à mesurer) et ne sont pas estimés au jugé.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_one":
    "terminés sans jamais revenir en arrière ({rework_count} retour en arrière). À lire avec le débit : un débit en hausse et un rendement en baisse signifient du travail renvoyé vers lui-même.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_other":
    "terminés sans jamais revenir en arrière ({rework_count} retours en arrière). À lire avec le débit : un débit en hausse et un rendement en baisse signifient du travail renvoyé vers lui-même.",
  "p.plans.pid.flow.of_items_finish_within_this_one":
    "{percent} des éléments se terminent dans ce délai, sur {sample} élément terminé",
  "p.plans.pid.flow.of_items_finish_within_this_other":
    "{percent} des éléments se terminent dans ce délai, sur {sample} éléments terminés",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_one":
    "{length} risque touche à la fois les délais et le coût : il survient une fois et agit sur les deux.",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_other":
    "{length} risques touchent à la fois les délais et le coût : chacun survient une fois et agit sur les deux.",
  "p.plans.pid.leveling.apply_move_one": "Appliquer {length} déplacement",
  "p.plans.pid.leveling.apply_move_other": "Appliquer {length} déplacements",
  "p.plans.pid.leveling.business_day_one": "{delay_days} jour ouvré",
  "p.plans.pid.leveling.business_day_other": "{delay_days} jours ouvrés",
  "p.plans.pid.leveling.other_task_would_be_pushed_one":
    "{length} autre tâche serait décalée en conséquence",
  "p.plans.pid.leveling.other_task_would_be_pushed_other":
    "{length} autres tâches seraient décalées en conséquence",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_one":
    "{hidden} liste de contrôle concernant des personnes ne vous est pas affichée.",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_other":
    "{hidden} listes de contrôle concernant des personnes ne vous sont pas affichées.",
  "p.plans.pid.schedule_health.preset_thresholds_task_one":
    "· préréglage {preset} (seuils : {source}) · {tasks} tâche",
  "p.plans.pid.schedule_health.preset_thresholds_task_other":
    "· préréglage {preset} (seuils : {source}) · {tasks} tâches",
  "p.plans.pid.schedule_health.task_one": "{length} tâche",
  "p.plans.pid.schedule_health.task_other": "{length} tâches",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_one":
    "{title} : {probability_percent} % de risque d'ajouter {impact_days} jour ouvré",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_other":
    "{title} : {probability_percent} % de risque d'ajouter {impact_days} jours ouvrés",
  "p.plans.pid.wbs.move_up": "Monter {title}",
  "p.plans.pid.wbs.move_down": "Descendre {title}",
  "p.plans.pid.wbs.moved": "{title} déplacé.",
  "p.plans.pid.wbs.truncated":
    "Affichage de {shown} tâches sur {total} : ce plan est trop volumineux pour être listé en entier, le plan ci-dessous est donc incomplet.",
  "p.plans.pid.critical_path.too_large":
    "Ce plan est trop volumineux pour être planifié en entier en une seule réponse.",
  "p.plans.pid.critical_path.show_critical_only":
    "N'afficher que la partie critique",
  "p.plans.pid.critical_path.critical_only_notice":
    "Seules les tâches critiques et quasi critiques sont affichées ; le reste du plan est omis.",
  "p.plans.pid.earned_value.names_truncated":
    "Ce plan est très volumineux : les chiffres couvrent toutes les tâches, mais certains noms de tâches ci-dessous peuvent manquer.",
  "p.plans.pid.wbs.could_not_move":
    "Impossible de déplacer la tâche : {reason}",
  "p.plans.pid.wbs.sub_task_one": "({children} sous-tâche)",
  "p.plans.pid.wbs.sub_task_other": "({children} sous-tâches)",
  "p.plans.pid.wbs.links": "Liens",
  "p.plans.pid.wbs.link_one": "· {links} lien",
  "p.plans.pid.wbs.link_other": "· {links} liens",
  "p.common.on": "activé",
  "p.common.off": "désactivé",
  "p.common.pre_gate": "avant jalon",
  "p.common.role_word": "rôle",
  "p.common.person_word": "personne",
  "c.NetworkDiagram.task_fallback": "tâche",
  "c.NetworkDiagram.task_summary":
    "{title}, du {early_start} au {early_finish}, {total_float} jours ouvrés de marge",
  "c.NetworkDiagram.task_summary_critical":
    "{title}, du {early_start} au {early_finish}, {total_float} jours ouvrés de marge, sur le chemin critique",
  "c.NetworkDiagram.lag_one": ", décalage {lag_days} jour ouvré",
  "c.NetworkDiagram.lag_other": ", décalage {lag_days} jours ouvrés",
  "c.KpiCard.direction_higher": "plus élevé",
  "c.KpiCard.direction_lower": "plus bas",
  "c.CalendarMonth.legend":
    "– jour non ouvré   ✕ jour férié   A absence enregistrée",
  "c.CalendarMonth.legend_editable":
    " · sélectionnez un jour ouvré pour marquer ou effacer un jour férié",
  "c.CostRiskView.run_summary":
    "graine {seed} · {iterations} itérations · {distribution}",
  "c.CostRiskView.dist_pert": "PERT",
  "c.CostRiskView.dist_triangular": "Triangulaire",
  "c.CostRiskView.risks_without_cost_one":
    "{count} risque du registre n'a aucun champ de coût et n'est pas pris en compte ici.",
  "c.CostRiskView.risks_without_cost_other":
    "{count} risques du registre n'ont aucun champ de coût et ne sont pas pris en compte ici.",
  "c.CumulativeFlow.hide_the_numbers": "Masquer les chiffres",
  "c.CumulativeFlow.show_the_numbers": "Afficher les chiffres",
  "c.CostRiskView.scope_all_one": "Le plan que vous pouvez lire ({plans})",
  "c.CostRiskView.scope_all_other": "Les {plans} plans que vous pouvez lire",
  "c.CostRiskView.scope_this_plan": "Ce plan",
  "c.MilestoneCard.reason": "Motif",
  "c.MilestoneCard.reason_required_open":
    " (obligatoire : des tâches dont il dépend sont encore ouvertes)",
  "c.MilestoneCard.reason_optional": " (facultatif)",
  "c.VarianceTable.matched_of_baselined":
    "{matched} des {baseline_tasks} tâches de la référence figurent encore dans le plan :",
  "c.VarianceTable.slipped_pulled_on":
    "{slipped} en retard, {pulled_in} avancées, {on_baseline} conformes à la référence{unmeasurable}.",
  "c.VarianceTable.cannot_be_measured": ", {unmeasurable} non mesurables",
  "c.VarianceTable.added_removed":
    "{added} ajoutées et {removed} supprimées depuis.",
  "c.VarianceTable.finish_later_one":
    "La fin du plan est {days} jour ouvré plus tard que dans la référence{note}.",
  "c.VarianceTable.finish_later_other":
    "La fin du plan est {days} jours ouvrés plus tard que dans la référence{note}.",
  "c.VarianceTable.finish_earlier_one":
    "La fin du plan est {days} jour ouvré plus tôt que dans la référence{note}.",
  "c.VarianceTable.finish_earlier_other":
    "La fin du plan est {days} jours ouvrés plus tôt que dans la référence{note}.",
  "c.VarianceTable.check_removed":
    " (vérifiez les tâches supprimées : supprimer du travail n'est pas une amélioration)",
  "p.plans.pid.critical_path.finishes": "Se termine le {date}.",
  "p.plans.pid.critical_path.nothing_scheduled": "Rien n'est encore planifié.",
  "p.plans.pid.critical_path.calendar_label": "Calendrier : {calendar}.",
  "p.plans.pid.critical_path.calendar_plan": "celui de ce plan",
  "p.plans.pid.critical_path.calendar_default":
    "par défaut (tous les jours sont ouvrés)",
  "p.plans.pid.critical_path.more_not_shown": " (d'autres non affichés)",
  "p.plans.pid.critical_path.pushed_later":
    "Ses prédécesseurs le repoussent de {days} jours ouvrés par rapport au prévu.",
  "p.plans.pid.earned_value.manual_percent_banner":
    "Une partie de ce chiffre repose sur un pourcentage saisi manuellement, et non sur du travail terminé : {tasks}.",
  "p.plans.pid.earned_value.a_task": "une tâche",
  "p.plans.pid.earned_value.overrun": " (dépassement)",
  "p.plans.pid.earned_value.target_budget": "le budget",
  "p.plans.pid.earned_value.target_forecast": "la prévision",
  "p.plans.pid.earned_value.lights_use":
    "Les voyants utilisent {which} : {values} (points de base).",
  "p.plans.pid.earned_value.thresholds_default": "les seuils par défaut",
  "p.plans.pid.earned_value.thresholds_deployment":
    "les seuils de ce déploiement",
  "p.plans.pid.earned_value.rollup_money":
    "Budget {bac} ; prévu {pv} ; acquis {ev} ; dépensé {ac}{unknown}.",
  "p.plans.pid.earned_value.cost_unknown_for":
    " (coût inconnu pour {plans} plans)",
  "p.plans.pid.earned_value.cpi_value": "CPI {value}",
  "p.plans.pid.earned_value.cpi_not_stated":
    "non indiqué : le coût est inconnu pour certains plans",
  "p.plans.pid.earned_value.weight_points": ", {weight} points",
  "p.plans.pid.critical_path.moved_line":
    "{title} : {moved}{finish}{start} jours ouvrés",
  "p.plans.pid.critical_path.moved_both": "début et fin",
  "p.plans.pid.critical_path.moved_start": "début",
  "p.plans.pid.critical_path.moved_finish": "fin",
  "p.plans.pid.critical_path.moved_finish_by": " fin {days}",
  "p.plans.pid.critical_path.moved_start_by": " début {days}",
  "p.plans.pid.flow.overdue_one":
    "{count} élément a déjà dépassé l'attente. Un élément vieillissant est la seule chose de cette page dont vous pouvez encore changer l'issue.",
  "p.plans.pid.flow.overdue_other":
    "{count} éléments ont déjà dépassé l'attente. Un élément vieillissant est la seule chose de cette page dont vous pouvez encore changer l'issue.",
  "p.plans.pid.flow.classified_with": "Classé avec {map} :",
  "p.plans.pid.flow.map_override": "une surcharge du déploiement",
  "p.plans.pid.flow.map_default": "la correspondance par défaut",
  "p.plans.pid.governance.proposed_not_counted":
    "proposé{requested} — non compté",
  "p.plans.pid.governance.requested": ", demandé",
  "p.plans.pid.joint_risk.run_summary":
    "{currency} · graine {seed} · {iterations} itérations · {distribution} · calendrier {calendar}",
  "p.plans.pid.joint_risk.dist_pert": "PERT",
  "p.plans.pid.joint_risk.dist_triangular": "Triangulaire",
  "p.plans.pid.leveling.applied_one":
    "{ok} déplacement appliqué sur {requested}{refused}.",
  "p.plans.pid.leveling.applied_other":
    "{ok} déplacements appliqués sur {requested}{refused}.",
  "p.plans.pid.leveling.applied_refused": " ; {refused} refusés",
  "p.plans.pid.leveling.proposal_summary":
    "Calendrier : {calendar} · charge {load} % par tâche · les tâches critiques {critical} être déplacées",
  "p.plans.pid.leveling.may": "peuvent",
  "p.plans.pid.leveling.may_not": "ne peuvent pas",
  "p.plans.pid.onboarding.due": ", échéance le {date}",
  "p.plans.pid.schedule_risk.risks_without_schedule_one":
    "{count} risque du registre n'a aucun champ de calendrier et n'est pas pris en compte ici.",
  "p.plans.pid.schedule_risk.risks_without_schedule_other":
    "{count} risques du registre n'ont aucun champ de calendrier et ne sont pas pris en compte ici.",
  "p.resources.from_date": "à partir du {date}",
  "p.resources.to_date": "jusqu'au {date}",
  "p.kpis.none": "Aucun KPI pour l'instant.",
  "p.kpis.none_with_status": "Aucun KPI avec ce statut pour l'instant.",
  "p.milestones.none":
    "Aucun jalon pour l'instant. Ajoutez-en depuis la page de gouvernance d'un plan.",
  "p.milestones.none_with_status":
    "Aucun jalon avec ce statut pour l'instant. Ajoutez-en depuis la page de gouvernance d'un plan.",
  "p.okrs.none": "Aucun objectif pour l'instant.",
  "p.okrs.none_for_period": "Aucun objectif pour {period} pour l'instant.",
  "p.me.calendar.own_set": "Votre calendrier personnel est défini.",
  "p.me.calendar.plan_applies":
    "Aucun calendrier personnel : le calendrier du plan s'applique à vous.",
  "p.me.calendar.leave_counts":
    "Les congés enregistrés pour vous comptent déjà comme jours non travaillés.",
  "p.documents.pid.page_title": "{title} — PPM",
  "p.documents.pid.document_fallback": "Document",
  "p.plans.pid.page_title": "{name} — Main X",
  "p.plans.pid.plan_fallback": "Plan",
  "p.plans.pid.custom_scheme": "Personnalisé({name})",
  "p.plans.pid.calendar.own_calendar": "Ce plan a son propre calendrier.",
  "p.plans.pid.calendar.no_calendar":
    "Aucun calendrier défini : tous les jours sont ouvrés.",
  "p.plans.pid.calendar.each_person_can_set":
    "Chaque personne peut aussi définir le sien (",
  "p.reviews.consensus_line":
    "score moyen : {mean} · majorité : {majority} · {state}",
  "p.reviews.no_majority": "aucune majorité",
  "p.reviews.everyone_answered": "tout le monde a répondu",
  "p.reviews.still_waiting": "en attente",
  "l.announcement_view.just_now": "à l'instant",
  "l.announcement_view.min_ago": "il y a {minutes} min",
  "l.announcement_view.h_ago": "il y a {hours} h",
  "l.announcement_view.calendar_days_ago": "il y a {days} jours calendaires",
  "l.announcement_view.scheduled": "programmée",
  "l.announcement_view.expired": "Expirée",
  "l.api_ppm.task": "tâche {uid}",
  "l.api_ppm.the_project_calendar_comes_across":
    " Le calendrier du projet est repris.",
  "l.api_tba.calendar_days": "{toFixed} jours calendaires",
  "l.api_tba.work_in_progress_is_growing": "Le travail en cours augmente",
  "l.api_tba.steady_state": "Régime stable",
  "l.api_tba.queue_draining": "La file se vide",
  "l.api_tba.not_enough_data": "Données insuffisantes",
  "l.capacity_view.committed_against_a_capacity_as":
    "{committed} % engagé pour une capacité descendant jusqu'à {capacity} %",
  "l.capacity_view.committed": "{committed} % engagé",
  "l.capacity_view.to": " au {to}",
  "l.capacity_view.over_capacity_from_asked_available":
    "au-delà de la capacité du {from}{v} ({demand} % demandé, {capacity} % disponible)",
  "l.capacity_view.capacity_undeclared":
    "aucune capacité déclarée certains jours travaillés : ni au-delà ni dans la capacité",
  "l.capacity_view.over_capacity_from_asked_undeclared":
    "au-delà de la capacité à partir du {from}{v} ({demand} demandés, plus que ne peut l'être toute capacité)",
  "l.capacity_view.asked_undeclared":
    "{from}{v} : {demand} demandés, plus que ne peut l'être toute capacité",
  "l.capacity_view.over_capacity": "au-delà de la capacité",
  "l.capacity_view.committing_it_would_put_them":
    " ; l'engager dépasserait sa capacité",
  "l.capacity_view.proposed_not_counted": "{percent} % proposé, non compté{v}",
  "l.capacity_view.asked_available":
    "{from}{v} : {demand} % demandé, {capacity} % disponible",
  "l.ceo_view.no_portfolios_to_report_on":
    "Aucun portefeuille à présenter pour l'instant.",
  "l.ceo_view.portfolios_exist_but_hold_no":
    "Des portefeuilles existent mais ne contiennent encore aucun plan.",
  "l.ceo_view.of_plans_are_red_amber":
    "{red} plans sur {n} sont rouges, {amber} ambre, {green} verts.",
  "c.CalendarMonth.not_a_business_day": "jour non ouvré",
  "c.CalendarMonth.business_day": "jour ouvré",
  "c.KpiCard.that_did_not_work": "Cela n'a pas fonctionné",
  "c.KpiCard.could_not_load_the_history": "Impossible de charger l'historique",
  "c.KpiCard.delete_the_kpi_its_readings":
    "Supprimer le KPI « {name} » ? Ses relevés sont conservés.",
  "c.LineRange.could_not_save": "Impossible d'enregistrer",
  "c.MilestoneCard.that_did_not_work": "Cela n'a pas fonctionné",
  "c.MilestoneCard.could_not_load_the_plan":
    "Impossible de charger les tâches du plan",
  "c.MilestoneCard.could_not_load_the_history":
    "Impossible de charger l'historique",
  "c.ObjectiveCard.that_did_not_work": "Cela n'a pas fonctionné",
  "c.ObjectiveCard.delete_the_objective_and_its":
    "Supprimer l'objectif « {title} » et ses résultats clés ?",
  "c.ObjectiveCard.delete_the_key_result":
    "Supprimer le résultat clé « {title} » ?",
  "c.RaciMatrix.accountable_moved_from_to_a":
    "Le rôle Approbateur est passé de {shortRef} à {shortRef2} : une tâche n'en a qu'un.",
  "c.RiskInputs.could_not_load_the_plan":
    "Impossible de charger les tâches du plan",
  "c.RiskInputs.could_not_save": "Impossible d'enregistrer",
  "c.RiskInputs.chance": "{probability_percent} % de probabilité",
  "c.RiskInputs.adds_business_days_to_a":
    "ajoute {schedule_impact_days} jours ouvrés à une tâche",
  "c.RiskInputs.adds": "ajoute {toFixed} {cost_currency}",
  "c.RiskInputs.not_used_in_the_simulations":
    "Non utilisé dans les simulations",
  "l.cpm_view.cannot_meet_its_constraint_or":
    "Ne peut pas respecter sa contrainte ou son échéance : {plural} de retard",
  "l.cpm_view.on_the_critical_path_any":
    "Sur le chemin critique : tout retard déplace la fin",
  "l.cpm_view.of_float_near_critical": "{d} de marge (quasi critique)",
  "l.cpm_view.of_float": "{d} de marge",
  "l.cpm_view.baselined_to": ". Référence du {start} au {finish}",
  "l.cpm_view.to": "{v} : du {early_start} au {early_finish}. {floatLabel}{v2}",
  "l.cpm_view.has_no_duration_yet": "n'a pas encore de durée",
  "l.cpm_view.has_no_start_date_and":
    "n'a pas de date de début et rien de ce qui la précède n'est planifié",
  "l.cpm_view.has_a_date_outside_the":
    "a une date hors de la plage prise en charge",
  "l.cpm_view.slipped": "En retard",
  "l.cpm_view.finishes_later_than_baselined":
    "se termine {d} plus tard que dans la référence",
  "l.cpm_view.pulled_in": "Avancée",
  "l.cpm_view.finishes_earlier_than_baselined":
    "se termine {d} plus tôt que dans la référence",
  "l.cpm_view.on_baseline": "Conforme à la référence",
  "l.cpm_view.finishes_when_it_was_baselined":
    "se termine à la date prévue dans la référence",
  "l.cpm_view.added": "Ajoutée",
  "l.cpm_view.not_in_the_baseline_so":
    "absente de la référence, donc sans retard",
  "l.cpm_view.removed": "Supprimée",
  "l.cpm_view.was_in_the_baseline_and":
    "figurait dans la référence et a disparu ; ses dates de référence sont conservées",
  "l.cpm_view.cannot_say": "Impossible à dire",
  "l.cpm_view.it_had_no_schedule_when":
    "elle n'avait pas de calendrier lors de la référence",
  "l.cpm_view.it_has_no_schedule_now":
    "elle n'a pas de calendrier actuellement",
  "l.document_view.version": "Version {version} — {v}",
  "l.document_view.since": "{base} depuis le {effective_from}",
  "l.document_view.by_version": " par la version {version}",
  "l.document_view.on": "{base} le {superseded_on}{v}",
  "l.document_view.nothing_in_force_yet": "Rien en vigueur pour l'instant",
  "l.document_view.version_in_force": "Version {in_force_today} en vigueur",
  "l.ev_view.nothing_was_planned_to_be":
    "rien n'était encore prévu comme terminé",
  "l.ev_view.no_cost_has_been_recorded": "aucun coût n'a encore été enregistré",
  "l.ev_view.the_baseline_plans_no_value":
    "la référence ne prévoit aucune valeur",
  "l.ev_view.not_available": "non disponible",
  "l.ev_view.schedule_no_ratio": "Délais : aucun ratio, {absentText}.",
  "l.ev_view.ahead_of_schedule": "En avance",
  "l.ev_view.behind_schedule": "En retard",
  "l.ev_view.on_schedule": "Dans les délais",
  "l.ev_view.spi": "{word} : SPI {ratioText}, {gap}.",
  "l.ev_view.cost_no_ratio": "Coût : aucun ratio, {absentText}.",
  "l.ev_view.under_budget": "Sous le budget",
  "l.ev_view.over_budget": "Au-dessus du budget",
  "l.ev_view.on_budget": "Dans le budget",
  "l.ev_view.earned_schedule_no_ratio":
    "Délai acquis : aucun ratio, {absentText}.",
  "l.ev_view.by_earned_schedule_the_work":
    "Selon le délai acquis, le travail est {word} : il a acquis en {days} ce que le plan prévoyait après {days2} (SPI(t) {ratioText}).",
  "l.ev_view.finished_work_counts_for_its":
    "le travail terminé compte pour ses points d'histoire",
  "l.ev_view.only_finished_work_counts": "seul le travail terminé compte",
  "l.ev_view.by_story_points": "par points d'histoire",
  "l.ev_view.counted_by_a_person_entered":
    "{length} comptées selon un pourcentage saisi manuellement",
  "l.ev_view.with_no_points_carry_no": "{length} sans points n'ont aucun poids",
  "l.ev_view.no_actual_cost_is_recorded": "aucun coût réel n'est enregistré",
  "l.ev_view.the_ratio_it_divides_by":
    "le ratio diviseur n'est pas encore défini",
  "l.ev_view.no_estimate_to_complete_has":
    "aucune estimation du reste à faire n'a été saisie",
  "l.ev_view.the_budget_is_already_spent": "le budget est déjà dépensé",
  "l.ev_view.no_cost_remains": "aucun coût restant",
  "l.ev_view.against_the_money_is_already":
    "Par rapport à {target} : l'argent est déjà dépensé, aucune efficacité ne permet donc de réaliser le travail restant dans cette enveloppe.",
  "l.ev_view.against": "Par rapport à {target} : {amountAbsentText}.",
  "l.ev_view.against_the_remaining_work_must":
    "Par rapport à {target} : le travail restant doit être réalisé avec une efficacité {ratioText} fois supérieure à celle du travail déjà fait (TCPI {ratioText2}), ce qui exige une meilleure efficacité.",
  "l.ev_view.against_the_remaining_work_can":
    "Par rapport à {target} : le travail restant peut être réalisé à {ratioText} de l'efficacité de coût du travail déjà fait (TCPI {ratioText2}).",
  "l.ev_view.earned": "{date} : acquis {ev_minor}",
  "l.ev_view.spent": "{date} : dépensé {ac_minor}",
  "l.import_view.would_create": "Créerait",
  "l.import_view.created": "Créé",
  "l.import_view.and": "{verb} {plural} et {upd} {tasks_updated}{tail}.",
  "l.import_view.preview_the_import": "Prévisualiser l'importation",
  "l.import_view.import": "Importer {plural}",
  "l.kpi_view.target_or_more_red_at":
    "Cible {target_value} ou plus ; rouge à {red_value} ou moins{u}",
  "l.kpi_view.target_or_less_red_at":
    "Cible {target_value} ou moins ; rouge à {red_value} ou plus{u}",
  "l.kpi_view.no_reading_within_twice_its":
    "Aucun relevé dans le double de sa cadence {cadence}",
  "l.kpi_view.no_reading_yet": "aucun relevé pour l'instant",
  "l.milestone_view.there_is_no_forecast": "Il n'y a aucune prévision.",
  "l.milestone_view.some_open_tasks_cannot_be":
    " (certaines tâches ouvertes ne peuvent pas être placées, cette date peut donc être trop tôt)",
  "l.milestone_view.forecast_to_finish_after_it":
    "Fin prévue le {forecast}, {businessDays} après son échéance ({due}){partial}.",
  "l.milestone_view.forecast_to_finish_on_the":
    "Fin prévue à l'échéance, le {due}{partial}.",
  "l.milestone_view.forecast_to_finish_before_it":
    "Fin prévue le {forecast}, {businessDays} avant son échéance ({due}){partial}.",
  "l.milestone_view.of_them_later": ", dont {later_moves} plus tard",
  "l.milestone_view.back_on_its_original_date": "revenu à sa date d'origine",
  "l.milestone_view.later_than_the_original":
    "{calendarDays} plus tard que l'échéance d'origine du {original_due}",
  "l.milestone_view.earlier_than_the_original":
    "{calendarDays} plus tôt que l'échéance d'origine du {original_due}",
  "l.milestone_view.originally_due_now":
    "Échéance d'origine le {original_due} ; {times}{later} ; désormais {net}.",
  "l.milestone_view.on": " le {achieved_on}",
  "l.milestone_view.achieved_every_task_it_waits":
    "Atteint{on} : toutes les tâches dont il dépend sont terminées.",
  "l.milestone_view.reason": " Motif : {done_reason}",
  "l.milestone_view.marked_done": "Marqué comme fait{on}.{why}",
  "l.milestone_view.due": "{name} : {toLowerCase}, échéance le {due}{state}",
  "l.my_work_view.this_is_already_late_for":
    "Cette tâche est déjà en retard pour le plan ({days} de retard).",
  "l.my_work_view.on_the_critical_path": "Sur le chemin critique",
  "l.my_work_view.of_float": "{days} de marge",
  "l.okr_view.not_measured": "Non mesuré",
  "l.okr_view.not_measurable": "Non mesurable ({reason}).",
  "l.okr_view.of": "{current} sur {target}{u}",
  "l.okr_view.confidence_not_part_of_the":
    "Confiance {confidence} % (n'entre pas dans le score)",
  "l.onboarding_view.give_the_date_it_takes":
    "Indiquez la date d'entrée en vigueur.",
  "l.onboarding_view.give_the_person_as_person":
    "Indiquez la personne sous la forme person:… (ou worker:…).",
  "l.onboarding_view.say_what_access_in_words":
    "Décrivez l'accès en toutes lettres.",
  "l.onboarding_view.choose_granted_or_revoked":
    "Choisissez accordé ou révoqué.",
  "l.onboarding_view.give_who_is_handing_over":
    "Indiquez qui transmet, sous la forme person:…",
  "l.onboarding_view.give_who_takes_over_as":
    "Indiquez qui reprend, sous la forme person:… (ou laissez vide pour libérer le travail).",
  "l.onboarding_view.hand_over_to_someone_else":
    "Transmettre à quelqu'un d'autre.",
  "l.onboarding_view.no_items": "Aucun élément",
  "l.onboarding_view.of_done": "{done} sur {total} faits",
  "l.onboarding_view.skipped": "{skipped} ignorés",
  "l.onboarding_view.overdue": "{overdue} en retard",
  "l.onboarding_view.skipped_2": "Ignoré",
  "l.onboarding_view.overdue_2": "En retard",
  "l.onboarding_view.nothing_is_left_open_this":
    "Plus rien n'est ouvert. Ce plan est prêt à être clôturé.",
  "l.onboarding_view.not_ready_to_close": "Pas prêt à être clôturé : {join}.",
  "l.onboarding_view.this_person_holds_nothing_live":
    "Cette personne ne détient rien d'actif sur ce plan.",
  "l.raci_view.nobody_is_accountable": "Personne n'est approbateur",
  "l.raci_view.nobody_is_responsible": "Personne n'est réalisateur",
  "l.raci_view.more_than_one_person_is": "Plus d'une personne est approbatrice",
  "l.raci_view.has_no_role": "{task} : {who} n'a aucun rôle",
  "l.raci_view.and": " et ",
  "l.raci_view.is": "{task} : {who} est {join}",
  "l.rbs.a_booking_that_needs_several":
    "Une réservation qui requiert plusieurs compétences compte entièrement pour chacune ; les compétences se chevauchent donc et ne s'additionnent pas. L'offre et les personnes ne sont pas connues par compétence.",
  "l.rbs.named": "{length} nommées",
  "l.rbs.supply_not_known": "offre inconnue",
  "l.rbs.supply_fte": "offre {supply_fte} FTE",
  "l.rbs.demand_fte": "{role} : {people}, demande {demand_fte} FTE, {supply}",
  "l.scheduling_view.pass": "Réussi",
  "l.scheduling_view.fail": "Échoué",
  "l.scheduling_view.not_computable": "Non calculable",
  "l.scheduling_view.not_enough_data": "Données insuffisantes.",
  "l.scheduling_view.not_enough_data_2": "Données insuffisantes : {reason}",
  "l.scheduling_view.must_pass_a_business_day":
    "Doit réussir : un retard de {v} jours ouvrés doit déplacer la fin",
  "l.scheduling_view.at_least_business_days": "Au moins {limit} jours ouvrés",
  "l.scheduling_view.at_most": "au plus",
  "l.scheduling_view.at_least": "au moins",
  "l.scheduling_view.no_figure": "Aucun chiffre",
  "l.scheduling_view.finish_moved_of_business_days":
    "Fin déplacée de {count} sur {of} jours ouvrés",
  "l.scheduling_view.of": "{toFixed} ({count} sur {of})",
  "l.scheduling_view.of_2": "{value} % ({count} sur {of} {of_what})",
  "l.scheduling_view.passed_failed_not_computable":
    "{passed} réussis, {failed} échoués, {not_computable} non calculables",
  "l.scheduling_view.not_resolved": "Non résolu.",
  "l.scheduling_view.it_is_on_the_critical":
    " Elle est sur le chemin critique, la fin du projet est donc déplacée.",
  "l.scheduling_view.starts_later_finishes_instead_of":
    "{pid} ({who}) commence {from_start} → {scheduled_start}, {days} plus tard ; se termine le {to_finish} au lieu du {from_finish}.{crit}",
  "l.scheduling_view.to": "du {from} au {to}",
  "l.scheduling_view.is_over_capacity_on":
    "{who} dépasse sa capacité sur {span}.",
  "l.scheduling_view.is_asked_for_on_and":
    "{who} est sollicité à {demand} % sur {span} et peut fournir {capacity} %.",
  "l.scheduling_view.no_project_finish_can_be":
    "Aucune fin de projet ne peut être calculée.",
  "l.scheduling_view.the_project_finish_stays":
    "La fin du projet reste le {before}.",
  "l.scheduling_view.the_project_finish_moves_from":
    "La fin du projet passe du {before} au {after} ({days} plus tard).",
  "l.scheduling_view.no_demand_or_supply": "Ni demande ni offre",
  "l.scheduling_view.demand_fte_supply_not_known":
    "Demande {demand_fte} FTE ; offre inconnue",
  "l.scheduling_view.short_fte": "Manque {abs} FTE",
  "l.scheduling_view.spare_fte": "Excédent {gap_fte} FTE",
  "l.scheduling_view.balanced": "Équilibré",
  "l.scheduling_view.the_critical_path_finish": "la fin du chemin critique",
  "l.scheduling_view.of_the_simulated_outcomes_finish":
    "{pct} % des résultats simulés se terminent le {date} ou avant.",
  "l.scheduling_view.no_variation": "Aucune variation",
  "l.scheduling_view.no_estimate_using_the_fallback":
    "Aucune estimation : utilisation de la plage de repli",
  "l.scheduling_view.milestone_no_spread": "Jalon : aucune dispersion",
  "l.scheduling_view.estimated_widened_to_include_the":
    "Estimé (élargi pour inclure la valeur probable)",
  "l.scheduling_view.estimated": "Estimé",
  "l.scheduling_view.pert_favours_the_likely_value":
    "PERT (privilégie la valeur probable)",
  "l.scheduling_view.of_the_simulated_outcomes_cost":
    "{round} % des résultats simulés ne coûtent pas plus que le budget ({budgetText}).",
  "l.scheduling_view.estimated_widened_to_include_the_2":
    "Estimé (élargi pour inclure le montant prévu)",
  "l.scheduling_view.stated_struck_in_of_runs":
    "indiqué {round} %, survenu dans {toFixed} % des exécutions",
  "l.scheduling_view.within_budget_and_on_time":
    "Dans le budget et dans les délais",
  "l.scheduling_view.within_budget_but_late": "Dans le budget mais en retard",
  "l.scheduling_view.on_time_but_over_budget":
    "Dans les délais mais au-dessus du budget",
  "l.scheduling_view.over_budget_and_late": "Au-dessus du budget et en retard",
  "l.scheduling_view.both_within_budget_and_on":
    "À la fois dans le budget et dans les délais : {pct} (dans le budget seul {pct2} ; dans les délais seul {pct3}).",
  "l.scheduling_view.a_late_finish_costs_nothing":
    "Une fin tardive ne coûte rien de plus dans cette exécution (le coût du retard par jour ouvré est de 0).",
  "l.scheduling_view.each_business_day_late_adds":
    "Chaque jour ouvré de retard ajoute {format} ; finir en avance ne rapporte rien.",
  "l.simulation_inputs.the_probability_must_be_a":
    "La probabilité doit être un nombre entier de 0 à 100.",
  "l.simulation_inputs.name_the_task_and_the":
    "Indiquez à la fois la tâche et les jours ouvrés ajoutés, ou laissez les deux vides.",
  "l.simulation_inputs.the_business_days_a_risk":
    "Les jours ouvrés ajoutés par un risque doivent être un nombre entier de 1 à 365.",
  "l.simulation_inputs.give_the_cost_and_its":
    "Indiquez à la fois le coût et sa devise, ou laissez les deux vides.",
  "l.simulation_inputs.the_cost_must_be_an":
    "Le coût doit être un montant supérieur à zéro.",
  "l.simulation_inputs.the_currency_must_be_a":
    "La devise doit être un code de 3 lettres, par exemple EUR, JPY ou GBP.",
  "l.simulation_inputs.a_risk_that_changes_the":
    "Un risque qui modifie le calendrier ou le coût nécessite une probabilité.",
  "l.simulation_inputs.amounts_must_be_numbers_zero":
    "Les montants doivent être des nombres positifs ou nuls.",
  "l.simulation_inputs.the_low_amount_cannot_be":
    "Le montant bas ne peut pas dépasser le montant prévu.",
  "l.simulation_inputs.the_high_amount_cannot_be":
    "Le montant haut ne peut pas être inférieur au montant prévu.",
  "l.simulation_inputs.business_days_must_be_whole":
    "Les jours ouvrés doivent être des nombres entiers de 0 à 3650.",
  "l.simulation_inputs.give_the_task_a_duration":
    "Donnez d'abord une durée à la tâche : c'est sa valeur probable.",
  "l.simulation_inputs.optimistic_cannot_be_above_the":
    "L'optimiste ne peut pas dépasser la durée probable ({likely}).",
  "l.simulation_inputs.pessimistic_cannot_be_below_the":
    "Le pessimiste ne peut pas être inférieur à la durée probable ({likely}).",
  "l.simulation_inputs.no_range_the_simulation_uses":
    "Aucune plage : la simulation utilise sa plage de repli",
  "l.simulation_inputs.to": "{fmt} à {fmt2}",
  "l.wbs.restricted_item": "Élément restreint",
  "l.whatif_view.choose_a_task": "Choisissez une tâche.",
  "l.whatif_view.give_the_new_start_date":
    "Indiquez la nouvelle date de début.",
  "l.whatif_view.duration_must_be_a_whole":
    "La durée doit être un nombre entier de jours ouvrés de 0 à 3650.",
  "l.whatif_view.choose_a_constraint": "Choisissez une contrainte.",
  "l.whatif_view.a_constraint_needs_a_date":
    "Une contrainte nécessite une date.",
  "l.whatif_view.a_hard_constraint_needs_a":
    "Une contrainte stricte nécessite un motif.",
  "l.whatif_view.choose_both_tasks": "Choisissez les deux tâches.",
  "l.whatif_view.a_task_cannot_depend_on":
    "Une tâche ne peut pas dépendre d'elle-même.",
  "l.whatif_view.lag_must_be_a_whole":
    "Le décalage doit être un nombre entier de jours ouvrés (négatif pour une avance).",
  "l.whatif_view.choose_a_link": "Choisissez un lien.",
  "l.whatif_view.start_on": "Commencer « {name} » le {start_date}",
  "l.whatif_view.make_a_milestone": "Faire de « {name} » un jalon",
  "l.whatif_view.for": "{constraint_kind} {constraint_date} pour « {name} »",
  "l.whatif_view.clear_the_constraint_on":
    "Supprimer la contrainte sur « {name} »",
  "l.whatif_view.lag": ", décalage {lag_days}",
  "l.whatif_view.follows": "« {name} » suit « {name2} » ({toUpperCase}{v})",
  "l.whatif_view.remove_the_link": "Supprimer le lien {slice}",
  "l.whatif_view.the_plan_has_no_finish":
    "Le plan n'a pas encore de date de fin.",
  "l.whatif_view.the_finish_stays": "La fin reste le {after}.",
  "l.whatif_view.the_finish_becomes": "La fin devient le {after}.",
  "l.whatif_view.the_plan_no_longer_has":
    "Le plan n'a plus de date de fin (auparavant le {before}).",
  "l.whatif_view.the_finish_moves_from_to":
    "La fin passe du {before} au {after}.",
  "l.whatif_view.cost_impact_is_finance_data":
    "L'impact sur les coûts relève des données financières ; il ne vous est donc pas affiché.",
  "l.whatif_view.no_task_with_work_has":
    "Aucune tâche avec du travail n'a de taux ; il n'y a donc aucun coût à comparer.",
  "l.work_view.work_must_be_a_number":
    "Le travail doit être un nombre d'heures supérieur à zéro.",
  "l.work_view.duration_must_be_a_whole":
    "La durée doit être un nombre entier de jours ouvrés de 1 à 3650.",
  "l.work_view.units_must_be_a_whole":
    "Les unités doivent être un pourcentage entier d'une journée, de 1 à 1000.",
  "l.work_view.give_the_units_too": "Indiquez aussi les unités.",
  "l.work_view.business_days_at_is":
    "{duration_days} jours ouvrés à {units_percent} % font {hoursText}",
  "l.work_view.the_duration_was_worked_out": "La durée a été calculée.",
  "l.work_view.the_share_of_the_day": "La part de la journée a été calculée.",
  "l.work_view.the_work_was_worked_out": "Le travail a été calculé.",
  "l.work_view.rounded_the_three_do_not":
    " Arrondi : les trois valeurs ne concordent pas à la minute près.",
  "l.work_view.capacity_not_checked_the_task":
    "Capacité non vérifiée : la tâche n'a pas encore de dates.",
  "l.work_view.capacity_undeclared":
    "Aucune capacité n'est déclarée pour certains de ces jours : ce n'est donc ni au-delà ni dans la capacité de la personne.",
  "l.work_view.within_the_person_s_capacity":
    "Dans la capacité de la personne.",
  "l.work_view.give_1_to_40_whole":
    "Indiquez de 1 à 40 pourcentages entiers, séparés par des virgules.",
  "l.work_view.the_percentages_add_up_to":
    "La somme des pourcentages est {sum}, et non 100.",
  "p.home.main_x_plans": "Main X · Plans",
  "p.announcements.could_not_load_the_announcements":
    "Impossible de charger les annonces",
  "p.announcements.could_not_publish": "Impossible de publier",
  "p.announcements.could_not_retract": "Impossible de retirer",
  "p.auditor.auditor_ppm": "Auditeur — PPM",
  "p.automations.could_not_load_automations":
    "Impossible de charger les automatisations",
  "p.automations.rule_saved_it_fires_from":
    "Règle enregistrée — elle se déclenche au prochain déplacement correspondant",
  "p.automations.could_not_save_the_rule": "Impossible d'enregistrer la règle",
  "p.automations.that_action_was_refused": "Cette action a été refusée",
  "p.automations.capped_at_run_again_for":
    " (plafonné à {cap} — relancez pour le reste)",
  "p.automations.swept_fired_already_claimed":
    "Balayage : {fired} déclenchées, {skipped_already_claimed} déjà prises en charge{v}",
  "p.automations.the_sweep_failed": "Le balayage a échoué",
  "p.board.board_ppm": "Conseil — PPM",
  "p.calendar.calendar_ppm": "Calendrier — PPM",
  "p.ceo.could_not_load": "Impossible de charger",
  "p.compliance.compliance_ppm": "Conformité — PPM",
  "p.cost_rates.could_not_load_the_rate":
    "Impossible de charger les tables de taux",
  "p.cost_rates.rate_table_added": "Table de taux ajoutée.",
  "p.cost_rates.could_not_add_the_table": "Impossible d'ajouter la table",
  "p.cost_rates.give_a_date_and_an":
    "Indiquez une date et un taux horaire positif ou nul.",
  "p.cost_rates.rate_added": "Taux ajouté.",
  "p.cost_rates.could_not_add_the_rate": "Impossible d'ajouter le taux",
  "p.cost_risk.could_not_run_the_simulation":
    "Impossible d'exécuter la simulation",
  "p.documents.could_not_load_the_documents":
    "Impossible de charger les documents",
  "p.documents.could_not_create_the_document":
    "Impossible de créer le document",
  "p.documents.pid.could_not_load_the_document":
    "Impossible de charger le document",
  "p.documents.pid.that_did_not_work": "Cela n'a pas fonctionné",
  "p.documents.pid.enter_the_approver_s_id":
    "Saisissez l'id de l'approbateur (un id de personne, pas vous-même).",
  "p.documents.pid.sent_for_review": "Envoyé pour revue.",
  "p.documents.pid.needs_changes": "Modifications requises",
  "p.documents.pid.approved": "Approuvé.",
  "p.documents.pid.sent_back_to_draft": "Renvoyé en brouillon.",
  "p.documents.pid.in_force_from_today": "En vigueur à partir d'aujourd'hui.",
  "p.documents.pid.next_draft_started": "Brouillon suivant commencé.",
  "p.documents.pid.could_not_look_that_up":
    "Impossible de rechercher cette information",
  "p.engineering.engineering_ppm": "Ingénierie — PPM",
  "p.executive.executive_ppm": "Direction — PPM",
  "p.financials.financials_ppm": "Finances — PPM",
  "p.gantt.gantt_main_x": "Gantt — Main X",
  "p.gantt.calendar_failed":
    "Le calendrier de travail n'a pas pu être chargé ; les jours non ouvrés ne sont donc pas grisés.",
  "p.kpis.could_not_load_the_kpis": "Impossible de charger les KPI",
  "p.kpis.could_not_define_the_kpi": "Impossible de définir le KPI",
  "p.lifecycle.could_not_load_the_funnel": "Impossible de charger l'entonnoir",
  "p.me.could_not_load_your_tasks": "Impossible de charger vos tâches",
  "p.me.calendar.could_not_load_your_calendar":
    "Impossible de charger votre calendrier",
  "p.me.calendar.could_not_save": "Impossible d'enregistrer",
  "p.me.calendar.could_not_remove": "Impossible de supprimer",
  "p.milestones.could_not_load_the_milestones":
    "Impossible de charger les jalons",
  "p.objectives.create_failed": "échec de la création",
  "p.objectives.alignment_failed": "échec de l'alignement",
  "p.okrs.could_not_load_the_okrs": "Impossible de charger les OKR",
  "p.okrs.could_not_create_the_objective": "Impossible de créer l'objectif",
  "p.plans.plans_main_x": "Plans — Main X",
  "p.plans.pid.main_x": "{v} — Main X",
  "p.plans.pid.board.pt": " · {points} pt",
  "p.plans.pid.board.board_ppm": "Tableau — PPM",
  "p.plans.pid.calendar.could_not_load_the_calendar":
    "Impossible de charger le calendrier",
  "p.plans.pid.calendar.could_not_save": "Impossible d'enregistrer",
  "p.plans.pid.cost_risk.could_not_run_the_simulation":
    "Impossible d'exécuter la simulation",
  "p.plans.pid.costs.could_not_load_the_costs":
    "Impossible de charger les coûts",
  "p.plans.pid.costs.choose_a_task": "Choisissez une tâche.",
  "p.plans.pid.costs.the_amount_must_be_above":
    "Le montant doit être supérieur à zéro.",
  "p.plans.pid.costs.the_currency_must_be_a":
    "La devise doit être un code de 3 lettres, par exemple EUR, JPY ou GBP.",
  "p.money.currency_hint": "par exemple EUR, JPY ou GBP",
  "p.money.amount_or_currency":
    "Indiquez le montant sous forme de nombre et la devise sous forme de code de 3 lettres, par exemple EUR, JPY ou GBP.",
  "p.plans.pid.costs.expense_added": "Dépense ajoutée.",
  "p.plans.pid.costs.could_not_add_the_expense":
    "Impossible d'ajouter la dépense",
  "p.plans.pid.costs.could_not_record": "Impossible d'enregistrer",
  "p.plans.pid.critical_path.could_not_load_the_baselines":
    "Impossible de charger les références",
  "p.plans.pid.critical_path.could_not_take_the_baseline":
    "Impossible de figer la référence",
  "p.plans.pid.critical_path.could_not_load_the_schedule":
    "Impossible de charger le calendrier",
  "p.plans.pid.critical_path.could_not_save_the_estimate":
    "Impossible d'enregistrer l'estimation",
  "p.plans.pid.critical_path.could_not_save": "Impossible d'enregistrer",
  "p.plans.pid.earned_value.could_not_load_earned_value":
    "Impossible de charger la valeur acquise",
  "p.plans.pid.earned_value.could_not_save": "Impossible d'enregistrer",
  "p.plans.pid.earned_value.could_not_record": "Impossible d'enregistrer",
  "p.plans.pid.edit.edit_case_main_x": "Modifier l'affaire — Main X",
  "p.plans.pid.flow.could_not_load_flow_data":
    "Impossible de charger les données de flux.",
  "p.plans.pid.handover.handover_is_about_people_so":
    "La passation concerne des personnes ; elle nécessite donc l'autorisation de gérer les personnes.",
  "p.plans.pid.handover.could_not_preview": "Impossible de prévisualiser",
  "p.plans.pid.handover.what_this_person_holds_changed":
    "Ce que détient cette personne a changé depuis l'aperçu ; rien n'a donc été modifié. Prévisualisez à nouveau, puis validez.",
  "p.plans.pid.handover.could_not_commit": "Impossible de valider",
  "p.plans.pid.handover.discarded_nothing_was_changed":
    "Abandonné. Rien n'a été modifié.",
  "p.plans.pid.import.choose_the_file_first": "Choisissez d'abord le fichier.",
  "p.plans.pid.import.the_import_did_not_work":
    "L'importation n'a pas fonctionné",
  "p.plans.pid.joint_risk.could_not_run_the_simulation":
    "Impossible d'exécuter la simulation",
  "p.plans.pid.leveling.could_not_propose_a_leveling":
    "Impossible de proposer un nivellement",
  "p.plans.pid.leveling.could_not_apply_the_moves":
    "Impossible d'appliquer les déplacements",
  "p.plans.pid.onboarding.could_not_load": "Impossible de charger",
  "p.plans.pid.onboarding.could_not_open_the_checklist":
    "Impossible d'ouvrir la liste de contrôle",
  "p.plans.pid.onboarding.checklist_started": "Liste de contrôle commencée.",
  "p.plans.pid.onboarding.could_not_start": "Impossible de commencer",
  "p.plans.pid.onboarding.could_not_save": "Impossible d'enregistrer",
  "p.plans.pid.onboarding.checklist_completed": "Liste de contrôle terminée.",
  "p.plans.pid.onboarding.could_not_complete": "Impossible de terminer",
  "p.plans.pid.onboarding.say_why_you_are_cancelling":
    "Indiquez pourquoi vous annulez.",
  "p.plans.pid.onboarding.checklist_cancelled": "Liste de contrôle annulée.",
  "p.plans.pid.onboarding.could_not_cancel": "Impossible d'annuler",
  "p.plans.pid.onboarding.could_not_record": "Impossible d'enregistrer",
  "p.plans.pid.raci.could_not_load_who_does":
    "Impossible de charger qui fait quoi",
  "p.plans.pid.raci.could_not_save": "Impossible d'enregistrer",
  "p.plans.pid.raci.use_a_person_or_worker":
    "Utilisez une référence person: ou worker: avec un UUID.",
  "p.plans.pid.schedule_health.could_not_run_the_schedule":
    "Impossible d'exécuter les contrôles de santé du calendrier",
  "p.plans.pid.schedule_risk.could_not_run_the_simulation":
    "Impossible d'exécuter la simulation",
  "p.plans.pid.wbs.could_not_load_the_tasks":
    "Impossible de charger les tâches",
  "p.plans.pid.wbs.link_counts_failed":
    "Le nombre de liens de chaque tâche n'a pas pu être chargé ; aucun décompte n'est donc affiché.",
  "p.plans.pid.what_if.could_not_load_the_plan":
    "Impossible de charger le plan",
  "p.plans.pid.what_if.at_most_edits_at_a":
    "Au plus {MAX_EDITS} modifications à la fois.",
  "p.plans.pid.what_if.edit_staged_preview_to_see":
    "Modification préparée. Prévisualisez pour voir son effet.",
  "p.plans.pid.what_if.discarded_nothing_was_changed":
    "Abandonné. Rien n'a été modifié.",
  "p.plans.pid.what_if.could_not_preview": "Impossible de prévisualiser",
  "p.plans.pid.what_if.the_plan_changed_since_this":
    "Le plan a changé depuis cet aperçu ; rien n'a donc été appliqué. Prévisualisez à nouveau, puis validez.",
  "p.plans.pid.what_if.could_not_commit": "Impossible de valider",
  "p.plans.pid.what_if.to": "du {early_start} au {early_finish}",
  "p.plans.pid.what_if.not_scheduled": "non planifié",
  "p.plans.pid.work.could_not_load_the_plan": "Impossible de charger le plan",
  "p.plans.pid.work.could_not_load_the_work":
    "Impossible de charger le profil de travail",
  "p.plans.pid.work.could_not_save": "Impossible d'enregistrer",
  "p.plans.pid.work.contour_saved": "Profil enregistré.",
  "p.plans.merge.merge_plans_ppm": "Fusionner des plans — PPM",
  "p.plans.new.new_case_main_x": "Nouvelle affaire — Main X",
  "p.prioritization.could_not_load_the_queue": "Impossible de charger la file",
  "p.prioritization.could_not_load_the_score": "Impossible de charger le score",
  "p.proposals.duplicate_check_failed": "échec de la vérification des doublons",
  "p.regulator.regulator_ppm": "Régulateur — PPM",
  "p.reports.save_failed": "échec de l'enregistrement",
  "p.reports.run_failed": "échec de l'exécution",
  "p.resources.could_not_load_the_resource":
    "Impossible de charger le plan des ressources",
  "p.resources.rbs.could_not_load_resources":
    "Impossible de charger les ressources",
  "p.reviews.could_not_load_reviews": "Impossible de charger les revues",
  "p.reviews.invitation_sent": "Invitation envoyée",
  "p.reviews.could_not_delegate": "Impossible de déléguer",
  "p.reviews.score_0_100_leave_blank": "Score 0–100 (laisser vide pour aucun)",
  "p.reviews.score_must_be_between_0":
    "Le score doit être compris entre 0 et 100",
  "p.reviews.that_action_was_refused": "Cette action a été refusée",
  "p.reviews.could_not_load_consensus": "Impossible de charger le consensus",
  "p.risk.risk_ppm": "Risque — PPM",
  "p.scenarios.evaluate_failed": "échec de l'évaluation",
  "p.security.security_ppm": "Sécurité — PPM",
  "p.technology.technology_ppm": "Technologie — PPM",
  "p.tour.tour_main_x_plans": "Visite guidée · Main X Plans",
  "p.verify.this_sign_in_link_is":
    "Il manque le jeton dans ce lien de connexion.",
  "p.verify.we_could_not_reach_the":
    "Impossible de joindre le service de connexion. Veuillez réessayer dans un instant.",
  "p.verify.this_sign_in_link_is_2":
    "Ce lien de connexion est invalide ou a expiré.",
  "l.ceo_view.gate_review": "Revue de jalon",
  "l.ceo_view.scenario_committed": "Scénario engagé",
  "l.ceo_view.proposal": "Proposition",
  "l.ceo_view.plans_merged": "Plans fusionnés",
  "c.CumulativeFlow.in_review": "En revue",
  "c.CumulativeFlow.in_progress": "En cours",
  "c.CumulativeFlow.todo": "À faire",
  "l.document_view.draft": "Brouillon",
  "l.document_view.in_review": "En revue",
  "l.document_view.approved_not_yet_in_force":
    "Approuvé, pas encore en vigueur",
  "l.document_view.replaced": "Remplacé",
  "l.document_view.send_for_review": "Envoyer pour revue",
  "l.document_view.approve": "Approuver",
  "l.document_view.bring_into_force_today": "Mettre en vigueur aujourd'hui",
  "l.document_view.start_the_next_draft": "Commencer le brouillon suivant",
  "l.document_view.waiting_for_the_approver": "En attente de l'approbateur",
  "l.document_view.approved": "Approuvé",
  "l.document_view.change_this_document": "Modifier ce document",
  "l.document_view.someone_else_approves_it_you":
    "Une autre personne l'approuve : vous ne pouvez pas approuver votre propre document.",
  "l.document_view.only_the_person_asked_can":
    "Seule la personne sollicitée peut décider.",
  "l.document_view.it_comes_into_force_today":
    "Il entre en vigueur aujourd'hui et remplace la version actuellement en vigueur.",
  "l.document_view.the_next_draft_starts_as":
    "Le brouillon suivant commence comme une copie du texte le plus récent.",
  "l.ev_view.on_track": "en bonne voie",
  "l.ev_view.off_plan": "hors plan",
  "l.ev_view.not_yet_known": "pas encore connu",
  "l.glossary.schedule_performance_index_work_done":
    "Indice de performance des délais : travail réalisé comparé au travail prévu à ce jour. Au-dessus de 1,00, le plan est en avance.",
  "l.glossary.cost_performance_index_value_of":
    "Indice de performance des coûts : valeur du travail réalisé comparée à ce qu'il a coûté. Au-dessus de 1,00, le plan est sous le budget.",
  "l.glossary.schedule_index_by_time_how":
    "Indice des délais en temps : l'avancement du plan mesuré en temps, qui reste fidèle même quand un projet se termine en retard.",
  "l.glossary.earned_value_the_budget_for":
    "Valeur acquise : le budget du travail effectivement terminé.",
  "l.glossary.planned_value_the_budget_for":
    "Valeur planifiée : le budget du travail qui devrait être terminé à ce jour.",
  "l.glossary.actual_cost_what_has_really":
    "Coût réel : ce qui a réellement été dépensé.",
  "l.glossary.estimate_at_completion_the_expected":
    "Estimation à l'achèvement : le coût final attendu.",
  "l.glossary.estimate_to_complete_what_is":
    "Reste à faire estimé : ce qu'il est encore prévu de dépenser.",
  "l.glossary.variance_at_completion_the_expected":
    "Écart à l'achèvement : le coût final attendu par rapport au budget.",
  "l.glossary.red_amber_or_green_a": "Rouge, ambre ou vert : un voyant d'état.",
  "l.glossary.responsible_accountable_consulted_in":
    "Réalisateur, Approbateur, Consulté, Informé : qui fait le travail, qui le valide, qui est consulté et qui est informé.",
  "l.glossary.a_way_of_weighting_the":
    "Une façon de pondérer l'estimation probable davantage que les cas optimiste et pessimiste.",
  "l.glossary.key_performance_indicator_a_measure":
    "Indicateur clé de performance : une mesure suivie dans le temps par rapport à une cible.",
  "l.glossary.key_performance_indicators_measures_":
    "Indicateurs clés de performance : des mesures suivies dans le temps par rapport à une cible.",
  "l.glossary.objective_and_key_results_a":
    "Objectif et résultats clés : un but assorti de quelques résultats mesurables.",
  "l.glossary.objectives_and_key_results_goals":
    "Objectifs et résultats clés : des buts assortis de quelques résultats mesurables.",
  "l.import_view.jira_csv_export": "Jira (export CSV)",
  "l.import_view.asana_csv_export": "Asana (export CSV)",
  "l.import_view.microsoft_project_xml_file": "Microsoft Project (fichier XML)",
  "l.import_view.columns_not_imported": "Colonnes non importées",
  "l.import_view.rows_skipped": "Lignes ignorées",
  "l.import_view.links_not_imported": "Liens non importés",
  "l.import_view.parents_not_used": "Parents non utilisés",
  "l.import_view.dates_not_understood": "Dates non reconnues",
  "l.import_view.fields_shortened": "Champs raccourcis",
  "l.import_view.status_notes": "Remarques sur les statuts",
  "l.import_view.resources_not_matched_to_people":
    "Ressources non associées à des personnes",
  "l.import_view.costs_not_imported": "Coûts non importés",
  "l.import_view.schedules_not_imported": "Calendriers non importés",
  "l.import_view.constraints_not_imported": "Contraintes non importées",
  "l.import_view.calendar_notes": "Remarques sur le calendrier",
  "l.kpi_view.improving": "En amélioration",
  "l.kpi_view.getting_worse": "En dégradation",
  "l.kpi_view.no_change": "Aucun changement",
  "l.milestone_view.on_track": "En bonne voie",
  "l.milestone_view.at_risk": "À risque",
  "l.milestone_view.forecast_late": "Retard prévu",
  "l.milestone_view.no_forecast": "Aucune prévision",
  "l.milestone_view.it_is_not_linked_to":
    "Il n'est lié à aucune tâche ; il n'y a donc rien sur quoi fonder une prévision.",
  "l.milestone_view.its_open_tasks_have_no":
    "Ses tâches ouvertes n'ont pas de dates que le calendrier peut placer (donnez-leur une durée et un début ou un prédécesseur).",
  "l.milestone_view.the_plan_s_dependencies_contain":
    "Les dépendances du plan contiennent un cycle ; aucune prévision n'est donc possible.",
  "l.okr_view.not_measurable_the_start_and":
    "Non mesurable : le départ et la cible sont identiques, il n'y a donc aucune distance à parcourir.",
  "l.okr_view.not_measurable_a_maintain_key":
    "Non mesurable : un résultat clé de maintien nécessite une plage de tolérance.",
  "l.okr_view.not_measurable_the_numbers_are":
    "Non mesurable : les nombres sont trop grands pour être notés.",
  "l.onboarding_view.someone_joins_this_plan": "Quelqu'un rejoint ce plan",
  "l.onboarding_view.someone_leaves_this_plan": "Quelqu'un quitte ce plan",
  "l.onboarding_view.take_this_plan_in": "Prendre ce plan en charge",
  "l.onboarding_view.close_this_plan_out": "Clôturer ce plan",
  "l.onboarding_view.booking": "Réservation",
  "l.onboarding.executive": "Direction",
  "l.onboarding.outcomes_and_exposure_across_the":
    "Résultats et exposition sur l'ensemble du parc, et non sur un seul plan.",
  "l.onboarding.read_the_executive_summary": "Lire la synthèse de direction",
  "l.onboarding.health_decisions_benefits_realized_a":
    "Santé, décisions, bénéfices réalisés et alignement stratégique dans une seule vue.",
  "l.onboarding.open_the_at_a_glance": "Ouvrir le tableau de bord synthétique",
  "l.onboarding.the_whole_estate_s_rag":
    "L'état RAG de tout le parc et les violations de calendrier d'un seul coup d'œil.",
  "l.onboarding.review_the_board_pack": "Examiner le dossier du conseil",
  "l.onboarding.the_pack_a_board_meeting":
    "Le dossier réellement remis lors d'un conseil : investissements et tendances.",
  "l.onboarding.check_budget_variance_and_exposure":
    "Vérifier l'écart budgétaire et l'exposition",
  "l.onboarding.where_spend_is_diverging_from":
    "Là où les dépenses s'écartent du plan, par devise — les devises ne sont jamais additionnées entre elles.",
  "l.onboarding.compare_two_candidate_portfolios":
    "Comparer deux portefeuilles candidats",
  "l.onboarding.what_if_scenarios_side_by":
    "Des scénarios hypothétiques côte à côte, avant d'engager des fonds sur l'un ou l'autre.",
  "l.onboarding.see_what_s_ranked_highest":
    "Voir ce qui est le mieux classé, et pourquoi",
  "l.onboarding.the_smart_score_queue_every":
    "La file Smart Score — chaque score montre les éléments qui le justifient, ou indique qu'il n'en a aucun.",
  "l.onboarding.pmo_portfolio_manager": "PMO / gestionnaire de portefeuille",
  "l.onboarding.intake_governance_and_the_schedule":
    "Entrée des demandes, gouvernance et calendrier de tous les plans.",
  "l.onboarding.open_the_intake_pipeline": "Ouvrir le circuit d'entrée",
  "l.onboarding.every_proposal_s_stage_from":
    "L'étape de chaque proposition, du brouillon à l'approbation.",
  "l.onboarding.register_a_plan": "Enregistrer un plan",
  "l.onboarding.the_first_plan_you_create":
    "Le premier plan que vous créez est un bon moyen de voir la structure complète des données.",
  "l.onboarding.browse_the_plan_registry": "Parcourir le registre des plans",
  "l.onboarding.one_recursive_collection_kind_is":
    "Une seule collection récursive — `kind` est une étiquette, pas un verrou.",
  "l.onboarding.open_a_plan_s_governance":
    "Ouvrir l'onglet gouvernance d'un plan",
  "l.onboarding.gate_reviews_risks_and_budget":
    "Revues de jalon, risques et lignes budgétaires de ce plan.",
  "l.onboarding.look_at_the_cross_plan": "Consulter le calendrier transversal",
  "l.onboarding.dependencies_the_critical_path_and":
    "Dépendances, chemin critique et tout successeur qui glisse.",
  "l.onboarding.check_the_risk_register": "Consulter le registre des risques",
  "l.onboarding.everything_open_or_mitigating_ranked":
    "Tout ce qui est ouvert ou en cours d'atténuation, classé par exposition.",
  "l.onboarding.review_the_compliance_register":
    "Examiner le registre de conformité",
  "l.onboarding.standing_findings_across_the_estate":
    "Les constats en cours sur tout le parc, pas seulement ceux d'un plan.",
  "l.onboarding.run_a_saved_report": "Exécuter un rapport enregistré",
  "l.onboarding.a_filter_field_projection_saved":
    "Un filtre et une sélection de champs enregistrés pour réutilisation, exécutés à la demande.",
  "l.onboarding.resource_manager": "Gestionnaire des ressources",
  "l.onboarding.who_is_allocated_where_and":
    "Qui est alloué où, et comment la livraison avance réellement.",
  "l.onboarding.check_the_capacity_rollup":
    "Vérifier la consolidation des capacités",
  "l.onboarding.who_is_over_allocated_this":
    "Qui est surallouée sur cette période, en cumulant tous ses plans.",
  "l.onboarding.open_a_plan_s_task": "Ouvrir le tableau des tâches d'un plan",
  "l.onboarding.the_kanban_board_that_task":
    "Le tableau Kanban par lequel passe réellement le travail au niveau des tâches.",
  "l.onboarding.read_that_plan_s_flow":
    "Lire les indicateurs de flux de ce plan",
  "l.onboarding.cycle_time_lead_time_and":
    "Temps de cycle, délai de traversée et efficacité du flux — tirés de l'historique du tableau.",
  "l.onboarding.scan_blocked_items_and_the":
    "Passer en revue les éléments bloqués et la répartition de la livraison",
  "l.onboarding.what_s_stuck_the_moscow":
    "Ce qui est bloqué, la répartition MoSCoW et le calendrier des jalons dans une seule vue.",
  "l.onboarding.look_at_the_milestone_calendar":
    "Consulter le calendrier des jalons",
  "l.onboarding.every_milestone_across_every_plan":
    "Tous les jalons de tous les plans, échéances comprises.",
  "l.onboarding.see_what_automations_are_configured":
    "Voir les automatisations configurées",
  "l.onboarding.what_fires_on_a_field":
    "Ce qui se déclenche lors d'un changement de champ, et ce que cela fait.",
  "l.onboarding.open_collaborative_reviews": "Ouvrir les revues collaboratives",
  "l.onboarding.where_a_plan_gets_a":
    "Là où un plan obtient un second avis avant qu'une décision soit prise.",
  "l.raci_view.responsible": "Réalisateur",
  "l.raci_view.accountable": "Approbateur",
  "l.raci_view.consulted": "Consulté",
  "l.raci_view.informed": "Informé",
  "l.raci_view.does_the_work": "réalise le travail",
  "l.raci_view.answers_for_it_and_signs":
    "en répond et le valide (un par tâche)",
  "l.raci_view.is_asked_before": "est consulté avant",
  "l.raci_view.is_told_after": "est informé après",
  "l.scheduling_view.no_schedule_baseline_has_been":
    "Aucune référence de calendrier n'a été figée pour ce plan.",
  "l.scheduling_view.nothing_was_baselined_to_finish":
    "Rien dans la référence ne devait se terminer à la date d'état.",
  "l.scheduling_view.the_plan_s_dependencies_contain":
    "Les dépendances du plan contiennent un cycle.",
  "l.scheduling_view.there_are_no_incomplete_tasks":
    "Il n'y a aucune tâche inachevée à mesurer.",
  "l.scheduling_view.there_are_no_relationships_between":
    "Il n'y a aucune relation entre tâches à mesurer.",
  "l.scheduling_view.no_incomplete_task_has_dates":
    "Aucune tâche inachevée n'a de dates que le calendrier peut placer.",
  "l.scheduling_view.no_incomplete_task_has_a":
    "Aucune tâche inachevée n'a de durée.",
  "l.scheduling_view.the_plan_has_no_tasks": "Le plan n'a aucune tâche.",
  "l.scheduling_view.there_is_no_critical_task":
    "Il n'y a aucune tâche critique à tester.",
  "l.scheduling_view.no_path_remains_to_measure":
    "Il ne reste aucun chemin à mesurer.",
  "l.scheduling_view.every_task_involved_has_already":
    "Toutes les tâches concernées ont déjà commencé ; aucune ne peut donc être déplacée.",
  "l.scheduling_view.the_tasks_involved_are_critical":
    "Les tâches concernées sont critiques ; en déplacer une déplacerait la fin. Autorisez les tâches critiques pour voir ce coût.",
  "l.scheduling_view.delaying_a_task_enough_would":
    "Retarder suffisamment une tâche casserait le calendrier (marge négative ou fin plus tardive).",
  "l.scheduling_view.a_task_involved_has_a":
    "Une tâche concernée a une contrainte stricte et ne peut pas être déplacée.",
  "l.scheduling_view.the_schedule_would_not_let":
    "Le calendrier ne permettrait pas à la tâche de commencer plus tard.",
  "l.scheduling_view.the_search_stopped_at_its":
    "La recherche s'est arrêtée à sa limite avant de résoudre ce point.",
  "l.scheduling_view.no_probability": "aucune probabilité",
  "l.scheduling_view.no_task_named": "aucune tâche indiquée",
  "l.scheduling_view.no_impact_in_business_days":
    "aucun impact en jours ouvrés",
  "l.scheduling_view.the_risk_is_not_open": "le risque n'est pas ouvert",
  "l.scheduling_view.its_task_is_not_on": "sa tâche n'est pas au calendrier",
  "l.scheduling_view.the_plan_has_no_budget":
    "le plan n'a aucune ligne budgétaire dans sa devise",
  "l.whatif_view.move_a_task_set_its": "Déplacer une tâche (définir son début)",
  "l.whatif_view.change_a_task_s_duration": "Modifier la durée d'une tâche",
  "l.whatif_view.change_a_task_s_constraint":
    "Modifier la contrainte d'une tâche",
  "l.whatif_view.add_a_link": "Ajouter un lien",
  "l.whatif_view.remove_a_link": "Supprimer un lien",
  "l.whatif_view.must_start_on_hard": "Doit commencer le (stricte)",
  "l.whatif_view.must_finish_on_hard": "Doit finir le (stricte)",
  "l.work_view.fixed_units": "Unités fixes",
  "l.work_view.fixed_duration": "Durée fixe",
  "l.work_view.fixed_work": "Travail fixe",
  "l.work_view.changing_the_work_changes_how":
    "Modifier le travail modifie la durée ; la part de la journée reste inchangée.",
  "l.work_view.changing_the_work_changes_the":
    "Modifier le travail modifie la part de la journée ; la durée reste inchangée.",
  "l.work_view.changing_the_duration_or_the":
    "Modifier la durée ou la part de la journée modifie l'autre ; le travail reste inchangé.",
  "l.work_view.flat_the_same_each_day": "Plat : identique chaque jour",
  "l.work_view.front_loaded_more_at_the":
    "Concentré au début : davantage au départ",
  "l.work_view.back_loaded_more_at_the":
    "Concentré à la fin : davantage à la fin",
  "l.work_view.bell_most_in_the_middle": "Cloche : le plus au milieu",
  "l.work_view.turtle_ramps_up_holds_ramps":
    "Tortue : montée, palier, descente",
  "l.work_view.custom_percentages": "Pourcentages personnalisés",
  "l.work_view.has_no_dates_yet": "n'a pas encore de dates",
  "l.work_view.has_no_business_day_to": "ne tombe sur aucun jour ouvré",
  "l.work_view.has_a_contour_that_gives":
    "a un profil qui ne lui donne aucun poids",
  "l.work_view.no_rate_applies": "aucun taux ne s'applique",
  "l.work_view.no_rate_applied_on_some": "aucun taux appliqué certains jours",
  "l.work_view.has_nobody_assigned": "n'a personne d'affecté",
  "p._layout_svelte.reviews": "Revues",
  "p._layout_svelte.automations": "Automatisations",
  "p._layout_svelte.goals_okrs": "Objectifs (OKR)",
  "p._layout_svelte.onboarding": "Intégration",
  "p.ceo.red": "Rouge",
  "p.ceo.amber": "Ambre",
  "p.ceo.green": "Vert",
  "p.display.saved_on_this_device": "Enregistré sur cet appareil.",
  "p.display.saved_to_your_account_so":
    "Enregistré dans votre compte ; il vous suit donc sur d'autres navigateurs.",
  "p.display.could_not_save_to_your":
    "Impossible d'enregistrer dans votre compte pour le moment ; il est conservé sur cet appareil.",
  "p.me.today": "Aujourd'hui",
  "p.me.this_week": "Cette semaine",
  "p.me.later": "Plus tard",
  "p.me.no_due_date": "Sans échéance",
  "p.plans.pid.leveling": "Nivellement",
  "p.plans.pid.cost_and_schedule_risk": "Risque de coût et de délai",
  "p.plans.pid.board.todo": "À faire",
  "p.plans.pid.board.in_progress": "En cours",
  "p.plans.pid.board.in_review": "En revue",
  "p.common.business_day_one": "{n} jour ouvré",
  "p.common.business_day_other": "{n} jours ouvrés",
  "p.common.calendar_day_one": "{n} jour calendaire",
  "p.common.calendar_day_other": "{n} jours calendaires",
  "l.milestone_view.rescheduled_one": "replanifié {count} fois",
  "l.milestone_view.rescheduled_other": "replanifié {count} fois",
  "l.milestone_view.open_of_linked_one":
    "{open_tasks} tâche sur {linked_tasks} encore ouverte",
  "l.milestone_view.open_of_linked_other":
    "{open_tasks} tâches sur {linked_tasks} encore ouvertes",
  "l.cpm_view.restricted_one":
    "{n} dépendance vers un plan que vous ne pouvez pas voir est exclue de ces dates.",
  "l.cpm_view.restricted_other":
    "{n} dépendances vers un plan que vous ne pouvez pas voir sont exclues de ces dates.",
  "l.cpm_view.monday": "Lundi",
  "l.cpm_view.tuesday": "Mardi",
  "l.cpm_view.wednesday": "Mercredi",
  "l.cpm_view.thursday": "Jeudi",
  "l.cpm_view.friday": "Vendredi",
  "l.cpm_view.saturday": "Samedi",
  "l.cpm_view.sunday": "Dimanche",
  "l.ev_view.gap_more": "{money} de valeur acquise de plus que prévu à ce jour",
  "l.ev_view.gap_less":
    "{money} de valeur acquise de moins que prévu à ce jour",
  "l.ev_view.cost_line_less":
    "{word} : CPI {ratio}, {money} dépensés de moins que la valeur acquise.",
  "l.ev_view.cost_line_more":
    "{word} : CPI {ratio}, {money} dépensés de plus que la valeur acquise.",
  "l.ev_view.tasks_counted_one":
    "{count} tâche comptée, pondérée {weighting} ; {how}",
  "l.ev_view.tasks_counted_other":
    "{count} tâches comptées, pondérées {weighting} ; {how}",
  "l.ev_view.equally": "à parts égales",
  "l.onboarding_view.would_move_one": "{n} élément serait transféré à {to}.",
  "l.onboarding_view.would_move_other":
    "{n} éléments seraient transférés à {to}.",
  "l.onboarding_view.would_release_one":
    "{n} élément serait libéré : tâches et risques désaffectés, réservations et rôles supprimés.",
  "l.onboarding_view.would_release_other":
    "{n} éléments seraient libérés : tâches et risques désaffectés, réservations et rôles supprimés.",
  "l.whatif_view.set_duration_one":
    "Fixer « {name} » à {duration_days} jour ouvré",
  "l.whatif_view.set_duration_other":
    "Fixer « {name} » à {duration_days} jours ouvrés",
  "l.work_view.over_capacity_one":
    "Au-delà de la capacité sur {days_over} jour calendaire (du {from} au {to}) : cela demande plus que ce que la personne peut fournir.",
  "l.work_view.over_capacity_other":
    "Au-delà de la capacité sur {days_over} jours calendaires (du {from} au {to}) : cela demande plus que ce que la personne peut fournir.",
  "p.plans.pid.handover.done_one":
    "Terminé : {count} élément transmis et enregistré.",
  "p.plans.pid.handover.done_other":
    "Terminé : {count} éléments transmis et enregistrés.",
  "p.plans.pid.what_if.committed_one":
    "Validé : {count} modification enregistrée.",
  "p.plans.pid.what_if.committed_other":
    "Validé : {count} modifications enregistrées.",
  "l.import_view.mspdi_hint":
    "Dans Microsoft Project, utilisez Fichier → Enregistrer sous → Format XML. Un fichier .mpp ne peut pas être lu.",
  "c.CalendarMonth.january": "Janvier",
  "c.CalendarMonth.february": "Février",
  "c.CalendarMonth.march": "Mars",
  "c.CalendarMonth.april": "Avril",
  "c.CalendarMonth.may": "Mai",
  "c.CalendarMonth.june": "Juin",
  "c.CalendarMonth.july": "Juillet",
  "c.CalendarMonth.august": "Août",
  "c.CalendarMonth.september": "Septembre",
  "c.CalendarMonth.october": "Octobre",
  "c.CalendarMonth.november": "Novembre",
  "c.CalendarMonth.december": "Décembre",
  "c.LilyGanttTable.schedule": "Calendrier",
  "c.LilyKanbanTable.board": "Tableau",
  "l.announcement_view.yesterday": "hier",
  "c.CalendarMonth.holiday_state": "férié",
  "c.CalendarMonth.absent_state": "absent",
  "l.ev_view.word_ahead": "en avance",
  "l.ev_view.word_behind": "en retard",
  "l.ev_view.slipping": "en glissement",
  "l.import_view.task_one": "{n} tâche",
  "l.import_view.task_other": "{n} tâches",
  "l.import_view.dependency_one": "{n} dépendance",
  "l.import_view.dependency_other": "{n} dépendances",
  "l.import_view.word_update": "mettre à jour",
  "l.import_view.word_updated": "mises à jour",
  "l.rbs.headcount_one": "{headcount} personne",
  "l.rbs.headcount_other": "{headcount} personnes",
  "l.scheduling_view.exactly": "exactement",
  "l.scheduling_view.strength_strong": "forte",
  "l.scheduling_view.strength_moderate": "modérée",
  "l.scheduling_view.strength_weak": "faible",
  "l.simulation_inputs.planned_word": "prévu",
  "p.plans.pid.board.unassigned": "non affecté",
  "p.plans.pid.costs.recorded_one":
    "{recorded} saisie de temps enregistrée ; {already_recorded} étaient déjà enregistrées ; {length} n'ont pas de taux.",
  "p.plans.pid.costs.recorded_other":
    "{recorded} saisies de temps enregistrées ; {already_recorded} étaient déjà enregistrées ; {length} n'ont pas de taux.",
  "c.CostRiskView.histogram_label":
    "Histogramme du coût simulé en {currency}, avec P50, P80, P90 et le budget indiqués",
  "c.CumulativeFlow.chart_label":
    "Flux cumulé sur {days} jours calendaires, avec un pic à {peak} tâches",
  "p.plans.pid.onboarding.owner_of": "Responsable de {title}",
  "p.plans.pid.onboarding.due_date_of": "Échéance de {title}",
  "p.plans.pid.onboarding.why_skip_title": "Pourquoi ignorer {title}",
  "p.plans.pid.what_if.remove_edit": "Supprimer la modification {n}",
  "c.LilyGanttTable.cell_active": "{name} : du {start} au {end}{ends}",
  "c.LilyGanttTable.ends_here": " (se termine ici)",
  "c.LilyGanttTable.cell_not_scheduled": "{name} : non planifié",
  "l.simulation_inputs.range_business_days":
    "{optimistic} / {likely} / {pessimistic} jours ouvrés (optimiste / probable / pessimiste)",
  "l.enum.task_status.todo": "à faire",
  "l.enum.task_status.in_progress": "en cours",
  "l.enum.task_status.in_review": "en revue",
  "l.enum.task_status.done": "terminé",
  "l.enum.task_status.blocked": "bloqué",
  "l.enum.kind.Portfolio": "Portefeuille",
  "l.enum.kind.Project": "Projet",
  "l.enum.kind.Product": "Produit",
  "l.enum.kind.Program": "Programme",
  "l.enum.kind.Practice": "Pratique",
  "l.enum.kind.Process": "Processus",
  "l.enum.kind.Purpose": "Finalité",
  "l.enum.kind.Pathway": "Parcours",
  "l.enum.kind.Proposal": "Proposition",
  "l.enum.plan_status.Proposed": "Proposé",
  "l.enum.plan_status.Active": "Actif",
  "l.enum.plan_status.OnHold": "En attente",
  "l.enum.plan_status.Completed": "Terminé",
  "l.enum.plan_status.Cancelled": "Annulé",
  "l.enum.collection.portfolios": "portefeuilles",
  "l.enum.collection.projects": "projets",
  "l.enum.collection.products": "produits",
  "l.enum.collection.programs": "programmes",
  "l.enum.collection.practices": "pratiques",
  "l.enum.collection.processes": "processus",
  "l.enum.collection.purposes": "finalités",
  "l.enum.collection.pathways": "parcours",
  "l.enum.proposal_status.draft": "brouillon",
  "l.enum.proposal_status.submitted": "soumise",
  "l.enum.proposal_status.in_review": "en revue",
  "l.enum.proposal_status.approved": "approuvée",
  "l.enum.proposal_status.rejected": "rejetée",
  "l.enum.proposal_status.promoted": "promue",
  "l.enum.band.high": "élevé",
  "l.enum.band.medium": "moyen",
  "l.enum.band.low": "faible",
  "l.enum.band.unscored": "non noté",
  "l.enum.api.unauthorized": "non authentifié",
  "l.enum.api.forbidden": "interdit",
  "l.enum.api.not_found": "introuvable",
  "l.enum.api.internal_error": "Erreur interne",
  "l.enum.api.rate_limited": "limite de requêtes atteinte",
  "l.enum.api.http_status": "HTTP {status}",
  "p.error.not_found": "Cette page n'existe pas.",
  "p.error.not_allowed": "Vous n'êtes pas autorisé à voir cette page.",
  "p.error.went_wrong": "Une erreur s'est produite. Veuillez réessayer.",
  "p.error.title": "Erreur {status} — PPM",
  "p.error.back": "Retour au début",
  "p.issues.issues_ppm": "Registre des problèmes — PPM",
  "p.issues.issues": "Registre des problèmes",
  "p.issues.intro":
    "Un problème est quelque chose qui s'est déjà produit et qu'il faut gérer ; un risque est quelque chose qui pourrait se produire. Les plus graves d'abord, puis les plus anciens. L'âge est exprimé en jours calendaires depuis le signalement du problème.",
  "p.issues.could_not_load": "Impossible de charger les problèmes",
  "p.issues.by_severity": "Problèmes par gravité",
  "p.issues.overdue_count": "En retard",
  "p.issues.oldest_open":
    "Plus ancien ouvert : {title} ({days} jours calendaires)",
  "p.issues.show": "Afficher",
  "p.issues.all_statuses": "Tous les statuts",
  "p.issues.open_only": "Ouverts et en cours uniquement",
  "p.issues.severity_filter": "Gravité",
  "p.issues.all_severities": "Toutes les gravités",
  "p.issues.older_than": "Âge minimal en jours calendaires",
  "p.issues.none": "Aucun problème enregistré.",
  "p.issues.none_match": "Aucun problème ne correspond.",
  "p.issues.loading": "Chargement…",
  "p.issues.count_one": "{n} problème, les plus graves d'abord",
  "p.issues.count_other": "{n} problèmes, les plus graves d'abord",
  "p.issues.truncated":
    "Affichage des {n} premiers ; affinez les filtres pour voir le reste.",
  "p.issues.title_col": "Problème",
  "p.issues.kind_col": "Type",
  "p.issues.severity_col": "Gravité",
  "p.issues.status_col": "Statut",
  "p.issues.owner_col": "Responsable",
  "p.issues.age_col": "Âge (jours calendaires)",
  "p.issues.due_col": "Échéance",
  "p.issues.overdue_flag": "En retard",
  "p.issues.no_owner": "Aucun responsable",
  "p.issues.no_due": "Sans échéance",
  "p.issues.plan_link": "Ouvrir les problèmes du plan",
  "p.issues.plan_issues_link": "Problèmes",
  "p.plan_issues.title_ppm": "Problèmes du plan — PPM",
  "p.plan_issues.title": "Problèmes du plan",
  "p.plan_issues.intro":
    "Les problèmes signalés sur ce plan, les plus graves d'abord. Un problème est quelque chose qui s'est déjà produit ; un risque est quelque chose qui pourrait se produire. Signalez un problème, modifiez-en un, ou créez-en un à partir d'un risque qui s'est produit.",
  "p.plan_issues.back": "Retour au plan",
  "p.plan_issues.register_link": "Problèmes de tous les plans",
  "p.plan_issues.could_not_load": "Impossible de charger les problèmes du plan",
  "p.plan_issues.could_not_save": "Impossible d'enregistrer le problème",
  "p.plan_issues.counts":
    "{total} au total, {open} ouverts ou en cours, {overdue} en retard",
  "p.plan_issues.actions_col": "Actions",
  "p.plan_issues.edit": "Modifier",
  "p.plan_issues.raise_heading": "Signaler un problème",
  "p.plan_issues.edit_heading": "Modifier le problème",
  "p.plan_issues.f_title": "Titre",
  "p.plan_issues.f_description": "Description",
  "p.plan_issues.f_owner": "Responsable (une référence person: ou worker:)",
  "p.plan_issues.f_raised_on": "Signalé le (laisser vide pour aujourd'hui)",
  "p.plan_issues.f_resolution":
    "Résolution (nécessaire pour clore le problème)",
  "p.plan_issues.raise": "Signaler le problème",
  "p.plan_issues.save": "Enregistrer les modifications",
  "p.plan_issues.cancel_edit": "Annuler la modification",
  "p.plan_issues.raised": "Le problème a été signalé.",
  "p.plan_issues.saved": "Le problème a été enregistré.",
  "p.plan_issues.problem_title_required": "Donnez un titre au problème.",
  "p.plan_issues.problem_resolution_required":
    "Un problème clos nécessite une résolution : indiquez comment il a été réglé.",
  "p.plan_issues.problem_due_before_raised":
    "L'échéance ne peut pas être antérieure au jour du signalement du problème.",
  "p.plan_issues.from_risk_heading":
    "Signaler un problème à partir d'un risque",
  "p.plan_issues.from_risk_intro":
    "Pour un risque qui s'est produit. Le problème reprend le titre, la description et le responsable du risque, ainsi qu'une gravité déduite de son exposition ; le risque lui-même n'est pas modifié. Un risque ne peut avoir qu'un seul problème ouvert à la fois.",
  "p.plan_issues.f_risk": "Risque",
  "p.plan_issues.choose_risk": "Choisissez un risque",
  "p.plan_issues.raise_from_risk": "Signaler un problème à partir de ce risque",
  "p.plan_issues.raised_from_risk":
    "Le problème a été signalé à partir du risque.",
  "l.enum.severity.low": "faible",
  "l.enum.severity.med": "moyenne",
  "l.enum.severity.high": "élevée",
  "l.enum.severity.critical": "critique",
  "l.enum.issue_status.open": "ouvert",
  "l.enum.issue_status.in_progress": "en cours",
  "l.enum.issue_status.resolved": "résolu",
  "l.enum.issue_status.closed": "clos",
  "l.enum.issue_kind.bug": "anomalie",
  "l.enum.issue_kind.risk": "risque",
  "l.enum.issue_kind.blocker": "blocage",
  "l.enum.issue_kind.question": "question",
  "l.enum.issue_kind.improvement": "amélioration",
  "l.enum.issue_kind.concern": "préoccupation",
  "p.signin.oidc_idle":
    "Vous avez été déconnecté après une période d'inactivité. Veuillez vous reconnecter.",
  "p.plans.pid.board.only_first_tasks":
    "Ce plan compte {total} tâches ; le tableau affiche les {shown} premières. Filtrez par composant, ou consultez le reste via l'API.",
  "p.plan_modules.title": "Outils de ce plan",
  "p.plan_modules.title_ppm": "Outils de ce plan — PPM",
  "p.plan_modules.back": "← Retour au plan",
  "p.plan_modules.intro":
    "Choisissez les outils que ce plan affiche. Un outil désactivé est masqué sur la page du plan, et ses pages répondent « introuvable ». Rien n’est supprimé : réactiver un outil affiche ce qu’il contenait.",
  "p.plan_modules.source_plan": "Ce plan a son propre choix d’outils.",
  "p.plan_modules.source_default":
    "Ce plan utilise les outils par défaut de l’installation.",
  "p.plan_modules.legend": "Outils affichés par ce plan",
  "p.plan_modules.module_tasks":
    "Tâches, le tableau et l’analyse du calendrier",
  "p.plan_modules.module_time": "Saisie des temps",
  "p.plan_modules.module_risks": "Risques et problèmes",
  "p.plan_modules.module_financials": "Budgets, coûts et valeur acquise",
  "p.plan_modules.module_milestones": "Jalons, le planning et le calendrier",
  "p.plan_modules.module_governance":
    "Gouvernance, jalons de décision et demandes de changement",
  "p.plan_modules.module_roadmap": "Feuille de route publique du produit",
  "p.plan_modules.none_chosen":
    "Sans outil choisi, la page du plan affiche sa fiche et aucun outil.",
  "p.plan_modules.save": "Enregistrer les outils",
  "p.plan_modules.presets_heading": "Choix prêts à l’emploi",
  "p.plan_modules.personal_is":
    "L’ensemble personnel, pour le travail d’une seule personne : {names}.",
  "p.plan_modules.use_personal": "Utiliser l’ensemble personnel",
  "p.plan_modules.default_is":
    "Le choix par défaut de l’installation : {names}.",
  "p.plan_modules.use_default": "Suivre le choix par défaut",
  "p.plan_modules.no_tools": "aucun outil",
  "p.plan_modules.unchanged":
    "Rien à enregistrer : ce plan suit déjà le choix par défaut, qui est celui-ci.",
  "p.plan_modules.saved": "Enregistré.",
  "p.plan_modules.could_not_load":
    "Les outils de ce plan n’ont pas pu être chargés.",
  "p.plan_modules.could_not_save": "La modification n’a pas été enregistrée.",
  "p.canvas.title": "Canevas de stratégie",
  "p.canvas.title_ppm": "Canevas de stratégie — PPM",
  "p.canvas.intro":
    "Un canevas est un tableau d'une page fait de cases, comme un Lean Canvas ou une analyse SWOT. Un élément peut devenir une idée, un objectif ou, sur le canevas d'un plan, un risque.",
  "p.canvas.plan_title": "Canevas de ce plan",
  "p.canvas.plan_title_ppm": "Canevas de ce plan — PPM",
  "p.canvas.plan_intro":
    "Les canevas de stratégie de ce plan. Un élément de l'un d'eux peut devenir un risque du registre de ce plan.",
  "p.canvas.one_title": "Canevas de stratégie",
  "p.canvas.one_title_ppm": "Canevas de stratégie — PPM",
  "p.canvas.back": "← Retour aux canevas",
  "p.canvas.list_heading": "Canevas",
  "p.canvas.filter": "Afficher",
  "p.canvas.filter_all": "Tous les types",
  "p.canvas.none_yet": "Il n'y a pas encore de canevas.",
  "p.canvas.none_of_kind": "Il n'y a pas de canevas de ce type.",
  "p.canvas.free_standing": "sans plan",
  "p.canvas.on_plan": "sur un plan",
  "p.canvas.new_heading": "Commencer un canevas",
  "p.canvas.new_on_plan": "Le nouveau canevas est sur ce plan.",
  "p.canvas.new_free_standing":
    "Le nouveau canevas n'est sur aucun plan : ses éléments peuvent devenir des idées ou des objectifs, mais pas des risques.",
  "p.canvas.f_template": "Type de canevas",
  "p.canvas.f_title": "Titre",
  "p.canvas.f_description": "Description (facultative)",
  "p.canvas.create": "Commencer le canevas",
  "p.canvas.could_not_load_list": "Les canevas n'ont pas pu être chargés.",
  "p.canvas.could_not_load_templates":
    "Les types de canevas n'ont pas pu être chargés ; un nouveau canevas ne peut donc pas être commencé.",
  "p.canvas.could_not_create": "Le canevas n'a pas été commencé.",
  "p.canvas.problem_title_blank": "Donnez un titre au canevas.",
  "p.canvas.problem_title_long":
    "Un titre peut compter au plus 200 caractères.",
  "p.canvas.problem_description_long":
    "Une description peut compter au plus 2 000 caractères.",
  "p.canvas.template_lean": "Canevas Lean",
  "p.canvas.template_business_model": "Canevas de modèle d'affaires",
  "p.canvas.template_swot": "Analyse SWOT",
  "p.canvas.template_empathy": "Carte d'empathie",
  "p.canvas.template_logic_model": "Modèle logique",
  "p.canvas.template_risk": "Canevas des risques",
  "p.canvas.template_goal": "Canevas d'objectif",
  "p.canvas.box_problem": "Problème",
  "p.canvas.box_solution": "Solution proposée",
  "p.canvas.box_key_metrics": "Indicateurs clés",
  "p.canvas.box_unique_value_proposition": "Proposition de valeur unique",
  "p.canvas.box_unfair_advantage": "Avantage déloyal",
  "p.canvas.box_channels": "Canaux",
  "p.canvas.box_customer_segments": "Segments de clientèle",
  "p.canvas.box_cost_structure": "Structure des coûts",
  "p.canvas.box_revenue_streams": "Sources de revenus",
  "p.canvas.box_key_partners": "Partenaires clés",
  "p.canvas.box_key_activities": "Activités clés",
  "p.canvas.box_key_resources": "Ressources clés",
  "p.canvas.box_value_propositions": "Propositions de valeur",
  "p.canvas.box_customer_relationships": "Relations avec les clients",
  "p.canvas.box_strengths": "Forces",
  "p.canvas.box_weaknesses": "Faiblesses",
  "p.canvas.box_opportunities": "Opportunités",
  "p.canvas.box_threats": "Menaces",
  "p.canvas.box_says": "Dit",
  "p.canvas.box_thinks": "Pense",
  "p.canvas.box_does": "Fait",
  "p.canvas.box_feels": "Ressent",
  "p.canvas.box_pains": "Douleurs",
  "p.canvas.box_gains": "Gains attendus",
  "p.canvas.box_inputs": "Ressources engagées",
  "p.canvas.box_activities": "Activités",
  "p.canvas.box_outputs": "Réalisations",
  "p.canvas.box_outcomes": "Résultats",
  "p.canvas.box_impact": "Impact à long terme",
  "p.canvas.box_vulnerabilities": "Vulnérabilités",
  "p.canvas.box_existing_controls": "Contrôles existants",
  "p.canvas.box_residual_risks": "Risques résiduels",
  "p.canvas.box_goal": "Objectif",
  "p.canvas.box_success_measures": "Mesures de réussite",
  "p.canvas.box_actions": "Actions à mener",
  "p.canvas.box_obstacles": "Obstacles à lever",
  "p.canvas.box_resources": "Ressources",
  "p.canvas.box_empty": "Rien ici pour l'instant.",
  "p.canvas.f_new_item": "Nouvel élément dans {box}",
  "p.canvas.add": "Ajouter",
  "p.canvas.added": "Ajouté.",
  "p.canvas.f_text": "Texte",
  "p.canvas.f_box": "Case",
  "p.canvas.save": "Enregistrer",
  "p.canvas.cancel": "Annuler",
  "p.canvas.saved": "Enregistré.",
  "p.canvas.unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.canvas.edit": "Modifier",
  "p.canvas.remove": "Retirer",
  "p.canvas.removed": "Retiré.",
  "p.canvas.confirm_remove_item": "Retirer « {text} » du canevas ?",
  "p.canvas.promote": "Transformer en…",
  "p.canvas.promote_legend": "Transformer cet élément en",
  "p.canvas.target_idea": "une idée",
  "p.canvas.target_objective": "un objectif",
  "p.canvas.target_risk": "un risque de ce plan",
  "p.canvas.promoted": "Transformé en {kind}.",
  "p.canvas.promote_once":
    "Un élément ne peut être transformé qu'une seule fois.",
  "p.canvas.promote_go": "Le transformer ainsi",
  "p.canvas.no_risk_free_standing":
    "Ce canevas n'est sur aucun plan : un élément ne peut donc pas devenir un risque.",
  "p.canvas.f_probability": "Probabilité (1 à 5)",
  "p.canvas.f_impact": "Impact (1 à 5)",
  "p.canvas.problem_risk_scores":
    "Un risque a besoin d'une probabilité et d'un impact, chacun un nombre entier de 1 à 5.",
  "p.canvas.problem_text_blank": "Écrivez d'abord le texte de l'élément.",
  "p.canvas.problem_text_long":
    "Un élément peut compter au plus 2 000 caractères.",
  "p.canvas.became_idea": "Devenu une idée",
  "p.canvas.became_risk": "Devenu un risque de ce plan",
  "p.canvas.became_objective": "Devenu un objectif",
  "p.canvas.owner": "Responsable : {owner}",
  "p.canvas.full":
    "Ce canevas contient le nombre maximal d'éléments ({max}) ; retirez-en un pour en ajouter un autre.",
  "p.canvas.could_not_load": "Ce canevas n'a pas pu être chargé.",
  "p.canvas.could_not_add": "L'élément n'a pas été ajouté.",
  "p.canvas.could_not_save": "La modification n'a pas été enregistrée.",
  "p.canvas.could_not_remove": "L'élément n'a pas été retiré.",
  "p.canvas.could_not_promote": "L'élément n'a été transformé en rien.",
  "p.canvas.remove_canvas_heading": "Retirer le canevas",
  "p.canvas.remove_canvas_says":
    "Retirer le canevas retire ses éléments. Les idées, objectifs et risques que ses éléments sont devenus restent.",
  "p.canvas.remove_canvas": "Retirer ce canevas",
  "p.canvas.confirm_remove_canvas":
    "Retirer le canevas « {title} » et ses éléments ?",
  "p.canvas.could_not_remove_canvas": "Le canevas n'a pas été retiré.",
  "p.plan_templates.title": "Modèles de plan",
  "p.plan_templates.title_ppm": "Modèles de plan — PPM",
  "p.plan_templates.intro":
    "Un modèle est un plan et les plans qui sont sous lui, gardés sans personnes et avec chaque date en nombre de jours après le début, pour que de nouveaux plans puissent en partir.",
  "p.plan_templates.saved_heading": "Modèles enregistrés",
  "p.plan_templates.starters_heading": "Modèles de départ fournis",
  "p.plan_templates.none_saved":
    "Aucun modèle n'a encore été enregistré. Enregistrez-en un depuis la page d'un plan.",
  "p.plan_templates.none_starters": "Il n'y a pas de modèle de départ fourni.",
  "p.plan_templates.plans_count_one": "{count} plan inclus",
  "p.plan_templates.plans_count_other": "{count} plans inclus",
  "p.plan_templates.needs_start": "demande une date de début",
  "p.plan_templates.could_not_load_list":
    "Les modèles n'ont pas pu être chargés.",
  "p.plan_templates.one_title": "Modèle de plan",
  "p.plan_templates.one_title_ppm": "Modèle de plan — PPM",
  "p.plan_templates.back": "← Retour aux modèles",
  "p.plan_templates.is_starter":
    "Un modèle de départ fourni. Il ne peut être ni modifié ni retiré ; commencez un plan à partir de lui et enregistrez-le sous votre propre nom.",
  "p.plan_templates.is_saved": "Un modèle enregistré, version {version}.",
  "p.plan_templates.holds_heading": "Ce qu'il contient",
  "p.plan_templates.holds_says":
    "Chaque plan, le nombre de ses tâches, et ses jalons en jours après le début. Aucune personne n'est gardée.",
  "p.plan_templates.tasks_count_one": "{count} tâche",
  "p.plan_templates.tasks_count_other": "{count} tâches",
  "p.plan_templates.milestone_at": "{name} : jour {days}",
  "p.plan_templates.start_heading": "Commencer des plans à partir de lui",
  "p.plan_templates.start_says":
    "Les nouveaux plans sont créés à partir du modèle, les dates comptées depuis la date de début. Les tâches commencent à leur première étape.",
  "p.plan_templates.f_plan_name": "Nom du plan du haut (facultatif)",
  "p.plan_templates.f_start": "Date de début",
  "p.plan_templates.no_dates":
    "Ce modèle ne contient aucune date : aucune date de début n'est nécessaire.",
  "p.plan_templates.f_parent": "Le placer sous",
  "p.plan_templates.no_parent":
    "Aucun plan : un nouveau plan de premier niveau",
  "p.plan_templates.start": "Commencer les plans",
  "p.plan_templates.started":
    "Fait. Plans : {plans} ; tâches : {tasks} ; jalons : {milestones}.",
  "p.plan_templates.open_new": "Ouvrir le nouveau plan",
  "p.plan_templates.could_not_load": "Ce modèle n'a pas pu être chargé.",
  "p.plan_templates.could_not_start": "Les plans n'ont pas été commencés.",
  "p.plan_templates.problem_plan_name_long":
    "Le nom d'un plan peut compter au plus 1 024 caractères.",
  "p.plan_templates.problem_start_needed":
    "Choisissez une date de début : ce modèle contient des dates.",
  "p.plan_templates.problem_start_bad":
    "Écrivez la date de début comme un jour, par exemple 2027-01-04.",
  "p.plan_templates.remove_heading": "Retirer le modèle",
  "p.plan_templates.remove_says":
    "Retirer le modèle ne change pas les plans déjà créés à partir de lui.",
  "p.plan_templates.remove": "Retirer ce modèle",
  "p.plan_templates.confirm_remove": "Retirer le modèle « {name} » ?",
  "p.plan_templates.could_not_remove": "Le modèle n'a pas été retiré.",
  "p.plan_templates.save_title": "Enregistrer comme modèle",
  "p.plan_templates.save_title_ppm": "Enregistrer comme modèle — PPM",
  "p.plan_templates.save_says":
    "Garde ce plan et les plans sous lui que vous pouvez lire, avec leurs tâches, composants, jalons et flux de travail. Aucune personne n'est gardée, et chaque date devient un nombre de jours après le début. Modifier le plan ensuite ne change pas le modèle.",
  "p.plan_templates.f_name": "Nom du modèle",
  "p.plan_templates.f_description": "Description (facultative)",
  "p.plan_templates.save": "Enregistrer le modèle",
  "p.plan_templates.saved":
    "Enregistré. Plans : {plans} ; tâches : {tasks} ; jalons : {milestones}.",
  "p.plan_templates.omitted_one":
    "{count} plan sous celui-ci a été laissé de côté, car vous ne pouvez pas le lire.",
  "p.plan_templates.omitted_other":
    "{count} plans sous celui-ci ont été laissés de côté, car vous ne pouvez pas les lire.",
  "p.plan_templates.open_template": "Ouvrir le modèle",
  "p.plan_templates.name_taken":
    "Un modèle porte déjà ce nom. Choisissez-en un autre.",
  "p.plan_templates.could_not_save": "Le modèle n'a pas été enregistré.",
  "p.plan_templates.problem_name_blank": "Donnez un nom au modèle.",
  "p.plan_templates.problem_name_long":
    "Le nom d'un modèle peut compter au plus 120 caractères.",
  "p.plan_export.title": "Exporter vers Microsoft Project",
  "p.plan_export.title_ppm": "Exporter vers Microsoft Project — PPM",
  "p.plan_export.intro":
    "Enregistre les tâches de ce plan dans un fichier XML MSPDI, le format qu'ouvrent Microsoft Project et d'autres outils de planification.",
  "p.plan_export.carried_heading": "Ce que contient le fichier",
  "p.plan_export.carried":
    "Chaque tâche avec son plan hiérarchique sous sa tâche parente, sa date de début, sa durée en jours ouvrés, ses jalons, contraintes, échéance et notes ; si elle est terminée ; les liens entre tâches avec leur type et leur décalage ; et le calendrier de travail du plan.",
  "p.plan_export.not_carried_heading": "Ce qu'il laisse de côté",
  "p.plan_export.not_carried":
    "Les personnes affectées et les ressources ; tout statut autre que terminé (une tâche est à 0 ou 100 pour cent) ; les sprints et composants ; l'effort ; les champs personnalisés ; les commentaires ; les références ; les liens entre tâches autres que « précède » ; et les décalages en temps écoulé (tout décalage est en jours ouvrés). Le fichier le dit aussi.",
  "p.plan_export.limit":
    "Un plan de plus de 5 000 tâches ne peut pas être exporté.",
  "p.plan_export.not_checked":
    "Le fichier a été relu par l'importateur de ce service, mais pas encore ouvert dans Microsoft Project lui-même.",
  "p.plan_export.download": "Télécharger le fichier MSPDI",
  "p.plan_export.to_import": "Importer plutôt un fichier MSPDI dans ce plan",
  "p.confidence.assess_heading": "Ajouter une évaluation",
  "p.confidence.assess_says":
    "Une valeur déclarée est l'avis d'une personne sur la capacité du plan à livrer. Elle s'ajoute à l'historique et ne remplace jamais une précédente.",
  "p.confidence.choose_value": "Choisissez une valeur",
  "p.confidence.could_not_load":
    "La confiance de livraison de ce plan n'a pas pu être chargée.",
  "p.confidence.could_not_save": "L'évaluation n'a pas été ajoutée.",
  "p.confidence.f_assessed_on": "Évaluée le (facultatif ; aujourd'hui si vide)",
  "p.confidence.f_kind": "Qui le dit",
  "p.confidence.f_rationale": "Pourquoi (au moins {min} caractères)",
  "p.confidence.f_value": "Confiance de livraison",
  "p.confidence.history_heading": "Historique, du plus récent au plus ancien",
  "p.confidence.intro":
    "L'avis déclaré d'une personne sur la capacité de ce plan à livrer, gardé à côté du RAG calculé à partir des données du plan et jamais mélangé avec lui.",
  "p.confidence.kind_independent": "Une évaluation indépendante",
  "p.confidence.kind_self": "L'équipe du plan elle-même (auto-évaluation)",
  "p.confidence.no_history": "Aucune évaluation n'a encore été déclarée.",
  "p.confidence.none_yet":
    "Non évalué : personne n'a encore donné d'avis. Ce n'est pas la même chose que vert.",
  "p.confidence.opens_exception":
    "Ambre/rouge ou rouge ouvre une exception pour ce plan, ou aggrave celle déjà ouverte. Quelqu'un doit ensuite la compléter.",
  "p.confidence.problem_future":
    "La date d'évaluation ne peut pas être dans le futur.",
  "p.confidence.problem_kind": "Choisissez qui le dit.",
  "p.confidence.problem_rationale":
    "Dites pourquoi, en au moins {min} caractères.",
  "p.confidence.problem_value": "Choisissez une valeur.",
  "p.confidence.refused_independent":
    "Une évaluation indépendante demande le droit d'évaluateur indépendant. Vous pouvez ajouter une auto-évaluation.",
  "p.confidence.save": "Ajouter l'évaluation",
  "p.confidence.saved_exception_one":
    "Ajoutée. Ce plan a maintenant une exception ouverte : elle est à compléter sous {hours} heure.",
  "p.confidence.saved_exception_other":
    "Ajoutée. Ce plan a maintenant une exception ouverte : elle est à compléter sous {hours} heures.",
  "p.confidence.saved": "Ajoutée.",
  "p.confidence.shown_heading": "Ce que montre la carte du portefeuille",
  "p.confidence.shown_independent": "Évaluation indépendante du {when}.",
  "p.confidence.shown_self":
    "Auto-évaluation du {when} ; une évaluation indépendante serait montrée à la place.",
  "p.confidence.stale": "périmée : antérieure à ce mois",
  "p.confidence.title_ppm": "Confiance de livraison — PPM",
  "p.confidence.title": "Confiance de livraison",
  "p.exceptions.a_plan": "Un plan",
  "p.exceptions.breached_one":
    "Dépassée : pas complétée dans le délai de {hours} heure après son ouverture.",
  "p.exceptions.breached_other":
    "Dépassée : pas complétée dans les {hours} heures après son ouverture.",
  "p.exceptions.cancel": "Annuler",
  "p.thresholds.could_not_load":
    "Le triage et les seuils n'ont pas pu être chargés.",
  "p.thresholds.could_not_save": "La modification n'a pas été enregistrée.",
  "p.thresholds.could_not_sweep": "Les seuils n'ont pas pu être vérifiés.",
  "p.thresholds.decide_says":
    "Enregistrer la décision clôt l'exception. Elle consigne ce qui a été décidé ; elle ne met pas en pause, ne réinitialise ni n'arrête le programme, ce que fait une personne par le statut du programme.",
  "p.thresholds.decide": "Enregistrer la décision",
  "p.thresholds.decided_done": "Décision enregistrée : {action}.",
  "p.thresholds.f_action": "Décision",
  "p.thresholds.f_amount": "Montant du plafond",
  "p.thresholds.f_count":
    "Dossiers publiés au rouge d'affilée (de {min} à {max})",
  "p.thresholds.f_currency": "Devise (trois lettres)",
  "p.thresholds.f_date": "Date de fin prévue à ne pas dépasser",
  "p.thresholds.f_expiry":
    "Dérogation jusqu'au (dans le futur, au plus {days} jours)",
  "p.thresholds.f_outcome": "Issue",
  "p.thresholds.f_reason": "Motif (au moins {min} caractères)",
  "p.thresholds.history_item": "{date} : {outcome}. {reason}",
  "p.thresholds.history": "Triages précédents",
  "p.thresholds.not_set": "Non défini",
  "p.thresholds.now_cap": "Franchi quand le coût P80 dépasse {amount}.",
  "p.thresholds.now_forecast":
    "Franchi quand la fin prévue est après le {date}.",
  "p.thresholds.now_red_packs_one":
    "Franchi quand la confiance est au rouge sur {n} dossier publié d'affilée.",
  "p.thresholds.now_red_packs_other":
    "Franchi quand la confiance est au rouge sur {n} dossiers publiés d'affilée.",
  "p.thresholds.opened_by_threshold":
    "Ouverte parce qu'un seuil du programme a été franchi.",
  "p.thresholds.problem_action":
    "Choisissez pause, réinitialisation, arrêt ou dérogation.",
  "p.thresholds.problem_cap":
    "Le plafond doit être un montant supérieur à zéro.",
  "p.thresholds.problem_count":
    "Le nombre de dossiers doit être un entier de {min} à {max}.",
  "p.thresholds.problem_currency":
    "La devise doit être un code ISO 4217 de trois lettres.",
  "p.thresholds.problem_date": "Choisissez une date.",
  "p.thresholds.problem_expiry_long":
    "Une dérogation peut durer au plus {days} jours.",
  "p.thresholds.problem_expiry_needed":
    "Une dérogation demande une date jusqu'à laquelle elle vaut.",
  "p.thresholds.problem_expiry_past":
    "La date d'une dérogation doit être dans le futur.",
  "p.thresholds.problem_outcome":
    "Choisissez terminer, corriger, mettre en pause ou arrêter.",
  "p.thresholds.problem_reason":
    "Le motif doit compter au moins {min} caractères.",
  "p.thresholds.remove_threshold": "Supprimer le seuil",
  "p.thresholds.save_threshold": "Enregistrer le seuil",
  "p.thresholds.save_triage": "Enregistrer le triage",
  "p.thresholds.sweep_done_one":
    "{checked} seuils vérifiés ; {n} exception ouverte.",
  "p.thresholds.sweep_done_other":
    "{checked} seuils vérifiés ; {n} exceptions ouvertes.",
  "p.thresholds.sweep_item": "{programme} — {kind} : {outcome}",
  "p.thresholds.sweep_list": "Seuils franchis",
  "p.thresholds.sweep_says":
    "La vérification ouvre une exception de livraison pour chaque seuil franchi, ou l'ajoute à celle déjà ouverte ; elle ne met rien en pause, ne réinitialise ni n'arrête rien.",
  "p.thresholds.sweep": "Vérifier les seuils maintenant",
  "p.thresholds.threshold_removed": "Seuil supprimé : {kind}.",
  "p.thresholds.threshold_saved": "Seuil enregistré : {kind}.",
  "p.thresholds.thresholds_heading": "Seuils",
  "p.thresholds.thresholds_says":
    "Franchir un seuil ouvre une exception de livraison, à laquelle une personne répond par une pause, une réinitialisation, un arrêt ou une dérogation datée. Rien n'est mis en pause, réinitialisé ni arrêté automatiquement.",
  "p.thresholds.triage_heading": "Triage du programme",
  "p.thresholds.triage_latest": "Dernier triage, {date} : {outcome}. {reason}",
  "p.thresholds.triage_none": "Aucun triage enregistré pour l'instant.",
  "p.thresholds.triage_saved": "Triage enregistré.",
  "p.thresholds.triage_says":
    "Un triage est un avis daté sur la direction du programme, avec son motif. Il consigne l'avis ; il ne change rien au programme.",
  "p.thresholds.unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.thresholds.action_pause": "Mettre en pause",
  "p.thresholds.action_reset": "Réinitialiser",
  "p.thresholds.action_stop": "Arrêter",
  "p.thresholds.action_override": "Déroger",
  "p.thresholds.kind_confidence_red_packs":
    "Confiance au rouge sur plusieurs dossiers",
  "p.thresholds.kind_forecast_past": "Fin prévue après une date",
  "p.thresholds.kind_p80_cap": "Coût P80 au-dessus d'un plafond",
  "p.thresholds.outcome_finish": "Terminer",
  "p.thresholds.outcome_fix": "Corriger",
  "p.thresholds.outcome_pause": "Mettre en pause",
  "p.thresholds.outcome_stop": "Arrêter",
  "p.thresholds.sweep_opened": "exception ouverte",
  "p.thresholds.sweep_already_open": "exception déjà ouverte",
  "p.thresholds.sweep_attached_to_open_exception":
    "ajouté à l'exception ouverte",
  "p.thresholds.sweep_suppressed_by_override":
    "non ouverte : une dérogation est en vigueur",
  "p.exceptions.closed_done": "Décision enregistrée ; l'exception est fermée.",
  "p.exceptions.complete_it": "La compléter",
  "p.exceptions.complete_says":
    "Une exception est complète quand sa cause, la décision nécessaire, un responsable et une date de décision sont tous là. Une partie laissée vide garde ce qu'elle avait.",
  "p.exceptions.complete":
    "Complète : elle a sa cause, la décision nécessaire, un responsable et une date de décision.",
  "p.exceptions.conflict_closed": "Cette exception est déjà fermée.",
  "p.exceptions.could_not_load": "Les exceptions n'ont pas pu être chargées.",
  "p.exceptions.could_not_save": "La modification n'a pas été enregistrée.",
  "p.exceptions.decide_it": "Enregistrer la décision",
  "p.exceptions.decide_says":
    "Enregistrer la décision ferme l'exception. Une meilleure confiance ne la ferme jamais d'elle-même.",
  "p.exceptions.decide": "Enregistrer la décision et fermer",
  "p.exceptions.f_breached": "Dépassées seulement",
  "p.exceptions.f_decision": "Décision (au moins {min} caractères)",
  "p.exceptions.f_owner_ref": "Responsable (une référence de personne)",
  "p.exceptions.f_owner_role": "Rôle du responsable (ce qu'un dossier montre)",
  "p.exceptions.f_status": "Afficher",
  "p.exceptions.intro_one":
    "Une exception s'ouvre quand la confiance de livraison d'un plan atteint ambre/rouge ou rouge. Elle est à compléter sous {hours} heure et n'est fermée que par une décision enregistrée.",
  "p.exceptions.intro_other":
    "Une exception s'ouvre quand la confiance de livraison d'un plan atteint ambre/rouge ou rouge. Elle est à compléter sous {hours} heures et n'est fermée que par une décision enregistrée.",
  "p.exceptions.is_closed": "fermée le {when}",
  "p.exceptions.is_open": "ouverte",
  "p.exceptions.missing": "Il manque encore : {parts}.",
  "p.exceptions.none": "Aucune exception ne correspond.",
  "p.exceptions.opened": "Ouverte le {when}",
  "p.exceptions.part_cause": "Origine",
  "p.exceptions.part_decide_by": "Décider avant le",
  "p.exceptions.part_decision_needed": "Décision nécessaire",
  "p.exceptions.part_decision": "Décision",
  "p.exceptions.part_owner": "Responsable",
  "p.exceptions.problem_decision":
    "Écrivez la décision en au moins {min} caractères.",
  "p.exceptions.problem_part_short":
    "Une cause, une décision nécessaire ou un responsable demande au moins {min} caractères pour compter.",
  "p.exceptions.refused_threshold":
    "Cette exception a été ouverte par un seuil : elle se ferme par une pause, une remise à zéro, un arrêt ou une dérogation enregistrée, pas par une décision ici.",
  "p.exceptions.save": "Enregistrer",
  "p.exceptions.saved": "Enregistré.",
  "p.exceptions.see_register": "Ouvrir le registre des exceptions",
  "p.exceptions.status_all": "Toutes les exceptions",
  "p.exceptions.status_closed": "Fermées",
  "p.exceptions.status_open": "Ouvertes",
  "p.exceptions.title_ppm": "Exceptions de livraison — PPM",
  "p.exceptions.title": "Exceptions de livraison",
  "p.exceptions.unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.exceptions.worsened_one": "Aggravée {count} fois depuis son ouverture",
  "p.exceptions.worsened_other": "Aggravée {count} fois depuis son ouverture",
  "p.exceptions.worst": "Pire confiance : {value}",
  "p.aims.aims_heading": "Objectifs stratégiques",
  "p.aims.aims_legend": "Objectifs qui sont des objectifs stratégiques",
  "p.aims.aims_says":
    "Un objectif stratégique est un objectif autour duquel le portefeuille est organisé. Le retirer cesse aussi d'en faire l'objectif principal d'un programme.",
  "p.aims.choose_aim": "Choisissez un objectif",
  "p.aims.choose_programme": "Choisissez un programme",
  "p.aims.could_not_load": "La carte du portefeuille n'a pas pu être chargée.",
  "p.aims.could_not_save": "La modification n'a pas été enregistrée.",
  "p.aims.f_aim": "Objectif principal",
  "p.aims.f_period": "Mois (vide pour ce mois)",
  "p.aims.f_programme": "Programme concerné",
  "p.aims.intro":
    "Les programmes que vous pouvez lire, chacun une fois sous son objectif stratégique principal. La confiance de livraison est la valeur déclarée, montrée à côté du RAG calculé à partir des données, jamais mélangée avec lui ; non évalué n'est pas vert.",
  "p.aims.map_heading": "Carte du portefeuille, {period}",
  "p.aims.marked": "Marqué comme objectif stratégique.",
  "p.aims.no_aims":
    "Aucun objectif n'est encore stratégique ; marquez-en un ci-dessus d'abord.",
  "p.aims.no_objectives": "Il n'y a pas encore d'objectifs.",
  "p.aims.no_programmes":
    "Il n'y a aucun programme sur la carte : un programme est un plan de type Programme.",
  "p.aims.primary_heading": "L'objectif principal d'un programme",
  "p.aims.primary_says":
    "Un programme apparaît sur la carte sous son seul objectif principal ; les autres objectifs qu'il sert sont nommés à côté. Choisir un nouvel objectif principal fait du précédent un lien ordinaire.",
  "p.aims.primary_set": "Objectif principal défini.",
  "p.aims.problem_already_primary":
    "C'est déjà l'objectif principal de ce programme.",
  "p.aims.problem_choose": "Choisissez un programme et un objectif.",
  "p.aims.problem_period":
    "Choisissez le mois sous la forme AAAA-MM, ou laissez vide pour ce mois.",
  "p.aims.set_primary": "Définir l'objectif principal",
  "p.aims.show": "Afficher",
  "p.aims.title_ppm": "Carte du portefeuille — PPM",
  "p.aims.title": "Carte du portefeuille",
  "p.aims.to_objectives": "Ouvrir les objectifs",
  "p.aims.unmarked": "N'est plus un objectif stratégique.",
  "p.programme.cited_exception":
    "Elle découle d'une exception de livraison de ce programme.",
  "p.programme.cited_review":
    "Elle découle d'une revue de jalon de ce programme.",
  "p.programme.cites_none": "Aucune",
  "p.programme.conflict_status":
    "Le programme a déjà ce statut, ou il est arrêté et le reste.",
  "p.programme.could_not_decide": "La décision n'a pas été enregistrée.",
  "p.programme.could_not_load":
    "Les jalons et le statut de ce plan n'ont pas pu être chargés.",
  "p.programme.could_not_save_gates":
    "Toutes les dates de jalon n'ont pas été enregistrées ; celles affichées sont celles du service.",
  "p.programme.decide": "Enregistrer la décision",
  "p.programme.decided": "Enregistrée : le programme est maintenant {status}.",
  "p.programme.do_pause": "Mettre le programme en pause",
  "p.programme.do_resume": "Reprendre le programme",
  "p.programme.do_stop": "Arrêter le programme",
  "p.programme.f_change": "Passer le programme à",
  "p.programme.f_exception": "Elle découle d'une exception (facultatif)",
  "p.programme.f_reason": "Motif (au moins {min} caractères)",
  "p.programme.f_review": "Elle découle d'une revue de jalon (facultatif)",
  "p.programme.gates_heading": "Dates de jalon prévues",
  "p.programme.gates_says":
    "La date à laquelle ce plan attend sa revue à chaque jalon. Une date fixée ne peut pas être effacée, seulement déplacée.",
  "p.programme.gates_saved": "Dates de jalon enregistrées.",
  "p.programme.gates_unchanged":
    "Aucune date de jalon n'a changé ; rien n'a été envoyé.",
  "p.programme.held_allocations_one":
    "{count} affectation est signalée, pas supprimée.",
  "p.programme.held_allocations_other":
    "{count} affectations sont signalées, pas supprimées.",
  "p.programme.held_benefits_one": "{count} bénéfice reste enregistré.",
  "p.programme.held_benefits_other": "{count} bénéfices restent enregistrés.",
  "p.programme.held_heading": "Ce que le programme détient encore",
  "p.programme.held_money": "Argent : prévu {planned}, réel {actual}.",
  "p.programme.history_heading":
    "Décisions, de la plus récente à la plus ancienne",
  "p.programme.no_history":
    "Aucune décision pour l'instant : le programme est actif.",
  "p.programme.not_a_programme":
    "Seul un plan de type Programme a un statut de programme. Ses dates de jalon peuvent quand même être prévues ci-dessus.",
  "p.programme.problem_reason": "Donnez le motif en au moins {min} caractères.",
  "p.programme.problem_status": "Choisissez le changement.",
  "p.programme.refused_decider":
    "Mettre en pause, reprendre ou arrêter un programme demande le droit de décideur de programme.",
  "p.programme.refused_not_programme":
    "Seul un plan de type Programme a un statut, et la revue ou l'exception citée doit être celle de ce programme.",
  "p.programme.save_gates": "Enregistrer les dates de jalon",
  "p.programme.status_heading": "Statut du programme",
  "p.programme.status_is": "Statut : {status}",
  "p.programme.status_says":
    "Un programme est actif jusqu'à ce qu'une décision enregistrée le mette en pause ou l'arrête. Une pause signale ses affectations et n'en supprime aucune.",
  "p.programme.stop_warns":
    "L'arrêt est définitif : le programme reste arrêté et garde son historique.",
  "p.programme.stopped_final":
    "Ce programme est arrêté. C'est définitif ; son historique est gardé.",
  "p.programme.title_ppm": "Jalons et statut du programme — PPM",
  "p.programme.title": "Jalons et statut du programme",
  "p.programme.buffer_by_days": "Un nombre de jours ouvrés",
  "p.programme.buffer_by_rule":
    "La moitié du travail critique de la référence, arrondie au supérieur",
  "p.programme.buffer_declared_one":
    "Déclaré : un tampon de {days} jour ouvré.",
  "p.programme.buffer_declared_other":
    "Déclaré : un tampon de {days} jours ouvrés.",
  "p.programme.buffer_heading": "Tampon du projet",
  "p.programme.buffer_says":
    "Le temps gardé en réserve à la fin du programme pour absorber les retards. Le graphique met la part consommée en regard de la part du travail critique achevée. Une déclaration ne remplace jamais une précédente ; la plus récente est le tampon. Un plan de tampons de chaîne critique, s'il existe, est utilisé à la place.",
  "p.programme.buffer_history":
    "Déclarations, de la plus récente à la plus ancienne",
  "p.programme.buffer_was_days_one": "{days} jour ouvré, déclaré",
  "p.programme.buffer_was_days_other": "{days} jours ouvrés, déclarés",
  "p.programme.buffer_was_rule_one": "{days} jour ouvré, par la règle",
  "p.programme.buffer_was_rule_other": "{days} jours ouvrés, par la règle",
  "p.programme.could_not_declare_buffer": "Le tampon n'a pas été déclaré.",
  "p.programme.could_not_load_buffer":
    "Le tampon du projet n'a pas pu être lu.",
  "p.programme.declare_buffer": "Déclarer le tampon",
  "p.programme.f_buffer_days": "Jours ouvrés (1 à {max})",
  "p.programme.f_buffer_how": "Durée du tampon",
  "p.programme.f_buffer_note": "Note (facultative)",
  "p.programme.problem_buffer_days":
    "Écrivez le tampon comme un nombre entier de jours ouvrés de 1 à 2000.",
  "p.programme.refused_buffer_rule":
    "La règle demande une référence de planning avec du travail critique ayant une durée ; définissez d'abord une référence ou donnez un nombre de jours.",
  "p.programme.refused_not_programme_buffer":
    "Seul un plan de type Programme a un tampon de projet.",
  "p.programme.profile_heading": "Modèle de livraison et compte rendu",
  "p.programme.f_model": "Modèle de livraison",
  "p.programme.f_owner": "Organisation responsable",
  "p.programme.no_owner": "Aucune organisation responsable",
  "p.programme.model_says":
    "Le modèle de livraison dit qui construit et qui déploie ; le dossier du directeur l'affiche à côté du programme. Le modèle et l'organisation sont enregistrés ensemble.",
  "p.programme.save_model": "Enregistrer le modèle de livraison",
  "p.programme.model_saved": "Le modèle de livraison est enregistré.",
  "p.programme.f_method": "Comment le programme rend compte de l'avancement",
  "p.programme.method_evm":
    "Valeur acquise (performance des coûts et des délais)",
  "p.programme.method_product": "Burn-up et flux (un programme produit)",
  "p.programme.save_method": "Enregistrer la méthode de compte rendu",
  "p.programme.method_saved_evm":
    "Le programme rend désormais compte par valeur acquise.",
  "p.programme.method_saved_product":
    "Le programme rend désormais compte par burn-up et flux.",
  "p.programme.profile_unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.programme.could_not_load_profile":
    "Le modèle de livraison n'a pas pu être chargé.",
  "p.programme.could_not_save_profile":
    "La modification n'a pas été enregistrée.",
  "p.benefits.classified": "Classement enregistré.",
  "p.benefits.classify_says":
    "L'enregistrement envoie tout le classement : un choix laissé non indiqué est effacé.",
  "p.benefits.could_not_load":
    "Les bénéfices de ce plan n'ont pas pu être chargés.",
  "p.benefits.could_not_save": "Le classement n'a pas été enregistré.",
  "p.benefits.could_not_validate": "La validation n'a pas été enregistrée.",
  "p.benefits.f_annual": "Montant annuel ({currency})",
  "p.benefits.f_classification": "Classement du bénéfice",
  "p.benefits.f_kpi": "Indicateur qui le mesure",
  "p.benefits.f_note": "Note (facultative)",
  "p.benefits.f_recurrence": "Récurrence",
  "p.benefits.f_validated": "Montant validé à ce jour ({currency})",
  "p.benefits.intro":
    "Un montant réalisé est une déclaration. Il n'est validé que par une validation enregistrée d'une personne extérieure au programme ; chacune indique le montant validé à ce jour, et la plus récente compte.",
  "p.benefits.measured_by_kpi":
    "Un bénéfice non financier se mesure par son indicateur, il n'est pas validé comme un montant.",
  "p.benefits.money_line": "Cible {target} ; réalisé (déclaré) {realized}.",
  "p.benefits.no_kpi": "Aucun indicateur",
  "p.benefits.non_financial": "Un bénéfice non financier.",
  "p.benefits.none": "Ce plan n'a pas encore de bénéfice.",
  "p.benefits.problem_annual_amount":
    "Écrivez le montant annuel comme un nombre, pas en dessous de 0.",
  "p.benefits.problem_annual_financial":
    "Seul un bénéfice financier, avec une devise, a un montant annuel.",
  "p.benefits.problem_annual_recurring":
    "Seul un bénéfice récurrent a un montant annuel : choisissez d'abord récurrent.",
  "p.benefits.problem_validated_amount":
    "Écrivez le montant validé à ce jour comme un nombre, pas en dessous de 0.",
  "p.benefits.refused_member":
    "Un validateur ne peut pas être membre du programme : demandez à une personne extérieure de valider.",
  "p.benefits.save": "Enregistrer le classement",
  "p.benefits.standing_claimed": "Déclaré, pas encore validé",
  "p.benefits.standing_not_claimed": "Rien de déclaré pour l'instant",
  "p.benefits.standing_partly_validated": "Validé en partie",
  "p.benefits.standing_validated": "Validé",
  "p.benefits.title_ppm": "Bénéfices — PPM",
  "p.benefits.title": "Bénéfices : classement et validation",
  "p.benefits.validate": "Enregistrer la validation",
  "p.benefits.validated_done": "Validation enregistrée.",
  "p.benefits.validated_to_date": "{amount} validé à ce jour",
  "p.benefits.validation_heading": "Validation du bénéfice",
  "p.benefits.validation_says":
    "Les validations sont gardées telles qu'enregistrées ; une nouvelle indique le montant validé à ce jour et n'en remplace aucune.",
  "p.funding.add": "Ajouter la source",
  "p.funding.added": "Source de financement ajoutée.",
  "p.funding.choose_plan": "Choisissez un plan",
  "p.funding.could_not_load_lines":
    "Les lignes budgétaires de ce plan n'ont pas pu être chargées.",
  "p.funding.could_not_load_money":
    "L'argent par source de financement n'a pas pu être chargé.",
  "p.funding.could_not_load":
    "Les sources de financement n'ont pas pu être chargées.",
  "p.funding.could_not_save": "La modification n'a pas été enregistrée.",
  "p.funding.f_combine": "Regrouper investissement et fonctionnement",
  "p.funding.f_fy": "Exercice (l'année où il commence ; vide pour l'actuel)",
  "p.funding.f_name": "Nouvelle source de financement",
  "p.funding.f_plan": "Plan concerné",
  "p.funding.f_source": "Source de financement",
  "p.funding.intro":
    "D'où vient l'argent de chaque programme. Une ligne budgétaire sans source de financement est comptée et listée, jamais cachée.",
  "p.funding.is_active": "en service",
  "p.funding.is_retired": "retirée",
  "p.funding.line_cleared": "La ligne n'a plus de source de financement.",
  "p.funding.line_set": "La source de financement de la ligne est définie.",
  "p.funding.lines_heading": "Lignes budgétaires et leur source de financement",
  "p.funding.lines_says":
    "Choisissez un plan pour voir ses lignes budgétaires et donner une source de financement à chacune.",
  "p.funding.money_heading": "Argent par source de financement",
  "p.funding.no_lines": "Ce plan n'a pas de ligne budgétaire.",
  "p.funding.no_source": "Aucune source de financement",
  "p.funding.no_sources":
    "Aucune source de financement n'a encore été ajoutée.",
  "p.funding.problem_fy":
    "Écrivez l'exercice comme l'année où il commence, de 2000 à 2100.",
  "p.funding.problem_name_taken":
    "Une source de financement porte déjà ce nom.",
  "p.funding.problem_name":
    "Le nom d'une source de financement compte de 2 à 120 caractères.",
  "p.funding.refused_inactive":
    "Cette source de financement est retirée : elle ne peut pas être donnée à une ligne.",
  "p.funding.restore": "Rétablir",
  "p.funding.restored": "Source de financement rétablie.",
  "p.funding.retire_says":
    "Une source retirée ne peut plus être donnée à une ligne, mais les lignes qui l'ont la gardent.",
  "p.funding.retire": "Retirer",
  "p.funding.retired_option": "{name} (retirée)",
  "p.funding.retired": "Source de financement retirée.",
  "p.funding.show": "Afficher",
  "p.funding.sources_heading": "Sources de financement",
  "p.funding.title_ppm": "Financement — PPM",
  "p.funding.title": "Financement",
  "p.director_pack.actions_heading": "Relire, émettre ou réémettre",
  "p.director_pack.aud_board": "Conseil",
  "p.director_pack.aud_escalation_board": "Comité d'escalade",
  "p.director_pack.aud_executive": "Direction",
  "p.director_pack.aud_org": "Organisation : {name}",
  "p.director_pack.aud_partner": "Partenaires",
  "p.director_pack.choose_audience": "Choisissez un public",
  "p.director_pack.conflict_draft":
    "Cette période a déjà un dossier pour ce public. Ouvrez-le : émettez son brouillon, ou réémettez-le avec une note de modification.",
  "p.director_pack.conflict_issue":
    "Ce dossier ne peut pas être émis maintenant : il est déjà émis, ou c'est un brouillon planifié que le bureau du portefeuille n'a pas relu.",
  "p.director_pack.conflict_reissue":
    "Seule la dernière version émise est réémise, et une version plus récente existe déjà.",
  "p.director_pack.conflict_review":
    "Ce brouillon a déjà été relu, ou ce n'est plus un brouillon.",
  "p.director_pack.could_not_act": "La modification n'a pas été faite.",
  "p.director_pack.could_not_draft": "Le dossier n'a pas été rédigé.",
  "p.director_pack.cut_off_says":
    "Laissez vide pour utiliser maintenant. Le dossier lit les plans tels qu'ils étaient à la date d'arrêté ; elle ne peut être ni future ni antérieure de plus de 400 jours.",
  "p.director_pack.draft_heading": "Rédiger un dossier",
  "p.director_pack.draft_says":
    "La rédaction lit les plans que vous pouvez partager tels qu'ils sont à la date d'arrêté et garde le résultat comme brouillon. Rien n'est envoyé au public avant qu'une personne ne l'émette.",
  "p.director_pack.draft": "Rédiger le dossier",
  "p.director_pack.f_audience": "Public",
  "p.director_pack.f_change_note": "Note de modification",
  "p.director_pack.f_cut_off": "Date d'arrêté (facultative)",
  "p.director_pack.f_period": "Période (mois)",
  "p.director_pack.from_schedule": "fait par la planification mensuelle",
  "p.director_pack.is_reviewed": "Relu par le bureau du portefeuille.",
  "p.director_pack.is_stale":
    "périmé : sa date d'arrêté est avant la fin de sa période",
  "p.director_pack.issue_says":
    "L'émission fige le dossier et prévient son public. Elle ne peut pas être annulée ; une modification ultérieure est une réémission.",
  "p.director_pack.issue_waits":
    "La planification mensuelle a fait ce brouillon : le bureau du portefeuille le relit avant qu'il puisse être émis.",
  "p.director_pack.issue": "Émettre le dossier",
  "p.director_pack.issued_done": "Émis.",
  "p.director_pack.not_reviewed": "Pas encore relu.",
  "p.director_pack.problem_audience": "Choisissez un public.",
  "p.director_pack.problem_cut_off_bad":
    "Écrivez la date d'arrêté comme une date et une heure.",
  "p.director_pack.problem_cut_off_future":
    "La date d'arrêté ne peut pas être dans le futur.",
  "p.director_pack.problem_cut_off_old":
    "La date d'arrêté ne peut pas remonter à plus de 400 jours.",
  "p.director_pack.problem_note":
    "Dites pourquoi il y a une nouvelle version, en au moins {min} caractères.",
  "p.director_pack.problem_period": "Choisissez la période comme un mois.",
  "p.director_pack.refused_edit":
    "Rédiger, relire et réémettre un dossier demande l'accès d'éditeur des dossiers de direction.",
  "p.director_pack.refused_issue":
    "Émettre un dossier demande l'accès d'émetteur des dossiers de direction.",
  "p.director_pack.reissue_says":
    "La réémission rédige la version suivante avec votre note ; cette version reste lisible telle quelle. Écrivez au moins {min} caractères.",
  "p.director_pack.reissue": "Réémettre avec une note de modification",
  "p.director_pack.review": "Marquer comme relu",
  "p.director_pack.reviewed_done": "Marqué comme relu.",
  "p.director_pack.add_schedule_heading": "Ajouter une planification",
  "p.director_pack.add_schedule": "Ajouter la planification",
  "p.director_pack.conflict_schedule":
    "Ce public a déjà une planification ; changez plutôt son jour.",
  "p.director_pack.could_not_change_schedule":
    "La planification n'a pas été modifiée.",
  "p.director_pack.could_not_load_schedules":
    "Les planifications n'ont pas pu être chargées.",
  "p.director_pack.could_not_load_subscriptions":
    "Les dossiers dont vous êtes prévenu n'ont pas pu être chargés.",
  "p.director_pack.could_not_subscribe":
    "La modification n'a pas été enregistrée.",
  "p.director_pack.day_says":
    "Un jour de 1 à 28, pour que chaque mois l'ait. La première exécution est le prochain jour de ce type.",
  "p.director_pack.f_day": "Jour du mois",
  "p.director_pack.last_drafted":
    "Dernière exécution {when} : dossier de {period} rédigé.",
  "p.director_pack.last_run": "Dernière exécution {when}.",
  "p.director_pack.last_skipped":
    "Dernière exécution {when} : {period} ignoré, il avait déjà un dossier.",
  "p.director_pack.move_day": "Passer à ce jour",
  "p.director_pack.no_schedules":
    "Aucun dossier n'est encore rédigé selon une planification.",
  "p.director_pack.problem_day": "Choisissez un jour du mois de 1 à 28.",
  "p.director_pack.refused_subscribe":
    "Vous ne pouvez être prévenu que pour les publics dont vous pouvez lire les dossiers.",
  "p.director_pack.schedule_added": "Planification ajoutée.",
  "p.director_pack.schedule_off":
    "Désactivée. Elle rédigerait le {day} de chaque mois.",
  "p.director_pack.schedule_on":
    "Rédige le {day} de chaque mois ; prochaine fois le {next}.",
  "p.director_pack.schedule_saved": "Enregistré.",
  "p.director_pack.schedule_switched_off": "Désactivée.",
  "p.director_pack.schedule_switched_on": "Activée.",
  "p.director_pack.schedule_unchanged":
    "C'est déjà son jour : rien n'a été envoyé.",
  "p.director_pack.schedules_intro":
    "Chaque planification rédige le dossier d'un public un jour de chaque mois. Elle n'en émet jamais : le bureau du portefeuille relit le brouillon et une personne l'émet.",
  "p.director_pack.schedules_link": "Planifications mensuelles",
  "p.director_pack.schedules_title_ppm": "Planifications mensuelles — PPM",
  "p.director_pack.schedules_title": "Planifications mensuelles",
  "p.director_pack.subs_heading": "Me prévenir quand un dossier est émis",
  "p.director_pack.subs_legend": "Dossiers dont être prévenu",
  "p.director_pack.subs_says":
    "Vous êtes prévenu dans le service, avec un lien, quand un dossier pour un public choisi est émis. L'avis ne contient jamais le dossier lui-même.",
  "p.director_pack.subscribed": "Vous serez prévenu.",
  "p.director_pack.sweep_heading": "Exécuter ce qui est dû maintenant",
  "p.director_pack.sweep_result":
    "Rédigés : {drafted} ; ignorés car un dossier existait déjà : {skipped}.",
  "p.director_pack.sweep_says":
    "Rédige chaque dossier dont le jour est venu et qui n'existe pas encore. Rien n'est émis.",
  "p.director_pack.sweep": "Exécuter les planifications dues",
  "p.director_pack.switch_off": "Désactiver",
  "p.director_pack.switch_on": "Activer",
  "p.director_pack.unsubscribed": "Vous ne serez plus prévenu.",
  "p.plans.pid.modules_failed":
    "Les outils utilisés par ce plan n'ont pas pu être vérifiés ; tous les outils sont donc listés.",
  "p._layout_svelte.handling_mark": "Marque de traitement : {mark}",
  "p._layout_svelte.handling_mark_unknown":
    "Marque de traitement : vérification impossible. Considérez ce plan comme marqué tant qu'elle n'a pas abouti.",
  "l.enum.cr_status.draft": "brouillon",
  "l.enum.cr_status.submitted": "soumise",
  "l.enum.cr_status.under_review": "en cours d'examen",
  "l.enum.cr_status.approved": "approuvée",
  "l.enum.cr_status.rejected": "rejetée",
  "l.enum.cr_status.implemented": "mise en œuvre",
  "l.enum.cr_status.withdrawn": "retirée",
  "l.enum.cr_kind.scope": "périmètre",
  "l.enum.cr_kind.schedule": "calendrier",
  "l.enum.cr_kind.cost": "coût",
  "l.enum.cr_kind.quality": "qualité",
  "l.enum.cr_kind.other": "autre",
  "p.plan_change_requests.title_ppm": "Demandes de modification du plan — PPM",
  "p.plan_change_requests.title": "Demandes de modification du plan",
  "p.plan_change_requests.intro":
    "Les demandes de modification de ce plan, les plus récentes d'abord. Rédigez-en une et modifiez-la tant qu'elle est un brouillon ; soumettez-la à une personne approbatrice désignée, qui n'est pas vous ; cette personne l'examine et consigne la décision avec un motif. Une demande ouverte peut être retirée.",
  "p.plan_change_requests.register_link":
    "Demandes de modification de tous les plans",
  "p.plan_change_requests.could_not_load":
    "Impossible de charger les demandes de modification du plan",
  "p.plan_change_requests.could_not_save":
    "Impossible d'enregistrer la demande de modification",
  "p.plan_change_requests.steps": "Étapes",
  "p.plan_change_requests.draft_heading": "Rédiger une demande de modification",
  "p.plan_change_requests.edit_heading": "Modifier le brouillon",
  "p.plan_change_requests.f_description": "Description (ce qui doit changer)",
  "p.plan_change_requests.f_reason":
    "Motif (pourquoi la modification est nécessaire)",
  "p.plan_change_requests.f_days":
    "Impact sur le calendrier en jours ouvrés (négatif avance la fin ; laissez vide si non évalué)",
  "p.plan_change_requests.f_cost":
    "Impact sur le coût (laissez vide si non évalué)",
  "p.plan_change_requests.f_currency":
    "Devise du coût (trois lettres, par exemple EUR)",
  "p.plan_change_requests.f_summary": "Résumé de l'impact",
  "p.plan_change_requests.save_draft": "Enregistrer le brouillon",
  "p.plan_change_requests.drafted": "La demande de modification a été rédigée.",
  "p.plan_change_requests.saved": "Le brouillon a été enregistré.",
  "p.plan_change_requests.problem_title_required":
    "Donnez un titre à la demande de modification.",
  "p.plan_change_requests.problem_description_required":
    "Indiquez ce qui doit changer.",
  "p.plan_change_requests.problem_reason_required":
    "Indiquez pourquoi la modification est nécessaire.",
  "p.plan_change_requests.problem_days_not_whole":
    "L'impact sur le calendrier est un nombre entier de jours ouvrés.",
  "p.plan_change_requests.problem_days_out_of_range":
    "L'impact sur le calendrier doit être compris entre -3650 et 3650 jours ouvrés.",
  "p.plan_change_requests.problem_cost_not_amount":
    "L'impact sur le coût n'est pas un montant.",
  "p.plan_change_requests.problem_cost_needs_currency":
    "Un impact sur le coût doit avoir sa devise.",
  "p.plan_change_requests.problem_currency_needs_cost":
    "Une devise doit accompagner un impact sur le coût.",
  "p.plan_change_requests.problem_currency_invalid":
    "La devise est un code de trois lettres, par exemple EUR, JPY ou GBP.",
  "p.plan_change_requests.steps_heading": "Étapes pour {title}",
  "p.plan_change_requests.status_now": "État actuel : {status}.",
  "p.plan_change_requests.decided": "Décidé par {who} le {on} : {reason}",
  "p.plan_change_requests.no_steps":
    "Aucune autre étape n'est possible pour cette demande de modification.",
  "p.plan_change_requests.who_note":
    "Le service décide qui peut franchir chaque étape : seule la personne qui l'a rédigée la modifie, la soumet ou la retire (un administrateur peut aussi la retirer), et seule la personne approbatrice désignée l'examine ou décide.",
  "p.plan_change_requests.submit_heading": "Soumettre pour décision",
  "p.plan_change_requests.f_approver":
    "Personne approbatrice (une référence person: ou worker:, pas la personne qui l'a rédigée)",
  "p.plan_change_requests.submit": "Soumettre",
  "p.plan_change_requests.submitted":
    "La demande de modification a été soumise.",
  "p.plan_change_requests.problem_approver_not_reference":
    "Indiquez la personne approbatrice sous forme de référence person: ou worker:.",
  "p.plan_change_requests.problem_approver_is_requester":
    "La personne approbatrice ne peut pas être celle qui a demandé la modification.",
  "p.plan_change_requests.review": "Commencer l'examen",
  "p.plan_change_requests.review_started": "L'examen a commencé.",
  "p.plan_change_requests.decide_heading": "Consigner la décision",
  "p.plan_change_requests.f_decision": "Décision",
  "p.plan_change_requests.approve": "Approuver",
  "p.plan_change_requests.reject": "Rejeter",
  "p.plan_change_requests.f_decision_reason":
    "Motif de la décision (au moins 10 caractères)",
  "p.plan_change_requests.decide": "Enregistrer la décision",
  "p.plan_change_requests.decision_recorded": "La décision a été consignée.",
  "p.plan_change_requests.problem_decision_reason":
    "Une décision doit avoir un motif d'au moins 10 caractères.",
  "p.plan_change_requests.withdraw": "Retirer la demande de modification",
  "p.plan_change_requests.withdrawn":
    "La demande de modification a été retirée.",
  "p.plan_change_requests.close_steps": "Fermer les étapes",
  "p.plan_change_requests.refused_requester":
    "Seule la personne qui a rédigé cette demande de modification peut le faire.",
  "p.plan_change_requests.refused_withdraw":
    "Seule la personne qui a rédigé cette demande de modification, ou un administrateur, peut la retirer.",
  "p.plan_change_requests.refused_approver":
    "Seule la personne approbatrice désignée peut examiner cette demande de modification ou en décider.",
  "p.plan_change_requests.refused_state":
    "Cette demande de modification a évolué et cette étape ne lui est plus ouverte ; la liste a été relue.",
  "p.plan_change_requests.refused_invalid":
    "Le service n'a pas accepté la saisie : vérifiez les champs et réessayez.",
  "p.change_requests.change_requests_ppm": "Demandes de modification — PPM",
  "p.change_requests.change_requests": "Demandes de modification",
  "p.change_requests.intro":
    "Une demande de modification est une proposition contrôlée de modifier une référence approuvée. Un approbateur désigné, distinct du demandeur, décide et motive sa décision. Les impacts sont ceux déclarés par le demandeur ; une prévision n'est affichée que si des modifications hypothétiques sont jointes.",
  "p.change_requests.could_not_load":
    "Impossible de charger les demandes de modification",
  "p.change_requests.by_status": "Demandes de modification par statut",
  "p.change_requests.awaiting": "En attente de décision",
  "p.change_requests.oldest_open":
    "Plus ancienne ouverte : {title} ({days} jours calendaires)",
  "p.change_requests.show": "Afficher",
  "p.change_requests.all_statuses": "Tous les statuts",
  "p.change_requests.open_only": "Ouvertes uniquement",
  "p.change_requests.none": "Aucune demande de modification enregistrée.",
  "p.change_requests.none_match":
    "Aucune demande de modification ne correspond.",
  "p.change_requests.loading": "Chargement…",
  "p.change_requests.count_one":
    "{n} demande de modification, les plus récentes d'abord",
  "p.change_requests.count_other":
    "{n} demandes de modification, les plus récentes d'abord",
  "p.change_requests.title_col": "Demande de modification",
  "p.change_requests.kind_col": "Type",
  "p.change_requests.status_col": "Statut",
  "p.change_requests.impact_col": "Impact déclaré",
  "p.change_requests.age_col": "Âge (jours calendaires)",
  "p.change_requests.approver_col": "Approbateur",
  "p.change_requests.no_approver": "Pas encore désigné",
  "p.change_requests.no_impact": "Aucun impact déclaré",
  "p.change_requests.impact_days": "{days} jours ouvrés",
  "p.change_requests.plan_link": "Ouvrir les demandes de modification du plan",
  "p.director_pack.director_packs_ppm": "Dossiers de direction — PPM",
  "p.director_pack.director_packs": "Dossiers de direction",
  "p.director_pack.intro":
    "Un dossier de direction est le rapport mensuel aux administrateurs, rédigé une fois, émis par une personne désignée puis figé : un dossier émis ne change jamais, et une correction est une nouvelle version accompagnée d'une note. Chaque chiffre ci-dessous est lu dans le dossier émis ; rien n'est recalculé ici.",
  "p.director_pack.could_not_load":
    "Impossible de charger les dossiers de direction",
  "p.director_pack.could_not_load_one":
    "Impossible de charger ce dossier de direction",
  "p.director_pack.loading": "Chargement…",
  "p.director_pack.none": "Aucun dossier de direction ne vous est accessible.",
  "p.director_pack.col_period": "Période",
  "p.director_pack.col_audience": "Destinataires",
  "p.director_pack.col_version": "Version",
  "p.director_pack.col_status": "Statut",
  "p.director_pack.col_issued": "Émis",
  "p.director_pack.status_draft": "brouillon",
  "p.director_pack.status_issued": "émis",
  "p.director_pack.open_pack":
    "Ouvrir le dossier de {period}, {audience}, version {version}",
  "p.director_pack.all_packs": "Tous les dossiers de direction",
  "p.director_pack.pack_heading": "Dossier de direction {period} — {audience}",
  "p.director_pack.version_line":
    "Version {version} · {status} · date d'arrêté {cutoff}",
  "p.director_pack.issued_on": "Émis le {when}",
  "p.director_pack.draft_note":
    "Ceci est un brouillon. Il n'est pas encore émis, il n'est donc pas figé.",
  "p.director_pack.change_note": "Note de modification : {note}",
  "p.director_pack.previous_pack":
    "Comparé au dossier de {period}, version {version}.",
  "p.director_pack.download_pdf": "Télécharger en PDF",
  "p.director_pack.pdf_note":
    "Le PDF contient les mêmes chiffres, mais ses libellés fixes sont en anglais. Pour conserver une autre langue, imprimez cette page.",
  "p.director_pack.basis":
    "Les données sont celles en vigueur lors de la rédaction du dossier, datées au plus tard de la date d'arrêté. Les problèmes et les demandes de modification ne conservent pas d'historique de statut ; un élément clos après la date d'arrêté mais avant la rédaction apparaît donc comme clos. Une section sans contenu l'indique ; une absence n'est jamais affichée comme zéro ou vert.",
  "p.director_pack.s_quality": "Qualité des données",
  "p.director_pack.s_changed": "Ce qui a changé",
  "p.director_pack.s_map": "Carte du portefeuille",
  "p.director_pack.s_exceptions": "Exceptions",
  "p.director_pack.s_gates": "Décisions de jalon",
  "p.director_pack.s_funding": "Financement",
  "p.director_pack.s_fever": "Graphique de consommation des tampons",
  "p.director_pack.s_evm": "Valeur acquise",
  "p.director_pack.s_benefits": "Bénéfices",
  "p.director_pack.s_capacity": "Capacité",
  "p.director_pack.s_issues": "Problèmes",
  "p.director_pack.s_changes": "Demandes de modification",
  "p.director_pack.plans_included": "Plans inclus : {n}",
  "p.director_pack.plans_left_out": "Plans exclus car privés ou marqués : {n}",
  "p.director_pack.sections_missing":
    "Sections pas encore construites : {list}",
  "p.director_pack.all_sections":
    "Toutes les sections du dossier sont présentes.",
  "p.director_pack.nothing_here": "Rien à afficher dans cette section.",
  "p.director_pack.changed_issues": "Problèmes ouverts : {now}, {change}",
  "p.director_pack.changed_changes":
    "Demandes de modification en attente de décision : {now}, {change}",
  "p.director_pack.changed_was": "auparavant {before}",
  "p.director_pack.changed_unchanged": "inchangé",
  "p.director_pack.changed_first": "aucun dossier antérieur pour comparer",
  "p.director_pack.c_programme": "Programme",
  "p.director_pack.c_aim": "Objectif stratégique",
  "p.director_pack.c_status": "Statut",
  "p.director_pack.c_stage": "Étape",
  "p.director_pack.c_phase": "Phase",
  "p.director_pack.c_confidence": "Confiance dans la livraison",
  "p.director_pack.c_trend": "Tendance",
  "p.director_pack.self_assessed": "autoévalué",
  "p.director_pack.stale": "obsolète",
  "p.director_pack.no_aim": "Aucun objectif stratégique",
  "p.director_pack.c_plan": "Plan",
  "p.director_pack.c_worst": "Pire évaluation",
  "p.director_pack.c_need": "Décision requise",
  "p.director_pack.c_owner_role": "Responsable (rôle)",
  "p.director_pack.c_decide_by": "Décider avant le",
  "p.director_pack.c_hours_open": "Heures d'ouverture",
  "p.director_pack.ex_window":
    "Une exception non traitée dans les {hours} heures suivant son ouverture est en dépassement.",
  "p.director_pack.ex_open": "Ouvertes : {n}",
  "p.director_pack.ex_breached": "En dépassement : {n}",
  "p.director_pack.ex_closed": "Clôturés sur la période : {n}",
  "p.director_pack.g_passed": "Franchis : {n}",
  "p.director_pack.g_failed": "Échoués : {n}",
  "p.director_pack.g_deferred": "Reportés : {n}",
  "p.director_pack.g_paused": "Programmes suspendus : {n}",
  "p.director_pack.g_stopped": "Programmes arrêtés : {n}",
  "p.director_pack.c_gate": "Jalon",
  "p.director_pack.c_decided": "Décidé",
  "p.director_pack.c_planned": "Prévu",
  "p.director_pack.c_reason": "Motif",
  "p.director_pack.c_days_late": "Jours de retard",
  "p.director_pack.c_since": "Depuis",
  "p.director_pack.c_allocs": "Allocations signalées",
  "p.director_pack.reason_hold": "examiné avec décision de suspension",
  "p.director_pack.reason_no_review": "date prévue dépassée sans examen",
  "p.director_pack.fy_line":
    "Exercice {label}, du {start} au {end}. L'investissement et le fonctionnement sont présentés séparément, et les devises ne sont jamais additionnées.",
  "p.director_pack.c_source": "Source de financement",
  "p.director_pack.c_class": "Catégorie",
  "p.director_pack.c_budget": "Budget",
  "p.director_pack.c_spend": "Dépenses à ce jour",
  "p.director_pack.c_variance": "Écart",
  "p.director_pack.no_funding_source": "aucune source de financement",
  "p.director_pack.forecast_line":
    "Prévision sur la durée de vie en {currency} : réel {actual}, estimation à l'achèvement {eac}",
  "p.director_pack.unfunded_lines":
    "Lignes sans source de financement : {n}. Lignes sans dates : {dated}.",
  "p.director_pack.c_chain": "Chaîne achevée (%)",
  "p.director_pack.c_buffer": "Tampon consommé (%)",
  "p.director_pack.c_zone": "Zone",
  "p.director_pack.c_slip": "Glissement (jours)",
  "p.director_pack.c_buffer_days": "Tampon (jours)",
  "p.director_pack.c_previous": "Dossier précédent",
  "p.director_pack.edge_below": "rattrapage : tracé sur le bord inférieur",
  "p.director_pack.edge_above": "dépassement : tracé sur le bord supérieur",
  "p.director_pack.fever_none_no_buffer":
    "aucun point sur le graphique de fièvre : aucun tampon de projet n'est déclaré",
  "p.director_pack.fever_none_no_baseline":
    "aucun point sur le graphique de fièvre : aucune référence de calendrier",
  "p.director_pack.fever_none_no_chain":
    "aucun point sur le graphique de fièvre : aucun travail critique avec une durée à mesurer",
  "p.director_pack.fever_none_no_forecast":
    "aucun point sur le graphique de fièvre : le calendrier ne peut pas être prévu pour l'instant",
  "p.director_pack.fever_lines":
    "Lignes de zone (% de tampon consommé à 0 % et à 100 % de chaîne achevée) : vert à ambre de {ga0} à {ga1} ; ambre à rouge de {ar0} à {ar1}.",
  "p.director_pack.c_bac": "Budget à l'achèvement",
  "p.director_pack.c_ev": "Valeur acquise",
  "p.director_pack.c_ac": "Coût réel",
  "p.director_pack.c_spi": "SPI",
  "p.director_pack.c_cpi": "CPI",
  "p.director_pack.below_threshold":
    "Sous le seuil de la valeur acquise : budget à l'achèvement {bac}",
  "p.director_pack.no_baseline": "aucune référence",
  "p.director_pack.evm_ratio_note":
    "Le SPI et le CPI sont affichés avec deux décimales ; une case vide signifie non défini, pas zéro.",
  "p.director_pack.c_recurrence": "Récurrence",
  "p.director_pack.c_classification": "Classification",
  "p.director_pack.c_claimed": "Déclaré",
  "p.director_pack.c_validated": "Validé",
  "p.director_pack.c_unvalidated": "Déclaré, non validé",
  "p.director_pack.benefits_note":
    "Un bénéfice réalisé mais non validé est une déclaration, pas un bénéfice validé. Les montants récurrents sont annuels et ne sont jamais additionnés aux montants ponctuels.",
  "p.director_pack.c_pool": "Vivier de compétences",
  "p.director_pack.c_demand": "Demande (FTE)",
  "p.director_pack.c_supply": "Offre (FTE)",
  "p.director_pack.c_gap": "Écart (FTE)",
  "p.director_pack.c_month": "Mois",
  "p.director_pack.c_reading": "Lecture",
  "p.director_pack.no_pool": "aucun vivier attribué",
  "p.director_pack.capacity_note":
    "Chaque cellule comporte un mot et un symbole en plus d'une couleur. L'absence d'offre enregistrée n'est pas une capacité nulle.",
  "p.director_pack.partners_short":
    "livraison des partenaires inférieure à l'engagement",
  "p.director_pack.open_issues": "Problèmes ouverts : {n}",
  "p.director_pack.awaiting":
    "Demandes de changement en attente de décision : {n}",
  "p.director_pack.dash": "non enregistré",
  "l.enum.pk_conf.green": "vert",
  "l.enum.pk_conf.amber_green": "ambre/vert",
  "l.enum.pk_conf.amber": "ambre",
  "l.enum.pk_conf.amber_red": "ambre/rouge",
  "l.enum.pk_conf.red": "rouge",
  "l.enum.pk_conf.not_assessed": "non évalué",
  "l.enum.pk_trend.up": "en amélioration",
  "l.enum.pk_trend.down": "en dégradation",
  "l.enum.pk_trend.same": "inchangé",
  "l.enum.pk_trend.new": "nouvellement évalué",
  "l.enum.pk_trend.not_assessed": "non évalué",
  "l.enum.pk_trend.no_earlier_pack": "aucun dossier antérieur",
  "l.enum.pk_status.active": "actif",
  "l.enum.pk_status.paused": "suspendu",
  "l.enum.pk_status.stopped": "arrêté",
  "l.enum.pk_zone.green": "vert",
  "l.enum.pk_zone.amber": "ambre",
  "l.enum.pk_zone.red": "rouge",
  "l.enum.pk_heat.shortfall": "déficit",
  "l.enum.pk_heat.tight": "tendu",
  "l.enum.pk_heat.enough": "suffisant",
  "l.enum.pk_heat.no_supply_recorded": "aucune offre enregistrée",
  "l.enum.pk_heat.no_demand": "aucune demande",
  "l.enum.pk_heat.nothing_recorded": "rien d'enregistré",
  "l.enum.pk_class.capex": "investissement",
  "l.enum.pk_class.opex": "fonctionnement",
  "l.enum.pk_class.all": "investissement et fonctionnement combinés",
  "l.enum.pk_recur.recurring": "récurrent",
  "l.enum.pk_recur.one_off": "ponctuel",
  "l.enum.pk_recur.not_stated": "non précisé",
  "l.enum.pk_cls.cash_releasing": "générateur de trésorerie",
  "l.enum.pk_cls.non_cash_releasing": "non générateur de trésorerie",
  "l.enum.pk_cls.societal": "sociétal",
  "l.enum.pk_cls.not_stated": "non précisé",
  "l.enum.pk_gate.g0_concept": "G0 conception",
  "l.enum.pk_gate.g1_feasibility": "G1 faisabilité",
  "l.enum.pk_gate.g2_definition": "G2 définition",
  "l.enum.pk_gate.g3_delivery": "G3 réalisation",
  "l.enum.pk_gate.g4_launch": "G4 lancement",
  "l.enum.pk_gate.g5_benefits": "G5 bénéfices",
  "p.director_pack.c_currency": "Devise",
  "p.critical_chain.chain_ppm": "Chaîne critique et tampons — PPM",
  "p.critical_chain.buffers_ppm": "Gestion des tampons — PPM",
  "p.critical_chain.pipeline_ppm": "Tambour et pipeline — PPM",
  "p.critical_chain.loading": "Chargement…",
  "p.critical_chain.yes": "oui",
  "p.critical_chain.no": "non",
  "p.critical_chain.zone_green": "vert",
  "p.critical_chain.zone_amber": "ambre",
  "p.critical_chain.zone_red": "rouge",
  "p.critical_chain.zone_none": "aucune lecture",
  "p.critical_chain.outcome_as_planned": "conserve ses dates",
  "p.critical_chain.outcome_staggered": "démarre plus tard",
  "p.critical_chain.outcome_exceeds_drum":
    "demande plus que ce dont dispose le tambour",
  "p.critical_chain.outcome_no_slot": "aucun démarrage possible",
  "p.critical_chain.advice":
    "Ceci est un conseil. Rien n'a été modifié, et c'est une personne qui décide de la suite.",
  "p.critical_chain.link_plan": "Plan",
  "p.critical_chain.link_buffers": "Gestion des tampons",
  "p.critical_chain.link_chain": "Chaîne critique",
  "p.critical_chain.link_critical_path": "Chemin critique",
  "p.critical_chain.nav_pipeline": "Tambour et pipeline",
  "p.critical_chain.chain_h1": "Chaîne critique et tampons",
  "p.critical_chain.chain_lede":
    "La chaîne critique est la plus longue suite de tâches compte tenu de la disponibilité des personnes : elle suit les dépendances et les attentes d'une personne occupée sur une autre tâche. Elle est présentée à côté du chemin critique, qui ne suit que les dépendances ; les deux ne sont jamais mélangés.",
  "p.critical_chain.chain_h2": "La chaîne critique",
  "p.critical_chain.no_chain": "Il n'y a pas de chaîne critique : {reason}",
  "p.critical_chain.reason_no_assignments":
    "personne n'est affecté à une tâche inachevée, il n'y a donc aucune attente de personne à suivre",
  "p.critical_chain.reason_no_scheduled_tasks": "aucune tâche n'est planifiée",
  "p.critical_chain.chain_work":
    "Jours ouvrés de travail sur la chaîne : {days}",
  "p.critical_chain.chain_tasks_count": "Tâches sur la chaîne : {tasks}",
  "p.critical_chain.chain_finish":
    "Compte tenu de la disponibilité des personnes, le plan se termine le {finish}.",
  "p.critical_chain.chain_delay_none":
    "Ce n'est pas plus tard que la fin du chemin critique.",
  "p.critical_chain.chain_delay":
    "Jours ouvrés après la fin du chemin critique : {days}",
  "p.critical_chain.crosses_yes":
    "La chaîne passe d'un chemin à l'autre : une personne est partagée.",
  "p.critical_chain.crosses_no":
    "La chaîne reste sur un seul chemin : aucune personne n'est partagée.",
  "p.critical_chain.tied":
    "Une autre chaîne est tout aussi longue ; celle-ci est affichée parce qu'elle vient en premier dans le plan.",
  "p.critical_chain.timeline_label": "La chaîne sur une frise chronologique",
  "p.critical_chain.timeline_caption":
    "Chaque barre pleine est une tâche de la chaîne. Une barre en pointillés est le tampon de projet. Le tableau ci-dessous donne les mêmes chiffres.",
  "p.critical_chain.col_task": "Tâche",
  "p.critical_chain.col_days": "Jours ouvrés",
  "p.critical_chain.col_start": "Début",
  "p.critical_chain.col_finish": "Fin",
  "p.critical_chain.col_held_by": "Retenue par",
  "p.critical_chain.col_on_path": "Sur le chemin critique",
  "p.critical_chain.held_first": "première de la chaîne",
  "p.critical_chain.held_dep": "une dépendance",
  "p.critical_chain.held_person": "une personne : {person}",
  "p.critical_chain.held_person_hidden": "une personne (masquée)",
  "p.critical_chain.cp_h2": "Le chemin critique en regard",
  "p.critical_chain.cp_finish": "Le chemin critique se termine le {finish}.",
  "p.critical_chain.cp_none": "Il n'y a aucun chemin critique à afficher.",
  "p.critical_chain.buffers_h2": "Tampons",
  "p.critical_chain.buffers_lede":
    "Les tâches sont planifiées selon leurs estimations agressives ; la marge de sécurité retirée de chaque tâche est regroupée dans un tampon. Les tailles sont en jours ouvrés. La règle : {rule}.",
  "p.critical_chain.rule_cut_and_paste":
    "la moitié de la marge retirée (couper-coller)",
  "p.critical_chain.rule_root_sum_squares":
    "la racine carrée de la somme des carrés des marges retirées",
  "p.critical_chain.no_buffers": "Il n'y a aucun tampon à afficher : {reason}",
  "p.critical_chain.reason_estimates_missing":
    "certaines tâches de la chaîne n'ont pas encore d'estimation agressive et sûre. Enregistrez-les via le service d'estimation ; aucun écran ne le permet encore.",
  "p.critical_chain.missing_h3":
    "Tâches de la chaîne sans les deux estimations",
  "p.critical_chain.project_buffer":
    "Tampon de projet : {days} jours ouvrés, protégeant une chaîne qui se termine le {finish}. Avec le tampon, la date est le {with}.",
  "p.critical_chain.project_safety":
    "Marge retirée des tâches de la chaîne : {days} jours ouvrés.",
  "p.critical_chain.project_buffer_bar": "Tampon de projet",
  "p.critical_chain.feeding_h3": "Tampons d'alimentation",
  "p.critical_chain.feeding_none":
    "Aucune autre chaîne de tâches ne rejoint la chaîne critique.",
  "p.critical_chain.col_feeding_tasks": "Tâches d'alimentation",
  "p.critical_chain.col_joins": "Rejoint",
  "p.critical_chain.col_buffer": "Tampon (jours ouvrés)",
  "p.critical_chain.col_room": "Marge avant la jonction (jours ouvrés)",
  "p.critical_chain.col_push": "Repousserait la chaîne",
  "p.critical_chain.push_yes": "oui, de {days} jours ouvrés",
  "p.critical_chain.unsized": "non dimensionné : une estimation manque",
  "p.critical_chain.resource_h3":
    "Tampons de ressources (alertes de disponibilité)",
  "p.critical_chain.resource_lede":
    "Un tampon de ressource n'est pas du temps. C'est un rappel qu'une personne doit être prête avant le début de sa tâche sur la chaîne, {lead} jours ouvrés à l'avance.",
  "p.critical_chain.col_person": "Personne",
  "p.critical_chain.col_alert_on": "Être prêt le",
  "p.critical_chain.col_task_starts": "Début de la tâche",
  "p.critical_chain.hidden": "(masqué)",
  "p.critical_chain.likely_h3":
    "Planifiées selon leur durée probable (sans estimation agressive)",
  "p.critical_chain.could_not_load_chain":
    "Impossible de charger la chaîne critique",
  "p.critical_chain.buffers_h1": "Gestion des tampons",
  "p.critical_chain.buffers_lede2":
    "Chaque tampon est lu par rapport au calendrier en cours : quelle part de la chaîne est achevée et quelle part du tampon le glissement prévu a consommée. Un tampon dans le rouge exige une action de rattrapage, convenue et enregistrée par une personne.",
  "p.critical_chain.no_plan":
    "Aucun plan de tampons n'a été figé pour ce plan. Figez-en un une fois la chaîne et les tampons convenus ; les lectures ultérieures seront mesurées par rapport à lui.",
  "p.critical_chain.freeze_confirm":
    "J'ai lu la chaîne et les tampons et je souhaite les figer comme plan",
  "p.critical_chain.freeze_button": "Figer le plan de tampons",
  "p.critical_chain.frozen_on":
    "Mesuré par rapport au plan de tampons figé le {date}.",
  "p.critical_chain.alerts_h2": "Alertes",
  "p.critical_chain.alerts_none": "Aucun tampon n'est dans le rouge.",
  "p.critical_chain.alert_project":
    "Le tampon de projet est dans le rouge : {consumed} % consommés. Chaîne : {chain}.",
  "p.critical_chain.alert_feeding":
    "Un tampon d'alimentation est dans le rouge : {consumed} % consommés. Chaîne : {chain}, rejoignant {joins}.",
  "p.critical_chain.needs_action":
    "Aucune action de rattrapage ouverte n'est enregistrée pour : {buffers}.",
  "p.critical_chain.charts_h2": "Graphiques de fièvre",
  "p.critical_chain.project_label": "Tampon de projet",
  "p.critical_chain.feeding_label": "Tampon d'alimentation : {chain}",
  "p.critical_chain.axis_x": "Chaîne achevée (%)",
  "p.critical_chain.axis_y": "Tampon consommé (%)",
  "p.critical_chain.chart_caption":
    "{buffer} : {zone} ; {consumed} % du tampon consommés ; {done} % de la chaîne achevés.",
  "p.critical_chain.beyond_edge":
    "La valeur réelle est hors du graphique ; le point est donc tracé sur le bord.",
  "p.critical_chain.table_h2": "Les mêmes chiffres sous forme de tableau",
  "p.critical_chain.col_size": "Tampon (jours ouvrés)",
  "p.critical_chain.col_done": "Chaîne achevée (%)",
  "p.critical_chain.col_used": "Tampon consommé (%)",
  "p.critical_chain.col_slip": "Glissement prévu (jours ouvrés)",
  "p.critical_chain.col_zone": "Zone",
  "p.critical_chain.no_point": "Aucune lecture : {reason}",
  "p.critical_chain.actions_h2": "Actions de rattrapage",
  "p.critical_chain.actions_none":
    "Aucune action de rattrapage n'a été enregistrée.",
  "p.critical_chain.col_buffer_name": "Tampon",
  "p.critical_chain.col_action": "Action",
  "p.critical_chain.col_owner": "Responsable",
  "p.critical_chain.col_due": "Échéance",
  "p.critical_chain.col_status": "Statut",
  "p.critical_chain.status_open": "ouverte",
  "p.critical_chain.status_done": "faite",
  "p.critical_chain.status_dropped": "abandonnée",
  "p.critical_chain.mark_done": "Marquer comme faite",
  "p.critical_chain.drop_action": "Abandonner",
  "p.critical_chain.add_action_h3": "Enregistrer une action de rattrapage",
  "p.critical_chain.f_buffer": "Tampon",
  "p.critical_chain.f_action": "Ce qui sera fait",
  "p.critical_chain.f_owner": "Responsable (une référence de personne)",
  "p.critical_chain.f_due": "Échéance (facultatif)",
  "p.critical_chain.add_button": "Enregistrer l'action",
  "p.critical_chain.snap_h2": "Historique",
  "p.critical_chain.snap_button": "Enregistrer un instantané maintenant",
  "p.critical_chain.snap_none": "Aucun instantané n'a été enregistré.",
  "p.critical_chain.col_snap_date": "Enregistré le",
  "p.critical_chain.could_not_load_buffers":
    "Impossible de charger la gestion des tampons",
  "p.critical_chain.could_not_save": "Impossible d'enregistrer",
  "p.critical_chain.pipeline_h1": "Tambour et pipeline",
  "p.critical_chain.pipeline_lede":
    "Le tambour est le vivier de compétences dont le portefeuille dépend le plus. Les projets qui en ont besoin en même temps sont échelonnés : le service propose des démarrages plus tardifs, avec un tampon de capacité entre les projets, et explique chacun d'eux. Il ne déplace jamais une date.",
  "p.critical_chain.drum_h2": "Le tambour",
  "p.critical_chain.drum_named":
    "Le tambour est {name} : désigné par le bureau du portefeuille.",
  "p.critical_chain.drum_overloaded":
    "Le tambour est {name} : aucun vivier n'est désigné, et celui-ci présente le plus grand déficit.",
  "p.critical_chain.no_drum": "Il n'y a pas de tambour : {why}",
  "p.critical_chain.set_h3": "Désigner le tambour",
  "p.critical_chain.pick_pool": "Vivier de compétences",
  "p.critical_chain.set_button": "Le désigner comme tambour",
  "p.critical_chain.clear_button": "Effacer la désignation",
  "p.critical_chain.capacity":
    "Capacité du tambour : {fte} FTE. Tampon de capacité entre les projets : {days} jours.",
  "p.critical_chain.no_capacity":
    "Le tambour n'a aucune offre enregistrée sur la période ; il n'y a donc rien par rapport à quoi échelonner.",
  "p.critical_chain.projects_h2": "Démarrages proposés",
  "p.critical_chain.projects_none":
    "Aucun projet n'a besoin du tambour sur cette période.",
  "p.critical_chain.pipeline_timeline":
    "Utilisation du tambour actuelle et proposée",
  "p.critical_chain.pipeline_timeline_caption":
    "Pour chaque projet, la barre du haut indique quand il utilise le tambour actuellement et celle du bas quand il l'utiliserait selon la proposition. Le tableau ci-dessous donne les mêmes dates.",
  "p.critical_chain.bar_now": "actuel",
  "p.critical_chain.bar_proposed": "proposé",
  "p.critical_chain.col_project": "Projet",
  "p.critical_chain.col_needs": "Besoin du tambour",
  "p.critical_chain.col_peak": "Pic (FTE)",
  "p.critical_chain.col_proposal": "Proposition",
  "p.critical_chain.col_proposed": "Première utilisation proposée",
  "p.critical_chain.col_delay": "Retard (jours)",
  "p.critical_chain.col_why": "Pourquoi",
  "p.critical_chain.could_not_load_pipeline":
    "Impossible de charger le pipeline",
  "p.start_queue.nav": "File de démarrage",
  "p.start_queue.title": "File de démarrage — PPM",
  "p.start_queue.h1": "File de démarrage",
  "p.start_queue.lede":
    "Les programmes qui ont demandé à démarrer leur réalisation et ont été refusés, dans l'ordre de leur demande. Un programme est refusé lorsque la limite de travaux en cours est atteinte ou lorsque le vivier contraint manque d'offre dans les mois à venir. L'ordre ne change jamais, et un démarrage peut tout de même avoir lieu avec une dérogation enregistrée et un motif.",
  "p.start_queue.loading": "Chargement de la file…",
  "p.start_queue.none": "Aucun programme n'attend de démarrer.",
  "p.start_queue.col_pos": "Rang",
  "p.start_queue.col_prog": "Programme",
  "p.start_queue.col_asked": "Demandé le",
  "p.start_queue.col_why": "Motif du refus",
  "p.start_queue.col_act": "Action",
  "p.start_queue.limit":
    "Les {limit} places sont occupées : {in_delivery} programmes sont en réalisation.",
  "p.start_queue.pool":
    "Le vivier contraint {pool} manque d'offre en : {months}.",
  "p.start_queue.withdraw": "Retirer {name} de la file",
  "p.start_queue.withdraw_short": "Retirer",
  "p.start_queue.could_not_load": "Impossible de charger la file de démarrage",
  "p.start_queue.could_not_withdraw": "Impossible de retirer de la file",
  "p.table.search": "Filtrer les lignes",
  "p.table.export": "Exporter",
  "p.table.find": "Rechercher",
  "p.table.moved": "{header} déplacée en position {position} sur {count}",
  "p.table.copy": "Copier",
  "p.table.copied": "{rows} lignes copiées dans le presse-papiers",
  "p.table.copy_failed": "Impossible de copier dans le presse-papiers",
  "p.table.matches": "{matches} sur {total} affichées",
  "p.table.search_in": "Rechercher dans",
  "p.table.all_columns": "Toutes les colonnes",
  "p.table.pages": "Pages",
  "p.table.export_csv": "CSV",
  "p.table.export_tsv": "TSV",
  "p.table.export_excel": "Excel",
  "p.table.exported": "{rows} lignes exportées vers {file}",
  "p.table.columns": "Colonnes",
  "p.table.show_column": "Afficher {header}",
  "p.table.resize": "Redimensionner la colonne {header}",
  "p.table.select_all": "Sélectionner toutes les lignes",
  "p.table.select_col": "Sélectionner",
  "p.table.select_row": "Sélectionner {row}",
  "p.table.prev": "Page précédente",
  "p.table.next": "Page suivante",
  "p.table.page_status": "Page {page} sur {pages} ({rows} lignes)",
  "p.table.page_announce": "Page {page} sur {pages}",
  "p.table.sorted": "{header} trié par ordre {direction}",
  "p.table.asc": "croissant",
  "p.table.desc": "décroissant",
  "p.table.none": "non trié",
  "p.table.filtered": "{matches} lignes sur {total} correspondent",
  "p.table.selected": "{n} lignes sélectionnées",
  "p.table.empty": "Rien à afficher.",
  "p.demo.tour_title": "Visite de démonstration — PPM",
  "p.demo.tour_h1": "Visite de démonstration",
  "p.demo.tour_lede":
    "Vous êtes connecté en tant qu'Example Person. Tout ici est fictif : la Northgate Transport Authority, ses collaborateurs, son argent et ses dates ont été inventés pour cette démonstration. Choisissez une fonctionnalité ci-dessous et utilisez-la : vous pouvez modifier les données, rien n'est réel.",
  "p.demo.tour_story_h2": "Le scénario",
  "p.demo.tour_story":
    "La Northgate Transport Authority modernise son réseau ferroviaire : nouvelle signalisation, gares accessibles de plain-pied et dépôt numérique. Dix plans, huit personnes, quatre organisations et quelques problèmes volontaires, pour que les écrans aient quelque chose à montrer.",
  "p.demo.tour_open": "Ouvrir",
  "p.demo.tour_no_plan":
    "Le plan North Line est introuvable ; les liens vers ses pages sont donc masqués. Relancez l'initialisation des données.",
  "p.demo.tour_toc": "Fonctionnalités",
  "p.demo.g_overview": "Le portefeuille en un coup d'œil",
  "p.demo.g_delivery": "Réaliser un projet (enclenchement de la North Line)",
  "p.demo.g_risk": "Risques, problèmes et changements",
  "p.demo.g_money": "Finances",
  "p.demo.g_people": "Personnes et capacité",
  "p.demo.g_strategy": "Stratégie et entrée des demandes",
  "p.demo.g_escalation": "Gouvernance et escalade",
  "p.demo.g_chain": "Chaîne critique",
  "p.demo.g_tools": "Outils",
  "p.demo.f_kanban_t": "Mon tableau Kanban",
  "p.demo.f_kanban_d":
    "Vos propres tâches ouvertes, en colonnes par statut, issues de tous les projets. Déplacez-en une à la souris ou par son menu ; le statut de la tâche change pour tout le monde.",
  "p.demo.f_me_t": "Mon travail",
  "p.demo.f_me_d":
    "Les mêmes tâches par échéance, dans votre fuseau horaire : en retard, aujourd'hui, cette semaine, plus tard.",
  "p.demo.f_plans_t": "Plans",
  "p.demo.f_plans_d":
    "Dix plans fictifs : un portefeuille, trois programmes et six projets pour la Northgate Transport Authority. Ouvrez-en un.",
  "p.demo.f_dashboard_t": "Tableau de bord du portefeuille",
  "p.demo.f_dashboard_d":
    "Statut par type et par étape, avec les décomptes rouge, ambre et vert.",
  "p.demo.f_gantt_t": "Gantt",
  "p.demo.f_gantt_d":
    "Le calendrier de chaque projet sur une seule frise chronologique.",
  "p.demo.f_milestones_t": "Jalons",
  "p.demo.f_milestones_d":
    "Conception approuvée, équipement livré et mise en service pour chaque projet.",
  "p.demo.f_calendar_t": "Calendrier",
  "p.demo.f_calendar_d": "Jalons et échéances par mois.",
  "p.demo.f_board_t": "Tableau des tâches",
  "p.demo.f_board_d":
    "Huit tâches, du relevé à la remise, en colonnes par statut. L'une est bloquée par l'assurance sécurité.",
  "p.demo.f_critical_path_t": "Chemin critique",
  "p.demo.f_critical_path_d":
    "La suite de tâches dépendantes qui fixe la date de fin.",
  "p.demo.f_earned_value_t": "Valeur acquise",
  "p.demo.f_earned_value_d":
    "Valeur planifiée, valeur acquise et coût, avec les indices de délai et de coût.",
  "p.demo.f_schedule_health_t": "Santé du calendrier",
  "p.demo.f_schedule_health_d":
    "Contrôles de la logique du plan : liens manquants, tâches longues, contraintes.",
  "p.demo.f_wbs_t": "Structure de découpage du projet",
  "p.demo.f_wbs_d": "Le projet découpé en lots de travaux.",
  "p.demo.f_risk_t": "Registre des risques",
  "p.demo.f_risk_d":
    "Trois risques sur chaque projet, notés selon la probabilité et l'impact.",
  "p.demo.f_issues_t": "Problèmes",
  "p.demo.f_issues_d":
    "Un tracé de câble bloqué, un banc d'essai réservé deux fois et un permis retardé.",
  "p.demo.f_change_requests_t": "Demandes de changement",
  "p.demo.f_change_requests_d":
    "Une demande de décaler la mise en service de deux semaines, avec son impact sur le calendrier et le coût, en attente du directeur de programme.",
  "p.demo.f_joint_risk_t": "Risques de coût et de délai conjoints",
  "p.demo.f_joint_risk_d":
    "L'effet conjoint des risques sur la date de fin et sur le coût.",
  "p.demo.f_financials_t": "Finances",
  "p.demo.f_financials_d":
    "Budget, dépenses réelles et écart sur l'ensemble du portefeuille.",
  "p.demo.f_cost_risk_t": "Risque de coût",
  "p.demo.f_cost_risk_d":
    "La dispersion du coût probable, avec le P50 et le P80.",
  "p.demo.f_ledger_t": "Registre des financements",
  "p.demo.f_ledger_d":
    "Une subvention et une réserve, rattachées aux lignes budgétaires qui y puisent. L'argent pas encore affecté à une ligne apparaît comme non rattaché.",
  "p.demo.f_costs_t": "Coûts d'un projet",
  "p.demo.f_costs_d":
    "Coûts de main-d'œuvre issus des grilles de taux, et autres coûts par tâche.",
  "p.demo.f_resources_t": "Ressources",
  "p.demo.f_resources_d":
    "Huit personnes fictives, leurs rôles et ce sur quoi elles sont affectées.",
  "p.demo.f_capacity_t": "Carte de chaleur de la capacité",
  "p.demo.f_capacity_d":
    "Offre et demande par vivier de compétences et par mois. Les mois en rouge montrent où les ingénieurs signalisation manquent.",
  "p.demo.f_key_persons_t": "Personnes clés",
  "p.demo.f_key_persons_d":
    "Points de défaillance uniques : un ingénieur que personne ne peut encore remplacer, et un rôle avec un successeur prêt.",
  "p.demo.f_objectives_t": "Objectifs",
  "p.demo.f_objectives_d":
    "Trois objectifs, chacun relié aux plans qui le servent.",
  "p.demo.f_okrs_t": "Objectifs et résultats clés",
  "p.demo.f_okrs_d":
    "Résultats clés avec points d'étape et niveau de confiance.",
  "p.demo.f_kpis_t": "Indicateurs clés",
  "p.demo.f_kpis_d":
    "Minutes de retard, satisfaction des voyageurs et dépenses par rapport au plan, avec un relevé chaque mois.",
  "p.demo.f_proposals_t": "Entrée de travaux",
  "p.demo.f_proposals_d":
    "Cinq propositions à différentes étapes, du brouillon à l'approbation ou au rejet.",
  "p.demo.f_ideas_t": "Idées",
  "p.demo.f_ideas_d":
    "Quatre idées avec des votes. Convertissez-en une en proposition.",
  "p.demo.f_scenarios_t": "Scénarios",
  "p.demo.f_scenarios_d":
    "Ce qui tient sous un plafond budgétaire si les travaux de signalisation passent en premier.",
  "p.demo.f_director_packs_t": "Dossiers de direction",
  "p.demo.f_director_packs_d":
    "Dossiers de direction et du conseil pour septembre, constitués à partir des données en direct. Ouvrez-en un pour voir chaque section.",
  "p.demo.f_organizations_t": "Organisations partenaires",
  "p.demo.f_organizations_d":
    "L'autorité, son entreprise de réalisation, un partenaire et un fournisseur.",
  "p.demo.f_milestone_signoffs_t": "Validation des jalons",
  "p.demo.f_milestone_signoffs_d":
    "La mise en service doit être signée par le partenaire et le fournisseur. Le fournisseur a signé, le partenaire non ; la mise en service est donc livrée mais non atteinte.",
  "p.demo.f_cutover_t": "Décision de bascule",
  "p.demo.f_cutover_d":
    "Trois critères d'arrêt, dont un rempli, et une cosignature obtenue. Un feu vert serait refusé, et l'écran explique pourquoi.",
  "p.demo.f_deployment_moves_t": "Décalages de déploiement",
  "p.demo.f_deployment_moves_d":
    "Une demande de décaler le début de l'installation de deux semaines, avec l'effet sur les autres tâches et son coût de main-d'œuvre, en attente d'approbation.",
  "p.demo.f_start_queue_t": "File de démarrage",
  "p.demo.f_start_queue_d":
    "Deux programmes sont en réalisation et la limite est de deux ; le troisième attend donc dans la file, à son rang.",
  "p.demo.f_critical_chain_t": "Chaîne critique",
  "p.demo.f_critical_chain_d":
    "La chaîne à travers les tâches dépendantes et les personnes qu'elles partagent, avec des estimations agressives et sûres.",
  "p.demo.f_buffers_t": "Gestion des tampons",
  "p.demo.f_buffers_d":
    "Le plan de tampons figé, les graphiques de fièvre et une action de rattrapage.",
  "p.demo.f_pipeline_t": "Tambour et pipeline",
  "p.demo.f_pipeline_d":
    "Le vivier de compétences dont le portefeuille dépend le plus, et les démarrages échelonnés proposés.",
  "p.demo.f_search_t": "Recherche",
  "p.demo.f_search_d":
    "Saisissez du texte dans la loupe de l'en-tête. Ce lien recherche « signal ».",
  "p.demo.f_email_deliveries_t": "Envois d'e-mails",
  "p.demo.f_email_deliveries_d":
    "Chaque e-mail que le service a tenté d'envoyer. Rien n'est envoyé depuis la démonstration ; cette liste est donc vide.",
  "p.demo.f_display_t": "Paramètres d'affichage",
  "p.demo.f_display_d": "Taille du texte, thèmes, vue simplifiée et langue.",
  "p.kanban.nav": "Mon Kanban",
  "p.kanban.title": "Mon tableau Kanban — PPM",
  "p.kanban.h1": "Mon tableau Kanban",
  "p.kanban.lede":
    "Vos tâches ouvertes dans tous les plans, en colonnes par statut. Déplacez une carte à la souris, ou appuyez sur Entrée sur une carte pour choisir une colonne dans un menu. Un déplacement change le statut de la tâche pour tout le monde.",
  "p.kanban.board_label": "Vos tâches par statut",
  "p.kanban.loading": "Chargement de votre tableau…",
  "p.kanban.none": "Vous n'avez aucune tâche ouverte.",
  "p.kanban.todo": "À faire",
  "p.kanban.in_progress": "En cours",
  "p.kanban.in_review": "En revue",
  "p.kanban.blocked": "Bloqué",
  "p.kanban.done": "Fait",
  "p.kanban.card_count": "{n} cartes",
  "p.kanban.move_button": "Déplacer {title}",
  "p.kanban.move_menu": "Déplacer vers la colonne",
  "p.kanban.moved": "{title} déplacée vers {column}",
  "p.kanban.due": "échéance le {date}",
  "p.kanban.points": "{n} points",
  "p.kanban.sign_in": "Connectez-vous pour voir votre tableau.",
  "p.kanban.could_not_load": "Impossible de charger votre tableau",
  "p.kanban.could_not_move":
    "Impossible de déplacer la tâche ; le tableau est inchangé.",
  "p.kanban.hidden":
    "{n} tâches ne sont pas affichées car vous n'avez pas accès en lecture à leurs plans.",
  "p.dashboard.my_home": "Ma page d'accueil",
  "p.demo.banner":
    "Ceci est une démonstration avec des données fictives. Rien ici n'est réel.",
  "p.demo.banner_tour": "Faire la visite",
  "p.demo.signin_h2": "Démonstration",
  "p.demo.signin_lede":
    "Ce site est une démonstration. Connectez-vous avec example@example.com pour voir toutes les fonctionnalités avec des données fictives. Aucun e-mail n'est envoyé.",
  "p.demo.signin_button": "Se connecter à la démonstration",
  "p.demo.signin_wrong": "La démonstration n'accepte que example@example.com.",
  "p.search.title": "Recherche — PPM",
  "p.search.h1": "Rechercher des plans",
  "p.search.lede":
    "Les plans dont le nom, les identifiants, les mots-clés ou les étiquettes correspondent à votre recherche. Les plans que vous ne pouvez pas lire ne sont pas listés.",
  "p.search.none_asked":
    "Saisissez du texte dans le champ de recherche de l'en-tête.",
  "p.search.searching": "Recherche en cours…",
  "p.search.results": "{n} plans correspondent à « {q} ».",
  "p.search.no_results": "Aucun plan ne correspond à « {q} ».",
  "p.search.could_not_search": "Impossible d'effectuer la recherche",
  "p.layout.link_picker": "Pages",
  "p.layout.search_picker": "Rechercher des plans",
  "p.layout.search_input": "Termes de recherche",
  "p.layout.search_submit": "Rechercher",
  "p.layout.lp_home": "Accueil",
  "p.layout.lp_plans": "Plans",
  "p.layout.lp_dashboard": "Tableau de bord",
  "p.layout.lp_milestones": "Jalons",
  "p.layout.lp_proposals": "Propositions",
  "p.cutover.link": "Décision de bascule",
  "p.cutover.title": "Décision de bascule — PPM",
  "p.cutover.h1": "Décision de bascule",
  "p.cutover.lede":
    "La décision de basculer un système critique pour la sécurité. Les critères d'arrêt sont d'abord rédigés et publiés, puis figés : les modifier crée une nouvelle version. Un feu vert exige que tous les critères soient remplis et toutes les cosignatures requises obtenues. Un refus, ou un report pour raison de sécurité, est une décision prise sur la base des critères ; ce n'est pas un glissement.",
  "p.cutover.loading": "Chargement des décisions…",
  "p.cutover.none": "Aucune décision de bascule n'a été rédigée.",
  "p.cutover.new_h2": "Rédiger une décision de bascule",
  "p.cutover.new_title": "Titre",
  "p.cutover.new_criteria":
    "Critères d'arrêt, un par ligne (de 5 à 500 caractères chacun)",
  "p.cutover.draft": "Rédiger la décision",
  "p.cutover.version": "version {n}",
  "p.cutover.status_draft": "brouillon",
  "p.cutover.status_published": "publiée, critères figés",
  "p.cutover.status_open": "ouverte à la décision",
  "p.cutover.status_decided": "décidée",
  "p.cutover.status_superseded": "remplacée",
  "p.cutover.outcome_go": "feu vert",
  "p.cutover.outcome_no_go": "refus",
  "p.cutover.outcome_delay_for_safety": "report pour raison de sécurité",
  "p.cutover.decided_line": "Décidé le {on} : {outcome}. {why}",
  "p.cutover.criteria_h3": "Critères d'arrêt",
  "p.cutover.col_n": "N°",
  "p.cutover.col_criterion": "Critère",
  "p.cutover.col_met": "Rempli",
  "p.cutover.col_evidence": "Preuve",
  "p.cutover.col_mark": "Marquer",
  "p.cutover.met": "rempli",
  "p.cutover.not_met": "non rempli",
  "p.cutover.evidence": "Preuve",
  "p.cutover.mark_met": "Marquer comme rempli : {name}",
  "p.cutover.mark_unmet": "Marquer comme non rempli : {name}",
  "p.cutover.mark_met_short": "Rempli",
  "p.cutover.mark_unmet_short": "Non rempli",
  "p.cutover.cosign_h3": "Cosignatures",
  "p.cutover.cosign_signed": "Signé : {roles}.",
  "p.cutover.cosign_missing": "Encore requis : {roles}.",
  "p.cutover.cosign_stale":
    "Signé pour des preuves antérieures, donc à signer de nouveau : {roles}.",
  "p.cutover.cosign_none": "aucun",
  "p.cutover.cosign_role": "Rôle de signature (votre jeton doit le porter)",
  "p.cutover.cosign": "Cosigner",
  "p.cutover.refused_h3": "Le feu vert serait refusé car",
  "p.cutover.publish": "Publier et figer les critères",
  "p.cutover.open": "Ouvrir la décision",
  "p.cutover.new_version":
    "Créer une nouvelle version pour modifier les critères",
  "p.cutover.decide_h3": "Décider",
  "p.cutover.outcome": "Résultat",
  "p.cutover.reason": "Motif",
  "p.cutover.decide": "Enregistrer la décision",
  "p.cutover.could_not_load": "Impossible de charger les décisions",
  "p.cutover.could_not_save": "Impossible d'enregistrer",
  "p.cutover.cancel_edit": "Arrêter la modification",
  "p.cutover.criteria_frozen":
    "Cette décision est publiée, ses critères sont donc figés : créez une nouvelle version pour les changer.",
  "p.cutover.criteria_rules":
    "Un critère d'arrêt par ligne : de 1 à 30, chacun de 5 à 500 caractères. Ils peuvent être changés jusqu'à la publication de la décision.",
  "p.cutover.criteria_saved": "Les critères d'arrêt sont enregistrés.",
  "p.cutover.criteria_unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.cutover.edit_criteria_label": "Critères d'arrêt, un par ligne",
  "p.cutover.edit_criteria": "Modifier les critères d'arrêt",
  "p.cutover.problem_count": "Une décision demande de 1 à 30 critères d'arrêt.",
  "p.cutover.problem_length":
    "Le critère d'arrêt {line} doit compter de 5 à 500 caractères.",
  "p.cutover.save_criteria": "Enregistrer les critères d'arrêt",
  "p.moves.link": "Décalages de déploiement",
  "p.moves.title": "Décalages de déploiement — PPM",
  "p.moves.h1": "Décalages de déploiement",
  "p.moves.lede":
    "Une demande de décaler le début d'une tâche. Rien ne change tant que l'approbateur ne l'a pas approuvée. L'effet sur les autres tâches, et son coût de main-d'œuvre, ont été calculés au moment de la demande, en recalculant le calendrier avec la date décalée.",
  "p.moves.loading": "Chargement des décalages…",
  "p.moves.none": "Aucun décalage n'a été demandé.",
  "p.moves.request_h2": "Demander un décalage",
  "p.moves.task": "Tâche",
  "p.moves.new_start": "Nouveau début",
  "p.moves.reason": "Motif",
  "p.moves.approver": "Approbateur (person:<id> ou role:<name>, facultatif)",
  "p.moves.request": "Demander le décalage",
  "p.moves.col_task": "Tâche",
  "p.moves.col_dates": "Début",
  "p.moves.col_reason": "Motif",
  "p.moves.col_by": "Demandé par",
  "p.moves.col_approver": "Approbateur",
  "p.moves.col_status": "Statut",
  "p.moves.col_knock": "Effet induit",
  "p.moves.col_act": "Décider",
  "p.moves.status_requested": "en attente de l'approbateur",
  "p.moves.status_approved": "approuvé",
  "p.moves.status_refused": "refusé",
  "p.moves.knock":
    "{n} autres tâches sont décalées ; coût de main-d'œuvre {cost}",
  "p.moves.no_cost": "aucune variation du coût de main-d'œuvre",
  "p.moves.decision_reason": "Motif de la décision",
  "p.moves.approve": "Approuver",
  "p.moves.refuse": "Refuser",
  "p.moves.approve_for": "Approuver le décalage de {name}",
  "p.moves.refuse_for": "Refuser le décalage de {name}",
  "p.moves.decided": "{by} le {on} : {why}",
  "p.moves.by_requester_h2": "Coût induit par demandeur",
  "p.moves.requester_row": "{who} : {n} demandes ; main-d'œuvre {cost}",
  "p.moves.could_not_load": "Impossible de charger les décalages",
  "p.moves.could_not_save": "Impossible d'enregistrer",
  "p.ledger.nav": "Registre des financements",
  "p.ledger.title": "Registre des financements — PPM",
  "p.ledger.h1": "Registre des financements",
  "p.ledger.lede":
    "Chaque allocation de financement, rattachée aux lignes budgétaires qui y puisent, à leurs jalons de décision, aux jalons atteints et aux bénéfices validés. L'argent alloué mais affecté à aucune ligne que vous pouvez lire est non rattaché, et apparaît avec son montant.",
  "p.ledger.scope":
    "La création des allocations, la correspondance des codes de coût et le rapprochement d'un export financier se font via l'API ; cet écran consulte le registre.",
  "p.ledger.loading": "Chargement du registre…",
  "p.ledger.none": "Aucune allocation de financement n'a été enregistrée.",
  "p.ledger.col_source": "Source de financement",
  "p.ledger.col_ref": "Référence",
  "p.ledger.col_on": "Alloué le",
  "p.ledger.col_amount": "Alloué",
  "p.ledger.col_planned": "Prévu sur les lignes",
  "p.ledger.col_untraced": "Non rattaché",
  "p.ledger.col_lines": "Lignes",
  "p.ledger.col_act": "Tracer",
  "p.ledger.trace": "Tracer {name}",
  "p.ledger.trace_short": "Tracer",
  "p.ledger.trace_h2": "Ce que {name} a financé",
  "p.ledger.money":
    "Alloué {allocated}, prévu sur les lignes {planned}, dépensé {spent}, non rattaché {untraced} ({currency}).",
  "p.ledger.gates":
    "Lignes avec un jalon de décision : {gated}. Débloquées : {released}.",
  "p.ledger.milestones_met": "Jalons atteints : {met} sur {total}.",
  "p.ledger.benefits": "Bénéfices enregistrés : {n}.",
  "p.ledger.validated": "Validé en {currency} : {amount}",
  "p.ledger.lines_h3": "Lignes budgétaires",
  "p.ledger.line_plan": "Programme",
  "p.ledger.line_desc": "Description",
  "p.ledger.line_planned": "Prévu",
  "p.ledger.line_actual": "Dépensé",
  "p.ledger.line_gate": "Jalon de décision",
  "p.ledger.released": "débloqué",
  "p.ledger.held": "retenu",
  "p.ledger.no_gate": "aucun jalon de décision",
  "p.ledger.could_not_load": "Impossible de charger le registre",
  "p.ledger.could_not_trace": "Impossible de tracer l'allocation",
  "p.ledger.a_line":
    "une ligne budgétaire (choisissez son plan ci-dessus pour la voir)",
  "p.ledger.choose_line": "Choisissez une ligne",
  "p.ledger.choose_plan_first": "Choisissez d'abord un plan ci-dessus",
  "p.ledger.choose_plan": "Choisissez un plan",
  "p.ledger.choose_source": "Choisissez une source de financement",
  "p.ledger.code_mapped": "Code de coût rattaché.",
  "p.ledger.codes_heading": "Codes de coût",
  "p.ledger.codes_says":
    "Chaque code de coût de la comptabilité correspond à une ligne budgétaire, pour qu'un export comptable puisse être mis à côté du registre. Rattacher un code déjà rattaché le déplace.",
  "p.ledger.could_not_load_lines":
    "Les lignes budgétaires de ce plan n'ont pas pu être chargées.",
  "p.ledger.could_not_reconcile": "L'export n'a pas pu être rapproché.",
  "p.ledger.could_not_record": "L'allocation n'a pas été enregistrée.",
  "p.ledger.could_not_save": "La modification n'a pas été enregistrée.",
  "p.ledger.f_allocation": "Allocation sur laquelle elle tire",
  "p.ledger.f_amount": "Montant",
  "p.ledger.f_code_line": "Ligne budgétaire (du plan ci-dessus)",
  "p.ledger.f_code": "Code de coût",
  "p.ledger.f_csv": "Ou collez l'export (CSV)",
  "p.ledger.f_currency": "Devise",
  "p.ledger.f_file": "Fichier d'export comptable (CSV)",
  "p.ledger.f_on": "Allouée le",
  "p.ledger.f_plan": "Plan concerné",
  "p.ledger.f_reference": "Référence (facultative)",
  "p.ledger.f_source": "Source de financement",
  "p.ledger.line_detached": "La ligne ne tire plus sur aucune allocation.",
  "p.ledger.line_drawn": "La ligne tire maintenant sur cette allocation.",
  "p.ledger.lines_heading":
    "Lignes budgétaires et allocations sur lesquelles elles tirent",
  "p.ledger.lines_says":
    "Une ligne tire sur une allocation de sa propre source de financement et de sa devise ; définissez d'abord sa source sur la page Financement.",
  "p.ledger.map_code": "Rattacher le code de coût",
  "p.ledger.matched_heading": "Codes de coût rapprochés",
  "p.ledger.matched_line":
    "dans l'export {ledger}, dans le registre {register}, écart {difference}",
  "p.ledger.mismatch_heading":
    "Lignes dans une autre devise que leur ligne budgétaire",
  "p.ledger.mismatch_line":
    "ligne en {row}, ligne budgétaire en {line} : {amount}, non additionné",
  "p.ledger.needs_source": "a d'abord besoin d'une source de financement",
  "p.ledger.no_allocation": "Ne tire sur aucune allocation",
  "p.ledger.no_codes": "Aucun code de coût n'est encore rattaché.",
  "p.ledger.no_lines": "Ce plan n'a pas de ligne budgétaire.",
  "p.ledger.no_matching_allocation":
    "Pas encore d'allocation de cette source et de cette devise",
  "p.ledger.no_source": "aucune source de financement",
  "p.ledger.not_reconciled":
    "Non rapproché : quelque chose ne correspond pas. Rien n'a été écrit.",
  "p.ledger.problem_amount":
    "Écrivez le montant comme un nombre supérieur à 0.",
  "p.ledger.problem_code_line": "Choisissez la ligne budgétaire du code.",
  "p.ledger.problem_code": "Un code de coût compte de 1 à 60 caractères.",
  "p.ledger.problem_currency":
    "Écrivez la devise en trois lettres, par exemple GBP.",
  "p.ledger.problem_date_future":
    "La date d'allocation ne peut pas être dans le futur.",
  "p.ledger.problem_date": "Choisissez la date de l'allocation.",
  "p.ledger.problem_export_empty":
    "Choisissez d'abord un fichier ou collez l'export.",
  "p.ledger.problem_export_header":
    "La première ligne de l'export doit nommer cost_code, amount_minor et currency.",
  "p.ledger.problem_export_large": "L'export dépasse 1 Mo : découpez-le.",
  "p.ledger.problem_reference":
    "Une référence peut compter au plus 120 caractères.",
  "p.ledger.problem_source": "Choisissez une source de financement.",
  "p.ledger.reconcile_heading": "Rapprocher un export comptable",
  "p.ledger.reconcile_says":
    "Le code de coût de chaque ligne est cherché dans la table et ses montants sont additionnés à côté de ce que le registre tient pour dépensé sur la ligne. Rien n'est écrit.",
  "p.ledger.reconcile": "Rapprocher",
  "p.ledger.reconciled":
    "Rapproché : chaque ligne correspond à une ligne budgétaire, dans sa devise, sans écart.",
  "p.ledger.record_heading": "Enregistrer une allocation",
  "p.ledger.record_says":
    "L'argent qu'une source de financement a alloué. Les allocations sont gardées telles qu'enregistrées et jamais modifiées.",
  "p.ledger.record": "Enregistrer l'allocation",
  "p.ledger.recorded": "Allocation enregistrée.",
  "p.ledger.refused_export":
    "L'export n'a pas pu être lu : chaque ligne demande un code de coût, un montant entier en unités mineures et une devise, sans dépasser ce que le service accepte.",
  "p.ledger.refused_line":
    "La ligne ne peut pas tirer sur cette allocation : elles doivent avoir la même source de financement et la même devise.",
  "p.ledger.rows_read_one": "{n} ligne lue.",
  "p.ledger.rows_read_other": "{n} lignes lues.",
  "p.ledger.to_funding":
    "Ajouter des sources de financement sur la page Financement",
  "p.ledger.unmatched_heading": "Codes de coût sans ligne que vous pouvez lire",
  "p.signoffs.nav": "Validation des jalons",
  "p.signoffs.title": "Validation des jalons — PPM",
  "p.signoffs.h1": "Validation des jalons",
  "p.signoffs.lede":
    "Certains jalons ne sont atteints que lorsque les organisations partenaires qu'ils désignent les ont validés. « Livré » signifie que l'organisation de réalisation déclare le travail terminé ; « atteint » signifie livré avec toutes les validations en vigueur. Une validation retirée est de nouveau comptée comme en attente.",
  "p.signoffs.loading": "Chargement des validations…",
  "p.signoffs.none": "Aucun jalon ne nécessite la validation d'un partenaire.",
  "p.signoffs.totals":
    "À valider : {n}. Livrés : {delivered}. Atteints : {met}. Livrés mais non atteints : {gap}. Part des livrés qui sont atteints : {share}.",
  "p.signoffs.no_share": "non mesuré",
  "p.signoffs.col_milestone": "Jalon",
  "p.signoffs.col_due": "Échéance",
  "p.signoffs.col_state": "État",
  "p.signoffs.col_delivered": "Livré le",
  "p.signoffs.col_met": "Atteint le",
  "p.signoffs.col_outstanding": "En attente de",
  "p.signoffs.col_act": "Signer ou retirer",
  "p.signoffs.state_met": "atteint",
  "p.signoffs.state_delivered_not_met": "livré, non atteint",
  "p.signoffs.state_not_delivered": "non livré",
  "p.signoffs.organization": "Organisation",
  "p.signoffs.reason": "Motif (pour un retrait)",
  "p.signoffs.sign": "Signer",
  "p.signoffs.withdraw": "Retirer",
  "p.signoffs.sign_for": "Signer {name} au nom de l'organisation choisie",
  "p.signoffs.withdraw_for": "Retirer la validation de {name}",
  "p.signoffs.not_built":
    "Les organisations dont un jalon a besoin se choisissent sur la page de gouvernance de chaque plan ; cet écran affiche et enregistre les validations.",
  "p.signoffs.problem_inactive":
    "{name} n'est plus active et ne peut donc pas être exigée : décochez-la pour enregistrer.",
  "p.signoffs.problem_too_many":
    "Un jalon peut exiger au plus {max} organisations.",
  "p.signoffs.required_cancel": "Annuler",
  "p.signoffs.required_change_for": "Modifier qui doit valider {name}",
  "p.signoffs.required_change": "Modifier qui doit valider",
  "p.signoffs.required_cleared":
    "{name} n'a plus besoin de la validation d'un partenaire.",
  "p.signoffs.required_heading": "Validation des partenaires",
  "p.signoffs.required_inactive": "{name} (plus active)",
  "p.signoffs.required_legend":
    "Organisations qui doivent valider {name} (au plus {max})",
  "p.signoffs.required_link": "Enregistrer une validation",
  "p.signoffs.required_no_orgs": "Aucune organisation active à choisir.",
  "p.signoffs.required_none": "aucune validation de partenaire requise",
  "p.signoffs.required_save": "Enregistrer qui doit valider",
  "p.signoffs.required_saved": "Qui doit valider {name} est enregistré.",
  "p.signoffs.required_says":
    "Un jalon soumis à la validation de partenaires n'est atteint que lorsque chaque organisation ci-dessous a validé ; livré et atteint sont des dates distinctes.",
  "p.signoffs.required_signed": "{name} a validé le {date}",
  "p.signoffs.required_unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.signoffs.required_waiting": "{name} n'a pas encore validé",
  "p.signoffs.could_not_load": "Impossible de charger les validations",
  "p.signoffs.could_not_save": "Impossible d'enregistrer la validation",
  "p.key_persons.nav": "Personnes clés",
  "p.key_persons.title": "Personnes clés — PPM",
  "p.key_persons.h1": "Personnes clés et points de défaillance uniques",
  "p.key_persons.lede":
    "Les personnes et les rôles que le portefeuille ne peut pas perdre sans dommage, qui prendrait le relais et dans quelle mesure ces successeurs sont prêts. Seuls les gestionnaires de ressources peuvent consulter cette page, car elle concerne des personnes nommées.",
  "p.key_persons.loading": "Chargement…",
  "p.key_persons.none": "Aucune personne ni aucun rôle clé n'a été enregistré.",
  "p.key_persons.not_allowed":
    "Seuls les gestionnaires de ressources peuvent consulter les personnes clés.",
  "p.key_persons.add_h2": "Enregistrer une personne ou un rôle clé",
  "p.key_persons.subject_kind": "Personne ou rôle",
  "p.key_persons.kind_person": "personne",
  "p.key_persons.kind_role": "rôle",
  "p.key_persons.subject": "Nom ou rôle",
  "p.key_persons.successor": "Successeur",
  "p.key_persons.readiness": "Préparation du successeur",
  "p.key_persons.ready_ready": "prêt",
  "p.key_persons.ready_partly": "partiellement prêt",
  "p.key_persons.ready_none": "pas prêt",
  "p.key_persons.note": "Note",
  "p.key_persons.add": "Enregistrer",
  "p.key_persons.col_subject": "Personne ou rôle",
  "p.key_persons.col_successor": "Successeur",
  "p.key_persons.col_ready": "Préparation",
  "p.key_persons.col_note": "Note",
  "p.key_persons.col_act": "Action",
  "p.key_persons.remove": "Supprimer {name}",
  "p.key_persons.remove_short": "Supprimer",
  "p.key_persons.no_successor": "aucun successeur désigné",
  "p.key_persons.contingent_h2": "Travailleurs temporaires",
  "p.key_persons.contingent_note":
    "Les personnes marquées comme main-d'œuvre temporaire, susceptibles de partir à bref délai.",
  "p.key_persons.contingent_none": "Personne n'est marqué comme temporaire.",
  "p.key_persons.contingent_person": "Référence de la personne",
  "p.key_persons.contingent_unmark": "Retirer la marque de {name}",
  "p.key_persons.mark": "Marquer",
  "p.key_persons.unmark": "Retirer la marque",
  "p.key_persons.could_not_load": "Impossible de charger les personnes clés",
  "p.key_persons.could_not_save": "Impossible d'enregistrer",
  "p.organizations.nav": "Organisations",
  "p.organizations.title": "Organisations — PPM",
  "p.organizations.h1": "Organisations partenaires",
  "p.organizations.lede":
    "Les organisations qui réalisent, parrainent, fournissent ou s'associent au portefeuille. Un dossier peut être adressé à une organisation, un jalon peut nécessiter sa validation et elle peut engager de la capacité. Retirer une organisation conserve son historique.",
  "p.organizations.loading": "Chargement des organisations…",
  "p.organizations.none": "Aucune organisation n'a encore été ajoutée.",
  "p.organizations.add_h2": "Ajouter une organisation",
  "p.organizations.name": "Nom",
  "p.organizations.kind": "Type",
  "p.organizations.add": "Ajouter",
  "p.organizations.col_name": "Nom",
  "p.organizations.col_kind": "Type",
  "p.organizations.col_state": "État",
  "p.organizations.col_audience": "Destinataire de dossier",
  "p.organizations.col_act": "Action",
  "p.organizations.kind_delivery": "réalisation",
  "p.organizations.kind_partner": "partenaire",
  "p.organizations.kind_sponsor": "commanditaire",
  "p.organizations.kind_supplier": "fournisseur",
  "p.organizations.active": "active",
  "p.organizations.retired": "retirée",
  "p.organizations.retire": "Retirer {name}",
  "p.organizations.restore": "Rétablir {name}",
  "p.organizations.retire_short": "Retirer",
  "p.organizations.restore_short": "Rétablir",
  "p.organizations.could_not_load": "Impossible de charger les organisations",
  "p.organizations.could_not_save": "Impossible d'enregistrer l'organisation",
  "p.email_deliveries.nav": "Envois d'e-mails",
  "p.email_deliveries.title": "Envois d'e-mails — PPM",
  "p.email_deliveries.h1": "Envois d'e-mails",
  "p.email_deliveries.lede":
    "Chaque e-mail que le service a tenté d'envoyer, une ligne par destinataire : s'il a été envoyé, s'il attend une nouvelle tentative ou s'il a échoué définitivement, le nombre de tentatives et la dernière erreur. Seuls les administrateurs peuvent consulter cette page, car elle contient des adresses e-mail.",
  "p.email_deliveries.loading": "Chargement des envois…",
  "p.email_deliveries.none": "Aucun e-mail n'a été envoyé ni tenté.",
  "p.email_deliveries.counts":
    "Envoyés : {sent}. En attente de nouvelle tentative : {retry}. Échecs définitifs : {failed}.",
  "p.email_deliveries.filter": "Afficher",
  "p.email_deliveries.all": "Tous",
  "p.email_deliveries.col_recipient": "Destinataire",
  "p.email_deliveries.col_subject": "Objet",
  "p.email_deliveries.col_status": "Statut",
  "p.email_deliveries.col_attempts": "Tentatives",
  "p.email_deliveries.col_error": "Dernière erreur",
  "p.email_deliveries.col_next": "Prochaine tentative",
  "p.email_deliveries.col_sent": "Envoyé le",
  "p.email_deliveries.status_queued": "en file d'attente",
  "p.email_deliveries.status_retry": "nouvelle tentative prévue",
  "p.email_deliveries.status_sent": "envoyé",
  "p.email_deliveries.status_failed": "échec",
  "p.email_deliveries.not_allowed":
    "Seuls les administrateurs peuvent consulter les envois d'e-mails.",
  "p.email_deliveries.could_not_load":
    "Impossible de charger les envois d'e-mails",
  "p.director_pack.s_signoffs": "Validation des jalons",
  "p.director_pack.signoffs_note":
    "Certains jalons ne sont atteints que lorsque les organisations partenaires qu'ils désignent les ont validés. « Livré » signifie que l'organisation de réalisation déclare le travail terminé ; « atteint » signifie livré avec toutes les validations en vigueur. Une validation retirée est comptée comme en attente.",
  "p.director_pack.signoffs_totals":
    "À valider : {n}. Livrés : {delivered}. Atteints : {met}. Livrés mais non atteints : {gap}. Part des livrés qui sont atteints : {share}.",
  "p.director_pack.so_met": "atteint",
  "p.director_pack.so_delivered_not_met": "livré, non atteint",
  "p.director_pack.so_not_delivered": "non livré",
  "p.director_pack.so_no_share": "aucun livré pour l'instant",
  "p.director_pack.c_milestone": "Jalon",
  "p.director_pack.c_due": "Échéance",
  "p.director_pack.c_delivered_on": "Livré le",
  "p.director_pack.c_met_on": "Atteint le",
  "p.director_pack.c_outstanding": "Validation attendue de",
  "p.director_pack.c_delivery": "Réalisation",
  "p.director_pack.dm_not_stated": "non précisé",
  "p.director_pack.dm_central_product": "produit central",
  "p.director_pack.dm_central_platform_local_deployment":
    "plateforme centrale, déploiement local",
  "p.director_pack.dm_local_programme_central_standards":
    "programme local, normes centrales",
  "p.director_pack.dm_owner": "responsable : {name}",
  "p.director_pack.dm_not_ours": "non réalisé par l'organisation",
  "p.director_pack.s_flow": "Burn-up et flux (programmes produit)",
  "p.director_pack.c_scope": "Tâches créées",
  "p.director_pack.c_finished": "Tâches terminées",
  "p.director_pack.c_open": "Tâches ouvertes",
  "p.director_pack.c_throughput":
    "Terminées par semaine (quatre dernières semaines)",
  "p.director_pack.c_burnup":
    "Terminées, semaine par semaine (la plus ancienne d'abord)",
  "p.director_pack.no_flow":
    "{name} : un programme produit sans tâche pour l'instant, donc sans burn-up ni flux.",
  "p.director_pack.s_asks": "Demandes externes et leurs arbitrages",
  "p.director_pack.asks_note":
    "Une demande du commanditaire ne peut pas être approuvée tant qu'elle n'indique pas ce qu'elle remplace. Voici les demandes formulées sur la période.",
  "p.director_pack.asks_count":
    "Demandes formulées sur la période : {n}. Sans arbitrage indiqué pour l'instant : {without}.",
  "p.director_pack.ask_none": "aucun arbitrage indiqué pour l'instant",
  "p.director_pack.c_ask": "Demande",
  "p.director_pack.c_tradeoff": "Ce qu'elle remplace",
  "p.director_pack.register_line":
    "Programmes dont le registre est complet : {complete} sur {n}.",
  "p.director_pack.register_enforced":
    "Aucune ligne budgétaire n'est débloquée, et aucune allocation ferme n'est réservée, sur un programme dont le registre est incomplet.",
  "p.director_pack.register_not_enforced":
    "Un registre incomplet est un avertissement, pas un blocage : l'argent peut toujours circuler, et l'avertissement est audité.",
  "p.director_pack.s_register": "Programmes dont le registre est incomplet",
  "p.director_pack.c_score": "Complet",
  "p.director_pack.c_missing": "Manquant",
  "p.director_pack.rf_sro": "responsable principal",
  "p.director_pack.rf_partners": "partenaires",
  "p.director_pack.rf_stage": "étape",
  "p.director_pack.rf_baseline": "référence de calendrier",
  "p.director_pack.rf_target_date": "date cible",
  "p.director_pack.rf_budget": "budget",
  "p.director_pack.rf_funding_source": "source de financement",
  "p.director_pack.rf_confidence": "confiance dans la réalisation",
  "p.director_pack.rf_top_risks": "principaux risques",
  "p.director_pack.rf_dependencies": "dépendances",
  "p.director_pack.s_watermelon": "Indicateurs pastèque",
  "p.director_pack.wm_note":
    "Vert à l'extérieur, rouge à l'intérieur. Ces indicateurs sont lus à partir de ce dossier et du dossier précédemment publié tels qu'enregistrés, sans recalcul. Un indicateur sans rien à mesurer en indique la raison ; ce n'est pas zéro.",
  "p.director_pack.wm_gap":
    "Désaccord entre la confiance autoévaluée et la confiance indépendante",
  "p.director_pack.wm_late":
    "Découverte tardive : rouge ou ambre/rouge maintenant, vert dans le dossier précédent",
  "p.director_pack.wm_red": "Rouge sans décision enregistrée sur la période",
  "p.director_pack.wm_signoff":
    "Jalons atteints avec validation, parmi les livrés",
  "p.director_pack.wm_signoff_of": "{met} atteints sur {delivered} livrés",
  "p.director_pack.wm_not_measured": "non mesuré",
  "p.director_pack.c_measure": "Indicateur",
  "p.director_pack.c_programmes":
    "Programmes, ou raison de l'absence de mesure",
  "p.director_pack.wm_count": "Programmes : {n}",
  "p.director_pack.s_keypeople": "Personnes clés et main-d'œuvre temporaire",
  "p.director_pack.kp_note":
    "Les points de défaillance uniques sont comptés ici mais jamais nommés ; le registre lui-même est réservé aux gestionnaires de ressources.",
  "p.director_pack.kp_counts":
    "Points de défaillance uniques : {n} ({people} personnes, {roles} rôles). Sans successeur : {nosucc}. Successeur prêt : {ready} ; partiellement prêt : {partly} ; pas prêt : {none}.",
  "p.director_pack.kp_no_people": "personne dans ce vivier",
  "p.director_pack.c_headcount": "Personnes",
  "p.director_pack.c_contingent": "Temporaires",
  "p.director_pack.c_contingent_share": "Part de temporaires",
  "p.director_pack.s_triage": "Tri et seuils",
  "p.director_pack.tr_note":
    "Le dernier tri de chaque programme est affiché. Le franchissement d'un seuil ouvre une exception qui exige une suspension, une réinitialisation, un arrêt ou une dérogation avec date d'expiration ; rien n'est suspendu ni arrêté automatiquement.",
  "p.director_pack.tr_decisions":
    "Décisions sur la période : suspension {pause}, réinitialisation {reset}, arrêt {stop}, dérogation {override}.",
  "p.director_pack.tr_open":
    "Exceptions de seuil en attente de décision : {n}.",
  "p.director_pack.tr_override_line":
    "Dérogation en vigueur pour {programme} jusqu'au {expiry}.",
  "p.director_pack.tr_finish": "achever",
  "p.director_pack.tr_fix": "corriger",
  "p.director_pack.tr_pause": "suspendre",
  "p.director_pack.tr_stop": "arrêter",
  "p.director_pack.c_outcome": "Tri",
  "p.director_pack.c_triaged_on": "Trié le",
  "l.enum.pk_conf.threshold": "seuil franchi",
  "p.director_pack.s_cutovers": "Décisions de bascule",
  "p.director_pack.co_note":
    "Une bascule est décidée au regard de critères d'arrêt publiés avant la décision. Un refus ou un report pour raison de sécurité est une décision prise sur ces critères, et n'est pas compté comme un glissement.",
  "p.director_pack.co_counts":
    "Décidées sur la période : feu vert {go}, refus {nogo}, report pour raison de sécurité {delay}.",
  "p.director_pack.co_go": "feu vert",
  "p.director_pack.co_no_go": "refus",
  "p.director_pack.co_delay_for_safety": "report pour raison de sécurité",
  "p.director_pack.c_cutover": "Bascule",
  "p.director_pack.c_decision": "Décision",
  "p.director_pack.c_decided_on": "Décidé le",
  "p.director_pack.s_outcomes": "Résultats et désescalade",
  "p.director_pack.oc_note":
    "Le niveau d'escalade est celui fixé par le commanditaire, et n'est jamais calculé ici. La série ci-dessous compte les trimestres achevés au cours desquels chaque indicateur du tableau de bord et chaque critère sont remplis ; elle est proposée à la personne qui décide.",
  "p.director_pack.oc_level": "Niveau d'escalade : {level}, fixé le {on}.",
  "p.director_pack.oc_no_level": "Aucun niveau d'escalade n'a été enregistré.",
  "p.director_pack.oc_run":
    "Trimestres consécutifs remplis : {n}. La règle en exige {need}.",
  "p.director_pack.oc_rule_yes":
    "La règle est satisfaite. C'est au commanditaire de décider s'il faut descendre d'un niveau.",
  "p.director_pack.oc_rule_no": "La règle n'est pas encore satisfaite.",
  "p.director_pack.oc_met": "rempli",
  "p.director_pack.oc_not_met": "non rempli",
  "p.director_pack.oc_missing": "manquant",
  "p.director_pack.c_quarter": "Trimestre",
  "p.director_pack.c_why": "Pourquoi",
  "p.balance.balance": "Équilibre du portefeuille",
  "p.balance.balance_ppm": "Équilibre du portefeuille — PPM",
  "p.balance.intro":
    "La position de chaque portefeuille par rapport aux autres en matière de coût prévu, de valeur cible des bénéfices et de risque ouvert, et les déséquilibres qui découlent de ces chiffres.",
  "p.balance.loading": "Chargement…",
  "p.balance.could_not_load":
    "Impossible de charger l'équilibre du portefeuille",
  "p.balance.none":
    "Aucun coût, aucune valeur de bénéfice ni aucun risque n'est encore enregistré pour un portefeuille.",
  "p.balance.currency": "Devise",
  "p.balance.unassigned": "Aucun portefeuille",
  "p.balance.caption":
    "Chaque bulle est un portefeuille. En abscisse : coût prévu en {currency}. En ordonnée : valeur cible des bénéfices en {currency}. Surface : exposition aux risques ouverts.",
  "p.balance.axis_cost": "Coût prévu ({currency})",
  "p.balance.axis_value": "Valeur cible des bénéfices ({currency})",
  "p.balance.imbalances": "Déséquilibres",
  "p.balance.no_imbalance":
    "Aucune des règles ci-dessous ne signale de déséquilibre dans ces chiffres.",
  "p.balance.rules":
    "Un signalement est levé lorsqu'un portefeuille détient {concentration} % ou plus du coût prévu ou du risque ouvert (avec au moins deux portefeuilles), lorsque sa part du coût prévu ou du risque dépasse de {gap} points de pourcentage ou plus sa part de la valeur cible, ou lorsqu'il a un coût prévu sans valeur cible dans la devise choisie. Un signalement invite à examiner, ce n'est pas un verdict.",
  "p.balance.flag_cost_concentration":
    "{name} détient {percent} % du coût prévu.",
  "p.balance.flag_risk_concentration":
    "{name} détient {percent} % de l'exposition aux risques ouverts.",
  "p.balance.flag_cost_ahead_of_value":
    "{name} a une part du coût prévu nettement supérieure à sa part de {percent} % de la valeur cible.",
  "p.balance.flag_risk_ahead_of_value":
    "{name} a une part du risque ouvert nettement supérieure à sa part de {percent} % de la valeur cible.",
  "p.balance.flag_no_value":
    "{name} a un coût prévu mais aucune valeur cible enregistrée en {currency}.",
  "p.balance.table": "Les mêmes chiffres sous forme de tableau",
  "p.balance.portfolio": "Portefeuille",
  "p.balance.cost": "Coût prévu ({currency})",
  "p.balance.value": "Valeur cible ({currency})",
  "p.balance.risk": "Exposition aux risques ouverts",
  "p.balance.cost_share": "Part du coût",
  "p.balance.value_share": "Part de la valeur",
  "p.balance.risk_share": "Part du risque",
  "p.balance.flags": "Signalements",
  "p.balance.no_flag": "aucun",
  "p.balance.notes":
    "Regroupé par portefeuille, et non par catégorie ou objectif stratégique : le portefeuille est le seul regroupement selon lequel le coût, la valeur des bénéfices et le risque sont tous déclarés. Le coût est le montant prévu sur les lignes budgétaires, la valeur est la cible des bénéfices enregistrés et le risque est le score d'exposition ouvert ; les montants en devises différentes ne sont jamais additionnés.",
  "p.roadmap.title":
    "Feuille de route — impact, effort et Maintenant / Ensuite / Plus tard",
  "p.roadmap.h1":
    "Feuille de route : impact, effort et Maintenant / Ensuite / Plus tard",
  "p.roadmap.lede":
    "Les idées ouvertes et les propositions encore en cours de décision, placées selon leur importance (impact) et la quantité de travail qu'elles demandent (effort). Le service ne stocke ni impact ni effort pour une idée ou une proposition ; vous évaluez donc chacune ici. Vos évaluations sont conservées uniquement dans ce navigateur et ne sont ni partagées ni envoyées au service. Un élément sans les deux évaluations n'est pas placé.",
  "p.roadmap.loading": "Chargement des idées et des propositions…",
  "p.roadmap.matrix_h2": "Matrice impact-effort",
  "p.roadmap.matrix_caption":
    "Impact en lignes, effort en colonnes. Chaque cellule liste les éléments évalués ainsi.",
  "p.roadmap.impact": "Impact",
  "p.roadmap.effort": "Effort",
  "p.roadmap.impact_level": "Impact {level}",
  "p.roadmap.effort_level": "Effort {level}",
  "p.roadmap.level_low": "Faible",
  "p.roadmap.level_medium": "Moyen",
  "p.roadmap.level_high": "Élevé",
  "p.roadmap.rate_h2": "Évaluer les éléments",
  "p.roadmap.rate_hint":
    "Ce tableau est l'accès clavier à la matrice : choisissez un impact et un effort pour chaque élément, et il se déplace dans la matrice et dans les listes Maintenant / Ensuite / Plus tard.",
  "p.roadmap.col_item": "Élément",
  "p.roadmap.col_type": "Type",
  "p.roadmap.col_horizon": "Quand",
  "p.roadmap.kind_idea": "Idée",
  "p.roadmap.kind_proposal": "Proposition",
  "p.roadmap.not_rated": "Non évalué",
  "p.roadmap.rate_impact": "Impact de {title}",
  "p.roadmap.rate_effort": "Effort de {title}",
  "p.roadmap.no_items":
    "Il n'y a aucune idée ni proposition ouverte à évaluer.",
  "p.roadmap.horizons_h2": "Maintenant / Ensuite / Plus tard",
  "p.roadmap.horizon_rule":
    "Maintenant : l'impact dépasse l'effort. Ensuite : l'impact égale l'effort et est au moins moyen. Plus tard : tout le reste des éléments évalués. Pas encore évalué : aucun impact ou aucun effort choisi.",
  "p.roadmap.h_now": "Maintenant",
  "p.roadmap.h_next": "Ensuite",
  "p.roadmap.h_later": "Plus tard",
  "p.roadmap.h_unrated": "Pas encore évalué",
  "p.roadmap.nothing_here": "Rien ici.",
  "p.roadmap.see_ideas":
    "Voir les idées et les propositions sur la feuille de route",
  "p.people_impact.title":
    "Impact sur les personnes (depuis le service des effectifs)",
  "p.people_impact.intro":
    "Uniquement des décomptes, issus du service des effectifs (WPM), demandés avec votre propre connexion. Aucune personne n'est nommée et rien n'est additionné.",
  "p.people_impact.ask": "Interroger le service des effectifs",
  "p.people_impact.asking": "Interrogation en cours…",
  "p.people_impact.failed": "Impossible de charger l'impact sur les personnes.",
  "p.people_impact.not_known": "Inconnu.",
  "p.people_impact.source": "Depuis le service des effectifs, au {date}.",
  "p.people_impact.unknown": "Inconnu : {reason}",
  "p.people_impact.end_sent":
    "Date de fin envoyée : {date} (date cible de ce plan).",
  "p.people_impact.committed": "Personnes engagées, par vivier",
  "p.people_impact.ending":
    "Engagements à durée déterminée et contrats se terminant avant la fin",
  "p.people_impact.in_no_pool": "Dans aucun vivier",
  "p.people_impact.end_not_recorded":
    "Aucune fin enregistrée (non compté comme se terminant)",
  "p.people_impact.hidden": "masqué (moins de {floor})",
  "p.people_impact.no_pools": "Aucun vivier à afficher.",
  "p.people_impact.part_unknown": "Inconnu : {reason}",
  "p.people_impact.over": "Mois-viviers surengagés",
  "p.people_impact.over_counts":
    "{over} mois-viviers sur {considered} sont surengagés ; {unknown} ne peuvent pas être déterminés.",
  "p.people_impact.no_total":
    "Aucun total n'est donné : une personne peut compter dans plusieurs viviers.",
  "l.enum.pk_heat.supply_undeclared": "disponibilité non déclarée",
  "l.enum.pk_heat.supply_unknown": "offre inconnue",
  "p.capacity_heatmap.nav": "Carte de chaleur de la capacité",
  "p.capacity_heatmap.title": "Carte de chaleur de la capacité — PPM",
  "p.capacity_heatmap.h1": "Capacité par vivier de compétences et par mois",
  "p.capacity_heatmap.lede":
    "Chaque case compare le travail demandé à un vivier de compétences au cours d'un mois avec les personnes disponibles, en équivalents temps plein (FTE).",
  "p.capacity_heatmap.bridge_note":
    "Un vivier relié au service des effectifs en tire son offre. Ce qui a été saisi ici pour ce vivier est affiché à côté et n'y est jamais ajouté. Un mois sans chiffre du service des effectifs est affiché comme inconnu, jamais comme zéro.",
  "p.capacity_heatmap.from": "Premier mois",
  "p.capacity_heatmap.months": "Nombre de mois",
  "p.capacity_heatmap.show": "Afficher",
  "p.capacity_heatmap.loading": "Chargement de la carte de chaleur…",
  "p.capacity_heatmap.could_not_load":
    "Impossible de charger la carte de chaleur de la capacité.",
  "p.capacity_heatmap.none":
    "Aucun vivier de compétences ni aucune demande à afficher.",
  "p.capacity_heatmap.caption":
    "Demande et offre en équivalents temps plein, par vivier de compétences et par mois",
  "p.capacity_heatmap.col_pool": "Vivier de compétences",
  "p.capacity_heatmap.linked_to": "Relié au service des effectifs",
  "p.capacity_heatmap.demand": "Demande : {fte}",
  "p.capacity_heatmap.supply": "Offre : {fte}",
  "p.capacity_heatmap.gap": "Écart : {fte}",
  "p.capacity_heatmap.unknown": "inconnu",
  "p.capacity_heatmap.not_declared": "non déclarée",
  "p.capacity_heatmap.none_recorded": "rien d'enregistré",
  "p.capacity_heatmap.source_wpm": "Offre issue du service des effectifs",
  "p.capacity_heatmap.source_here": "Offre saisie ici",
  "p.capacity_heatmap.peer_live": "En direct, relevé {when}.",
  "p.capacity_heatmap.peer_live_no_time": "En direct.",
  "p.capacity_heatmap.peer_last":
    "Dernière réception {age} (relevé {when}) ; plus en direct : {reason}",
  "p.capacity_heatmap.peer_hidden":
    "Masqué : le service des effectifs masque un mois comptant moins de travailleurs que son seuil.",
  "p.capacity_heatmap.peer_unknown": "Inconnu : {reason}",
  "p.capacity_heatmap.no_reason": "aucun chiffre reçu du service des effectifs",
  "p.capacity_heatmap.entered_here":
    "Saisi ici : {fte} (affiché à côté, non ajouté)",
  "p.capacity_heatmap.entered_here_none": "Saisi ici : aucun",
  "p.capacity_heatmap.partners_short":
    "les partenaires ont livré moins que leur engagement",
  "p.capacity_heatmap.no_pool": "Aucun vivier attribué",
  "p.capacity_heatmap.key":
    "▲ déficit · ◆ tendu · ● suffisant · ○ aucune demande · – aucune offre enregistrée · ? offre inconnue. Le mot est toujours écrit aussi.",
  "p.capacity_heatmap.every_figure": "Tous les chiffres sous forme de tableau",
  "p.capacity_heatmap.table_label":
    "Capacité par vivier de compétences et par mois",
  "p.capacity_heatmap.c_pool": "Vivier de compétences",
  "p.capacity_heatmap.c_month": "Mois",
  "p.capacity_heatmap.c_reading": "Lecture",
  "p.capacity_heatmap.c_demand": "Demande (équivalent temps plein)",
  "p.capacity_heatmap.c_supply": "Offre (équivalent temps plein)",
  "p.capacity_heatmap.c_source": "Offre issue de",
  "p.capacity_heatmap.c_status": "Relevé du service des effectifs",
  "p.capacity_heatmap.c_entered_here": "Saisi ici",
  "p.capacity_heatmap.see_pools":
    "Relier des viviers de compétences au service des effectifs",
  "p.skill_pools.nav": "Viviers de compétences",
  "p.skill_pools.title": "Viviers de compétences — PPM",
  "p.skill_pools.h1": "Viviers de compétences",
  "p.skill_pools.lede":
    "Un vivier de compétences regroupe les rôles et les compétences capables d'effectuer le même travail. Vous pouvez ici relier un vivier au vivier correspondant du service des effectifs.",
  "p.skill_pools.rule":
    "Le lien doit être établi des deux côtés, ici et dans le service des effectifs, par une personne qui voit les deux. Il n'est jamais établi par correspondance de nom. Tant qu'un vivier est relié, son offre sur la carte de chaleur de la capacité provient du service des effectifs et sa demande y est envoyée.",
  "p.skill_pools.loading": "Chargement des viviers de compétences…",
  "p.skill_pools.could_not_load":
    "Impossible de charger les viviers de compétences.",
  "p.skill_pools.none": "Aucun vivier de compétences n'a encore été créé.",
  "p.skill_pools.col_name": "Nom",
  "p.skill_pools.col_roles": "Rôles",
  "p.skill_pools.col_skills": "Compétences",
  "p.skill_pools.col_state": "État",
  "p.skill_pools.col_link": "Vivier du service des effectifs",
  "p.skill_pools.col_act": "Modifier",
  "p.skill_pools.active": "actif",
  "p.skill_pools.retired": "retiré",
  "p.skill_pools.not_linked": "non relié",
  "p.skill_pools.change_short": "Modifier le lien",
  "p.skill_pools.change": "Modifier le lien de {name}",
  "p.skill_pools.edit_h2": "Relier {name} au service des effectifs",
  "p.skill_pools.ref_label": "Id du vivier dans le service des effectifs",
  "p.skill_pools.ref_hint":
    "Copiez-le depuis le vivier dans le service des effectifs : 36 caractères, en minuscules, avec des tirets, comme 3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b.",
  "p.skill_pools.ref_invalid":
    "Ce n'est pas un id de vivier du service des effectifs : il doit comporter 36 caractères, en minuscules, avec des tirets.",
  "p.skill_pools.save": "Enregistrer le lien",
  "p.skill_pools.clear": "Supprimer le lien",
  "p.skill_pools.cancel": "Annuler",
  "p.skill_pools.confirm_clear":
    "Supprimer le lien entre {name} et le service des effectifs ? Son offre sur la carte de chaleur redeviendra celle saisie ici. Chaque chiffre de demande déjà envoyé au service des effectifs pour ce vivier y sera retiré (envoyé à zéro).",
  "p.skill_pools.saved":
    "{name} est maintenant relié au vivier {ref} du service des effectifs.",
  "p.skill_pools.cleared": "{name} n'est plus relié au service des effectifs.",
  "p.skill_pools.taken":
    "Un autre vivier ici est déjà relié à ce vivier du service des effectifs. Supprimez d'abord ce lien : un vivier ici pour un vivier là-bas.",
  "p.skill_pools.invalid":
    "Le service n'a pas accepté cet id. Il doit s'agir de l'id de vivier du service des effectifs : 36 caractères, en minuscules, avec des tirets.",
  "p.skill_pools.missing": "Ce vivier n'existe plus. La liste a été rechargée.",
  "p.skill_pools.could_not_save": "Impossible de modifier le lien.",
  "p.skill_pools.see_heatmap": "Voir la carte de chaleur de la capacité",
  "p.skill_pools.withdrawing":
    "Chiffres de demande en cours de retrait du vivier du service des effectifs auquel il était relié : {n}.",
  "p.skill_pools.add_heading": "Ajouter un vivier",
  "p.skill_pools.add_says":
    "Un vivier regroupe des rôles et des compétences, pour que la carte de chaleur des capacités compte une seule fois leur demande et leur offre. Un rôle ou une compétence est dans un vivier au plus.",
  "p.skill_pools.add": "Ajouter le vivier",
  "p.skill_pools.added": "Vivier {name} ajouté.",
  "p.skill_pools.choose_pool": "Choisissez un vivier",
  "p.skill_pools.clash_item": "{value} (dans {pool})",
  "p.skill_pools.conflict_members":
    "Un vivier porte déjà ce nom, ou l'un de ces rôles ou compétences est déjà dans un autre vivier.",
  "p.skill_pools.could_not_add": "Le vivier n'a pas été ajouté.",
  "p.skill_pools.f_name": "Nom du vivier",
  "p.skill_pools.f_pool": "Vivier",
  "p.skill_pools.f_roles": "Rôles",
  "p.skill_pools.f_skills": "Compétences",
  "p.skill_pools.list_says":
    "Séparez les rôles ou compétences par des virgules ou des retours à la ligne ; une répétition est gardée une fois.",
  "p.skill_pools.members_heading": "Modifier un vivier",
  "p.skill_pools.members_saved":
    "Les rôles et compétences de {name} sont enregistrés.",
  "p.skill_pools.problem_clash": "Déjà dans un autre vivier : {list}.",
  "p.skill_pools.problem_name_taken": "Un vivier porte déjà ce nom.",
  "p.skill_pools.problem_name":
    "Le nom d'un vivier compte de 2 à 120 caractères.",
  "p.skill_pools.restore": "Rétablir le vivier",
  "p.skill_pools.restored": "Vivier {name} rétabli.",
  "p.skill_pools.retire_says":
    "Un vivier retiré reste dans l'historique et garde ses rôles et compétences ; le rétablir le compte de nouveau.",
  "p.skill_pools.retire": "Retirer le vivier",
  "p.skill_pools.retired_done": "Vivier {name} retiré.",
  "p.skill_pools.save_members": "Enregistrer les rôles et compétences",
  "p.skill_pools.unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.capacity_heatmap.set_commitment": "Définir un engagement de capacité",
  "p.commitments.title": "Engagements de capacité — PPM",
  "p.commitments.h1": "Engagements de capacité",
  "p.commitments.lede":
    "Enregistrez la capacité promise à un programme (offre) ou le travail dont un programme a besoin de la part d'un vivier de compétences (demande), pour un mois, en équivalents temps plein.",
  "p.commitments.bridge_note":
    "Pour un vivier relié au service des effectifs, un engagement de demande y envoie aussi la demande totale du programme pour ce vivier et ce mois. Un total de zéro y retire le chiffre. Aucune nouvelle tentative n'est faite : enregistrez de nouveau pour renvoyer.",
  "p.commitments.see_heatmap": "Voir la carte de chaleur de la capacité",
  "p.commitments.from_plan": "Engagements de capacité pour ce programme",
  "p.commitments.form_h2": "Définir un engagement",
  "p.commitments.choose": "Choisir…",
  "p.commitments.pool": "Vivier de compétences",
  "p.commitments.pool_linked": "{name} (relié au service des effectifs)",
  "p.commitments.pool_hint": "Seuls les viviers actifs sont listés.",
  "p.commitments.pool_missing": "Choisissez un vivier de compétences.",
  "p.commitments.plan": "Programme",
  "p.commitments.plan_hint":
    "Le plan doit être un programme (un plan de type Program).",
  "p.commitments.plan_missing": "Choisissez un programme.",
  "p.commitments.kind": "Objet de l'engagement",
  "p.commitments.kind_demand":
    "Demande : travail dont le programme a besoin de la part du vivier",
  "p.commitments.kind_supply": "Offre : capacité promise au programme",
  "p.commitments.organization": "Organisation partenaire (facultatif)",
  "p.commitments.organization_hint":
    "Laissez vide pour votre propre organisation.",
  "p.commitments.month": "Mois",
  "p.commitments.month_hint": "Un mois : année et mois.",
  "p.commitments.month_invalid":
    "Indiquez un mois sous la forme année et mois.",
  "p.commitments.committed": "Engagé (équivalent temps plein)",
  "p.commitments.committed_hint":
    "Un nombre de 0 à 1000, écrit comme vous écrivez habituellement les nombres. Zéro retire un chiffre de demande envoyé au service des effectifs.",
  "p.commitments.delivered": "Livré (équivalent temps plein, facultatif)",
  "p.commitments.delivered_hint":
    "Ce qui a réellement été livré, s'il est connu.",
  "p.commitments.fte_invalid":
    "Un nombre de 0 à 1000, écrit comme vous écrivez habituellement les nombres.",
  "p.commitments.note": "Note (facultatif)",
  "p.commitments.save": "Enregistrer l'engagement",
  "p.commitments.saved": "Enregistré : {pool}, {plan}, {month}.",
  "p.commitments.not_found":
    "Ce programme est introuvable, ou vous ne pouvez pas le consulter.",
  "p.commitments.refused":
    "Le service ne l'a pas accepté. Le plan doit être un programme, le vivier doit être actif, et chaque chiffre doit être compris entre 0 et 1000.",
  "p.commitments.could_not_save": "Impossible d'enregistrer l'engagement.",
  "p.commitments.could_not_load":
    "Impossible de charger les engagements de capacité.",
  "p.commitments.loading": "Chargement des envois récents…",
  "p.commitments.sends_h2": "Envois récents au service des effectifs",
  "p.commitments.sends_note":
    "Le dernier envoi pour chaque programme, vivier et mois, du plus récent au plus ancien. Un envoi lancé affiche ici sa réponse dès qu'elle arrive : actualisez pour la voir.",
  "p.commitments.refresh": "Actualiser",
  "p.commitments.no_sends":
    "Rien n'a encore été envoyé au service des effectifs.",
  "p.commitments.c_pool": "Vivier de compétences",
  "p.commitments.c_programme": "Programme",
  "p.commitments.c_month": "Mois",
  "p.commitments.c_sent": "Envoyé",
  "p.commitments.c_outcome": "Résultat",
  "p.commitments.c_at": "Quand",
  "p.commitments.sent_fte": "{fte} équivalent temps plein",
  "p.commitments.sent_withdrawal": "un retrait (zéro)",
  "p.commitments.sent_nothing": "rien",
  "p.commitments.out_not_sent_at_all":
    "Cet engagement n'est pas envoyé au service des effectifs : seule la demande sur un vivier qui y est relié l'est.",
  "p.commitments.out_none_yet":
    "Rien n'a encore été envoyé au service des effectifs pour ce programme, ce vivier et ce mois.",
  "p.commitments.out_started":
    "Envoi de {fte} équivalent temps plein (le total du programme pour ce vivier et ce mois) au service des effectifs. Sa réponse apparaît sous Envois récents.",
  "p.commitments.out_started_zero":
    "Retrait du chiffre de ce programme pour ce vivier et ce mois auprès du service des effectifs, car le total est désormais nul. Sa réponse apparaît sous Envois récents.",
  "p.commitments.out_sent":
    "Le service des effectifs a pris en compte {fte} équivalent temps plein.",
  "p.commitments.out_sent_zero":
    "Le service des effectifs a retiré son chiffre pour ce programme, ce vivier et ce mois.",
  "p.commitments.out_duplicate":
    "Le service des effectifs disposait déjà de ce chiffre ; rien n'y a changé.",
  "p.commitments.out_refused":
    "Le service des effectifs l'a refusé. Aucune nouvelle tentative n'est faite ; enregistrez de nouveau pour renvoyer.",
  "p.commitments.out_unreachable":
    "Le service des effectifs était injoignable ou n'a pas répondu à temps. Aucune nouvelle tentative n'est faite ; enregistrez de nouveau pour renvoyer.",
  "p.commitments.out_not_configured":
    "Non envoyé : ce service n'est pas configuré pour joindre le service des effectifs.",
  "p.commitments.out_no_token":
    "Non envoyé : aucune connexion à transmettre au service des effectifs.",
  "p.commitments.out_too_large":
    "Non envoyé : le service des effectifs accepte au maximum 10 000 équivalents temps plein.",
  "p.commitments.out_old_zero":
    "Non envoyé : un zéro enregistré avant que les zéros ne soient envoyés ; le service des effectifs a donc conservé son dernier chiffre. Enregistrez de nouveau pour le retirer.",
  "p.commitments.out_other": "Le service a enregistré : {outcome}.",
  "p.discussion.heading": "Échanges",
  "p.discussion.loading": "Chargement…",
  "p.discussion.could_not_load_comments":
    "Les commentaires n'ont pas pu être chargés.",
  "p.discussion.could_not_load_reactions":
    "Les réactions n'ont pas pu être chargées.",
  "p.discussion.could_not_load_files":
    "Les fichiers n'ont pas pu être chargés.",
  "p.discussion.could_not_load_watch":
    "Impossible de savoir si vous suivez ceci.",
  "p.discussion.sign_in_to_follow":
    "Connectez-vous pour suivre ceci et être averti des nouveaux commentaires et changements.",
  "p.discussion.you_follow":
    "Vous suivez ceci : vous êtes averti des nouveaux commentaires et changements.",
  "p.discussion.you_do_not_follow": "Vous ne suivez pas ceci.",
  "p.discussion.followers_one": "{n} personne le suit.",
  "p.discussion.followers_other": "{n} personnes le suivent.",
  "p.discussion.follow": "Suivre",
  "p.discussion.unfollow": "Ne plus suivre",
  "p.discussion.followed": "Vous suivez désormais ceci.",
  "p.discussion.unfollowed": "Vous ne suivez plus ceci.",
  "p.discussion.reactions_label": "Réactions",
  "p.discussion.reaction_named_one": "{reaction} : {n} personne",
  "p.discussion.reaction_named_other": "{reaction} : {n} personnes",
  "p.discussion.r_thumbs_up": "Pouce levé",
  "p.discussion.r_heart": "Cœur",
  "p.discussion.r_celebrate": "Fête",
  "p.discussion.r_eyes": "Je regarde",
  "p.discussion.r_question": "Une question",
  "p.discussion.comments_heading": "Commentaires",
  "p.discussion.no_comments": "Aucun commentaire pour l'instant.",
  "p.discussion.no_author": "quelqu'un de non connecté",
  "p.discussion.by_at": "Par {who}, {when}",
  "p.discussion.edited_at": "modifié {when}",
  "p.discussion.edit": "Modifier",
  "p.discussion.edit_named": "Modifier le commentaire de {who}, {when}",
  "p.discussion.delete": "Supprimer",
  "p.discussion.delete_named": "Supprimer le commentaire de {who}, {when}",
  "p.discussion.confirm_delete_comment":
    "Supprimer ce commentaire ? Ses versions antérieures sont conservées, mais personne ne le verra plus sur la page.",
  "p.discussion.earlier": "Versions antérieures",
  "p.discussion.no_earlier": "Aucune version antérieure n'est conservée.",
  "p.discussion.replaced_at": "Remplacé {when}",
  "p.discussion.f_edit": "Modifier le texte du commentaire",
  "p.discussion.save_edit": "Enregistrer le commentaire",
  "p.discussion.cancel": "Annuler",
  "p.discussion.f_comment": "Votre commentaire",
  "p.discussion.chars_left": "Il reste {n} caractères sur {max}.",
  "p.discussion.chars_over": "{n} caractères au-delà de la limite de {max}.",
  "p.discussion.post": "Publier le commentaire",
  "p.discussion.posted": "Le commentaire a été publié.",
  "p.discussion.edited":
    "Le commentaire a été modifié ; sa version antérieure est conservée.",
  "p.discussion.deleted": "Le commentaire a été supprimé.",
  "p.discussion.problem_blank": "Écrivez d'abord le commentaire.",
  "p.discussion.problem_too_long":
    "Un commentaire peut compter au plus 4000 caractères.",
  "p.discussion.files_heading": "Fichiers",
  "p.discussion.files_intro":
    "Les fichiers joints ici sont enregistrés sur votre ordinateur quand vous les ouvrez ; ils ne sont jamais affichés dans cette page.",
  "p.discussion.no_files": "Aucun fichier joint.",
  "p.discussion.file_facts": "{type}, {size}, ajouté par {who}, {when}",
  "p.discussion.remove_file": "Retirer",
  "p.discussion.remove_file_named": "Retirer le fichier {name}",
  "p.discussion.confirm_remove_file":
    "Retirer le fichier {name} ? Personne ne pourra plus le télécharger d'ici.",
  "p.discussion.removed_file": "Le fichier {name} a été retiré.",
  "p.discussion.f_file": "Fichier à joindre",
  "p.discussion.files_rules":
    "Un PDF, une image (PNG, JPEG, GIF ou WebP), du texte brut, CSV, Markdown, JSON ou un fichier Word, Excel ou PowerPoint, de 10 Mo au plus. Les pages web, dessins SVG, archives et programmes sont refusés.",
  "p.discussion.attach": "Joindre le fichier",
  "p.discussion.attached": "Le fichier {name} a été joint.",
  "p.discussion.problem_choose_file": "Choisissez d'abord un fichier.",
  "p.discussion.problem_empty": "Ce fichier est vide.",
  "p.discussion.problem_too_large":
    "Ce fichier dépasse 10 Mo, la limite pour un fichier.",
  "p.discussion.problem_type":
    "Ce type de fichier ne peut pas être joint. La liste ci-dessous indique les types acceptés.",
  "p.discussion.type_pdf": "Document PDF",
  "p.discussion.type_png": "Image PNG",
  "p.discussion.type_jpeg": "Image JPEG",
  "p.discussion.type_gif": "Image GIF",
  "p.discussion.type_webp": "Image WebP",
  "p.discussion.type_text": "texte brut",
  "p.discussion.type_csv": "tableau CSV",
  "p.discussion.type_markdown": "texte Markdown",
  "p.discussion.type_json": "données JSON",
  "p.discussion.type_word": "document Word",
  "p.discussion.type_excel": "classeur Excel",
  "p.discussion.type_powerpoint": "diapositives PowerPoint",
  "p.discussion.size_b": "{n} octets",
  "p.discussion.size_kb": "{n} Ko",
  "p.discussion.size_mb": "{n} Mo",
  "p.discussion.refused_sign_in":
    "Connectez-vous d'abord : le service ne sait pas qui vous êtes.",
  "p.discussion.refused_not_author":
    "Seule la personne qui a écrit un commentaire peut le modifier ou le supprimer.",
  "p.discussion.refused_not_uploader":
    "Seule la personne qui a joint un fichier, ou un administrateur, peut le retirer.",
  "p.discussion.refused_forbidden": "Vous ne pouvez ni lire ni modifier ceci.",
  "p.discussion.refused_comment_gone":
    "Ce commentaire n'existe plus ; les commentaires ont été relus.",
  "p.discussion.refused_file_gone":
    "Ce fichier avait déjà été retiré ; les fichiers ont été relus.",
  "p.discussion.refused_gone":
    "Introuvable : cela a peut-être été retiré, ou vous ne pouvez pas le voir.",
  "p.discussion.refused_comment_invalid":
    "Le service n'a pas accepté le commentaire : il ne doit pas être vide et peut compter au plus 4000 caractères.",
  "p.discussion.refused_file_invalid":
    "Le service n'a pas accepté le fichier : son type, sa taille ou son contenu n'ont pas passé les contrôles (un fichier doit être ce que dit son type, et aucun programme n'est accepté).",
  "p.discussion.refused_too_large":
    "Le fichier est trop volumineux pour être envoyé.",
  "p.discussion.refused_scan_unavailable":
    "Le fichier n'a pas été joint : l'analyseur de logiciels malveillants n'a pas répondu. Réessayez plus tard.",
  "p.me.timesheet_link": "Ma feuille de temps",
  "p.me.timesheet_note":
    "— votre temps par jour et par plan, semaine après semaine, et l'envoi d'une semaine pour approbation.",
  "p.timesheet.title_ppm": "Ma feuille de temps — PPM",
  "p.timesheet.heading": "Ma feuille de temps",
  "p.timesheet.back": "← Mon travail",
  "p.timesheet.intro":
    "Le temps que vous avez saisi pour une semaine, par jour et par plan, en heures et minutes, le temps facturable et non facturable toujours séparés. Envoyez une semaine terminée pour approbation ; une fois approuvée, elle est définitive et plus aucun temps ne peut y être saisi.",
  "p.timesheet.loading": "Chargement…",
  "p.timesheet.week_nav": "Choisir la semaine",
  "p.timesheet.previous_week": "Semaine précédente",
  "p.timesheet.this_week": "Cette semaine",
  "p.timesheet.next_week": "Semaine suivante",
  "p.timesheet.f_week": "Semaine, par exemple 2026-W46",
  "p.timesheet.show_week": "Afficher la semaine",
  "p.timesheet.week_hint":
    "Les semaines vont du lundi au dimanche et sont numérotées selon la norme internationale, quel que soit votre premier jour de la semaine. Nous sommes le {today} dans votre fuseau horaire, {tz}.",
  "p.timesheet.problem_week":
    "Ce n'est pas une semaine : écrivez l'année, ‘-W’ et le numéro de la semaine, par exemple 2026-W46.",
  "p.timesheet.mine_heading": "Votre semaine {week}",
  "p.timesheet.mine_caption":
    "Votre temps de la semaine {week}, par plan et par jour",
  "p.timesheet.status_open": "Ouverte : pas encore envoyée pour approbation",
  "p.timesheet.status_submitted":
    "Envoyée pour approbation : en attente d'une décision",
  "p.timesheet.status_approved":
    "Approuvée : définitive et verrouillée contre tout nouveau temps",
  "p.timesheet.status_rejected":
    "Renvoyée : modifiez-la et envoyez-la de nouveau",
  "p.timesheet.rejected_because": "l'approbateur a écrit : {comment}",
  "p.timesheet.decision_comment": "avec la note : {comment}",
  "p.timesheet.no_entries":
    "Aucun temps n'est saisi cette semaine, il n'y a donc pas de total.",
  "p.timesheet.col_plan": "Nom du plan",
  "p.timesheet.col_total": "Somme",
  "p.timesheet.col_billable": "Facturable",
  "p.timesheet.col_non_billable": "Non facturable",
  "p.timesheet.row_day_totals": "Chaque jour",
  "p.timesheet.plan_unnamed": "Plan {id}… (nom non affiché)",
  "p.timesheet.split_line":
    "Facturable {billable}, non facturable {non_billable} : {total} au total.",
  "p.timesheet.omitted":
    "Saisies omises parce qu'elles sont dans des plans que vous ne pouvez pas lire : {n}.",
  "p.timesheet.hours_one": "{n} h",
  "p.timesheet.hours_other": "{n} h",
  "p.timesheet.minutes_one": "{n} minute",
  "p.timesheet.minutes_other": "{n} minutes",
  "p.timesheet.duration_hm": "{h} {m}",
  "p.timesheet.submit": "Envoyer cette semaine pour approbation",
  "p.timesheet.submit_again":
    "Envoyer de nouveau cette semaine pour approbation",
  "p.timesheet.submitted_notice":
    "La semaine {week} a été envoyée pour approbation.",
  "p.timesheet.nothing_to_submit":
    "Une semaine sans temps saisi ne peut pas être envoyée pour approbation.",
  "p.timesheet.waiting": "Cette semaine attend la décision d'un approbateur.",
  "p.timesheet.locked":
    "Une semaine approuvée est définitive : plus aucun temps ne peut y être saisi.",
  "p.timesheet.could_not_load": "Impossible de charger votre semaine.",
  "p.timesheet.could_not_submit":
    "Impossible d'envoyer la semaine pour approbation.",
  "p.timesheet.other_heading":
    "La semaine d'une autre personne, pour les approbateurs",
  "p.timesheet.other_intro":
    "Si vous pouvez approuver les feuilles de temps, indiquez la référence de la personne pour lire sa semaine, celle choisie ci-dessus, puis approuvez-la ou renvoyez-la. Personne ne décide de sa propre semaine, renvoyer une semaine exige une raison, et une approbation est définitive.",
  "p.timesheet.f_person":
    "Personne : ‘person:’ ou ‘worker:’ et son identifiant",
  "p.timesheet.show_person": "Afficher sa semaine",
  "p.timesheet.problem_person":
    "Indiquez la personne sous la forme ‘person:’ ou ‘worker:’ suivi de son identifiant, sans rien d'autre.",
  "p.timesheet.other_week_heading": "Semaine {week} de {person}",
  "p.timesheet.other_caption":
    "Temps de {person} la semaine {week}, par plan et par jour",
  "p.timesheet.decision_legend": "Votre décision",
  "p.timesheet.approve": "Approuver",
  "p.timesheet.reject": "Renvoyer",
  "p.timesheet.f_comment": "Note, si vous le souhaitez",
  "p.timesheet.f_comment_required": "Pourquoi elle est renvoyée (obligatoire)",
  "p.timesheet.approve_final":
    "Une approbation est définitive : la semaine est alors verrouillée et personne ne peut la renvoyer.",
  "p.timesheet.record_decision": "Enregistrer la décision",
  "p.timesheet.approved_notice": "La semaine {week} de {person} est approuvée.",
  "p.timesheet.rejected_notice":
    "La semaine {week} de {person} a été renvoyée.",
  "p.timesheet.other_not_submitted":
    "Cette semaine n'a pas été envoyée pour approbation, il n'y a donc encore rien à décider.",
  "p.timesheet.other_rejected":
    "Cette semaine a été renvoyée ; elle pourra être décidée de nouveau quand la personne l'enverra de nouveau.",
  "p.timesheet.problem_own_week":
    "C'est votre propre semaine : personne ne décide de sa propre feuille de temps.",
  "p.timesheet.problem_comment": "Dites pourquoi la semaine est renvoyée.",
  "p.timesheet.could_not_load_other":
    "Impossible de charger la semaine de cette personne.",
  "p.timesheet.could_not_decide": "Impossible d'enregistrer la décision.",
  "p.timesheet.refused_read_forbidden":
    "Seule la personne elle-même ou un approbateur des feuilles de temps peut lire cette semaine.",
  "p.timesheet.refused_submit_forbidden":
    "Seule la personne à qui appartient une semaine peut l'envoyer pour approbation.",
  "p.timesheet.refused_decide_forbidden":
    "Vous ne pouvez pas décider de cette semaine : il faut le droit d'approuver les feuilles de temps, et personne ne décide de sa propre semaine.",
  "p.timesheet.refused_submit_conflict":
    "Cette semaine a déjà été envoyée pour approbation ou approuvée ; elle a été relue.",
  "p.timesheet.refused_decide_conflict":
    "Cette semaine n'attend plus de décision ; elle a été relue.",
  "p.timesheet.refused_not_addressable":
    "Votre identité de connexion n'a pas de référence de personne, il n'y a donc pas de feuille de temps à afficher.",
  "p.timesheet.refused_read_invalid":
    "Le service n'a pas accepté cette semaine ou cette personne.",
  "p.timesheet.refused_submit_invalid":
    "La semaine n'a pas été envoyée : aucun temps n'y est saisi.",
  "p.timesheet.refused_decide_invalid":
    "Le service n'a pas accepté la décision : renvoyer une semaine exige une raison.",
  "p.security.access_link": "Revue des accès et accès d'urgence",
  "p.security.access_note":
    "— qui détient quel accès, pour la recertification, et l'accès d'urgence limité dans le temps à un plan privé, qui est enregistré et audité.",
  "p.access.title_ppm": "Revue des accès et accès d'urgence — PPM",
  "p.access.back": "← Sécurité",
  "p.access.heading": "Revue des accès et accès d'urgence",
  "p.access.intro":
    "La revue des accès liste les accès enregistrés comme accordés, pour la recertification. L'accès d'urgence vous ouvre un plan privé pour une durée limitée quand il n'y a pas d'autre moyen ; chaque utilisation est enregistrée et auditée.",
  "p.access.review_heading": "Revue des accès",
  "p.access.review_intro":
    "Les accès enregistrés comme accordés dans le registre des accès et non révoqués depuis, les plus anciens d'abord, avec qui les a accordés et quand. Elle montre ce qui a été enregistré, pas ce que le service de connexion applique : comparez les deux. Réservé aux administrateurs ; chaque lecture est enregistrée dans la piste d'audit, la revue n'est donc lue que lorsque vous la demandez.",
  "p.access.f_review_plan":
    "Identifiant du plan (laissez vide pour tous les plans que vous pouvez lire)",
  "p.access.f_review_days": "À revoir après ce nombre de jours calendaires",
  "p.access.show_review": "Afficher la revue",
  "p.access.problem_review_plan":
    "Ce n'est pas un identifiant de plan : collez l'identifiant du plan, ou laissez la case vide pour tous les plans.",
  "p.access.problem_review_days":
    "Indiquez le nombre de jours calendaires par un nombre entier, 0 ou plus.",
  "p.access.could_not_load_review": "Impossible de charger la revue des accès.",
  "p.access.review_totals":
    "Au {as_of}. Accès en vigueur : {standing}. À revoir, âgés de {days} jours calendaires ou plus : {due}.",
  "p.access.review_note":
    "L'ancienneté est comptée en jours calendaires du jour où l'accès a été enregistré jusqu'à aujourd'hui pour le déploiement.",
  "p.access.download_review":
    "Télécharger la revue sous forme de tableur, avec une colonne pour vos décisions de conserver ou révoquer",
  "p.access.review_caption":
    "Accès en vigueur aujourd'hui, les plus anciens d'abord",
  "p.access.review_empty":
    "Aucun accès enregistré comme accordé n'est en vigueur dans les plans que vous pouvez lire.",
  "p.access.col_person": "Personne",
  "p.access.col_access": "Accès",
  "p.access.col_plan": "Nom du plan",
  "p.access.col_granted_on": "Accordé le",
  "p.access.col_granted_by": "Accordé par",
  "p.access.col_age": "Ancienneté en jours calendaires",
  "p.access.col_review": "Revue",
  "p.access.col_note": "Remarque",
  "p.access.due": "À revoir",
  "p.access.not_due": "Pas encore à revoir",
  "p.access.plan_unnamed": "Plan {id}… (nom non affiché)",
  "p.access.bg_heading": "Accès d'urgence à un plan privé (bris de glace)",
  "p.access.bg_audited_heading":
    "Chaque utilisation de l'accès d'urgence est enregistrée et auditée.",
  "p.access.bg_audited_open":
    "Ouvrir un plan enregistre dans la piste d'audit qui vous êtes, le plan, votre raison, la durée et la personne que vous nommez.",
  "p.access.bg_audited_told":
    "La personne que vous nommez est prévenue immédiatement. Si elle ne peut pas l'être, le plan n'est pas ouvert.",
  "p.access.bg_audited_each":
    "Chaque requête que seul cet accès laisse passer est enregistrée dans la piste d'audit comme accès d'urgence.",
  "p.access.bg_audited_visibility":
    "Il vous permet de voir le plan. Ce que vous pouvez modifier reste décidé par vos propres droits.",
  "p.access.bg_audited_ends":
    "Il prend fin de lui-même à l'heure indiquée, ou plus tôt si vous y mettez fin. Cinq au plus par jour, et un à la fois par plan.",
  "p.access.bg_intro":
    "Ne l'utilisez que lorsque les membres du plan sont injoignables et que le travail ne peut pas attendre. Il faut l'habilitation bris de glace ou un accès administrateur ; le service décide.",
  "p.access.problems_heading": "Le plan n'a pas été ouvert :",
  "p.access.problem_plan": "Indiquez l'identifiant du plan privé.",
  "p.access.problem_reason_short":
    "Dites pourquoi vous avez besoin d'accès, en au moins {reason_min} caractères.",
  "p.access.problem_reason_long": "La raison dépasse {reason_max} caractères.",
  "p.access.problem_minutes":
    "Indiquez la durée en minutes entières, de {minutes_min} à {minutes_max}.",
  "p.access.problem_notify":
    "Nommez la personne à prévenir sous la forme ‘person:’ ou ‘worker:’ suivi de son identifiant, sans rien d'autre.",
  "p.access.problem_acknowledged":
    "Cochez la case pour indiquer que vous comprenez que ceci est enregistré et audité et que la personne est prévenue.",
  "p.access.could_not_open": "Impossible d'ouvrir le plan.",
  "p.access.opened_notice":
    "Le plan vous est ouvert jusqu'au {until} (votre fuseau horaire, {tz}). {person} a été prévenu.",
  "p.access.go_to_plan": "Aller au plan",
  "p.access.f_plan": "Identifiant du plan privé",
  "p.access.f_reason":
    "Pourquoi vous avez besoin d'accès (obligatoire, au moins {min} caractères)",
  "p.access.reason_count": "{n} caractères ; au moins {min} et au plus {max}.",
  "p.access.f_minutes": "Pour combien de temps, en minutes ({min} à {max})",
  "p.access.f_notify":
    "Personne à prévenir : ‘person:’ ou ‘worker:’ et son identifiant (pas vous)",
  "p.access.f_acknowledge":
    "Je comprends que ceci est enregistré et audité, et que la personne que je nomme sera prévenue.",
  "p.access.open": "Ouvrir le plan en accès d'urgence",
  "p.access.grants_heading": "Accès d'urgence accordés",
  "p.access.grants_intro":
    "Les vôtres, les plus récents d'abord ; un administrateur voit ceux de tout le monde. Un accès n'est jamais supprimé. Les heures sont dans votre fuseau horaire, {tz}.",
  "p.access.grants_caption":
    "Accès d'urgence accordés, les plus récents d'abord",
  "p.access.grants_empty": "Aucun accès d'urgence n'a été ouvert.",
  "p.access.loading": "Chargement…",
  "p.access.could_not_load_grants":
    "Impossible de charger les accès d'urgence.",
  "p.access.could_not_revoke": "Impossible de mettre fin à l'accès d'urgence.",
  "p.access.revoked_notice": "L'accès d'urgence à {plan} a pris fin.",
  "p.access.revoke": "Terminer maintenant",
  "p.access.revoke_for": "Terminer maintenant : accès d'urgence à {plan}",
  "p.access.state_live": "En vigueur",
  "p.access.state_revoked": "Terminé plus tôt",
  "p.access.state_expired": "Expiré",
  "p.access.ended_by": "{when}, terminé par {who}",
  "p.access.col_state": "État",
  "p.access.col_holder": "Détenu par",
  "p.access.col_opened": "Ouvert",
  "p.access.col_ends": "Se termine ou s'est terminé",
  "p.access.col_reason": "Raison donnée",
  "p.access.col_told": "Personne prévenue",
  "p.access.col_act": "Agir",
  "p.access.refused_sign_in": "Connectez-vous d'abord.",
  "p.access.refused_review_forbidden":
    "La revue des accès est réservée aux administrateurs.",
  "p.access.refused_review_invalid":
    "Le service n'a pas accepté le plan ou le nombre de jours.",
  "p.access.refused_open_forbidden":
    "Vous ne pouvez pas utiliser l'accès d'urgence : il faut l'habilitation bris de glace ou un accès administrateur.",
  "p.access.refused_open_not_found": "Aucun plan ne porte cet identifiant.",
  "p.access.refused_open_conflict":
    "Vous avez déjà un accès d'urgence à ce plan ; la liste ci-dessous a été relue.",
  "p.access.refused_open_invalid":
    "Le plan n'a pas été ouvert. Soit il n'est pas privé, il n'y a donc rien à ouvrir, soit la personne à prévenir est vous, soit cette personne ne peut pas être prévenue. Aucun accès n'a été accordé.",
  "p.access.refused_open_limit":
    "Vous avez ouvert le maximum d'accès d'urgence autorisé en une journée (cinq, comptés sur les dernières 24 heures).",
  "p.access.refused_revoke_not_found":
    "Cet accès est introuvable, ou il ne vous revient pas d'y mettre fin.",
  "p.deescalation.nav": "Tableaux de bord et désescalade",
  "p.deescalation.title": "Tableaux de bord et désescalade — PPM",
  "p.deescalation.h1": "Tableaux de bord et désescalade",
  "p.deescalation.lede":
    "Le niveau d'escalade tel que le commanditaire l'a fixé, les critères pour redescendre avec leurs preuves à chaque trimestre, et les tableaux de bord dont les indicateurs clés sont lus par trimestre terminé.",
  "p.deescalation.tracker_heading": "Le suivi de la désescalade",
  "p.deescalation.tracker_says":
    "Le niveau est enregistré tel que le commanditaire l'a fixé. Il n'est jamais calculé ici, et rien sur cette page ne le modifie : la série de trimestres atteints est proposée à la personne qui décide.",
  "p.deescalation.loading": "Chargement…",
  "p.deescalation.could_not_load":
    "Le suivi de la désescalade n'a pas pu être chargé.",
  "p.deescalation.could_not_save": "La modification n'a pas été enregistrée.",
  "p.deescalation.level_heading": "Niveau d'escalade",
  "p.deescalation.level_now": "Niveau {level}, fixé le {date}.",
  "p.deescalation.level_note": "Note du commanditaire : {note}",
  "p.deescalation.level_none":
    "Aucun niveau d'escalade n'a encore été enregistré.",
  "p.deescalation.level_history": "Niveaux précédents",
  "p.deescalation.level_history_item": "{level}, fixé le {date}",
  "p.deescalation.run_heading": "Trimestres atteints d'affilée",
  "p.deescalation.run_one":
    "{n} trimestre terminé atteint d'affilée, sur les {need} que demande la règle.",
  "p.deescalation.run_other":
    "{n} trimestres terminés atteints d'affilée, sur les {need} que demande la règle.",
  "p.deescalation.rule_satisfied":
    "La règle est satisfaite. C'est au commanditaire d'en juger ; le niveau ne change pas de lui-même.",
  "p.deescalation.rule_not_satisfied": "La règle n'est pas encore satisfaite.",
  "p.deescalation.setting_unusable":
    "Le réglage de l'installation sur le nombre de trimestres que demande la règle n'a pas pu être utilisé ; la valeur par défaut de {n} s'applique donc.",
  "p.deescalation.reads_scorecard": "Le suivi lit le tableau de bord {name}.",
  "p.deescalation.reads_no_scorecard":
    "Aucun tableau de bord n'est celui que lit le suivi ; seuls les critères comptent donc. Marquez-en un comme tel en l'ajoutant, ci-dessous.",
  "p.deescalation.hidden_kpis_one":
    "{n} indicateur clé de ce tableau de bord appartient à un plan que vous ne pouvez pas lire : il n'est ni affiché ni compté ici.",
  "p.deescalation.hidden_kpis_other":
    "{n} indicateurs clés de ce tableau de bord appartiennent à des plans que vous ne pouvez pas lire : ils ne sont ni affichés ni comptés ici.",
  "p.deescalation.quarters_label": "Chaque trimestre terminé",
  "p.deescalation.col_quarter": "Trimestre",
  "p.deescalation.col_state": "Situation",
  "p.deescalation.col_reasons": "Pourquoi",
  "p.deescalation.state_met": "Atteint",
  "p.deescalation.state_not_met": "Non atteint",
  "p.deescalation.state_missing": "Rien d'enregistré",
  "p.deescalation.state_with_value": "{state} (valeur {value})",
  "p.deescalation.reason_all_met":
    "Chaque indicateur clé et chaque critère ont été atteints.",
  "p.deescalation.reason_nothing":
    "Il n'y a rien à mesurer : aucun indicateur clé et aucun critère.",
  "p.deescalation.reason_no_kpis":
    "Le tableau de bord n'a aucun indicateur clé que vous pouvez lire.",
  "p.deescalation.reason_not_met": "Non atteints : {list}.",
  "p.deescalation.reason_missing": "Rien d'enregistré pour : {list}.",
  "p.deescalation.quarter": "T{q} {year}",
  "p.deescalation.quarter_standing": "{quarter} : {state}. {reasons}",
  "p.deescalation.tracker_kpis_label":
    "Le tableau de bord du suivi, par trimestre",
  "p.deescalation.set_level_heading": "Enregistrer le niveau d'escalade",
  "p.deescalation.set_level_says":
    "Seul le commanditaire (une personne ayant le rôle sponsor) ou un administrateur peut enregistrer le niveau, tel qu'il l'a fixé. Un niveau compte de {min} à {max} caractères ; la note est facultative.",
  "p.deescalation.f_level": "Niveau",
  "p.deescalation.f_level_note": "Note (facultative)",
  "p.deescalation.save_level": "Enregistrer le niveau",
  "p.deescalation.level_saved": "Niveau {level} enregistré.",
  "p.deescalation.unchanged": "Rien n'a changé, donc rien n'a été envoyé.",
  "p.deescalation.problem_level":
    "Un niveau compte de {min} à {max} caractères.",
  "p.deescalation.refused_level_forbidden":
    "Refusé : seul le commanditaire ou un administrateur peut enregistrer le niveau d'escalade.",
  "p.deescalation.criteria_heading": "Critères pour redescendre",
  "p.deescalation.criteria_says":
    "Un critère est ajouté et n'est jamais modifié ni retiré. Sa preuve est enregistrée pour un trimestre terminé, et c'est le dernier enregistrement de ce trimestre qui compte.",
  "p.deescalation.criteria_none": "Aucun critère pour l'instant.",
  "p.deescalation.evidence_of": "Preuves pour : {criterion}",
  "p.deescalation.evidence_link": "Preuve",
  "p.deescalation.add_criterion_heading": "Ajouter un critère",
  "p.deescalation.f_criterion": "Critère (de {min} à {max} caractères)",
  "p.deescalation.add_criterion": "Ajouter le critère",
  "p.deescalation.criterion_added": "Critère ajouté.",
  "p.deescalation.problem_criterion":
    "Un critère compte de {min} à {max} caractères.",
  "p.deescalation.evidence_heading": "Enregistrer une preuve",
  "p.deescalation.evidence_says":
    "Choisissez un critère et un trimestre terminé (un trimestre en cours ne peut pas être enregistré), indiquez s'il a été atteint et rédigez une note d'au moins {min} caractères. Un lien est facultatif : une adresse http ou https sans espaces, jusqu'à {max} caractères.",
  "p.deescalation.f_ev_criterion": "Critère",
  "p.deescalation.f_ev_quarter": "Trimestre terminé",
  "p.deescalation.f_ev_met": "A-t-il été atteint ?",
  "p.deescalation.f_ev_note": "Note (au moins {min} caractères)",
  "p.deescalation.f_ev_link": "Lien vers la preuve (facultatif)",
  "p.deescalation.save_evidence": "Enregistrer la preuve",
  "p.deescalation.evidence_saved": "Preuve enregistrée pour {quarter}.",
  "p.deescalation.problem_criterion_choose":
    "Ajoutez d'abord un critère, puis enregistrez sa preuve.",
  "p.deescalation.problem_quarter": "Choisissez un trimestre terminé.",
  "p.deescalation.problem_quarter_unfinished":
    "Ce trimestre n'est pas terminé ; il ne peut donc pas encore être enregistré.",
  "p.deescalation.problem_met": "Indiquez si le critère a été atteint.",
  "p.deescalation.problem_note":
    "Une note d'au moins {min} caractères est nécessaire.",
  "p.deescalation.problem_link":
    "Un lien est une adresse http ou https sans espaces, jusqu'à {max} caractères.",
  "p.deescalation.refused_evidence":
    "Refusé : le trimestre doit être terminé, la note doit compter au moins cinq caractères et un lien doit être une adresse http ou https sans espaces.",
  "p.deescalation.refused_gone":
    "Cet enregistrement n'existe plus. Rechargez la page.",
  "p.deescalation.scorecards_heading": "Tableaux de bord",
  "p.deescalation.scorecards_says":
    "Un tableau de bord est un ensemble nommé d'indicateurs clés, lu par trimestre terminé. Un indicateur clé est atteint dans un trimestre quand sa dernière mesure de ce trimestre atteint ou dépasse sa cible ; un trimestre sans mesure est manquant, ce qui n'est pas la même chose que non atteint.",
  "p.deescalation.scorecards_none": "Aucun tableau de bord pour l'instant.",
  "p.deescalation.could_not_load_scorecards":
    "Les tableaux de bord n'ont pas pu être chargés.",
  "p.deescalation.col_scorecard": "Tableau de bord",
  "p.deescalation.col_tracker": "Lu par le suivi",
  "p.deescalation.is_tracker": "Oui, celui que lit le suivi",
  "p.deescalation.not_tracker": "Non",
  "p.deescalation.add_scorecard_heading": "Ajouter un tableau de bord",
  "p.deescalation.add_scorecard_says":
    "Un nom de {min} à {max} caractères, pas encore utilisé (sans tenir compte de la casse). Au plus un tableau de bord est celui que lit le suivi.",
  "p.deescalation.f_name": "Nom",
  "p.deescalation.f_for_tracker": "Celui que lit le suivi",
  "p.deescalation.add_scorecard": "Ajouter le tableau de bord",
  "p.deescalation.scorecard_added": "Tableau de bord {name} ajouté.",
  "p.deescalation.problem_name":
    "Le nom d'un tableau de bord compte de {min} à {max} caractères.",
  "p.deescalation.problem_name_taken": "Un tableau de bord porte déjà ce nom.",
  "p.deescalation.problem_tracker_taken":
    "Le tableau de bord {name} est déjà celui que lit le suivi.",
  "p.deescalation.scorecard_heading": "Un tableau de bord par trimestre",
  "p.deescalation.f_scorecard": "Tableau de bord",
  "p.deescalation.choose_scorecard": "Choisir un tableau de bord",
  "p.deescalation.could_not_load_scorecard":
    "Le tableau de bord n'a pas pu être chargé.",
  "p.deescalation.scorecard_says":
    "Chaque indicateur clé par trimestre terminé : atteint, non atteint ou rien d'enregistré.",
  "p.deescalation.scorecard_kpis_label":
    "Indicateurs clés de {name}, par trimestre",
  "p.deescalation.scorecard_quarters_label":
    "Chaque trimestre du tableau de bord",
  "p.deescalation.col_kpi": "Indicateur clé",
  "p.deescalation.col_target": "Cible",
  "p.deescalation.target_at_least": "au moins {value} {unit}",
  "p.deescalation.target_at_most": "au plus {value} {unit}",
  "p.deescalation.no_kpis":
    "Ce tableau de bord n'a aucun indicateur clé que vous pouvez lire.",
  "p.deescalation.unit_percent": "pour cent",
  "p.deescalation.unit_count": "un décompte",
  "p.deescalation.kpis_heading": "Choisir ses indicateurs clés (KPI)",
  "p.deescalation.kpis_says":
    "Cochez les indicateurs clés de ce tableau de bord, {max} au plus. Enregistrer remplace l'ensemble ; un indicateur d'un plan que vous ne pouvez pas lire reste sur le tableau.",
  "p.deescalation.f_kpis": "Indicateurs clés du tableau de bord",
  "p.deescalation.kpis_none":
    "Il n'y a aucun indicateur clé à choisir. Ajoutez-en sur la page des indicateurs clés, ou depuis le modèle ci-dessous.",
  "p.deescalation.save_kpis": "Enregistrer les indicateurs clés",
  "p.deescalation.kpis_saved_one":
    "Le tableau de bord compte désormais {n} indicateur clé.",
  "p.deescalation.kpis_saved_other":
    "Le tableau de bord compte désormais {n} indicateurs clés.",
  "p.deescalation.problem_kpis_max":
    "Un tableau de bord compte au plus {max} indicateurs clés.",
  "p.deescalation.refused_kpis":
    "Refusé : chacun doit être un indicateur clé que vous pouvez lire, et un tableau de bord en compte au plus 60.",
  "p.deescalation.template_heading": "Ajouter des mesures depuis le modèle",
  "p.deescalation.template_says":
    "Un ensemble de départ, proposé et non appliqué. Il n'a pas de cibles : vous fixez chaque cible et chaque ligne rouge, et les mesures choisies deviennent des indicateurs clés de ce tableau de bord.",
  "p.deescalation.template_rule":
    "Quand plus haut est meilleur, la ligne rouge doit être sous la cible ; quand plus bas est meilleur, au-dessus. Les cibles et les lignes rouges sont des nombres entiers.",
  "p.deescalation.could_not_load_template": "Le modèle n'a pas pu être chargé.",
  "p.deescalation.higher_better": "Plus haut est meilleur",
  "p.deescalation.lower_better": "Plus bas est meilleur",
  "p.deescalation.unit": "mesuré en {unit}",
  "p.deescalation.f_pick": "Ajouter {measure}",
  "p.deescalation.f_target": "Cible",
  "p.deescalation.f_red": "Ligne rouge",
  "p.deescalation.add_measures": "Ajouter les mesures",
  "p.deescalation.measures_added_one": "{n} mesure ajoutée au tableau de bord.",
  "p.deescalation.measures_added_other":
    "{n} mesures ajoutées au tableau de bord.",
  "p.deescalation.problem_pick_one": "Choisissez au moins une mesure.",
  "p.deescalation.problem_whole":
    "{measure} : la cible et la ligne rouge sont des nombres entiers.",
  "p.deescalation.problem_red_higher":
    "{measure} : plus haut est meilleur, donc la ligne rouge doit être sous la cible.",
  "p.deescalation.problem_red_lower":
    "{measure} : plus bas est meilleur, donc la ligne rouge doit être au-dessus de la cible.",
  "p.deescalation.problem_not_in_template":
    "{measure} ne figure pas dans le modèle.",
  "p.deescalation.refused_template":
    "Refusé : vérifiez chaque cible et chaque ligne rouge selon la règle de sens.",
  "p.deescalation.measure_programmes_with_independent_assurance":
    "Programmes ayant eu une assurance indépendante au dernier trimestre",
  "p.deescalation.measure_milestones_met_on_time":
    "Jalons atteints à leur date ou avant",
  "p.deescalation.measure_benefits_validated":
    "Bénéfices prévus validés par une personne extérieure au programme",
  "p.deescalation.measure_red_without_decision":
    "Programmes au rouge sans décision enregistrée",
  "p.deescalation.measure_open_critical_issues":
    "Problèmes ouverts de gravité critique",
  "p.deescalation.measure_change_requests_waiting":
    "Demandes de changement en attente de décision depuis plus de 30 jours",
  "p.phase.name_initiating": "Lancement",
  "p.phase.name_planning": "Planification",
  "p.phase.name_executing": "Exécution",
  "p.phase.name_controlling": "Pilotage",
  "p.phase.name_closing": "Clôture",
  "p.phase.heading": "Phase du plan",
  "p.phase.heading_programme": "Phase et démarrage",
  "p.phase.says":
    "Un plan avance d'une phase à la fois ; il peut revenir à une phase antérieure avec une raison, enregistrée avec le changement. La phase n'empêche jamais d'enregistrer du travail.",
  "p.phase.could_not_load": "Impossible de charger la phase du plan",
  "p.phase.now": "Le plan est dans la phase {phase}.",
  "p.phase.none":
    "Aucune phase n'est encore enregistrée : le premier passage se fait vers Lancement.",
  "p.phase.time_heading": "Temps passé dans chaque phase",
  "p.phase.days_in_one": "{phase} : {days} jour",
  "p.phase.days_in_other": "{phase} : {days} jours",
  "p.phase.visits_one": "{visits} entrée",
  "p.phase.visits_other": "{visits} entrées",
  "p.phase.current": "en cours",
  "p.phase.check_heading": "Contrôle de démarrage",
  "p.phase.check_says":
    "Un programme démarre quand sa phase passe à Exécution. Le démarrage est refusé quand toutes les places pour les programmes en réalisation active sont prises (les programmes en pause ou arrêtés n'en occupent aucune) ou quand le pool contraint manque d'offre dans l'un des six prochains mois. Un programme refusé entre dans la file de démarrage et garde sa place ; il peut démarrer quand même avec une raison de dérogation de {min} caractères ou plus, enregistrée et auditée. Lire le contrôle ne change rien.",
  "p.phase.could_not_check": "Impossible de lire le contrôle de démarrage",
  "p.phase.places":
    "{in_delivery} places sur {limit} pour les programmes en réalisation active sont prises.",
  "p.phase.no_limit":
    "Aucune limite n'est fixée pour les programmes en réalisation active.",
  "p.phase.limit_unusable":
    "La limite de l'installation pour les programmes en réalisation n'a pas pu être utilisée ; aucune limite ne s'applique donc.",
  "p.phase.pool_is": "Le pool contraint est {pool}.",
  "p.phase.no_pool": "Aucun pool contraint n'est identifié.",
  "p.phase.would_refuse": "Un démarrage maintenant serait refusé :",
  "p.phase.would_start": "Un démarrage maintenant serait accepté.",
  "p.phase.in_queue":
    "Ce programme est le numéro {n} dans la file de démarrage.",
  "p.phase.see_queue": "Voir la file de démarrage",
  "p.phase.refusal_limit":
    "Les {limit} places pour les programmes en réalisation active sont prises ({in_delivery} en réalisation).",
  "p.phase.refusal_pool":
    "Le pool contraint {pool} manque d'offre en {months}.",
  "p.phase.demand_heading": "Demande ferme de ce programme",
  "p.phase.demand_caption":
    "Équivalents temps plein par pool de compétences et par mois, issus des affectations fermes sur ce programme et les plans en dessous",
  "p.phase.no_demand":
    "Aucune demande ferme de ce programme dans un pool de compétences au cours des six prochains mois.",
  "p.phase.col_pool": "Pool de compétences",
  "p.phase.check_again": "Vérifier à nouveau",
  "p.phase.move_heading": "Changer de phase",
  "p.phase.f_to": "Passer à",
  "p.phase.forward_to": "Avancer à {phase}",
  "p.phase.back_to": "Revenir à {phase}",
  "p.phase.back_needs_reason": "Un retour en arrière demande une raison.",
  "p.phase.f_reason":
    "Raison (nécessaire pour revenir en arrière ; enregistrée avec le changement)",
  "p.phase.start_may_queue":
    "Passer à Exécution est le démarrage de ce programme : si le contrôle ne trouve pas de place à l'envoi, le démarrage est refusé et le programme entre dans la file de démarrage.",
  "p.phase.start_would_queue":
    "Ce démarrage serait refusé. Envoyé sans raison de dérogation, le programme entre dans la file de démarrage (ou y garde sa place).",
  "p.phase.f_override":
    "Raison de dérogation, pour démarrer quand même ({min} caractères ou plus ; enregistrée et auditée)",
  "p.phase.refused_why": "Pourquoi le démarrage a été refusé",
  "p.phase.move": "Déplacer la phase",
  "p.phase.moved": "Le plan est maintenant dans la phase {phase}.",
  "p.phase.started_override":
    "Le programme a démarré par dérogation : il est maintenant dans la phase {phase}, et la dérogation est enregistrée.",
  "p.phase.start_refused_place":
    "Le démarrage a été refusé : le programme est donc le numéro {n} dans la file de démarrage. Pour le démarrer quand même, donnez une raison de dérogation.",
  "p.phase.start_refused":
    "Le démarrage a été refusé et le programme est entré dans la file de démarrage. Pour le démarrer quand même, donnez une raison de dérogation.",
  "p.phase.could_not_move": "Impossible de changer la phase",
  "p.phase.problem_choose": "Choisissez la phase vers laquelle passer.",
  "p.phase.problem_same":
    "Le plan est déjà dans cette phase ; rien n'a donc été envoyé.",
  "p.phase.problem_skip": "On avance d'une phase à la fois.",
  "p.phase.problem_reason": "Revenir en arrière nécessite une raison.",
  "p.phase.problem_override":
    "Une raison de dérogation compte {min} caractères ou plus.",
  "p.phase.refused_move":
    "Refusé : on avance d'une phase à la fois, un retour en arrière demande une raison, et le plan ne doit pas déjà être dans cette phase.",
  "p.phase.refused_gone": "Ce plan n'existe plus.",
  "p.proposals.act_submit": "Soumettre",
  "p.proposals.act_review": "Commencer l'examen",
  "p.proposals.act_approve": "Approuver",
  "p.proposals.act_reject": "Rejeter",
  "p.proposals.act_submit_named": "Soumettre {title}",
  "p.proposals.act_review_named": "Commencer l'examen de {title}",
  "p.proposals.act_approve_named": "Approuver {title}",
  "p.proposals.act_reject_named": "Rejeter {title}",
  "p.proposals.ask_rule":
    "Une demande externe du commanditaire doit nommer ce qu'elle déplace (un arbitrage de {min} caractères ou plus) avant de pouvoir être approuvée.",
  "p.proposals.new_form": "Nouvelle proposition",
  "p.proposals.f_external_ask": "Une demande externe du commanditaire",
  "p.proposals.f_new_tradeoff":
    "Ce qu'elle déplace (l'arbitrage, de {min} à {max} caractères ; il peut être nommé plus tard, avant l'approbation)",
  "p.proposals.col_tradeoff": "Demande externe et arbitrage",
  "p.proposals.not_ask": "Pas une demande externe",
  "p.proposals.ask_with": "Demande externe. Arbitrage : {tradeoff}",
  "p.proposals.ask_without":
    "Demande externe. Aucun arbitrage nommé pour l'instant.",
  "p.proposals.tradeoff_name": "Nommer l'arbitrage",
  "p.proposals.tradeoff_change": "Modifier l'arbitrage",
  "p.proposals.tradeoff_name_named": "Nommer l'arbitrage de {title}",
  "p.proposals.tradeoff_change_named": "Modifier l'arbitrage de {title}",
  "p.proposals.tradeoff_heading": "Arbitrage de {title}",
  "p.proposals.tradeoff_says":
    "Nommez ce que cette demande déplace, en {min} à {max} caractères. Il peut être nommé ou modifié jusqu'à ce que la proposition soit approuvée ou rejetée, et chaque modification est enregistrée.",
  "p.proposals.f_tradeoff": "Ce qu'elle déplace (de {min} à {max} caractères)",
  "p.proposals.tradeoff_save": "Enregistrer l'arbitrage",
  "p.proposals.tradeoff_close": "Fermer le formulaire d'arbitrage",
  "p.proposals.tradeoff_unchanged":
    "Rien n'a changé ; rien n'a donc été envoyé.",
  "p.proposals.tradeoff_saved": "Arbitrage enregistré pour {title}.",
  "p.proposals.problem_title": "Une proposition a besoin d'un titre.",
  "p.proposals.problem_tradeoff":
    "Un arbitrage compte de {min} à {max} caractères.",
  "p.proposals.problem_not_ask": "Seule une demande externe a un arbitrage.",
  "p.proposals.problem_decided":
    "La proposition est tranchée ; son arbitrage ne peut donc plus changer.",
  "p.proposals.problem_approve":
    "{title} est une demande externe : nommez son arbitrage ({min} caractères ou plus) avant de l'approuver.",
  "p.proposals.refused_create":
    "Refusé : une proposition a besoin d'un titre, un montant d'une devise, et un arbitrage compte au plus {max} caractères.",
  "p.proposals.refused_tradeoff":
    "Refusé : un arbitrage compte de {min} à {max} caractères, sur une demande externe pas encore tranchée.",
  "p.proposals.refused_step":
    "Refusé : cette étape ne découle pas du statut de la proposition.",
  "p.proposals.refused_gone": "Cette proposition n'existe plus.",
  "p.thresholds.opened_by_kind":
    "Ouverte parce que le programme a franchi son seuil : {kind}.",
  "p.thresholds.joined_by_kind":
    "Pendant qu'elle était ouverte, le programme a aussi franchi un seuil : {kind}.",
  "p.deescalation.refused_tracker_taken":
    "Refusé : un autre tableau de bord est déjà celui que lit le suivi.",
  "p.deescalation.level_now_by": "Niveau {level}, fixé le {date} par {who}.",
  "p.deescalation.level_history_item_by": "{level}, fixé le {date} par {who}",
  "p.deescalation.evidence_recorded_by": "enregistrée par {who} le {date}",
  "p.deescalation.evidence_recorded_on": "enregistrée le {date}",
  "p.proposals.refused_approve_status":
    "Refusé : seule une proposition en examen peut être approuvée.",
  "p.proposals.refused_tradeoff_required":
    "Refusé : une demande externe doit nommer un arbitrage de {min} caractères ou plus avant de pouvoir être approuvée.",
  "p.ledger.untraced_no_readable_line":
    "Aucune ligne budgétaire que vous pouvez lire ne puise dans cette allocation : on ne peut donc pas suivre ici où va son argent.",
  "p.ledger.untraced_not_on_a_line":
    "Une partie de cette allocation n'est encore prévue sur aucune ligne budgétaire.",
  "p.ledger.untraced_lines_over_plan":
    "Les lignes budgétaires prévoient plus que ce que contient cette allocation.",
  "p.ledger.untraced_other":
    "Une partie de l'argent de cette allocation ne peut pas être suivie.",
  "p.status.title_ppm": "Points d'état — PPM",
  "p.status.title": "Points d'état",
  "p.status.link": "Points d'état et leurs preuves",
  "p.status.intro":
    "Un point d'état est l'appréciation d'une personne sur l'avancement du plan (dans les temps, à risque ou hors trajectoire), avec ce qui se passe en mots. Rien ici ne calcule de note.",
  "p.status.could_not_load": "Impossible de charger l'état du plan",
  "p.status.now_heading": "Où en est le plan",
  "p.status.no_update": "Aucun point d'état n'a encore été publié.",
  "p.status.cadence_is_one": "Un point est attendu chaque jour.",
  "p.status.cadence_is_other": "Un point est attendu tous les {n} jours.",
  "p.status.standing_overdue_one":
    "Le prochain point était dû le {date} : il a {n} jour de retard.",
  "p.status.standing_overdue_other":
    "Le prochain point était dû le {date} : il a {n} jours de retard.",
  "p.status.standing_due": "Un point est dû aujourd'hui, le {date}.",
  "p.status.standing_next": "Le prochain point est dû le {date}.",
  "p.status.no_cadence":
    "Aucune cadence n'est fixée : aucun point n'est donc jamais dû ni en retard.",
  "p.status.as_of": "Au {date}, dans le fuseau horaire du déploiement.",
  "p.status.loading": "Chargement…",
  "p.status.posted_on_by": "publié le {date} par {who}",
  "p.status.posted_on": "publié le {date}",
  "p.status.claims_of": "Affirmations du point du {date}",
  "p.status.source_metric": "{metric} : {value} actuellement",
  "p.status.source_metric_unknown": "{metric} : inconnu (pas zéro)",
  "p.status.source_unavailable": "Une source qui n'est plus disponible",
  "p.status.kind_task": "Tâche : {title}",
  "p.status.kind_risk": "Risque : {title}",
  "p.status.kind_milestone": "Jalon : {title}",
  "p.status.kind_issue": "Problème : {title}",
  "p.status.kind_other": "Enregistrement : {title}",
  "p.status.metric_overdue": "Tâches en retard",
  "p.status.metric_due_soon": "Tâches bientôt dues",
  "p.status.metric_blocked": "Tâches bloquées",
  "p.status.metric_no_due_date": "Tâches sans échéance",
  "p.status.metric_def_overdue":
    "Tâches ouvertes dont l'échéance est avant aujourd'hui. Une tâche due aujourd'hui n'est pas en retard. Inconnu quand il y a des tâches ouvertes et qu'aucune n'a d'échéance.",
  "p.status.metric_def_due_soon":
    "Tâches ouvertes dues aujourd'hui ou dans les {days} prochains jours. Inconnu quand il y a des tâches ouvertes et qu'aucune n'a d'échéance.",
  "p.status.metric_def_blocked": "Tâches ouvertes dont le statut est bloqué.",
  "p.status.metric_def_no_due_date":
    "Tâches ouvertes sans échéance : celles que les autres chiffres ne voient pas.",
  "p.status.metrics_heading": "Comment les chiffres sont comptés",
  "p.status.metrics_says":
    "Sur les tâches ouvertes du plan (une tâche dont l'état n'est pas terminé), comptées quand un point est lu, pas quand il a été publié.",
  "p.status.cadence_heading": "À quelle fréquence un point est attendu",
  "p.status.cadence_says":
    "Tous les {min} à {max} jours. La fixer relance l'horloge à partir d'aujourd'hui : la même cadence n'est donc pas renvoyée. Une cadence ne peut pas être retirée une fois fixée.",
  "p.status.cadence_set_by": "Fixée le {date} par {who}.",
  "p.status.cadence_set_on": "Fixée le {date}.",
  "p.status.f_cadence": "Jours entre deux points ({min} à {max})",
  "p.status.save_cadence": "Enregistrer la cadence",
  "p.status.unchanged": "Rien n'a changé, donc rien n'a été envoyé.",
  "p.status.problem_cadence":
    "Une cadence est un nombre entier de jours de {min} à {max}.",
  "p.status.cadence_saved_one":
    "Un point est désormais attendu chaque jour ; l'horloge démarre aujourd'hui.",
  "p.status.cadence_saved_other":
    "Un point est désormais attendu tous les {n} jours ; l'horloge démarre aujourd'hui.",
  "p.status.could_not_save": "Impossible d'enregistrer la cadence",
  "p.status.refused_gone": "Ce plan n'existe plus.",
  "p.status.history_heading": "Tous les points, du plus récent au plus ancien",
  "p.status.history_says":
    "Les points ne sont jamais modifiés ni retirés. Chaque source est montrée telle qu'elle est maintenant : un enregistrement disparu n'est plus disponible, et chaque chiffre a sa valeur d'aujourd'hui.",
  "p.status.history_none": "Aucun point pour l'instant.",
  "p.status.post_heading": "Publier un point",
  "p.status.post_says":
    "Choisissez l'appréciation et dites ce qui se passe (jusqu'à {max} caractères). Un point ne peut être ni modifié ni retiré une fois publié, et en publier un relance l'horloge de la cadence.",
  "p.status.claims_rule":
    "Vous pouvez ajouter jusqu'à {claims} affirmations, chacune une phrase de {chars} caractères au plus citant jusqu'à {sources} sources : les chiffres ci-dessous, ou les tâches, risques, jalons et problèmes du plan. Chaque source est vérifiée à la publication.",
  "p.status.f_call": "L'appréciation",
  "p.status.call_on_track": "Dans les temps",
  "p.status.call_at_risk": "À risque",
  "p.status.call_off_track": "Hors trajectoire",
  "p.status.f_narrative": "Ce qui se passe (jusqu'à {max} caractères)",
  "p.status.claim_legend": "Affirmation {n}",
  "p.status.f_claim_text": "L'affirmation (jusqu'à {chars} caractères)",
  "p.status.f_metrics": "Chiffres qu'elle cite",
  "p.status.f_record": "Un enregistrement du plan que cite l'affirmation {n}",
  "p.status.choose_record": "Choisir un enregistrement",
  "p.status.add_record": "Ajouter l'enregistrement à l'affirmation {n}",
  "p.status.claim_records": "Enregistrements que cite l'affirmation {n}",
  "p.status.remove": "Retirer",
  "p.status.remove_source": "Retirer {source} de l'affirmation {n}",
  "p.status.remove_claim": "Retirer l'affirmation {n}",
  "p.status.add_claim": "Ajouter une affirmation",
  "p.status.post": "Publier le point",
  "p.status.posted": "Point publié : {call}.",
  "p.status.could_not_post": "Impossible de publier le point",
  "p.status.problem_call":
    "Choisissez dans les temps, à risque ou hors trajectoire.",
  "p.status.problem_narrative":
    "Dites ce qui se passe, en {max} caractères au plus.",
  "p.status.problem_claims": "Un point porte au plus {claims} affirmations.",
  "p.status.problem_claim_text":
    "Une affirmation est une phrase de {chars} caractères au plus.",
  "p.status.problem_sources": "Une affirmation cite au plus {sources} sources.",
  "p.status.problem_in_claim": "Affirmation {n} : {problem}",
  "p.status.refused_update":
    "Refusé : une source n'est peut-être plus un enregistrement de ce plan. Rien n'a été publié ; vérifiez les affirmations et publiez à nouveau.",
  "p.status.due_heading": "Points d'état dus",
  "p.status.due_none":
    "Aucun plan que vous pouvez lire n'a de point d'état dû.",
  "p.status.due_could_not_load": "Impossible de charger les points d'état dus",
  "p.status.a_plan": "Un plan",
  "p.status.due_last": "Dernier point : {call}, le {date}.",
  "p.status.due_never": "Aucun point pour l'instant.",
  "p.status.group_task": "Tâches",
  "p.status.group_risk": "Risques",
  "p.status.group_milestone": "Jalons",
  "p.status.group_issue": "Problèmes",
  "l.enum.risk_status.open": "Ouvert",
  "l.enum.risk_status.mitigating": "En atténuation",
  "l.enum.risk_status.closed": "Clos",
  "l.enum.risk_status.materialized": "Survenu",
  "p.risk_edit.change": "Modifier",
  "p.risk_edit.change_named": "Modifier : {title}",
  "p.risk_edit.heading": "Modifier le risque « {title} »",
  "p.risk_edit.says":
    "La probabilité et l'impact sont des nombres entiers de {min} à {max} ; l'exposition est leur produit. L'atténuation fait au plus {max_bytes} octets (moins de caractères dans certaines écritures). Une date de revue peut être changée mais pas retirée. Seul ce que vous modifiez est envoyé.",
  "p.risk_edit.says_escalate":
    "Pour consigner qu'un risque s'est produit, utilisez « Escalader » dans le tableau, qui en note aussi le moment.",
  "p.risk_edit.problem_status": "Choisissez un statut.",
  "p.risk_edit.problem_score":
    "La probabilité et l'impact doivent chacun être un nombre entier de {min} à {max}.",
  "p.risk_edit.problem_mitigation": "L'atténuation dépasse {max_bytes} octets.",
  "p.risk_edit.problem_review_cleared":
    "Une date de revue ne peut pas être retirée une fois fixée ; choisissez une autre date.",
  "p.risk_edit.problem_review_date": "La date de revue n'est pas une date.",
  "p.risk_edit.unchanged": "Rien n'a changé, donc rien n'a été envoyé.",
  "p.risk_edit.saved": "Le risque « {title} » a été enregistré.",
  "p.risk_edit.could_not_save": "Impossible d'enregistrer le risque",
  "p.risk_edit.refused_gone":
    "Ce risque n'est plus sur ce plan ; la liste a été relue.",
  "p.risk_edit.refused_invalid":
    "Refusé : le service n'a pas accepté le statut, les notes ou l'atténuation. Rien n'a été modifié.",
  "p.risk_edit.f_status": "Statut du risque",
  "p.risk_edit.f_probability": "Probabilité ({min} à {max})",
  "p.risk_edit.f_impact": "Impact ({min} à {max})",
  "p.risk_edit.f_mitigation": "Atténuation",
  "p.risk_edit.f_review_date": "Date de revue",
  "p.risk_edit.save": "Enregistrer le risque",
  "p.risk_edit.close": "Fermer le formulaire",
  "p.risk_edit.f_reason": "Pourquoi la note a changé (facultatif)",
  "p.risk_edit.history_h": "Historique de la note",
  "p.risk_edit.history_row":
    "{date} : probabilité {probability} × impact {impact} = {score}.",
  "p.risk_edit.history_reason": "Raison : {reason}",
  "p.risk_edit.history_first":
    "Première note connue ; l’historique commence ici.",
  "p.plan_issues.refused_gone":
    "Ce problème n'est plus sur ce plan ; la liste a été relue.",
  "p.plan_issues.refused_remove":
    "Refusé : vous ne pouvez pas retirer de problèmes de ce plan. Rien n'a été retiré.",
  "p.plan_issues.refused_remove_admin":
    "Refusé : seul un administrateur peut retirer un problème soumis au workflow de ce type. Rien n'a été retiré.",
  "p.plan_issues.remove_admin_only":
    "Seul un administrateur peut retirer « {title} » : le workflow de son type encadre ses transitions ou ses champs. Rien n'a été envoyé.",
  "p.plan_issues.confirm_remove":
    "Retirer le problème « {title} » ? Il quitte toutes les listes ; sa trace est conservée pour l'audit.",
  "p.plan_issues.removed": "Le problème « {title} » a été retiré.",
  "p.plan_issues.remove": "Retirer",
  "p.plan_issues.remove_named": "Retirer : {title}",
  "p.plan_issues.remove_says":
    "Retirer un problème l'ôte de toutes les listes et ne peut pas être annulé ici ; l'audit en garde une trace. Sous un workflow qui encadre transitions ou champs, seul un administrateur peut en retirer un.",
  "p.plan_issues.raised_on_by": "Signalé le {date} par {who}.",
  "p.plan_issues.raised_on": "Signalé le {date}.",
  "p.plan_issues.from_risk_named": "Signalé à partir du risque « {title} ».",
  "p.plan_issues.from_a_risk":
    "Signalé à partir d'un risque qui n'est plus listé.",
  "p.plan_issues.from_a_task":
    "Signalé à partir d'une tâche bloquée (ouvrir la tâche)",
  "p.plan_issues.answered_by_change":
    "Traité par une demande de changement (voir les demandes de changement du plan)",
  "p.plan_issues.resolved_on": "Résolu le {date}.",
  "p.task.issue_heading": "Un problème à partir de cette tâche",
  "p.task.issue_says":
    "Seule une tâche bloquée peut être signalée comme problème, et une seule fois tant que ce problème est ouvert. Le problème est un blocage de gravité moyenne, confié à la personne assignée à la tâche ; la tâche elle-même n'est pas modifiée.",
  "p.task.issue_raised": "Le problème a été signalé.",
  "p.task.issue_open_list": "Voir les problèmes du plan",
  "p.task.issue_exists":
    "Un problème ouvert a été signalé à partir de cette tâche : « {title} ».",
  "p.task.issue_not_blocked":
    "Cette tâche n'est pas bloquée ; elle ne peut donc pas être signalée comme problème. Marquez-la d'abord comme bloquée sur le tableau.",
  "p.task.issue_raise": "Signaler un problème à partir de cette tâche",
  "p.task.refused_from_task_not_blocked":
    "Refusé : seule une tâche bloquée peut être signalée comme problème, et celle-ci ne l'est plus. La tâche a été relue.",
  "p.task.refused_from_task_exists":
    "Refusé : un problème ouvert a déjà été signalé à partir de cette tâche. Les problèmes ont été relus.",
  "p.task.refused_from_task_forbidden":
    "Refusé : vous ne pouvez pas signaler de problèmes sur ce plan.",
  "p.task.refused_from_task_gone": "Cette tâche n'est plus sur ce plan.",
  "p.plan_change_requests.refused_gone":
    "Cette demande de changement n'est plus sur ce plan ; la liste a été relue.",
  "p.plan_change_requests.refused_links_conflict":
    "Refusé : la demande est close, ou l'un des problèmes relève désormais d'une autre demande de changement. La demande et les problèmes du plan ont été relus.",
  "p.plan_change_requests.refused_links_invalid":
    "Refusé : un problème ou un risque choisi n'est plus sur ce plan. Les problèmes et les risques du plan ont été relus.",
  "p.plan_change_requests.refused_preview":
    "Refusé : cette demande ne porte aucune modification de simulation lisible par l'ordonnanceur ; il n'y a donc pas de prévision.",
  "p.plan_change_requests.refused_baseline":
    "Refusé : une référence a déjà été prise pour ce changement, ou il n'est plus approuvé. La demande a été relue.",
  "p.plan_change_requests.problem_links_too_many":
    "Choisissez au plus {max} problèmes et {max} risques.",
  "p.plan_change_requests.problem_links_elsewhere":
    "« {title} » relève déjà d'une autre demande de changement ; décochez-le.",
  "p.plan_change_requests.links_unchanged":
    "Rien n'a changé, donc rien n'a été envoyé.",
  "p.plan_change_requests.links_saved":
    "Les problèmes et les risques traités par ce changement ont été enregistrés.",
  "p.plan_change_requests.confirm_baseline":
    "Prendre une référence de planning à cause de « {title} » ? Une seule peut être prise pour ce changement, et elle ne peut pas être annulée.",
  "p.plan_change_requests.baseline_taken":
    "La version {version} de la référence a été prise à cause de ce changement.",
  "p.plan_change_requests.stated_impact":
    "Impact indiqué par le demandeur : {impact}.",
  "p.plan_change_requests.implemented_on": "Mis en œuvre le {date}.",
  "p.plan_change_requests.implement": "Marquer comme mis en œuvre",
  "p.plan_change_requests.implemented":
    "Le changement a été marqué comme mis en œuvre.",
  "p.plan_change_requests.links_heading":
    "Problèmes et risques traités par ce changement",
  "p.plan_change_requests.links_none":
    "Il ne traite encore aucun problème ni risque.",
  "p.plan_change_requests.answers_issue": "Problème : {title}",
  "p.plan_change_requests.answers_risk": "Risque : {title}",
  "p.plan_change_requests.an_issue": "un problème qui n'est plus listé",
  "p.plan_change_requests.a_risk": "un risque qui n'est plus listé",
  "p.plan_change_requests.links_says":
    "Choisis tant que la demande est ouverte (brouillon, soumise, en revue ou approuvée). Un problème relève d'au plus une demande de changement ; jusqu'à {max} problèmes et {max} risques. Enregistrer remplace tout l'ensemble.",
  "p.plan_change_requests.links_issues": "Problèmes de ce plan",
  "p.plan_change_requests.issue_elsewhere":
    "{title} (relève d'une autre demande de changement)",
  "p.plan_change_requests.links_no_issues": "Ce plan n'a aucun problème.",
  "p.plan_change_requests.links_risks": "Risques de ce plan",
  "p.plan_change_requests.links_no_risks": "Ce plan n'a aucun risque.",
  "p.plan_change_requests.links_save": "Enregistrer ce qu'il traite",
  "p.plan_change_requests.links_closed":
    "Une demande close ne peut pas être reliée de nouveau.",
  "p.plan_change_requests.forecast_heading": "Prévision de simulation",
  "p.plan_change_requests.forecast_says":
    "Une prévision tirée du planning actuel avec les modifications de simulation de cette demande appliquées. La lire ne change rien ; l'impact indiqué plus haut est le chiffre du demandeur lui-même.",
  "p.plan_change_requests.forecast_show": "Afficher la prévision",
  "p.plan_change_requests.forecast_label": "Prévision — rien n'a été modifié",
  "p.plan_change_requests.forecast_finish":
    "Fin du plan : {before} actuellement, {after} avec le changement.",
  "p.plan_change_requests.forecast_finish_unknown":
    "La fin du plan ne peut pas être prévue.",
  "p.plan_change_requests.forecast_moved_one":
    "1 tâche se terminerait un autre jour.",
  "p.plan_change_requests.forecast_moved_other":
    "{n} tâches se termineraient un autre jour.",
  "p.plan_change_requests.forecast_later_one":
    "{title} : se termine 1 jour ouvré plus tard",
  "p.plan_change_requests.forecast_later_other":
    "{title} : se termine {n} jours ouvrés plus tard",
  "p.plan_change_requests.forecast_earlier_one":
    "{title} : se termine 1 jour ouvré plus tôt",
  "p.plan_change_requests.forecast_earlier_other":
    "{title} : se termine {n} jours ouvrés plus tôt",
  "p.plan_change_requests.a_task": "Une tâche",
  "p.plan_change_requests.forecast_labour":
    "Le coût de main-d'œuvre prévu changerait de {amount}.",
  "p.plan_change_requests.forecast_labour_hidden":
    "La prévision du coût de main-d'œuvre n'est montrée qu'aux personnes autorisées à voir les taux.",
  "p.plan_change_requests.forecast_labour_none":
    "Aucun travail chiffré, donc pas de prévision du coût de main-d'œuvre.",
  "p.plan_change_requests.forecast_labour_unknown":
    "Pas de prévision du coût de main-d'œuvre.",
  "p.plan_change_requests.forecast_no_edits":
    "Cette demande ne porte aucune modification de simulation ; il n'y a donc pas de prévision, et son impact indiqué est le chiffre du demandeur lui-même.",
  "p.plan_change_requests.baseline_heading":
    "Une référence à cause de ce changement",
  "p.plan_change_requests.baseline_done":
    "Une référence de planning a été prise à cause de ce changement.",
  "p.plan_change_requests.baseline_says":
    "En prendre une est le choix d'une personne ; rien n'est re-référencé automatiquement. Une référence par changement, dont le motif nomme la demande, et elle ne peut pas être annulée.",
  "p.plan_change_requests.baseline_take": "Prendre une référence",
  "p.plan_change_requests.baseline_not_yet":
    "Une référence peut être prise une fois le changement approuvé.",
  "p.escalations.problem_outcome":
    "Choisissez un résultat : décidé, orientation donnée ou rejeté.",
  "p.escalations.problem_response":
    "Rédigez une réponse de {min} à {max} caractères.",
  "p.escalations.refused_ack_right":
    "Refusé : accuser réception d'une escalade exige le droit d'accuser réception des escalades (un administrateur le détient). Rien n'a été enregistré.",
  "p.escalations.refused_resolve_right":
    "Refusé : résoudre une escalade exige le droit de résoudre les escalades (un administrateur le détient). Rien n'a été enregistré.",
  "p.escalations.refused_ack_state":
    "Refusé : cette escalade a été résolue ou retirée entre-temps, on ne peut donc plus en accuser réception. Elle a été relue.",
  "p.escalations.refused_resolve_state":
    "Refusé : cette escalade a été retirée, ou elle est déjà résolue avec un autre résultat ou une autre réponse ; une résolution n'est jamais écrasée. Elle a été relue.",
  "p.escalations.refused_resolve_invalid":
    "Refusé : le résultat ou la réponse n'a pas été accepté. Vérifiez les règles ci-dessus et réessayez.",
  "p.escalations.refused_gone":
    "Cette escalade n'existe plus, ou vous ne pouvez plus la voir. La liste a été relue.",
  "p.escalations.could_not_save": "Cela n'a pas été enregistré.",
  "p.escalations.by_unrecorded": "une personne non enregistrée",
  "p.escalations.ack_record":
    "Réception accusée au niveau {level} par {by}, {at}.",
  "p.escalations.resolution_record": "Résolue {at} par {by} : {outcome}.",
  "p.escalations.outcome_unrecorded": "aucun résultat enregistré",
  "p.escalations.records_heading": "Accusé de réception et résolution",
  "p.escalations.records_failed":
    "L'accusé de réception ou la résolution n'a pas pu être lu ; l'historique ci-dessous montre toujours ce qui s'est passé.",
  "p.escalations.ack_note_line": "Remarque : {note}",
  "p.escalations.ack_none": "Pas d'accusé de réception à son niveau actuel.",
  "p.escalations.resolution_kept":
    "Une résolution est un enregistrement : elle n'est ni modifiée ni écrasée.",
  "p.escalations.records_rules":
    "Accuser réception exige le droit d'accuser réception des escalades, et résoudre le droit de les résoudre ; un administrateur détient les deux, et cette page ne peut pas savoir à l'avance si vous les détenez. Un accusé de réception vaut au niveau actuel : faire monter l'escalade d'un niveau l'efface. Une résolution est un enregistrement et n'est jamais écrasée.",
  "p.escalations.ack_heading": "Accuser réception de cette escalade",
  "p.escalations.ack_note": "Remarque (facultative)",
  "p.escalations.ack_done":
    "Réception accusée ; l'enregistrement figure ci-dessous.",
  "p.escalations.resolve_heading": "Résoudre cette escalade",
  "p.escalations.resolve_says":
    "La réponse compte de {min} à {max} caractères. On vous demande confirmation avant de l'enregistrer, et elle ne peut plus être modifiée ensuite.",
  "p.escalations.confirm_resolve":
    "Enregistrer cette résolution de « {title} » ? Elle ne pourra plus être modifiée ni écrasée ensuite.",
  "p.escalations.resolution_done":
    "Résolue ; l'enregistrement figure ci-dessous.",
  "p.register_completeness.link": "Complétude du registre",
  "p.register_completeness.title": "Complétude du registre",
  "p.register_completeness.title_ppm": "Complétude du registre — PPM",
  "p.register_completeness.intro":
    "Le degré de complétude du registre de chaque programme : parmi les éléments qu'un programme devrait avoir enregistrés, la part qu'il a, et chaque élément manquant, nommé. Seuls les programmes (plans de type Programme) que vous pouvez lire sont listés.",
  "p.register_completeness.rules":
    "Les dix : un responsable principal (le responsable du programme en tient lieu), des partenaires, une étape, une référence de calendrier, une date cible, un budget, une source de financement sur chaque ligne budgétaire, une évaluation de la confiance de livraison, des risques et des dépendances. Les partenaires ne sont demandés que si le modèle de livraison en comporte, et une source de financement que s'il existe une ligne budgétaire. Le programme et les plans qui en dépendent comptent ensemble.",
  "p.register_completeness.switch_on":
    "« Pas de registre, pas d'argent » est activé : libérer une ligne budgétaire ou réserver une affectation ferme sur un programme incomplet est refusé, en nommant ce qui manque.",
  "p.register_completeness.switch_off":
    "« Pas de registre, pas d'argent » est désactivé : libérer une ligne budgétaire ou réserver une affectation ferme sur un programme incomplet se fait, et l'avertissement est conservé dans la piste d'audit.",
  "p.register_completeness.count_one":
    "{complete} programme sur {n} est complet.",
  "p.register_completeness.count_other":
    "{complete} programmes sur {n} sont complets.",
  "p.register_completeness.none":
    "Il n'y a aucun programme que vous pouvez lire.",
  "p.register_completeness.loading": "Chargement…",
  "p.register_completeness.could_not_load":
    "Impossible de charger la complétude du registre.",
  "p.register_completeness.table": "Les programmes et leurs registres",
  "p.register_completeness.col_programme": "Nom du programme",
  "p.register_completeness.col_state": "État",
  "p.register_completeness.col_score": "Part enregistrée",
  "p.register_completeness.col_on_record": "Éléments enregistrés",
  "p.register_completeness.col_missing": "Manquant",
  "p.register_completeness.col_not_asked": "Non demandé",
  "p.register_completeness.of": "{present} sur {applicable}",
  "p.register_completeness.state_complete": "complet",
  "p.register_completeness.state_incomplete": "incomplet",
  "p.register_completeness.field_other": "un autre élément",
  "p.register_completeness.none_listed": "aucun",
  "p.meetings.title": "Réunions",
  "p.meetings.title_ppm": "Réunions — PPM",
  "p.meetings.link": "Réunions, procès-verbaux et actions à mener",
  "p.meetings.intro":
    "Une réunion est planifiée avec un titre, une date et un ordre du jour. Ensuite, elle est enregistrée une seule fois comme tenue, avec son procès-verbal et ses résultats (décisions, actions à mener et remarques), ou bien elle est annulée. Une réunion tenue ou annulée est un enregistrement : un procès-verbal enregistré ne peut pas être corrigé et une réunion annulée ne peut pas être rétablie. Les participants ne sont pas enregistrés.",
  "p.meetings.plan_heading": "Planifier une réunion",
  "p.meetings.plan_says":
    "Un titre de {title} caractères au plus et une date. L'ordre du jour contient au plus {items} points de {chars} caractères au plus chacun ; un point peut renvoyer à une tâche, un risque, une étape clé ou un problème de ce plan.",
  "p.meetings.f_title": "Titre ({max} caractères au plus)",
  "p.meetings.f_date": "Date prévue",
  "p.meetings.agenda_item": "Point {n} de l'ordre du jour",
  "p.meetings.f_item_text":
    "Sujet du point {n} de l'ordre du jour ({max} caractères au plus)",
  "p.meetings.f_item_link":
    "L'élément concerné par le point {n} de l'ordre du jour",
  "p.meetings.no_record": "Aucun élément",
  "p.meetings.records_loading": "Chargement des éléments du plan…",
  "p.meetings.remove_item": "Retirer le point {n} de l'ordre du jour",
  "p.meetings.add_item": "Ajouter un point à l'ordre du jour",
  "p.meetings.plan": "Planifier la réunion",
  "p.meetings.planned": "Planifiée : {title}, le {date}.",
  "p.meetings.list_heading":
    "Les réunions du plan, de la plus récente à la plus ancienne",
  "p.meetings.pdf_says":
    "Chaque réunion se télécharge en document PDF reprenant ce que montre cette page ; ses mots fixes (les titres et l'état) sont en anglais.",
  "p.meetings.loading": "Chargement des réunions…",
  "p.meetings.none": "Aucune réunion pour l'instant.",
  "p.meetings.status_planned": "planifiée",
  "p.meetings.status_held": "tenue",
  "p.meetings.status_cancelled": "annulée",
  "p.meetings.held_on": "tenue le {date}",
  "p.meetings.scheduled_on": "prévue le {date}",
  "p.meetings.show": "Ouvrir la réunion {title}",
  "p.meetings.hide": "Fermer la réunion {title}",
  "p.meetings.pdf": "Télécharger en PDF",
  "p.meetings.pdf_named": "Télécharger la réunion {title} en PDF",
  "p.meetings.agenda_heading": "Ordre du jour",
  "p.meetings.agenda_none": "Aucun point à l'ordre du jour.",
  "p.meetings.minutes_heading": "Procès-verbal",
  "p.meetings.outcomes_heading": "Résultats",
  "p.meetings.outcomes_none": "Aucun résultat n'a été enregistré.",
  "p.meetings.outcome_decision": "Décision",
  "p.meetings.outcome_action": "Action à mener",
  "p.meetings.outcome_note": "Remarque",
  "p.meetings.about": "concerne {record}",
  "p.meetings.link_gone": "concerne un élément qui n'existe plus",
  "p.meetings.task_made": "la tâche créée à partir du résultat {n}",
  "p.meetings.held_kept":
    "Cette réunion est un enregistrement : son procès-verbal et ses résultats ne sont pas modifiés.",
  "p.meetings.cancelled_kept":
    "Cette réunion a été annulée. C'est un enregistrement et elle ne peut pas être rétablie.",
  "p.meetings.held_heading": "Enregistrer {title} comme tenue",
  "p.meetings.held_says":
    "Un procès-verbal de {minutes} caractères au plus, et jusqu'à {items} résultats de {chars} caractères au plus chacun : chacun est une décision, une action à mener ou une remarque, et peut renvoyer à un élément de ce plan. La date de tenue est la date prévue, sauf si vous en indiquez une autre. Cette opération se fait une seule fois : le procès-verbal ne pourra plus être corrigé ensuite.",
  "p.meetings.f_minutes": "Procès-verbal ({max} caractères au plus)",
  "p.meetings.f_held_on": "Date de tenue (vide pour la date prévue)",
  "p.meetings.outcome_n": "Résultat {n}",
  "p.meetings.f_outcome_kind": "Nature du résultat {n}",
  "p.meetings.choose_kind": "Choisir…",
  "p.meetings.f_outcome_text":
    "Le résultat {n} en quelques mots ({max} caractères au plus)",
  "p.meetings.f_outcome_link": "L'élément concerné par le résultat {n}",
  "p.meetings.remove_outcome": "Retirer le résultat {n}",
  "p.meetings.add_outcome": "Ajouter un résultat",
  "p.meetings.record_held": "Enregistrer comme tenue",
  "p.meetings.cancel_meeting": "Annuler la réunion {title}",
  "p.meetings.confirm_held":
    "Enregistrer {title} comme tenue ? Son procès-verbal et ses résultats ne pourront plus être corrigés ensuite.",
  "p.meetings.confirm_cancel":
    "Annuler {title} ? Une réunion annulée ne peut pas être rétablie.",
  "p.meetings.held_done": "Enregistrée comme tenue : {title}.",
  "p.meetings.cancelled_done": "Annulée : {title}.",
  "p.meetings.problem_title":
    "Saisissez un titre de {title} caractères au plus.",
  "p.meetings.problem_date": "Saisissez une date.",
  "p.meetings.problem_agenda_count":
    "Un ordre du jour contient au plus {items} points.",
  "p.meetings.problem_outcome_count":
    "Une réunion contient au plus {items} résultats.",
  "p.meetings.problem_item_text": "Rédigez-le en {chars} caractères au plus.",
  "p.meetings.problem_minutes":
    "Rédigez le procès-verbal, en {minutes} caractères au plus.",
  "p.meetings.problem_outcome_kind":
    "Choisissez une décision, une action à mener ou une remarque.",
  "p.meetings.problem_on_line": "Pour {line} : {problem}",
  "p.meetings.could_not_load": "Impossible de charger les réunions.",
  "p.meetings.could_not_save": "La réunion n'a pas été modifiée.",
  "p.meetings.refused_gone":
    "Refusé : cette réunion, ou le plan, n'existe plus. La liste a été relue.",
  "p.meetings.refused_settled":
    "Refusé : cette réunion a déjà été tenue ou annulée ; c'est donc un enregistrement et elle n'est pas modifiée. Elle a été relue.",
  "p.meetings.refused_link":
    "Refusé : un élément auquel renvoie un point n'appartient plus à ce plan. Rien n'a été enregistré ; choisissez de nouveau les éléments.",
  "p.meetings.refused_not_held":
    "Refusé : des tâches ne sont proposées qu'à partir d'une réunion tenue et dotée d'un procès-verbal.",
  "p.meetings.refused_commit_state":
    "Refusé : la réunion n'est pas tenue, ou l'une de ces actions à mener a entre-temps une tâche. Rien n'a été créé ; la réunion a été relue, vérifiez de nouveau.",
  "p.meetings.refused_commit_invalid":
    "Refusé : une proposition n'est plus prête. Rien n'a été créé ; vérifiez de nouveau les propositions.",
  "p.meetings.refused_proposals":
    "Refusé : envoyez entre 1 et {count} propositions.",
  "p.meetings.tasks_heading": "Tâches issues des actions à mener",
  "p.meetings.tasks_says":
    "Pour chaque action à mener sans tâche, vous pouvez en proposer une : un titre de {max} caractères au plus et un passage du procès-verbal qui l'appuie, d'au moins {min} caractères (majuscules et espacement ignorés). Un responsable est facultatif et doit déjà figurer parmi les tâches de ce plan ou son tableau des responsabilités, sous la forme d'une référence telle que person:… ; une date limite est facultative, du {first} au {last} ; les points vont de 0 à {points}. Jusqu'à {count} à la fois. La vérification n'écrit rien ; les tâches sont ensuite créées toutes ou aucune, et une action à mener n'obtient qu'une seule tâche.",
  "p.meetings.proposal_legend": "Action à mener {n} : {text}",
  "p.meetings.f_propose": "Proposer une tâche pour l'action à mener {n}",
  "p.meetings.f_task_title":
    "Titre de la tâche pour l'action à mener {n} ({max} caractères au plus)",
  "p.meetings.f_excerpt":
    "Passage du procès-verbal pour l'action à mener {n} (au moins {min} caractères)",
  "p.meetings.f_assignee":
    "Responsable de l'action à mener {n} (référence facultative)",
  "p.meetings.f_due": "Date limite de l'action à mener {n} (facultative)",
  "p.meetings.f_points":
    "Points de l'action à mener {n} (facultatifs, de 0 à {max})",
  "p.meetings.check": "Vérifier les propositions",
  "p.meetings.create_tasks": "Créer les tâches",
  "p.meetings.none_chosen":
    "Aucune proposition n'est choisie ; rien n'a donc été envoyé.",
  "p.meetings.checked_one":
    "Vérifié, rien n'est écrit : {n} proposition est prête, {dropped} ne le sont pas.",
  "p.meetings.checked_other":
    "Vérifié, rien n'est écrit : {n} propositions sont prêtes, {dropped} ne le sont pas.",
  "p.meetings.ready": "Prête, sans responsable.",
  "p.meetings.ready_for": "Prête, pour {who}.",
  "p.meetings.not_all_ready":
    "Chaque proposition choisie doit être prête avant de pouvoir créer les tâches : modifiez ou décochez celles qui ne le sont pas, puis vérifiez de nouveau.",
  "p.meetings.check_again":
    "Les propositions ont changé depuis leur vérification : vérifiez-les de nouveau.",
  "p.meetings.confirm_tasks_one":
    "Créer {n} tâche à partir des actions à mener de {title} ? Une action à mener n'obtient qu'une seule tâche.",
  "p.meetings.confirm_tasks_other":
    "Créer {n} tâches à partir des actions à mener de {title} ? Une action à mener n'obtient qu'une seule tâche.",
  "p.meetings.tasks_made_one":
    "{n} tâche créée ; elle est liée sous son action à mener.",
  "p.meetings.tasks_made_other":
    "{n} tâches créées ; chacune est liée sous son action à mener.",
  "p.meetings.could_not_check": "Impossible de vérifier les propositions.",
  "p.meetings.could_not_create": "Les tâches n'ont pas été créées.",
  "p.meetings.dropped_title_invalid":
    "Non prête : le titre est vide ou trop long.",
  "p.meetings.dropped_excerpt_not_in_minutes":
    "Non prête : le passage est introuvable dans le procès-verbal, ou trop court.",
  "p.meetings.dropped_outcome_not_found":
    "Non prête : la réunion n'a pas un tel résultat.",
  "p.meetings.dropped_outcome_not_an_action":
    "Non prête : ce résultat n'est pas une action à mener.",
  "p.meetings.dropped_outcome_already_has_a_task":
    "Non prête : cette action à mener a déjà une tâche.",
  "p.meetings.dropped_due_outside_window":
    "Non prête : la date limite est en dehors des dates autorisées.",
  "p.meetings.dropped_points_invalid":
    "Non prête : les points sont en dehors de la plage autorisée.",
  "p.meetings.dropped_assignee_not_in_plan":
    "Non prête : le responsable ne fait pas partie des personnes de ce plan (dans ses tâches ou son tableau des responsabilités).",
  "p.meetings.dropped_outcome_proposed_twice":
    "Non prête : la même action à mener est proposée deux fois.",
  "p.meetings.dropped_other":
    "Non prête, pour une raison que cette page ne connaît pas.",
  "p.meetings.problem_proposals_count":
    "Proposez au plus {count} tâches à la fois.",
  "p.meetings.problem_task_title":
    "Donnez à la tâche un titre de {max} caractères au plus.",
  "p.meetings.problem_excerpt":
    "Citez un passage d'au moins {min} caractères qui figure dans le procès-verbal.",
  "p.meetings.problem_due":
    "La date limite doit se situer entre une semaine avant la réunion et un an après.",
  "p.meetings.problem_points":
    "Les points sont un nombre entier de 0 à {points}.",
  "p.lessons.title": "Leçons apprises",
  "p.lessons.title_ppm": "Leçons apprises — PPM",
  "p.lessons.link": "Leçons apprises",
  "p.lessons.intro":
    "Une leçon est enregistrée à un jalon de phase ou à la clôture du plan. Elle n'est jamais modifiée : elle peut seulement être retirée, et une leçon retirée n'est plus affichée (elle reste dans la piste d'audit). Une nouvelle proposition dont le titre ressemble à celui de ce plan voit s'afficher ses leçons.",
  "p.lessons.record_heading": "Enregistrer une leçon",
  "p.lessons.record_says":
    "Quand (un jalon ou la clôture), de quoi il s'agit, un résumé de {max} caractères au plus et, si vous le souhaitez, un détail de {bytes} octets au plus (une lettre accentuée ou un autre alphabet occupe plus d'un octet) ainsi que la tâche, le risque, l'étape clé ou le problème de ce plan qu'elle concerne.",
  "p.lessons.f_occasion": "Quand",
  "p.lessons.choose_occasion": "Choisir un jalon ou la clôture…",
  "p.lessons.closure": "Clôture",
  "p.lessons.occasion_other": "Une autre occasion",
  "p.lessons.f_category": "De quoi il s'agit",
  "p.lessons.category_went_well": "S'est bien passé",
  "p.lessons.category_went_badly": "S'est mal passé",
  "p.lessons.category_recommendation": "Recommandation",
  "p.lessons.category_other": "Un autre type",
  "p.lessons.f_summary": "Résumé ({max} caractères au plus)",
  "p.lessons.f_detail": "Détail (facultatif, {bytes} octets au plus)",
  "p.lessons.f_link": "L'élément concerné (facultatif)",
  "p.lessons.record": "Enregistrer la leçon",
  "p.lessons.recorded": "Enregistrée : {summary}",
  "p.lessons.problem_occasion": "Choisissez un jalon ou la clôture.",
  "p.lessons.problem_category":
    "Choisissez s'est bien passé, s'est mal passé ou recommandation.",
  "p.lessons.problem_summary": "Rédigez un résumé de {max} caractères au plus.",
  "p.lessons.problem_detail": "Le détail dépasse {bytes} octets.",
  "p.lessons.list_heading":
    "Les leçons du plan, de la plus ancienne à la plus récente",
  "p.lessons.loading": "Chargement des leçons…",
  "p.lessons.none": "Aucune leçon pour l'instant.",
  "p.lessons.count_one": "{n} leçon.",
  "p.lessons.count_other": "{n} leçons.",
  "p.lessons.recorded_on": "enregistrée le {date}",
  "p.lessons.recorded_on_by": "enregistrée le {date} par {who}",
  "p.lessons.link_unlisted":
    "concerne un élément qui ne figure pas parmi les tâches, risques, étapes clés et problèmes du plan listés ici",
  "p.lessons.withdraw": "Retirer la leçon {summary}",
  "p.lessons.withdraw_short": "Retirer",
  "p.lessons.confirm_withdraw":
    "Retirer la leçon « {summary} » ? Elle ne sera plus affichée, ni ici ni dans les nouvelles propositions ; elle reste dans la piste d'audit.",
  "p.lessons.withdrawn": "Retirée : {summary}",
  "p.lessons.could_not_load": "Impossible de charger les leçons.",
  "p.lessons.could_not_save": "La leçon n'a pas été modifiée.",
  "p.lessons.refused_gone": "Refusé : le plan n'existe plus.",
  "p.lessons.refused_withdrawn":
    "Refusé : cette leçon a déjà été retirée. La liste a été relue.",
  "p.lessons.refused_link":
    "Refusé : l'élément choisi n'appartient plus à ce plan. Rien n'a été enregistré ; choisissez-le de nouveau.",
  "p.lessons.refs_heading": "Ce qui renvoie à un élément",
  "p.lessons.refs_says":
    "Choisissez une tâche, un risque, une étape clé ou un problème du plan pour lister les points de réunion et les leçons qui y renvoient (jusqu'à 100 de chaque).",
  "p.lessons.f_target": "L'élément",
  "p.lessons.choose_target": "Choisir un élément…",
  "p.lessons.show_refs": "Afficher ce qui y renvoie",
  "p.lessons.problem_target": "Choisissez d'abord un élément.",
  "p.lessons.refs_count_one": "{n} point y renvoie.",
  "p.lessons.refs_count_other": "{n} points y renvoient.",
  "p.lessons.refs_meetings": "Points de réunion",
  "p.lessons.refs_lessons": "Leçons qui y renvoient",
  "p.lessons.refs_agenda": "Ordre du jour",
  "p.lessons.refused_no_record":
    "Cet élément n'appartient plus à ce plan, ou vous ne pouvez pas le lire.",
  "p.lessons.could_not_read_refs": "Impossible de lire ce qui y renvoie.",
  "p.lessons.similar": "Leçons de plans similaires",
  "p.lessons.similar_named": "Leçons de plans semblables à {title}",
  "p.lessons.similar_heading": "Leçons de plans semblables à {title}",
  "p.lessons.similar_says":
    "Rapprochement sur le titre seulement, avec tous les plans ayant des leçons, clos ou non, le meilleur rapprochement en premier. Le score et la confiance sont ceux du moteur de rapprochement.",
  "p.lessons.similar_none": "Aucun plan similaire n'a de leçons.",
  "p.lessons.match": "correspondance {score}, confiance {confidence}.",
  "p.lessons.match_title": "Titres semblables à {score}.",
  "p.lessons.confidence_high": "élevée",
  "p.lessons.confidence_medium": "moyenne",
  "p.lessons.confidence_low": "faible",
  "p.lessons.refused_proposal_gone":
    "Cette proposition n'existe plus, ou vous ne pouvez pas la lire.",
  "p.lessons.could_not_read_similar":
    "Impossible de lire les leçons des plans similaires.",
  "p.director_pack.open_by_severity": "Ouverts par gravité : {list}",
  "p.director_pack.issues_overdue": "Problèmes en retard : {n}",
  "p.director_pack.issues_in_window":
    "Sur la période : {raised} signalés, {resolved} résolus",
  "p.director_pack.oldest_issue":
    "Plus ancien problème ouvert : {title} (jours calendaires ouvert : {n})",
  "p.director_pack.approved_not_implemented":
    "Approuvées mais pas encore mises en œuvre : {n}",
  "p.director_pack.decided_in_window": "Décidées sur la période : {n}",
  "p.director_pack.changes_by_status": "Par statut : {list}",
  "p.director_pack.oldest_change":
    "Plus ancienne demande de changement ouverte : {title} (jours calendaires ouverte : {n})",
  "p.live_pack.heading": "Relevés des programmes, maintenant",
  "p.live_pack.says":
    "Lus maintenant à partir des plans que vous pouvez lire, présentés comme le dossier de direction les présente. Un dossier publié garde ses chiffres ; le point précédent du graphique de fièvre ne vient que d'un dossier publié.",
  "p.live_pack.f_ac": "Coût réel pour la valeur acquise",
  "p.live_pack.ac_budget_lines": "Lignes budgétaires (par défaut)",
  "p.live_pack.ac_labour": "Coût de main-d'œuvre enregistré",
  "p.live_pack.ac_both": "Lignes budgétaires et coût de main-d'œuvre",
  "p.live_pack.f_window":
    "Problèmes et demandes de changement des derniers jours ({min} à {max} ; vide pour 30)",
  "p.live_pack.read_again": "Relire",
  "p.live_pack.problem_window":
    "Le nombre de jours doit être un entier de {min} à {max}, ou vide pour 30.",
  "p.live_pack.could_not_read_fever":
    "Impossible de lire le graphique de fièvre du tampon.",
  "p.live_pack.could_not_read_evm": "Impossible de lire la valeur acquise.",
  "p.live_pack.could_not_read_benefits": "Impossible de lire les bénéfices.",
  "p.live_pack.could_not_read_issues":
    "Impossible de lire les problèmes et demandes de changement.",
  "p.live_pack.issues_heading": "Problèmes et demandes de changement",
  "p.live_pack.issues_none":
    "Vous ne pouvez lire aucun plan comportant des problèmes ou des demandes de changement.",
  "p.live_pack.nothing": "Rien à afficher.",
  "p.forecast.heading": "Prévision à l'achèvement",
  "p.forecast.says":
    "L'estimation à l'achèvement est le coût réel à ce jour plus l'estimation pour achever. L'estimation pour achever est le dernier chiffre de contrôle total du projet dans la devise de la prévision, sinon ce que la dernière référence prévoit encore. Lue maintenant ; rien n'est stocké.",
  "p.forecast.absent_no_currency":
    "Pas de prévision : le plan n'a ni référence budgétaire ni chiffre de contrôle total du projet, donc aucune devise dans laquelle prévoir.",
  "p.forecast.c_actual": "Coût réel à ce jour",
  "p.forecast.c_etc": "Estimation pour achever",
  "p.forecast.c_eac": "Estimation à l'achèvement",
  "p.forecast.c_plans": "Plans contributeurs",
  "p.forecast.not_known": "inconnu",
  "p.forecast.etc_tpc":
    "d'après le dernier chiffre de contrôle total du projet",
  "p.forecast.etc_baseline": "d'après ce que la référence prévoit encore",
  "p.forecast.no_baseline_used":
    "Aucune référence budgétaire n'a été utilisée.",
  "p.forecast.baseline_used":
    "Utilise la version {version} de la référence budgétaire.",
  "p.forecast.as_of": "Lu le {date}.",
  "p.forecast.other_currency":
    "Le coût réel enregistré dans une autre devise que {currency} n'est pas ajouté.",
  "p.forecast.baselines_heading": "Références budgétaires",
  "p.forecast.baselines_says":
    "Une référence est le budget approuvé par période, dans une devise. Une fois approuvée, elle est figée et jamais modifiée ; une nouvelle référence est une nouvelle version, et chaque version après la première exige un motif. Une référence compte de 1 à {max} périodes, chacune avec une fin non antérieure à son début et un montant prévu non négatif.",
  "p.forecast.no_baselines": "Aucune référence budgétaire n'a été approuvée.",
  "p.forecast.version_line":
    "Version {version} · {currency} · approuvée le {date}",
  "p.forecast.period_line": "{start} au {end} : {amount}",
  "p.forecast.first_baseline_heading": "Approuver la première référence",
  "p.forecast.rebaseline_heading": "Approuver la version {version}",
  "p.forecast.f_currency": "Devise (trois lettres)",
  "p.forecast.periods_legend": "Périodes",
  "p.forecast.f_start": "Début de la période {n}",
  "p.forecast.f_end": "Fin de la période {n}",
  "p.forecast.f_amount": "Montant prévu de la période {n}",
  "p.forecast.remove_period": "Retirer la période {n}",
  "p.forecast.add_period": "Ajouter une période",
  "p.forecast.f_reason": "Motif (facultatif pour la première)",
  "p.forecast.f_reason_needed":
    "Pourquoi le budget est re-référencé (obligatoire)",
  "p.forecast.approve": "Approuver la référence",
  "p.forecast.confirm_baseline":
    "Approuver la version {version} de la référence budgétaire ? Une fois approuvée, elle est figée et ne peut être ni modifiée ni retirée.",
  "p.forecast.baseline_approved": "Version {version} approuvée.",
  "p.forecast.problem_currency":
    "La devise doit être un code de trois lettres, comme GBP ou EUR.",
  "p.forecast.problem_no_periods": "Indiquez au moins une période.",
  "p.forecast.problem_too_many": "Une référence compte au plus {max} périodes.",
  "p.forecast.problem_dates":
    "Chaque période doit avoir une date de début et une date de fin.",
  "p.forecast.problem_order":
    "Une période ne peut pas finir avant de commencer.",
  "p.forecast.problem_amount":
    "Chaque montant prévu doit être un nombre, non négatif.",
  "p.forecast.problem_reason": "Une nouvelle référence exige un motif.",
  "p.forecast.problem_reason_long":
    "Le motif est trop long : au plus {bytes} octets.",
  "p.forecast.problem_same":
    "C'est identique à la dernière version ; rien n'a été envoyé.",
  "p.forecast.refused_gone":
    "Ce plan n'existe plus, ou vous ne pouvez pas le lire.",
  "p.forecast.refused_baseline":
    "La référence a été refusée : une autre version a peut-être été approuvée entre-temps, ce qui rend un motif nécessaire. La liste a été relue.",
  "p.forecast.could_not_approve": "Impossible d'approuver la référence.",
  "p.forecast.could_not_read": "Impossible de lire la prévision.",
  "p.forecast.could_not_read_baselines":
    "Impossible de lire les références budgétaires.",
  "p.forecast.could_not_read_lines":
    "Impossible de lire les lignes budgétaires.",
  "p.forecast.tranches_heading": "Financement retenu jusqu'à un jalon",
  "p.forecast.tranches_says":
    "Une ligne budgétaire avec un jalon de financement est retenue jusqu'à ce que l'étape du plan atteigne ce jalon ; elle peut alors être débloquée, une fois, et seulement alors un coût réel peut y être enregistré. Un déblocage ne peut pas être annulé.",
  "p.forecast.stage_now": "Étape du plan : {stage}.",
  "p.forecast.stage_none": "Le plan n'a encore franchi aucun jalon.",
  "p.forecast.no_tranches":
    "Aucune ligne budgétaire n'est retenue jusqu'à un jalon.",
  "p.forecast.gate_line": "retenue jusqu'à {gate}",
  "p.forecast.state_released": "débloquée le {date}",
  "p.forecast.state_held": "encore retenue : le jalon n'est pas franchi",
  "p.forecast.release": "Débloquer {description}",
  "p.forecast.confirm_release":
    "Débloquer {description}, retenue jusqu'à {gate} ? Un déblocage ne peut pas être annulé.",
  "p.forecast.released": "{description} débloquée.",
  "p.forecast.refused_line_gone":
    "Cette ligne budgétaire ou ce plan n'existe plus.",
  "p.forecast.refused_register":
    "Refusé : le registre du programme est incomplet. Complétez-le d'abord sur la page de complétude du registre.",
  "p.forecast.refused_release":
    "Refusé : la ligne n'est plus retenue, ou son jalon n'est pas franchi. Les lignes ont été relues.",
  "p.forecast.could_not_release": "Impossible de débloquer la ligne.",
  "p.forecast.rollup_heading": "Prévision à l'achèvement, consolidée",
  "p.forecast.rollup_says":
    "D'un plan sur les plans qui en dépendent, jusqu'à {max} niveaux, une ligne par devise : les devises ne sont jamais additionnées. Les plans sans devise de prévision sont comptés, pas ajoutés.",
  "p.forecast.f_plan": "Plan de départ",
  "p.forecast.choose_plan": "Choisissez un plan",
  "p.forecast.f_depth": "Niveaux de profondeur (1 à {max} ; vide pour tous)",
  "p.forecast.show_rollup": "Afficher la prévision",
  "p.forecast.problem_plan": "Choisissez un plan.",
  "p.forecast.problem_depth":
    "Les niveaux doivent être un entier de 1 à {max}, ou vide.",
  "p.forecast.refused_plan_gone":
    "Ce plan n'existe plus, ou vous ne pouvez pas le lire.",
  "p.forecast.could_not_read_rollup":
    "Impossible de lire la prévision consolidée.",
  "p.forecast.rollup_line_one": "Depuis {name} : {n} plan lu, {date}.",
  "p.forecast.rollup_line_other": "Depuis {name} : {n} plans lus, {date}.",
  "p.forecast.rollup_truncated":
    "L'arbre est plus grand que ce que le service parcourt ; les chiffres en couvrent une partie.",
  "p.forecast.rollup_revisits_one":
    "{n} plan a été atteint deux fois et compté une fois.",
  "p.forecast.rollup_revisits_other":
    "{n} plans ont été atteints deux fois et comptés une fois.",
  "p.forecast.rollup_no_currency_one":
    "{n} plan n'a pas de devise de prévision et n'est pas ajouté.",
  "p.forecast.rollup_no_currency_other":
    "{n} plans n'ont pas de devise de prévision et ne sont pas ajoutés.",
  "p.forecast.rollup_empty": "Aucun plan ici n'a de prévision.",
  "p.value.heading": "Valeur réalisée",
  "p.value.says":
    "Lu maintenant à partir de la valeur enregistrée ; rien n'est stocké. Le retour sur investissement compare chaque point de valeur au coût réel enregistré, et il est retenu quand les lignes budgétaires utilisent plus d'une devise, aucune devise n'étant convertie. Un chiffre impossible à donner dit pourquoi, jamais zéro.",
  "p.value.roi": "Retour sur investissement : {pct}",
  "p.value.roi_absent": "Retour sur investissement non indiqué : {reason}",
  "p.value.measured_share": "preuves mesurées : {pct} des points de valeur",
  "p.value.ttv_one":
    "Délai de valeur : médiane {p50} jours calendaires, 85e centile {p85} ({n} observation)",
  "p.value.ttv_other":
    "Délai de valeur : médiane {p50} jours calendaires, 85e centile {p85} ({n} observations)",
  "p.value.ttv_absent": "Délai de valeur non indiqué : {reason}",
  "p.value.adoption":
    "Adoption : {pct} ({active} sur {target} utilisateurs actifs sur {days} jours)",
  "p.value.adoption_definition": "un utilisateur actif est : {definition}",
  "p.value.adoption_absent": "Adoption non indiquée : {reason}",
  "p.value.adoption_none": "Aucun relevé d'adoption n'est enregistré.",
  "p.value.absent_unrealized": "aucune valeur n'a été enregistrée",
  "p.value.absent_no_denominator":
    "il n'y a rien à quoi le comparer (pas de coût réel ou pas de cible)",
  "p.value.absent_mixed_currency":
    "les lignes budgétaires utilisent plus d'une devise",
  "p.value.absent_no_baseline": "il n'y a pas de référence budgétaire",
  "p.value.absent_no_responses": "personne n'a répondu",
  "p.value.absent_overflow": "le chiffre est trop grand pour être calculé",
  "p.value.absent_other": "le service n'a pas dit pourquoi",
  "p.value.case_heading": "Cibles de l'analyse de rentabilité",
  "p.value.no_targets":
    "Aucune cible d'analyse de rentabilité n'est enregistrée.",
  "p.value.target_line":
    "{metric} : de {baseline} à {target} · {source} · approuvée le {date}",
  "p.value.source_charter": "la charte",
  "p.value.source_gate_review": "une revue de jalon",
  "p.value.source_other": "une autre source",
  "p.value.to_costs": "Enregistrer une cible sur la page des coûts",
  "p.value.point_heading": "Enregistrer une valeur observée",
  "p.value.point_says":
    "Une valeur est mesurée, estimée ou affirmée. Avec une devise, c'est un montant ; sans, un entier dans l'unité de la mesure. Un point de valeur ne peut pas être modifié ici une fois enregistré. Un point de valeur par plan peut être la première valeur mesurable : il arrête l'horloge du délai de valeur, lancée par la première cible de l'analyse de rentabilité, et ne peut pas être déplacé.",
  "p.value.f_value": "Valeur observée",
  "p.value.f_currency": "Devise (trois lettres, ou vide)",
  "p.value.f_method": "Comment elle a été obtenue",
  "p.value.method_measured": "mesurée",
  "p.value.method_estimated": "estimée",
  "p.value.method_asserted": "affirmée",
  "p.value.f_evidence": "Preuve (lien ou référence, facultatif)",
  "p.value.f_first": "C'est la première valeur mesurable",
  "p.value.record_point": "Enregistrer la valeur",
  "p.value.confirm_first":
    "Enregistrer ceci comme première valeur mesurable du plan ? Cela arrête l'horloge du délai de valeur et ne peut pas être déplacé.",
  "p.value.point_recorded": "Valeur enregistrée.",
  "p.value.problem_currency":
    "La devise doit être un code de trois lettres, ou vide.",
  "p.value.problem_amount": "La valeur doit être un montant.",
  "p.value.problem_whole": "Sans devise, la valeur doit être un nombre entier.",
  "p.value.problem_method": "Choisissez comment la valeur a été obtenue.",
  "p.value.refused_first_taken":
    "Refusé : ce plan a déjà une première valeur mesurable ; l'horloge ne s'arrête qu'une fois.",
  "p.value.refused_value": "La valeur a été refusée.",
  "p.value.refused_gone":
    "Ce plan n'existe plus, ou vous n'avez pas accès en lecture.",
  "p.value.could_not_read": "Impossible de lire la valeur réalisée.",
  "p.value.could_not_record_point": "Impossible d'enregistrer la valeur.",
  "p.value.case_says":
    "Une cible est une mesure promise dans la charte ou lors d'une revue de jalon, avec sa valeur à l'approbation et la valeur promise, en nombres entiers dans l'unité de la mesure. La date d'approbation lance l'horloge du délai de valeur et ne peut pas être modifiée ; une cible ne peut être ni modifiée ni retirée ici.",
  "p.value.to_benefits": "La valeur réalisée figure sur la page des bénéfices.",
  "p.value.f_metric": "Mesure",
  "p.value.f_baseline": "Valeur à l'approbation",
  "p.value.f_target": "Valeur promise",
  "p.value.f_unit": "Unité (facultatif)",
  "p.value.f_source": "Promise dans",
  "p.value.record_target": "Enregistrer la cible",
  "p.value.confirm_target":
    "Enregistrer la cible {metric} ? Elle ne peut être ni modifiée ni retirée ici.",
  "p.value.target_recorded": "Cible {metric} enregistrée.",
  "p.value.problem_metric": "Nommez la mesure.",
  "p.value.problem_target_values":
    "La valeur à l'approbation et la valeur promise doivent être des nombres entiers.",
  "p.value.problem_source": "Choisissez où la cible a été promise.",
  "p.value.refused_target": "La cible a été refusée.",
  "p.value.could_not_record_target": "Impossible d'enregistrer la cible.",
  "p.scoring.rank_by": "Classer par",
  "p.scoring.model_smart": "Score Smart (d'après les preuves)",
  "p.scoring.model_wsjf": "WSJF (travail le plus court pondéré d'abord)",
  "p.scoring.model_rice": "RICE (portée, incidence, confiance, effort)",
  "p.scoring.models_say":
    "Le score Smart est tiré des preuves que détient le service ; WSJF et RICE sont calculés à partir des estimations d'une personne, saisies pour chaque plan sous son bouton Estimations. Un plan auquel il manque une entrée requise par un modèle reste sans score et est classé en dernier, jamais à zéro. La bande appartient au score Smart.",
  "p.scoring.of_plans_one": "{length} sur {scored_of} plan, classé par {model}",
  "p.scoring.of_plans_other":
    "{length} sur {scored_of} plans, classés par {model}",
  "p.scoring.ranked_by": "Plans classés par {model}",
  "p.scoring.waiting_for": "En attente de",
  "p.scoring.open": "Estimations",
  "p.scoring.open_named": "Ouvrir les estimations de {name}",
  "p.scoring.heading": "Estimations pour {name}",
  "p.scoring.form": "Les estimations de {name}",
  "p.scoring.says":
    "WSJF et RICE sont calculés à partir de ces estimations. S'il manque une entrée, aucun score n'est donné, jamais deviné ; un zéro déclaré n'est pas une entrée manquante. Enregistrer remplace chaque estimation : une case vide l'efface.",
  "p.scoring.last_set": "Saisies pour la dernière fois {when}.",
  "p.scoring.none_set": "Aucune estimation n'a été saisie.",
  "p.scoring.wsjf_inputs":
    "Entrées WSJF, chacune un nombre entier de {min} à {max}",
  "p.scoring.rice_inputs": "Entrées RICE",
  "p.scoring.input_business_value": "Valeur métier",
  "p.scoring.input_time_criticality": "Urgence temporelle",
  "p.scoring.input_risk_reduction":
    "Réduction du risque ou opportunité ouverte",
  "p.scoring.input_job_size": "Taille du travail",
  "p.scoring.input_reach": "Portée (personnes ou événements sur une période)",
  "p.scoring.input_impact": "Incidence",
  "p.scoring.input_confidence": "Confiance (pour cent)",
  "p.scoring.input_effort": "Effort (mois-personnes)",
  "p.scoring.input_other": "une autre entrée",
  "p.scoring.component_cost_of_delay": "Coût du retard",
  "p.scoring.impact_minimal": "minime",
  "p.scoring.impact_low": "faible",
  "p.scoring.impact_medium": "moyenne",
  "p.scoring.impact_high": "forte",
  "p.scoring.impact_massive": "énorme",
  "p.scoring.impact_value": "{value} (incidence {word})",
  "p.scoring.not_estimated": "non estimé",
  "p.scoring.save": "Enregistrer les estimations",
  "p.scoring.close": "Fermer les estimations",
  "p.scoring.unchanged":
    "Les estimations sont telles qu'enregistrées ; rien n'a été envoyé.",
  "p.scoring.saved": "Estimations de {name} enregistrées.",
  "p.scoring.could_not_read": "Impossible de lire les estimations.",
  "p.scoring.could_not_save": "Impossible d'enregistrer les estimations.",
  "p.scoring.refused_gone":
    "Ce plan n'existe plus, ou vous ne pouvez pas le lire.",
  "p.scoring.refused_inputs":
    "Les estimations ont été refusées : une entrée est hors de sa plage.",
  "p.scoring.problem_wsjf":
    "{input} doit être un nombre entier de {min} à {max}.",
  "p.scoring.problem_reach":
    "{input} doit être un nombre entier, pas en dessous de zéro.",
  "p.scoring.problem_impact": "{input} doit valoir 0,25, 0,5, 1, 2 ou 3.",
  "p.scoring.problem_confidence":
    "{input} doit être un nombre entier de 0 à 100.",
  "p.scoring.problem_effort": "{input} doit être supérieur à zéro.",
  "p.scoring.problem_number": "{input} doit être un nombre.",
  "p.scoring.formula_wsjf":
    "WSJF vaut (valeur métier + urgence temporelle + réduction du risque ou opportunité ouverte) ÷ taille du travail.",
  "p.scoring.formula_rice":
    "RICE vaut portée × incidence × confiance ÷ effort.",
  "p.scoring.score_is": "Score : {score}",
  "p.scoring.no_score": "Aucun score : il attend ces entrées.",
  "p.scoring.needs_delay_part": "elle fait partie du coût du retard",
  "p.scoring.needs_divisor":
    "c'est le diviseur : sans lui, il n'y a pas de valeur par unité de travail",
  "p.scoring.needs_reach":
    "elle dit combien de personnes le travail touche sur une période",
  "p.scoring.needs_impact":
    "elle dit combien le travail compte pour chaque personne touchée",
  "p.scoring.needs_confidence":
    "elle minore les estimations selon leur degré de certitude",
  "p.scoring.needs_other": "le score en a besoin",
  "p.scoring.means_business_value":
    "valeur relative pour l'utilisateur ou le métier, de 1 à 20",
  "p.scoring.means_time_criticality":
    "la vitesse à laquelle la valeur se perd en cas de retard, de 1 à 20",
  "p.scoring.means_risk_reduction":
    "le risque qu'il supprime ou l'opportunité qu'il ouvre, de 1 à 20",
  "p.scoring.means_cost_of_delay":
    "la somme des trois ci-dessus : ce que coûte un retard",
  "p.scoring.means_job_size":
    "la taille relative du travail, de 1 à 20 ; le diviseur",
  "p.scoring.means_reach": "personnes ou événements touchés sur une période",
  "p.scoring.means_impact":
    "0,25 minime, 0,5 faible, 1 moyenne, 2 forte, 3 énorme",
  "p.scoring.means_confidence": "le degré de certitude des estimations",
  "p.scoring.means_effort": "mois-personnes de travail ; le diviseur",
  "p.scoring.means_other": "une composante que cette page ne connaît pas",
  "p.frontier.heading": "Choisir un portefeuille",
  "p.frontier.says":
    "Le générateur note chaque plan que vous pouvez lire par son score Smart et le chiffre par ses lignes budgétaires, puis prend les plans par score par unité de coût jusqu'à épuiser le plafond. Les plans à inclure sont pris d'abord, quel que soit leur coût. Un plan sans score Smart est écarté ; un plan sans ligne budgétaire ne coûte rien. Le choix n'est pas toujours le meilleur possible : un plafond qui n'achète pas plus de valeur qu'un plafond moins cher est signalé, pas caché.",
  "p.frontier.must_include":
    "Plans que chaque choix doit inclure (jusqu'à {max})",
  "p.frontier.plans_loading": "Lecture des plans…",
  "p.frontier.plans_none": "Aucun plan que vous pouvez lire.",
  "p.frontier.frontier_heading": "Frontière efficiente",
  "p.frontier.frontier_says":
    "Le générateur est exécuté une fois par plafond, et rien n'est enregistré. Répartissez de {min} à {max} plafonds régulièrement de zéro au coût de tous les plans chiffrables (vide pour {fallback}), ou indiquez vous-même jusqu'à {list} plafonds, chacun un montant non négatif.",
  "p.frontier.f_currency": "Devise des coûts (trois lettres)",
  "p.frontier.f_how": "Quels plafonds budgétaires",
  "p.frontier.how_steps": "Répartis régulièrement",
  "p.frontier.how_caps": "Plafonds que j'indique",
  "p.frontier.f_steps": "Combien de plafonds (vide pour {fallback})",
  "p.frontier.f_caps": "Plafonds budgétaires, un par ligne (jusqu'à {list})",
  "p.frontier.run": "Tracer la frontière",
  "p.frontier.value_is":
    "La valeur est la somme des scores Smart des plans choisis.",
  "p.frontier.table": "La frontière, point par point",
  "p.frontier.col_cap": "Plafond budgétaire",
  "p.frontier.col_cost": "Coût du choix",
  "p.frontier.col_value": "Valeur",
  "p.frontier.col_included": "Plans choisis",
  "p.frontier.col_more": "Achète plus de valeur",
  "p.frontier.more_yes": "Oui",
  "p.frontier.more_no": "Non : un plafond moins cher atteint autant",
  "p.frontier.show_choice": "Montrer le choix",
  "p.frontier.show_choice_named": "Montrer le choix à {cap}",
  "p.frontier.save_point": "Enregistrer comme brouillon",
  "p.frontier.save_point_named": "Enregistrer le choix à {cap} comme brouillon",
  "p.frontier.point_name": "Frontière à {cap}",
  "p.frontier.axis_cost": "Coût des plans choisis",
  "p.frontier.axis_value": "Valeur (somme des scores Smart)",
  "p.frontier.chart_points_one": "{n} plafond budgétaire.",
  "p.frontier.chart_points_other": "{n} plafonds budgétaires.",
  "p.frontier.chart_best": "La plus grande valeur, {value}, coûte {cost}.",
  "p.frontier.chart_dominated_one":
    "{n} plafond n'achète pas plus de valeur qu'un moins cher.",
  "p.frontier.chart_dominated_other":
    "{n} plafonds n'achètent pas plus de valeur qu'un moins cher.",
  "p.frontier.legend_efficient": "achète plus de valeur",
  "p.frontier.legend_dominated":
    "n'achète pas plus de valeur qu'un plafond moins cher",
  "p.frontier.no_points": "La frontière n'a aucun point.",
  "p.frontier.generate_heading": "Générer un brouillon",
  "p.frontier.generate_says":
    "Enregistré comme scénario brouillon, à évaluer, comparer et engager comme les autres. Un nom est nécessaire, de {bytes} octets au plus. Avec une devise, chaque plan y est chiffré ; un plafond a besoin de sa devise.",
  "p.frontier.g_name": "Nom du brouillon",
  "p.frontier.g_cap": "Plafond budgétaire (facultatif)",
  "p.frontier.g_currency": "Devise (requise avec un plafond)",
  "p.frontier.generate": "Générer le brouillon",
  "p.frontier.draft_saved":
    "Brouillon {name} enregistré ; il figure dans la liste ci-dessus.",
  "p.frontier.choice_at": "Ce que choisit un plafond de {cap}",
  "p.frontier.choice_of": "Ce que {name} a choisi",
  "p.frontier.chosen_count_one": "{n} plan choisi sur {of}.",
  "p.frontier.chosen_count_other": "{n} plans choisis sur {of}.",
  "p.frontier.col_plan": "Plan du choix",
  "p.frontier.col_chosen": "Choisi ou non",
  "p.frontier.col_score": "Son score Smart",
  "p.frontier.col_plan_cost": "Son coût",
  "p.frontier.col_why": "Pourquoi écarté",
  "p.frontier.chosen": "choisi",
  "p.frontier.left_out": "écarté",
  "p.frontier.no_score": "pas de score Smart",
  "p.frontier.plan_unknown":
    "Un plan que vous ne pouvez pas lire, ou qui n'existe plus",
  "p.frontier.reason_no_score":
    "il n'a pas de score Smart, ce qui n'est pas un score de zéro",
  "p.frontier.reason_foreign_currency":
    "toutes ses lignes budgétaires sont dans une autre devise, il ne peut donc pas être chiffré",
  "p.frontier.reason_over_cap": "le plafond était épuisé avant lui",
  "p.frontier.reason_must_include_conflict":
    "il doit être inclus, mais ce n'est pas un plan que vous pouvez lire",
  "p.frontier.reason_other":
    "écarté pour une raison que cette page ne connaît pas",
  "p.frontier.problem_currency": "Indiquez la devise en trois lettres.",
  "p.frontier.problem_steps":
    "Le nombre de plafonds doit être un nombre entier de {min} à {max}.",
  "p.frontier.problem_caps_none": "Indiquez au moins un plafond.",
  "p.frontier.problem_caps_many": "Indiquez au plus {list} plafonds.",
  "p.frontier.problem_caps_amount":
    "Chaque plafond doit être un montant non négatif.",
  "p.frontier.problem_must_include":
    "Choisissez au plus {list} plans à inclure.",
  "p.frontier.problem_name": "Donnez un nom au brouillon.",
  "p.frontier.problem_name_long": "Le nom dépasse {bytes} octets.",
  "p.frontier.problem_cap_currency": "Un plafond a besoin de sa devise.",
  "p.frontier.problem_cap_amount":
    "Le plafond doit être un montant non négatif.",
  "p.frontier.refused_frontier": "La frontière a été refusée.",
  "p.frontier.refused_generate": "Le brouillon a été refusé.",
  "p.frontier.could_not_read": "Impossible de tracer la frontière.",
  "p.frontier.could_not_generate": "Impossible de générer le brouillon.",
  "p.frontier.rollback": "Annuler l'engagement",
  "p.frontier.rollback_named": "Annuler l'engagement de {name}",
  "p.frontier.rollback_says":
    "Un scénario engagé peut redevenir un brouillon, après une question ; cela ne change que le scénario.",
  "p.frontier.confirm_rollback":
    "Faire redevenir {name} un brouillon ? Il ne sera plus le portefeuille engagé ; l'engagement reste dans la piste d'audit.",
  "p.frontier.rolled_back": "{name} est de nouveau un brouillon.",
  "p.frontier.refused_rollback_status":
    "{name} n'est plus engagé : il n'y a rien à annuler. La liste a été relue.",
  "p.frontier.refused_rollback_gone":
    "{name} n'existe plus, ou vous ne pouvez pas le lire. La liste a été relue.",
  "p.frontier.could_not_roll_back":
    "Impossible d'annuler l'engagement du scénario.",
  "p.frontier.status_draft": "brouillon",
  "p.frontier.status_committed": "engagé",
  "p.frontier.violations_one": "{n} contrainte enfreinte",
  "p.frontier.violations_other": "{n} contraintes enfreintes",
  "p.frontier.compare_note":
    "B moins A, une devise à la fois ; les devises ne sont jamais additionnées.",
  "p.intake.edit": "Modifier",
  "p.intake.edit_named": "Modifier le brouillon {title}",
  "p.intake.edit_heading": "Modifier {title}",
  "p.intake.edit_heading_loading": "Lecture du brouillon…",
  "p.intake.edit_form": "Les champs du brouillon",
  "p.intake.edit_says":
    "Seul un brouillon peut être modifié, et c'est aussi là qu'une proposition est marquée comme demande externe. Un titre est nécessaire ; le titre et le résumé font au plus {bytes} octets chacun ; un montant a besoin de sa devise ; une contrepartie nommée fait de {min} à {max} caractères. Enregistrer remplace chaque champ.",
  "p.intake.save": "Enregistrer le brouillon",
  "p.intake.saved": "{title} enregistré.",
  "p.intake.unchanged": "Rien n'a changé ; rien n'a été envoyé.",
  "p.intake.could_not_read_draft": "Impossible de lire le brouillon.",
  "p.intake.problem_not_draft": "Seul un brouillon peut être modifié.",
  "p.intake.problem_title_long": "Le titre dépasse {bytes} octets.",
  "p.intake.problem_summary_long": "Le résumé dépasse {bytes} octets.",
  "p.intake.refused_not_draft":
    "La modification a été refusée : la proposition n'est plus un brouillon. La liste a été relue.",
  "p.intake.refused_edit":
    "La modification a été refusée : un champ sort de ce que le service accepte.",
  "p.intake.forecast_heading": "Combien seront approuvées",
  "p.intake.forecast_says":
    "Obtenue par simulation à partir du nombre de propositions approuvées à chaque période passée ; les propositions qui arrivent n'y entrent pas. Une prévision a besoin d'au moins {needed} périodes d'historique.",
  "p.intake.f_periods": "Périodes à venir (1 à {max} ; vide pour {fallback})",
  "p.intake.f_period_days":
    "Jours par période (1 à {max} ; vide pour {fallback})",
  "p.intake.f_history":
    "Périodes d'historique (1 à {max} ; vide pour {fallback})",
  "p.intake.forecast_run": "Prévoir",
  "p.intake.problem_periods":
    "Les périodes à venir doivent être un nombre entier de 1 à {periods}.",
  "p.intake.problem_period_days":
    "Une période doit être un nombre entier de jours de 1 à {days}.",
  "p.intake.problem_history":
    "L'historique doit être un nombre entier de périodes de 1 à {history}.",
  "p.intake.refused_forecast": "La prévision a été refusée.",
  "p.intake.could_not_forecast": "Impossible d'établir la prévision.",
  "p.intake.window":
    "Historique du {from} au {to}, par périodes de {days} jours.",
  "p.intake.at_least_label_one":
    "Dans la prochaine {n} période ({days} jours), au moins",
  "p.intake.at_least_label_other":
    "Dans les {n} prochaines périodes ({days} jours), au moins",
  "p.intake.at_least_means":
    "avec 85 % de confiance : le chiffre sur lequel planifier",
  "p.intake.median_label": "En général",
  "p.intake.median_means":
    "la moitié des essais approuve plus, l'autre moitié moins : un pile ou face, pas un engagement",
  "p.intake.at_most_label": "Au plus",
  "p.intake.at_most_means":
    "l'extrémité optimiste : 85 % des essais n'approuvent pas plus",
  "p.intake.proposals_n_one": "{n} proposition",
  "p.intake.proposals_n_other": "{n} propositions",
  "p.intake.absent_history":
    "Pas de prévision : il faut au moins {needed} périodes d'historique, et il y en a {n}.",
  "p.intake.absent_other":
    "Aucune prévision n'a pu être tirée de cet historique.",
  "p.intake.approvals_history":
    "Approuvées par période, de la plus ancienne à la plus récente : {list}",
  "p.intake.arrivals_history":
    "Propositions arrivées par période, de la plus ancienne à la plus récente (montrées à côté ; elles n'entrent pas dans la prévision) : {list}",
  "p.planner.heading": "Temps planifié",
  "p.planner.says":
    "Réservez du temps un jour pour l'une de vos tâches ouvertes. Un créneau n'appartient qu'à vous et ne change jamais l'échéance de la tâche ; il finit après avoir commencé et dure au plus 12 heures. Les créneaux peuvent se chevaucher : un chevauchement est signalé comme conflit, et un créneau qui finit quand le suivant commence n'en est pas un. Les heures sont dans votre fuseau horaire, {tz}.",
  "p.planner.day_nav": "Choisir le jour",
  "p.planner.previous_day": "Jour précédent",
  "p.planner.back_to_today": "Revenir à aujourd'hui",
  "p.planner.next_day": "Jour suivant",
  "p.planner.f_day": "Jour",
  "p.planner.show_day": "Afficher le jour",
  "p.planner.problem_day": "Indiquez le jour sous forme de date.",
  "p.planner.day_heading": "Planifié le {day}",
  "p.planner.day_hours":
    "Ce jour compte {hours} heures, car on change d'heure.",
  "p.planner.nothing_planned": "Rien n'est planifié ce jour-là.",
  "p.planner.clashes_one": "En conflit avec {n} autre créneau",
  "p.planner.clashes_other": "En conflit avec {n} autres créneaux",
  "p.planner.remove": "Retirer",
  "p.planner.remove_named": "Retirer le créneau de {title}, {time}",
  "p.planner.confirm_remove":
    "Retirer le créneau de {title}, {time} ? La tâche elle-même ne change pas.",
  "p.planner.removed": "Créneau de {title} retiré, {time}.",
  "p.planner.omitted_one":
    "{n} créneau sur une tâche que vous ne pouvez plus lire n'est pas affiché.",
  "p.planner.omitted_other":
    "{n} créneaux sur des tâches que vous ne pouvez plus lire ne sont pas affichés.",
  "p.planner.due_heading": "À échéance ce jour-là",
  "p.planner.nothing_due":
    "Rien de ce qui vous est attribué n'arrive à échéance ce jour-là.",
  "p.planner.loading": "Lecture du jour…",
  "p.planner.form": "Planifier du temps pour une tâche",
  "p.planner.form_says":
    "Le créneau est le {day}, dans votre fuseau horaire. Un créneau qui dépasse minuit se planifie en deux.",
  "p.planner.f_task": "Tâche",
  "p.planner.choose_task": "Choisissez l'une de vos tâches ouvertes",
  "p.planner.loading_tasks": "Lecture de vos tâches…",
  "p.planner.task_option": "{title}, dans {plan}",
  "p.planner.no_tasks":
    "Rien ne vous est attribué : il n'y a donc aucune tâche pour laquelle planifier du temps.",
  "p.planner.f_start": "Commence",
  "p.planner.f_end": "Finit",
  "p.planner.plan_time": "Planifier ce temps",
  "p.planner.planned": "{title} planifié, {time}.",
  "p.planner.problem_task": "Choisissez une tâche.",
  "p.planner.problem_start": "Indiquez l'heure de début.",
  "p.planner.problem_end": "Indiquez l'heure de fin.",
  "p.planner.problem_start_skipped":
    "Ce jour-là, l'heure de début n'existe pas à cause du changement d'heure ; choisissez-en une autre.",
  "p.planner.problem_end_skipped":
    "Ce jour-là, l'heure de fin n'existe pas à cause du changement d'heure ; choisissez-en une autre.",
  "p.planner.problem_order": "Un créneau doit finir après avoir commencé.",
  "p.planner.problem_too_long": "Un créneau dure au plus 12 heures.",
  "p.planner.refused_zone":
    "Le fuseau horaire de votre navigateur n'a pas été accepté : le jour ne peut pas être affiché.",
  "p.planner.refused_slot":
    "Le créneau n'a pas été accepté : il doit finir après avoir commencé et durer au plus 12 heures, et une personne détient au plus {max} créneaux.",
  "p.planner.refused_task":
    "Cette tâche n'existe plus, ou elle est dans un plan que vous ne pouvez pas lire.",
  "p.planner.refused_gone":
    "Ce créneau n'existe déjà plus. Le jour a été relu.",
  "p.planner.could_not_load": "Impossible de lire le jour.",
  "p.planner.could_not_plan": "Impossible de planifier le temps.",
  "p.planner.could_not_remove": "Impossible de retirer le créneau.",
  "p.feed.heading": "Flux de calendrier",
  "p.feed.says":
    "Une adresse privée à laquelle une application de calendrier peut s'abonner : les tâches ouvertes qui vous sont attribuées et ont une échéance, les jalons et sprints de leurs plans, et le temps que vous avez planifié. Une seconde adresse est un flux de vos notifications pour un lecteur de flux. Quiconque possède une adresse peut lire ce qu'elle montre : gardez-la pour vous.",
  "p.feed.rules":
    "Une adresse s'affiche une seule fois, ici, et cette page ne la conserve pas ; elle dure 365 jours. En créer une nouvelle arrête aussitôt toute adresse précédente.",
  "p.feed.issue": "Créer une adresse de flux",
  "p.feed.confirm_issue":
    "Créer une nouvelle adresse de flux ? Toute adresse créée auparavant cesse aussitôt de fonctionner.",
  "p.feed.issued":
    "Vos nouvelles adresses figurent ci-dessous. Copiez-les maintenant : elles ne seront plus affichées.",
  "p.feed.calendar_address": "Adresse du calendrier (iCalendar)",
  "p.feed.atom_address": "Adresse du flux de notifications (Atom)",
  "p.feed.revoke": "Arrêter toutes les adresses de flux",
  "p.feed.confirm_revoke":
    "Arrêter toutes les adresses de flux que vous avez créées ? Une application de calendrier qui en utilise une cesse aussitôt de se mettre à jour.",
  "p.feed.revoked":
    "Toutes les adresses de flux que vous avez créées ont cessé de fonctionner.",
  "p.feed.no_address":
    "La réponse ne contenait aucune adresse que cette page peut afficher : aucune n'est affichée. Toute adresse précédente est arrêtée ; créez-en une autre si besoin.",
  "p.feed.refused_forbidden": "Vous ne pouvez pas gérer d'adresse de flux.",
  "p.feed.could_not_issue": "Impossible de créer une adresse de flux.",
  "p.feed.could_not_revoke": "Impossible d'arrêter les adresses de flux.",
  "p.watching.heading": "Ce que vous suivez",
  "p.watching.says":
    "Vous êtes averti quand quelque chose que vous suivez change. Vous suivez une tâche quand vous la créez, la commentez ou qu'elle vous est confiée, sauf si vous l'avez désactivé dans vos choix de notification. Personne d'autre n'apprend qui suit. Un plan est nommé ; le reste est indiqué par sa nature et le début de sa référence.",
  "p.watching.count_one": "Vous suivez {n} élément.",
  "p.watching.count_other": "Vous suivez {n} éléments.",
  "p.watching.capped": "Seuls les {max} plus récents sont affichés.",
  "p.watching.none": "Vous ne suivez rien pour le moment.",
  "p.watching.named_plan": "Le plan {name}",
  "p.watching.unnamed": "{kind}, référence {ref}",
  "p.watching.kind_plan": "Un plan",
  "p.watching.kind_task": "Tâche",
  "p.watching.kind_risk": "Risque",
  "p.watching.kind_proposal": "Proposition",
  "p.watching.kind_idea": "Idée",
  "p.watching.kind_canvas_item": "Élément de canevas",
  "p.watching.kind_budget_line": "Ligne budgétaire",
  "p.watching.kind_scenario": "Scénario",
  "p.watching.kind_document_version": "Version de document",
  "p.watching.kind_other": "Autre chose",
  "p.watching.stop": "Ne plus suivre",
  "p.watching.stop_named": "Ne plus suivre {what}",
  "p.watching.confirm_stop":
    "Ne plus suivre {what} ? Vous ne serez plus averti de ses changements.",
  "p.watching.stopped": "Vous ne suivez plus {what}.",
  "p.watching.refused": "L'arrêt de ce suivi n'a pas été accepté.",
  "p.watching.loading": "Lecture de ce que vous suivez…",
  "p.watching.could_not_load": "Impossible de lire ce que vous suivez.",
  "p.watching.could_not_stop": "Impossible d'arrêter de le suivre.",
  "p.workprofile.heading": "Travail dans le temps",
  "p.workprofile.says":
    "Le travail porté par les tâches de chaque plan que vous pouvez lire, réparti sur les jours ouvrés de chaque tâche selon son profil, en minutes entières. Le travail impossible à placer (une tâche pas encore planifiée, ou sans jour ouvré) est compté à part, jamais écarté.",
  "p.workprofile.rules":
    "Les semaines vont du lundi au dimanche. Laissez les dates vides pour couvrir tout le travail placé. Une fenêtre couvre au plus dix ans et {max} périodes.",
  "p.workprofile.form": "Afficher le travail dans le temps",
  "p.workprofile.f_period": "Périodes",
  "p.workprofile.period_week": "Semaines",
  "p.workprofile.period_day": "Jours",
  "p.workprofile.period_month": "Mois",
  "p.workprofile.f_from": "Du (vide pour le premier jour placé)",
  "p.workprofile.f_to": "Au (vide pour le dernier jour placé)",
  "p.workprofile.show": "Afficher le travail",
  "p.workprofile.problem_from": "Indiquez le premier jour sous forme de date.",
  "p.workprofile.problem_to": "Indiquez le dernier jour sous forme de date.",
  "p.workprofile.problem_order":
    "Le premier jour ne doit pas être après le dernier.",
  "p.workprofile.problem_too_long": "Une fenêtre couvre au plus dix ans.",
  "p.workprofile.problem_periods":
    "Cette fenêtre compte plus de {max} périodes ; choisissez des périodes plus longues ou une fenêtre plus courte.",
  "p.workprofile.refused":
    "Cette fenêtre n'a pas été acceptée : si aucun travail n'est encore placé, indiquez les deux dates, et restez dans les périodes indiquées ci-dessus.",
  "p.workprofile.none":
    "Aucun plan que vous pouvez lire n'a de tâche portant du travail.",
  "p.workprofile.summary":
    "{range} : {work} de travail, dont {unplaced} non placé.",
  "p.workprofile.plans_table": "Travail par plan",
  "p.workprofile.periods_table": "Travail par période",
  "p.workprofile.col_plan": "Nom du plan",
  "p.workprofile.col_work": "Travail",
  "p.workprofile.col_unplaced": "Non placé",
  "p.workprofile.col_period": "Période",
  "p.workprofile.plan_unnamed": "Un plan sans nom",
  "p.workprofile.could_not_load":
    "Impossible de lire le travail dans le temps.",
  "p.timeentry.heading": "Saisir du temps",
  "p.timeentry.says":
    "Le temps est saisi à votre nom, en minutes entières, de 1 minute à {max} sur une date, dans un plan et, si vous le souhaitez, l'une de ses tâches. Une semaine approuvée n'accepte plus de temps. Une saisie ne peut être ni modifiée ni retirée une fois enregistrée.",
  "p.timeentry.open": "Ajouter du temps",
  "p.timeentry.form": "Saisir du temps dans un plan",
  "p.timeentry.f_plan": "Dans quel plan",
  "p.timeentry.choose_plan": "Choisissez un plan",
  "p.timeentry.loading": "Lecture des plans…",
  "p.timeentry.f_task": "Tâche (facultative)",
  "p.timeentry.no_task": "Aucune tâche : le plan lui-même",
  "p.timeentry.task_unnamed": "Une tâche qui n'est plus dans ce plan",
  "p.timeentry.f_date": "Jour",
  "p.timeentry.f_duration": "Temps passé",
  "p.timeentry.f_hours": "Heures",
  "p.timeentry.f_minutes": "Nombre de minutes",
  "p.timeentry.typed": "Soit {time}.",
  "p.timeentry.f_category": "Catégorie",
  "p.timeentry.category_unclassified": "Non classé",
  "p.timeentry.category_capex": "Dépense d'investissement (capex)",
  "p.timeentry.category_opex": "Dépense de fonctionnement (opex)",
  "p.timeentry.f_billable": "Facturable",
  "p.timeentry.f_note": "Remarque (facultative)",
  "p.timeentry.record": "Enregistrer ce temps",
  "p.timeentry.confirm":
    "Enregistrer {time} le {day} dans {plan} ? Une saisie ne peut être ni modifiée ni retirée une fois enregistrée.",
  "p.timeentry.recorded": "{time} enregistré le {day} dans {plan}.",
  "p.timeentry.problem_plan": "Choisissez un plan.",
  "p.timeentry.problem_date":
    "Indiquez la date à laquelle le temps a été passé.",
  "p.timeentry.problem_locked":
    "Cette semaine est approuvée et verrouillée : elle n'accepte plus de temps.",
  "p.timeentry.problem_duration":
    "Indiquez le temps en heures entières (0 à 24) et minutes (0 à 59), de 1 à {max} minutes au total.",
  "p.timeentry.refused_locked":
    "Cette semaine est approuvée et verrouillée : elle n'accepte plus de temps. Rien n'a été enregistré.",
  "p.timeentry.refused_invalid":
    "La saisie n'a pas été acceptée : son temps, ou sa tâche, n'est pas accepté par le plan. Rien n'a été enregistré.",
  "p.timeentry.refused_plan":
    "Ce plan n'existe plus, ou vous ne pouvez pas le lire.",
  "p.timeentry.refused_forbidden":
    "Vous ne pouvez pas saisir de temps dans ce plan.",
  "p.timeentry.could_not_load_plans": "Impossible de lire les plans.",
  "p.timeentry.could_not_load_plan":
    "Impossible de lire les tâches et le temps de ce plan.",
  "p.timeentry.could_not_record": "Impossible d'enregistrer le temps.",
  "p.timeentry.yours_heading": "Votre temps dans {plan}",
  "p.timeentry.yours_none": "Vous n'avez saisi aucun temps dans ce plan.",
  "p.timeentry.yours_table":
    "Vos saisies de temps dans {plan}, les plus récentes d'abord",
  "p.timeentry.col_date": "Jour",
  "p.timeentry.col_time": "Temps",
  "p.timeentry.col_task": "Tâche",
  "p.timeentry.col_category": "Catégorie",
  "p.timeentry.col_billable": "Facturable",
  "p.timeentry.col_note": "Remarque",
  "p.timeentry.billable_yes": "Facturable",
  "p.timeentry.billable_no": "Non facturable",
  "p.timereport.heading": "Rapport de temps",
  "p.timereport.says":
    "Le temps saisi dans les plans que vous pouvez lire, regroupé à votre choix, facturable et non facturable toujours séparés. Laissez les dates vides pour les 90 derniers jours jusqu'à aujourd'hui. Une personne est une référence person: ou worker: ; laissez vide pour tout le monde.",
  "p.timereport.form": "Demander un rapport de temps",
  "p.timereport.f_group_by": "Regrouper par",
  "p.timereport.by_plan": "Par plan",
  "p.timereport.by_person": "Par personne",
  "p.timereport.by_category": "Par catégorie",
  "p.timereport.by_billable": "Facturable ou non",
  "p.timereport.by_month": "Par mois",
  "p.timereport.f_from": "Du",
  "p.timereport.f_to": "Au",
  "p.timereport.f_plan": "Seulement ce plan",
  "p.timereport.every_plan": "Tous les plans que vous pouvez lire",
  "p.timereport.loading_plans": "Lecture des plans…",
  "p.timereport.f_person": "Personne (une référence)",
  "p.timereport.show": "Afficher le rapport de temps",
  "p.timereport.problem_from": "Indiquez le premier jour sous forme de date.",
  "p.timereport.problem_to": "Indiquez le dernier jour sous forme de date.",
  "p.timereport.problem_order":
    "Le premier jour ne doit pas être après le dernier.",
  "p.timereport.problem_person":
    "Une personne s'écrit person: ou worker: suivi d'un identifiant, par exemple person:6f1c2a3b-0000-4000-8000-000000000009.",
  "p.timereport.refused":
    "Le rapport de temps n'a pas été accepté pour ces choix.",
  "p.timereport.window": "Temps saisi {range}.",
  "p.timereport.no_entries":
    "Aucun temps n'a été saisi dans ces plans sur cette période.",
  "p.timereport.table": "Temps par groupe",
  "p.timereport.col_entries": "Saisies",
  "p.timereport.col_billable": "Facturable",
  "p.timereport.col_not_billable": "Non facturable",
  "p.timereport.col_total": "Au total",
  "p.timereport.total":
    "Au total : {billable} facturable, {not_billable} non facturable, {total} ensemble.",
  "p.timereport.omitted_one":
    "{n} saisie dans un plan que vous ne pouvez pas lire est laissée de côté.",
  "p.timereport.omitted_other":
    "{n} saisies dans des plans que vous ne pouvez pas lire sont laissées de côté.",
  "p.timereport.truncated":
    "Seules les {max} premières saisies ont été lues : le rapport est incomplet.",
  "p.timereport.csv": "Télécharger ce rapport de temps en CSV",
  "p.timereport.billable": "Facturable",
  "p.timereport.not_billable": "Non facturable",
  "p.timereport.plan_unnamed": "Un plan non nommé ici ({ref})",
  "p.timereport.could_not_load": "Impossible de lire le rapport de temps.",
  "p.timereport.could_not_load_plans":
    "Impossible de lire les plans à choisir.",
  "p.hammock.heading":
    "Tâches hamac : des tâches de synthèse qui en couvrent d'autres",
  "p.hammock.says":
    "Une tâche hamac est une synthèse dont les dates viennent des tâches de ce plan qu'elle relie, du début le plus tôt à la fin la plus tardive. Elle n'est pas dans le réseau des dépendances, donc elle ne change jamais la marge d'une tâche ni la fin. Une tâche ne devient hamac que si elle n'a ni durée, ni travail, ni contrainte, ni dépendance, et son début prévu est effacé ; la redevenir tâche retire tous ses liens. Un hamac relie une tâche ou un autre hamac de ce plan, pas lui-même, pas deux fois et pas en boucle.",
  "p.hammock.none": "Ce plan n'a pas de tâches hamac.",
  "p.hammock.spans": "Couvre {range}.",
  "p.hammock.no_links": "Pas encore de dates : il ne relie aucune tâche.",
  "p.hammock.members_unscheduled":
    "Pas encore de dates : aucune des tâches qu'il relie n'a de dates.",
  "p.hammock.untitled": "Une tâche sans titre",
  "p.hammock.member_hammock": "{title} (un hamac)",
  "p.hammock.unmake_named": "Refaire de {title} une tâche",
  "p.hammock.unlink": "Retirer le lien",
  "p.hammock.unlink_named": "Retirer le lien de {hammock} vers {title}",
  "p.hammock.make_form": "Faire d'une tâche un hamac",
  "p.hammock.f_task": "Tâche",
  "p.hammock.choose_task": "Choisissez une tâche",
  "p.hammock.make": "En faire un hamac",
  "p.hammock.link_form": "Relier un hamac à une tâche",
  "p.hammock.f_hammock": "Hamac",
  "p.hammock.choose_hammock": "Choisissez un hamac",
  "p.hammock.f_member": "Tâche couverte",
  "p.hammock.choose_member": "Choisissez une tâche à couvrir",
  "p.hammock.link": "Ajouter le lien",
  "p.hammock.confirm_make":
    "Faire de {title} un hamac ? Son début prévu est effacé, et ses dates viennent ensuite des tâches qu'il relie.",
  "p.hammock.confirm_unmake_one":
    "Refaire de {title} une tâche ? Son {n} lien disparaît.",
  "p.hammock.confirm_unmake_other":
    "Refaire de {title} une tâche ? Ses {n} liens disparaissent.",
  "p.hammock.confirm_unlink":
    "Retirer le lien de {hammock} vers {title} ? La tâche elle-même ne change pas.",
  "p.hammock.made": "{title} est désormais un hamac.",
  "p.hammock.unmade": "{title} est de nouveau une tâche.",
  "p.hammock.linked": "{hammock} couvre désormais {title}.",
  "p.hammock.unlinked": "{hammock} ne couvre plus {title}.",
  "p.hammock.problem_choose_task":
    "Choisissez une tâche à transformer en hamac.",
  "p.hammock.problem_carries_work":
    "Cette tâche a une durée, du travail ou une contrainte : effacez-les d'abord dans le planning.",
  "p.hammock.problem_has_dependencies":
    "Cette tâche a des dépendances : retirez-les d'abord, car un hamac n'en prend aucune.",
  "p.hammock.problem_choose_hammock": "Choisissez un hamac.",
  "p.hammock.problem_choose_member":
    "Choisissez une tâche que le hamac couvrira.",
  "p.hammock.problem_self": "Un hamac ne peut pas se couvrir lui-même.",
  "p.hammock.problem_linked": "Ce hamac couvre déjà cette tâche.",
  "p.hammock.problem_loop":
    "Ce hamac-là couvre déjà celui-ci, le lien ferait donc une boucle.",
  "p.hammock.refused_make":
    "Le service n'a pas fait de cette tâche un hamac : elle a encore une durée, du travail, une contrainte ou une dépendance.",
  "p.hammock.refused_link":
    "Le service n'a pas ajouté le lien : la tâche n'est peut-être plus dans ce plan, déjà couverte, ou dans une boucle.",
  "p.hammock.refused_gone":
    "La tâche ou le hamac n'existe plus, ou vous ne pouvez plus le lire.",
  "p.hammock.refused_link_gone": "Ce lien n'existe déjà plus.",
  "p.hammock.could_not_load": "Impossible de lire les tâches hamac.",
  "p.hammock.could_not_save": "Impossible de modifier le hamac.",
  "p.crossplan.heading": "Tâches des plans liés",
  "p.crossplan.says":
    "Voici les tâches d'autres plans que vous pouvez lire et auxquelles des tâches de ce plan sont liées, planifiées selon le calendrier de leur propre plan. Une barre en pointillés est une tâche hors du chemin critique ; une barre pleine et un losange en marquent une dessus.",
  "p.crossplan.network_finish":
    "Avec les plans liés, le réseau se termine le {date}.",
  "p.crossplan.truncated":
    "Tous les plans liés ne sont pas affichés : le réseau a atteint sa limite de plans ou de tâches.",
  "p.crossplan.links_heading": "Liens qui passent dans un autre plan",
  "p.crossplan.lag": "décalage de {days} jours ouvrés",
  "p.crossplan.timeline": "Tâches de {plan} sur une frise",
  "p.crossplan.table": "Dates des tâches des plans liés",
  "p.crossplan.col_task": "Tâche",
  "p.crossplan.col_plan": "Projet",
  "p.crossplan.col_start": "Début au plus tôt",
  "p.crossplan.col_finish": "Fin au plus tôt",
  "p.crossplan.col_critical": "Sur le chemin critique",
  "p.crossplan.yes": "Oui",
  "p.crossplan.no": "Non",
  "p.crossplan.untitled": "Une tâche sans titre",
  "p.crossplan.kind_fs": "fin à début",
  "p.crossplan.kind_ss": "début à début",
  "p.crossplan.kind_ff": "fin à fin",
  "p.crossplan.kind_sf": "début à fin",
  "p.crossplan.task_heading": "Dépendances, dans ce plan ou un autre",
  "p.crossplan.task_says":
    "Une dépendance relie cette tâche à une tâche d'un autre plan que vous pouvez lire. Son type est fin à début sauf autre choix ; son décalage est un nombre entier de jours ouvrés de moins {max} à {max}, un décalage négatif étant une avance. Une dépendance n'est pas créée deux fois dans le même sens, ni en boucle entre plans, ni avec un hamac. Une dont vous ne pouvez pas lire l'autre extrémité est comptée, jamais nommée, et reste hors du planning.",
  "p.crossplan.no_links": "Cette tâche n'a pas encore de dépendances.",
  "p.crossplan.this_before": "Cette tâche vient avant {other}",
  "p.crossplan.this_after": "Cette tâche vient après {other}",
  "p.crossplan.end_own": "{title}, dans ce plan",
  "p.crossplan.end_external": "{title}, dans {plan}",
  "p.crossplan.end_unnamed": "une tâche non nommée ici ({ref})",
  "p.crossplan.remove": "Retirer",
  "p.crossplan.remove_named": "Retirer la dépendance envers {other}",
  "p.crossplan.restricted_one":
    "{n} autre dépendance relie à une tâche que vous ne pouvez pas lire.",
  "p.crossplan.restricted_other":
    "{n} autres dépendances relient à des tâches que vous ne pouvez pas lire.",
  "p.crossplan.see_network": "Voir le planning entre les plans",
  "p.crossplan.form": "Ajouter une dépendance envers une tâche d'un autre plan",
  "p.crossplan.f_order": "Ordre",
  "p.crossplan.order_before": "Cette tâche vient d'abord",
  "p.crossplan.order_after": "L'autre tâche vient d'abord",
  "p.crossplan.f_plan": "Autre plan",
  "p.crossplan.choose_plan": "Choisissez un plan",
  "p.crossplan.f_task": "Tâche de ce plan",
  "p.crossplan.choose_task": "Choisissez une tâche",
  "p.crossplan.f_kind": "Type de dépendance",
  "p.crossplan.f_lag": "Décalage en jours ouvrés (0 si vide)",
  "p.crossplan.add": "Ajouter la dépendance",
  "p.crossplan.added": "Dépendance envers {title} ajoutée.",
  "p.crossplan.removed": "Dépendance retirée.",
  "p.crossplan.confirm_remove_before":
    "Retirer la dépendance : {task} vient avant {other} ? Aucune des deux tâches ne change.",
  "p.crossplan.confirm_remove_after":
    "Retirer la dépendance : {task} vient après {other} ? Aucune des deux tâches ne change.",
  "p.crossplan.problem_plan": "Choisissez l'autre plan.",
  "p.crossplan.problem_task": "Choisissez une tâche de ce plan.",
  "p.crossplan.problem_lag":
    "Un décalage est un nombre entier de jours ouvrés de moins {max} à {max}.",
  "p.crossplan.problem_self": "Une tâche ne peut pas dépendre d'elle-même.",
  "p.crossplan.problem_exists": "Cette dépendance existe déjà.",
  "p.crossplan.refused_unreadable":
    "L'une des deux tâches n'existe plus, ou vous ne pouvez pas la lire.",
  "p.crossplan.refused_exists": "Cette dépendance existe déjà.",
  "p.crossplan.refused_add":
    "Le service n'a pas ajouté la dépendance : elle existe peut-être déjà, ferait une boucle ou relie un hamac.",
  "p.crossplan.refused_gone":
    "Cette dépendance n'existe déjà plus, ou vous ne pouvez plus lire les deux tâches.",
  "p.crossplan.could_not_load":
    "Impossible de lire les dépendances de cette tâche.",
  "p.crossplan.could_not_load_plans":
    "Impossible de lire les plans parmi lesquels choisir.",
  "p.crossplan.could_not_load_tasks":
    "Impossible de lire les tâches de ce plan.",
  "p.crossplan.could_not_save": "Impossible de modifier la dépendance.",
  "p.taskrollup.heading": "Sous-tâches agrégées",
  "p.taskrollup.says":
    "Calculé à chaque fois à partir des sous-tâches de cette tâche, jamais stocké : l'échéance la plus tardive parmi elles, leurs points additionnés, l'avancement pondéré par les seules sous-tâches estimées, et le temps saisi sur l'ensemble.",
  "p.taskrollup.subtasks": "Sous-tâches",
  "p.taskrollup.done_of_one": "{done} sur {n} terminée",
  "p.taskrollup.done_of_other": "{done} sur {n} terminées",
  "p.taskrollup.due": "Échéance la plus tardive",
  "p.taskrollup.points": "Points additionnés",
  "p.taskrollup.progress": "Avancement",
  "p.taskrollup.spent": "Temps saisi",
  "p.taskrollup.no_children": "Cette tâche n'a pas de sous-tâches.",
  "p.taskrollup.no_dated_children": "Aucune de ses sous-tâches n'a d'échéance.",
  "p.taskrollup.no_estimates": "Aucune de ses sous-tâches n'est estimée.",
  "p.taskrollup.absent": "Inconnu.",
  "p.taskrollup.unestimated_one":
    "{n} sous-tâche n'a pas d'estimation, elle reste donc hors de l'avancement (pas comptée comme zéro) :",
  "p.taskrollup.unestimated_other":
    "{n} sous-tâches n'ont pas d'estimation, elles restent donc hors de l'avancement (pas comptées comme zéro) :",
  "p.taskrollup.unnamed": "Une sous-tâche non nommée ici ({ref})",
  "p.taskrollup.refused_gone":
    "Cette tâche n'existe plus, ou vous ne pouvez plus la lire.",
  "p.taskrollup.could_not_load": "Impossible de lire les sous-tâches agrégées.",
  "p.readings.heading": "Plus sur ce plan",
  "p.readings.says":
    "Chacune de ces lectures n'est faite que lorsque vous la demandez.",
  "p.readings.read": "Lire",
  "p.readings.could_not_read": "Impossible de lire ceci.",
  "p.readings.not_measured": "non mesuré",
  "p.readings.days": "{n} jours",
  "p.readings.rollup_heading": "Flux dans les plans qu'il contient",
  "p.readings.rollup_says":
    "Les tâches de ce plan et de chaque plan en dessous, comptées ensemble et non moyennées ; jusqu'à {max} niveaux si laissé vide.",
  "p.readings.f_depth": "Niveaux en dessous",
  "p.readings.rollup_plans_one": "{n} plan compté.",
  "p.readings.rollup_plans_other": "{n} plans comptés.",
  "p.readings.rollup_tasks_one":
    "{n} tâche : {finished} terminées, {wip} en cours, {not_started} non commencées.",
  "p.readings.rollup_tasks_other":
    "{n} tâches : {finished} terminées, {wip} en cours, {not_started} non commencées.",
  "p.readings.rollup_times":
    "La plupart du travail (85 sur 100) s'est terminé en {cycle} après son début et en {lead} après la demande ; temps passé à travailler plutôt qu'à attendre : {efficiency}.",
  "p.readings.rollup_truncated":
    "Tous les plans en dessous n'ont pas été comptés : le parcours a atteint sa limite.",
  "p.readings.rollup_revisits_one":
    "{n} plan est atteint deux fois et compté une fois.",
  "p.readings.rollup_revisits_other":
    "{n} plans sont atteints deux fois et comptés une fois.",
  "p.readings.rollup_table": "Flux par plan",
  "p.readings.col_plan": "Projet",
  "p.readings.col_depth": "Niveau",
  "p.readings.col_tasks": "Tâches",
  "p.readings.col_finished": "Terminées",
  "p.readings.col_wip": "En cours",
  "p.readings.col_efficiency": "Temps à travailler plutôt qu'à attendre",
  "p.readings.effort_heading": "Temps saisi",
  "p.readings.effort_says":
    "Le temps que les personnes ont saisi sur ce plan, tel qu'elles l'ont déclaré ; rien ne le vérifie.",
  "p.readings.read_effort": "Lire le temps saisi",
  "p.readings.effort_none": "Aucun temps n'est saisi sur ce plan.",
  "p.readings.effort_total": "{total} saisi, dont {billable} facturable.",
  "p.readings.effort_split":
    "Dépenses d'investissement {capex}, dépenses de fonctionnement {opex}, non classé {unclassified}.",
  "p.readings.effort_by_person": "Temps par personne",
  "p.readings.effort_by_task": "Temps par tâche",
  "p.readings.col_person": "Personne",
  "p.readings.col_time": "Temps",
  "p.readings.col_billable": "Facturable",
  "p.readings.col_task": "Tâche",
  "p.readings.task_unnamed": "Une tâche non nommée ici ({ref})",
  "p.readings.forecast_heading": "Prévision de livraison",
  "p.readings.forecast_says":
    "Dix mille tirages à partir de ce que ce plan a terminé à chaque période passée : combien de temps le travail ouvert peut prendre, et combien peut être fait en un nombre de périodes. Une case vide est la valeur par défaut ; il faut au moins six périodes d'historique.",
  "p.readings.f_items": "Éléments à terminer (les ouverts si vide)",
  "p.readings.f_periods": "Périodes à venir ({n} si vide)",
  "p.readings.f_period_days": "Jours par période ({n} si vide)",
  "p.readings.f_history": "Périodes d'historique ({n} si vide)",
  "p.readings.read_forecast": "Lire la prévision",
  "p.readings.forecast_history_one": "Tiré de {n} période de {days} jours.",
  "p.readings.forecast_history_other": "Tiré de {n} périodes de {days} jours.",
  "p.readings.forecast_batch_one":
    "{n} élément : la moitié des tirages se terminent en {p50} jours, 85 sur 100 en {p85} jours et 95 sur 100 en {p95} jours.",
  "p.readings.forecast_batch_other":
    "{n} éléments : la moitié des tirages se terminent en {p50} jours, 85 sur 100 en {p85} jours et 95 sur 100 en {p95} jours.",
  "p.readings.forecast_no_batch":
    "La durée du travail ne peut être prévue : l'historique est trop court, ou rien n'y a été terminé.",
  "p.readings.forecast_horizon":
    "Dans les {days} prochains jours, environ {median} éléments peuvent être faits : au moins {least} dans 85 tirages sur 100, et au plus {most}.",
  "p.readings.forecast_no_horizon":
    "La quantité réalisable ne peut être prévue : l'historique est trop court, ou rien n'y a été terminé.",
  "p.readings.performance_heading": "Délais, coûts et parties prenantes",
  "p.readings.performance_says":
    "La performance des délais est le travail fait par rapport au travail prévu jusqu'ici, et celle des coûts le travail fait par rapport à ce qu'il a coûté ; au-dessus de 100 % est mieux que prévu. Le score des parties prenantes va de moins 100 à 100.",
  "p.readings.read_performance": "Lire la performance",
  "p.readings.schedule": "Performance des délais",
  "p.readings.cost": "Performance des coûts",
  "p.readings.stakeholders": "Score des parties prenantes",
  "p.readings.nps_one":
    "{score}, sur {n} réponse ({promoters} promoteurs, {detractors} détracteurs)",
  "p.readings.nps_other":
    "{score}, sur {n} réponses ({promoters} promoteurs, {detractors} détracteurs)",
  "p.readings.nps_none": "Pas encore de réponses.",
  "p.readings.earned_value_link": "Voir la valeur acquise en entier",
  "p.readings.absent_no_baseline":
    "Non mesuré : le plan n'a pas de référence budgétaire.",
  "p.readings.absent_no_weighted_tasks":
    "Non mesuré : aucune tâche n'a de poids.",
  "p.readings.absent_not_yet_defined":
    "Pas encore mesuré : rien n'a été prévu ni dépensé jusqu'ici.",
  "p.readings.absent_several_currencies":
    "Pas de chiffre unique : le plan a des budgets en plusieurs devises ; voir la valeur acquise.",
  "p.readings.absent_other": "Non mesuré.",
  "p.readings.workflow_heading": "Flux de travail des tâches",
  "p.readings.read_workflow": "Lire le flux de travail des tâches",
  "p.readings.workflow_plan":
    "Ce plan a son propre flux de travail des tâches.",
  "p.readings.workflow_built_in":
    "Ce plan utilise le flux de travail des tâches intégré.",
  "p.readings.workflow_constrained": "Seuls les passages listés sont permis.",
  "p.readings.workflow_free":
    "Une tâche peut passer de n'importe quel état à n'importe quel autre.",
  "p.readings.cat_todo": "non commencé",
  "p.readings.cat_active": "en cours",
  "p.readings.cat_waiting": "en attente",
  "p.readings.cat_done": "terminé",
  "p.readings.cat_other": "un autre type",
  "p.readings.initial": "où une tâche commence",
  "p.readings.terminal": "où une tâche finit",
  "p.readings.wip_limit": "au plus {n} à la fois",
  "p.readings.anyone": "tout le monde",
  "p.readings.roles": "rôles : {roles}",
  "p.readings.problem_depth":
    "Les niveaux en dessous sont un nombre entier de 1 à {max}.",
  "p.readings.problem_depth_zero":
    "Les niveaux en dessous sont un nombre entier de 0 à {max}.",
  "p.readings.problem_items":
    "Les éléments à terminer sont un nombre entier d'au moins 1.",
  "p.readings.problem_periods":
    "Les périodes à venir sont un nombre entier de 1 à 520.",
  "p.readings.problem_period_days":
    "Les jours par période sont un nombre entier de 1 à 90.",
  "p.readings.problem_history":
    "Les périodes d'historique sont un nombre entier de 6 à 260.",
  "p.readings.problem_window":
    "La fenêtre est un nombre entier de jours de 1 à 3650.",
  "p.readings.refused_query": "Le service n'a pas accepté cette demande.",
  "p.readings.refused_module": "Cette partie du plan est désactivée.",
  "p.readings.refused_gone":
    "Ce plan n'existe plus, ou vous ne pouvez plus le lire.",
  "p.readings.flow_feature": "Nouvelles fonctions",
  "p.readings.flow_defect": "Défauts",
  "p.readings.flow_risk": "Travail sur les risques",
  "p.readings.flow_debt": "Dette technique",
  "p.readings.flow_unclassified": "Non classé",
  "p.readings.flow_other": "Un autre type",
  "p.flowmix.heading": "La répartition du travail terminé",
  "p.flowmix.says":
    "Comment le travail terminé dans une fenêtre se répartit entre nouvelles fonctions, défauts, travail sur les risques et dette technique, par rapport à la répartition visée par le plan s'il en a déclaré une. La fenêtre est de {n} jours si laissée vide.",
  "p.flowmix.f_window": "Jours en arrière ({n} si vide)",
  "p.flowmix.f_rollup": "Inclure les plans qu'il contient",
  "p.flowmix.f_depth": "Niveaux en dessous ({max} si vide)",
  "p.flowmix.read": "Lire la répartition",
  "p.flowmix.window_one": "Travail terminé le dernier {n} jour.",
  "p.flowmix.window_other": "Travail terminé les {n} derniers jours.",
  "p.flowmix.plans_one": "{n} plan compté.",
  "p.flowmix.plans_other": "{n} plans comptés.",
  "p.flowmix.truncated":
    "Tous les plans en dessous n'ont pas été comptés : le parcours a atteint sa limite.",
  "p.flowmix.none": "Aucun travail n'a été terminé dans cette fenêtre.",
  "p.flowmix.no_intent":
    "Le plan ne déclare pas de répartition visée, il n'y a donc rien à comparer.",
  "p.flowmix.table": "Travail terminé par type",
  "p.flowmix.col_kind": "Type de travail",
  "p.flowmix.col_count": "Terminé",
  "p.flowmix.col_share": "Part",
  "p.flowmix.col_intended": "Part visée",
  "p.flowmix.col_gap": "Écart",
  "p.ccpm.heading": "Estimations agressive et sûre",
  "p.ccpm.says":
    "Pour la chaîne critique, chaque tâche a besoin d'une estimation agressive (environ une chance sur deux) et d'une sûre, en jours ouvrés entiers de 1 à {max}, l'agressive pas au-dessus de la sûre. Un chiffre affiché est celui saisi, sinon celui de la plage de risque, sinon celui que dérive la règle de l'installation, et chacun dit lequel. Un jalon et une tâche sans durée ne sont pas estimés. Vider les deux retire ce qui est saisi.",
  "p.ccpm.rule_explicit_only":
    "Règle : seules les estimations données servent ; rien n'est dérivé.",
  "p.ccpm.rule_half_of_safe":
    "Règle : une estimation sûre manquante est la durée de la tâche, et une agressive manquante la moitié de la sûre.",
  "p.ccpm.rule_other": "Règle : une que cette page ne connaît pas.",
  "p.ccpm.rule_unusable":
    "Le réglage de la règle de l'installation est illisible, seules les estimations données servent donc.",
  "p.ccpm.attention_one": "{n} tâche a besoin de ses estimations.",
  "p.ccpm.attention_other": "{n} tâches ont besoin de leurs estimations.",
  "p.ccpm.more": "Les {shown} premières tâches sur {total} sont affichées.",
  "p.ccpm.no_tasks": "Ce plan n'a pas de tâches à estimer.",
  "p.ccpm.table": "Estimations par tâche",
  "p.ccpm.col_task": "Tâche",
  "p.ccpm.col_likely": "Durée en jours ouvrés",
  "p.ccpm.col_aggressive": "Agressive",
  "p.ccpm.col_safe": "Sûre",
  "p.ccpm.col_state": "Situation",
  "p.ccpm.figure": "{days} jours, {source}",
  "p.ccpm.none": "aucune",
  "p.ccpm.source_recorded": "saisie",
  "p.ccpm.source_risk_range": "de la plage de risque",
  "p.ccpm.source_rule": "par la règle",
  "p.ccpm.state_complete": "Les deux données",
  "p.ccpm.state_derived": "En partie dérivée",
  "p.ccpm.state_missing": "Une estimation manque",
  "p.ccpm.state_inconsistent": "Agressive au-dessus de la sûre",
  "p.ccpm.state_milestone": "Un jalon, non estimé",
  "p.ccpm.state_no_duration": "Sans durée, non estimée",
  "p.ccpm.state_other": "Une autre situation",
  "p.ccpm.form": "Saisir les estimations d'une tâche",
  "p.ccpm.f_task": "Tâche",
  "p.ccpm.choose_task": "Choisissez une tâche",
  "p.ccpm.f_aggressive": "Agressive, en jours ouvrés",
  "p.ccpm.f_safe": "Sûre, en jours ouvrés",
  "p.ccpm.save": "Enregistrer les estimations",
  "p.ccpm.unchanged": "Rien n'a changé, donc rien n'a été envoyé.",
  "p.ccpm.saved": "Estimations de {title} enregistrées.",
  "p.ccpm.cleared": "Estimations saisies pour {title} vidées.",
  "p.ccpm.confirm_clear":
    "Vider les estimations saisies pour {title} ? Un chiffre de la plage de risque ou de la règle peut encore s'afficher.",
  "p.ccpm.problem_task": "Choisissez une tâche.",
  "p.ccpm.problem_aggressive":
    "L'estimation agressive est un nombre entier de jours ouvrés de 1 à {max}.",
  "p.ccpm.problem_safe":
    "L'estimation sûre est un nombre entier de jours ouvrés de 1 à {max}.",
  "p.ccpm.problem_order":
    "L'estimation agressive ne peut pas dépasser la sûre.",
  "p.ccpm.refused_invalid":
    "Le service n'a pas accepté ces estimations : la tâche a peut-être perdu sa durée ou est devenue un jalon.",
  "p.ccpm.refused_gone":
    "Cette tâche n'existe plus, ou vous ne pouvez plus la lire.",
  "p.ccpm.could_not_load": "Impossible de lire les estimations.",
  "p.ccpm.could_not_save": "Impossible d'enregistrer les estimations.",
  "p.frozen.heading": "Le plan de tampons figé",
  "p.frozen.none": "Aucun plan de tampons n'a encore été figé.",
  "p.frozen.freeze_link": "En figer un dans la gestion des tampons",
  "p.frozen.taken": "Figé le {date} ; {rule}",
  "p.frozen.table": "Tampons tels que figés",
  "p.frozen.col_buffer": "Tampon",
  "p.frozen.col_days": "Jours ouvrés",
  "p.frozen.col_chain": "Chaîne",
  "p.frozen.col_finish": "Fin prévue",
  "p.frozen.project": "Tampon du projet",
  "p.frozen.feeding": "Tampon d'alimentation vers {joins}",
  "p.frozen.unsized": "non dimensionné",
  "p.frozen.untitled": "Une tâche sans titre",
  "p.frozen.history_one": "{n} plan de tampons figé : {dates}.",
  "p.frozen.history_other":
    "{n} plans de tampons figés, le plus récent d'abord : {dates}.",
  "p.frozen.could_not_load": "Impossible de lire le plan de tampons figé.",
  "p.workingtime.heading": "La journée et la semaine de travail",
  "p.workingtime.says":
    "La durée d'une journée de travail (1 minute à 24 heures) et le nombre de jours travaillés par semaine (1 à 7), pour ce plan ou comme valeur par défaut de l'organisation. Chacune est un nouvel enregistrement qui ne peut être ni modifié ni retiré ici, la question est donc posée d'abord ; une seconde pour ce plan est refusée.",
  "p.workingtime.form": "Fixer la journée et la semaine de travail",
  "p.workingtime.f_scope": "Pour",
  "p.workingtime.scope_plan": "Ce plan",
  "p.workingtime.scope_default": "La valeur par défaut de l'organisation",
  "p.workingtime.f_hours": "Heures par jour",
  "p.workingtime.f_minutes": "Et minutes",
  "p.workingtime.f_days": "Jours par semaine",
  "p.workingtime.save": "Fixer la journée de travail",
  "p.workingtime.confirm_plan_one":
    "Fixer la journée de travail de ce plan à {day}, {days} jour par semaine ? Elle ne pourra plus être modifiée ici.",
  "p.workingtime.confirm_plan_other":
    "Fixer la journée de travail de ce plan à {day}, {days} jours par semaine ? Elle ne pourra plus être modifiée ici.",
  "p.workingtime.confirm_default_one":
    "Fixer la journée de travail par défaut de l'organisation à {day}, {days} jour par semaine ? Elle ne pourra plus être modifiée ici.",
  "p.workingtime.confirm_default_other":
    "Fixer la journée de travail par défaut de l'organisation à {day}, {days} jours par semaine ? Elle ne pourra plus être modifiée ici.",
  "p.workingtime.saved_one":
    "Journée de travail de {day} fixée, {days} jour par semaine.",
  "p.workingtime.saved_other":
    "Journée de travail de {day} fixée, {days} jours par semaine.",
  "p.workingtime.away_heading": "Absences du travail du projet",
  "p.workingtime.away_says":
    "Un congé, un jour férié, un congé de formation ou une mission hors du projet, pour une personne d'un premier à un dernier jour, afin que ce temps ne soit pas lu comme inactif. La personne est une référence comme person: et un identifiant. Une fois saisie, l'absence ne peut être ni modifiée ni retirée, la question est donc posée d'abord ; une note vide n'est pas envoyée.",
  "p.workingtime.f_person": "Référence de la personne",
  "p.workingtime.f_starts": "Premier jour",
  "p.workingtime.f_ends": "Dernier jour",
  "p.workingtime.f_kind": "Type",
  "p.workingtime.f_note": "Note (facultative)",
  "p.workingtime.record": "Saisir l'absence",
  "p.workingtime.confirm_away":
    "Saisir {kind} pour {person}, {range} ? Elle ne pourra plus être modifiée ni retirée.",
  "p.workingtime.recorded": "{kind} saisi pour {person}, {range}.",
  "p.workingtime.kind_leave": "un congé",
  "p.workingtime.kind_holiday": "un jour férié",
  "p.workingtime.kind_study_leave": "un congé de formation",
  "p.workingtime.kind_non_project_duty": "une mission hors du projet",
  "p.workingtime.problem_hours": "Les heures sont un nombre entier de 0 à 24.",
  "p.workingtime.problem_minutes":
    "Les minutes sont un nombre entier de 0 à 59.",
  "p.workingtime.problem_day":
    "Une journée de travail va de 1 minute à 24 heures.",
  "p.workingtime.problem_week":
    "Les jours par semaine sont un nombre entier de 1 à 7.",
  "p.workingtime.problem_person":
    "Indiquez la personne sous la forme person: ou worker: et un identifiant.",
  "p.workingtime.problem_starts": "Indiquez le premier jour.",
  "p.workingtime.problem_ends": "Indiquez le dernier jour.",
  "p.workingtime.problem_order":
    "Le dernier jour ne peut pas précéder le premier.",
  "p.workingtime.refused_exists":
    "Ce plan a déjà une journée de travail fixée, et elle ne peut pas être remplacée ici.",
  "p.workingtime.refused_working":
    "Le service n'a pas accepté cette journée de travail.",
  "p.workingtime.refused_non_working":
    "Le service n'a pas accepté cette absence.",
  "p.workingtime.refused_forbidden": "Vous ne pouvez pas saisir ceci.",
  "p.workingtime.could_not_save": "Impossible de saisir ceci.",
  "p.task.is_hammock":
    "Cette tâche est un hamac : ses dates viennent des tâches qu'elle couvre.",
  "p.task.hammock_link": "Voir son étendue sur le chemin critique",
  "p.adoption.could_not": "Impossible d'enregistrer l'adoption.",
  "p.adoption.f_active": "Utilisateurs actifs",
  "p.adoption.f_definition": "Ce qui comptait comme utilisateur actif",
  "p.adoption.f_target": "Utilisateurs visés",
  "p.adoption.f_window": "Sur combien de jours",
  "p.adoption.form": "Enregistrer l'adoption",
  "p.adoption.heading": "Combien de personnes l'utilisent",
  "p.adoption.problem_active":
    "Indiquez les utilisateurs actifs en nombre entier, 0 ou plus.",
  "p.adoption.problem_definition":
    "Dites ce qui comptait comme utilisateur actif, pour comparer les relevés suivants.",
  "p.adoption.problem_target":
    "Indiquez les utilisateurs visés en nombre entier supérieur à 0.",
  "p.adoption.problem_window":
    "Indiquez la période en nombre entier de jours, 1 ou plus.",
  "p.adoption.record": "Enregistrer cette adoption",
  "p.adoption.refused": "Le service n'a pas accepté ces chiffres.",
  "p.adoption.refused_forbidden":
    "Vous ne pouvez pas enregistrer l'adoption de ce plan.",
  "p.adoption.refused_gone": "Ce plan n'existe plus.",
  "p.adoption.saved": "Adoption enregistrée.",
  "p.adoption.says":
    "Utilisateurs actifs rapportés aux utilisateurs visés, sur une période de jours. Ce qui comptait comme actif est gardé à côté du chiffre, pour comparer un relevé ultérieur.",
  "p.bulk.apply": "Appliquer aux tâches choisies",
  "p.bulk.choose_all": "Choisir toutes les tâches",
  "p.bulk.choose_none": "N'en choisir aucune",
  "p.bulk.choose_plan": "Choisissez un plan",
  "p.bulk.chosen_one": "{n} tâche choisie",
  "p.bulk.chosen_other": "{n} tâches choisies",
  "p.bulk.confirm_delete_one":
    "Supprimer {n} tâche ? Elle quitte le tableau et toutes les vues du plan.",
  "p.bulk.confirm_delete_other":
    "Supprimer {n} tâches ? Elles quittent le tableau et toutes les vues du plan.",
  "p.bulk.confirm_move_one":
    "Déplacer {n} tâche vers {plan} ? Son sprint, son composant et sa tâche parente sont effacés, et un statut que ce plan n'a pas devient son premier.",
  "p.bulk.confirm_move_other":
    "Déplacer {n} tâches vers {plan} ? Leur sprint, leur composant et leur tâche parente sont effacés, et un statut que ce plan n'a pas devient son premier.",
  "p.bulk.copy": "Copier vers le plan",
  "p.bulk.could_not": "Impossible de modifier les tâches.",
  "p.bulk.could_not_load_plans": "Impossible de lire les plans.",
  "p.bulk.delete": "Supprimer les tâches choisies",
  "p.bulk.delete_failed": "Non supprimées : {titles}.",
  "p.bulk.deleted_gone_one": "{n} tâche n'existait déjà plus.",
  "p.bulk.deleted_gone_other": "{n} tâches n'existaient déjà plus.",
  "p.bulk.deleted_one": "{n} tâche supprimée.",
  "p.bulk.deleted_other": "{n} tâches supprimées.",
  "p.bulk.done_one": "Fait pour {done} tâche sur {requested}.",
  "p.bulk.done_other": "Fait pour {done} tâches sur {requested}.",
  "p.bulk.edit_form": "Définir des champs sur les tâches choisies",
  "p.bulk.f_assignee": "Responsable",
  "p.bulk.f_due": "Date d'échéance",
  "p.bulk.f_plan": "Plan de destination",
  "p.bulk.f_start": "Date de début",
  "p.bulk.f_value": "Nouvelle valeur : {field}",
  "p.bulk.heading": "Modifier plusieurs tâches à la fois",
  "p.bulk.keep_assignee": "Garder le responsable lors de la copie",
  "p.bulk.mapped_one":
    "{n} tâche avait un statut que ce plan n'a pas : elle commence dans son premier état.",
  "p.bulk.mapped_other":
    "{n} tâches avaient un statut que ce plan n'a pas : elles commencent dans son premier état.",
  "p.bulk.mode_clear": "L'effacer",
  "p.bulk.mode_keep": "Le laisser tel quel",
  "p.bulk.mode_set": "Le définir à",
  "p.bulk.move": "Déplacer vers le plan",
  "p.bulk.no_tasks": "Ce plan n'a aucune tâche.",
  "p.bulk.problem_assignee":
    "Indiquez le responsable sous la forme person: ou worker: suivi d'un id.",
  "p.bulk.problem_due": "Indiquez une date d'échéance valide.",
  "p.bulk.problem_no_field":
    "Choisissez au moins un champ à définir ou à effacer.",
  "p.bulk.problem_none": "Choisissez au moins une tâche.",
  "p.bulk.problem_plan": "Choisissez le plan vers lequel copier ou déplacer.",
  "p.bulk.problem_same_plan":
    "Les tâches sont déjà dans ce plan : choisissez-en un autre où les déplacer.",
  "p.bulk.problem_start":
    "Indiquez une date de début valide entre {min} et {latest}.",
  "p.bulk.problem_too_many": "Choisissez au plus {max} tâches à la fois.",
  "p.bulk.refused_edit": "Le service n'a pas accepté ces valeurs.",
  "p.bulk.refused_forbidden": "Vous ne pouvez pas modifier ces tâches.",
  "p.bulk.refused_plan_gone":
    "Ce plan n'existe plus ou vous ne pouvez pas le voir.",
  "p.bulk.refused_titles":
    "Refusées, pour une raison que le service ne donne qu'en anglais : {titles}.",
  "p.bulk.refused_transfer":
    "Vous ne pouvez pas voir ce plan, ou le service n'a pas accepté la liste.",
  "p.bulk.says":
    "Choisissez des tâches, puis donnez-leur à toutes le même responsable, la même échéance ou le même début, copiez-les ou déplacez-les vers un plan, ou supprimez-les. Au plus {max} à la fois ; le début va du {min} au {latest}. Le statut se change sur le tableau, carte par carte. Une copie commence dans le premier état de ce plan et ne garde le responsable que sur demande ; un déplacement efface le sprint, le composant et la tâche parente, et une tâche avec des sous-tâches n'est pas déplacée.",
  "p.bulk.this_plan": "{name} (ce plan)",
  "p.bulk.transfer_form": "Copier ou déplacer les tâches choisies",
  "p.bulk.unchanged":
    "Les tâches choisies ont déjà ces valeurs : rien n'a été envoyé.",
  "p.ceremony.choose_kind": "Choisissez une cérémonie",
  "p.ceremony.could_not_load": "Impossible de lire les cérémonies du sprint.",
  "p.ceremony.could_not_save": "Impossible d'enregistrer la cérémonie.",
  "p.ceremony.counts": "Cérémonies tenues",
  "p.ceremony.f_facilitator":
    "Animateur (person: ou worker: suivi d'un id, facultatif)",
  "p.ceremony.f_kind": "Cérémonie",
  "p.ceremony.f_summary": "Ce qui a été décidé (facultatif)",
  "p.ceremony.facilitated_by": "animée par {ref}",
  "p.ceremony.form": "Enregistrer une cérémonie",
  "p.ceremony.heading": "Cérémonies de {name}",
  "p.ceremony.held_count_one": "{kind} : tenue {n} fois",
  "p.ceremony.held_count_other": "{kind} : tenue {n} fois",
  "p.ceremony.held_saved": "Enregistrée : {kind}.",
  "p.ceremony.kind_daily": "Point quotidien",
  "p.ceremony.kind_other": "Autre cérémonie",
  "p.ceremony.kind_planning": "Planification",
  "p.ceremony.kind_retrospective": "Rétrospective",
  "p.ceremony.kind_review": "Revue",
  "p.ceremony.none_held": "{kind} : pas tenue",
  "p.ceremony.problem_facilitator":
    "Indiquez l'animateur sous la forme person: ou worker: suivi d'un id, ou laissez vide.",
  "p.ceremony.problem_kind": "Choisissez la cérémonie.",
  "p.ceremony.problem_once":
    "Ce sprint a déjà tenu sa {kind} : une seconde est une replanification, donc un nouveau sprint.",
  "p.ceremony.record": "Enregistrer cette cérémonie",
  "p.ceremony.refused_ceremony":
    "Le service l'a refusé : ce sprint a peut-être déjà tenu cette cérémonie.",
  "p.ceremony.refused_forbidden":
    "Vous ne pouvez pas enregistrer cela pour le sprint.",
  "p.ceremony.refused_gone": "Ce sprint n'existe plus.",
  "p.ceremony.says":
    "Un sprint tient une planification et une revue, et autant de points quotidiens et de rétrospectives que voulu. Chaque type est compté, même s'il n'a pas eu lieu.",
  "p.checkins.heading": "Points d'étape de {title}",
  "p.checkins.none": "Aucun point d'étape pour l'instant.",
  "p.checkins.row": "{date} : valeur {value}",
  "p.commitment.added_one": "{n} tâche ajoutée depuis : {titles}.",
  "p.commitment.added_other": "{n} tâches ajoutées depuis : {titles}.",
  "p.commitment.commit": "Engager les tâches du sprint",
  "p.commitment.confirm_one":
    "Engager maintenant la {n} tâche de {name} ? C'est fait une fois et ne peut être changé ; les tâches ajoutées ou retirées ensuite apparaissent comme un changement.",
  "p.commitment.confirm_other":
    "Engager maintenant les {n} tâches de {name} ? C'est fait une fois et ne peut être changé ; les tâches ajoutées ou retirées ensuite apparaissent comme un changement.",
  "p.commitment.could_not": "Impossible d'engager le sprint.",
  "p.commitment.heading": "Ce à quoi le sprint s'est engagé",
  "p.commitment.not_yet_one":
    "Pas encore engagé ; {n} tâche est dans le sprint.",
  "p.commitment.not_yet_other":
    "Pas encore engagé ; {n} tâches sont dans le sprint.",
  "p.commitment.problem_done":
    "Ce sprint est déjà engagé : un engagement n'est jamais réécrit.",
  "p.commitment.problem_empty": "Ce sprint n'a aucune tâche à engager.",
  "p.commitment.problem_unread":
    "L'engagement du sprint n'a pas encore été lu.",
  "p.commitment.refused_commit":
    "Le service l'a refusé : ce sprint est déjà engagé.",
  "p.commitment.removed_one": "{n} tâche retirée depuis : {titles}.",
  "p.commitment.removed_other": "{n} tâches retirées depuis : {titles}.",
  "p.commitment.saved_one": "{n} tâche engagée.",
  "p.commitment.saved_other": "{n} tâches engagées.",
  "p.commitment.says":
    "Les tâches du sprint telles qu'elles sont à l'engagement, prises une fois, afin qu'une tâche ajoutée ou retirée ensuite se lise comme un changement et non comme le plan d'origine.",
  "p.commitment.state":
    "{committed} engagées ; {current} dans le sprint maintenant.",
  "p.commitment.unchanged": "Rien d'ajouté ni de retiré depuis.",
  "p.hours.col_actual": "Saisies",
  "p.hours.col_placeholder": "Dont non attribuées",
  "p.hours.col_plan": "Plan du projet",
  "p.hours.col_planned": "Prévues",
  "p.hours.col_variance": "Saisies moins prévues",
  "p.hours.could_not": "Impossible de lire les heures.",
  "p.hours.f_from": "Du",
  "p.hours.f_to": "Au",
  "p.hours.form": "Heures prévues face aux heures saisies",
  "p.hours.fte_says":
    "Les équivalents temps plein sont des jours-pour-cent divisés par 100 fois les jours de la période ; un écart est l'offre moins la demande (sous zéro, il manque). Les réservations proposées sont une série à part.",
  "p.hours.h": "{h} h",
  "p.hours.heading": "Heures prévues et saisies",
  "p.hours.masked":
    "Les personnes ne sont pas affichées : vous ne pouvez pas les gérer.",
  "p.hours.none":
    "Aucun plan n'avait d'heures prévues ou saisies sur cette période.",
  "p.hours.nothing_recorded": "rien de saisi",
  "p.hours.problem_from":
    "Indiquez le premier jour par une date valide, ou laissez vide.",
  "p.hours.problem_order": "Le dernier jour ne peut précéder le premier.",
  "p.hours.problem_span": "La période peut compter au plus {max} jours.",
  "p.hours.problem_to":
    "Indiquez le dernier jour par une date valide, ou laissez vide.",
  "p.hours.read": "Lire les heures",
  "p.hours.refused": "Le service n'a pas accepté cette période.",
  "p.hours.says":
    "Le prévu est la part de chaque jour ouvré d'une réservation ferme et nommée ; le saisi est le temps entré, approuvé ou non. Laissez les jours vides pour les quatre semaines jusqu'à aujourd'hui ; au plus {max} jours.",
  "p.hours.table": "Heures par plan",
  "p.hours.totals": "Prévues {planned} ; saisies {actual}.",
  "p.hours.truncated":
    "Seules les premières saisies ont été lues ; les totaux peuvent être bas.",
  "p.hours.unplanned": "{plan} : {hours} saisies",
  "p.hours.unplanned_heading":
    "Temps saisi par des personnes sans réservation sur le plan",
  "p.hours.unplanned_people_one": "({n} personne)",
  "p.hours.unplanned_people_other": "({n} personnes)",
  "p.hours.window": "Du {from} au {to}.",
  "p.kpireadings.col_standing": "Situation",
  "p.kpireadings.counted": "comptée",
  "p.kpireadings.replaced": "remplacée par un relevé ultérieur ce jour-là",
  "p.kpireadings.says":
    "Tous les relevés, du plus récent au plus ancien. Un relevé ultérieur le même jour remplace le précédent, qui est conservé.",
  "p.live.burndown_says":
    "Tâches restantes chaque jour, d'après les achèvements enregistrés ; aucune ligne idéale, rien n'est complété.",
  "p.live.event_comment_added": "un commentaire ajouté",
  "p.live.event_other": "un changement",
  "p.live.event_raci_updated": "qui fait quoi a changé",
  "p.live.event_resync": "le tableau relu en entier",
  "p.live.event_task_created": "une tâche ajoutée",
  "p.live.event_task_moved": "une tâche déplacée",
  "p.live.event_task_updated": "une tâche modifiée",
  "p.live.follow": "Suivre les changements en direct",
  "p.live.heading": "Changements en temps réel",
  "p.live.last_change": "Dernier : {change} à {time}.",
  "p.live.says":
    "Pendant le suivi, le tableau est relu chaque fois que quelqu'un ajoute, déplace ou modifie une tâche ici. Il s'arrête quand vous quittez la page.",
  "p.live.state_connecting": "Connexion…",
  "p.live.state_off": "Pas de suivi.",
  "p.live.state_on": "Suivi actif.",
  "p.live.state_retrying": "Connexion perdue ; nouvel essai.",
  "p.live.state_stopped": "Arrêté : le service n'envoie pas les changements.",
  "p.live.stop": "Arrêter le suivi",
  "p.live.velocity_says":
    "Tâches et points terminés à chaque sprint, comptés depuis le tableau ; ce n'est pas une mesure de flux.",
  "p.placeholder.choose": "Choisissez un espace réservé",
  "p.placeholder.confirm_over":
    "Le donner à {person} dépasserait sa capacité certains jours. Le faire quand même ?",
  "p.placeholder.confirm_remove":
    "Retirer cet espace réservé : {what} ? La demande qu'il représentait part avec lui.",
  "p.placeholder.could_not": "Impossible de modifier l'espace réservé.",
  "p.placeholder.f_person": "Personne (person: ou worker: suivi d'un id)",
  "p.placeholder.f_placeholder": "Espace réservé",
  "p.placeholder.form": "Attribuer un espace réservé à une personne",
  "p.placeholder.give": "L'attribuer à la personne",
  "p.placeholder.given": "Attribué à {person}.",
  "p.placeholder.heading":
    "Attribuer un espace réservé à une personne, ou le retirer",
  "p.placeholder.not_given": "Non attribué : rien n'a changé.",
  "p.placeholder.open": "ouvert",
  "p.placeholder.option": "{role} sur {plan}, {percent}, du {from} au {to}",
  "p.placeholder.problem_choose": "Choisissez un espace réservé.",
  "p.placeholder.problem_person":
    "Indiquez la personne sous la forme person: ou worker: suivi d'un id.",
  "p.placeholder.refused_forbidden":
    "Seul un administrateur ou un gestionnaire des ressources peut le faire.",
  "p.placeholder.refused_gone": "Cette réservation n'existe plus.",
  "p.placeholder.refused_named":
    "Cette réservation nomme déjà une personne ; la liste a été relue.",
  "p.placeholder.refused_person": "Le service n'a pas accepté cette personne.",
  "p.placeholder.remove": "Retirer l'espace réservé",
  "p.placeholder.removed": "Espace réservé retiré.",
  "p.placeholder.says":
    "Un espace réservé est un rôle demandé sans personne encore. L'attribuer relève d'un gestionnaire des ressources ; si cela dépasse la capacité, on vous le demande d'abord.",
  "p.planokr.as_of": "calculé le {at}",
  "p.planokr.choose_plan": "Choisissez un plan",
  "p.planokr.could_not": "Impossible de lire les objectifs du plan.",
  "p.planokr.could_not_load_plans": "Impossible de lire les plans.",
  "p.planokr.currencies": "résultats clés dans plusieurs devises",
  "p.planokr.f_plan": "Plan à évaluer",
  "p.planokr.form": "Score d'objectifs d'un plan",
  "p.planokr.heading": "Score d'objectifs d'un plan",
  "p.planokr.its_key_results": "Résultats clés de {title}",
  "p.planokr.key_results_one": "{n} résultat clé",
  "p.planokr.key_results_other": "{n} résultats clés",
  "p.planokr.left_out_one":
    "{n} objectif n'est pas mesuré et reste hors du score.",
  "p.planokr.left_out_other":
    "{n} objectifs ne sont pas mesurés et restent hors du score.",
  "p.planokr.no_objectives": "Aucun objectif n'est aligné sur ce plan.",
  "p.planokr.not_measured":
    "Non mesuré : aucun objectif aligné ne peut encore être mesuré.",
  "p.planokr.objective_not_measured": "non mesuré",
  "p.planokr.problem_plan": "Choisissez un plan.",
  "p.planokr.read": "Lire le score",
  "p.planokr.refused_gone": "Ce plan ou cet objectif n'existe plus.",
  "p.planokr.says":
    "Le score de chaque objectif aligné sur le plan, pondéré par la force de l'alignement. Un objectif que rien ne mesure est laissé de côté, jamais compté à 0 %.",
  "p.planokr.score": "Note {score}",
  "p.planokr.unmeasurable_no_range":
    "pas de progression : départ et cible identiques",
  "p.planokr.unmeasurable_no_tolerance":
    "pas de progression : à maintenir sans tolérance",
  "p.planokr.unmeasurable_other": "aucune progression calculable",
  "p.planokr.unmeasurable_overflow":
    "pas de progression : chiffres trop grands",
  "p.planokr.unnamed": "Un objectif ({id})",
  "p.planokr.weight": "poids {weight}",
  "p.rate.answer_person":
    "{rate} de l'heure le {date}, d'après la table propre à la personne, en vigueur depuis le {from}.",
  "p.rate.answer_role":
    "{rate} de l'heure le {date}, d'après la table de son rôle, en vigueur depuis le {from}.",
  "p.rate.could_not": "Impossible de lire le taux.",
  "p.rate.f_date": "Le",
  "p.rate.f_person": "Personne",
  "p.rate.f_table": "Table des taux",
  "p.rate.form": "Le taux de coût en vigueur",
  "p.rate.heading": "Le taux de coût en vigueur",
  "p.rate.none": "Aucun taux n'est en vigueur pour cette personne le {date}.",
  "p.rate.problem_date": "Indiquez le jour par une date valide.",
  "p.rate.problem_person": "Indiquez la référence de la personne.",
  "p.rate.read": "Lire le taux",
  "p.rate.refused_hidden":
    "Aucun taux visible pour vous : les taux ne sont montrés qu'aux finances.",
  "p.rate.refused_query": "Le service n'a pas accepté cette demande.",
  "p.rate.says":
    "Le taux horaire en vigueur pour une personne un jour donné : celui de sa propre table, sinon celui de son rôle. La table standard sauf si vous en nommez une autre.",
  "p.skillgap.choose_plan": "Choisissez un plan",
  "p.skillgap.col_person": "Personne",
  "p.skillgap.col_skill": "Compétence",
  "p.skillgap.col_status": "Situation",
  "p.skillgap.could_not": "Impossible de lire les écarts de compétences.",
  "p.skillgap.count_covered_one": "{n} compétence acquise ;",
  "p.skillgap.count_covered_other": "{n} compétences acquises ;",
  "p.skillgap.count_missing_one": "{n} compétence manquante ;",
  "p.skillgap.count_missing_other": "{n} compétences manquantes ;",
  "p.skillgap.count_unknown_one": "{n} inconnue.",
  "p.skillgap.count_unknown_other": "{n} inconnues.",
  "p.skillgap.f_plan": "Plan à examiner",
  "p.skillgap.form": "Écarts de compétences d'un plan",
  "p.skillgap.heading": "Compétences demandées par les réservations",
  "p.skillgap.none": "Aucune réservation de ce plan ne demande de compétence.",
  "p.skillgap.problem_plan": "Choisissez un plan.",
  "p.skillgap.read": "Lire les écarts",
  "p.skillgap.says":
    "Chaque compétence demandée par une réservation du plan, face à ce que la personne détient dans le service des effectifs. Inconnue signifie que ses compétences n'ont pu être lues ; un espace réservé n'a pas encore de personne et est exclu.",
  "p.skillgap.status_covered": "acquise",
  "p.skillgap.status_missing": "manquante",
  "p.skillgap.status_unknown": "inconnue",
  "p.skillgap.table": "Compétences par réservation",
  "p.util.by_person": "Par personne",
  "p.util.by_team": "L'équipe",
  "p.util.col_available": "Disponible",
  "p.util.col_away": "Absent",
  "p.util.col_effort": "Effort fourni",
  "p.util.col_figure": "Utilisation de la capacité",
  "p.util.col_person": "Personne",
  "p.util.could_not": "Impossible de lire l'utilisation de la capacité.",
  "p.util.every_plan": "Tous les plans",
  "p.util.f_by": "Afficher",
  "p.util.f_plan": "Seulement le plan",
  "p.util.f_window": "Jours en arrière depuis aujourd'hui",
  "p.util.form": "Effort face à la capacité",
  "p.util.heading": "Effort face à la capacité",
  "p.util.none": "Personne n'a saisi d'effort ni d'absence sur cette période.",
  "p.util.over":
    "à ou au-delà de la capacité : un avertissement, pas un exploit",
  "p.util.people_counted_one": "{n} personne comptée.",
  "p.util.people_counted_other": "{n} personnes comptées.",
  "p.util.problem_window":
    "Indiquez la période en nombre entier de jours de 1 à {max}.",
  "p.util.read": "Lire l'utilisation de la capacité",
  "p.util.refused": "Le service n'a pas accepté cette demande.",
  "p.util.says":
    "Effort saisi rapporté à la capacité déclarée, moins les absences, sur les {days} jours jusqu'à aujourd'hui sauf si vous choisissez de 1 à {max}. À ou au-delà de la capacité, c'est un avertissement sur la file. Les petits groupes et petits chiffres sont masqués.",
  "p.util.suppression":
    "Masqués : un groupe de moins de {people} personnes, ou un chiffre sous {hours}.",
  "p.util.table": "Utilisation de la capacité par personne",
  "p.util.team_detail":
    "{effort} sur {available} disponibles, {away} d'absence",
  "p.util.unavailable_all_away": "absent toute la période, pas 0 %",
  "p.util.unavailable_below_floor": "trop peu pour être indiqué",
  "p.util.unavailable_no_capacity": "aucune capacité déclarée",
  "p.util.unavailable_other": "non communicable",
  "p.util.unavailable_small_group": "trop peu de personnes pour afficher",
  "p.util.window_one": "Sur le dernier {n} jour.",
  "p.util.window_other": "Sur les {n} derniers jours.",
  "p.jobs.title": "Imports et exports",
  "p.jobs.title_ppm": "Imports et exports — PPM",
  "p.jobs.intro":
    "Imports et exports en masse de plans, les plus récents d'abord. Une tâche s'exécute en arrière-plan : relisez la liste pour la voir avancer.",
  "p.jobs.loading": "Chargement…",
  "p.jobs.could_not_load": "Impossible de charger les tâches.",
  "p.jobs.none": "Aucune tâche ne vous est visible.",
  "p.jobs.refresh": "Relire",
  "p.jobs.filter_kind": "Type",
  "p.jobs.filter_status": "État",
  "p.jobs.any": "tous",
  "p.jobs.kind_import": "importation",
  "p.jobs.kind_export": "exportation",
  "p.jobs.fmt_jsonl": "lignes JSON",
  "p.jobs.status_queued": "en file d'attente",
  "p.jobs.status_running": "en cours",
  "p.jobs.status_completed": "terminée",
  "p.jobs.status_completed_with_errors": "terminée avec des erreurs",
  "p.jobs.status_failed": "échouée",
  "p.jobs.status_cancelled": "annulée",
  "p.jobs.status_unknown": "état inconnu",
  "p.jobs.started": "Commencée {at}",
  "p.jobs.not_started": "Pas encore commencée",
  "p.jobs.finished": "terminée {at}",
  "p.jobs.progress": "{percent} effectué",
  "p.jobs.rows":
    "{total} lignes : {created} créées, {updated} mises à jour, {unchanged} inchangées, {review} retenues pour examen, {failed} en échec.",
  "p.jobs.not_reconciled": "Ces nombres ne font pas le total.",
  "p.jobs.held":
    "Les lignes retenues pour examen figurent plus bas sous « Retenues pour examen ».",
  "p.jobs.stopping":
    "Arrêt demandé : elle termine la ligne en cours et garde ce qui a été traité.",
  "p.jobs.errors": "Rapport d'erreurs (CSV)",
  "p.jobs.download": "Télécharger l'export",
  "p.jobs.cancel": "Annuler",
  "p.jobs.cancel_named": "Annuler la tâche d'{kind}",
  "p.jobs.cancel_ask":
    "Annuler cette tâche ? Une tâche en attente ne démarre pas ; une tâche en cours termine la ligne en cours et garde ce qui a été traité.",
  "p.jobs.nothing_to_cancel":
    "Rien à annuler : la tâche était déjà terminée ou son arrêt déjà demandé.",
  "p.jobs.could_not_cancel": "Impossible d'annuler la tâche.",
  "p.jobs.gone": "Cette tâche n'existe plus.",
  "p.jobs.export_h": "Exporter des plans",
  "p.jobs.export_rules":
    "Seulement les plans que vous pouvez lire, au plus 100 000. Les références aux personnes sont masquées sauf si vous demandez un export complet, qui exige le droit destructif.",
  "p.jobs.format": "Format du fichier",
  "p.jobs.full": "Export complet, avec les références aux personnes",
  "p.jobs.kind_filter": "Type de plan (facultatif)",
  "p.jobs.name_filter": "Le nom contient (facultatif)",
  "p.jobs.start_export": "Lancer l'export",
  "p.jobs.export_forbidden": "Un export complet exige le droit destructif.",
  "p.jobs.export_refused":
    "Le service a refusé l'export : resserrez le filtre (au plus 100 000 plans).",
  "p.jobs.could_not_export": "Impossible de lancer l'export.",
  "p.jobs.import_h": "Importer des plans",
  "p.jobs.import_rules":
    "Un plan par ligne (lignes JSON) ou par rangée (CSV), au plus 100 000 lignes ; l'installation limite la taille du fichier. Renvoyer le même fichier ne lance rien de nouveau.",
  "p.jobs.file": "Fichier",
  "p.jobs.keyless":
    "Une ligne sans identifiant qui ressemble à un plan existant",
  "p.jobs.keyless_review": "est retenue pour examen",
  "p.jobs.keyless_create": "est créée quand même",
  "p.jobs.start_import": "Lancer l'import",
  "p.jobs.no_file": "Choisissez d'abord un fichier.",
  "p.jobs.too_big": "Le fichier dépasse la taille maximale de l'installation.",
  "p.jobs.import_refused":
    "Le service a refusé le fichier : il est vide, trop grand ou pas au format choisi.",
  "p.jobs.could_not_import": "Impossible de lancer l'import.",
  "p.jobs.queued_ok": "Lancée : la tâche est en haut de la liste.",
  "p.jobs.replayed":
    "Ce fichier a déjà été envoyé ; sa tâche est dans la liste.",
  "p.jobs.review_h": "Retenues pour examen",
  "p.jobs.review_intro":
    "Les lignes qui ressemblaient à un plan déjà présent sont retenues, pas créées. Décidez pour chacune : la créer comme nouveau plan ou l'écarter comme doublon. La fusion avec le plan auquel elle ressemble n'est pas proposée ici : écartez-la, puis fusionnez les plans depuis la liste des plans.",
  "p.jobs.review_none": "Rien n'est retenu pour examen.",
  "p.jobs.could_not_load_review":
    "Impossible de charger les lignes retenues pour examen.",
  "p.jobs.review_line": "Ligne {line} du fichier",
  "p.jobs.review_matches": "Elle ressemble à :",
  "p.jobs.review_match": "{score} de ressemblance, confiance {confidence}",
  "p.jobs.confidence_high": "élevée",
  "p.jobs.confidence_medium": "moyenne",
  "p.jobs.confidence_low": "faible",
  "p.jobs.confidence_unknown": "inconnue",
  "p.jobs.review_unnamed": "(sans nom)",
  "p.jobs.review_create": "Elle est nouvelle : la créer",
  "p.jobs.review_discard": "C'est un doublon : l'écarter",
  "p.jobs.review_create_ask": "Créer « {name} » comme nouveau plan ?",
  "p.jobs.review_discard_ask":
    "Écarter « {name} » ? Rien n'a été créé à partir d'elle.",
  "p.jobs.review_created": "« {name} » créé.",
  "p.jobs.review_discarded": "« {name} » écarté.",
  "p.jobs.review_already": "Cette ligne a déjà été tranchée.",
  "p.jobs.review_invalid":
    "La ligne retenue n'est plus un plan valide, elle ne peut donc pas être créée.",
  "p.jobs.review_gone": "Cette ligne n'existe plus.",
  "p.jobs.could_not_decide": "Impossible d'enregistrer la décision.",
  "p.jobs.review_show": "Afficher ses lignes retenues",
  "p.jobs.review_all": "Afficher toutes les lignes retenues",
  "p.jobs.review_one_job": "Lignes retenues par un seul import.",
  "p.jobs.review_more": "Seules les 500 premières sont listées.",
  "p.plan_export.record_heading": "Le dossier du plan (JSON)",
  "p.plan_export.record_intro":
    "Tout ce qui est enregistré sur le plan, en JSON, pour une demande de dossier ou d'accès. Là où votre accès masque les personnes, qui le dirige et qui le possède sont masqués, et le fichier le dit. Chaque export de ce type est inscrit dans la piste d'audit du plan.",
  "p.plan_export.record_download": "Télécharger le dossier du plan",
  "p.template_sync.link": "Modifications du modèle",
  "p.template_sync.title": "Modifications du modèle",
  "p.template_sync.title_ppm": "Modifications du modèle — PPM",
  "p.template_sync.loading": "Chargement…",
  "p.template_sync.could_not_load":
    "Impossible de charger les modifications du modèle.",
  "p.template_sync.not_from_template":
    "Ce plan n'a pas été créé à partir d'un modèle enregistré ; il n'y a rien à mettre à jour.",
  "p.template_sync.made_from": "Créé à partir du modèle « {name} ».",
  "p.template_sync.versions":
    "Mis à jour avec la version {synced} ; le modèle en est à la version {version}.",
  "p.template_sync.intro":
    "Chaque modification ci-dessous vient du modèle depuis la dernière mise à jour de ce plan. Cochez celles à reprendre ; les autres restent en attente. Une modification d'une tâche que les personnes de ce plan ont elles-mêmes modifiée ou supprimée n'est appliquée que si vous cochez aussi « Écraser la modification de ce plan ».",
  "p.template_sync.up_to_date":
    "À jour : aucune modification que ce plan n'a pas reprise.",
  "p.template_sync.kind_add": "ajoutée",
  "p.template_sync.kind_change": "modifiée",
  "p.template_sync.kind_remove": "supprimée",
  "p.template_sync.kind_unknown": "modification inconnue",
  "p.template_sync.take": "Reprendre cette modification",
  "p.template_sync.edited":
    "Les personnes de ce plan ont modifié ou supprimé cette tâche.",
  "p.template_sync.overwrite": "Écraser la modification de ce plan",
  "p.template_sync.diff_title": "Titre : {from} → {to}",
  "p.template_sync.diff_description": "Description : {from} → {to}",
  "p.template_sync.diff_points": "Points : {from} → {to}",
  "p.template_sync.none_value": "aucun",
  "p.template_sync.apply": "Appliquer les modifications choisies",
  "p.template_sync.apply_ask":
    "Appliquer les modifications choisies à ce plan ? Les tâches sont ajoutées, modifiées ou supprimées comme indiqué, et le changement est consigné.",
  "p.template_sync.choose_first": "Cochez d'abord au moins une modification.",
  "p.template_sync.applied": "Appliquées : {n}.",
  "p.template_sync.overwritten":
    "Dont en écrasant des modifications de ce plan : {n}.",
  "p.template_sync.remaining": "Encore en attente : {n}.",
  "p.template_sync.refused_edited":
    "« {title} » n'a pas été appliquée : ce plan l'a modifiée ; cochez « Écraser la modification de ce plan » pour l'appliquer quand même.",
  "p.template_sync.refused_removed":
    "« {title} » n'a pas été appliquée : ce plan a supprimé la tâche, il n'y a donc rien à modifier.",
  "p.template_sync.could_not_apply":
    "Impossible d'appliquer les modifications.",
  "p.template_sync.apply_refused":
    "Le service a refusé les modifications : le modèle a peut-être changé entre-temps. Relisez les modifications.",
  "p.template_sync.plans_h": "Plans créés à partir de lui",
  "p.template_sync.plans_none":
    "Aucun plan que vous pouvez lire n'a été créé à partir de lui.",
  "p.template_sync.plans_row": "version {synced} sur {version}",
  "p.template_sync.waiting": "modifications en attente : {n}",
  "p.template_sync.plan_up_to_date": "à jour",
  "p.template_sync.open_changes": "Voir les modifications",
  "p.template_sync.could_not_load_plans":
    "Impossible de charger les plans créés à partir de lui.",
  "p.template_sync.revise_h": "Le réviser à partir d'un plan",
  "p.template_sync.revise_says":
    "Les tâches de l'arbre de plans choisi deviennent la version suivante du modèle. L'arbre doit garder la forme du modèle : les mêmes plans aux mêmes places, de nouveaux plans seulement à la fin. Les plans créés à partir du modèle ne changent pas ; chacun voit la différence sur sa page Modifications du modèle.",
  "p.template_sync.revise_plan": "Arbre de plans",
  "p.template_sync.revise_choose": "Choisissez un plan",
  "p.template_sync.revise": "Réviser le modèle",
  "p.template_sync.revise_ask":
    "Faire des tâches de « {name} » la version suivante de « {template} » ?",
  "p.template_sync.revised": "Désormais version {version}.",
  "p.template_sync.revised_omitted":
    "Plans de l'arbre que vous ne pouvez pas lire, laissés de côté : {n}.",
  "p.template_sync.revise_shape":
    "L'arbre de plans n'a plus la forme du modèle : une révision peut modifier des tâches et ajouter des plans à la fin, pas déplacer ni renommer des plans.",
  "p.template_sync.could_not_revise": "Impossible de réviser le modèle.",
  "p.template_sync.revise_pick": "Choisissez d'abord l'arbre de plans.",
  "p.mspdi_batch.title": "Importer plusieurs fichiers MS Project",
  "p.mspdi_batch.title_ppm": "Importer plusieurs fichiers MS Project — PPM",
  "p.mspdi_batch.intro":
    "Faites entrer jusqu'à dix fichiers MS Project (MSPDI XML) d'un coup. Chacun va dans un plan que vous choisissez ou dans un nouveau plan. La première étape est toujours un aperçu qui n'écrit rien ; le même bouton importe ensuite.",
  "p.mspdi_batch.files": "Fichiers (XML MS Project, dix au plus)",
  "p.mspdi_batch.too_many": "Choisissez dix fichiers au plus.",
  "p.mspdi_batch.none_chosen": "Choisissez d'abord au moins un fichier.",
  "p.mspdi_batch.target": "Dans",
  "p.mspdi_batch.target_new": "un nouveau plan",
  "p.mspdi_batch.target_existing": "un plan existant",
  "p.mspdi_batch.new_name":
    "Nom du nouveau plan (facultatif : le nom du projet dans le fichier)",
  "p.mspdi_batch.existing_plan": "Plan",
  "p.mspdi_batch.choose_plan": "Choisissez un plan",
  "p.mspdi_batch.choose_plan_first":
    "Choisissez d'abord le plan de « {file} ».",
  "p.mspdi_batch.parent": "Placer les nouveaux plans sous (facultatif)",
  "p.mspdi_batch.no_parent": "aucun plan",
  "p.mspdi_batch.preview": "Prévisualiser l'import",
  "p.mspdi_batch.import": "Importer",
  "p.mspdi_batch.import_ask":
    "Importer les fichiers maintenant ? Les plans sont créés et les tâches écrites comme dans l'aperçu. Ce n'est pas une seule transaction : un projet en échec arrête les autres, et les plans déjà créés restent.",
  "p.mspdi_batch.start_over": "Choisir d'autres fichiers",
  "p.mspdi_batch.result_preview_h": "Aperçu",
  "p.mspdi_batch.result_done_h": "Importé",
  "p.mspdi_batch.outcome_would_import": "serait importé",
  "p.mspdi_batch.outcome_imported": "importé",
  "p.mspdi_batch.outcome_failed": "échec",
  "p.mspdi_batch.outcome_not_attempted": "pas tenté",
  "p.mspdi_batch.outcome_unknown": "résultat inconnu",
  "p.mspdi_batch.new_plan_note": "un nouveau plan serait créé",
  "p.mspdi_batch.plan_created": "un nouveau plan a été créé",
  "p.mspdi_batch.failed_note":
    "Ce projet a échoué, les suivants n'ont donc pas été tentés. Corrigez le fichier et importez à nouveau : les plans existants sont mis à jour, pas dupliqués.",
  "p.mspdi_batch.pool_h": "Personnes et ressources dans les fichiers",
  "p.mspdi_batch.pool_note":
    "Les ressources sont listées une fois pour l'ensemble des fichiers, avec le nombre d'affectations de chacune. Elles ne sont pas associées à des personnes : un nom n'est pas une identité.",
  "p.mspdi_batch.pool_row": "{name} — affectations : {assignments}",
  "p.mspdi_batch.links_h": "Liens entre les fichiers",
  "p.mspdi_batch.links_would":
    "Liens entre les fichiers qui seraient créés : {n}.",
  "p.mspdi_batch.links_made": "Liens créés : {n} ; déjà présents : {m}.",
  "p.mspdi_batch.links_unresolved":
    "Liens vers l'extérieur des fichiers ou vers une tâche absente, non créés : {n}.",
  "p.mspdi_batch.links_after_failure":
    "Aucun lien entre les fichiers n'a été créé, car un projet a échoué.",
  "p.mspdi_batch.not_offered":
    "Non proposé ici : faire correspondre les champs personnalisés des fichiers, associer les ressources à des personnes et importer leurs taux.",
  "p.mspdi_batch.could_not": "L'import n'a pas fonctionné.",
  "p.mspdi_batch.refused":
    "Le service a refusé les fichiers : l'un n'est pas du XML MS Project, est trop gros ou désigne un plan que vous ne pouvez pas voir.",
  "p.mspdi_batch.link_from_jobs": "Importer plusieurs fichiers MS Project",
  "p.mspdi_batch.read_failed": "Impossible de lire « {file} ».",
  "p.components.link": "Composants",
  "p.components.title": "Composants",
  "p.components.title_ppm": "Composants — PPM",
  "p.components.intro":
    "Les composants regroupent les tâches d'un plan par partie du travail, par exemple l'API ou l'application mobile. Une tâche appartient à un composant au plus. Toutes les tâches sont comptées : celles qui ne sont dans aucun composant sont listées comme non affectées.",
  "p.components.loading": "Chargement…",
  "p.components.could_not_load": "Impossible de charger les composants.",
  "p.components.none": "Ce plan n'a pas encore de composants.",
  "p.components.unassigned": "Non affectées",
  "p.components.counts": "{open} ouvertes, {done} faites, {total} en tout",
  "p.components.lead": "Responsable : {lead}",
  "p.components.no_lead": "pas de responsable",
  "p.components.edit": "Modifier",
  "p.components.save": "Enregistrer",
  "p.components.cancel": "Annuler",
  "p.components.remove": "Retirer",
  "p.components.remove_ask":
    "Retirer le composant « {name} » ? Aucune tâche n'y appartient, rien d'autre ne change.",
  "p.components.in_use":
    "Tâches qui y appartiennent encore : {n}. Déplacez-les vers un autre composant avant de le retirer.",
  "p.components.in_use_refused":
    "Des tâches appartiennent encore à ce composant. Déplacez-les d'abord vers un autre.",
  "p.components.add_h": "Ajouter un composant",
  "p.components.name": "Nom (jusqu'à 120 caractères)",
  "p.components.lead_field":
    "Responsable (facultatif : une référence person: ou worker:)",
  "p.components.add": "Ajouter le composant",
  "p.components.name_required": "Un nom est requis.",
  "p.components.name_too_long": "Le nom dépasse 120 caractères.",
  "p.components.lead_bad":
    "Le responsable doit être une référence person: ou worker:.",
  "p.components.taken": "Un composant de ce nom existe déjà dans le plan.",
  "p.components.too_many": "Un plan compte au plus 200 composants.",
  "p.components.could_not_save": "Impossible d'enregistrer le composant.",
  "p.components.could_not_remove": "Impossible de retirer le composant.",
  "p.components.removed": "« {name} » retiré.",
  "p.components.saved": "« {name} » enregistré.",
  "p.components.added": "« {name} » ajouté.",
  "p.resource_roles.heading": "Rôles principaux des personnes",
  "p.resource_roles.says":
    "Le rôle principal d'une personne décide dans quelle ligne ci-dessus sa capacité déclarée compte comme offre. En définir un exige le rôle de gestionnaire des ressources ou un administrateur ; les autres ne voient que le leur.",
  "p.resource_roles.none": "Personne n'a encore de rôle principal.",
  "p.resource_roles.person": "Personne (une référence person: ou worker:)",
  "p.resource_roles.role": "Rôle (60 caractères au plus ; vide l'efface)",
  "p.resource_roles.set": "Définir le rôle",
  "p.resource_roles.form": "Définir le rôle principal d'une personne",
  "p.resource_roles.person_bad":
    "La personne doit être une référence person: ou worker:.",
  "p.resource_roles.role_too_long": "Le rôle dépasse 60 caractères.",
  "p.resource_roles.forbidden":
    "Définir un rôle exige le rôle de gestionnaire des ressources ou un administrateur.",
  "p.resource_roles.could_not_load": "Impossible de charger les rôles.",
  "p.resource_roles.could_not_set": "Impossible de définir le rôle.",
  "p.resource_roles.set_ok":
    "{person} a désormais le rôle principal « {role} ».",
  "p.resource_roles.cleared_ok": "{person} n'a plus de rôle principal.",
  "p.goals.link": "Objectifs",
  "p.goals.title": "Objectifs",
  "p.goals.title_ppm": "Objectifs — PPM",
  "p.goals.intro":
    "Ce que ce plan vise à atteindre, dans l'ordre. Un objectif est repéré par sa place dans la liste : une modification est donc refusée si quelqu'un d'autre a changé les objectifs après que cette page les a lus.",
  "p.goals.loading": "Chargement des objectifs…",
  "p.goals.could_not_load": "Impossible de charger les objectifs.",
  "p.goals.none": "Ce plan n'a pas encore d'objectif.",
  "p.goals.target": "Date cible : {date}",
  "p.goals.status": "État : {status}",
  "p.goals.st.NotStarted": "Pas commencé",
  "p.goals.st.InProgress": "En cours",
  "p.goals.st.Achieved": "Atteint",
  "p.goals.st.Missed": "Manqué",
  "p.goals.edit": "Modifier",
  "p.goals.save": "Enregistrer l'objectif",
  "p.goals.cancel": "Annuler",
  "p.goals.remove": "Retirer",
  "p.goals.remove_ask":
    "Retirer l'objectif « {title} » ? Les objectifs suivants remontent d'une place.",
  "p.goals.add_h": "Ajouter un objectif",
  "p.goals.title_field": "Titre (jusqu'à 512 caractères)",
  "p.goals.description_field": "Description (facultative)",
  "p.goals.target_field": "Date cible (facultative)",
  "p.goals.status_field": "État",
  "p.goals.status_none": "Aucun état",
  "p.goals.status_custom": "Un autre état, nommé ci-dessous",
  "p.goals.custom_field": "Nom de l'état",
  "p.goals.add": "Ajouter l'objectif",
  "p.goals.title_required": "Un titre est requis.",
  "p.goals.title_too_long": "Le titre dépasse 512 caractères.",
  "p.goals.date_bad": "La date cible doit être un vrai jour.",
  "p.goals.custom_required": "Nommez l'état ou choisissez-en un autre.",
  "p.goals.full": "Un plan contient au plus 256 objectifs.",
  "p.goals.changed":
    "Les objectifs ont changé après que cette page les a lus : rien n'a été enregistré. Ils ont été relus : vérifiez-les, puis réessayez.",
  "p.goals.gone": "Cet objectif n'existe plus. Les objectifs ont été relus.",
  "p.goals.could_not_save": "Impossible d'enregistrer l'objectif.",
  "p.goals.could_not_remove": "Impossible de retirer l'objectif.",
  "p.goals.added": "« {title} » ajouté.",
  "p.goals.saved": "« {title} » enregistré.",
  "p.goals.removed": "« {title} » retiré.",
  "p.custom_fields.title": "Champs personnalisés",
  "p.custom_fields.title_ppm": "Champs personnalisés — PPM",
  "p.custom_fields.intro":
    "Des champs qu'un administrateur ajoute à chaque plan ou tâche, chacun avec un type par rapport auquel ses valeurs sont vérifiées. Les valeurs sont stockées à part du plan : elles n'influent jamais sur la mise en correspondance.",
  "p.custom_fields.loading": "Chargement des champs personnalisés…",
  "p.custom_fields.could_not_load":
    "Impossible de charger les champs personnalisés.",
  "p.custom_fields.for.plan": "Champs des plans",
  "p.custom_fields.for.task": "Champs des tâches",
  "p.custom_fields.none": "Aucun déclaré pour l'instant.",
  "p.custom_fields.type.text": "Texte",
  "p.custom_fields.type.date": "Une date",
  "p.custom_fields.type.bool": "Oui ou non",
  "p.custom_fields.type.integer": "Nombre entier",
  "p.custom_fields.type.list": "Un choix dans une liste",
  "p.custom_fields.type.multi_list": "Plusieurs choix dans une liste",
  "p.custom_fields.is_required": "obligatoire",
  "p.custom_fields.kinds_only": "uniquement pour les plans de type : {kinds}",
  "p.custom_fields.choices": "choix : {options}",
  "p.custom_fields.withdraw": "Retirer",
  "p.custom_fields.withdraw_ask":
    "Retirer le champ « {label} » ? Les valeurs déjà enregistrées restent lisibles, marquées comme retirées, et aucune nouvelle ne peut être saisie.",
  "p.custom_fields.withdrawn": "« {label} » retiré.",
  "p.custom_fields.could_not_withdraw": "Impossible de retirer le champ.",
  "p.custom_fields.declare_h": "Déclarer un champ",
  "p.custom_fields.key_field":
    "Clé (minuscules, chiffres et tirets bas, commençant par une lettre)",
  "p.custom_fields.label_field": "Libellé (jusqu'à 100 caractères)",
  "p.custom_fields.type_field": "Type de valeur",
  "p.custom_fields.applies_field": "S'applique à",
  "p.custom_fields.applies.plan": "Les plans",
  "p.custom_fields.applies.task": "Les tâches",
  "p.custom_fields.kinds_field":
    "Uniquement pour ces types de plan (facultatif, séparés par des virgules)",
  "p.custom_fields.options_field":
    "Choix, un par ligne (jusqu'à 100, chacun jusqu'à 100 caractères)",
  "p.custom_fields.required_field":
    "Chaque plan ou tâche doit avoir une valeur",
  "p.custom_fields.declare": "Déclarer le champ",
  "p.custom_fields.key_bad":
    "La clé doit contenir des minuscules, chiffres et tirets bas, commencer par une lettre et faire au plus 100 caractères.",
  "p.custom_fields.label_bad":
    "Un libellé est requis, de 100 caractères au plus.",
  "p.custom_fields.options_none":
    "Un champ de liste demande au moins un choix.",
  "p.custom_fields.options_long":
    "Une liste contient au plus 100 choix, de 100 caractères chacun au plus.",
  "p.custom_fields.options_twice": "Chaque choix ne peut figurer qu'une fois.",
  "p.custom_fields.key_taken":
    "Un champ avec cette clé existe déjà pour ce type d'élément.",
  "p.custom_fields.could_not_declare": "Impossible de déclarer le champ.",
  "p.custom_fields.declared": "« {label} » déclaré.",
  "p.custom_fields.search_h": "Chercher par la valeur d'un champ",
  "p.custom_fields.field_field": "Champ",
  "p.custom_fields.choose": "Choisir…",
  "p.custom_fields.value_field": "Valeur",
  "p.custom_fields.yes": "Oui",
  "p.custom_fields.no": "Non",
  "p.custom_fields.search": "Chercher",
  "p.custom_fields.found.plan":
    "Plans correspondants que vous pouvez voir : {n}",
  "p.custom_fields.found.task":
    "Tâches correspondantes que vous pouvez voir : {n}",
  "p.custom_fields.truncated":
    "Plus de 5 000 valeurs sont enregistrées pour ce champ ; seules les 5 000 premières ont été examinées.",
  "p.custom_fields.field_required": "Choisissez un champ.",
  "p.custom_fields.value_required": "Indiquez une valeur à chercher.",
  "p.custom_fields.value_integer": "La valeur doit être un nombre entier.",
  "p.custom_fields.value_date": "La valeur doit être un vrai jour.",
  "p.custom_fields.could_not_search": "Impossible de chercher dans le champ.",
  "p.custom_fields.link": "Champs personnalisés",
  "p.custom_fields.values_title": "Champs personnalisés de ce plan",
  "p.custom_fields.values_title_ppm": "Champs personnalisés du plan — PPM",
  "p.custom_fields.values_intro":
    "Les valeurs des champs qu'un administrateur a déclarés pour les plans de ce type. L'enregistrement n'envoie que ce qui a changé ; si une valeur est fausse, rien n'est enregistré.",
  "p.custom_fields.values_could_not_load":
    "Impossible de charger les champs personnalisés de ce plan.",
  "p.custom_fields.values_missing":
    "Champs obligatoires encore sans valeur : {fields}",
  "p.custom_fields.values_none":
    "Aucun champ personnalisé ne s'applique encore au type de ce plan. Les champs sont déclarés sur la page",
  "p.custom_fields.values_h": "Valeurs",
  "p.custom_fields.values_required_label": "{label} (obligatoire)",
  "p.custom_fields.values_unset": "Aucune valeur",
  "p.custom_fields.values_save": "Enregistrer les valeurs",
  "p.custom_fields.values_unchanged": "Rien n'a changé : rien n'a été envoyé.",
  "p.custom_fields.values_saved": "Valeurs enregistrées.",
  "p.custom_fields.values_could_not_save":
    "Impossible d'enregistrer les valeurs.",
  "p.custom_fields.values_withdrawn_h":
    "Valeurs des champs retirés (lecture seule)",
  "p.custom_fields.v_integer":
    "{label} : la valeur doit être un nombre entier.",
  "p.custom_fields.v_date": "{label} : la valeur doit être un vrai jour.",
  "p.custom_fields.v_choice": "{label} : choisissez l'un des choix du champ.",
  "p.custom_fields.v_too_long": "{label} : le texte dépasse 1 000 caractères.",
  "p.custom_fields.v_required":
    "{label} est obligatoire et ne peut pas rester vide.",
  "p.onboarding_templates.title": "Modèles de listes de contrôle",
  "p.onboarding_templates.title_ppm": "Modèles de listes de contrôle — PPM",
  "p.onboarding_templates.intro":
    "La liste de contrôle par laquelle commence chaque arrivée ou départ, avec l'échéance de chaque point à partir de la date d'effet. Un dossier déjà commencé garde ses points.",
  "p.onboarding_templates.loading": "Chargement des listes…",
  "p.onboarding_templates.could_not_load": "Impossible de charger les listes.",
  "p.onboarding_templates.is_custom": "Ce type a sa propre liste.",
  "p.onboarding_templates.is_builtin":
    "Ce type utilise la liste intégrée ; ses points sont ceux du service, en anglais.",
  "p.onboarding_templates.before": "jours avant la date : {n}",
  "p.onboarding_templates.after": "jours après la date : {n}",
  "p.onboarding_templates.on_the_day": "à la date",
  "p.onboarding_templates.edit": "Modifier",
  "p.onboarding_templates.reset": "Revenir à la liste intégrée",
  "p.onboarding_templates.reset_ask":
    "Revenir à la liste intégrée pour « {kind} » ? La liste propre est supprimée ; les dossiers déjà commencés gardent leurs points.",
  "p.onboarding_templates.reset_done":
    "« {kind} » utilise de nouveau la liste intégrée.",
  "p.onboarding_templates.could_not_reset":
    "Impossible de revenir à la liste intégrée.",
  "p.onboarding_templates.item_title":
    "Point {n} : ce qu'il faut faire (jusqu'à 200 caractères)",
  "p.onboarding_templates.item_offset":
    "Point {n} : jours à partir de la date (−365 à 365 ; négatif signifie avant)",
  "p.onboarding_templates.up": "Monter le point {n}",
  "p.onboarding_templates.down": "Descendre le point {n}",
  "p.onboarding_templates.remove": "Retirer le point {n}",
  "p.onboarding_templates.add": "Ajouter un point",
  "p.onboarding_templates.save": "Enregistrer la liste",
  "p.onboarding_templates.cancel": "Annuler",
  "p.onboarding_templates.count_bad": "Une liste compte de 1 à {max} points.",
  "p.onboarding_templates.title_bad":
    "Point {n} : un titre est requis, jusqu'à 200 caractères.",
  "p.onboarding_templates.offset_bad":
    "Point {n} : les jours doivent être un nombre entier de −365 à 365.",
  "p.onboarding_templates.saved": "Liste de « {kind} » enregistrée.",
  "p.onboarding_templates.could_not_save": "Impossible d'enregistrer la liste.",
  "p.onboarding_templates.forbidden":
    "Modifier une liste demande l'autorisation de gérer les personnes.",
  "p.frameworks.version_n": "Version numéro {n}",
  "p.frameworks.behind":
    "Ce plan suit la version {from} ; la plus récente est la version {to}.",
  "p.frameworks.move": "Passer à la version {n}",
  "p.frameworks.move_ask":
    "Passer ce plan à la version {to} de « {name} » ? Il suit la version {from} ; le texte le plus récent s'appliquera ensuite.",
  "p.frameworks.moved": "Ce plan suit maintenant la version {n}.",
  "p.frameworks.behind_inherited":
    "Un plan au-dessus de celui-ci décide de la version qui s'applique ; changez-la là-bas.",
  "p.frameworks.show_versions": "Afficher les versions",
  "p.frameworks.could_not_load_versions": "Impossible de charger les versions.",
  "p.frameworks.versions_heading":
    "Versions de « {name} », la plus récente d'abord",
  "p.frameworks.plans_following": "plans qui la suivent : {n}",
  "p.frameworks.newest": "la plus récente",
  "p.frameworks.this_plan": "celle de ce plan",
  "p.frameworks.pin": "Suivre la version {n}",
  "p.frameworks.pin_ask":
    "Faire suivre à ce plan la version {n} de « {name} » ? Il la garde jusqu'à ce qu'on le change.",
  "p.frameworks.pinned": "Ce plan suit maintenant la version {n}.",
  "p.processes.not_yours":
    "Seule la personne qui a défini ce processus, ou un administrateur, peut le modifier ou le retirer.",
  "p.processes.unchanged": "Rien n'a changé : aucune version n'a été créée.",
  "p.processes.changed":
    "« {name} » enregistré en version {v} ; les exécutions en cours gardent leur version.",
  "p.processes.retire_ask":
    "Retirer « {name} » ? Aucune nouvelle exécution ne pourra commencer ; celles en cours continuent.",
  "p.processes.retired": "« {name} » retiré.",
  "p.processes.edit": "Modifier « {name} »",
  "p.processes.retire": "Retirer « {name} »",
  "p.processes.edit_heading": "Modifier « {name} » (version {v} actuellement)",
  "p.processes.edit_note":
    "Enregistrer crée une nouvelle version ; les exécutions en cours gardent la version avec laquelle elles ont commencé.",
  "p.processes.save_version": "Enregistrer en nouvelle version",
  "p.processes.cancel_edit": "Annuler",
  "p.processes.versions": "Versions, la plus récente d'abord",
  "p.processes.current": "actuelle",
  "p.announcements.title_bad": "Un titre est requis, jusqu'à 120 caractères.",
  "p.announcements.body_bad":
    "Un message est requis, jusqu'à 4 000 caractères.",
  "p.announcements.time_bad":
    "Une heure n'a pas pu être lue ; choisissez-la de nouveau.",
  "p.announcements.expires_bad":
    "L'expiration doit suivre l'heure de publication.",
  "p.announcements.saved": "« {title} » enregistré.",
  "p.announcements.could_not_save": "Impossible d'enregistrer l'annonce.",
  "p.announcements.change_h": "Modifier « {title} »",
  "p.announcements.change": "Modifier",
  "p.announcements.publish_at": "Publié à partir de",
  "p.announcements.save": "Enregistrer les modifications",
  "p.announcements.cancel": "Annuler",
  "p.approvals.title": "Approbations",
  "p.approvals.title_ppm": "Approbations — PPM",
  "p.approvals.intro":
    "Une personne est sollicitée pour approuver un plan, une tâche ou un autre objet, hors de tout processus d'approbation. Une décision est définitive, et un refus dit pourquoi.",
  "p.approvals.loading": "Chargement des approbations…",
  "p.approvals.could_not_load": "Impossible de charger les approbations.",
  "p.approvals.waiting_h": "En attente de votre part",
  "p.approvals.waiting_none": "Rien n'attend votre décision.",
  "p.approvals.asked_by": "Demandé par {who} le {at}",
  "p.approvals.comment_field":
    "Commentaire (obligatoire pour refuser ; jusqu'à 2 000 caractères)",
  "p.approvals.approve": "Approuver",
  "p.approvals.reject": "Refuser",
  "p.approvals.approved_done": "Votre approbation est enregistrée.",
  "p.approvals.rejected_done": "Votre refus est enregistré.",
  "p.approvals.asked_h": "Ce que vous avez demandé",
  "p.approvals.asked_none": "Vous n'avez encore rien demandé.",
  "p.approvals.asked_of": "Demandé à {approver}",
  "p.approvals.their_comment": "Son commentaire : {comment}",
  "p.approvals.from_process":
    "Créée par un processus d'approbation : annulez son exécution pour l'arrêter.",
  "p.approvals.withdraw": "Retirer",
  "p.approvals.withdraw_ask":
    "Retirer cette demande ? La personne sollicitée ne pourra plus la trancher.",
  "p.approvals.withdrawn_done": "Demande retirée.",
  "p.approvals.ask_h": "Demander une approbation",
  "p.approvals.subject_kind_field": "Ce qu'elle concerne",
  "p.approvals.subject_field": "Son identifiant (un UUID)",
  "p.approvals.approver_field":
    "Personne sollicitée (person: ou worker: et un UUID)",
  "p.approvals.message_field": "Un mot pour elle (facultatif)",
  "p.approvals.ask": "Demander l'approbation",
  "p.approvals.asked_done": "{approver} a été sollicité pour approuver.",
  "p.approvals.subject_bad": "L'identifiant de l'objet doit être un UUID.",
  "p.approvals.approver_bad":
    "La personne sollicitée doit être person: ou worker: suivi d'un UUID.",
  "p.approvals.text_long":
    "Un mot ou un commentaire compte au plus 2 000 caractères.",
  "p.approvals.why_required":
    "Un refus doit dire pourquoi : écrivez un commentaire.",
  "p.approvals.already_settled":
    "Cette demande a déjà été tranchée ou retirée, et une décision est définitive. Les listes ont été relues.",
  "p.approvals.not_found":
    "Cet objet ou cette demande n'existe pas, ou vous ne pouvez pas le lire.",
  "p.approvals.not_approver":
    "Seule la personne sollicitée peut trancher cette demande.",
  "p.approvals.not_requester":
    "Seule la personne qui a demandé peut retirer cette demande.",
  "p.approvals.could_not_request": "Impossible de demander l'approbation.",
  "p.approvals.could_not_decide": "Impossible d'enregistrer la décision.",
  "p.approvals.could_not_withdraw": "Impossible de retirer la demande.",
  "p.approvals.status.pending": "En attente",
  "p.approvals.status.approved": "Approuvée",
  "p.approvals.status.rejected": "Refusée",
  "p.approvals.status.withdrawn": "Retirée",
  "p.approvals.kind.plan": "Un plan",
  "p.approvals.kind.task": "Tâche",
  "p.approvals.kind.risk": "Risque",
  "p.approvals.kind.proposal": "Proposition",
  "p.approvals.kind.idea": "Idée",
  "p.approvals.kind.canvas_item": "Élément de canevas",
  "p.approvals.kind.budget_line": "Ligne budgétaire",
  "p.approvals.kind.scenario": "Scénario",
  "p.approvals.kind.document_version": "Version de document",
  "p.custom_fields.task_values_none":
    "Aucun champ personnalisé n'est encore déclaré pour les tâches. Les champs sont déclarés sur la page",
  "p.custom_fields.task_values_could_not_load":
    "Impossible de charger les champs personnalisés de cette tâche.",
  "p.product_roadmap.title": "Feuille de route du produit",
  "p.product_roadmap.title_ppm": "Feuille de route du produit — PPM",
  "p.product_roadmap.intro":
    "Ce que fait ce produit maintenant, ensuite et plus tard, tel que le montre sa feuille de route publique. Une feuille de route appartient à un plan de type Product.",
  "p.product_roadmap.loading": "Chargement de la feuille de route…",
  "p.product_roadmap.could_not_load":
    "Impossible de charger la feuille de route.",
  "p.product_roadmap.not_product":
    "Seul un plan de type Product a une feuille de route publique. Changez le type du plan pour en créer une.",
  "p.product_roadmap.state_h": "Son état",
  "p.product_roadmap.published": "Publiée",
  "p.product_roadmap.not_published": "Non publiée",
  "p.product_roadmap.fresh.fresh": "À jour",
  "p.product_roadmap.fresh.stale": "Périmée : revue à faire",
  "p.product_roadmap.fresh.unknown": "Jamais revue",
  "p.product_roadmap.reviewed_on": "revue le {date}",
  "p.product_roadmap.stale_rule":
    "Une feuille de route est périmée {days} jours après sa dernière revue.",
  "p.product_roadmap.mark_reviewed": "La marquer revue aujourd'hui",
  "p.product_roadmap.reviewed_done": "Marquée revue aujourd'hui.",
  "p.product_roadmap.own_h": "La feuille de route",
  "p.product_roadmap.make_h": "Créer la feuille de route",
  "p.product_roadmap.f_title": "Titre (jusqu'à 200 caractères)",
  "p.product_roadmap.f_slug":
    "Adresse (minuscules, chiffres et tirets simples, jusqu'à 64)",
  "p.product_roadmap.f_slug_new":
    "Adresse (facultative ; tirée du titre si elle est vide)",
  "p.product_roadmap.f_vision":
    "Vision (facultative, jusqu'à 2 000 caractères)",
  "p.product_roadmap.f_service":
    "Ce qu'est le produit, pour qui le découvre (facultatif, jusqu'à 2 000 caractères)",
  "p.product_roadmap.save": "Enregistrer la feuille de route",
  "p.product_roadmap.make": "Créer la feuille de route maintenant",
  "p.product_roadmap.made": "Feuille de route créée.",
  "p.product_roadmap.saved": "Feuille de route enregistrée.",
  "p.product_roadmap.unchanged": "Rien n'a changé : rien n'a été envoyé.",
  "p.product_roadmap.items_h": "Éléments",
  "p.product_roadmap.no_items": "Aucun élément pour l'instant.",
  "p.product_roadmap.placement.recently_delivered": "Livré récemment",
  "p.product_roadmap.placement.now": "Maintenant",
  "p.product_roadmap.placement.next": "Ensuite",
  "p.product_roadmap.placement.later": "Plus tard",
  "p.product_roadmap.placement.delivered_this_year": "Livré cette année",
  "p.product_roadmap.placement.not_doing": "Ne sera pas fait",
  "p.product_roadmap.placement.none": "Non placé",
  "p.product_roadmap.phase_is": "Phase : {phase}",
  "p.product_roadmap.tags_are": "Étiquettes : {tags}",
  "p.product_roadmap.outcome_is": "Résultat attendu : {outcome}",
  "p.product_roadmap.change_item": "Modifier « {title} »",
  "p.product_roadmap.save_item": "Enregistrer l'élément",
  "p.product_roadmap.cancel": "Annuler",
  "p.product_roadmap.change": "Modifier",
  "p.product_roadmap.up": "Monter",
  "p.product_roadmap.down": "Descendre",
  "p.product_roadmap.move_to": "Déplacer vers",
  "p.product_roadmap.move": "Déplacer",
  "p.product_roadmap.remove": "Retirer",
  "p.product_roadmap.remove_ask":
    "Retirer « {title} » ({id}) ? Son identifiant public n'est jamais donné à un autre élément.",
  "p.product_roadmap.add_h": "Ajouter un élément",
  "p.product_roadmap.f_placement": "Où il va",
  "p.product_roadmap.f_description":
    "Ce qu'est le travail (facultatif, jusqu'à 2 000 caractères)",
  "p.product_roadmap.f_outcome":
    "Ce qui change, et pour qui (facultatif, jusqu'à 2 000 caractères)",
  "p.product_roadmap.f_phase":
    "Phase, dans les mots du produit (facultative, jusqu'à 100 caractères)",
  "p.product_roadmap.f_tags":
    "Étiquettes, séparées par des virgules (facultatives ; jusqu'à 20, de 50 caractères chacune au plus)",
  "p.product_roadmap.add": "Ajouter l'élément",
  "p.product_roadmap.item_added": "« {title} » ajouté.",
  "p.product_roadmap.item_saved": "« {title} » enregistré.",
  "p.product_roadmap.item_moved": "« {title} » déplacé vers {placement}.",
  "p.product_roadmap.item_removed": "« {title} » retiré.",
  "p.product_roadmap.title_bad": "Un titre est requis, jusqu'à 200 caractères.",
  "p.product_roadmap.slug_bad":
    "L'adresse doit contenir des minuscules, chiffres et tirets simples, jusqu'à 64 caractères.",
  "p.product_roadmap.text_long": "Un texte compte au plus 2 000 caractères.",
  "p.product_roadmap.phase_long": "La phase compte au plus 100 caractères.",
  "p.product_roadmap.tags_many": "Au plus 20 étiquettes.",
  "p.product_roadmap.tag_long": "Une étiquette compte au plus 50 caractères.",
  "p.product_roadmap.tag_twice":
    "Chaque étiquette ne peut figurer qu'une fois.",
  "p.product_roadmap.slug_taken":
    "Une autre feuille de route a ou avait cette adresse ; choisissez-en une autre.",
  "p.product_roadmap.gone":
    "La feuille de route ou l'élément n'existe plus ; la page a été relue.",
  "p.product_roadmap.could_not_save":
    "Impossible d'enregistrer la feuille de route.",
  "p.product_roadmap.could_not_change_item":
    "Impossible de modifier l'élément.",
  "p.product_roadmap.publish": "La publier…",
  "p.product_roadmap.unpublish": "La retirer du lecteur public…",
  "p.product_roadmap.preview_h": "Ce que ferait la publication",
  "p.product_roadmap.preview_nothing":
    "Cela ne change rien : la feuille de route est déjà ainsi.",
  "p.product_roadmap.preview_conflicts":
    "Quelqu'un d'autre a modifié ceci depuis que cette page l'a lu : rien ne peut être publié :",
  "p.product_roadmap.preview_problems":
    "Le service refuserait cet ensemble de changements :",
  "p.product_roadmap.publish_now": "Publier l'ensemble de changements",
  "p.product_roadmap.published_done": "Ensemble de changements publié.",
  "p.product_roadmap.conflict_refused":
    "Quelqu'un d'autre a modifié la feuille de route entre-temps : rien n'a été publié. La page a été relue.",
  "p.product_roadmap.change_line": "{field} : de {from} à {to}",
  "p.product_roadmap.item_change_line":
    "« {title} », {field} : de {from} à {to}",
  "p.product_roadmap.new_item_line": "Un nouvel élément : « {title} »",
  "p.product_roadmap.conflict_line":
    "{field} : maintenant {current}, alors que vous demandiez {value}",
  "p.product_roadmap.conflict_gone": "« {title} » n'existe plus",
  "p.product_roadmap.yes": "oui",
  "p.product_roadmap.no": "non",
  "p.product_roadmap.none": "aucun",
  "p.product_roadmap.field.title": "Titre",
  "p.product_roadmap.field.vision": "Ambition",
  "p.product_roadmap.field.service_description": "Ce qu'est le produit",
  "p.product_roadmap.field.category_pid": "Catégorie",
  "p.product_roadmap.field.published": "Publiée sur le lecteur public",
  "p.product_roadmap.field.description": "Descriptif",
  "p.product_roadmap.field.expected_outcome": "Résultat attendu",
  "p.product_roadmap.field.phase": "Étape",
  "p.product_roadmap.field.tags": "Étiquettes",
  "p.product_roadmap.field.organization_pids": "Organisations",
  "p.product_roadmap.field.details": "Détails",
  "p.product_roadmap.field.placement": "Emplacement",
  "p.product_roadmap.field.item": "L'élément",
  "p.product_roadmap.could_not_publish":
    "Impossible de publier l'ensemble de changements.",
  "p.product_roadmaps.title": "Feuilles de route des produits",
  "p.product_roadmaps.title_ppm": "Feuilles de route des produits — PPM",
  "p.product_roadmaps.intro":
    "Toutes les feuilles de route que vous pouvez lire, publiées ou non, et celles qui doivent être revues. Ouvrez-en une pour la modifier.",
  "p.product_roadmaps.loading": "Chargement des feuilles de route…",
  "p.product_roadmaps.could_not_load":
    "Impossible de charger les feuilles de route.",
  "p.product_roadmaps.due_h": "À revoir",
  "p.product_roadmaps.due_none": "Toutes les feuilles de route sont à jour.",
  "p.product_roadmaps.all_h": "Toutes les feuilles de route",
  "p.product_roadmaps.all_none":
    "Aucune feuille de route pour l'instant. Un plan Product en crée une sur sa propre page.",
  "p.product_roadmaps.counts":
    "éléments placés : {placed} ; non placés : {unplaced}",
  "p.product_roadmaps.in_family": "famille : {family}",
  "p.product_roadmaps.families_h": "Familles de produits",
  "p.product_roadmaps.families_none":
    "Aucune famille de produits pour l'instant.",
  "p.product_roadmaps.products": "produits : {n}",
  "p.roadmap_categories.title": "Catégories de feuilles de route",
  "p.roadmap_categories.title_ppm": "Catégories de feuilles de route — PPM",
  "p.roadmap_categories.intro":
    "Les catégories sous lesquelles une feuille de route de produit est classée. Une catégorie est définie une fois : la renommer la renomme sur chaque feuille de route qui l'utilise.",
  "p.roadmap_categories.back": "Retour aux feuilles de route des produits",
  "p.roadmap_categories.loading": "Chargement des catégories…",
  "p.roadmap_categories.could_not_load":
    "Impossible de charger les catégories.",
  "p.roadmap_categories.none": "Aucune catégorie pour l'instant.",
  "p.roadmap_categories.is_retired": "retirée",
  "p.roadmap_categories.translations_are": "traductions : {list}",
  "p.roadmap_categories.change_h": "Modifier « {name} »",
  "p.roadmap_categories.f_name": "Nom (jusqu'à 100 caractères)",
  "p.roadmap_categories.f_translations":
    "Son nom dans d'autres langues, un par ligne : un code de langue, deux-points, le nom (cy: Cleifion)",
  "p.roadmap_categories.save": "Enregistrer la catégorie",
  "p.roadmap_categories.cancel": "Annuler",
  "p.roadmap_categories.change": "Modifier",
  "p.roadmap_categories.retire": "Retirer",
  "p.roadmap_categories.restore": "Rétablir",
  "p.roadmap_categories.retire_ask":
    "Retirer « {name} » ? Elle ne pourra plus être choisie ; une feuille de route qui l'a la garde.",
  "p.roadmap_categories.retired": "« {name} » retirée.",
  "p.roadmap_categories.restored": "« {name} » rétablie.",
  "p.roadmap_categories.add_h": "Ajouter une catégorie",
  "p.roadmap_categories.add": "Ajouter la catégorie",
  "p.roadmap_categories.added": "« {name} » ajoutée.",
  "p.roadmap_categories.saved": "« {name} » enregistrée.",
  "p.roadmap_categories.name_bad": "Un nom est requis, jusqu'à 100 caractères.",
  "p.roadmap_categories.code_bad":
    "Chaque traduction commence par un code de langue comme cy ou zh-cn, puis deux-points.",
  "p.roadmap_categories.text_bad":
    "Chaque traduction compte de 1 à 100 caractères après ses deux-points.",
  "p.roadmap_categories.code_twice":
    "Chaque langue ne peut être donnée qu'une fois.",
  "p.roadmap_categories.too_many": "Au plus 50 traductions.",
  "p.roadmap_categories.taken": "Une catégorie de ce nom existe déjà.",
  "p.roadmap_categories.forbidden":
    "Modifier les catégories demande un administrateur.",
  "p.roadmap_categories.could_not_save":
    "Impossible d'enregistrer la catégorie.",
  "p.product_roadmap.f_category": "Catégorie (facultative)",
  "p.product_roadmap.no_category": "Aucune catégorie",
  "p.product_roadmap.manage_categories": "Modifier les catégories",
  "p.roadmap_family.title": "Famille de produits",
  "p.roadmap_family.title_ppm": "Famille de produits — PPM",
  "p.roadmap_family.intro":
    "Un plan qui contient des plans de produit. Sa couleur marque ses produits sur les feuilles de route publiques ; toute couleur peut être choisie, et on montre comment elle se lit.",
  "p.roadmap_family.loading": "Chargement de la famille…",
  "p.roadmap_family.could_not_load": "Impossible de charger la famille.",
  "p.roadmap_family.products": "produits dans la famille : {n}",
  "p.roadmap_family.colour_is": "couleur : {colour}",
  "p.roadmap_family.with_white": "avec un texte blanc : {ratio} pour 1",
  "p.roadmap_family.with_black": "avec un texte noir : {ratio} pour 1",
  "p.roadmap_family.meets_aa": "atteint AA pour le texte courant (4,5 pour 1)",
  "p.roadmap_family.below_aa":
    "n'atteint pas AA pour le texte courant (4,5 pour 1)",
  "p.roadmap_family.better_white": "Un texte blanc s'y lit mieux.",
  "p.roadmap_family.better_black": "Un texte noir s'y lit mieux.",
  "p.roadmap_family.no_colour": "Pas encore de couleur.",
  "p.roadmap_family.set_h": "Sa couleur et ses noms",
  "p.roadmap_family.f_colour":
    "Couleur (# et six chiffres hexadécimaux, comme #rrggbb)",
  "p.roadmap_family.f_picker": "Ou la choisir",
  "p.roadmap_family.f_translations":
    "Son nom dans d'autres langues, un par ligne : un code de langue, deux-points, le nom (cy: Arian)",
  "p.roadmap_family.save": "Enregistrer la couleur",
  "p.roadmap_family.remove": "Retirer la couleur",
  "p.roadmap_family.remove_ask":
    "Retirer la couleur ? Ses noms dans d'autres langues partent avec elle.",
  "p.roadmap_family.removed": "Couleur et traductions retirées.",
  "p.roadmap_family.saved": "Famille enregistrée.",
  "p.roadmap_family.unchanged": "Rien n'a changé : rien n'a été envoyé.",
  "p.roadmap_family.colour_bad":
    "La couleur est # et six chiffres hexadécimaux, comme #rrggbb.",
  "p.roadmap_family.could_not_save": "Impossible d'enregistrer la famille.",
  "p.roadmap_family.link": "Famille de produits",
  "p.product_roadmap.tr_h": "Traductions",
  "p.product_roadmap.tr_intro":
    "La feuille de route et ses éléments dans d'autres langues, champ par champ. Une traduction est signalée quand sa source a changé depuis.",
  "p.product_roadmap.tr_lang": "Code de langue (comme cy ou zh-cn)",
  "p.product_roadmap.tr_lang_review": "{lang} (à revoir : {n})",
  "p.product_roadmap.tr_roadmap_h": "Les champs propres de la feuille de route",
  "p.product_roadmap.tr_item": "Élément : {title}",
  "p.product_roadmap.tr_source": "source : {text}",
  "p.product_roadmap.tr_needs_review":
    "sa source a changé depuis : vérifiez-la",
  "p.product_roadmap.tr_save": "Enregistrer la traduction",
  "p.product_roadmap.tr_saved": "Traduction en {lang} enregistrée.",
  "p.product_roadmap.tr_lang_bad":
    "Indiquez un code de langue comme cy ou zh-cn.",
  "p.product_roadmap.tr_tags_count":
    "{field} : donnez autant d'étiquettes que la source, séparées par des virgules.",
  "p.product_roadmap.tr_too_long": "{field} : la traduction est trop longue.",
  "p.product_roadmap.tr_tag_long":
    "{field} : une étiquette compte au plus 50 caractères.",
  "p.organizations.col_short": "Nom court",
  "p.organizations.col_code": "Code court",
  "p.organizations.name_national": "{name} (nationale)",
  "p.organizations.names_btn": "Noms",
  "p.organizations.names_for": "Modifier les noms de {name}",
  "p.organizations.edit_h": "Les noms de {name}",
  "p.organizations.f_name": "Nom (1 à 200 caractères)",
  "p.organizations.f_short": "Nom court (facultatif, jusqu'à 50 caractères)",
  "p.organizations.f_code":
    "Code court (facultatif : une lettre ou un chiffre, puis jusqu'à 19 lettres, chiffres ou traits d'union)",
  "p.organizations.f_national":
    "Nationale : la seule organisation de portée nationale, listée en premier",
  "p.organizations.f_translations":
    "Noms dans d'autres langues, un par ligne sous la forme code : nom | nom court (le nom court est facultatif)",
  "p.organizations.save": "Enregistrer les noms",
  "p.organizations.cancel": "Annuler",
  "p.organizations.saved": "Noms de {name} enregistrés.",
  "p.organizations.unchanged": "Rien n'a changé, donc rien n'a été envoyé.",
  "p.organizations.taken_409":
    "Une autre organisation a ce nom ou ce code ; choisissez-en un autre.",
  "p.organizations.name_bad": "Le nom doit compter de 1 à 200 caractères.",
  "p.organizations.name_taken": "Une autre organisation a ce nom.",
  "p.organizations.short_bad": "Le nom court compte au plus 50 caractères.",
  "p.organizations.code_bad":
    "Un code est une lettre ou un chiffre, puis jusqu'à 19 lettres, chiffres ou traits d'union.",
  "p.organizations.code_taken": "Une autre organisation a ce code.",
  "p.organizations.national_taken":
    "Une autre organisation est déjà nationale ; retirez-le-lui d'abord.",
  "p.organizations.tr_too_many": "Des noms dans 50 langues au plus.",
  "p.organizations.tr_code_bad":
    "Chaque ligne commence par un code de langue comme cy ou zh-cn, puis deux-points.",
  "p.organizations.tr_name_bad":
    "Le nom de chaque langue doit compter de 1 à 200 caractères.",
  "p.organizations.tr_short_bad":
    "Le nom court de chaque langue compte au plus 50 caractères.",
  "p.organizations.tr_code_twice": "Chaque langue ne peut figurer qu'une fois.",
  "p.product_roadmap.f_details":
    "Détails (facultatif : une ligne chacun, deux espaces de plus par niveau, deux niveaux au plus)",
  "p.product_roadmap.f_orgs": "Pour qui (facultatif)",
  "p.product_roadmap.orgs_are": "Pour : {orgs}",
  "p.product_roadmap.org_retired": "{name} (retirée)",
  "p.product_roadmap.detail_long":
    "Une ligne de détail compte au plus 300 caractères.",
  "p.product_roadmap.detail_deep":
    "La première ligne de détail est en haut, et chaque ligne au plus un niveau plus bas que la précédente, deux niveaux au plus.",
  "p.product_roadmap.details_many": "50 lignes de détail au plus.",
  "p.product_roadmap.orgs_many": "50 organisations au plus.",
};
