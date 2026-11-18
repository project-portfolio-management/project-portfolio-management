// Spanish (international) (es-001) for the page and component catalogue (WW-1.1). Typed as a complete record of
// the English keys, so a missing string is a compile error.
//
// STATUS: MACHINE DRAFT. Written by Claude (the AI assistant) on 2026-10-31 and NOT reviewed by a
// speaker or a professional translator. The review is recorded in
// `governance/translation-reviews/es-001.md`; update the status there, not here.

import type { PageKey } from "./i18n-pages";

export const PAGES_ES: Record<PageKey, string> = {
  "c.CalendarMonth.next_month": "Mes siguiente",
  "p.calendar.day": "Día",
  "p.calendar.agenda": "Agenda",
  "p.calendar.status": "Estado",
  "p.calendar.status_done": "hecho",
  "p.calendar.status_open": "abierto",
  "p.calendar.item": "Elemento de trabajo",
  "p.calendar.showing": "Mostrando {view}, del {start} al {end}",
  "p.calendar.more": "+{n} más",
  "p.calendar.milestone": "Hito",
  "p.calendar.due": "Vencimiento",
  "p.plans.pid.board.assignee": "Responsable",
  "p.plans.pid.board.moving": "Moviendo {title} a {column}…",
  "p.plans.pid.board.move_failed":
    "No se pudo mover {title} a {column}; ha vuelto a su lugar.",
  "p.plans.pid.board.empty_column": "Sin tareas",
  "p.plans.pid.board.search": "Buscar en el tablero",
  "p.plans.pid.board.task": "Tarea",
  "p.plans.pid.board.status": "Estado",
  "c.GanttView.holiday": "Festivo",
  "c.GanttView.shaded":
    "Las columnas sombreadas son días no laborables: fines de semana y festivos del calendario laboral de este plan.",
  "c.GanttView.kind": "Tipo",
  "c.GanttView.stage": "Etapa",
  "c.GanttView.task": "Tarea",
  "c.GanttView.start": "Inicio",
  "c.GanttView.end": "Fin",
  "c.GanttView.predecessors": "Depende de",
  "c.GanttView.depends_on": "Depende de: {names}",
  "c.GanttView.critical": "{name} (ruta crítica)",
  "c.GanttView.zoom_in": "Escala de tiempo más fina",
  "c.GanttView.zoom_out": "Escala de tiempo más amplia",
  "c.GanttView.zoomed": "Escala de tiempo: {unit}",
  "c.GanttView.unit_day": "día",
  "c.GanttView.unit_week": "semana",
  "c.GanttView.unit_month": "mes",
  "c.GanttView.export": "Exportar el cronograma",
  "c.GanttView.csv": "CSV",
  "c.GanttView.excel": "Excel",
  "c.GanttView.exported": "Se exportaron {n} tareas a {file}",
  "c.CalendarMonth.week": "Semana",
  "c.CalendarMonth.four_weeks": "4 semanas",
  "c.CalendarMonth.month": "Mes",
  "c.CalendarMonth.today": "Hoy",
  "c.CalendarMonth.previous": "{view} anterior",
  "c.CalendarMonth.next": "{view} siguiente",
  "c.CalendarMonth.working_calendar": "Calendario laboral",
  "c.CalendarMonth.previous_month": "Mes anterior",
  "c.CapacityMeter.days_over_capacity": "Días por encima de la capacidad",
  "c.CostRiskView.adds": "{risk}: añade {money} — {eventHitText}",
  "c.CostRiskView.already_spent": "Ya gastado",
  "c.CostRiskView.assumptions": "Supuestos",
  "c.CostRiskView.budget": "presupuesto",
  "c.CostRiskView.budget_2": "Presupuesto",
  "c.CostRiskView.budget_sum_of_planned_amounts":
    "Presupuesto (suma de importes previstos)",
  "c.CostRiskView.by_plan": "Por plan",
  "c.CostRiskView.cost": "Costo",
  "c.CostRiskView.cost_at_each_percentile":
    "Costo en {currency} en cada percentil",
  "c.CostRiskView.cost_from": "Costo desde",
  "c.CostRiskView.each_bar_is_a_cost":
    "Cada barra es un rango de costo; cuanto más alta, más simulaciones terminaron ahí. Las líneas continuas son P50, P80 y P90 (rotuladas); la línea discontinua es el presupuesto.",
  "c.CostRiskView.each_plan_s_budget_and":
    "Presupuesto y costo simulado medio de cada plan en {currency}",
  "c.CostRiskView.estimate": "Estimación",
  "c.CostRiskView.independence": "Independencia.",
  "c.CostRiskView.left_out": "Excluido:",
  "c.CostRiskView.line": "Línea",
  "c.CostRiskView.low_planned_high": "Bajo / previsto / alto",
  "c.CostRiskView.mean_simulated_cost": "Costo simulado medio",
  "c.CostRiskView.no_register_risk_was_used":
    "No se usó ningún riesgo del registro como evento de costo.",
  "c.CostRiskView.outcome": "Resultado",
  "c.CostRiskView.per_line_estimate_and_sensitivity":
    "Estimación y sensibilidad por línea en {currency}",
  "c.CostRiskView.plan": "Plan",
  "c.CostRiskView.risks_that_added_cost": "Riesgos que añadieron costo",
  "c.CostRiskView.seed_the_same_seed_gives":
    "Semilla {seed}: la misma semilla da el mismo resultado. Hasta {max_iterations} iteraciones.",
  "c.CostRiskView.sensitivity": "Sensibilidad",
  "c.CostRiskView.simulations": "Simulaciones",
  "c.CostRiskView.spent": "Gastado",
  "c.CostRiskView.the_same_figures_as_a": "Las mismas cifras en una tabla",
  "c.CostRiskView.to": "a",
  "c.CostRiskView.to_of": "{money} a {money2}: {count} de {iterations}",
  "c.CostRiskView.what_drives_it": "Qué lo impulsa",
  "c.CpmGantt.cannot_meet_a_constraint_or":
    "no puede cumplir una restricción o fecha límite (la línea marca el límite)",
  "c.CpmGantt.critical_any_slip_moves_the":
    "crítica: cualquier retraso mueve el final",
  "c.CpmGantt.has_float_the_hatched_tail": "tiene holgura (la cola rayada)",
  "c.CpmGantt.legend": "Leyenda",
  "c.CpmGantt.near_critical": "casi crítica",
  "c.CpmGantt.no_scheduled_tasks_to_draw":
    "Aún no hay tareas programadas que dibujar. Asigne a una tarea una duración (y una fecha de inicio o una predecesora).",
  "c.CpmGantt.the_outline_under_a_bar":
    "el contorno bajo una barra indica su línea base",
  "c.CpmGantt.to_float_is_counted_in":
    "{from} a {to}. La holgura se cuenta en días hábiles del plan; las barras se dibujan en días naturales.",
  "c.CpmTable.calendar": "Calendario",
  "c.CpmTable.constraint": "Restricción",
  "c.CpmTable.deadline": "Fecha límite",
  "c.CpmTable.finishes": "Termina",
  "c.CpmTable.free_float": "Holgura libre",
  "c.CpmTable.overrides_its_predecessors": "(prevalece sobre sus predecesoras)",
  "c.CpmTable.plan": "plan",
  "c.CpmTable.plan_assignee": "plan + responsable",
  "c.CpmTable.starts": "Comienza",
  "c.CpmTable.state": "Estado",
  "c.CpmTable.task": "Tarea",
  "c.CpmTable.task_schedule_with_float_in":
    "Cronograma de tareas con holgura, en días hábiles del plan",
  "c.CpmTable.total_float": "Holgura total",
  "c.CpmTable.untitled": "(sin título)",
  "c.CumulativeFlow.cumulative_flow": "Flujo acumulado",
  "c.CumulativeFlow.day": "Día",
  "c.CumulativeFlow.every_value_in_the_chart":
    "Todos los valores del gráfico. {note}",
  "c.CumulativeFlow.no_samples_in_this_window":
    "No hay muestras en este período.",
  "c.CumulativeFlow.the_vertical_gap_above_done":
    "la distancia vertical sobre «Hecho» es el trabajo en curso; la distancia horizontal hasta ella es aproximadamente el tiempo de ciclo",
  "c.CumulativeFlow.total": "Total",
  "c.CumulativeFlow.total_in_progress_done":
    "{total} en total · {work_in_progress} en curso · {done} hechas",
  "c.EvBars.the_double_line_is_the":
    "La línea doble es el presupuesto a la finalización: {money}.",
  "c.EvBars.unknown": "desconocido",
  "c.KpiCard.add_a_reading": "Añadir una lectura",
  "c.KpiCard.cadence": "Frecuencia",
  "c.KpiCard.daily": "Diaria",
  "c.KpiCard.delete": "Eliminar",
  "c.KpiCard.edit_or_delete": "Editar o eliminar",
  "c.KpiCard.history": "Historial ({readings})",
  "c.KpiCard.loading": "Cargando…",
  "c.KpiCard.monthly": "Mensual",
  "c.KpiCard.name": "Nombre",
  "c.KpiCard.no_reading_yet": "Aún no hay lecturas.",
  "c.KpiCard.no_readings": "Sin lecturas.",
  "c.KpiCard.none": "Ninguna",
  "c.KpiCard.note": "Nota",
  "c.KpiCard.observed": "Observado",
  "c.KpiCard.observed_on": "Observado el",
  "c.KpiCard.on": "{valueText} el {observed_on}",
  "c.KpiCard.quarterly": "Trimestral",
  "c.KpiCard.read": "· lectura {cadence}",
  "c.KpiCard.record": "Registrar",
  "c.KpiCard.red_line": "Línea roja",
  "c.KpiCard.save": "Guardar",
  "c.KpiCard.target": "Objetivo",
  "c.KpiCard.unit": "Unidad",
  "c.KpiCard.value": "Valor",
  "c.KpiCard.weekly": "Semanal",
  "c.KpiCard.which_way_is_good_cannot":
    "El sentido favorable ({v}) no se puede cambiar: volvería a colorear todas las lecturas anteriores.",
  "c.LilyGanttTable.critical_path": "ruta crítica",
  "c.LilyGanttTable.day": "Día",
  "c.LilyGanttTable.month": "Mes",
  "c.LilyGanttTable.no_dated_tasks_to_show":
    "No hay tareas con fechas que mostrar.",
  "c.LilyGanttTable.no_usable_dates": "Sin fechas utilizables:",
  "c.LilyGanttTable.task": "Tarea",
  "c.LilyGanttTable.that_range_is_too_long":
    "Ese rango es demasiado largo para una columna por {unit}; se muestra una por {unit2}.",
  "c.LilyGanttTable.time_scale": "Escala de tiempo",
  "c.LilyGanttTable.week": "Semana",
  "c.LilyKanbanTable.move_to": "Mover {title} a",
  "c.LilyKanbanTable.no_tasks_yet": "Aún no hay tareas.",
  "c.LilyKanbanTable.unknown_status": "Estado desconocido:",
  "c.LineRange.high": "Alto",
  "c.LineRange.low": "Bajo",
  "c.LineRange.planned_is_the_likely_cost":
    "El previsto es {money}, el costo probable. Deje un campo vacío para borrarlo.",
  "c.LineRange.save_range": "Guardar rango",
  "c.MilestoneCard.by": "Por",
  "c.MilestoneCard.by_hand_only": "Solo manualmente",
  "c.MilestoneCard.complete": "Completar",
  "c.MilestoneCard.completes": "Se completa",
  "c.MilestoneCard.done": "Hecho",
  "c.MilestoneCard.due": "Vence {due} · {kind}",
  "c.MilestoneCard.every_move_is_kept_in":
    "Cada cambio se conserva en el historial: la fecha anterior, la nueva, el motivo y quién lo hizo.",
  "c.MilestoneCard.forecast": "· previsión {early_finish}",
  "c.MilestoneCard.from": "Desde",
  "c.MilestoneCard.loading": "Cargando…",
  "c.MilestoneCard.mark_done": "Marcar como hecho",
  "c.MilestoneCard.move": "Mover",
  "c.MilestoneCard.never_rescheduled": "Nunca reprogramado.",
  "c.MilestoneCard.new_due_date": "Nueva fecha de vencimiento",
  "c.MilestoneCard.no_reason_given": "sin motivo indicado",
  "c.MilestoneCard.open": "Abierto",
  "c.MilestoneCard.pick_the_tasks_that_must":
    "Elija las tareas que deben completarse",
  "c.MilestoneCard.reason_required": "Motivo (obligatorio)",
  "c.MilestoneCard.reopen": "Reabrir",
  "c.MilestoneCard.reschedule": "Reprogramar",
  "c.MilestoneCard.save": "Guardar",
  "c.MilestoneCard.slip_history": "Historial de retrasos ({reschedules})",
  "c.MilestoneCard.tasks_it_waits_for": "Tareas que espera",
  "c.MilestoneCard.tasks_this_milestone_waits_for":
    "Tareas que espera este hito",
  "c.MilestoneCard.this_plan_has_no_tasks": "Este plan aún no tiene tareas.",
  "c.MilestoneCard.to": "A",
  "c.MilestoneCard.view_plan": "Ver plan",
  "c.MilestoneCard.when_all_those_tasks_are":
    "Cuando todas esas tareas estén hechas",
  "c.MilestoneCard.why": "Motivo",
  "c.NetworkDiagram.float": "{slice} → {slice2} · holgura {total_float}",
  "c.NetworkDiagram.no_dependencies": "Sin dependencias.",
  "c.NetworkDiagram.no_scheduled_tasks_to_draw":
    "Aún no hay tareas programadas que dibujar.",
  "c.NetworkDiagram.task_network_boxes_are_tasks":
    "Red de tareas: los recuadros son tareas y las flechas, dependencias",
  "c.NetworkDiagram.the_same_network_as_a": "La misma red en forma de lista",
  "c.ObjectiveCard.add_a_key_result": "Añadir un resultado clave",
  "c.ObjectiveCard.add_key_result": "Añadir resultado clave",
  "c.ObjectiveCard.check_in": "Registrar avance",
  "c.ObjectiveCard.confidence_0_100": "Confianza (0–100)",
  "c.ObjectiveCard.currency": "Moneda",
  "c.ObjectiveCard.currency_e_g_gbp": "Moneda (por ejemplo, EUR, JPY o GBP)",
  "c.ObjectiveCard.decrease": "Disminuir",
  "c.CommandPalette.cmd_dashboard": "Panel",
  "c.CommandPalette.cmd_escalations": "Escalamientos",
  "c.CommandPalette.cmd_my_work": "Mi trabajo",
  "c.CommandPalette.cmd_new_plan": "Nuevo plan",
  "c.CommandPalette.count_one": "{n} resultado",
  "c.CommandPalette.count_other": "{n} resultados",
  "c.CommandPalette.failed":
    "La búsqueda de planes no está disponible en este momento. Los comandos siguen funcionando.",
  "c.CommandPalette.kind_command": "Comando",
  "c.CommandPalette.kind_plan": "Plan",
  "c.CommandPalette.kind_recent": "Reciente",
  "c.CommandPalette.label": "Buscar planes y comandos",
  "c.CommandPalette.none": "No hay coincidencias",
  "c.CommandPalette.open": "Búsqueda y comandos",
  "c.CommandPalette.placeholder": "Escriba el nombre de un plan o un comando",
  "c.CommandPalette.title": "Búsqueda y comandos",
  "c.NavItems.portfolios": "Carteras",
  "c.NavItems.projects": "Proyectos",
  "c.NavItems.products": "Productos",
  "c.NavItems.programs": "Programas",
  "c.NavItems.practices": "Prácticas",
  "c.NavItems.processes": "Procesos",
  "c.NavItems.purposes": "Propósitos",
  "c.NavItems.pathways": "Itinerarios",
  "c.PasteTasks.check": "Comprobar con el servicio",
  "c.PasteTasks.clear": "Borrar",
  "c.PasteTasks.col_due": "Fecha límite",
  "c.PasteTasks.col_line": "Línea",
  "c.PasteTasks.col_notes": "Notas",
  "c.PasteTasks.col_status": "Estado",
  "c.PasteTasks.col_title": "Título",
  "c.PasteTasks.create_one": "Crear 1 tarea",
  "c.PasteTasks.create_other": "Crear {n} tareas",
  "c.PasteTasks.done": "Hecho",
  "c.PasteTasks.failed": "No se pudo pegar.",
  "c.PasteTasks.help":
    "Copie celdas de una hoja de cálculo y péguelas aquí: una tarea por línea, con las columnas Título, Fecha límite (aaaa-mm-dd) y Notas, en ese orden. La fecha pasa a ser la fecha límite de la tarea. Se omite una primera línea que nombre las columnas. Una fila con un problema se muestra con el motivo y se excluye.",
  "c.PasteTasks.label": "Filas pegadas",
  "c.PasteTasks.left_out": "Excluida: {why}",
  "c.PasteTasks.problem_beyond_limit": "más de {max} filas",
  "c.PasteTasks.problem_date_bad":
    "«{value}» no es una fecha; escríbala como aaaa-mm-dd",
  "c.PasteTasks.problem_no_title": "sin título",
  "c.PasteTasks.problem_notes_long": "las notas superan los {max} caracteres",
  "c.PasteTasks.problem_same_as": "igual que la línea {line}",
  "c.PasteTasks.problem_title_long": "el título supera los {max} caracteres",
  "c.PasteTasks.ready": "Lista",
  "c.PasteTasks.tally": "{good} listas, {left} excluidas",
  "c.PasteTasks.title": "Pegar filas de una hoja de cálculo",
  "c.PasteTasks.would": "Qué ocurriría",
  "c.ConfirmHost.cancel": "Cancelar",
  "c.ConfirmHost.ok": "Aceptar",
  "c.ConfirmHost.title": "Confirme, por favor",
  "c.ObjectiveCard.delete": "Eliminar",
  "c.ObjectiveCard.direction": "Sentido",
  "c.ObjectiveCard.edit_or_delete": "Editar o eliminar",
  "c.ObjectiveCard.edit_or_delete_this_objective":
    "Editar o eliminar este objetivo",
  "c.ObjectiveCard.from": "{valueText} (desde {start_value})",
  "c.ObjectiveCard.increase": "Aumentar",
  "c.ObjectiveCard.it_has_no_measurable_key":
    "— aún no tiene ningún resultado clave medible, por lo que no tiene puntuación (no es cero).",
  "c.ObjectiveCard.key_results_in_more_than":
    "Los resultados clave en más de una moneda se puntúan por separado; las monedas nunca se convierten.",
  "c.ObjectiveCard.last_check_in": "· último registro de avance {slice}",
  "c.ObjectiveCard.maintain": "Mantener",
  "c.ObjectiveCard.metric": "Métrica",
  "c.ObjectiveCard.no_key_results_yet": "Aún no hay resultados clave.",
  "c.ObjectiveCard.note": "Nota",
  "c.ObjectiveCard.number": "Número",
  "c.ObjectiveCard.percent": "Porcentaje",
  "c.ObjectiveCard.period": "Período",
  "c.ObjectiveCard.record": "Registrar",
  "c.ObjectiveCard.save": "Guardar",
  "c.ObjectiveCard.start": "Inicio",
  "c.ObjectiveCard.target": "Objetivo",
  "c.ObjectiveCard.the_baseline_metric_and_direction":
    "La línea base, la métrica y el sentido no se pueden cambiar: el progreso respecto a una línea base que se mueve no es progreso.",
  "c.ObjectiveCard.title": "Título",
  "c.ObjectiveCard.tolerance": "· tolerancia ±{tolerance}",
  "c.ObjectiveCard.tolerance_2": "Tolerancia",
  "c.ObjectiveCard.unit": "Unidad",
  "c.ObjectiveCard.value": "Valor",
  "c.ObjectiveCard.yes_no": "Sí / no",
  "c.PlanForm.organization": "organization:…",
  "c.PlanForm.parent_plan_pid_uuid": "pid del plan padre (UUID)",
  "c.PlanForm.proj_2026": "PROJ-2026",
  "c.RaciMatrix.complete": "completa",
  "c.RaciMatrix.gaps": "Carencias",
  "c.RaciMatrix.task": "Tarea",
  "c.RaciMatrix.who_does_what_raci_matrix":
    "Quién hace qué (matriz RACI): quién es Responsable, quién rinde cuentas (Accountable), a quién se Consulta y a quién se Informa en cada tarea",
  "c.RiskInputs.adds_cost": "Añade costo",
  "c.RiskInputs.by_business_days": "en (días hábiles)",
  "c.RiskInputs.currency": "Moneda",
  "c.RiskInputs.delays_task": "Retrasa la tarea",
  "c.RiskInputs.gbp": "EUR",
  "c.RiskInputs.none": "Ninguna",
  "c.RiskInputs.one_probability_covers_both_effects":
    "Una sola probabilidad cubre ambos efectos: un riesgo que retrasa una tarea y añade costo se produce una vez y hace ambas cosas. Deje una parte vacía para excluirla.",
  "c.RiskInputs.probability": "Probabilidad (%)",
  "c.RiskInputs.save": "Guardar",
  "c.RiskInputs.saving": "Guardando…",
  "c.RiskInputs.simulation_inputs": "Datos de entrada de la simulación",
  "c.SCurve.actual": "Real",
  "c.SCurve.date": "Fecha",
  "c.SCurve.earned": "Ganado",
  "c.SCurve.line_planned_value_to_earned":
    "Línea: valor planificado ({from} a {to}). ● valor ganado y ■ costo real, solo en los días registrados ({snapshots} hasta ahora). Discontinua: presupuesto a la finalización, {money}.",
  "c.SCurve.no_curve_to_draw": "No hay curva que dibujar.",
  "c.SCurve.not_recorded": "no registrado",
  "c.SCurve.planned": "Planificado",
  "c.SCurve.s_curve_planned_value_over":
    "Curva S: valor planificado a lo largo del tiempo, con el valor ganado y el costo real registrados",
  "c.SCurve.the_same_figures_as_a": "Las mismas cifras en una tabla",
  "c.VarianceTable.baselined": "Línea base",
  "c.VarianceTable.duration": "Δ duración",
  "c.VarianceTable.finish": "Δ fin",
  "c.VarianceTable.now": "Ahora",
  "c.VarianceTable.schedule_variance_against_the_baseli":
    "Variación del cronograma respecto a la línea base, en días hábiles",
  "c.VarianceTable.standing": "Situación",
  "c.VarianceTable.start": "Δ inicio",
  "c.VarianceTable.task": "Tarea",
  "c.VarianceTable.the_plan_s_finish_is": "El fin del plan no ha cambiado.",
  "c.VarianceTable.the_plan_s_own_finish":
    "El fin del propio plan no se puede comparar.",
  "p.announcements.announcements": "Anuncios",
  "p.announcements.announcements_ppm": "Anuncios — PPM",
  "p.announcements.expires_optional": "Caduca (opcional)",
  "p.announcements.important": "Importante",
  "p.announcements.info": "Información",
  "p.announcements.loading": "Cargando…",
  "p.announcements.message": "Mensaje",
  "p.announcements.nothing_has_been_announced": "No se ha anunciado nada.",
  "p.announcements.pin_to_the_top": "Fijar arriba",
  "p.announcements.pinned": "· Fijado",
  "p.announcements.publish": "Publicar",
  "p.announcements.publish_an_announcement": "Publicar un anuncio",
  "p.announcements.publishing_needs_the_announcements_p":
    "Para publicar se necesita el permiso de anuncios.",
  "p.announcements.retract": "Retirar",
  "p.announcements.severity": "Gravedad",
  "p.announcements.title": "Título",
  "p.announcements.urgent": "Urgente",
  "p.auditor.action": "Acción",
  "p.auditor.actor": "Actor",
  "p.auditor.actor_less_recorded_actions": "Acciones registradas sin actor:",
  "p.auditor.audit_trail": "Registro de auditoría",
  "p.auditor.bearer_sub": "sub del portador",
  "p.auditor.detail": "Detalle",
  "p.auditor.e_g_merged": "p. ej., merged",
  "p.auditor.entity": "Entidad",
  "p.auditor.evidence_pack_csv": "Paquete de evidencias (CSV)",
  "p.auditor.filter": "Filtrar",
  "p.auditor.findings": "Hallazgos",
  "p.auditor.no_audit_rows_match": "Ninguna fila de auditoría coincide.",
  "p.auditor.no_findings": "Sin hallazgos.",
  "p.auditor.ppm": "{t} — PPM",
  "p.auditor.rows_distinct_actors_actor_less":
    "{returned} filas · {distinct_actors} actores distintos · {actorless} sin actor",
  "p.auditor.rule": "Regla",
  "p.auditor.submitted_without_a_verified_token":
    "(enviado sin un token verificado)",
  "p.auditor.when": "Cuándo",
  "p.automations.action": "acción",
  "p.automations.action_2": "Acción",
  "p.automations.add_label": "añadir etiqueta",
  "p.automations.any": "cualquiera",
  "p.automations.assign": "asignar",
  "p.automations.assignee": "responsable",
  "p.automations.automation": "automatización",
  "p.automations.automations_ppm": "Automatizaciones — PPM",
  "p.automations.blank_every_plan": "vacío = todos los planes",
  "p.automations.cancel": "Cancelar",
  "p.automations.claim_based_so_a_deadline":
    "Basado en reservas, de modo que un plazo se activa exactamente una vez aunque el temporizador y un barrido manual coincidan.",
  "p.automations.configure_a_rule_once_as":
    "Configure una regla una vez; a medida que los elementos se mueven por el tablero, la plataforma la aplica. Las reglas nunca bloquean el movimiento que las activó: un fallo se registra como una ejecución fallida, no como una reversión.",
  "p.automations.delete": "Eliminar",
  "p.automations.detail": "Detalle",
  "p.automations.disable": "Desactivar",
  "p.automations.due": "Vencimiento",
  "p.automations.enable": "Activar",
  "p.automations.every_plan": "todos los planes",
  "p.automations.expire_review": "caducar revisión",
  "p.automations.from": "desde",
  "p.automations.in_calendar_days": "en días naturales",
  "p.automations.label": "etiqueta",
  "p.automations.message": "mensaje",
  "p.automations.name": "nombre",
  "p.automations.name_2": "Nombre",
  "p.automations.no_rules_configured": "No hay reglas configuradas.",
  "p.automations.nothing_has_fired_yet": "Todavía no se ha activado nada.",
  "p.automations.nothing_pending": "Nada pendiente.",
  "p.automations.notify": "notificar",
  "p.automations.off": "desactivada",
  "p.automations.on": "activada",
  "p.automations.operator": "operador",
  "p.automations.outcome": "Resultado",
  "p.automations.person_uuid": "person:<uuid>",
  "p.automations.plan_pid": "pid del plan",
  "p.automations.plan_stage_changed": "cambio de etapa del plan",
  "p.automations.recipient": "destinatario",
  "p.automations.review_submitted": "revisión enviada",
  "p.automations.rules": "Reglas",
  "p.automations.runs": "Ejecuciones",
  "p.automations.save_rule": "Guardar regla",
  "p.automations.schedule_action": "programar acción",
  "p.automations.scope": "Alcance",
  "p.automations.set_and_forget": "Configurar y olvidarse",
  "p.automations.set_task_status": "establecer estado de la tarea",
  "p.automations.source": "Origen",
  "p.automations.state": "Estado",
  "p.automations.status": "estado",
  "p.automations.subject": "Asunto",
  "p.automations.sweep_due_actions": "Procesar acciones vencidas",
  "p.automations.task_moved": "tarea movida",
  "p.automations.then": "entonces",
  "p.automations.to": "a",
  "p.automations.trigger": "desencadenante",
  "p.automations.trigger_2": "Desencadenante",
  "p.automations.when": "Cuándo",
  "p.automations.workflow_automation": "Automatización de flujos de trabajo",
  "p.board.amber": "ámbar",
  "p.board.amount": "Importe",
  "p.board.benefit_realizations": "beneficios materializados",
  "p.board.board_pack": "Dossier del consejo",
  "p.board.decision": "Decisión",
  "p.board.decisions_in_window": "Decisiones en el período",
  "p.board.green": "verde",
  "p.board.investment_decisions": "Decisiones de inversión",
  "p.board.kind": "Tipo",
  "p.board.milestones_completed": "hitos completados",
  "p.board.no_decisions_in_the_window": "No hay decisiones en el período.",
  "p.board.no_investment_decisions_recorded":
    "No hay decisiones de inversión registradas.",
  "p.board.no_snapshots_captured_yet": "Aún no se han tomado instantáneas.",
  "p.board.open_exposure": "Exposición abierta",
  "p.board.portfolios": "Carteras",
  "p.board.ppm": "{t} — PPM",
  "p.board.realized_in_window": "Materializado en el período:",
  "p.board.red_portfolios": "carteras en rojo",
  "p.board.subject": "Asunto",
  "p.board.take_snapshot_now": "Tomar instantánea ahora",
  "p.board.taken": "Tomada",
  "p.board.tranches_released": "tramos liberados",
  "p.board.trends": "Tendencias",
  "p.board.unattributed_events": "({unattributed_events} eventos sin atribuir)",
  "p.board.what": "Qué",
  "p.board.when": "Cuándo",
  "p.board.window": "período {from} → {to}",
  "p.board.work_items": "Elementos de trabajo",
  "p.calendar.all": "(todos)",
  "p.calendar.done": "· hecho",
  "p.calendar.kind": "Tipo",
  "p.calendar.no_milestones_match": "Ningún hito coincide.",
  "p.calendar.ppm": "{t} — PPM",
  "p.capacity.capacity_ppm": "Capacidad — PPM",
  "p.capacity.from": "desde",
  "p.capacity.hatched_extension_proposed_not_count":
    "extensión rayada: propuesta, no contabilizada",
  "p.capacity.legend": "Leyenda",
  "p.capacity.over": "excedido",
  "p.capacity.over_capacity_solid_bar_firm":
    "por encima de la capacidad (barra continua: reservas firmes; línea: capacidad vigente)",
  "p.capacity.proposed": "Propuesto",
  "p.capacity.to": "a",
  "p.capacity.window": "período: {from} → {to}",
  "p.capacity.within_capacity": "dentro de la capacidad",
  "p.capacity.within_capacity_but_committing_the":
    "dentro de la capacidad, pero confirmar las reservas propuestas la superaría",
  "p.ceo.all_announcements": "Todos los anuncios",
  "p.ceo.all_key_measures": "Todos los indicadores clave",
  "p.ceo.all_milestones": "Todos los hitos",
  "p.ceo.and_more": "y {more} más",
  "p.ceo.announcements": "Anuncios",
  "p.ceo.ceo_dashboard": "Panel de dirección general",
  "p.ceo.ceo_dashboard_ppm": "Panel de dirección general — PPM",
  "p.ceo.decision_log": "Registro de decisiones",
  "p.ceo.escalated_risks": "riesgos escalados",
  "p.ceo.every_kpi_is_on_target": "● Todos los KPI están en objetivo.",
  "p.ceo.executive_detail": "Detalle ejecutivo",
  "p.ceo.key_measures_needing_a_look":
    "Indicadores clave que requieren atención",
  "p.ceo.loading": "Cargando…",
  "p.ceo.milestones_to_watch": "Hitos a vigilar",
  "p.ceo.no_data_yet": "Aún no hay datos",
  "p.ceo.no_decisions_recorded_yet": "Aún no hay decisiones registradas.",
  "p.ceo.no_kpis_defined_yet": "Aún no hay KPI definidos.",
  "p.ceo.no_open_milestones": "No hay hitos abiertos.",
  "p.ceo.no_portfolio_is_over_budget":
    "● Ninguna cartera supera el presupuesto",
  "p.ceo.nothing_has_been_announced": "No se ha anunciado nada.",
  "p.ceo.over_budget": "▲ Por encima del presupuesto: {join}",
  "p.ceo.overdue_milestones": "{overdueMilestones} hitos vencidos",
  "p.ceo.portfolio_health": "Salud de la cartera",
  "p.ceo.recent_decisions": "Decisiones recientes",
  "p.ceo.red_amber_green": "■ {red} rojo · ▲ {amber} ámbar · ● {green} verde",
  "p.ceo.risk_and_money": "Riesgo y dinero",
  "p.ceo.risk_register": "Registro de riesgos",
  "p.compliance.compliance_risk_register":
    "Registro de riesgos de cumplimiento",
  "p.compliance.conformance_findings": "Hallazgos de conformidad",
  "p.compliance.detail": "Detalle",
  "p.compliance.exposure": "Exposición",
  "p.compliance.gate_review_recency_threshold_calend":
    "umbral de antigüedad de la revisión de fase: {review_days} días naturales",
  "p.compliance.item": "Elemento",
  "p.compliance.no_compliance_risks_recorded":
    "No hay riesgos de cumplimiento registrados.",
  "p.compliance.no_conformance_findings": "Sin hallazgos de conformidad.",
  "p.compliance.open_exposure": "{note} · exposición abierta {open_exposure}",
  "p.compliance.owner": "Propietario",
  "p.compliance.ppm": "{t} — PPM",
  "p.compliance.risk": "Riesgo",
  "p.compliance.rule": "Regla",
  "p.compliance.status": "Estado",
  "p.cost_rates.a_person": "Una persona",
  "p.cost_rates.a_role": "Un rol",
  "p.cost_rates.add_a_rate_table": "Añadir una tabla de tarifas",
  "p.cost_rates.add_rate": "Añadir tarifa",
  "p.cost_rates.add_table": "Añadir tabla",
  "p.cost_rates.cost_rates": "Tarifas de costo",
  "p.cost_rates.cost_rates_are_finance_data":
    "Las tarifas de costo son datos financieros. Usted no tiene acceso a ellas.",
  "p.cost_rates.cost_rates_ppm": "Tarifas de costo — PPM",
  "p.cost_rates.currency": "Moneda",
  "p.cost_rates.effective_from": "Vigente desde",
  "p.cost_rates.for": "Para",
  "p.cost_rates.from": "Desde",
  "p.cost_rates.name": "Nombre",
  "p.cost_rates.no_rate_tables_yet": "Aún no hay tablas de tarifas.",
  "p.cost_rates.no_rate_yet_work_by":
    "Aún no hay tarifa: el trabajo de este {v} no se valora.",
  "p.cost_rates.per_hour": "Por hora",
  "p.cost_rates.person_person_reference": "Persona (referencia person:…)",
  "p.cost_rates.role": "Rol",
  "p.cost_risk.band_for_lines_with_no":
    "Banda para líneas sin estimación (± %)",
  "p.cost_risk.distribution": "Distribución",
  "p.cost_risk.iterations": "Iteraciones",
  "p.cost_risk.pert": "PERT",
  "p.cost_risk.portfolio_cost_risk": "Riesgo de costo de la cartera",
  "p.cost_risk.portfolio_cost_risk_ppm": "Riesgo de costo de la cartera — PPM",
  "p.cost_risk.resources": "Recursos",
  "p.cost_risk.run": "Ejecutar",
  "p.cost_risk.running": "Ejecutando…",
  "p.cost_risk.seed": "Semilla",
  "p.cost_risk.shows_people_instead": "muestra personas en su lugar.",
  "p.cost_risk.triangular": "Triangular",
  "p.cost_risk.what_all_your_plans_together":
    "Lo que podrían costar todos sus planes juntos, según el rango de cada línea de presupuesto y los riesgos que añaden costo. Cada moneda va por separado; nunca se suman.",
  "p.dashboard.dashboard_ppm": "Panel — PPM",
  "p.display.display": "Visualización",
  "p.display.display_ppm": "Visualización — PPM",
  "p.display.dyslexia_friendly_font": "Fuente adaptada a la dislexia",
  "p.display.go_back_to_the_default":
    "Volver a la configuración predeterminada de mi rol",
  "p.display.how_the_app_looks_and":
    "Cómo se ve y se lee la aplicación. Esto nunca cambia lo que usted puede hacer.",
  "p.display.keeps_the_everyday_pages_in":
    "Mantiene delante las páginas de uso diario (Mi trabajo, Planes, Tablero, Calendario, Recorrido) y coloca el resto tras un enlace «Más». Todas las páginas siguen ahí y siguen funcionando.",
  "p.display.sign_in_to_keep_these":
    "Inicie sesión para conservar esta configuración en su cuenta además de en este dispositivo.",
  "p.display.simple_view": "Vista simple",
  "p.display.this_is_the_default_for":
    "Esta es la configuración predeterminada de su rol: {v}.",
  "p.display.use_simple_view": "Usar la vista simple",
  "p.display.sections_title": "Secciones de la barra superior",
  "p.display.sections_help":
    "Desmarque una sección para ocultarla de la barra superior. Sigue disponible en su dirección y puede volver a mostrarla aquí. No se guarda nada hasta que oculte una, así que le llegarán las secciones que se añadan más adelante.",
  "p.display.sections_legend": "Mostrar estas secciones",
  "p.display.sections_hidden_count_one": "{n} sección oculta",
  "p.display.sections_hidden_count_other": "{n} secciones ocultas",
  "p.display.sections_none_hidden": "No hay ninguna sección oculta.",
  "p.display.sections_show_all": "Mostrar todas las secciones",
  "p.display.week_start_title": "Primer día de la semana",
  "p.display.week_start_help":
    "Dónde empiezan los calendarios cada semana. De forma predeterminada sigue el idioma elegido; los números de semana siguen siendo semanas ISO, que empiezan el lunes.",
  "p.display.week_start_label": "Las semanas empiezan el",
  "p.display.week_start_locale": "El día habitual del idioma",
  "p.display.week_start_mon": "Lunes",
  "p.display.week_start_sun": "Domingo",
  "p.display.week_start_sat": "Sábado",
  "p.display.use_the_dyslexia_friendly_font":
    "Usar la fuente adaptada a la dislexia",
  "p.display.uses_the_opendyslexic_typeface_with":
    "Usa la tipografía OpenDyslexic, con más espacio entre letras, palabras y líneas.",
  "p.display.you_chose_this": "Usted eligió esto.",
  "p.documents.controlled_documents": "Documentos controlados",
  "p.documents.create_document": "Crear documento",
  "p.documents.document": "Documento",
  "p.documents.documents": "Documentos",
  "p.documents.documents_ppm": "Documentos — PPM",
  "p.documents.first_draft": "Primer borrador",
  "p.documents.in_force": "Vigente",
  "p.documents.loading": "Cargando…",
  "p.documents.new_document": "Nuevo documento",
  "p.documents.no_documents_yet": "Aún no hay documentos.",
  "p.documents.pid.approver_s_id": "Id del aprobador",
  "p.documents.pid.date": "Fecha",
  "p.documents.pid.document": "Documento",
  "p.documents.pid.documents": "← Documentos",
  "p.documents.pid.look_up": "Consultar",
  "p.documents.pid.nothing_was_in_force_on": "No había nada vigente el {on}.",
  "p.documents.pid.person_id": "id de persona",
  "p.documents.pid.save_draft": "Guardar borrador",
  "p.documents.pid.send_back": "Devolver",
  "p.documents.pid.text": "Texto",
  "p.documents.pid.this_version_s_text_is":
    "El texto de esta versión es definitivo. Cualquier cambio va en el siguiente borrador.",
  "p.documents.pid.version": "Versión {version}",
  "p.documents.pid.version_was_in_force_on":
    "La versión {version} estaba vigente el {on}.",
  "p.documents.pid.versions": "Versiones",
  "p.documents.pid.what_changed": "Qué cambió",
  "p.documents.pid.what_was_in_force_on": "Qué estaba vigente en una fecha",
  "p.documents.policies_and_procedures_a_new":
    "Políticas y procedimientos. Una nueva versión se revisa y aprueba antes de entrar en vigor, y una versión vigente nunca cambia.",
  "p.documents.title": "Título",
  "p.documents.versions": "Versiones",
  "p.engineering.assignee": "Responsable",
  "p.engineering.blocked_calendar_days": "Bloqueado (días naturales)",
  "p.engineering.blocked_work": "Trabajo bloqueado",
  "p.engineering.declared_cause_failure_rate":
    "tasa de fallos con causa declarada",
  "p.engineering.delivery_links": "Vínculos de entrega",
  "p.engineering.deploys_mo": "despliegues ({window_months} meses)",
  "p.engineering.devops_metrics": "Métricas de DevOps",
  "p.engineering.environment": "Entorno",
  "p.engineering.incidents": "incidentes",
  "p.engineering.incidents_unresolved_counted_never_t":
    "{unresolved_incidents} incidentes sin resolver (contados, nunca cronometrados).",
  "p.engineering.item": "Elemento",
  "p.engineering.items_carry_no_moscow_tag":
    "{untagged} elementos no tienen etiqueta MoSCoW.",
  "p.engineering.median_recovery_h": "recuperación mediana (h)",
  "p.engineering.moscow_scope": "Alcance MoSCoW",
  "p.engineering.no_deploy_events_ingested_yet":
    "Aún no se han recibido eventos de despliegue.",
  "p.engineering.no_externally_tracked_items":
    "No hay elementos con seguimiento externo.",
  "p.engineering.nothing_blocked_ship_it": "Nada bloqueado. A entregar.",
  "p.engineering.ppm": "{t} — PPM",
  "p.engineering.releases": "Versiones publicadas",
  "p.engineering.task": "Tarea",
  "p.engineering.trackers": "Herramientas de seguimiento",
  "p.engineering.untracked": "Sin seguimiento:",
  "p.engineering.version": "Versión",
  "p.engineering.when": "Cuándo",
  "p.executive.actor": "Actor",
  "p.executive.aligned": "Alineado",
  "p.executive.as_of": "a fecha de {as_of} · {derivation}",
  "p.executive.benefits": "Beneficios",
  "p.executive.benefits_realization": "Materialización de beneficios",
  "p.executive.calendar_days_since_update":
    "Días naturales desde la actualización",
  "p.executive.collection": "Colección",
  "p.executive.currency": "Moneda",
  "p.executive.decision": "Decisión",
  "p.executive.decision_log": "Registro de decisiones",
  "p.executive.escalated_risks": "Riesgos escalados",
  "p.executive.kind": "Tipo",
  "p.executive.largest_unaligned_items": "Mayores elementos no alineados",
  "p.executive.members": "Miembros",
  "p.executive.no_decisions_recorded_yet": "Aún no hay decisiones registradas.",
  "p.executive.no_target": "sin objetivo",
  "p.executive.non_financial_only": "solo no financieros ({non_financial})",
  "p.executive.open_exposure": "Exposición abierta",
  "p.executive.overdue_milestones": "Hitos vencidos",
  "p.executive.overrun_currencies": "Monedas con sobrecosto",
  "p.executive.planned_with_no_objective": "planificado sin objetivo",
  "p.executive.portfolio": "Cartera",
  "p.executive.portfolio_health": "Salud de la cartera",
  "p.executive.ppm": "{t} — PPM",
  "p.executive.realization": "Materialización",
  "p.executive.realized": "Materializado",
  "p.executive.status": "Estado",
  "p.executive.strategic_alignment": "Alineación estratégica",
  "p.executive.subject": "Asunto",
  "p.executive.target": "Objetivo",
  "p.executive.total": "Total",
  "p.executive.unaligned": "No alineado",
  "p.executive.unaligned_spend": "Gasto no alineado",
  "p.executive.when": "Cuándo",
  "p.financials.actual": "Real",
  "p.financials.as_of": "a fecha de {as_of} · {note}",
  "p.financials.currency": "Moneda",
  "p.financials.currency_exposure": "Exposición por moneda",
  "p.financials.lines": "Líneas",
  "p.financials.no_budget_lines_recorded_yet":
    "Aún no hay líneas de presupuesto registradas.",
  "p.financials.planned": "Previsto",
  "p.financials.ppm": "{t} — PPM",
  "p.financials.remaining": "Restante",
  "p.financials.work_items": "Elementos de trabajo",
  "p.gantt.main_x": "{t} — Main X",
  "p.ideas.convert_target": "Destino de conversión",
  "p.ideas.ideas_ppm": "Ideas — PPM",
  "p.kpis.all": "Todos",
  "p.kpis.blank_for_the_whole_portfolio": "vacío para toda la cartera",
  "p.kpis.daily": "Diaria",
  "p.kpis.define": "Definir",
  "p.kpis.define_a_key_measure_kpi": "Definir un indicador clave (KPI)",
  "p.kpis.higher_is_better": "Cuanto más alto, mejor",
  "p.kpis.key_measures_kpis": "Indicadores clave (KPI)",
  "p.kpis.kpis_by_status": "KPI por estado",
  "p.kpis.kpis_ppm": "KPI — PPM",
  "p.kpis.loading": "Cargando…",
  "p.kpis.lower_is_better": "Cuanto más bajo, mejor",
  "p.kpis.monthly": "Mensual",
  "p.kpis.name": "Nombre",
  "p.kpis.no_data_yet": "Aún no hay datos",
  "p.kpis.not_set": "Sin definir",
  "p.kpis.off_target": "Fuera de objetivo",
  "p.kpis.on_target": "En objetivo",
  "p.kpis.percent_defects_gbp": "porcentaje, defectos, EUR…",
  "p.kpis.plan_optional_pid": "Plan (pid opcional)",
  "p.kpis.quarterly": "Trimestral",
  "p.kpis.read_how_often": "Frecuencia de lectura",
  "p.kpis.red_line": "Línea roja",
  "p.kpis.show": "Mostrar",
  "p.kpis.something_you_track_over_time":
    "Algo que usted sigue a lo largo del tiempo frente a un objetivo, para ver pronto si va por buen camino.",
  "p.kpis.standing_measures_green_is_at":
    "Indicadores permanentes. Verde es igual o mejor que el objetivo; rojo es igual o peor que la línea roja; ámbar está en medio. Un KPI que nadie ha leído aún muestra «Aún no hay datos».",
  "p.kpis.target": "Objetivo",
  "p.kpis.the_red_line_must_be":
    "La línea roja debe estar del lado desfavorable del objetivo. Escale los valores a números enteros (puntos básicos, céntimos).",
  "p.kpis.unit": "Unidad",
  "p.kpis.watch": "Vigilar",
  "p.kpis.weekly": "Semanal",
  "p.kpis.which_way_is_good": "Sentido favorable",
  "p.layout.display": "Visualización",
  "p.layout.more": "Más",
  "p.layout.simple_view": "Vista simple: {v}",
  "p.lifecycle.as_of": "a fecha de {as_of}",
  "p.lifecycle.in_an_unrecognized_phase": "en una fase no reconocida",
  "p.lifecycle.lifecycle": "Ciclo de vida",
  "p.lifecycle.lifecycle_ppm": "Ciclo de vida — PPM",
  "p.lifecycle.live": "Activos",
  "p.lifecycle.live_items_one":
    "{total} elemento activo · umbral de estancamiento {stall}",
  "p.lifecycle.live_items_other":
    "{total} elementos activos · umbral de estancamiento {stall}",
  "p.lifecycle.phase": "Fase",
  "p.lifecycle.share": "Proporción",
  "p.lifecycle.stalled": "Estancados",
  "p.lifecycle.where_everything_sits_across_the":
    "Dónde se encuentra cada elemento a lo largo del ciclo de vida del reto, para que la siguiente fase nunca sea una sorpresa. Estancados cuenta los elementos que llevan en una fase más tiempo que el umbral.",
  "p.me.calendar.add": "Añadir",
  "p.me.calendar.at_least_one_weekday_must":
    "Al menos un día de la semana debe ser laborable.",
  "p.me.calendar.from": "Desde",
  "p.me.calendar.loading": "Cargando…",
  "p.me.calendar.most_i_can_commit": "Máximo que puedo dedicar (%)",
  "p.me.calendar.my_calendar": "Mi calendario",
  "p.me.calendar.my_calendar_ppm": "Mi calendario — PPM",
  "p.me.calendar.my_days_off": "Mis días libres",
  "p.me.calendar.my_work": "← Mi trabajo",
  "p.me.calendar.none_set": "Ninguno definido.",
  "p.me.calendar.note": "Nota",
  "p.me.calendar.part_time_spells_and_leave":
    "Períodos a tiempo parcial y ausencias",
  "p.me.calendar.remove": "Eliminar",
  "p.me.calendar.save": "Guardar",
  "p.me.calendar.saved": "Guardado.",
  "p.me.calendar.the_days_i_work": "Los días que trabajo",
  "p.me.calendar.the_most_of_your_time":
    "La mayor parte de su tiempo que puede dedicar entre dos fechas: 60 para un período a tiempo parcial, 0 para una ausencia. Fuera de esas fechas es 100. La capacidad se calcula día a día, por lo que un período corto nunca se diluye en un promedio.",
  "p.me.calendar.to": "{from} a {to}: {max_percent}%{v}",
  "p.me.calendar.to_2": "Hasta",
  "p.me.due": "· vence {due_date}",
  "p.me.in_plans_you_cannot_read":
    "· {omitted_unreadable} en planes que usted no puede leer",
  "p.me.loading": "Cargando…",
  "p.me.my_calendar": "Mi calendario",
  "p.me.my_work": "Mi trabajo",
  "p.me.my_work_ppm": "Mi trabajo — PPM",
  "p.me.nothing_assigned_to_you_enjoy": "No tiene nada asignado. Disfrútelo.",
  "p.me.open": "{today} ({tz}) · {total} abiertas",
  "p.me.showing_the_first_500": "· se muestran las primeras 500",
  "p.me.the_days_you_work_so":
    "— los días que trabaja, para que sus fechas sean realistas.",
  "p.me.your_sign_in_identity_cannot":
    "Su identidad de inicio de sesión no se puede asociar con asignaciones de tareas, por lo que aún no se puede mostrar nada aquí.",
  "p.milestones.a_milestone_is_a_dated":
    "Un hito es un compromiso con fecha. Vincúlelo a las tareas que espera y se alcanzará cuando estén hechas, con una previsión a partir del cronograma; si lo deja sin vincular, una persona lo marca como hecho.",
  "p.milestones.achieved": "Alcanzado",
  "p.milestones.all": "Todos",
  "p.milestones.due_soon": "Vence pronto",
  "p.milestones.forecast": "Previsión:",
  "p.milestones.late": "Atrasado",
  "p.milestones.loading": "Cargando…",
  "p.milestones.milestones": "Hitos",
  "p.milestones.milestones_by_status": "Hitos por estado",
  "p.milestones.milestones_ppm": "Hitos — PPM",
  "p.milestones.show": "Mostrar",
  "p.milestones.upcoming": "Próximos",
  "p.objectives.objectives_ppm": "Objetivos — PPM",
  "p.objectives.period_e_g_2026_h2": "Período (p. ej., 2026-H2)",
  "p.okrs.2026_q4_blank_for_all": "2026-Q4 (vacío para todos)",
  "p.okrs.a_goal_with_a_few":
    "Una meta, con unos pocos resultados medibles que muestran si se ha alcanzado.",
  "p.okrs.create": "Crear",
  "p.okrs.goals_and_key_results_okrs": "Metas y resultados clave (OKR)",
  "p.okrs.loading": "Cargando…",
  "p.okrs.new_objective": "Nuevo objetivo",
  "p.okrs.objectives_and_key_results_scores":
    "Objetivos y resultados clave. Las puntuaciones se calculan a partir de los registros de avance cada vez que los consulta; no se almacena nada que pueda quedar desactualizado.",
  "p.okrs.okrs_ppm": "OKR — PPM",
  "p.okrs.period": "Período",
  "p.okrs.show": "Mostrar",
  "p.okrs.title": "Título",
  "p.onboarding.a_role_by_role_first":
    "Un recorrido de la primera hora, rol por rol. Cada paso indica la página a la que lleva. Es un orden sugerido, no una afirmación medida del tipo «será productivo en N minutos»: nadie lo ha cronometrado.",
  "p.onboarding.onboarding_ppm": "Incorporación — PPM",
  "p.onboarding.operator_onboarding": "Incorporación de operadores",
  "p.plans.main_x": "{t} — Main X",
  "p.plans.merge.ppm": "{t} — PPM",
  "p.plans.new.main_x": "{t} — Main X",
  "p.plans.pid.board.action": "acción",
  "p.plans.pid.board.add_note": "Añadir nota",
  "p.plans.pid.board.add_sprint": "Añadir sprint",
  "p.plans.pid.board.add_task": "Añadir tarea",
  "p.plans.pid.board.board": "Tablero",
  "p.plans.pid.board.burndown": "Burndown — {name}",
  "p.plans.pid.board.convert_to_task": "Convertir en tarea",
  "p.plans.pid.board.created_moves_blocked_now":
    "{length} creadas · {length2} movimientos · {length3} bloqueadas ahora",
  "p.plans.pid.board.date": "Fecha",
  "p.plans.pid.board.feedback": "comentarios",
  "p.plans.pid.board.improve": "a mejorar",
  "p.plans.pid.board.new_task_title": "Título de la nueva tarea",
  "p.plans.pid.board.no_notes_for_this_sprint":
    "Aún no hay notas para este sprint.",
  "p.plans.pid.board.none": "(ninguno)",
  "p.plans.pid.board.note": "Nota",
  "p.plans.pid.board.note_body": "Texto de la nota",
  "p.plans.pid.board.note_category": "Categoría de la nota",
  "p.plans.pid.board.points_done": "Puntos completados",
  "p.plans.pid.board.ppm": "{t} — PPM",
  "p.plans.pid.board.pts": "ptos.",
  "p.plans.pid.board.remaining_of": "Restantes de {total_tasks}",
  "p.plans.pid.board.retro_feedback_notes":
    "Notas de retrospectiva / comentarios",
  "p.plans.pid.board.sprint": "Sprint",
  "p.plans.pid.board.sprint_end": "Fin del sprint",
  "p.plans.pid.board.sprint_name": "Nombre del sprint",
  "p.plans.pid.board.sprint_start": "Inicio del sprint",
  "p.plans.pid.board.sprints": "Sprints",
  "p.plans.pid.board.standup_last_24h": "Standup (últimas 24 h)",
  "p.plans.pid.board.story_points": "Puntos de historia",
  "p.plans.pid.board.task_created": "→ tarea creada",
  "p.plans.pid.board.tasks_done": "Tareas completadas",
  "p.plans.pid.board.unpointed": "Sin puntos",
  "p.plans.pid.board.velocity": "Velocidad",
  "p.plans.pid.board.went_well": "salió bien",
  "p.plans.pid.calendar.at_least_one_weekday_must":
    "Al menos un día de la semana debe ser laborable.",
  "p.plans.pid.calendar.critical_path": "← Ruta crítica",
  "p.plans.pid.calendar.holidays": "Festivos",
  "p.plans.pid.calendar.loading": "Cargando…",
  "p.plans.pid.calendar.my_calendar": "Mi calendario",
  "p.plans.pid.calendar.save_calendar": "Guardar calendario",
  "p.plans.pid.calendar.saved": "Guardado.",
  "p.plans.pid.calendar.their_work_then_happens_only":
    "); su trabajo solo se realiza entonces en los días laborables de ambos calendarios.",
  "p.plans.pid.calendar.working_calendar": "Calendario laboral",
  "p.plans.pid.calendar.working_calendar_ppm": "Calendario laboral — PPM",
  "p.plans.pid.calendar.working_weekdays": "Días laborables de la semana",
  "p.plans.pid.cost_risk.band_for_lines_with_no":
    "Banda para líneas sin estimación (± %)",
  "p.plans.pid.cost_risk.cost_and_schedule_together":
    "Costo y cronograma juntos",
  "p.plans.pid.cost_risk.cost_risk": "Riesgo de costo",
  "p.plans.pid.cost_risk.cost_risk_ppm": "Riesgo de costo — PPM",
  "p.plans.pid.cost_risk.distribution": "Distribución",
  "p.plans.pid.cost_risk.iterations": "Iteraciones",
  "p.plans.pid.cost_risk.many_what_if_runs_of":
    "Muchas simulaciones hipotéticas del presupuesto muestran el rango de costos finales y la probabilidad de cada uno.",
  "p.plans.pid.cost_risk.pert": "PERT",
  "p.plans.pid.cost_risk.plan": "← Plan",
  "p.plans.pid.cost_risk.run": "Ejecutar",
  "p.plans.pid.cost_risk.running": "Ejecutando…",
  "p.plans.pid.cost_risk.seed": "Semilla",
  "p.plans.pid.cost_risk.triangular": "Triangular",
  "p.plans.pid.cost_risk.whole_portfolio": "Toda la cartera",
  "p.plans.pid.costs.a_recorded_cost_keeps_the":
    "Un costo registrado conserva la tarifa con la que se valoró; un cambio posterior de tarifa nunca lo modifica.",
  "p.plans.pid.costs.add_an_expense": "Añadir un gasto",
  "p.plans.pid.costs.add_expense": "Añadir gasto",
  "p.plans.pid.costs.all_at_the_end": "Todo al final",
  "p.plans.pid.costs.all_at_the_start": "Todo al inicio",
  "p.plans.pid.costs.amount": "Importe",
  "p.plans.pid.costs.an_expense_of_so_it":
    "▲ Un gasto de {money} {reason}, por lo que no figura en los períodos.",
  "p.plans.pid.costs.choose": "Elegir…",
  "p.plans.pid.costs.cost_rates": "Tarifas de costo",
  "p.plans.pid.costs.costs": "Costos",
  "p.plans.pid.costs.costs_and_rates_are_finance":
    "Los costos y las tarifas son datos financieros. Usted no tiene acceso a ellos.",
  "p.plans.pid.costs.costs_ppm": "Costos — PPM",
  "p.plans.pid.costs.currency": "Moneda",
  "p.plans.pid.costs.description": "Descripción",
  "p.plans.pid.costs.evenly_across_the_task":
    "Repartido de forma uniforme en la tarea",
  "p.plans.pid.costs.falls": "Se imputa",
  "p.plans.pid.costs.fixed_cost": "Costo fijo",
  "p.plans.pid.costs.kind": "Tipo",
  "p.plans.pid.costs.material": "Material",
  "p.plans.pid.costs.no_time_has_been_costed":
    "Aún no se ha valorado ningún tiempo.",
  "p.plans.pid.costs.nothing_could_be_costed_yet":
    "Aún no se pudo valorar nada: añada tarifas para las personas que hacen el trabajo o añada un gasto.",
  "p.plans.pid.costs.per_period": "{currency}, por período",
  "p.plans.pid.costs.plan": "← Plan",
  "p.plans.pid.costs.planned_cost": "Costo previsto",
  "p.plans.pid.costs.record_labour_costs": "Registrar costos de mano de obra",
  "p.plans.pid.costs.recorded_labour_cost": "Costo de mano de obra registrado",
  "p.plans.pid.costs.so_its_labour_is_not":
    "▲ {title} ({hoursText}): {reason}, por lo que su mano de obra no se valora.",
  "p.plans.pid.costs.task": "Tarea",
  "p.plans.pid.costs.time_entries_are_not_costed":
    "▲ {entries} registros de tiempo ({hoursText}) no se valoran: aún no registrados o sin tarifa. Su costo es desconocido, no cero.",
  "p.plans.pid.costs.total": "Total",
  "p.plans.pid.costs.work": "Trabajo",
  "p.plans.pid.critical_path.a_baseline_is_a_numbered":
    "Una línea base es una instantánea numerada del cronograma que se conserva para siempre: volver a establecerla crea la versión siguiente y nunca sustituye a una existente.",
  "p.plans.pid.critical_path.baselines": "Líneas base",
  "p.plans.pid.critical_path.business_days": "días hábiles",
  "p.plans.pid.critical_path.compare_with": "Comparar con",
  "p.plans.pid.critical_path.constraint": "Restricción",
  "p.plans.pid.critical_path.constraint_date": "Fecha de la restricción",
  "p.plans.pid.critical_path.critical_chains":
    "{count} cadenas críticas{more}.",
  "p.plans.pid.critical_path.critical_path": "Ruta crítica",
  "p.plans.pid.critical_path.critical_path_ppm": "Ruta crítica — PPM",
  "p.plans.pid.critical_path.deadline_a_goal_it_shows":
    "Fecha límite (una meta: se muestra como holgura negativa y nunca mueve la tarea)",
  "p.plans.pid.critical_path.duration_business_days_0_is":
    "Duración (días hábiles; 0 es un hito)",
  "p.plans.pid.critical_path.finish_no_earlier_than": "Terminar no antes de",
  "p.plans.pid.critical_path.finish_no_later_than": "Terminar no después de",
  "p.plans.pid.critical_path.gantt": "Gantt",
  "p.plans.pid.critical_path.loading": "Cargando…",
  "p.plans.pid.critical_path.must_finish_on_overrides_logic":
    "Debe terminar el (prevalece sobre la lógica)",
  "p.plans.pid.critical_path.must_start_on_overrides_logic":
    "Debe comenzar el (prevalece sobre la lógica)",
  "p.plans.pid.critical_path.near_critical_within": "Casi crítica dentro de",
  "p.plans.pid.critical_path.network": "Red",
  "p.plans.pid.critical_path.no_baseline_yet_so_nothing":
    "Aún no hay línea base, por lo que no se puede decir nada sobre retrasos.",
  "p.plans.pid.critical_path.none_as_soon_as_possible":
    "Ninguna (lo antes posible)",
  "p.plans.pid.critical_path.not_scheduled": "Sin programar",
  "p.plans.pid.critical_path.optimistic_business_days":
    "Optimista (días hábiles)",
  "p.plans.pid.critical_path.pessimistic_business_days":
    "Pesimista (días hábiles)",
  "p.plans.pid.critical_path.plan": "← Plan",
  "p.plans.pid.critical_path.planned_start": "Inicio previsto",
  "p.plans.pid.critical_path.range_for_schedule_risk":
    "Rango para el riesgo de cronograma",
  "p.plans.pid.critical_path.range_saved": "Rango guardado.",
  "p.plans.pid.critical_path.reason_required_this_overrides_the":
    "Motivo (obligatorio: esto prevalece sobre la lógica del cronograma)",
  "p.plans.pid.critical_path.save_range": "Guardar rango",
  "p.plans.pid.critical_path.save_schedule": "Guardar cronograma",
  "p.plans.pid.critical_path.saved_moved": "Guardado. Se movió:",
  "p.plans.pid.critical_path.saved_nothing_else_moved":
    "Guardado. No se movió nada más.",
  "p.plans.pid.critical_path.saving": "Guardando…",
  "p.plans.pid.critical_path.show_the_baseline_under_each":
    "Mostrar la línea base bajo cada barra",
  "p.plans.pid.critical_path.start_no_earlier_than": "Comenzar no antes de",
  "p.plans.pid.critical_path.start_no_later_than": "Comenzar no después de",
  "p.plans.pid.critical_path.table": "Tabla",
  "p.plans.pid.critical_path.take_baseline": "Establecer línea base",
  "p.plans.pid.critical_path.task": "Tarea",
  "p.plans.pid.critical_path.the_duration_above_is_the":
    "{taskRangeText}. La duración indicada arriba es el valor probable.",
  "p.plans.pid.critical_path.the_longest_chain_of_tasks":
    "La cadena de tareas más larga, que fija la fecha de fin. Un retraso en cualquiera de sus tareas retrasa todo el plan.",
  "p.plans.pid.critical_path.these_assignees_share_no_business":
    "Estos responsables no comparten ningún día hábil con el calendario del plan, por lo que se usó el calendario del plan para ellos: {join}.",
  "p.plans.pid.critical_path.this_network_has_tasks_too":
    "Esta red tiene {length} tareas, demasiadas para dibujarla de forma útil. La tabla siguiente dice lo mismo.",
  "p.plans.pid.critical_path.untitled": "(sin título)",
  "p.plans.pid.critical_path.version": "Versión {version}: {reason} ({slice})",
  "p.plans.pid.critical_path.view": "Vista",
  "p.plans.pid.critical_path.why_a_baseline_now":
    "¿Por qué una línea base ahora?",
  "p.plans.pid.critical_path.working_calendar": "Calendario laboral",
  "p.plans.pid.earned_value.a_percent_i_enter_flagged":
    "Un porcentaje que introduzco (marcado)",
  "p.plans.pid.earned_value.actual_cost": "Costo real",
  "p.plans.pid.earned_value.against_the_budget_vac":
    "Frente al presupuesto (VAC)",
  "p.plans.pid.earned_value.baseline_version":
    "versión de línea base {baseline_version}",
  "p.plans.pid.earned_value.budget_at_completion":
    "Presupuesto a la finalización",
  "p.plans.pid.earned_value.budget_lines_in_have_no":
    "Las líneas de presupuesto en {join} no tienen línea base, por lo que no forman parte de ninguna cifra anterior y nunca se suman a otra moneda.",
  "p.plans.pid.earned_value.by_story_points_when_finished":
    "por puntos de historia, al terminar",
  "p.plans.pid.earned_value.cancel": "Cancelar",
  "p.plans.pid.earned_value.change": "Cambiar",
  "p.plans.pid.earned_value.complete": "{round} % completado",
  "p.plans.pid.earned_value.cost_index": "Índice de costo (",
  "p.plans.pid.earned_value.cost_is": "El costo está {word}.",
  "p.plans.pid.earned_value.cost_variance_ev_ac":
    "Variación de costo (EV − AC)",
  "p.plans.pid.earned_value.count_a_task_another_way":
    "Contar una tarea de otra forma",
  "p.plans.pid.earned_value.count_tasks": "Contar tareas",
  "p.plans.pid.earned_value.critical_path": "Ruta crítica",
  "p.plans.pid.earned_value.earned_value": "Valor ganado",
  "p.plans.pid.earned_value.earned_value_figures":
    "Cifras de valor ganado en {currency}",
  "p.plans.pid.earned_value.earned_value_ppm": "Valor ganado — PPM",
  "p.plans.pid.earned_value.final_cost_eac": "Costo final (EAC)",
  "p.plans.pid.earned_value.finished_only": "solo terminadas",
  "p.plans.pid.earned_value.forecast": "Previsión",
  "p.plans.pid.earned_value.forecast_final_cost":
    "· costo final previsto {money}",
  "p.plans.pid.earned_value.forecasts_of_the_final_cost":
    "Previsiones del costo final en {currency}, cada una según su propia fórmula",
  "p.plans.pid.earned_value.governance": "← Gobernanza",
  "p.plans.pid.earned_value.half_for_started": "mitad si se ha iniciado",
  "p.plans.pid.earned_value.is_the_work_finished_worth":
    "¿El trabajo terminado vale lo previsto a estas alturas y cuánto costó? Un índice superior a 1,00 es mejor de lo previsto.",
  "p.plans.pid.earned_value.leading_with_that_one_because":
    "Se destaca esa porque {why}. Las demás se muestran al lado: cuál es la correcta depende de si los sobrecostos pasados continuarán.",
  "p.plans.pid.earned_value.loading": "Cargando…",
  "p.plans.pid.earned_value.manual": "manual",
  "p.plans.pid.earned_value.measured_as_of_percent_complete":
    "Medido a fecha de {as_of}. El porcentaje completado es trabajo terminado, no tiempo transcurrido.",
  "p.plans.pid.earned_value.method": "Método",
  "p.plans.pid.earned_value.only_when_finished": "solo al terminar",
  "p.plans.pid.earned_value.only_when_finished_2": "Solo al terminar",
  "p.plans.pid.earned_value.over_time": "A lo largo del tiempo",
  "p.plans.pid.earned_value.percent_complete": "Porcentaje completado",
  "p.plans.pid.earned_value.planned_value": "Valor planificado",
  "p.plans.pid.earned_value.plans": "{plans} planes",
  "p.plans.pid.earned_value.progress_against_budget_earned_value":
    "Avance frente al presupuesto (valor ganado)",
  "p.plans.pid.earned_value.record_today_s_figures_now":
    "Registrar ahora las cifras de hoy",
  "p.plans.pid.earned_value.recorded": "Registrado.",
  "p.plans.pid.earned_value.save": "Guardar",
  "p.plans.pid.earned_value.schedule_index": "Índice de cronograma (",
  "p.plans.pid.earned_value.schedule_index_by_time":
    "Índice de cronograma por tiempo (",
  "p.plans.pid.earned_value.schedule_is": "El cronograma está {word}.",
  "p.plans.pid.earned_value.schedule_variance_ev_pv":
    "Variación del cronograma (EV − PV)",
  "p.plans.pid.earned_value.spi": "SPI {ratioOrReason} ·",
  "p.plans.pid.earned_value.started_work_counts_half":
    "el trabajo iniciado cuenta la mitad",
  "p.plans.pid.earned_value.started_work_counts_half_2":
    "El trabajo iniciado cuenta la mitad",
  "p.plans.pid.earned_value.still_to_spend_etc": "Pendiente de gastar (ETC)",
  "p.plans.pid.earned_value.task": "(tarea)",
  "p.plans.pid.earned_value.task_2": "Tarea",
  "p.plans.pid.earned_value.the_one_to_lead_with": "— la que se destaca",
  "p.plans.pid.earned_value.the_scheduler_also_records_them":
    "El programador también las registra a diario.",
  "p.plans.pid.earned_value.this_plan_and_everything_beneath":
    "Este plan y todo lo que contiene",
  "p.plans.pid.earned_value.unknown": "desconocido",
  "p.plans.pid.earned_value.use_the_plan_s_method": "Usar el método del plan",
  "p.plans.pid.earned_value.where_it_is_heading": "Hacia dónde se dirige",
  "p.plans.pid.earned_value.why_required": "Motivo (obligatorio)",
  "p.plans.pid.edit.main_x": "{t} — Main X",
  "p.plans.pid.flow.180_calendar_days": "180 días naturales",
  "p.plans.pid.flow.30_calendar_days": "30 días naturales",
  "p.plans.pid.flow.60_calendar_days": "60 días naturales",
  "p.plans.pid.flow.90_calendar_days": "90 días naturales",
  "p.plans.pid.flow.age": "Antigüedad",
  "p.plans.pid.flow.aging_work_in_progress": "Antigüedad del trabajo en curso",
  "p.plans.pid.flow.arrival_rate": "Tasa de llegada (λ)",
  "p.plans.pid.flow.assignee": "Responsable",
  "p.plans.pid.flow.blocked": "Bloqueado",
  "p.plans.pid.flow.calendar_day": "{v} / día natural",
  "p.plans.pid.flow.column": "Columna",
  "p.plans.pid.flow.columns": "Columnas",
  "p.plans.pid.flow.concentrated": "concentrado",
  "p.plans.pid.flow.constraints": "Restricciones",
  "p.plans.pid.flow.count": "Recuento",
  "p.plans.pid.flow.cumulative_flow": "Flujo acumulado",
  "p.plans.pid.flow.cycle_time_is_work_in":
    "El tiempo de ciclo es el trabajo en curso dividido por el rendimiento, así que bajar un límite lo acorta sin que nadie trabaje más rápido.",
  "p.plans.pid.flow.detail": "Detalle",
  "p.plans.pid.flow.distributions": "Distribuciones",
  "p.plans.pid.flow.first_pass_yield": "Rendimiento a la primera",
  "p.plans.pid.flow.flow": "Flujo",
  "p.plans.pid.flow.flow_efficiency": "Eficiencia del flujo",
  "p.plans.pid.flow.flow_ppm": "Flujo — PPM",
  "p.plans.pid.flow.how_long_work_takes": "Cuánto tarda el trabajo",
  "p.plans.pid.flow.how_long_work_takes_to":
    "Cuánto tarda el trabajo en completarse y cuánto hay en curso a la vez.",
  "p.plans.pid.flow.implied_cycle_time": "Tiempo de ciclo implícito",
  "p.plans.pid.flow.item": "Elemento",
  "p.plans.pid.flow.limit": "Límite",
  "p.plans.pid.flow.max": "máx.",
  "p.plans.pid.flow.mean": "media",
  "p.plans.pid.flow.measure": "Indicador",
  "p.plans.pid.flow.min": "mín.",
  "p.plans.pid.flow.no_cap": "sin límite",
  "p.plans.pid.flow.no_expectation_yet": "aún sin expectativa",
  "p.plans.pid.flow.no_findings_yet": "Aún no hay hallazgos.",
  "p.plans.pid.flow.no_finished_work_to_measure":
    "Aún no hay trabajo terminado que medir.",
  "p.plans.pid.flow.not_enough_finished_items_yet":
    "Aún no hay suficientes elementos terminados.",
  "p.plans.pid.flow.not_started_finished_of":
    "{not_started} sin iniciar · {finished} terminadas de {tasks}",
  "p.plans.pid.flow.nothing_in_progress": "Nada en curso.",
  "p.plans.pid.flow.observed_median": "Mediana observada",
  "p.plans.pid.flow.of_the_calendar_time_these":
    "Del tiempo de calendario que llevaron estos elementos, ¿cuánto estuvo alguien trabajando en ellos? La eficiencia del flujo en el trabajo del conocimiento suele situarse entre el 5 y el 15 %, así que una cifra baja es normal: el hallazgo es la cola, no el esfuerzo.",
  "p.plans.pid.flow.of_the_transitions_behind_these":
    "El {percent} de las transiciones detrás de estas cifras se sintetizaron al introducir el registro, no se observaron. Se consolidan a medida que se acumulan movimientos reales en el tablero.",
  "p.plans.pid.flow.percentiles_are_nearest_rank_so":
    "Los percentiles son de rango más cercano, por lo que cada uno es un elemento real. El tiempo de ciclo es lo que controla el equipo; el tiempo de entrega es lo que espera el solicitante, y la diferencia entre ambos es cuánto tiempo estuvo el trabajo en el backlog.",
  "p.plans.pid.flow.recoverable": "Recuperable",
  "p.plans.pid.flow.rework": "Retrabajo",
  "p.plans.pid.flow.rule": "Regla",
  "p.plans.pid.flow.service_level_expectation":
    "Expectativa de nivel de servicio",
  "p.plans.pid.flow.status": "Estado",
  "p.plans.pid.flow.strong_against_the_5_15":
    "Alta frente a la norma del 5–15 %.",
  "p.plans.pid.flow.subject": "Asunto",
  "p.plans.pid.flow.the_mean_is_shown_for":
    "La media se muestra por completitud y no describe ningún elemento real: cite el p85.",
  "p.plans.pid.flow.the_plan_s_overall_ratio":
    ": la proporción global del plan ({percent}) está muy lejos de la del elemento típico ({percent2}). La mayoría de los elementos fluyen; una minoría está atascada. Eso requiere una solución distinta a la de una entrega uniformemente lenta: localice esos elementos en lugar de rehacer el proceso.",
  "p.plans.pid.flow.the_waste_is": "El desperdicio es",
  "p.plans.pid.flow.throughput": "Rendimiento (μ)",
  "p.plans.pid.flow.typical_median_item":
    "Típica. Elemento mediano: {percent}.",
  "p.plans.pid.flow.unassigned": "sin asignar",
  "p.plans.pid.flow.unusually_high_this_usually_means":
    "Inusualmente alta: esto suele significar que el tablero no se mantiene al día, más que que la cola haya desaparecido.",
  "p.plans.pid.flow.utilization": "Utilización (ρ)",
  "p.plans.pid.flow.vs_expectation": "frente a la expectativa",
  "p.plans.pid.flow.window": "Período",
  "p.plans.pid.flow.work_in_progress": "Trabajo en curso",
  "p.plans.pid.flow.work_in_progress_2": "Trabajo en curso (κ)",
  "p.plans.pid.governance.actual": "Real",
  "p.plans.pid.governance.approved": "aprobado",
  "p.plans.pid.governance.approved_with_conditions": "aprobado con condiciones",
  "p.plans.pid.governance.benefit": "Beneficio",
  "p.plans.pid.governance.benefit_category": "Categoría de beneficio",
  "p.plans.pid.governance.capex": "inversión de capital",
  "p.plans.pid.governance.category": "Categoría",
  "p.plans.pid.governance.choose_objective": "elija un objetivo…",
  "p.plans.pid.governance.commit": "Confirmar",
  "p.plans.pid.governance.commit_anyway": "Confirmar de todos modos",
  "p.plans.pid.governance.committing_this_puts_them_over":
    "Confirmar esto los deja por encima de su capacidad.",
  "p.plans.pid.governance.compliance": "cumplimiento",
  "p.plans.pid.governance.cost_saving": "ahorro de costes",
  "p.plans.pid.governance.currency": "Moneda",
  "p.plans.pid.governance.decision": "Decisión",
  "p.plans.pid.governance.governance_ppm": "Gobernanza — PPM",
  "p.plans.pid.governance.hold": "en espera",
  "p.plans.pid.governance.line": "Línea",
  "p.plans.pid.governance.materialized": "materializado: {materialized}",
  "p.plans.pid.governance.max_exposure": "exposición máxima: {max_exposure}",
  "p.plans.pid.governance.new_benefit": "Nuevo beneficio",
  "p.plans.pid.governance.new_budget_line": "Nueva línea de presupuesto",
  "p.plans.pid.governance.new_milestone": "Nuevo hito",
  "p.plans.pid.governance.new_milestone_due": "Fecha prevista del nuevo hito",
  "p.plans.pid.governance.new_risk": "Nuevo riesgo",
  "p.plans.pid.governance.next": "siguiente: {next_gate}",
  "p.plans.pid.governance.objective": "Objetivo",
  "p.plans.pid.governance.open_risks": "riesgos abiertos: {open}",
  "p.plans.pid.governance.opex": "gasto operativo",
  "p.plans.pid.governance.other": "otro",
  "p.plans.pid.governance.planned": "Previsto",
  "p.plans.pid.governance.planned_actual_variance":
    ": previsto {money} · real {money2} · variación {money3}",
  "p.plans.pid.governance.planned_major": "Previsto (unidades mayores)",
  "p.plans.pid.governance.proposed_does_not_count_until":
    "propuesta (no cuenta hasta que se confirma)",
  "p.plans.pid.governance.quality": "calidad",
  "p.plans.pid.governance.range_for_cost_risk": "Rango para el riesgo de coste",
  "p.plans.pid.governance.realized": "Realizado",
  "p.plans.pid.governance.record_actual": "Registrar real",
  "p.plans.pid.governance.rejected": "rechazado",
  "p.plans.pid.governance.revenue": "ingresos",
  "p.plans.pid.governance.review": "Revisión",
  "p.plans.pid.governance.risk": "Riesgo",
  "p.plans.pid.governance.risk_reduction": "reducción de riesgos",
  "p.plans.pid.governance.roi": "· ROI {toFixed}%",
  "p.plans.pid.governance.stage": "etapa: {v}",
  "p.plans.pid.governance.status": "Estado",
  "p.plans.pid.governance.target": "Objetivo",
  "p.plans.pid.governance.target_major": "Objetivo (unidades mayores)",
  "p.plans.pid.governance.target_realized_spend":
    ": objetivo {money} · realizado {money2} · gasto {money3}",
  "p.plans.pid.governance.weight": "peso",
  "p.plans.pid.governance.worker_uuid": "worker:<uuid>",
  "p.plans.pid.handover.discard": "Descartar",
  "p.plans.pid.handover.from": "De",
  "p.plans.pid.handover.hand_over": "Traspasar",
  "p.plans.pid.handover.handover": "Traspaso",
  "p.plans.pid.handover.handover_ppm": "Traspaso — PPM",
  "p.plans.pid.handover.kind": "Tipo",
  "p.plans.pid.handover.onboarding_and_offboarding": "Incorporación y salida",
  "p.plans.pid.handover.person": "person:…",
  "p.plans.pid.handover.person_empty_releases_the_work":
    "person:… (vacío libera el trabajo)",
  "p.plans.pid.handover.plan": "← Plan",
  "p.plans.pid.handover.preview": "Vista previa",
  "p.plans.pid.handover.reason_recorded_with_each_item":
    "Motivo (se registra con cada elemento)",
  "p.plans.pid.handover.release": "Liberar",
  "p.plans.pid.handover.the_new_person_s_capacity":
    "Aquí no se comprueba la capacidad de la nueva persona.",
  "p.plans.pid.handover.to": "A",
  "p.plans.pid.handover.what": "Qué",
  "p.plans.pid.handover.what_holds_that_is_still":
    "Lo que {from_person} tiene y sigue activo",
  "p.plans.pid.handover.who_is_handing_over": "¿Quién hace el traspaso?",
  "p.plans.pid.import.bring_tasks_in_from_jira":
    "Traiga tareas desde Jira, Asana o Microsoft Project. Primero verá lo que ocurriría; no se escribe nada hasta que confirme.",
  "p.plans.pid.import.choose_another_file": "Elegir otro archivo",
  "p.plans.pid.import.file": "Archivo",
  "p.plans.pid.import.import": "Importar",
  "p.plans.pid.import.import_ppm": "Importar — PPM",
  "p.plans.pid.import.imported": "Importado",
  "p.plans.pid.import.plan": "← Plan",
  "p.plans.pid.import.preview": "Vista previa",
  "p.plans.pid.import.tracker": "Gestor de tareas",
  "p.plans.pid.import.where_from": "Origen",
  "p.plans.pid.joint_risk.a_late_project_usually_costs":
    "Un proyecto retrasado suele costar también más, por lo que esto simula coste y cronograma a la vez.",
  "p.plans.pid.joint_risk.assumptions": "Supuestos",
  "p.plans.pid.joint_risk.band_when_there_is_no":
    "Banda cuando no hay estimación (± %)",
  "p.plans.pid.joint_risk.both_at_once": "Ambos a la vez",
  "p.plans.pid.joint_risk.budget": "presupuesto",
  "p.plans.pid.joint_risk.budget_2": "Presupuesto",
  "p.plans.pid.joint_risk.budget_lines_in_are_not":
    "Las líneas de presupuesto en {join} no forman parte de esta simulación; las monedas nunca se suman.",
  "p.plans.pid.joint_risk.business_days_late":
    "{days} días hábiles de retraso, {money}",
  "p.plans.pid.joint_risk.business_days_late_2": "días hábiles de retraso →",
  "p.plans.pid.joint_risk.cost": "Coste",
  "p.plans.pid.joint_risk.cost_2": "Coste — {title}: {costSkippedText}",
  "p.plans.pid.joint_risk.cost_and_schedule_risk_ppm":
    "Riesgo de coste y cronograma — PPM",
  "p.plans.pid.joint_risk.cost_and_schedule_together":
    "Coste y cronograma juntos",
  "p.plans.pid.joint_risk.cost_lines": "Líneas de coste",
  "p.plans.pid.joint_risk.cost_of_each_business_day":
    "Coste de cada día hábil de retraso (unidades mayores)",
  "p.plans.pid.joint_risk.cost_risk": "Riesgo de coste",
  "p.plans.pid.joint_risk.critical_path_finish_the_plan":
    "Fin según la ruta crítica (la fecha propia del plan)",
  "p.plans.pid.joint_risk.currency_if_the_plan_has":
    "Moneda (si el plan tiene varias)",
  "p.plans.pid.joint_risk.distribution": "Distribución",
  "p.plans.pid.joint_risk.each_dot_is_one_simulation":
    "Cada punto es una simulación (una muestra reducida). A la derecha de la línea de puntualidad hay retraso; por encima de la línea de presupuesto hay sobrecoste. La tabla de arriba tiene los recuentos completos.",
  "p.plans.pid.joint_risk.estimate": "Estimación",
  "p.plans.pid.joint_risk.finish": "Fin",
  "p.plans.pid.joint_risk.finish_date_at_each_percentile":
    "Fecha de fin en cada percentil",
  "p.plans.pid.joint_risk.how_the_simulated_outcomes_fall":
    "Cómo se distribuyen los resultados simulados en torno al presupuesto y al fin según la ruta crítica",
  "p.plans.pid.joint_risk.independence": "Independencia.",
  "p.plans.pid.joint_risk.iterations": "Iteraciones",
  "p.plans.pid.joint_risk.line": "Línea",
  "p.plans.pid.joint_risk.low_planned_high": "Bajo / previsto / alto",
  "p.plans.pid.joint_risk.mean_delay_cost_included":
    "Media (incluido el coste del retraso)",
  "p.plans.pid.joint_risk.no_register_risk_was_used":
    "En esta simulación no se usó ningún riesgo del registro.",
  "p.plans.pid.joint_risk.not_enough_data": "datos insuficientes",
  "p.plans.pid.joint_risk.of_which_delay_cost_on":
    "De los cuales, coste del retraso, en promedio",
  "p.plans.pid.joint_risk.on_time": "a tiempo",
  "p.plans.pid.joint_risk.outcome": "Resultado",
  "p.plans.pid.joint_risk.per_line_estimate_in":
    "Estimación por línea en {currency}",
  "p.plans.pid.joint_risk.pert": "PERT",
  "p.plans.pid.joint_risk.plan": "← Plan",
  "p.plans.pid.joint_risk.risks": "Riesgos",
  "p.plans.pid.joint_risk.run": "Ejecutar",
  "p.plans.pid.joint_risk.running": "Ejecutando…",
  "p.plans.pid.joint_risk.scatter_of_simulated_total_cost":
    "Dispersión del coste total simulado frente a los días hábiles de retraso, con el presupuesto y la línea de puntualidad",
  "p.plans.pid.joint_risk.schedule":
    "Cronograma — {title}: {skippedReasonText}",
  "p.plans.pid.joint_risk.schedule_risk": "Riesgo de cronograma",
  "p.plans.pid.joint_risk.seed": "Semilla",
  "p.plans.pid.joint_risk.seed_the_same_seed_gives":
    "Semilla {seed}: la misma semilla da el mismo resultado.",
  "p.plans.pid.joint_risk.share": "Proporción",
  "p.plans.pid.joint_risk.simulations": "Simulaciones",
  "p.plans.pid.joint_risk.total_cost_delay_cost_included":
    "Coste total, incluido el coste del retraso",
  "p.plans.pid.joint_risk.triangular": "Triangular",
  "p.plans.pid.leveling.allow_critical_tasks_to_move":
    "Permitir mover tareas críticas (esto puede retrasar el fin)",
  "p.plans.pid.leveling.apply": "Aplicar",
  "p.plans.pid.leveling.apply_the_ticked_moves":
    "Aplicar los movimientos marcados",
  "p.plans.pid.leveling.balance_workloads_resource_leveling":
    "Equilibrar cargas de trabajo (nivelación de recursos)",
  "p.plans.pid.leveling.could_not_be_fixed": "No se pudo corregir",
  "p.plans.pid.leveling.critical": "crítica",
  "p.plans.pid.leveling.each_start_date_the_proposal":
    "Cada fecha de inicio que cambiaría la propuesta",
  "p.plans.pid.leveling.finish": "Fin",
  "p.plans.pid.leveling.float": "Holgura",
  "p.plans.pid.leveling.i_have_read_the_proposal":
    "He leído la propuesta y quiero cambiar estas fechas de inicio",
  "p.plans.pid.leveling.later_by": "Más tarde en",
  "p.plans.pid.leveling.load_per_task_of_a":
    "Carga por tarea (% de la jornada de una persona)",
  "p.plans.pid.leveling.loading": "Cargando…",
  "p.plans.pid.leveling.no_start_date_would_change":
    "No cambiaría ninguna fecha de inicio.",
  "p.plans.pid.leveling.nobody_is_over_allocated_so":
    "Nadie está sobreasignado, así que no hay nada que nivelar.",
  "p.plans.pid.leveling.over_allocated_now": "Sobreasignado ahora",
  "p.plans.pid.leveling.person": "Persona",
  "p.plans.pid.leveling.plan": "← Plan",
  "p.plans.pid.leveling.propose_again": "Proponer de nuevo",
  "p.plans.pid.leveling.proposed_start_dates": "Fechas de inicio propuestas",
  "p.plans.pid.leveling.refused": "Rechazado: {note}",
  "p.plans.pid.leveling.resource_leveling_ppm": "Nivelación de recursos — PPM",
  "p.plans.pid.leveling.start": "Inicio",
  "p.plans.pid.leveling.start_finish_business_days":
    "{pid}: inicio {v}{start_days}, fin {v2}{finish_days} días hábiles",
  "p.plans.pid.leveling.suggests_moving_tasks_so_that":
    "Sugiere mover tareas para que nadie tenga asignado más de lo que puede hacer. Nada cambia hasta que confirme.",
  "p.plans.pid.leveling.task": "Tarea",
  "p.plans.pid.leveling.the_search_stopped_at_its":
    "la búsqueda se detuvo en su límite",
  "p.plans.pid.leveling.this_is_advice_nothing_changes":
    "Esto es un consejo. Nada cambia hasta que elija movimientos abajo y los aplique.",
  "p.plans.pid.leveling.why_kept_with_each_audit":
    "Por qué (se guarda con cada entrada de auditoría)",
  "p.plans.pid.leveling.working_calendar": "Calendario laboral",
  "p.plans.pid.more_tools": "Más herramientas",
  "p.plans.pid.onboarding.access": "Acceso",
  "p.plans.pid.onboarding.access_record": "Registro de acceso",
  "p.plans.pid.onboarding.action": "Acción",
  "p.plans.pid.onboarding.cancel_checklist": "Cancelar lista de verificación",
  "p.plans.pid.onboarding.checklists": "Listas de verificación",
  "p.plans.pid.onboarding.complete_checklist":
    "Completar lista de verificación",
  "p.plans.pid.onboarding.done": "Hecho",
  "p.plans.pid.onboarding.effective":
    "{status}{v} · en vigor {effective_date} · {progressText}",
  "p.plans.pid.onboarding.from":
    "{status} · desde {effective_date} · {progressText}",
  "p.plans.pid.onboarding.granted": "Concedido",
  "p.plans.pid.onboarding.hand_over_someone_s_work":
    "Traspasar el trabajo de alguien",
  "p.plans.pid.onboarding.no_access_is_outstanding":
    "● No hay accesos pendientes.",
  "p.plans.pid.onboarding.none_yet": "Ninguno todavía.",
  "p.plans.pid.onboarding.onboarding_and_offboarding": "Incorporación y salida",
  "p.plans.pid.onboarding.onboarding_and_offboarding_ppm":
    "Incorporación y salida — PPM",
  "p.plans.pid.onboarding.person": "person:…",
  "p.plans.pid.onboarding.person_2": "Persona",
  "p.plans.pid.onboarding.plan": "← Plan",
  "p.plans.pid.onboarding.ready_to_close": "¿Listo para cerrar?",
  "p.plans.pid.onboarding.reason": "Motivo",
  "p.plans.pid.onboarding.reason_for_cancelling": "Motivo de la cancelación",
  "p.plans.pid.onboarding.record": "Registrar",
  "p.plans.pid.onboarding.record_access": "Registrar acceso",
  "p.plans.pid.onboarding.reopen": "Reabrir",
  "p.plans.pid.onboarding.repo_write": "escritura en el repositorio",
  "p.plans.pid.onboarding.revoked": "Revocado",
  "p.plans.pid.onboarding.save": "Guardar",
  "p.plans.pid.onboarding.since":
    "▲ {person_ref}: {access} (desde {granted_on})",
  "p.plans.pid.onboarding.skip": "Omitir",
  "p.plans.pid.onboarding.start": "Iniciar",
  "p.plans.pid.onboarding.start_a_checklist":
    "Iniciar una lista de verificación",
  "p.plans.pid.onboarding.takes_effect": "Entra en vigor",
  "p.plans.pid.onboarding.what": "Qué",
  "p.plans.pid.onboarding.who_holds_what_access_is":
    "Quién tiene qué acceso es información sobre personas, por lo que requiere el permiso para gestionar personas.",
  "p.plans.pid.onboarding.why_skip": "¿Por qué omitir?",
  "p.plans.pid.raci.add_a_person_person_or":
    "Añadir una persona (person:… o worker:…)",
  "p.plans.pid.raci.add_a_person_to_start":
    "Añada una persona para empezar a asignar letras.",
  "p.plans.pid.raci.add_column": "Añadir columna",
  "p.plans.pid.raci.export_to_a_spreadsheet": "Exportar a una hoja de cálculo",
  "p.plans.pid.raci.for_each_task_who_does":
    "Para cada tarea: quién la hace, quién la aprueba, a quién se consulta y a quién se informa.",
  "p.plans.pid.raci.have_nobody_accountable":
    "{missing_accountable} no tienen ningún responsable final.",
  "p.plans.pid.raci.have_nobody_responsible":
    "{missing_responsible} no tienen ningún ejecutor.",
  "p.plans.pid.raci.how_many_tasks_each_person":
    "En cuántas tareas tiene cada persona cada letra",
  "p.plans.pid.raci.legend": "Leyenda",
  "p.plans.pid.raci.load_by_person": "Carga por persona",
  "p.plans.pid.raci.loading": "Cargando…",
  "p.plans.pid.raci.of_tasks_are_complete_one":
    "{complete} de {tasks} tareas están completas (un responsable final, al menos un ejecutor).",
  "p.plans.pid.raci.person": "Persona",
  "p.plans.pid.raci.plan": "← Plan",
  "p.plans.pid.raci.raci_ppm": "RACI — PPM",
  "p.plans.pid.raci.this_plan_has_no_tasks": "Este plan aún no tiene tareas.",
  "p.plans.pid.raci.who_does_what_raci_matrix": "Quién hace qué (matriz RACI)",
  "p.plans.pid.schedule.dependency_violated_successor_starts":
    "Dependencia incumplida: la sucesora empieza el {actual_start}, pero no puede empezar antes del {earliest_start}.",
  "p.plans.pid.schedule.schedule_ppm": "Cronograma — PPM",
  "p.plans.pid.schedule_health.as_of": "A fecha de {as_of} ·",
  "p.plans.pid.schedule_health.baseline_version":
    "versión de línea base {version}",
  "p.plans.pid.schedule_health.check": "Comprobar",
  "p.plans.pid.schedule_health.checks_the_schedule_for_common":
    "Comprueba si el cronograma tiene problemas habituales, como tareas sin vínculos o sin estimación.",
  "p.plans.pid.schedule_health.critical_path": "Ruta crítica",
  "p.plans.pid.schedule_health.held_to": "Respecto a",
  "p.plans.pid.schedule_health.loading": "Cargando…",
  "p.plans.pid.schedule_health.no_baseline_taken":
    "no se ha tomado ninguna línea base",
  "p.plans.pid.schedule_health.plan": "← Plan",
  "p.plans.pid.schedule_health.re_run": "Volver a ejecutar",
  "p.plans.pid.schedule_health.result": "Resultado",
  "p.plans.pid.schedule_health.schedule_health": "Salud del cronograma",
  "p.plans.pid.schedule_health.schedule_health_ppm":
    "Salud del cronograma — PPM",
  "p.plans.pid.schedule_health.see_the_note": "Ver la nota",
  "p.plans.pid.schedule_health.status_date": "Fecha de estado",
  "p.plans.pid.schedule_health.the_fourteen_schedule_health_checks":
    "Las catorce comprobaciones de salud del cronograma",
  "p.plans.pid.schedule_health.to_fix": "Para corregir",
  "p.plans.pid.schedule_health.what_was_found": "Qué se encontró",
  "p.plans.pid.schedule_risk.assumptions": "Supuestos",
  "p.plans.pid.schedule_risk.band_for_tasks_with_no":
    "Banda para tareas sin estimación (± %)",
  "p.plans.pid.schedule_risk.critical_in": "Crítica en",
  "p.plans.pid.schedule_risk.critical_path": "Ruta crítica",
  "p.plans.pid.schedule_risk.critical_path_finish_the_plan":
    "Fin según la ruta crítica (la fecha propia del plan)",
  "p.plans.pid.schedule_risk.distribution": "Distribución",
  "p.plans.pid.schedule_risk.each_bar_is_a_finish":
    "Cada barra es una fecha de fin; cuanto más alta, más simulaciones terminaron ahí. Las líneas continuas son P50, P80 y P90 (etiquetadas); la línea discontinua es la fecha propia del plan.",
  "p.plans.pid.schedule_risk.earliest_latest":
    "{join}. La más temprana {earliest}, la más tardía {latest}.",
  "p.plans.pid.schedule_risk.estimate": "Estimación",
  "p.plans.pid.schedule_risk.finish": "Fin",
  "p.plans.pid.schedule_risk.finish_date": "Fecha de fin",
  "p.plans.pid.schedule_risk.finish_date_at_each_percentile":
    "Fecha de fin en cada percentil",
  "p.plans.pid.schedule_risk.histogram_of_simulated_finish_dates":
    "Histograma de fechas de fin simuladas, con P50, P80 y P90 marcados",
  "p.plans.pid.schedule_risk.independence": "Independencia.",
  "p.plans.pid.schedule_risk.iterations": "Iteraciones",
  "p.plans.pid.schedule_risk.left_out": "Excluidas:",
  "p.plans.pid.schedule_risk.many_what_if_runs_of":
    "Muchas simulaciones hipotéticas del cronograma muestran el rango de fechas de fin y la probabilidad de cada una.",
  "p.plans.pid.schedule_risk.no_register_risk_was_used":
    "No se usó ningún riesgo del registro como evento del cronograma.",
  "p.plans.pid.schedule_risk.not_enough_data": "datos insuficientes",
  "p.plans.pid.schedule_risk.not_simulated_because_the_schedule":
    "No simuladas, porque el cronograma no puede situarlas: {join}.",
  "p.plans.pid.schedule_risk.of": "{date}: {count} de {iterations}",
  "p.plans.pid.schedule_risk.of_runs": "{criticalityText} de las simulaciones",
  "p.plans.pid.schedule_risk.optimistic_likely_pessimistic_busine":
    "Optimista / probable / pesimista (días hábiles)",
  "p.plans.pid.schedule_risk.outcome": "Resultado",
  "p.plans.pid.schedule_risk.per_task_estimate_criticality_and":
    "Estimación, criticidad y sensibilidad por tarea",
  "p.plans.pid.schedule_risk.plan": "← Plan",
  "p.plans.pid.schedule_risk.plan_2": "plan",
  "p.plans.pid.schedule_risk.risks_used": "Riesgos usados",
  "p.plans.pid.schedule_risk.run": "Ejecutar",
  "p.plans.pid.schedule_risk.run_on_against_the_plan":
    "Ejecutado el {ranOn} con el plan tal como está · semilla {seed} · {iterations} iteraciones · {distributionText} · calendario {calendar}",
  "p.plans.pid.schedule_risk.running": "Ejecutando…",
  "p.plans.pid.schedule_risk.schedule_risk": "Riesgo de cronograma",
  "p.plans.pid.schedule_risk.schedule_risk_ppm": "Riesgo de cronograma — PPM",
  "p.plans.pid.schedule_risk.seed": "Semilla",
  "p.plans.pid.schedule_risk.seed_the_same_seed_gives":
    "Semilla {seed}: la misma semilla da el mismo resultado. Hasta {max_iterations} iteraciones.",
  "p.plans.pid.schedule_risk.sensitivity": "Sensibilidad",
  "p.plans.pid.schedule_risk.share": "Proporción",
  "p.plans.pid.schedule_risk.simulations": "Simulaciones",
  "p.plans.pid.schedule_risk.task": "Tarea",
  "p.plans.pid.schedule_risk.the_same_figures_as_a":
    "Las mismas cifras en una tabla",
  "p.plans.pid.schedule_risk.triangular": "Triangular",
  "p.plans.pid.schedule_risk.weighted_towards_the_likely_value":
    "Ponderada hacia el valor probable (PERT)",
  "p.plans.pid.schedule_risk.what_drives_it": "Qué lo determina",
  "p.plans.pid.schedule_risk.when_it_finishes": "Cuándo termina",
  "p.plans.pid.wbs.code": "Código",
  "p.plans.pid.wbs.level": "Nivel",
  "p.plans.pid.wbs.no_tasks_yet": "Aún no hay tareas.",
  "p.plans.pid.wbs.plan": "← Plan",
  "p.plans.pid.wbs.sub_tasks": "Subtareas",
  "p.plans.pid.wbs.task": "Tarea",
  "p.plans.pid.wbs.tasks_is_too_many_to":
    "{length} tareas son demasiadas para anidarlas de forma útil; la tabla las muestra todas.",
  "p.plans.pid.wbs.the_plan_s_tasks_each":
    "Las tareas del plan, cada una desglosada en sus subtareas. El código de cada línea es solo para lectura: cambia cuando se añade o se mueve trabajo.",
  "p.plans.pid.wbs.work_breakdown": "Desglose del trabajo",
  "p.plans.pid.wbs.work_breakdown_as_a_table":
    "Desglose del trabajo en una tabla",
  "p.plans.pid.wbs.work_breakdown_ppm": "Desglose del trabajo — PPM",
  "p.plans.pid.wbs.work_breakdown_tree": "Árbol de desglose del trabajo",
  "p.plans.pid.what_if.after": "Después",
  "p.plans.pid.what_if.before": "Antes",
  "p.plans.pid.what_if.business_days": "Días hábiles",
  "p.plans.pid.what_if.choose": "Elegir…",
  "p.plans.pid.what_if.commit_these_changes": "Confirmar estos cambios",
  "p.plans.pid.what_if.constraint": "Restricción",
  "p.plans.pid.what_if.critical": "■ crítica",
  "p.plans.pid.what_if.critical_path": "Ruta crítica",
  "p.plans.pid.what_if.date": "Fecha",
  "p.plans.pid.what_if.discard": "Descartar",
  "p.plans.pid.what_if.edit": "Editar",
  "p.plans.pid.what_if.finish_to_finish": "Fin a fin",
  "p.plans.pid.what_if.finish_to_start": "Fin a inicio",
  "p.plans.pid.what_if.float": "Holgura",
  "p.plans.pid.what_if.follows": "{name} sigue a {name2} ({toUpperCase})",
  "p.plans.pid.what_if.joins_the_critical_path":
    "Se incorpora a la ruta crítica: {join}.",
  "p.plans.pid.what_if.lag_business_days": "Desfase (días hábiles)",
  "p.plans.pid.what_if.leaves_it": "Sale de ella: {join}.",
  "p.plans.pid.what_if.link": "Vínculo",
  "p.plans.pid.what_if.new_start": "Nuevo inicio",
  "p.plans.pid.what_if.no_task_s_dates_or":
    "No cambian las fechas ni la holgura de ninguna tarea.",
  "p.plans.pid.what_if.none_clear_it": "Ninguna (borrarla)",
  "p.plans.pid.what_if.none_yet": "Ninguno todavía.",
  "p.plans.pid.what_if.now_takes_of_the_day":
    "«{name}» ocupa ahora el {units_percent} % de la jornada para {hoursText} de trabajo (calculado a partir de su regla de programación).",
  "p.plans.pid.what_if.of_work_has_no_rate":
    "▲ {hoursText} de trabajo no tienen tarifa, por lo que no se costean.",
  "p.plans.pid.what_if.plan": "← Plan",
  "p.plans.pid.what_if.planned_labour_cost":
    "Coste de mano de obra previsto: {deltaText} ({deltaText2} → {deltaText3}).",
  "p.plans.pid.what_if.predecessor": "Predecesora",
  "p.plans.pid.what_if.preview": "Vista previa",
  "p.plans.pid.what_if.reason": "Motivo",
  "p.plans.pid.what_if.reason_recorded_with_each_change":
    "Motivo (se registra con cada cambio)",
  "p.plans.pid.what_if.remove": "Quitar",
  "p.plans.pid.what_if.stage_an_edit": "Preparar una edición",
  "p.plans.pid.what_if.stage_edit": "Preparar edición",
  "p.plans.pid.what_if.staged_edits": "Ediciones preparadas",
  "p.plans.pid.what_if.start_to_finish": "Inicio a fin",
  "p.plans.pid.what_if.start_to_start": "Inicio a inicio",
  "p.plans.pid.what_if.successor": "Sucesora",
  "p.plans.pid.what_if.task": "Tarea",
  "p.plans.pid.what_if.tasks_whose_dates_float_or":
    "Tareas cuyas fechas, holgura o estado crítico cambian",
  "p.plans.pid.what_if.try_changes_on_a_copy":
    "Pruebe cambios en una copia. No se guarda nada hasta que confirme.",
  "p.plans.pid.what_if.type": "Tipo",
  "p.plans.pid.what_if.what_if": "Hipótesis",
  "p.plans.pid.what_if.what_if_ppm": "Hipótesis — PPM",
  "p.plans.pid.work.business_days_of_a_day":
    "{duration_days} días hábiles · {units_percent} % de una jornada · {hoursText} de trabajo · {v}",
  "p.plans.pid.work.change": "Cambiar",
  "p.plans.pid.work.costs": "Costes",
  "p.plans.pid.work.day": "Día",
  "p.plans.pid.work.duration_business_days": "Duración (días hábiles)",
  "p.plans.pid.work.edit_assignment": "Editar asignación",
  "p.plans.pid.work.falls_outside_this_window":
    "{hoursText} quedan fuera de esta ventana.",
  "p.plans.pid.work.hours_per_each_row_adds":
    "Horas por {grain}; cada fila suma el trabajo de la tarea y la última fila, el del plan.",
  "p.plans.pid.work.how_long_each_task_takes":
    "Cuánto dura cada tarea, qué parte de la jornada de una persona ocupa y a cuánto trabajo equivale.",
  "p.plans.pid.work.month": "Mes",
  "p.plans.pid.work.no_assignment_yet_give_a":
    "Aún no hay asignación. Indique una duración y unas unidades y se calculará el trabajo.",
  "p.plans.pid.work.no_task_carries_work_yet":
    "Ninguna tarea tiene trabajo todavía, así que no hay nada que repartir.",
  "p.plans.pid.work.not_placed": "(sin situar)",
  "p.plans.pid.work.percentages": "Porcentajes",
  "p.plans.pid.work.period": "Periodo",
  "p.plans.pid.work.plan": "← Plan",
  "p.plans.pid.work.plan_2": "Plan",
  "p.plans.pid.work.rule": "Regla",
  "p.plans.pid.work.save_assignment": "Guardar asignación",
  "p.plans.pid.work.save_contour": "Guardar perfil",
  "p.plans.pid.work.so_its_work_is_not":
    "▲ {title} ({hoursText}) {reason}, por lo que su trabajo no está en los intervalos.",
  "p.plans.pid.work.spread": "Reparto",
  "p.plans.pid.work.task": "Tarea",
  "p.plans.pid.work.tasks": "Tareas",
  "p.plans.pid.work.this_plan_has_no_scheduled":
    "Este plan aún no tiene tareas programadas.",
  "p.plans.pid.work.total": "Total",
  "p.plans.pid.work.units": "Unidades (%)",
  "p.plans.pid.work.units_of_the_day": "Unidades (% de la jornada)",
  "p.plans.pid.work.week": "Semana",
  "p.plans.pid.work.work_and_assignments": "Trabajo y asignaciones",
  "p.plans.pid.work.work_and_assignments_ppm": "Trabajo y asignaciones — PPM",
  "p.plans.pid.work.work_hours": "Trabajo (horas)",
  "p.plans.pid.work.work_over_time": "Trabajo a lo largo del tiempo",
  "p.prioritization.all": "todos",
  "p.prioritization.apply": "Aplicar",
  "p.prioritization.band": "banda",
  "p.prioritization.band_2": "Banda",
  "p.prioritization.component": "Componente",
  "p.prioritization.coverage": "cobertura",
  "p.prioritization.coverage_2": "Cobertura",
  "p.prioritization.explain": "Explicar",
  "p.prioritization.hide": "Ocultar",
  "p.prioritization.high": "alta",
  "p.prioritization.loading_the_breakdown": "Cargando el desglose…",
  "p.prioritization.low": "baja",
  "p.prioritization.medium": "media",
  "p.prioritization.missing_evidence": "Falta evidencia",
  "p.prioritization.no_evidence_for_any_component":
    "No hay evidencia para ningún componente: este plan no tiene puntuación.",
  "p.prioritization.no_plans_match_that_filter":
    "Ningún plan coincide con ese filtro.",
  "p.prioritization.none": "ninguna",
  "p.prioritization.of_plans_one": "{length} de {scored_of} plan",
  "p.prioritization.of_plans_other": "{length} de {scored_of} planes",
  "p.prioritization.plan": "Plan",
  "p.prioritization.points": "Puntos",
  "p.prioritization.prioritization": "Priorización",
  "p.prioritization.prioritization_ppm": "Priorización — PPM",
  "p.prioritization.score": "Puntuación",
  "p.prioritization.shows_how_much_of_the":
    "muestra qué parte de la puntuación estaba realmente respaldada por datos.",
  "p.prioritization.smart_score_over_roi_strategic":
    "Puntuación inteligente basada en ROI, alineación estratégica, revisión experta, riesgo, demanda, prioridad MoSCoW e impulso. Los componentes sin evidencia se descartan y el resto se renormaliza —",
  "p.prioritization.unscored": "sin puntuación",
  "p.prioritization.value": "Valor",
  "p.prioritization.weight": "Peso",
  "p.proposals.actions": "Acciones",
  "p.proposals.currency": "Moneda",
  "p.proposals.live": "activo:",
  "p.proposals.promote": "promover",
  "p.proposals.proposal": "propuesta:",
  "p.proposals.proposals_ppm": "Propuestas — PPM",
  "p.proposals.requested": "Solicitado",
  "p.proposals.status": "Estado",
  "p.proposals.status_filter": "Filtro de estado",
  "p.proposals.target": "Destino",
  "p.proposals.target_collection": "Colección de destino",
  "p.proposals.title": "Título",
  "p.regulator.actual": "Real",
  "p.regulator.benefits": "Beneficios:",
  "p.regulator.currency": "Moneda",
  "p.regulator.gate_decisions": "· Decisiones de fase:",
  "p.regulator.members": "Miembros:",
  "p.regulator.names_withheld_abac_mask_obligation":
    "Nombres ocultos (obligación de enmascaramiento ABAC).",
  "p.regulator.no_budget_lines": "No hay líneas de presupuesto.",
  "p.regulator.no_portfolios_recorded": "No hay carteras registradas.",
  "p.regulator.none": "ninguno",
  "p.regulator.planned": "Previsto",
  "p.regulator.ppm": "{t} — PPM",
  "p.regulator.stage": "etapa: {v}",
  "p.regulator.target_realized": "objetivo {money} / realizado {money2}",
  "p.reports.collection": "Colección",
  "p.reports.csv": "CSV",
  "p.reports.fields_comma_separated": "campos (separados por comas)",
  "p.reports.name_contains": "el nombre contiene…",
  "p.reports.reports_ppm": "Informes — PPM",
  "p.resources.a_multi_skill_booking_counts":
    "Una reserva con varias competencias cuenta completa para cada una de ellas, por lo que las filas de competencias se solapan y no suman el total.",
  "p.resources.by": "Por",
  "p.resources.demand_fte": "Demanda FTE",
  "p.resources.demand_supply_and_gap_by":
    "Demanda, oferta y brecha por periodo y {group_by}",
  "p.resources.every_figure_as_a_table": "Todas las cifras en una tabla",
  "p.resources.from": "Desde",
  "p.resources.gap_fte": "Brecha FTE",
  "p.resources.gap_per_in_fte_supply":
    "Brecha por {period}, en FTE (oferta menos demanda)",
  "p.resources.loading": "Cargando…",
  "p.resources.month": "Mes",
  "p.resources.no_demand_or_supply_in":
    "No hay demanda ni oferta en esta ventana.",
  "p.resources.not_known": "desconocido",
  "p.resources.of_which_unassigned": "De los cuales, sin asignar",
  "p.resources.people": "Personas",
  "p.resources.per": "Por",
  "p.resources.per_by_scope":
    "{periodLabel} · por {period} · por {group_by} · ámbito: {kind}",
  "p.resources.period": "Periodo",
  "p.resources.proposed_fte": "FTE propuestos",
  "l.processes.name_problem":
    "Asigne al proceso un nombre de 1 a 120 caracteres.",
  "l.processes.steps_problem": "Un proceso tiene de 1 a {max} pasos.",
  "l.processes.title_problem":
    "Paso {n}: asígnele un título de 1 a 200 caracteres.",
  "l.processes.person_problem":
    "Paso {n}: la persona debe ser una referencia person: o worker:.",
  "l.processes.kind_approve": "Aprobar",
  "l.processes.kind_todo": "Por hacer",
  "l.processes.kind_notify": "Notificar",
  "l.processes.state_pending": "no alcanzado",
  "l.processes.state_waiting": "en espera",
  "l.processes.state_completed": "hecho",
  "l.processes.state_skipped": "omitido",
  "l.processes.state_rejected": "rechazado",
  "l.processes.state_failed": "fallido",
  "l.processes.run_running": "en curso",
  "l.processes.run_completed": "completado",
  "l.processes.run_rejected": "rechazado",
  "l.processes.run_cancelled": "cancelado",
  "l.processes.run_failed": "fallido",
  "p.processes.processes": "Procesos de aprobación",
  "p.processes.processes_ppm": "Procesos de aprobación — PPM",
  "p.processes.intro":
    "Un proceso es una lista de pasos que se ejecutan uno tras otro: pedir a alguien que apruebe, asignar a alguien una tarea o informar a alguien. Un rechazo termina la ejecución y omite los pasos posteriores. Editar un proceso nunca cambia una ejecución ya en curso.",
  "p.processes.definitions": "Procesos",
  "p.processes.none_defined": "Aún no hay procesos definidos.",
  "p.processes.version": "versión {v}",
  "p.processes.new_process": "Definir un proceso",
  "p.processes.name": "Nombre",
  "p.processes.step_n": "Paso {n}",
  "p.processes.kind": "Qué hace",
  "p.processes.person": "Persona (person:… o worker:…)",
  "p.processes.step_title": "Qué dice",
  "p.processes.add_step": "Añadir un paso",
  "p.processes.remove_step": "Quitar el paso {n}",
  "p.processes.create": "Definir el proceso",
  "p.processes.defined": "Se definió {name}.",
  "p.processes.start_heading": "Iniciar una ejecución de {name}",
  "p.processes.subject_kind": "A qué se refiere",
  "p.processes.subject_pid": "Su identificador",
  "p.processes.start": "Iniciar la ejecución",
  "p.processes.started": "Se inició una ejecución de {name}.",
  "p.processes.runs": "Sus ejecuciones",
  "p.processes.none_runs": "No ha iniciado ninguna ejecución.",
  "p.processes.run_of": "{name} sobre {kind} {pid}",
  "p.processes.status": "Estado",
  "p.processes.show_run": "Mostrar {name}",
  "p.processes.hide_run": "Ocultar {name}",
  "p.processes.steps": "Pasos",
  "p.processes.log": "Qué ocurrió",
  "p.processes.complete_step": "Marcar el paso {n} como hecho",
  "p.processes.cancel": "Cancelar la ejecución",
  "p.processes.resume": "Comprobar de nuevo la aprobación",
  "p.processes.done": "Hecho.",
  "p.processes.could_not": "No funcionó: {reason}",
  "p.processes.could_not_load": "No se pudieron cargar los procesos",
  "l.tolerance.green": "dentro de la tolerancia",
  "l.tolerance.amber": "cerca del límite",
  "l.tolerance.red": "por encima de la tolerancia",
  "l.tolerance.unknown": "no medible",
  "l.tolerance.not_set": "sin límite definido",
  "l.tolerance.time": "Tiempo",
  "l.tolerance.schedule": "Cronograma frente a la línea base",
  "l.tolerance.cost": "Coste",
  "l.tolerance.risk": "Riesgo",
  "l.tolerance.resources": "Recursos",
  "l.tolerance.day_one": "{n} día",
  "l.tolerance.day_other": "{n} días",
  "l.tolerance.score": "puntuación {n}",
  "l.tolerance.points": "{n} puntos por encima del 100 %",
  "l.tolerance.field_time": "Límite (días naturales de retraso)",
  "l.tolerance.field_schedule":
    "Límite (días naturales posteriores a la línea base)",
  "l.tolerance.field_cost":
    "Límite (porcentaje por encima del presupuesto aprobado)",
  "l.tolerance.field_risk":
    "Límite (puntuación de riesgo máxima aceptable, de 1 a 25)",
  "l.tolerance.field_resources":
    "Límite (puntos porcentuales de sobrerreserva)",
  "l.tolerance.no_limit":
    "No hay ningún límite definido aquí ni en niveles superiores.",
  "l.tolerance.could_not_measure": "No se pudo medir.",
  "l.tolerance.zero_tolerance":
    "Tolerancia cero: cualquier desviación supera el límite.",
  "l.tolerance.used": "{percent} % del límite usado",
  "l.tolerance.own": "Definido en este plan",
  "l.tolerance.inherited": "Heredado de un plan superior",
  "p.issue_settings.title": "Tipos, estados y funciones de incidencias",
  "p.issue_settings.title_ppm":
    "Tipos, estados y funciones de incidencias — PPM",
  "p.issue_settings.intro":
    "El vocabulario de incidencias propio de la organización. Solo un administrador puede cambiarlo. Si no se configura nada, las incidencias usan los tipos y estados integrados.",
  "p.issue_settings.types_heading": "Tipos de incidencia",
  "p.issue_settings.types_intro":
    "Un tipo retirado ya no se puede elegir. Las incidencias que lo llevan lo conservan.",
  "p.issue_settings.key": "Clave",
  "p.issue_settings.label": "Nombre",
  "p.issue_settings.state": "Estado",
  "p.issue_settings.in_use": "En uso",
  "p.issue_settings.retired": "Retirado",
  "p.issue_settings.retire": "Retirar",
  "p.issue_settings.restore": "Restaurar",
  "p.issue_settings.actions": "Acciones",
  "p.issue_settings.add_type": "Añadir tipo",
  "p.issue_settings.roles_heading": "Funciones",
  "p.issue_settings.roles_intro":
    "Las funciones que reconoce esta organización y lo que permite cada una en un plan con miembros de función. Una función solo puede restringir lo que el inicio de sesión de alguien ya permite. Un flujo de trabajo solo puede nombrar una función de la lista, para que un movimiento no quede restringido a una mal escrita.",
  "p.issue_settings.description": "Descripción",
  "p.issue_settings.add_role": "Añadir función",
  "p.issue_settings.workflows_heading": "Estados y pasos",
  "p.issue_settings.workflows_intro":
    "Un flujo de trabajo fija los estados de un tipo de incidencia, quién puede hacer cada movimiento y qué campos exige o bloquea cada estado. Un flujo para un tipo prevalece sobre el de todos los tipos. Las incidencias en un estado que elimine lo conservan.",
  "p.issue_settings.none_workflows":
    "No hay ningún flujo de incidencias registrado, así que se usan los estados integrados.",
  "p.issue_settings.wf_name": "Nombre del flujo",
  "p.issue_settings.wf_type": "Se aplica a",
  "p.issue_settings.wf_all_types": "Todos los tipos sin flujo propio",
  "p.issue_settings.wf_plan": "Un plan",
  "p.issue_settings.statuses": "Estados",
  "p.issue_settings.category": "Significado",
  "p.issue_settings.initial": "Empieza aquí",
  "p.issue_settings.terminal": "Requiere resolución",
  "p.issue_settings.add_status": "Añadir estado",
  "p.issue_settings.moves": "Pasos permitidos",
  "p.issue_settings.moves_intro":
    "Si no se indica ningún paso, se permite cualquiera. En cuanto se indica uno, solo se permiten los indicados.",
  "p.issue_settings.from": "Desde",
  "p.issue_settings.to": "Hasta",
  "p.issue_settings.move_roles":
    "Funciones (separadas por comas; en blanco para todos)",
  "p.issue_settings.add_move": "Añadir paso",
  "p.issue_settings.remove": "Quitar",
  "p.issue_settings.register": "Registrar flujo",
  "p.issue_settings.withdraw": "Retirar",
  "p.issue_settings.saved": "Guardado.",
  "p.issue_settings.could_not_load":
    "No se pudieron cargar los ajustes de incidencias.",
  "p.issue_settings.could_not_save":
    "No se pudo guardar. Solo un administrador puede cambiar estos ajustes, y cada clave debe ser nueva y estar bien formada.",
  "p.issue_settings.cat_todo": "Sin empezar",
  "p.issue_settings.cat_active": "En curso",
  "p.issue_settings.cat_waiting": "En espera",
  "p.issue_settings.cat_done": "Terminado",
  "p.issue_settings.translations":
    "Traducciones (idioma=nombre, separadas por punto y coma)",
  "p.issue_settings.bad_translations":
    "Escriba cada traducción como idioma=nombre, por ejemplo cy=Risg.",
  "p.issue_settings.save": "Guardar",
  "p.issue_settings.permissions": "Permisos",
  "p.issue_settings.perm_issue_add": "Registrar incidencias",
  "p.issue_settings.perm_issue_edit": "Editar incidencias",
  "p.issue_settings.perm_issue_move": "Mover incidencias",
  "p.issue_settings.perm_issue_delete": "Quitar incidencias",
  "p.issue_settings.edit": "Editar",
  "p.issue_settings.save_workflow": "Guardar flujo de trabajo",
  "p.issue_settings.cancel": "Cancelar",
  "p.issue_settings.editing": "Cambiando {name}",
  "p.issue_settings.rule_field": "Campo",
  "p.issue_settings.rule_kind": "Regla",
  "p.issue_settings.rule_required": "Obligatorio",
  "p.issue_settings.rule_read_only": "Solo lectura",
  "p.issue_settings.rule_roles":
    "Funciones (separadas por comas; en blanco para todos)",
  "p.issue_settings.add_rule": "Añadir regla de campo",
  "p.issue_settings.field_title": "Título",
  "p.issue_settings.field_description": "Descripción",
  "p.issue_settings.field_severity": "Gravedad",
  "p.issue_settings.field_priority": "Prioridad",
  "p.issue_settings.field_owner_ref": "Responsable",
  "p.issue_settings.field_due_date": "Fecha de vencimiento",
  "p.issue_settings.field_resolution": "Resolución",
  "p.plan_settings.title": "Ajustes de incidencias de este plan",
  "p.plan_settings.title_ppm": "Ajustes de incidencias de este plan — PPM",
  "p.plan_settings.intro":
    "Quién tiene cada función aquí, qué tipos de incidencia ofrece este plan y sus propios flujos de trabajo. Estos cambios los hace un administrador.",
  "p.plan_settings.back": "Volver a las incidencias de este plan",
  "p.plan_settings.all_settings": "Ajustes de toda la organización",
  "p.plan_settings.members_heading": "Personas y funciones",
  "p.plan_settings.members_intro":
    "Cuando un plan tiene al menos una persona en esta lista, registrar, editar, mover y quitar incidencias en él requiere una función que lo permita. Una función no puede dar a nadie más de lo que permite su inicio de sesión, y no abre un plan privado.",
  "p.plan_settings.enforced":
    "Las acciones sobre incidencias de este plan requieren una función.",
  "p.plan_settings.not_enforced":
    "Cualquiera que pueda escribir puede actuar sobre las incidencias aquí.",
  "p.plan_settings.person": "Id. de usuario de la persona",
  "p.plan_settings.role": "Función",
  "p.plan_settings.choose_role": "Elija una función",
  "p.plan_settings.add_member": "Añadir persona",
  "p.plan_settings.types_heading": "Tipos de incidencia de este plan",
  "p.plan_settings.types_intro":
    "Marque los tipos que ofrece este plan. Sin ninguno marcado, ofrece todos. Una incidencia conserva un tipo que deje fuera.",
  "p.plan_settings.save_types": "Guardar tipos",
  "p.plan_settings.workflows_intro":
    "Un flujo de trabajo aquí se aplica solo a este plan y prevalece sobre el de la organización. Un flujo para un tipo prevalece sobre el de todos los tipos. Las incidencias en un estado que elimine lo conservan.",
  "p.plan_issues.settings_link": "Ajustes de incidencias de este plan",
  "p.plan_issues.problem_fields_required": "Este estado necesita: {fields}.",
  "p.tolerances.tolerances": "Tolerancias",
  "p.tolerances.tolerances_ppm": "Tolerancias — PPM",
  "p.tolerances.plan_link": "Tolerancias",
  "p.tolerances.intro":
    "Cuánto puede desviarse cada proyecto en tiempo, coste, riesgo y recursos antes de tener que remitirse a un nivel superior, y qué parte de cada límite se ha usado. Dentro del límite pero cerca de él es ámbar; por encima es rojo; un límite que no se puede medir se muestra como no medible, nunca como verde.",
  "p.tolerances.board_intro":
    "Todos los planes que puede leer que tienen una tolerancia, del peor al mejor.",
  "p.tolerances.none": "Ningún plan tiene todavía una tolerancia.",
  "p.tolerances.none_match": "Ningún plan coincide.",
  "p.tolerances.show": "Mostrar",
  "p.tolerances.all_statuses": "Todos los colores",
  "p.tolerances.plan": "Plan",
  "p.tolerances.overall": "General",
  "p.tolerances.truncated":
    "Solo se midieron los primeros {n} planes, por lo que esta lista puede estar incompleta.",
  "p.tolerances.loading": "Cargando…",
  "p.tolerances.could_not_load": "No se pudieron cargar las tolerancias",
  "p.tolerances.back": "← Plan",
  "p.tolerances.board_link": "Todas las tolerancias",
  "p.tolerances.limit": "Límite",
  "p.tolerances.measured": "Medido",
  "p.tolerances.how": "Cómo se mide",
  "p.tolerances.amber_from": "Ámbar a partir de (porcentaje del límite)",
  "p.tolerances.note": "Nota",
  "p.tolerances.save": "Definir el límite",
  "p.tolerances.remove": "Quitar el límite propio de este plan",
  "p.tolerances.saved": "Se guardó el límite de {dimension}.",
  "p.tolerances.removed": "Se quitó el límite de {dimension}.",
  "p.tolerances.could_not": "No funcionó: {reason}",
  "p.tolerances.limit_problem": "Indique el límite como un número.",
  "l.escalation.open": "abierta",
  "l.escalation.acknowledged": "reconocida",
  "l.escalation.resolved": "resuelta",
  "l.escalation.withdrawn": "retirada",
  "l.escalation.reason_tolerance": "Se supera una tolerancia",
  "l.escalation.reason_risk": "Un riesgo",
  "l.escalation.reason_issue": "Una incidencia",
  "l.escalation.reason_dependency": "Una dependencia",
  "l.escalation.reason_decision": "Se necesita una decisión",
  "l.escalation.reason_resource": "Recursos",
  "l.escalation.reason_other": "Otra cosa",
  "l.escalation.outcome_decided": "Decidido",
  "l.escalation.outcome_guidance_given": "Orientación dada",
  "l.escalation.outcome_rejected": "Rechazado",
  "l.escalation.event_raised": "Planteada",
  "l.escalation.event_acknowledged": "Reconocida",
  "l.escalation.event_level_raised": "Elevada un nivel",
  "l.escalation.event_resolved": "Resuelta",
  "l.escalation.event_withdrawn": "Retirada",
  "l.escalation.title_problem": "Asígnele un título de 1 a 200 caracteres.",
  "l.escalation.dimension_problem": "Indique a qué tolerancia se refiere.",
  "l.escalation.level_problem": "El nivel es un número entero de 1 a 9.",
  "l.escalation.owner_problem":
    "La persona debe ser una referencia person: o worker:.",
  "l.escalation.level_to": "Nivel {level}: {to}",
  "l.escalation.level": "Nivel {level}",
  "l.escalation.open_for": "Abierta desde hace {n} días",
  "l.escalation.took": "Tardó {n} días",
  "l.escalation.overdue": "vencida",
  "l.escalation.automatic": "Planteada automáticamente",
  "p.escalations.escalations": "Escalados",
  "p.escalations.escalations_ppm": "Escalados — PPM",
  "p.escalations.plan_link": "Escalados",
  "p.task.task": "Tarea",
  "p.task.task_ppm": "Tarea — PPM",
  "p.task.back": "Volver al desglose del trabajo",
  "p.task.could_not_load": "No se pudo cargar la tarea.",
  "p.task.loading": "Cargando…",
  "p.task.status": "Estado",
  "p.task.due": "Vencimiento",
  "p.task.assignee": "Asignada a",
  "p.task.description": "Descripción",
  "p.task.s_todo": "Por hacer",
  "p.task.s_in_progress": "En curso",
  "p.task.s_in_review": "En revisión",
  "p.task.s_done": "Hecha",
  "p.task.s_blocked": "Bloqueada",
  "p.task.links_intro":
    "Dónde está esta tarea en otras herramientas, como la incidencia de Jira o Linear con la que se sigue. La dirección se guarda y se muestra; nunca se consulta y no se sincroniza nada.",
  "p.task.back_board": "Volver al tablero",
  "p.task.could_not_load_links":
    "No se pudieron cargar los vínculos de la tarea con otras tareas.",
  "p.task.could_not_load_tasks":
    "No se pudieron cargar las tareas del plan para vincularlas.",
  "p.task.edit_heading": "Editar la tarea",
  "p.task.edit_note":
    "El estado cambia en el tablero. El sprint, el componente y la tarea principal se mantienen como están.",
  "p.task.f_title": "Título",
  "p.task.f_assignee":
    "Asignada a (una referencia person: o worker:; déjelo vacío para nadie)",
  "p.task.f_points":
    "Puntos de historia (un número entero de 0 a 100; déjelo vacío para ninguno)",
  "p.task.f_due": "Fecha de vencimiento (déjela vacía para ninguna)",
  "p.task.save": "Guardar la tarea",
  "p.task.saved": "Se guardó la tarea.",
  "p.task.problem_title_required": "Dé un título a la tarea.",
  "p.task.problem_title_too_long": "El título es demasiado largo.",
  "p.task.problem_assignee":
    "Indique la persona asignada como referencia person: o worker:, o déjelo vacío.",
  "p.task.problem_points_whole": "Los puntos de historia son un número entero.",
  "p.task.problem_points_range":
    "Los puntos de historia deben estar entre 0 y 100.",
  "p.task.problem_due": "La fecha de vencimiento no es una fecha.",
  "p.task.relations_heading": "Vínculos con otras tareas",
  "p.task.relations_intro":
    "«Bloquea» impide terminar la otra tarea mientras esta siga abierta. «Va antes de» exige que la otra tarea venza después de esta. Cuando se termina una tarea, también se termina toda tarea que sea un duplicado de ella. «Relacionada con» y «Copiada de» son solo informativos.",
  "p.task.no_relations": "Esta tarea no tiene vínculos con otras tareas.",
  "p.task.rel_related": "Relacionada con",
  "p.task.rel_blocks": "Bloquea",
  "p.task.rel_blocked_by": "Bloqueada por",
  "p.task.rel_precedes": "Va antes de",
  "p.task.rel_follows": "Va después de",
  "p.task.rel_duplicates": "Duplicado de",
  "p.task.rel_duplicated_by": "Duplicada por",
  "p.task.rel_copied_from": "Copiada de",
  "p.task.rel_copied_to": "Copiada a",
  "p.task.other_state": "(ahora {status})",
  "p.task.other_state_due": "(ahora {status}, vence {date})",
  "p.task.remove_link": "Quitar",
  "p.task.remove_link_named": "Quitar el vínculo: {link} {title}",
  "p.task.hidden_links_one":
    "{n} vínculo más lleva a una tarea de un plan que usted no puede ver.",
  "p.task.hidden_links_other":
    "{n} vínculos más llevan a tareas de planes que usted no puede ver.",
  "p.task.add_link_heading": "Vincular con otra tarea",
  "p.task.f_kind": "Esta tarea",
  "p.task.f_other": "Otra tarea",
  "p.task.choose_task": "Elija una tarea",
  "p.task.add_link": "Vincular las tareas",
  "p.task.link_added": "Se añadió el vínculo.",
  "p.task.link_removed": "Se quitó el vínculo.",
  "p.task.problem_choose_task": "Elija la otra tarea.",
  "p.task.problem_self": "Una tarea no puede vincularse consigo misma.",
  "p.task.problem_exists": "Esta tarea ya tiene ese vínculo.",
  "p.task.problem_cycle":
    "La otra tarea ya tiene ese vínculo con esta; las dos se esperarían mutuamente.",
  "p.task.problem_precedes_dates":
    "«Va antes de» exige que la otra tarea venza después de esta: cambie antes una fecha de vencimiento.",
  "p.task.refused_forbidden": "No puede cambiar las tareas de este plan.",
  "p.task.refused_gone":
    "Esta tarea ya no existe o no puede verla; se ha vuelto a leer.",
  "p.task.refused_link_gone":
    "Ese vínculo ya se había quitado; se han vuelto a leer los vínculos.",
  "p.task.refused_link_exists":
    "Ese vínculo ya existe; se han vuelto a leer los vínculos.",
  "p.task.refused_edit_invalid":
    "El servicio no aceptó el cambio: revise los campos. Una nueva fecha de vencimiento también debe mantener en orden cada vínculo «Va antes de».",
  "p.task.refused_link_invalid":
    "El servicio no aceptó el vínculo: puede que la otra tarea ya no exista, un vínculo «Bloquea» o «Va antes de» no puede formar un círculo, y «Va antes de» exige que la otra tarea venza después de esta.",
  "p.me.notifications_link": "Mis notificaciones",
  "p.me.notifications_note":
    "— su bandeja de entrada, de qué se le avisa y adónde van las copias.",
  "p.notifications.title_ppm": "Mis notificaciones — PPM",
  "p.notifications.heading": "Mis notificaciones",
  "p.notifications.back": "← Mi trabajo",
  "p.notifications.intro":
    "Todo lo que hay aquí es solo suyo: nadie más puede leer su bandeja de entrada, sus preferencias ni sus destinos.",
  "p.notifications.loading": "Cargando…",
  "p.notifications.inbox_heading": "Bandeja de entrada",
  "p.notifications.unread_only": "Mostrar solo las no leídas",
  "p.notifications.unread_count_one": "{n} notificación sin leer.",
  "p.notifications.unread_count_other": "{n} notificaciones sin leer.",
  "p.notifications.inbox_tz": "Los días se cuentan en su zona horaria, {tz}.",
  "p.notifications.inbox_empty": "No hay nada en su bandeja de entrada.",
  "p.notifications.inbox_unread_empty": "No hay nada sin leer.",
  "p.notifications.status_unread": "Sin leer",
  "p.notifications.status_read": "Leída",
  "p.notifications.digest_count_one": "resumen diario de {n} notificación",
  "p.notifications.digest_count_other": "resumen diario de {n} notificaciones",
  "p.notifications.open_subject": "Abrir el plan",
  "p.notifications.mark_read": "Marcar como leída",
  "p.notifications.mark_read_named": "Marcar como leída: {message}",
  "p.notifications.marked_read": "Marcada como leída.",
  "p.notifications.could_not_load_inbox":
    "No se pudo cargar su bandeja de entrada.",
  "p.notifications.could_not_mark":
    "No se pudo marcar la notificación como leída.",
  "p.notifications.prefs_heading": "De qué se le avisa",
  "p.notifications.prefs_intro":
    "Active o desactive cada tipo de notificación para la bandeja de entrada y para cada tipo de destino. Una copia va a un webhook, un chat o un correo electrónico solo cuando ha añadido abajo un destino de ese tipo. Un resumen diario reúne las notificaciones de un día sobre una misma cosa en una sola línea de la bandeja; nada se retiene ni se descarta.",
  "p.notifications.pref_daily": "Resumen diario en la bandeja de entrada",
  "p.notifications.pref_saved": "Se guardó su preferencia.",
  "p.notifications.pref_reset_done":
    "De vuelta al valor predeterminado: activadas, según llegan.",
  "p.notifications.reset": "Volver al valor predeterminado",
  "p.notifications.reset_named": "Volver al valor predeterminado: {kind}",
  "p.notifications.could_not_load_prefs":
    "No se pudieron cargar sus preferencias.",
  "p.notifications.could_not_save_pref": "No se pudo guardar la preferencia.",
  "p.notifications.sinks_heading": "Adónde van las copias",
  "p.notifications.sinks_intro":
    "Un destino recibe una copia de cada una de sus notificaciones, que lleva solo el texto de la propia notificación. La dirección es secreta: una vez guardada, solo se muestra acortada. Un destino lento o averiado nunca retrasa la bandeja de entrada; un envío fallido se reintenta cuatro veces más durante unas dos horas y media y después se abandona.",
  "p.notifications.no_sinks":
    "No tiene destinos; sus notificaciones van solo a la bandeja de entrada.",
  "p.notifications.chat_named": "Chat de {name}",
  "p.notifications.sink_sending": "Enviando",
  "p.notifications.sink_paused": "En pausa",
  "p.notifications.show_deliveries": "Entregas recientes",
  "p.notifications.deliveries_named": "Entregas recientes: {destination}",
  "p.notifications.remove_sink": "Quitar",
  "p.notifications.remove_sink_named": "Quitar: {destination}",
  "p.notifications.sink_removed": "Se quitó el destino.",
  "p.notifications.deliveries_heading": "Entregas recientes a {destination}",
  "p.notifications.deliveries_note":
    "Aquí no se muestra por qué falló un envío, ya que el motivo puede citar la dirección secreta.",
  "p.notifications.no_deliveries": "Aún no se ha enviado nada a este destino.",
  "p.notifications.attempts_one": "{n} intento",
  "p.notifications.attempts_other": "{n} intentos",
  "p.notifications.delivered_at": "enviado {date}",
  "p.notifications.next_attempt": "próximo intento {date}",
  "p.notifications.close_deliveries": "Cerrar las entregas",
  "p.notifications.add_heading": "Añadir un destino",
  "p.notifications.f_kind": "Tipo",
  "p.notifications.f_destination_email": "Dirección de correo electrónico",
  "p.notifications.f_destination_url":
    "Dirección web (https, a un servidor público)",
  "p.notifications.f_chat_id": "Chat de Telegram (su número, o @ y su nombre)",
  "p.notifications.add_sink": "Añadir el destino",
  "p.notifications.sink_added": "Se añadió el destino: {destination}.",
  "p.notifications.could_not_load_sinks": "No se pudieron cargar sus destinos.",
  "p.notifications.could_not_load_deliveries":
    "No se pudieron cargar las entregas.",
  "p.notifications.could_not_save_sink": "No se pudo añadir el destino.",
  "p.notifications.could_not_remove_sink": "No se pudo quitar el destino.",
  "p.notifications.problem_too_many":
    "Ya tiene diez destinos, el máximo permitido: quite uno primero.",
  "p.notifications.problem_destination_required":
    "Indique la dirección a la que enviar.",
  "p.notifications.problem_email_invalid":
    "Eso no es una dirección de correo electrónico.",
  "p.notifications.problem_url_invalid": "Eso no es una dirección web.",
  "p.notifications.problem_url_not_https":
    "La dirección web debe empezar por https://.",
  "p.notifications.problem_url_credentials":
    "La dirección web no debe llevar nombre de usuario ni contraseña.",
  "p.notifications.problem_url_private":
    "La dirección web debe ser un servidor público, no uno local, privado o interno.",
  "p.notifications.problem_url_too_long":
    "La dirección web es demasiado larga.",
  "p.notifications.problem_chat_id_required":
    "Un destino de Telegram necesita el chat en el que publica.",
  "p.notifications.refused_forbidden":
    "No puede cambiar estas preferencias de notificación.",
  "p.notifications.refused_note_gone":
    "Esa notificación ya no existe o no es suya; se ha vuelto a leer la bandeja de entrada.",
  "p.notifications.refused_sink_gone":
    "Ese destino ya se había quitado; se han vuelto a leer sus destinos.",
  "p.notifications.refused_not_addressable":
    "Su identidad de inicio de sesión no es una a la que se puedan dirigir notificaciones, así que aquí no hay nada que mostrar.",
  "p.notifications.refused_pref_invalid":
    "El servicio no aceptó esa preferencia; se han vuelto a leer sus preferencias.",
  "p.notifications.refused_sink_invalid":
    "El servicio no aceptó el destino: debe ser una dirección de correo electrónico o una dirección web https a un servidor público; uno de Telegram necesita su chat; y puede haber diez como máximo.",
  "l.enum.notify_kind.approval_decided":
    "Se decidió una aprobación que usted pidió",
  "l.enum.notify_kind.approval_requested": "Se le pide que apruebe algo",
  "l.enum.notify_kind.auto_watch":
    "Seguir automáticamente aquello en lo que trabaja",
  "l.enum.notify_kind.automation": "Una regla le avisa",
  "l.enum.notify_kind.comment": "Comentarios",
  "l.enum.notify_kind.escalation": "Escalados",
  "l.enum.notify_kind.exceptions_digest": "El resumen semanal de excepciones",
  "l.enum.notify_kind.reschedule_proposed": "Se propone una nueva fecha",
  "l.enum.notify_kind.review_invited": "Se le invita a revisar",
  "l.enum.notify_kind.review_submitted": "Se presentó una revisión",
  "l.enum.notify_kind.scheduled": "Un recordatorio programado",
  "l.enum.notify_kind.status_update_due": "Toca un informe de estado",
  "l.enum.notify_kind.task_assigned": "Se le asignó una tarea",
  "l.enum.notify_kind.watch_update": "Algo que sigue ha cambiado",
  "l.enum.notify_kind.process_todo": "Le toca hacer un paso de un proceso",
  "l.enum.notify_kind.process_notice":
    "Un paso de un proceso del que se le avisa",
  "l.enum.notify_kind.break_glass":
    "Se abrió un plan privado en una emergencia",
  "l.enum.notify_channel.in_app": "En la bandeja de entrada",
  "l.enum.notify_channel.webhook": "Por webhook firmado",
  "l.enum.notify_channel.chat": "En un chat",
  "l.enum.notify_channel.email": "Por correo electrónico",
  "l.enum.sink_kind.email": "Correo electrónico",
  "l.enum.sink_kind.webhook": "Webhook firmado",
  "l.enum.delivery_status.pending": "Esperando el envío",
  "l.enum.delivery_status.sent": "Enviado",
  "l.enum.delivery_status.failed": "Abandonado",
  "l.enum.delivery_status.skipped":
    "No enviado: el destino o la notificación ya no existe",
  "l.link.kind_docs": "Documentos",
  "l.link.kind_repo": "Repositorio de código",
  "l.link.kind_ticket": "Ticket",
  "l.link.kind_design": "Diseño",
  "l.link.kind_chat": "Chat",
  "l.link.kind_tracker": "Gestor de trabajo",
  "l.link.kind_other": "Otro",
  "l.link.system_other": "Otra herramienta",
  "l.fw.method_waterfall": "Cascada",
  "l.fw.method_hybrid": "Híbrido",
  "l.fw.method_other": "Otro",
  "l.fw.model_sponsor": "Decide el patrocinador",
  "l.fw.model_steering_committee": "Comité de dirección",
  "l.fw.model_product_owner": "Propietario del producto",
  "l.fw.model_delegated": "Delegado",
  "l.fw.model_consensus": "Consenso",
  "l.fw.model_other": "Otro",
  "p.frameworks.wow": "Formas de trabajo",
  "p.frameworks.wow_ppm": "Formas de trabajo — PPM",
  "p.frameworks.gov": "Marco de gobernanza",
  "p.frameworks.gov_ppm": "Marco de gobernanza — PPM",
  "p.frameworks.intro_wow":
    "Cómo trabaja el equipo: método, duración de la iteración, límite de trabajo en curso. Un plan sigue uno, compartido o propio; si no tiene ninguno, hereda el del plan superior más cercano.",
  "p.frameworks.intro_gov":
    "Cómo se gobierna el trabajo: quién decide, con qué frecuencia se revisa y la ruta de escalado que siguen los escalados. Un plan sigue uno, compartido o propio; si no tiene ninguno, hereda el del plan superior más cercano.",
  "p.frameworks.back": "Volver al plan",
  "p.frameworks.loading": "Cargando…",
  "p.frameworks.could_not_load": "No se pudo cargar.",
  "p.frameworks.could_not": "No se pudo hacer: {reason}",
  "p.frameworks.in_force": "En vigor para este plan",
  "p.frameworks.source_own": "definido en este plan",
  "p.frameworks.source_inherited": "heredado de un plan superior",
  "p.frameworks.source_none": "ninguno",
  "p.frameworks.from_plan": "ver ese plan",
  "p.frameworks.scope_plan": "escrito para un plan",
  "p.frameworks.scope_org": "compartido por la organización",
  "p.frameworks.none": "No hay nada en vigor.",
  "p.frameworks.edit": "Editar",
  "p.frameworks.unfollow": "Dejar de seguir (volver a heredar)",
  "p.frameworks.follow_heading": "Seguir otro distinto",
  "p.frameworks.no_others": "No hay ningún otro compartido que seguir.",
  "p.frameworks.choose": "Compartido o existente",
  "p.frameworks.follow": "Seguir este",
  "p.frameworks.write_new": "Escribir uno nuevo para este plan",
  "p.frameworks.followed": "Ahora se sigue.",
  "p.frameworks.unfollowed": "Se dejó de seguir; el plan vuelve a heredar.",
  "p.frameworks.saved": "Guardado.",
  "p.frameworks.new_heading": "Uno nuevo para este plan",
  "p.frameworks.edit_heading": "Editar este",
  "p.frameworks.f_name": "Nombre",
  "p.frameworks.f_description": "Descripción",
  "p.frameworks.save": "Guardar",
  "p.frameworks.cancel": "Cancelar",
  "p.frameworks.methodology": "Método",
  "p.frameworks.cadence_days": "Duración de la iteración (días)",
  "p.frameworks.wip_limit": "Límite de trabajo en curso",
  "p.frameworks.ready": "Definición de preparado",
  "p.frameworks.done": "Definición de hecho",
  "p.frameworks.ceremonies": "Ceremonias",
  "p.frameworks.decision_model": "Modelo de decisión",
  "p.frameworks.review_cadence": "Revisar cada (días)",
  "p.frameworks.reporting_cadence": "Informar al nivel superior cada (días)",
  "p.frameworks.path": "Ruta de escalado",
  "p.frameworks.rights": "Derechos de decisión",
  "p.frameworks.forums": "Foros",
  "p.frameworks.days": "{n} días",
  "p.frameworks.path_level": "Nivel",
  "p.frameworks.path_body": "Quién lo recibe",
  "p.frameworks.path_trigger": "Qué lo envía allí",
  "p.frameworks.path_add": "Añadir un nivel",
  "p.frameworks.path_remove": "Quitar el nivel {n}",
  "p.frameworks.name_problem": "Asígnele un nombre de 1 a 120 caracteres.",
  "p.frameworks.number_problem":
    "Los días son números enteros de 1 a 366 y el límite, de 1 a 1000.",
  "p.frameworks.path_problem":
    "Cada nivel de escalado es un número entero distinto de 1 a 9, con quién lo recibe y qué lo envía allí.",
  "p.frameworks.confirm_shared_one":
    "Es compartido: al guardar, también cambia para {n} plan más.",
  "p.frameworks.confirm_shared_other":
    "Es compartido: al guardar, también cambia para otros {n} planes.",
  "p.frameworks.confirm_problem_one":
    "Marque la casilla para confirmar el cambio para {n} plan más.",
  "p.frameworks.confirm_problem_other":
    "Marque la casilla para confirmar el cambio para otros {n} planes.",
  "p.frameworks.deleted": "Se eliminó {name}.",
  "p.frameworks.delete": "Eliminar {name} (nadie lo sigue)",
  "p.frameworks.text_problem":
    "Una definición tiene como máximo 10000 caracteres.",
  "p.frameworks.practice_problem":
    "Cada ceremonia o foro necesita un nombre y su frecuencia, y hay un máximo de 20.",
  "p.frameworks.rights_problem":
    "Cada derecho de decisión necesita la decisión y quién la toma, y hay un máximo de 20.",
  "p.frameworks.practice_name": "Nombre",
  "p.frameworks.practice_cadence": "Frecuencia",
  "p.frameworks.practice_purpose": "Para qué sirve",
  "p.frameworks.row_remove": "Quitar la fila {n}",
  "p.frameworks.ceremony_add": "Añadir una ceremonia",
  "p.frameworks.forum_add": "Añadir un foro",
  "p.frameworks.right_decision": "Decisión",
  "p.frameworks.right_decides": "Quién decide",
  "p.frameworks.right_consulted": "A quién se consulta primero",
  "p.frameworks.right_add": "Añadir un derecho de decisión",
  "p.links.links": "Enlaces",
  "p.links.links_ppm": "Enlaces — PPM",
  "p.links.intro":
    "Dónde están los documentos y el trabajo de este plan, incluido su tablero de Jira, Trello, Asana, Linear o Monday. La dirección se guarda y se muestra; nunca se consulta.",
  "p.links.back": "Volver al plan",
  "p.links.loading": "Cargando…",
  "p.links.could_not_load": "No se pudieron cargar los enlaces.",
  "p.links.could_not": "No se pudo hacer: {reason}",
  "p.links.none": "Aún no hay enlaces.",
  "p.links.opens_new": "se abre en una pestaña nueva",
  "p.links.remove": "Quitar {label}",
  "p.links.added": "Se añadió {label}.",
  "p.links.removed": "Se quitó {label}.",
  "p.links.add_heading": "Añadir un enlace",
  "p.links.add": "Añadir el enlace",
  "p.links.f_label": "Etiqueta",
  "p.links.f_url": "Dirección (http o https)",
  "p.links.f_kind": "Tipo",
  "p.links.f_system": "Herramienta externa",
  "p.links.f_id": "Su identificador allí",
  "p.links.url_problem":
    "La dirección debe empezar por http:// o https:// e indicar un host.",
  "p.links.label_problem": "Asígnele una etiqueta.",
  "p.links.system_problem":
    "Un enlace de gestor de trabajo debe indicar su herramienta.",
  "p.links.system_kind_problem":
    "Solo un enlace de gestor de trabajo o de ticket indica una herramienta.",
  "p.links.id_problem":
    "Un identificador necesita una herramienta y no tiene espacios.",
  "p.links.domain_problem":
    "Los enlaces de esa herramienta están en {domains}.",
  "p.escalations.intro":
    "Remita algo a un nivel superior cuando el proyecto no pueda o no deba resolverlo por sí mismo. Se plantea en un nivel de la ruta de escalado del marco de gobernanza del proyecto, se reconoce, se eleva un nivel si nadie responde y termina resuelto o retirado. La persona indicada como responsable y la persona que lo planteó reciben un aviso en su bandeja de entrada (no se envía correo electrónico).",
  "p.escalations.register_intro":
    "Todos los escalados de los planes que puede leer.",
  "p.escalations.none": "Aún no hay escalados.",
  "p.escalations.none_match": "Ningún escalado coincide.",
  "p.escalations.raise_heading": "Plantear un escalado",
  "p.escalations.f_title": "Título",
  "p.escalations.f_reason": "Por qué",
  "p.escalations.f_dimension": "Qué tolerancia",
  "p.escalations.f_level": "Nivel (en blanco para el primer nivel de la ruta)",
  "p.escalations.f_to": "Escalar a (en blanco para usar la ruta)",
  "p.escalations.f_owner": "Persona destinataria (person:… o worker:…)",
  "p.escalations.f_respond": "Respuesta necesaria antes del",
  "p.escalations.f_description": "Qué y por qué",
  "p.escalations.raise": "Plantearlo",
  "p.escalations.raised": "Se planteó {title}.",
  "p.escalations.acknowledge": "Reconocer",
  "p.escalations.take_up": "Elevarlo un nivel",
  "p.escalations.take_up_note": "Por qué se eleva",
  "p.escalations.resolve": "Resolver",
  "p.escalations.outcome": "Resultado",
  "p.escalations.response": "Respuesta",
  "p.escalations.withdraw": "Retirar",
  "p.escalations.done": "Hecho.",
  "p.escalations.could_not": "No funcionó: {reason}",
  "p.error.status_401": "No ha iniciado sesión o su sesión ha terminado.",
  "p.error.status_403": "No tiene permiso para hacer eso.",
  "p.error.status_404": "No se encontró o no puede verlo.",
  "p.error.status_409":
    "Entra en conflicto con el estado actual: otra persona lo cambió o lo decidió, o ya está hecho.",
  "p.error.status_412":
    "Cambió desde que lo cargó: recargue e inténtelo de nuevo.",
  "p.error.status_413": "Es demasiado grande.",
  "p.error.status_422": "Algo que introdujo no se acepta.",
  "p.error.status_429":
    "Demasiadas solicitudes: espere un momento e inténtelo de nuevo.",
  "p.bridge.open_workforce": "Abrir {name} en la planificación de personal",
  "p.bridge.open_contacts": "Abrir {name} en contactos",
  "p.error.client": "Se rechazó la solicitud.",
  "p.error.server": "El servicio tuvo un problema: inténtelo de nuevo.",
  "p.escalations.note_problem": "Escriba al menos 10 caracteres.",
  "p.escalations.show": "Mostrar {title}",
  "p.escalations.hide": "Ocultar {title}",
  "p.escalations.history": "Historial",
  "p.escalations.status": "Estado",
  "p.escalations.plan": "Plan",
  "p.escalations.level_col": "Nivel",
  "p.escalations.age": "Antigüedad",
  "p.escalations.respond_by": "Respuesta necesaria antes del",
  "p.escalations.filter": "Mostrar",
  "p.escalations.all": "Todos",
  "p.escalations.live": "Aún abiertos",
  "p.escalations.overdue_count": "{n} vencidos",
  "p.escalations.loading": "Cargando…",
  "p.escalations.could_not_load": "No se pudieron cargar los escalados",
  "p.escalations.back": "← Plan",
  "p.escalations.register_link": "Todos los escalados",
  "p.tolerances.needs_escalation":
    "Esto supera la tolerancia y nadie lo ha escalado.",
  "p.tolerances.raise_one": "Plantear un escalado",
  "p.tolerances.escalate_all": "Escalar todas las tolerancias en rojo",
  "p.tolerances.escalate_all_intro":
    "Cada dimensión que supera la tolerancia y aún no se ha escalado recibe un escalado en el primer nivel de la ruta de escalado. Nada se escala por sí solo.",
  "p.tolerances.escalated_one": "Se planteó {n} escalado.",
  "p.tolerances.escalated_other": "Se plantearon {n} escalados.",
  "p.tolerances.escalations_one": "{n} escalado abierto",
  "p.tolerances.escalations_other": "{n} escalados abiertos",
  "p.resources.rbs.by_role": "Por rol",
  "p.resources.rbs.show_tasks": "Mostrar las tareas abiertas de {role}",
  "p.resources.rbs.hide_tasks": "Ocultar las tareas abiertas de {role}",
  "p.resources.rbs.no_open_tasks": "no hay tareas abiertas",
  "p.resources.rbs.tasks_in_plans_you_cannot_read":
    "No se muestran {count} en planes que usted no puede leer.",
  "p.resources.rbs.only_the_first_500_tasks":
    "Solo se leyeron las primeras 500 tareas, por lo que esta lista puede estar incompleta.",
  "p.resources.rbs.could_not_load_tasks":
    "No se pudieron cargar las tareas: {reason}",
  "p.resources.rbs.by_skill": "Por competencia",
  "p.resources.rbs.demand_by_skill": "Demanda por competencia",
  "p.resources.rbs.demand_fte": "Demanda FTE",
  "p.resources.rbs.no_roles_in_this_window": "No hay roles en esta ventana.",
  "p.resources.rbs.no_skill_demand_in_this":
    "No hay demanda de competencias en esta ventana.",
  "p.resources.rbs.not_known": "desconocido",
  "p.resources.rbs.people": "Personas",
  "p.resources.rbs.resource_breakdown": "Desglose de recursos",
  "p.resources.rbs.resource_breakdown_ppm": "Desglose de recursos — PPM",
  "p.resources.rbs.role": "Rol",
  "p.resources.rbs.roles_with_people_demand_and":
    "Roles con personas, demanda y oferta",
  "p.resources.rbs.skill": "Competencia",
  "p.resources.rbs.supply_fte": "Oferta FTE",
  "p.resources.rbs.you_can_see_headcounts_only":
    "Solo puede ver el número de personas, no los nombres.",
  "p.resources.resources": "Recursos",
  "p.resources.resources_ppm": "Recursos — PPM",
  "p.resources.role": "Rol",
  "p.resources.roles_and_totals_only": "solo roles y totales",
  "p.resources.roles_asked_for_with_no": "Roles solicitados aún sin persona",
  "p.resources.short_spare_balanced_supply_not":
    "▼ déficit · ▲ excedente · ● equilibrado · ? oferta desconocida. El sombreado solo indica el tamaño de la brecha.",
  "p.resources.show": "Mostrar",
  "p.resources.skill": "Competencia",
  "p.resources.skill_demand_only": "Competencia (solo demanda)",
  "p.resources.supply_fte": "Oferta FTE",
  "p.resources.to": "Hasta",
  "p.resources.unclassified": "sin clasificar",
  "p.resources.week": "Semana",
  "p.resources.what_the_plans_ask_for":
    "Lo que piden los planes frente a lo que pueden aportar las personas de cada rol, en equivalentes a tiempo completo. Las reservas propuestas se mantienen aparte y nunca se cuentan.",
  "p.reviews.accept": "Aceptar",
  "p.reviews.accepted": "aceptada",
  "p.reviews.actions": "Acciones",
  "p.reviews.advance": "Avanzar",
  "p.reviews.all": "todas",
  "p.reviews.collaborative_review": "Revisión colaborativa",
  "p.reviews.consensus": "Consenso",
  "p.reviews.consensus_for_this_subject": "Consenso para este asunto",
  "p.reviews.decline": "Rechazar",
  "p.reviews.declined": "rechazada",
  "p.reviews.delegate": "Delegar",
  "p.reviews.delegate_a_specific_item_to":
    "Delegue un elemento concreto en el experto adecuado. Solo un revisor que haya aceptado puede emitir un veredicto, por lo que una invitación sin respuesta nunca se convierte en evidencia.",
  "p.reviews.due": "vence",
  "p.reviews.e_g_clinical_safety": "p. ej., seguridad clínica",
  "p.reviews.expertise": "especialidad",
  "p.reviews.expertise_2": "Especialidad",
  "p.reviews.expired": "caducada",
  "p.reviews.external": "externa",
  "p.reviews.hold": "En espera",
  "p.reviews.idea": "idea",
  "p.reviews.internal": "interna",
  "p.reviews.invited": "invitado",
  "p.reviews.no_review_invitations_yet": "Aún no hay invitaciones de revisión.",
  "p.reviews.no_score": "sin puntuación",
  "p.reviews.person_uuid": "person:<uuid>",
  "p.reviews.plan": "plan",
  "p.reviews.proposal": "propuesta",
  "p.reviews.reject": "Rechazar",
  "p.reviews.reviewer": "revisor",
  "p.reviews.reviewer_2": "Revisor",
  "p.reviews.reviews_ppm": "Revisiones — PPM",
  "p.reviews.scope": "ámbito",
  "p.reviews.scope_2": "Ámbito",
  "p.reviews.status": "estado",
  "p.reviews.status_2": "Estado",
  "p.reviews.subject": "asunto",
  "p.reviews.subject_2": "Asunto",
  "p.reviews.subject_pid": "pid del asunto",
  "p.reviews.submitted": "enviada",
  "p.reviews.submitted_outstanding_declined_exter":
    "{submitted} enviadas · {outstanding} pendientes · {declined} rechazadas · {external_submitted} externas",
  "p.reviews.uuid": "uuid",
  "p.reviews.verdict": "Veredicto",
  "p.reviews.withdraw": "Retirar",
  "p.risk.category": "Categoría",
  "p.risk.concentration": "Concentración",
  "p.risk.escalated": "Escalado",
  "p.risk.exposure": "Exposición",
  "p.risk.exposure_25_of_estate": "— exposición {exposure} (≥ 25 % del total)",
  "p.risk.heatmap": "Mapa de calor",
  "p.risk.item": "Elemento",
  "p.risk.materialized": "Materializado",
  "p.risk.max_estate_max_single_risk":
    "— máximo total {v}, máximo de un solo riesgo {v2}",
  "p.risk.no_open_risks": "No hay riesgos abiertos.",
  "p.risk.open_exposure": "Exposición abierta",
  "p.risk.open_risks_estate_open_exposure":
    "riesgos abiertos · exposición abierta total",
  "p.risk.overdue_reviews": "Revisiones vencidas",
  "p.risk.portfolio": "Cartera",
  "p.risk.posture_by_portfolio": "Situación por cartera",
  "p.risk.ppm": "{t} — PPM",
  "p.risk.probability_impact": "probabilidad ↓ / impacto →",
  "p.risk.review_due": "(revisión prevista el {review_date})",
  "p.risk.risk": "Riesgo",
  "p.risk.top_risks": "Riesgos principales",
  "p.risk.unassigned": "(sin asignar)",
  "p.scenarios.alignment": "Alineación",
  "p.scenarios.cap": "Tope",
  "p.scenarios.compare": "Comparar",
  "p.scenarios.currency": "Moneda",
  "p.scenarios.delta_b_a": "Diferencia (b − a)",
  "p.scenarios.evaluation": "Evaluación",
  "p.scenarios.feasible": "Viable",
  "p.scenarios.no": "no",
  "p.scenarios.open_exposure": "Exposición abierta",
  "p.scenarios.pick": "elegir…",
  "p.scenarios.planned": "Previsto {currency}",
  "p.scenarios.scenario": "Escenario",
  "p.scenarios.scenarios_ppm": "Escenarios — PPM",
  "p.scenarios.status": "Estado",
  "p.scenarios.work_item_pids_space_comma":
    "pid de elementos de trabajo (separados por espacios o comas)",
  "p.scenarios.yes": "sí",
  "p.security.exposure": "Exposición",
  "p.security.item": "Elemento",
  "p.security.late_stage_items_with_no":
    "Elementos en etapa avanzada sin riesgo de seguridad registrado",
  "p.security.no_security_risks_recorded":
    "No hay riesgos de seguridad registrados.",
  "p.security.none_every_late_stage_item":
    "Ninguno: todos los elementos en etapa avanzada tienen un riesgo de seguridad registrado.",
  "p.security.open_exposure": "{note} · exposición abierta {open_exposure}",
  "p.security.owner": "Responsable",
  "p.security.ppm": "{t} — PPM",
  "p.security.risk": "Riesgo",
  "p.security.security_risk_register": "Registro de riesgos de seguridad",
  "p.security.status": "Estado",
  "p.signin.check_your_email_for_a":
    "Revise su correo electrónico para ver el enlace de inicio de sesión.",
  "p.signin.could_not_send_the_sign":
    "No se pudo enviar el enlace de inicio de sesión. Inténtelo de nuevo.",
  "p.signin.email": "Correo electrónico",
  "p.signin.send_magic_link": "Enviar enlace mágico",
  "p.signin.sign_in": "Iniciar sesión",
  "p.technology.as_of": "a fecha de {as_of} · {convention}",
  "p.technology.as_of_edges": "a fecha de {as_of} · {edges} relaciones",
  "p.technology.calendar_days": "{median_lead_days} días naturales",
  "p.technology.completed_before_timing_existed_unti":
    "· {undated_completions} completados antes de que existiera la medición de tiempos (sin tiempo)",
  "p.technology.cross_portfolio_edges": "Relaciones entre carteras",
  "p.technology.delivery_flow": "Flujo de entrega",
  "p.technology.dependency_risk": "Riesgo de dependencias",
  "p.technology.dependents": "Dependientes",
  "p.technology.edges_importing_risk_from_a":
    "Relaciones que importan riesgo de una predecesora en rojo",
  "p.technology.exposure": "Exposición",
  "p.technology.item": "Elemento",
  "p.technology.kind": "Tipo",
  "p.technology.median_lead": "Plazo mediano:",
  "p.technology.milestones_completed": "Hitos completados",
  "p.technology.month": "Mes",
  "p.technology.most_depended_on_single_point":
    "Los más dependidos (candidatos a punto único de fallo)",
  "p.technology.no_dependencies_recorded_yet":
    "Aún no hay dependencias registradas.",
  "p.technology.no_tech_debt_risks_recorded":
    "No hay riesgos de deuda técnica (tech_debt) registrados.",
  "p.technology.no_tech_tags_recorded_yet":
    "Aún no hay etiquetas `tech:*` registradas.",
  "p.technology.no_timed_completions": "no hay finalizaciones con tiempo",
  "p.technology.no_timed_completions_in_the":
    "No hay finalizaciones con tiempo en la ventana.",
  "p.technology.none_all_edges_stay_within":
    "Ninguna: todas las relaciones quedan dentro de una cartera.",
  "p.technology.none_no_red_predecessors":
    "Ninguna: no hay predecesoras en rojo.",
  "p.technology.open_exposure": "{note} · exposición abierta {open_exposure}",
  "p.technology.over_timed_completions":
    "sobre {timed_completions} finalizaciones con tiempo",
  "p.technology.owner": "Responsable",
  "p.technology.ppm": "{t} — PPM",
  "p.technology.predecessor": "Predecesora",
  "p.technology.predecessor_red": "Predecesora (en rojo)",
  "p.technology.ring_votes": "({ring_votes} votos de anillo)",
  "p.technology.risk": "Riesgo",
  "p.technology.status": "Estado (",
  "p.technology.status_2": "Estado",
  "p.technology.successor": "Sucesora",
  "p.technology.successor_at_risk": "Sucesora en riesgo",
  "p.technology.technical_debt_register": "Registro de deuda técnica",
  "p.technology.technology_radar": "Radar tecnológico",
  "p.verify.request_a_new_link": "Solicitar un enlace nuevo",
  "p.verify.sign_in_link": "Enlace de inicio de sesión",
  "p.milestones.milestone_by_due_date_one":
    "{length} hito, por fecha de vencimiento",
  "p.milestones.milestone_by_due_date_other":
    "{length} hitos, por fecha de vencimiento",
  "p.okrs.objective_least_progressed_first_one":
    "{length} objetivo, del menos avanzado al más avanzado",
  "p.okrs.objective_least_progressed_first_other":
    "{length} objetivos, del menos avanzado al más avanzado",
  "p.plans.pid.board.blocked_calendar_day_one":
    "bloqueada {blocked_days} día natural",
  "p.plans.pid.board.blocked_calendar_day_other":
    "bloqueada {blocked_days} días naturales",
  "p.plans.pid.calendar.holiday_one": "{length} festivo: {join}",
  "p.plans.pid.calendar.holiday_other": "{length} festivos: {join}",
  "p.plans.pid.costs.for_across_one":
    "{money} por {hoursText} en {entries} registro.",
  "p.plans.pid.costs.for_across_other":
    "{money} por {hoursText} en {entries} registros.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_one":
    "{plans_without_figures} plan de abajo aún no tiene cifras (sin línea base o sin nada que medir) y no se estima.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_other":
    "{plans_without_figures} planes de abajo aún no tienen cifras (sin línea base o sin nada que medir) y no se estiman.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_one":
    "terminaron sin retroceder nunca ({rework_count} retroceso). Léalo junto al rendimiento: un rendimiento creciente con un índice decreciente significa trabajo que vuelve a sí mismo.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_other":
    "terminaron sin retroceder nunca ({rework_count} retrocesos). Léalo junto al rendimiento: un rendimiento creciente con un índice decreciente significa trabajo que vuelve a sí mismo.",
  "p.plans.pid.flow.of_items_finish_within_this_one":
    "El {percent} de los elementos termina dentro de este plazo, a partir de {sample} elemento terminado",
  "p.plans.pid.flow.of_items_finish_within_this_other":
    "El {percent} de los elementos termina dentro de este plazo, a partir de {sample} elementos terminados",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_one":
    "{length} riesgo afecta al cronograma y al coste: se produce una vez e influye en ambos.",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_other":
    "{length} riesgos afectan al cronograma y al coste: cada uno se produce una vez e influye en ambos.",
  "p.plans.pid.leveling.apply_move_one": "Aplicar {length} movimiento",
  "p.plans.pid.leveling.apply_move_other": "Aplicar {length} movimientos",
  "p.plans.pid.leveling.business_day_one": "{delay_days} día hábil",
  "p.plans.pid.leveling.business_day_other": "{delay_days} días hábiles",
  "p.plans.pid.leveling.other_task_would_be_pushed_one":
    "{length} tarea más se retrasaría como consecuencia",
  "p.plans.pid.leveling.other_task_would_be_pushed_other":
    "{length} tareas más se retrasarían como consecuencia",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_one":
    "No se le muestra {hidden} lista de verificación sobre personas.",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_other":
    "No se le muestran {hidden} listas de verificación sobre personas.",
  "p.plans.pid.schedule_health.preset_thresholds_task_one":
    "· configuración {preset} (umbrales: {source}) · {tasks} tarea",
  "p.plans.pid.schedule_health.preset_thresholds_task_other":
    "· configuración {preset} (umbrales: {source}) · {tasks} tareas",
  "p.plans.pid.schedule_health.task_one": "{length} tarea",
  "p.plans.pid.schedule_health.task_other": "{length} tareas",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_one":
    "{title}: {probability_percent} % de probabilidad de añadir {impact_days} día hábil",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_other":
    "{title}: {probability_percent} % de probabilidad de añadir {impact_days} días hábiles",
  "p.plans.pid.wbs.move_up": "Subir {title}",
  "p.plans.pid.wbs.move_down": "Bajar {title}",
  "p.plans.pid.wbs.moved": "Se movió {title}.",
  "p.plans.pid.wbs.truncated":
    "Se muestran {shown} de {total} tareas: este plan es demasiado grande para listarlo completo, por lo que el esquema de abajo está incompleto.",
  "p.plans.pid.critical_path.too_large":
    "Este plan es demasiado grande para programarlo completo en una sola respuesta.",
  "p.plans.pid.critical_path.show_critical_only":
    "Mostrar solo la parte crítica",
  "p.plans.pid.critical_path.critical_only_notice":
    "Se muestran solo las tareas críticas y casi críticas; el resto del plan queda excluido.",
  "p.plans.pid.earned_value.names_truncated":
    "Este plan es muy grande: las cifras abarcan todas las tareas, pero pueden faltar algunos nombres de tareas abajo.",
  "p.plans.pid.wbs.could_not_move": "No se pudo mover la tarea: {reason}",
  "p.plans.pid.wbs.sub_task_one": "({children} subtarea)",
  "p.plans.pid.wbs.sub_task_other": "({children} subtareas)",
  "p.plans.pid.wbs.links": "Enlaces",
  "p.plans.pid.wbs.link_one": "· {links} enlace",
  "p.plans.pid.wbs.link_other": "· {links} enlaces",
  "p.common.on": "activado",
  "p.common.off": "desactivado",
  "p.common.pre_gate": "antes de la fase",
  "p.common.role_word": "rol",
  "p.common.person_word": "persona",
  "c.NetworkDiagram.task_fallback": "tarea",
  "c.NetworkDiagram.task_summary":
    "{title}, del {early_start} al {early_finish}, {total_float} días hábiles de holgura",
  "c.NetworkDiagram.task_summary_critical":
    "{title}, del {early_start} al {early_finish}, {total_float} días hábiles de holgura, en la ruta crítica",
  "c.NetworkDiagram.lag_one": ", desfase de {lag_days} día hábil",
  "c.NetworkDiagram.lag_other": ", desfase de {lag_days} días hábiles",
  "c.KpiCard.direction_higher": "mayor",
  "c.KpiCard.direction_lower": "menor",
  "c.CalendarMonth.legend":
    "– día no hábil   ✕ festivo   A ausencia registrada",
  "c.CalendarMonth.legend_editable":
    " · seleccione un día hábil para marcar o quitar un festivo",
  "c.CostRiskView.run_summary":
    "semilla {seed} · {iterations} iteraciones · {distribution}",
  "c.CostRiskView.dist_pert": "PERT",
  "c.CostRiskView.dist_triangular": "Triangular",
  "c.CostRiskView.risks_without_cost_one":
    "{count} riesgo del registro no tiene campos de coste y no forma parte de esto.",
  "c.CostRiskView.risks_without_cost_other":
    "{count} riesgos del registro no tienen campos de coste y no forman parte de esto.",
  "c.CumulativeFlow.hide_the_numbers": "Ocultar las cifras",
  "c.CumulativeFlow.show_the_numbers": "Mostrar las cifras",
  "c.CostRiskView.scope_all_one": "El {plans} plan que puede leer",
  "c.CostRiskView.scope_all_other": "Los {plans} planes que puede leer",
  "c.CostRiskView.scope_this_plan": "Este plan",
  "c.MilestoneCard.reason": "Motivo",
  "c.MilestoneCard.reason_required_open":
    " (obligatorio: las tareas de las que depende siguen abiertas)",
  "c.MilestoneCard.reason_optional": " (opcional)",
  "c.VarianceTable.matched_of_baselined":
    "{matched} de {baseline_tasks} tareas de la línea base siguen en el plan:",
  "c.VarianceTable.slipped_pulled_on":
    "{slipped} retrasadas, {pulled_in} adelantadas, {on_baseline} según la línea base{unmeasurable}.",
  "c.VarianceTable.cannot_be_measured": ", {unmeasurable} no se pueden medir",
  "c.VarianceTable.added_removed":
    "{added} añadidas y {removed} eliminadas desde entonces.",
  "c.VarianceTable.finish_later_one":
    "El fin del plan es {days} día hábil posterior a la línea base{note}.",
  "c.VarianceTable.finish_later_other":
    "El fin del plan es {days} días hábiles posterior a la línea base{note}.",
  "c.VarianceTable.finish_earlier_one":
    "El fin del plan es {days} día hábil anterior a la línea base{note}.",
  "c.VarianceTable.finish_earlier_other":
    "El fin del plan es {days} días hábiles anterior a la línea base{note}.",
  "c.VarianceTable.check_removed":
    " (revise las tareas eliminadas: eliminar trabajo no es una mejora)",
  "p.plans.pid.critical_path.finishes": "Termina el {date}.",
  "p.plans.pid.critical_path.nothing_scheduled":
    "Todavía no hay nada programado.",
  "p.plans.pid.critical_path.calendar_label": "Calendario: {calendar}.",
  "p.plans.pid.critical_path.calendar_plan": "el de este plan",
  "p.plans.pid.critical_path.calendar_default":
    "predeterminado (todos los días son laborables)",
  "p.plans.pid.critical_path.more_not_shown": " (hay más sin mostrar)",
  "p.plans.pid.critical_path.pushed_later":
    "Sus predecesoras la retrasan {days} días hábiles respecto a lo previsto.",
  "p.plans.pid.earned_value.manual_percent_banner":
    "Parte de esta cifra se basa en un porcentaje introducido por una persona, no en trabajo terminado: {tasks}.",
  "p.plans.pid.earned_value.a_task": "una tarea",
  "p.plans.pid.earned_value.overrun": " (sobrecoste)",
  "p.plans.pid.earned_value.target_budget": "el presupuesto",
  "p.plans.pid.earned_value.target_forecast": "la previsión",
  "p.plans.pid.earned_value.lights_use":
    "Los semáforos usan {which}: {values} (puntos básicos).",
  "p.plans.pid.earned_value.thresholds_default": "los umbrales predeterminados",
  "p.plans.pid.earned_value.thresholds_deployment":
    "los umbrales de esta implantación",
  "p.plans.pid.earned_value.rollup_money":
    "Presupuesto {bac}; planificado {pv}; ganado {ev}; gastado {ac}{unknown}.",
  "p.plans.pid.earned_value.cost_unknown_for":
    " (coste desconocido para {plans} planes)",
  "p.plans.pid.earned_value.cpi_value": "CPI {value}",
  "p.plans.pid.earned_value.cpi_not_stated":
    "no indicado: el coste es desconocido para algunos planes",
  "p.plans.pid.earned_value.weight_points": ", {weight} puntos",
  "p.plans.pid.critical_path.moved_line":
    "{title}: {moved}{finish}{start} días hábiles",
  "p.plans.pid.critical_path.moved_both": "inicio y fin",
  "p.plans.pid.critical_path.moved_start": "inicio",
  "p.plans.pid.critical_path.moved_finish": "fin",
  "p.plans.pid.critical_path.moved_finish_by": " fin {days}",
  "p.plans.pid.critical_path.moved_start_by": " inicio {days}",
  "p.plans.pid.flow.overdue_one":
    "{count} elemento ya ha superado la expectativa. Un elemento envejecido es lo único de esta página cuyo resultado aún puede cambiar.",
  "p.plans.pid.flow.overdue_other":
    "{count} elementos ya han superado la expectativa. Un elemento envejecido es lo único de esta página cuyo resultado aún puede cambiar.",
  "p.plans.pid.flow.classified_with": "Clasificado con {map}:",
  "p.plans.pid.flow.map_override": "una anulación de la implantación",
  "p.plans.pid.flow.map_default": "el mapa predeterminado",
  "p.plans.pid.governance.proposed_not_counted":
    "propuesto{requested} — no contabilizado",
  "p.plans.pid.governance.requested": ", solicitado",
  "p.plans.pid.joint_risk.run_summary":
    "{currency} · semilla {seed} · {iterations} iteraciones · {distribution} · calendario {calendar}",
  "p.plans.pid.joint_risk.dist_pert": "PERT",
  "p.plans.pid.joint_risk.dist_triangular": "Triangular",
  "p.plans.pid.leveling.applied_one":
    "Se aplicó {ok} de {requested} movimiento{refused}.",
  "p.plans.pid.leveling.applied_other":
    "Se aplicaron {ok} de {requested} movimientos{refused}.",
  "p.plans.pid.leveling.applied_refused": "; {refused} rechazados",
  "p.plans.pid.leveling.proposal_summary":
    "Calendario: {calendar} · carga {load} % por tarea · las tareas críticas {critical} moverse",
  "p.plans.pid.leveling.may": "pueden",
  "p.plans.pid.leveling.may_not": "no pueden",
  "p.plans.pid.onboarding.due": ", vence el {date}",
  "p.plans.pid.schedule_risk.risks_without_schedule_one":
    "{count} riesgo del registro no tiene campos de cronograma y no forma parte de esto.",
  "p.plans.pid.schedule_risk.risks_without_schedule_other":
    "{count} riesgos del registro no tienen campos de cronograma y no forman parte de esto.",
  "p.resources.from_date": "desde el {date}",
  "p.resources.to_date": "hasta el {date}",
  "p.kpis.none": "Todavía no hay KPI.",
  "p.kpis.none_with_status": "Todavía no hay KPI con ese estado.",
  "p.milestones.none":
    "Todavía no hay hitos. Añádalos desde la página de gobernanza de un plan.",
  "p.milestones.none_with_status":
    "Todavía no hay hitos con ese estado. Añádalos desde la página de gobernanza de un plan.",
  "p.okrs.none": "Todavía no hay objetivos.",
  "p.okrs.none_for_period": "Todavía no hay objetivos para {period}.",
  "p.me.calendar.own_set": "Su calendario propio está configurado.",
  "p.me.calendar.plan_applies":
    "Sin calendario personal: se le aplica el calendario del plan.",
  "p.me.calendar.leave_counts":
    "Las ausencias registradas a su nombre ya cuentan como días libres.",
  "p.documents.pid.page_title": "{title} — PPM",
  "p.documents.pid.document_fallback": "Documento",
  "p.plans.pid.page_title": "{name} — Main X",
  "p.plans.pid.plan_fallback": "Plan",
  "p.plans.pid.custom_scheme": "Personalizado({name})",
  "p.plans.pid.calendar.own_calendar": "Este plan tiene su propio calendario.",
  "p.plans.pid.calendar.no_calendar":
    "Sin calendario: todos los días son laborables.",
  "p.plans.pid.calendar.each_person_can_set":
    "Cada persona también puede configurar el suyo (",
  "p.reviews.consensus_line":
    "puntuación media: {mean} · mayoría: {majority} · {state}",
  "p.reviews.no_majority": "sin mayoría",
  "p.reviews.everyone_answered": "todos han respondido",
  "p.reviews.still_waiting": "aún en espera",
  "l.announcement_view.just_now": "ahora mismo",
  "l.announcement_view.min_ago": "hace {minutes} min",
  "l.announcement_view.h_ago": "hace {hours} h",
  "l.announcement_view.calendar_days_ago": "hace {days} días naturales",
  "l.announcement_view.scheduled": "programado",
  "l.announcement_view.expired": "Caducado",
  "l.api_ppm.task": "tarea {uid}",
  "l.api_ppm.the_project_calendar_comes_across":
    " El calendario del proyecto se transfiere.",
  "l.api_tba.calendar_days": "{toFixed} días naturales",
  "l.api_tba.work_in_progress_is_growing": "El trabajo en curso está creciendo",
  "l.api_tba.steady_state": "Estado estable",
  "l.api_tba.queue_draining": "La cola se está vaciando",
  "l.api_tba.not_enough_data": "No hay datos suficientes",
  "l.capacity_view.committed_against_a_capacity_as":
    "{committed} % comprometido frente a una capacidad de solo {capacity} %",
  "l.capacity_view.committed": "{committed} % comprometido",
  "l.capacity_view.to": " hasta {to}",
  "l.capacity_view.over_capacity_from_asked_available":
    "por encima de la capacidad desde {from}{v} ({demand} % solicitado, {capacity} % disponible)",
  "l.capacity_view.over_capacity": "por encima de la capacidad",
  "l.capacity_view.committing_it_would_put_them":
    "; comprometerlo la situaría por encima de su capacidad",
  "l.capacity_view.proposed_not_counted":
    "{percent} % propuesto, no contabilizado{v}",
  "l.capacity_view.asked_available":
    "{from}{v}: {demand} % solicitado, {capacity} % disponible",
  "l.ceo_view.no_portfolios_to_report_on":
    "Todavía no hay carteras sobre las que informar.",
  "l.ceo_view.portfolios_exist_but_hold_no":
    "Existen carteras, pero todavía no contienen planes.",
  "l.ceo_view.of_plans_are_red_amber":
    "{red} de {n} planes están en rojo, {amber} en ámbar y {green} en verde.",
  "c.CalendarMonth.not_a_business_day": "no es día hábil",
  "c.CalendarMonth.business_day": "día hábil",
  "c.KpiCard.that_did_not_work": "No funcionó",
  "c.KpiCard.could_not_load_the_history": "No se pudo cargar el historial",
  "c.KpiCard.delete_the_kpi_its_readings":
    "¿Eliminar el KPI «{name}»? Sus lecturas se conservan.",
  "c.LineRange.could_not_save": "No se pudo guardar",
  "c.MilestoneCard.that_did_not_work": "No funcionó",
  "c.MilestoneCard.could_not_load_the_plan":
    "No se pudieron cargar las tareas del plan",
  "c.MilestoneCard.could_not_load_the_history":
    "No se pudo cargar el historial",
  "c.ObjectiveCard.that_did_not_work": "No funcionó",
  "c.ObjectiveCard.delete_the_objective_and_its":
    "¿Eliminar el objetivo «{title}» y sus resultados clave?",
  "c.ObjectiveCard.delete_the_key_result":
    "¿Eliminar el resultado clave «{title}»?",
  "c.RaciMatrix.accountable_moved_from_to_a":
    "La persona que rinde cuentas pasó de {shortRef} a {shortRef2}: una tarea solo tiene una.",
  "c.RiskInputs.could_not_load_the_plan":
    "No se pudieron cargar las tareas del plan",
  "c.RiskInputs.could_not_save": "No se pudo guardar",
  "c.RiskInputs.chance": "{probability_percent} % de probabilidad",
  "c.RiskInputs.adds_business_days_to_a":
    "añade {schedule_impact_days} días hábiles a una tarea",
  "c.RiskInputs.adds": "añade {toFixed} {cost_currency}",
  "c.RiskInputs.not_used_in_the_simulations": "No se usa en las simulaciones",
  "l.cpm_view.cannot_meet_its_constraint_or":
    "No puede cumplir su restricción o fecha límite: faltan {plural}",
  "l.cpm_view.on_the_critical_path_any":
    "En la ruta crítica: cualquier retraso mueve el fin",
  "l.cpm_view.of_float_near_critical": "{d} de holgura (casi crítica)",
  "l.cpm_view.of_float": "{d} de holgura",
  "l.cpm_view.baselined_to": ". Línea base del {start} al {finish}",
  "l.cpm_view.to": "{v}: del {early_start} al {early_finish}. {floatLabel}{v2}",
  "l.cpm_view.has_no_duration_yet": "todavía no tiene duración",
  "l.cpm_view.has_no_start_date_and":
    "no tiene fecha de inicio y no hay nada programado antes",
  "l.cpm_view.has_a_date_outside_the":
    "tiene una fecha fuera del intervalo admitido",
  "l.cpm_view.slipped": "Retrasada",
  "l.cpm_view.finishes_later_than_baselined":
    "termina {d} más tarde que en la línea base",
  "l.cpm_view.pulled_in": "Adelantada",
  "l.cpm_view.finishes_earlier_than_baselined":
    "termina {d} antes que en la línea base",
  "l.cpm_view.on_baseline": "Según la línea base",
  "l.cpm_view.finishes_when_it_was_baselined":
    "termina cuando estaba previsto en la línea base,",
  "l.cpm_view.added": "Añadida",
  "l.cpm_view.not_in_the_baseline_so":
    "no está en la línea base, así que no tiene retraso",
  "l.cpm_view.removed": "Eliminada",
  "l.cpm_view.was_in_the_baseline_and":
    "estaba en la línea base y ya no está; se conservan sus fechas de la línea base",
  "l.cpm_view.cannot_say": "No se puede determinar",
  "l.cpm_view.it_had_no_schedule_when":
    "no tenía cronograma cuando se fijó la línea base",
  "l.cpm_view.it_has_no_schedule_now": "ahora no tiene cronograma",
  "l.document_view.version": "Versión {version} — {v}",
  "l.document_view.since": "{base} desde el {effective_from}",
  "l.document_view.by_version": " por la versión {version}",
  "l.document_view.on": "{base} el {superseded_on}{v}",
  "l.document_view.nothing_in_force_yet": "Todavía no hay nada en vigor",
  "l.document_view.version_in_force": "Versión {in_force_today} en vigor",
  "l.ev_view.nothing_was_planned_to_be":
    "todavía no se había planificado hacer nada",
  "l.ev_view.no_cost_has_been_recorded":
    "todavía no se ha registrado ningún coste",
  "l.ev_view.the_baseline_plans_no_value":
    "la línea base no planifica ningún valor",
  "l.ev_view.not_available": "no disponible",
  "l.ev_view.schedule_no_ratio": "Cronograma: sin índice, {absentText}.",
  "l.ev_view.ahead_of_schedule": "Adelantado respecto al cronograma",
  "l.ev_view.behind_schedule": "Retrasado respecto al cronograma",
  "l.ev_view.on_schedule": "Según el cronograma",
  "l.ev_view.spi": "{word}: SPI {ratioText}, {gap}.",
  "l.ev_view.cost_no_ratio": "Coste: sin índice, {absentText}.",
  "l.ev_view.under_budget": "Por debajo del presupuesto",
  "l.ev_view.over_budget": "Por encima del presupuesto",
  "l.ev_view.on_budget": "Dentro del presupuesto",
  "l.ev_view.earned_schedule_no_ratio":
    "Cronograma ganado: sin índice, {absentText}.",
  "l.ev_view.by_earned_schedule_the_work":
    "Según el cronograma ganado, el trabajo va {word}: ha ganado en {days} lo que el plan esperaba tras {days2} (SPI(t) {ratioText}).",
  "l.ev_view.finished_work_counts_for_its":
    "el trabajo terminado cuenta por sus puntos de historia",
  "l.ev_view.only_finished_work_counts": "solo cuenta el trabajo terminado",
  "l.ev_view.by_story_points": "por puntos de historia",
  "l.ev_view.counted_by_a_person_entered":
    "{length} contabilizadas por un porcentaje introducido por una persona (manual)",
  "l.ev_view.with_no_points_carry_no": "{length} sin puntos no tienen peso",
  "l.ev_view.no_actual_cost_is_recorded": "no hay coste real registrado",
  "l.ev_view.the_ratio_it_divides_by":
    "el índice por el que divide aún no está definido",
  "l.ev_view.no_estimate_to_complete_has":
    "no se ha introducido ninguna estimación hasta la conclusión",
  "l.ev_view.the_budget_is_already_spent": "el presupuesto ya está gastado",
  "l.ev_view.no_cost_remains": "no queda coste",
  "l.ev_view.against_the_money_is_already":
    "Frente a {target}: el dinero ya está gastado, así que ninguna eficiencia puede completar el trabajo restante dentro de ese importe.",
  "l.ev_view.against": "Frente a {target}: {amountAbsentText}.",
  "l.ev_view.against_the_remaining_work_must":
    "Frente a {target}: el trabajo restante debe hacerse {ratioText} veces más eficientemente que el trabajo realizado hasta ahora (TCPI {ratioText2}), lo que exige mejorar la eficiencia.",
  "l.ev_view.against_the_remaining_work_can":
    "Frente a {target}: el trabajo restante puede hacerse con un {ratioText} de la eficiencia de costes del trabajo realizado hasta ahora (TCPI {ratioText2}).",
  "l.ev_view.earned": "{date}: ganado {ev_minor}",
  "l.ev_view.spent": "{date}: gastado {ac_minor}",
  "l.import_view.would_create": "Se crearían",
  "l.import_view.created": "Se crearon",
  "l.import_view.and": "{verb} {plural} y {upd} {tasks_updated}{tail}.",
  "l.import_view.preview_the_import": "Previsualizar la importación",
  "l.import_view.import": "Importar {plural}",
  "l.kpi_view.target_or_more_red_at":
    "Objetivo {target_value} o más; rojo en {red_value} o menos{u}",
  "l.kpi_view.target_or_less_red_at":
    "Objetivo {target_value} o menos; rojo en {red_value} o más{u}",
  "l.kpi_view.no_reading_within_twice_its":
    "Sin lecturas en el doble de su periodicidad {cadence}",
  "l.kpi_view.no_reading_yet": "todavía sin lecturas",
  "l.milestone_view.there_is_no_forecast": "No hay previsión.",
  "l.milestone_view.some_open_tasks_cannot_be":
    " (algunas tareas abiertas no se pueden situar, así que podría ser una estimación temprana)",
  "l.milestone_view.forecast_to_finish_after_it":
    "Se prevé que termine el {forecast}, {businessDays} después de su vencimiento ({due}){partial}.",
  "l.milestone_view.forecast_to_finish_on_the":
    "Se prevé que termine en la fecha de vencimiento, {due}{partial}.",
  "l.milestone_view.forecast_to_finish_before_it":
    "Se prevé que termine el {forecast}, {businessDays} antes de su vencimiento ({due}){partial}.",
  "l.milestone_view.of_them_later":
    ", {later_moves} de ellos a una fecha posterior",
  "l.milestone_view.back_on_its_original_date": "de nuevo en su fecha original",
  "l.milestone_view.later_than_the_original":
    "{calendarDays} después de la fecha original, {original_due}",
  "l.milestone_view.earlier_than_the_original":
    "{calendarDays} antes de la fecha original, {original_due}",
  "l.milestone_view.originally_due_now":
    "Vencimiento original: {original_due}; {times}{later}; ahora {net}.",
  "l.milestone_view.on": " el {achieved_on}",
  "l.milestone_view.achieved_every_task_it_waits":
    "Alcanzado{on}: todas las tareas de las que depende están hechas.",
  "l.milestone_view.reason": " Motivo: {done_reason}",
  "l.milestone_view.marked_done": "Marcado como hecho{on}.{why}",
  "l.milestone_view.due": "{name}: {toLowerCase}, vence el {due}{state}",
  "l.my_work_view.this_is_already_late_for":
    "Esto ya va con retraso para el plan ({days} de retraso).",
  "l.my_work_view.on_the_critical_path": "En la ruta crítica",
  "l.my_work_view.of_float": "{days} de holgura",
  "l.okr_view.not_measured": "Sin medir",
  "l.okr_view.not_measurable": "No medible ({reason}).",
  "l.okr_view.of": "{current} de {target}{u}",
  "l.okr_view.confidence_not_part_of_the":
    "Confianza {confidence} % (no forma parte de la puntuación)",
  "l.onboarding_view.give_the_date_it_takes":
    "Indique la fecha en que entra en vigor.",
  "l.onboarding_view.give_the_person_as_person":
    "Indique la persona como person:… (o worker:…).",
  "l.onboarding_view.say_what_access_in_words":
    "Describa el acceso con palabras.",
  "l.onboarding_view.choose_granted_or_revoked": "Elija concedido o revocado.",
  "l.onboarding_view.give_who_is_handing_over":
    "Indique quién hace el traspaso, como person:…",
  "l.onboarding_view.give_who_takes_over_as":
    "Indique quién asume el trabajo como person:… (o déjelo vacío para liberar el trabajo).",
  "l.onboarding_view.hand_over_to_someone_else": "Traspasar a otra persona.",
  "l.onboarding_view.no_items": "Sin elementos",
  "l.onboarding_view.of_done": "{done} de {total} hechos",
  "l.onboarding_view.skipped": "{skipped} omitidos",
  "l.onboarding_view.overdue": "{overdue} vencidos",
  "l.onboarding_view.skipped_2": "Omitido",
  "l.onboarding_view.overdue_2": "Vencido",
  "l.onboarding_view.nothing_is_left_open_this":
    "No queda nada abierto. Este plan está listo para cerrarse.",
  "l.onboarding_view.not_ready_to_close":
    "No está listo para cerrarse: {join}.",
  "l.onboarding_view.this_person_holds_nothing_live":
    "Esta persona no tiene nada activo en este plan.",
  "l.raci_view.nobody_is_accountable": "Nadie rinde cuentas",
  "l.raci_view.nobody_is_responsible": "Nadie es responsable",
  "l.raci_view.more_than_one_person_is": "Más de una persona rinde cuentas",
  "l.raci_view.has_no_role": "{task}: {who} no tiene rol",
  "l.raci_view.and": " y ",
  "l.raci_view.is": "{task}: {who} es {join}",
  "l.rbs.a_booking_that_needs_several":
    "Una reserva que requiere varias competencias cuenta por completo para cada una, por lo que las competencias se solapan y no se suman. La oferta y las personas no se conocen por competencia.",
  "l.rbs.named": "{length} con nombre",
  "l.rbs.supply_not_known": "oferta desconocida",
  "l.rbs.supply_fte": "oferta {supply_fte} FTE",
  "l.rbs.demand_fte": "{role}: {people}, demanda {demand_fte} FTE, {supply}",
  "l.scheduling_view.pass": "Aprobado",
  "l.scheduling_view.fail": "Suspenso",
  "l.scheduling_view.not_computable": "No calculable",
  "l.scheduling_view.not_enough_data": "No hay datos suficientes.",
  "l.scheduling_view.not_enough_data_2": "No hay datos suficientes: {reason}",
  "l.scheduling_view.must_pass_a_business_day":
    "Debe aprobarse: un retraso de {v} días hábiles tiene que mover la finalización",
  "l.scheduling_view.at_least_business_days": "Al menos {limit} días hábiles",
  "l.scheduling_view.at_most": "como máximo",
  "l.scheduling_view.at_least": "al menos",
  "l.scheduling_view.no_figure": "Sin cifra",
  "l.scheduling_view.finish_moved_of_business_days":
    "La finalización se movió {count} de {of} días hábiles",
  "l.scheduling_view.of": "{toFixed} ({count} de {of})",
  "l.scheduling_view.of_2": "{value} % ({count} de {of} {of_what})",
  "l.scheduling_view.passed_failed_not_computable":
    "{passed} aprobadas, {failed} suspendidas, {not_computable} no calculables",
  "l.scheduling_view.not_resolved": "Sin resolver.",
  "l.scheduling_view.it_is_on_the_critical":
    " Está en la ruta crítica, por lo que la finalización del proyecto se mueve.",
  "l.scheduling_view.starts_later_finishes_instead_of":
    "{pid} ({who}) empieza {from_start} → {scheduled_start}, {days} más tarde; termina el {to_finish} en lugar del {from_finish}.{crit}",
  "l.scheduling_view.to": "{from} a {to}",
  "l.scheduling_view.is_over_capacity_on":
    "{who} supera su capacidad en {span}.",
  "l.scheduling_view.is_asked_for_on_and":
    "A {who} se le pide un {demand} % en {span} y puede dar un {capacity} %.",
  "l.scheduling_view.no_project_finish_can_be":
    "No se puede calcular la finalización del proyecto.",
  "l.scheduling_view.the_project_finish_stays":
    "La finalización del proyecto se mantiene en {before}.",
  "l.scheduling_view.the_project_finish_moves_from":
    "La finalización del proyecto pasa de {before} a {after} ({days} más tarde).",
  "l.scheduling_view.no_demand_or_supply": "Sin demanda ni oferta",
  "l.scheduling_view.demand_fte_supply_not_known":
    "Demanda {demand_fte} FTE; oferta desconocida",
  "l.scheduling_view.short_fte": "Faltan {abs} FTE",
  "l.scheduling_view.spare_fte": "Sobran {gap_fte} FTE",
  "l.scheduling_view.balanced": "Equilibrado",
  "l.scheduling_view.the_critical_path_finish":
    "la finalización de la ruta crítica",
  "l.scheduling_view.of_the_simulated_outcomes_finish":
    "El {pct} % de los resultados simulados termina el {date} o antes.",
  "l.scheduling_view.no_variation": "Sin variación",
  "l.scheduling_view.no_estimate_using_the_fallback":
    "Sin estimación: se usa la banda predeterminada",
  "l.scheduling_view.milestone_no_spread": "Hito: sin dispersión",
  "l.scheduling_view.estimated_widened_to_include_the":
    "Estimado (ampliado para incluir el valor probable)",
  "l.scheduling_view.estimated": "Estimado",
  "l.scheduling_view.pert_favours_the_likely_value":
    "PERT (favorece el valor probable)",
  "l.scheduling_view.of_the_simulated_outcomes_cost":
    "El {round} % de los resultados simulados no cuesta más que el presupuesto ({budgetText}).",
  "l.scheduling_view.estimated_widened_to_include_the_2":
    "Estimado (ampliado para incluir el importe previsto)",
  "l.scheduling_view.stated_struck_in_of_runs":
    "declarado {round} %, ocurrido en el {toFixed} % de las ejecuciones",
  "l.scheduling_view.within_budget_and_on_time":
    "Dentro del presupuesto y a tiempo",
  "l.scheduling_view.within_budget_but_late":
    "Dentro del presupuesto pero con retraso",
  "l.scheduling_view.on_time_but_over_budget":
    "A tiempo pero por encima del presupuesto",
  "l.scheduling_view.over_budget_and_late":
    "Por encima del presupuesto y con retraso",
  "l.scheduling_view.both_within_budget_and_on":
    "Dentro del presupuesto y a tiempo a la vez: {pct} (solo dentro del presupuesto {pct2}; solo a tiempo {pct3}).",
  "l.scheduling_view.a_late_finish_costs_nothing":
    "En esta ejecución, terminar tarde no cuesta nada adicional (el coste del retraso por día hábil es 0).",
  "l.scheduling_view.each_business_day_late_adds":
    "Cada día hábil de retraso suma {format}; terminar antes no recupera nada.",
  "l.simulation_inputs.the_probability_must_be_a":
    "La probabilidad debe ser un número entero de 0 a 100.",
  "l.simulation_inputs.name_the_task_and_the":
    "Indique la tarea y los días hábiles que añade a la vez, o deje ambos vacíos.",
  "l.simulation_inputs.the_business_days_a_risk":
    "Los días hábiles que añade un riesgo deben ser un número entero de 1 a 365.",
  "l.simulation_inputs.give_the_cost_and_its":
    "Indique el coste y su moneda a la vez, o deje ambos vacíos.",
  "l.simulation_inputs.the_cost_must_be_an":
    "El coste debe ser un importe mayor que cero.",
  "l.simulation_inputs.the_currency_must_be_a":
    "La moneda debe ser un código de 3 letras, por ejemplo EUR, JPY o GBP.",
  "l.simulation_inputs.a_risk_that_changes_the":
    "Un riesgo que cambia el cronograma o el coste necesita una probabilidad.",
  "l.simulation_inputs.amounts_must_be_numbers_zero":
    "Los importes deben ser números, cero o más.",
  "l.simulation_inputs.the_low_amount_cannot_be":
    "El importe bajo no puede superar el importe previsto.",
  "l.simulation_inputs.the_high_amount_cannot_be":
    "El importe alto no puede ser inferior al importe previsto.",
  "l.simulation_inputs.business_days_must_be_whole":
    "Los días hábiles deben ser números enteros de 0 a 3650.",
  "l.simulation_inputs.give_the_task_a_duration":
    "Indique primero una duración para la tarea: ese es su valor probable.",
  "l.simulation_inputs.optimistic_cannot_be_above_the":
    "El valor optimista no puede superar la duración probable ({likely}).",
  "l.simulation_inputs.pessimistic_cannot_be_below_the":
    "El valor pesimista no puede ser inferior a la duración probable ({likely}).",
  "l.simulation_inputs.no_range_the_simulation_uses":
    "Sin rango: la simulación usa su banda predeterminada",
  "l.simulation_inputs.to": "{fmt} a {fmt2}",
  "l.wbs.restricted_item": "Elemento restringido",
  "l.whatif_view.choose_a_task": "Elija una tarea.",
  "l.whatif_view.give_the_new_start_date": "Indique la nueva fecha de inicio.",
  "l.whatif_view.duration_must_be_a_whole":
    "La duración debe ser un número entero de días hábiles de 0 a 3650.",
  "l.whatif_view.choose_a_constraint": "Elija una restricción.",
  "l.whatif_view.a_constraint_needs_a_date":
    "Una restricción necesita una fecha.",
  "l.whatif_view.a_hard_constraint_needs_a":
    "Una restricción estricta necesita un motivo.",
  "l.whatif_view.choose_both_tasks": "Elija ambas tareas.",
  "l.whatif_view.a_task_cannot_depend_on":
    "Una tarea no puede depender de sí misma.",
  "l.whatif_view.lag_must_be_a_whole":
    "El desfase debe ser un número entero de días hábiles (negativo es un adelanto).",
  "l.whatif_view.choose_a_link": "Elija un vínculo.",
  "l.whatif_view.start_on": "Iniciar «{name}» el {start_date}",
  "l.whatif_view.make_a_milestone": "Convertir «{name}» en hito",
  "l.whatif_view.for": "{constraint_kind} {constraint_date} para «{name}»",
  "l.whatif_view.clear_the_constraint_on": "Quitar la restricción de «{name}»",
  "l.whatif_view.lag": ", desfase {lag_days}",
  "l.whatif_view.follows": "«{name}» sigue a «{name2}» ({toUpperCase}{v})",
  "l.whatif_view.remove_the_link": "Eliminar el vínculo {slice}",
  "l.whatif_view.the_plan_has_no_finish":
    "El plan aún no tiene fecha de finalización.",
  "l.whatif_view.the_finish_stays": "La finalización se mantiene en {after}.",
  "l.whatif_view.the_finish_becomes": "La finalización pasa a ser {after}.",
  "l.whatif_view.the_plan_no_longer_has":
    "El plan ya no tiene fecha de finalización (era {before}).",
  "l.whatif_view.the_finish_moves_from_to":
    "La finalización pasa de {before} a {after}.",
  "l.whatif_view.cost_impact_is_finance_data":
    "El impacto en el coste son datos financieros, por lo que no se le muestran.",
  "l.whatif_view.no_task_with_work_has":
    "Ninguna tarea con trabajo tiene tarifa, por lo que no hay coste que comparar.",
  "l.work_view.work_must_be_a_number":
    "El trabajo debe ser un número de horas mayor que cero.",
  "l.work_view.duration_must_be_a_whole":
    "La duración debe ser un número entero de días hábiles de 1 a 3650.",
  "l.work_view.units_must_be_a_whole":
    "Las unidades deben ser un porcentaje entero de un día de 1 a 1000.",
  "l.work_view.give_the_units_too": "Indique también las unidades.",
  "l.work_view.business_days_at_is":
    "{duration_days} días hábiles al {units_percent} % son {hoursText}",
  "l.work_view.the_duration_was_worked_out": "Se calculó la duración.",
  "l.work_view.the_share_of_the_day": "Se calculó la proporción del día.",
  "l.work_view.the_work_was_worked_out": "Se calculó el trabajo.",
  "l.work_view.rounded_the_three_do_not":
    " Redondeado: los tres valores no coinciden al minuto.",
  "l.work_view.capacity_not_checked_the_task":
    "Capacidad no comprobada: la tarea aún no tiene fechas.",
  "l.work_view.within_the_person_s_capacity":
    "Dentro de la capacidad de la persona.",
  "l.work_view.give_1_to_40_whole":
    "Indique de 1 a 40 porcentajes enteros, separados por comas.",
  "l.work_view.the_percentages_add_up_to":
    "Los porcentajes suman {sum}, no 100.",
  "p.home.main_x_plans": "Main X · Planes",
  "p.announcements.could_not_load_the_announcements":
    "No se pudieron cargar los anuncios",
  "p.announcements.could_not_publish": "No se pudo publicar",
  "p.announcements.could_not_retract": "No se pudo retirar",
  "p.auditor.auditor_ppm": "Auditor — PPM",
  "p.automations.could_not_load_automations":
    "No se pudieron cargar las automatizaciones",
  "p.automations.rule_saved_it_fires_from":
    "Regla guardada — se activa a partir del próximo movimiento que coincida",
  "p.automations.could_not_save_the_rule": "No se pudo guardar la regla",
  "p.automations.that_action_was_refused": "Esa acción fue rechazada",
  "p.automations.capped_at_run_again_for":
    " (limitado a {cap} — ejecute de nuevo para el resto)",
  "p.automations.swept_fired_already_claimed":
    "Barrido: {fired} activadas, {skipped_already_claimed} ya reclamadas{v}",
  "p.automations.the_sweep_failed": "El barrido falló",
  "p.board.board_ppm": "Consejo — PPM",
  "p.calendar.calendar_ppm": "Calendario — PPM",
  "p.ceo.could_not_load": "No se pudo cargar",
  "p.compliance.compliance_ppm": "Cumplimiento — PPM",
  "p.cost_rates.could_not_load_the_rate":
    "No se pudieron cargar las tablas de tarifas",
  "p.cost_rates.rate_table_added": "Tabla de tarifas añadida.",
  "p.cost_rates.could_not_add_the_table": "No se pudo añadir la tabla",
  "p.cost_rates.give_a_date_and_an":
    "Indique una fecha y una tarifa por hora de cero o más.",
  "p.cost_rates.rate_added": "Tarifa añadida.",
  "p.cost_rates.could_not_add_the_rate": "No se pudo añadir la tarifa",
  "p.cost_risk.could_not_run_the_simulation":
    "No se pudo ejecutar la simulación",
  "p.documents.could_not_load_the_documents":
    "No se pudieron cargar los documentos",
  "p.documents.could_not_create_the_document": "No se pudo crear el documento",
  "p.documents.pid.could_not_load_the_document":
    "No se pudo cargar el documento",
  "p.documents.pid.that_did_not_work": "Eso no funcionó",
  "p.documents.pid.enter_the_approver_s_id":
    "Introduzca el id del aprobador (un id de persona, no usted).",
  "p.documents.pid.sent_for_review": "Enviado a revisión.",
  "p.documents.pid.needs_changes": "Requiere cambios",
  "p.documents.pid.approved": "Aprobado.",
  "p.documents.pid.sent_back_to_draft": "Devuelto a borrador.",
  "p.documents.pid.in_force_from_today": "En vigor desde hoy.",
  "p.documents.pid.next_draft_started": "Siguiente borrador iniciado.",
  "p.documents.pid.could_not_look_that_up": "No se pudo consultar",
  "p.engineering.engineering_ppm": "Ingeniería — PPM",
  "p.executive.executive_ppm": "Ejecutivo — PPM",
  "p.financials.financials_ppm": "Finanzas — PPM",
  "p.gantt.gantt_main_x": "Gantt — Main X",
  "p.gantt.calendar_failed":
    "No se pudo cargar el calendario laboral, por lo que los días no laborables no aparecen sombreados.",
  "p.kpis.could_not_load_the_kpis": "No se pudieron cargar los KPI",
  "p.kpis.could_not_define_the_kpi": "No se pudo definir el KPI",
  "p.lifecycle.could_not_load_the_funnel": "No se pudo cargar el embudo",
  "p.me.could_not_load_your_tasks": "No se pudieron cargar sus tareas",
  "p.me.calendar.could_not_load_your_calendar":
    "No se pudo cargar su calendario",
  "p.me.calendar.could_not_save": "No se pudo guardar",
  "p.me.calendar.could_not_remove": "No se pudo eliminar",
  "p.milestones.could_not_load_the_milestones":
    "No se pudieron cargar los hitos",
  "p.objectives.create_failed": "error al crear",
  "p.objectives.alignment_failed": "error de alineación",
  "p.okrs.could_not_load_the_okrs": "No se pudieron cargar los OKR",
  "p.okrs.could_not_create_the_objective": "No se pudo crear el objetivo",
  "p.plans.plans_main_x": "Planes — Main X",
  "p.plans.pid.main_x": "{v} — Main X",
  "p.plans.pid.board.pt": " · {points} pt",
  "p.plans.pid.board.board_ppm": "Tablero — PPM",
  "p.plans.pid.calendar.could_not_load_the_calendar":
    "No se pudo cargar el calendario",
  "p.plans.pid.calendar.could_not_save": "No se pudo guardar",
  "p.plans.pid.cost_risk.could_not_run_the_simulation":
    "No se pudo ejecutar la simulación",
  "p.plans.pid.costs.could_not_load_the_costs":
    "No se pudieron cargar los costes",
  "p.plans.pid.costs.choose_a_task": "Elija una tarea.",
  "p.plans.pid.costs.the_amount_must_be_above":
    "El importe debe ser mayor que cero.",
  "p.plans.pid.costs.the_currency_must_be_a":
    "La moneda debe ser un código de 3 letras, por ejemplo EUR, JPY o GBP.",
  "p.money.currency_hint": "por ejemplo EUR, JPY o GBP",
  "p.money.amount_or_currency":
    "Indique el importe como número y la moneda como código de 3 letras, por ejemplo EUR, JPY o GBP.",
  "p.plans.pid.costs.expense_added": "Gasto añadido.",
  "p.plans.pid.costs.could_not_add_the_expense": "No se pudo añadir el gasto",
  "p.plans.pid.costs.could_not_record": "No se pudo registrar",
  "p.plans.pid.critical_path.could_not_load_the_baselines":
    "No se pudieron cargar las líneas base",
  "p.plans.pid.critical_path.could_not_take_the_baseline":
    "No se pudo tomar la línea base",
  "p.plans.pid.critical_path.could_not_load_the_schedule":
    "No se pudo cargar el cronograma",
  "p.plans.pid.critical_path.could_not_save_the_estimate":
    "No se pudo guardar la estimación",
  "p.plans.pid.critical_path.could_not_save": "No se pudo guardar",
  "p.plans.pid.earned_value.could_not_load_earned_value":
    "No se pudo cargar el valor ganado",
  "p.plans.pid.earned_value.could_not_save": "No se pudo guardar",
  "p.plans.pid.earned_value.could_not_record": "No se pudo registrar",
  "p.plans.pid.edit.edit_case_main_x": "Editar caso — Main X",
  "p.plans.pid.flow.could_not_load_flow_data":
    "No se pudieron cargar los datos de flujo.",
  "p.plans.pid.handover.handover_is_about_people_so":
    "El traspaso afecta a personas, por lo que requiere el permiso para gestionar personas.",
  "p.plans.pid.handover.could_not_preview":
    "No se pudo obtener la vista previa",
  "p.plans.pid.handover.what_this_person_holds_changed":
    "Lo que tiene esta persona cambió desde la vista previa, por lo que no se cambió nada. Vuelva a obtener la vista previa y confirme.",
  "p.plans.pid.handover.could_not_commit": "No se pudo confirmar",
  "p.plans.pid.handover.discarded_nothing_was_changed":
    "Descartado. No se cambió nada.",
  "p.plans.pid.import.choose_the_file_first": "Elija primero el archivo.",
  "p.plans.pid.import.the_import_did_not_work": "La importación no funcionó",
  "p.plans.pid.joint_risk.could_not_run_the_simulation":
    "No se pudo ejecutar la simulación",
  "p.plans.pid.leveling.could_not_propose_a_leveling":
    "No se pudo proponer una nivelación",
  "p.plans.pid.leveling.could_not_apply_the_moves":
    "No se pudieron aplicar los movimientos",
  "p.plans.pid.onboarding.could_not_load": "No se pudo cargar",
  "p.plans.pid.onboarding.could_not_open_the_checklist":
    "No se pudo abrir la lista de comprobación",
  "p.plans.pid.onboarding.checklist_started": "Lista de comprobación iniciada.",
  "p.plans.pid.onboarding.could_not_start": "No se pudo iniciar",
  "p.plans.pid.onboarding.could_not_save": "No se pudo guardar",
  "p.plans.pid.onboarding.checklist_completed":
    "Lista de comprobación completada.",
  "p.plans.pid.onboarding.could_not_complete": "No se pudo completar",
  "p.plans.pid.onboarding.say_why_you_are_cancelling":
    "Indique por qué cancela.",
  "p.plans.pid.onboarding.checklist_cancelled":
    "Lista de comprobación cancelada.",
  "p.plans.pid.onboarding.could_not_cancel": "No se pudo cancelar",
  "p.plans.pid.onboarding.could_not_record": "No se pudo registrar",
  "p.plans.pid.raci.could_not_load_who_does":
    "No se pudo cargar quién hace qué",
  "p.plans.pid.raci.could_not_save": "No se pudo guardar",
  "p.plans.pid.raci.use_a_person_or_worker":
    "Use una referencia person: o worker: con un UUID.",
  "p.plans.pid.schedule_health.could_not_run_the_schedule":
    "No se pudieron ejecutar las comprobaciones de salud del cronograma",
  "p.plans.pid.schedule_risk.could_not_run_the_simulation":
    "No se pudo ejecutar la simulación",
  "p.plans.pid.wbs.could_not_load_the_tasks":
    "No se pudieron cargar las tareas",
  "p.plans.pid.wbs.link_counts_failed":
    "No se pudo cargar cuántos vínculos tiene cada tarea, por lo que no se muestran recuentos.",
  "p.plans.pid.what_if.could_not_load_the_plan": "No se pudo cargar el plan",
  "p.plans.pid.what_if.at_most_edits_at_a":
    "Como máximo {MAX_EDITS} cambios a la vez.",
  "p.plans.pid.what_if.edit_staged_preview_to_see":
    "Cambio preparado. Obtenga la vista previa para ver qué hace.",
  "p.plans.pid.what_if.discarded_nothing_was_changed":
    "Descartado. No se cambió nada.",
  "p.plans.pid.what_if.could_not_preview": "No se pudo obtener la vista previa",
  "p.plans.pid.what_if.the_plan_changed_since_this":
    "El plan cambió desde esta vista previa, por lo que no se aplicó nada. Vuelva a obtener la vista previa y confirme.",
  "p.plans.pid.what_if.could_not_commit": "No se pudo confirmar",
  "p.plans.pid.what_if.to": "{early_start} a {early_finish}",
  "p.plans.pid.what_if.not_scheduled": "sin programar",
  "p.plans.pid.work.could_not_load_the_plan": "No se pudo cargar el plan",
  "p.plans.pid.work.could_not_load_the_work":
    "No se pudo cargar el perfil de trabajo",
  "p.plans.pid.work.could_not_save": "No se pudo guardar",
  "p.plans.pid.work.contour_saved": "Contorno guardado.",
  "p.plans.merge.merge_plans_ppm": "Combinar planes — PPM",
  "p.plans.new.new_case_main_x": "Nuevo caso — Main X",
  "p.prioritization.could_not_load_the_queue": "No se pudo cargar la cola",
  "p.prioritization.could_not_load_the_score":
    "No se pudo cargar la puntuación",
  "p.proposals.duplicate_check_failed":
    "error en la comprobación de duplicados",
  "p.regulator.regulator_ppm": "Regulador — PPM",
  "p.reports.save_failed": "error al guardar",
  "p.reports.run_failed": "error al ejecutar",
  "p.resources.could_not_load_the_resource":
    "No se pudo cargar el plan de recursos",
  "p.resources.rbs.could_not_load_resources":
    "No se pudieron cargar los recursos",
  "p.reviews.could_not_load_reviews": "No se pudieron cargar las revisiones",
  "p.reviews.invitation_sent": "Invitación enviada",
  "p.reviews.could_not_delegate": "No se pudo delegar",
  "p.reviews.score_0_100_leave_blank":
    "Puntuación 0–100 (déjela vacía si no hay)",
  "p.reviews.score_must_be_between_0": "La puntuación debe estar entre 0 y 100",
  "p.reviews.that_action_was_refused": "Esa acción fue rechazada",
  "p.reviews.could_not_load_consensus": "No se pudo cargar el consenso",
  "p.risk.risk_ppm": "Riesgo — PPM",
  "p.scenarios.evaluate_failed": "error al evaluar",
  "p.security.security_ppm": "Seguridad — PPM",
  "p.technology.technology_ppm": "Tecnología — PPM",
  "p.tour.tour_main_x_plans": "Recorrido · Main X Planes",
  "p.verify.this_sign_in_link_is":
    "A este enlace de inicio de sesión le falta el token.",
  "p.verify.we_could_not_reach_the":
    "No pudimos contactar con el servicio de inicio de sesión. Inténtelo de nuevo en un momento.",
  "p.verify.this_sign_in_link_is_2":
    "Este enlace de inicio de sesión no es válido o ha caducado.",
  "l.ceo_view.gate_review": "Revisión de fase",
  "l.ceo_view.scenario_committed": "Escenario confirmado",
  "l.ceo_view.proposal": "Propuesta",
  "l.ceo_view.plans_merged": "Planes combinados",
  "c.CumulativeFlow.in_review": "En revisión",
  "c.CumulativeFlow.in_progress": "En curso",
  "c.CumulativeFlow.todo": "Pendiente",
  "l.document_view.draft": "Borrador",
  "l.document_view.in_review": "En revisión",
  "l.document_view.approved_not_yet_in_force": "Aprobado, aún no en vigor",
  "l.document_view.replaced": "Sustituido",
  "l.document_view.send_for_review": "Enviar a revisión",
  "l.document_view.approve": "Aprobar",
  "l.document_view.bring_into_force_today": "Poner en vigor hoy",
  "l.document_view.start_the_next_draft": "Iniciar el siguiente borrador",
  "l.document_view.waiting_for_the_approver": "Esperando al aprobador",
  "l.document_view.approved": "Aprobado",
  "l.document_view.change_this_document": "Modificar este documento",
  "l.document_view.someone_else_approves_it_you":
    "Lo aprueba otra persona: no puede aprobar su propio documento.",
  "l.document_view.only_the_person_asked_can":
    "Solo la persona a quien se pidió puede decidir.",
  "l.document_view.it_comes_into_force_today":
    "Entra en vigor hoy y sustituye a la versión vigente.",
  "l.document_view.the_next_draft_starts_as":
    "El siguiente borrador empieza como copia del texto más reciente.",
  "l.ev_view.on_track": "en curso según lo previsto",
  "l.ev_view.off_plan": "fuera del plan",
  "l.ev_view.not_yet_known": "aún desconocido",
  "l.glossary.schedule_performance_index_work_done":
    "Índice de desempeño del cronograma: trabajo realizado frente al trabajo previsto hasta ahora. Por encima de 1,00 va por delante del plan.",
  "l.glossary.cost_performance_index_value_of":
    "Índice de desempeño del coste: valor del trabajo realizado frente a lo que costó. Por encima de 1,00 está por debajo del presupuesto.",
  "l.glossary.schedule_index_by_time_how":
    "Índice de cronograma por tiempo: cuánto ha avanzado el plan en el tiempo, que sigue siendo fiable incluso cuando un proyecto termina tarde.",
  "l.glossary.earned_value_the_budget_for":
    "Valor ganado: el presupuesto del trabajo realmente terminado.",
  "l.glossary.planned_value_the_budget_for":
    "Valor planificado: el presupuesto del trabajo que debería estar terminado a estas alturas.",
  "l.glossary.actual_cost_what_has_really":
    "Coste real: lo que realmente se ha gastado.",
  "l.glossary.estimate_at_completion_the_expected":
    "Estimación a la conclusión: el coste final esperado.",
  "l.glossary.estimate_to_complete_what_is":
    "Estimación hasta la conclusión: lo que aún se espera gastar.",
  "l.glossary.variance_at_completion_the_expected":
    "Variación a la conclusión: el coste final esperado frente al presupuesto.",
  "l.glossary.red_amber_or_green_a":
    "Rojo, ámbar o verde: un indicador de estado.",
  "l.glossary.responsible_accountable_consulted_in":
    "Responsable, Rinde cuentas, Consultado, Informado: quién hace el trabajo, quién lo aprueba, a quién se consulta y a quién se informa.",
  "l.glossary.a_way_of_weighting_the":
    "Una forma de ponderar la estimación probable más que los casos mejor y peor.",
  "l.glossary.key_performance_indicator_a_measure":
    "Indicador clave de rendimiento: una medida que se sigue en el tiempo frente a un objetivo.",
  "l.glossary.key_performance_indicators_measures_":
    "Indicadores clave de rendimiento: medidas que se siguen en el tiempo frente a un objetivo.",
  "l.glossary.objective_and_key_results_a":
    "Objetivo y resultados clave: una meta con unos pocos resultados medibles.",
  "l.glossary.objectives_and_key_results_goals":
    "Objetivos y resultados clave: metas con unos pocos resultados medibles.",
  "l.import_view.jira_csv_export": "Jira (exportación CSV)",
  "l.import_view.asana_csv_export": "Asana (exportación CSV)",
  "l.import_view.microsoft_project_xml_file": "Microsoft Project (archivo XML)",
  "l.import_view.columns_not_imported": "Columnas no importadas",
  "l.import_view.rows_skipped": "Filas omitidas",
  "l.import_view.links_not_imported": "Vínculos no importados",
  "l.import_view.parents_not_used": "Padres no usados",
  "l.import_view.dates_not_understood": "Fechas no interpretadas",
  "l.import_view.fields_shortened": "Campos acortados",
  "l.import_view.status_notes": "Notas de estado",
  "l.import_view.resources_not_matched_to_people":
    "Recursos no asociados a personas",
  "l.import_view.costs_not_imported": "Costes no importados",
  "l.import_view.schedules_not_imported": "Cronogramas no importados",
  "l.import_view.constraints_not_imported": "Restricciones no importadas",
  "l.import_view.calendar_notes": "Notas del calendario",
  "l.kpi_view.improving": "Mejorando",
  "l.kpi_view.getting_worse": "Empeorando",
  "l.kpi_view.no_change": "Sin cambios",
  "l.milestone_view.on_track": "Según lo previsto",
  "l.milestone_view.at_risk": "En riesgo",
  "l.milestone_view.forecast_late": "Previsto con retraso",
  "l.milestone_view.no_forecast": "Sin previsión",
  "l.milestone_view.it_is_not_linked_to":
    "No está vinculado a ninguna tarea, por lo que no hay nada a partir de lo cual prever.",
  "l.milestone_view.its_open_tasks_have_no":
    "Sus tareas abiertas no tienen fechas que el cronograma pueda situar (indique una duración y un inicio o un predecesor).",
  "l.milestone_view.the_plan_s_dependencies_contain":
    "Las dependencias del plan contienen un ciclo, por lo que no se puede hacer ninguna previsión.",
  "l.okr_view.not_measurable_the_start_and":
    "No medible: el inicio y el objetivo son iguales, por lo que no hay distancia que recorrer.",
  "l.okr_view.not_measurable_a_maintain_key":
    "No medible: un resultado clave de mantenimiento necesita una banda de tolerancia.",
  "l.okr_view.not_measurable_the_numbers_are":
    "No medible: los números son demasiado grandes para puntuar.",
  "l.onboarding_view.someone_joins_this_plan":
    "Alguien se incorpora a este plan",
  "l.onboarding_view.someone_leaves_this_plan": "Alguien deja este plan",
  "l.onboarding_view.take_this_plan_in": "Asumir este plan",
  "l.onboarding_view.close_this_plan_out": "Cerrar este plan",
  "l.onboarding_view.booking": "Reserva",
  "l.onboarding.executive": "Ejecutivo",
  "l.onboarding.outcomes_and_exposure_across_the":
    "Resultados y exposición en todo el conjunto, no en un solo plan.",
  "l.onboarding.read_the_executive_summary": "Leer el resumen ejecutivo",
  "l.onboarding.health_decisions_benefits_realized_a":
    "Salud, decisiones, beneficios materializados y alineación estratégica en una sola vista.",
  "l.onboarding.open_the_at_a_glance": "Abrir el panel de un vistazo",
  "l.onboarding.the_whole_estate_s_rag":
    "El estado RAG de todo el conjunto y las violaciones de cronograma a la vez.",
  "l.onboarding.review_the_board_pack": "Revisar el dossier del consejo",
  "l.onboarding.the_pack_a_board_meeting":
    "El dossier que realmente se entregaría en una reunión del consejo: inversiones y tendencias.",
  "l.onboarding.check_budget_variance_and_exposure":
    "Comprobar la desviación presupuestaria y la exposición",
  "l.onboarding.where_spend_is_diverging_from":
    "Dónde el gasto se desvía del plan, por moneda — las monedas nunca se suman entre sí.",
  "l.onboarding.compare_two_candidate_portfolios":
    "Comparar dos carteras candidatas",
  "l.onboarding.what_if_scenarios_side_by":
    "Escenarios hipotéticos lado a lado, antes de comprometer financiación en cualquiera de ellos.",
  "l.onboarding.see_what_s_ranked_highest":
    "Ver qué ocupa los primeros puestos y por qué",
  "l.onboarding.the_smart_score_queue_every":
    "La cola de Smart Score — cada puntuación muestra la evidencia que la respalda, o indica que no la tiene.",
  "l.onboarding.pmo_portfolio_manager": "PMO / responsable de cartera",
  "l.onboarding.intake_governance_and_the_schedule":
    "Entrada, gobernanza y el cronograma de todos los planes.",
  "l.onboarding.open_the_intake_pipeline": "Abrir el flujo de entrada",
  "l.onboarding.every_proposal_s_stage_from":
    "La etapa de cada propuesta, desde el borrador hasta la aprobación.",
  "l.onboarding.register_a_plan": "Registrar un plan",
  "l.onboarding.the_first_plan_you_create":
    "El primer plan que cree es una buena forma de ver la estructura completa de los datos.",
  "l.onboarding.browse_the_plan_registry": "Explorar el registro de planes",
  "l.onboarding.one_recursive_collection_kind_is":
    "Una única colección recursiva — `kind` es una etiqueta, no una barrera.",
  "l.onboarding.open_a_plan_s_governance":
    "Abrir la pestaña de gobernanza de un plan",
  "l.onboarding.gate_reviews_risks_and_budget":
    "Revisiones de fase, riesgos y líneas de presupuesto de ese plan.",
  "l.onboarding.look_at_the_cross_plan": "Ver el cronograma entre planes",
  "l.onboarding.dependencies_the_critical_path_and":
    "Dependencias, la ruta crítica y cualquier sucesor que se retrase.",
  "l.onboarding.check_the_risk_register": "Consultar el registro de riesgos",
  "l.onboarding.everything_open_or_mitigating_ranked":
    "Todo lo abierto o en mitigación, ordenado por exposición.",
  "l.onboarding.review_the_compliance_register":
    "Revisar el registro de cumplimiento",
  "l.onboarding.standing_findings_across_the_estate":
    "Hallazgos vigentes en todo el conjunto, no solo de un plan.",
  "l.onboarding.run_a_saved_report": "Ejecutar un informe guardado",
  "l.onboarding.a_filter_field_projection_saved":
    "Un filtro + proyección de campos guardados para reutilizar, ejecutados a petición.",
  "l.onboarding.resource_manager": "Gestor de recursos",
  "l.onboarding.who_is_allocated_where_and":
    "Quién está asignado dónde y cómo fluye realmente la entrega.",
  "l.onboarding.check_the_capacity_rollup": "Consultar el resumen de capacidad",
  "l.onboarding.who_is_over_allocated_this":
    "Quién está sobreasignado en este periodo, sumando todos los planes en los que participa.",
  "l.onboarding.open_a_plan_s_task": "Abrir el tablero de tareas de un plan",
  "l.onboarding.the_kanban_board_that_task":
    "El tablero Kanban por el que realmente avanza el trabajo de las tareas.",
  "l.onboarding.read_that_plan_s_flow": "Leer las cifras de flujo de ese plan",
  "l.onboarding.cycle_time_lead_time_and":
    "Tiempo de ciclo, tiempo de entrega y eficiencia del flujo — derivados del propio historial del tablero.",
  "l.onboarding.scan_blocked_items_and_the":
    "Revisar los elementos bloqueados y la combinación de entrega",
  "l.onboarding.what_s_stuck_the_moscow":
    "Lo que está atascado, la combinación MoSCoW y el calendario de hitos en una sola vista.",
  "l.onboarding.look_at_the_milestone_calendar": "Ver el calendario de hitos",
  "l.onboarding.every_milestone_across_every_plan":
    "Todos los hitos de todos los planes, con sus fechas de vencimiento.",
  "l.onboarding.see_what_automations_are_configured":
    "Ver qué automatizaciones están configuradas",
  "l.onboarding.what_fires_on_a_field":
    "Qué se activa al cambiar un campo y qué hace cuando se activa.",
  "l.onboarding.open_collaborative_reviews":
    "Abrir las revisiones colaborativas",
  "l.onboarding.where_a_plan_gets_a":
    "Donde un plan recibe una segunda opinión antes de tomar una decisión sobre él.",
  "l.raci_view.responsible": "Responsable",
  "l.raci_view.accountable": "Rinde cuentas",
  "l.raci_view.consulted": "Consultado",
  "l.raci_view.informed": "Informado",
  "l.raci_view.does_the_work": "hace el trabajo",
  "l.raci_view.answers_for_it_and_signs":
    "responde de él y lo aprueba (uno por tarea)",
  "l.raci_view.is_asked_before": "se le consulta antes",
  "l.raci_view.is_told_after": "se le informa después",
  "l.scheduling_view.no_schedule_baseline_has_been":
    "No se ha tomado ninguna línea base del cronograma para este plan.",
  "l.scheduling_view.nothing_was_baselined_to_finish":
    "Nada tenía previsto en la línea base terminar antes de la fecha de estado.",
  "l.scheduling_view.the_plan_s_dependencies_contain":
    "Las dependencias del plan contienen un ciclo.",
  "l.scheduling_view.there_are_no_incomplete_tasks":
    "No hay tareas incompletas que medir.",
  "l.scheduling_view.there_are_no_relationships_between":
    "No hay relaciones entre tareas que medir.",
  "l.scheduling_view.no_incomplete_task_has_dates":
    "Ninguna tarea incompleta tiene fechas que el cronograma pueda situar.",
  "l.scheduling_view.no_incomplete_task_has_a":
    "Ninguna tarea incompleta tiene duración.",
  "l.scheduling_view.the_plan_has_no_tasks": "El plan no tiene tareas.",
  "l.scheduling_view.there_is_no_critical_task":
    "No hay ninguna tarea crítica que probar.",
  "l.scheduling_view.no_path_remains_to_measure":
    "No queda ninguna ruta que medir.",
  "l.scheduling_view.every_task_involved_has_already":
    "Todas las tareas implicadas ya han empezado, por lo que ninguna puede moverse.",
  "l.scheduling_view.the_tasks_involved_are_critical":
    "Las tareas implicadas son críticas; mover una movería la finalización. Permita las tareas críticas para ver ese coste.",
  "l.scheduling_view.delaying_a_task_enough_would":
    "Retrasar lo suficiente una tarea rompería el cronograma (holgura negativa o finalización más tardía).",
  "l.scheduling_view.a_task_involved_has_a":
    "Una tarea implicada tiene una restricción estricta y no se puede mover.",
  "l.scheduling_view.the_schedule_would_not_let":
    "El cronograma no permitiría que la tarea empezara más tarde.",
  "l.scheduling_view.the_search_stopped_at_its":
    "La búsqueda se detuvo en su límite antes de resolver esto.",
  "l.scheduling_view.no_probability": "sin probabilidad",
  "l.scheduling_view.no_task_named": "sin tarea indicada",
  "l.scheduling_view.no_impact_in_business_days": "sin impacto en días hábiles",
  "l.scheduling_view.the_risk_is_not_open": "el riesgo no está abierto",
  "l.scheduling_view.its_task_is_not_on": "su tarea no está en el cronograma",
  "l.scheduling_view.the_plan_has_no_budget":
    "el plan no tiene línea de presupuesto en su moneda",
  "l.whatif_view.move_a_task_set_its": "Mover una tarea (fijar su inicio)",
  "l.whatif_view.change_a_task_s_duration": "Cambiar la duración de una tarea",
  "l.whatif_view.change_a_task_s_constraint":
    "Cambiar la restricción de una tarea",
  "l.whatif_view.add_a_link": "Añadir un vínculo",
  "l.whatif_view.remove_a_link": "Eliminar un vínculo",
  "l.whatif_view.must_start_on_hard": "Debe empezar el (estricta)",
  "l.whatif_view.must_finish_on_hard": "Debe terminar el (estricta)",
  "l.work_view.fixed_units": "Unidades fijas",
  "l.work_view.fixed_duration": "Duración fija",
  "l.work_view.fixed_work": "Trabajo fijo",
  "l.work_view.changing_the_work_changes_how":
    "Cambiar el trabajo cambia cuánto tarda; la proporción del día se mantiene.",
  "l.work_view.changing_the_work_changes_the":
    "Cambiar el trabajo cambia la proporción del día; la duración se mantiene.",
  "l.work_view.changing_the_duration_or_the":
    "Cambiar la duración o la proporción del día cambia la otra; el trabajo se mantiene.",
  "l.work_view.flat_the_same_each_day": "Plano: lo mismo cada día",
  "l.work_view.front_loaded_more_at_the": "Cargado al inicio: más al principio",
  "l.work_view.back_loaded_more_at_the": "Cargado al final: más al final",
  "l.work_view.bell_most_in_the_middle": "Campana: la mayor parte en el medio",
  "l.work_view.turtle_ramps_up_holds_ramps": "Tortuga: sube, se mantiene, baja",
  "l.work_view.custom_percentages": "Porcentajes personalizados",
  "l.work_view.has_no_dates_yet": "aún no tiene fechas",
  "l.work_view.has_no_business_day_to":
    "no tiene ningún día hábil en el que caer",
  "l.work_view.has_a_contour_that_gives": "tiene un contorno que no le da peso",
  "l.work_view.no_rate_applies": "no se aplica ninguna tarifa",
  "l.work_view.no_rate_applied_on_some": "no se aplicó tarifa algunos días",
  "l.work_view.has_nobody_assigned": "no tiene a nadie asignado",
  "p._layout_svelte.reviews": "Revisiones",
  "p._layout_svelte.automations": "Automatizaciones",
  "p._layout_svelte.goals_okrs": "Metas (OKR)",
  "p._layout_svelte.onboarding": "Incorporación",
  "p.ceo.red": "Rojo",
  "p.ceo.amber": "Ámbar",
  "p.ceo.green": "Verde",
  "p.display.saved_on_this_device": "Guardado en este dispositivo.",
  "p.display.saved_to_your_account_so":
    "Guardado en su cuenta, por lo que le acompaña en otros navegadores.",
  "p.display.could_not_save_to_your":
    "No se pudo guardar en su cuenta en este momento; se conserva en este dispositivo.",
  "p.me.today": "Hoy",
  "p.me.this_week": "Esta semana",
  "p.me.later": "Más adelante",
  "p.me.no_due_date": "Sin fecha de vencimiento",
  "p.plans.pid.leveling": "Nivelación",
  "p.plans.pid.cost_and_schedule_risk": "Riesgo de coste y cronograma",
  "p.plans.pid.board.todo": "Pendiente",
  "p.plans.pid.board.in_progress": "En curso",
  "p.plans.pid.board.in_review": "En revisión",
  "p.common.business_day_one": "{n} día hábil",
  "p.common.business_day_other": "{n} días hábiles",
  "p.common.calendar_day_one": "{n} día natural",
  "p.common.calendar_day_other": "{n} días naturales",
  "l.milestone_view.rescheduled_one": "reprogramado {count} vez",
  "l.milestone_view.rescheduled_other": "reprogramado {count} veces",
  "l.milestone_view.open_of_linked_one":
    "{open_tasks} de {linked_tasks} tarea sigue abierta",
  "l.milestone_view.open_of_linked_other":
    "{open_tasks} de {linked_tasks} tareas siguen abiertas",
  "l.cpm_view.restricted_one":
    "{n} dependencia con un plan que no puede ver queda fuera de estas fechas.",
  "l.cpm_view.restricted_other":
    "{n} dependencias con un plan que no puede ver quedan fuera de estas fechas.",
  "l.cpm_view.monday": "Lunes",
  "l.cpm_view.tuesday": "Martes",
  "l.cpm_view.wednesday": "Miércoles",
  "l.cpm_view.thursday": "Jueves",
  "l.cpm_view.friday": "Viernes",
  "l.cpm_view.saturday": "Sábado",
  "l.cpm_view.sunday": "Domingo",
  "l.ev_view.gap_more":
    "{money} más de valor ganado que el previsto a estas alturas",
  "l.ev_view.gap_less":
    "{money} menos de valor ganado que el previsto a estas alturas",
  "l.ev_view.cost_line_less":
    "{word}: CPI {ratio}, {money} menos gastado de lo que ganó el trabajo.",
  "l.ev_view.cost_line_more":
    "{word}: CPI {ratio}, {money} más gastado de lo que ganó el trabajo.",
  "l.ev_view.tasks_counted_one":
    "{count} tarea contabilizada, ponderada {weighting}; {how}",
  "l.ev_view.tasks_counted_other":
    "{count} tareas contabilizadas, ponderadas {weighting}; {how}",
  "l.ev_view.equally": "por igual",
  "l.onboarding_view.would_move_one": "{n} elemento pasaría a {to}.",
  "l.onboarding_view.would_move_other": "{n} elementos pasarían a {to}.",
  "l.onboarding_view.would_release_one":
    "Se liberaría {n} elemento: tareas y riesgos sin asignar, reservas y roles eliminados.",
  "l.onboarding_view.would_release_other":
    "Se liberarían {n} elementos: tareas y riesgos sin asignar, reservas y roles eliminados.",
  "l.whatif_view.set_duration_one":
    "Fijar «{name}» en {duration_days} día hábil",
  "l.whatif_view.set_duration_other":
    "Fijar «{name}» en {duration_days} días hábiles",
  "l.work_view.over_capacity_one":
    "Por encima de la capacidad durante {days_over} día natural ({from} a {to}): se pide más de lo que la persona puede dar.",
  "l.work_view.over_capacity_other":
    "Por encima de la capacidad durante {days_over} días naturales ({from} a {to}): se pide más de lo que la persona puede dar.",
  "p.plans.pid.handover.done_one":
    "Hecho: {count} elemento traspasado y registrado.",
  "p.plans.pid.handover.done_other":
    "Hecho: {count} elementos traspasados y registrados.",
  "p.plans.pid.what_if.committed_one": "Confirmado: {count} cambio registrado.",
  "p.plans.pid.what_if.committed_other":
    "Confirmado: {count} cambios registrados.",
  "l.import_view.mspdi_hint":
    "En Microsoft Project use Archivo → Guardar como → Formato XML. No se puede leer un archivo .mpp.",
  "c.CalendarMonth.january": "Enero",
  "c.CalendarMonth.february": "Febrero",
  "c.CalendarMonth.march": "Marzo",
  "c.CalendarMonth.april": "Abril",
  "c.CalendarMonth.may": "Mayo",
  "c.CalendarMonth.june": "Junio",
  "c.CalendarMonth.july": "Julio",
  "c.CalendarMonth.august": "Agosto",
  "c.CalendarMonth.september": "Septiembre",
  "c.CalendarMonth.october": "Octubre",
  "c.CalendarMonth.november": "Noviembre",
  "c.CalendarMonth.december": "Diciembre",
  "c.LilyGanttTable.schedule": "Cronograma",
  "c.LilyKanbanTable.board": "Tablero",
  "l.announcement_view.yesterday": "ayer",
  "c.CalendarMonth.holiday_state": "festivo",
  "c.CalendarMonth.absent_state": "ausente",
  "l.ev_view.word_ahead": "adelantado",
  "l.ev_view.word_behind": "retrasado",
  "l.ev_view.slipping": "retrasándose",
  "l.import_view.task_one": "{n} tarea",
  "l.import_view.task_other": "{n} tareas",
  "l.import_view.dependency_one": "{n} dependencia",
  "l.import_view.dependency_other": "{n} dependencias",
  "l.import_view.word_update": "actualizar",
  "l.import_view.word_updated": "actualizado",
  "l.rbs.headcount_one": "{headcount} persona",
  "l.rbs.headcount_other": "{headcount} personas",
  "l.scheduling_view.exactly": "exactamente",
  "l.scheduling_view.strength_strong": "fuerte",
  "l.scheduling_view.strength_moderate": "moderada",
  "l.scheduling_view.strength_weak": "débil",
  "l.simulation_inputs.planned_word": "previsto",
  "p.plans.pid.board.unassigned": "sin asignar",
  "p.plans.pid.costs.recorded_one":
    "Se registró {recorded} entrada de tiempo; {already_recorded} ya estaban registradas; {length} no tienen tarifa.",
  "p.plans.pid.costs.recorded_other":
    "Se registraron {recorded} entradas de tiempo; {already_recorded} ya estaban registradas; {length} no tienen tarifa.",
  "c.CostRiskView.histogram_label":
    "Histograma del coste simulado en {currency}, con P50, P80, P90 y el presupuesto marcados",
  "c.CumulativeFlow.chart_label":
    "Flujo acumulado durante {days} días naturales, con un máximo de {peak} tareas",
  "p.plans.pid.onboarding.owner_of": "Responsable de {title}",
  "p.plans.pid.onboarding.due_date_of": "Fecha de vencimiento de {title}",
  "p.plans.pid.onboarding.why_skip_title": "Por qué omitir {title}",
  "p.plans.pid.what_if.remove_edit": "Eliminar el cambio {n}",
  "c.LilyGanttTable.cell_active": "{name}: {start} a {end}{ends}",
  "c.LilyGanttTable.ends_here": " (termina aquí)",
  "c.LilyGanttTable.cell_not_scheduled": "{name}: sin programar",
  "l.simulation_inputs.range_business_days":
    "{optimistic} / {likely} / {pessimistic} días hábiles (optimista / probable / pesimista)",
  "l.enum.task_status.todo": "pendiente",
  "l.enum.task_status.in_progress": "en curso",
  "l.enum.task_status.in_review": "en revisión",
  "l.enum.task_status.done": "hecho",
  "l.enum.task_status.blocked": "bloqueado",
  "l.enum.kind.Portfolio": "Cartera",
  "l.enum.kind.Project": "Proyecto",
  "l.enum.kind.Product": "Producto",
  "l.enum.kind.Program": "Programa",
  "l.enum.kind.Practice": "Práctica",
  "l.enum.kind.Process": "Proceso",
  "l.enum.kind.Purpose": "Propósito",
  "l.enum.kind.Pathway": "Itinerario",
  "l.enum.kind.Proposal": "Propuesta",
  "l.enum.plan_status.Proposed": "Propuesto",
  "l.enum.plan_status.Active": "Activo",
  "l.enum.plan_status.OnHold": "En espera",
  "l.enum.plan_status.Completed": "Completado",
  "l.enum.plan_status.Cancelled": "Cancelado",
  "l.enum.collection.portfolios": "carteras",
  "l.enum.collection.projects": "proyectos",
  "l.enum.collection.products": "productos",
  "l.enum.collection.programs": "programas",
  "l.enum.collection.practices": "prácticas",
  "l.enum.collection.processes": "procesos",
  "l.enum.collection.purposes": "propósitos",
  "l.enum.collection.pathways": "itinerarios",
  "l.enum.proposal_status.draft": "borrador",
  "l.enum.proposal_status.submitted": "enviada",
  "l.enum.proposal_status.in_review": "en revisión",
  "l.enum.proposal_status.approved": "aprobada",
  "l.enum.proposal_status.rejected": "rechazada",
  "l.enum.proposal_status.promoted": "promovida",
  "l.enum.band.high": "alta",
  "l.enum.band.medium": "media",
  "l.enum.band.low": "baja",
  "l.enum.band.unscored": "sin puntuar",
  "l.enum.api.unauthorized": "no autorizado",
  "l.enum.api.forbidden": "prohibido",
  "l.enum.api.not_found": "no encontrado",
  "l.enum.api.internal_error": "Error interno",
  "l.enum.api.rate_limited": "límite de solicitudes superado",
  "l.enum.api.http_status": "HTTP {status}",
  "p.error.not_found": "Esa página no existe.",
  "p.error.not_allowed": "No tiene permiso para ver esa página.",
  "p.error.went_wrong": "Algo salió mal. Inténtelo de nuevo.",
  "p.error.title": "Error {status} — PPM",
  "p.error.back": "Volver al inicio",
  "p.issues.issues_ppm": "Registro de incidencias — PPM",
  "p.issues.issues": "Registro de incidencias",
  "p.issues.intro":
    "Una incidencia es algo que ya ha ocurrido y hay que gestionar; un riesgo es algo que podría ocurrir. Primero las más graves y después las más antiguas. La antigüedad se mide en días naturales desde que se registró la incidencia.",
  "p.issues.could_not_load": "No se pudieron cargar las incidencias",
  "p.issues.by_severity": "Incidencias por gravedad",
  "p.issues.overdue_count": "Vencidas",
  "p.issues.oldest_open":
    "Abierta más antigua: {title} ({days} días naturales)",
  "p.issues.show": "Mostrar",
  "p.issues.all_statuses": "Todos los estados",
  "p.issues.open_only": "Solo abiertas y en curso",
  "p.issues.severity_filter": "Gravedad",
  "p.issues.all_severities": "Todas las gravedades",
  "p.issues.older_than": "Con al menos estos días naturales de antigüedad",
  "p.issues.none": "No hay incidencias registradas.",
  "p.issues.none_match": "Ninguna incidencia coincide.",
  "p.issues.loading": "Cargando…",
  "p.issues.count_one": "{n} incidencia, las más graves primero",
  "p.issues.count_other": "{n} incidencias, las más graves primero",
  "p.issues.truncated":
    "Se muestran las primeras {n}; acote los filtros para ver el resto.",
  "p.issues.title_col": "Incidencia",
  "p.issues.kind_col": "Tipo",
  "p.issues.severity_col": "Gravedad",
  "p.issues.status_col": "Estado",
  "p.issues.owner_col": "Responsable",
  "p.issues.age_col": "Antigüedad (días naturales)",
  "p.issues.due_col": "Vencimiento",
  "p.issues.overdue_flag": "Vencida",
  "p.issues.no_owner": "Sin responsable",
  "p.issues.no_due": "Sin fecha de vencimiento",
  "p.issues.plan_link": "Abrir las incidencias del plan",
  "p.issues.plan_issues_link": "Incidencias",
  "p.plan_issues.title_ppm": "Incidencias del plan — PPM",
  "p.plan_issues.title": "Incidencias del plan",
  "p.plan_issues.intro":
    "Las incidencias registradas en este plan, primero las más graves. Una incidencia es algo que ya ha ocurrido; un riesgo es algo que podría ocurrir. Registre una incidencia, edite una o regístrela a partir de un riesgo que se ha materializado.",
  "p.plan_issues.back": "Volver al plan",
  "p.plan_issues.register_link": "Incidencias de todos los planes",
  "p.plan_issues.could_not_load":
    "No se pudieron cargar las incidencias del plan",
  "p.plan_issues.could_not_save": "No se pudo guardar la incidencia",
  "p.plan_issues.counts":
    "{total} en total, {open} abiertas o en curso, {overdue} vencidas",
  "p.plan_issues.actions_col": "Acciones",
  "p.plan_issues.edit": "Editar",
  "p.plan_issues.raise_heading": "Registrar una incidencia",
  "p.plan_issues.edit_heading": "Editar la incidencia",
  "p.plan_issues.f_title": "Título",
  "p.plan_issues.f_description": "Descripción",
  "p.plan_issues.f_owner": "Responsable (una referencia person: o worker:)",
  "p.plan_issues.f_raised_on": "Registrada el (déjelo vacío para hoy)",
  "p.plan_issues.f_resolution":
    "Resolución (necesaria para cerrar la incidencia)",
  "p.plan_issues.raise": "Registrar incidencia",
  "p.plan_issues.save": "Guardar cambios",
  "p.plan_issues.cancel_edit": "Cancelar la edición",
  "p.plan_issues.raised": "Se registró la incidencia.",
  "p.plan_issues.saved": "Se guardó la incidencia.",
  "p.plan_issues.problem_title_required":
    "Indique un título para la incidencia.",
  "p.plan_issues.problem_resolution_required":
    "Una incidencia cerrada necesita una resolución: indique cómo se resolvió.",
  "p.plan_issues.problem_due_before_raised":
    "La fecha de vencimiento no puede ser anterior al día en que se registró la incidencia.",
  "p.plan_issues.from_risk_heading":
    "Registrar una incidencia a partir de un riesgo",
  "p.plan_issues.from_risk_intro":
    "Para un riesgo que se ha materializado. La incidencia toma el título, la descripción y el responsable del riesgo, y una gravedad según su exposición; el riesgo en sí no cambia. Un riesgo puede tener una sola incidencia abierta a la vez.",
  "p.plan_issues.f_risk": "Riesgo",
  "p.plan_issues.choose_risk": "Elija un riesgo",
  "p.plan_issues.raise_from_risk":
    "Registrar incidencia a partir de este riesgo",
  "p.plan_issues.raised_from_risk":
    "Se registró la incidencia a partir del riesgo.",
  "l.enum.severity.low": "baja",
  "l.enum.severity.med": "media",
  "l.enum.severity.high": "alta",
  "l.enum.severity.critical": "crítica",
  "l.enum.issue_status.open": "abierta",
  "l.enum.issue_status.in_progress": "en curso",
  "l.enum.issue_status.resolved": "resuelta",
  "l.enum.issue_status.closed": "cerrada",
  "l.enum.issue_kind.bug": "error",
  "l.enum.issue_kind.risk": "riesgo",
  "l.enum.issue_kind.blocker": "bloqueo",
  "l.enum.issue_kind.question": "pregunta",
  "l.enum.issue_kind.improvement": "mejora",
  "l.enum.issue_kind.concern": "inquietud",
  "p.signin.oidc_idle":
    "Se cerró su sesión por inactividad prolongada. Vuelva a iniciar sesión.",
  "p.plans.pid.board.only_first_tasks":
    "Este plan tiene {total} tareas; el tablero muestra las primeras {shown}. Acótelo por componente o consulte el resto a través de la API.",
  "p.plan_modules.title": "Herramientas de este plan",
  "p.plan_modules.title_ppm": "Herramientas de este plan — PPM",
  "p.plan_modules.back": "← Volver al plan",
  "p.plan_modules.intro":
    "Elija qué herramientas muestra este plan. Una herramienta desactivada se oculta en la página del plan y sus páginas responden «no encontrado». No se elimina nada: al volver a activar una herramienta se muestra lo que contenía.",
  "p.plan_modules.source_plan":
    "Este plan tiene su propia selección de herramientas.",
  "p.plan_modules.source_default":
    "Este plan usa las herramientas predeterminadas de la instalación.",
  "p.plan_modules.legend": "Herramientas que muestra este plan",
  "p.plan_modules.module_tasks":
    "Tareas, el tablero y el análisis del cronograma",
  "p.plan_modules.module_time": "Registro de tiempo",
  "p.plan_modules.module_risks": "Riesgos e incidencias",
  "p.plan_modules.module_financials": "Presupuestos, costes y valor ganado",
  "p.plan_modules.module_milestones": "Hitos, el cronograma y el calendario",
  "p.plan_modules.module_governance":
    "Gobernanza, puertas y solicitudes de cambio",
  "p.plan_modules.module_roadmap": "Hoja de ruta pública del producto",
  "p.plan_modules.none_chosen":
    "Sin ninguna herramienta elegida, la página del plan muestra su registro y ninguna herramienta.",
  "p.plan_modules.save": "Guardar herramientas",
  "p.plan_modules.presets_heading": "Selecciones predefinidas",
  "p.plan_modules.personal_is":
    "La selección personal, para el trabajo propio de una persona: {names}.",
  "p.plan_modules.use_personal": "Usar la selección personal",
  "p.plan_modules.default_is":
    "La selección predeterminada de la instalación: {names}.",
  "p.plan_modules.use_default": "Seguir la selección predeterminada",
  "p.plan_modules.no_tools": "ninguna herramienta",
  "p.plan_modules.unchanged":
    "Nada que guardar: este plan ya sigue la selección predeterminada, que es esta.",
  "p.plan_modules.saved": "Guardado.",
  "p.plan_modules.could_not_load":
    "No se pudieron cargar las herramientas de este plan.",
  "p.plan_modules.could_not_save": "El cambio no se guardó.",
  "p.canvas.title": "Lienzos de estrategia",
  "p.canvas.title_ppm": "Lienzos de estrategia — PPM",
  "p.canvas.intro":
    "Un lienzo es un tablero de una página con casillas, como un Lean Canvas o un análisis DAFO. Un elemento puede convertirse en una idea, un objetivo o, en el lienzo de un plan, un riesgo.",
  "p.canvas.plan_title": "Lienzos de este plan",
  "p.canvas.plan_title_ppm": "Lienzos de este plan — PPM",
  "p.canvas.plan_intro":
    "Los lienzos de estrategia de este plan. Un elemento de uno de ellos puede convertirse en un riesgo del registro de este plan.",
  "p.canvas.one_title": "Lienzo de estrategia",
  "p.canvas.one_title_ppm": "Lienzo de estrategia — PPM",
  "p.canvas.back": "← Volver a los lienzos",
  "p.canvas.list_heading": "Lienzos",
  "p.canvas.filter": "Mostrar",
  "p.canvas.filter_all": "Todos los tipos",
  "p.canvas.none_yet": "Todavía no hay lienzos.",
  "p.canvas.none_of_kind": "No hay lienzos de este tipo.",
  "p.canvas.free_standing": "sin plan",
  "p.canvas.on_plan": "en un plan",
  "p.canvas.new_heading": "Empezar un lienzo",
  "p.canvas.new_on_plan": "El nuevo lienzo está en este plan.",
  "p.canvas.new_free_standing":
    "El nuevo lienzo no está en ningún plan, así que sus elementos pueden convertirse en ideas u objetivos, pero no en riesgos.",
  "p.canvas.f_template": "Tipo de lienzo",
  "p.canvas.f_title": "Título",
  "p.canvas.f_description": "Descripción (opcional)",
  "p.canvas.create": "Empezar el lienzo",
  "p.canvas.could_not_load_list": "No se pudieron cargar los lienzos.",
  "p.canvas.could_not_load_templates":
    "No se pudieron cargar los tipos de lienzo, así que no se puede empezar uno nuevo.",
  "p.canvas.could_not_create": "No se empezó el lienzo.",
  "p.canvas.problem_title_blank": "Dé un título al lienzo.",
  "p.canvas.problem_title_long":
    "Un título puede tener como máximo 200 caracteres.",
  "p.canvas.problem_description_long":
    "Una descripción puede tener como máximo 2.000 caracteres.",
  "p.canvas.template_lean": "Lienzo Lean",
  "p.canvas.template_business_model": "Lienzo de modelo de negocio",
  "p.canvas.template_swot": "Análisis DAFO",
  "p.canvas.template_empathy": "Mapa de empatía",
  "p.canvas.template_logic_model": "Modelo lógico",
  "p.canvas.template_risk": "Lienzo de riesgos",
  "p.canvas.template_goal": "Lienzo de metas",
  "p.canvas.box_problem": "Problema",
  "p.canvas.box_solution": "Solución",
  "p.canvas.box_key_metrics": "Métricas clave",
  "p.canvas.box_unique_value_proposition": "Propuesta de valor única",
  "p.canvas.box_unfair_advantage": "Ventaja injusta",
  "p.canvas.box_channels": "Canales",
  "p.canvas.box_customer_segments": "Segmentos de clientes",
  "p.canvas.box_cost_structure": "Estructura de costes",
  "p.canvas.box_revenue_streams": "Fuentes de ingresos",
  "p.canvas.box_key_partners": "Socios clave",
  "p.canvas.box_key_activities": "Actividades clave",
  "p.canvas.box_key_resources": "Recursos clave",
  "p.canvas.box_value_propositions": "Propuestas de valor",
  "p.canvas.box_customer_relationships": "Relaciones con clientes",
  "p.canvas.box_strengths": "Fortalezas",
  "p.canvas.box_weaknesses": "Debilidades",
  "p.canvas.box_opportunities": "Oportunidades",
  "p.canvas.box_threats": "Amenazas",
  "p.canvas.box_says": "Dice",
  "p.canvas.box_thinks": "Piensa",
  "p.canvas.box_does": "Hace",
  "p.canvas.box_feels": "Siente",
  "p.canvas.box_pains": "Dolores",
  "p.canvas.box_gains": "Ganancias",
  "p.canvas.box_inputs": "Insumos",
  "p.canvas.box_activities": "Actividades",
  "p.canvas.box_outputs": "Productos",
  "p.canvas.box_outcomes": "Resultados",
  "p.canvas.box_impact": "Impacto",
  "p.canvas.box_vulnerabilities": "Vulnerabilidades",
  "p.canvas.box_existing_controls": "Controles existentes",
  "p.canvas.box_residual_risks": "Riesgos residuales",
  "p.canvas.box_goal": "Meta",
  "p.canvas.box_success_measures": "Medidas de éxito",
  "p.canvas.box_actions": "Acciones",
  "p.canvas.box_obstacles": "Obstáculos",
  "p.canvas.box_resources": "Recursos",
  "p.canvas.box_empty": "Aquí no hay nada todavía.",
  "p.canvas.f_new_item": "Nuevo elemento en {box}",
  "p.canvas.add": "Añadir",
  "p.canvas.added": "Añadido.",
  "p.canvas.f_text": "Texto",
  "p.canvas.f_box": "Casilla",
  "p.canvas.save": "Guardar",
  "p.canvas.cancel": "Cancelar",
  "p.canvas.saved": "Guardado.",
  "p.canvas.unchanged": "No cambió nada, así que no se envió nada.",
  "p.canvas.edit": "Editar",
  "p.canvas.remove": "Quitar",
  "p.canvas.removed": "Quitado.",
  "p.canvas.confirm_remove_item": "¿Quitar «{text}» del lienzo?",
  "p.canvas.promote": "Convertir en…",
  "p.canvas.promote_legend": "Convertir este elemento en",
  "p.canvas.target_idea": "una idea",
  "p.canvas.target_objective": "un objetivo",
  "p.canvas.target_risk": "un riesgo de este plan",
  "p.canvas.promoted": "Convertido en {kind}.",
  "p.canvas.promote_once":
    "Un elemento solo puede convertirse en algo una vez.",
  "p.canvas.promote_go": "Convertirlo en esto",
  "p.canvas.no_risk_free_standing":
    "Este lienzo no está en ningún plan, así que un elemento no puede convertirse en un riesgo.",
  "p.canvas.f_probability": "Probabilidad (1 a 5)",
  "p.canvas.f_impact": "Impacto (1 a 5)",
  "p.canvas.problem_risk_scores":
    "Un riesgo necesita una probabilidad y un impacto, cada uno un número entero del 1 al 5.",
  "p.canvas.problem_text_blank": "Escriba primero el texto del elemento.",
  "p.canvas.problem_text_long":
    "Un elemento puede tener como máximo 2.000 caracteres.",
  "p.canvas.became_idea": "Se convirtió en una idea",
  "p.canvas.became_risk": "Se convirtió en un riesgo de este plan",
  "p.canvas.became_objective": "Se convirtió en un objetivo",
  "p.canvas.owner": "Responsable: {owner}",
  "p.canvas.full":
    "Este lienzo tiene el máximo de elementos posible ({max}); quite uno para añadir otro.",
  "p.canvas.could_not_load": "No se pudo cargar este lienzo.",
  "p.canvas.could_not_add": "No se añadió el elemento.",
  "p.canvas.could_not_save": "No se guardó el cambio.",
  "p.canvas.could_not_remove": "No se quitó el elemento.",
  "p.canvas.could_not_promote": "El elemento no se convirtió en nada.",
  "p.canvas.remove_canvas_heading": "Quitar el lienzo",
  "p.canvas.remove_canvas_says":
    "Quitar el lienzo quita sus elementos. Las ideas, objetivos y riesgos en que se convirtieron sus elementos se mantienen.",
  "p.canvas.remove_canvas": "Quitar este lienzo",
  "p.canvas.confirm_remove_canvas":
    "¿Quitar el lienzo «{title}» y sus elementos?",
  "p.canvas.could_not_remove_canvas": "No se quitó el lienzo.",
  "p.plan_templates.title": "Plantillas de plan",
  "p.plan_templates.title_ppm": "Plantillas de plan — PPM",
  "p.plan_templates.intro":
    "Una plantilla es un plan y los planes que tiene debajo, guardados sin personas y con cada fecha como un número de días tras el inicio, para que nuevos planes puedan empezar a partir de ella.",
  "p.plan_templates.saved_heading": "Plantillas guardadas",
  "p.plan_templates.starters_heading": "Plantillas de inicio incluidas",
  "p.plan_templates.none_saved":
    "Todavía no se ha guardado ninguna plantilla. Guarde una desde la página de un plan.",
  "p.plan_templates.none_starters": "No hay plantillas de inicio incluidas.",
  "p.plan_templates.plans_count_one": "{count} plan incluido",
  "p.plan_templates.plans_count_other": "{count} planes incluidos",
  "p.plan_templates.needs_start": "necesita una fecha de inicio",
  "p.plan_templates.could_not_load_list":
    "No se pudieron cargar las plantillas.",
  "p.plan_templates.one_title": "Plantilla de plan",
  "p.plan_templates.one_title_ppm": "Plantilla de plan — PPM",
  "p.plan_templates.back": "← Volver a las plantillas",
  "p.plan_templates.is_starter":
    "Una plantilla de inicio incluida. No se puede cambiar ni quitar; empiece un plan a partir de ella y guárdelo con su propio nombre.",
  "p.plan_templates.is_saved": "Una plantilla guardada, versión {version}.",
  "p.plan_templates.holds_heading": "Qué contiene",
  "p.plan_templates.holds_says":
    "Cada plan, el número de sus tareas y sus hitos como días tras el inicio. No se guarda ninguna persona.",
  "p.plan_templates.tasks_count_one": "{count} tarea",
  "p.plan_templates.tasks_count_other": "{count} tareas",
  "p.plan_templates.milestone_at": "{name}: día {days}",
  "p.plan_templates.start_heading": "Empezar planes a partir de ella",
  "p.plan_templates.start_says":
    "Los nuevos planes se crean a partir de la plantilla, con las fechas contadas desde la fecha de inicio. Las tareas empiezan en su primer paso.",
  "p.plan_templates.f_plan_name": "Nombre del plan superior (opcional)",
  "p.plan_templates.f_start": "Fecha de inicio",
  "p.plan_templates.no_dates":
    "Esta plantilla no tiene fechas, así que no hace falta fecha de inicio.",
  "p.plan_templates.f_parent": "Ponerlo bajo",
  "p.plan_templates.no_parent": "Ningún plan: un nuevo plan de primer nivel",
  "p.plan_templates.start": "Empezar los planes",
  "p.plan_templates.started":
    "Hecho. Planes: {plans}; tareas: {tasks}; hitos: {milestones}.",
  "p.plan_templates.open_new": "Abrir el nuevo plan",
  "p.plan_templates.could_not_load": "No se pudo cargar esta plantilla.",
  "p.plan_templates.could_not_start": "No se empezaron los planes.",
  "p.plan_templates.problem_plan_name_long":
    "El nombre de un plan puede tener como máximo 1.024 caracteres.",
  "p.plan_templates.problem_start_needed":
    "Elija una fecha de inicio: esta plantilla tiene fechas.",
  "p.plan_templates.problem_start_bad":
    "Escriba la fecha de inicio como un día, por ejemplo 2027-01-04.",
  "p.plan_templates.remove_heading": "Quitar la plantilla",
  "p.plan_templates.remove_says":
    "Quitar la plantilla no cambia los planes ya creados a partir de ella.",
  "p.plan_templates.remove": "Quitar esta plantilla",
  "p.plan_templates.confirm_remove": "¿Quitar la plantilla «{name}»?",
  "p.plan_templates.could_not_remove": "No se quitó la plantilla.",
  "p.plan_templates.save_title": "Guardar como plantilla",
  "p.plan_templates.save_title_ppm": "Guardar como plantilla — PPM",
  "p.plan_templates.save_says":
    "Guarda este plan y los planes que tiene debajo y que usted puede leer, con sus tareas, componentes, hitos y flujos de trabajo. No se guarda ninguna persona, y cada fecha pasa a ser un número de días tras el inicio. Cambiar el plan después no cambia la plantilla.",
  "p.plan_templates.f_name": "Nombre de la plantilla",
  "p.plan_templates.f_description": "Descripción (opcional)",
  "p.plan_templates.save": "Guardar la plantilla",
  "p.plan_templates.saved":
    "Guardada. Planes: {plans}; tareas: {tasks}; hitos: {milestones}.",
  "p.plan_templates.omitted_one":
    "Se dejó fuera {count} plan que está debajo, porque usted no puede leerlo.",
  "p.plan_templates.omitted_other":
    "Se dejaron fuera {count} planes que están debajo, porque usted no puede leerlos.",
  "p.plan_templates.open_template": "Abrir la plantilla",
  "p.plan_templates.name_taken":
    "Ya existe una plantilla con este nombre. Elija otro nombre.",
  "p.plan_templates.could_not_save": "No se guardó la plantilla.",
  "p.plan_templates.problem_name_blank": "Dé un nombre a la plantilla.",
  "p.plan_templates.problem_name_long":
    "El nombre de una plantilla puede tener como máximo 120 caracteres.",
  "p.plan_export.title": "Exportar a Microsoft Project",
  "p.plan_export.title_ppm": "Exportar a Microsoft Project — PPM",
  "p.plan_export.intro":
    "Guarda las tareas de este plan como un archivo XML MSPDI, el formato que abren Microsoft Project y otras herramientas de planificación.",
  "p.plan_export.carried_heading": "Qué contiene el archivo",
  "p.plan_export.carried":
    "Cada tarea con su esquema bajo su tarea superior, fecha de inicio, duración en días laborables, hitos, restricciones, fecha límite y notas; si está terminada; los vínculos entre tareas con su tipo y su demora; y el calendario laboral del plan.",
  "p.plan_export.not_carried_heading": "Qué deja fuera",
  "p.plan_export.not_carried":
    "Personas asignadas y recursos; cualquier estado salvo terminada (una tarea está al 0 o al 100 por ciento); sprints y componentes; esfuerzo; campos personalizados; comentarios; líneas base; vínculos entre tareas distintos de «precede»; y demoras en tiempo transcurrido (toda demora va en días laborables). El archivo también lo dice.",
  "p.plan_export.limit": "No se puede exportar un plan de más de 5.000 tareas.",
  "p.plan_export.not_checked":
    "El importador de este servicio ha vuelto a leer el archivo, pero aún no se ha abierto en el propio Microsoft Project.",
  "p.plan_export.download": "Descargar el archivo MSPDI",
  "p.plan_export.to_import": "Traer un archivo MSPDI a este plan en su lugar",
  "p.confidence.assess_heading": "Añadir una valoración",
  "p.confidence.assess_says":
    "Un valor declarado es la opinión de una persona sobre si el plan cumplirá. Se añade al historial y nunca sustituye a uno anterior.",
  "p.confidence.choose_value": "Elija un valor",
  "p.confidence.could_not_load":
    "No se pudo cargar la confianza de entrega de este plan.",
  "p.confidence.could_not_save": "No se añadió la valoración.",
  "p.confidence.f_assessed_on": "Valorado el (opcional; hoy si se deja vacío)",
  "p.confidence.f_kind": "Quién lo dice",
  "p.confidence.f_rationale": "Por qué (al menos {min} caracteres)",
  "p.confidence.f_value": "Confianza de entrega",
  "p.confidence.history_heading": "Historial, lo más reciente primero",
  "p.confidence.intro":
    "La opinión declarada de una persona sobre si este plan cumplirá, guardada junto al semáforo calculado con los datos del plan y nunca mezclada con él.",
  "p.confidence.kind_independent": "Una valoración independiente",
  "p.confidence.kind_self": "El propio equipo del plan (autoevaluación)",
  "p.confidence.no_history": "Todavía no se ha declarado ninguna valoración.",
  "p.confidence.none_yet":
    "Sin valorar: nadie ha declarado una opinión todavía. No es lo mismo que verde.",
  "p.confidence.opens_exception":
    "Ámbar/rojo o rojo abre una excepción para este plan, o agrava la que ya está abierta. Luego alguien tiene que completarla.",
  "p.confidence.problem_future":
    "La fecha de la valoración no puede ser futura.",
  "p.confidence.problem_kind": "Elija quién lo dice.",
  "p.confidence.problem_rationale":
    "Diga por qué, en al menos {min} caracteres.",
  "p.confidence.problem_value": "Elija un valor.",
  "p.confidence.refused_independent":
    "Una valoración independiente necesita el derecho de evaluador independiente. Puede añadir una autoevaluación.",
  "p.confidence.save": "Añadir la valoración",
  "p.confidence.saved_exception_one":
    "Añadida. Este plan tiene ahora una excepción abierta: debe completarse en {hours} hora.",
  "p.confidence.saved_exception_other":
    "Añadida. Este plan tiene ahora una excepción abierta: debe completarse en {hours} horas.",
  "p.confidence.saved": "Añadida.",
  "p.confidence.shown_heading": "Lo que muestra el mapa de cartera",
  "p.confidence.shown_independent": "Valoración independiente del {when}.",
  "p.confidence.shown_self":
    "Autoevaluación del {when}; una valoración independiente se mostraría en su lugar.",
  "p.confidence.stale": "desfasada: anterior a este mes",
  "p.confidence.title_ppm": "Confianza de entrega — PPM",
  "p.confidence.title": "Confianza de entrega",
  "p.exceptions.a_plan": "Un plan",
  "p.exceptions.breached_one":
    "Incumplida: no se completó en {hours} hora desde que se abrió.",
  "p.exceptions.breached_other":
    "Incumplida: no se completó en {hours} horas desde que se abrió.",
  "p.exceptions.cancel": "Cancelar",
  "p.thresholds.could_not_load":
    "No se pudieron cargar el triaje y los umbrales.",
  "p.thresholds.could_not_save": "No se guardó el cambio.",
  "p.thresholds.could_not_sweep": "No se pudieron comprobar los umbrales.",
  "p.thresholds.decide_says":
    "Registrar la decisión cierra la excepción. Registra lo decidido; no pausa, reinicia ni detiene el programa, cosa que hace una persona mediante el estado del programa.",
  "p.thresholds.decide": "Registrar la decisión",
  "p.thresholds.decided_done": "Decisión registrada: {action}.",
  "p.thresholds.f_action": "Decisión",
  "p.thresholds.f_amount": "Importe del límite",
  "p.thresholds.f_count":
    "Paquetes emitidos en rojo seguidos (de {min} a {max})",
  "p.thresholds.f_currency": "Moneda (tres letras)",
  "p.thresholds.f_date": "Fecha de fin prevista que no debe superarse",
  "p.thresholds.f_expiry":
    "Anulación hasta (en el futuro, como mucho {days} días)",
  "p.thresholds.f_outcome": "Resultado",
  "p.thresholds.f_reason": "Motivo (al menos {min} caracteres)",
  "p.thresholds.history_item": "El {date}: {outcome}. {reason}",
  "p.thresholds.history": "Triajes anteriores",
  "p.thresholds.not_set": "Sin definir",
  "p.thresholds.now_cap": "Se cruza cuando el coste P80 supera {amount}.",
  "p.thresholds.now_forecast":
    "Se cruza cuando el fin previsto es posterior al {date}.",
  "p.thresholds.now_red_packs_one":
    "Se cruza cuando la confianza está en rojo en {n} paquete emitido seguido.",
  "p.thresholds.now_red_packs_other":
    "Se cruza cuando la confianza está en rojo en {n} paquetes emitidos seguidos.",
  "p.thresholds.opened_by_threshold":
    "Abierta porque se cruzó un umbral del programa.",
  "p.thresholds.problem_action": "Elija pausar, reiniciar, detener o anular.",
  "p.thresholds.problem_cap": "El límite debe ser un importe mayor que cero.",
  "p.thresholds.problem_count":
    "El número de paquetes debe ser un entero de {min} a {max}.",
  "p.thresholds.problem_currency":
    "La moneda debe ser un código ISO 4217 de tres letras.",
  "p.thresholds.problem_date": "Elija una fecha.",
  "p.thresholds.problem_expiry_long":
    "Una anulación puede durar como mucho {days} días.",
  "p.thresholds.problem_expiry_needed":
    "Una anulación necesita la fecha hasta la que se mantiene.",
  "p.thresholds.problem_expiry_past":
    "La fecha de una anulación debe estar en el futuro.",
  "p.thresholds.problem_outcome": "Elija terminar, corregir, pausar o detener.",
  "p.thresholds.problem_reason":
    "El motivo debe tener al menos {min} caracteres.",
  "p.thresholds.remove_threshold": "Quitar el umbral",
  "p.thresholds.save_threshold": "Guardar el umbral",
  "p.thresholds.save_triage": "Registrar el triaje",
  "p.thresholds.sweep_done_one":
    "{checked} umbrales comprobados; {n} excepción abierta.",
  "p.thresholds.sweep_done_other":
    "{checked} umbrales comprobados; {n} excepciones abiertas.",
  "p.thresholds.sweep_item": "{programme} — {kind}: {outcome}",
  "p.thresholds.sweep_list": "Umbrales cruzados",
  "p.thresholds.sweep_says":
    "Comprobar abre una excepción de entrega por cada umbral cruzado, o la añade a la que ya está abierta; no pausa, reinicia ni detiene nada.",
  "p.thresholds.sweep": "Comprobar los umbrales ahora",
  "p.thresholds.threshold_removed": "Umbral quitado: {kind}.",
  "p.thresholds.threshold_saved": "Umbral guardado: {kind}.",
  "p.thresholds.thresholds_heading": "Umbrales",
  "p.thresholds.thresholds_says":
    "Cruzar un umbral abre una excepción de entrega, que una persona responde con pausar, reiniciar, detener o una anulación con caducidad. Nada se pausa, reinicia ni detiene automáticamente.",
  "p.thresholds.triage_heading": "Triaje",
  "p.thresholds.triage_latest": "Último triaje, {date}: {outcome}. {reason}",
  "p.thresholds.triage_none": "Aún no hay triaje registrado.",
  "p.thresholds.triage_saved": "Triaje registrado.",
  "p.thresholds.triage_says":
    "Un triaje es una valoración fechada de hacia dónde debe ir el programa, con el motivo. Registra la valoración; no cambia nada del programa.",
  "p.thresholds.unchanged": "No cambió nada, así que no se envió nada.",
  "p.thresholds.action_pause": "Pausar",
  "p.thresholds.action_reset": "Reiniciar",
  "p.thresholds.action_stop": "Detener",
  "p.thresholds.action_override": "Anular",
  "p.thresholds.kind_confidence_red_packs":
    "Confianza en rojo durante varios paquetes",
  "p.thresholds.kind_forecast_past": "Fin previsto después de una fecha",
  "p.thresholds.kind_p80_cap": "Coste P80 por encima de un límite",
  "p.thresholds.outcome_finish": "Terminar",
  "p.thresholds.outcome_fix": "Corregir",
  "p.thresholds.outcome_pause": "Pausar",
  "p.thresholds.outcome_stop": "Detener",
  "p.thresholds.sweep_opened": "excepción abierta",
  "p.thresholds.sweep_already_open": "excepción ya abierta",
  "p.thresholds.sweep_attached_to_open_exception":
    "añadido a la excepción abierta",
  "p.thresholds.sweep_suppressed_by_override":
    "no abierta: hay una anulación vigente",
  "p.exceptions.closed_done": "Decisión registrada; la excepción está cerrada.",
  "p.exceptions.complete_it": "Completarla",
  "p.exceptions.complete_says":
    "Una excepción está completa cuando tiene su causa, la decisión necesaria, un responsable y una fecha límite de decisión. Una parte que se deja vacía conserva lo que tenía.",
  "p.exceptions.complete":
    "Completa: tiene su causa, la decisión necesaria, un responsable y una fecha límite de decisión.",
  "p.exceptions.conflict_closed": "Esta excepción ya está cerrada.",
  "p.exceptions.could_not_load": "No se pudieron cargar las excepciones.",
  "p.exceptions.could_not_save": "No se guardó el cambio.",
  "p.exceptions.decide_it": "Registrar la decisión",
  "p.exceptions.decide_says":
    "Registrar la decisión cierra la excepción. Que mejore la confianza nunca la cierra por sí solo.",
  "p.exceptions.decide": "Registrar la decisión y cerrar",
  "p.exceptions.f_breached": "Solo incumplidas",
  "p.exceptions.f_decision": "Decisión (al menos {min} caracteres)",
  "p.exceptions.f_owner_ref": "Responsable (una referencia de persona)",
  "p.exceptions.f_owner_role":
    "Rol del responsable (lo que muestra un paquete)",
  "p.exceptions.f_status": "Mostrar",
  "p.exceptions.intro_one":
    "Una excepción se abre cuando la confianza de entrega de un plan llega a ámbar/rojo o rojo. Debe completarse en {hours} hora y solo se cierra con una decisión registrada.",
  "p.exceptions.intro_other":
    "Una excepción se abre cuando la confianza de entrega de un plan llega a ámbar/rojo o rojo. Debe completarse en {hours} horas y solo se cierra con una decisión registrada.",
  "p.exceptions.is_closed": "cerrada el {when}",
  "p.exceptions.is_open": "abierta",
  "p.exceptions.missing": "Aún falta: {parts}.",
  "p.exceptions.none": "Ninguna excepción coincide.",
  "p.exceptions.opened": "Abierta el {when}",
  "p.exceptions.part_cause": "Causa",
  "p.exceptions.part_decide_by": "Decidir antes del",
  "p.exceptions.part_decision_needed": "Decisión necesaria",
  "p.exceptions.part_decision": "Decisión",
  "p.exceptions.part_owner": "Responsable",
  "p.exceptions.problem_decision":
    "Escriba la decisión con al menos {min} caracteres.",
  "p.exceptions.problem_part_short":
    "Una causa, una decisión necesaria o un responsable necesitan al menos {min} caracteres para contar.",
  "p.exceptions.refused_threshold":
    "Esta excepción la abrió un umbral, así que se cierra con una pausa, un reinicio, una parada o una anulación registrada, no con una decisión aquí.",
  "p.exceptions.save": "Guardar",
  "p.exceptions.saved": "Guardado.",
  "p.exceptions.see_register": "Abrir el registro de excepciones",
  "p.exceptions.status_all": "Todas las excepciones",
  "p.exceptions.status_closed": "Cerradas",
  "p.exceptions.status_open": "Abiertas",
  "p.exceptions.title_ppm": "Excepciones — PPM",
  "p.exceptions.title": "Excepciones",
  "p.exceptions.unchanged": "No cambió nada, así que no se envió nada.",
  "p.exceptions.worsened_one": "Empeoró {count} vez desde que se abrió",
  "p.exceptions.worsened_other": "Empeoró {count} veces desde que se abrió",
  "p.exceptions.worst": "Peor confianza: {value}",
  "p.aims.aims_heading": "Objetivos estratégicos",
  "p.aims.aims_legend": "Objetivos que son objetivos estratégicos",
  "p.aims.aims_says":
    "Un objetivo estratégico es un objetivo en torno al cual se organiza la cartera. Desmarcarlo también hace que deje de ser el objetivo principal de cualquier programa.",
  "p.aims.choose_aim": "Elija un objetivo",
  "p.aims.choose_programme": "Elija un programa",
  "p.aims.could_not_load": "No se pudo cargar el mapa de cartera.",
  "p.aims.could_not_save": "No se guardó el cambio.",
  "p.aims.f_aim": "Objetivo principal",
  "p.aims.f_period": "Mes (vacío para este mes)",
  "p.aims.f_programme": "Programa",
  "p.aims.intro":
    "Los programas que puede leer, cada uno una vez bajo su objetivo estratégico principal. La confianza de entrega es el valor declarado, junto al semáforo calculado con los datos y nunca mezclado con él; sin valorar no es verde.",
  "p.aims.map_heading": "Mapa de cartera, {period}",
  "p.aims.marked": "Marcado como objetivo estratégico.",
  "p.aims.no_aims":
    "Ningún objetivo es estratégico todavía; marque uno arriba primero.",
  "p.aims.no_objectives": "Todavía no hay objetivos.",
  "p.aims.no_programmes":
    "No hay programas en el mapa: un programa es un plan de tipo Programa.",
  "p.aims.primary_heading": "El objetivo principal de un programa",
  "p.aims.primary_says":
    "Un programa aparece en el mapa bajo su único objetivo principal; los demás objetivos a los que sirve se nombran a su lado. Elegir uno nuevo convierte el anterior en un vínculo normal.",
  "p.aims.primary_set": "Objetivo principal fijado.",
  "p.aims.problem_already_primary":
    "Ese ya es el objetivo principal de este programa.",
  "p.aims.problem_choose": "Elija un programa y un objetivo.",
  "p.aims.problem_period":
    "Elija el mes como AAAA-MM, o déjelo vacío para este mes.",
  "p.aims.set_primary": "Fijar el objetivo principal",
  "p.aims.show": "Mostrar",
  "p.aims.title_ppm": "Mapa de cartera — PPM",
  "p.aims.title": "Mapa de cartera",
  "p.aims.to_objectives": "Abrir los objetivos",
  "p.aims.unmarked": "Ya no es un objetivo estratégico.",
  "p.programme.cited_exception":
    "Se derivó de una excepción de entrega de este programa.",
  "p.programme.cited_review":
    "Se derivó de una revisión de fase de este programa.",
  "p.programme.cites_none": "Ninguna",
  "p.programme.conflict_status":
    "El programa ya tiene ese estado, o está detenido y sigue detenido.",
  "p.programme.could_not_decide": "No se registró la decisión.",
  "p.programme.could_not_load":
    "No se pudieron cargar las fases y el estado de este plan.",
  "p.programme.could_not_save_gates":
    "No se guardaron todas las fechas de fase; las que se muestran son las que tiene el servicio.",
  "p.programme.decide": "Registrar la decisión",
  "p.programme.decided": "Registrada: el programa está ahora {status}.",
  "p.programme.do_pause": "Pausar el programa",
  "p.programme.do_resume": "Reanudar el programa",
  "p.programme.do_stop": "Detener el programa",
  "p.programme.f_change": "Cambiar el programa a",
  "p.programme.f_exception": "Se deriva de una excepción (opcional)",
  "p.programme.f_reason": "Motivo (al menos {min} caracteres)",
  "p.programme.f_review": "Se deriva de una revisión de fase (opcional)",
  "p.programme.gates_heading": "Fechas previstas de fase",
  "p.programme.gates_says":
    "La fecha en que este plan espera su revisión en cada fase. Una fecha fijada no se puede borrar, solo mover.",
  "p.programme.gates_saved": "Fechas de fase guardadas.",
  "p.programme.gates_unchanged":
    "No cambió ninguna fecha de fase, así que no se envió nada.",
  "p.programme.held_allocations_one":
    "{count} asignación está marcada, no borrada.",
  "p.programme.held_allocations_other":
    "{count} asignaciones están marcadas, no borradas.",
  "p.programme.held_benefits_one":
    "{count} beneficio se conserva en el registro.",
  "p.programme.held_benefits_other":
    "{count} beneficios se conservan en el registro.",
  "p.programme.held_heading": "Lo que el programa aún mantiene",
  "p.programme.held_money": "Dinero: previsto {planned}, real {actual}.",
  "p.programme.history_heading": "Decisiones, lo más reciente primero",
  "p.programme.no_history": "Aún no hay decisiones: el programa está activo.",
  "p.programme.not_a_programme":
    "Solo un plan de tipo Programa tiene estado de programa. Sus fechas de fase pueden planificarse igualmente arriba.",
  "p.programme.problem_reason": "Dé el motivo con al menos {min} caracteres.",
  "p.programme.problem_status": "Elija el cambio.",
  "p.programme.refused_decider":
    "Pausar, reanudar o detener un programa necesita el derecho de decisor de programas.",
  "p.programme.refused_not_programme":
    "Solo un plan de tipo Programa tiene estado, y la revisión o excepción citada debe ser de este programa.",
  "p.programme.save_gates": "Guardar las fechas de fase",
  "p.programme.status_heading": "Estado del programa",
  "p.programme.status_is": "Estado: {status}",
  "p.programme.status_says":
    "Un programa está activo hasta que una decisión registrada lo pausa o lo detiene. Una pausa marca sus asignaciones y no borra ninguna.",
  "p.programme.stop_warns":
    "Detenerlo es definitivo: el programa sigue detenido y conserva su registro.",
  "p.programme.stopped_final":
    "Este programa está detenido. Es definitivo; se conserva su registro.",
  "p.programme.title_ppm": "Fases y estado del programa — PPM",
  "p.programme.title": "Fases y estado del programa",
  "p.programme.buffer_by_days": "Un número de días laborables",
  "p.programme.buffer_by_rule":
    "La mitad del trabajo crítico de la línea base, redondeada hacia arriba",
  "p.programme.buffer_declared_one":
    "Declarado: un colchón de {days} día laborable.",
  "p.programme.buffer_declared_other":
    "Declarado: un colchón de {days} días laborables.",
  "p.programme.buffer_heading": "Colchón del proyecto",
  "p.programme.buffer_says":
    "El tiempo que se reserva al final del programa para absorber retrasos. El gráfico compara cuánto se ha consumido con cuánto trabajo crítico está hecho. Una declaración nunca sustituye a otra anterior; la más reciente es el colchón. Si hay un plan de colchones de cadena crítica, se usa en su lugar.",
  "p.programme.buffer_history": "Declaraciones, lo más reciente primero",
  "p.programme.buffer_was_days_one": "{days} día laborable, declarado",
  "p.programme.buffer_was_days_other": "{days} días laborables, declarados",
  "p.programme.buffer_was_rule_one": "{days} día laborable, por la regla",
  "p.programme.buffer_was_rule_other": "{days} días laborables, por la regla",
  "p.programme.could_not_declare_buffer": "No se declaró el colchón.",
  "p.programme.could_not_load_buffer":
    "No se pudo leer el colchón del proyecto.",
  "p.programme.declare_buffer": "Declarar el colchón",
  "p.programme.f_buffer_days": "Días laborables (1 a {max})",
  "p.programme.f_buffer_how": "Cuánto dura el colchón",
  "p.programme.f_buffer_note": "Nota (opcional)",
  "p.programme.problem_buffer_days":
    "Escriba el colchón como un número entero de días laborables de 1 a 2000.",
  "p.programme.refused_buffer_rule":
    "La regla necesita una línea base del cronograma con trabajo crítico que tenga duración; fije primero una línea base o dé un número de días.",
  "p.programme.refused_not_programme_buffer":
    "Solo un plan de tipo Programa tiene colchón de proyecto.",
  "p.programme.profile_heading": "Modelo de entrega e informes",
  "p.programme.f_model": "Modelo de entrega",
  "p.programme.f_owner": "Organización responsable",
  "p.programme.no_owner": "Sin organización responsable",
  "p.programme.model_says":
    "El modelo de entrega dice quién construye y quién despliega; el paquete del director lo muestra junto al programa. El modelo y la organización se guardan juntos.",
  "p.programme.save_model": "Guardar el modelo de entrega",
  "p.programme.model_saved": "Se guardó el modelo de entrega.",
  "p.programme.f_method": "Cómo informa el programa de su avance",
  "p.programme.method_evm": "Valor ganado (rendimiento de coste y plazo)",
  "p.programme.method_product": "Burn-up y flujo (un programa de producto)",
  "p.programme.save_method": "Guardar el método de informe",
  "p.programme.method_saved_evm": "El programa ahora informa por valor ganado.",
  "p.programme.method_saved_product":
    "El programa ahora informa por burn-up y flujo.",
  "p.programme.profile_unchanged": "No cambió nada, así que no se envió nada.",
  "p.programme.could_not_load_profile":
    "No se pudo cargar el modelo de entrega.",
  "p.programme.could_not_save_profile": "No se guardó el cambio.",
  "p.benefits.classified": "Clasificación guardada.",
  "p.benefits.classify_says":
    "Guardar envía la clasificación completa: una opción dejada sin indicar se borra.",
  "p.benefits.could_not_load":
    "No se pudieron cargar los beneficios de este plan.",
  "p.benefits.could_not_save": "No se guardó la clasificación.",
  "p.benefits.could_not_validate": "No se registró la validación.",
  "p.benefits.f_annual": "Importe anual ({currency})",
  "p.benefits.f_classification": "Clasificación",
  "p.benefits.f_kpi": "KPI que lo mide",
  "p.benefits.f_note": "Nota (opcional)",
  "p.benefits.f_recurrence": "Recurrencia",
  "p.benefits.f_validated": "Importe validado hasta la fecha ({currency})",
  "p.benefits.intro":
    "Un importe realizado es una afirmación. Solo se valida con una validación registrada por alguien de fuera del programa; cada una indica el importe validado hasta la fecha, y cuenta la más reciente.",
  "p.benefits.measured_by_kpi":
    "Un beneficio no financiero se mide con su KPI, no se valida como importe.",
  "p.benefits.money_line":
    "Objetivo {target}; realizado (afirmado) {realized}.",
  "p.benefits.no_kpi": "Sin KPI",
  "p.benefits.non_financial": "Un beneficio no financiero.",
  "p.benefits.none": "Este plan aún no tiene beneficios.",
  "p.benefits.problem_annual_amount":
    "Escriba el importe anual como un número, no menor que 0.",
  "p.benefits.problem_annual_financial":
    "Solo un beneficio financiero, con moneda, tiene importe anual.",
  "p.benefits.problem_annual_recurring":
    "Solo un beneficio recurrente tiene importe anual: elija primero recurrente.",
  "p.benefits.problem_validated_amount":
    "Escriba el importe validado hasta la fecha como un número, no menor que 0.",
  "p.benefits.refused_member":
    "Quien valida no puede ser miembro del programa: pida a alguien de fuera que lo valide.",
  "p.benefits.save": "Guardar la clasificación",
  "p.benefits.standing_claimed": "Afirmado, aún sin validar",
  "p.benefits.standing_not_claimed": "Nada afirmado aún",
  "p.benefits.standing_partly_validated": "Validado en parte",
  "p.benefits.standing_validated": "Validado",
  "p.benefits.title_ppm": "Beneficios — PPM",
  "p.benefits.title": "Beneficios: clasificación y validación",
  "p.benefits.validate": "Registrar la validación",
  "p.benefits.validated_done": "Validación registrada.",
  "p.benefits.validated_to_date": "{amount} validado hasta la fecha",
  "p.benefits.validation_heading": "Validación",
  "p.benefits.validation_says":
    "Las validaciones se conservan tal como se registraron; una nueva indica el importe validado hasta la fecha y no sustituye a ninguna.",
  "p.funding.add": "Añadir la fuente",
  "p.funding.added": "Fuente de financiación añadida.",
  "p.funding.choose_plan": "Elija un plan",
  "p.funding.could_not_load_lines":
    "No se pudieron cargar las partidas de presupuesto de este plan.",
  "p.funding.could_not_load_money":
    "No se pudo cargar el dinero por fuente de financiación.",
  "p.funding.could_not_load":
    "No se pudieron cargar las fuentes de financiación.",
  "p.funding.could_not_save": "No se guardó el cambio.",
  "p.funding.f_combine": "Combinar capital y gasto corriente",
  "p.funding.f_fy": "Ejercicio (el año en que empieza; vacío para el actual)",
  "p.funding.f_name": "Nueva fuente de financiación",
  "p.funding.f_plan": "Plan elegido",
  "p.funding.f_source": "Fuente de financiación",
  "p.funding.intro":
    "De dónde viene el dinero de cada programa. Una partida de presupuesto sin fuente de financiación se cuenta y se lista, nunca se oculta.",
  "p.funding.is_active": "activa",
  "p.funding.is_retired": "retirada",
  "p.funding.line_cleared": "La partida ya no tiene fuente de financiación.",
  "p.funding.line_set": "Se ha fijado la fuente de financiación de la partida.",
  "p.funding.lines_heading":
    "Partidas de presupuesto y su fuente de financiación",
  "p.funding.lines_says":
    "Elija un plan para ver sus partidas de presupuesto y dar a cada una una fuente de financiación.",
  "p.funding.money_heading": "Dinero por fuente de financiación",
  "p.funding.no_lines": "Este plan no tiene partidas de presupuesto.",
  "p.funding.no_source": "Sin fuente de financiación",
  "p.funding.no_sources":
    "Todavía no se ha añadido ninguna fuente de financiación.",
  "p.funding.problem_fy":
    "Escriba el ejercicio como el año en que empieza, de 2000 a 2100.",
  "p.funding.problem_name_taken":
    "Ya existe una fuente de financiación con ese nombre.",
  "p.funding.problem_name":
    "El nombre de una fuente de financiación tiene entre 2 y 120 caracteres.",
  "p.funding.refused_inactive":
    "Esa fuente de financiación está retirada, así que no se puede dar a una partida.",
  "p.funding.restore": "Restaurar",
  "p.funding.restored": "Fuente de financiación restaurada.",
  "p.funding.retire_says":
    "Una fuente retirada ya no se puede dar a una partida, pero las partidas que la tienen la conservan.",
  "p.funding.retire": "Retirar",
  "p.funding.retired_option": "{name} (retirada)",
  "p.funding.retired": "Fuente de financiación retirada.",
  "p.funding.show": "Mostrar",
  "p.funding.sources_heading": "Fuentes de financiación",
  "p.funding.title_ppm": "Financiación — PPM",
  "p.funding.title": "Financiación",
  "p.director_pack.actions_heading": "Revisar, emitir o reemitir",
  "p.director_pack.aud_board": "Consejo",
  "p.director_pack.aud_escalation_board": "Comité de escalado",
  "p.director_pack.aud_executive": "Dirección",
  "p.director_pack.aud_org": "Organización: {name}",
  "p.director_pack.aud_partner": "Socios",
  "p.director_pack.choose_audience": "Elija una audiencia",
  "p.director_pack.conflict_draft":
    "Este periodo ya tiene un paquete para esa audiencia. Ábralo: emita su borrador o reemítalo con una nota de cambio.",
  "p.director_pack.conflict_issue":
    "Este paquete no se puede emitir ahora: ya está emitido, o es un borrador programado que la oficina de cartera no ha revisado.",
  "p.director_pack.conflict_reissue":
    "Solo se reemite la última versión emitida, y ya existe una versión posterior.",
  "p.director_pack.conflict_review":
    "Este borrador ya se ha revisado, o ya no es un borrador.",
  "p.director_pack.could_not_act": "No se hizo el cambio.",
  "p.director_pack.could_not_draft": "No se redactó el paquete.",
  "p.director_pack.cut_off_says":
    "Déjelo vacío para usar ahora. El paquete lee los planes tal como estaban en la fecha de corte; no puede ser futura ni de hace más de 400 días.",
  "p.director_pack.draft_heading": "Redactar un paquete",
  "p.director_pack.draft_says":
    "Redactar lee los planes que usted puede compartir tal como están en la fecha de corte y guarda el resultado como borrador. No se envía nada a la audiencia hasta que alguien lo emita.",
  "p.director_pack.draft": "Redactar el paquete",
  "p.director_pack.f_audience": "Audiencia",
  "p.director_pack.f_change_note": "Nota de cambio",
  "p.director_pack.f_cut_off": "Fecha de corte (opcional)",
  "p.director_pack.f_period": "Periodo (mes)",
  "p.director_pack.from_schedule": "hecho por la programación mensual",
  "p.director_pack.is_reviewed": "Revisado por la oficina de cartera.",
  "p.director_pack.is_stale":
    "desfasado: su fecha de corte es anterior al final de su periodo",
  "p.director_pack.issue_says":
    "Emitir congela el paquete y avisa a su audiencia. No se puede deshacer; un cambio posterior es una reemisión.",
  "p.director_pack.issue_waits":
    "La programación mensual hizo este borrador, así que la oficina de cartera lo revisa antes de que pueda emitirse.",
  "p.director_pack.issue": "Emitir el paquete",
  "p.director_pack.issued_done": "Emitido.",
  "p.director_pack.not_reviewed": "Aún no revisado.",
  "p.director_pack.problem_audience": "Elija una audiencia.",
  "p.director_pack.problem_cut_off_bad":
    "Escriba la fecha de corte como fecha y hora.",
  "p.director_pack.problem_cut_off_future":
    "La fecha de corte no puede ser futura.",
  "p.director_pack.problem_cut_off_old":
    "La fecha de corte no puede ser de hace más de 400 días.",
  "p.director_pack.problem_note":
    "Diga por qué hay una nueva versión, con al menos {min} caracteres.",
  "p.director_pack.problem_period": "Elija el periodo como un mes.",
  "p.director_pack.refused_edit":
    "Redactar, revisar y reemitir un paquete necesita acceso de editor de paquetes de dirección.",
  "p.director_pack.refused_issue":
    "Emitir un paquete necesita acceso de emisor de paquetes de dirección.",
  "p.director_pack.reissue_says":
    "Reemitir redacta la versión siguiente con su nota; esta versión sigue legible tal como está. Escriba al menos {min} caracteres.",
  "p.director_pack.reissue": "Reemitir con una nota de cambio",
  "p.director_pack.review": "Marcar como revisado",
  "p.director_pack.reviewed_done": "Marcado como revisado.",
  "p.director_pack.add_schedule_heading": "Añadir una programación",
  "p.director_pack.add_schedule": "Añadir la programación",
  "p.director_pack.conflict_schedule":
    "Esa audiencia ya tiene una programación; cambie su día en su lugar.",
  "p.director_pack.could_not_change_schedule": "No se cambió la programación.",
  "p.director_pack.could_not_load_schedules":
    "No se pudieron cargar las programaciones.",
  "p.director_pack.could_not_load_subscriptions":
    "No se pudo cargar de qué paquetes se le avisa.",
  "p.director_pack.could_not_subscribe": "No se guardó el cambio.",
  "p.director_pack.day_says":
    "Un día del 1 al 28, para que todos los meses lo tengan. La primera ejecución es el siguiente día así.",
  "p.director_pack.f_day": "Día del mes",
  "p.director_pack.last_drafted":
    "Última ejecución {when}: redactó el paquete de {period}.",
  "p.director_pack.last_run": "Última ejecución {when}.",
  "p.director_pack.last_skipped":
    "Última ejecución {when}: omitió {period}, que ya tenía paquete.",
  "p.director_pack.move_day": "Pasar a este día",
  "p.director_pack.no_schedules":
    "Todavía no se redacta ningún paquete según una programación.",
  "p.director_pack.problem_day": "Elija un día del mes del 1 al 28.",
  "p.director_pack.refused_subscribe":
    "Solo se le puede avisar de las audiencias cuyos paquetes puede leer.",
  "p.director_pack.schedule_added": "Programación añadida.",
  "p.director_pack.schedule_off":
    "Desactivada. Redactaría el día {day} de cada mes.",
  "p.director_pack.schedule_on":
    "Redacta el día {day} de cada mes; la próxima, el {next}.",
  "p.director_pack.schedule_saved": "Guardado.",
  "p.director_pack.schedule_switched_off": "Desactivada.",
  "p.director_pack.schedule_switched_on": "Activada.",
  "p.director_pack.schedule_unchanged":
    "Ese ya es su día, así que no se envió nada.",
  "p.director_pack.schedules_intro":
    "Cada programación redacta el paquete de una audiencia un día de cada mes. Nunca emite ninguno: la oficina de cartera revisa el borrador y una persona lo emite.",
  "p.director_pack.schedules_link": "Programaciones mensuales",
  "p.director_pack.schedules_title_ppm": "Programaciones mensuales — PPM",
  "p.director_pack.schedules_title": "Programaciones mensuales",
  "p.director_pack.subs_heading": "Avisarme cuando se emita un paquete",
  "p.director_pack.subs_legend": "Paquetes de los que avisar",
  "p.director_pack.subs_says":
    "Se le avisa en el servicio, con un enlace, cuando se emite un paquete para una audiencia que elija. El aviso nunca contiene el paquete.",
  "p.director_pack.subscribed": "Se le avisará.",
  "p.director_pack.sweep_heading": "Ejecutar lo pendiente ahora",
  "p.director_pack.sweep_result":
    "Redactados: {drafted}; omitidos porque ya había paquete: {skipped}.",
  "p.director_pack.sweep_says":
    "Redacta cada paquete cuyo día ha llegado y que aún no existe. No se emite nada.",
  "p.director_pack.sweep": "Ejecutar las programaciones pendientes",
  "p.director_pack.switch_off": "Desactivar",
  "p.director_pack.switch_on": "Activar",
  "p.director_pack.unsubscribed": "Ya no se le avisará.",
  "p.plans.pid.modules_failed":
    "No se pudo comprobar qué herramientas usa este plan, por lo que se muestran todas.",
  "p._layout_svelte.handling_mark": "Marca de tratamiento: {mark}",
  "p._layout_svelte.handling_mark_unknown":
    "Marca de tratamiento: no se pudo comprobar. Trate este plan como marcado hasta que se compruebe.",
  "l.enum.cr_status.draft": "borrador",
  "l.enum.cr_status.submitted": "enviada",
  "l.enum.cr_status.under_review": "en revisión",
  "l.enum.cr_status.approved": "aprobada",
  "l.enum.cr_status.rejected": "rechazada",
  "l.enum.cr_status.implemented": "implementada",
  "l.enum.cr_status.withdrawn": "retirada",
  "l.enum.cr_kind.scope": "alcance",
  "l.enum.cr_kind.schedule": "cronograma",
  "l.enum.cr_kind.cost": "coste",
  "l.enum.cr_kind.quality": "calidad",
  "l.enum.cr_kind.other": "otro",
  "p.plan_change_requests.title_ppm": "Solicitudes de cambio del plan — PPM",
  "p.plan_change_requests.title": "Solicitudes de cambio del plan",
  "p.plan_change_requests.intro":
    "Las solicitudes de cambio de este plan, las más recientes primero. Redacte una y edítela mientras sea un borrador; envíela a una persona aprobadora designada, que no sea usted; esa persona la revisa y registra la decisión con un motivo. Una solicitud abierta se puede retirar.",
  "p.plan_change_requests.register_link":
    "Solicitudes de cambio de todos los planes",
  "p.plan_change_requests.could_not_load":
    "No se pudieron cargar las solicitudes de cambio del plan",
  "p.plan_change_requests.could_not_save":
    "No se pudo guardar la solicitud de cambio",
  "p.plan_change_requests.steps": "Pasos",
  "p.plan_change_requests.draft_heading": "Redactar una solicitud de cambio",
  "p.plan_change_requests.edit_heading": "Editar el borrador",
  "p.plan_change_requests.f_description": "Descripción (qué ha de cambiar)",
  "p.plan_change_requests.f_reason": "Motivo (por qué se necesita el cambio)",
  "p.plan_change_requests.f_days":
    "Impacto en el calendario en días hábiles (negativo adelanta el final; déjelo vacío si no se ha evaluado)",
  "p.plan_change_requests.f_cost":
    "Impacto en el coste (déjelo vacío si no se ha evaluado)",
  "p.plan_change_requests.f_currency":
    "Moneda del coste (tres letras, por ejemplo EUR)",
  "p.plan_change_requests.f_summary": "Resumen del impacto",
  "p.plan_change_requests.save_draft": "Guardar el borrador",
  "p.plan_change_requests.drafted": "Se redactó la solicitud de cambio.",
  "p.plan_change_requests.saved": "Se guardó el borrador.",
  "p.plan_change_requests.problem_title_required":
    "Dé un título a la solicitud de cambio.",
  "p.plan_change_requests.problem_description_required":
    "Indique qué ha de cambiar.",
  "p.plan_change_requests.problem_reason_required":
    "Indique por qué se necesita el cambio.",
  "p.plan_change_requests.problem_days_not_whole":
    "El impacto en el calendario es un número entero de días hábiles.",
  "p.plan_change_requests.problem_days_out_of_range":
    "El impacto en el calendario debe estar entre -3650 y 3650 días hábiles.",
  "p.plan_change_requests.problem_cost_not_amount":
    "El impacto en el coste no es un importe.",
  "p.plan_change_requests.problem_cost_needs_currency":
    "Un impacto en el coste necesita su moneda.",
  "p.plan_change_requests.problem_currency_needs_cost":
    "Una moneda necesita un impacto en el coste.",
  "p.plan_change_requests.problem_currency_invalid":
    "La moneda es un código de tres letras, por ejemplo EUR, JPY o GBP.",
  "p.plan_change_requests.steps_heading": "Pasos para {title}",
  "p.plan_change_requests.status_now": "Ahora está en estado {status}.",
  "p.plan_change_requests.decided": "Decidido por {who} el {on}: {reason}",
  "p.plan_change_requests.no_steps":
    "No se puede dar ningún paso más con esta solicitud de cambio.",
  "p.plan_change_requests.who_note":
    "El servicio decide quién puede dar cada paso: solo quien la redactó la edita, la envía o la retira (una persona administradora también puede retirarla), y solo la persona aprobadora designada la revisa o decide.",
  "p.plan_change_requests.submit_heading": "Enviar para decisión",
  "p.plan_change_requests.f_approver":
    "Persona aprobadora (una referencia person: o worker:, no quien la redactó)",
  "p.plan_change_requests.submit": "Enviar",
  "p.plan_change_requests.submitted": "Se envió la solicitud de cambio.",
  "p.plan_change_requests.problem_approver_not_reference":
    "Indique la persona aprobadora como referencia person: o worker:.",
  "p.plan_change_requests.problem_approver_is_requester":
    "La persona aprobadora no puede ser quien pidió el cambio.",
  "p.plan_change_requests.review": "Empezar la revisión",
  "p.plan_change_requests.review_started": "La revisión ha empezado.",
  "p.plan_change_requests.decide_heading": "Registrar la decisión",
  "p.plan_change_requests.f_decision": "Decisión",
  "p.plan_change_requests.approve": "Aprobar",
  "p.plan_change_requests.reject": "Rechazar",
  "p.plan_change_requests.f_decision_reason":
    "Motivo de la decisión (al menos 10 caracteres)",
  "p.plan_change_requests.decide": "Guardar la decisión",
  "p.plan_change_requests.decision_recorded": "Se registró la decisión.",
  "p.plan_change_requests.problem_decision_reason":
    "Una decisión necesita un motivo de al menos 10 caracteres.",
  "p.plan_change_requests.withdraw": "Retirar la solicitud de cambio",
  "p.plan_change_requests.withdrawn": "Se retiró la solicitud de cambio.",
  "p.plan_change_requests.close_steps": "Cerrar los pasos",
  "p.plan_change_requests.refused_requester":
    "Solo quien redactó esta solicitud de cambio puede hacerlo.",
  "p.plan_change_requests.refused_withdraw":
    "Solo quien redactó esta solicitud de cambio, o una persona administradora, puede retirarla.",
  "p.plan_change_requests.refused_approver":
    "Solo la persona aprobadora designada puede revisar o decidir esta solicitud de cambio.",
  "p.plan_change_requests.refused_state":
    "Esta solicitud de cambio ha avanzado y ese paso ya no está disponible; se ha vuelto a leer la lista.",
  "p.plan_change_requests.refused_invalid":
    "El servicio no aceptó lo introducido: revise los campos e inténtelo de nuevo.",
  "p.change_requests.change_requests_ppm": "Solicitudes de cambio — PPM",
  "p.change_requests.change_requests": "Solicitudes de cambio",
  "p.change_requests.intro":
    "Una solicitud de cambio es una propuesta controlada para cambiar una línea base aprobada. Un aprobador designado, que no es el solicitante, decide y da un motivo. Los impactos son los que declaró el solicitante; solo se muestra una previsión cuando hay cambios hipotéticos adjuntos.",
  "p.change_requests.could_not_load":
    "No se pudieron cargar las solicitudes de cambio",
  "p.change_requests.by_status": "Solicitudes de cambio por estado",
  "p.change_requests.awaiting": "Pendientes de decisión",
  "p.change_requests.oldest_open":
    "Abierta más antigua: {title} ({days} días naturales)",
  "p.change_requests.show": "Mostrar",
  "p.change_requests.all_statuses": "Todos los estados",
  "p.change_requests.open_only": "Solo las abiertas",
  "p.change_requests.none": "No hay solicitudes de cambio registradas.",
  "p.change_requests.none_match": "Ninguna solicitud de cambio coincide.",
  "p.change_requests.loading": "Cargando…",
  "p.change_requests.count_one":
    "{n} solicitud de cambio, las más recientes primero",
  "p.change_requests.count_other":
    "{n} solicitudes de cambio, las más recientes primero",
  "p.change_requests.title_col": "Solicitud de cambio",
  "p.change_requests.kind_col": "Tipo",
  "p.change_requests.status_col": "Estado",
  "p.change_requests.impact_col": "Impacto declarado",
  "p.change_requests.age_col": "Antigüedad (días naturales)",
  "p.change_requests.approver_col": "Aprobador",
  "p.change_requests.no_approver": "Aún sin designar",
  "p.change_requests.no_impact": "Sin impacto declarado",
  "p.change_requests.impact_days": "{days} días hábiles",
  "p.change_requests.plan_link": "Abrir las solicitudes de cambio del plan",
  "p.director_pack.director_packs_ppm": "Dosieres para la dirección — PPM",
  "p.director_pack.director_packs": "Dosieres para la dirección",
  "p.director_pack.intro":
    "Un dosier para la dirección es el informe mensual a los directivos, redactado una vez, emitido por una persona designada y luego congelado: un dosier emitido nunca cambia, y una corrección es una nueva versión con una nota. Cada cifra de abajo se lee del dosier emitido; aquí no se recalcula nada.",
  "p.director_pack.could_not_load":
    "No se pudieron cargar los dosieres para la dirección",
  "p.director_pack.could_not_load_one":
    "No se pudo cargar este dosier para la dirección",
  "p.director_pack.loading": "Cargando…",
  "p.director_pack.none":
    "No tiene disponible ningún dosier para la dirección.",
  "p.director_pack.col_period": "Periodo",
  "p.director_pack.col_audience": "Destinatarios",
  "p.director_pack.col_version": "Versión",
  "p.director_pack.col_status": "Estado",
  "p.director_pack.col_issued": "Emitido",
  "p.director_pack.status_draft": "borrador",
  "p.director_pack.status_issued": "emitido",
  "p.director_pack.open_pack":
    "Abrir el dosier de {period}, {audience}, versión {version}",
  "p.director_pack.all_packs": "Todos los dosieres para la dirección",
  "p.director_pack.pack_heading":
    "Dosier para la dirección {period} — {audience}",
  "p.director_pack.version_line":
    "Versión {version} · {status} · fecha de corte {cutoff}",
  "p.director_pack.issued_on": "Emitido {when}",
  "p.director_pack.draft_note":
    "Esto es un borrador. Aún no se ha emitido, por lo que no está congelado.",
  "p.director_pack.change_note": "Nota de cambio: {note}",
  "p.director_pack.previous_pack":
    "Comparado con el dosier de {period}, versión {version}.",
  "p.director_pack.download_pdf": "Descargar en PDF",
  "p.director_pack.pdf_note":
    "El PDF tiene las mismas cifras, pero sus textos fijos están en inglés. Para conservar otro idioma, imprima esta página.",
  "p.director_pack.basis":
    "Los registros están tal como estaban cuando se redactó el dosier, con fecha igual o anterior a la de corte. Las incidencias y las solicitudes de cambio no guardan historial de estados, por lo que una cerrada después de la fecha de corte pero antes de la redacción aparece como cerrada. Una sección sin nada que mostrar lo indica; la ausencia nunca se muestra como cero ni como verde.",
  "p.director_pack.s_quality": "Calidad de los datos",
  "p.director_pack.s_changed": "Qué ha cambiado",
  "p.director_pack.s_map": "Mapa de la cartera",
  "p.director_pack.s_exceptions": "Excepciones",
  "p.director_pack.s_gates": "Decisiones de fase",
  "p.director_pack.s_funding": "Financiación",
  "p.director_pack.s_fever": "Gráfico de fiebre del colchón",
  "p.director_pack.s_evm": "Valor ganado",
  "p.director_pack.s_benefits": "Beneficios",
  "p.director_pack.s_capacity": "Capacidad",
  "p.director_pack.s_issues": "Incidencias",
  "p.director_pack.s_changes": "Solicitudes de cambio",
  "p.director_pack.plans_included": "Planes incluidos: {n}",
  "p.director_pack.plans_left_out":
    "Planes excluidos por ser privados o estar marcados: {n}",
  "p.director_pack.sections_missing": "Secciones aún no elaboradas: {list}",
  "p.director_pack.all_sections":
    "Todas las secciones del dosier están presentes.",
  "p.director_pack.nothing_here": "No hay nada que mostrar en esta sección.",
  "p.director_pack.changed_issues": "Incidencias abiertas: {now}, {change}",
  "p.director_pack.changed_changes":
    "Solicitudes de cambio pendientes de decisión: {now}, {change}",
  "p.director_pack.changed_was": "antes {before}",
  "p.director_pack.changed_unchanged": "sin cambios",
  "p.director_pack.changed_first":
    "no hay un dosier anterior con el que comparar",
  "p.director_pack.c_programme": "Programa",
  "p.director_pack.c_aim": "Objetivo estratégico",
  "p.director_pack.c_status": "Estado",
  "p.director_pack.c_stage": "Etapa",
  "p.director_pack.c_phase": "Fase",
  "p.director_pack.c_confidence": "Confianza en la entrega",
  "p.director_pack.c_trend": "Tendencia",
  "p.director_pack.self_assessed": "autoevaluado",
  "p.director_pack.stale": "desactualizado",
  "p.director_pack.no_aim": "Sin objetivo estratégico",
  "p.director_pack.c_plan": "Plan",
  "p.director_pack.c_worst": "Peor calificación",
  "p.director_pack.c_need": "Decisión necesaria",
  "p.director_pack.c_owner_role": "Responsable (rol)",
  "p.director_pack.c_decide_by": "Decidir antes del",
  "p.director_pack.c_hours_open": "Horas abierta",
  "p.director_pack.ex_window":
    "Una excepción que no se completa en las {hours} horas siguientes a su apertura se considera incumplida.",
  "p.director_pack.ex_open": "Abiertas: {n}",
  "p.director_pack.ex_breached": "Incumplidas: {n}",
  "p.director_pack.ex_closed": "Cerrados en el periodo: {n}",
  "p.director_pack.g_passed": "Superadas: {n}",
  "p.director_pack.g_failed": "No superadas: {n}",
  "p.director_pack.g_deferred": "Aplazadas: {n}",
  "p.director_pack.g_paused": "Programas en pausa: {n}",
  "p.director_pack.g_stopped": "Programas detenidos: {n}",
  "p.director_pack.c_gate": "Fase",
  "p.director_pack.c_decided": "Decidida",
  "p.director_pack.c_planned": "Prevista",
  "p.director_pack.c_reason": "Motivo",
  "p.director_pack.c_days_late": "Días de retraso",
  "p.director_pack.c_since": "Desde",
  "p.director_pack.c_allocs": "Asignaciones señaladas",
  "p.director_pack.reason_hold": "revisada como en espera",
  "p.director_pack.reason_no_review": "la fecha prevista pasó sin revisión",
  "p.director_pack.fy_line":
    "Ejercicio {label}, del {start} al {end}. El capital y los gastos corrientes se muestran por separado, y las monedas nunca se suman entre sí.",
  "p.director_pack.c_source": "Fuente de financiación",
  "p.director_pack.c_class": "Clase",
  "p.director_pack.c_budget": "Presupuesto",
  "p.director_pack.c_spend": "Gasto a la fecha",
  "p.director_pack.c_variance": "Desviación",
  "p.director_pack.no_funding_source": "sin fuente de financiación",
  "p.director_pack.forecast_line":
    "Previsión de vida completa en {currency}: real {actual}, estimación a la conclusión {eac}",
  "p.director_pack.unfunded_lines":
    "Líneas sin fuente de financiación: {n}. Líneas sin fechas: {dated}.",
  "p.director_pack.c_chain": "Cadena completada (%)",
  "p.director_pack.c_buffer": "Colchón consumido (%)",
  "p.director_pack.c_zone": "Zona",
  "p.director_pack.c_slip": "Desfase (días)",
  "p.director_pack.c_buffer_days": "Colchón (días)",
  "p.director_pack.c_previous": "Paquete anterior",
  "p.director_pack.edge_below": "recuperación: dibujado en el borde inferior",
  "p.director_pack.edge_above": "exceso: dibujado en el borde superior",
  "p.director_pack.fever_none_no_buffer":
    "sin punto en el gráfico de fiebre: no se ha declarado ningún colchón del proyecto",
  "p.director_pack.fever_none_no_baseline":
    "sin punto en el gráfico de fiebre: no hay línea base del cronograma",
  "p.director_pack.fever_none_no_chain":
    "sin punto en el gráfico de fiebre: no hay trabajo crítico con una duración que medir",
  "p.director_pack.fever_none_no_forecast":
    "sin punto en el gráfico de fiebre: ahora no se puede prever el cronograma",
  "p.director_pack.fever_lines":
    "Líneas de zona (% de colchón consumido con el 0 % y el 100 % de la cadena completada): de verde a ámbar, de {ga0} a {ga1}; de ámbar a rojo, de {ar0} a {ar1}.",
  "p.director_pack.c_bac": "Presupuesto a la conclusión",
  "p.director_pack.c_ev": "Valor ganado",
  "p.director_pack.c_ac": "Coste real",
  "p.director_pack.c_spi": "SPI",
  "p.director_pack.c_cpi": "CPI",
  "p.director_pack.below_threshold":
    "Por debajo del umbral de valor ganado: presupuesto a la conclusión {bac}",
  "p.director_pack.no_baseline": "sin línea base",
  "p.director_pack.evm_ratio_note":
    "SPI y CPI se muestran con dos decimales; en blanco significa no definido, no cero.",
  "p.director_pack.c_recurrence": "Recurrencia",
  "p.director_pack.c_classification": "Clasificación",
  "p.director_pack.c_claimed": "Declarado",
  "p.director_pack.c_validated": "Validado",
  "p.director_pack.c_unvalidated": "Declarado, no validado",
  "p.director_pack.benefits_note":
    "Un beneficio materializado pero no validado es una declaración, no un beneficio validado. Los importes recurrentes son anuales y nunca se suman a los importes puntuales.",
  "p.director_pack.c_pool": "Grupo de competencias",
  "p.director_pack.c_demand": "Demanda (FTE)",
  "p.director_pack.c_supply": "Oferta (FTE)",
  "p.director_pack.c_gap": "Brecha (FTE)",
  "p.director_pack.c_month": "Mes",
  "p.director_pack.c_reading": "Lectura",
  "p.director_pack.no_pool": "sin grupo asignado",
  "p.director_pack.capacity_note":
    "Cada celda tiene una palabra y un símbolo además de un color. Que no haya oferta registrada no equivale a capacidad cero.",
  "p.director_pack.partners_short":
    "entrega de socios por debajo del compromiso",
  "p.director_pack.open_issues": "Incidencias abiertas: {n}",
  "p.director_pack.awaiting":
    "Solicitudes de cambio pendientes de decisión: {n}",
  "p.director_pack.dash": "no registrado",
  "l.enum.pk_conf.green": "verde",
  "l.enum.pk_conf.amber_green": "ámbar/verde",
  "l.enum.pk_conf.amber": "ámbar",
  "l.enum.pk_conf.amber_red": "ámbar/rojo",
  "l.enum.pk_conf.red": "rojo",
  "l.enum.pk_conf.not_assessed": "no evaluado",
  "l.enum.pk_trend.up": "mejorando",
  "l.enum.pk_trend.down": "empeorando",
  "l.enum.pk_trend.same": "sin cambios",
  "l.enum.pk_trend.new": "evaluado por primera vez",
  "l.enum.pk_trend.not_assessed": "no evaluado",
  "l.enum.pk_trend.no_earlier_pack": "sin paquete anterior",
  "l.enum.pk_status.active": "activo",
  "l.enum.pk_status.paused": "en pausa",
  "l.enum.pk_status.stopped": "detenido",
  "l.enum.pk_zone.green": "verde",
  "l.enum.pk_zone.amber": "ámbar",
  "l.enum.pk_zone.red": "rojo",
  "l.enum.pk_heat.shortfall": "déficit",
  "l.enum.pk_heat.tight": "ajustado",
  "l.enum.pk_heat.enough": "suficiente",
  "l.enum.pk_heat.no_supply_recorded": "sin oferta registrada",
  "l.enum.pk_heat.no_demand": "sin demanda",
  "l.enum.pk_heat.nothing_recorded": "nada registrado",
  "l.enum.pk_class.capex": "capital",
  "l.enum.pk_class.opex": "gasto corriente",
  "l.enum.pk_class.all": "capital y gasto corriente combinados",
  "l.enum.pk_recur.recurring": "recurrente",
  "l.enum.pk_recur.one_off": "puntual",
  "l.enum.pk_recur.not_stated": "no indicado",
  "l.enum.pk_cls.cash_releasing": "con liberación de efectivo",
  "l.enum.pk_cls.non_cash_releasing": "sin liberación de efectivo",
  "l.enum.pk_cls.societal": "social",
  "l.enum.pk_cls.not_stated": "no indicado",
  "l.enum.pk_gate.g0_concept": "G0 concepto",
  "l.enum.pk_gate.g1_feasibility": "G1 viabilidad",
  "l.enum.pk_gate.g2_definition": "G2 definición",
  "l.enum.pk_gate.g3_delivery": "G3 entrega",
  "l.enum.pk_gate.g4_launch": "G4 lanzamiento",
  "l.enum.pk_gate.g5_benefits": "G5 beneficios",
  "p.director_pack.c_currency": "Moneda",
  "p.critical_chain.chain_ppm": "Cadena crítica y colchones — PPM",
  "p.critical_chain.buffers_ppm": "Gestión de colchones — PPM",
  "p.critical_chain.pipeline_ppm": "Tambor y canalización — PPM",
  "p.critical_chain.loading": "Cargando…",
  "p.critical_chain.yes": "sí",
  "p.critical_chain.no": "no",
  "p.critical_chain.zone_green": "verde",
  "p.critical_chain.zone_amber": "ámbar",
  "p.critical_chain.zone_red": "rojo",
  "p.critical_chain.zone_none": "sin lectura",
  "p.critical_chain.outcome_as_planned": "mantiene sus fechas",
  "p.critical_chain.outcome_staggered": "empieza más tarde",
  "p.critical_chain.outcome_exceeds_drum": "pide más de lo que tiene el tambor",
  "p.critical_chain.outcome_no_slot": "no cabe ningún inicio",
  "p.critical_chain.advice":
    "Esto es una recomendación. No se ha cambiado nada, y una persona decide qué hacer.",
  "p.critical_chain.link_plan": "Plan",
  "p.critical_chain.link_buffers": "Gestión de colchones",
  "p.critical_chain.link_chain": "Cadena crítica",
  "p.critical_chain.link_critical_path": "Ruta crítica",
  "p.critical_chain.nav_pipeline": "Tambor y canalización",
  "p.critical_chain.chain_h1": "Cadena crítica y colchones",
  "p.critical_chain.chain_lede":
    "La cadena crítica es la secuencia de tareas más larga una vez que se respeta el tiempo de las personas: sigue las dependencias y las esperas por una persona ocupada en otra tarea. Se muestra junto a la ruta crítica, que sigue solo las dependencias; ambas nunca se mezclan.",
  "p.critical_chain.chain_h2": "La cadena crítica",
  "p.critical_chain.no_chain": "No hay cadena crítica: {reason}",
  "p.critical_chain.reason_no_assignments":
    "nadie está asignado a una tarea sin terminar, así que no hay esperas por personas que seguir",
  "p.critical_chain.reason_no_scheduled_tasks":
    "ninguna tarea tiene cronograma",
  "p.critical_chain.chain_work": "Días hábiles de trabajo en la cadena: {days}",
  "p.critical_chain.chain_tasks_count": "Tareas en la cadena: {tasks}",
  "p.critical_chain.chain_finish":
    "Una vez respetado el tiempo de las personas, el plan termina el {finish}.",
  "p.critical_chain.chain_delay_none":
    "No es posterior al fin de la ruta crítica.",
  "p.critical_chain.chain_delay":
    "Días hábiles de retraso respecto al fin de la ruta crítica: {days}",
  "p.critical_chain.crosses_yes":
    "La cadena pasa de una ruta a otra: se comparte una persona.",
  "p.critical_chain.crosses_no":
    "La cadena se mantiene en una sola ruta: no se comparte ninguna persona.",
  "p.critical_chain.tied":
    "Hay otra cadena igual de larga; se muestra esta porque aparece primero en el plan.",
  "p.critical_chain.timeline_label": "La cadena en una línea de tiempo",
  "p.critical_chain.timeline_caption":
    "Cada barra continua es una tarea de la cadena. Una barra discontinua es el colchón del proyecto. La tabla siguiente ofrece las mismas cifras.",
  "p.critical_chain.col_task": "Tarea",
  "p.critical_chain.col_days": "Días hábiles",
  "p.critical_chain.col_start": "Inicio",
  "p.critical_chain.col_finish": "Fin",
  "p.critical_chain.col_held_by": "Retenida por",
  "p.critical_chain.col_on_path": "En la ruta crítica",
  "p.critical_chain.held_first": "primera de la cadena",
  "p.critical_chain.held_dep": "una dependencia",
  "p.critical_chain.held_person": "una persona: {person}",
  "p.critical_chain.held_person_hidden": "una persona (oculta)",
  "p.critical_chain.cp_h2": "La ruta crítica junto a ella",
  "p.critical_chain.cp_finish": "La ruta crítica termina el {finish}.",
  "p.critical_chain.cp_none": "No hay ruta crítica que mostrar.",
  "p.critical_chain.buffers_h2": "Colchones",
  "p.critical_chain.buffers_lede":
    "Las tareas se programan con sus estimaciones agresivas; el margen de seguridad retirado de cada tarea se agrupa en un colchón. Los tamaños se expresan en días hábiles. La regla: {rule}.",
  "p.critical_chain.rule_cut_and_paste":
    "la mitad del margen retirado (cortar y pegar)",
  "p.critical_chain.rule_root_sum_squares":
    "la raíz cuadrada de la suma de los cuadrados del margen retirado",
  "p.critical_chain.no_buffers": "No hay colchones que mostrar: {reason}",
  "p.critical_chain.reason_estimates_missing":
    "algunas tareas de la cadena aún no tienen estimación agresiva y segura. Regístrelas mediante el servicio de estimaciones; todavía no hay pantalla para ello.",
  "p.critical_chain.missing_h3": "Tareas de la cadena sin ambas estimaciones",
  "p.critical_chain.project_buffer":
    "Colchón del proyecto: {days} días hábiles, que protege una cadena que termina el {finish}. Con el colchón, la fecha es {with}.",
  "p.critical_chain.project_safety":
    "Margen retirado de las tareas de la cadena: {days} días hábiles.",
  "p.critical_chain.project_buffer_bar": "Colchón del proyecto",
  "p.critical_chain.feeding_h3": "Colchones de alimentación",
  "p.critical_chain.feeding_none":
    "Ninguna otra cadena de tareas se une a la cadena crítica.",
  "p.critical_chain.col_feeding_tasks": "Tareas de alimentación",
  "p.critical_chain.col_joins": "Se une a",
  "p.critical_chain.col_buffer": "Colchón (días hábiles)",
  "p.critical_chain.col_room": "Holgura antes de la unión (días hábiles)",
  "p.critical_chain.col_push": "Retrasaría la cadena",
  "p.critical_chain.push_yes": "sí, {days} días hábiles",
  "p.critical_chain.unsized": "sin dimensionar: falta una estimación",
  "p.critical_chain.resource_h3":
    "Colchones de recursos (alertas de disponibilidad)",
  "p.critical_chain.resource_lede":
    "Un colchón de recursos no es tiempo. Es un recordatorio de que una persona debe estar lista antes de que empiece su tarea en la cadena, con {lead} días hábiles de antelación.",
  "p.critical_chain.col_person": "Persona",
  "p.critical_chain.col_alert_on": "Lista para",
  "p.critical_chain.col_task_starts": "Inicio de la tarea",
  "p.critical_chain.hidden": "(oculto)",
  "p.critical_chain.likely_h3":
    "Programadas con su duración probable (sin estimación agresiva)",
  "p.critical_chain.could_not_load_chain":
    "No se pudo cargar la cadena crítica",
  "p.critical_chain.buffers_h1": "Gestión de colchones",
  "p.critical_chain.buffers_lede2":
    "Cada colchón se lee frente al cronograma vigente: qué parte de la cadena está hecha y qué parte del colchón ha consumido el desfase previsto. Un colchón en rojo necesita una acción de recuperación, que una persona acuerda y registra.",
  "p.critical_chain.no_plan":
    "No se ha congelado ningún plan de colchones para este plan. Congele uno cuando se acuerden la cadena y los colchones; las lecturas posteriores se miden frente a él.",
  "p.critical_chain.freeze_confirm":
    "He leído la cadena y los colchones y quiero congelarlos como plan",
  "p.critical_chain.freeze_button": "Congelar el plan de colchones",
  "p.critical_chain.frozen_on":
    "Medido frente al plan de colchones congelado el {date}.",
  "p.critical_chain.alerts_h2": "Alertas",
  "p.critical_chain.alerts_none": "Ningún colchón está en rojo.",
  "p.critical_chain.alert_project":
    "El colchón del proyecto está en rojo: {consumed} % consumido. Cadena: {chain}.",
  "p.critical_chain.alert_feeding":
    "Un colchón de alimentación está en rojo: {consumed} % consumido. Cadena: {chain}, que se une a {joins}.",
  "p.critical_chain.needs_action":
    "No hay ninguna acción de recuperación abierta registrada para: {buffers}.",
  "p.critical_chain.charts_h2": "Gráficos de fiebre",
  "p.critical_chain.project_label": "Colchón del proyecto",
  "p.critical_chain.feeding_label": "Colchón de alimentación: {chain}",
  "p.critical_chain.axis_x": "Cadena completada (%)",
  "p.critical_chain.axis_y": "Colchón consumido (%)",
  "p.critical_chain.chart_caption":
    "{buffer}: {zone}; {consumed} % del colchón consumido; {done} % de la cadena hecho.",
  "p.critical_chain.beyond_edge":
    "La cifra real queda fuera del gráfico, así que el punto se dibuja en el borde.",
  "p.critical_chain.table_h2": "Las mismas cifras en una tabla",
  "p.critical_chain.col_size": "Colchón (días hábiles)",
  "p.critical_chain.col_done": "Cadena hecha (%)",
  "p.critical_chain.col_used": "Colchón consumido (%)",
  "p.critical_chain.col_slip": "Desfase previsto (días hábiles)",
  "p.critical_chain.col_zone": "Zona",
  "p.critical_chain.no_point": "Sin lectura: {reason}",
  "p.critical_chain.actions_h2": "Acciones de recuperación",
  "p.critical_chain.actions_none":
    "No se ha registrado ninguna acción de recuperación.",
  "p.critical_chain.col_buffer_name": "Colchón",
  "p.critical_chain.col_action": "Acción",
  "p.critical_chain.col_owner": "Responsable",
  "p.critical_chain.col_due": "Vencimiento",
  "p.critical_chain.col_status": "Estado",
  "p.critical_chain.status_open": "abierta",
  "p.critical_chain.status_done": "hecha",
  "p.critical_chain.status_dropped": "descartada",
  "p.critical_chain.mark_done": "Marcar como hecha",
  "p.critical_chain.drop_action": "Descartar",
  "p.critical_chain.add_action_h3": "Registrar una acción de recuperación",
  "p.critical_chain.f_buffer": "Colchón",
  "p.critical_chain.f_action": "Qué se hará",
  "p.critical_chain.f_owner": "Responsable (una referencia de persona)",
  "p.critical_chain.f_due": "Vencimiento (opcional)",
  "p.critical_chain.add_button": "Registrar la acción",
  "p.critical_chain.snap_h2": "Historial",
  "p.critical_chain.snap_button": "Registrar una instantánea ahora",
  "p.critical_chain.snap_none": "No se ha registrado ninguna instantánea.",
  "p.critical_chain.col_snap_date": "Registrada el",
  "p.critical_chain.could_not_load_buffers":
    "No se pudo cargar la gestión de colchones",
  "p.critical_chain.could_not_save": "No se pudo guardar",
  "p.critical_chain.pipeline_h1": "Tambor y canalización",
  "p.critical_chain.pipeline_lede":
    "El tambor es el grupo de competencias del que más depende la cartera. Los proyectos que lo necesitan al mismo tiempo se escalonan: el servicio propone inicios posteriores, con un colchón de capacidad entre proyectos, y explica cada uno. Nunca mueve una fecha.",
  "p.critical_chain.drum_h2": "El tambor",
  "p.critical_chain.drum_named":
    "El tambor es {name}: designado por la oficina de cartera.",
  "p.critical_chain.drum_overloaded":
    "El tambor es {name}: no se ha designado ningún grupo, y este tiene el mayor déficit.",
  "p.critical_chain.no_drum": "No hay tambor: {why}",
  "p.critical_chain.set_h3": "Designar el tambor",
  "p.critical_chain.pick_pool": "Grupo de competencias",
  "p.critical_chain.set_button": "Designarlo como tambor",
  "p.critical_chain.clear_button": "Quitar la designación",
  "p.critical_chain.capacity":
    "Capacidad del tambor: {fte} FTE. Colchón de capacidad entre proyectos: {days} días.",
  "p.critical_chain.no_capacity":
    "El tambor no tiene oferta registrada en la ventana, así que no hay nada con respecto a lo que escalonar.",
  "p.critical_chain.projects_h2": "Inicios propuestos",
  "p.critical_chain.projects_none":
    "Ningún proyecto necesita el tambor en esta ventana.",
  "p.critical_chain.pipeline_timeline": "Uso del tambor actual y propuesto",
  "p.critical_chain.pipeline_timeline_caption":
    "Para cada proyecto, la barra superior indica cuándo usa el tambor ahora y la inferior cuándo lo usaría según la propuesta. La tabla siguiente ofrece las mismas fechas.",
  "p.critical_chain.bar_now": "ahora",
  "p.critical_chain.bar_proposed": "propuesto",
  "p.critical_chain.col_project": "Proyecto",
  "p.critical_chain.col_needs": "Necesita el tambor",
  "p.critical_chain.col_peak": "Pico (FTE)",
  "p.critical_chain.col_proposal": "Propuesta",
  "p.critical_chain.col_proposed": "Primer uso propuesto",
  "p.critical_chain.col_delay": "Retraso (días)",
  "p.critical_chain.col_why": "Motivo",
  "p.critical_chain.could_not_load_pipeline":
    "No se pudo cargar la canalización",
  "p.start_queue.nav": "Cola de inicio",
  "p.start_queue.title": "Cola de inicio — PPM",
  "p.start_queue.h1": "Cola de inicio",
  "p.start_queue.lede":
    "Programas que solicitaron iniciar la entrega y fueron rechazados, en el orden en que lo solicitaron. Un programa se rechaza cuando el límite de trabajo en curso está completo o cuando el grupo de restricción carece de oferta en los próximos meses. El orden nunca cambia, y un inicio aún puede seguir adelante con una anulación registrada y un motivo.",
  "p.start_queue.loading": "Cargando la cola…",
  "p.start_queue.none": "Ningún programa está esperando para iniciar.",
  "p.start_queue.col_pos": "Puesto",
  "p.start_queue.col_prog": "Programa",
  "p.start_queue.col_asked": "Solicitado el",
  "p.start_queue.col_why": "Motivo del rechazo",
  "p.start_queue.col_act": "Acción",
  "p.start_queue.limit":
    "Los {limit} puestos están ocupados: {in_delivery} programas están en entrega.",
  "p.start_queue.pool":
    "El grupo de restricción {pool} carece de oferta en: {months}.",
  "p.start_queue.withdraw": "Retirar {name} de la cola",
  "p.start_queue.withdraw_short": "Retirar",
  "p.start_queue.could_not_load": "No se pudo cargar la cola de inicio",
  "p.start_queue.could_not_withdraw": "No se pudo retirar de la cola",
  "p.table.search": "Filtrar filas",
  "p.table.export": "Exportar",
  "p.table.find": "Buscar",
  "p.table.moved": "{header} movida a la posición {position} de {count}",
  "p.table.copy": "Copiar",
  "p.table.copied": "{rows} filas copiadas al portapapeles",
  "p.table.copy_failed": "No se pudo copiar al portapapeles",
  "p.table.matches": "{matches} de {total} mostradas",
  "p.table.search_in": "Buscar en",
  "p.table.all_columns": "Todas las columnas",
  "p.table.pages": "Páginas",
  "p.table.export_csv": "CSV",
  "p.table.export_tsv": "TSV",
  "p.table.export_excel": "Excel",
  "p.table.exported": "{rows} filas exportadas a {file}",
  "p.table.columns": "Columnas",
  "p.table.show_column": "Mostrar {header}",
  "p.table.resize": "Cambiar el tamaño de la columna {header}",
  "p.table.select_all": "Seleccionar todas las filas",
  "p.table.select_col": "Seleccionar",
  "p.table.select_row": "Seleccionar {row}",
  "p.table.prev": "Página anterior",
  "p.table.next": "Página siguiente",
  "p.table.page_status": "Página {page} de {pages} ({rows} filas)",
  "p.table.page_announce": "Página {page} de {pages}",
  "p.table.sorted": "{header} ordenada en orden {direction}",
  "p.table.asc": "ascendente",
  "p.table.desc": "descendente",
  "p.table.none": "sin ordenar",
  "p.table.filtered": "{matches} de {total} filas coinciden",
  "p.table.selected": "{n} filas seleccionadas",
  "p.table.empty": "No hay nada que mostrar.",
  "p.demo.tour_title": "Recorrido de demostración — PPM",
  "p.demo.tour_h1": "Recorrido de demostración",
  "p.demo.tour_lede":
    "Ha iniciado sesión como Persona de Ejemplo. Todo aquí es ficticio: la Autoridad de Transporte de Northgate, su personal, su dinero y sus fechas se inventaron para esta demostración. Elija cualquier función de abajo y úsela: puede cambiar cosas, y nada es real.",
  "p.demo.tour_story_h2": "La historia",
  "p.demo.tour_story":
    "La Autoridad de Transporte de Northgate está modernizando su red ferroviaria: nueva señalización, estaciones sin escalones y un depósito digital. Diez planes, ocho personas, cuatro organizaciones y algunas cosas que salen mal a propósito, para que las pantallas tengan algo que mostrar.",
  "p.demo.tour_open": "Abrir",
  "p.demo.tour_no_plan":
    "No se encontró el plan de la Línea Norte, así que los enlaces a sus páginas están ocultos. Vuelva a ejecutar la carga de datos.",
  "p.demo.tour_toc": "Funciones",
  "p.demo.g_overview": "La cartera de un vistazo",
  "p.demo.g_delivery":
    "Entrega de un proyecto (enclavamiento de la Línea Norte)",
  "p.demo.g_risk": "Riesgos, incidencias y cambios",
  "p.demo.g_money": "Dinero",
  "p.demo.g_people": "Personas y capacidad",
  "p.demo.g_strategy": "Estrategia y entrada de trabajo",
  "p.demo.g_escalation": "Gobernanza y escalado",
  "p.demo.g_chain": "Cadena crítica",
  "p.demo.g_tools": "Herramientas",
  "p.demo.f_kanban_t": "Mi tablero Kanban",
  "p.demo.f_kanban_d":
    "Sus propias tareas abiertas, en columnas por estado, de todos los proyectos. Mueva una con el ratón o con su menú; el estado de la tarea cambia para todos.",
  "p.demo.f_me_t": "Mi trabajo",
  "p.demo.f_me_d":
    "Las mismas tareas por fecha de vencimiento en su propia zona horaria: vencidas, hoy, esta semana, más adelante.",
  "p.demo.f_plans_t": "Planes",
  "p.demo.f_plans_d":
    "Diez planes ficticios: una cartera, tres programas y seis proyectos para la Autoridad de Transporte de Northgate. Abra cualquiera.",
  "p.demo.f_dashboard_t": "Panel de cartera",
  "p.demo.f_dashboard_d":
    "Estado por tipo y etapa, con recuentos en rojo, ámbar y verde.",
  "p.demo.f_gantt_t": "Gantt",
  "p.demo.f_gantt_d":
    "El cronograma de cada proyecto en una sola línea de tiempo.",
  "p.demo.f_milestones_t": "Hitos",
  "p.demo.f_milestones_d":
    "Diseño aprobado, equipos entregados y puesta en servicio de cada proyecto.",
  "p.demo.f_calendar_t": "Calendario",
  "p.demo.f_calendar_d": "Hitos y fechas de vencimiento por mes.",
  "p.demo.f_board_t": "Tablero de tareas",
  "p.demo.f_board_d":
    "Ocho tareas, desde el estudio hasta la entrega, en columnas por estado. Una está bloqueada por el aseguramiento de la seguridad.",
  "p.demo.f_critical_path_t": "Ruta crítica",
  "p.demo.f_critical_path_d":
    "La cadena de tareas dependientes que fija la fecha de fin.",
  "p.demo.f_earned_value_t": "Valor ganado",
  "p.demo.f_earned_value_d":
    "Valor planificado, valor ganado y coste, con los índices de cronograma y de coste.",
  "p.demo.f_schedule_health_t": "Salud del cronograma",
  "p.demo.f_schedule_health_d":
    "Comprobaciones de la lógica del plan: vínculos que faltan, tareas largas, restricciones.",
  "p.demo.f_wbs_t": "Desglose del trabajo",
  "p.demo.f_wbs_d": "El proyecto desglosado en paquetes de trabajo.",
  "p.demo.f_risk_t": "Registro de riesgos",
  "p.demo.f_risk_d":
    "Tres riesgos en cada proyecto, puntuados por probabilidad e impacto.",
  "p.demo.f_issues_t": "Incidencias",
  "p.demo.f_issues_d":
    "Un trazado de cable bloqueado, un banco de pruebas reservado dos veces y un permiso retrasado.",
  "p.demo.f_change_requests_t": "Solicitudes de cambio",
  "p.demo.f_change_requests_d":
    "Una solicitud para mover la puesta en servicio dos semanas, con su impacto en cronograma y coste, a la espera del director del programa.",
  "p.demo.f_joint_risk_t": "Riesgo de coste y cronograma conjunto",
  "p.demo.f_joint_risk_d":
    "Lo que los riesgos hacen a la fecha de fin y al coste, en conjunto.",
  "p.demo.f_financials_t": "Finanzas",
  "p.demo.f_financials_d":
    "Presupuesto, gasto real y desviación en toda la cartera.",
  "p.demo.f_cost_risk_t": "Riesgo de coste",
  "p.demo.f_cost_risk_d":
    "La dispersión del coste probable, con el P50 y el P80.",
  "p.demo.f_ledger_t": "Libro de financiación",
  "p.demo.f_ledger_d":
    "Una subvención y una reserva, vinculadas a las líneas de presupuesto que se nutren de ellas. El dinero aún no planificado en una línea se muestra como no vinculado.",
  "p.demo.f_costs_t": "Costes de un proyecto",
  "p.demo.f_costs_d":
    "Costes de mano de obra según las tablas de tarifas, y otros costes por tarea.",
  "p.demo.f_resources_t": "Recursos",
  "p.demo.f_resources_d":
    "Ocho personas ficticias, sus funciones y en qué están asignadas.",
  "p.demo.f_capacity_t": "Mapa de calor de capacidad",
  "p.demo.f_capacity_d":
    "Oferta y demanda por grupo de competencias y mes. Los meses en rojo muestran dónde faltan ingenieros de señalización.",
  "p.demo.f_key_persons_t": "Personas clave",
  "p.demo.f_key_persons_d":
    "Puntos únicos de fallo: un ingeniero al que nadie puede sustituir todavía, y una función con un sucesor preparado.",
  "p.demo.f_objectives_t": "Objetivos",
  "p.demo.f_objectives_d":
    "Tres objetivos, cada uno vinculado a los planes que lo sirven.",
  "p.demo.f_okrs_t": "Objetivos y resultados clave",
  "p.demo.f_okrs_d": "Resultados clave con seguimientos y nivel de confianza.",
  "p.demo.f_kpis_t": "Indicadores clave",
  "p.demo.f_kpis_d":
    "Minutos de retraso, satisfacción de los pasajeros y gasto frente al plan, con una lectura cada mes.",
  "p.demo.f_proposals_t": "Entrada de trabajo",
  "p.demo.f_proposals_d":
    "Cinco propuestas en distintas etapas, desde borrador hasta aprobada y rechazada.",
  "p.demo.f_ideas_t": "Ideas",
  "p.demo.f_ideas_d": "Cuatro ideas con votos. Convierta una en propuesta.",
  "p.demo.f_scenarios_t": "Escenarios",
  "p.demo.f_scenarios_d":
    "Qué cabe bajo un tope presupuestario si el trabajo de señalización va primero.",
  "p.demo.f_director_packs_t": "Paquetes de dirección",
  "p.demo.f_director_packs_d":
    "Paquetes ejecutivos y del consejo de septiembre, elaborados a partir de los datos en vivo. Abra uno para ver todas las secciones.",
  "p.demo.f_organizations_t": "Organizaciones asociadas",
  "p.demo.f_organizations_d":
    "La autoridad, su empresa de ejecución, un socio y un proveedor.",
  "p.demo.f_milestone_signoffs_t": "Aprobación de hitos",
  "p.demo.f_milestone_signoffs_d":
    "La puesta en servicio requiere la firma del socio y del proveedor. El proveedor ha firmado; el socio no, así que la puesta en servicio está entregada pero no cumplida.",
  "p.demo.f_cutover_t": "Decisión de transición (sí/no)",
  "p.demo.f_cutover_d":
    "Tres criterios de parada, uno cumplido, una cofirma recibida. Se rechazaría el «sí», y la pantalla explica por qué.",
  "p.demo.f_deployment_moves_t": "Movimientos de despliegue",
  "p.demo.f_deployment_moves_d":
    "Una solicitud para mover dos semanas el inicio de la instalación, con el efecto en cadena sobre otras tareas y su coste de mano de obra, a la espera de aprobación.",
  "p.demo.f_start_queue_t": "Cola de inicio",
  "p.demo.f_start_queue_d":
    "Dos programas están en entrega y el límite es dos, así que el tercero espera en la cola, por orden.",
  "p.demo.f_critical_chain_t": "Cadena crítica",
  "p.demo.f_critical_chain_d":
    "La cadena a través de las tareas dependientes y de las personas que comparten, con estimaciones agresivas y seguras.",
  "p.demo.f_buffers_t": "Gestión de colchones",
  "p.demo.f_buffers_d":
    "El plan de colchones congelado, los gráficos de fiebre y una acción de recuperación.",
  "p.demo.f_pipeline_t": "Tambor y canalización",
  "p.demo.f_pipeline_d":
    "El grupo de competencias del que más depende la cartera, e inicios escalonados propuestos.",
  "p.demo.f_search_t": "Buscar",
  "p.demo.f_search_d":
    "Escriba en la lupa del encabezado. Este enlace busca «signal».",
  "p.demo.f_email_deliveries_t": "Envíos de correo",
  "p.demo.f_email_deliveries_d":
    "Cada correo que el servicio intentó enviar. Desde la demostración no se envía nada, así que esta lista está vacía.",
  "p.demo.f_display_t": "Ajustes de visualización",
  "p.demo.f_display_d": "Tamaño del texto, temas, vista simple e idioma.",
  "p.kanban.nav": "Mi Kanban",
  "p.kanban.title": "Mi tablero Kanban — PPM",
  "p.kanban.h1": "Mi tablero Kanban",
  "p.kanban.lede":
    "Sus tareas abiertas de todos los planes, en columnas por estado. Mueva una tarjeta con el ratón, o pulse Intro sobre una tarjeta para elegir una columna en un menú. Un movimiento cambia el estado de la tarea para todos.",
  "p.kanban.board_label": "Sus tareas por estado",
  "p.kanban.loading": "Cargando su tablero…",
  "p.kanban.none": "No tiene tareas abiertas.",
  "p.kanban.todo": "Por hacer",
  "p.kanban.in_progress": "En curso",
  "p.kanban.in_review": "En revisión",
  "p.kanban.blocked": "Bloqueada",
  "p.kanban.done": "Hecha",
  "p.kanban.card_count": "{n} tarjetas",
  "p.kanban.move_button": "Mover {title}",
  "p.kanban.move_menu": "Mover a la columna",
  "p.kanban.moved": "{title} movida a {column}",
  "p.kanban.due": "vence el {date}",
  "p.kanban.points": "{n} puntos",
  "p.kanban.sign_in": "Inicie sesión para ver su tablero.",
  "p.kanban.could_not_load": "No se pudo cargar su tablero",
  "p.kanban.could_not_move":
    "No se pudo mover la tarea; el tablero sigue como estaba.",
  "p.kanban.hidden":
    "{n} tareas no se muestran porque usted no puede leer sus planes.",
  "p.dashboard.my_home": "Mi página de inicio",
  "p.demo.banner":
    "Esta es una demostración con datos ficticios. Nada de lo que aparece aquí es real.",
  "p.demo.banner_tour": "Haga el recorrido",
  "p.demo.signin_h2": "Demostración",
  "p.demo.signin_lede":
    "Este sitio es una demostración. Inicie sesión con example@example.com para ver todas las funciones con datos ficticios. No se envía ningún correo.",
  "p.demo.signin_button": "Iniciar sesión en la demostración",
  "p.demo.signin_wrong": "La demostración solo acepta example@example.com.",
  "p.search.title": "Buscar — PPM",
  "p.search.h1": "Buscar planes",
  "p.search.lede":
    "Planes cuyo nombre, identificadores, palabras clave o etiquetas coinciden con su búsqueda. No se muestran los planes que usted no puede leer.",
  "p.search.none_asked":
    "Escriba algo en el cuadro de búsqueda del encabezado.",
  "p.search.searching": "Buscando…",
  "p.search.results": "{n} planes coinciden con «{q}».",
  "p.search.no_results": "Ningún plan coincide con «{q}».",
  "p.search.could_not_search": "No se pudo buscar",
  "p.layout.link_picker": "Páginas",
  "p.layout.search_picker": "Buscar planes",
  "p.layout.search_input": "Términos de búsqueda",
  "p.layout.search_submit": "Buscar",
  "p.layout.lp_home": "Inicio",
  "p.layout.lp_plans": "Planes",
  "p.layout.lp_dashboard": "Panel",
  "p.layout.lp_milestones": "Hitos",
  "p.layout.lp_proposals": "Propuestas",
  "p.cutover.link": "Decisión de transición (sí/no)",
  "p.cutover.title": "Decisión de transición (sí/no) — PPM",
  "p.cutover.h1": "Decisión de transición (sí/no)",
  "p.cutover.lede":
    "La decisión de realizar la transición de un sistema crítico para la seguridad. Los criterios de parada se redactan y publican primero, y después se congelan: cambiarlos crea una nueva versión. El «sí» requiere que se cumplan todos los criterios y todas las cofirmas necesarias. Un «no», o un aplazamiento por seguridad, es una decisión tomada según los criterios; no es un retraso.",
  "p.cutover.loading": "Cargando las decisiones…",
  "p.cutover.none": "No se ha redactado ninguna decisión de transición.",
  "p.cutover.new_h2": "Redactar una decisión de transición",
  "p.cutover.new_title": "Título",
  "p.cutover.new_criteria":
    "Criterios de parada, uno por línea (de 5 a 500 caracteres cada uno)",
  "p.cutover.draft": "Redactar la decisión",
  "p.cutover.version": "versión {n}",
  "p.cutover.status_draft": "borrador",
  "p.cutover.status_published": "publicada, criterios congelados",
  "p.cutover.status_open": "abierta para decisión",
  "p.cutover.status_decided": "decidida",
  "p.cutover.status_superseded": "sustituida",
  "p.cutover.outcome_go": "sí",
  "p.cutover.outcome_no_go": "no",
  "p.cutover.outcome_delay_for_safety": "aplazamiento por seguridad",
  "p.cutover.decided_line": "Decidida el {on}: {outcome}. {why}",
  "p.cutover.criteria_h3": "Criterios de parada",
  "p.cutover.col_n": "N.º",
  "p.cutover.col_criterion": "Criterio",
  "p.cutover.col_met": "Cumplido",
  "p.cutover.col_evidence": "Evidencia",
  "p.cutover.col_mark": "Marcar",
  "p.cutover.met": "cumplido",
  "p.cutover.not_met": "no cumplido",
  "p.cutover.evidence": "Evidencia",
  "p.cutover.mark_met": "Marcar como cumplido: {name}",
  "p.cutover.mark_unmet": "Marcar como no cumplido: {name}",
  "p.cutover.mark_met_short": "Cumplido",
  "p.cutover.mark_unmet_short": "No cumplido",
  "p.cutover.cosign_h3": "Cofirmas",
  "p.cutover.cosign_signed": "Firmado: {roles}.",
  "p.cutover.cosign_missing": "Aún se necesitan: {roles}.",
  "p.cutover.cosign_none": "ninguna",
  "p.cutover.cosign_role":
    "Función con la que firmar (su token debe incluirla)",
  "p.cutover.cosign": "Cofirmar",
  "p.cutover.refused_h3": "El «sí» se rechazaría porque",
  "p.cutover.publish": "Publicar y congelar los criterios",
  "p.cutover.open": "Abrir la decisión",
  "p.cutover.new_version":
    "Iniciar una nueva versión para cambiar los criterios",
  "p.cutover.decide_h3": "Decidir",
  "p.cutover.outcome": "Resultado",
  "p.cutover.reason": "Motivo",
  "p.cutover.decide": "Registrar la decisión",
  "p.cutover.could_not_load": "No se pudieron cargar las decisiones",
  "p.cutover.could_not_save": "No se pudo guardar",
  "p.cutover.cancel_edit": "Dejar de editar",
  "p.cutover.criteria_frozen":
    "Esta decisión está publicada, así que sus criterios están congelados: cree una versión nueva para cambiarlos.",
  "p.cutover.criteria_rules":
    "Un criterio de parada por línea: de 1 a 30, cada uno de 5 a 500 caracteres. Se pueden cambiar hasta que se publique la decisión.",
  "p.cutover.criteria_saved": "Se guardaron los criterios de parada.",
  "p.cutover.criteria_unchanged": "No cambió nada, así que no se envió nada.",
  "p.cutover.edit_criteria_label": "Criterios de parada, uno por línea",
  "p.cutover.edit_criteria": "Cambiar los criterios de parada",
  "p.cutover.problem_count":
    "Una decisión necesita de 1 a 30 criterios de parada.",
  "p.cutover.problem_length":
    "El criterio de parada {line} debe tener de 5 a 500 caracteres.",
  "p.cutover.save_criteria": "Guardar los criterios de parada",
  "p.moves.link": "Movimientos de despliegue",
  "p.moves.title": "Movimientos de despliegue — PPM",
  "p.moves.h1": "Movimientos de despliegue",
  "p.moves.lede":
    "Una solicitud para mover el inicio de una tarea. Nada cambia hasta que el aprobador la aprueba. El efecto en cadena sobre otras tareas, y su coste de mano de obra, se calculó al solicitar el movimiento, ejecutando el cronograma con la fecha movida.",
  "p.moves.loading": "Cargando los movimientos…",
  "p.moves.none": "No se ha solicitado ningún movimiento.",
  "p.moves.request_h2": "Solicitar un movimiento",
  "p.moves.task": "Tarea",
  "p.moves.new_start": "Nuevo inicio",
  "p.moves.reason": "Motivo",
  "p.moves.approver": "Aprobador (person:<id> o role:<name>, opcional)",
  "p.moves.request": "Solicitar el movimiento",
  "p.moves.col_task": "Tarea",
  "p.moves.col_dates": "Inicio",
  "p.moves.col_reason": "Motivo",
  "p.moves.col_by": "Solicitado por",
  "p.moves.col_approver": "Aprobador",
  "p.moves.col_status": "Estado",
  "p.moves.col_knock": "Efecto en cadena",
  "p.moves.col_act": "Decidir",
  "p.moves.status_requested": "a la espera del aprobador",
  "p.moves.status_approved": "aprobado",
  "p.moves.status_refused": "rechazado",
  "p.moves.knock": "se mueven {n} tareas más; coste de mano de obra {cost}",
  "p.moves.no_cost": "sin cambio en el coste de mano de obra",
  "p.moves.decision_reason": "Motivo de la decisión",
  "p.moves.approve": "Aprobar",
  "p.moves.refuse": "Rechazar",
  "p.moves.approve_for": "Aprobar el movimiento de {name}",
  "p.moves.refuse_for": "Rechazar el movimiento de {name}",
  "p.moves.decided": "{by} el {on}: {why}",
  "p.moves.by_requester_h2": "Coste del efecto en cadena por solicitante",
  "p.moves.requester_row": "{who}: {n} solicitudes; mano de obra {cost}",
  "p.moves.could_not_load": "No se pudieron cargar los movimientos",
  "p.moves.could_not_save": "No se pudo guardar",
  "p.ledger.nav": "Libro de financiación",
  "p.ledger.title": "Libro de financiación — PPM",
  "p.ledger.h1": "Libro de financiación",
  "p.ledger.lede":
    "Cada asignación de financiación, vinculada a las líneas de presupuesto que se nutren de ella, sus fases, los hitos cumplidos y los beneficios validados. El dinero asignado pero no planificado en ninguna línea que usted pueda leer queda sin vincular, y se muestra con su importe.",
  "p.ledger.scope":
    "La creación de asignaciones, la correspondencia de códigos de coste y la conciliación de una exportación financiera se realizan mediante la API; esta pantalla lee el libro.",
  "p.ledger.loading": "Cargando el libro…",
  "p.ledger.none": "No se ha registrado ninguna asignación de financiación.",
  "p.ledger.col_source": "Fuente de financiación",
  "p.ledger.col_ref": "Referencia",
  "p.ledger.col_on": "Asignada el",
  "p.ledger.col_amount": "Asignado",
  "p.ledger.col_planned": "Planificado en líneas",
  "p.ledger.col_untraced": "Sin vincular",
  "p.ledger.col_lines": "Líneas",
  "p.ledger.col_act": "Rastrear",
  "p.ledger.trace": "Rastrear {name}",
  "p.ledger.trace_short": "Rastrear",
  "p.ledger.trace_h2": "Qué se adquirió con {name}",
  "p.ledger.money":
    "Asignado {allocated}, planificado en líneas {planned}, gastado {spent}, sin vincular {untraced} ({currency}).",
  "p.ledger.gates": "Líneas con fase: {gated}. Liberadas: {released}.",
  "p.ledger.milestones_met": "Hitos cumplidos: {met} de {total}.",
  "p.ledger.benefits": "Beneficios registrados: {n}.",
  "p.ledger.validated": "Validado en {currency}: {amount}",
  "p.ledger.lines_h3": "Líneas de presupuesto",
  "p.ledger.line_plan": "Programa",
  "p.ledger.line_desc": "Descripción",
  "p.ledger.line_planned": "Planificado",
  "p.ledger.line_actual": "Gastado",
  "p.ledger.line_gate": "Fase",
  "p.ledger.released": "liberada",
  "p.ledger.held": "retenida",
  "p.ledger.no_gate": "sin fase",
  "p.ledger.could_not_load": "No se pudo cargar el libro",
  "p.ledger.could_not_trace": "No se pudo rastrear la asignación",
  "p.ledger.a_line":
    "una partida de presupuesto (elija su plan arriba para verla)",
  "p.ledger.choose_line": "Elija una partida",
  "p.ledger.choose_plan_first": "Elija primero un plan arriba",
  "p.ledger.choose_plan": "Elija un plan",
  "p.ledger.choose_source": "Elija una fuente de financiación",
  "p.ledger.code_mapped": "Código de coste asignado.",
  "p.ledger.codes_heading": "Códigos de coste",
  "p.ledger.codes_says":
    "Cada código de coste financiero corresponde a una partida de presupuesto, para poder poner una exportación financiera junto al registro. Asignar un código ya asignado lo mueve.",
  "p.ledger.could_not_load_lines":
    "No se pudieron cargar las partidas de presupuesto de este plan.",
  "p.ledger.could_not_reconcile": "No se pudo conciliar la exportación.",
  "p.ledger.could_not_record": "No se registró la asignación.",
  "p.ledger.could_not_save": "No se guardó el cambio.",
  "p.ledger.f_allocation": "Asignación de la que tira",
  "p.ledger.f_amount": "Importe",
  "p.ledger.f_code_line": "Partida de presupuesto (del plan de arriba)",
  "p.ledger.f_code": "Código de coste",
  "p.ledger.f_csv": "O pegue la exportación (CSV)",
  "p.ledger.f_currency": "Moneda",
  "p.ledger.f_file": "Archivo de exportación financiera (CSV)",
  "p.ledger.f_on": "Asignado el",
  "p.ledger.f_plan": "Plan elegido",
  "p.ledger.f_reference": "Referencia (opcional)",
  "p.ledger.f_source": "Fuente de financiación",
  "p.ledger.line_detached": "La partida ya no tira de ninguna asignación.",
  "p.ledger.line_drawn": "La partida ahora tira de esa asignación.",
  "p.ledger.lines_heading":
    "Partidas de presupuesto y las asignaciones de las que tiran",
  "p.ledger.lines_says":
    "Una partida tira de una asignación de su propia fuente de financiación y moneda; fije primero su fuente en la página Financiación.",
  "p.ledger.map_code": "Asignar el código de coste",
  "p.ledger.matched_heading": "Códigos de coste coincidentes",
  "p.ledger.matched_line":
    "en la exportación {ledger}, en el registro {register}, diferencia {difference}",
  "p.ledger.mismatch_heading": "Filas en otra moneda que su partida",
  "p.ledger.mismatch_line":
    "fila en {row}, partida en {line}: {amount}, no sumada",
  "p.ledger.needs_source": "necesita antes una fuente de financiación",
  "p.ledger.no_allocation": "No tira de ninguna asignación",
  "p.ledger.no_codes": "Todavía no hay ningún código de coste asignado.",
  "p.ledger.no_lines": "Este plan no tiene partidas de presupuesto.",
  "p.ledger.no_matching_allocation":
    "Aún no hay asignación de la fuente y moneda de esta partida",
  "p.ledger.no_source": "sin fuente de financiación",
  "p.ledger.not_reconciled":
    "Sin conciliar: algo no coincide. No se escribió nada.",
  "p.ledger.problem_amount": "Escriba el importe como un número mayor que 0.",
  "p.ledger.problem_code_line": "Elija la partida de presupuesto del código.",
  "p.ledger.problem_code": "Un código de coste tiene entre 1 y 60 caracteres.",
  "p.ledger.problem_currency": "Escriba la moneda con tres letras, como GBP.",
  "p.ledger.problem_date_future": "La fecha de asignación no puede ser futura.",
  "p.ledger.problem_date": "Elija la fecha de la asignación.",
  "p.ledger.problem_export_empty":
    "Elija un archivo o pegue antes la exportación.",
  "p.ledger.problem_export_header":
    "La primera línea de la exportación debe nombrar cost_code, amount_minor y currency.",
  "p.ledger.problem_export_large": "La exportación supera 1 MB: divídala.",
  "p.ledger.problem_reference":
    "Una referencia puede tener como máximo 120 caracteres.",
  "p.ledger.problem_source": "Elija una fuente de financiación.",
  "p.ledger.reconcile_heading": "Conciliar una exportación financiera",
  "p.ledger.reconcile_says":
    "El código de coste de cada fila se busca en la tabla y sus importes se suman junto a lo que el registro tiene como gastado en la partida. No se escribe nada.",
  "p.ledger.reconcile": "Conciliar",
  "p.ledger.reconciled":
    "Conciliada: cada fila coincidió con una partida, en su moneda, sin diferencia.",
  "p.ledger.record_heading": "Registrar una asignación",
  "p.ledger.record_says":
    "Dinero que asignó una fuente de financiación. Las asignaciones se conservan tal como se registraron y nunca cambian.",
  "p.ledger.record": "Registrar la asignación",
  "p.ledger.recorded": "Asignación registrada.",
  "p.ledger.refused_export":
    "No se pudo leer la exportación: cada fila necesita un código de coste, un importe entero en unidades menores y una moneda, sin superar lo que admite el servicio.",
  "p.ledger.refused_line":
    "La partida no puede tirar de esa asignación: deben tener la misma fuente de financiación y la misma moneda.",
  "p.ledger.rows_read_one": "{n} fila leída.",
  "p.ledger.rows_read_other": "{n} filas leídas.",
  "p.ledger.to_funding":
    "Añada fuentes de financiación en la página Financiación",
  "p.ledger.unmatched_heading":
    "Códigos de coste sin una partida que pueda leer",
  "p.signoffs.nav": "Aprobación de hitos",
  "p.signoffs.title": "Aprobación de hitos — PPM",
  "p.signoffs.h1": "Aprobación de hitos",
  "p.signoffs.lede":
    "Algunos hitos solo se cumplen cuando las organizaciones asociadas que nombran los han aprobado. «Entregado» significa que la organización de ejecución dice que está hecho; «cumplido» es entregado con todas las aprobaciones vigentes. Una aprobación retirada vuelve a contar como pendiente.",
  "p.signoffs.loading": "Cargando las aprobaciones…",
  "p.signoffs.none": "Ningún hito necesita la aprobación de un socio.",
  "p.signoffs.totals":
    "Necesitan aprobación: {n}. Entregados: {delivered}. Cumplidos: {met}. Entregados pero no cumplidos: {gap}. Cumplidos, como proporción de los entregados: {share}.",
  "p.signoffs.no_share": "no medido",
  "p.signoffs.col_milestone": "Hito",
  "p.signoffs.col_due": "Vencimiento",
  "p.signoffs.col_state": "Situación",
  "p.signoffs.col_delivered": "Entregado el",
  "p.signoffs.col_met": "Cumplido el",
  "p.signoffs.col_outstanding": "A la espera de",
  "p.signoffs.col_act": "Firmar o retirar",
  "p.signoffs.state_met": "cumplido",
  "p.signoffs.state_delivered_not_met": "entregado, no cumplido",
  "p.signoffs.state_not_delivered": "no entregado",
  "p.signoffs.organization": "Organización",
  "p.signoffs.reason": "Motivo (para una retirada)",
  "p.signoffs.sign": "Firmar",
  "p.signoffs.withdraw": "Retirar",
  "p.signoffs.sign_for": "Firmar {name} como la organización elegida",
  "p.signoffs.withdraw_for": "Retirar la aprobación de {name}",
  "p.signoffs.not_built":
    "Qué organizaciones necesita un hito se elige en la página de gobernanza de cada plan; esta pantalla muestra y registra las firmas.",
  "p.signoffs.problem_inactive":
    "{name} ya no está activa, así que no puede exigirse: desmárquela para guardar.",
  "p.signoffs.problem_too_many":
    "Un hito puede exigir como mucho {max} organizaciones.",
  "p.signoffs.required_cancel": "Cancelar",
  "p.signoffs.required_change_for": "Cambiar quién debe firmar {name}",
  "p.signoffs.required_change": "Cambiar quién debe firmar",
  "p.signoffs.required_cleared": "{name} ya no necesita la firma de un socio.",
  "p.signoffs.required_heading": "Firma de socios",
  "p.signoffs.required_inactive": "{name} (ya no activa)",
  "p.signoffs.required_legend":
    "Organizaciones que deben firmar {name} (como mucho {max})",
  "p.signoffs.required_link": "Registrar una firma",
  "p.signoffs.required_no_orgs":
    "No hay ninguna organización activa para elegir.",
  "p.signoffs.required_none": "no necesita firma de socios",
  "p.signoffs.required_save": "Guardar quién debe firmar",
  "p.signoffs.required_saved": "Se guardó quién debe firmar {name}.",
  "p.signoffs.required_says":
    "Un hito con firma de socios solo se cumple cuando todas las organizaciones de abajo han firmado; entregado y cumplido son fechas distintas.",
  "p.signoffs.required_signed": "{name} firmó el {date}",
  "p.signoffs.required_unchanged": "No cambió nada, así que no se envió nada.",
  "p.signoffs.required_waiting": "{name} aún no ha firmado",
  "p.signoffs.could_not_load": "No se pudieron cargar las aprobaciones",
  "p.signoffs.could_not_save": "No se pudo registrar la aprobación",
  "p.key_persons.nav": "Personas clave",
  "p.key_persons.title": "Personas clave — PPM",
  "p.key_persons.h1": "Personas clave y puntos únicos de fallo",
  "p.key_persons.lede":
    "Las personas y funciones que la cartera no puede perder sin sufrir daños, quién las sustituiría y qué tan preparado está. Solo los gestores de recursos pueden leer esto, porque se refiere a personas con nombre.",
  "p.key_persons.loading": "Cargando…",
  "p.key_persons.none": "No se ha registrado ninguna persona ni función clave.",
  "p.key_persons.not_allowed":
    "Solo los gestores de recursos pueden leer las personas clave.",
  "p.key_persons.add_h2": "Registrar una persona o función clave",
  "p.key_persons.subject_kind": "Persona o función",
  "p.key_persons.kind_person": "persona",
  "p.key_persons.kind_role": "función",
  "p.key_persons.subject": "Nombre o función",
  "p.key_persons.successor": "Sucesor",
  "p.key_persons.readiness": "Preparación del sucesor",
  "p.key_persons.ready_ready": "preparado",
  "p.key_persons.ready_partly": "parcialmente preparado",
  "p.key_persons.ready_none": "no preparado",
  "p.key_persons.note": "Nota",
  "p.key_persons.add": "Registrar",
  "p.key_persons.col_subject": "Persona o función",
  "p.key_persons.col_successor": "Sucesor",
  "p.key_persons.col_ready": "Preparación",
  "p.key_persons.col_note": "Nota",
  "p.key_persons.col_act": "Acción",
  "p.key_persons.remove": "Eliminar {name}",
  "p.key_persons.remove_short": "Eliminar",
  "p.key_persons.no_successor": "ningún sucesor designado",
  "p.key_persons.contingent_h2": "Trabajadores eventuales",
  "p.key_persons.contingent_note":
    "Personas marcadas como mano de obra eventual, que pueden marcharse con poco preaviso.",
  "p.key_persons.contingent_none": "Nadie está marcado como eventual.",
  "p.key_persons.contingent_person": "Referencia de persona",
  "p.key_persons.contingent_unmark": "Desmarcar {name}",
  "p.key_persons.mark": "Marcar",
  "p.key_persons.unmark": "Desmarcar",
  "p.key_persons.could_not_load": "No se pudieron cargar las personas clave",
  "p.key_persons.could_not_save": "No se pudo guardar",
  "p.organizations.nav": "Organizaciones",
  "p.organizations.title": "Organizaciones — PPM",
  "p.organizations.h1": "Organizaciones asociadas",
  "p.organizations.lede":
    "Las organizaciones que ejecutan, patrocinan, suministran o colaboran en la cartera. Un paquete puede dirigirse a una organización, un hito puede necesitar su aprobación y una organización puede comprometer capacidad. Retirar una organización conserva su historial.",
  "p.organizations.loading": "Cargando las organizaciones…",
  "p.organizations.none": "Todavía no se ha añadido ninguna organización.",
  "p.organizations.add_h2": "Añadir una organización",
  "p.organizations.name": "Nombre",
  "p.organizations.kind": "Tipo",
  "p.organizations.add": "Añadir",
  "p.organizations.col_name": "Nombre",
  "p.organizations.col_kind": "Tipo",
  "p.organizations.col_state": "Situación",
  "p.organizations.col_audience": "Destinatario de paquetes",
  "p.organizations.col_act": "Acción",
  "p.organizations.kind_delivery": "ejecución",
  "p.organizations.kind_partner": "socio",
  "p.organizations.kind_sponsor": "patrocinador",
  "p.organizations.kind_supplier": "proveedor",
  "p.organizations.active": "activa",
  "p.organizations.retired": "retirada",
  "p.organizations.retire": "Retirar {name}",
  "p.organizations.restore": "Restaurar {name}",
  "p.organizations.retire_short": "Retirar",
  "p.organizations.restore_short": "Restaurar",
  "p.organizations.could_not_load": "No se pudieron cargar las organizaciones",
  "p.organizations.could_not_save": "No se pudo guardar la organización",
  "p.email_deliveries.nav": "Envíos de correo",
  "p.email_deliveries.title": "Envíos de correo — PPM",
  "p.email_deliveries.h1": "Envíos de correo",
  "p.email_deliveries.lede":
    "Cada correo que el servicio intentó enviar, una fila por destinatario: si se envió, si está a la espera de reintentarse o si falló definitivamente, cuántos intentos se hicieron y el último error. Solo los administradores pueden leer esto, porque contiene direcciones de correo electrónico.",
  "p.email_deliveries.loading": "Cargando los envíos…",
  "p.email_deliveries.none":
    "No se ha enviado ni intentado enviar ningún correo.",
  "p.email_deliveries.counts":
    "Enviados: {sent}. A la espera de reintento: {retry}. Fallidos definitivamente: {failed}.",
  "p.email_deliveries.filter": "Mostrar",
  "p.email_deliveries.all": "Todos",
  "p.email_deliveries.col_recipient": "Destinatario",
  "p.email_deliveries.col_subject": "Asunto",
  "p.email_deliveries.col_status": "Estado",
  "p.email_deliveries.col_attempts": "Intentos",
  "p.email_deliveries.col_error": "Último error",
  "p.email_deliveries.col_next": "Próximo intento",
  "p.email_deliveries.col_sent": "Enviado el",
  "p.email_deliveries.status_queued": "en cola",
  "p.email_deliveries.status_retry": "se reintentará",
  "p.email_deliveries.status_sent": "enviado",
  "p.email_deliveries.status_failed": "fallido",
  "p.email_deliveries.not_allowed":
    "Solo los administradores pueden leer los envíos de correo.",
  "p.email_deliveries.could_not_load":
    "No se pudieron cargar los envíos de correo",
  "p.director_pack.s_signoffs": "Aprobación de hitos",
  "p.director_pack.signoffs_note":
    "Algunos hitos solo se cumplen cuando las organizaciones asociadas que nombran los han aprobado. «Entregado» significa que la organización de ejecución dice que está hecho; «cumplido» es entregado con todas las aprobaciones vigentes. Una aprobación retirada cuenta como pendiente.",
  "p.director_pack.signoffs_totals":
    "Necesitan aprobación: {n}. Entregados: {delivered}. Cumplidos: {met}. Entregados pero no cumplidos: {gap}. Cumplidos, como proporción de los entregados: {share}.",
  "p.director_pack.so_met": "cumplido",
  "p.director_pack.so_delivered_not_met": "entregado, no cumplido",
  "p.director_pack.so_not_delivered": "no entregado",
  "p.director_pack.so_no_share": "todavía no hay ninguno entregado",
  "p.director_pack.c_milestone": "Hito",
  "p.director_pack.c_due": "Vencimiento",
  "p.director_pack.c_delivered_on": "Entregado el",
  "p.director_pack.c_met_on": "Cumplido el",
  "p.director_pack.c_outstanding": "Aprobación pendiente de",
  "p.director_pack.c_delivery": "Ejecución",
  "p.director_pack.dm_not_stated": "no indicado",
  "p.director_pack.dm_central_product": "producto central",
  "p.director_pack.dm_central_platform_local_deployment":
    "plataforma central, despliegue local",
  "p.director_pack.dm_local_programme_central_standards":
    "programa local, normas centrales",
  "p.director_pack.dm_owner": "responsable: {name}",
  "p.director_pack.dm_not_ours": "no ejecutado por la organización",
  "p.director_pack.s_flow": "Burn-up y flujo (programas de producto)",
  "p.director_pack.c_scope": "Tareas creadas",
  "p.director_pack.c_finished": "Tareas terminadas",
  "p.director_pack.c_open": "Tareas abiertas",
  "p.director_pack.c_throughput":
    "Terminadas por semana (últimas cuatro semanas)",
  "p.director_pack.c_burnup":
    "Terminadas, semana a semana (de la más antigua a la más reciente)",
  "p.director_pack.no_flow":
    "{name}: un programa de producto aún sin tareas, así que no hay burn-up ni flujo.",
  "p.director_pack.s_asks": "Peticiones externas y sus contrapartidas",
  "p.director_pack.asks_note":
    "Una petición del patrocinador no puede aprobarse hasta que indique qué desplaza. Aquí se enumeran las peticiones planteadas en el periodo.",
  "p.director_pack.asks_count":
    "Peticiones planteadas en el periodo: {n}. Aún sin contrapartida indicada: {without}.",
  "p.director_pack.ask_none": "aún no se ha indicado ninguna contrapartida",
  "p.director_pack.c_ask": "Petición",
  "p.director_pack.c_tradeoff": "Qué desplaza",
  "p.director_pack.register_line":
    "Programas con un registro completo: {complete} de {n}.",
  "p.director_pack.register_enforced":
    "En un programa con un registro incompleto no se libera ninguna línea de presupuesto ni se reserva ninguna asignación firme.",
  "p.director_pack.register_not_enforced":
    "Un registro incompleto es una advertencia, no un impedimento: el dinero aún puede moverse, y la advertencia queda auditada.",
  "p.director_pack.s_register": "Programas con un registro incompleto",
  "p.director_pack.c_score": "Completo",
  "p.director_pack.c_missing": "Falta",
  "p.director_pack.rf_sro": "responsable sénior",
  "p.director_pack.rf_partners": "socios",
  "p.director_pack.rf_stage": "etapa",
  "p.director_pack.rf_baseline": "línea base del cronograma",
  "p.director_pack.rf_target_date": "fecha objetivo",
  "p.director_pack.rf_budget": "presupuesto",
  "p.director_pack.rf_funding_source": "fuente de financiación",
  "p.director_pack.rf_confidence": "confianza en la entrega",
  "p.director_pack.rf_top_risks": "riesgos principales",
  "p.director_pack.rf_dependencies": "dependencias",
  "p.director_pack.s_watermelon": "Indicadores sandía",
  "p.director_pack.wm_note":
    "Verde por fuera, rojo por dentro. Se leen de este paquete y del paquete emitido anterior tal como se almacenaron, sin recalcularse. Un indicador sin nada que medir indica por qué; no es cero.",
  "p.director_pack.wm_gap":
    "La confianza autoevaluada y la independiente no coinciden",
  "p.director_pack.wm_late":
    "Descubrimiento tardío: rojo o ámbar-rojo ahora, verde en el paquete anterior",
  "p.director_pack.wm_red": "Rojo sin decisión registrada en el periodo",
  "p.director_pack.wm_signoff":
    "Hitos cumplidos con aprobación, de los entregados",
  "p.director_pack.wm_signoff_of": "{met} cumplidos de {delivered} entregados",
  "p.director_pack.wm_not_measured": "no medido",
  "p.director_pack.c_measure": "Indicador",
  "p.director_pack.c_programmes": "Programas, o por qué no se mide",
  "p.director_pack.wm_count": "Programas: {n}",
  "p.director_pack.s_keypeople": "Personas clave y mano de obra eventual",
  "p.director_pack.kp_note":
    "Los puntos únicos de fallo se cuentan aquí y nunca se nombran; el registro en sí es para los gestores de recursos.",
  "p.director_pack.kp_counts":
    "Puntos únicos de fallo: {n} ({people} personas, {roles} funciones). Sin sucesor: {nosucc}. Sucesor preparado: {ready}; parcialmente preparado: {partly}; no preparado: {none}.",
  "p.director_pack.kp_no_people": "nadie en este grupo",
  "p.director_pack.c_headcount": "Personas",
  "p.director_pack.c_contingent": "Eventuales",
  "p.director_pack.c_contingent_share": "Proporción eventual",
  "p.director_pack.s_triage": "Triaje y umbrales",
  "p.director_pack.tr_note":
    "Se muestra el triaje más reciente de cada programa. Rebasar un umbral abre una excepción que requiere una pausa, un reinicio, una detención o una anulación con caducidad; nada se pausa ni se detiene automáticamente.",
  "p.director_pack.tr_decisions":
    "Decisiones de este periodo: pausa {pause}, reinicio {reset}, detención {stop}, anulación {override}.",
  "p.director_pack.tr_open":
    "Excepciones de umbral a la espera de decisión: {n}.",
  "p.director_pack.tr_override_line":
    "Anulación vigente para {programme} hasta el {expiry}.",
  "p.director_pack.tr_finish": "terminar",
  "p.director_pack.tr_fix": "corregir",
  "p.director_pack.tr_pause": "pausar",
  "p.director_pack.tr_stop": "detener",
  "p.director_pack.c_outcome": "Triaje",
  "p.director_pack.c_triaged_on": "Triaje realizado el",
  "l.enum.pk_conf.threshold": "umbral rebasado",
  "p.director_pack.s_cutovers": "Decisiones de transición",
  "p.director_pack.co_note":
    "Una transición se decide según criterios de parada publicados antes de la decisión. Un «no» o un aplazamiento por seguridad es una decisión tomada según esos criterios, y no se cuenta como retraso.",
  "p.director_pack.co_counts":
    "Decididas en el periodo: sí {go}, no {nogo}, aplazamiento por seguridad {delay}.",
  "p.director_pack.co_go": "seguir adelante",
  "p.director_pack.co_no_go": "no",
  "p.director_pack.co_delay_for_safety": "aplazamiento por seguridad",
  "p.director_pack.c_cutover": "Transición",
  "p.director_pack.c_decision": "Decisión",
  "p.director_pack.c_decided_on": "Decidida el",
  "p.director_pack.s_outcomes": "Resultados y desescalado",
  "p.director_pack.oc_note":
    "El nivel de escalado es el que fijó el patrocinador, y nunca se calcula aquí. La racha siguiente cuenta los trimestres terminados en los que se cumplen todos los indicadores del cuadro de mando y todos los criterios; se ofrece a la persona que decide.",
  "p.director_pack.oc_level": "Nivel de escalado: {level}, fijado el {on}.",
  "p.director_pack.oc_no_level":
    "No se ha registrado ningún nivel de escalado.",
  "p.director_pack.oc_run":
    "Trimestres consecutivos cumplidos: {n}. La regla exige {need}.",
  "p.director_pack.oc_rule_yes":
    "La regla se cumple. Corresponde al patrocinador decidir si se baja un nivel.",
  "p.director_pack.oc_rule_no": "La regla aún no se cumple.",
  "p.director_pack.oc_met": "cumplido",
  "p.director_pack.oc_not_met": "no cumplido",
  "p.director_pack.oc_missing": "falta",
  "p.director_pack.c_quarter": "Trimestre",
  "p.director_pack.c_why": "Motivo",
  "p.balance.balance": "Equilibrio de la cartera",
  "p.balance.balance_ppm": "Equilibrio de la cartera — PPM",
  "p.balance.intro":
    "Dónde se sitúan el coste previsto, el valor de beneficio objetivo y el riesgo abierto de cada cartera frente a las demás, y los desequilibrios que se derivan de las cifras.",
  "p.balance.loading": "Cargando…",
  "p.balance.could_not_load": "No se pudo cargar el equilibrio de la cartera",
  "p.balance.none":
    "Todavía no hay coste, valor de beneficio ni riesgo registrados para ninguna cartera.",
  "p.balance.currency": "Moneda",
  "p.balance.unassigned": "Sin cartera",
  "p.balance.caption":
    "Cada burbuja es una cartera. Horizontal: coste previsto en {currency}. Vertical: valor de beneficio objetivo en {currency}. Área: exposición al riesgo abierto.",
  "p.balance.axis_cost": "Coste previsto ({currency})",
  "p.balance.axis_value": "Valor de beneficio objetivo ({currency})",
  "p.balance.imbalances": "Desequilibrios",
  "p.balance.no_imbalance":
    "Ninguna de las reglas siguientes señala un desequilibrio en estas cifras.",
  "p.balance.rules":
    "Se genera una señal cuando una cartera concentra el {concentration} % o más del coste previsto o del riesgo abierto (con al menos dos carteras), cuando su proporción de coste previsto o de riesgo supera en {gap} puntos porcentuales o más su proporción de valor objetivo, o cuando tiene coste previsto y ningún valor objetivo en la moneda elegida. Una señal es una invitación a revisar, no un veredicto.",
  "p.balance.flag_cost_concentration":
    "{name} concentra el {percent} % del coste previsto.",
  "p.balance.flag_risk_concentration":
    "{name} concentra el {percent} % de la exposición al riesgo abierto.",
  "p.balance.flag_cost_ahead_of_value":
    "{name} tiene una proporción de coste previsto muy superior a su {percent} % del valor objetivo.",
  "p.balance.flag_risk_ahead_of_value":
    "{name} tiene una proporción de riesgo abierto muy superior a su {percent} % del valor objetivo.",
  "p.balance.flag_no_value":
    "{name} tiene coste previsto pero ningún valor objetivo registrado en {currency}.",
  "p.balance.table": "Las mismas cifras en una tabla",
  "p.balance.portfolio": "Cartera",
  "p.balance.cost": "Coste previsto ({currency})",
  "p.balance.value": "Valor objetivo ({currency})",
  "p.balance.risk": "Exposición al riesgo abierto",
  "p.balance.cost_share": "Proporción del coste",
  "p.balance.value_share": "Proporción del valor",
  "p.balance.risk_share": "Proporción del riesgo",
  "p.balance.flags": "Señales",
  "p.balance.no_flag": "ninguna",
  "p.balance.notes":
    "Agrupado por cartera, no por categoría ni por objetivo estratégico: la cartera es la única agrupación por la que se informan a la vez el coste, el valor de beneficio y el riesgo. El coste es el importe previsto en las líneas de presupuesto, el valor es el objetivo de los beneficios registrados y el riesgo es la puntuación de exposición abierta; el dinero en distintas monedas nunca se suma.",
  "p.roadmap.title":
    "Hoja de ruta — impacto, esfuerzo y Ahora / Después / Más adelante",
  "p.roadmap.h1":
    "Hoja de ruta: impacto, esfuerzo y Ahora / Después / Más adelante",
  "p.roadmap.lede":
    "Ideas abiertas y propuestas aún pendientes de decisión, situadas según cuánto importan (impacto) y cuánto trabajo requieren (esfuerzo). El servicio no guarda impacto ni esfuerzo para una idea o propuesta, así que usted valora cada una aquí; sus valoraciones se guardan solo en este navegador y no se comparten ni se envían al servicio. Un elemento sin ambas valoraciones no se sitúa.",
  "p.roadmap.loading": "Cargando las ideas y propuestas…",
  "p.roadmap.matrix_h2": "Matriz de impacto y esfuerzo",
  "p.roadmap.matrix_caption":
    "Impacto en filas, esfuerzo en columnas. Cada celda enumera los elementos valorados de esa forma.",
  "p.roadmap.impact": "Impacto",
  "p.roadmap.effort": "Esfuerzo",
  "p.roadmap.impact_level": "Impacto {level}",
  "p.roadmap.effort_level": "Esfuerzo {level}",
  "p.roadmap.level_low": "Bajo",
  "p.roadmap.level_medium": "Medio",
  "p.roadmap.level_high": "Alto",
  "p.roadmap.rate_h2": "Valorar los elementos",
  "p.roadmap.rate_hint":
    "Esta tabla es la forma de usar la matriz con el teclado: elija un impacto y un esfuerzo para cada elemento y este se moverá en la matriz y en las listas Ahora / Después / Más adelante.",
  "p.roadmap.col_item": "Elemento",
  "p.roadmap.col_type": "Tipo",
  "p.roadmap.col_horizon": "Cuándo",
  "p.roadmap.kind_idea": "Idea",
  "p.roadmap.kind_proposal": "Propuesta",
  "p.roadmap.not_rated": "Sin valorar",
  "p.roadmap.rate_impact": "Impacto de {title}",
  "p.roadmap.rate_effort": "Esfuerzo de {title}",
  "p.roadmap.no_items": "No hay ideas ni propuestas abiertas que valorar.",
  "p.roadmap.horizons_h2": "Ahora / Después / Más adelante",
  "p.roadmap.horizon_rule":
    "Ahora: el impacto es mayor que el esfuerzo. Después: el impacto es igual al esfuerzo y al menos medio. Más adelante: todo lo demás valorado. Aún sin valorar: no se ha elegido impacto o esfuerzo.",
  "p.roadmap.h_now": "Ahora",
  "p.roadmap.h_next": "Después",
  "p.roadmap.h_later": "Más adelante",
  "p.roadmap.h_unrated": "Aún sin valorar",
  "p.roadmap.nothing_here": "Aquí no hay nada.",
  "p.roadmap.see_ideas": "Ver ideas y propuestas en la hoja de ruta",
  "p.people_impact.title": "Impacto en las personas (del servicio de personal)",
  "p.people_impact.intro":
    "Solo recuentos, del servicio de personal (WPM), consultados con su propio inicio de sesión. No se nombra a ninguna persona ni se suma nada.",
  "p.people_impact.ask": "Consultar al servicio de personal",
  "p.people_impact.asking": "Consultando…",
  "p.people_impact.failed": "No se pudo cargar el impacto en las personas.",
  "p.people_impact.not_known": "Desconocido.",
  "p.people_impact.source": "Del servicio de personal, a fecha de {date}.",
  "p.people_impact.unknown": "Desconocido: {reason}",
  "p.people_impact.end_sent":
    "Fecha de fin enviada: {date} (la fecha objetivo de este plan).",
  "p.people_impact.committed": "Personas comprometidas, por grupo",
  "p.people_impact.ending":
    "Contratos temporales y de servicios que terminan antes del fin",
  "p.people_impact.in_no_pool": "En ningún grupo",
  "p.people_impact.end_not_recorded":
    "Sin fin registrado (no se cuenta como finalización)",
  "p.people_impact.hidden": "oculto (menos de {floor})",
  "p.people_impact.no_pools": "No hay grupos que mostrar.",
  "p.people_impact.part_unknown": "Desconocido: {reason}",
  "p.people_impact.over": "Meses-grupo sobrecomprometidos",
  "p.people_impact.over_counts":
    "{over} de {considered} meses-grupo están sobrecomprometidos; de {unknown} no se puede saber.",
  "p.people_impact.no_total":
    "No se ofrece un total: una persona puede contar en más de un grupo.",
  "l.enum.pk_heat.supply_unknown": "oferta desconocida",
  "p.capacity_heatmap.nav": "Mapa de calor de capacidad",
  "p.capacity_heatmap.title": "Mapa de calor de capacidad — PPM",
  "p.capacity_heatmap.h1": "Capacidad por grupo de competencias y mes",
  "p.capacity_heatmap.lede":
    "Cada casilla compara el trabajo pedido a un grupo de competencias en un mes con las personas disponibles para él, en equivalentes a tiempo completo (FTE).",
  "p.capacity_heatmap.bridge_note":
    "Un grupo vinculado al servicio de personal obtiene de allí su oferta. Lo que se introdujo aquí para ese grupo se muestra al lado y nunca se suma. Un mes sin cifra del servicio de personal se muestra como desconocido, nunca como cero.",
  "p.capacity_heatmap.from": "Primer mes",
  "p.capacity_heatmap.months": "Número de meses",
  "p.capacity_heatmap.show": "Mostrar",
  "p.capacity_heatmap.loading": "Cargando el mapa de calor…",
  "p.capacity_heatmap.could_not_load":
    "No se pudo cargar el mapa de calor de capacidad.",
  "p.capacity_heatmap.none":
    "No hay grupos de competencias ni demanda que mostrar.",
  "p.capacity_heatmap.caption":
    "Demanda y oferta en equivalentes a tiempo completo, por grupo de competencias y mes",
  "p.capacity_heatmap.col_pool": "Grupo de competencias",
  "p.capacity_heatmap.linked_to": "Vinculado al servicio de personal",
  "p.capacity_heatmap.demand": "Demanda: {fte}",
  "p.capacity_heatmap.supply": "Oferta: {fte}",
  "p.capacity_heatmap.gap": "Brecha: {fte}",
  "p.capacity_heatmap.unknown": "desconocido",
  "p.capacity_heatmap.none_recorded": "nada registrado",
  "p.capacity_heatmap.source_wpm": "Oferta del servicio de personal",
  "p.capacity_heatmap.source_here": "Oferta introducida aquí",
  "p.capacity_heatmap.peer_live": "En vivo, tomada {when}.",
  "p.capacity_heatmap.peer_live_no_time": "En vivo.",
  "p.capacity_heatmap.peer_last":
    "Última recepción {age} (tomada {when}); ahora no está en vivo: {reason}",
  "p.capacity_heatmap.peer_hidden":
    "Oculto: el servicio de personal oculta un mes compuesto por menos trabajadores que su umbral mínimo.",
  "p.capacity_heatmap.peer_unknown": "Desconocido: {reason}",
  "p.capacity_heatmap.no_reason":
    "no se recibió ninguna cifra del servicio de personal",
  "p.capacity_heatmap.entered_here":
    "Introducido aquí: {fte} (se muestra al lado, sin sumarse)",
  "p.capacity_heatmap.entered_here_none": "Introducido aquí: nada",
  "p.capacity_heatmap.partners_short":
    "los socios entregaron menos de lo comprometido",
  "p.capacity_heatmap.no_pool": "Sin grupo asignado",
  "p.capacity_heatmap.key":
    "▲ déficit · ◆ ajustado · ● suficiente · ○ sin demanda · – sin oferta registrada · ? oferta desconocida. La palabra siempre se escribe también.",
  "p.capacity_heatmap.every_figure": "Todas las cifras en una tabla",
  "p.capacity_heatmap.table_label": "Capacidad por grupo de competencias y mes",
  "p.capacity_heatmap.c_pool": "Grupo de competencias",
  "p.capacity_heatmap.c_month": "Mes",
  "p.capacity_heatmap.c_reading": "Lectura",
  "p.capacity_heatmap.c_demand": "Demanda (equivalente a tiempo completo)",
  "p.capacity_heatmap.c_supply": "Oferta (equivalente a tiempo completo)",
  "p.capacity_heatmap.c_source": "Oferta de",
  "p.capacity_heatmap.c_status": "Lectura del servicio de personal",
  "p.capacity_heatmap.c_entered_here": "Introducido aquí",
  "p.capacity_heatmap.see_pools":
    "Vincular grupos de competencias al servicio de personal",
  "p.skill_pools.nav": "Grupos de competencias",
  "p.skill_pools.title": "Grupos de competencias — PPM",
  "p.skill_pools.h1": "Grupos de competencias",
  "p.skill_pools.lede":
    "Un grupo de competencias reúne las funciones y competencias que pueden hacer el mismo trabajo. Aquí puede vincular un grupo con el grupo correspondiente del servicio de personal.",
  "p.skill_pools.rule":
    "El vínculo debe establecerse en ambos lados, aquí y en el servicio de personal, por una persona que pueda ver ambos. Nunca se empareja por nombre. Mientras un grupo está vinculado, su oferta en el mapa de calor de capacidad procede del servicio de personal y su demanda se envía allí.",
  "p.skill_pools.loading": "Cargando los grupos de competencias…",
  "p.skill_pools.could_not_load":
    "No se pudieron cargar los grupos de competencias.",
  "p.skill_pools.none": "Todavía no se ha creado ningún grupo de competencias.",
  "p.skill_pools.col_name": "Nombre",
  "p.skill_pools.col_roles": "Funciones",
  "p.skill_pools.col_skills": "Competencias",
  "p.skill_pools.col_state": "Situación",
  "p.skill_pools.col_link": "Grupo del servicio de personal",
  "p.skill_pools.col_act": "Cambiar",
  "p.skill_pools.active": "activo",
  "p.skill_pools.retired": "retirado",
  "p.skill_pools.not_linked": "sin vincular",
  "p.skill_pools.change_short": "Cambiar vínculo",
  "p.skill_pools.change": "Cambiar el vínculo de {name}",
  "p.skill_pools.edit_h2": "Vincular {name} al servicio de personal",
  "p.skill_pools.ref_label": "Id del grupo en el servicio de personal",
  "p.skill_pools.ref_hint":
    "Cópielo del grupo en el servicio de personal: 36 caracteres, en minúsculas, con guiones, como 3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b.",
  "p.skill_pools.ref_invalid":
    "Eso no es un id de grupo del servicio de personal: debe tener 36 caracteres, en minúsculas, con guiones.",
  "p.skill_pools.save": "Guardar el vínculo",
  "p.skill_pools.clear": "Quitar el vínculo",
  "p.skill_pools.cancel": "Cancelar",
  "p.skill_pools.confirm_clear":
    "¿Quitar el vínculo entre {name} y el servicio de personal? Su oferta en el mapa de calor volverá a ser la introducida aquí. Todas las cifras de demanda ya enviadas al servicio de personal para este grupo se retirarán allí (se enviarán como cero).",
  "p.skill_pools.saved":
    "{name} está ahora vinculado al grupo {ref} del servicio de personal.",
  "p.skill_pools.cleared":
    "{name} ya no está vinculado al servicio de personal.",
  "p.skill_pools.taken":
    "Otro grupo de aquí ya está vinculado a ese grupo del servicio de personal. Quite primero ese vínculo: un grupo aquí por cada grupo allí.",
  "p.skill_pools.invalid":
    "El servicio no aceptó ese id. Debe ser el id de grupo del servicio de personal: 36 caracteres, en minúsculas, con guiones.",
  "p.skill_pools.missing":
    "Ese grupo ya no existe. La lista se ha vuelto a cargar.",
  "p.skill_pools.could_not_save": "No se pudo cambiar el vínculo.",
  "p.skill_pools.see_heatmap": "Ver el mapa de calor de capacidad",
  "p.skill_pools.withdrawing":
    "Cifras de demanda que se están retirando del grupo del servicio de personal al que estaba vinculado: {n}.",
  "p.skill_pools.add_heading": "Añadir un grupo",
  "p.skill_pools.add_says":
    "Un grupo reúne roles y habilidades de recursos, para que el mapa de calor de capacidad cuente su demanda y oferta una sola vez. Un rol o habilidad está como mucho en un grupo.",
  "p.skill_pools.add": "Añadir el grupo",
  "p.skill_pools.added": "Grupo {name} añadido.",
  "p.skill_pools.choose_pool": "Elija un grupo",
  "p.skill_pools.clash_item": "{value} (en {pool})",
  "p.skill_pools.conflict_members":
    "Ya existe un grupo con ese nombre, o uno de estos roles o habilidades ya está en otro grupo.",
  "p.skill_pools.could_not_add": "No se añadió el grupo.",
  "p.skill_pools.f_name": "Nombre del grupo",
  "p.skill_pools.f_pool": "Grupo",
  "p.skill_pools.f_roles": "Roles de recurso",
  "p.skill_pools.f_skills": "Habilidades",
  "p.skill_pools.list_says":
    "Separe los roles o habilidades con comas o saltos de línea; una repetición se guarda una vez.",
  "p.skill_pools.members_heading": "Cambiar un grupo",
  "p.skill_pools.members_saved":
    "Se guardaron los roles y habilidades de {name}.",
  "p.skill_pools.problem_clash": "Ya en otro grupo: {list}.",
  "p.skill_pools.problem_name_taken": "Ya existe un grupo con ese nombre.",
  "p.skill_pools.problem_name":
    "El nombre de un grupo tiene entre 2 y 120 caracteres.",
  "p.skill_pools.restore": "Restaurar el grupo",
  "p.skill_pools.restored": "Grupo {name} restaurado.",
  "p.skill_pools.retire_says":
    "Un grupo retirado sigue en el registro y conserva sus roles y habilidades; al restaurarlo vuelve a contar.",
  "p.skill_pools.retire": "Retirar el grupo",
  "p.skill_pools.retired_done": "Grupo {name} retirado.",
  "p.skill_pools.save_members": "Guardar los roles y habilidades",
  "p.skill_pools.unchanged": "No cambió nada, así que no se envió nada.",
  "p.capacity_heatmap.set_commitment": "Establecer un compromiso de capacidad",
  "p.commitments.title": "Compromisos de capacidad — PPM",
  "p.commitments.h1": "Compromisos de capacidad",
  "p.commitments.lede":
    "Registre la capacidad prometida a un programa (oferta) o el trabajo que un programa necesita de un grupo de competencias (demanda), para un mes, en equivalentes a tiempo completo.",
  "p.commitments.bridge_note":
    "Para un grupo vinculado al servicio de personal, un compromiso de demanda también envía allí la demanda total del programa para ese grupo y mes. Un total de cero retira allí la cifra. No se reintenta nada: guarde de nuevo para volver a enviar.",
  "p.commitments.see_heatmap": "Ver el mapa de calor de capacidad",
  "p.commitments.from_plan": "Compromisos de capacidad de este programa",
  "p.commitments.form_h2": "Establecer un compromiso",
  "p.commitments.choose": "Elija…",
  "p.commitments.pool": "Grupo de competencias",
  "p.commitments.pool_linked": "{name} (vinculado al servicio de personal)",
  "p.commitments.pool_hint": "Solo se muestran los grupos activos.",
  "p.commitments.pool_missing": "Elija un grupo de competencias.",
  "p.commitments.plan": "Programa",
  "p.commitments.plan_hint":
    "El plan debe ser un programa (un plan de tipo Programa).",
  "p.commitments.plan_missing": "Elija un programa.",
  "p.commitments.kind": "Qué se compromete",
  "p.commitments.kind_demand":
    "Demanda: trabajo que el programa necesita del grupo",
  "p.commitments.kind_supply": "Oferta: capacidad prometida al programa",
  "p.commitments.organization": "Organización asociada (opcional)",
  "p.commitments.organization_hint":
    "Déjelo vacío para su propia organización.",
  "p.commitments.month": "Mes",
  "p.commitments.month_hint": "Un mes: año y mes.",
  "p.commitments.month_invalid": "Indique un mes como año y mes.",
  "p.commitments.committed": "Comprometido (equivalente a tiempo completo)",
  "p.commitments.committed_hint":
    "Un número de 0 a 1000, escrito como usted escribe los números. Cero retira una cifra de demanda enviada al servicio de personal.",
  "p.commitments.delivered":
    "Entregado (equivalente a tiempo completo, opcional)",
  "p.commitments.delivered_hint": "Lo que realmente se entregó, si se conoce.",
  "p.commitments.fte_invalid":
    "Un número de 0 a 1000, escrito como usted escribe los números.",
  "p.commitments.note": "Nota (opcional)",
  "p.commitments.save": "Guardar el compromiso",
  "p.commitments.saved": "Guardado: {pool}, {plan}, {month}.",
  "p.commitments.not_found":
    "No se encontró ese programa, o usted no puede leerlo.",
  "p.commitments.refused":
    "El servicio no lo aceptó. El plan debe ser un programa, el grupo uno activo y cada cifra de 0 a 1000.",
  "p.commitments.could_not_save": "No se pudo guardar el compromiso.",
  "p.commitments.could_not_load":
    "No se pudieron cargar los compromisos de capacidad.",
  "p.commitments.loading": "Cargando los envíos recientes…",
  "p.commitments.sends_h2": "Envíos recientes al servicio de personal",
  "p.commitments.sends_note":
    "El último envío de cada programa, grupo y mes, del más reciente al más antiguo. Un envío iniciado muestra aquí su respuesta cuando llega: actualice para verla.",
  "p.commitments.refresh": "Actualizar",
  "p.commitments.no_sends":
    "Todavía no se ha enviado nada al servicio de personal.",
  "p.commitments.c_pool": "Grupo de competencias",
  "p.commitments.c_programme": "Programa",
  "p.commitments.c_month": "Mes",
  "p.commitments.c_sent": "Enviado",
  "p.commitments.c_outcome": "Qué ocurrió",
  "p.commitments.c_at": "Cuándo",
  "p.commitments.sent_fte": "{fte} equivalentes a tiempo completo",
  "p.commitments.sent_withdrawal": "una retirada (cero)",
  "p.commitments.sent_nothing": "nada",
  "p.commitments.out_not_sent_at_all":
    "Este compromiso no se envía al servicio de personal: solo se envía la demanda de un grupo vinculado a él.",
  "p.commitments.out_none_yet":
    "Todavía no se ha enviado nada al servicio de personal para este programa, grupo y mes.",
  "p.commitments.out_started":
    "Enviando {fte} equivalentes a tiempo completo (el total del programa para este grupo y mes) al servicio de personal. Su respuesta aparece en Envíos recientes.",
  "p.commitments.out_started_zero":
    "Retirando del servicio de personal la cifra de este programa para este grupo y mes, porque el total es ahora cero. Su respuesta aparece en Envíos recientes.",
  "p.commitments.out_sent":
    "El servicio de personal aceptó {fte} equivalentes a tiempo completo.",
  "p.commitments.out_sent_zero":
    "El servicio de personal retiró su cifra para este programa, grupo y mes.",
  "p.commitments.out_duplicate":
    "El servicio de personal ya tenía esta cifra; allí no cambió nada.",
  "p.commitments.out_refused":
    "El servicio de personal la rechazó. No se reintenta nada; guarde de nuevo para volver a enviar.",
  "p.commitments.out_unreachable":
    "No se pudo contactar con el servicio de personal, o no respondió a tiempo. No se reintenta nada; guarde de nuevo para volver a enviar.",
  "p.commitments.out_not_configured":
    "No enviado: este servicio no está configurado para conectar con el servicio de personal.",
  "p.commitments.out_no_token":
    "No enviado: no había ningún inicio de sesión que transmitir al servicio de personal.",
  "p.commitments.out_too_large":
    "No enviado: el servicio de personal admite como máximo 10 000 equivalentes a tiempo completo.",
  "p.commitments.out_old_zero":
    "No enviado: un cero registrado antes de que se enviaran los ceros, así que el servicio de personal conservó su última cifra. Guarde de nuevo para retirarla.",
  "p.commitments.out_other": "El servicio registró: {outcome}.",
  "p.discussion.heading": "Conversación",
  "p.discussion.loading": "Cargando…",
  "p.discussion.could_not_load_comments":
    "No se pudieron cargar los comentarios.",
  "p.discussion.could_not_load_reactions":
    "No se pudieron cargar las reacciones.",
  "p.discussion.could_not_load_files": "No se pudieron cargar los archivos.",
  "p.discussion.could_not_load_watch": "No se pudo leer si sigue esto.",
  "p.discussion.sign_in_to_follow":
    "Inicie sesión para seguir esto y recibir avisos de comentarios y cambios nuevos.",
  "p.discussion.you_follow":
    "Sigue esto: recibe avisos de comentarios y cambios nuevos.",
  "p.discussion.you_do_not_follow": "No sigue esto.",
  "p.discussion.followers_one": "{n} persona lo sigue.",
  "p.discussion.followers_other": "{n} personas lo siguen.",
  "p.discussion.follow": "Seguir",
  "p.discussion.unfollow": "Dejar de seguir",
  "p.discussion.followed": "Ahora sigue esto.",
  "p.discussion.unfollowed": "Ya no sigue esto.",
  "p.discussion.reactions_label": "Reacciones",
  "p.discussion.reaction_named_one": "{reaction}: {n} persona",
  "p.discussion.reaction_named_other": "{reaction}: {n} personas",
  "p.discussion.r_thumbs_up": "Pulgar arriba",
  "p.discussion.r_heart": "Corazón",
  "p.discussion.r_celebrate": "Celebrar",
  "p.discussion.r_eyes": "Lo estoy mirando",
  "p.discussion.r_question": "Pregunta",
  "p.discussion.comments_heading": "Comentarios",
  "p.discussion.no_comments": "Todavía no hay comentarios.",
  "p.discussion.no_author": "alguien sin sesión iniciada",
  "p.discussion.by_at": "De {who}, {when}",
  "p.discussion.edited_at": "editado {when}",
  "p.discussion.edit": "Editar",
  "p.discussion.edit_named": "Editar el comentario de {who}, {when}",
  "p.discussion.delete": "Eliminar",
  "p.discussion.delete_named": "Eliminar el comentario de {who}, {when}",
  "p.discussion.confirm_delete_comment":
    "¿Eliminar este comentario? Se conservan sus versiones anteriores, pero nadie volverá a verlo en la página.",
  "p.discussion.earlier": "Versiones anteriores",
  "p.discussion.no_earlier": "No se conserva ninguna versión anterior.",
  "p.discussion.replaced_at": "Sustituido {when}",
  "p.discussion.f_edit": "Cambiar el comentario",
  "p.discussion.save_edit": "Guardar el comentario",
  "p.discussion.cancel": "Cancelar",
  "p.discussion.f_comment": "Su comentario",
  "p.discussion.chars_left": "Quedan {n} de {max} caracteres.",
  "p.discussion.chars_over": "{n} caracteres por encima del límite de {max}.",
  "p.discussion.post": "Publicar el comentario",
  "p.discussion.posted": "Se publicó el comentario.",
  "p.discussion.edited":
    "Se cambió el comentario; se conserva su versión anterior.",
  "p.discussion.deleted": "Se eliminó el comentario.",
  "p.discussion.problem_blank": "Escriba primero el comentario.",
  "p.discussion.problem_too_long":
    "Un comentario puede tener como máximo 4000 caracteres.",
  "p.discussion.files_heading": "Archivos",
  "p.discussion.files_intro":
    "Los archivos adjuntos aquí se guardan en su equipo al abrirlos; nunca se muestran dentro de esta página.",
  "p.discussion.no_files": "No hay archivos adjuntos.",
  "p.discussion.file_facts": "{type}, {size}, añadido por {who}, {when}",
  "p.discussion.remove_file": "Quitar",
  "p.discussion.remove_file_named": "Quitar el archivo {name}",
  "p.discussion.confirm_remove_file":
    "¿Quitar el archivo {name}? Nadie podrá volver a descargarlo desde aquí.",
  "p.discussion.removed_file": "Se quitó el archivo {name}.",
  "p.discussion.f_file": "Archivo que adjuntar",
  "p.discussion.files_rules":
    "Un PDF, una imagen (PNG, JPEG, GIF o WebP), texto sin formato, CSV, Markdown, JSON o un archivo de Word, Excel o PowerPoint, de 10 MB como máximo. Se rechazan páginas web, dibujos SVG, archivos comprimidos y programas.",
  "p.discussion.attach": "Adjuntar el archivo",
  "p.discussion.attached": "Se adjuntó el archivo {name}.",
  "p.discussion.problem_choose_file": "Elija primero un archivo.",
  "p.discussion.problem_empty": "Ese archivo está vacío.",
  "p.discussion.problem_too_large":
    "Ese archivo supera los 10 MB, el límite para un archivo.",
  "p.discussion.problem_type":
    "No se puede adjuntar ese tipo de archivo. La lista de abajo indica qué tipos se admiten.",
  "p.discussion.type_pdf": "Documento PDF",
  "p.discussion.type_png": "Imagen PNG",
  "p.discussion.type_jpeg": "Imagen JPEG",
  "p.discussion.type_gif": "Imagen GIF",
  "p.discussion.type_webp": "Imagen WebP",
  "p.discussion.type_text": "texto sin formato",
  "p.discussion.type_csv": "tabla CSV",
  "p.discussion.type_markdown": "texto Markdown",
  "p.discussion.type_json": "datos JSON",
  "p.discussion.type_word": "documento de Word",
  "p.discussion.type_excel": "libro de Excel",
  "p.discussion.type_powerpoint": "diapositivas de PowerPoint",
  "p.discussion.size_b": "{n} B",
  "p.discussion.size_kb": "{n} KB",
  "p.discussion.size_mb": "{n} MB",
  "p.discussion.refused_sign_in":
    "Inicie sesión primero: el servicio no sabe quién es usted.",
  "p.discussion.refused_not_author":
    "Solo quien escribió un comentario puede cambiarlo o eliminarlo.",
  "p.discussion.refused_not_uploader":
    "Solo quien adjuntó un archivo, o un administrador, puede quitarlo.",
  "p.discussion.refused_forbidden": "No puede leer ni cambiar esto.",
  "p.discussion.refused_comment_gone":
    "Ese comentario ya no existe; se han vuelto a leer los comentarios.",
  "p.discussion.refused_file_gone":
    "Ese archivo ya se había quitado; se han vuelto a leer los archivos.",
  "p.discussion.refused_gone":
    "No se encontró: puede que se haya quitado o que usted no pueda verlo.",
  "p.discussion.refused_comment_invalid":
    "El servicio no aceptó el comentario: no puede estar vacío y puede tener como máximo 4000 caracteres.",
  "p.discussion.refused_file_invalid":
    "El servicio no aceptó el archivo: su tipo, su tamaño o su contenido no superaron las comprobaciones (un archivo debe ser lo que dice su tipo, y no se admiten programas).",
  "p.discussion.refused_too_large":
    "El archivo es demasiado grande para enviarlo.",
  "p.discussion.refused_scan_unavailable":
    "No se adjuntó el archivo: el analizador de malware no respondió. Inténtelo más tarde.",
  "p.me.timesheet_link": "Mi hoja de horas",
  "p.me.timesheet_note":
    "— su tiempo por día y por plan, semana a semana, y el envío de una semana para su aprobación.",
  "p.timesheet.title_ppm": "Mi hoja de horas — PPM",
  "p.timesheet.heading": "Mi hoja de horas",
  "p.timesheet.back": "← Mi trabajo",
  "p.timesheet.intro":
    "El tiempo que registró en una semana, por día y por plan, en horas y minutos, con el tiempo facturable y el no facturable siempre por separado. Envíe una semana terminada para su aprobación; una vez aprobada es definitiva y ya no se puede registrar más tiempo en ella.",
  "p.timesheet.loading": "Cargando…",
  "p.timesheet.week_nav": "Elegir la semana",
  "p.timesheet.previous_week": "Semana anterior",
  "p.timesheet.this_week": "Esta semana",
  "p.timesheet.next_week": "Semana siguiente",
  "p.timesheet.f_week": "Semana, por ejemplo 2026-W46",
  "p.timesheet.show_week": "Mostrar la semana",
  "p.timesheet.week_hint":
    "Las semanas van de lunes a domingo y se numeran según la norma internacional, sea cual sea su primer día de la semana. Hoy es {today} en su zona horaria, {tz}.",
  "p.timesheet.problem_week":
    "Eso no es una semana: escriba el año, ‘-W’ y el número de la semana, por ejemplo 2026-W46.",
  "p.timesheet.mine_heading": "Su semana {week}",
  "p.timesheet.mine_caption": "Su tiempo en la semana {week}, por plan y día",
  "p.timesheet.status_open": "Abierta: aún no enviada para aprobación",
  "p.timesheet.status_submitted":
    "Enviada para aprobación: a la espera de una decisión",
  "p.timesheet.status_approved":
    "Aprobada: definitiva y bloqueada contra tiempo nuevo",
  "p.timesheet.status_rejected": "Devuelta: modifíquela y vuelva a enviarla",
  "p.timesheet.rejected_because": "quien aprueba escribió: {comment}",
  "p.timesheet.decision_comment": "con la nota: {comment}",
  "p.timesheet.no_entries":
    "No hay tiempo registrado en esta semana, así que no hay total.",
  "p.timesheet.col_plan": "Nombre del plan",
  "p.timesheet.col_total": "Suma",
  "p.timesheet.col_billable": "Facturable",
  "p.timesheet.col_non_billable": "No facturable",
  "p.timesheet.row_day_totals": "Cada día",
  "p.timesheet.plan_unnamed": "Plan {id}… (nombre no mostrado)",
  "p.timesheet.split_line":
    "Facturable {billable}, no facturable {non_billable}: {total} en total.",
  "p.timesheet.omitted":
    "Registros omitidos porque están en planes que usted no puede leer: {n}.",
  "p.timesheet.hours_one": "{n} h",
  "p.timesheet.hours_other": "{n} h",
  "p.timesheet.minutes_one": "{n} minuto",
  "p.timesheet.minutes_other": "{n} minutos",
  "p.timesheet.duration_hm": "{h} {m}",
  "p.timesheet.submit": "Enviar esta semana para aprobación",
  "p.timesheet.submit_again": "Volver a enviar esta semana para aprobación",
  "p.timesheet.submitted_notice": "La semana {week} se envió para aprobación.",
  "p.timesheet.nothing_to_submit":
    "Una semana sin tiempo registrado no se puede enviar para aprobación.",
  "p.timesheet.waiting": "Esta semana espera la decisión de quien aprueba.",
  "p.timesheet.locked":
    "Una semana aprobada es definitiva: ya no se puede registrar más tiempo en ella.",
  "p.timesheet.could_not_load": "No se pudo cargar su semana.",
  "p.timesheet.could_not_submit":
    "No se pudo enviar la semana para aprobación.",
  "p.timesheet.other_heading":
    "La semana de otra persona, para quienes aprueban",
  "p.timesheet.other_intro":
    "Si puede aprobar hojas de horas, indique la referencia de la persona para leer su semana, la elegida arriba, y apruébela o devuélvala. Nadie decide sobre su propia semana, devolver una exige un motivo y una aprobación es definitiva.",
  "p.timesheet.f_person": "Persona: ‘person:’ o ‘worker:’ y su identificador",
  "p.timesheet.show_person": "Mostrar su semana",
  "p.timesheet.problem_person":
    "Indique la persona como ‘person:’ o ‘worker:’ seguido de su identificador, sin nada más.",
  "p.timesheet.other_week_heading": "Semana {week} de {person}",
  "p.timesheet.other_caption":
    "Tiempo de {person} en la semana {week}, por plan y día",
  "p.timesheet.decision_legend": "Su decisión",
  "p.timesheet.approve": "Aprobar",
  "p.timesheet.reject": "Devolver",
  "p.timesheet.f_comment": "Nota, si quiere añadir una",
  "p.timesheet.f_comment_required": "Por qué se devuelve (obligatorio)",
  "p.timesheet.approve_final":
    "Una aprobación es definitiva: la semana queda bloqueada y nadie puede devolverla.",
  "p.timesheet.record_decision": "Registrar la decisión",
  "p.timesheet.approved_notice": "La semana {week} de {person} está aprobada.",
  "p.timesheet.rejected_notice": "La semana {week} de {person} se devolvió.",
  "p.timesheet.other_not_submitted":
    "Esta semana no se ha enviado para aprobación, así que aún no hay nada que decidir.",
  "p.timesheet.other_rejected":
    "Esta semana se devolvió; podrá decidirse de nuevo cuando la persona vuelva a enviarla.",
  "p.timesheet.problem_own_week":
    "Es su propia semana: nadie decide sobre su propia hoja de horas.",
  "p.timesheet.problem_comment": "Diga por qué se devuelve la semana.",
  "p.timesheet.could_not_load_other":
    "No se pudo cargar la semana de esa persona.",
  "p.timesheet.could_not_decide": "No se pudo registrar la decisión.",
  "p.timesheet.refused_read_forbidden":
    "Solo la propia persona o quien aprueba hojas de horas puede leer esa semana.",
  "p.timesheet.refused_submit_forbidden":
    "Solo la persona a quien pertenece una semana puede enviarla para aprobación.",
  "p.timesheet.refused_decide_forbidden":
    "No puede decidir sobre esta semana: hace falta el derecho de aprobar hojas de horas, y nadie decide sobre su propia semana.",
  "p.timesheet.refused_submit_conflict":
    "Esta semana ya se envió para aprobación o ya está aprobada; se ha vuelto a leer.",
  "p.timesheet.refused_decide_conflict":
    "Esta semana ya no espera una decisión; se ha vuelto a leer.",
  "p.timesheet.refused_not_addressable":
    "Su identidad de inicio de sesión no tiene referencia de persona, así que no hay hoja de horas que mostrar.",
  "p.timesheet.refused_read_invalid":
    "El servicio no aceptó esa semana o esa persona.",
  "p.timesheet.refused_submit_invalid":
    "La semana no se envió: no tiene tiempo registrado.",
  "p.timesheet.refused_decide_invalid":
    "El servicio no aceptó la decisión: devolver una semana exige un motivo.",
  "p.security.access_link": "Revisión de accesos y acceso de emergencia",
  "p.security.access_note":
    "— quién tiene qué acceso, para la recertificación, y acceso de emergencia por tiempo limitado a un plan privado, que se registra y audita.",
  "p.access.title_ppm": "Revisión de accesos y acceso de emergencia — PPM",
  "p.access.back": "← Seguridad",
  "p.access.heading": "Revisión de accesos y acceso de emergencia",
  "p.access.intro":
    "La revisión de accesos enumera los accesos que se registraron como concedidos, para la recertificación. El acceso de emergencia le abre un plan privado durante un tiempo limitado cuando no hay otra forma; cada uso se registra y se audita.",
  "p.access.review_heading": "Revisión de accesos",
  "p.access.review_intro":
    "Los accesos registrados como concedidos en el registro de accesos y no revocados desde entonces, los más antiguos primero, con quién los concedió y cuándo. Muestra lo que se registró, no lo que aplica el servicio de inicio de sesión: compare ambos. Solo para administradores; cada lectura se registra en el registro de auditoría, por eso la revisión solo se lee cuando usted lo pide.",
  "p.access.f_review_plan":
    "Identificador del plan (déjelo en blanco para todos los planes que puede leer)",
  "p.access.f_review_days":
    "Pendiente de revisión tras este número de días naturales",
  "p.access.show_review": "Mostrar la revisión",
  "p.access.problem_review_plan":
    "Eso no es un identificador de plan: pegue el identificador del plan o deje la casilla en blanco para todos los planes.",
  "p.access.problem_review_days":
    "Escriba el número de días naturales como un número entero, 0 o más.",
  "p.access.could_not_load_review": "No se pudo cargar la revisión de accesos.",
  "p.access.review_totals":
    "A fecha de {as_of}. Accesos vigentes: {standing}. Pendientes de revisión, con {days} días naturales o más: {due}.",
  "p.access.review_note":
    "La antigüedad se cuenta en días naturales desde el día en que se registró el acceso hasta el día de hoy de la instalación.",
  "p.access.download_review":
    "Descargar la revisión como hoja de cálculo, con una columna para sus decisiones de mantener o revocar",
  "p.access.review_caption": "Accesos vigentes hoy, los más antiguos primero",
  "p.access.review_empty":
    "No hay accesos registrados como concedidos vigentes en los planes que puede leer.",
  "p.access.col_person": "Persona",
  "p.access.col_access": "Acceso",
  "p.access.col_plan": "Nombre del plan",
  "p.access.col_granted_on": "Concedido el",
  "p.access.col_granted_by": "Concedido por",
  "p.access.col_age": "Antigüedad en días naturales",
  "p.access.col_review": "Revisión",
  "p.access.col_note": "Nota",
  "p.access.due": "Pendiente de revisión",
  "p.access.not_due": "Aún no pendiente",
  "p.access.plan_unnamed": "Plan {id}… (nombre no mostrado)",
  "p.access.bg_heading":
    "Acceso de emergencia a un plan privado (romper el cristal)",
  "p.access.bg_audited_heading":
    "Cada uso del acceso de emergencia se registra y se audita.",
  "p.access.bg_audited_open":
    "Abrir un plan registra en el registro de auditoría quién es usted, el plan, su motivo, cuánto tiempo y la persona que nombra.",
  "p.access.bg_audited_told":
    "La persona que nombra recibe un aviso de inmediato. Si no se le puede avisar, el plan no se abre.",
  "p.access.bg_audited_each":
    "Cada solicitud que solo este acceso deja pasar se registra en el registro de auditoría como acceso de emergencia.",
  "p.access.bg_audited_visibility":
    "Le permite ver el plan. Lo que puede cambiar lo siguen decidiendo sus propios derechos.",
  "p.access.bg_audited_ends":
    "Termina por sí solo a la hora indicada, o antes si usted lo termina. Como máximo cinco al día, y uno a la vez en cada plan.",
  "p.access.bg_intro":
    "Úselo solo cuando no se pueda contactar con los miembros del plan y el trabajo no pueda esperar. Requiere la habilitación para romper el cristal o acceso de administrador; decide el servicio.",
  "p.access.problems_heading": "El plan no se abrió:",
  "p.access.problem_plan": "Indique el identificador del plan privado.",
  "p.access.problem_reason_short":
    "Diga por qué necesita acceso, en al menos {reason_min} caracteres.",
  "p.access.problem_reason_long":
    "El motivo tiene más de {reason_max} caracteres.",
  "p.access.problem_minutes":
    "Indique la duración en minutos enteros, de {minutes_min} a {minutes_max}.",
  "p.access.problem_notify":
    "Indique la persona a quien avisar como ‘person:’ o ‘worker:’ seguido de su identificador, sin nada más.",
  "p.access.problem_acknowledged":
    "Marque la casilla para indicar que entiende que esto se registra y audita y que se avisa a la persona.",
  "p.access.could_not_open": "No se pudo abrir el plan.",
  "p.access.opened_notice":
    "El plan está abierto para usted hasta {until} (su zona horaria, {tz}). Se ha avisado a {person}.",
  "p.access.go_to_plan": "Ir al plan",
  "p.access.f_plan": "Identificador del plan privado",
  "p.access.f_reason":
    "Por qué necesita acceso (obligatorio, al menos {min} caracteres)",
  "p.access.reason_count":
    "{n} caracteres; al menos {min} y como máximo {max}.",
  "p.access.f_minutes": "Durante cuánto tiempo, en minutos ({min} a {max})",
  "p.access.f_notify":
    "Persona a quien avisar: ‘person:’ o ‘worker:’ y su identificador (no usted)",
  "p.access.f_acknowledge":
    "Entiendo que esto se registra y audita, y que se avisará a la persona que nombro.",
  "p.access.open": "Abrir el plan con acceso de emergencia",
  "p.access.grants_heading": "Accesos de emergencia concedidos",
  "p.access.grants_intro":
    "Los suyos, los más recientes primero; un administrador ve los de todos. Un acceso nunca se borra. Las horas están en su zona horaria, {tz}.",
  "p.access.grants_caption":
    "Accesos de emergencia concedidos, los más recientes primero",
  "p.access.grants_empty": "No se ha abierto ningún acceso de emergencia.",
  "p.access.loading": "Cargando…",
  "p.access.could_not_load_grants":
    "No se pudieron cargar los accesos de emergencia.",
  "p.access.could_not_revoke": "No se pudo terminar el acceso de emergencia.",
  "p.access.revoked_notice": "El acceso de emergencia a {plan} ha terminado.",
  "p.access.revoke": "Terminar ahora",
  "p.access.revoke_for": "Terminar ahora: acceso de emergencia a {plan}",
  "p.access.state_live": "Vigente",
  "p.access.state_revoked": "Terminado antes de tiempo",
  "p.access.state_expired": "Caducado",
  "p.access.ended_by": "{when}, terminado por {who}",
  "p.access.col_state": "Estado",
  "p.access.col_holder": "Titular",
  "p.access.col_opened": "Abierto",
  "p.access.col_ends": "Termina o terminó",
  "p.access.col_reason": "Motivo indicado",
  "p.access.col_told": "Persona avisada",
  "p.access.col_act": "Acción",
  "p.access.refused_sign_in": "Inicie sesión primero.",
  "p.access.refused_review_forbidden":
    "La revisión de accesos es solo para administradores.",
  "p.access.refused_review_invalid":
    "El servicio no aceptó el plan o el número de días.",
  "p.access.refused_open_forbidden":
    "No puede usar el acceso de emergencia: requiere la habilitación para romper el cristal o acceso de administrador.",
  "p.access.refused_open_not_found":
    "No hay ningún plan con ese identificador.",
  "p.access.refused_open_conflict":
    "Ya tiene acceso de emergencia a este plan; la lista de abajo se ha vuelto a leer.",
  "p.access.refused_open_invalid":
    "El plan no se abrió. O no es privado, así que no hay nada que abrir, o la persona a quien avisar es usted, o no se puede avisar a esa persona. No se concedió acceso.",
  "p.access.refused_open_limit":
    "Ha abierto el máximo de accesos de emergencia permitidos en un día (cinco, contados en las últimas 24 horas).",
  "p.access.refused_revoke_not_found":
    "No se encontró ese acceso, o no le corresponde terminarlo.",
  "p.deescalation.nav": "Cuadros de mando y desescalada",
  "p.deescalation.title": "Cuadros de mando y desescalada — PPM",
  "p.deescalation.h1": "Cuadros de mando y desescalada",
  "p.deescalation.lede":
    "El nivel de escalada tal como lo fijó el patrocinador, los criterios para bajarlo con su evidencia en cada trimestre y los cuadros de mando cuyos indicadores clave se leen por trimestre terminado.",
  "p.deescalation.tracker_heading": "El seguimiento de la desescalada",
  "p.deescalation.tracker_says":
    "El nivel se registra tal como lo fijó el patrocinador. Nunca se calcula aquí y nada en esta página lo mueve: la racha de trimestres cumplidos se ofrece a la persona que decide.",
  "p.deescalation.loading": "Cargando…",
  "p.deescalation.could_not_load":
    "No se pudo cargar el seguimiento de la desescalada.",
  "p.deescalation.could_not_save": "El cambio no se guardó.",
  "p.deescalation.level_heading": "Nivel de escalada",
  "p.deescalation.level_now": "Nivel {level}, fijado el {date}.",
  "p.deescalation.level_note": "Nota del patrocinador: {note}",
  "p.deescalation.level_none":
    "Todavía no se ha registrado ningún nivel de escalada.",
  "p.deescalation.level_history": "Niveles anteriores",
  "p.deescalation.level_history_item": "{level}, fijado el {date}",
  "p.deescalation.run_heading": "Trimestres cumplidos seguidos",
  "p.deescalation.run_one":
    "{n} trimestre terminado cumplido seguido, de los {need} que pide la regla.",
  "p.deescalation.run_other":
    "{n} trimestres terminados cumplidos seguidos, de los {need} que pide la regla.",
  "p.deescalation.rule_satisfied":
    "La regla se cumple. Le corresponde al patrocinador valorarlo; el nivel no cambia por sí solo.",
  "p.deescalation.rule_not_satisfied": "La regla todavía no se cumple.",
  "p.deescalation.setting_unusable":
    "No se pudo usar el ajuste de la instalación sobre cuántos trimestres pide la regla, así que se aplica el valor por defecto de {n}.",
  "p.deescalation.reads_scorecard":
    "El seguimiento lee el cuadro de mando {name}.",
  "p.deescalation.reads_no_scorecard":
    "Ningún cuadro de mando es el que lee el seguimiento, así que solo cuentan los criterios. Marque uno como tal al añadirlo, más abajo.",
  "p.deescalation.hidden_kpis_one":
    "{n} indicador clave de este cuadro de mando pertenece a un plan que usted no puede leer: aquí no se muestra ni se cuenta.",
  "p.deescalation.hidden_kpis_other":
    "{n} indicadores clave de este cuadro de mando pertenecen a planes que usted no puede leer: aquí no se muestran ni se cuentan.",
  "p.deescalation.quarters_label": "Cada trimestre terminado",
  "p.deescalation.col_quarter": "Trimestre",
  "p.deescalation.col_state": "Situación",
  "p.deescalation.col_reasons": "Por qué",
  "p.deescalation.state_met": "Cumplido",
  "p.deescalation.state_not_met": "No cumplido",
  "p.deescalation.state_missing": "Nada registrado",
  "p.deescalation.state_with_value": "{state} (valor {value})",
  "p.deescalation.reason_all_met":
    "Se cumplieron todos los indicadores clave y todos los criterios.",
  "p.deescalation.reason_nothing":
    "No hay nada que medir: ningún indicador clave y ningún criterio.",
  "p.deescalation.reason_no_kpis":
    "El cuadro de mando no tiene ningún indicador clave que usted pueda leer.",
  "p.deescalation.reason_not_met": "No cumplido: {list}.",
  "p.deescalation.reason_missing": "Nada registrado para: {list}.",
  "p.deescalation.quarter": "T{q} {year}",
  "p.deescalation.quarter_standing": "{quarter} · {state}. {reasons}",
  "p.deescalation.tracker_kpis_label":
    "El cuadro de mando del seguimiento, por trimestre",
  "p.deescalation.set_level_heading": "Registrar el nivel de escalada",
  "p.deescalation.set_level_says":
    "Solo el patrocinador (alguien con el rol sponsor) o un administrador puede registrar el nivel, tal como lo ha fijado. Un nivel tiene de {min} a {max} caracteres; la nota es opcional.",
  "p.deescalation.f_level": "Nivel",
  "p.deescalation.f_level_note": "Nota (opcional)",
  "p.deescalation.save_level": "Registrar el nivel",
  "p.deescalation.level_saved": "Nivel {level} registrado.",
  "p.deescalation.unchanged": "No cambió nada, así que no se envió nada.",
  "p.deescalation.problem_level": "Un nivel tiene de {min} a {max} caracteres.",
  "p.deescalation.refused_level_forbidden":
    "Rechazado: solo el patrocinador o un administrador puede registrar el nivel de escalada.",
  "p.deescalation.criteria_heading": "Criterios para bajar",
  "p.deescalation.criteria_says":
    "Un criterio se añade y nunca se edita ni se retira. Su evidencia se registra para un trimestre terminado, y cuenta el registro más reciente de ese trimestre.",
  "p.deescalation.criteria_none": "Todavía no hay criterios.",
  "p.deescalation.evidence_of": "Evidencia de: {criterion}",
  "p.deescalation.evidence_link": "Evidencia",
  "p.deescalation.add_criterion_heading": "Añadir un criterio",
  "p.deescalation.f_criterion": "Criterio (de {min} a {max} caracteres)",
  "p.deescalation.add_criterion": "Añadir el criterio",
  "p.deescalation.criterion_added": "Criterio añadido.",
  "p.deescalation.problem_criterion":
    "Un criterio tiene de {min} a {max} caracteres.",
  "p.deescalation.evidence_heading": "Registrar evidencia",
  "p.deescalation.evidence_says":
    "Elija un criterio y un trimestre terminado (un trimestre en curso no se puede registrar), indique si se cumplió y escriba una nota de al menos {min} caracteres. Un enlace es opcional: una dirección http o https sin espacios, de hasta {max} caracteres.",
  "p.deescalation.f_ev_criterion": "Criterio",
  "p.deescalation.f_ev_quarter": "Trimestre terminado",
  "p.deescalation.f_ev_met": "¿Se cumplió?",
  "p.deescalation.f_ev_note": "Nota (al menos {min} caracteres)",
  "p.deescalation.f_ev_link": "Enlace a la evidencia (opcional)",
  "p.deescalation.save_evidence": "Registrar la evidencia",
  "p.deescalation.evidence_saved": "Evidencia registrada para {quarter}.",
  "p.deescalation.problem_criterion_choose":
    "Añada primero un criterio y después registre su evidencia.",
  "p.deescalation.problem_quarter": "Elija un trimestre terminado.",
  "p.deescalation.problem_quarter_unfinished":
    "Ese trimestre no ha terminado, así que todavía no se puede registrar.",
  "p.deescalation.problem_met": "Indique si se cumplió el criterio.",
  "p.deescalation.problem_note":
    "Hace falta una nota de al menos {min} caracteres.",
  "p.deescalation.problem_link":
    "Un enlace es una dirección http o https sin espacios, de hasta {max} caracteres.",
  "p.deescalation.refused_evidence":
    "Rechazado: el trimestre debe haber terminado, la nota debe tener al menos cinco caracteres y un enlace debe ser una dirección http o https sin espacios.",
  "p.deescalation.refused_gone":
    "Ese registro ya no existe. Vuelva a cargar la página.",
  "p.deescalation.scorecards_heading": "Cuadros de mando",
  "p.deescalation.scorecards_says":
    "Un cuadro de mando es un conjunto con nombre de indicadores clave, leído por trimestre terminado. Un indicador clave se cumple en un trimestre cuando su lectura más reciente en ese trimestre alcanza o mejora su objetivo; un trimestre sin lectura falta, lo que no es lo mismo que no cumplido.",
  "p.deescalation.scorecards_none": "Todavía no hay cuadros de mando.",
  "p.deescalation.could_not_load_scorecards":
    "No se pudieron cargar los cuadros de mando.",
  "p.deescalation.col_scorecard": "Cuadro de mando",
  "p.deescalation.col_tracker": "Lo lee el seguimiento",
  "p.deescalation.is_tracker": "Sí, el que lee el seguimiento",
  "p.deescalation.not_tracker": "No lo lee",
  "p.deescalation.add_scorecard_heading": "Añadir un cuadro de mando",
  "p.deescalation.add_scorecard_says":
    "Un nombre de {min} a {max} caracteres que no esté ya en uso (sin distinguir mayúsculas). Como mucho un cuadro de mando es el que lee el seguimiento.",
  "p.deescalation.f_name": "Nombre",
  "p.deescalation.f_for_tracker": "El que lee el seguimiento",
  "p.deescalation.add_scorecard": "Añadir el cuadro de mando",
  "p.deescalation.scorecard_added": "Cuadro de mando {name} añadido.",
  "p.deescalation.problem_name":
    "El nombre de un cuadro de mando tiene de {min} a {max} caracteres.",
  "p.deescalation.problem_name_taken":
    "Ya hay un cuadro de mando con ese nombre.",
  "p.deescalation.problem_tracker_taken":
    "El cuadro de mando {name} ya es el que lee el seguimiento.",
  "p.deescalation.scorecard_heading": "Un cuadro de mando por trimestre",
  "p.deescalation.f_scorecard": "Cuadro de mando",
  "p.deescalation.choose_scorecard": "Elija un cuadro de mando",
  "p.deescalation.could_not_load_scorecard":
    "No se pudo cargar el cuadro de mando.",
  "p.deescalation.scorecard_says":
    "Cada indicador clave por trimestre terminado: cumplido, no cumplido o nada registrado.",
  "p.deescalation.scorecard_kpis_label":
    "Indicadores clave de {name}, por trimestre",
  "p.deescalation.scorecard_quarters_label":
    "Cada trimestre del cuadro de mando",
  "p.deescalation.col_kpi": "Indicador clave",
  "p.deescalation.col_target": "Objetivo",
  "p.deescalation.target_at_least": "al menos {value} {unit}",
  "p.deescalation.target_at_most": "como mucho {value} {unit}",
  "p.deescalation.no_kpis":
    "Este cuadro de mando no tiene indicadores clave que usted pueda leer.",
  "p.deescalation.unit_percent": "por ciento",
  "p.deescalation.unit_count": "un recuento",
  "p.deescalation.kpis_heading": "Elegir sus indicadores clave (KPI)",
  "p.deescalation.kpis_says":
    "Marque los indicadores clave que tiene este cuadro de mando, como mucho {max}. Guardar sustituye el conjunto; uno de un plan que usted no puede leer se queda en el cuadro.",
  "p.deescalation.f_kpis": "Indicadores clave del cuadro de mando",
  "p.deescalation.kpis_none":
    "No hay indicadores clave que elegir. Añádalos en la página de indicadores clave o desde la plantilla de abajo.",
  "p.deescalation.save_kpis": "Guardar los indicadores clave",
  "p.deescalation.kpis_saved_one":
    "El cuadro de mando tiene ahora {n} indicador clave.",
  "p.deescalation.kpis_saved_other":
    "El cuadro de mando tiene ahora {n} indicadores clave.",
  "p.deescalation.problem_kpis_max":
    "Un cuadro de mando tiene como mucho {max} indicadores clave.",
  "p.deescalation.refused_kpis":
    "Rechazado: cada uno debe ser un indicador clave que usted pueda leer, y un cuadro de mando tiene como mucho 60.",
  "p.deescalation.template_heading": "Añadir medidas desde la plantilla",
  "p.deescalation.template_says":
    "Un conjunto inicial, ofrecido y no aplicado. No tiene objetivos: usted fija cada objetivo y cada línea roja, y las medidas que elija pasan a ser indicadores clave de este cuadro de mando.",
  "p.deescalation.template_rule":
    "Donde más es mejor, la línea roja debe estar por debajo del objetivo; donde menos es mejor, por encima. Los objetivos y las líneas rojas son números enteros.",
  "p.deescalation.could_not_load_template": "No se pudo cargar la plantilla.",
  "p.deescalation.higher_better": "Más es mejor",
  "p.deescalation.lower_better": "Menos es mejor",
  "p.deescalation.unit": "medido como {unit}",
  "p.deescalation.f_pick": "Añadir {measure}",
  "p.deescalation.f_target": "Objetivo",
  "p.deescalation.f_red": "Línea roja",
  "p.deescalation.add_measures": "Añadir las medidas",
  "p.deescalation.measures_added_one": "{n} medida añadida al cuadro de mando.",
  "p.deescalation.measures_added_other":
    "{n} medidas añadidas al cuadro de mando.",
  "p.deescalation.problem_pick_one": "Elija al menos una medida.",
  "p.deescalation.problem_whole":
    "{measure}: el objetivo y la línea roja son números enteros.",
  "p.deescalation.problem_red_higher":
    "{measure}: más es mejor, así que la línea roja debe estar por debajo del objetivo.",
  "p.deescalation.problem_red_lower":
    "{measure}: menos es mejor, así que la línea roja debe estar por encima del objetivo.",
  "p.deescalation.problem_not_in_template":
    "{measure} no está en la plantilla.",
  "p.deescalation.refused_template":
    "Rechazado: compruebe cada objetivo y cada línea roja con la regla de dirección.",
  "p.deescalation.measure_programmes_with_independent_assurance":
    "Programas con aseguramiento independiente en el último trimestre",
  "p.deescalation.measure_milestones_met_on_time":
    "Hitos cumplidos en su fecha o antes",
  "p.deescalation.measure_benefits_validated":
    "Beneficios previstos validados por alguien ajeno al programa",
  "p.deescalation.measure_red_without_decision":
    "Programas en rojo sin decisión registrada",
  "p.deescalation.measure_open_critical_issues":
    "Incidencias abiertas de gravedad crítica",
  "p.deescalation.measure_change_requests_waiting":
    "Solicitudes de cambio que esperan más de 30 días una decisión",
  "p.phase.name_initiating": "Inicio",
  "p.phase.name_planning": "Planificación",
  "p.phase.name_executing": "Ejecución",
  "p.phase.name_controlling": "Control",
  "p.phase.name_closing": "Cierre",
  "p.phase.heading": "Fase",
  "p.phase.heading_programme": "Fase e inicio",
  "p.phase.says":
    "Un plan avanza una fase cada vez; puede volver a una fase anterior con un motivo, que se registra con el cambio. La fase nunca impide registrar trabajo.",
  "p.phase.could_not_load": "No se pudo cargar la fase del plan",
  "p.phase.now": "El plan está en la fase {phase}.",
  "p.phase.none": "Aún no hay fase registrada: el primer cambio es a Inicio.",
  "p.phase.time_heading": "Tiempo en cada fase",
  "p.phase.days_in_one": "{phase}: {days} día",
  "p.phase.days_in_other": "{phase}: {days} días",
  "p.phase.visits_one": "con {visits} entrada",
  "p.phase.visits_other": "con {visits} entradas",
  "p.phase.current": "actual",
  "p.phase.check_heading": "Comprobación de inicio",
  "p.phase.check_says":
    "Un programa empieza cuando su fase pasa a Ejecución. El inicio se rechaza cuando todas las plazas para programas en ejecución activa están ocupadas (los programas en pausa o detenidos no ocupan ninguna) o cuando el grupo restrictivo tiene falta de oferta en alguno de los seis meses siguientes. Un programa rechazado entra en la cola de inicio y conserva su puesto; puede empezar igualmente con un motivo de excepción de {min} o más caracteres, que se registra y audita. Leer la comprobación no mueve nada.",
  "p.phase.could_not_check": "No se pudo leer la comprobación de inicio",
  "p.phase.places":
    "Hay {in_delivery} de {limit} plazas ocupadas para programas en ejecución activa.",
  "p.phase.no_limit":
    "No hay límite fijado para programas en ejecución activa.",
  "p.phase.limit_unusable":
    "No se pudo usar el límite de la instalación para programas en ejecución, así que no se aplica ninguno.",
  "p.phase.pool_is": "El grupo restrictivo es {pool}.",
  "p.phase.no_pool": "No se ha identificado ningún grupo restrictivo.",
  "p.phase.would_refuse": "Un inicio ahora se rechazaría:",
  "p.phase.would_start": "Un inicio ahora seguiría adelante.",
  "p.phase.in_queue": "Este programa es el número {n} de la cola de inicio.",
  "p.phase.see_queue": "Ver la cola de inicio",
  "p.phase.refusal_limit":
    "Las {limit} plazas para programas en ejecución activa están ocupadas ({in_delivery} en ejecución).",
  "p.phase.refusal_pool":
    "El grupo restrictivo {pool} tiene falta de oferta en {months}.",
  "p.phase.demand_heading": "Demanda firme de este programa",
  "p.phase.demand_caption":
    "Equivalentes a tiempo completo por grupo de competencias y mes, de asignaciones firmes en este programa y los planes que dependen de él",
  "p.phase.no_demand":
    "No hay demanda firme de este programa en un grupo de competencias en los próximos seis meses.",
  "p.phase.col_pool": "Grupo de competencias",
  "p.phase.check_again": "Comprobar de nuevo",
  "p.phase.move_heading": "Cambiar la fase",
  "p.phase.f_to": "Pasar a",
  "p.phase.forward_to": "Avanzar a {phase}",
  "p.phase.back_to": "Volver a {phase}",
  "p.phase.back_needs_reason": "Volver atrás necesita un motivo.",
  "p.phase.f_reason":
    "Motivo (necesario para volver atrás; se registra con el cambio)",
  "p.phase.start_may_queue":
    "Pasar a Ejecución es el inicio de este programa: si al enviarlo la comprobación no encuentra sitio, el inicio se rechaza y el programa entra en la cola de inicio.",
  "p.phase.start_would_queue":
    "Este inicio se rechazaría. Si se envía sin motivo de excepción, el programa entra en la cola de inicio (o conserva su puesto en ella).",
  "p.phase.f_override":
    "Motivo de excepción, para empezar igualmente ({min} o más caracteres; se registra y audita)",
  "p.phase.refused_why": "Por qué se rechazó el inicio",
  "p.phase.move": "Mover la fase",
  "p.phase.moved": "El plan está ahora en la fase {phase}.",
  "p.phase.started_override":
    "El programa empezó con una excepción: ahora está en la fase {phase} y la excepción queda registrada.",
  "p.phase.start_refused_place":
    "Se rechazó el inicio, así que el programa es el número {n} de la cola de inicio. Para empezarlo igualmente, indique un motivo de excepción.",
  "p.phase.start_refused":
    "Se rechazó el inicio y el programa entró en la cola de inicio. Para empezarlo igualmente, indique un motivo de excepción.",
  "p.phase.could_not_move": "No se pudo mover la fase",
  "p.phase.problem_choose": "Elija la fase a la que pasar.",
  "p.phase.problem_same":
    "El plan ya está en esa fase, así que no se envió nada.",
  "p.phase.problem_skip": "Se avanza una fase cada vez.",
  "p.phase.problem_reason": "Para volver atrás hace falta un motivo.",
  "p.phase.problem_override":
    "Un motivo de excepción tiene {min} o más caracteres.",
  "p.phase.refused_move":
    "Rechazado: se avanza una fase cada vez, volver atrás necesita un motivo y el plan no debe estar ya en esa fase.",
  "p.phase.refused_gone": "Ese plan ya no existe.",
  "p.proposals.act_submit": "Enviar",
  "p.proposals.act_review": "Iniciar revisión",
  "p.proposals.act_approve": "Aprobar",
  "p.proposals.act_reject": "Rechazar",
  "p.proposals.act_submit_named": "Enviar {title}",
  "p.proposals.act_review_named": "Iniciar revisión de {title}",
  "p.proposals.act_approve_named": "Aprobar {title}",
  "p.proposals.act_reject_named": "Rechazar {title}",
  "p.proposals.ask_rule":
    "Una petición externa del patrocinador debe indicar qué desplaza (una contrapartida de {min} o más caracteres) antes de poder aprobarse.",
  "p.proposals.new_form": "Nueva propuesta",
  "p.proposals.f_external_ask": "Una petición externa del patrocinador",
  "p.proposals.f_new_tradeoff":
    "Qué desplaza (la contrapartida, de {min} a {max} caracteres; puede indicarse más tarde, antes de aprobar)",
  "p.proposals.col_tradeoff": "Petición externa y contrapartida",
  "p.proposals.not_ask": "No es una petición externa",
  "p.proposals.ask_with": "Petición externa. Contrapartida: {tradeoff}",
  "p.proposals.ask_without":
    "Petición externa. Aún no hay contrapartida indicada.",
  "p.proposals.tradeoff_name": "Indicar la contrapartida",
  "p.proposals.tradeoff_change": "Cambiar la contrapartida",
  "p.proposals.tradeoff_name_named": "Indicar la contrapartida de {title}",
  "p.proposals.tradeoff_change_named": "Cambiar la contrapartida de {title}",
  "p.proposals.tradeoff_heading": "Contrapartida de {title}",
  "p.proposals.tradeoff_says":
    "Indique qué desplaza esta petición, en {min} a {max} caracteres. Puede indicarse o cambiarse hasta que la propuesta se apruebe o rechace, y cada cambio queda registrado.",
  "p.proposals.f_tradeoff": "Qué desplaza (de {min} a {max} caracteres)",
  "p.proposals.tradeoff_save": "Guardar la contrapartida",
  "p.proposals.tradeoff_close": "Cerrar el formulario de contrapartida",
  "p.proposals.tradeoff_unchanged": "No cambió nada, así que no se envió nada.",
  "p.proposals.tradeoff_saved": "Contrapartida guardada para {title}.",
  "p.proposals.problem_title": "Una propuesta necesita un título.",
  "p.proposals.problem_tradeoff":
    "Una contrapartida tiene de {min} a {max} caracteres.",
  "p.proposals.problem_not_ask":
    "Solo una petición externa tiene contrapartida.",
  "p.proposals.problem_decided":
    "La propuesta ya está decidida, así que su contrapartida ya no puede cambiar.",
  "p.proposals.problem_approve":
    "{title} es una petición externa: indique su contrapartida ({min} o más caracteres) antes de aprobarla.",
  "p.proposals.refused_create":
    "Rechazado: una propuesta necesita un título, un importe necesita una moneda y una contrapartida tiene como máximo {max} caracteres.",
  "p.proposals.refused_tradeoff":
    "Rechazado: una contrapartida tiene de {min} a {max} caracteres, en una petición externa aún no decidida.",
  "p.proposals.refused_step":
    "Rechazado: ese paso no corresponde al estado de la propuesta.",
  "p.proposals.refused_gone": "Esa propuesta ya no existe.",
  "p.thresholds.opened_by_kind":
    "Abierta porque el programa cruzó su umbral: {kind}.",
  "p.thresholds.joined_by_kind":
    "Mientras estaba abierta, el programa también cruzó un umbral: {kind}.",
  "p.deescalation.refused_tracker_taken":
    "Rechazado: otro cuadro de mando ya es el que lee el seguimiento.",
  "p.deescalation.level_now_by": "Nivel {level}, fijado el {date} por {who}.",
  "p.deescalation.level_history_item_by": "{level}, fijado el {date} por {who}",
  "p.deescalation.evidence_recorded_by": "registrada por {who} el {date}",
  "p.deescalation.evidence_recorded_on": "registrada el {date}",
  "p.proposals.refused_approve_status":
    "Rechazado: solo se puede aprobar una propuesta en revisión.",
  "p.proposals.refused_tradeoff_required":
    "Rechazado: una petición externa necesita una contrapartida indicada de {min} o más caracteres antes de poder aprobarse.",
  "p.ledger.untraced_no_readable_line":
    "Ninguna línea presupuestaria que pueda leer usa esta asignación, así que aquí no se puede rastrear adónde va su dinero.",
  "p.ledger.untraced_not_on_a_line":
    "Parte de esta asignación aún no está planificada en ninguna línea presupuestaria.",
  "p.ledger.untraced_lines_over_plan":
    "Las líneas presupuestarias planifican más de lo que contiene esta asignación.",
  "p.ledger.untraced_other":
    "Parte del dinero de esta asignación no se puede rastrear.",
  "p.status.title_ppm": "Actualizaciones de estado — PPM",
  "p.status.title": "Actualizaciones de estado",
  "p.status.link": "Actualizaciones de estado y su evidencia",
  "p.status.intro":
    "Una actualización de estado es la valoración de una persona sobre cómo va el plan (en curso, en riesgo o desviado), con lo que ocurre en palabras. Aquí no se calcula ninguna puntuación.",
  "p.status.could_not_load": "No se pudo cargar el estado del plan",
  "p.status.now_heading": "Dónde está el plan",
  "p.status.no_update":
    "Aún no se ha publicado ninguna actualización de estado.",
  "p.status.cadence_is_one": "Se espera una actualización cada día.",
  "p.status.cadence_is_other": "Se espera una actualización cada {n} días.",
  "p.status.standing_overdue_one":
    "La próxima actualización vencía el {date}: lleva {n} día de retraso.",
  "p.status.standing_overdue_other":
    "La próxima actualización vencía el {date}: lleva {n} días de retraso.",
  "p.status.standing_due": "Hoy, {date}, vence una actualización.",
  "p.status.standing_next": "La próxima actualización vence el {date}.",
  "p.status.no_cadence":
    "No hay cadencia fijada, así que ninguna actualización vence ni se retrasa.",
  "p.status.as_of": "A fecha de {date}, en la zona horaria de la instalación.",
  "p.status.loading": "Cargando…",
  "p.status.posted_on_by": "publicada el {date} por {who}",
  "p.status.posted_on": "publicada el {date}",
  "p.status.claims_of": "Afirmaciones de la actualización del {date}",
  "p.status.source_metric": "{metric}: {value} ahora",
  "p.status.source_metric_unknown": "{metric}: desconocido (no cero)",
  "p.status.source_unavailable": "Una fuente que ya no está disponible",
  "p.status.kind_task": "Tarea: {title}",
  "p.status.kind_risk": "Riesgo: {title}",
  "p.status.kind_milestone": "Hito: {title}",
  "p.status.kind_issue": "Incidencia: {title}",
  "p.status.kind_other": "Registro: {title}",
  "p.status.metric_overdue": "Tareas vencidas",
  "p.status.metric_due_soon": "Tareas que vencen pronto",
  "p.status.metric_blocked": "Tareas bloqueadas",
  "p.status.metric_no_due_date": "Tareas sin fecha de vencimiento",
  "p.status.metric_def_overdue":
    "Tareas abiertas cuya fecha de vencimiento es anterior a hoy. Una tarea que vence hoy no está vencida. Desconocido cuando hay tareas abiertas y ninguna tiene fecha de vencimiento.",
  "p.status.metric_def_due_soon":
    "Tareas abiertas que vencen hoy o en los próximos {days} días. Desconocido cuando hay tareas abiertas y ninguna tiene fecha de vencimiento.",
  "p.status.metric_def_blocked": "Tareas abiertas cuyo estado es bloqueado.",
  "p.status.metric_def_no_due_date":
    "Tareas abiertas sin fecha de vencimiento: las que las demás cifras no pueden ver.",
  "p.status.metrics_heading": "Cómo se cuentan las cifras",
  "p.status.metrics_says":
    "Sobre las tareas abiertas del plan (una tarea cuyo estado no es terminado), contadas al leer una actualización, no al publicarla.",
  "p.status.cadence_heading": "Con qué frecuencia se espera una actualización",
  "p.status.cadence_says":
    "Cada {min} a {max} días. Fijarla reinicia el reloj desde hoy, así que la misma cadencia no se vuelve a enviar. Una cadencia no se puede quitar una vez fijada.",
  "p.status.cadence_set_by": "Fijada el {date} por {who}.",
  "p.status.cadence_set_on": "Fijada el {date}.",
  "p.status.f_cadence": "Días entre actualizaciones ({min} a {max})",
  "p.status.save_cadence": "Guardar la cadencia",
  "p.status.unchanged": "No cambió nada, así que no se envió nada.",
  "p.status.problem_cadence":
    "Una cadencia es un número entero de días de {min} a {max}.",
  "p.status.cadence_saved_one":
    "Ahora se espera una actualización cada día; el reloj empieza hoy.",
  "p.status.cadence_saved_other":
    "Ahora se espera una actualización cada {n} días; el reloj empieza hoy.",
  "p.status.could_not_save": "No se pudo guardar la cadencia",
  "p.status.refused_gone": "Ese plan ya no existe.",
  "p.status.history_heading":
    "Todas las actualizaciones, las más recientes primero",
  "p.status.history_says":
    "Las actualizaciones nunca se cambian ni se quitan. Cada fuente se muestra como está ahora: un registro que ya no existe no está disponible, y cada cifra tiene su valor de hoy.",
  "p.status.history_none": "Aún no hay actualizaciones.",
  "p.status.post_heading": "Publicar una actualización",
  "p.status.post_says":
    "Elija la valoración y diga qué está ocurriendo (hasta {max} caracteres). Una actualización no se puede cambiar ni quitar una vez publicada, y publicar una reinicia el reloj de la cadencia.",
  "p.status.claims_rule":
    "Puede añadir hasta {claims} afirmaciones, cada una una frase de hasta {chars} caracteres que cite hasta {sources} fuentes: las cifras de abajo, o las tareas, riesgos, hitos e incidencias del plan. Cada fuente se comprueba al publicar la actualización.",
  "p.status.f_call": "La valoración",
  "p.status.call_on_track": "En curso",
  "p.status.call_at_risk": "En riesgo",
  "p.status.call_off_track": "Desviado",
  "p.status.f_narrative": "Qué está ocurriendo (hasta {max} caracteres)",
  "p.status.claim_legend": "Afirmación {n}",
  "p.status.f_claim_text": "La afirmación (hasta {chars} caracteres)",
  "p.status.f_metrics": "Cifras que cita",
  "p.status.f_record": "Un registro del plan que cita la afirmación {n}",
  "p.status.choose_record": "Elija un registro",
  "p.status.add_record": "Añadir el registro a la afirmación {n}",
  "p.status.claim_records": "Registros que cita la afirmación {n}",
  "p.status.remove": "Quitar",
  "p.status.remove_source": "Quitar {source} de la afirmación {n}",
  "p.status.remove_claim": "Quitar la afirmación {n}",
  "p.status.add_claim": "Añadir una afirmación",
  "p.status.post": "Publicar la actualización",
  "p.status.posted": "Actualización publicada: {call}.",
  "p.status.could_not_post": "No se pudo publicar la actualización",
  "p.status.problem_call": "Elija en curso, en riesgo o desviado.",
  "p.status.problem_narrative":
    "Diga qué está ocurriendo, en hasta {max} caracteres.",
  "p.status.problem_claims":
    "Una actualización lleva como máximo {claims} afirmaciones.",
  "p.status.problem_claim_text":
    "Una afirmación es una frase de hasta {chars} caracteres.",
  "p.status.problem_sources":
    "Una afirmación cita como máximo {sources} fuentes.",
  "p.status.problem_in_claim": "Afirmación {n}: {problem}",
  "p.status.refused_update":
    "Rechazado: puede que una fuente ya no sea un registro de este plan. No se publicó nada; revise las afirmaciones y publique de nuevo.",
  "p.status.due_heading": "Actualizaciones de estado pendientes",
  "p.status.due_none":
    "Ningún plan que pueda leer tiene una actualización de estado pendiente.",
  "p.status.due_could_not_load":
    "No se pudieron cargar las actualizaciones de estado pendientes",
  "p.status.a_plan": "Un plan",
  "p.status.due_last": "Última actualización: {call}, el {date}.",
  "p.status.due_never": "Aún sin actualización.",
  "p.status.group_task": "Tareas",
  "p.status.group_risk": "Riesgos",
  "p.status.group_milestone": "Hitos",
  "p.status.group_issue": "Incidencias",
  "l.enum.risk_status.open": "Abierto",
  "l.enum.risk_status.mitigating": "En mitigación",
  "l.enum.risk_status.closed": "Cerrado",
  "l.enum.risk_status.materialized": "Materializado",
  "p.risk_edit.change": "Cambiar",
  "p.risk_edit.change_named": "Cambiar: {title}",
  "p.risk_edit.heading": "Cambiar el riesgo «{title}»",
  "p.risk_edit.says":
    "La probabilidad y el impacto son números enteros de {min} a {max}; la exposición es su producto. La mitigación tiene como máximo {max_bytes} bytes (menos caracteres en algunas escrituras). Una fecha de revisión puede cambiarse, pero no quitarse. Solo se envía lo que cambie.",
  "p.risk_edit.says_escalate":
    "Para registrar que un riesgo se ha producido, use Escalar en la tabla, que también registra cuándo.",
  "p.risk_edit.problem_status": "Elija un estado.",
  "p.risk_edit.problem_score":
    "La probabilidad y el impacto deben ser cada uno un número entero de {min} a {max}.",
  "p.risk_edit.problem_mitigation":
    "La mitigación supera los {max_bytes} bytes.",
  "p.risk_edit.problem_review_cleared":
    "Una fecha de revisión no puede quitarse una vez fijada; elija otra fecha.",
  "p.risk_edit.problem_review_date": "La fecha de revisión no es una fecha.",
  "p.risk_edit.unchanged": "No cambió nada, así que no se envió nada.",
  "p.risk_edit.saved": "Se guardó el riesgo «{title}».",
  "p.risk_edit.could_not_save": "No se pudo guardar el riesgo",
  "p.risk_edit.refused_gone":
    "Ese riesgo ya no está en este plan; se volvió a leer la lista.",
  "p.risk_edit.refused_invalid":
    "Rechazado: el servicio no aceptó el estado, las puntuaciones ni la mitigación. No se cambió nada.",
  "p.risk_edit.f_status": "Estado del riesgo",
  "p.risk_edit.f_probability": "Probabilidad ({min} a {max})",
  "p.risk_edit.f_impact": "Impacto ({min} a {max})",
  "p.risk_edit.f_mitigation": "Mitigación",
  "p.risk_edit.f_review_date": "Fecha de revisión",
  "p.risk_edit.save": "Guardar el riesgo",
  "p.risk_edit.close": "Cerrar el formulario",
  "p.plan_issues.refused_gone":
    "Esa incidencia ya no está en este plan; se volvió a leer la lista.",
  "p.plan_issues.refused_remove":
    "Rechazado: no puede quitar incidencias de este plan. No se quitó nada.",
  "p.plan_issues.refused_remove_admin":
    "Rechazado: solo un administrador puede quitar una incidencia bajo el flujo de trabajo de este tipo. No se quitó nada.",
  "p.plan_issues.remove_admin_only":
    "Solo un administrador puede quitar «{title}»: el flujo de trabajo de su tipo controla sus cambios de estado o sus campos. No se envió nada.",
  "p.plan_issues.confirm_remove":
    "¿Quitar la incidencia «{title}»? Desaparece de todas las listas; su registro se conserva para la auditoría.",
  "p.plan_issues.removed": "Se quitó la incidencia «{title}».",
  "p.plan_issues.remove": "Quitar",
  "p.plan_issues.remove_named": "Quitar: {title}",
  "p.plan_issues.remove_says":
    "Quitar una incidencia la saca de todas las listas y no se puede deshacer aquí; la auditoría guarda un registro. Bajo un flujo de trabajo que controla cambios o campos, solo un administrador puede quitarla.",
  "p.plan_issues.raised_on_by": "Registrada el {date} por {who}.",
  "p.plan_issues.raised_on": "Registrada el {date}.",
  "p.plan_issues.from_risk_named": "Registrada a partir del riesgo «{title}».",
  "p.plan_issues.from_a_risk":
    "Registrada a partir de un riesgo que ya no figura.",
  "p.plan_issues.from_a_task":
    "Registrada a partir de una tarea bloqueada (abrir la tarea)",
  "p.plan_issues.answered_by_change":
    "Respondida por una solicitud de cambio (ver las solicitudes de cambio del plan)",
  "p.plan_issues.resolved_on": "Resuelta el {date}.",
  "p.task.issue_heading": "Una incidencia a partir de esta tarea",
  "p.task.issue_says":
    "Solo una tarea bloqueada puede registrarse como incidencia, y solo una vez mientras esa incidencia siga abierta. La incidencia es un bloqueo de gravedad media, cuyo responsable es el asignado de la tarea; la tarea en sí no cambia.",
  "p.task.issue_raised": "Se registró la incidencia.",
  "p.task.issue_open_list": "Ver las incidencias del plan",
  "p.task.issue_exists":
    "Desde esta tarea se registró una incidencia abierta: «{title}».",
  "p.task.issue_not_blocked":
    "Esta tarea no está bloqueada, así que no puede registrarse como incidencia. Márquela como bloqueada en el tablero primero.",
  "p.task.issue_raise": "Registrar una incidencia desde esta tarea",
  "p.task.refused_from_task_not_blocked":
    "Rechazado: solo una tarea bloqueada puede registrarse como incidencia, y esta ya no lo está. Se volvió a leer la tarea.",
  "p.task.refused_from_task_exists":
    "Rechazado: ya se registró una incidencia abierta desde esta tarea. Se volvieron a leer las incidencias.",
  "p.task.refused_from_task_forbidden":
    "Rechazado: no puede registrar incidencias en este plan.",
  "p.task.refused_from_task_gone": "Esa tarea ya no está en este plan.",
  "p.plan_change_requests.refused_gone":
    "Esa solicitud de cambio ya no está en este plan; se volvió a leer la lista.",
  "p.plan_change_requests.refused_links_conflict":
    "Rechazado: la solicitud se ha cerrado, o una de las incidencias ya responde a otra solicitud de cambio. Se volvieron a leer la solicitud y las incidencias del plan.",
  "p.plan_change_requests.refused_links_invalid":
    "Rechazado: una incidencia o un riesgo elegido ya no está en este plan. Se volvieron a leer las incidencias y los riesgos del plan.",
  "p.plan_change_requests.refused_preview":
    "Rechazado: esta solicitud no lleva cambios hipotéticos que el planificador pueda leer, así que no hay previsión.",
  "p.plan_change_requests.refused_baseline":
    "Rechazado: ya se tomó una línea base por este cambio, o ya no está aprobado. Se volvió a leer la solicitud.",
  "p.plan_change_requests.problem_links_too_many":
    "Elija como máximo {max} incidencias y {max} riesgos.",
  "p.plan_change_requests.problem_links_elsewhere":
    "«{title}» ya responde a otra solicitud de cambio; desmárquela.",
  "p.plan_change_requests.links_unchanged":
    "No cambió nada, así que no se envió nada.",
  "p.plan_change_requests.links_saved":
    "Se guardaron las incidencias y los riesgos a los que responde este cambio.",
  "p.plan_change_requests.confirm_baseline":
    "¿Tomar una línea base del cronograma por «{title}»? Solo se puede tomar una para este cambio, y no se puede deshacer.",
  "p.plan_change_requests.baseline_taken":
    "Se tomó la versión {version} de la línea base por este cambio.",
  "p.plan_change_requests.stated_impact":
    "Impacto indicado por el solicitante: {impact}.",
  "p.plan_change_requests.implemented_on": "Implementado el {date}.",
  "p.plan_change_requests.implement": "Marcar como implementado",
  "p.plan_change_requests.implemented": "El cambio se marcó como implementado.",
  "p.plan_change_requests.links_heading":
    "Incidencias y riesgos a los que responde este cambio",
  "p.plan_change_requests.links_none":
    "Todavía no responde a ninguna incidencia ni riesgo.",
  "p.plan_change_requests.answers_issue": "Incidencia: {title}",
  "p.plan_change_requests.answers_risk": "Riesgo: {title}",
  "p.plan_change_requests.an_issue": "una incidencia que ya no figura",
  "p.plan_change_requests.a_risk": "un riesgo que ya no figura",
  "p.plan_change_requests.links_says":
    "Se eligen mientras la solicitud está abierta (borrador, enviada, en revisión o aprobada). Una incidencia responde como máximo a una solicitud de cambio; hasta {max} incidencias y {max} riesgos. Guardar sustituye el conjunto entero.",
  "p.plan_change_requests.links_issues": "Incidencias de este plan",
  "p.plan_change_requests.issue_elsewhere":
    "{title} (responde a otra solicitud de cambio)",
  "p.plan_change_requests.links_no_issues": "Este plan no tiene incidencias.",
  "p.plan_change_requests.links_risks": "Riesgos de este plan",
  "p.plan_change_requests.links_no_risks": "Este plan no tiene riesgos.",
  "p.plan_change_requests.links_save": "Guardar a qué responde",
  "p.plan_change_requests.links_closed":
    "Una solicitud cerrada no puede volver a vincularse.",
  "p.plan_change_requests.forecast_heading": "Previsión hipotética",
  "p.plan_change_requests.forecast_says":
    "Una previsión a partir del cronograma actual con los cambios hipotéticos de esta solicitud aplicados. Leerla no cambia nada; el impacto indicado arriba es la cifra del propio solicitante.",
  "p.plan_change_requests.forecast_show": "Mostrar la previsión",
  "p.plan_change_requests.forecast_label": "Previsión: no se cambió nada",
  "p.plan_change_requests.forecast_finish":
    "Fin del plan: {before} ahora, {after} con el cambio.",
  "p.plan_change_requests.forecast_finish_unknown":
    "No se puede prever el fin del plan.",
  "p.plan_change_requests.forecast_moved_one": "1 tarea terminaría otro día.",
  "p.plan_change_requests.forecast_moved_other":
    "{n} tareas terminarían otro día.",
  "p.plan_change_requests.forecast_later_one":
    "{title}: termina 1 día hábil más tarde",
  "p.plan_change_requests.forecast_later_other":
    "{title}: termina {n} días hábiles más tarde",
  "p.plan_change_requests.forecast_earlier_one":
    "{title}: termina 1 día hábil antes",
  "p.plan_change_requests.forecast_earlier_other":
    "{title}: termina {n} días hábiles antes",
  "p.plan_change_requests.a_task": "Una tarea",
  "p.plan_change_requests.forecast_labour":
    "El coste de mano de obra previsto cambiaría en {amount}.",
  "p.plan_change_requests.forecast_labour_hidden":
    "La previsión del coste de mano de obra solo se muestra a quien puede ver las tarifas.",
  "p.plan_change_requests.forecast_labour_none":
    "No hay trabajo con coste, así que no hay previsión del coste de mano de obra.",
  "p.plan_change_requests.forecast_labour_unknown":
    "Sin previsión del coste de mano de obra.",
  "p.plan_change_requests.forecast_no_edits":
    "Esta solicitud no lleva cambios hipotéticos, así que no hay previsión; su impacto indicado es la cifra del propio solicitante.",
  "p.plan_change_requests.baseline_heading": "Una línea base por este cambio",
  "p.plan_change_requests.baseline_done":
    "Se ha tomado una línea base del cronograma por este cambio.",
  "p.plan_change_requests.baseline_says":
    "Tomarla es decisión de una persona; nunca se toma una línea base nueva automáticamente. Una línea base por cambio, con un motivo que nombra la solicitud, y no se puede deshacer.",
  "p.plan_change_requests.baseline_take": "Tomar una línea base",
  "p.plan_change_requests.baseline_not_yet":
    "Se puede tomar una línea base cuando el cambio esté aprobado.",
  "p.escalations.problem_outcome":
    "Elija un resultado: decidido, orientación dada o rechazado.",
  "p.escalations.problem_response":
    "Escriba una respuesta de {min} a {max} caracteres.",
  "p.escalations.refused_ack_right":
    "Rechazado: reconocer una escalada requiere el derecho a reconocer escaladas (lo tiene un administrador). No se registró nada.",
  "p.escalations.refused_resolve_right":
    "Rechazado: resolver una escalada requiere el derecho a resolver escaladas (lo tiene un administrador). No se registró nada.",
  "p.escalations.refused_ack_state":
    "Rechazado: esta escalada se resolvió o se retiró entretanto, así que no se puede reconocer. Se volvió a leer.",
  "p.escalations.refused_resolve_state":
    "Rechazado: esta escalada se retiró, o ya está resuelta con otro resultado u otra respuesta; una resolución nunca se sobrescribe. Se volvió a leer.",
  "p.escalations.refused_resolve_invalid":
    "Rechazado: no se aceptó el resultado o la respuesta. Revise las reglas de arriba e inténtelo de nuevo.",
  "p.escalations.refused_gone":
    "Esta escalada ya no existe, o ya no puede verla. Se volvió a leer la lista.",
  "p.escalations.could_not_save": "Eso no se registró.",
  "p.escalations.by_unrecorded": "alguien no registrado",
  "p.escalations.ack_record": "Reconocida en el nivel {level} por {by}, {at}.",
  "p.escalations.resolution_record": "Resuelta {at} por {by}: {outcome}.",
  "p.escalations.outcome_unrecorded": "sin resultado registrado",
  "p.escalations.records_heading": "Reconocimiento y resolución",
  "p.escalations.records_failed":
    "No se pudo leer el reconocimiento o la resolución; el historial de abajo sigue mostrando lo que pasó.",
  "p.escalations.ack_note_line": "Nota: {note}",
  "p.escalations.ack_none": "Sin reconocer en su nivel actual.",
  "p.escalations.resolution_kept":
    "Una resolución es un registro: no se cambia ni se sobrescribe.",
  "p.escalations.records_rules":
    "Reconocer requiere el derecho a reconocer escaladas, y resolver el derecho a resolverlas; un administrador tiene ambos, y esta página no puede saber de antemano si usted los tiene. Un reconocimiento cuenta en el nivel actual: subirla de nivel lo borra. Una resolución es un registro y nunca se sobrescribe.",
  "p.escalations.ack_heading": "Reconocer esta escalada",
  "p.escalations.ack_note": "Nota (opcional)",
  "p.escalations.ack_done": "Reconocida; el registro se muestra abajo.",
  "p.escalations.resolve_heading": "Resolver esta escalada",
  "p.escalations.resolve_says":
    "La respuesta tiene de {min} a {max} caracteres. Se le pregunta antes de registrarla, y después no se puede cambiar.",
  "p.escalations.confirm_resolve":
    "¿Registrar esta resolución de «{title}»? Después no se podrá cambiar ni sobrescribir.",
  "p.escalations.resolution_done": "Resuelta; el registro se muestra abajo.",
  "p.register_completeness.link": "Integridad del registro",
  "p.register_completeness.title": "Integridad del registro",
  "p.register_completeness.title_ppm": "Integridad del registro — PPM",
  "p.register_completeness.intro":
    "Lo completo que está el registro de cada programa: de lo que un programa debería tener registrado, la parte que tiene, y cada dato que falta, nombrado. Solo se listan los programas (planes de tipo Programa) que usted puede leer.",
  "p.register_completeness.rules":
    "Los diez: un responsable sénior (el líder del programa hace sus veces), socios, una etapa, una línea base del calendario, una fecha objetivo, un presupuesto, una fuente de financiación en cada línea presupuestaria, una evaluación de la confianza en la entrega, riesgos y dependencias. Los socios solo se piden donde el modelo de entrega tiene socios, y una fuente de financiación solo donde hay una línea presupuestaria. El programa y los planes que tiene debajo cuentan juntos.",
  "p.register_completeness.switch_on":
    "«Sin registro, no hay dinero» está activado: se rechaza liberar una línea presupuestaria o reservar una asignación firme en un programa incompleto, nombrando lo que falta.",
  "p.register_completeness.switch_off":
    "«Sin registro, no hay dinero» está desactivado: liberar una línea presupuestaria o reservar una asignación firme en un programa incompleto sigue adelante, y el aviso queda en el registro de auditoría.",
  "p.register_completeness.count_one":
    "{complete} de {n} programa está completo.",
  "p.register_completeness.count_other":
    "{complete} de {n} programas están completos.",
  "p.register_completeness.none":
    "No hay ningún programa que usted pueda leer.",
  "p.register_completeness.loading": "Cargando…",
  "p.register_completeness.could_not_load":
    "No se pudo cargar la integridad del registro.",
  "p.register_completeness.table": "Programas y sus registros",
  "p.register_completeness.col_programme": "Programa",
  "p.register_completeness.col_state": "Estado",
  "p.register_completeness.col_score": "Parte registrada",
  "p.register_completeness.col_on_record": "Datos registrados",
  "p.register_completeness.col_missing": "Falta",
  "p.register_completeness.col_not_asked": "No se pide",
  "p.register_completeness.of": "{present} de {applicable}",
  "p.register_completeness.state_complete": "completo",
  "p.register_completeness.state_incomplete": "incompleto",
  "p.register_completeness.field_other": "otro dato",
  "p.register_completeness.none_listed": "ninguno",
  "p.meetings.title": "Reuniones",
  "p.meetings.title_ppm": "Reuniones — PPM",
  "p.meetings.link": "Reuniones, actas y acciones",
  "p.meetings.intro":
    "Una reunión se planifica con un título, una fecha y un orden del día. Después se registra una sola vez como celebrada, con su acta y sus resultados (decisiones, acciones y notas), o se cancela. Una reunión celebrada o cancelada es un registro: el acta de una reunión celebrada no se puede corregir y una reunión cancelada no se puede restaurar. No se registran los asistentes.",
  "p.meetings.plan_heading": "Planificar una reunión",
  "p.meetings.plan_says":
    "Un título de hasta {title} caracteres y una fecha. El orden del día contiene hasta {items} puntos de hasta {chars} caracteres cada uno; un punto puede referirse a una tarea, un riesgo, un hito o una incidencia de este plan.",
  "p.meetings.f_title": "Título (hasta {max} caracteres)",
  "p.meetings.f_date": "Fecha prevista",
  "p.meetings.agenda_item": "Punto del orden del día {n}",
  "p.meetings.f_item_text":
    "De qué trata el punto {n} del orden del día (hasta {max} caracteres)",
  "p.meetings.f_item_link":
    "El registro al que se refiere el punto {n} del orden del día",
  "p.meetings.no_record": "Ningún registro",
  "p.meetings.records_loading": "Cargando los registros del plan…",
  "p.meetings.remove_item": "Quitar el punto {n} del orden del día",
  "p.meetings.add_item": "Añadir un punto al orden del día",
  "p.meetings.plan": "Planificar la reunión",
  "p.meetings.planned": "Planificada: {title}, el {date}.",
  "p.meetings.list_heading":
    "Las reuniones del plan, de la más reciente a la más antigua",
  "p.meetings.pdf_says":
    "Cada reunión se descarga como un documento PDF con lo que muestra esta página; sus textos fijos (los encabezados y el estado) están en inglés.",
  "p.meetings.loading": "Cargando las reuniones…",
  "p.meetings.none": "Aún no hay reuniones.",
  "p.meetings.status_planned": "planificada",
  "p.meetings.status_held": "celebrada",
  "p.meetings.status_cancelled": "cancelada",
  "p.meetings.held_on": "celebrada el {date}",
  "p.meetings.scheduled_on": "prevista para el {date}",
  "p.meetings.show": "Abrir la reunión {title}",
  "p.meetings.hide": "Cerrar la reunión {title}",
  "p.meetings.pdf": "Descargar como PDF",
  "p.meetings.pdf_named": "Descargar la reunión {title} como PDF",
  "p.meetings.agenda_heading": "Orden del día",
  "p.meetings.agenda_none": "No hay puntos en el orden del día.",
  "p.meetings.minutes_heading": "Acta",
  "p.meetings.outcomes_heading": "Resultados",
  "p.meetings.outcomes_none": "No se registró ningún resultado.",
  "p.meetings.outcome_decision": "Decisión",
  "p.meetings.outcome_action": "Acción",
  "p.meetings.outcome_note": "Nota",
  "p.meetings.about": "sobre {record}",
  "p.meetings.link_gone": "sobre un registro que ya no existe",
  "p.meetings.task_made": "la tarea creada a partir del resultado {n}",
  "p.meetings.held_kept":
    "Esta reunión es un registro: su acta y sus resultados no se modifican.",
  "p.meetings.cancelled_kept":
    "Esta reunión se canceló. Es un registro y no se puede restaurar.",
  "p.meetings.held_heading": "Registrar {title} como celebrada",
  "p.meetings.held_says":
    "Un acta de hasta {minutes} caracteres y hasta {items} resultados de hasta {chars} caracteres cada uno: cada uno es una decisión, una acción o una nota, y cada uno puede referirse a un registro de este plan. La fecha de celebración es la fecha prevista, salvo que indique otra. Esto se hace una sola vez: después no se podrá corregir el acta.",
  "p.meetings.f_minutes": "Acta (hasta {max} caracteres)",
  "p.meetings.f_held_on":
    "Fecha en que se celebró (en blanco para la fecha prevista)",
  "p.meetings.outcome_n": "Resultado {n}",
  "p.meetings.f_outcome_kind": "Qué es el resultado {n}",
  "p.meetings.choose_kind": "Elija…",
  "p.meetings.f_outcome_text":
    "El resultado {n} con palabras (hasta {max} caracteres)",
  "p.meetings.f_outcome_link": "El registro al que se refiere el resultado {n}",
  "p.meetings.remove_outcome": "Quitar el resultado {n}",
  "p.meetings.add_outcome": "Añadir un resultado",
  "p.meetings.record_held": "Registrar como celebrada",
  "p.meetings.cancel_meeting": "Cancelar la reunión {title}",
  "p.meetings.confirm_held":
    "¿Registrar {title} como celebrada? Después no se podrán corregir su acta ni sus resultados.",
  "p.meetings.confirm_cancel":
    "¿Cancelar {title}? Una reunión cancelada no se puede restaurar.",
  "p.meetings.held_done": "Registrada como celebrada: {title}.",
  "p.meetings.cancelled_done": "Cancelada: {title}.",
  "p.meetings.problem_title": "Indique un título de hasta {title} caracteres.",
  "p.meetings.problem_date": "Indique una fecha.",
  "p.meetings.problem_agenda_count":
    "Un orden del día contiene como máximo {items} puntos.",
  "p.meetings.problem_outcome_count":
    "Una reunión contiene como máximo {items} resultados.",
  "p.meetings.problem_item_text":
    "Escríbalo en un máximo de {chars} caracteres.",
  "p.meetings.problem_minutes":
    "Escriba el acta, en un máximo de {minutes} caracteres.",
  "p.meetings.problem_outcome_kind":
    "Elija una decisión, una acción o una nota.",
  "p.meetings.problem_on_line": "En {line}: {problem}",
  "p.meetings.could_not_load": "No se pudieron cargar las reuniones.",
  "p.meetings.could_not_save": "La reunión no se modificó.",
  "p.meetings.refused_gone":
    "Rechazado: esta reunión, o el plan, ya no existe. Se volvió a leer la lista.",
  "p.meetings.refused_settled":
    "Rechazado: esta reunión ya se celebró o se canceló, así que es un registro y no se modifica. Se volvió a leer.",
  "p.meetings.refused_link":
    "Rechazado: un registro al que remite un punto ya no es un registro de este plan. No se guardó nada; elija de nuevo los registros.",
  "p.meetings.refused_not_held":
    "Rechazado: las tareas solo se proponen a partir de una reunión celebrada que tenga acta.",
  "p.meetings.refused_commit_state":
    "Rechazado: la reunión no está celebrada, o una de estas acciones ya tiene una tarea. No se creó nada; se volvió a leer la reunión, así que revise de nuevo.",
  "p.meetings.refused_commit_invalid":
    "Rechazado: una propuesta ya no está lista. No se creó nada; revise de nuevo las propuestas.",
  "p.meetings.refused_proposals":
    "Rechazado: envíe entre 1 y {count} propuestas.",
  "p.meetings.tasks_heading": "Tareas a partir de las acciones",
  "p.meetings.tasks_says":
    "Para cada acción que aún no tenga tarea puede proponer una: un título de hasta {max} caracteres y un pasaje del acta que la respalde, de al menos {min} caracteres (se ignoran las mayúsculas y los espacios). La persona asignada es opcional y ya debe figurar en las tareas de este plan o en su matriz de responsabilidades, como una referencia del tipo person:…; la fecha de vencimiento es opcional, entre {first} y {last}; los puntos van de 0 a {points}. Hasta {count} a la vez. La comprobación no escribe nada; después las tareas se crean todas o ninguna, y una acción recibe una sola tarea.",
  "p.meetings.proposal_legend": "Acción {n}: {text}",
  "p.meetings.f_propose": "Proponer una tarea para la acción {n}",
  "p.meetings.f_task_title":
    "Título de la tarea para la acción {n} (hasta {max} caracteres)",
  "p.meetings.f_excerpt":
    "Pasaje del acta para la acción {n} (al menos {min} caracteres)",
  "p.meetings.f_assignee":
    "Persona asignada a la acción {n} (referencia opcional)",
  "p.meetings.f_due": "Fecha de vencimiento de la acción {n} (opcional)",
  "p.meetings.f_points": "Puntos de la acción {n} (opcional, de 0 a {max})",
  "p.meetings.check": "Comprobar las propuestas",
  "p.meetings.create_tasks": "Crear las tareas",
  "p.meetings.none_chosen":
    "No se eligió ninguna propuesta, así que no se envió nada.",
  "p.meetings.checked_one":
    "Comprobado, nada escrito: {n} propuesta está lista, {dropped} no.",
  "p.meetings.checked_other":
    "Comprobado, nada escrito: {n} propuestas están listas, {dropped} no.",
  "p.meetings.ready": "Lista, sin persona asignada.",
  "p.meetings.ready_for": "Lista, para {who}.",
  "p.meetings.not_all_ready":
    "Todas las propuestas elegidas deben estar listas antes de crear las tareas: cambie o desmarque las que no lo estén y compruebe de nuevo.",
  "p.meetings.check_again":
    "Las propuestas cambiaron desde que se comprobaron: compruébelas de nuevo.",
  "p.meetings.confirm_tasks_one":
    "¿Crear {n} tarea a partir de las acciones de {title}? Una acción recibe una sola tarea.",
  "p.meetings.confirm_tasks_other":
    "¿Crear {n} tareas a partir de las acciones de {title}? Una acción recibe una sola tarea.",
  "p.meetings.tasks_made_one":
    "Se creó {n} tarea; está vinculada bajo su acción.",
  "p.meetings.tasks_made_other":
    "Se crearon {n} tareas; cada una está vinculada bajo su acción.",
  "p.meetings.could_not_check": "No se pudieron comprobar las propuestas.",
  "p.meetings.could_not_create": "No se crearon las tareas.",
  "p.meetings.dropped_title_invalid":
    "No está lista: el título está en blanco o es demasiado largo.",
  "p.meetings.dropped_excerpt_not_in_minutes":
    "No está lista: el pasaje no se encuentra en el acta, o es demasiado corto.",
  "p.meetings.dropped_outcome_not_found":
    "No está lista: la reunión no tiene ese resultado.",
  "p.meetings.dropped_outcome_not_an_action":
    "No está lista: ese resultado no es una acción.",
  "p.meetings.dropped_outcome_already_has_a_task":
    "No está lista: esa acción ya tiene una tarea.",
  "p.meetings.dropped_due_outside_window":
    "No está lista: la fecha de vencimiento queda fuera de las fechas permitidas.",
  "p.meetings.dropped_points_invalid":
    "No está lista: los puntos quedan fuera del intervalo permitido.",
  "p.meetings.dropped_assignee_not_in_plan":
    "No está lista: la persona asignada no es una de las personas de este plan (en sus tareas o en su matriz de responsabilidades).",
  "p.meetings.dropped_outcome_proposed_twice":
    "No está lista: la misma acción se propone dos veces.",
  "p.meetings.dropped_other":
    "No está lista, por un motivo que esta página no conoce.",
  "p.meetings.problem_proposals_count":
    "Proponga como máximo {count} tareas a la vez.",
  "p.meetings.problem_task_title":
    "Dé a la tarea un título de hasta {max} caracteres.",
  "p.meetings.problem_excerpt":
    "Cite un pasaje del acta de al menos {min} caracteres.",
  "p.meetings.problem_due":
    "La fecha de vencimiento debe estar entre una semana antes de la reunión y un año después.",
  "p.meetings.problem_points":
    "Los puntos son un número entero de 0 a {points}.",
  "p.lessons.title": "Lecciones aprendidas",
  "p.lessons.title_ppm": "Lecciones aprendidas — PPM",
  "p.lessons.link": "Lecciones aprendidas",
  "p.lessons.intro":
    "Una lección se registra en una puerta de fase o al cierre del plan. Nunca se modifica: solo se puede retirar, y una lección retirada ya no se muestra (queda en el registro de auditoría). A una propuesta nueva cuyo título se parezca al de este plan se le muestran sus lecciones.",
  "p.lessons.record_heading": "Registrar una lección",
  "p.lessons.record_says":
    "Cuándo (una puerta de fase o el cierre), qué es, un resumen de hasta {max} caracteres y, si lo desea, un detalle de hasta {bytes} bytes (una letra con acento u otra escritura ocupa más de uno) y la tarea, el riesgo, el hito o la incidencia de este plan a que se refiere.",
  "p.lessons.f_occasion": "Cuándo",
  "p.lessons.choose_occasion": "Elija una puerta de fase o el cierre…",
  "p.lessons.closure": "Cierre",
  "p.lessons.occasion_other": "Otra ocasión",
  "p.lessons.f_category": "Qué es",
  "p.lessons.category_went_well": "Salió bien",
  "p.lessons.category_went_badly": "Salió mal",
  "p.lessons.category_recommendation": "Recomendación",
  "p.lessons.category_other": "Otro tipo",
  "p.lessons.f_summary": "Resumen (hasta {max} caracteres)",
  "p.lessons.f_detail": "Detalle (opcional, hasta {bytes} bytes)",
  "p.lessons.f_link": "El registro al que se refiere (opcional)",
  "p.lessons.record": "Registrar la lección",
  "p.lessons.recorded": "Registrada: {summary}",
  "p.lessons.problem_occasion": "Elija una puerta de fase o el cierre.",
  "p.lessons.problem_category": "Elija salió bien, salió mal o recomendación.",
  "p.lessons.problem_summary": "Escriba un resumen de hasta {max} caracteres.",
  "p.lessons.problem_detail": "El detalle supera los {bytes} bytes.",
  "p.lessons.list_heading":
    "Las lecciones del plan, de la más antigua a la más reciente",
  "p.lessons.loading": "Cargando las lecciones…",
  "p.lessons.none": "Aún no hay lecciones.",
  "p.lessons.count_one": "{n} lección.",
  "p.lessons.count_other": "{n} lecciones.",
  "p.lessons.recorded_on": "registrada el {date}",
  "p.lessons.recorded_on_by": "registrada el {date} por {who}",
  "p.lessons.link_unlisted":
    "sobre un registro que no figura entre las tareas, los riesgos, los hitos y las incidencias del plan que se listan aquí",
  "p.lessons.withdraw": "Retirar la lección {summary}",
  "p.lessons.withdraw_short": "Retirar",
  "p.lessons.confirm_withdraw":
    "¿Retirar la lección “{summary}”? Ya no se mostrará, ni aquí ni en las propuestas nuevas; queda en el registro de auditoría.",
  "p.lessons.withdrawn": "Retirada: {summary}",
  "p.lessons.could_not_load": "No se pudieron cargar las lecciones.",
  "p.lessons.could_not_save": "La lección no se modificó.",
  "p.lessons.refused_gone": "Rechazado: el plan ya no existe.",
  "p.lessons.refused_withdrawn":
    "Rechazado: esa lección ya se retiró. Se volvió a leer la lista.",
  "p.lessons.refused_link":
    "Rechazado: el registro elegido ya no es un registro de este plan. No se registró nada; elíjalo de nuevo.",
  "p.lessons.refs_heading": "Qué hace referencia a un registro",
  "p.lessons.refs_says":
    "Elija una de las tareas, riesgos, hitos o incidencias del plan para listar los puntos de reunión y las lecciones que remiten a él (hasta 100 de cada tipo).",
  "p.lessons.f_target": "El registro",
  "p.lessons.choose_target": "Elija un registro…",
  "p.lessons.show_refs": "Mostrar qué hace referencia a él",
  "p.lessons.problem_target": "Elija primero un registro.",
  "p.lessons.refs_count_one": "{n} elemento hace referencia a él.",
  "p.lessons.refs_count_other": "{n} elementos hacen referencia a él.",
  "p.lessons.refs_meetings": "Puntos de reunión",
  "p.lessons.refs_lessons": "Lecciones que remiten a él",
  "p.lessons.refs_agenda": "Orden del día",
  "p.lessons.refused_no_record":
    "Ese registro ya no es un registro de este plan, o usted no puede leerlo.",
  "p.lessons.could_not_read_refs": "No se pudo leer qué hace referencia a él.",
  "p.lessons.similar": "Lecciones de planes similares",
  "p.lessons.similar_named": "Lecciones de planes como {title}",
  "p.lessons.similar_heading": "Lecciones de planes como {title}",
  "p.lessons.similar_says":
    "Coincidencia solo por el título, con todos los planes que tienen lecciones, cerrados o no, de mejor a peor coincidencia. La puntuación y la confianza son las del comparador.",
  "p.lessons.similar_none": "Ningún plan similar tiene lecciones.",
  "p.lessons.match": "coincidencia {score}, confianza {confidence}.",
  "p.lessons.match_title": "Títulos parecidos en un {score}.",
  "p.lessons.confidence_high": "alta",
  "p.lessons.confidence_medium": "media",
  "p.lessons.confidence_low": "baja",
  "p.lessons.refused_proposal_gone":
    "Esa propuesta ya no existe, o usted no puede leerla.",
  "p.lessons.could_not_read_similar":
    "No se pudieron leer las lecciones de planes similares.",
  "p.director_pack.open_by_severity": "Abiertas por gravedad: {list}",
  "p.director_pack.issues_overdue": "Incidencias vencidas: {n}",
  "p.director_pack.issues_in_window":
    "En el periodo: {raised} planteadas, {resolved} resueltas",
  "p.director_pack.oldest_issue":
    "Incidencia abierta más antigua: {title} (días naturales abierta: {n})",
  "p.director_pack.approved_not_implemented":
    "Aprobadas pero aún no implantadas: {n}",
  "p.director_pack.decided_in_window": "Decididas en el periodo: {n}",
  "p.director_pack.changes_by_status": "Por estado: {list}",
  "p.director_pack.oldest_change":
    "Solicitud de cambio abierta más antigua: {title} (días naturales abierta: {n})",
  "p.live_pack.heading": "Lecturas de los programas, ahora",
  "p.live_pack.says":
    "Leídas ahora de los planes que puede leer, presentadas como las presenta el informe para la dirección. Un informe emitido conserva sus cifras; el punto anterior del gráfico de fiebre solo procede de un informe emitido.",
  "p.live_pack.f_ac": "Coste real para el valor ganado",
  "p.live_pack.ac_budget_lines": "Partidas presupuestarias (predeterminado)",
  "p.live_pack.ac_labour": "Coste de mano de obra registrado",
  "p.live_pack.ac_both": "Partidas presupuestarias y coste de mano de obra",
  "p.live_pack.f_window":
    "Incidencias y solicitudes de cambio de los últimos días ({min} a {max}; vacío para 30)",
  "p.live_pack.read_again": "Volver a leer",
  "p.live_pack.problem_window":
    "El número de días debe ser un entero de {min} a {max}, o vacío para 30.",
  "p.live_pack.could_not_read_fever":
    "No se pudo leer el gráfico de fiebre del colchón.",
  "p.live_pack.could_not_read_evm": "No se pudo leer el valor ganado.",
  "p.live_pack.could_not_read_benefits": "No se pudieron leer los beneficios.",
  "p.live_pack.could_not_read_issues":
    "No se pudieron leer las incidencias y solicitudes de cambio.",
  "p.live_pack.issues_heading": "Incidencias y solicitudes de cambio",
  "p.live_pack.issues_none":
    "No puede leer ningún plan con incidencias o solicitudes de cambio.",
  "p.live_pack.nothing": "Nada que mostrar.",
  "p.forecast.heading": "Previsión al finalizar",
  "p.forecast.says":
    "La estimación al finalizar es el coste real hasta ahora más la estimación hasta finalizar. La estimación hasta finalizar es la última cifra de control total del proyecto en la moneda de la previsión, o si no, lo que la última línea base aún planifica. Leída ahora; no se guarda nada.",
  "p.forecast.absent_no_currency":
    "Sin previsión: el plan no tiene línea base de presupuesto ni cifra de control total del proyecto, así que no hay moneda en la que prever.",
  "p.forecast.c_actual": "Coste real hasta ahora",
  "p.forecast.c_etc": "Estimación hasta finalizar",
  "p.forecast.c_eac": "Estimación al finalizar",
  "p.forecast.c_plans": "Planes",
  "p.forecast.not_known": "desconocido",
  "p.forecast.etc_tpc": "de la última cifra de control total del proyecto",
  "p.forecast.etc_baseline": "de lo que la línea base aún planifica",
  "p.forecast.no_baseline_used": "No se usó ninguna línea base de presupuesto.",
  "p.forecast.baseline_used":
    "Usa la versión {version} de la línea base de presupuesto.",
  "p.forecast.as_of": "Leído el {date}.",
  "p.forecast.other_currency":
    "El coste real registrado en una moneda distinta de {currency} no se suma.",
  "p.forecast.baselines_heading": "Líneas base de presupuesto",
  "p.forecast.baselines_says":
    "Una línea base es el presupuesto aprobado por periodo, en una moneda. Una vez aprobada queda congelada y nunca se edita; una nueva línea base es una versión nueva, y cada versión después de la primera necesita un motivo. Una línea base tiene de 1 a {max} periodos, cada uno con un final no anterior a su inicio y un importe previsto no inferior a cero.",
  "p.forecast.no_baselines":
    "No se ha aprobado ninguna línea base de presupuesto.",
  "p.forecast.version_line":
    "Versión {version} · {currency} · aprobada el {date}",
  "p.forecast.period_line": "{start} a {end}: {amount}",
  "p.forecast.first_baseline_heading": "Aprobar la primera línea base",
  "p.forecast.rebaseline_heading": "Aprobar la versión {version}",
  "p.forecast.f_currency": "Moneda (tres letras)",
  "p.forecast.periods_legend": "Periodos",
  "p.forecast.f_start": "Inicio del periodo {n}",
  "p.forecast.f_end": "Fin del periodo {n}",
  "p.forecast.f_amount": "Importe previsto del periodo {n}",
  "p.forecast.remove_period": "Quitar el periodo {n}",
  "p.forecast.add_period": "Añadir un periodo",
  "p.forecast.f_reason": "Motivo (opcional para la primera)",
  "p.forecast.f_reason_needed": "Por qué se rehace la línea base (obligatorio)",
  "p.forecast.approve": "Aprobar la línea base",
  "p.forecast.confirm_baseline":
    "¿Aprobar la versión {version} de la línea base de presupuesto? Una vez aprobada queda congelada y no se puede cambiar ni quitar.",
  "p.forecast.baseline_approved": "Versión {version} aprobada.",
  "p.forecast.problem_currency":
    "La moneda debe ser un código de tres letras, como GBP o EUR.",
  "p.forecast.problem_no_periods": "Indique al menos un periodo.",
  "p.forecast.problem_too_many":
    "Una línea base tiene como máximo {max} periodos.",
  "p.forecast.problem_dates":
    "Cada periodo necesita una fecha de inicio y una de fin.",
  "p.forecast.problem_order": "Un periodo no puede terminar antes de empezar.",
  "p.forecast.problem_amount":
    "Cada importe previsto debe ser un número, no inferior a cero.",
  "p.forecast.problem_reason": "Una nueva línea base necesita un motivo.",
  "p.forecast.problem_reason_long":
    "El motivo es demasiado largo: como máximo {bytes} bytes.",
  "p.forecast.problem_same":
    "Es igual que la última versión; no se envió nada.",
  "p.forecast.refused_gone": "Este plan ya no existe, o no puede leerlo.",
  "p.forecast.refused_baseline":
    "Se rechazó la línea base: puede que se haya aprobado otra versión entretanto, lo que hace necesario un motivo. Se ha vuelto a leer la lista.",
  "p.forecast.could_not_approve": "No se pudo aprobar la línea base.",
  "p.forecast.could_not_read": "No se pudo leer la previsión.",
  "p.forecast.could_not_read_baselines":
    "No se pudieron leer las líneas base de presupuesto.",
  "p.forecast.could_not_read_lines":
    "No se pudieron leer las partidas presupuestarias.",
  "p.forecast.tranches_heading": "Financiación retenida hasta una puerta",
  "p.forecast.tranches_says":
    "Una partida con puerta de financiación se retiene hasta que la fase del plan alcanza esa puerta; entonces puede liberarse, una vez, y solo entonces se puede registrar coste real contra ella. Una liberación no se puede deshacer.",
  "p.forecast.stage_now": "Fase del plan: {stage}.",
  "p.forecast.stage_none": "El plan aún no ha superado ninguna puerta.",
  "p.forecast.no_tranches": "Ninguna partida se retiene hasta una puerta.",
  "p.forecast.gate_line": "retenida hasta {gate}",
  "p.forecast.state_released": "liberada el {date}",
  "p.forecast.state_held": "aún retenida: la puerta no se ha superado",
  "p.forecast.release": "Liberar {description}",
  "p.forecast.confirm_release":
    "¿Liberar {description}, retenida hasta {gate}? Una liberación no se puede deshacer.",
  "p.forecast.released": "{description} liberada.",
  "p.forecast.refused_line_gone": "Esa partida o ese plan ya no existe.",
  "p.forecast.refused_register":
    "Rechazado: el registro del programa está incompleto. Complételo primero en la página de integridad del registro.",
  "p.forecast.refused_release":
    "Rechazado: la partida ya no está retenida, o su puerta no se ha superado. Se han vuelto a leer las partidas.",
  "p.forecast.could_not_release": "No se pudo liberar la partida.",
  "p.forecast.rollup_heading": "Previsión al finalizar, agregada",
  "p.forecast.rollup_says":
    "Desde un plan sobre los planes que tiene debajo, hasta {max} niveles, una fila por moneda: las monedas nunca se suman. Los planes sin moneda en la que prever se cuentan, no se suman.",
  "p.forecast.f_plan": "Plan de partida",
  "p.forecast.choose_plan": "Elija un plan",
  "p.forecast.f_depth": "Niveles de profundidad (1 a {max}; vacío para todos)",
  "p.forecast.show_rollup": "Mostrar la previsión",
  "p.forecast.problem_plan": "Elija un plan.",
  "p.forecast.problem_depth":
    "Los niveles deben ser un entero de 1 a {max}, o vacío.",
  "p.forecast.refused_plan_gone": "Ese plan ya no existe, o no puede leerlo.",
  "p.forecast.could_not_read_rollup": "No se pudo leer la previsión agregada.",
  "p.forecast.rollup_line_one": "Desde {name}: {n} plan leído, {date}.",
  "p.forecast.rollup_line_other": "Desde {name}: {n} planes leídos, {date}.",
  "p.forecast.rollup_truncated":
    "El árbol es mayor de lo que recorre el servicio; las cifras cubren una parte.",
  "p.forecast.rollup_revisits_one":
    "{n} plan se alcanzó dos veces y se contó una.",
  "p.forecast.rollup_revisits_other":
    "{n} planes se alcanzaron dos veces y se contaron una.",
  "p.forecast.rollup_no_currency_one":
    "{n} plan no tiene moneda en la que prever y no se suma.",
  "p.forecast.rollup_no_currency_other":
    "{n} planes no tienen moneda en la que prever y no se suman.",
  "p.forecast.rollup_empty": "Ningún plan aquí tiene previsión.",
  "p.value.heading": "Valor obtenido",
  "p.value.says":
    "Leído ahora del valor registrado; no se guarda nada. El retorno de la inversión compara cada punto de valor con el coste real registrado y se omite cuando las partidas usan más de una moneda, pues no se convierte ninguna. Una cifra que no puede indicarse dice por qué, nunca cero.",
  "p.value.roi": "Retorno de la inversión: {pct}",
  "p.value.roi_absent": "Retorno de la inversión no indicado: {reason}",
  "p.value.measured_share": "evidencia medida: {pct} de los puntos de valor",
  "p.value.ttv_one":
    "Tiempo hasta el valor: mediana {p50} días naturales, percentil 85 {p85} ({n} observación)",
  "p.value.ttv_other":
    "Tiempo hasta el valor: mediana {p50} días naturales, percentil 85 {p85} ({n} observaciones)",
  "p.value.ttv_absent": "Tiempo hasta el valor no indicado: {reason}",
  "p.value.adoption":
    "Adopción: {pct} ({active} de {target} usuarios activos en {days} días)",
  "p.value.adoption_definition": "un usuario activo es: {definition}",
  "p.value.adoption_absent": "Adopción no indicada: {reason}",
  "p.value.adoption_none": "No hay ninguna lectura de adopción registrada.",
  "p.value.absent_unrealized": "no se ha registrado ningún valor",
  "p.value.absent_no_denominator":
    "no hay nada con que compararlo (sin coste real o sin objetivo)",
  "p.value.absent_mixed_currency": "las partidas usan más de una moneda",
  "p.value.absent_no_baseline": "no hay línea base de presupuesto",
  "p.value.absent_no_responses": "nadie ha respondido",
  "p.value.absent_overflow": "la cifra es demasiado grande para calcularla",
  "p.value.absent_other": "el servicio no dijo por qué",
  "p.value.case_heading": "Objetivos del caso de negocio",
  "p.value.no_targets":
    "No hay ningún objetivo del caso de negocio registrado.",
  "p.value.target_line":
    "{metric}: de {baseline} a {target} · {source} · aprobado el {date}",
  "p.value.source_charter": "el acta de constitución",
  "p.value.source_gate_review": "una revisión de puerta",
  "p.value.source_other": "otra fuente",
  "p.value.to_costs": "Registrar un objetivo en la página de costes",
  "p.value.point_heading": "Registrar un valor observado",
  "p.value.point_says":
    "Un valor es medido, estimado o declarado. Con moneda es una cantidad de dinero; sin ella, un entero en la unidad de la medida. Un punto de valor no se puede cambiar aquí una vez registrado. Un punto de valor por plan puede ser el primer valor medible: detiene el reloj del tiempo hasta el valor, que inició el primer objetivo del caso de negocio, y no se puede mover.",
  "p.value.f_value": "Valor",
  "p.value.f_currency": "Moneda (tres letras, o vacío)",
  "p.value.f_method": "Cómo se obtuvo",
  "p.value.method_measured": "medido",
  "p.value.method_estimated": "estimado",
  "p.value.method_asserted": "declarado",
  "p.value.f_evidence": "Evidencia (enlace o referencia, opcional)",
  "p.value.f_first": "Este es el primer valor medible",
  "p.value.record_point": "Registrar el valor",
  "p.value.confirm_first":
    "¿Registrar esto como el primer valor medible del plan? Detiene el reloj del tiempo hasta el valor y no se puede mover.",
  "p.value.point_recorded": "Valor registrado.",
  "p.value.problem_currency":
    "La moneda debe ser un código de tres letras, o vacía.",
  "p.value.problem_amount": "El valor debe ser una cantidad de dinero.",
  "p.value.problem_whole": "Sin moneda, el valor debe ser un número entero.",
  "p.value.problem_method": "Elija cómo se obtuvo el valor.",
  "p.value.refused_first_taken":
    "Rechazado: este plan ya tiene un primer valor medible; el reloj se detiene una vez.",
  "p.value.refused_value": "Se rechazó el valor.",
  "p.value.refused_gone": "Este plan ya no existe, o usted no puede leerlo.",
  "p.value.could_not_read": "No se pudo leer el valor obtenido.",
  "p.value.could_not_record_point": "No se pudo registrar el valor.",
  "p.value.case_says":
    "Un objetivo es una medida prometida en el acta de constitución o en una revisión de puerta, con su valor en la aprobación y el valor prometido, como enteros en la unidad de la medida. La fecha de aprobación inicia el reloj del tiempo hasta el valor y no se puede cambiar; un objetivo no se puede cambiar ni quitar aquí.",
  "p.value.to_benefits": "El valor obtenido está en la página de beneficios.",
  "p.value.f_metric": "Medida",
  "p.value.f_baseline": "Valor en la aprobación",
  "p.value.f_target": "Valor prometido",
  "p.value.f_unit": "Unidad (opcional)",
  "p.value.f_source": "Prometido en",
  "p.value.record_target": "Registrar el objetivo",
  "p.value.confirm_target":
    "¿Registrar el objetivo {metric}? No se puede cambiar ni quitar aquí.",
  "p.value.target_recorded": "Objetivo {metric} registrado.",
  "p.value.problem_metric": "Nombre la medida.",
  "p.value.problem_target_values":
    "El valor en la aprobación y el valor prometido deben ser números enteros.",
  "p.value.problem_source": "Elija dónde se prometió el objetivo.",
  "p.value.refused_target": "Se rechazó el objetivo.",
  "p.value.could_not_record_target": "No se pudo registrar el objetivo.",
  "p.scoring.rank_by": "Ordenar por",
  "p.scoring.model_smart": "Puntuación Smart (a partir de pruebas)",
  "p.scoring.model_wsjf": "WSJF (el trabajo más corto ponderado primero)",
  "p.scoring.model_rice": "RICE (alcance, impacto, confianza, esfuerzo)",
  "p.scoring.models_say":
    "La Puntuación Smart sale de las pruebas que tiene el servicio; WSJF y RICE se puntúan a partir de las estimaciones de una persona, fijadas para cada plan con su botón Estimaciones. Un plan sin todas las entradas que necesita un modelo queda sin puntuar y va al final, nunca con cero. La banda es de la Puntuación Smart.",
  "p.scoring.of_plans_one":
    "{length} de {scored_of} plan, ordenado por {model}",
  "p.scoring.of_plans_other":
    "{length} de {scored_of} planes, ordenados por {model}",
  "p.scoring.ranked_by": "Planes ordenados por {model}",
  "p.scoring.waiting_for": "A la espera de",
  "p.scoring.open": "Estimaciones",
  "p.scoring.open_named": "Abrir las estimaciones de {name}",
  "p.scoring.heading": "Estimaciones para {name}",
  "p.scoring.form": "Las estimaciones de {name}",
  "p.scoring.says":
    "WSJF y RICE se puntúan a partir de estas estimaciones. Si falta alguna entrada no hay puntuación, nunca una suposición; un cero declarado no es una entrada que falta. Guardar sustituye cada estimación: una casilla vacía la borra.",
  "p.scoring.last_set": "Fijadas por última vez {when}.",
  "p.scoring.none_set": "No se han fijado estimaciones.",
  "p.scoring.wsjf_inputs":
    "Entradas de WSJF, cada una un número entero de {min} a {max}",
  "p.scoring.rice_inputs": "Entradas de RICE",
  "p.scoring.input_business_value": "Valor de negocio",
  "p.scoring.input_time_criticality": "Urgencia temporal",
  "p.scoring.input_risk_reduction": "Reducción de riesgo u oportunidad abierta",
  "p.scoring.input_job_size": "Tamaño del trabajo",
  "p.scoring.input_reach": "Alcance (personas o eventos en un periodo)",
  "p.scoring.input_impact": "Impacto",
  "p.scoring.input_confidence": "Confianza (por ciento)",
  "p.scoring.input_effort": "Esfuerzo (meses-persona)",
  "p.scoring.input_other": "otra entrada",
  "p.scoring.component_cost_of_delay": "Coste del retraso",
  "p.scoring.impact_minimal": "mínimo",
  "p.scoring.impact_low": "bajo",
  "p.scoring.impact_medium": "medio",
  "p.scoring.impact_high": "alto",
  "p.scoring.impact_massive": "enorme",
  "p.scoring.impact_value": "{value} (impacto {word})",
  "p.scoring.not_estimated": "sin estimar",
  "p.scoring.save": "Guardar las estimaciones",
  "p.scoring.close": "Cerrar las estimaciones",
  "p.scoring.unchanged":
    "Las estimaciones están como se guardaron; no se envió nada.",
  "p.scoring.saved": "Estimaciones de {name} guardadas.",
  "p.scoring.could_not_read": "No se pudieron leer las estimaciones.",
  "p.scoring.could_not_save": "No se pudieron guardar las estimaciones.",
  "p.scoring.refused_gone": "Ese plan ya no existe, o no puede leerlo.",
  "p.scoring.refused_inputs":
    "Se rechazaron las estimaciones: una entrada está fuera de su rango.",
  "p.scoring.problem_wsjf":
    "{input} debe ser un número entero de {min} a {max}.",
  "p.scoring.problem_reach":
    "{input} debe ser un número entero, no inferior a cero.",
  "p.scoring.problem_impact": "{input} debe ser 0,25, 0,5, 1, 2 o 3.",
  "p.scoring.problem_confidence":
    "{input} debe ser un número entero de 0 a 100.",
  "p.scoring.problem_effort": "{input} debe ser mayor que cero.",
  "p.scoring.problem_number": "{input} debe ser un número.",
  "p.scoring.formula_wsjf":
    "WSJF es (valor de negocio + urgencia temporal + reducción de riesgo u oportunidad abierta) ÷ tamaño del trabajo.",
  "p.scoring.formula_rice": "RICE es alcance × impacto × confianza ÷ esfuerzo.",
  "p.scoring.score_is": "Puntuación: {score}",
  "p.scoring.no_score": "Sin puntuación: espera estas entradas.",
  "p.scoring.needs_delay_part": "forma parte del coste del retraso",
  "p.scoring.needs_divisor":
    "es el divisor: sin ella no hay valor por unidad de trabajo",
  "p.scoring.needs_reach":
    "dice a cuántas personas llega el trabajo en un periodo",
  "p.scoring.needs_impact":
    "dice cuánto importa el trabajo a cada persona alcanzada",
  "p.scoring.needs_confidence":
    "rebaja las estimaciones según lo seguras que son",
  "p.scoring.needs_other": "la puntuación la necesita",
  "p.scoring.means_business_value":
    "valor relativo para el usuario o el negocio, de 1 a 20",
  "p.scoring.means_time_criticality":
    "lo rápido que se pierde el valor si llega tarde, de 1 a 20",
  "p.scoring.means_risk_reduction":
    "el riesgo que elimina o la oportunidad que abre, de 1 a 20",
  "p.scoring.means_cost_of_delay":
    "la suma de los tres anteriores: lo que cuesta un retraso",
  "p.scoring.means_job_size":
    "el tamaño relativo del trabajo, de 1 a 20; el divisor",
  "p.scoring.means_reach": "personas o eventos alcanzados en un periodo",
  "p.scoring.means_impact": "0,25 mínimo, 0,5 bajo, 1 medio, 2 alto, 3 enorme",
  "p.scoring.means_confidence": "lo seguras que son las estimaciones",
  "p.scoring.means_effort": "meses-persona de trabajo; el divisor",
  "p.scoring.means_other": "un componente que esta página no conoce",
  "p.frontier.heading": "Elegir una cartera",
  "p.frontier.says":
    "El generador puntúa cada plan que puede leer por su Puntuación Smart y le pone precio por sus líneas de presupuesto; después toma planes por puntuación por unidad de coste hasta agotar el tope. Los planes que deben incluirse se toman primero, cueste lo que cueste. Un plan sin Puntuación Smart queda fuera; un plan sin líneas de presupuesto no cuesta nada. La elección no siempre es la mejor posible, así que un tope que no compra más valor que otro más barato se marca, no se oculta.",
  "p.frontier.must_include":
    "Planes que toda elección debe incluir (hasta {max})",
  "p.frontier.plans_loading": "Leyendo los planes…",
  "p.frontier.plans_none": "No hay planes que pueda leer.",
  "p.frontier.frontier_heading": "Frontera eficiente",
  "p.frontier.frontier_says":
    "El generador se ejecuta una vez en cada tope y no se guarda nada. Reparta de {min} a {max} topes por igual desde cero hasta el coste de todos los planes con precio (vacío para {fallback}), o indique usted hasta {list} topes, cada uno un importe no inferior a cero.",
  "p.frontier.f_currency": "Moneda de los costes (tres letras)",
  "p.frontier.f_how": "Qué topes de presupuesto",
  "p.frontier.how_steps": "Repartidos por igual",
  "p.frontier.how_caps": "Topes que indico",
  "p.frontier.f_steps": "Cuántos topes (vacío para {fallback})",
  "p.frontier.f_caps": "Topes de presupuesto, uno por línea (hasta {list})",
  "p.frontier.run": "Trazar la frontera",
  "p.frontier.value_is":
    "El valor es la suma de las Puntuaciones Smart de los planes elegidos.",
  "p.frontier.table": "La frontera, punto por punto",
  "p.frontier.col_cap": "Tope de presupuesto",
  "p.frontier.col_cost": "Coste de la elección",
  "p.frontier.col_value": "Valor",
  "p.frontier.col_included": "Planes elegidos",
  "p.frontier.col_more": "Compra más valor",
  "p.frontier.more_yes": "Sí",
  "p.frontier.more_no": "No: un tope más barato llega a lo mismo",
  "p.frontier.show_choice": "Mostrar la elección",
  "p.frontier.show_choice_named": "Mostrar la elección con {cap}",
  "p.frontier.save_point": "Guardar como borrador",
  "p.frontier.save_point_named": "Guardar la elección con {cap} como borrador",
  "p.frontier.point_name": "Frontera con {cap}",
  "p.frontier.axis_cost": "Coste de los planes elegidos",
  "p.frontier.axis_value": "Valor (Puntuación Smart sumada)",
  "p.frontier.chart_points_one": "{n} tope de presupuesto.",
  "p.frontier.chart_points_other": "{n} topes de presupuesto.",
  "p.frontier.chart_best": "El mayor valor, {value}, cuesta {cost}.",
  "p.frontier.chart_dominated_one":
    "{n} tope no compra más valor que uno más barato.",
  "p.frontier.chart_dominated_other":
    "{n} topes no compran más valor que uno más barato.",
  "p.frontier.legend_efficient": "compra más valor",
  "p.frontier.legend_dominated": "no compra más valor que un tope más barato",
  "p.frontier.no_points": "La frontera no tiene puntos.",
  "p.frontier.generate_heading": "Generar un borrador",
  "p.frontier.generate_says":
    "Se guarda como escenario en borrador, para evaluarlo, compararlo y comprometerlo como cualquier otro. Hace falta un nombre, de {bytes} bytes como máximo. Con una moneda, cada plan se valora en ella; un tope necesita su moneda.",
  "p.frontier.g_name": "Nombre del borrador",
  "p.frontier.g_cap": "Tope de presupuesto (opcional)",
  "p.frontier.g_currency": "Moneda (necesaria con un tope)",
  "p.frontier.generate": "Generar el borrador",
  "p.frontier.draft_saved":
    "Borrador {name} guardado; está en la lista de arriba.",
  "p.frontier.choice_at": "Lo que elige un tope de {cap}",
  "p.frontier.choice_of": "Lo que eligió {name}",
  "p.frontier.chosen_count_one": "{n} plan elegido de {of}.",
  "p.frontier.chosen_count_other": "{n} planes elegidos de {of}.",
  "p.frontier.col_plan": "Plan de la elección",
  "p.frontier.col_chosen": "Elegido o no",
  "p.frontier.col_score": "Su Puntuación Smart",
  "p.frontier.col_plan_cost": "Su coste",
  "p.frontier.col_why": "Por qué quedó fuera",
  "p.frontier.chosen": "elegido",
  "p.frontier.left_out": "fuera",
  "p.frontier.no_score": "sin Puntuación Smart",
  "p.frontier.plan_unknown": "Un plan que no puede leer, o que ya no existe",
  "p.frontier.reason_no_score":
    "no tiene Puntuación Smart, que no es una puntuación de cero",
  "p.frontier.reason_foreign_currency":
    "todas sus líneas de presupuesto están en otra moneda, así que no se le puede poner precio",
  "p.frontier.reason_over_cap": "el tope se agotó antes de llegar a él",
  "p.frontier.reason_must_include_conflict":
    "debe incluirse, pero no es un plan que pueda leer",
  "p.frontier.reason_other": "fuera por un motivo que esta página no conoce",
  "p.frontier.problem_currency": "Indique la moneda con tres letras.",
  "p.frontier.problem_steps":
    "El número de topes debe ser un número entero de {min} a {max}.",
  "p.frontier.problem_caps_none": "Indique al menos un tope.",
  "p.frontier.problem_caps_many": "Indique como máximo {list} topes.",
  "p.frontier.problem_caps_amount":
    "Cada tope debe ser un importe no inferior a cero.",
  "p.frontier.problem_must_include":
    "Elija como máximo {list} planes para incluir.",
  "p.frontier.problem_name": "Ponga un nombre al borrador.",
  "p.frontier.problem_name_long": "El nombre supera los {bytes} bytes.",
  "p.frontier.problem_cap_currency": "Un tope necesita su moneda.",
  "p.frontier.problem_cap_amount":
    "El tope debe ser un importe no inferior a cero.",
  "p.frontier.refused_frontier": "Se rechazó la frontera.",
  "p.frontier.refused_generate": "Se rechazó el borrador.",
  "p.frontier.could_not_read": "No se pudo trazar la frontera.",
  "p.frontier.could_not_generate": "No se pudo generar el borrador.",
  "p.frontier.rollback": "Deshacer el compromiso",
  "p.frontier.rollback_named": "Deshacer el compromiso de {name}",
  "p.frontier.rollback_says":
    "Un escenario comprometido puede volver a ser borrador, tras una pregunta; no cambia nada salvo el escenario.",
  "p.frontier.confirm_rollback":
    "¿Devolver {name} a borrador? Dejará de ser la cartera comprometida; el compromiso queda en el registro de auditoría.",
  "p.frontier.rolled_back": "{name} vuelve a ser un borrador.",
  "p.frontier.refused_rollback_status":
    "{name} ya no está comprometido, así que no hay nada que deshacer. Se ha vuelto a leer la lista.",
  "p.frontier.refused_rollback_gone":
    "{name} ya no existe, o no puede leerlo. Se ha vuelto a leer la lista.",
  "p.frontier.could_not_roll_back":
    "No se pudo deshacer el compromiso del escenario.",
  "p.frontier.status_draft": "borrador",
  "p.frontier.status_committed": "comprometido",
  "p.frontier.violations_one": "{n} restricción incumplida",
  "p.frontier.violations_other": "{n} restricciones incumplidas",
  "p.frontier.compare_note":
    "B menos A, una moneda cada vez; las monedas nunca se suman entre sí.",
  "p.intake.edit": "Editar",
  "p.intake.edit_named": "Editar el borrador {title}",
  "p.intake.edit_heading": "Editar {title}",
  "p.intake.edit_heading_loading": "Leyendo el borrador…",
  "p.intake.edit_form": "Los campos del borrador",
  "p.intake.edit_says":
    "Solo se puede editar un borrador, y ahí también se marca una propuesta como petición externa. Hace falta un título; el título y el resumen tienen como máximo {bytes} bytes cada uno; un importe necesita su moneda; una contrapartida indicada tiene de {min} a {max} caracteres. Guardar sustituye cada campo.",
  "p.intake.save": "Guardar el borrador",
  "p.intake.saved": "{title} guardado.",
  "p.intake.unchanged": "No ha cambiado nada; no se envió nada.",
  "p.intake.could_not_read_draft": "No se pudo leer el borrador.",
  "p.intake.problem_not_draft": "Solo se puede editar un borrador.",
  "p.intake.problem_title_long": "El título supera los {bytes} bytes.",
  "p.intake.problem_summary_long": "El resumen supera los {bytes} bytes.",
  "p.intake.refused_not_draft":
    "Se rechazó la edición: la propuesta ya no es un borrador. Se ha vuelto a leer la lista.",
  "p.intake.refused_edit":
    "Se rechazó la edición: un campo queda fuera de lo que acepta el servicio.",
  "p.intake.forecast_heading": "Cuántas se aprobarán",
  "p.intake.forecast_says":
    "Se obtiene por simulación de cuántas propuestas se aprobaron en cada periodo pasado; las propuestas que llegan no influyen. Una previsión necesita al menos {needed} periodos de historial.",
  "p.intake.f_periods":
    "Periodos por delante (1 a {max}; vacío para {fallback})",
  "p.intake.f_period_days":
    "Días por periodo (1 a {max}; vacío para {fallback})",
  "p.intake.f_history":
    "Periodos de historial (1 a {max}; vacío para {fallback})",
  "p.intake.forecast_run": "Prever",
  "p.intake.problem_periods":
    "Los periodos por delante deben ser un número entero de 1 a {periods}.",
  "p.intake.problem_period_days":
    "Un periodo debe ser un número entero de días de 1 a {days}.",
  "p.intake.problem_history":
    "El historial debe ser un número entero de periodos de 1 a {history}.",
  "p.intake.refused_forecast": "Se rechazó la previsión.",
  "p.intake.could_not_forecast": "No se pudo obtener la previsión.",
  "p.intake.window":
    "Historial del {from} al {to}, en periodos de {days} días.",
  "p.intake.at_least_label_one":
    "En el próximo {n} periodo ({days} días), al menos",
  "p.intake.at_least_label_other":
    "En los próximos {n} periodos ({days} días), al menos",
  "p.intake.at_least_means":
    "con un 85 % de confianza: la cifra con la que planificar",
  "p.intake.median_label": "Normalmente",
  "p.intake.median_means":
    "la mitad de las pruebas aprueba más y la mitad menos: una moneda al aire, no un compromiso",
  "p.intake.at_most_label": "Como mucho",
  "p.intake.at_most_means":
    "el extremo optimista: el 85 % de las pruebas no aprueba más",
  "p.intake.proposals_n_one": "{n} propuesta",
  "p.intake.proposals_n_other": "{n} propuestas",
  "p.intake.absent_history":
    "Sin previsión: necesita al menos {needed} periodos de historial, y hay {n}.",
  "p.intake.absent_other":
    "No se pudo obtener una previsión de este historial.",
  "p.intake.approvals_history":
    "Aprobadas por periodo, de la más antigua a la más reciente: {list}",
  "p.intake.arrivals_history":
    "Propuestas llegadas por periodo, de la más antigua a la más reciente (se muestran al lado; no influyen en la previsión): {list}",
  "p.planner.heading": "Tiempo planificado",
  "p.planner.says":
    "Reserve tiempo un día para una de sus tareas abiertas. Un bloque es solo suyo y nunca cambia la fecha de vencimiento de la tarea; termina después de empezar y dura como máximo 12 horas. Los bloques pueden solaparse: un solapamiento se muestra como conflicto, y un bloque que termina cuando empieza el siguiente no lo es. Las horas están en su zona horaria, {tz}.",
  "p.planner.day_nav": "Elegir el día",
  "p.planner.previous_day": "Día anterior",
  "p.planner.back_to_today": "Volver a hoy",
  "p.planner.next_day": "Día siguiente",
  "p.planner.f_day": "Día",
  "p.planner.show_day": "Mostrar el día",
  "p.planner.problem_day": "Indique el día como una fecha.",
  "p.planner.day_heading": "Planificado el {day}",
  "p.planner.day_hours": "Este día tiene {hours} horas, porque cambia la hora.",
  "p.planner.nothing_planned": "No hay nada planificado este día.",
  "p.planner.clashes_one": "Choca con {n} bloque más",
  "p.planner.clashes_other": "Choca con {n} bloques más",
  "p.planner.remove": "Quitar",
  "p.planner.remove_named": "Quitar el bloque de {title}, {time}",
  "p.planner.confirm_remove":
    "¿Quitar el bloque de {title}, {time}? La tarea en sí no cambia.",
  "p.planner.removed": "Se quitó el bloque de {title}, {time}.",
  "p.planner.omitted_one":
    "No se muestra {n} bloque de una tarea que ya no puede leer.",
  "p.planner.omitted_other":
    "No se muestran {n} bloques de tareas que ya no puede leer.",
  "p.planner.due_heading": "Vence este día",
  "p.planner.nothing_due": "Nada de lo que tiene asignado vence este día.",
  "p.planner.loading": "Leyendo el día…",
  "p.planner.form": "Planificar tiempo para una tarea",
  "p.planner.form_says":
    "El bloque es el {day}, en su zona horaria. Uno que pasa de la medianoche se planifica como dos.",
  "p.planner.f_task": "Tarea",
  "p.planner.choose_task": "Elija una de sus tareas abiertas",
  "p.planner.loading_tasks": "Leyendo sus tareas…",
  "p.planner.task_option": "{title}, en {plan}",
  "p.planner.no_tasks":
    "No tiene nada asignado, así que no hay tarea para la que planificar tiempo.",
  "p.planner.f_start": "Empieza",
  "p.planner.f_end": "Termina",
  "p.planner.plan_time": "Planificar este tiempo",
  "p.planner.planned": "Se planificó {title}, {time}.",
  "p.planner.problem_task": "Elija una tarea.",
  "p.planner.problem_start": "Indique la hora de inicio.",
  "p.planner.problem_end": "Indique la hora de fin.",
  "p.planner.problem_start_skipped":
    "Ese día el reloj se salta la hora de inicio; elija otra.",
  "p.planner.problem_end_skipped":
    "Ese día el reloj se salta la hora de fin; elija otra.",
  "p.planner.problem_order": "Un bloque debe terminar después de empezar.",
  "p.planner.problem_too_long": "Un bloque dura como máximo 12 horas.",
  "p.planner.refused_zone":
    "No se aceptó la zona horaria de su navegador, así que no se puede mostrar el día.",
  "p.planner.refused_slot":
    "No se aceptó el bloque: debe terminar después de empezar y durar como máximo 12 horas, y una persona tiene como máximo {max} bloques.",
  "p.planner.refused_task":
    "Esa tarea ya no existe o está en un plan que no puede leer.",
  "p.planner.refused_gone":
    "Ese bloque ya no existe. Se ha vuelto a leer el día.",
  "p.planner.could_not_load": "No se pudo leer el día.",
  "p.planner.could_not_plan": "No se pudo planificar el tiempo.",
  "p.planner.could_not_remove": "No se pudo quitar el bloque.",
  "p.feed.heading": "Fuente de calendario",
  "p.feed.says":
    "Una dirección privada a la que puede suscribirse una aplicación de calendario: las tareas abiertas que tiene asignadas con fecha de vencimiento, los hitos y sprints de sus planes, y el tiempo que ha planificado. Una segunda dirección es una fuente de sus notificaciones para un lector de fuentes. Quien tenga una dirección puede leer lo que muestra, así que guárdela para usted.",
  "p.feed.rules":
    "Una dirección se muestra una vez, aquí, y esta página no la guarda; dura 365 días. Crear una nueva detiene al instante cualquier anterior.",
  "p.feed.issue": "Crear una dirección de fuente",
  "p.feed.confirm_issue":
    "¿Crear una nueva dirección de fuente? Cualquier dirección que creara antes deja de funcionar al instante.",
  "p.feed.issued":
    "Sus nuevas direcciones están abajo. Cópielas ahora: no se vuelven a mostrar.",
  "p.feed.calendar_address": "Dirección del calendario (iCalendar)",
  "p.feed.atom_address": "Dirección de la fuente de notificaciones (Atom)",
  "p.feed.revoke": "Detener todas las direcciones de fuente",
  "p.feed.confirm_revoke":
    "¿Detener todas las direcciones de fuente que ha creado? Una aplicación de calendario que use una deja de actualizarse al instante.",
  "p.feed.revoked":
    "Todas las direcciones de fuente que creó han dejado de funcionar.",
  "p.feed.no_address":
    "La respuesta no traía ninguna dirección que esta página pueda mostrar, así que no se muestra ninguna. Cualquier dirección anterior se ha detenido; cree otra si la necesita.",
  "p.feed.refused_forbidden": "No puede gestionar una dirección de fuente.",
  "p.feed.could_not_issue": "No se pudo crear una dirección de fuente.",
  "p.feed.could_not_revoke":
    "No se pudieron detener las direcciones de fuente.",
  "p.watching.heading": "Lo que sigue",
  "p.watching.says":
    "Se le avisa cuando cambia algo que sigue. Sigue una tarea cuando la crea, la comenta o se le asigna, salvo que lo haya desactivado en sus opciones de notificación. A nadie más se le dice quién sigue. Un plan se nombra; todo lo demás se muestra por su tipo y el comienzo de su referencia.",
  "p.watching.count_one": "Sigue {n} elemento.",
  "p.watching.count_other": "Sigue {n} elementos.",
  "p.watching.capped": "Solo se muestran los {max} más recientes.",
  "p.watching.none": "Ahora mismo no sigue nada.",
  "p.watching.named_plan": "El plan {name}",
  "p.watching.unnamed": "{kind}, referencia {ref}",
  "p.watching.kind_plan": "Un plan",
  "p.watching.kind_task": "Tarea",
  "p.watching.kind_risk": "Riesgo",
  "p.watching.kind_proposal": "Propuesta",
  "p.watching.kind_idea": "Una idea",
  "p.watching.kind_canvas_item": "Elemento de lienzo",
  "p.watching.kind_budget_line": "Partida presupuestaria",
  "p.watching.kind_scenario": "Escenario",
  "p.watching.kind_document_version": "Versión de documento",
  "p.watching.kind_other": "Otra cosa",
  "p.watching.stop": "Dejar de seguir",
  "p.watching.stop_named": "Dejar de seguir {what}",
  "p.watching.confirm_stop":
    "¿Dejar de seguir {what}? Ya no se le avisará cuando cambie.",
  "p.watching.stopped": "Ya no sigue {what}.",
  "p.watching.refused": "No se aceptó dejar de seguir eso.",
  "p.watching.loading": "Leyendo lo que sigue…",
  "p.watching.could_not_load": "No se pudo leer lo que sigue.",
  "p.watching.could_not_stop": "No se pudo dejar de seguirlo.",
  "p.workprofile.heading": "Trabajo a lo largo del tiempo",
  "p.workprofile.says":
    "El trabajo de las tareas de cada plan que puede leer, repartido por los días laborables de cada tarea según su perfil, en minutos enteros. El trabajo que no se puede ubicar (una tarea aún sin programar, o sin ningún día laborable) se cuenta aparte y nunca se descarta.",
  "p.workprofile.rules":
    "Las semanas van de lunes a domingo. Deje las fechas vacías para abarcar todo el trabajo ubicado. Una ventana abarca como máximo diez años y {max} periodos.",
  "p.workprofile.form": "Mostrar el trabajo a lo largo del tiempo",
  "p.workprofile.f_period": "Periodos",
  "p.workprofile.period_week": "Semanas",
  "p.workprofile.period_day": "Días",
  "p.workprofile.period_month": "Meses",
  "p.workprofile.f_from": "Desde (vacío para el primer día ubicado)",
  "p.workprofile.f_to": "Hasta (vacío para el último día ubicado)",
  "p.workprofile.show": "Mostrar el trabajo",
  "p.workprofile.problem_from": "Indique el primer día como una fecha.",
  "p.workprofile.problem_to": "Indique el último día como una fecha.",
  "p.workprofile.problem_order":
    "El primer día no puede ser posterior al último.",
  "p.workprofile.problem_too_long": "Una ventana abarca como máximo diez años.",
  "p.workprofile.problem_periods":
    "Esa ventana tiene más de {max} periodos; elija periodos más largos o una ventana más corta.",
  "p.workprofile.refused":
    "No se aceptó esa ventana: si aún no hay trabajo ubicado, indique ambas fechas, y no supere los periodos indicados arriba.",
  "p.workprofile.none":
    "Ningún plan que pueda leer tiene una tarea con trabajo.",
  "p.workprofile.summary": "{range}: {work} de trabajo, {unplaced} sin ubicar.",
  "p.workprofile.plans_table": "Trabajo por plan",
  "p.workprofile.periods_table": "Trabajo por periodo",
  "p.workprofile.col_plan": "Nombre del plan",
  "p.workprofile.col_work": "Trabajo",
  "p.workprofile.col_unplaced": "Sin ubicar",
  "p.workprofile.col_period": "Periodo",
  "p.workprofile.plan_unnamed": "Un plan sin nombre",
  "p.workprofile.could_not_load":
    "No se pudo leer el trabajo a lo largo del tiempo.",
  "p.timeentry.heading": "Registrar tiempo",
  "p.timeentry.says":
    "El tiempo se registra a su nombre, en minutos enteros, de 1 minuto a {max} en una fecha, en un plan y, si quiere, en una de sus tareas. Una semana aprobada no admite más tiempo. Un registro no se puede cambiar ni quitar una vez hecho.",
  "p.timeentry.open": "Anotar tiempo",
  "p.timeentry.form": "Registrar tiempo en un plan",
  "p.timeentry.f_plan": "En qué plan",
  "p.timeentry.choose_plan": "Elija un plan",
  "p.timeentry.loading": "Leyendo los planes…",
  "p.timeentry.f_task": "Tarea (opcional)",
  "p.timeentry.no_task": "Sin tarea: el plan en sí",
  "p.timeentry.task_unnamed": "Una tarea que ya no está en este plan",
  "p.timeentry.f_date": "Fecha",
  "p.timeentry.f_duration": "Tiempo dedicado",
  "p.timeentry.f_hours": "Horas",
  "p.timeentry.f_minutes": "Minutos",
  "p.timeentry.typed": "Son {time}.",
  "p.timeentry.f_category": "Categoría",
  "p.timeentry.category_unclassified": "Sin clasificar",
  "p.timeentry.category_capex": "Gasto de capital (capex)",
  "p.timeentry.category_opex": "Gasto operativo (opex)",
  "p.timeentry.f_billable": "Facturable",
  "p.timeentry.f_note": "Nota (opcional)",
  "p.timeentry.record": "Registrar este tiempo",
  "p.timeentry.confirm":
    "¿Registrar {time} el {day} en {plan}? Un registro no se puede cambiar ni quitar una vez hecho.",
  "p.timeentry.recorded": "Se registró {time} el {day} en {plan}.",
  "p.timeentry.problem_plan": "Elija un plan.",
  "p.timeentry.problem_date": "Indique la fecha en que se dedicó el tiempo.",
  "p.timeentry.problem_locked":
    "Esa semana está aprobada y bloqueada, así que no admite más tiempo.",
  "p.timeentry.problem_duration":
    "Indique el tiempo en horas enteras (0 a 24) y minutos (0 a 59), de 1 a {max} minutos en total.",
  "p.timeentry.refused_locked":
    "Esa semana está aprobada y bloqueada, así que no admite más tiempo. No se registró nada.",
  "p.timeentry.refused_invalid":
    "No se aceptó el registro: su tiempo, o su tarea, no es uno que el plan admita. No se registró nada.",
  "p.timeentry.refused_plan": "Ese plan ya no existe o no puede leerlo.",
  "p.timeentry.refused_forbidden": "No puede registrar tiempo en este plan.",
  "p.timeentry.could_not_load_plans": "No se pudieron leer los planes.",
  "p.timeentry.could_not_load_plan":
    "No se pudieron leer las tareas y el tiempo de ese plan.",
  "p.timeentry.could_not_record": "No se pudo registrar el tiempo.",
  "p.timeentry.yours_heading": "Su tiempo en {plan}",
  "p.timeentry.yours_none": "No ha registrado tiempo en este plan.",
  "p.timeentry.yours_table":
    "Sus registros de tiempo en {plan}, los más recientes primero",
  "p.timeentry.col_date": "Fecha",
  "p.timeentry.col_time": "Tiempo",
  "p.timeentry.col_task": "Tarea",
  "p.timeentry.col_category": "Categoría",
  "p.timeentry.col_billable": "Facturable",
  "p.timeentry.col_note": "Nota",
  "p.timeentry.billable_yes": "Facturable",
  "p.timeentry.billable_no": "No facturable",
  "p.timereport.heading": "Informe de tiempo",
  "p.timereport.says":
    "Tiempo registrado en los planes que puede leer, agrupado como elija, con lo facturable y lo no facturable siempre aparte. Deje las fechas vacías para los últimos 90 días hasta hoy. Una persona es una referencia person: o worker:; déjela vacía para todos.",
  "p.timereport.form": "Pedir un informe de tiempo",
  "p.timereport.f_group_by": "Agrupar por",
  "p.timereport.by_plan": "Por plan",
  "p.timereport.by_person": "Por persona",
  "p.timereport.by_category": "Por categoría",
  "p.timereport.by_billable": "Facturable o no",
  "p.timereport.by_month": "Por mes",
  "p.timereport.f_from": "Desde",
  "p.timereport.f_to": "Hasta",
  "p.timereport.f_plan": "Solo este plan",
  "p.timereport.every_plan": "Todos los planes que puede leer",
  "p.timereport.loading_plans": "Leyendo los planes…",
  "p.timereport.f_person": "Persona (una referencia)",
  "p.timereport.show": "Mostrar el informe de tiempo",
  "p.timereport.problem_from": "Indique el primer día como una fecha.",
  "p.timereport.problem_to": "Indique el último día como una fecha.",
  "p.timereport.problem_order":
    "El primer día no puede ser posterior al último.",
  "p.timereport.problem_person":
    "Una persona es person: o worker: y un identificador, como person:6f1c2a3b-0000-4000-8000-000000000009.",
  "p.timereport.refused":
    "No se aceptó el informe de tiempo con esas opciones.",
  "p.timereport.window": "Tiempo registrado {range}.",
  "p.timereport.no_entries":
    "No se registró tiempo en estos planes en ese intervalo.",
  "p.timereport.table": "Tiempo por grupo",
  "p.timereport.col_entries": "Registros",
  "p.timereport.col_billable": "Facturable",
  "p.timereport.col_not_billable": "No facturable",
  "p.timereport.col_total": "En total",
  "p.timereport.total":
    "En total: {billable} facturable, {not_billable} no facturable, {total} en conjunto.",
  "p.timereport.omitted_one":
    "Se omite {n} registro de un plan que no puede leer.",
  "p.timereport.omitted_other":
    "Se omiten {n} registros de planes que no puede leer.",
  "p.timereport.truncated":
    "Solo se leyeron los primeros {max} registros, así que el informe está incompleto.",
  "p.timereport.csv": "Descargar este informe de tiempo como CSV",
  "p.timereport.billable": "Facturable",
  "p.timereport.not_billable": "No facturable",
  "p.timereport.plan_unnamed": "Un plan sin nombre aquí ({ref})",
  "p.timereport.could_not_load": "No se pudo leer el informe de tiempo.",
  "p.timereport.could_not_load_plans":
    "No se pudieron leer los planes para elegir.",
  "p.hammock.heading": "Tareas hamaca: tareas resumen que abarcan otras",
  "p.hammock.says":
    "Una tarea hamaca es un resumen cuyas fechas salen de las tareas de este plan que enlaza, desde el inicio más temprano hasta el fin más tardío. No está en la red de dependencias, así que nunca cambia la holgura de ninguna tarea ni el fin. Una tarea solo se convierte en hamaca si no tiene duración, trabajo, restricción ni dependencia, y se borra su inicio previsto; volver a hacerla tarea quita todos sus enlaces. Una hamaca enlaza una tarea u otra hamaca de este plan, no a sí misma, no dos veces y no en bucle.",
  "p.hammock.none": "Este plan no tiene tareas hamaca.",
  "p.hammock.spans": "Abarca {range}.",
  "p.hammock.no_links": "Aún sin fechas: no enlaza ninguna tarea.",
  "p.hammock.members_unscheduled":
    "Aún sin fechas: ninguna de las tareas que enlaza tiene fechas.",
  "p.hammock.untitled": "Una tarea sin título",
  "p.hammock.member_hammock": "{title} (una hamaca)",
  "p.hammock.unmake_named": "Volver a hacer de {title} una tarea",
  "p.hammock.unlink": "Quitar el enlace",
  "p.hammock.unlink_named": "Quitar el enlace de {hammock} a {title}",
  "p.hammock.make_form": "Convertir una tarea en hamaca",
  "p.hammock.f_task": "Tarea",
  "p.hammock.choose_task": "Elija una tarea",
  "p.hammock.make": "Convertirla en hamaca",
  "p.hammock.link_form": "Enlazar una hamaca con una tarea",
  "p.hammock.f_hammock": "Hamaca",
  "p.hammock.choose_hammock": "Elija una hamaca",
  "p.hammock.f_member": "Tarea que abarca",
  "p.hammock.choose_member": "Elija una tarea que abarcar",
  "p.hammock.link": "Añadir el enlace",
  "p.hammock.confirm_make":
    "¿Convertir {title} en hamaca? Se borra su inicio previsto, y sus fechas pasan a salir de las tareas que enlaza.",
  "p.hammock.confirm_unmake_one":
    "¿Volver a hacer de {title} una tarea? Se va su {n} enlace.",
  "p.hammock.confirm_unmake_other":
    "¿Volver a hacer de {title} una tarea? Se van sus {n} enlaces.",
  "p.hammock.confirm_unlink":
    "¿Quitar el enlace de {hammock} a {title}? La tarea en sí no cambia.",
  "p.hammock.made": "{title} ya es una hamaca.",
  "p.hammock.unmade": "{title} vuelve a ser una tarea.",
  "p.hammock.linked": "{hammock} ahora abarca {title}.",
  "p.hammock.unlinked": "{hammock} ya no abarca {title}.",
  "p.hammock.problem_choose_task":
    "Elija una tarea para convertirla en hamaca.",
  "p.hammock.problem_carries_work":
    "Esta tarea tiene duración, trabajo o una restricción: bórrelos antes en el cronograma.",
  "p.hammock.problem_has_dependencies":
    "Esta tarea tiene dependencias: quítelas antes, pues una hamaca no admite ninguna.",
  "p.hammock.problem_choose_hammock": "Elija una hamaca.",
  "p.hammock.problem_choose_member": "Elija una tarea que la hamaca abarque.",
  "p.hammock.problem_self": "Una hamaca no puede abarcarse a sí misma.",
  "p.hammock.problem_linked": "Esta hamaca ya abarca esa tarea.",
  "p.hammock.problem_loop":
    "Esa hamaca ya abarca esta, así que el enlace formaría un bucle.",
  "p.hammock.refused_make":
    "El servicio no convirtió esta tarea en hamaca: aún tiene duración, trabajo, una restricción o una dependencia.",
  "p.hammock.refused_link":
    "El servicio no añadió el enlace: puede que la tarea ya no esté en este plan, ya esté abarcada o forme parte de un bucle.",
  "p.hammock.refused_gone":
    "La tarea o la hamaca ya no existe, o ya no puede leerla.",
  "p.hammock.refused_link_gone": "Ese enlace ya no existe.",
  "p.hammock.could_not_load": "No se pudieron leer las tareas hamaca.",
  "p.hammock.could_not_save": "No se pudo cambiar la hamaca.",
  "p.crossplan.heading": "Tareas de planes enlazados",
  "p.crossplan.says":
    "Estas son las tareas de otros planes que puede leer y con las que están enlazadas tareas de este plan, programadas con el calendario de su propio plan. Una barra discontinua es una tarea fuera del camino crítico; una barra sólida y un rombo marcan una en él.",
  "p.crossplan.network_finish":
    "Con los planes enlazados, la red termina el {date}.",
  "p.crossplan.truncated":
    "No se muestran todos los planes enlazados: la red llegó a su límite de planes o tareas.",
  "p.crossplan.links_heading": "Enlaces que cruzan a otro plan",
  "p.crossplan.lag": "retardo de {days} días laborables",
  "p.crossplan.timeline": "Tareas de {plan} en una línea de tiempo",
  "p.crossplan.table": "Fechas de las tareas de planes enlazados",
  "p.crossplan.col_task": "Tarea",
  "p.crossplan.col_plan": "Proyecto",
  "p.crossplan.col_start": "Inicio más temprano",
  "p.crossplan.col_finish": "Fin más temprano",
  "p.crossplan.col_critical": "En el camino crítico",
  "p.crossplan.yes": "Sí",
  "p.crossplan.no": "No es así",
  "p.crossplan.untitled": "Una tarea sin título",
  "p.crossplan.kind_fs": "fin a inicio",
  "p.crossplan.kind_ss": "inicio a inicio",
  "p.crossplan.kind_ff": "fin a fin",
  "p.crossplan.kind_sf": "inicio a fin",
  "p.crossplan.task_heading": "Dependencias, en este plan o en otro",
  "p.crossplan.task_says":
    "Una dependencia une esta tarea con una de otro plan que puede leer. Su tipo es fin a inicio salvo que elija otro; su retardo son días laborables enteros de menos {max} a {max}, y uno negativo es un adelanto. No se crea dos veces en el mismo sentido, ni en bucle entre planes, ni con una hamaca. Una cuyo otro extremo no puede leer se cuenta, nunca se nombra, y queda fuera del cronograma.",
  "p.crossplan.no_links": "Esta tarea aún no tiene dependencias.",
  "p.crossplan.this_before": "Esta tarea va antes de {other}",
  "p.crossplan.this_after": "Esta tarea va después de {other}",
  "p.crossplan.end_own": "{title}, en este plan",
  "p.crossplan.end_external": "{title}, en {plan}",
  "p.crossplan.end_unnamed": "una tarea no nombrada aquí ({ref})",
  "p.crossplan.remove": "Quitar",
  "p.crossplan.remove_named": "Quitar la dependencia de {other}",
  "p.crossplan.restricted_one":
    "{n} dependencia más une con una tarea que no puede leer.",
  "p.crossplan.restricted_other":
    "{n} dependencias más unen con tareas que no puede leer.",
  "p.crossplan.see_network": "Ver el cronograma entre planes",
  "p.crossplan.form": "Añadir una dependencia de una tarea de otro plan",
  "p.crossplan.f_order": "Orden",
  "p.crossplan.order_before": "Esta tarea va primero",
  "p.crossplan.order_after": "La otra tarea va primero",
  "p.crossplan.f_plan": "Otro plan",
  "p.crossplan.choose_plan": "Elija un plan",
  "p.crossplan.f_task": "Tarea de ese plan",
  "p.crossplan.choose_task": "Elija una tarea",
  "p.crossplan.f_kind": "Tipo de dependencia",
  "p.crossplan.f_lag": "Retardo en días laborables (0 si está vacío)",
  "p.crossplan.add": "Añadir la dependencia",
  "p.crossplan.added": "Se añadió una dependencia de {title}.",
  "p.crossplan.removed": "Se quitó la dependencia.",
  "p.crossplan.confirm_remove_before":
    "¿Quitar la dependencia: {task} va antes de {other}? Ninguna tarea cambia.",
  "p.crossplan.confirm_remove_after":
    "¿Quitar la dependencia: {task} va después de {other}? Ninguna tarea cambia.",
  "p.crossplan.problem_plan": "Elija el otro plan.",
  "p.crossplan.problem_task": "Elija una tarea de ese plan.",
  "p.crossplan.problem_lag":
    "Un retardo es un número entero de días laborables de menos {max} a {max}.",
  "p.crossplan.problem_self": "Una tarea no puede depender de sí misma.",
  "p.crossplan.problem_exists": "Esta dependencia ya existe.",
  "p.crossplan.refused_unreadable":
    "Una de las dos tareas ya no existe, o no puede leerla.",
  "p.crossplan.refused_exists": "Esta dependencia ya existe.",
  "p.crossplan.refused_add":
    "El servicio no añadió la dependencia: puede que ya exista, forme un bucle o una con una hamaca.",
  "p.crossplan.refused_gone":
    "Esa dependencia ya no existe, o ya no puede leer ambas tareas.",
  "p.crossplan.could_not_load":
    "No se pudieron leer las dependencias de esta tarea.",
  "p.crossplan.could_not_load_plans":
    "No se pudieron leer los planes para elegir.",
  "p.crossplan.could_not_load_tasks":
    "No se pudieron leer las tareas de ese plan.",
  "p.crossplan.could_not_save": "No se pudo cambiar la dependencia.",
  "p.taskrollup.heading": "Subtareas agregadas",
  "p.taskrollup.says":
    "Calculado cada vez a partir de las subtareas de esta tarea, nunca guardado: la fecha de vencimiento más tardía entre ellas, sus puntos sumados, el avance ponderado solo por las estimadas y el tiempo registrado en todas.",
  "p.taskrollup.subtasks": "Subtareas",
  "p.taskrollup.done_of_one": "{done} de {n} hecha",
  "p.taskrollup.done_of_other": "{done} de {n} hechas",
  "p.taskrollup.due": "Vencimiento más tardío",
  "p.taskrollup.points": "Puntos sumados",
  "p.taskrollup.progress": "Avance",
  "p.taskrollup.spent": "Tiempo registrado",
  "p.taskrollup.no_children": "Esta tarea no tiene subtareas.",
  "p.taskrollup.no_dated_children":
    "Ninguna de sus subtareas tiene fecha de vencimiento.",
  "p.taskrollup.no_estimates": "Ninguna de sus subtareas está estimada.",
  "p.taskrollup.absent": "Se desconoce.",
  "p.taskrollup.unestimated_one":
    "{n} subtarea no tiene estimación, así que queda fuera del avance (no cuenta como cero):",
  "p.taskrollup.unestimated_other":
    "{n} subtareas no tienen estimación, así que quedan fuera del avance (no cuentan como cero):",
  "p.taskrollup.unnamed": "Una subtarea no nombrada aquí ({ref})",
  "p.taskrollup.refused_gone": "Esta tarea ya no existe, o ya no puede leerla.",
  "p.taskrollup.could_not_load": "No se pudieron leer las subtareas agregadas.",
  "p.readings.heading": "Más sobre este plan",
  "p.readings.says": "Cada una de estas lecturas se hace solo cuando la pide.",
  "p.readings.read": "Leer",
  "p.readings.could_not_read": "No se pudo leer esto.",
  "p.readings.not_measured": "sin medir",
  "p.readings.days": "{n} días",
  "p.readings.rollup_heading": "Flujo en los planes que contiene",
  "p.readings.rollup_says":
    "Las tareas de este plan y de cada plan por debajo, contadas juntas, no promediadas; hasta {max} niveles si se deja vacío.",
  "p.readings.f_depth": "Niveles hacia abajo",
  "p.readings.rollup_plans_one": "{n} plan contado.",
  "p.readings.rollup_plans_other": "{n} planes.",
  "p.readings.rollup_tasks_one":
    "{n} tarea: {finished} terminadas, {wip} en curso, {not_started} sin empezar.",
  "p.readings.rollup_tasks_other":
    "{n} tareas: {finished} terminadas, {wip} en curso, {not_started} sin empezar.",
  "p.readings.rollup_times":
    "La mayor parte del trabajo (85 de cada 100) terminó en {cycle} desde que empezó y en {lead} desde que se pidió; tiempo trabajando y no esperando: {efficiency}.",
  "p.readings.rollup_truncated":
    "No se contaron todos los planes por debajo: el recorrido llegó a su límite.",
  "p.readings.rollup_revisits_one":
    "{n} plan se alcanza dos veces y se cuenta una.",
  "p.readings.rollup_revisits_other":
    "{n} planes se alcanzan dos veces y se cuentan una.",
  "p.readings.rollup_table": "Flujo por plan",
  "p.readings.col_plan": "Proyecto",
  "p.readings.col_depth": "Nivel",
  "p.readings.col_tasks": "Tareas",
  "p.readings.col_finished": "Terminadas",
  "p.readings.col_wip": "En curso",
  "p.readings.col_efficiency": "Tiempo trabajando y no esperando",
  "p.readings.effort_heading": "Tiempo registrado",
  "p.readings.effort_says":
    "El tiempo que las personas registraron en este plan, tal como lo declararon; nada lo comprueba.",
  "p.readings.read_effort": "Leer el tiempo registrado",
  "p.readings.effort_none": "No hay tiempo registrado en este plan.",
  "p.readings.effort_total":
    "{total} registrado, de lo cual {billable} facturable.",
  "p.readings.effort_split":
    "Gasto de capital {capex}, gasto operativo {opex}, sin clasificar {unclassified}.",
  "p.readings.effort_by_person": "Tiempo por persona",
  "p.readings.effort_by_task": "Tiempo por tarea",
  "p.readings.col_person": "Persona",
  "p.readings.col_time": "Tiempo",
  "p.readings.col_billable": "Facturable",
  "p.readings.col_task": "Tarea",
  "p.readings.task_unnamed": "Una tarea no nombrada aquí ({ref})",
  "p.readings.forecast_heading": "Previsión de entrega",
  "p.readings.forecast_says":
    "Diez mil simulaciones a partir de cuánto terminó este plan en cada periodo pasado: cuánto puede tardar el trabajo abierto y cuánto puede hacerse en un número de periodos. Una casilla vacía es el valor por defecto; hacen falta al menos seis periodos de historial.",
  "p.readings.f_items": "Elementos por terminar (los abiertos si está vacío)",
  "p.readings.f_periods": "Periodos por delante ({n} si está vacío)",
  "p.readings.f_period_days": "Días por periodo ({n} si está vacío)",
  "p.readings.f_history": "Periodos de historial ({n} si está vacío)",
  "p.readings.read_forecast": "Leer la previsión",
  "p.readings.forecast_history_one": "A partir de {n} periodo de {days} días.",
  "p.readings.forecast_history_other":
    "A partir de {n} periodos de {days} días.",
  "p.readings.forecast_batch_one":
    "{n} elemento: la mitad de las simulaciones termina en {p50} días, 85 de cada 100 en {p85} días y 95 de cada 100 en {p95} días.",
  "p.readings.forecast_batch_other":
    "{n} elementos: la mitad de las simulaciones termina en {p50} días, 85 de cada 100 en {p85} días y 95 de cada 100 en {p95} días.",
  "p.readings.forecast_no_batch":
    "No se puede prever cuánto tardará el trabajo: hay poco historial, o no se terminó nada en él.",
  "p.readings.forecast_horizon":
    "En los próximos {days} días pueden hacerse unos {median} elementos: al menos {least} en 85 de cada 100 simulaciones, y como mucho {most}.",
  "p.readings.forecast_no_horizon":
    "No se puede prever cuánto se hará: hay poco historial, o no se terminó nada en él.",
  "p.readings.performance_heading": "Plazo, coste y partes interesadas",
  "p.readings.performance_says":
    "El rendimiento del plazo es el trabajo hecho frente al previsto hasta ahora, y el del coste el trabajo hecho frente a lo que costó; por encima del 100 % es mejor que lo previsto. La puntuación de las partes interesadas va de menos 100 a 100.",
  "p.readings.read_performance": "Leer el rendimiento",
  "p.readings.schedule": "Rendimiento del plazo",
  "p.readings.cost": "Rendimiento del coste",
  "p.readings.stakeholders": "Puntuación de las partes interesadas",
  "p.readings.nps_one":
    "{score}, de {n} respuesta ({promoters} promotores, {detractors} detractores)",
  "p.readings.nps_other":
    "{score}, de {n} respuestas ({promoters} promotores, {detractors} detractores)",
  "p.readings.nps_none": "Aún no hay respuestas.",
  "p.readings.earned_value_link": "Ver el valor ganado completo",
  "p.readings.absent_no_baseline":
    "Sin medir: el plan no tiene línea base de presupuesto.",
  "p.readings.absent_no_weighted_tasks": "Sin medir: ninguna tarea tiene peso.",
  "p.readings.absent_not_yet_defined":
    "Aún sin medir: no se ha previsto ni gastado nada.",
  "p.readings.absent_several_currencies":
    "No hay una sola cifra: el plan tiene presupuestos en varias monedas; vea el valor ganado.",
  "p.readings.absent_other": "Sin medir.",
  "p.readings.workflow_heading": "Flujo de trabajo de las tareas",
  "p.readings.read_workflow": "Leer el flujo de trabajo de las tareas",
  "p.readings.workflow_plan":
    "Este plan tiene su propio flujo de trabajo de tareas.",
  "p.readings.workflow_built_in":
    "Este plan usa el flujo de trabajo de tareas integrado.",
  "p.readings.workflow_constrained": "Solo se permiten los cambios listados.",
  "p.readings.workflow_free":
    "Una tarea puede pasar de cualquier estado a cualquier otro.",
  "p.readings.cat_todo": "sin empezar",
  "p.readings.cat_active": "en curso",
  "p.readings.cat_waiting": "en espera",
  "p.readings.cat_done": "terminada",
  "p.readings.cat_other": "otro tipo",
  "p.readings.initial": "donde empieza una tarea",
  "p.readings.terminal": "donde acaba una tarea",
  "p.readings.wip_limit": "como mucho {n} a la vez",
  "p.readings.anyone": "cualquiera",
  "p.readings.roles": "funciones: {roles}",
  "p.readings.problem_depth":
    "Los niveles hacia abajo son un número entero de 1 a {max}.",
  "p.readings.problem_depth_zero":
    "Los niveles hacia abajo son un número entero de 0 a {max}.",
  "p.readings.problem_items":
    "Los elementos por terminar son un número entero de al menos 1.",
  "p.readings.problem_periods":
    "Los periodos por delante son un número entero de 1 a 520.",
  "p.readings.problem_period_days":
    "Los días por periodo son un número entero de 1 a 90.",
  "p.readings.problem_history":
    "Los periodos de historial son un número entero de 6 a 260.",
  "p.readings.problem_window":
    "La ventana es un número entero de días de 1 a 3650.",
  "p.readings.refused_query": "El servicio no aceptó esa consulta.",
  "p.readings.refused_module": "Esta parte del plan está desactivada.",
  "p.readings.refused_gone": "Este plan ya no existe, o ya no puede leerlo.",
  "p.readings.flow_feature": "Funciones nuevas",
  "p.readings.flow_defect": "Defectos",
  "p.readings.flow_risk": "Trabajo de riesgos",
  "p.readings.flow_debt": "Deuda técnica",
  "p.readings.flow_unclassified": "Sin clasificar",
  "p.readings.flow_other": "Otro tipo",
  "p.flowmix.heading": "La mezcla del trabajo terminado",
  "p.flowmix.says":
    "Cómo se reparte el trabajo terminado en una ventana entre funciones nuevas, defectos, trabajo de riesgos y deuda técnica, frente a la mezcla que el plan pretende donde la declaró. La ventana es de {n} días si se deja vacía.",
  "p.flowmix.f_window": "Días hacia atrás ({n} si está vacío)",
  "p.flowmix.f_rollup": "Incluir los planes que contiene",
  "p.flowmix.f_depth": "Niveles hacia abajo ({max} si está vacío)",
  "p.flowmix.read": "Leer la mezcla",
  "p.flowmix.window_one": "Trabajo terminado en el último {n} día.",
  "p.flowmix.window_other": "Trabajo terminado en los últimos {n} días.",
  "p.flowmix.plans_one": "{n} plan contado.",
  "p.flowmix.plans_other": "{n} planes contados.",
  "p.flowmix.truncated":
    "No se contaron todos los planes por debajo: el recorrido llegó a su límite.",
  "p.flowmix.none": "No se terminó trabajo en esa ventana.",
  "p.flowmix.no_intent":
    "El plan no declara una mezcla prevista, así que no hay con qué comparar.",
  "p.flowmix.table": "Trabajo terminado por tipo",
  "p.flowmix.col_kind": "Tipo de trabajo",
  "p.flowmix.col_count": "Terminado",
  "p.flowmix.col_share": "Proporción",
  "p.flowmix.col_intended": "Proporción prevista",
  "p.flowmix.col_gap": "Diferencia",
  "p.ccpm.heading": "Estimaciones agresiva y segura",
  "p.ccpm.says":
    "Para la cadena crítica cada tarea necesita una estimación agresiva (probabilidad de más o menos la mitad) y una segura, en días laborables enteros de 1 a {max}, sin que la agresiva supere la segura. Una cifra mostrada es la registrada, si no la del rango de riesgo, si no una que deriva la regla de la instalación, y cada una dice cuál. Un hito y una tarea sin duración no se estiman. Vaciar ambas quita lo registrado.",
  "p.ccpm.rule_explicit_only":
    "Regla: solo se usan las estimaciones dadas; no se deriva nada.",
  "p.ccpm.rule_half_of_safe":
    "Regla: una estimación segura que falta es la duración de la tarea, y una agresiva que falta la mitad de la segura.",
  "p.ccpm.rule_other": "Regla: una que esta página no conoce.",
  "p.ccpm.rule_unusable":
    "No se pudo leer el ajuste de la regla de la instalación, así que solo se usan las estimaciones dadas.",
  "p.ccpm.attention_one": "{n} tarea necesita sus estimaciones.",
  "p.ccpm.attention_other": "{n} tareas necesitan sus estimaciones.",
  "p.ccpm.more": "Se muestran las primeras {shown} de {total} tareas.",
  "p.ccpm.no_tasks": "Este plan no tiene tareas que estimar.",
  "p.ccpm.table": "Estimaciones por tarea",
  "p.ccpm.col_task": "Tarea",
  "p.ccpm.col_likely": "Duración en días laborables",
  "p.ccpm.col_aggressive": "Agresiva",
  "p.ccpm.col_safe": "Segura",
  "p.ccpm.col_state": "Situación",
  "p.ccpm.figure": "{days} días, {source}",
  "p.ccpm.none": "ninguna",
  "p.ccpm.source_recorded": "registrada",
  "p.ccpm.source_risk_range": "del rango de riesgo",
  "p.ccpm.source_rule": "por la regla",
  "p.ccpm.state_complete": "Ambas dadas",
  "p.ccpm.state_derived": "En parte derivada",
  "p.ccpm.state_missing": "Falta una estimación",
  "p.ccpm.state_inconsistent": "Agresiva por encima de la segura",
  "p.ccpm.state_milestone": "Un hito, no se estima",
  "p.ccpm.state_no_duration": "Sin duración, no se estima",
  "p.ccpm.state_other": "Otra situación",
  "p.ccpm.form": "Registrar las estimaciones de una tarea",
  "p.ccpm.f_task": "Tarea",
  "p.ccpm.choose_task": "Elija una tarea",
  "p.ccpm.f_aggressive": "Agresiva, en días laborables",
  "p.ccpm.f_safe": "Segura, en días laborables",
  "p.ccpm.save": "Guardar las estimaciones",
  "p.ccpm.unchanged": "No cambió nada, así que no se envió nada.",
  "p.ccpm.saved": "Se guardaron las estimaciones de {title}.",
  "p.ccpm.cleared": "Se vaciaron las estimaciones registradas de {title}.",
  "p.ccpm.confirm_clear":
    "¿Vaciar las estimaciones registradas de {title}? Puede seguir mostrándose una cifra del rango de riesgo o de la regla.",
  "p.ccpm.problem_task": "Elija una tarea.",
  "p.ccpm.problem_aggressive":
    "La estimación agresiva es un número entero de días laborables de 1 a {max}.",
  "p.ccpm.problem_safe":
    "La estimación segura es un número entero de días laborables de 1 a {max}.",
  "p.ccpm.problem_order": "La estimación agresiva no puede superar la segura.",
  "p.ccpm.refused_invalid":
    "El servicio no aceptó estas estimaciones: puede que la tarea haya perdido su duración o se haya convertido en hito.",
  "p.ccpm.refused_gone": "Esta tarea ya no existe, o ya no puede leerla.",
  "p.ccpm.could_not_load": "No se pudieron leer las estimaciones.",
  "p.ccpm.could_not_save": "No se pudieron guardar las estimaciones.",
  "p.frozen.heading": "El plan de colchones congelado",
  "p.frozen.none": "Aún no se ha congelado ningún plan de colchones.",
  "p.frozen.freeze_link": "Congelar uno en la gestión de colchones",
  "p.frozen.taken": "Congelado el {date}; {rule}",
  "p.frozen.table": "Colchones tal como se congelaron",
  "p.frozen.col_buffer": "Colchón",
  "p.frozen.col_days": "Días laborables",
  "p.frozen.col_chain": "Cadena",
  "p.frozen.col_finish": "Fin previsto",
  "p.frozen.project": "Colchón del proyecto",
  "p.frozen.feeding": "Colchón de alimentación hacia {joins}",
  "p.frozen.unsized": "sin dimensionar",
  "p.frozen.untitled": "Una tarea sin título",
  "p.frozen.history_one": "{n} plan de colchones congelado: {dates}.",
  "p.frozen.history_other":
    "{n} planes de colchones congelados, el más reciente primero: {dates}.",
  "p.frozen.could_not_load": "No se pudo leer el plan de colchones congelado.",
  "p.workingtime.heading": "La jornada y la semana laborables",
  "p.workingtime.says":
    "Cuánto dura una jornada (de 1 minuto a 24 horas) y cuántos días a la semana se trabaja (de 1 a 7), para este plan o como valor por defecto de la organización. Cada uno es un registro nuevo que aquí no puede cambiarse ni quitarse, así que se le pregunta antes; un segundo para este plan se rechaza.",
  "p.workingtime.form": "Fijar la jornada y la semana laborables",
  "p.workingtime.f_scope": "Para",
  "p.workingtime.scope_plan": "Este plan",
  "p.workingtime.scope_default": "El valor por defecto de la organización",
  "p.workingtime.f_hours": "Horas al día",
  "p.workingtime.f_minutes": "Y minutos",
  "p.workingtime.f_days": "Días a la semana",
  "p.workingtime.save": "Fijar la jornada",
  "p.workingtime.confirm_plan_one":
    "¿Fijar la jornada de este plan en {day}, {days} día a la semana? Después no podrá cambiarse aquí.",
  "p.workingtime.confirm_plan_other":
    "¿Fijar la jornada de este plan en {day}, {days} días a la semana? Después no podrá cambiarse aquí.",
  "p.workingtime.confirm_default_one":
    "¿Fijar la jornada por defecto de la organización en {day}, {days} día a la semana? Después no podrá cambiarse aquí.",
  "p.workingtime.confirm_default_other":
    "¿Fijar la jornada por defecto de la organización en {day}, {days} días a la semana? Después no podrá cambiarse aquí.",
  "p.workingtime.saved_one":
    "Se fijó una jornada de {day}, {days} día a la semana.",
  "p.workingtime.saved_other":
    "Se fijó una jornada de {day}, {days} días a la semana.",
  "p.workingtime.away_heading": "Tiempo fuera del trabajo del proyecto",
  "p.workingtime.away_says":
    "Vacaciones, un festivo, un permiso de estudios o una tarea ajena al proyecto, para una persona de un primer a un último día, para que ese tiempo no se lea como inactivo. La persona es una referencia como person: y un identificador. No puede cambiarse ni quitarse una vez registrado, así que se le pregunta antes; una nota vacía no se envía.",
  "p.workingtime.f_person": "Referencia de la persona",
  "p.workingtime.f_starts": "Primer día",
  "p.workingtime.f_ends": "Último día",
  "p.workingtime.f_kind": "Tipo",
  "p.workingtime.f_note": "Nota (opcional)",
  "p.workingtime.record": "Registrar el tiempo fuera",
  "p.workingtime.confirm_away":
    "¿Registrar {kind} para {person}, {range}? Después no podrá cambiarse ni quitarse.",
  "p.workingtime.recorded": "Se registró {kind} para {person}, {range}.",
  "p.workingtime.kind_leave": "vacaciones",
  "p.workingtime.kind_holiday": "un festivo",
  "p.workingtime.kind_study_leave": "un permiso de estudios",
  "p.workingtime.kind_non_project_duty": "una tarea ajena al proyecto",
  "p.workingtime.problem_hours": "Las horas son un número entero de 0 a 24.",
  "p.workingtime.problem_minutes":
    "Los minutos son un número entero de 0 a 59.",
  "p.workingtime.problem_day": "Una jornada dura de 1 minuto a 24 horas.",
  "p.workingtime.problem_week":
    "Los días a la semana son un número entero de 1 a 7.",
  "p.workingtime.problem_person":
    "Indique la persona como person: o worker: y un identificador.",
  "p.workingtime.problem_starts": "Indique el primer día.",
  "p.workingtime.problem_ends": "Indique el último día.",
  "p.workingtime.problem_order":
    "El último día no puede ser anterior al primero.",
  "p.workingtime.refused_exists":
    "Este plan ya tiene una jornada fijada, y aquí no puede sustituirse.",
  "p.workingtime.refused_working": "El servicio no aceptó esta jornada.",
  "p.workingtime.refused_non_working":
    "El servicio no aceptó este tiempo fuera.",
  "p.workingtime.refused_forbidden": "No puede registrar esto.",
  "p.workingtime.could_not_save": "No se pudo registrar esto.",
  "p.task.is_hammock":
    "Esta tarea es una hamaca: sus fechas salen de las tareas que abarca.",
  "p.task.hammock_link": "Ver su alcance en el camino crítico",
  "p.adoption.could_not": "No se pudo registrar la adopción.",
  "p.adoption.f_active": "Usuarios activos",
  "p.adoption.f_definition": "Qué contó como usuario activo",
  "p.adoption.f_target": "Usuarios a los que va dirigido",
  "p.adoption.f_window": "Durante cuántos días",
  "p.adoption.form": "Registrar la adopción",
  "p.adoption.heading": "Cuántas personas lo usan",
  "p.adoption.problem_active":
    "Indique los usuarios activos como número entero, 0 o más.",
  "p.adoption.problem_definition":
    "Diga qué contó como usuario activo, para comparar lecturas posteriores.",
  "p.adoption.problem_target":
    "Indique los usuarios a los que va dirigido como número entero mayor que 0.",
  "p.adoption.problem_window":
    "Indique el periodo como número entero de días, 1 o más.",
  "p.adoption.record": "Registrar esta adopción",
  "p.adoption.refused": "El servicio no aceptó estas cifras.",
  "p.adoption.refused_forbidden":
    "No puede registrar la adopción de este plan.",
  "p.adoption.refused_gone": "Este plan ya no existe.",
  "p.adoption.saved": "Adopción registrada.",
  "p.adoption.says":
    "Usuarios activos frente a los usuarios a los que va dirigido, durante un periodo de días. Qué contó como activo se guarda junto a la cifra, para comparar una lectura posterior.",
  "p.bulk.apply": "Aplicar a las tareas elegidas",
  "p.bulk.choose_all": "Elegir todas las tareas",
  "p.bulk.choose_none": "No elegir ninguna",
  "p.bulk.choose_plan": "Elija un plan",
  "p.bulk.chosen_one": "{n} tarea elegida",
  "p.bulk.chosen_other": "{n} tareas elegidas",
  "p.bulk.confirm_delete_one":
    "¿Eliminar {n} tarea? Sale del tablero y de todas las vistas del plan.",
  "p.bulk.confirm_delete_other":
    "¿Eliminar {n} tareas? Salen del tablero y de todas las vistas del plan.",
  "p.bulk.confirm_move_one":
    "¿Mover {n} tarea a {plan}? Se quitan su sprint, componente y tarea padre, y un estado que ese plan no tiene pasa a ser el primero.",
  "p.bulk.confirm_move_other":
    "¿Mover {n} tareas a {plan}? Se quitan su sprint, componente y tarea padre, y un estado que ese plan no tiene pasa a ser el primero.",
  "p.bulk.copy": "Copiar al plan",
  "p.bulk.could_not": "No se pudieron cambiar las tareas.",
  "p.bulk.could_not_load_plans": "No se pudieron leer los planes.",
  "p.bulk.delete": "Eliminar las tareas elegidas",
  "p.bulk.delete_failed": "Sin eliminar: {titles}.",
  "p.bulk.deleted_gone_one": "{n} tarea ya no existía.",
  "p.bulk.deleted_gone_other": "{n} tareas ya no existían.",
  "p.bulk.deleted_one": "{n} tarea eliminada.",
  "p.bulk.deleted_other": "{n} tareas eliminadas.",
  "p.bulk.done_one": "Hecho en {done} de {requested} tarea.",
  "p.bulk.done_other": "Hecho en {done} de {requested} tareas.",
  "p.bulk.edit_form": "Fijar campos en las tareas elegidas",
  "p.bulk.f_assignee": "Responsable",
  "p.bulk.f_due": "Fecha de entrega",
  "p.bulk.f_plan": "Plan de destino",
  "p.bulk.f_start": "Fecha de inicio",
  "p.bulk.f_value": "Nuevo valor: {field}",
  "p.bulk.heading": "Cambiar muchas tareas a la vez",
  "p.bulk.keep_assignee": "Conservar el responsable al copiar",
  "p.bulk.mapped_one":
    "{n} tarea tenía un estado que ese plan no tiene, así que empieza en su primer estado.",
  "p.bulk.mapped_other":
    "{n} tareas tenían un estado que ese plan no tiene, así que empiezan en su primer estado.",
  "p.bulk.mode_clear": "Vaciarlo",
  "p.bulk.mode_keep": "Dejarlo como está",
  "p.bulk.mode_set": "Fijarlo en",
  "p.bulk.move": "Mover al plan",
  "p.bulk.no_tasks": "Este plan no tiene tareas.",
  "p.bulk.problem_assignee":
    "Indique el responsable como person: o worker: y un id.",
  "p.bulk.problem_due": "Indique una fecha de entrega válida.",
  "p.bulk.problem_no_field": "Elija al menos un campo para fijar o vaciar.",
  "p.bulk.problem_none": "Elija al menos una tarea.",
  "p.bulk.problem_plan": "Elija el plan al que copiar o mover.",
  "p.bulk.problem_same_plan":
    "Las tareas ya están en este plan: elija otro al que moverlas.",
  "p.bulk.problem_start":
    "Indique una fecha de inicio válida entre {min} y {latest}.",
  "p.bulk.problem_too_many": "Elija como máximo {max} tareas a la vez.",
  "p.bulk.refused_edit": "El servicio no aceptó estos valores.",
  "p.bulk.refused_forbidden": "No puede cambiar estas tareas.",
  "p.bulk.refused_plan_gone": "Ese plan ya no existe o no puede verlo.",
  "p.bulk.refused_titles":
    "Rechazadas, por un motivo que el servicio da solo en inglés: {titles}.",
  "p.bulk.refused_transfer":
    "No puede ver ese plan, o el servicio no aceptó la lista.",
  "p.bulk.says":
    "Elija tareas y fije en todas el mismo responsable, fecha de entrega o fecha de inicio, cópielas o muévalas a un plan, o elimínelas. Como máximo {max} a la vez; la fecha de inicio va de {min} a {latest}. El estado se cambia en el tablero, tarjeta a tarjeta. Una copia empieza en el primer estado de ese plan y conserva el responsable solo si se pide; mover quita el sprint, el componente y la tarea padre, y una tarea con subtareas no se mueve.",
  "p.bulk.this_plan": "{name} (este plan)",
  "p.bulk.transfer_form": "Copiar o mover las tareas elegidas",
  "p.bulk.unchanged":
    "Las tareas elegidas ya tienen estos valores, así que no se envió nada.",
  "p.ceremony.choose_kind": "Elija una ceremonia",
  "p.ceremony.could_not_load": "No se pudieron leer las ceremonias del sprint.",
  "p.ceremony.could_not_save": "No se pudo registrar la ceremonia.",
  "p.ceremony.counts": "Ceremonias celebradas",
  "p.ceremony.f_facilitator":
    "Facilitador (person: o worker: y un id, opcional)",
  "p.ceremony.f_kind": "Ceremonia",
  "p.ceremony.f_summary": "Qué se decidió (opcional)",
  "p.ceremony.facilitated_by": "facilitada por {ref}",
  "p.ceremony.form": "Registrar una ceremonia",
  "p.ceremony.heading": "Ceremonias de {name}",
  "p.ceremony.held_count_one": "{kind}: celebrada {n} vez",
  "p.ceremony.held_count_other": "{kind}: celebrada {n} veces",
  "p.ceremony.held_saved": "Registrada: {kind}.",
  "p.ceremony.kind_daily": "Reunión diaria",
  "p.ceremony.kind_other": "Otra ceremonia",
  "p.ceremony.kind_planning": "Planificación",
  "p.ceremony.kind_retrospective": "Retrospectiva",
  "p.ceremony.kind_review": "Revisión",
  "p.ceremony.none_held": "{kind}: no celebrada",
  "p.ceremony.problem_facilitator":
    "Indique el facilitador como person: o worker: y un id, o déjelo en blanco.",
  "p.ceremony.problem_kind": "Elija la ceremonia.",
  "p.ceremony.problem_once":
    "Este sprint ya celebró su {kind}: una segunda es replanificar, que es un sprint nuevo.",
  "p.ceremony.record": "Registrar esta ceremonia",
  "p.ceremony.refused_ceremony":
    "El servicio lo rechazó: puede que este sprint ya celebrara esa ceremonia.",
  "p.ceremony.refused_forbidden": "No puede registrar esto para el sprint.",
  "p.ceremony.refused_gone": "Este sprint ya no existe.",
  "p.ceremony.says":
    "Un sprint celebra una planificación y una revisión, y cualquier número de reuniones diarias y retrospectivas. Se cuenta cada tipo, también los no celebrados.",
  "p.checkins.heading": "Seguimientos de {title}",
  "p.checkins.none": "Aún no hay seguimientos.",
  "p.checkins.row": "{date}: valor {value}",
  "p.commitment.added_one": "{n} tarea añadida desde entonces: {titles}.",
  "p.commitment.added_other": "{n} tareas añadidas desde entonces: {titles}.",
  "p.commitment.commit": "Comprometer las tareas del sprint",
  "p.commitment.confirm_one":
    "¿Comprometer ahora la {n} tarea de {name}? Se toma una vez y no puede cambiarse; las tareas añadidas o quitadas después se ven como un cambio.",
  "p.commitment.confirm_other":
    "¿Comprometer ahora las {n} tareas de {name}? Se toma una vez y no puede cambiarse; las tareas añadidas o quitadas después se ven como un cambio.",
  "p.commitment.could_not": "No se pudo comprometer el sprint.",
  "p.commitment.heading": "A qué se comprometió el sprint",
  "p.commitment.not_yet_one":
    "Aún sin comprometer; ahora hay {n} tarea en el sprint.",
  "p.commitment.not_yet_other":
    "Aún sin comprometer; ahora hay {n} tareas en el sprint.",
  "p.commitment.problem_done":
    "Este sprint ya está comprometido: un compromiso nunca se reescribe.",
  "p.commitment.problem_empty": "Este sprint no tiene tareas que comprometer.",
  "p.commitment.problem_unread": "Aún no se ha leído el compromiso del sprint.",
  "p.commitment.refused_commit":
    "El servicio lo rechazó: este sprint ya está comprometido.",
  "p.commitment.removed_one": "{n} tarea quitada desde entonces: {titles}.",
  "p.commitment.removed_other": "{n} tareas quitadas desde entonces: {titles}.",
  "p.commitment.saved_one": "{n} tarea comprometida.",
  "p.commitment.saved_other": "{n} tareas comprometidas.",
  "p.commitment.says":
    "Las tareas del sprint tal como están al comprometerse, tomadas una vez, para que una tarea añadida o quitada después se vea como un cambio y no como lo planeado desde el principio.",
  "p.commitment.state":
    "{committed} comprometidas; {current} en el sprint ahora.",
  "p.commitment.unchanged": "Nada añadido ni quitado desde entonces.",
  "p.hours.col_actual": "Registradas",
  "p.hours.col_placeholder": "De ellas sin asignar",
  "p.hours.col_plan": "Plan del proyecto",
  "p.hours.col_planned": "Planificadas",
  "p.hours.col_variance": "Registradas menos planificadas",
  "p.hours.could_not": "No se pudieron leer las horas.",
  "p.hours.f_from": "Desde",
  "p.hours.f_to": "Hasta",
  "p.hours.form": "Horas planificadas frente a registradas",
  "p.hours.fte_says":
    "Los equivalentes a tiempo completo son días-porcentaje divididos entre 100 por los días del periodo; un hueco es oferta menos demanda (bajo cero falta). Las reservas propuestas son una serie aparte.",
  "p.hours.h": "{h} h",
  "p.hours.heading": "Horas planificadas y registradas",
  "p.hours.masked": "No se muestran personas: no puede gestionarlas.",
  "p.hours.none":
    "Ningún plan tuvo horas planificadas ni registradas en este periodo.",
  "p.hours.nothing_recorded": "nada registrado",
  "p.hours.problem_from":
    "Indique el primer día como fecha válida, o déjelo en blanco.",
  "p.hours.problem_order": "El último día no puede ser anterior al primero.",
  "p.hours.problem_span": "El periodo puede tener como máximo {max} días.",
  "p.hours.problem_to":
    "Indique el último día como fecha válida, o déjelo en blanco.",
  "p.hours.read": "Leer las horas",
  "p.hours.refused": "El servicio no aceptó este periodo.",
  "p.hours.says":
    "Lo planificado es la parte de cada día laborable de una reserva firme con nombre; lo registrado es el tiempo que la gente anotó, aprobado o no. Deje los días en blanco para las cuatro semanas hasta hoy; como máximo {max} días.",
  "p.hours.table": "Horas por plan",
  "p.hours.totals": "Planificadas {planned}; registradas {actual}.",
  "p.hours.truncated":
    "Solo se leyeron las primeras entradas; los totales pueden quedarse cortos.",
  "p.hours.unplanned": "{plan}: {hours} registradas",
  "p.hours.unplanned_heading":
    "Tiempo registrado por personas sin reserva en el plan",
  "p.hours.unplanned_people_one": "({n} persona)",
  "p.hours.unplanned_people_other": "({n} personas)",
  "p.hours.window": "Del {from} al {to}.",
  "p.kpireadings.col_standing": "Situación",
  "p.kpireadings.counted": "cuenta",
  "p.kpireadings.replaced": "sustituida por una lectura posterior ese día",
  "p.kpireadings.says":
    "Todas las lecturas, las más recientes primero. Una lectura posterior del mismo día sustituye a una anterior, que se conserva.",
  "p.live.burndown_says":
    "Tareas pendientes cada día, según las finalizaciones registradas; no se dibuja línea ideal ni se rellena nada.",
  "p.live.event_comment_added": "un comentario añadido",
  "p.live.event_other": "un cambio",
  "p.live.event_raci_updated": "cambió quién hace qué",
  "p.live.event_resync": "el tablero se leyó de nuevo entero",
  "p.live.event_task_created": "una tarea añadida",
  "p.live.event_task_moved": "una tarea movida",
  "p.live.event_task_updated": "una tarea cambiada",
  "p.live.follow": "Seguir los cambios en vivo",
  "p.live.heading": "Cambios al momento",
  "p.live.last_change": "Último: {change} a las {time}.",
  "p.live.says":
    "Mientras lo sigue, el tablero se vuelve a leer cada vez que alguien añade, mueve o cambia una tarea aquí. Se detiene al salir de la página.",
  "p.live.state_connecting": "Conectando…",
  "p.live.state_off": "Sin seguir.",
  "p.live.state_on": "Siguiendo.",
  "p.live.state_retrying": "Conexión perdida; reintentando.",
  "p.live.state_stopped": "Detenido: el servicio no envía cambios.",
  "p.live.stop": "Dejar de seguir",
  "p.live.velocity_says":
    "Tareas y puntos terminados en cada sprint, contados desde el tablero; no es una medida de flujo.",
  "p.placeholder.choose": "Elija un marcador",
  "p.placeholder.confirm_over":
    "Dárselo a {person} superaría su capacidad algunos días. ¿Hacerlo de todos modos?",
  "p.placeholder.confirm_remove":
    "¿Quitar este marcador: {what}? La demanda que representaba se va con él.",
  "p.placeholder.could_not": "No se pudo cambiar el marcador.",
  "p.placeholder.f_person": "Persona (person: o worker: y un id)",
  "p.placeholder.f_placeholder": "Marcador",
  "p.placeholder.form": "Asignar un marcador a una persona",
  "p.placeholder.give": "Asignarlo a la persona",
  "p.placeholder.given": "Asignado a {person}.",
  "p.placeholder.heading": "Asignar un marcador a una persona, o quitarlo",
  "p.placeholder.not_given": "Sin asignar: no se cambió nada.",
  "p.placeholder.open": "abierto",
  "p.placeholder.option": "{role} en {plan}, {percent}, del {from} al {to}",
  "p.placeholder.problem_choose": "Elija un marcador.",
  "p.placeholder.problem_person":
    "Indique la persona como person: o worker: y un id.",
  "p.placeholder.refused_forbidden":
    "Solo un administrador o un gestor de recursos puede hacerlo.",
  "p.placeholder.refused_gone": "Esa reserva ya no existe.",
  "p.placeholder.refused_named":
    "Esta reserva ya nombra a una persona; la lista se leyó de nuevo.",
  "p.placeholder.refused_person": "El servicio no aceptó a esa persona.",
  "p.placeholder.remove": "Quitar el marcador",
  "p.placeholder.removed": "Marcador quitado.",
  "p.placeholder.says":
    "Un marcador es un rol pedido aún sin persona. Asignarlo es decisión de un gestor de recursos; si superara su capacidad, se le pregunta antes.",
  "p.planokr.as_of": "calculado {at}",
  "p.planokr.choose_plan": "Elija un plan",
  "p.planokr.could_not": "No se pudieron leer los objetivos del plan.",
  "p.planokr.could_not_load_plans": "No se pudieron leer los planes.",
  "p.planokr.currencies": "resultados clave en más de una moneda",
  "p.planokr.f_plan": "Plan a puntuar",
  "p.planokr.form": "Puntuación de objetivos de un plan",
  "p.planokr.heading": "Puntuación de objetivos de un plan",
  "p.planokr.its_key_results": "Resultados clave de {title}",
  "p.planokr.key_results_one": "{n} resultado clave",
  "p.planokr.key_results_other": "{n} resultados clave",
  "p.planokr.left_out_one":
    "{n} objetivo no se mide y queda fuera de la puntuación.",
  "p.planokr.left_out_other":
    "{n} objetivos no se miden y quedan fuera de la puntuación.",
  "p.planokr.no_objectives": "Ningún objetivo está alineado con este plan.",
  "p.planokr.not_measured":
    "Sin medir: aún no puede medirse ningún objetivo alineado.",
  "p.planokr.objective_not_measured": "sin medir",
  "p.planokr.problem_plan": "Elija un plan.",
  "p.planokr.read": "Leer la puntuación",
  "p.planokr.refused_gone": "Ese plan u objetivo ya no existe.",
  "p.planokr.says":
    "La puntuación de cada objetivo alineado con el plan, ponderada según la fuerza de la alineación. Un objetivo que nada puede medir queda fuera, nunca cuenta como 0 %.",
  "p.planokr.score": "Puntuación {score}",
  "p.planokr.unmeasurable_no_range":
    "sin avance: su inicio y su meta coinciden",
  "p.planokr.unmeasurable_no_tolerance":
    "sin avance: se mantiene sin tolerancia",
  "p.planokr.unmeasurable_other": "no puede calcularse el avance",
  "p.planokr.unmeasurable_overflow":
    "sin avance: las cifras son demasiado grandes",
  "p.planokr.unnamed": "Un objetivo ({id})",
  "p.planokr.weight": "peso {weight}",
  "p.rate.answer_person":
    "{rate} por hora el {date}, de la tabla propia de la persona, vigente desde el {from}.",
  "p.rate.answer_role":
    "{rate} por hora el {date}, de la tabla de su rol, vigente desde el {from}.",
  "p.rate.could_not": "No se pudo leer la tarifa.",
  "p.rate.f_date": "El día",
  "p.rate.f_person": "Persona",
  "p.rate.f_table": "Tabla",
  "p.rate.form": "La tarifa de coste vigente",
  "p.rate.heading": "La tarifa de coste vigente",
  "p.rate.none": "No hay tarifa vigente para esta persona el {date}.",
  "p.rate.problem_date": "Indique el día como fecha válida.",
  "p.rate.problem_person": "Indique la referencia de la persona.",
  "p.rate.read": "Leer la tarifa",
  "p.rate.refused_hidden":
    "No hay tarifa visible para usted: las tarifas solo se muestran a finanzas.",
  "p.rate.refused_query": "El servicio no aceptó esta consulta.",
  "p.rate.says":
    "La tarifa por hora vigente para una persona en un día: la de su propia tabla, si no la de su rol. La tabla estándar salvo que nombre otra.",
  "p.skillgap.choose_plan": "Elija un plan",
  "p.skillgap.col_person": "Persona",
  "p.skillgap.col_skill": "Habilidad",
  "p.skillgap.col_status": "Situación",
  "p.skillgap.could_not": "No se pudieron leer las carencias de habilidades.",
  "p.skillgap.count_covered_one": "{n} habilidad cubierta;",
  "p.skillgap.count_covered_other": "{n} habilidades cubiertas;",
  "p.skillgap.count_missing_one": "{n} habilidad falta;",
  "p.skillgap.count_missing_other": "{n} habilidades faltan;",
  "p.skillgap.count_unknown_one": "{n} desconocida.",
  "p.skillgap.count_unknown_other": "{n} desconocidas.",
  "p.skillgap.f_plan": "Plan a revisar",
  "p.skillgap.form": "Carencias de habilidades de un plan",
  "p.skillgap.heading": "Habilidades que piden las reservas",
  "p.skillgap.none": "Ninguna reserva de este plan pide una habilidad.",
  "p.skillgap.problem_plan": "Elija un plan.",
  "p.skillgap.read": "Leer las carencias",
  "p.skillgap.says":
    "Cada habilidad que pide una reserva del plan, frente a lo que su persona tiene en el servicio de plantilla. Desconocida significa que no se pudieron leer sus habilidades; un marcador aún no tiene persona y queda fuera.",
  "p.skillgap.status_covered": "cubierta",
  "p.skillgap.status_missing": "falta",
  "p.skillgap.status_unknown": "desconocida",
  "p.skillgap.table": "Habilidades por reserva",
  "p.util.by_person": "Por persona",
  "p.util.by_team": "El equipo",
  "p.util.col_available": "Disponible",
  "p.util.col_away": "Ausente",
  "p.util.col_effort": "Esfuerzo",
  "p.util.col_figure": "Uso de la capacidad",
  "p.util.col_person": "Persona",
  "p.util.could_not": "No se pudo leer el uso de la capacidad.",
  "p.util.every_plan": "Todos los planes",
  "p.util.f_by": "Mostrar",
  "p.util.f_plan": "Solo el plan",
  "p.util.f_window": "Días hacia atrás desde hoy",
  "p.util.form": "Esfuerzo frente a capacidad",
  "p.util.heading": "Esfuerzo frente a capacidad",
  "p.util.none": "Nadie registró esfuerzo ni ausencias en este periodo.",
  "p.util.over": "en o sobre la capacidad: un aviso, no un logro",
  "p.util.people_counted_one": "{n} persona contada.",
  "p.util.people_counted_other": "{n} personas contadas.",
  "p.util.problem_window":
    "Indique el periodo como número entero de días de 1 a {max}.",
  "p.util.read": "Leer el uso de la capacidad",
  "p.util.refused": "El servicio no aceptó esta consulta.",
  "p.util.says":
    "Esfuerzo registrado sobre la capacidad declarada, menos ausencias, en los {days} días hasta hoy salvo que elija de 1 a {max}. Estar en o sobre la capacidad es un aviso sobre la cola. Se ocultan grupos y cifras pequeños.",
  "p.util.suppression":
    "Se ocultan: un grupo de menos de {people} personas o una cifra inferior a {hours}.",
  "p.util.table": "Uso de la capacidad por persona",
  "p.util.team_detail":
    "{effort} de {available} disponibles, {away} de ausencia",
  "p.util.unavailable_all_away": "ausente todo el periodo, no 0 %",
  "p.util.unavailable_below_floor": "demasiado poco para informar",
  "p.util.unavailable_no_capacity": "sin capacidad declarada",
  "p.util.unavailable_other": "no se puede informar",
  "p.util.unavailable_small_group": "demasiado pocas personas para mostrar",
  "p.util.window_one": "En el último {n} día.",
  "p.util.window_other": "En los últimos {n} días.",
};
