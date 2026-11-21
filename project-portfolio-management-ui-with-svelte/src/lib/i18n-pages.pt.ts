// Portuguese (pt-001) for the page and component catalogue (WW-1.1). Typed as a complete record of
// the English keys, so a missing string is a compile error.
//
// STATUS: MACHINE DRAFT. Written by Claude (the AI assistant) on 2026-11-21 and NOT reviewed by a
// speaker or a professional translator. The review is recorded in
// `governance/translation-reviews/pt-001.md`; update the status there, not here.

import type { PageKey } from "./i18n-pages";

export const PAGES_PT: Record<PageKey, string> = {
  "c.CalendarMonth.next_month": "Mês seguinte",
  "p.calendar.day": "Dia",
  "p.calendar.agenda": "Agenda",
  "p.calendar.status": "Status",
  "p.calendar.status_done": "concluído",
  "p.calendar.status_open": "aberto",
  "p.calendar.item": "Item de trabalho",
  "p.calendar.showing": "Exibindo {view}, de {start} a {end}",
  "p.calendar.more": "+{n} mais",
  "p.calendar.milestone": "Marco",
  "p.calendar.due": "Prazo",
  "p.plans.pid.board.assignee": "Responsável",
  "p.plans.pid.board.moving": "Movendo {title} para {column}…",
  "p.plans.pid.board.move_failed":
    "Não foi possível mover {title} para {column}; voltou ao lugar de antes.",
  "p.plans.pid.board.empty_column": "Nenhuma tarefa",
  "p.plans.pid.board.search": "Pesquisar no quadro",
  "p.plans.pid.board.task": "Tarefa",
  "p.plans.pid.board.status": "Status",
  "c.GanttView.holiday": "Feriado",
  "c.GanttView.shaded":
    "As colunas sombreadas são dias de folga: fins de semana e feriados no calendário de trabalho deste plano.",
  "c.GanttView.kind": "Tipo",
  "c.GanttView.stage": "Etapa",
  "c.GanttView.task": "Tarefa",
  "c.GanttView.start": "Início",
  "c.GanttView.end": "Término",
  "c.GanttView.predecessors": "Depende de",
  "c.GanttView.depends_on": "Depende de: {names}",
  "c.GanttView.critical": "{name} (caminho crítico)",
  "c.GanttView.zoom_in": "Escala de tempo mais fina",
  "c.GanttView.zoom_out": "Escala de tempo mais ampla",
  "c.GanttView.zoomed": "Escala de tempo: {unit}",
  "c.GanttView.unit_day": "dia",
  "c.GanttView.unit_week": "semana",
  "c.GanttView.unit_month": "mês",
  "c.GanttView.export": "Exportar o cronograma",
  "c.GanttView.csv": "CSV",
  "c.GanttView.excel": "Excel",
  "c.GanttView.exported": "{n} tarefas exportadas para {file}",
  "c.CalendarMonth.week": "Semana",
  "c.CalendarMonth.four_weeks": "4 semanas",
  "c.CalendarMonth.month": "Mês",
  "c.CalendarMonth.today": "Hoje",
  "c.CalendarMonth.previous": "{view} anterior",
  "c.CalendarMonth.next": "{view} seguinte",
  "c.CalendarMonth.working_calendar": "Calendário de trabalho",
  "c.CalendarMonth.previous_month": "Mês anterior",
  "c.CapacityMeter.days_over_capacity": "Dias acima da capacidade",
  "c.CostRiskView.adds": "{risk}: acrescenta {money} — {eventHitText}",
  "c.CostRiskView.already_spent": "Já gasto",
  "c.CostRiskView.assumptions": "Premissas",
  "c.CostRiskView.budget": "orçamento",
  "c.CostRiskView.budget_2": "Orçamento",
  "c.CostRiskView.budget_sum_of_planned_amounts":
    "Orçamento (soma dos valores planejados)",
  "c.CostRiskView.by_plan": "Por plano",
  "c.CostRiskView.cost": "Custo",
  "c.CostRiskView.cost_at_each_percentile":
    "Custo em {currency} em cada percentil",
  "c.CostRiskView.cost_from": "Custo de",
  "c.CostRiskView.each_bar_is_a_cost":
    "Cada barra é uma faixa de custo; quanto mais alta, mais simulações terminaram ali. As linhas contínuas são P50, P80 e P90 (rotuladas); a linha tracejada é o orçamento.",
  "c.CostRiskView.each_plan_s_budget_and":
    "Orçamento de cada plano e custo simulado médio em {currency}",
  "c.CostRiskView.estimate": "Estimativa",
  "c.CostRiskView.independence": "Independência.",
  "c.CostRiskView.left_out": "Deixados de fora:",
  "c.CostRiskView.line": "Linha",
  "c.CostRiskView.low_planned_high": "Baixo / planejado / alto",
  "c.CostRiskView.mean_simulated_cost": "Custo simulado médio",
  "c.CostRiskView.no_register_risk_was_used":
    "Nenhum risco do registro foi usado como evento de custo.",
  "c.CostRiskView.outcome": "Resultado",
  "c.CostRiskView.per_line_estimate_and_sensitivity":
    "Estimativa e sensibilidade por linha em {currency}",
  "c.CostRiskView.plan": "Plano",
  "c.CostRiskView.risks_that_added_cost": "Riscos que acrescentaram custo",
  "c.CostRiskView.seed_the_same_seed_gives":
    "Semente {seed}: a mesma semente dá o mesmo resultado. Até {max_iterations} iterações.",
  "c.CostRiskView.sensitivity": "Sensibilidade",
  "c.CostRiskView.simulations": "Simulações",
  "c.CostRiskView.spent": "Gasto",
  "c.CostRiskView.the_same_figures_as_a": "Os mesmos números como tabela",
  "c.CostRiskView.to": "a",
  "c.CostRiskView.to_of": "{money} a {money2}: {count} de {iterations}",
  "c.CostRiskView.what_drives_it": "O que o determina",
  "c.CpmGantt.cannot_meet_a_constraint_or":
    "não consegue cumprir uma restrição ou prazo (a linha marca o limite)",
  "c.CpmGantt.critical_any_slip_moves_the":
    "crítico: qualquer atraso adia o término",
  "c.CpmGantt.has_float_the_hatched_tail": "tem folga (a cauda hachurada)",
  "c.CpmGantt.legend": "Legenda",
  "c.CpmGantt.near_critical": "quase crítico",
  "c.CpmGantt.no_scheduled_tasks_to_draw":
    "Ainda não há tarefas programadas para desenhar. Dê a uma tarefa uma duração (e uma data de início ou uma predecessora).",
  "c.CpmGantt.the_outline_under_a_bar":
    "o contorno sob uma barra mostra onde ela estava na linha de base",
  "c.CpmGantt.to_float_is_counted_in":
    "{from} a {to}. A folga é contada nos dias úteis do plano; as barras são desenhadas em dias corridos.",
  "c.CpmTable.calendar": "Calendário",
  "c.CpmTable.constraint": "Restrição",
  "c.CpmTable.deadline": "Prazo final",
  "c.CpmTable.finishes": "Termina",
  "c.CpmTable.free_float": "Folga livre",
  "c.CpmTable.overrides_its_predecessors": "(sobrepõe suas predecessoras)",
  "c.CpmTable.plan": "plano",
  "c.CpmTable.plan_assignee": "plano + responsável",
  "c.CpmTable.starts": "Inicia",
  "c.CpmTable.state": "Estado",
  "c.CpmTable.task": "Tarefa",
  "c.CpmTable.task_schedule_with_float_in":
    "Cronograma de tarefas com folga, em dias úteis do plano",
  "c.CpmTable.total_float": "Folga total",
  "c.CpmTable.untitled": "(sem título)",
  "c.CumulativeFlow.cumulative_flow": "Fluxo cumulativo",
  "c.CumulativeFlow.day": "Dia",
  "c.CumulativeFlow.every_value_in_the_chart":
    "Todos os valores do gráfico. {note}",
  "c.CumulativeFlow.no_samples_in_this_window": "Nenhuma amostra nesta janela.",
  "c.CumulativeFlow.the_vertical_gap_above_done":
    "o intervalo vertical acima de “Concluído” é o trabalho em andamento; o intervalo horizontal até ele é aproximadamente o tempo de ciclo",
  "c.CumulativeFlow.total": "Total",
  "c.CumulativeFlow.total_in_progress_done":
    "{total} no total · {work_in_progress} em andamento · {done} concluídos",
  "c.EvBars.the_double_line_is_the":
    "A linha dupla é o orçamento no término: {money}.",
  "c.EvBars.unknown": "desconhecido",
  "c.KpiCard.add_a_reading": "Adicionar uma leitura",
  "c.KpiCard.cadence": "Periodicidade",
  "c.KpiCard.daily": "Diária",
  "c.KpiCard.delete": "Excluir",
  "c.KpiCard.edit_or_delete": "Editar ou excluir",
  "c.KpiCard.history": "Histórico ({readings})",
  "c.KpiCard.loading": "Carregando…",
  "c.KpiCard.monthly": "Mensal",
  "c.KpiCard.name": "Nome",
  "c.KpiCard.no_reading_yet": "Nenhuma leitura ainda.",
  "c.KpiCard.no_readings": "Nenhuma leitura.",
  "c.KpiCard.none": "Nenhuma",
  "c.KpiCard.note": "Nota",
  "c.KpiCard.observed": "Observado",
  "c.KpiCard.observed_on": "Observado em",
  "c.KpiCard.on": "{valueText} em {observed_on}",
  "c.KpiCard.quarterly": "Trimestral",
  "c.KpiCard.read": "· lido {cadence}",
  "c.KpiCard.record": "Registrar",
  "c.KpiCard.red_line": "Linha vermelha",
  "c.KpiCard.save": "Salvar",
  "c.KpiCard.target": "Meta",
  "c.KpiCard.unit": "Unidade",
  "c.KpiCard.value": "Valor",
  "c.KpiCard.weekly": "Semanal",
  "c.KpiCard.which_way_is_good_cannot":
    "Qual sentido é bom ({v}) não pode ser alterado: isso recolore todas as leituras passadas.",
  "c.LilyGanttTable.critical_path": "caminho crítico",
  "c.LilyGanttTable.day": "Dia",
  "c.LilyGanttTable.month": "Mês",
  "c.LilyGanttTable.no_dated_tasks_to_show":
    "Nenhuma tarefa com data para exibir.",
  "c.LilyGanttTable.no_usable_dates": "Nenhuma data utilizável:",
  "c.LilyGanttTable.task": "Tarefa",
  "c.LilyGanttTable.that_range_is_too_long":
    "Esse intervalo é longo demais para uma coluna por {unit}; exibindo uma por {unit2}.",
  "c.LilyGanttTable.time_scale": "Escala de tempo",
  "c.LilyGanttTable.week": "Semana",
  "c.LilyKanbanTable.move_to": "Mover {title} para",
  "c.LilyKanbanTable.no_tasks_yet": "Nenhuma tarefa ainda.",
  "c.LilyKanbanTable.unknown_status": "Status desconhecido:",
  "c.LineRange.high": "Alto",
  "c.LineRange.low": "Baixo",
  "c.LineRange.planned_is_the_likely_cost":
    "O planejado é {money}, o custo provável. Deixe um campo vazio para limpá-lo.",
  "c.LineRange.save_range": "Salvar faixa",
  "c.MilestoneCard.by": "Até",
  "c.MilestoneCard.by_hand_only": "Somente manualmente",
  "c.MilestoneCard.complete": "Concluir",
  "c.MilestoneCard.completes": "Conclui",
  "c.MilestoneCard.done": "Concluído",
  "c.MilestoneCard.due": "Prazo {due} · {kind}",
  "c.MilestoneCard.every_move_is_kept_in":
    "Cada alteração fica no histórico: a data antiga, a nova, o motivo e quem a fez.",
  "c.MilestoneCard.forecast": "· previsão {early_finish}",
  "c.MilestoneCard.from": "De",
  "c.MilestoneCard.loading": "Carregando…",
  "c.MilestoneCard.mark_done": "Marcar como concluído",
  "c.MilestoneCard.move": "Mover",
  "c.MilestoneCard.never_rescheduled": "Nunca reprogramado.",
  "c.MilestoneCard.new_due_date": "Novo prazo",
  "c.MilestoneCard.no_reason_given": "sem motivo informado",
  "c.MilestoneCard.open": "Aberto",
  "c.MilestoneCard.pick_the_tasks_that_must":
    "Escolha as tarefas que devem estar concluídas",
  "c.MilestoneCard.reason_required": "Motivo (obrigatório)",
  "c.MilestoneCard.reopen": "Reabrir",
  "c.MilestoneCard.reschedule": "Reprogramar",
  "c.MilestoneCard.save": "Salvar",
  "c.MilestoneCard.slip_history": "Histórico de atrasos ({reschedules})",
  "c.MilestoneCard.tasks_it_waits_for": "Tarefas que ele aguarda",
  "c.MilestoneCard.tasks_this_milestone_waits_for":
    "Tarefas que este marco aguarda",
  "c.MilestoneCard.this_plan_has_no_tasks": "Este plano ainda não tem tarefas.",
  "c.MilestoneCard.to": "Para",
  "c.MilestoneCard.view_plan": "Ver plano",
  "c.MilestoneCard.when_all_those_tasks_are":
    "Quando todas essas tarefas estiverem concluídas",
  "c.MilestoneCard.why": "Motivo",
  "c.NetworkDiagram.float": "{slice} → {slice2} · folga {total_float}",
  "c.NetworkDiagram.no_dependencies": "Nenhuma dependência.",
  "c.NetworkDiagram.no_scheduled_tasks_to_draw":
    "Ainda não há tarefas programadas para desenhar.",
  "c.NetworkDiagram.task_network_boxes_are_tasks":
    "Rede de tarefas: as caixas são tarefas, as setas são dependências",
  "c.NetworkDiagram.the_same_network_as_a": "A mesma rede como lista",
  "c.ObjectiveCard.add_a_key_result": "Adicionar um resultado-chave",
  "c.ObjectiveCard.add_key_result": "Adicionar resultado-chave",
  "c.ObjectiveCard.check_in": "Atualizar",
  "c.ObjectiveCard.confidence_0_100": "Confiança (0–100)",
  "c.ObjectiveCard.currency": "Moeda",
  "c.ObjectiveCard.currency_e_g_gbp": "Moeda (por exemplo EUR, JPY ou GBP)",
  "c.ObjectiveCard.decrease": "Diminuir",
  "c.CommandPalette.cmd_dashboard": "Painel",
  "c.CommandPalette.cmd_escalations": "Escalonamentos",
  "c.CommandPalette.cmd_my_work": "Meu trabalho",
  "c.CommandPalette.cmd_new_plan": "Novo plano",
  "c.CommandPalette.count_one": "{n} resultado",
  "c.CommandPalette.count_other": "{n} resultados",
  "c.CommandPalette.failed":
    "A pesquisa de planos não está disponível agora. Os comandos continuam funcionando.",
  "c.CommandPalette.kind_command": "Comando",
  "c.CommandPalette.kind_plan": "Plano",
  "c.CommandPalette.kind_recent": "Recente",
  "c.CommandPalette.label": "Pesquisar planos e comandos",
  "c.CommandPalette.none": "Nada corresponde",
  "c.CommandPalette.open": "Pesquisa e comandos",
  "c.CommandPalette.placeholder": "Digite o nome de um plano ou um comando",
  "c.CommandPalette.title": "Pesquisa e comandos",
  "c.NavItems.portfolios": "Portfólios",
  "c.NavItems.projects": "Projetos",
  "c.NavItems.products": "Produtos",
  "c.NavItems.programs": "Programas",
  "c.NavItems.practices": "Práticas",
  "c.NavItems.processes": "Processos",
  "c.NavItems.purposes": "Propósitos",
  "c.NavItems.pathways": "Trilhas",
  "c.PasteTasks.check": "Verificar com o serviço",
  "c.PasteTasks.clear": "Limpar",
  "c.PasteTasks.col_due": "Prazo final",
  "c.PasteTasks.col_line": "Linha",
  "c.PasteTasks.col_notes": "Notas",
  "c.PasteTasks.col_status": "Status",
  "c.PasteTasks.col_title": "Título",
  "c.PasteTasks.create_one": "Criar 1 tarefa",
  "c.PasteTasks.create_other": "Criar {n} tarefas",
  "c.PasteTasks.done": "Concluído",
  "c.PasteTasks.failed": "A colagem não funcionou.",
  "c.PasteTasks.help":
    "Copie células de uma planilha e cole aqui: uma tarefa por linha, com as colunas Título, Prazo final (aaaa-mm-dd) e Notas, nessa ordem. A data vira o prazo final da tarefa. Uma primeira linha que nomeia as colunas é ignorada. Uma linha com problema é exibida com o motivo e fica de fora.",
  "c.PasteTasks.label": "Linhas coladas",
  "c.PasteTasks.left_out": "Deixada de fora: {why}",
  "c.PasteTasks.problem_beyond_limit": "mais de {max} linhas",
  "c.PasteTasks.problem_date_bad":
    "“{value}” não é uma data; escreva no formato aaaa-mm-dd",
  "c.PasteTasks.problem_no_title": "sem título",
  "c.PasteTasks.problem_notes_long": "as notas têm mais de {max} caracteres",
  "c.PasteTasks.problem_same_as": "igual à linha {line}",
  "c.PasteTasks.problem_title_long": "o título tem mais de {max} caracteres",
  "c.PasteTasks.ready": "Pronta",
  "c.PasteTasks.tally": "{good} prontas, {left} deixadas de fora",
  "c.PasteTasks.title": "Colar linhas de uma planilha",
  "c.PasteTasks.would": "O que aconteceria",
  "c.ConfirmHost.cancel": "Cancelar",
  "c.ConfirmHost.ok": "OK",
  "c.ConfirmHost.title": "Confirme, por favor",
  "c.ObjectiveCard.delete": "Excluir",
  "c.ObjectiveCard.direction": "Sentido",
  "c.ObjectiveCard.edit_or_delete": "Editar ou excluir",
  "c.ObjectiveCard.edit_or_delete_this_objective":
    "Editar ou excluir este objetivo",
  "c.ObjectiveCard.from": "{valueText} (de {start_value})",
  "c.ObjectiveCard.increase": "Aumentar",
  "c.ObjectiveCard.it_has_no_measurable_key":
    "— ainda não tem resultado-chave mensurável, portanto não tem pontuação (não é zero).",
  "c.ObjectiveCard.key_results_in_more_than":
    "Resultados-chave em mais de uma moeda são pontuados separadamente; as moedas nunca são convertidas.",
  "c.ObjectiveCard.last_check_in": "· última atualização {slice}",
  "c.ObjectiveCard.maintain": "Manter",
  "c.ObjectiveCard.metric": "Métrica",
  "c.ObjectiveCard.no_key_results_yet": "Nenhum resultado-chave ainda.",
  "c.ObjectiveCard.note": "Nota",
  "c.ObjectiveCard.number": "Número",
  "c.ObjectiveCard.percent": "Percentual",
  "c.ObjectiveCard.period": "Período",
  "c.ObjectiveCard.record": "Registrar",
  "c.ObjectiveCard.save": "Salvar",
  "c.ObjectiveCard.start": "Início",
  "c.ObjectiveCard.target": "Meta",
  "c.ObjectiveCard.the_baseline_metric_and_direction":
    "A linha de base, a métrica e o sentido não podem ser alterados: progresso a partir de uma linha de base móvel não é progresso.",
  "c.ObjectiveCard.title": "Título",
  "c.ObjectiveCard.tolerance": "· tolerância ±{tolerance}",
  "c.ObjectiveCard.tolerance_2": "Tolerância",
  "c.ObjectiveCard.unit": "Unidade",
  "c.ObjectiveCard.value": "Valor",
  "c.ObjectiveCard.yes_no": "Sim / não",
  "c.PlanForm.organization": "organização:…",
  "c.PlanForm.parent_plan_pid_uuid": "pid do plano pai (UUID)",
  "c.PlanForm.proj_2026": "PROJ-2026",
  "c.RaciMatrix.complete": "completo",
  "c.RaciMatrix.gaps": "Lacunas",
  "c.RaciMatrix.task": "Tarefa",
  "c.RaciMatrix.who_does_what_raci_matrix":
    "Quem faz o quê (matriz RACI): quem é Responsável, Aprovador, Consultado e Informado em cada tarefa",
  "c.RiskInputs.adds_cost": "Acrescenta custo",
  "c.RiskInputs.by_business_days": "em (dias úteis)",
  "c.RiskInputs.currency": "Moeda",
  "c.RiskInputs.delays_task": "Atrasa a tarefa",
  "c.RiskInputs.gbp": "EUR",
  "c.RiskInputs.none": "Nenhum",
  "c.RiskInputs.one_probability_covers_both_effects":
    "Uma probabilidade cobre os dois efeitos: um risco que atrasa uma tarefa e acrescenta custo ocorre uma só vez e faz os dois. Deixe uma parte vazia para omiti-la.",
  "c.RiskInputs.probability": "Probabilidade (%)",
  "c.RiskInputs.save": "Salvar",
  "c.RiskInputs.saving": "Salvando…",
  "c.RiskInputs.simulation_inputs": "Entradas da simulação",
  "c.SCurve.actual": "Real",
  "c.SCurve.date": "Data",
  "c.SCurve.earned": "Agregado",
  "c.SCurve.line_planned_value_to_earned":
    "Linha: valor planejado ({from} a {to}). ● valor agregado e ■ custo real, somente nos dias registrados ({snapshots} até agora). Tracejado: orçamento no término, {money}.",
  "c.SCurve.no_curve_to_draw": "Nenhuma curva para desenhar.",
  "c.SCurve.not_recorded": "não registrado",
  "c.SCurve.planned": "Planejado",
  "c.SCurve.s_curve_planned_value_over":
    "Curva S: valor planejado ao longo do tempo, com o valor agregado e o custo real registrados",
  "c.SCurve.the_same_figures_as_a": "Os mesmos números como tabela",
  "c.VarianceTable.baselined": "Com linha de base",
  "c.VarianceTable.duration": "Δ de duração",
  "c.VarianceTable.finish": "Δ de término",
  "c.VarianceTable.now": "Agora",
  "c.VarianceTable.schedule_variance_against_the_baseli":
    "Variação de cronograma em relação à linha de base, em dias úteis",
  "c.VarianceTable.standing": "Situação",
  "c.VarianceTable.start": "Δ de início",
  "c.VarianceTable.task": "Tarefa",
  "c.VarianceTable.the_plan_s_finish_is": "O término do plano não mudou.",
  "c.VarianceTable.the_plan_s_own_finish":
    "O término do próprio plano não pode ser comparado.",
  "p.announcements.announcements": "Avisos",
  "p.announcements.announcements_ppm": "Avisos — PPM",
  "p.announcements.expires_optional": "Expira em (opcional)",
  "p.announcements.important": "Importante",
  "p.announcements.info": "Informativo",
  "p.announcements.loading": "Carregando…",
  "p.announcements.message": "Mensagem",
  "p.announcements.nothing_has_been_announced": "Nenhum aviso foi publicado.",
  "p.announcements.pin_to_the_top": "Fixar no topo",
  "p.announcements.pinned": "· Fixado",
  "p.announcements.publish": "Publicar",
  "p.announcements.publish_an_announcement": "Publicar um aviso",
  "p.announcements.publishing_needs_the_announcements_p":
    "Publicar exige a permissão de avisos.",
  "p.announcements.retract": "Retirar",
  "p.announcements.severity": "Gravidade",
  "p.announcements.title": "Título",
  "p.announcements.urgent": "Urgente",
  "p.auditor.action": "Ação",
  "p.auditor.actor": "Autor",
  "p.auditor.actor_less_recorded_actions": "Ações registradas sem autor:",
  "p.auditor.audit_trail": "Trilha de auditoria",
  "p.auditor.bearer_sub": "sub do portador",
  "p.auditor.detail": "Detalhe",
  "p.auditor.e_g_merged": "ex.: mesclado",
  "p.auditor.entity": "Entidade",
  "p.auditor.evidence_pack_csv": "Pacote de evidências (CSV)",
  "p.auditor.filter": "Filtro",
  "p.auditor.findings": "Constatações",
  "p.auditor.no_audit_rows_match": "Nenhuma linha de auditoria corresponde.",
  "p.auditor.no_findings": "Nenhuma constatação.",
  "p.auditor.ppm": "{t} — PPM",
  "p.auditor.rows_distinct_actors_actor_less":
    "{returned} linhas · {distinct_actors} autores distintos · {actorless} sem autor",
  "p.auditor.rule": "Regra",
  "p.auditor.submitted_without_a_verified_token":
    "(enviado sem um token verificado)",
  "p.auditor.when": "Quando",
  "p.automations.action": "ação",
  "p.automations.action_2": "Ação",
  "p.automations.add_label": "adicionar rótulo",
  "p.automations.any": "qualquer",
  "p.automations.assign": "atribuir",
  "p.automations.assignee": "responsável",
  "p.automations.automation": "automação",
  "p.automations.automations_ppm": "Automações — PPM",
  "p.automations.blank_every_plan": "em branco = todos os planos",
  "p.automations.cancel": "Cancelar",
  "p.automations.claim_based_so_a_deadline":
    "Baseado em reivindicação, de modo que um prazo dispara exatamente uma vez, mesmo que o temporizador e uma varredura manual disputem.",
  "p.automations.configure_a_rule_once_as":
    "Configure uma regra uma vez; à medida que os itens se movem pelo quadro, a plataforma a aplica. As regras nunca bloqueiam a movimentação que as disparou — uma falha é registrada como execução com falha, não como desfazer.",
  "p.automations.delete": "Excluir",
  "p.automations.detail": "Detalhe",
  "p.automations.disable": "Desativar",
  "p.automations.due": "Prazo",
  "p.automations.enable": "Ativar",
  "p.automations.every_plan": "todos os planos",
  "p.automations.expire_review": "expirar revisão",
  "p.automations.from": "de",
  "p.automations.in_calendar_days": "em dias corridos",
  "p.automations.label": "rótulo",
  "p.automations.message": "mensagem",
  "p.automations.name": "nome",
  "p.automations.name_2": "Nome",
  "p.automations.no_rules_configured": "Nenhuma regra configurada.",
  "p.automations.nothing_has_fired_yet": "Nada foi disparado ainda.",
  "p.automations.nothing_pending": "Nada pendente.",
  "p.automations.notify": "notificar",
  "p.automations.off": "desligada",
  "p.automations.on": "ligada",
  "p.automations.operator": "operador",
  "p.automations.outcome": "Resultado",
  "p.automations.person_uuid": "person:<uuid>",
  "p.automations.plan_pid": "pid do plano",
  "p.automations.plan_stage_changed": "etapa do plano alterada",
  "p.automations.recipient": "destinatário",
  "p.automations.review_submitted": "revisão enviada",
  "p.automations.rules": "Regras",
  "p.automations.runs": "Execuções",
  "p.automations.save_rule": "Salvar regra",
  "p.automations.schedule_action": "programar ação",
  "p.automations.scope": "Escopo",
  "p.automations.set_and_forget": "Configure e esqueça",
  "p.automations.set_task_status": "definir status da tarefa",
  "p.automations.source": "Origem",
  "p.automations.state": "Estado",
  "p.automations.status": "status",
  "p.automations.subject": "Assunto",
  "p.automations.sweep_due_actions": "Varrer ações vencidas",
  "p.automations.task_moved": "tarefa movida",
  "p.automations.then": "então",
  "p.automations.to": "para",
  "p.automations.trigger": "gatilho",
  "p.automations.trigger_2": "Gatilho",
  "p.automations.when": "Quando",
  "p.automations.workflow_automation": "Automação de fluxo de trabalho",
  "p.board.amber": "âmbar",
  "p.board.amount": "Valor",
  "p.board.benefit_realizations": "realizações de benefícios",
  "p.board.board_pack": "Pacote do conselho",
  "p.board.decision": "Decisão",
  "p.board.decisions_in_window": "Decisões na janela",
  "p.board.green": "verde",
  "p.board.investment_decisions": "Decisões de investimento",
  "p.board.kind": "Tipo",
  "p.board.milestones_completed": "marcos concluídos",
  "p.board.no_decisions_in_the_window": "Nenhuma decisão na janela.",
  "p.board.no_investment_decisions_recorded":
    "Nenhuma decisão de investimento registrada.",
  "p.board.no_snapshots_captured_yet": "Nenhum instantâneo capturado ainda.",
  "p.board.open_exposure": "Exposição em aberto",
  "p.board.portfolios": "Portfólios",
  "p.board.ppm": "{t} — PPM",
  "p.board.realized_in_window": "Realizados na janela:",
  "p.board.red_portfolios": "portfólios vermelhos",
  "p.board.subject": "Assunto",
  "p.board.take_snapshot_now": "Capturar instantâneo agora",
  "p.board.taken": "Capturado",
  "p.board.tranches_released": "parcelas liberadas",
  "p.board.trends": "Tendências",
  "p.board.unattributed_events":
    "({unattributed_events} eventos sem atribuição)",
  "p.board.what": "O quê",
  "p.board.when": "Quando",
  "p.board.window": "janela {from} → {to}",
  "p.board.work_items": "Itens de trabalho",
  "p.calendar.all": "(todos)",
  "p.calendar.done": "· concluído",
  "p.calendar.kind": "Tipo",
  "p.calendar.no_milestones_match": "Nenhum marco corresponde.",
  "p.calendar.ppm": "{t} — PPM",
  "p.capacity.capacity_ppm": "Capacidade — PPM",
  "p.capacity.from": "de",
  "p.capacity.hatched_extension_proposed_not_count":
    "extensão hachurada: proposta, não contada",
  "p.capacity.legend": "Legenda",
  "p.capacity.over": "acima",
  "p.capacity.over_capacity_solid_bar_firm":
    "acima da capacidade (barra cheia: reservas firmes; linha: capacidade em vigor)",
  "p.capacity.proposed": "Proposto",
  "p.capacity.to": "a",
  "p.capacity.window": "janela: {from} → {to}",
  "p.capacity.within_capacity": "dentro da capacidade",
  "p.capacity.within_capacity_but_committing_the":
    "dentro da capacidade, mas confirmar as reservas propostas a excederia",
  "p.ceo.all_announcements": "Todos os avisos",
  "p.ceo.all_key_measures": "Todas as medidas-chave",
  "p.ceo.all_milestones": "Todos os marcos",
  "p.ceo.and_more": "e mais {more}",
  "p.ceo.announcements": "Avisos",
  "p.ceo.ceo_dashboard": "Painel do CEO",
  "p.ceo.ceo_dashboard_ppm": "Painel do CEO — PPM",
  "p.ceo.decision_log": "Registro de decisões",
  "p.ceo.escalated_risks": "riscos escalados",
  "p.ceo.every_kpi_is_on_target": "● Todos os KPIs estão na meta.",
  "p.ceo.executive_detail": "Detalhe executivo",
  "p.ceo.key_measures_needing_a_look": "Medidas-chave que pedem atenção",
  "p.ceo.loading": "Carregando…",
  "p.ceo.milestones_to_watch": "Marcos a acompanhar",
  "p.ceo.no_data_yet": "Ainda sem dados",
  "p.ceo.no_decisions_recorded_yet": "Nenhuma decisão registrada ainda.",
  "p.ceo.no_kpis_defined_yet": "Nenhum KPI definido ainda.",
  "p.ceo.no_open_milestones": "Nenhum marco aberto.",
  "p.ceo.no_portfolio_is_over_budget":
    "● Nenhum portfólio está acima do orçamento",
  "p.ceo.nothing_has_been_announced": "Nenhum aviso foi publicado.",
  "p.ceo.over_budget": "▲ Acima do orçamento: {join}",
  "p.ceo.overdue_milestones": "{overdueMilestones} marcos atrasados",
  "p.ceo.portfolio_health": "Saúde do portfólio",
  "p.ceo.recent_decisions": "Decisões recentes",
  "p.ceo.red_amber_green":
    "■ {red} vermelhos · ▲ {amber} âmbar · ● {green} verdes",
  "p.ceo.risk_and_money": "Risco e dinheiro",
  "p.ceo.risk_register": "Registro de riscos",
  "p.compliance.compliance_risk_register": "Registro de riscos de conformidade",
  "p.compliance.conformance_findings": "Constatações de conformidade",
  "p.compliance.detail": "Detalhe",
  "p.compliance.exposure": "Exposição",
  "p.compliance.gate_review_recency_threshold_calend":
    "limite de atualidade da revisão de portão: {review_days} dias corridos",
  "p.compliance.item": "Item",
  "p.compliance.no_compliance_risks_recorded":
    "Nenhum risco de conformidade registrado.",
  "p.compliance.no_conformance_findings":
    "Nenhuma constatação de conformidade.",
  "p.compliance.open_exposure": "{note} · exposição em aberto {open_exposure}",
  "p.compliance.owner": "Responsável",
  "p.compliance.ppm": "{t} — PPM",
  "p.compliance.risk": "Risco",
  "p.compliance.rule": "Regra",
  "p.compliance.status": "Status",
  "p.cost_rates.a_person": "Uma pessoa",
  "p.cost_rates.a_role": "Um papel",
  "p.cost_rates.add_a_rate_table": "Adicionar uma tabela de taxas",
  "p.cost_rates.add_rate": "Adicionar taxa",
  "p.cost_rates.add_table": "Adicionar tabela",
  "p.cost_rates.cost_rates": "Taxas de custo",
  "p.cost_rates.cost_rates_are_finance_data":
    "As taxas de custo são dados financeiros. Você não tem acesso a elas.",
  "p.cost_rates.cost_rates_ppm": "Taxas de custo — PPM",
  "p.cost_rates.currency": "Moeda",
  "p.cost_rates.effective_from": "Em vigor desde",
  "p.cost_rates.for": "Para",
  "p.cost_rates.from": "De",
  "p.cost_rates.name": "Nome",
  "p.cost_rates.no_rate_tables_yet": "Nenhuma tabela de taxas ainda.",
  "p.cost_rates.no_rate_yet_work_by":
    "Nenhuma taxa ainda: o trabalho deste {v} não tem custo apurado.",
  "p.cost_rates.per_hour": "Por hora",
  "p.cost_rates.person_person_reference": "Pessoa (referência person:…)",
  "p.cost_rates.role": "Papel",
  "p.cost_risk.band_for_lines_with_no":
    "Faixa para linhas sem estimativa (± %)",
  "p.cost_risk.distribution": "Distribuição",
  "p.cost_risk.iterations": "Iterações",
  "p.cost_risk.pert": "PERT",
  "p.cost_risk.portfolio_cost_risk": "Risco de custo do portfólio",
  "p.cost_risk.portfolio_cost_risk_ppm": "Risco de custo do portfólio — PPM",
  "p.cost_risk.resources": "Recursos",
  "p.cost_risk.run": "Executar",
  "p.cost_risk.running": "Executando…",
  "p.cost_risk.seed": "Semente",
  "p.cost_risk.shows_people_instead": "mostra pessoas em vez disso.",
  "p.cost_risk.triangular": "Triangular",
  "p.cost_risk.what_all_your_plans_together":
    "Quanto todos os seus planos juntos poderiam custar, dadas a faixa de cada linha de orçamento e os riscos que acrescentam custo. Cada moeda é tratada separadamente; elas nunca são somadas.",
  "p.dashboard.dashboard_ppm": "Painel — PPM",
  "p.display.display": "Exibição",
  "p.display.display_ppm": "Exibição — PPM",
  "p.display.dyslexia_friendly_font": "Fonte adequada para dislexia",
  "p.display.go_back_to_the_default": "Voltar ao padrão do meu papel",
  "p.display.how_the_app_looks_and":
    "Como o aplicativo se apresenta e é lido. Isso nunca altera o que você pode fazer.",
  "p.display.keeps_the_everyday_pages_in":
    "Mantém as páginas do dia a dia à frente (Meu trabalho, Planos, Quadro, Calendário, Visita guiada) e coloca o restante atrás de um link “Mais”. Todas as páginas continuam lá e funcionando.",
  "p.display.sign_in_to_keep_these":
    "Entre para manter estas opções na sua conta, além deste dispositivo.",
  "p.display.simple_view": "Visão simples",
  "p.display.this_is_the_default_for": "Este é o padrão para o seu papel: {v}.",
  "p.display.use_simple_view": "Usar a visão simples",
  "p.display.sections_title": "Seções na barra superior",
  "p.display.sections_help":
    "Desmarque uma seção para ocultá-la da barra superior. Ela continua acessível pelo endereço e você pode exibi-la novamente aqui. Nada é salvo até você ocultar uma, para que as seções adicionadas depois cheguem até você.",
  "p.display.sections_legend": "Exibir estas seções",
  "p.display.sections_hidden_count_one": "{n} seção oculta",
  "p.display.sections_hidden_count_other": "{n} seções ocultas",
  "p.display.sections_none_hidden": "Nenhuma seção está oculta.",
  "p.display.sections_show_all": "Exibir todas as seções",
  "p.display.week_start_title": "Primeiro dia da semana",
  "p.display.week_start_help":
    "Onde os calendários começam cada semana. Por padrão segue o idioma escolhido; os números de semana continuam sendo semanas ISO, que começam na segunda-feira.",
  "p.display.week_start_label": "As semanas começam em",
  "p.display.week_start_locale": "O dia habitual do idioma",
  "p.display.week_start_mon": "Segunda-feira",
  "p.display.week_start_sun": "Domingo",
  "p.display.week_start_sat": "Sábado",
  "p.display.use_the_dyslexia_friendly_font":
    "Usar a fonte adequada para dislexia",
  "p.display.uses_the_opendyslexic_typeface_with":
    "Usa a fonte OpenDyslexic, com maior espaçamento entre letras, palavras e linhas.",
  "p.display.you_chose_this": "Você escolheu isto.",
  "p.documents.controlled_documents": "Documentos controlados",
  "p.documents.create_document": "Criar documento",
  "p.documents.document": "Documento",
  "p.documents.documents": "Documentos",
  "p.documents.documents_ppm": "Documentos — PPM",
  "p.documents.first_draft": "Primeiro rascunho",
  "p.documents.in_force": "Em vigor",
  "p.documents.loading": "Carregando…",
  "p.documents.new_document": "Novo documento",
  "p.documents.no_documents_yet": "Nenhum documento ainda.",
  "p.documents.pid.approver_s_id": "ID do aprovador",
  "p.documents.pid.date": "Data",
  "p.documents.pid.document": "Documento",
  "p.documents.pid.documents": "← Documentos",
  "p.documents.pid.look_up": "Consultar",
  "p.documents.pid.nothing_was_in_force_on": "Nada estava em vigor em {on}.",
  "p.documents.pid.person_id": "ID da pessoa",
  "p.documents.pid.save_draft": "Salvar rascunho",
  "p.documents.pid.send_back": "Devolver",
  "p.documents.pid.text": "Texto",
  "p.documents.pid.this_version_s_text_is":
    "O texto desta versão é final. Uma alteração vai no próximo rascunho.",
  "p.documents.pid.version": "Versão {version}",
  "p.documents.pid.version_was_in_force_on":
    "A versão {version} estava em vigor em {on}.",
  "p.documents.pid.versions": "Versões",
  "p.documents.pid.what_changed": "O que mudou",
  "p.documents.pid.what_was_in_force_on": "O que estava em vigor em uma data",
  "p.documents.policies_and_procedures_a_new":
    "Políticas e procedimentos. Uma nova versão é revisada e aprovada antes de entrar em vigor, e uma versão em vigor nunca muda.",
  "p.documents.title": "Título",
  "p.documents.versions": "Versões",
  "p.engineering.assignee": "Responsável",
  "p.engineering.blocked_calendar_days": "Bloqueado (dias corridos)",
  "p.engineering.blocked_work": "Trabalho bloqueado",
  "p.engineering.declared_cause_failure_rate":
    "taxa de falha por causa declarada",
  "p.engineering.delivery_links": "Vínculos de entrega",
  "p.engineering.deploys_mo": "implantações ({window_months} m)",
  "p.engineering.devops_metrics": "Métricas de DevOps",
  "p.engineering.environment": "Ambiente",
  "p.engineering.incidents": "incidentes",
  "p.engineering.incidents_unresolved_counted_never_t":
    "{unresolved_incidents} incidentes não resolvidos (contados, nunca cronometrados).",
  "p.engineering.item": "Item",
  "p.engineering.items_carry_no_moscow_tag":
    "{untagged} itens não têm etiqueta MoSCoW.",
  "p.engineering.median_recovery_h": "recuperação mediana (h)",
  "p.engineering.moscow_scope": "Escopo MoSCoW",
  "p.engineering.no_deploy_events_ingested_yet":
    "Nenhum evento de implantação ingerido ainda.",
  "p.engineering.no_externally_tracked_items":
    "Nenhum item acompanhado externamente.",
  "p.engineering.nothing_blocked_ship_it": "Nada bloqueado. Pode entregar.",
  "p.engineering.ppm": "{t} — PPM",
  "p.engineering.releases": "Lançamentos",
  "p.engineering.task": "Tarefa",
  "p.engineering.trackers": "Rastreadores",
  "p.engineering.untracked": "Sem acompanhamento:",
  "p.engineering.version": "Versão",
  "p.engineering.when": "Quando",
  "p.executive.actor": "Autor",
  "p.executive.aligned": "Alinhado",
  "p.executive.as_of": "em {as_of} · {derivation}",
  "p.executive.benefits": "Benefícios",
  "p.executive.benefits_realization": "Realização de benefícios",
  "p.executive.calendar_days_since_update": "Dias corridos desde a atualização",
  "p.executive.collection": "Coleção",
  "p.executive.currency": "Moeda",
  "p.executive.decision": "Decisão",
  "p.executive.decision_log": "Registro de decisões",
  "p.executive.escalated_risks": "Riscos escalados",
  "p.executive.kind": "Tipo",
  "p.executive.largest_unaligned_items": "Maiores itens não alinhados",
  "p.executive.members": "Membros",
  "p.executive.no_decisions_recorded_yet": "Nenhuma decisão registrada ainda.",
  "p.executive.no_target": "sem meta",
  "p.executive.non_financial_only": "somente não financeiros ({non_financial})",
  "p.executive.open_exposure": "Exposição em aberto",
  "p.executive.overdue_milestones": "Marcos atrasados",
  "p.executive.overrun_currencies": "Moedas com estouro",
  "p.executive.planned_with_no_objective": "planejado sem objetivo",
  "p.executive.portfolio": "Portfólio",
  "p.executive.portfolio_health": "Saúde do portfólio",
  "p.executive.ppm": "{t} — PPM",
  "p.executive.realization": "Realização",
  "p.executive.realized": "Realizado",
  "p.executive.status": "Status",
  "p.executive.strategic_alignment": "Alinhamento estratégico",
  "p.executive.subject": "Assunto",
  "p.executive.target": "Meta",
  "p.executive.total": "Total",
  "p.executive.unaligned": "Não alinhado",
  "p.executive.unaligned_spend": "Gasto não alinhado",
  "p.executive.when": "Quando",
  "p.financials.actual": "Real",
  "p.financials.as_of": "em {as_of} · {note}",
  "p.financials.currency": "Moeda",
  "p.financials.currency_exposure": "Exposição por moeda",
  "p.financials.lines": "Linhas",
  "p.financials.no_budget_lines_recorded_yet":
    "Nenhuma linha de orçamento registrada ainda.",
  "p.financials.planned": "Planejado",
  "p.financials.ppm": "{t} — PPM",
  "p.financials.remaining": "Restante",
  "p.financials.work_items": "Itens de trabalho",
  "p.gantt.main_x": "{t} — Main X",
  "p.ideas.convert_target": "Destino da conversão",
  "p.ideas.ideas_ppm": "Ideias — PPM",
  "p.kpis.all": "Todos",
  "p.kpis.blank_for_the_whole_portfolio": "em branco para todo o portfólio",
  "p.kpis.daily": "Diária",
  "p.kpis.define": "Definir",
  "p.kpis.define_a_key_measure_kpi": "Definir uma medida-chave (KPI)",
  "p.kpis.higher_is_better": "Quanto maior, melhor",
  "p.kpis.key_measures_kpis": "Medidas-chave (KPIs)",
  "p.kpis.kpis_by_status": "KPIs por status",
  "p.kpis.kpis_ppm": "KPIs — PPM",
  "p.kpis.loading": "Carregando…",
  "p.kpis.lower_is_better": "Quanto menor, melhor",
  "p.kpis.monthly": "Mensal",
  "p.kpis.name": "Nome",
  "p.kpis.no_data_yet": "Ainda sem dados",
  "p.kpis.not_set": "Não definido",
  "p.kpis.off_target": "Fora da meta",
  "p.kpis.on_target": "Na meta",
  "p.kpis.percent_defects_gbp": "percentual, defeitos, EUR…",
  "p.kpis.plan_optional_pid": "Plano (pid opcional)",
  "p.kpis.quarterly": "Trimestral",
  "p.kpis.read_how_often": "Frequência de leitura",
  "p.kpis.red_line": "Linha vermelha",
  "p.kpis.show": "Exibir",
  "p.kpis.something_you_track_over_time":
    "Algo que você acompanha ao longo do tempo em relação a uma meta, para ver cedo se está no rumo certo.",
  "p.kpis.standing_measures_green_is_at":
    "Medidas permanentes. Verde é igual ou melhor que a meta; vermelho é igual ou pior que a linha vermelha; âmbar fica no meio. Um KPI que ninguém leu ainda mostra “Ainda sem dados”.",
  "p.kpis.target": "Meta",
  "p.kpis.the_red_line_must_be":
    "A linha vermelha deve ficar do lado pior da meta. Converta os valores em números inteiros (pontos-base, centavos).",
  "p.kpis.unit": "Unidade",
  "p.kpis.watch": "Acompanhar",
  "p.kpis.weekly": "Semanal",
  "p.kpis.which_way_is_good": "Qual sentido é bom",
  "p.layout.display": "Exibição",
  "p.layout.more": "Mais",
  "p.layout.simple_view": "Visão simples: {v}",
  "p.lifecycle.as_of": "em {as_of}",
  "p.lifecycle.in_an_unrecognized_phase": "em uma fase não reconhecida",
  "p.lifecycle.lifecycle": "Ciclo de vida",
  "p.lifecycle.lifecycle_ppm": "Ciclo de vida — PPM",
  "p.lifecycle.live": "Ativo",
  "p.lifecycle.live_items_one":
    "{total} item ativo · limite de estagnação {stall}",
  "p.lifecycle.live_items_other":
    "{total} itens ativos · limite de estagnação {stall}",
  "p.lifecycle.phase": "Fase",
  "p.lifecycle.share": "Participação",
  "p.lifecycle.stalled": "Estagnado",
  "p.lifecycle.where_everything_sits_across_the":
    "Onde cada coisa está ao longo do ciclo de vida do desafio, para que a próxima fase nunca seja uma surpresa. Estagnado conta os itens que ficaram numa fase por mais tempo que o limite.",
  "p.me.calendar.add": "Adicionar",
  "p.me.calendar.at_least_one_weekday_must":
    "Pelo menos um dia da semana deve ser de trabalho.",
  "p.me.calendar.from": "De",
  "p.me.calendar.loading": "Carregando…",
  "p.me.calendar.most_i_can_commit": "O máximo que posso assumir (%)",
  "p.me.calendar.my_calendar": "Meu calendário",
  "p.me.calendar.my_calendar_ppm": "Meu calendário — PPM",
  "p.me.calendar.my_days_off": "Meus dias de folga",
  "p.me.calendar.my_work": "← Meu trabalho",
  "p.me.calendar.none_set": "Nenhum definido.",
  "p.me.calendar.note": "Nota",
  "p.me.calendar.part_time_spells_and_leave":
    "Períodos de meio período e licenças",
  "p.me.calendar.remove": "Remover",
  "p.me.calendar.save": "Salvar",
  "p.me.calendar.saved": "Salvo.",
  "p.me.calendar.the_days_i_work": "Os dias em que trabalho",
  "p.me.calendar.the_most_of_your_time":
    "O máximo do seu tempo que você pode assumir entre duas datas: 60 para um período de meio período, 0 para licença. Fora dessas datas é 100. A capacidade é lida dia a dia, de modo que um período curto nunca se dilui na média.",
  "p.me.calendar.to": "{from} a {to}: {max_percent}%{v}",
  "p.me.calendar.to_2": "Até",
  "p.me.due": "· prazo {due_date}",
  "p.me.in_plans_you_cannot_read":
    "· {omitted_unreadable} em planos que você não pode ler",
  "p.me.loading": "Carregando…",
  "p.me.my_calendar": "Meu calendário",
  "p.me.my_work": "Meu trabalho",
  "p.me.my_work_ppm": "Meu trabalho — PPM",
  "p.me.nothing_assigned_to_you_enjoy": "Nada atribuído a você. Aproveite.",
  "p.me.open": "{today} ({tz}) · {total} abertos",
  "p.me.showing_the_first_500": "· exibindo os primeiros 500",
  "p.me.the_days_you_work_so":
    "— os dias em que você trabalha, para que suas datas sejam fiéis.",
  "p.me.your_sign_in_identity_cannot":
    "Sua identidade de login não pode ser associada às atribuições de tarefas, então nada pode ser listado aqui ainda.",
  "p.milestones.a_milestone_is_a_dated":
    "Um marco é uma promessa com data. Vincule-o às tarefas que ele aguarda e ele é alcançado quando elas estiverem concluídas, com uma previsão do cronograma; deixe-o sem vínculo e uma pessoa o marca como concluído.",
  "p.milestones.achieved": "Alcançado",
  "p.milestones.all": "Todos",
  "p.milestones.due_soon": "Vence em breve",
  "p.milestones.forecast": "Previsão:",
  "p.milestones.late": "Atrasado",
  "p.milestones.loading": "Carregando…",
  "p.milestones.milestones": "Marcos",
  "p.milestones.milestones_by_status": "Marcos por status",
  "p.milestones.milestones_ppm": "Marcos — PPM",
  "p.milestones.show": "Exibir",
  "p.milestones.upcoming": "Próximos",
  "p.objectives.objectives_ppm": "Objetivos — PPM",
  "p.objectives.period_e_g_2026_h2": "Período (ex.: 2026-H2)",
  "p.okrs.2026_q4_blank_for_all": "2026-Q4 (em branco para todos)",
  "p.okrs.a_goal_with_a_few":
    "Uma meta, com alguns resultados mensuráveis que mostram se você a alcançou.",
  "p.okrs.create": "Criar",
  "p.okrs.goals_and_key_results_okrs": "Metas e resultados-chave (OKRs)",
  "p.okrs.loading": "Carregando…",
  "p.okrs.new_objective": "Novo objetivo",
  "p.okrs.objectives_and_key_results_scores":
    "Objetivos e resultados-chave. As pontuações são calculadas a partir das atualizações a cada vez que você olha; nada é armazenado que possa ficar desatualizado.",
  "p.okrs.okrs_ppm": "OKRs — PPM",
  "p.okrs.period": "Período",
  "p.okrs.show": "Exibir",
  "p.okrs.title": "Título",
  "p.onboarding.a_role_by_role_first":
    'Um passo a passo da primeira hora, papel por papel. Cada etapa indica a página em que termina. É uma ordem sugerida, não uma promessa medida de "você será produtivo em N minutos" — ninguém cronometrou.',
  "p.onboarding.onboarding_ppm": "Integração — PPM",
  "p.onboarding.operator_onboarding": "Integração do operador",
  "p.plans.main_x": "{t} — Main X",
  "p.plans.merge.ppm": "{t} — PPM",
  "p.plans.new.main_x": "{t} — Main X",
  "p.plans.pid.board.action": "ação",
  "p.plans.pid.board.add_note": "Adicionar nota",
  "p.plans.pid.board.add_sprint": "Adicionar sprint",
  "p.plans.pid.board.add_task": "Adicionar tarefa",
  "p.plans.pid.board.board": "Quadro",
  "p.plans.pid.board.burndown": "Burndown — {name}",
  "p.plans.pid.board.convert_to_task": "Converter em tarefa",
  "p.plans.pid.board.created_moves_blocked_now":
    "{length} criadas · {length2} movimentações · {length3} bloqueadas agora",
  "p.plans.pid.board.date": "Data",
  "p.plans.pid.board.feedback": "feedback",
  "p.plans.pid.board.improve": "melhorar",
  "p.plans.pid.board.new_task_title": "Título da nova tarefa",
  "p.plans.pid.board.no_notes_for_this_sprint":
    "Nenhuma nota para esta sprint ainda.",
  "p.plans.pid.board.none": "(nenhum)",
  "p.plans.pid.board.note": "Nota",
  "p.plans.pid.board.note_body": "Texto da nota",
  "p.plans.pid.board.note_category": "Categoria da nota",
  "p.plans.pid.board.points_done": "Pontos concluídos",
  "p.plans.pid.board.ppm": "{t} — PPM",
  "p.plans.pid.board.pts": "pts",
  "p.plans.pid.board.remaining_of": "Restante de {total_tasks}",
  "p.plans.pid.board.retro_feedback_notes": "Notas de retrospectiva / feedback",
  "p.plans.pid.board.sprint": "Sprint",
  "p.plans.pid.board.sprint_end": "Término da sprint",
  "p.plans.pid.board.sprint_name": "Nome da sprint",
  "p.plans.pid.board.sprint_start": "Início da sprint",
  "p.plans.pid.board.sprints": "Sprints",
  "p.plans.pid.board.standup_last_24h": "Reunião diária (últimas 24 h)",
  "p.plans.pid.board.story_points": "Pontos de história",
  "p.plans.pid.board.task_created": "→ tarefa criada",
  "p.plans.pid.board.tasks_done": "Tarefas concluídas",
  "p.plans.pid.board.unpointed": "Sem pontuação",
  "p.plans.pid.board.velocity": "Velocidade",
  "p.plans.pid.board.went_well": "foi bem",
  "p.plans.pid.calendar.at_least_one_weekday_must":
    "Pelo menos um dia da semana deve ser de trabalho.",
  "p.plans.pid.calendar.critical_path": "← Caminho crítico",
  "p.plans.pid.calendar.holidays": "Feriados",
  "p.plans.pid.calendar.loading": "Carregando…",
  "p.plans.pid.calendar.my_calendar": "Meu calendário",
  "p.plans.pid.calendar.save_calendar": "Salvar calendário",
  "p.plans.pid.calendar.saved": "Salvo.",
  "p.plans.pid.calendar.their_work_then_happens_only":
    "); o trabalho deles então acontece apenas nos dias em que os dois calendários funcionam.",
  "p.plans.pid.calendar.working_calendar": "Calendário de trabalho",
  "p.plans.pid.calendar.working_calendar_ppm": "Calendário de trabalho — PPM",
  "p.plans.pid.calendar.working_weekdays": "Dias úteis da semana",
  "p.plans.pid.cost_risk.band_for_lines_with_no":
    "Faixa para linhas sem estimativa (± %)",
  "p.plans.pid.cost_risk.cost_and_schedule_together":
    "Custo e cronograma juntos",
  "p.plans.pid.cost_risk.cost_risk": "Risco de custo",
  "p.plans.pid.cost_risk.cost_risk_ppm": "Risco de custo — PPM",
  "p.plans.pid.cost_risk.distribution": "Distribuição",
  "p.plans.pid.cost_risk.iterations": "Iterações",
  "p.plans.pid.cost_risk.many_what_if_runs_of":
    "Muitas execuções hipotéticas do orçamento mostram a faixa de custos finais e a probabilidade de cada um.",
  "p.plans.pid.cost_risk.pert": "PERT",
  "p.plans.pid.cost_risk.plan": "← Plano",
  "p.plans.pid.cost_risk.run": "Executar",
  "p.plans.pid.cost_risk.running": "Executando…",
  "p.plans.pid.cost_risk.seed": "Semente",
  "p.plans.pid.cost_risk.triangular": "Triangular",
  "p.plans.pid.cost_risk.whole_portfolio": "Portfólio inteiro",
  "p.plans.pid.costs.a_recorded_cost_keeps_the":
    "Um custo registrado mantém a taxa pela qual foi precificado; uma mudança posterior de taxa nunca o reescreve.",
  "p.plans.pid.costs.add_an_expense": "Adicionar uma despesa",
  "p.plans.pid.costs.add_expense": "Adicionar despesa",
  "p.plans.pid.costs.all_at_the_end": "Tudo no fim",
  "p.plans.pid.costs.all_at_the_start": "Tudo no início",
  "p.plans.pid.costs.amount": "Valor",
  "p.plans.pid.costs.an_expense_of_so_it":
    "▲ Uma despesa de {money} {reason}, portanto não está nos períodos.",
  "p.plans.pid.costs.choose": "Escolher…",
  "p.plans.pid.costs.cost_rates": "Taxas de custo",
  "p.plans.pid.costs.costs": "Custos",
  "p.plans.pid.costs.costs_and_rates_are_finance":
    "Custos e taxas são dados financeiros. Você não tem acesso a eles.",
  "p.plans.pid.costs.costs_ppm": "Custos — PPM",
  "p.plans.pid.costs.currency": "Moeda",
  "p.plans.pid.costs.description": "Descrição",
  "p.plans.pid.costs.evenly_across_the_task":
    "Uniformemente ao longo da tarefa",
  "p.plans.pid.costs.falls": "Ocorre em",
  "p.plans.pid.costs.fixed_cost": "Custo fixo",
  "p.plans.pid.costs.kind": "Tipo",
  "p.plans.pid.costs.material": "Material",
  "p.plans.pid.costs.no_time_has_been_costed":
    "Nenhum tempo teve o custo apurado ainda.",
  "p.plans.pid.costs.nothing_could_be_costed_yet":
    "Nada pôde ter o custo apurado ainda: adicione taxas para as pessoas que fazem o trabalho ou adicione uma despesa.",
  "p.plans.pid.costs.per_period": "{currency}, por período",
  "p.plans.pid.costs.plan": "← Plano",
  "p.plans.pid.costs.planned_cost": "Custo planejado",
  "p.plans.pid.costs.record_labour_costs": "Registrar custos de mão de obra",
  "p.plans.pid.costs.recorded_labour_cost": "Custo de mão de obra registrado",
  "p.plans.pid.costs.so_its_labour_is_not":
    "▲ {title} ({hoursText}): {reason}, portanto a mão de obra não tem o custo apurado.",
  "p.plans.pid.costs.task": "Tarefa",
  "p.plans.pid.costs.time_entries_are_not_costed":
    "▲ {entries} lançamentos de tempo ({hoursText}) não têm o custo apurado: ainda não registrados ou sem taxa. O custo deles é desconhecido, não zero.",
  "p.plans.pid.costs.total": "Total",
  "p.plans.pid.costs.work": "Trabalho",
  "p.plans.pid.critical_path.a_baseline_is_a_numbered":
    "Uma linha de base é um instantâneo numerado do cronograma, mantido para sempre: refazer a linha de base gera a versão seguinte e nunca substitui uma.",
  "p.plans.pid.critical_path.baselines": "Linhas de base",
  "p.plans.pid.critical_path.business_days": "dias úteis",
  "p.plans.pid.critical_path.compare_with": "Comparar com",
  "p.plans.pid.critical_path.constraint": "Restrição",
  "p.plans.pid.critical_path.constraint_date": "Data da restrição",
  "p.plans.pid.critical_path.critical_chains":
    "{count} cadeias críticas{more}.",
  "p.plans.pid.critical_path.critical_path": "Caminho crítico",
  "p.plans.pid.critical_path.critical_path_ppm": "Caminho crítico — PPM",
  "p.plans.pid.critical_path.deadline_a_goal_it_shows":
    "Prazo final (uma meta: aparece como folga negativa e nunca move a tarefa)",
  "p.plans.pid.critical_path.duration_business_days_0_is":
    "Duração (dias úteis; 0 é um marco)",
  "p.plans.pid.critical_path.finish_no_earlier_than": "Terminar não antes de",
  "p.plans.pid.critical_path.finish_no_later_than": "Terminar não depois de",
  "p.plans.pid.critical_path.gantt": "Gantt",
  "p.plans.pid.critical_path.loading": "Carregando…",
  "p.plans.pid.critical_path.must_finish_on_overrides_logic":
    "Deve terminar em (sobrepõe a lógica)",
  "p.plans.pid.critical_path.must_start_on_overrides_logic":
    "Deve iniciar em (sobrepõe a lógica)",
  "p.plans.pid.critical_path.near_critical_within": "Quase crítico dentro de",
  "p.plans.pid.critical_path.network": "Rede",
  "p.plans.pid.critical_path.no_baseline_yet_so_nothing":
    "Ainda não há linha de base, portanto nada pode ser dito sobre atrasos.",
  "p.plans.pid.critical_path.none_as_soon_as_possible":
    "Nenhuma (o mais cedo possível)",
  "p.plans.pid.critical_path.not_scheduled": "Não programado",
  "p.plans.pid.critical_path.optimistic_business_days": "Otimista (dias úteis)",
  "p.plans.pid.critical_path.pessimistic_business_days":
    "Pessimista (dias úteis)",
  "p.plans.pid.critical_path.plan": "← Plano",
  "p.plans.pid.critical_path.planned_start": "Início planejado",
  "p.plans.pid.critical_path.range_for_schedule_risk":
    "Faixa para o risco de cronograma",
  "p.plans.pid.critical_path.range_saved": "Faixa salva.",
  "p.plans.pid.critical_path.reason_required_this_overrides_the":
    "Motivo (obrigatório: isto sobrepõe a lógica do cronograma)",
  "p.plans.pid.critical_path.save_range": "Salvar faixa",
  "p.plans.pid.critical_path.save_schedule": "Salvar cronograma",
  "p.plans.pid.critical_path.saved_moved": "Salvo. Movidas:",
  "p.plans.pid.critical_path.saved_nothing_else_moved":
    "Salvo. Nada mais foi movido.",
  "p.plans.pid.critical_path.saving": "Salvando…",
  "p.plans.pid.critical_path.show_the_baseline_under_each":
    "Exibir a linha de base sob cada barra",
  "p.plans.pid.critical_path.start_no_earlier_than": "Iniciar não antes de",
  "p.plans.pid.critical_path.start_no_later_than": "Iniciar não depois de",
  "p.plans.pid.critical_path.table": "Tabela",
  "p.plans.pid.critical_path.take_baseline": "Criar linha de base",
  "p.plans.pid.critical_path.task": "Tarefa",
  "p.plans.pid.critical_path.the_duration_above_is_the":
    "{taskRangeText}. A duração acima é o valor provável.",
  "p.plans.pid.critical_path.the_longest_chain_of_tasks":
    "A cadeia mais longa de tarefas, que define a data de término. Um atraso em qualquer tarefa dela atrasa o plano inteiro.",
  "p.plans.pid.critical_path.these_assignees_share_no_business":
    "Estes responsáveis não compartilham nenhum dia útil com o calendário do plano, então o calendário do plano foi usado para eles: {join}.",
  "p.plans.pid.critical_path.this_network_has_tasks_too":
    "Esta rede tem {length} tarefas, muitas para desenhar de forma útil. A tabela abaixo diz o mesmo.",
  "p.plans.pid.critical_path.untitled": "(sem título)",
  "p.plans.pid.critical_path.version": "Versão {version}: {reason} ({slice})",
  "p.plans.pid.critical_path.view": "Visualização",
  "p.plans.pid.critical_path.why_a_baseline_now":
    "Por que uma linha de base agora?",
  "p.plans.pid.critical_path.working_calendar": "Calendário de trabalho",
  "p.plans.pid.earned_value.a_percent_i_enter_flagged":
    "Um percentual que eu informo (sinalizado)",
  "p.plans.pid.earned_value.actual_cost": "Custo real",
  "p.plans.pid.earned_value.against_the_budget_vac":
    "Em relação ao orçamento (VAC)",
  "p.plans.pid.earned_value.baseline_version":
    "versão da linha de base {baseline_version}",
  "p.plans.pid.earned_value.budget_at_completion": "Orçamento no término",
  "p.plans.pid.earned_value.budget_lines_in_have_no":
    "As linhas de orçamento em {join} não têm linha de base, portanto não fazem parte de nenhum número acima e nunca são somadas a outra moeda.",
  "p.plans.pid.earned_value.by_story_points_when_finished":
    "por pontos de história, quando concluída",
  "p.plans.pid.earned_value.cancel": "Cancelar",
  "p.plans.pid.earned_value.change": "Alterar",
  "p.plans.pid.earned_value.complete": "{round}% concluído",
  "p.plans.pid.earned_value.cost_index": "Índice de custo (",
  "p.plans.pid.earned_value.cost_is": "O custo está {word}.",
  "p.plans.pid.earned_value.cost_variance_ev_ac": "Variação de custo (EV − AC)",
  "p.plans.pid.earned_value.count_a_task_another_way":
    "Contar uma tarefa de outra forma",
  "p.plans.pid.earned_value.count_tasks": "Contar tarefas",
  "p.plans.pid.earned_value.critical_path": "Caminho crítico",
  "p.plans.pid.earned_value.earned_value": "Valor agregado",
  "p.plans.pid.earned_value.earned_value_figures":
    "Números de valor agregado em {currency}",
  "p.plans.pid.earned_value.earned_value_ppm": "Valor agregado — PPM",
  "p.plans.pid.earned_value.final_cost_eac": "Custo final (EAC)",
  "p.plans.pid.earned_value.finished_only": "somente concluídas",
  "p.plans.pid.earned_value.forecast": "Previsão",
  "p.plans.pid.earned_value.forecast_final_cost":
    "· previsão do custo final {money}",
  "p.plans.pid.earned_value.forecasts_of_the_final_cost":
    "Previsões do custo final em {currency}, cada uma pela sua própria fórmula",
  "p.plans.pid.earned_value.governance": "← Governança",
  "p.plans.pid.earned_value.half_for_started": "metade para iniciadas",
  "p.plans.pid.earned_value.is_the_work_finished_worth":
    "O trabalho concluído vale o que estava planejado até agora, e quanto custou? Um índice acima de 1,00 é melhor que o planejado.",
  "p.plans.pid.earned_value.leading_with_that_one_because":
    "Começando por esta porque {why}. As outras aparecem ao lado: qual está certa depende de os estouros passados continuarem ou não.",
  "p.plans.pid.earned_value.loading": "Carregando…",
  "p.plans.pid.earned_value.manual": "manual",
  "p.plans.pid.earned_value.measured_as_of_percent_complete":
    "Medido em {as_of}. O percentual concluído é o trabalho terminado, não o tempo decorrido.",
  "p.plans.pid.earned_value.method": "Método",
  "p.plans.pid.earned_value.only_when_finished": "somente quando concluída",
  "p.plans.pid.earned_value.only_when_finished_2": "Somente quando concluída",
  "p.plans.pid.earned_value.over_time": "Ao longo do tempo",
  "p.plans.pid.earned_value.percent_complete": "Percentual concluído",
  "p.plans.pid.earned_value.planned_value": "Valor planejado",
  "p.plans.pid.earned_value.plans": "{plans} planos",
  "p.plans.pid.earned_value.progress_against_budget_earned_value":
    "Progresso em relação ao orçamento (valor agregado)",
  "p.plans.pid.earned_value.record_today_s_figures_now":
    "Registrar agora os números de hoje",
  "p.plans.pid.earned_value.recorded": "Registrado.",
  "p.plans.pid.earned_value.save": "Salvar",
  "p.plans.pid.earned_value.schedule_index": "Índice de prazo (",
  "p.plans.pid.earned_value.schedule_index_by_time":
    "Índice de prazo por tempo (",
  "p.plans.pid.earned_value.schedule_is": "O prazo está {word}.",
  "p.plans.pid.earned_value.schedule_variance_ev_pv":
    "Variação de prazo (EV − PV)",
  "p.plans.pid.earned_value.spi": "SPI {ratioOrReason} ·",
  "p.plans.pid.earned_value.started_work_counts_half":
    "trabalho iniciado conta metade",
  "p.plans.pid.earned_value.started_work_counts_half_2":
    "Trabalho iniciado conta metade",
  "p.plans.pid.earned_value.still_to_spend_etc": "Ainda a gastar (ETC)",
  "p.plans.pid.earned_value.task": "(tarefa)",
  "p.plans.pid.earned_value.task_2": "Tarefa",
  "p.plans.pid.earned_value.the_one_to_lead_with": "— a que deve vir primeiro",
  "p.plans.pid.earned_value.the_scheduler_also_records_them":
    "O agendador também os registra diariamente.",
  "p.plans.pid.earned_value.this_plan_and_everything_beneath":
    "Este plano e tudo abaixo dele",
  "p.plans.pid.earned_value.unknown": "desconhecido",
  "p.plans.pid.earned_value.use_the_plan_s_method": "Usar o método do plano",
  "p.plans.pid.earned_value.where_it_is_heading": "Para onde está indo",
  "p.plans.pid.earned_value.why_required": "Motivo (obrigatório)",
  "p.plans.pid.edit.main_x": "{t} — Main X",
  "p.plans.pid.flow.180_calendar_days": "180 dias corridos",
  "p.plans.pid.flow.30_calendar_days": "30 dias corridos",
  "p.plans.pid.flow.60_calendar_days": "60 dias corridos",
  "p.plans.pid.flow.90_calendar_days": "90 dias corridos",
  "p.plans.pid.flow.age": "Idade",
  "p.plans.pid.flow.aging_work_in_progress":
    "Envelhecimento do trabalho em andamento",
  "p.plans.pid.flow.arrival_rate": "Taxa de chegada (λ)",
  "p.plans.pid.flow.assignee": "Responsável",
  "p.plans.pid.flow.blocked": "Bloqueado",
  "p.plans.pid.flow.calendar_day": "{v} / dia corrido",
  "p.plans.pid.flow.column": "Coluna",
  "p.plans.pid.flow.columns": "Colunas",
  "p.plans.pid.flow.concentrated": "concentrado",
  "p.plans.pid.flow.constraints": "Restrições",
  "p.plans.pid.flow.count": "Contagem",
  "p.plans.pid.flow.cumulative_flow": "Fluxo cumulativo",
  "p.plans.pid.flow.cycle_time_is_work_in":
    "O tempo de ciclo é o trabalho em andamento dividido pela vazão, então reduzir um limite o encurta sem que ninguém trabalhe mais rápido.",
  "p.plans.pid.flow.detail": "Detalhe",
  "p.plans.pid.flow.distributions": "Distribuições",
  "p.plans.pid.flow.first_pass_yield": "Rendimento na primeira passagem",
  "p.plans.pid.flow.flow": "Fluxo",
  "p.plans.pid.flow.flow_efficiency": "Eficiência de fluxo",
  "p.plans.pid.flow.flow_ppm": "Fluxo — PPM",
  "p.plans.pid.flow.how_long_work_takes": "Quanto tempo o trabalho leva",
  "p.plans.pid.flow.how_long_work_takes_to":
    "Quanto tempo o trabalho leva para ser concluído e quanto está em andamento ao mesmo tempo.",
  "p.plans.pid.flow.implied_cycle_time": "Tempo de ciclo implícito",
  "p.plans.pid.flow.item": "Item",
  "p.plans.pid.flow.limit": "Limite",
  "p.plans.pid.flow.max": "máx.",
  "p.plans.pid.flow.mean": "média",
  "p.plans.pid.flow.measure": "Medida",
  "p.plans.pid.flow.min": "mín.",
  "p.plans.pid.flow.no_cap": "sem limite",
  "p.plans.pid.flow.no_expectation_yet": "sem expectativa ainda",
  "p.plans.pid.flow.no_findings_yet": "Nenhuma constatação ainda.",
  "p.plans.pid.flow.no_finished_work_to_measure":
    "Ainda não há trabalho concluído para medir.",
  "p.plans.pid.flow.not_enough_finished_items_yet":
    "Ainda não há itens concluídos suficientes.",
  "p.plans.pid.flow.not_started_finished_of":
    "{not_started} não iniciados · {finished} concluídos de {tasks}",
  "p.plans.pid.flow.nothing_in_progress": "Nada em andamento.",
  "p.plans.pid.flow.observed_median": "Mediana observada",
  "p.plans.pid.flow.of_the_calendar_time_these":
    "Do tempo de calendário que estes itens levaram, quanto foi de alguém trabalhando neles? A eficiência de fluxo no trabalho do conhecimento costuma ficar entre 5% e 15%, então um valor baixo é normal — a fila é a constatação, não o esforço.",
  "p.plans.pid.flow.of_the_transitions_behind_these":
    "{percent} das transições por trás destes números foram sintetizadas quando o registro foi introduzido, não observadas. Elas se firmam à medida que as movimentações reais do quadro se acumulam.",
  "p.plans.pid.flow.percentiles_are_nearest_rank_so":
    "Os percentis usam o posto mais próximo, então cada um é um item real. O tempo de ciclo é o que a equipe controla; o lead time é o que o solicitante espera, e a diferença entre os dois é quanto tempo o trabalho ficou no backlog.",
  "p.plans.pid.flow.recoverable": "Recuperável",
  "p.plans.pid.flow.rework": "Retrabalho",
  "p.plans.pid.flow.rule": "Regra",
  "p.plans.pid.flow.service_level_expectation":
    "Expectativa de nível de serviço",
  "p.plans.pid.flow.status": "Status",
  "p.plans.pid.flow.strong_against_the_5_15":
    "Forte em relação à norma de 5–15%.",
  "p.plans.pid.flow.subject": "Assunto",
  "p.plans.pid.flow.the_mean_is_shown_for":
    "A média é exibida por completude e não descreve nenhum item real — cite o p85.",
  "p.plans.pid.flow.the_plan_s_overall_ratio":
    ": a razão geral do plano ({percent}) está bem longe da do item típico ({percent2}). A maioria dos itens flui; uma minoria está travada. Isso exige uma correção diferente de uma entrega uniformemente lenta — encontre esses itens em vez de refazer o processo.",
  "p.plans.pid.flow.the_waste_is": "O desperdício é",
  "p.plans.pid.flow.throughput": "Vazão (μ)",
  "p.plans.pid.flow.typical_median_item": "Típico. Item mediano: {percent}.",
  "p.plans.pid.flow.unassigned": "sem responsável",
  "p.plans.pid.flow.unusually_high_this_usually_means":
    "Incomumente alto — isso geralmente significa que o quadro não está sendo mantido atualizado, e não que a fila acabou.",
  "p.plans.pid.flow.utilization": "Utilização (ρ)",
  "p.plans.pid.flow.vs_expectation": "vs expectativa",
  "p.plans.pid.flow.window": "Janela",
  "p.plans.pid.flow.work_in_progress": "Trabalho em andamento",
  "p.plans.pid.flow.work_in_progress_2": "Trabalho em andamento (κ)",
  "p.plans.pid.governance.actual": "Real",
  "p.plans.pid.governance.approved": "aprovado",
  "p.plans.pid.governance.approved_with_conditions": "aprovado com condições",
  "p.plans.pid.governance.benefit": "Benefício",
  "p.plans.pid.governance.benefit_category": "Categoria do benefício",
  "p.plans.pid.governance.capex": "capex",
  "p.plans.pid.governance.category": "Categoria",
  "p.plans.pid.governance.choose_objective": "escolher objetivo…",
  "p.plans.pid.governance.commit": "Confirmar",
  "p.plans.pid.governance.commit_anyway": "Confirmar mesmo assim",
  "p.plans.pid.governance.committing_this_puts_them_over":
    "Confirmar isto os deixa acima da capacidade.",
  "p.plans.pid.governance.compliance": "conformidade",
  "p.plans.pid.governance.cost_saving": "economia de custos",
  "p.plans.pid.governance.currency": "Moeda",
  "p.plans.pid.governance.decision": "Decisão",
  "p.plans.pid.governance.governance_ppm": "Governança — PPM",
  "p.plans.pid.governance.hold": "em espera",
  "p.plans.pid.governance.line": "Linha",
  "p.plans.pid.governance.materialized": "materializados: {materialized}",
  "p.plans.pid.governance.max_exposure": "exposição máxima: {max_exposure}",
  "p.plans.pid.governance.new_benefit": "Novo benefício",
  "p.plans.pid.governance.new_budget_line": "Nova linha de orçamento",
  "p.plans.pid.governance.new_milestone": "Novo marco",
  "p.plans.pid.governance.new_milestone_due": "Prazo do novo marco",
  "p.plans.pid.governance.new_risk": "Novo risco",
  "p.plans.pid.governance.next": "próximo: {next_gate}",
  "p.plans.pid.governance.objective": "Objetivo",
  "p.plans.pid.governance.open_risks": "riscos abertos: {open}",
  "p.plans.pid.governance.opex": "opex",
  "p.plans.pid.governance.other": "outro",
  "p.plans.pid.governance.planned": "Planejado",
  "p.plans.pid.governance.planned_actual_variance":
    ": planejado {money} · real {money2} · variação {money3}",
  "p.plans.pid.governance.planned_major": "Planejado (principal)",
  "p.plans.pid.governance.proposed_does_not_count_until":
    "proposto (não conta até ser confirmado)",
  "p.plans.pid.governance.quality": "qualidade",
  "p.plans.pid.governance.range_for_cost_risk": "Faixa para o risco de custo",
  "p.plans.pid.governance.realized": "Realizado",
  "p.plans.pid.governance.record_actual": "Registrar real",
  "p.plans.pid.governance.rejected": "rejeitado",
  "p.plans.pid.governance.revenue": "receita",
  "p.plans.pid.governance.review": "Revisão",
  "p.plans.pid.governance.risk": "Risco",
  "p.plans.pid.governance.risk_reduction": "redução de risco",
  "p.plans.pid.governance.roi": "· ROI {toFixed}%",
  "p.plans.pid.governance.stage": "etapa: {v}",
  "p.plans.pid.governance.status": "Status",
  "p.plans.pid.governance.target": "Meta",
  "p.plans.pid.governance.target_major": "Meta (principal)",
  "p.plans.pid.governance.target_realized_spend":
    ": meta {money} · realizado {money2} · gasto {money3}",
  "p.plans.pid.governance.weight": "peso",
  "p.plans.pid.governance.worker_uuid": "worker:<uuid>",
  "p.plans.pid.handover.discard": "Descartar",
  "p.plans.pid.handover.from": "De",
  "p.plans.pid.handover.hand_over": "Transferir",
  "p.plans.pid.handover.handover": "Transferência",
  "p.plans.pid.handover.handover_ppm": "Transferência — PPM",
  "p.plans.pid.handover.kind": "Tipo",
  "p.plans.pid.handover.onboarding_and_offboarding":
    "Integração e desligamento",
  "p.plans.pid.handover.person": "person:…",
  "p.plans.pid.handover.person_empty_releases_the_work":
    "person:… (vazio libera o trabalho)",
  "p.plans.pid.handover.plan": "← Plano",
  "p.plans.pid.handover.preview": "Pré-visualização",
  "p.plans.pid.handover.reason_recorded_with_each_item":
    "Motivo (registrado com cada item)",
  "p.plans.pid.handover.release": "Liberar",
  "p.plans.pid.handover.the_new_person_s_capacity":
    "A capacidade da nova pessoa não é verificada aqui.",
  "p.plans.pid.handover.to": "Para",
  "p.plans.pid.handover.what": "O quê",
  "p.plans.pid.handover.what_holds_that_is_still":
    "O que {from_person} detém e ainda está ativo",
  "p.plans.pid.handover.who_is_handing_over": "Quem está transferindo?",
  "p.plans.pid.import.bring_tasks_in_from_jira":
    "Traga tarefas do Jira, do Asana ou do Microsoft Project. Você vê primeiro o que aconteceria; nada é gravado até você confirmar.",
  "p.plans.pid.import.choose_another_file": "Escolher outro arquivo",
  "p.plans.pid.import.file": "Arquivo",
  "p.plans.pid.import.import": "Importar",
  "p.plans.pid.import.import_ppm": "Importar — PPM",
  "p.plans.pid.import.imported": "Importado",
  "p.plans.pid.import.plan": "← Plano",
  "p.plans.pid.import.preview": "Pré-visualização",
  "p.plans.pid.import.tracker": "Rastreador",
  "p.plans.pid.import.where_from": "De onde",
  "p.plans.pid.joint_risk.a_late_project_usually_costs":
    "Um projeto atrasado costuma custar mais também, então isto executa custo e cronograma juntos.",
  "p.plans.pid.joint_risk.assumptions": "Premissas",
  "p.plans.pid.joint_risk.band_when_there_is_no":
    "Faixa quando não há estimativa (± %)",
  "p.plans.pid.joint_risk.both_at_once": "Os dois ao mesmo tempo",
  "p.plans.pid.joint_risk.budget": "orçamento",
  "p.plans.pid.joint_risk.budget_2": "Orçamento",
  "p.plans.pid.joint_risk.budget_lines_in_are_not":
    "As linhas de orçamento em {join} não fazem parte desta execução; as moedas nunca são somadas.",
  "p.plans.pid.joint_risk.business_days_late":
    "{days} dias úteis de atraso, {money}",
  "p.plans.pid.joint_risk.business_days_late_2": "dias úteis de atraso →",
  "p.plans.pid.joint_risk.cost": "Custo",
  "p.plans.pid.joint_risk.cost_2": "Custo — {title}: {costSkippedText}",
  "p.plans.pid.joint_risk.cost_and_schedule_risk_ppm":
    "Risco de custo e cronograma — PPM",
  "p.plans.pid.joint_risk.cost_and_schedule_together":
    "Custo e cronograma juntos",
  "p.plans.pid.joint_risk.cost_lines": "Linhas de custo",
  "p.plans.pid.joint_risk.cost_of_each_business_day":
    "Custo de cada dia útil de atraso (unidades principais)",
  "p.plans.pid.joint_risk.cost_risk": "Risco de custo",
  "p.plans.pid.joint_risk.critical_path_finish_the_plan":
    "Término do caminho crítico (a data do próprio plano)",
  "p.plans.pid.joint_risk.currency_if_the_plan_has":
    "Moeda (se o plano tiver várias)",
  "p.plans.pid.joint_risk.distribution": "Distribuição",
  "p.plans.pid.joint_risk.each_dot_is_one_simulation":
    "Cada ponto é uma simulação (uma amostra reduzida). À direita da linha de pontualidade é atraso; acima da linha do orçamento é estouro. A tabela acima tem as contagens completas.",
  "p.plans.pid.joint_risk.estimate": "Estimativa",
  "p.plans.pid.joint_risk.finish": "Término",
  "p.plans.pid.joint_risk.finish_date_at_each_percentile":
    "Data de término em cada percentil",
  "p.plans.pid.joint_risk.how_the_simulated_outcomes_fall":
    "Como os resultados simulados se distribuem em torno do orçamento e do término do caminho crítico",
  "p.plans.pid.joint_risk.independence": "Independência.",
  "p.plans.pid.joint_risk.iterations": "Iterações",
  "p.plans.pid.joint_risk.line": "Linha",
  "p.plans.pid.joint_risk.low_planned_high": "Baixo / planejado / alto",
  "p.plans.pid.joint_risk.mean_delay_cost_included":
    "Média (custo de atraso incluído)",
  "p.plans.pid.joint_risk.no_register_risk_was_used":
    "Nenhum risco do registro foi usado nesta execução.",
  "p.plans.pid.joint_risk.not_enough_data": "dados insuficientes",
  "p.plans.pid.joint_risk.of_which_delay_cost_on":
    "Do qual custo de atraso, em média",
  "p.plans.pid.joint_risk.on_time": "no prazo",
  "p.plans.pid.joint_risk.outcome": "Resultado",
  "p.plans.pid.joint_risk.per_line_estimate_in":
    "Estimativa por linha em {currency}",
  "p.plans.pid.joint_risk.pert": "PERT",
  "p.plans.pid.joint_risk.plan": "← Plano",
  "p.plans.pid.joint_risk.risks": "Riscos",
  "p.plans.pid.joint_risk.run": "Executar",
  "p.plans.pid.joint_risk.running": "Executando…",
  "p.plans.pid.joint_risk.scatter_of_simulated_total_cost":
    "Dispersão do custo total simulado contra os dias úteis de atraso, com o orçamento e a linha de pontualidade",
  "p.plans.pid.joint_risk.schedule":
    "Cronograma — {title}: {skippedReasonText}",
  "p.plans.pid.joint_risk.schedule_risk": "Risco de cronograma",
  "p.plans.pid.joint_risk.seed": "Semente",
  "p.plans.pid.joint_risk.seed_the_same_seed_gives":
    "Semente {seed}: a mesma semente dá o mesmo resultado.",
  "p.plans.pid.joint_risk.share": "Participação",
  "p.plans.pid.joint_risk.simulations": "Simulações",
  "p.plans.pid.joint_risk.total_cost_delay_cost_included":
    "Custo total, custo de atraso incluído",
  "p.plans.pid.joint_risk.triangular": "Triangular",
  "p.plans.pid.leveling.allow_critical_tasks_to_move":
    "Permitir que tarefas críticas se movam (isso pode atrasar o término)",
  "p.plans.pid.leveling.apply": "Aplicar",
  "p.plans.pid.leveling.apply_the_ticked_moves":
    "Aplicar as movimentações marcadas",
  "p.plans.pid.leveling.balance_workloads_resource_leveling":
    "Equilibrar cargas de trabalho (nivelamento de recursos)",
  "p.plans.pid.leveling.could_not_be_fixed": "Não foi possível corrigir",
  "p.plans.pid.leveling.critical": "crítico",
  "p.plans.pid.leveling.each_start_date_the_proposal":
    "Cada data de início que a proposta alteraria",
  "p.plans.pid.leveling.finish": "Término",
  "p.plans.pid.leveling.float": "Folga",
  "p.plans.pid.leveling.i_have_read_the_proposal":
    "Li a proposta e quero que estas datas de início sejam alteradas",
  "p.plans.pid.leveling.later_by": "Adiado em",
  "p.plans.pid.leveling.load_per_task_of_a":
    "Carga por tarefa (% do dia de uma pessoa)",
  "p.plans.pid.leveling.loading": "Carregando…",
  "p.plans.pid.leveling.no_start_date_would_change":
    "Nenhuma data de início seria alterada.",
  "p.plans.pid.leveling.nobody_is_over_allocated_so":
    "Ninguém está superalocado, portanto não há nada a nivelar.",
  "p.plans.pid.leveling.over_allocated_now": "Superalocados agora",
  "p.plans.pid.leveling.person": "Pessoa",
  "p.plans.pid.leveling.plan": "← Plano",
  "p.plans.pid.leveling.propose_again": "Propor novamente",
  "p.plans.pid.leveling.proposed_start_dates": "Datas de início propostas",
  "p.plans.pid.leveling.refused": "Recusado: {note}",
  "p.plans.pid.leveling.resource_leveling_ppm": "Nivelamento de recursos — PPM",
  "p.plans.pid.leveling.start": "Início",
  "p.plans.pid.leveling.start_finish_business_days":
    "{pid}: início {v}{start_days}, término {v2}{finish_days} dias úteis",
  "p.plans.pid.leveling.suggests_moving_tasks_so_that":
    "Sugere mover tarefas para que ninguém seja reservado além do que consegue fazer. Nada muda até você confirmar.",
  "p.plans.pid.leveling.task": "Tarefa",
  "p.plans.pid.leveling.the_search_stopped_at_its":
    "a busca parou no seu limite",
  "p.plans.pid.leveling.this_is_advice_nothing_changes":
    "Isto é uma recomendação. Nada muda até você escolher movimentações abaixo e aplicá-las.",
  "p.plans.pid.leveling.why_kept_with_each_audit":
    "Motivo (mantido com cada registro de auditoria)",
  "p.plans.pid.leveling.working_calendar": "Calendário de trabalho",
  "p.plans.pid.more_tools": "Mais ferramentas",
  "p.plans.pid.onboarding.access": "Acesso",
  "p.plans.pid.onboarding.access_record": "Registro de acesso",
  "p.plans.pid.onboarding.action": "Ação",
  "p.plans.pid.onboarding.cancel_checklist": "Cancelar lista de verificação",
  "p.plans.pid.onboarding.checklists": "Listas de verificação",
  "p.plans.pid.onboarding.complete_checklist": "Concluir lista de verificação",
  "p.plans.pid.onboarding.done": "Concluído",
  "p.plans.pid.onboarding.effective":
    "{status}{v} · em vigor desde {effective_date} · {progressText}",
  "p.plans.pid.onboarding.from":
    "{status} · desde {effective_date} · {progressText}",
  "p.plans.pid.onboarding.granted": "Concedido",
  "p.plans.pid.onboarding.hand_over_someone_s_work":
    "Transferir o trabalho de alguém",
  "p.plans.pid.onboarding.no_access_is_outstanding":
    "● Nenhum acesso pendente.",
  "p.plans.pid.onboarding.none_yet": "Nenhum ainda.",
  "p.plans.pid.onboarding.onboarding_and_offboarding":
    "Integração e desligamento",
  "p.plans.pid.onboarding.onboarding_and_offboarding_ppm":
    "Integração e desligamento — PPM",
  "p.plans.pid.onboarding.person": "person:…",
  "p.plans.pid.onboarding.person_2": "Pessoa",
  "p.plans.pid.onboarding.plan": "← Plano",
  "p.plans.pid.onboarding.ready_to_close": "Pronto para encerrar?",
  "p.plans.pid.onboarding.reason": "Motivo",
  "p.plans.pid.onboarding.reason_for_cancelling": "Motivo do cancelamento",
  "p.plans.pid.onboarding.record": "Registrar",
  "p.plans.pid.onboarding.record_access": "Registrar acesso",
  "p.plans.pid.onboarding.reopen": "Reabrir",
  "p.plans.pid.onboarding.repo_write": "gravação no repositório",
  "p.plans.pid.onboarding.revoked": "Revogado",
  "p.plans.pid.onboarding.save": "Salvar",
  "p.plans.pid.onboarding.since":
    "▲ {person_ref}: {access} (desde {granted_on})",
  "p.plans.pid.onboarding.skip": "Pular",
  "p.plans.pid.onboarding.start": "Iniciar",
  "p.plans.pid.onboarding.start_a_checklist":
    "Iniciar uma lista de verificação",
  "p.plans.pid.onboarding.takes_effect": "Entra em vigor",
  "p.plans.pid.onboarding.what": "O quê",
  "p.plans.pid.onboarding.who_holds_what_access_is":
    "Quem tem qual acesso diz respeito a pessoas, por isso exige a permissão de gerenciar pessoas.",
  "p.plans.pid.onboarding.why_skip": "Por que pular?",
  "p.plans.pid.raci.add_a_person_person_or":
    "Adicionar uma pessoa (person:… ou worker:…)",
  "p.plans.pid.raci.add_a_person_to_start":
    "Adicione uma pessoa para começar a atribuir as letras.",
  "p.plans.pid.raci.add_column": "Adicionar coluna",
  "p.plans.pid.raci.export_to_a_spreadsheet": "Exportar para uma planilha",
  "p.plans.pid.raci.for_each_task_who_does":
    "Para cada tarefa: quem a executa, quem a aprova, quem é consultado e quem é informado.",
  "p.plans.pid.raci.have_nobody_accountable":
    "{missing_accountable} sem nenhum responsável final.",
  "p.plans.pid.raci.have_nobody_responsible":
    "{missing_responsible} sem nenhum executor.",
  "p.plans.pid.raci.how_many_tasks_each_person":
    "Em quantas tarefas cada pessoa tem cada letra",
  "p.plans.pid.raci.legend": "Legenda",
  "p.plans.pid.raci.load_by_person": "Carga por pessoa",
  "p.plans.pid.raci.loading": "Carregando…",
  "p.plans.pid.raci.of_tasks_are_complete_one":
    "{complete} de {tasks} tarefas estão completas (um responsável final, pelo menos um executor).",
  "p.plans.pid.raci.person": "Pessoa",
  "p.plans.pid.raci.plan": "← Plano",
  "p.plans.pid.raci.raci_ppm": "RACI — PPM",
  "p.plans.pid.raci.this_plan_has_no_tasks":
    "Este plano ainda não tem tarefas.",
  "p.plans.pid.raci.who_does_what_raci_matrix": "Quem faz o quê (matriz RACI)",
  "p.plans.pid.schedule.dependency_violated_successor_starts":
    "Dependência violada: a sucessora começa em {actual_start}, mas só pode começar a partir de {earliest_start}.",
  "p.plans.pid.schedule.schedule_ppm": "Cronograma — PPM",
  "p.plans.pid.schedule_health.as_of": "Em {as_of} ·",
  "p.plans.pid.schedule_health.baseline_version":
    "versão da linha de base {version}",
  "p.plans.pid.schedule_health.check": "Verificação",
  "p.plans.pid.schedule_health.checks_the_schedule_for_common":
    "Verifica o cronograma em busca de problemas comuns, como tarefas sem vínculos ou sem estimativa.",
  "p.plans.pid.schedule_health.critical_path": "Caminho crítico",
  "p.plans.pid.schedule_health.held_to": "Limitado a",
  "p.plans.pid.schedule_health.loading": "Carregando…",
  "p.plans.pid.schedule_health.no_baseline_taken":
    "nenhuma linha de base registrada",
  "p.plans.pid.schedule_health.plan": "← Plano",
  "p.plans.pid.schedule_health.re_run": "Executar de novo",
  "p.plans.pid.schedule_health.result": "Resultado",
  "p.plans.pid.schedule_health.schedule_health": "Saúde do cronograma",
  "p.plans.pid.schedule_health.schedule_health_ppm":
    "Saúde do cronograma — PPM",
  "p.plans.pid.schedule_health.see_the_note": "Ver a nota",
  "p.plans.pid.schedule_health.status_date": "Data de referência",
  "p.plans.pid.schedule_health.the_fourteen_schedule_health_checks":
    "As quatorze verificações de saúde do cronograma",
  "p.plans.pid.schedule_health.to_fix": "A corrigir",
  "p.plans.pid.schedule_health.what_was_found": "O que foi encontrado",
  "p.plans.pid.schedule_risk.assumptions": "Premissas",
  "p.plans.pid.schedule_risk.band_for_tasks_with_no":
    "Faixa para tarefas sem estimativa (± %)",
  "p.plans.pid.schedule_risk.critical_in": "Crítica em",
  "p.plans.pid.schedule_risk.critical_path": "Caminho crítico",
  "p.plans.pid.schedule_risk.critical_path_finish_the_plan":
    "Término pelo caminho crítico (a data do próprio plano)",
  "p.plans.pid.schedule_risk.distribution": "Distribuição",
  "p.plans.pid.schedule_risk.each_bar_is_a_finish":
    "Cada barra é uma data de término; quanto mais alta, mais simulações terminaram ali. As linhas contínuas são P50, P80 e P90 (rotuladas); a linha tracejada é a data do próprio plano.",
  "p.plans.pid.schedule_risk.earliest_latest":
    "{join}. Mais cedo {earliest}, mais tarde {latest}.",
  "p.plans.pid.schedule_risk.estimate": "Estimativa",
  "p.plans.pid.schedule_risk.finish": "Término",
  "p.plans.pid.schedule_risk.finish_date": "Data de término",
  "p.plans.pid.schedule_risk.finish_date_at_each_percentile":
    "Data de término em cada percentil",
  "p.plans.pid.schedule_risk.histogram_of_simulated_finish_dates":
    "Histograma das datas de término simuladas, com P50, P80 e P90 marcados",
  "p.plans.pid.schedule_risk.independence": "Independência.",
  "p.plans.pid.schedule_risk.iterations": "Iterações",
  "p.plans.pid.schedule_risk.left_out": "Deixados de fora:",
  "p.plans.pid.schedule_risk.many_what_if_runs_of":
    "Muitas execuções hipotéticas do cronograma mostram a faixa de datas de término e a probabilidade de cada uma.",
  "p.plans.pid.schedule_risk.no_register_risk_was_used":
    "Nenhum risco do registro foi usado como evento do cronograma.",
  "p.plans.pid.schedule_risk.not_enough_data": "dados insuficientes",
  "p.plans.pid.schedule_risk.not_simulated_because_the_schedule":
    "Não simuladas, porque o cronograma não consegue posicioná-las: {join}.",
  "p.plans.pid.schedule_risk.of": "{date}: {count} de {iterations}",
  "p.plans.pid.schedule_risk.of_runs": "{criticalityText} das execuções",
  "p.plans.pid.schedule_risk.optimistic_likely_pessimistic_busine":
    "Otimista / provável / pessimista (dias úteis)",
  "p.plans.pid.schedule_risk.outcome": "Desfecho",
  "p.plans.pid.schedule_risk.per_task_estimate_criticality_and":
    "Estimativa, criticidade e sensibilidade por tarefa",
  "p.plans.pid.schedule_risk.plan": "← Plano",
  "p.plans.pid.schedule_risk.plan_2": "plano",
  "p.plans.pid.schedule_risk.risks_used": "Riscos usados",
  "p.plans.pid.schedule_risk.run": "Executar",
  "p.plans.pid.schedule_risk.run_on_against_the_plan":
    "Executado em {ranOn} sobre o plano como está · semente {seed} · {iterations} iterações · {distributionText} · calendário {calendar}",
  "p.plans.pid.schedule_risk.running": "Executando…",
  "p.plans.pid.schedule_risk.schedule_risk": "Risco do cronograma",
  "p.plans.pid.schedule_risk.schedule_risk_ppm": "Risco do cronograma — PPM",
  "p.plans.pid.schedule_risk.seed": "Semente",
  "p.plans.pid.schedule_risk.seed_the_same_seed_gives":
    "Semente {seed}: a mesma semente dá o mesmo resultado. Até {max_iterations} iterações.",
  "p.plans.pid.schedule_risk.sensitivity": "Sensibilidade",
  "p.plans.pid.schedule_risk.share": "Participação",
  "p.plans.pid.schedule_risk.simulations": "Simulações",
  "p.plans.pid.schedule_risk.task": "Tarefa",
  "p.plans.pid.schedule_risk.the_same_figures_as_a":
    "Os mesmos números em tabela",
  "p.plans.pid.schedule_risk.triangular": "Distribuição triangular",
  "p.plans.pid.schedule_risk.weighted_towards_the_likely_value":
    "Ponderada em direção ao valor provável (PERT)",
  "p.plans.pid.schedule_risk.what_drives_it": "O que a determina",
  "p.plans.pid.schedule_risk.when_it_finishes": "Quando termina",
  "p.plans.pid.wbs.code": "Código",
  "p.plans.pid.wbs.level": "Nível",
  "p.plans.pid.wbs.no_tasks_yet": "Ainda não há tarefas.",
  "p.plans.pid.wbs.plan": "← Plano",
  "p.plans.pid.wbs.sub_tasks": "Subtarefas",
  "p.plans.pid.wbs.task": "Tarefa",
  "p.plans.pid.wbs.tasks_is_too_many_to":
    "{length} tarefas são demais para aninhar de forma útil; a tabela mostra todas.",
  "p.plans.pid.wbs.the_plan_s_tasks_each":
    "As tarefas do plano, cada uma dividida em suas subtarefas. O código de cada linha serve para leitura: muda quando o trabalho é adicionado ou movido.",
  "p.plans.pid.wbs.work_breakdown": "Estrutura analítica do trabalho",
  "p.plans.pid.wbs.work_breakdown_as_a_table":
    "Estrutura analítica do trabalho em tabela",
  "p.plans.pid.wbs.work_breakdown_ppm": "Estrutura analítica do trabalho — PPM",
  "p.plans.pid.wbs.work_breakdown_tree":
    "Árvore da estrutura analítica do trabalho",
  "p.plans.pid.what_if.after": "Depois",
  "p.plans.pid.what_if.before": "Antes",
  "p.plans.pid.what_if.business_days": "Dias úteis",
  "p.plans.pid.what_if.choose": "Escolher…",
  "p.plans.pid.what_if.commit_these_changes": "Confirmar estas alterações",
  "p.plans.pid.what_if.constraint": "Restrição",
  "p.plans.pid.what_if.critical": "■ crítica",
  "p.plans.pid.what_if.critical_path": "Caminho crítico",
  "p.plans.pid.what_if.date": "Data",
  "p.plans.pid.what_if.discard": "Descartar",
  "p.plans.pid.what_if.edit": "Editar",
  "p.plans.pid.what_if.finish_to_finish": "Término a término",
  "p.plans.pid.what_if.finish_to_start": "Término a início",
  "p.plans.pid.what_if.float": "Folga",
  "p.plans.pid.what_if.follows": "{name} segue {name2} ({toUpperCase})",
  "p.plans.pid.what_if.joins_the_critical_path":
    "Entra no caminho crítico: {join}.",
  "p.plans.pid.what_if.lag_business_days": "Defasagem (dias úteis)",
  "p.plans.pid.what_if.leaves_it": "Sai dele: {join}.",
  "p.plans.pid.what_if.link": "Vínculo",
  "p.plans.pid.what_if.new_start": "Novo início",
  "p.plans.pid.what_if.no_task_s_dates_or":
    "Nenhuma data ou folga de tarefa muda.",
  "p.plans.pid.what_if.none_clear_it": "Nenhuma (limpar)",
  "p.plans.pid.what_if.none_yet": "Ainda nenhuma.",
  "p.plans.pid.what_if.now_takes_of_the_day":
    "“{name}” agora ocupa {units_percent}% do dia para {hoursText} de trabalho (calculado a partir da sua regra de agendamento).",
  "p.plans.pid.what_if.of_work_has_no_rate":
    "▲ {hoursText} de trabalho não têm taxa, portanto não são custeadas.",
  "p.plans.pid.what_if.plan": "← Plano",
  "p.plans.pid.what_if.planned_labour_cost":
    "Custo planejado de mão de obra: {deltaText} ({deltaText2} → {deltaText3}).",
  "p.plans.pid.what_if.predecessor": "Predecessora",
  "p.plans.pid.what_if.preview": "Pré-visualizar",
  "p.plans.pid.what_if.reason": "Motivo",
  "p.plans.pid.what_if.reason_recorded_with_each_change":
    "Motivo (registrado com cada alteração)",
  "p.plans.pid.what_if.remove": "Remover",
  "p.plans.pid.what_if.stage_an_edit": "Preparar uma edição",
  "p.plans.pid.what_if.stage_edit": "Preparar edição",
  "p.plans.pid.what_if.staged_edits": "Edições preparadas",
  "p.plans.pid.what_if.start_to_finish": "Início a término",
  "p.plans.pid.what_if.start_to_start": "Início a início",
  "p.plans.pid.what_if.successor": "Sucessora",
  "p.plans.pid.what_if.task": "Tarefa",
  "p.plans.pid.what_if.tasks_whose_dates_float_or":
    "Tarefas cujas datas, folga ou situação crítica mudam",
  "p.plans.pid.what_if.try_changes_on_a_copy":
    "Teste alterações em uma cópia. Nada é salvo até você confirmar.",
  "p.plans.pid.what_if.type": "Tipo",
  "p.plans.pid.what_if.what_if": "Hipótese",
  "p.plans.pid.what_if.what_if_ppm": "Hipótese — PPM",
  "p.plans.pid.work.business_days_of_a_day":
    "{duration_days} dias úteis · {units_percent}% de um dia · {hoursText} de trabalho · {v}",
  "p.plans.pid.work.change": "Alterar",
  "p.plans.pid.work.costs": "Custos",
  "p.plans.pid.work.day": "Dia",
  "p.plans.pid.work.duration_business_days": "Duração (dias úteis)",
  "p.plans.pid.work.edit_assignment": "Editar atribuição",
  "p.plans.pid.work.falls_outside_this_window":
    "{hoursText} ficam fora desta janela.",
  "p.plans.pid.work.hours_per_each_row_adds":
    "Horas por {grain}; cada linha soma o trabalho da tarefa, e a última linha soma o do plano.",
  "p.plans.pid.work.how_long_each_task_takes":
    "Quanto tempo cada tarefa leva, quanto do dia de uma pessoa ela usa e a quanto trabalho isso equivale.",
  "p.plans.pid.work.month": "Mês",
  "p.plans.pid.work.no_assignment_yet_give_a":
    "Ainda não há atribuição. Informe uma duração e unidades, e o trabalho é calculado.",
  "p.plans.pid.work.no_task_carries_work_yet":
    "Nenhuma tarefa tem trabalho ainda, portanto não há nada a distribuir.",
  "p.plans.pid.work.not_placed": "(não posicionada)",
  "p.plans.pid.work.percentages": "Porcentagens",
  "p.plans.pid.work.period": "Período",
  "p.plans.pid.work.plan": "← Plano",
  "p.plans.pid.work.plan_2": "Plano",
  "p.plans.pid.work.rule": "Regra",
  "p.plans.pid.work.save_assignment": "Salvar atribuição",
  "p.plans.pid.work.save_contour": "Salvar perfil",
  "p.plans.pid.work.so_its_work_is_not":
    "▲ {title} ({hoursText}) {reason}, portanto seu trabalho não está nos intervalos.",
  "p.plans.pid.work.spread": "Distribuição",
  "p.plans.pid.work.task": "Tarefa",
  "p.plans.pid.work.tasks": "Tarefas",
  "p.plans.pid.work.this_plan_has_no_scheduled":
    "Este plano ainda não tem tarefas agendadas.",
  "p.plans.pid.work.total": "Total geral",
  "p.plans.pid.work.units": "Unidades (%)",
  "p.plans.pid.work.units_of_the_day": "Unidades (% do dia)",
  "p.plans.pid.work.week": "Semana",
  "p.plans.pid.work.work_and_assignments": "Trabalho e atribuições",
  "p.plans.pid.work.work_and_assignments_ppm": "Trabalho e atribuições — PPM",
  "p.plans.pid.work.work_hours": "Trabalho (horas)",
  "p.plans.pid.work.work_over_time": "Trabalho ao longo do tempo",
  "p.prioritization.all": "todos",
  "p.prioritization.apply": "Aplicar",
  "p.prioritization.band": "faixa",
  "p.prioritization.band_2": "Faixa",
  "p.prioritization.component": "Componente",
  "p.prioritization.coverage": "cobertura",
  "p.prioritization.coverage_2": "Cobertura",
  "p.prioritization.explain": "Explicar",
  "p.prioritization.hide": "Ocultar",
  "p.prioritization.high": "alta",
  "p.prioritization.loading_the_breakdown": "Carregando o detalhamento…",
  "p.prioritization.low": "baixa",
  "p.prioritization.medium": "média",
  "p.prioritization.missing_evidence": "Evidências ausentes",
  "p.prioritization.no_evidence_for_any_component":
    "Nenhuma evidência para qualquer componente — este plano não tem pontuação.",
  "p.prioritization.no_plans_match_that_filter":
    "Nenhum plano corresponde a esse filtro.",
  "p.prioritization.none": "nenhuma",
  "p.prioritization.of_plans_one": "{length} de {scored_of} plano",
  "p.prioritization.of_plans_other": "{length} de {scored_of} planos",
  "p.prioritization.plan": "Plano",
  "p.prioritization.points": "Pontos",
  "p.prioritization.prioritization": "Priorização",
  "p.prioritization.prioritization_ppm": "Priorização — PPM",
  "p.prioritization.score": "Pontuação",
  "p.prioritization.shows_how_much_of_the":
    "mostra quanto da pontuação foi de fato respaldado por dados.",
  "p.prioritization.smart_score_over_roi_strategic":
    "Pontuação inteligente sobre ROI, alinhamento estratégico, revisão de especialistas, risco, demanda, prioridade MoSCoW e ímpeto. Os componentes sem evidências são descartados e os demais renormalizados —",
  "p.prioritization.unscored": "sem pontuação",
  "p.prioritization.value": "Valor",
  "p.prioritization.weight": "Peso",
  "p.proposals.actions": "Ações",
  "p.proposals.currency": "Moeda",
  "p.proposals.live": "ativo:",
  "p.proposals.promote": "promover",
  "p.proposals.proposal": "proposta:",
  "p.proposals.proposals_ppm": "Propostas — PPM",
  "p.proposals.requested": "Solicitado",
  "p.proposals.status": "Situação",
  "p.proposals.status_filter": "Filtro de situação",
  "p.proposals.target": "Destino",
  "p.proposals.target_collection": "Coleção de destino",
  "p.proposals.title": "Título",
  "p.regulator.actual": "Real",
  "p.regulator.benefits": "Benefícios:",
  "p.regulator.currency": "Moeda",
  "p.regulator.gate_decisions": "· Decisões de portão:",
  "p.regulator.members": "Membros:",
  "p.regulator.names_withheld_abac_mask_obligation":
    "Nomes ocultos (obrigação de máscara ABAC).",
  "p.regulator.no_budget_lines": "Nenhuma linha de orçamento.",
  "p.regulator.no_portfolios_recorded": "Nenhum portfólio registrado.",
  "p.regulator.none": "nenhum",
  "p.regulator.planned": "Planejado",
  "p.regulator.ppm": "{t} — PPM",
  "p.regulator.stage": "etapa: {v}",
  "p.regulator.target_realized": "meta {money} / realizado {money2}",
  "p.reports.collection": "Coleção",
  "p.reports.csv": "CSV",
  "p.reports.fields_comma_separated": "campos (separados por vírgula)",
  "p.reports.name_contains": "o nome contém…",
  "p.reports.reports_ppm": "Relatórios — PPM",
  "p.resources.a_multi_skill_booking_counts":
    "Uma reserva com várias habilidades conta por inteiro para cada uma delas, então as linhas de habilidades se sobrepõem e não somam o total.",
  "p.resources.by": "Por",
  "p.resources.demand_fte": "Demanda em FTE",
  "p.resources.demand_supply_and_gap_by":
    "Demanda, oferta e lacuna por período e {group_by}",
  "p.resources.every_figure_as_a_table": "Todos os números em tabela",
  "p.resources.from": "De",
  "p.resources.gap_fte": "Lacuna em FTE",
  "p.resources.gap_per_in_fte_supply":
    "Lacuna por {period}, em FTE (oferta menos demanda)",
  "p.resources.loading": "Carregando…",
  "p.resources.month": "Mês",
  "p.resources.no_demand_or_supply_in":
    "Nenhuma demanda ou oferta nesta janela.",
  "p.resources.not_known": "desconhecido",
  "p.resources.of_which_unassigned": "Dos quais não atribuídos",
  "p.resources.people": "Pessoas",
  "p.resources.per": "Por",
  "p.resources.per_by_scope":
    "{periodLabel} · por {period} · por {group_by} · escopo: {kind}",
  "p.resources.period": "Período",
  "p.resources.proposed_fte": "FTE proposto",
  "l.processes.name_problem": "Dê ao processo um nome de 1 a 120 caracteres.",
  "l.processes.steps_problem": "Um processo tem de 1 a {max} etapas.",
  "l.processes.title_problem":
    "Etapa {n}: dê a ela um título de 1 a 200 caracteres.",
  "l.processes.person_problem":
    "Etapa {n}: a pessoa deve ser uma referência person: ou worker:.",
  "l.processes.kind_approve": "Aprovar",
  "l.processes.kind_todo": "A fazer",
  "l.processes.kind_notify": "Notificar",
  "l.processes.state_pending": "não alcançada",
  "l.processes.state_waiting": "em espera",
  "l.processes.state_completed": "concluída",
  "l.processes.state_skipped": "ignorada",
  "l.processes.state_rejected": "rejeitada",
  "l.processes.state_failed": "com falha",
  "l.processes.run_running": "em execução",
  "l.processes.run_completed": "concluída",
  "l.processes.run_rejected": "rejeitada",
  "l.processes.run_cancelled": "cancelada",
  "l.processes.run_failed": "com falha",
  "p.processes.processes": "Processos de aprovação",
  "p.processes.processes_ppm": "Processos de aprovação — PPM",
  "p.processes.intro":
    "Um processo é uma lista de etapas, executadas uma após a outra: pedir a alguém que aprove, passar a alguém uma tarefa a fazer ou avisar alguém. Uma rejeição encerra a execução e ignora as etapas seguintes. Editar um processo nunca altera uma execução já em andamento.",
  "p.processes.definitions": "Processos",
  "p.processes.none_defined": "Ainda não há processos definidos.",
  "p.processes.version": "versão {v}",
  "p.processes.new_process": "Definir um processo",
  "p.processes.name": "Nome",
  "p.processes.step_n": "Etapa {n}",
  "p.processes.kind": "O que ela faz",
  "p.processes.person": "Pessoa (person:… ou worker:…)",
  "p.processes.step_title": "O que ela diz",
  "p.processes.add_step": "Adicionar uma etapa",
  "p.processes.remove_step": "Remover a etapa {n}",
  "p.processes.create": "Definir o processo",
  "p.processes.defined": "{name} definido.",
  "p.processes.start_heading": "Iniciar uma execução de {name}",
  "p.processes.subject_kind": "Sobre o que é",
  "p.processes.subject_pid": "Seu identificador",
  "p.processes.start": "Iniciar a execução",
  "p.processes.started": "Execução de {name} iniciada.",
  "p.processes.runs": "Suas execuções",
  "p.processes.none_runs": "Você não iniciou nenhuma execução.",
  "p.processes.run_of": "{name} em {kind} {pid}",
  "p.processes.status": "Situação",
  "p.processes.show_run": "Mostrar {name}",
  "p.processes.hide_run": "Ocultar {name}",
  "p.processes.steps": "Etapas",
  "p.processes.log": "O que aconteceu",
  "p.processes.complete_step": "Marcar a etapa {n} como concluída",
  "p.processes.cancel": "Cancelar a execução",
  "p.processes.resume": "Verificar a aprovação de novo",
  "p.processes.done": "Concluído.",
  "p.processes.could_not": "Não funcionou: {reason}",
  "p.processes.could_not_load": "Não foi possível carregar os processos",
  "l.tolerance.green": "dentro da tolerância",
  "l.tolerance.amber": "perto do limite",
  "l.tolerance.red": "acima da tolerância",
  "l.tolerance.unknown": "não mensurável",
  "l.tolerance.not_set": "nenhum limite definido",
  "l.tolerance.time": "Prazo",
  "l.tolerance.schedule": "Cronograma em relação à linha de base",
  "l.tolerance.cost": "Custo",
  "l.tolerance.risk": "Risco",
  "l.tolerance.resources": "Recursos",
  "l.tolerance.day_one": "{n} dia",
  "l.tolerance.day_other": "{n} dias",
  "l.tolerance.score": "pontuação {n}",
  "l.tolerance.points": "{n} pontos acima de 100%",
  "l.tolerance.field_time": "Limite (dias corridos de atraso)",
  "l.tolerance.field_schedule": "Limite (dias corridos após a linha de base)",
  "l.tolerance.field_cost": "Limite (porcentagem acima do orçamento aprovado)",
  "l.tolerance.field_risk":
    "Limite (maior pontuação de risco aceitável, de 1 a 25)",
  "l.tolerance.field_resources": "Limite (pontos percentuais de superalocação)",
  "l.tolerance.no_limit": "Nenhum limite está definido aqui nem acima.",
  "l.tolerance.could_not_measure": "Não foi possível medir.",
  "l.tolerance.zero_tolerance":
    "Tolerância zero: qualquer desvio está acima do limite.",
  "l.tolerance.used": "{percent}% do limite usado",
  "l.tolerance.own": "Definido neste plano",
  "l.tolerance.inherited": "Herdado de um plano superior",
  "p.issue_settings.title": "Tipos, situações e papéis de problemas",
  "p.issue_settings.title_ppm": "Tipos, situações e papéis de problemas — PPM",
  "p.issue_settings.intro":
    "O vocabulário de problemas da própria organização. Somente um administrador pode alterá-lo. Sem nada configurado, os problemas usam os tipos e as situações internos.",
  "p.issue_settings.types_heading": "Tipos de problema",
  "p.issue_settings.types_intro":
    "Um tipo desativado não pode mais ser escolhido. Os problemas que o têm continuam com ele.",
  "p.issue_settings.key": "Chave",
  "p.issue_settings.label": "Nome",
  "p.issue_settings.state": "Estado",
  "p.issue_settings.in_use": "Em uso",
  "p.issue_settings.retired": "Desativado",
  "p.issue_settings.retire": "Desativar",
  "p.issue_settings.restore": "Restaurar",
  "p.issue_settings.actions": "Ações",
  "p.issue_settings.add_type": "Adicionar tipo",
  "p.issue_settings.roles_heading": "Papéis",
  "p.issue_settings.roles_intro":
    "Os papéis que esta organização reconhece e o que cada um permite em um plano com membros por papel. Um papel só pode restringir o que o login de alguém já permite. Um fluxo de trabalho só pode citar um papel listado, para que uma transição não seja restringida a um papel com grafia errada.",
  "p.issue_settings.description": "Descrição",
  "p.issue_settings.add_role": "Adicionar papel",
  "p.issue_settings.workflows_heading": "Situações e transições",
  "p.issue_settings.workflows_intro":
    "Um fluxo de trabalho define as situações de um tipo de problema, quem pode fazer cada transição e quais campos cada situação exige ou bloqueia. O fluxo de um tipo tem precedência sobre o de todos os tipos. Os problemas em uma situação removida continuam nela.",
  "p.issue_settings.none_workflows":
    "Nenhum fluxo de trabalho de problemas está registrado, portanto os problemas usam as situações internas.",
  "p.issue_settings.wf_name": "Nome do fluxo de trabalho",
  "p.issue_settings.wf_type": "Aplica-se a",
  "p.issue_settings.wf_all_types": "Todos os tipos sem fluxo próprio",
  "p.issue_settings.wf_plan": "Um plano",
  "p.issue_settings.statuses": "Situações",
  "p.issue_settings.category": "Significado",
  "p.issue_settings.initial": "Começa aqui",
  "p.issue_settings.terminal": "Exige uma resolução",
  "p.issue_settings.add_status": "Adicionar situação",
  "p.issue_settings.moves": "Transições permitidas",
  "p.issue_settings.moves_intro":
    "Sem transições listadas, qualquer transição é permitida. Depois que uma é listada, só as listadas são permitidas.",
  "p.issue_settings.from": "De",
  "p.issue_settings.to": "Para",
  "p.issue_settings.move_roles":
    "Papéis (separados por vírgula; em branco para qualquer pessoa)",
  "p.issue_settings.add_move": "Adicionar transição",
  "p.issue_settings.remove": "Remover",
  "p.issue_settings.register": "Registrar fluxo de trabalho",
  "p.issue_settings.withdraw": "Retirar",
  "p.issue_settings.saved": "Salvo.",
  "p.issue_settings.could_not_load":
    "Não foi possível carregar as configurações de problemas.",
  "p.issue_settings.could_not_save":
    "Não foi possível salvar. Somente um administrador pode alterar estas configurações, e cada chave deve ser nova e bem formada.",
  "p.issue_settings.cat_todo": "Não iniciado",
  "p.issue_settings.cat_active": "Em andamento",
  "p.issue_settings.cat_waiting": "Em espera",
  "p.issue_settings.cat_done": "Concluído",
  "p.issue_settings.translations":
    "Traduções (idioma=nome, separadas por ponto e vírgula)",
  "p.issue_settings.bad_translations":
    "Escreva cada tradução como idioma=nome, por exemplo cy=Risg.",
  "p.issue_settings.save": "Salvar",
  "p.issue_settings.permissions": "Permissões",
  "p.issue_settings.perm_issue_add": "Registrar problemas",
  "p.issue_settings.perm_issue_edit": "Editar problemas",
  "p.issue_settings.perm_issue_move": "Mover problemas",
  "p.issue_settings.perm_issue_delete": "Remover problemas",
  "p.issue_settings.edit": "Editar",
  "p.issue_settings.save_workflow": "Salvar fluxo de trabalho",
  "p.issue_settings.cancel": "Cancelar",
  "p.issue_settings.editing": "Alterando {name}",
  "p.issue_settings.rule_field": "Campo",
  "p.issue_settings.rule_kind": "Regra",
  "p.issue_settings.rule_required": "Obrigatório",
  "p.issue_settings.rule_read_only": "Somente leitura",
  "p.issue_settings.rule_roles":
    "Papéis (separados por vírgula; em branco para todos)",
  "p.issue_settings.add_rule": "Adicionar regra de campo",
  "p.issue_settings.field_title": "Título",
  "p.issue_settings.field_description": "Descrição",
  "p.issue_settings.field_severity": "Gravidade",
  "p.issue_settings.field_priority": "Prioridade",
  "p.issue_settings.field_owner_ref": "Responsável",
  "p.issue_settings.field_due_date": "Data de vencimento",
  "p.issue_settings.field_resolution": "Resolução",
  "p.plan_settings.title": "Configurações de problemas deste plano",
  "p.plan_settings.title_ppm": "Configurações de problemas deste plano — PPM",
  "p.plan_settings.intro":
    "Quem tem qual papel aqui, quais tipos de problema este plano oferece e seus próprios fluxos de trabalho. Um administrador faz essas alterações.",
  "p.plan_settings.back": "Voltar aos problemas deste plano",
  "p.plan_settings.all_settings": "Configurações de toda a organização",
  "p.plan_settings.members_heading": "Pessoas e papéis",
  "p.plan_settings.members_intro":
    "Quando um plano tem pelo menos uma pessoa listada aqui, registrar, editar, mover e remover problemas nele exige um papel que o permita. Um papel não pode dar a ninguém mais do que o login da pessoa permite, e não abre um plano privado.",
  "p.plan_settings.enforced":
    "As ações sobre problemas neste plano exigem um papel.",
  "p.plan_settings.not_enforced":
    "Qualquer pessoa com permissão de escrita pode agir sobre os problemas aqui.",
  "p.plan_settings.person": "ID de usuário da pessoa",
  "p.plan_settings.role": "Papel",
  "p.plan_settings.choose_role": "Escolher um papel",
  "p.plan_settings.add_member": "Adicionar pessoa",
  "p.plan_settings.types_heading": "Tipos de problema neste plano",
  "p.plan_settings.types_intro":
    "Marque os tipos que este plano oferece. Com nenhum marcado, ele oferece todos os tipos. Um problema mantém um tipo que você deixar de fora.",
  "p.plan_settings.save_types": "Salvar tipos",
  "p.plan_settings.workflows_intro":
    "Um fluxo de trabalho aqui se aplica somente a este plano e tem precedência sobre os da organização. O fluxo de um tipo tem precedência sobre o de todos os tipos. Os problemas em uma situação removida continuam nela.",
  "p.plan_issues.settings_link": "Configurações de problemas deste plano",
  "p.plan_issues.problem_fields_required": "Esta situação exige: {fields}.",
  "p.tolerances.tolerances": "Tolerâncias",
  "p.tolerances.tolerances_ppm": "Tolerâncias — PPM",
  "p.tolerances.plan_link": "Tolerâncias",
  "p.tolerances.intro":
    "Quanto cada projeto pode se desviar em prazo, custo, risco e recursos antes de precisar ser encaminhado a um nível superior, e quanto de cada limite foi usado. Dentro do limite, mas perto dele, é âmbar; além dele, é vermelho; um limite que não pode ser medido aparece como não mensurável, nunca como verde.",
  "p.tolerances.board_intro":
    "Todos os planos que você pode ler e que têm uma tolerância, do pior ao melhor.",
  "p.tolerances.none": "Nenhum plano tem tolerância ainda.",
  "p.tolerances.none_match": "Nenhum plano corresponde.",
  "p.tolerances.show": "Mostrar",
  "p.tolerances.all_statuses": "Todas as cores",
  "p.tolerances.plan": "Plano",
  "p.tolerances.overall": "Geral",
  "p.tolerances.truncated":
    "Somente os primeiros {n} planos foram medidos, então esta lista pode estar incompleta.",
  "p.tolerances.loading": "Carregando…",
  "p.tolerances.could_not_load": "Não foi possível carregar as tolerâncias",
  "p.tolerances.back": "← Plano",
  "p.tolerances.board_link": "Todas as tolerâncias",
  "p.tolerances.limit": "Limite",
  "p.tolerances.measured": "Medido",
  "p.tolerances.how": "Como é medido",
  "p.tolerances.amber_from": "Âmbar a partir de (porcentagem do limite)",
  "p.tolerances.note": "Nota",
  "p.tolerances.save": "Definir o limite",
  "p.tolerances.remove": "Remover o limite próprio deste plano",
  "p.tolerances.saved": "Limite de {dimension} salvo.",
  "p.tolerances.removed": "Limite de {dimension} removido.",
  "p.tolerances.could_not": "Não funcionou: {reason}",
  "p.tolerances.limit_problem": "Informe um limite como número.",
  "l.escalation.open": "aberto",
  "l.escalation.acknowledged": "reconhecido",
  "l.escalation.resolved": "resolvido",
  "l.escalation.withdrawn": "retirado",
  "l.escalation.reason_tolerance": "Uma tolerância foi excedida",
  "l.escalation.reason_risk": "Um risco",
  "l.escalation.reason_issue": "Um problema",
  "l.escalation.reason_dependency": "Uma dependência",
  "l.escalation.reason_decision": "É necessária uma decisão",
  "l.escalation.reason_resource": "Recursos",
  "l.escalation.reason_other": "Outro motivo",
  "l.escalation.outcome_decided": "Decidido",
  "l.escalation.outcome_guidance_given": "Orientação dada",
  "l.escalation.outcome_rejected": "Rejeitado",
  "l.escalation.event_raised": "Registrado",
  "l.escalation.event_acknowledged": "Reconhecido",
  "l.escalation.event_level_raised": "Levado a um nível acima",
  "l.escalation.event_resolved": "Resolvido",
  "l.escalation.event_withdrawn": "Retirado",
  "l.escalation.title_problem": "Dê a ele um título de 1 a 200 caracteres.",
  "l.escalation.dimension_problem": "Diga a qual tolerância ele se refere.",
  "l.escalation.level_problem": "O nível é um número inteiro de 1 a 9.",
  "l.escalation.owner_problem":
    "A pessoa deve ser uma referência person: ou worker:.",
  "l.escalation.level_to": "Nível {level}: {to}",
  "l.escalation.level": "Nível {level}",
  "l.escalation.open_for": "Aberto há {n} dias",
  "l.escalation.took": "Levou {n} dias",
  "l.escalation.overdue": "em atraso",
  "l.escalation.automatic": "Registrado automaticamente",
  "p.escalations.escalations": "Escalonamentos",
  "p.escalations.escalations_ppm": "Escalonamentos — PPM",
  "p.escalations.plan_link": "Escalonamentos",
  "p.task.task": "Tarefa",
  "p.task.task_ppm": "Tarefa — PPM",
  "p.task.back": "Voltar à estrutura analítica do trabalho",
  "p.task.could_not_load": "Não foi possível carregar a tarefa.",
  "p.task.loading": "Carregando…",
  "p.task.status": "Situação",
  "p.task.due": "Vencimento",
  "p.task.assignee": "Atribuída a",
  "p.task.description": "Descrição",
  "p.task.s_todo": "A fazer",
  "p.task.s_in_progress": "Em andamento",
  "p.task.s_in_review": "Em revisão",
  "p.task.s_done": "Concluída",
  "p.task.s_blocked": "Bloqueada",
  "p.task.links_intro":
    "Onde esta tarefa existe em outras ferramentas, como o problema do Jira ou do Linear em que ela é acompanhada. O endereço é armazenado e exibido; nunca é acessado e nada é sincronizado.",
  "p.task.back_board": "Voltar ao quadro",
  "p.task.could_not_load_links":
    "Não foi possível carregar os vínculos da tarefa com outras tarefas.",
  "p.task.could_not_load_tasks":
    "Não foi possível carregar as tarefas do plano para vincular.",
  "p.task.edit_heading": "Editar a tarefa",
  "p.task.edit_note":
    "A situação muda no quadro. O sprint, o componente e a tarefa-mãe são mantidos como estão.",
  "p.task.f_title": "Título",
  "p.task.f_assignee":
    "Atribuída a (uma referência person: ou worker:; deixe vazio para ninguém)",
  "p.task.f_points":
    "Pontos de história (um número inteiro de 0 a 100; deixe vazio para nenhum)",
  "p.task.f_due": "Data de vencimento (deixe vazio para nenhuma)",
  "p.task.save": "Salvar a tarefa",
  "p.task.saved": "A tarefa foi salva.",
  "p.task.problem_title_required": "Dê um título à tarefa.",
  "p.task.problem_title_too_long": "O título é longo demais.",
  "p.task.problem_assignee":
    "Informe o responsável como uma referência person: ou worker:, ou deixe vazio.",
  "p.task.problem_points_whole": "Os pontos de história são um número inteiro.",
  "p.task.problem_points_range": "Os pontos de história devem ser de 0 a 100.",
  "p.task.problem_due": "A data de vencimento não é uma data.",
  "p.task.relations_heading": "Vínculos com outras tarefas",
  "p.task.relations_intro":
    "‘Bloqueia’ impede que a outra tarefa seja concluída enquanto esta estiver aberta. ‘Vem antes de’ exige que a outra tarefa vença depois desta. Quando uma tarefa é concluída, uma tarefa que seja duplicada dela também é concluída. ‘Relacionada a’ e ‘Copiada de’ são apenas informativos.",
  "p.task.no_relations": "Esta tarefa não tem vínculos com outras tarefas.",
  "p.task.rel_related": "Relacionada a",
  "p.task.rel_blocks": "Bloqueia",
  "p.task.rel_blocked_by": "Bloqueada por",
  "p.task.rel_precedes": "Vem antes de",
  "p.task.rel_follows": "Vem depois de",
  "p.task.rel_duplicates": "Duplicada de",
  "p.task.rel_duplicated_by": "Duplicada por",
  "p.task.rel_copied_from": "Copiada de",
  "p.task.rel_copied_to": "Copiada para",
  "p.task.other_state": "(agora {status})",
  "p.task.other_state_due": "(agora {status}, vence em {date})",
  "p.task.remove_link": "Remover",
  "p.task.remove_link_named": "Remover o vínculo: {link} {title}",
  "p.task.hidden_links_one":
    "Mais {n} vínculo leva a uma tarefa de um plano que você não pode ver.",
  "p.task.hidden_links_other":
    "Mais {n} vínculos levam a tarefas de planos que você não pode ver.",
  "p.task.add_link_heading": "Vincular a outra tarefa",
  "p.task.f_kind": "Esta tarefa",
  "p.task.f_other": "Outra tarefa",
  "p.task.choose_task": "Escolher uma tarefa",
  "p.task.add_link": "Vincular as tarefas",
  "p.task.link_added": "O vínculo foi adicionado.",
  "p.task.link_removed": "O vínculo foi removido.",
  "p.task.problem_choose_task": "Escolha a outra tarefa.",
  "p.task.problem_self": "Uma tarefa não pode ser vinculada a si mesma.",
  "p.task.problem_exists": "Esta tarefa já tem esse vínculo.",
  "p.task.problem_cycle":
    "A outra tarefa já tem esse vínculo com esta; as duas ficariam esperando uma pela outra.",
  "p.task.problem_precedes_dates":
    "‘Vem antes de’ exige que a outra tarefa vença depois desta: altere antes uma data de vencimento.",
  "p.task.refused_forbidden":
    "Você não tem permissão para alterar as tarefas deste plano.",
  "p.task.refused_gone":
    "Esta tarefa não existe mais ou você não pode vê-la; ela foi lida de novo.",
  "p.task.refused_link_gone":
    "Esse vínculo já tinha sido removido; os vínculos foram lidos de novo.",
  "p.task.refused_link_exists":
    "Esse vínculo já existe; os vínculos foram lidos de novo.",
  "p.task.refused_edit_invalid":
    "O serviço não aceitou a alteração: verifique os campos. Uma nova data de vencimento também deve manter em ordem todos os vínculos ‘Vem antes de’.",
  "p.task.refused_link_invalid":
    "O serviço não aceitou o vínculo: a outra tarefa pode ter sido removida, um vínculo ‘Bloqueia’ ou ‘Vem antes de’ não pode formar um ciclo, e ‘Vem antes de’ exige que a outra tarefa vença depois desta.",
  "p.me.notifications_link": "Minhas notificações",
  "p.me.notifications_note":
    "— sua caixa de entrada, sobre o que você é avisado e para onde vão as cópias.",
  "p.notifications.title_ppm": "Minhas notificações — PPM",
  "p.notifications.heading": "Minhas notificações",
  "p.notifications.back": "← Meu trabalho",
  "p.notifications.intro":
    "Tudo aqui é somente seu: ninguém mais pode ler sua caixa de entrada, suas escolhas ou seus destinos.",
  "p.notifications.loading": "Carregando…",
  "p.notifications.inbox_heading": "Caixa de entrada",
  "p.notifications.unread_only": "Mostrar somente não lidas",
  "p.notifications.unread_count_one": "{n} notificação não lida.",
  "p.notifications.unread_count_other": "{n} notificações não lidas.",
  "p.notifications.inbox_tz": "Os dias são contados no seu fuso horário, {tz}.",
  "p.notifications.inbox_empty": "Nada na sua caixa de entrada.",
  "p.notifications.inbox_unread_empty": "Nada não lido.",
  "p.notifications.status_unread": "Não lida",
  "p.notifications.status_read": "Lida",
  "p.notifications.digest_count_one": "resumo diário de {n} notificação",
  "p.notifications.digest_count_other": "resumo diário de {n} notificações",
  "p.notifications.open_subject": "Abrir o plano",
  "p.notifications.mark_read": "Marcar como lida",
  "p.notifications.mark_read_named": "Marcar como lida: {message}",
  "p.notifications.marked_read": "Marcada como lida.",
  "p.notifications.could_not_load_inbox":
    "Não foi possível carregar sua caixa de entrada.",
  "p.notifications.could_not_mark":
    "Não foi possível marcar a notificação como lida.",
  "p.notifications.prefs_heading": "Sobre o que você é avisado",
  "p.notifications.prefs_intro":
    "Ative ou desative cada tipo de notificação para a caixa de entrada e para cada tipo de destino. Uma cópia vai para um webhook, um chat ou um e-mail somente quando você tiver adicionado, abaixo, um destino desse tipo. Um resumo diário reúne as notificações de um dia sobre um mesmo assunto em uma linha da caixa de entrada; nada é retido nem descartado.",
  "p.notifications.pref_daily": "Resumo diário na caixa de entrada",
  "p.notifications.pref_saved": "Sua escolha foi salva.",
  "p.notifications.pref_reset_done":
    "De volta ao padrão: ativado, à medida que chegam.",
  "p.notifications.reset": "Voltar ao padrão",
  "p.notifications.reset_named": "Voltar ao padrão: {kind}",
  "p.notifications.could_not_load_prefs":
    "Não foi possível carregar suas escolhas.",
  "p.notifications.could_not_save_pref": "Não foi possível salvar a escolha.",
  "p.notifications.sinks_heading": "Para onde vão as cópias",
  "p.notifications.sinks_intro":
    "Um destino recebe uma cópia de cada uma das suas notificações, contendo apenas o texto da própria notificação. O endereço é um segredo: depois de salvo, só é exibido de forma abreviada. Um destino lento ou com defeito nunca atrasa a caixa de entrada; um envio que falha é tentado de novo mais quatro vezes ao longo de cerca de duas horas e meia e depois é abandonado.",
  "p.notifications.no_sinks":
    "Você não tem destinos; suas notificações vão somente para a caixa de entrada.",
  "p.notifications.chat_named": "Chat {name}",
  "p.notifications.sink_sending": "Enviando",
  "p.notifications.sink_paused": "Em pausa",
  "p.notifications.show_deliveries": "Entregas recentes",
  "p.notifications.deliveries_named": "Entregas recentes: {destination}",
  "p.notifications.remove_sink": "Remover",
  "p.notifications.remove_sink_named": "Remover: {destination}",
  "p.notifications.sink_removed": "O destino foi removido.",
  "p.notifications.deliveries_heading": "Entregas recentes para {destination}",
  "p.notifications.deliveries_note":
    "O motivo da falha de um envio não é exibido aqui, pois pode citar o endereço secreto.",
  "p.notifications.no_deliveries": "Nada foi enviado a este destino ainda.",
  "p.notifications.attempts_one": "{n} tentativa",
  "p.notifications.attempts_other": "{n} tentativas",
  "p.notifications.delivered_at": "enviada em {date}",
  "p.notifications.next_attempt": "próxima tentativa em {date}",
  "p.notifications.close_deliveries": "Fechar as entregas",
  "p.notifications.add_heading": "Adicionar um destino",
  "p.notifications.f_kind": "Tipo",
  "p.notifications.f_destination_email": "Endereço de e-mail",
  "p.notifications.f_destination_url":
    "Endereço web (https, para um host público)",
  "p.notifications.f_chat_id": "Chat do Telegram (seu número, ou @ e seu nome)",
  "p.notifications.add_sink": "Adicionar o destino",
  "p.notifications.sink_added": "O destino foi adicionado: {destination}.",
  "p.notifications.could_not_load_sinks":
    "Não foi possível carregar seus destinos.",
  "p.notifications.could_not_load_deliveries":
    "Não foi possível carregar as entregas.",
  "p.notifications.could_not_save_sink":
    "Não foi possível adicionar o destino.",
  "p.notifications.could_not_remove_sink":
    "Não foi possível remover o destino.",
  "p.notifications.problem_too_many":
    "Você já tem dez destinos, o máximo permitido: remova um antes.",
  "p.notifications.problem_destination_required":
    "Informe o endereço para o envio.",
  "p.notifications.problem_email_invalid": "Isso não é um endereço de e-mail.",
  "p.notifications.problem_url_invalid": "Isso não é um endereço web.",
  "p.notifications.problem_url_not_https":
    "O endereço web deve começar com https://.",
  "p.notifications.problem_url_credentials":
    "O endereço web não deve conter nome de usuário nem senha.",
  "p.notifications.problem_url_private":
    "O endereço web deve ser de um host público, não local, privado ou interno.",
  "p.notifications.problem_url_too_long": "O endereço web é longo demais.",
  "p.notifications.problem_chat_id_required":
    "Um destino do Telegram precisa do chat em que publica.",
  "p.notifications.refused_forbidden":
    "Você não tem permissão para alterar estas configurações de notificação.",
  "p.notifications.refused_note_gone":
    "Essa notificação não existe mais ou não é sua; a caixa de entrada foi lida de novo.",
  "p.notifications.refused_sink_gone":
    "Esse destino já tinha sido removido; seus destinos foram lidos de novo.",
  "p.notifications.refused_not_addressable":
    "Sua identidade de login não é uma a que se possam endereçar notificações, portanto não há nada a mostrar aqui.",
  "p.notifications.refused_pref_invalid":
    "O serviço não aceitou essa escolha; suas escolhas foram lidas de novo.",
  "p.notifications.refused_sink_invalid":
    "O serviço não aceitou o destino: deve ser um endereço de e-mail ou um endereço web https para um host público; um destino do Telegram precisa de seu chat; e pode haver no máximo dez.",
  "l.enum.notify_kind.approval_decided":
    "Uma aprovação que você solicitou foi decidida",
  "l.enum.notify_kind.approval_requested": "Pedem que você aprove algo",
  "l.enum.notify_kind.auto_watch":
    "Acompanhar automaticamente aquilo em que você trabalha",
  "l.enum.notify_kind.automation": "Uma regra avisa você",
  "l.enum.notify_kind.comment": "Comentários",
  "l.enum.notify_kind.escalation": "Escalonamentos",
  "l.enum.notify_kind.exceptions_digest": "O resumo semanal de exceções",
  "l.enum.notify_kind.reschedule_proposed": "Uma nova data é proposta",
  "l.enum.notify_kind.review_invited": "Você é convidado a revisar",
  "l.enum.notify_kind.review_submitted": "Uma revisão foi enviada",
  "l.enum.notify_kind.scheduled": "Um lembrete agendado",
  "l.enum.notify_kind.status_update_due":
    "Uma atualização de situação está para vencer",
  "l.enum.notify_kind.task_assigned": "Uma tarefa foi atribuída a você",
  "l.enum.notify_kind.watch_update": "Algo que você acompanha mudou",
  "l.enum.notify_kind.process_todo": "Uma etapa de processo cabe a você",
  "l.enum.notify_kind.process_notice":
    "Uma etapa de processo sobre a qual você é avisado",
  "l.enum.notify_kind.break_glass": "Um plano privado foi aberto em emergência",
  "l.enum.notify_channel.in_app": "Na caixa de entrada",
  "l.enum.notify_channel.webhook": "Por webhook assinado",
  "l.enum.notify_channel.chat": "No chat",
  "l.enum.notify_channel.email": "Por e-mail",
  "l.enum.sink_kind.email": "E-mail",
  "l.enum.sink_kind.webhook": "Webhook assinado",
  "l.enum.delivery_status.pending": "Aguardando envio",
  "l.enum.delivery_status.sent": "Enviada",
  "l.enum.delivery_status.failed": "Abandonada",
  "l.enum.delivery_status.skipped":
    "Não enviada: o destino ou a notificação não existe mais",
  "l.link.kind_docs": "Documentos",
  "l.link.kind_repo": "Repositório de código",
  "l.link.kind_ticket": "Chamado",
  "l.link.kind_design": "Projeto de design",
  "l.link.kind_chat": "Conversa",
  "l.link.kind_tracker": "Rastreador de trabalho",
  "l.link.kind_other": "Outro",
  "l.link.system_other": "Outra ferramenta",
  "l.fw.method_waterfall": "Cascata",
  "l.fw.method_hybrid": "Híbrido",
  "l.fw.method_other": "Outro",
  "l.fw.model_sponsor": "O patrocinador decide",
  "l.fw.model_steering_committee": "Comitê diretor",
  "l.fw.model_product_owner": "Dono do produto",
  "l.fw.model_delegated": "Delegado",
  "l.fw.model_consensus": "Consenso",
  "l.fw.model_other": "Outro",
  "p.frameworks.wow": "Formas de trabalho",
  "p.frameworks.wow_ppm": "Formas de trabalho — PPM",
  "p.frameworks.gov": "Estrutura de governança",
  "p.frameworks.gov_ppm": "Estrutura de governança — PPM",
  "p.frameworks.intro_wow":
    "Como a equipe trabalha: método, duração da iteração, limite de trabalho em andamento. Um plano segue uma, compartilhada ou própria; sem nenhuma, herda a do plano mais próximo acima.",
  "p.frameworks.intro_gov":
    "Como o trabalho é governado: quem decide, com que frequência é revisado e o caminho de escalonamento que os escalonamentos seguem. Um plano segue uma, compartilhada ou própria; sem nenhuma, herda a do plano mais próximo acima.",
  "p.frameworks.back": "Voltar ao plano",
  "p.frameworks.loading": "Carregando…",
  "p.frameworks.could_not_load": "Não foi possível carregar isto.",
  "p.frameworks.could_not": "Não foi possível fazer isso: {reason}",
  "p.frameworks.in_force": "Em vigor para este plano",
  "p.frameworks.source_own": "definida neste plano",
  "p.frameworks.source_inherited": "herdada de um plano superior",
  "p.frameworks.source_none": "nenhuma",
  "p.frameworks.from_plan": "ver esse plano",
  "p.frameworks.scope_plan": "escrita para um plano",
  "p.frameworks.scope_org": "compartilhada pela organização",
  "p.frameworks.none": "Nada está em vigor.",
  "p.frameworks.edit": "Editar",
  "p.frameworks.unfollow": "Deixar de seguir (voltar a herdar)",
  "p.frameworks.follow_heading": "Seguir outra",
  "p.frameworks.no_others": "Não há outra compartilhada para seguir.",
  "p.frameworks.choose": "Compartilhada ou existente",
  "p.frameworks.follow": "Seguir esta",
  "p.frameworks.write_new": "Escrever uma nova para este plano",
  "p.frameworks.followed": "Agora seguindo.",
  "p.frameworks.unfollowed": "Deixou de seguir; o plano volta a herdar.",
  "p.frameworks.saved": "Salvo.",
  "p.frameworks.new_heading": "Uma nova para este plano",
  "p.frameworks.edit_heading": "Editar esta",
  "p.frameworks.f_name": "Nome",
  "p.frameworks.f_description": "Descrição",
  "p.frameworks.save": "Salvar",
  "p.frameworks.cancel": "Cancelar",
  "p.frameworks.methodology": "Método",
  "p.frameworks.cadence_days": "Duração da iteração (dias)",
  "p.frameworks.wip_limit": "Limite de trabalho em andamento",
  "p.frameworks.ready": "Definição de pronto para começar",
  "p.frameworks.done": "Definição de concluído",
  "p.frameworks.ceremonies": "Cerimônias",
  "p.frameworks.decision_model": "Modelo de decisão",
  "p.frameworks.review_cadence": "Revisar a cada (dias)",
  "p.frameworks.reporting_cadence": "Reportar ao nível superior a cada (dias)",
  "p.frameworks.path": "Caminho de escalonamento",
  "p.frameworks.rights": "Direitos de decisão",
  "p.frameworks.forums": "Fóruns",
  "p.frameworks.days": "{n} dias",
  "p.frameworks.path_level": "Nível",
  "p.frameworks.path_body": "Quem a ouve",
  "p.frameworks.path_trigger": "O que a leva até lá",
  "p.frameworks.path_add": "Adicionar um nível",
  "p.frameworks.path_remove": "Remover o nível {n}",
  "p.frameworks.name_problem": "Dê a ela um nome de 1 a 120 caracteres.",
  "p.frameworks.number_problem":
    "Os dias são números inteiros de 1 a 366 e o limite, de 1 a 1000.",
  "p.frameworks.path_problem":
    "Cada nível de escalonamento é um número inteiro diferente de 1 a 9, com quem o ouve e o que o leva até lá.",
  "p.frameworks.confirm_shared_one":
    "Isto é compartilhado: salvar altera também para {n} outro plano.",
  "p.frameworks.confirm_shared_other":
    "Isto é compartilhado: salvar altera também para {n} outros planos.",
  "p.frameworks.confirm_problem_one":
    "Marque a caixa para confirmar a alteração para {n} outro plano.",
  "p.frameworks.confirm_problem_other":
    "Marque a caixa para confirmar a alteração para {n} outros planos.",
  "p.frameworks.deleted": "{name} excluído.",
  "p.frameworks.delete": "Excluir {name} (ninguém a segue)",
  "p.frameworks.text_problem": "Uma definição tem no máximo 10000 caracteres.",
  "p.frameworks.practice_problem":
    "Cada cerimônia ou fórum precisa de um nome e de com que frequência ocorre, e há no máximo 20.",
  "p.frameworks.rights_problem":
    "Cada direito de decisão precisa da decisão e de quem a toma, e há no máximo 20.",
  "p.frameworks.practice_name": "Nome",
  "p.frameworks.practice_cadence": "Com que frequência",
  "p.frameworks.practice_purpose": "Para que serve",
  "p.frameworks.row_remove": "Remover a linha {n}",
  "p.frameworks.ceremony_add": "Adicionar uma cerimônia",
  "p.frameworks.forum_add": "Adicionar um fórum",
  "p.frameworks.right_decision": "Decisão",
  "p.frameworks.right_decides": "Quem decide",
  "p.frameworks.right_consulted": "Quem é consultado primeiro",
  "p.frameworks.right_add": "Adicionar um direito de decisão",
  "p.links.links": "Vínculos",
  "p.links.links_ppm": "Vínculos — PPM",
  "p.links.intro":
    "Onde ficam os documentos e o trabalho deste plano, incluindo seu quadro do Jira, Trello, Asana, Linear ou Monday. O endereço é armazenado e exibido; nunca é acessado.",
  "p.links.back": "Voltar ao plano",
  "p.links.loading": "Carregando…",
  "p.links.could_not_load": "Não foi possível carregar os vínculos.",
  "p.links.could_not": "Não foi possível fazer isso: {reason}",
  "p.links.none": "Ainda não há vínculos.",
  "p.links.opens_new": "abre em uma nova aba",
  "p.links.remove": "Remover {label}",
  "p.links.added": "{label} adicionado.",
  "p.links.removed": "{label} removido.",
  "p.links.add_heading": "Adicionar um vínculo",
  "p.links.add": "Adicionar o vínculo",
  "p.links.f_label": "Rótulo",
  "p.links.f_url": "Endereço (http ou https)",
  "p.links.f_kind": "Tipo",
  "p.links.f_system": "Ferramenta externa",
  "p.links.f_id": "Seu identificador lá",
  "p.links.url_problem":
    "O endereço deve começar com http:// ou https:// e indicar um host.",
  "p.links.label_problem": "Dê a ele um rótulo.",
  "p.links.system_problem":
    "Um vínculo de rastreador deve indicar sua ferramenta.",
  "p.links.system_kind_problem":
    "Somente um vínculo de rastreador ou de chamado indica uma ferramenta.",
  "p.links.id_problem":
    "Um identificador precisa de uma ferramenta e não tem espaços.",
  "p.links.domain_problem": "Os vínculos dessa ferramenta ficam em {domains}.",
  "p.escalations.intro":
    "Encaminhe algo a um nível superior quando o projeto não puder ou não devesse resolvê-lo sozinho. Ele é registrado em um nível do caminho de escalonamento da estrutura de governança do projeto, reconhecido, levado a um nível acima se ninguém responder, e termina resolvido ou retirado. A pessoa indicada como responsável e a pessoa que o registrou são avisadas em sua caixa de entrada (nenhum e-mail é enviado).",
  "p.escalations.register_intro":
    "Todos os escalonamentos dos planos que você pode ler.",
  "p.escalations.none": "Ainda não há escalonamentos.",
  "p.escalations.none_match": "Nenhum escalonamento corresponde.",
  "p.escalations.raise_heading": "Registrar um escalonamento",
  "p.escalations.f_title": "Título",
  "p.escalations.f_reason": "Por quê",
  "p.escalations.f_dimension": "Qual tolerância",
  "p.escalations.f_level": "Nível (em branco para o primeiro nível do caminho)",
  "p.escalations.f_to": "Escalar para (em branco para usar o caminho)",
  "p.escalations.f_owner": "Pessoa a quem se destina (person:… ou worker:…)",
  "p.escalations.f_respond": "Resposta desejada até",
  "p.escalations.f_description": "O quê e por quê",
  "p.escalations.raise": "Registrar",
  "p.escalations.raised": "{title} registrado.",
  "p.escalations.acknowledge": "Reconhecer",
  "p.escalations.take_up": "Levar a um nível acima",
  "p.escalations.take_up_note": "Por que está subindo",
  "p.escalations.resolve": "Resolver",
  "p.escalations.outcome": "Desfecho",
  "p.escalations.response": "Resposta",
  "p.escalations.withdraw": "Retirar",
  "p.escalations.done": "Concluído.",
  "p.escalations.could_not": "Não funcionou: {reason}",
  "p.error.status_401": "Você não fez login ou sua sessão terminou.",
  "p.error.status_403": "Você não tem permissão para fazer isso.",
  "p.error.status_404": "Isso não foi encontrado ou você não pode vê-lo.",
  "p.error.status_409":
    "Isso entra em conflito com o estado atual: outra pessoa alterou ou decidiu, ou já está concluído.",
  "p.error.status_412":
    "Isso mudou desde que você o carregou: recarregue e tente de novo.",
  "p.error.status_413": "Isso é grande demais.",
  "p.error.status_422": "Algo que você informou não é aceito.",
  "p.error.status_429":
    "Solicitações demais: aguarde um instante e tente de novo.",
  "p.bridge.open_workforce":
    "Abrir {name} no planejamento de força de trabalho",
  "p.bridge.open_contacts": "Abrir {name} nos contatos",
  "p.error.client": "A solicitação foi recusada.",
  "p.error.server": "O serviço teve um problema: tente de novo.",
  "p.escalations.note_problem": "Escreva pelo menos 10 caracteres.",
  "p.escalations.show": "Mostrar {title}",
  "p.escalations.hide": "Ocultar {title}",
  "p.escalations.history": "Histórico",
  "p.escalations.status": "Situação",
  "p.escalations.plan": "Plano",
  "p.escalations.level_col": "Nível",
  "p.escalations.age": "Idade",
  "p.escalations.respond_by": "Resposta desejada até",
  "p.escalations.filter": "Mostrar",
  "p.escalations.all": "Todos",
  "p.escalations.live": "Ainda abertos",
  "p.escalations.overdue_count": "{n} em atraso",
  "p.escalations.loading": "Carregando…",
  "p.escalations.could_not_load": "Não foi possível carregar os escalonamentos",
  "p.escalations.back": "← Plano",
  "p.escalations.register_link": "Todos os escalonamentos",
  "p.tolerances.needs_escalation":
    "Isto está acima da tolerância e ninguém o escalou.",
  "p.tolerances.raise_one": "Registrar um escalonamento",
  "p.tolerances.escalate_all": "Escalar todas as tolerâncias vermelhas",
  "p.tolerances.escalate_all_intro":
    "Toda dimensão que está acima da tolerância e ainda não foi escalada recebe um escalonamento no primeiro nível do caminho de escalonamento. Nada é escalado por conta própria.",
  "p.tolerances.escalated_one": "{n} escalonamento registrado.",
  "p.tolerances.escalated_other": "{n} escalonamentos registrados.",
  "p.tolerances.escalations_one": "{n} escalonamento aberto",
  "p.tolerances.escalations_other": "{n} escalonamentos abertos",
  "p.resources.rbs.by_role": "Por papel",
  "p.resources.rbs.show_tasks": "Mostrar as tarefas abertas de {role}",
  "p.resources.rbs.hide_tasks": "Ocultar as tarefas abertas de {role}",
  "p.resources.rbs.no_open_tasks": "nenhuma tarefa aberta",
  "p.resources.rbs.tasks_in_plans_you_cannot_read":
    "{count} em planos que você não pode ler não são mostradas.",
  "p.resources.rbs.only_the_first_500_tasks":
    "Somente as primeiras 500 tarefas foram lidas, então esta lista pode estar incompleta.",
  "p.resources.rbs.could_not_load_tasks":
    "Não foi possível carregar as tarefas: {reason}",
  "p.resources.rbs.by_skill": "Por habilidade",
  "p.resources.rbs.demand_by_skill": "Demanda por habilidade",
  "p.resources.rbs.demand_fte": "Demanda em FTE",
  "p.resources.rbs.no_roles_in_this_window": "Nenhum papel nesta janela.",
  "p.resources.rbs.no_skill_demand_in_this":
    "Nenhuma demanda por habilidade nesta janela.",
  "p.resources.rbs.not_known": "desconhecido",
  "p.resources.rbs.people": "Pessoas",
  "p.resources.rbs.resource_breakdown": "Estrutura analítica de recursos",
  "p.resources.rbs.resource_breakdown_ppm":
    "Estrutura analítica de recursos — PPM",
  "p.resources.rbs.role": "Papel",
  "p.resources.rbs.roles_with_people_demand_and":
    "Papéis com pessoas, demanda e oferta",
  "p.resources.rbs.skill": "Habilidade",
  "p.resources.rbs.supply_fte": "Oferta em FTE",
  "p.resources.rbs.you_can_see_headcounts_only":
    "Você vê somente contagens de pessoas, não nomes.",
  "p.resources.resources": "Recursos",
  "p.resources.resources_ppm": "Recursos — PPM",
  "p.resources.role": "Papel",
  "p.resources.roles_and_totals_only": "somente papéis e totais",
  "p.resources.roles_asked_for_with_no": "Papéis solicitados ainda sem pessoa",
  "p.resources.short_spare_balanced_supply_not":
    "▼ falta · ▲ sobra · ● equilibrado · ? oferta desconhecida. O sombreamento mostra apenas o tamanho da lacuna.",
  "p.resources.show": "Mostrar",
  "p.resources.skill": "Habilidade",
  "p.resources.skill_demand_only": "Habilidade (somente demanda)",
  "p.resources.supply_fte": "Oferta em FTE",
  "p.resources.to": "Até",
  "p.resources.unclassified": "sem classificação",
  "p.resources.week": "Semana",
  "p.resources.what_the_plans_ask_for":
    "O que os planos pedem em comparação com o que as pessoas de cada papel podem oferecer, em equivalentes de tempo integral. As reservas propostas ficam separadas e nunca são contadas.",
  "p.reviews.accept": "Aceitar",
  "p.reviews.accepted": "aceito",
  "p.reviews.actions": "Ações",
  "p.reviews.advance": "Avançar",
  "p.reviews.all": "todos",
  "p.reviews.collaborative_review": "Revisão colaborativa",
  "p.reviews.consensus": "Consenso",
  "p.reviews.consensus_for_this_subject": "Consenso sobre este assunto",
  "p.reviews.decline": "Recusar",
  "p.reviews.declined": "recusado",
  "p.reviews.delegate": "Delegar",
  "p.reviews.delegate_a_specific_item_to":
    "Delegue um item específico ao especialista certo. Somente um revisor que aceitou pode enviar um parecer, então um convite sem resposta nunca vira evidência.",
  "p.reviews.due": "prazo",
  "p.reviews.e_g_clinical_safety": "p. ex., segurança clínica",
  "p.reviews.expertise": "especialidade",
  "p.reviews.expertise_2": "Especialidade",
  "p.reviews.expired": "expirado",
  "p.reviews.external": "externo",
  "p.reviews.hold": "Pôr em espera",
  "p.reviews.idea": "ideia",
  "p.reviews.internal": "interno",
  "p.reviews.invited": "convidado",
  "p.reviews.no_review_invitations_yet": "Ainda não há convites de revisão.",
  "p.reviews.no_score": "sem pontuação",
  "p.reviews.person_uuid": "person:<uuid>",
  "p.reviews.plan": "plano",
  "p.reviews.proposal": "proposta",
  "p.reviews.reject": "Rejeitar",
  "p.reviews.reviewer": "revisor",
  "p.reviews.reviewer_2": "Revisor",
  "p.reviews.reviews_ppm": "Revisões — PPM",
  "p.reviews.scope": "escopo",
  "p.reviews.scope_2": "Escopo",
  "p.reviews.status": "situação",
  "p.reviews.status_2": "Situação",
  "p.reviews.subject": "assunto",
  "p.reviews.subject_2": "Assunto",
  "p.reviews.subject_pid": "pid do assunto",
  "p.reviews.submitted": "enviado",
  "p.reviews.submitted_outstanding_declined_exter":
    "{submitted} enviados · {outstanding} pendentes · {declined} recusados · {external_submitted} externos",
  "p.reviews.uuid": "UUID",
  "p.reviews.verdict": "Parecer",
  "p.reviews.withdraw": "Retirar",
  "p.risk.category": "Categoria",
  "p.risk.concentration": "Concentração",
  "p.risk.escalated": "Escalonado",
  "p.risk.exposure": "Exposição",
  "p.risk.exposure_25_of_estate": "— exposição {exposure} (≥ 25% do conjunto)",
  "p.risk.heatmap": "Mapa de calor",
  "p.risk.item": "Elemento",
  "p.risk.materialized": "Concretizado",
  "p.risk.max_estate_max_single_risk":
    "— máximo do conjunto {v}, máximo de um único risco {v2}",
  "p.risk.no_open_risks": "Nenhum risco aberto.",
  "p.risk.open_exposure": "Exposição aberta",
  "p.risk.open_risks_estate_open_exposure":
    "riscos abertos · exposição aberta do conjunto",
  "p.risk.overdue_reviews": "Revisões em atraso",
  "p.risk.portfolio": "Portfólio",
  "p.risk.posture_by_portfolio": "Postura por portfólio",
  "p.risk.ppm": "{t} — PPM",
  "p.risk.probability_impact": "probabilidade ↓ / impacto →",
  "p.risk.review_due": "(revisão prevista para {review_date})",
  "p.risk.risk": "Risco",
  "p.risk.top_risks": "Principais riscos",
  "p.risk.unassigned": "(não atribuído)",
  "p.scenarios.alignment": "Alinhamento",
  "p.scenarios.cap": "Teto",
  "p.scenarios.compare": "Comparar",
  "p.scenarios.currency": "Moeda",
  "p.scenarios.delta_b_a": "Diferença (b − a)",
  "p.scenarios.evaluation": "Avaliação",
  "p.scenarios.feasible": "Viável",
  "p.scenarios.no": "não",
  "p.scenarios.open_exposure": "Exposição aberta",
  "p.scenarios.pick": "escolher…",
  "p.scenarios.planned": "{currency} planejado",
  "p.scenarios.scenario": "Cenário",
  "p.scenarios.scenarios_ppm": "Cenários — PPM",
  "p.scenarios.status": "Situação",
  "p.scenarios.work_item_pids_space_comma":
    "pids de itens de trabalho (separados por espaço ou vírgula)",
  "p.scenarios.yes": "sim",
  "p.security.exposure": "Exposição",
  "p.security.item": "Elemento",
  "p.security.late_stage_items_with_no":
    "Itens em estágio avançado sem risco de segurança registrado",
  "p.security.no_security_risks_recorded":
    "Nenhum risco de segurança registrado.",
  "p.security.none_every_late_stage_item":
    "Nenhum — todo item em estágio avançado tem um risco de segurança registrado.",
  "p.security.open_exposure": "{note} · exposição aberta {open_exposure}",
  "p.security.owner": "Responsável",
  "p.security.ppm": "{t} — PPM",
  "p.security.risk": "Risco",
  "p.security.security_risk_register": "Registro de riscos de segurança",
  "p.security.status": "Situação",
  "p.signin.check_your_email_for_a":
    "Verifique seu e-mail para ver o link de login.",
  "p.signin.could_not_send_the_sign":
    "Não foi possível enviar o link de login. Tente de novo.",
  "p.signin.email": "E-mail",
  "p.signin.send_magic_link": "Enviar link mágico",
  "p.signin.sign_in": "Entrar",
  "p.technology.as_of": "em {as_of} · {convention}",
  "p.technology.as_of_edges": "em {as_of} · {edges} vínculos",
  "p.technology.calendar_days": "{median_lead_days} dias corridos",
  "p.technology.completed_before_timing_existed_unti":
    "· {undated_completions} concluídos antes de existir a medição de tempo (sem tempo)",
  "p.technology.cross_portfolio_edges": "Vínculos entre portfólios",
  "p.technology.delivery_flow": "Fluxo de entrega",
  "p.technology.dependency_risk": "Risco de dependência",
  "p.technology.dependents": "Dependentes",
  "p.technology.edges_importing_risk_from_a":
    "Vínculos que importam risco de uma predecessora vermelha",
  "p.technology.exposure": "Exposição",
  "p.technology.item": "Elemento",
  "p.technology.kind": "Tipo",
  "p.technology.median_lead": "Prazo de entrega mediano:",
  "p.technology.milestones_completed": "Marcos concluídos",
  "p.technology.month": "Mês",
  "p.technology.most_depended_on_single_point":
    "Mais dependidos (candidatos a ponto único de falha)",
  "p.technology.no_dependencies_recorded_yet":
    "Ainda não há dependências registradas.",
  "p.technology.no_tech_debt_risks_recorded":
    "Nenhum risco tech_debt registrado.",
  "p.technology.no_tech_tags_recorded_yet":
    "Ainda não há etiquetas `tech:*` registradas.",
  "p.technology.no_timed_completions": "nenhuma conclusão com tempo medido",
  "p.technology.no_timed_completions_in_the":
    "Nenhuma conclusão com tempo medido na janela.",
  "p.technology.none_all_edges_stay_within":
    "Nenhum — todos os vínculos ficam dentro de um único portfólio.",
  "p.technology.none_no_red_predecessors":
    "Nenhum — não há predecessoras vermelhas.",
  "p.technology.open_exposure": "{note} · exposição aberta {open_exposure}",
  "p.technology.over_timed_completions":
    "em {timed_completions} conclusões com tempo medido",
  "p.technology.owner": "Responsável",
  "p.technology.ppm": "{t} — PPM",
  "p.technology.predecessor": "Predecessora",
  "p.technology.predecessor_red": "Predecessora (vermelha)",
  "p.technology.ring_votes": "({ring_votes} votos de anel)",
  "p.technology.risk": "Risco",
  "p.technology.status": "Situação (",
  "p.technology.status_2": "Situação",
  "p.technology.successor": "Sucessora",
  "p.technology.successor_at_risk": "Sucessora em risco",
  "p.technology.technical_debt_register": "Registro de dívida técnica",
  "p.technology.technology_radar": "Radar de tecnologia",
  "p.verify.request_a_new_link": "Solicitar um novo link",
  "p.verify.sign_in_link": "Link de login",
  "p.milestones.milestone_by_due_date_one":
    "{length} marco, por data de vencimento",
  "p.milestones.milestone_by_due_date_other":
    "{length} marcos, por data de vencimento",
  "p.okrs.objective_least_progressed_first_one":
    "{length} objetivo, o de menor progresso primeiro",
  "p.okrs.objective_least_progressed_first_other":
    "{length} objetivos, os de menor progresso primeiro",
  "p.plans.pid.board.blocked_calendar_day_one":
    "bloqueada há {blocked_days} dia corrido",
  "p.plans.pid.board.blocked_calendar_day_other":
    "bloqueada há {blocked_days} dias corridos",
  "p.plans.pid.calendar.holiday_one": "{length} feriado: {join}",
  "p.plans.pid.calendar.holiday_other": "{length} feriados: {join}",
  "p.plans.pid.costs.for_across_one":
    "{money} por {hoursText} em {entries} lançamento.",
  "p.plans.pid.costs.for_across_other":
    "{money} por {hoursText} em {entries} lançamentos.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_one":
    "{plans_without_figures} plano abaixo ainda não tem números (sem linha de base ou nada a medir) e não é estimado.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_other":
    "{plans_without_figures} planos abaixo ainda não têm números (sem linha de base ou nada a medir) e não são estimados.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_one":
    "concluídos sem nunca retroceder ({rework_count} retrocesso). Leia isto junto com a vazão: vazão em alta com rendimento em queda é trabalho sendo devolvido a si mesmo.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_other":
    "concluídos sem nunca retroceder ({rework_count} retrocessos). Leia isto junto com a vazão: vazão em alta com rendimento em queda é trabalho sendo devolvido a si mesmo.",
  "p.plans.pid.flow.of_items_finish_within_this_one":
    "{percent} dos itens terminam dentro deste prazo, a partir de {sample} item concluído",
  "p.plans.pid.flow.of_items_finish_within_this_other":
    "{percent} dos itens terminam dentro deste prazo, a partir de {sample} itens concluídos",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_one":
    "{length} risco afeta tanto o cronograma quanto o custo: cada um ocorre uma vez e afeta ambos.",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_other":
    "{length} riscos afetam tanto o cronograma quanto o custo: cada um ocorre uma vez e afeta ambos.",
  "p.plans.pid.leveling.apply_move_one": "Aplicar {length} movimento",
  "p.plans.pid.leveling.apply_move_other": "Aplicar {length} movimentos",
  "p.plans.pid.leveling.business_day_one": "{delay_days} dia útil",
  "p.plans.pid.leveling.business_day_other": "{delay_days} dias úteis",
  "p.plans.pid.leveling.other_task_would_be_pushed_one":
    "{length} outra tarefa seria adiada em consequência",
  "p.plans.pid.leveling.other_task_would_be_pushed_other":
    "{length} outras tarefas seriam adiadas em consequência",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_one":
    "{hidden} lista de verificação sobre pessoas não é mostrada a você.",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_other":
    "{hidden} listas de verificação sobre pessoas não são mostradas a você.",
  "p.plans.pid.schedule_health.preset_thresholds_task_one":
    "· predefinição {preset} (limites: {source}) · {tasks} tarefa",
  "p.plans.pid.schedule_health.preset_thresholds_task_other":
    "· predefinição {preset} (limites: {source}) · {tasks} tarefas",
  "p.plans.pid.schedule_health.task_one": "{length} tarefa",
  "p.plans.pid.schedule_health.task_other": "{length} tarefas",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_one":
    "{title}: {probability_percent}% de chance de acrescentar {impact_days} dia útil",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_other":
    "{title}: {probability_percent}% de chance de acrescentar {impact_days} dias úteis",
  "p.plans.pid.wbs.move_up": "Mover {title} para cima",
  "p.plans.pid.wbs.move_down": "Mover {title} para baixo",
  "p.plans.pid.wbs.moved": "{title} movida.",
  "p.plans.pid.wbs.truncated":
    "Mostrando {shown} de {total} tarefas: este plano é grande demais para ser listado por inteiro, então a estrutura abaixo está incompleta.",
  "p.plans.pid.critical_path.too_large":
    "Este plano é grande demais para ser agendado por inteiro em uma única resposta.",
  "p.plans.pid.critical_path.show_critical_only":
    "Mostrar somente a parte crítica",
  "p.plans.pid.critical_path.critical_only_notice":
    "Mostrando somente as tarefas críticas e quase críticas; o restante do plano ficou de fora.",
  "p.plans.pid.earned_value.names_truncated":
    "Este plano é muito grande: os números cobrem todas as tarefas, mas alguns nomes de tarefas abaixo podem estar ausentes.",
  "p.plans.pid.wbs.could_not_move": "Não foi possível mover a tarefa: {reason}",
  "p.plans.pid.wbs.sub_task_one": "({children} subtarefa)",
  "p.plans.pid.wbs.sub_task_other": "({children} subtarefas)",
  "p.plans.pid.wbs.links": "Vínculos",
  "p.plans.pid.wbs.link_one": "· {links} vínculo",
  "p.plans.pid.wbs.link_other": "· {links} vínculos",
  "p.common.on": "ativado",
  "p.common.off": "desativado",
  "p.common.pre_gate": "pré-portão",
  "p.common.role_word": "papel",
  "p.common.person_word": "pessoa",
  "c.NetworkDiagram.task_fallback": "tarefa",
  "c.NetworkDiagram.task_summary":
    "{title}, de {early_start} a {early_finish}, {total_float} dias úteis de folga",
  "c.NetworkDiagram.task_summary_critical":
    "{title}, de {early_start} a {early_finish}, {total_float} dias úteis de folga, no caminho crítico",
  "c.NetworkDiagram.lag_one": ", defasagem de {lag_days} dia útil",
  "c.NetworkDiagram.lag_other": ", defasagem de {lag_days} dias úteis",
  "c.KpiCard.direction_higher": "maior",
  "c.KpiCard.direction_lower": "menor",
  "c.CalendarMonth.legend":
    "– não é dia útil   ✕ feriado   A ausência registrada",
  "c.CalendarMonth.legend_editable":
    " · selecione um dia útil para marcar ou limpar um feriado",
  "c.CostRiskView.run_summary":
    "semente {seed} · {iterations} iterações · {distribution}",
  "c.CostRiskView.dist_pert": "PERT",
  "c.CostRiskView.dist_triangular": "Distribuição triangular",
  "c.CostRiskView.risks_without_cost_one":
    "{count} risco do registro não tem campos de custo e não faz parte disto.",
  "c.CostRiskView.risks_without_cost_other":
    "{count} riscos do registro não têm campos de custo e não fazem parte disto.",
  "c.CumulativeFlow.hide_the_numbers": "Ocultar os números",
  "c.CumulativeFlow.show_the_numbers": "Mostrar os números",
  "c.CostRiskView.scope_all_one": "Todos os planos que você pode ler ({plans})",
  "c.CostRiskView.scope_all_other":
    "Todos os planos que você pode ler ({plans})",
  "c.CostRiskView.scope_this_plan": "Este plano",
  "c.MilestoneCard.reason": "Motivo",
  "c.MilestoneCard.reason_required_open":
    " (obrigatório: as tarefas de que ele depende ainda estão abertas)",
  "c.MilestoneCard.reason_optional": " (opcional)",
  "c.VarianceTable.matched_of_baselined":
    "{matched} de {baseline_tasks} tarefas na linha de base ainda estão no plano:",
  "c.VarianceTable.slipped_pulled_on":
    "{slipped} atrasadas, {pulled_in} adiantadas, {on_baseline} na linha de base{unmeasurable}.",
  "c.VarianceTable.cannot_be_measured":
    ", {unmeasurable} não podem ser medidas",
  "c.VarianceTable.added_removed":
    "{added} adicionadas e {removed} removidas desde então.",
  "c.VarianceTable.finish_later_one":
    "O término do plano está {days} dia útil depois do previsto na linha de base{note}.",
  "c.VarianceTable.finish_later_other":
    "O término do plano está {days} dias úteis depois do previsto na linha de base{note}.",
  "c.VarianceTable.finish_earlier_one":
    "O término do plano está {days} dia útil antes do previsto na linha de base{note}.",
  "c.VarianceTable.finish_earlier_other":
    "O término do plano está {days} dias úteis antes do previsto na linha de base{note}.",
  "c.VarianceTable.check_removed":
    " (verifique as tarefas removidas: excluir trabalho não é uma melhoria)",
  "p.plans.pid.critical_path.finishes": "Termina em {date}.",
  "p.plans.pid.critical_path.nothing_scheduled": "Nada está agendado ainda.",
  "p.plans.pid.critical_path.calendar_label": "Calendário: {calendar}.",
  "p.plans.pid.critical_path.calendar_plan": "o deste plano",
  "p.plans.pid.critical_path.calendar_default":
    "padrão (todos os dias são úteis)",
  "p.plans.pid.critical_path.more_not_shown": " (mais não mostrados)",
  "p.plans.pid.critical_path.pushed_later":
    "Suas predecessoras a empurram {days} dias úteis para depois do planejado.",
  "p.plans.pid.earned_value.manual_percent_banner":
    "Parte deste número se baseia em uma porcentagem informada por uma pessoa, e não em trabalho concluído: {tasks}.",
  "p.plans.pid.earned_value.a_task": "uma tarefa",
  "p.plans.pid.earned_value.overrun": " (estouro)",
  "p.plans.pid.earned_value.target_budget": "o orçamento",
  "p.plans.pid.earned_value.target_forecast": "a previsão",
  "p.plans.pid.earned_value.lights_use":
    "Os semáforos usam {which}: {values} (pontos-base).",
  "p.plans.pid.earned_value.thresholds_default": "os limites padrão",
  "p.plans.pid.earned_value.thresholds_deployment":
    "os limites desta implantação",
  "p.plans.pid.earned_value.rollup_money":
    "Orçamento {bac}; planejado {pv}; agregado {ev}; gasto {ac}{unknown}.",
  "p.plans.pid.earned_value.cost_unknown_for":
    " (custo desconhecido para {plans} planos)",
  "p.plans.pid.earned_value.cpi_value": "CPI {value}",
  "p.plans.pid.earned_value.cpi_not_stated":
    "não informado: o custo é desconhecido para alguns planos",
  "p.plans.pid.earned_value.weight_points": ", {weight} pontos",
  "p.plans.pid.critical_path.moved_line":
    "{title}: {moved}{finish}{start} dias úteis",
  "p.plans.pid.critical_path.moved_both": "início e término",
  "p.plans.pid.critical_path.moved_start": "início",
  "p.plans.pid.critical_path.moved_finish": "término",
  "p.plans.pid.critical_path.moved_finish_by": " término {days}",
  "p.plans.pid.critical_path.moved_start_by": " início {days}",
  "p.plans.pid.flow.overdue_one":
    "{count} item já passou do esperado. Um item envelhecido é a única coisa nesta página cujo desfecho você ainda pode mudar.",
  "p.plans.pid.flow.overdue_other":
    "{count} itens já passaram do esperado. Um item envelhecido é a única coisa nesta página cujo desfecho você ainda pode mudar.",
  "p.plans.pid.flow.classified_with": "Classificado com {map}:",
  "p.plans.pid.flow.map_override": "uma substituição da implantação",
  "p.plans.pid.flow.map_default": "o mapa padrão",
  "p.plans.pid.governance.proposed_not_counted":
    "proposto{requested} — não contado",
  "p.plans.pid.governance.requested": ", solicitado",
  "p.plans.pid.joint_risk.run_summary":
    "{currency} · semente {seed} · {iterations} iterações · {distribution} · calendário {calendar}",
  "p.plans.pid.joint_risk.dist_pert": "PERT",
  "p.plans.pid.joint_risk.dist_triangular": "Distribuição triangular",
  "p.plans.pid.leveling.applied_one":
    "Aplicado {ok} de {requested} movimento{refused}.",
  "p.plans.pid.leveling.applied_other":
    "Aplicados {ok} de {requested} movimentos{refused}.",
  "p.plans.pid.leveling.applied_refused": "; {refused} recusados",
  "p.plans.pid.leveling.proposal_summary":
    "Calendário: {calendar} · carga de {load}% por tarefa · tarefas críticas {critical} movem",
  "p.plans.pid.leveling.may": "podem",
  "p.plans.pid.leveling.may_not": "não podem",
  "p.plans.pid.onboarding.due": ", vence em {date}",
  "p.plans.pid.schedule_risk.risks_without_schedule_one":
    "{count} risco do registro não tem campos de cronograma e não faz parte disto.",
  "p.plans.pid.schedule_risk.risks_without_schedule_other":
    "{count} riscos do registro não têm campos de cronograma e não fazem parte disto.",
  "p.resources.from_date": "de {date}",
  "p.resources.to_date": "até {date}",
  "p.kpis.none": "Ainda não há KPIs.",
  "p.kpis.none_with_status": "Ainda não há KPIs com essa situação.",
  "p.milestones.none":
    "Ainda não há marcos. Adicione-os na página de governança de um plano.",
  "p.milestones.none_with_status":
    "Ainda não há marcos com essa situação. Adicione-os na página de governança de um plano.",
  "p.okrs.none": "Ainda não há objetivos.",
  "p.okrs.none_for_period": "Ainda não há objetivos para {period}.",
  "p.me.calendar.own_set": "Seu calendário próprio está definido.",
  "p.me.calendar.plan_applies":
    "Sem calendário pessoal: o calendário do plano se aplica a você.",
  "p.me.calendar.leave_counts":
    "As licenças registradas para você já contam como dias de folga.",
  "p.documents.pid.page_title": "{title} — PPM",
  "p.documents.pid.document_fallback": "Documento",
  "p.plans.pid.page_title": "{name} — Main X",
  "p.plans.pid.plan_fallback": "Plano",
  "p.plans.pid.custom_scheme": "Personalizado({name})",
  "p.plans.pid.calendar.own_calendar": "Este plano tem seu próprio calendário.",
  "p.plans.pid.calendar.no_calendar":
    "Nenhum calendário definido: todos os dias são úteis.",
  "p.plans.pid.calendar.each_person_can_set":
    "Cada pessoa também pode definir o seu (",
  "p.reviews.consensus_line":
    "pontuação média: {mean} · maioria: {majority} · {state}",
  "p.reviews.no_majority": "sem maioria",
  "p.reviews.everyone_answered": "todos responderam",
  "p.reviews.still_waiting": "ainda aguardando",
  "l.announcement_view.just_now": "agora mesmo",
  "l.announcement_view.min_ago": "há {minutes} min",
  "l.announcement_view.h_ago": "há {hours} h",
  "l.announcement_view.calendar_days_ago": "há {days} dias corridos",
  "l.announcement_view.scheduled": "agendado",
  "l.announcement_view.expired": "Expirado",
  "l.api_ppm.task": "tarefa {uid}",
  "l.api_ppm.the_project_calendar_comes_across":
    " O calendário do projeto é incluído.",
  "l.api_tba.calendar_days": "{toFixed} dias corridos",
  "l.api_tba.work_in_progress_is_growing":
    "O trabalho em andamento está crescendo",
  "l.api_tba.steady_state": "Estado estável",
  "l.api_tba.queue_draining": "Fila esvaziando",
  "l.api_tba.not_enough_data": "Dados insuficientes",
  "l.capacity_view.committed_against_a_capacity_as":
    "{committed}% comprometido para uma capacidade de apenas {capacity}%",
  "l.capacity_view.committed": "{committed}% comprometido",
  "l.capacity_view.to": " até {to}",
  "l.capacity_view.over_capacity_from_asked_available":
    "acima da capacidade a partir de {from}{v} ({demand}% solicitado, {capacity}% disponível)",
  "l.capacity_view.over_capacity": "acima da capacidade",
  "l.capacity_view.committing_it_would_put_them":
    "; comprometê-lo os deixaria acima da capacidade",
  "l.capacity_view.proposed_not_counted": "{percent}% proposto, não contado{v}",
  "l.capacity_view.asked_available":
    "{from}{v}: {demand}% solicitado, {capacity}% disponível",
  "l.ceo_view.no_portfolios_to_report_on":
    "Ainda não há portfólios para reportar.",
  "l.ceo_view.portfolios_exist_but_hold_no":
    "Há portfólios, mas ainda sem planos.",
  "l.ceo_view.of_plans_are_red_amber":
    "{red} de {n} planos estão vermelhos, {amber} âmbar, {green} verdes.",
  "c.CalendarMonth.not_a_business_day": "não é dia útil",
  "c.CalendarMonth.business_day": "dia útil",
  "c.KpiCard.that_did_not_work": "Não funcionou",
  "c.KpiCard.could_not_load_the_history":
    "Não foi possível carregar o histórico",
  "c.KpiCard.delete_the_kpi_its_readings":
    "Excluir o KPI “{name}”? Suas leituras são mantidas.",
  "c.LineRange.could_not_save": "Não foi possível salvar",
  "c.MilestoneCard.that_did_not_work": "Não funcionou",
  "c.MilestoneCard.could_not_load_the_plan":
    "Não foi possível carregar as tarefas do plano",
  "c.MilestoneCard.could_not_load_the_history":
    "Não foi possível carregar o histórico",
  "c.ObjectiveCard.that_did_not_work": "Não funcionou",
  "c.ObjectiveCard.delete_the_objective_and_its":
    "Excluir o objetivo “{title}” e seus resultados-chave?",
  "c.ObjectiveCard.delete_the_key_result":
    "Excluir o resultado-chave “{title}”?",
  "c.RaciMatrix.accountable_moved_from_to_a":
    "O responsável final passou de {shortRef} para {shortRef2}: uma tarefa tem somente um.",
  "c.RiskInputs.could_not_load_the_plan":
    "Não foi possível carregar as tarefas do plano",
  "c.RiskInputs.could_not_save": "Não foi possível salvar",
  "c.RiskInputs.chance": "{probability_percent}% de chance",
  "c.RiskInputs.adds_business_days_to_a":
    "acrescenta {schedule_impact_days} dias úteis a uma tarefa",
  "c.RiskInputs.adds": "acrescenta {toFixed} {cost_currency}",
  "c.RiskInputs.not_used_in_the_simulations": "Não usado nas simulações",
  "l.cpm_view.cannot_meet_its_constraint_or":
    "Não consegue cumprir sua restrição ou prazo: faltam {plural}",
  "l.cpm_view.on_the_critical_path_any":
    "No caminho crítico: qualquer atraso move o término",
  "l.cpm_view.of_float_near_critical": "{d} de folga (quase crítica)",
  "l.cpm_view.of_float": "{d} de folga",
  "l.cpm_view.baselined_to": ". Linha de base de {start} a {finish}",
  "l.cpm_view.to": "{v}: de {early_start} a {early_finish}. {floatLabel}{v2}",
  "l.cpm_view.has_no_duration_yet": "ainda não tem duração",
  "l.cpm_view.has_no_start_date_and":
    "não tem data de início e nada antes dela está agendado",
  "l.cpm_view.has_a_date_outside_the": "tem uma data fora do intervalo aceito",
  "l.cpm_view.slipped": "Atrasada",
  "l.cpm_view.finishes_later_than_baselined":
    "termina {d} depois do previsto na linha de base",
  "l.cpm_view.pulled_in": "Adiantada",
  "l.cpm_view.finishes_earlier_than_baselined":
    "termina {d} antes do previsto na linha de base",
  "l.cpm_view.on_baseline": "Na linha de base",
  "l.cpm_view.finishes_when_it_was_baselined":
    "termina quando estava previsto na linha de base",
  "l.cpm_view.added": "Adicionada",
  "l.cpm_view.not_in_the_baseline_so":
    "não está na linha de base, portanto não tem atraso",
  "l.cpm_view.removed": "Removida",
  "l.cpm_view.was_in_the_baseline_and":
    "estava na linha de base e não existe mais; suas datas da linha de base são mantidas",
  "l.cpm_view.cannot_say": "Não é possível dizer",
  "l.cpm_view.it_had_no_schedule_when":
    "não tinha cronograma quando a linha de base foi registrada",
  "l.cpm_view.it_has_no_schedule_now": "não tem cronograma agora",
  "l.document_view.version": "Versão {version} — {v}",
  "l.document_view.since": "{base} desde {effective_from}",
  "l.document_view.by_version": " pela versão {version}",
  "l.document_view.on": "{base} em {superseded_on}{v}",
  "l.document_view.nothing_in_force_yet": "Nada em vigor ainda",
  "l.document_view.version_in_force": "Versão {in_force_today} em vigor",
  "l.ev_view.nothing_was_planned_to_be":
    "nada estava planejado para ser feito ainda",
  "l.ev_view.no_cost_has_been_recorded": "nenhum custo foi registrado ainda",
  "l.ev_view.the_baseline_plans_no_value":
    "a linha de base não planeja nenhum valor",
  "l.ev_view.not_available": "não disponível",
  "l.ev_view.schedule_no_ratio": "Cronograma: sem índice, {absentText}.",
  "l.ev_view.ahead_of_schedule": "Adiantado no cronograma",
  "l.ev_view.behind_schedule": "Atrasado no cronograma",
  "l.ev_view.on_schedule": "No cronograma",
  "l.ev_view.spi": "{word}: SPI {ratioText}, {gap}.",
  "l.ev_view.cost_no_ratio": "Custo: sem índice, {absentText}.",
  "l.ev_view.under_budget": "Abaixo do orçamento",
  "l.ev_view.over_budget": "Acima do orçamento",
  "l.ev_view.on_budget": "Dentro do orçamento",
  "l.ev_view.earned_schedule_no_ratio":
    "Cronograma agregado: sem índice, {absentText}.",
  "l.ev_view.by_earned_schedule_the_work":
    "Pelo cronograma agregado, o trabalho está {word}: agregou em {days} o que o plano esperava após {days2} (SPI(t) {ratioText}).",
  "l.ev_view.finished_work_counts_for_its":
    "o trabalho concluído conta pelos seus pontos de história",
  "l.ev_view.only_finished_work_counts": "só o trabalho concluído conta",
  "l.ev_view.by_story_points": "por pontos de história",
  "l.ev_view.counted_by_a_person_entered":
    "{length} contadas por uma porcentagem informada por uma pessoa (manual)",
  "l.ev_view.with_no_points_carry_no": "{length} sem pontos não têm peso",
  "l.ev_view.no_actual_cost_is_recorded": "nenhum custo real está registrado",
  "l.ev_view.the_ratio_it_divides_by":
    "o índice pelo qual é dividido ainda não está definido",
  "l.ev_view.no_estimate_to_complete_has":
    "nenhuma estimativa para concluir foi informada",
  "l.ev_view.the_budget_is_already_spent": "o orçamento já foi gasto",
  "l.ev_view.no_cost_remains": "não resta nenhum custo",
  "l.ev_view.against_the_money_is_already":
    "Em relação a {target}: o dinheiro já foi gasto, então nenhuma eficiência consegue entregar o trabalho restante dentro dele.",
  "l.ev_view.against": "Em relação a {target}: {amountAbsentText}.",
  "l.ev_view.against_the_remaining_work_must":
    "Em relação a {target}: o trabalho restante deve ser feito com {ratioText} vez a eficiência do trabalho até agora (TCPI {ratioText2}), o que exige maior eficiência.",
  "l.ev_view.against_the_remaining_work_can":
    "Em relação a {target}: o trabalho restante pode ser feito com {ratioText} da eficiência de custo do trabalho até agora (TCPI {ratioText2}).",
  "l.ev_view.earned": "{date}: agregado {ev_minor}",
  "l.ev_view.spent": "{date}: gasto {ac_minor}",
  "l.import_view.would_create": "Criaria",
  "l.import_view.created": "Criou",
  "l.import_view.and": "{verb} {plural} e {upd} {tasks_updated}{tail}.",
  "l.import_view.preview_the_import": "Pré-visualizar a importação",
  "l.import_view.import": "Importar {plural}",
  "l.kpi_view.target_or_more_red_at":
    "Meta {target_value} ou mais; vermelho em {red_value} ou menos{u}",
  "l.kpi_view.target_or_less_red_at":
    "Meta {target_value} ou menos; vermelho em {red_value} ou mais{u}",
  "l.kpi_view.no_reading_within_twice_its":
    "Nenhuma leitura dentro do dobro da sua periodicidade {cadence}",
  "l.kpi_view.no_reading_yet": "nenhuma leitura ainda",
  "l.milestone_view.there_is_no_forecast": "Não há previsão.",
  "l.milestone_view.some_open_tasks_cannot_be":
    " (algumas tarefas abertas não podem ser posicionadas, então isto pode estar adiantado)",
  "l.milestone_view.forecast_to_finish_after_it":
    "Previsão de término em {forecast}, {businessDays} depois do vencimento ({due}){partial}.",
  "l.milestone_view.forecast_to_finish_on_the":
    "Previsão de término na data de vencimento, {due}{partial}.",
  "l.milestone_view.forecast_to_finish_before_it":
    "Previsão de término em {forecast}, {businessDays} antes do vencimento ({due}){partial}.",
  "l.milestone_view.of_them_later": ", {later_moves} delas para mais tarde",
  "l.milestone_view.back_on_its_original_date": "de volta à data original",
  "l.milestone_view.later_than_the_original":
    "{calendarDays} depois da data original {original_due}",
  "l.milestone_view.earlier_than_the_original":
    "{calendarDays} antes da data original {original_due}",
  "l.milestone_view.originally_due_now":
    "Vencimento original {original_due}; {times}{later}; agora {net}.",
  "l.milestone_view.on": " em {achieved_on}",
  "l.milestone_view.achieved_every_task_it_waits":
    "Alcançado{on}: todas as tarefas de que depende estão concluídas.",
  "l.milestone_view.reason": " Motivo: {done_reason}",
  "l.milestone_view.marked_done": "Marcado como concluído{on}.{why}",
  "l.milestone_view.due": "{name}: {toLowerCase}, vencimento {due}{state}",
  "l.my_work_view.this_is_already_late_for":
    "Isto já está atrasado para o plano ({days} de atraso).",
  "l.my_work_view.on_the_critical_path": "No caminho crítico",
  "l.my_work_view.of_float": "{days} de folga",
  "l.okr_view.not_measured": "Não medido",
  "l.okr_view.not_measurable": "Não mensurável ({reason}).",
  "l.okr_view.of": "{current} de {target}{u}",
  "l.okr_view.confidence_not_part_of_the":
    "Confiança {confidence}% (não faz parte da pontuação)",
  "l.onboarding_view.give_the_date_it_takes":
    "Informe a data em que entra em vigor.",
  "l.onboarding_view.give_the_person_as_person":
    "Informe a pessoa como person:… (ou worker:…).",
  "l.onboarding_view.say_what_access_in_words":
    "Diga qual acesso, em palavras.",
  "l.onboarding_view.choose_granted_or_revoked":
    "Escolha concedido ou revogado.",
  "l.onboarding_view.give_who_is_handing_over":
    "Informe quem está passando o trabalho, como person:…",
  "l.onboarding_view.give_who_takes_over_as":
    "Informe quem assume, como person:… (ou deixe vazio para liberar o trabalho).",
  "l.onboarding_view.hand_over_to_someone_else": "Passe para outra pessoa.",
  "l.onboarding_view.no_items": "Nenhum item",
  "l.onboarding_view.of_done": "{done} de {total} concluídos",
  "l.onboarding_view.skipped": "{skipped} ignorados",
  "l.onboarding_view.overdue": "{overdue} em atraso",
  "l.onboarding_view.skipped_2": "Ignorados",
  "l.onboarding_view.overdue_2": "Em atraso",
  "l.onboarding_view.nothing_is_left_open_this":
    "Nada ficou em aberto. Este plano está pronto para ser encerrado.",
  "l.onboarding_view.not_ready_to_close":
    "Não está pronto para encerrar: {join}.",
  "l.onboarding_view.this_person_holds_nothing_live":
    "Esta pessoa não tem nada ativo neste plano.",
  "l.raci_view.nobody_is_accountable": "Ninguém é o responsável final",
  "l.raci_view.nobody_is_responsible": "Ninguém é o executor",
  "l.raci_view.more_than_one_person_is":
    "Mais de uma pessoa é responsável final",
  "l.raci_view.has_no_role": "{task}: {who} não tem papel",
  "l.raci_view.and": " e ",
  "l.raci_view.is": "{task}: {who} é {join}",
  "l.rbs.a_booking_that_needs_several":
    "Uma reserva que exige várias habilidades conta por inteiro para cada uma, então as habilidades se sobrepõem e não somam. A oferta e as pessoas não são conhecidas por habilidade.",
  "l.rbs.named": "{length} nomeadas",
  "l.rbs.supply_not_known": "oferta desconhecida",
  "l.rbs.supply_fte": "oferta {supply_fte} FTE",
  "l.rbs.demand_fte": "{role}: {people}, demanda {demand_fte} FTE, {supply}",
  "l.scheduling_view.pass": "Aprovado",
  "l.scheduling_view.fail": "Reprovado",
  "l.scheduling_view.not_computable": "Não calculável",
  "l.scheduling_view.not_enough_data": "Dados insuficientes.",
  "l.scheduling_view.not_enough_data_2": "Dados insuficientes: {reason}",
  "l.scheduling_view.must_pass_a_business_day":
    "Deve passar: um atraso de {v} dias úteis tem de mover o término",
  "l.scheduling_view.at_least_business_days": "Pelo menos {limit} dias úteis",
  "l.scheduling_view.at_most": "no máximo",
  "l.scheduling_view.at_least": "pelo menos",
  "l.scheduling_view.no_figure": "Sem valor",
  "l.scheduling_view.finish_moved_of_business_days":
    "O término mudou em {count} de {of} dias úteis",
  "l.scheduling_view.of": "{toFixed} ({count} de {of})",
  "l.scheduling_view.of_2": "{value}% ({count} de {of} {of_what})",
  "l.scheduling_view.passed_failed_not_computable":
    "{passed} aprovadas, {failed} reprovadas, {not_computable} não calculáveis",
  "l.scheduling_view.not_resolved": "Não resolvido.",
  "l.scheduling_view.it_is_on_the_critical":
    " Está no caminho crítico, portanto o término do projeto se move.",
  "l.scheduling_view.starts_later_finishes_instead_of":
    "{pid} ({who}) começa {from_start} → {scheduled_start}, {days} depois; termina {to_finish} em vez de {from_finish}.{crit}",
  "l.scheduling_view.to": "{from} a {to}",
  "l.scheduling_view.is_over_capacity_on":
    "{who} está acima da capacidade em {span}.",
  "l.scheduling_view.is_asked_for_on_and":
    "{who} é solicitado(a) em {demand}% em {span} e pode dar {capacity}%.",
  "l.scheduling_view.no_project_finish_can_be":
    "Nenhum término de projeto pode ser calculado.",
  "l.scheduling_view.the_project_finish_stays":
    "O término do projeto continua {before}.",
  "l.scheduling_view.the_project_finish_moves_from":
    "O término do projeto passa de {before} para {after} ({days} depois).",
  "l.scheduling_view.no_demand_or_supply": "Sem demanda nem oferta",
  "l.scheduling_view.demand_fte_supply_not_known":
    "Demanda {demand_fte} FTE; oferta desconhecida",
  "l.scheduling_view.short_fte": "Faltam {abs} FTE",
  "l.scheduling_view.spare_fte": "Sobram {gap_fte} FTE",
  "l.scheduling_view.balanced": "Equilibrado",
  "l.scheduling_view.the_critical_path_finish":
    "o término pelo caminho crítico",
  "l.scheduling_view.of_the_simulated_outcomes_finish":
    "{pct}% dos desfechos simulados terminam em {date} ou antes.",
  "l.scheduling_view.no_variation": "Sem variação",
  "l.scheduling_view.no_estimate_using_the_fallback":
    "Sem estimativa: usando a faixa de reserva",
  "l.scheduling_view.milestone_no_spread": "Marco: sem dispersão",
  "l.scheduling_view.estimated_widened_to_include_the":
    "Estimado (ampliado para incluir o valor provável)",
  "l.scheduling_view.estimated": "Estimado",
  "l.scheduling_view.pert_favours_the_likely_value":
    "PERT (favorece o valor provável)",
  "l.scheduling_view.of_the_simulated_outcomes_cost":
    "{round}% dos desfechos simulados custam no máximo o orçamento ({budgetText}).",
  "l.scheduling_view.estimated_widened_to_include_the_2":
    "Estimado (ampliado para incluir o valor planejado)",
  "l.scheduling_view.stated_struck_in_of_runs":
    "declarado {round}%, ocorreu em {toFixed}% das execuções",
  "l.scheduling_view.within_budget_and_on_time":
    "Dentro do orçamento e no prazo",
  "l.scheduling_view.within_budget_but_late":
    "Dentro do orçamento, mas atrasado",
  "l.scheduling_view.on_time_but_over_budget":
    "No prazo, mas acima do orçamento",
  "l.scheduling_view.over_budget_and_late": "Acima do orçamento e atrasado",
  "l.scheduling_view.both_within_budget_and_on":
    "Dentro do orçamento e no prazo ao mesmo tempo: {pct} (só dentro do orçamento {pct2}; só no prazo {pct3}).",
  "l.scheduling_view.a_late_finish_costs_nothing":
    "Um término atrasado não custa nada a mais nesta execução (o custo de atraso por dia útil é 0).",
  "l.scheduling_view.each_business_day_late_adds":
    "Cada dia útil de atraso acrescenta {format}; terminar antes não devolve nada.",
  "l.simulation_inputs.the_probability_must_be_a":
    "A probabilidade deve ser um número inteiro de 0 a 100.",
  "l.simulation_inputs.name_the_task_and_the":
    "Indique a tarefa e os dias úteis que ela acrescenta juntos, ou deixe ambos vazios.",
  "l.simulation_inputs.the_business_days_a_risk":
    "Os dias úteis que um risco acrescenta devem ser um número inteiro de 1 a 365.",
  "l.simulation_inputs.give_the_cost_and_its":
    "Informe o custo e sua moeda juntos, ou deixe ambos vazios.",
  "l.simulation_inputs.the_cost_must_be_an":
    "O custo deve ser um valor acima de zero.",
  "l.simulation_inputs.the_currency_must_be_a":
    "A moeda deve ser um código de 3 letras, por exemplo EUR, JPY ou GBP.",
  "l.simulation_inputs.a_risk_that_changes_the":
    "Um risco que altera o cronograma ou o custo precisa de uma probabilidade.",
  "l.simulation_inputs.amounts_must_be_numbers_zero":
    "Os valores devem ser números, zero ou mais.",
  "l.simulation_inputs.the_low_amount_cannot_be":
    "O valor baixo não pode estar acima do valor planejado.",
  "l.simulation_inputs.the_high_amount_cannot_be":
    "O valor alto não pode estar abaixo do valor planejado.",
  "l.simulation_inputs.business_days_must_be_whole":
    "Os dias úteis devem ser números inteiros de 0 a 3650.",
  "l.simulation_inputs.give_the_task_a_duration":
    "Dê primeiro uma duração à tarefa: ela é o valor provável.",
  "l.simulation_inputs.optimistic_cannot_be_above_the":
    "O otimista não pode estar acima da duração provável ({likely}).",
  "l.simulation_inputs.pessimistic_cannot_be_below_the":
    "O pessimista não pode estar abaixo da duração provável ({likely}).",
  "l.simulation_inputs.no_range_the_simulation_uses":
    "Sem faixa: a simulação usa sua faixa de reserva",
  "l.simulation_inputs.to": "{fmt} a {fmt2}",
  "l.wbs.restricted_item": "Item restrito",
  "l.whatif_view.choose_a_task": "Escolha uma tarefa.",
  "l.whatif_view.give_the_new_start_date": "Informe a nova data de início.",
  "l.whatif_view.duration_must_be_a_whole":
    "A duração deve ser um número inteiro de dias úteis de 0 a 3650.",
  "l.whatif_view.choose_a_constraint": "Escolha uma restrição.",
  "l.whatif_view.a_constraint_needs_a_date":
    "Uma restrição precisa de uma data.",
  "l.whatif_view.a_hard_constraint_needs_a":
    "Uma restrição rígida precisa de um motivo.",
  "l.whatif_view.choose_both_tasks": "Escolha as duas tarefas.",
  "l.whatif_view.a_task_cannot_depend_on":
    "Uma tarefa não pode depender de si mesma.",
  "l.whatif_view.lag_must_be_a_whole":
    "A defasagem deve ser um número inteiro de dias úteis (negativo é uma antecipação).",
  "l.whatif_view.choose_a_link": "Escolha um vínculo.",
  "l.whatif_view.start_on": "Iniciar “{name}” em {start_date}",
  "l.whatif_view.make_a_milestone": "Tornar “{name}” um marco",
  "l.whatif_view.for": "{constraint_kind} {constraint_date} para “{name}”",
  "l.whatif_view.clear_the_constraint_on": "Limpar a restrição de “{name}”",
  "l.whatif_view.lag": ", defasagem {lag_days}",
  "l.whatif_view.follows": "“{name}” segue “{name2}” ({toUpperCase}{v})",
  "l.whatif_view.remove_the_link": "Remover o vínculo {slice}",
  "l.whatif_view.the_plan_has_no_finish":
    "O plano ainda não tem data de término.",
  "l.whatif_view.the_finish_stays": "O término continua {after}.",
  "l.whatif_view.the_finish_becomes": "O término passa a ser {after}.",
  "l.whatif_view.the_plan_no_longer_has":
    "O plano não tem mais data de término (era {before}).",
  "l.whatif_view.the_finish_moves_from_to":
    "O término passa de {before} para {after}.",
  "l.whatif_view.cost_impact_is_finance_data":
    "O impacto no custo é dado financeiro, por isso não é mostrado a você.",
  "l.whatif_view.no_task_with_work_has":
    "Nenhuma tarefa com trabalho tem taxa, então não há custo para comparar.",
  "l.work_view.work_must_be_a_number":
    "O trabalho deve ser um número de horas acima de zero.",
  "l.work_view.duration_must_be_a_whole":
    "A duração deve ser um número inteiro de dias úteis de 1 a 3650.",
  "l.work_view.units_must_be_a_whole":
    "As unidades devem ser uma porcentagem inteira de um dia, de 1 a 1000.",
  "l.work_view.give_the_units_too": "Informe também as unidades.",
  "l.work_view.business_days_at_is":
    "{duration_days} dias úteis a {units_percent}% é {hoursText}",
  "l.work_view.the_duration_was_worked_out": "A duração foi calculada.",
  "l.work_view.the_share_of_the_day": "A parcela do dia foi calculada.",
  "l.work_view.the_work_was_worked_out": "O trabalho foi calculado.",
  "l.work_view.rounded_the_three_do_not":
    " Arredondado: os três não coincidem ao minuto.",
  "l.work_view.capacity_not_checked_the_task":
    "Capacidade não verificada: a tarefa ainda não tem datas.",
  "l.work_view.within_the_person_s_capacity": "Dentro da capacidade da pessoa.",
  "l.work_view.give_1_to_40_whole":
    "Informe de 1 a 40 porcentagens inteiras, separadas por vírgulas.",
  "l.work_view.the_percentages_add_up_to":
    "As porcentagens somam {sum}, não 100.",
  "p.home.main_x_plans": "Main X · Planos",
  "p.announcements.could_not_load_the_announcements":
    "Não foi possível carregar os comunicados",
  "p.announcements.could_not_publish": "Não foi possível publicar",
  "p.announcements.could_not_retract": "Não foi possível retirar",
  "p.auditor.auditor_ppm": "Auditor(a) — PPM",
  "p.automations.could_not_load_automations":
    "Não foi possível carregar as automações",
  "p.automations.rule_saved_it_fires_from":
    "Regra salva — ela dispara a partir da próxima transição correspondente",
  "p.automations.could_not_save_the_rule": "Não foi possível salvar a regra",
  "p.automations.that_action_was_refused": "Essa ação foi recusada",
  "p.automations.capped_at_run_again_for":
    " (limitado a {cap} — execute de novo para o restante)",
  "p.automations.swept_fired_already_claimed":
    "Varredura: {fired} disparadas, {skipped_already_claimed} já reivindicadas{v}",
  "p.automations.the_sweep_failed": "A varredura falhou",
  "p.board.board_ppm": "Quadro — PPM",
  "p.calendar.calendar_ppm": "Calendário — PPM",
  "p.ceo.could_not_load": "Não foi possível carregar",
  "p.compliance.compliance_ppm": "Conformidade — PPM",
  "p.cost_rates.could_not_load_the_rate":
    "Não foi possível carregar as tabelas de taxas",
  "p.cost_rates.rate_table_added": "Tabela de taxas adicionada.",
  "p.cost_rates.could_not_add_the_table": "Não foi possível adicionar a tabela",
  "p.cost_rates.give_a_date_and_an":
    "Informe uma data e uma taxa horária de zero ou mais.",
  "p.cost_rates.rate_added": "Taxa adicionada.",
  "p.cost_rates.could_not_add_the_rate": "Não foi possível adicionar a taxa",
  "p.cost_risk.could_not_run_the_simulation":
    "Não foi possível executar a simulação",
  "p.documents.could_not_load_the_documents":
    "Não foi possível carregar os documentos",
  "p.documents.could_not_create_the_document":
    "Não foi possível criar o documento",
  "p.documents.pid.could_not_load_the_document":
    "Não foi possível carregar o documento",
  "p.documents.pid.that_did_not_work": "Não funcionou",
  "p.documents.pid.enter_the_approver_s_id":
    "Informe o ID do aprovador (um ID de pessoa, não o seu).",
  "p.documents.pid.sent_for_review": "Enviado para revisão.",
  "p.documents.pid.needs_changes": "Precisa de alterações",
  "p.documents.pid.approved": "Aprovado.",
  "p.documents.pid.sent_back_to_draft": "Devolvido ao rascunho.",
  "p.documents.pid.in_force_from_today": "Em vigor a partir de hoje.",
  "p.documents.pid.next_draft_started": "Próximo rascunho iniciado.",
  "p.documents.pid.could_not_look_that_up": "Não foi possível consultar isso",
  "p.engineering.engineering_ppm": "Engenharia — PPM",
  "p.executive.executive_ppm": "Executivo — PPM",
  "p.financials.financials_ppm": "Finanças — PPM",
  "p.gantt.gantt_main_x": "Gantt — Main X",
  "p.gantt.calendar_failed":
    "Não foi possível carregar o calendário de trabalho, então os dias não úteis não são sombreados.",
  "p.kpis.could_not_load_the_kpis": "Não foi possível carregar os KPIs",
  "p.kpis.could_not_define_the_kpi": "Não foi possível definir o KPI",
  "p.lifecycle.could_not_load_the_funnel": "Não foi possível carregar o funil",
  "p.me.could_not_load_your_tasks": "Não foi possível carregar suas tarefas",
  "p.me.calendar.could_not_load_your_calendar":
    "Não foi possível carregar seu calendário",
  "p.me.calendar.could_not_save": "Não foi possível salvar",
  "p.me.calendar.could_not_remove": "Não foi possível remover",
  "p.milestones.could_not_load_the_milestones":
    "Não foi possível carregar os marcos",
  "p.objectives.create_failed": "falha ao criar",
  "p.objectives.alignment_failed": "falha no alinhamento",
  "p.okrs.could_not_load_the_okrs": "Não foi possível carregar os OKRs",
  "p.okrs.could_not_create_the_objective": "Não foi possível criar o objetivo",
  "p.plans.plans_main_x": "Planos — Main X",
  "p.plans.pid.main_x": "{v} — Main X",
  "p.plans.pid.board.pt": " · {points}pt",
  "p.plans.pid.board.board_ppm": "Quadro — PPM",
  "p.plans.pid.calendar.could_not_load_the_calendar":
    "Não foi possível carregar o calendário",
  "p.plans.pid.calendar.could_not_save": "Não foi possível salvar",
  "p.plans.pid.cost_risk.could_not_run_the_simulation":
    "Não foi possível executar a simulação",
  "p.plans.pid.costs.could_not_load_the_costs":
    "Não foi possível carregar os custos",
  "p.plans.pid.costs.choose_a_task": "Escolha uma tarefa.",
  "p.plans.pid.costs.the_amount_must_be_above":
    "O valor deve estar acima de zero.",
  "p.plans.pid.costs.the_currency_must_be_a":
    "A moeda deve ser um código de 3 letras, por exemplo EUR, JPY ou GBP.",
  "p.money.currency_hint": "por exemplo EUR, JPY ou GBP",
  "p.money.amount_or_currency":
    "Informe o valor como número e a moeda como um código de 3 letras, por exemplo EUR, JPY ou GBP.",
  "p.plans.pid.costs.expense_added": "Despesa adicionada.",
  "p.plans.pid.costs.could_not_add_the_expense":
    "Não foi possível adicionar a despesa",
  "p.plans.pid.costs.could_not_record": "Não foi possível registrar",
  "p.plans.pid.critical_path.could_not_load_the_baselines":
    "Não foi possível carregar as linhas de base",
  "p.plans.pid.critical_path.could_not_take_the_baseline":
    "Não foi possível registrar a linha de base",
  "p.plans.pid.critical_path.could_not_load_the_schedule":
    "Não foi possível carregar o cronograma",
  "p.plans.pid.critical_path.could_not_save_the_estimate":
    "Não foi possível salvar a estimativa",
  "p.plans.pid.critical_path.could_not_save": "Não foi possível salvar",
  "p.plans.pid.earned_value.could_not_load_earned_value":
    "Não foi possível carregar o valor agregado",
  "p.plans.pid.earned_value.could_not_save": "Não foi possível salvar",
  "p.plans.pid.earned_value.could_not_record": "Não foi possível registrar",
  "p.plans.pid.edit.edit_case_main_x": "Editar caso — Main X",
  "p.plans.pid.flow.could_not_load_flow_data":
    "Não foi possível carregar os dados de fluxo.",
  "p.plans.pid.handover.handover_is_about_people_so":
    "A passagem de trabalho diz respeito a pessoas, por isso exige a permissão de gerenciar pessoas.",
  "p.plans.pid.handover.could_not_preview": "Não foi possível pré-visualizar",
  "p.plans.pid.handover.what_this_person_holds_changed":
    "O que esta pessoa tem mudou desde a pré-visualização, então nada foi alterado. Pré-visualize de novo e depois confirme.",
  "p.plans.pid.handover.could_not_commit": "Não foi possível confirmar",
  "p.plans.pid.handover.discarded_nothing_was_changed":
    "Descartado. Nada foi alterado.",
  "p.plans.pid.import.choose_the_file_first": "Escolha primeiro o arquivo.",
  "p.plans.pid.import.the_import_did_not_work": "A importação não funcionou",
  "p.plans.pid.joint_risk.could_not_run_the_simulation":
    "Não foi possível executar a simulação",
  "p.plans.pid.leveling.could_not_propose_a_leveling":
    "Não foi possível propor um nivelamento",
  "p.plans.pid.leveling.could_not_apply_the_moves":
    "Não foi possível aplicar os movimentos",
  "p.plans.pid.onboarding.could_not_load": "Não foi possível carregar",
  "p.plans.pid.onboarding.could_not_open_the_checklist":
    "Não foi possível abrir a lista de verificação",
  "p.plans.pid.onboarding.checklist_started": "Lista de verificação iniciada.",
  "p.plans.pid.onboarding.could_not_start": "Não foi possível iniciar",
  "p.plans.pid.onboarding.could_not_save": "Não foi possível salvar",
  "p.plans.pid.onboarding.checklist_completed":
    "Lista de verificação concluída.",
  "p.plans.pid.onboarding.could_not_complete": "Não foi possível concluir",
  "p.plans.pid.onboarding.say_why_you_are_cancelling":
    "Diga por que você está cancelando.",
  "p.plans.pid.onboarding.checklist_cancelled":
    "Lista de verificação cancelada.",
  "p.plans.pid.onboarding.could_not_cancel": "Não foi possível cancelar",
  "p.plans.pid.onboarding.could_not_record": "Não foi possível registrar",
  "p.plans.pid.raci.could_not_load_who_does":
    "Não foi possível carregar quem faz o quê",
  "p.plans.pid.raci.could_not_save": "Não foi possível salvar",
  "p.plans.pid.raci.use_a_person_or_worker":
    "Use uma referência person: ou worker: com um UUID.",
  "p.plans.pid.schedule_health.could_not_run_the_schedule":
    "Não foi possível executar as verificações de saúde do cronograma",
  "p.plans.pid.schedule_risk.could_not_run_the_simulation":
    "Não foi possível executar a simulação",
  "p.plans.pid.wbs.could_not_load_the_tasks":
    "Não foi possível carregar as tarefas",
  "p.plans.pid.wbs.link_counts_failed":
    "Não foi possível carregar quantos vínculos cada tarefa tem, então nenhuma contagem é mostrada.",
  "p.plans.pid.what_if.could_not_load_the_plan":
    "Não foi possível carregar o plano",
  "p.plans.pid.what_if.at_most_edits_at_a":
    "No máximo {MAX_EDITS} edições por vez.",
  "p.plans.pid.what_if.edit_staged_preview_to_see":
    "Edição preparada. Pré-visualize para ver o que ela faz.",
  "p.plans.pid.what_if.discarded_nothing_was_changed":
    "Descartado. Nada foi alterado.",
  "p.plans.pid.what_if.could_not_preview": "Não foi possível pré-visualizar",
  "p.plans.pid.what_if.the_plan_changed_since_this":
    "O plano mudou desde esta pré-visualização, então nada foi aplicado. Pré-visualize de novo e depois confirme.",
  "p.plans.pid.what_if.could_not_commit": "Não foi possível confirmar",
  "p.plans.pid.what_if.to": "{early_start} a {early_finish}",
  "p.plans.pid.what_if.not_scheduled": "não agendada",
  "p.plans.pid.work.could_not_load_the_plan":
    "Não foi possível carregar o plano",
  "p.plans.pid.work.could_not_load_the_work":
    "Não foi possível carregar o perfil de trabalho",
  "p.plans.pid.work.could_not_save": "Não foi possível salvar",
  "p.plans.pid.work.contour_saved": "Perfil salvo.",
  "p.plans.merge.merge_plans_ppm": "Unificar planos — PPM",
  "p.plans.new.new_case_main_x": "Novo caso — Main X",
  "p.prioritization.could_not_load_the_queue":
    "Não foi possível carregar a fila",
  "p.prioritization.could_not_load_the_score":
    "Não foi possível carregar a pontuação",
  "p.proposals.duplicate_check_failed": "falha na verificação de duplicidade",
  "p.regulator.regulator_ppm": "Regulador — PPM",
  "p.reports.save_failed": "falha ao salvar",
  "p.reports.run_failed": "falha ao executar",
  "p.resources.could_not_load_the_resource":
    "Não foi possível carregar o plano de recursos",
  "p.resources.rbs.could_not_load_resources":
    "Não foi possível carregar os recursos",
  "p.reviews.could_not_load_reviews": "Não foi possível carregar as revisões",
  "p.reviews.invitation_sent": "Convite enviado",
  "p.reviews.could_not_delegate": "Não foi possível delegar",
  "p.reviews.score_0_100_leave_blank":
    "Pontuação de 0 a 100 (deixe em branco para nenhuma)",
  "p.reviews.score_must_be_between_0": "A pontuação deve estar entre 0 e 100",
  "p.reviews.that_action_was_refused": "Essa ação foi recusada",
  "p.reviews.could_not_load_consensus": "Não foi possível carregar o consenso",
  "p.risk.risk_ppm": "Risco — PPM",
  "p.scenarios.evaluate_failed": "falha na avaliação",
  "p.security.security_ppm": "Segurança — PPM",
  "p.technology.technology_ppm": "Tecnologia — PPM",
  "p.tour.tour_main_x_plans": "Visita guiada · Planos do Main X",
  "p.verify.this_sign_in_link_is": "Este link de login está sem o token.",
  "p.verify.we_could_not_reach_the":
    "Não foi possível alcançar o serviço de login. Tente de novo em instantes.",
  "p.verify.this_sign_in_link_is_2":
    "Este link de login é inválido ou expirou.",
  "l.ceo_view.gate_review": "Revisão de portão",
  "l.ceo_view.scenario_committed": "Cenário confirmado",
  "l.ceo_view.proposal": "Proposta",
  "l.ceo_view.plans_merged": "Planos unificados",
  "c.CumulativeFlow.in_review": "Em revisão",
  "c.CumulativeFlow.in_progress": "Em andamento",
  "c.CumulativeFlow.todo": "A fazer",
  "l.document_view.draft": "Rascunho",
  "l.document_view.in_review": "Em revisão",
  "l.document_view.approved_not_yet_in_force": "Aprovado, ainda não em vigor",
  "l.document_view.replaced": "Substituído",
  "l.document_view.send_for_review": "Enviar para revisão",
  "l.document_view.approve": "Aprovar",
  "l.document_view.bring_into_force_today": "Pôr em vigor hoje",
  "l.document_view.start_the_next_draft": "Iniciar o próximo rascunho",
  "l.document_view.waiting_for_the_approver": "Aguardando o aprovador",
  "l.document_view.approved": "Aprovado",
  "l.document_view.change_this_document": "Alterar este documento",
  "l.document_view.someone_else_approves_it_you":
    "Outra pessoa o aprova: você não pode aprovar o seu próprio documento.",
  "l.document_view.only_the_person_asked_can":
    "Somente a pessoa solicitada pode decidir.",
  "l.document_view.it_comes_into_force_today":
    "Ele entra em vigor hoje e substitui a versão atualmente em vigor.",
  "l.document_view.the_next_draft_starts_as":
    "O próximo rascunho começa como uma cópia do texto mais recente.",
  "l.ev_view.on_track": "no rumo",
  "l.ev_view.off_plan": "fora do plano",
  "l.ev_view.not_yet_known": "ainda não conhecido",
  "l.glossary.schedule_performance_index_work_done":
    "Índice de desempenho de prazo: trabalho feito em comparação com o trabalho planejado até agora. Acima de 1,00 está adiantado em relação ao plano.",
  "l.glossary.cost_performance_index_value_of":
    "Índice de desempenho de custo: valor do trabalho feito em comparação com o que ele custou. Acima de 1,00 está abaixo do orçamento.",
  "l.glossary.schedule_index_by_time_how":
    "Índice de prazo por tempo: quanto do plano já avançou no tempo, o que continua fiel mesmo quando um projeto termina atrasado.",
  "l.glossary.earned_value_the_budget_for":
    "Valor agregado: o orçamento do trabalho efetivamente concluído.",
  "l.glossary.planned_value_the_budget_for":
    "Valor planejado: o orçamento do trabalho que deveria estar concluído até agora.",
  "l.glossary.actual_cost_what_has_really":
    "Custo real: o que realmente foi gasto.",
  "l.glossary.estimate_at_completion_the_expected":
    "Estimativa no término: o custo final esperado.",
  "l.glossary.estimate_to_complete_what_is":
    "Estimativa para concluir: o que ainda se espera gastar.",
  "l.glossary.variance_at_completion_the_expected":
    "Variação no término: o custo final esperado em relação ao orçamento.",
  "l.glossary.red_amber_or_green_a":
    "Vermelho, âmbar ou verde: uma luz de situação.",
  "l.glossary.responsible_accountable_consulted_in":
    "Responsável, Aprovador, Consultado, Informado: quem faz o trabalho, quem o aprova, quem é consultado e quem é informado.",
  "l.glossary.a_way_of_weighting_the":
    "Uma forma de dar à estimativa provável mais peso do que aos melhores e piores casos.",
  "l.glossary.key_performance_indicator_a_measure":
    "Indicador-chave de desempenho: uma medida acompanhada ao longo do tempo em relação a uma meta.",
  "l.glossary.key_performance_indicators_measures_":
    "Indicadores-chave de desempenho: medidas acompanhadas ao longo do tempo em relação a uma meta.",
  "l.glossary.objective_and_key_results_a":
    "Objetivo e resultados-chave: uma meta com alguns resultados mensuráveis.",
  "l.glossary.objectives_and_key_results_goals":
    "Objetivos e resultados-chave: metas com alguns resultados mensuráveis.",
  "l.import_view.jira_csv_export": "Jira (exportação CSV)",
  "l.import_view.asana_csv_export": "Asana (exportação CSV)",
  "l.import_view.microsoft_project_xml_file": "Microsoft Project (arquivo XML)",
  "l.import_view.columns_not_imported": "Colunas não importadas",
  "l.import_view.rows_skipped": "Linhas ignoradas",
  "l.import_view.links_not_imported": "Vínculos não importados",
  "l.import_view.parents_not_used": "Itens-pai não usados",
  "l.import_view.dates_not_understood": "Datas não compreendidas",
  "l.import_view.fields_shortened": "Campos abreviados",
  "l.import_view.status_notes": "Notas de situação",
  "l.import_view.resources_not_matched_to_people":
    "Recursos não associados a pessoas",
  "l.import_view.costs_not_imported": "Custos não importados",
  "l.import_view.schedules_not_imported": "Cronogramas não importados",
  "l.import_view.constraints_not_imported": "Restrições não importadas",
  "l.import_view.calendar_notes": "Notas de calendário",
  "l.kpi_view.improving": "Melhorando",
  "l.kpi_view.getting_worse": "Piorando",
  "l.kpi_view.no_change": "Sem mudança",
  "l.milestone_view.on_track": "No rumo",
  "l.milestone_view.at_risk": "Em risco",
  "l.milestone_view.forecast_late": "Previsto com atraso",
  "l.milestone_view.no_forecast": "Sem previsão",
  "l.milestone_view.it_is_not_linked_to":
    "Ele não está vinculado a nenhuma tarefa, então não há base para uma previsão.",
  "l.milestone_view.its_open_tasks_have_no":
    "Suas tarefas abertas não têm datas que o cronograma consiga posicionar (dê a elas uma duração e um início ou uma predecessora).",
  "l.milestone_view.the_plan_s_dependencies_contain":
    "As dependências do plano contêm um ciclo, então não é possível fazer uma previsão.",
  "l.okr_view.not_measurable_the_start_and":
    "Não mensurável: o início e a meta são iguais, então não há distância a percorrer.",
  "l.okr_view.not_measurable_a_maintain_key":
    "Não mensurável: um resultado-chave de manutenção precisa de uma faixa de tolerância.",
  "l.okr_view.not_measurable_the_numbers_are":
    "Não mensurável: os números são grandes demais para pontuar.",
  "l.onboarding_view.someone_joins_this_plan": "Alguém entra neste plano",
  "l.onboarding_view.someone_leaves_this_plan": "Alguém sai deste plano",
  "l.onboarding_view.take_this_plan_in": "Receber este plano",
  "l.onboarding_view.close_this_plan_out": "Encerrar este plano",
  "l.onboarding_view.booking": "Reserva",
  "l.onboarding.executive": "Executivo",
  "l.onboarding.outcomes_and_exposure_across_the":
    "Resultados e exposição em todo o conjunto, e não em um plano isolado.",
  "l.onboarding.read_the_executive_summary": "Ler o resumo executivo",
  "l.onboarding.health_decisions_benefits_realized_a":
    "Saúde, decisões, benefícios realizados e alinhamento estratégico em uma única visão.",
  "l.onboarding.open_the_at_a_glance": "Abrir o painel de visão geral",
  "l.onboarding.the_whole_estate_s_rag":
    "A situação RAG de todo o conjunto e as violações de cronograma de uma só vez.",
  "l.onboarding.review_the_board_pack": "Revisar o pacote da diretoria",
  "l.onboarding.the_pack_a_board_meeting":
    "O pacote que uma reunião da diretoria receberia de fato: investimentos e tendências.",
  "l.onboarding.check_budget_variance_and_exposure":
    "Verificar a variação do orçamento e a exposição",
  "l.onboarding.where_spend_is_diverging_from":
    "Onde o gasto está se afastando do plano, por moeda — as moedas nunca são somadas entre si.",
  "l.onboarding.compare_two_candidate_portfolios":
    "Comparar dois portfólios candidatos",
  "l.onboarding.what_if_scenarios_side_by":
    "Cenários hipotéticos lado a lado, antes de comprometer o financiamento com qualquer um.",
  "l.onboarding.see_what_s_ranked_highest":
    "Ver o que está mais bem classificado e por quê",
  "l.onboarding.the_smart_score_queue_every":
    "A fila da pontuação inteligente — cada pontuação mostra as evidências por trás dela ou diz que não tem nenhuma.",
  "l.onboarding.pmo_portfolio_manager": "PMO / gerente de portfólio",
  "l.onboarding.intake_governance_and_the_schedule":
    "Entrada de demandas, governança e cronograma de todos os planos.",
  "l.onboarding.open_the_intake_pipeline":
    "Abrir o funil de entrada de demandas",
  "l.onboarding.every_proposal_s_stage_from":
    "A etapa de cada proposta, do rascunho à aprovação.",
  "l.onboarding.register_a_plan": "Registrar um plano",
  "l.onboarding.the_first_plan_you_create":
    "O primeiro plano que você cria é uma boa forma de ver o formato completo dos dados.",
  "l.onboarding.browse_the_plan_registry": "Explorar o registro de planos",
  "l.onboarding.one_recursive_collection_kind_is":
    "Uma coleção recursiva — `kind` é um rótulo, não uma barreira.",
  "l.onboarding.open_a_plan_s_governance":
    "Abrir a aba de governança de um plano",
  "l.onboarding.gate_reviews_risks_and_budget":
    "Revisões de portão, riscos e linhas de orçamento desse plano.",
  "l.onboarding.look_at_the_cross_plan": "Ver o cronograma entre planos",
  "l.onboarding.dependencies_the_critical_path_and":
    "Dependências, o caminho crítico e qualquer sucessora que esteja atrasando.",
  "l.onboarding.check_the_risk_register": "Verificar o registro de riscos",
  "l.onboarding.everything_open_or_mitigating_ranked":
    "Tudo o que está aberto ou em mitigação, classificado por exposição.",
  "l.onboarding.review_the_compliance_register":
    "Revisar o registro de conformidade",
  "l.onboarding.standing_findings_across_the_estate":
    "Achados permanentes em todo o conjunto, e não só de um plano.",
  "l.onboarding.run_a_saved_report": "Executar um relatório salvo",
  "l.onboarding.a_filter_field_projection_saved":
    "Um filtro mais uma projeção de campos salvos para reutilização, executados sob demanda.",
  "l.onboarding.resource_manager": "Gerente de recursos",
  "l.onboarding.who_is_allocated_where_and":
    "Quem está alocado onde e como a entrega está de fato fluindo.",
  "l.onboarding.check_the_capacity_rollup":
    "Verificar o consolidado de capacidade",
  "l.onboarding.who_is_over_allocated_this":
    "Quem está superalocado nesta janela, somando todos os planos em que atua.",
  "l.onboarding.open_a_plan_s_task": "Abrir o quadro de tarefas de um plano",
  "l.onboarding.the_kanban_board_that_task":
    "O quadro kanban pelo qual o trabalho no nível de tarefa realmente se move.",
  "l.onboarding.read_that_plan_s_flow": "Ler os números de fluxo desse plano",
  "l.onboarding.cycle_time_lead_time_and":
    "Tempo de ciclo, prazo de entrega e eficiência de fluxo — derivados do próprio histórico do quadro.",
  "l.onboarding.scan_blocked_items_and_the":
    "Examinar os itens bloqueados e a mistura de entregas",
  "l.onboarding.what_s_stuck_the_moscow":
    "O que está parado, a mistura MoSCoW e o calendário de marcos em uma única visão.",
  "l.onboarding.look_at_the_milestone_calendar": "Ver o calendário de marcos",
  "l.onboarding.every_milestone_across_every_plan":
    "Todos os marcos de todos os planos, com as datas de vencimento.",
  "l.onboarding.see_what_automations_are_configured":
    "Ver quais automações estão configuradas",
  "l.onboarding.what_fires_on_a_field":
    "O que dispara quando um campo muda e o que isso faz.",
  "l.onboarding.open_collaborative_reviews": "Abrir as revisões colaborativas",
  "l.onboarding.where_a_plan_gets_a":
    "Onde um plano recebe uma segunda opinião antes de uma decisão sobre ele.",
  "l.raci_view.responsible": "Responsável",
  "l.raci_view.accountable": "Aprovador",
  "l.raci_view.consulted": "Consultado",
  "l.raci_view.informed": "Informado",
  "l.raci_view.does_the_work": "faz o trabalho",
  "l.raci_view.answers_for_it_and_signs":
    "responde por ele e o aprova (um por tarefa)",
  "l.raci_view.is_asked_before": "é consultado antes",
  "l.raci_view.is_told_after": "é informado depois",
  "l.scheduling_view.no_schedule_baseline_has_been":
    "Nenhuma linha de base de cronograma foi registrada para este plano.",
  "l.scheduling_view.nothing_was_baselined_to_finish":
    "Nada estava previsto na linha de base para terminar até a data de referência.",
  "l.scheduling_view.the_plan_s_dependencies_contain":
    "As dependências do plano contêm um ciclo.",
  "l.scheduling_view.there_are_no_incomplete_tasks":
    "Não há tarefas incompletas para medir.",
  "l.scheduling_view.there_are_no_relationships_between":
    "Não há relações entre tarefas para medir.",
  "l.scheduling_view.no_incomplete_task_has_dates":
    "Nenhuma tarefa incompleta tem datas que o cronograma consiga posicionar.",
  "l.scheduling_view.no_incomplete_task_has_a":
    "Nenhuma tarefa incompleta tem duração.",
  "l.scheduling_view.the_plan_has_no_tasks": "O plano não tem tarefas.",
  "l.scheduling_view.there_is_no_critical_task":
    "Não há tarefa crítica para testar.",
  "l.scheduling_view.no_path_remains_to_measure":
    "Não resta nenhum caminho para medir.",
  "l.scheduling_view.every_task_involved_has_already":
    "Todas as tarefas envolvidas já começaram, então nenhuma pode ser movida.",
  "l.scheduling_view.the_tasks_involved_are_critical":
    "As tarefas envolvidas são críticas; mover uma delas moveria o término. Permita tarefas críticas para ver esse custo.",
  "l.scheduling_view.delaying_a_task_enough_would":
    "Atrasar uma tarefa o suficiente quebraria o cronograma (folga negativa ou término mais tardio).",
  "l.scheduling_view.a_task_involved_has_a":
    "Uma tarefa envolvida tem uma restrição rígida e não pode ser movida.",
  "l.scheduling_view.the_schedule_would_not_let":
    "O cronograma não deixaria a tarefa começar mais tarde.",
  "l.scheduling_view.the_search_stopped_at_its":
    "A busca parou em seu limite antes de resolver isto.",
  "l.scheduling_view.no_probability": "sem probabilidade",
  "l.scheduling_view.no_task_named": "nenhuma tarefa indicada",
  "l.scheduling_view.no_impact_in_business_days": "sem impacto em dias úteis",
  "l.scheduling_view.the_risk_is_not_open": "o risco não está aberto",
  "l.scheduling_view.its_task_is_not_on": "sua tarefa não está no cronograma",
  "l.scheduling_view.the_plan_has_no_budget":
    "o plano não tem linha de orçamento na sua moeda",
  "l.whatif_view.move_a_task_set_its": "Mover uma tarefa (definir seu início)",
  "l.whatif_view.change_a_task_s_duration": "Alterar a duração de uma tarefa",
  "l.whatif_view.change_a_task_s_constraint":
    "Alterar a restrição de uma tarefa",
  "l.whatif_view.add_a_link": "Adicionar um vínculo",
  "l.whatif_view.remove_a_link": "Remover um vínculo",
  "l.whatif_view.must_start_on_hard": "Deve começar em (rígida)",
  "l.whatif_view.must_finish_on_hard": "Deve terminar em (rígida)",
  "l.work_view.fixed_units": "Unidades fixas",
  "l.work_view.fixed_duration": "Duração fixa",
  "l.work_view.fixed_work": "Trabalho fixo",
  "l.work_view.changing_the_work_changes_how":
    "Alterar o trabalho altera quanto tempo ele leva; a parcela do dia permanece.",
  "l.work_view.changing_the_work_changes_the":
    "Alterar o trabalho altera a parcela do dia; a duração permanece.",
  "l.work_view.changing_the_duration_or_the":
    "Alterar a duração ou a parcela do dia altera a outra; o trabalho permanece.",
  "l.work_view.flat_the_same_each_day": "Uniforme: o mesmo em cada dia",
  "l.work_view.front_loaded_more_at_the":
    "Concentrado no início: mais no começo",
  "l.work_view.back_loaded_more_at_the": "Concentrado no fim: mais no final",
  "l.work_view.bell_most_in_the_middle": "Sino: mais no meio",
  "l.work_view.turtle_ramps_up_holds_ramps": "Tartaruga: sobe, mantém, desce",
  "l.work_view.custom_percentages": "Porcentagens personalizadas",
  "l.work_view.has_no_dates_yet": "ainda não tem datas",
  "l.work_view.has_no_business_day_to": "não tem dia útil em que se encaixar",
  "l.work_view.has_a_contour_that_gives":
    "tem um perfil que não lhe dá nenhum peso",
  "l.work_view.no_rate_applies": "nenhuma taxa se aplica",
  "l.work_view.no_rate_applied_on_some":
    "nenhuma taxa se aplicou em alguns dias",
  "l.work_view.has_nobody_assigned": "não tem ninguém atribuído",
  "p._layout_svelte.reviews": "Revisões",
  "p._layout_svelte.automations": "Automações",
  "p._layout_svelte.goals_okrs": "Metas (OKRs)",
  "p._layout_svelte.onboarding": "Integração",
  "p.ceo.red": "Vermelho",
  "p.ceo.amber": "Âmbar",
  "p.ceo.green": "Verde",
  "p.display.saved_on_this_device": "Salvo neste dispositivo.",
  "p.display.saved_to_your_account_so":
    "Salvo na sua conta, então acompanha você em outros navegadores.",
  "p.display.could_not_save_to_your":
    "Não foi possível salvar na sua conta agora; fica guardado neste dispositivo.",
  "p.me.today": "Hoje",
  "p.me.this_week": "Esta semana",
  "p.me.later": "Mais tarde",
  "p.me.no_due_date": "Sem data de vencimento",
  "p.plans.pid.leveling": "Nivelamento",
  "p.plans.pid.cost_and_schedule_risk": "Risco de custo e de cronograma",
  "p.plans.pid.board.todo": "A fazer",
  "p.plans.pid.board.in_progress": "Em andamento",
  "p.plans.pid.board.in_review": "Em revisão",
  "p.common.business_day_one": "{n} dia útil",
  "p.common.business_day_other": "{n} dias úteis",
  "p.common.calendar_day_one": "{n} dia corrido",
  "p.common.calendar_day_other": "{n} dias corridos",
  "l.milestone_view.rescheduled_one": "reagendado {count} vez",
  "l.milestone_view.rescheduled_other": "reagendado {count} vezes",
  "l.milestone_view.open_of_linked_one":
    "{open_tasks} de {linked_tasks} tarefa ainda aberta",
  "l.milestone_view.open_of_linked_other":
    "{open_tasks} de {linked_tasks} tarefas ainda abertas",
  "l.cpm_view.restricted_one":
    "{n} dependência de um plano que você não pode ver ficou de fora destas datas.",
  "l.cpm_view.restricted_other":
    "{n} dependências de um plano que você não pode ver ficaram de fora destas datas.",
  "l.cpm_view.monday": "Segunda-feira",
  "l.cpm_view.tuesday": "Terça-feira",
  "l.cpm_view.wednesday": "Quarta-feira",
  "l.cpm_view.thursday": "Quinta-feira",
  "l.cpm_view.friday": "Sexta-feira",
  "l.cpm_view.saturday": "Sábado",
  "l.cpm_view.sunday": "Domingo",
  "l.ev_view.gap_more":
    "{money} a mais de valor agregado do que o planejado até agora",
  "l.ev_view.gap_less":
    "{money} a menos de valor agregado do que o planejado até agora",
  "l.ev_view.cost_line_less":
    "{word}: CPI {ratio}, {money} a menos gastos do que o trabalho agregou.",
  "l.ev_view.cost_line_more":
    "{word}: CPI {ratio}, {money} a mais gastos do que o trabalho agregou.",
  "l.ev_view.tasks_counted_one":
    "{count} tarefa contada, ponderada {weighting}; {how}",
  "l.ev_view.tasks_counted_other":
    "{count} tarefas contadas, ponderadas {weighting}; {how}",
  "l.ev_view.equally": "igualmente",
  "l.onboarding_view.would_move_one": "{n} item seria movido para {to}.",
  "l.onboarding_view.would_move_other": "{n} itens seriam movidos para {to}.",
  "l.onboarding_view.would_release_one":
    "{n} item seria liberado: tarefas e riscos sem atribuição, reservas e papéis removidos.",
  "l.onboarding_view.would_release_other":
    "{n} itens seriam liberados: tarefas e riscos sem atribuição, reservas e papéis removidos.",
  "l.whatif_view.set_duration_one":
    "Definir “{name}” como {duration_days} dia útil",
  "l.whatif_view.set_duration_other":
    "Definir “{name}” como {duration_days} dias úteis",
  "l.work_view.over_capacity_one":
    "Acima da capacidade por {days_over} dia corrido ({from} a {to}): isto pede mais do que a pessoa pode dar.",
  "l.work_view.over_capacity_other":
    "Acima da capacidade por {days_over} dias corridos ({from} a {to}): isto pede mais do que a pessoa pode dar.",
  "p.plans.pid.handover.done_one":
    "Concluído: {count} item passado e registrado.",
  "p.plans.pid.handover.done_other":
    "Concluído: {count} itens passados e registrados.",
  "p.plans.pid.what_if.committed_one":
    "Confirmado: {count} alteração registrada.",
  "p.plans.pid.what_if.committed_other":
    "Confirmado: {count} alterações registradas.",
  "l.import_view.mspdi_hint":
    "No Microsoft Project, use Arquivo → Salvar como → Formato XML. Um arquivo .mpp não pode ser lido.",
  "c.CalendarMonth.january": "Janeiro",
  "c.CalendarMonth.february": "Fevereiro",
  "c.CalendarMonth.march": "Março",
  "c.CalendarMonth.april": "Abril",
  "c.CalendarMonth.may": "Maio",
  "c.CalendarMonth.june": "Junho",
  "c.CalendarMonth.july": "Julho",
  "c.CalendarMonth.august": "Agosto",
  "c.CalendarMonth.september": "Setembro",
  "c.CalendarMonth.october": "Outubro",
  "c.CalendarMonth.november": "Novembro",
  "c.CalendarMonth.december": "Dezembro",
  "c.LilyGanttTable.schedule": "Cronograma",
  "c.LilyKanbanTable.board": "Quadro",
  "l.announcement_view.yesterday": "ontem",
  "c.CalendarMonth.holiday_state": "feriado",
  "c.CalendarMonth.absent_state": "ausente",
  "l.ev_view.word_ahead": "adiantado",
  "l.ev_view.word_behind": "atrasado",
  "l.ev_view.slipping": "em deslizamento",
  "l.import_view.task_one": "{n} tarefa",
  "l.import_view.task_other": "{n} tarefas",
  "l.import_view.dependency_one": "{n} dependência",
  "l.import_view.dependency_other": "{n} dependências",
  "l.import_view.word_update": "atualizar",
  "l.import_view.word_updated": "atualizado",
  "l.rbs.headcount_one": "{headcount} pessoa",
  "l.rbs.headcount_other": "{headcount} pessoas",
  "l.scheduling_view.exactly": "exatamente",
  "l.scheduling_view.strength_strong": "forte",
  "l.scheduling_view.strength_moderate": "moderada",
  "l.scheduling_view.strength_weak": "fraca",
  "l.simulation_inputs.planned_word": "planejado",
  "p.plans.pid.board.unassigned": "sem responsável",
  "p.plans.pid.costs.recorded_one":
    "{recorded} lançamento de tempo registrado; {already_recorded} já estavam registrados; {length} não têm taxa.",
  "p.plans.pid.costs.recorded_other":
    "{recorded} lançamentos de tempo registrados; {already_recorded} já estavam registrados; {length} não têm taxa.",
  "c.CostRiskView.histogram_label":
    "Histograma do custo simulado em {currency}, com P50, P80, P90 e o orçamento marcados",
  "c.CumulativeFlow.chart_label":
    "Fluxo acumulado ao longo de {days} dias corridos, com pico de {peak} tarefas",
  "p.plans.pid.onboarding.owner_of": "Responsável por {title}",
  "p.plans.pid.onboarding.due_date_of": "Data de vencimento de {title}",
  "p.plans.pid.onboarding.why_skip_title": "Por que ignorar {title}",
  "p.plans.pid.what_if.remove_edit": "Remover a edição {n}",
  "c.LilyGanttTable.cell_active": "{name}: de {start} a {end}{ends}",
  "c.LilyGanttTable.ends_here": " (termina aqui)",
  "c.LilyGanttTable.cell_not_scheduled": "{name}: não agendada",
  "l.simulation_inputs.range_business_days":
    "{optimistic} / {likely} / {pessimistic} dias úteis (otimista / provável / pessimista)",
  "l.enum.task_status.todo": "a fazer",
  "l.enum.task_status.in_progress": "em andamento",
  "l.enum.task_status.in_review": "em revisão",
  "l.enum.task_status.done": "concluída",
  "l.enum.task_status.blocked": "bloqueada",
  "l.enum.kind.Portfolio": "Portfólio",
  "l.enum.kind.Project": "Projeto",
  "l.enum.kind.Product": "Produto",
  "l.enum.kind.Program": "Programa",
  "l.enum.kind.Practice": "Prática",
  "l.enum.kind.Process": "Processo",
  "l.enum.kind.Purpose": "Propósito",
  "l.enum.kind.Pathway": "Trajetória",
  "l.enum.kind.Proposal": "Proposta",
  "l.enum.plan_status.Proposed": "Proposto",
  "l.enum.plan_status.Active": "Ativo",
  "l.enum.plan_status.OnHold": "Em espera",
  "l.enum.plan_status.Completed": "Concluído",
  "l.enum.plan_status.Cancelled": "Cancelado",
  "l.enum.collection.portfolios": "portfólios",
  "l.enum.collection.projects": "projetos",
  "l.enum.collection.products": "produtos",
  "l.enum.collection.programs": "programas",
  "l.enum.collection.practices": "práticas",
  "l.enum.collection.processes": "processos",
  "l.enum.collection.purposes": "propósitos",
  "l.enum.collection.pathways": "trajetórias",
  "l.enum.proposal_status.draft": "rascunho",
  "l.enum.proposal_status.submitted": "enviada",
  "l.enum.proposal_status.in_review": "em revisão",
  "l.enum.proposal_status.approved": "aprovada",
  "l.enum.proposal_status.rejected": "rejeitada",
  "l.enum.proposal_status.promoted": "promovida",
  "l.enum.band.high": "alta",
  "l.enum.band.medium": "média",
  "l.enum.band.low": "baixa",
  "l.enum.band.unscored": "sem pontuação",
  "l.enum.api.unauthorized": "não autorizado",
  "l.enum.api.forbidden": "proibido",
  "l.enum.api.not_found": "não encontrado",
  "l.enum.api.internal_error": "Erro interno",
  "l.enum.api.rate_limited": "limite de requisições excedido",
  "l.enum.api.http_status": "HTTP {status}",
  "p.error.not_found": "Essa página não existe.",
  "p.error.not_allowed": "Você não tem permissão para ver essa página.",
  "p.error.went_wrong": "Algo deu errado. Tente novamente.",
  "p.error.title": "Erro {status} — PPM",
  "p.error.back": "Voltar ao início",
  "p.issues.issues_ppm": "Registro de problemas — PPM",
  "p.issues.issues": "Registro de problemas",
  "p.issues.intro":
    "Um problema é algo que já aconteceu e precisa ser gerenciado; um risco é algo que pode acontecer. Os mais graves primeiro, depois os mais antigos. A idade é contada em dias corridos desde que o problema foi levantado.",
  "p.issues.could_not_load": "Não foi possível carregar os problemas",
  "p.issues.by_severity": "Problemas por gravidade",
  "p.issues.overdue_count": "Em atraso",
  "p.issues.oldest_open": "Aberto mais antigo: {title} ({days} dias corridos)",
  "p.issues.show": "Mostrar",
  "p.issues.all_statuses": "Todos os status",
  "p.issues.open_only": "Somente abertos e em andamento",
  "p.issues.severity_filter": "Gravidade",
  "p.issues.all_severities": "Todas as gravidades",
  "p.issues.older_than": "Com pelo menos esta quantidade de dias corridos",
  "p.issues.none": "Nenhum problema registrado.",
  "p.issues.none_match": "Nenhum problema corresponde.",
  "p.issues.loading": "Carregando…",
  "p.issues.count_one": "{n} problema, o mais grave primeiro",
  "p.issues.count_other": "{n} problemas, os mais graves primeiro",
  "p.issues.truncated":
    "Mostrando os primeiros {n}; restrinja os filtros para ver o restante.",
  "p.issues.title_col": "Problema",
  "p.issues.kind_col": "Tipo",
  "p.issues.severity_col": "Gravidade",
  "p.issues.status_col": "Situação",
  "p.issues.owner_col": "Responsável",
  "p.issues.age_col": "Idade (dias corridos)",
  "p.issues.due_col": "Vencimento",
  "p.issues.overdue_flag": "Em atraso",
  "p.issues.no_owner": "Sem responsável",
  "p.issues.no_due": "Sem data de vencimento",
  "p.issues.plan_link": "Abrir os problemas do plano",
  "p.issues.plan_issues_link": "Problemas",
  "p.plan_issues.title_ppm": "Problemas do plano — PPM",
  "p.plan_issues.title": "Problemas do plano",
  "p.plan_issues.intro":
    "Os problemas levantados neste plano, os mais graves primeiro. Um problema é algo que já aconteceu; um risco é algo que pode acontecer. Levante um problema, edite um existente ou levante um a partir de um risco que se concretizou.",
  "p.plan_issues.back": "Voltar ao plano",
  "p.plan_issues.register_link": "Problemas de todos os planos",
  "p.plan_issues.could_not_load":
    "Não foi possível carregar os problemas do plano",
  "p.plan_issues.could_not_save": "Não foi possível salvar o problema",
  "p.plan_issues.counts":
    "{total} no total, {open} abertos ou em andamento, {overdue} em atraso",
  "p.plan_issues.actions_col": "Ações",
  "p.plan_issues.edit": "Editar",
  "p.plan_issues.raise_heading": "Levantar um problema",
  "p.plan_issues.edit_heading": "Editar o problema",
  "p.plan_issues.f_title": "Título",
  "p.plan_issues.f_description": "Descrição",
  "p.plan_issues.f_owner": "Responsável (uma referência person: ou worker:)",
  "p.plan_issues.f_raised_on": "Levantado em (deixe vazio para hoje)",
  "p.plan_issues.f_resolution":
    "Resolução (necessária para encerrar o problema)",
  "p.plan_issues.raise": "Levantar problema",
  "p.plan_issues.save": "Salvar alterações",
  "p.plan_issues.cancel_edit": "Cancelar a edição",
  "p.plan_issues.raised": "O problema foi levantado.",
  "p.plan_issues.saved": "O problema foi salvo.",
  "p.plan_issues.problem_title_required": "Dê um título ao problema.",
  "p.plan_issues.problem_resolution_required":
    "Um problema encerrado precisa de uma resolução: informe como foi resolvido.",
  "p.plan_issues.problem_due_before_raised":
    "A data de vencimento não pode ser anterior ao dia em que o problema foi levantado.",
  "p.plan_issues.from_risk_heading":
    "Levantar um problema a partir de um risco",
  "p.plan_issues.from_risk_intro":
    "Para um risco que se concretizou. O problema herda o título, a descrição e o responsável do risco, e uma gravidade derivada da sua exposição; o risco em si não é alterado. Um risco pode ter um problema aberto de cada vez.",
  "p.plan_issues.f_risk": "Risco",
  "p.plan_issues.choose_risk": "Escolha um risco",
  "p.plan_issues.raise_from_risk": "Levantar problema a partir deste risco",
  "p.plan_issues.raised_from_risk":
    "O problema foi levantado a partir do risco.",
  "l.enum.severity.low": "baixa",
  "l.enum.severity.med": "média",
  "l.enum.severity.high": "alta",
  "l.enum.severity.critical": "crítica",
  "l.enum.issue_status.open": "aberto",
  "l.enum.issue_status.in_progress": "em andamento",
  "l.enum.issue_status.resolved": "resolvido",
  "l.enum.issue_status.closed": "encerrado",
  "l.enum.issue_kind.bug": "defeito",
  "l.enum.issue_kind.risk": "risco",
  "l.enum.issue_kind.blocker": "impedimento",
  "l.enum.issue_kind.question": "dúvida",
  "l.enum.issue_kind.improvement": "melhoria",
  "l.enum.issue_kind.concern": "preocupação",
  "p.signin.oidc_idle":
    "Você foi desconectado por ficar inativo por algum tempo. Entre novamente.",
  "p.plans.pid.board.only_first_tasks":
    "Este plano tem {total} tarefas; o quadro mostra as primeiras {shown}. Restrinja por componente ou leia o restante pela API.",
  "p.plan_modules.title": "Ferramentas deste plano",
  "p.plan_modules.title_ppm": "Ferramentas deste plano — PPM",
  "p.plan_modules.back": "← Voltar ao plano",
  "p.plan_modules.intro":
    "Escolha quais ferramentas este plano mostra. Uma ferramenta desativada fica oculta na página do plano, e suas páginas respondem “não encontrado”. Nada é excluído: ao reativar uma ferramenta, o que ela continha volta a aparecer.",
  "p.plan_modules.source_plan":
    "Este plano tem sua própria escolha de ferramentas.",
  "p.plan_modules.source_default":
    "Este plano usa as ferramentas padrão da implantação.",
  "p.plan_modules.legend": "Ferramentas que este plano mostra",
  "p.plan_modules.module_tasks": "Tarefas, o quadro e a análise do cronograma",
  "p.plan_modules.module_time": "Registro de tempo",
  "p.plan_modules.module_risks": "Riscos e problemas",
  "p.plan_modules.module_financials": "Orçamentos, custos e valor agregado",
  "p.plan_modules.module_milestones": "Marcos, o cronograma e o calendário",
  "p.plan_modules.module_governance":
    "Governança, portões e solicitações de mudança",
  "p.plan_modules.module_roadmap": "Roteiro público do produto",
  "p.plan_modules.none_chosen":
    "Sem nenhuma ferramenta escolhida, a página do plano mostra seu registro e nenhuma ferramenta.",
  "p.plan_modules.save": "Salvar ferramentas",
  "p.plan_modules.presets_heading": "Escolhas prontas",
  "p.plan_modules.personal_is":
    "O conjunto pessoal, para o trabalho de uma só pessoa: {names}.",
  "p.plan_modules.use_personal": "Usar o conjunto pessoal",
  "p.plan_modules.default_is": "O padrão da implantação: {names}.",
  "p.plan_modules.use_default": "Seguir o padrão da implantação",
  "p.plan_modules.no_tools": "nenhuma ferramenta",
  "p.plan_modules.unchanged":
    "Nada a salvar: este plano já segue o padrão, que é esta escolha.",
  "p.plan_modules.saved": "Salvo.",
  "p.plan_modules.could_not_load":
    "Não foi possível carregar as ferramentas deste plano.",
  "p.plan_modules.could_not_save": "A alteração não foi salva.",
  "p.canvas.title": "Canvas estratégicos",
  "p.canvas.title_ppm": "Canvas estratégicos — PPM",
  "p.canvas.intro":
    "Um canvas é um quadro de uma página com caixas, como um Lean Canvas ou uma análise SWOT. Um item nele pode se tornar uma ideia, um objetivo ou, no canvas de um plano, um risco.",
  "p.canvas.plan_title": "Canvas deste plano",
  "p.canvas.plan_title_ppm": "Canvas deste plano — PPM",
  "p.canvas.plan_intro":
    "Os canvas estratégicos deste plano. Um item de qualquer um deles pode se tornar um risco no registro deste plano.",
  "p.canvas.one_title": "Canvas estratégico",
  "p.canvas.one_title_ppm": "Canvas estratégico — PPM",
  "p.canvas.back": "← Voltar aos canvas",
  "p.canvas.list_heading": "Canvas",
  "p.canvas.filter": "Mostrar",
  "p.canvas.filter_all": "Todos os tipos",
  "p.canvas.none_yet": "Ainda não há canvas.",
  "p.canvas.none_of_kind": "Não há canvas deste tipo.",
  "p.canvas.free_standing": "sem plano",
  "p.canvas.on_plan": "em um plano",
  "p.canvas.new_heading": "Iniciar um canvas",
  "p.canvas.new_on_plan": "O novo canvas fica neste plano.",
  "p.canvas.new_free_standing":
    "O novo canvas não pertence a nenhum plano; por isso seus itens podem se tornar ideias ou objetivos, mas não riscos.",
  "p.canvas.f_template": "Tipo de canvas",
  "p.canvas.f_title": "Título",
  "p.canvas.f_description": "Descrição (opcional)",
  "p.canvas.create": "Iniciar o canvas",
  "p.canvas.could_not_load_list": "Não foi possível carregar os canvas.",
  "p.canvas.could_not_load_templates":
    "Não foi possível carregar os tipos de canvas; por isso não é possível iniciar um novo canvas.",
  "p.canvas.could_not_create": "O canvas não foi iniciado.",
  "p.canvas.problem_title_blank": "Dê um título ao canvas.",
  "p.canvas.problem_title_long": "Um título pode ter no máximo 200 caracteres.",
  "p.canvas.problem_description_long":
    "Uma descrição pode ter no máximo 2.000 caracteres.",
  "p.canvas.template_lean": "Canvas Lean",
  "p.canvas.template_business_model": "Canvas de modelo de negócio",
  "p.canvas.template_swot": "Análise SWOT",
  "p.canvas.template_empathy": "Mapa de empatia",
  "p.canvas.template_logic_model": "Modelo lógico",
  "p.canvas.template_risk": "Canvas de risco",
  "p.canvas.template_goal": "Canvas de meta",
  "p.canvas.box_problem": "Problema",
  "p.canvas.box_solution": "Solução",
  "p.canvas.box_key_metrics": "Métricas-chave",
  "p.canvas.box_unique_value_proposition": "Proposta de valor única",
  "p.canvas.box_unfair_advantage": "Vantagem competitiva",
  "p.canvas.box_channels": "Canais",
  "p.canvas.box_customer_segments": "Segmentos de clientes",
  "p.canvas.box_cost_structure": "Estrutura de custos",
  "p.canvas.box_revenue_streams": "Fontes de receita",
  "p.canvas.box_key_partners": "Parceiros-chave",
  "p.canvas.box_key_activities": "Atividades-chave",
  "p.canvas.box_key_resources": "Recursos-chave",
  "p.canvas.box_value_propositions": "Propostas de valor",
  "p.canvas.box_customer_relationships": "Relacionamento com clientes",
  "p.canvas.box_strengths": "Forças",
  "p.canvas.box_weaknesses": "Fraquezas",
  "p.canvas.box_opportunities": "Oportunidades",
  "p.canvas.box_threats": "Ameaças",
  "p.canvas.box_says": "Diz",
  "p.canvas.box_thinks": "Pensa",
  "p.canvas.box_does": "Faz",
  "p.canvas.box_feels": "Sente",
  "p.canvas.box_pains": "Dores",
  "p.canvas.box_gains": "Ganhos",
  "p.canvas.box_inputs": "Insumos",
  "p.canvas.box_activities": "Atividades",
  "p.canvas.box_outputs": "Produtos entregues",
  "p.canvas.box_outcomes": "Resultados",
  "p.canvas.box_impact": "Impacto",
  "p.canvas.box_vulnerabilities": "Vulnerabilidades",
  "p.canvas.box_existing_controls": "Controles existentes",
  "p.canvas.box_residual_risks": "Riscos residuais",
  "p.canvas.box_goal": "Meta",
  "p.canvas.box_success_measures": "Medidas de sucesso",
  "p.canvas.box_actions": "Ações",
  "p.canvas.box_obstacles": "Obstáculos",
  "p.canvas.box_resources": "Recursos",
  "p.canvas.box_empty": "Nada aqui ainda.",
  "p.canvas.f_new_item": "Novo item em {box}",
  "p.canvas.add": "Adicionar",
  "p.canvas.added": "Adicionado.",
  "p.canvas.f_text": "Texto",
  "p.canvas.f_box": "Caixa",
  "p.canvas.save": "Salvar",
  "p.canvas.cancel": "Cancelar",
  "p.canvas.saved": "Salvo.",
  "p.canvas.unchanged": "Nada mudou; por isso nada foi enviado.",
  "p.canvas.edit": "Editar",
  "p.canvas.remove": "Remover",
  "p.canvas.removed": "Removido.",
  "p.canvas.confirm_remove_item": "Remover “{text}” do canvas?",
  "p.canvas.promote": "Transformar em…",
  "p.canvas.promote_legend": "Transformar este item em",
  "p.canvas.target_idea": "uma ideia",
  "p.canvas.target_objective": "um objetivo",
  "p.canvas.target_risk": "um risco neste plano",
  "p.canvas.promoted": "Transformado em {kind}.",
  "p.canvas.promote_once":
    "Um item só pode ser transformado em outra coisa uma vez.",
  "p.canvas.promote_go": "Transformar nisto",
  "p.canvas.no_risk_free_standing":
    "Este canvas não pertence a nenhum plano; por isso um item não pode se tornar um risco.",
  "p.canvas.f_probability": "Probabilidade (1 a 5)",
  "p.canvas.f_impact": "Impacto (1 a 5)",
  "p.canvas.problem_risk_scores":
    "Um risco precisa de uma probabilidade e de um impacto, cada um um número inteiro de 1 a 5.",
  "p.canvas.problem_text_blank": "Escreva primeiro o texto do item.",
  "p.canvas.problem_text_long": "Um item pode ter no máximo 2.000 caracteres.",
  "p.canvas.became_idea": "Tornou-se uma ideia",
  "p.canvas.became_risk": "Tornou-se um risco neste plano",
  "p.canvas.became_objective": "Tornou-se um objetivo",
  "p.canvas.owner": "Responsável: {owner}",
  "p.canvas.full":
    "Este canvas contém o máximo de itens possível ({max}); remova um para adicionar outro.",
  "p.canvas.could_not_load": "Não foi possível carregar este canvas.",
  "p.canvas.could_not_add": "O item não foi adicionado.",
  "p.canvas.could_not_save": "A alteração não foi salva.",
  "p.canvas.could_not_remove": "O item não foi removido.",
  "p.canvas.could_not_promote": "O item não foi transformado em nada.",
  "p.canvas.remove_canvas_heading": "Remover o canvas",
  "p.canvas.remove_canvas_says":
    "Remover o canvas remove os seus itens. As ideias, os objetivos e os riscos em que seus itens se tornaram permanecem.",
  "p.canvas.remove_canvas": "Remover este canvas",
  "p.canvas.confirm_remove_canvas": "Remover o canvas “{title}” e seus itens?",
  "p.canvas.could_not_remove_canvas": "O canvas não foi removido.",
  "p.plan_templates.title": "Modelos de plano",
  "p.plan_templates.title_ppm": "Modelos de plano — PPM",
  "p.plan_templates.intro":
    "Um modelo é um plano e os planos subordinados a ele, mantidos sem pessoas e com cada data como um número de dias após o início, para que novos planos possam partir dele.",
  "p.plan_templates.saved_heading": "Modelos salvos",
  "p.plan_templates.starters_heading": "Modelos iniciais incluídos",
  "p.plan_templates.none_saved":
    "Nenhum modelo foi salvo ainda. Salve um a partir da página de um plano.",
  "p.plan_templates.none_starters": "Não há modelos iniciais incluídos.",
  "p.plan_templates.plans_count_one": "{count} plano",
  "p.plan_templates.plans_count_other": "{count} planos",
  "p.plan_templates.needs_start": "precisa de uma data de início",
  "p.plan_templates.could_not_load_list":
    "Não foi possível carregar os modelos.",
  "p.plan_templates.one_title": "Modelo de plano",
  "p.plan_templates.one_title_ppm": "Modelo de plano — PPM",
  "p.plan_templates.back": "← Voltar aos modelos",
  "p.plan_templates.is_starter":
    "Um modelo inicial incluído. Não pode ser alterado nem removido; inicie um plano a partir dele e salve-o com o seu próprio nome.",
  "p.plan_templates.is_saved": "Um modelo salvo, versão {version}.",
  "p.plan_templates.holds_heading": "O que ele contém",
  "p.plan_templates.holds_says":
    "Cada plano, o número de suas tarefas e seus marcos como dias após o início. Nenhuma pessoa é mantida.",
  "p.plan_templates.tasks_count_one": "{count} tarefa",
  "p.plan_templates.tasks_count_other": "{count} tarefas",
  "p.plan_templates.milestone_at": "{name}: dia {days}",
  "p.plan_templates.start_heading": "Iniciar planos a partir dele",
  "p.plan_templates.start_says":
    "Novos planos são criados a partir do modelo, com as datas contadas a partir da data de início. As tarefas começam na primeira etapa.",
  "p.plan_templates.f_plan_name": "Nome do plano de nível superior (opcional)",
  "p.plan_templates.f_start": "Data de início",
  "p.plan_templates.no_dates":
    "Este modelo não contém datas; por isso não é preciso uma data de início.",
  "p.plan_templates.f_parent": "Colocar sob",
  "p.plan_templates.no_parent": "Nenhum plano: um novo plano de nível superior",
  "p.plan_templates.start": "Iniciar os planos",
  "p.plan_templates.started":
    "Criado. Planos: {plans}; tarefas: {tasks}; marcos: {milestones}.",
  "p.plan_templates.open_new": "Abrir o novo plano",
  "p.plan_templates.could_not_load": "Não foi possível carregar este modelo.",
  "p.plan_templates.could_not_start": "Os planos não foram iniciados.",
  "p.plan_templates.problem_plan_name_long":
    "O nome de um plano pode ter no máximo 1.024 caracteres.",
  "p.plan_templates.problem_start_needed":
    "Escolha uma data de início: este modelo contém datas.",
  "p.plan_templates.problem_start_bad":
    "Escreva a data de início como um dia, por exemplo 2027-01-04.",
  "p.plan_templates.remove_heading": "Remover o modelo",
  "p.plan_templates.remove_says":
    "Remover o modelo não altera os planos já criados a partir dele.",
  "p.plan_templates.remove": "Remover este modelo",
  "p.plan_templates.confirm_remove": "Remover o modelo “{name}”?",
  "p.plan_templates.could_not_remove": "O modelo não foi removido.",
  "p.plan_templates.save_title": "Salvar como modelo",
  "p.plan_templates.save_title_ppm": "Salvar como modelo — PPM",
  "p.plan_templates.save_says":
    "Mantém este plano e os planos subordinados a ele que você pode ler, com suas tarefas, componentes, marcos e fluxos de trabalho. Nenhuma pessoa é mantida, e cada data se torna um número de dias após o início. Alterar o plano depois não altera o modelo.",
  "p.plan_templates.f_name": "Nome do modelo",
  "p.plan_templates.f_description": "Descrição (opcional)",
  "p.plan_templates.save": "Salvar o modelo",
  "p.plan_templates.saved":
    "Salvo. Planos: {plans}; tarefas: {tasks}; marcos: {milestones}.",
  "p.plan_templates.omitted_one":
    "{count} plano subordinado a este foi deixado de fora, porque você não pode lê-lo.",
  "p.plan_templates.omitted_other":
    "{count} planos subordinados a este foram deixados de fora, porque você não pode lê-los.",
  "p.plan_templates.open_template": "Abrir o modelo",
  "p.plan_templates.name_taken":
    "Já existe um modelo com este nome. Escolha outro nome.",
  "p.plan_templates.could_not_save": "O modelo não foi salvo.",
  "p.plan_templates.problem_name_blank": "Dê um nome ao modelo.",
  "p.plan_templates.problem_name_long":
    "O nome de um modelo pode ter no máximo 120 caracteres.",
  "p.plan_export.title": "Exportar para o Microsoft Project",
  "p.plan_export.title_ppm": "Exportar para o Microsoft Project — PPM",
  "p.plan_export.intro":
    "Salve as tarefas deste plano como um arquivo XML MSPDI, o formato que o Microsoft Project e outras ferramentas de cronograma abrem.",
  "p.plan_export.carried_heading": "O que o arquivo contém",
  "p.plan_export.carried":
    "Cada tarefa com sua estrutura sob a tarefa-mãe, data de início, duração em dias úteis, marcos, restrições, prazo e notas; se está concluída; os vínculos entre tarefas com seu tipo e defasagem; e o calendário de trabalho do plano.",
  "p.plan_export.not_carried_heading": "O que ele deixa de fora",
  "p.plan_export.not_carried":
    "Pessoas atribuídas e recursos; qualquer status além de concluída (uma tarefa está 0 ou 100 por cento concluída); sprints e componentes; esforço; campos personalizados; comentários; linhas de base; vínculos entre tarefas diferentes de “precede”; e defasagens em tempo decorrido (toda defasagem é em dias úteis). O arquivo também informa isso.",
  "p.plan_export.limit":
    "Um plano com mais de 5.000 tarefas não pode ser exportado.",
  "p.plan_export.not_checked":
    "O arquivo foi relido pelo importador do próprio serviço, mas ainda não foi aberto no próprio Microsoft Project.",
  "p.plan_export.download": "Baixar o arquivo MSPDI",
  "p.plan_export.to_import":
    "Importar um arquivo MSPDI para este plano em vez disso",
  "p.confidence.assess_heading": "Adicionar uma avaliação",
  "p.confidence.assess_says":
    "Um valor declarado é a opinião de uma pessoa sobre se o plano vai entregar. Ele é adicionado ao histórico e nunca substitui um anterior.",
  "p.confidence.choose_value": "Escolha um valor",
  "p.confidence.could_not_load":
    "Não foi possível carregar a confiança de entrega deste plano.",
  "p.confidence.could_not_save": "A avaliação não foi adicionada.",
  "p.confidence.f_assessed_on": "Avaliado em (opcional; hoje se ficar vazio)",
  "p.confidence.f_kind": "Quem está declarando",
  "p.confidence.f_rationale": "Por quê (pelo menos {min} caracteres)",
  "p.confidence.f_value": "Confiança de entrega",
  "p.confidence.history_heading": "Histórico, do mais recente ao mais antigo",
  "p.confidence.intro":
    "A opinião declarada de uma pessoa sobre se este plano vai entregar, mantida ao lado do RAG calculado a partir dos dados do plano e nunca misturada com ele.",
  "p.confidence.kind_independent": "Uma avaliação independente",
  "p.confidence.kind_self": "A própria equipe do plano (autoavaliação)",
  "p.confidence.no_history": "Nenhuma avaliação foi declarada ainda.",
  "p.confidence.none_yet":
    "Não avaliado: ninguém declarou uma opinião ainda. Isso não é o mesmo que verde.",
  "p.confidence.opens_exception":
    "Âmbar/vermelho ou vermelho abre uma exceção para este plano, ou agrava a que já está aberta. Alguém então precisa completá-la.",
  "p.confidence.problem_future":
    "A data da avaliação não pode estar no futuro.",
  "p.confidence.problem_kind": "Escolha quem está declarando.",
  "p.confidence.problem_rationale":
    "Diga por quê, com pelo menos {min} caracteres.",
  "p.confidence.problem_value": "Escolha um valor.",
  "p.confidence.refused_independent":
    "Uma avaliação independente exige o direito de avaliador independente. Você pode adicionar uma autoavaliação.",
  "p.confidence.save": "Adicionar a avaliação",
  "p.confidence.saved_exception_one":
    "Adicionada. Este plano agora tem uma exceção aberta: ela deve ser completada em até {hours} hora.",
  "p.confidence.saved_exception_other":
    "Adicionada. Este plano agora tem uma exceção aberta: ela deve ser completada em até {hours} horas.",
  "p.confidence.saved": "Adicionada.",
  "p.confidence.shown_heading": "O que o mapa do portfólio mostra",
  "p.confidence.shown_independent": "Avaliação independente de {when}.",
  "p.confidence.shown_self":
    "Autoavaliação de {when}; uma avaliação independente seria mostrada no lugar.",
  "p.confidence.stale": "desatualizada: anterior a este mês",
  "p.confidence.title_ppm": "Confiança de entrega — PPM",
  "p.confidence.title": "Confiança de entrega",
  "p.exceptions.a_plan": "Um plano",
  "p.exceptions.breached_one":
    "Violada: não completada em {hours} hora após a abertura.",
  "p.exceptions.breached_other":
    "Violada: não completada em {hours} horas após a abertura.",
  "p.exceptions.cancel": "Cancelar",
  "p.thresholds.could_not_load":
    "Não foi possível carregar a triagem e os limites.",
  "p.thresholds.could_not_save": "A alteração não foi salva.",
  "p.thresholds.could_not_sweep": "Não foi possível verificar os limites.",
  "p.thresholds.decide_says":
    "Registrar a decisão encerra a exceção. Ela registra o que foi decidido; não pausa, reinicia nem interrompe o programa, o que uma pessoa faz pela situação do programa.",
  "p.thresholds.decide": "Registrar a decisão",
  "p.thresholds.decided_done": "Decisão registrada: {action}.",
  "p.thresholds.f_action": "Decisão",
  "p.thresholds.f_amount": "Valor do teto",
  "p.thresholds.f_count":
    "Pacotes emitidos em vermelho em sequência ({min} a {max})",
  "p.thresholds.f_currency": "Moeda (três letras)",
  "p.thresholds.f_date":
    "Data de término prevista que não deve ser ultrapassada",
  "p.thresholds.f_expiry":
    "Exceção até (no futuro, no máximo {days} dias à frente)",
  "p.thresholds.f_outcome": "Resultado",
  "p.thresholds.f_reason": "Motivo (pelo menos {min} caracteres)",
  "p.thresholds.history_item": "{date}: {outcome}. {reason}",
  "p.thresholds.history": "Triagens anteriores",
  "p.thresholds.not_set": "Não definido",
  "p.thresholds.now_cap":
    "Ultrapassado quando o custo P80 está acima de {amount}.",
  "p.thresholds.now_forecast":
    "Ultrapassado quando o término previsto é posterior a {date}.",
  "p.thresholds.now_red_packs_one":
    "Ultrapassado quando a confiança está vermelha em {n} pacote emitido em sequência.",
  "p.thresholds.now_red_packs_other":
    "Ultrapassado quando a confiança está vermelha em {n} pacotes emitidos em sequência.",
  "p.thresholds.opened_by_threshold":
    "Aberta porque um limite do programa foi ultrapassado.",
  "p.thresholds.problem_action":
    "Escolha pausar, reiniciar, interromper ou uma substituição.",
  "p.thresholds.problem_cap": "O teto deve ser um valor acima de zero.",
  "p.thresholds.problem_count":
    "O número de pacotes deve ser um número inteiro de {min} a {max}.",
  "p.thresholds.problem_currency":
    "A moeda deve ser um código ISO 4217 de três letras.",
  "p.thresholds.problem_date": "Escolha uma data.",
  "p.thresholds.problem_expiry_long":
    "Uma substituição pode durar no máximo {days} dias.",
  "p.thresholds.problem_expiry_needed":
    "Uma substituição precisa de uma data até a qual vale.",
  "p.thresholds.problem_expiry_past":
    "A data de uma substituição deve estar no futuro.",
  "p.thresholds.problem_outcome":
    "Escolha concluir, corrigir, pausar ou interromper.",
  "p.thresholds.problem_reason":
    "O motivo deve ter pelo menos {min} caracteres.",
  "p.thresholds.remove_threshold": "Remover o limite",
  "p.thresholds.save_threshold": "Salvar o limite",
  "p.thresholds.save_triage": "Registrar a triagem",
  "p.thresholds.sweep_done_one":
    "{checked} limites verificados; {n} exceção aberta.",
  "p.thresholds.sweep_done_other":
    "{checked} limites verificados; {n} exceções abertas.",
  "p.thresholds.sweep_item": "{programme}: {kind} — {outcome}",
  "p.thresholds.sweep_list": "Limites ultrapassados",
  "p.thresholds.sweep_says":
    "A verificação abre uma exceção de entrega para cada limite ultrapassado, ou acrescenta à que já está aberta; ela não pausa, reinicia nem interrompe nada.",
  "p.thresholds.sweep": "Verificar os limites agora",
  "p.thresholds.threshold_removed": "Limite removido: {kind}.",
  "p.thresholds.threshold_saved": "Limite salvo: {kind}.",
  "p.thresholds.thresholds_heading": "Limites",
  "p.thresholds.thresholds_says":
    "Ultrapassar um limite abre uma exceção de entrega, à qual uma pessoa responde com pausa, reinício, interrupção ou uma substituição com prazo de validade. Nada é pausado, reiniciado ou interrompido automaticamente.",
  "p.thresholds.triage_heading": "Triagem",
  "p.thresholds.triage_latest": "Última triagem, {date}: {outcome}. {reason}",
  "p.thresholds.triage_none": "Nenhuma triagem registrada ainda.",
  "p.thresholds.triage_saved": "Triagem registrada.",
  "p.thresholds.triage_says":
    "Uma triagem é uma visão datada de para onde o programa deve ir, com o motivo. Ela registra a visão; não altera nada no programa.",
  "p.thresholds.unchanged": "Nada mudou; por isso nada foi enviado.",
  "p.thresholds.action_pause": "Pausar",
  "p.thresholds.action_reset": "Reiniciar",
  "p.thresholds.action_stop": "Interromper",
  "p.thresholds.action_override": "Substituir",
  "p.thresholds.kind_confidence_red_packs":
    "Confiança vermelha por vários pacotes",
  "p.thresholds.kind_forecast_past": "Término previsto além de uma data",
  "p.thresholds.kind_p80_cap": "Custo P80 acima de um teto",
  "p.thresholds.outcome_finish": "Concluir",
  "p.thresholds.outcome_fix": "Corrigir",
  "p.thresholds.outcome_pause": "Pausar",
  "p.thresholds.outcome_stop": "Interromper",
  "p.thresholds.sweep_opened": "exceção aberta",
  "p.thresholds.sweep_already_open": "exceção já aberta",
  "p.thresholds.sweep_attached_to_open_exception":
    "acrescentado à exceção aberta",
  "p.thresholds.sweep_suppressed_by_override":
    "não aberta: há uma substituição em vigor",
  "p.exceptions.closed_done": "Decisão registrada; a exceção está encerrada.",
  "p.exceptions.complete_it": "Completá-la",
  "p.exceptions.complete_says":
    "Uma exceção está completa quando tem a causa, a decisão necessária, um responsável e uma data-limite para decidir. Uma parte deixada vazia mantém o que tinha.",
  "p.exceptions.complete":
    "Completa: tem a causa, a decisão necessária, um responsável e uma data-limite para decidir.",
  "p.exceptions.conflict_closed": "Esta exceção já está encerrada.",
  "p.exceptions.could_not_load": "Não foi possível carregar as exceções.",
  "p.exceptions.could_not_save": "A alteração não foi salva.",
  "p.exceptions.decide_it": "Registrar a decisão",
  "p.exceptions.decide_says":
    "Registrar a decisão encerra a exceção. Melhorar a confiança nunca a encerra por si só.",
  "p.exceptions.decide": "Registrar a decisão e encerrar",
  "p.exceptions.f_breached": "Somente violadas",
  "p.exceptions.f_decision": "Decisão (pelo menos {min} caracteres)",
  "p.exceptions.f_owner_ref": "Responsável (uma referência de pessoa)",
  "p.exceptions.f_owner_role": "Função do responsável (o que um pacote mostra)",
  "p.exceptions.f_status": "Mostrar",
  "p.exceptions.intro_one":
    "Uma exceção é aberta quando a confiança de entrega de um plano chega a âmbar/vermelho ou vermelho. Ela deve ser completada em até {hours} hora e só é encerrada por uma decisão registrada.",
  "p.exceptions.intro_other":
    "Uma exceção é aberta quando a confiança de entrega de um plano chega a âmbar/vermelho ou vermelho. Ela deve ser completada em até {hours} horas e só é encerrada por uma decisão registrada.",
  "p.exceptions.is_closed": "encerrada {when}",
  "p.exceptions.is_open": "aberta",
  "p.exceptions.missing": "Ainda necessário: {parts}.",
  "p.exceptions.none": "Nenhuma exceção corresponde.",
  "p.exceptions.opened": "Aberta {when}",
  "p.exceptions.part_cause": "Causa",
  "p.exceptions.part_decide_by": "Decidir até",
  "p.exceptions.part_decision_needed": "Decisão necessária",
  "p.exceptions.part_decision": "Decisão",
  "p.exceptions.part_owner": "Responsável",
  "p.exceptions.problem_decision":
    "Escreva a decisão com pelo menos {min} caracteres.",
  "p.exceptions.problem_part_short":
    "Uma causa, uma decisão necessária ou um responsável precisa de pelo menos {min} caracteres para valer.",
  "p.exceptions.refused_threshold":
    "Esta exceção foi aberta por um limite; por isso ela é encerrada por uma pausa, um reinício, uma interrupção ou uma substituição registrada, e não por uma decisão aqui.",
  "p.exceptions.save": "Salvar",
  "p.exceptions.saved": "Salvo.",
  "p.exceptions.see_register": "Abrir o registro de exceções",
  "p.exceptions.status_all": "Todas as exceções",
  "p.exceptions.status_closed": "Encerradas",
  "p.exceptions.status_open": "Abertas",
  "p.exceptions.title_ppm": "Exceções — PPM",
  "p.exceptions.title": "Exceções",
  "p.exceptions.unchanged": "Nada mudou; por isso nada foi enviado.",
  "p.exceptions.worsened_one": "Agravada {count} vez desde que foi aberta",
  "p.exceptions.worsened_other": "Agravada {count} vezes desde que foi aberta",
  "p.exceptions.worst": "Pior confiança: {value}",
  "p.aims.aims_heading": "Objetivos estratégicos",
  "p.aims.aims_legend": "Objetivos que são metas estratégicas",
  "p.aims.aims_says":
    "Um objetivo estratégico é um objetivo em torno do qual o portfólio se organiza. Desmarcar um também deixa de torná-lo o objetivo principal de qualquer programa.",
  "p.aims.choose_aim": "Escolha um objetivo estratégico",
  "p.aims.choose_programme": "Escolha um programa",
  "p.aims.could_not_load": "Não foi possível carregar o mapa do portfólio.",
  "p.aims.could_not_save": "A alteração não foi salva.",
  "p.aims.f_aim": "Objetivo estratégico principal",
  "p.aims.f_period": "Mês (vazio para este mês)",
  "p.aims.f_programme": "Programa",
  "p.aims.intro":
    "Os programas que você pode ler, cada um uma vez sob o seu objetivo estratégico principal. A confiança de entrega é o valor declarado, mostrado ao lado do RAG calculado a partir dos dados, nunca misturado com ele; não avaliado não é verde.",
  "p.aims.map_heading": "Mapa do portfólio, {period}",
  "p.aims.marked": "Marcado como objetivo estratégico.",
  "p.aims.no_aims":
    "Nenhum objetivo é um objetivo estratégico ainda; marque um acima primeiro.",
  "p.aims.no_objectives": "Ainda não há objetivos.",
  "p.aims.no_programmes":
    "Não há programas no mapa: um programa é um plano do tipo Programa.",
  "p.aims.primary_heading": "O objetivo estratégico principal de um programa",
  "p.aims.primary_says":
    "Um programa aparece no mapa sob o seu único objetivo estratégico principal; quaisquer outros objetivos a que serve são citados ao lado. Escolher um novo principal torna o anterior um vínculo comum.",
  "p.aims.primary_set": "Objetivo estratégico principal definido.",
  "p.aims.problem_already_primary":
    "Esse já é o objetivo estratégico principal deste programa.",
  "p.aims.problem_choose": "Escolha um programa e um objetivo estratégico.",
  "p.aims.problem_period":
    "Escolha o mês como AAAA-MM, ou deixe vazio para este mês.",
  "p.aims.set_primary": "Definir o objetivo estratégico principal",
  "p.aims.show": "Mostrar",
  "p.aims.title_ppm": "Mapa do portfólio — PPM",
  "p.aims.title": "Mapa do portfólio",
  "p.aims.to_objectives": "Abrir os objetivos",
  "p.aims.unmarked": "Não é mais um objetivo estratégico.",
  "p.programme.cited_exception":
    "Decorreu de uma exceção de entrega deste programa.",
  "p.programme.cited_review":
    "Decorreu de uma revisão de portão deste programa.",
  "p.programme.cites_none": "Nenhuma das duas",
  "p.programme.conflict_status":
    "O programa já tem essa situação, ou está interrompido e continua interrompido.",
  "p.programme.could_not_decide": "A decisão não foi registrada.",
  "p.programme.could_not_load":
    "Não foi possível carregar os portões e a situação deste plano.",
  "p.programme.could_not_save_gates":
    "Nem todas as datas de portão foram salvas; as datas mostradas são as que o serviço tem.",
  "p.programme.decide": "Registrar a decisão",
  "p.programme.decided": "Registrado: o programa agora está {status}.",
  "p.programme.do_pause": "Pausar o programa",
  "p.programme.do_resume": "Retomar o programa",
  "p.programme.do_stop": "Interromper o programa",
  "p.programme.f_change": "Mudar o programa para",
  "p.programme.f_exception": "Decorre de uma exceção (opcional)",
  "p.programme.f_reason": "Motivo (pelo menos {min} caracteres)",
  "p.programme.f_review": "Decorre de uma revisão de portão (opcional)",
  "p.programme.gates_heading": "Datas planejadas dos portões",
  "p.programme.gates_says":
    "A data em que este plano espera a sua revisão em cada portão. Uma data não pode ser apagada depois de definida, apenas movida.",
  "p.programme.gates_saved": "Datas dos portões salvas.",
  "p.programme.gates_unchanged":
    "Nenhuma data de portão mudou; por isso nada foi enviado.",
  "p.programme.held_allocations_one":
    "{count} alocação está sinalizada, não excluída.",
  "p.programme.held_allocations_other":
    "{count} alocações estão sinalizadas, não excluídas.",
  "p.programme.held_benefits_one": "{count} benefício é mantido no registro.",
  "p.programme.held_benefits_other":
    "{count} benefícios são mantidos no registro.",
  "p.programme.held_heading": "O que o programa ainda contém",
  "p.programme.held_money": "Dinheiro: planejado {planned}, real {actual}.",
  "p.programme.history_heading": "Decisões, da mais recente à mais antiga",
  "p.programme.no_history": "Nenhuma decisão ainda: o programa está ativo.",
  "p.programme.not_a_programme":
    "Somente um plano do tipo Programa tem situação de programa. Suas datas de portão ainda podem ser planejadas acima.",
  "p.programme.problem_reason": "Dê o motivo com pelo menos {min} caracteres.",
  "p.programme.problem_status": "Escolha a mudança.",
  "p.programme.refused_decider":
    "Pausar, retomar ou interromper um programa exige o direito de decisor do programa.",
  "p.programme.refused_not_programme":
    "Somente um plano do tipo Programa tem situação, e uma revisão ou exceção citada deve ser deste programa.",
  "p.programme.save_gates": "Salvar as datas dos portões",
  "p.programme.status_heading": "Situação do programa",
  "p.programme.status_is": "Situação: {status}",
  "p.programme.status_says":
    "Um programa está ativo até que uma decisão registrada o pause ou o interrompa. Uma pausa sinaliza suas alocações e não exclui nenhuma.",
  "p.programme.stop_warns":
    "Interromper é definitivo: o programa continua interrompido e mantém o seu registro.",
  "p.programme.stopped_final":
    "Este programa está interrompido. Isso é definitivo; o seu registro é mantido.",
  "p.programme.title_ppm": "Portões e situação do programa — PPM",
  "p.programme.title": "Portões e situação do programa",
  "p.programme.buffer_by_days": "Um número de dias úteis",
  "p.programme.buffer_by_rule":
    "Metade do trabalho crítico da linha de base, arredondada para cima",
  "p.programme.buffer_declared_one":
    "Declarado: uma reserva de {days} dia útil.",
  "p.programme.buffer_declared_other":
    "Declarado: uma reserva de {days} dias úteis.",
  "p.programme.buffer_heading": "Reserva do projeto",
  "p.programme.buffer_says":
    "O tempo reservado no fim do programa para absorver atrasos. O gráfico mostra quanto dela é usado em relação a quanto do trabalho crítico está concluído. Uma declaração nunca substitui uma anterior; a mais recente é a reserva. Um plano de reservas da corrente crítica, onde houver, é usado no lugar.",
  "p.programme.buffer_history": "Declarações, da mais recente à mais antiga",
  "p.programme.buffer_was_days_one": "{days} dia útil, declarado",
  "p.programme.buffer_was_days_other": "{days} dias úteis, declarados",
  "p.programme.buffer_was_rule_one": "{days} dia útil, pela regra",
  "p.programme.buffer_was_rule_other": "{days} dias úteis, pela regra",
  "p.programme.could_not_declare_buffer": "A reserva não foi declarada.",
  "p.programme.could_not_load_buffer":
    "Não foi possível ler a reserva do projeto.",
  "p.programme.declare_buffer": "Declarar a reserva",
  "p.programme.f_buffer_days": "Dias úteis (1 a {max})",
  "p.programme.f_buffer_how": "Qual o tamanho da reserva",
  "p.programme.f_buffer_note": "Nota (opcional)",
  "p.programme.problem_buffer_days":
    "Escreva a reserva como um número inteiro de dias úteis de 1 a 2000.",
  "p.programme.refused_buffer_rule":
    "A regra exige uma linha de base de cronograma com trabalho crítico que tenha duração; defina primeiro uma linha de base ou informe um número de dias.",
  "p.programme.refused_not_programme_buffer":
    "Somente um plano do tipo Programa tem uma reserva de projeto.",
  "p.programme.profile_heading": "Modelo de entrega e relatórios",
  "p.programme.f_model": "Modelo de entrega",
  "p.programme.f_owner": "Organização responsável",
  "p.programme.no_owner": "Nenhuma organização responsável",
  "p.programme.model_says":
    "O modelo de entrega diz quem constrói e quem implanta; o pacote da diretoria o mostra ao lado do programa. O modelo e o responsável são salvos juntos.",
  "p.programme.save_model": "Salvar o modelo de entrega",
  "p.programme.model_saved": "O modelo de entrega foi salvo.",
  "p.programme.f_method": "Como o programa relata o progresso",
  "p.programme.method_evm":
    "Valor agregado (desempenho de custo e de cronograma)",
  "p.programme.method_product": "Burn-up e fluxo (um programa de produto)",
  "p.programme.save_method": "Salvar o método de relatório",
  "p.programme.method_saved_evm": "O programa agora relata por valor agregado.",
  "p.programme.method_saved_product":
    "O programa agora relata por burn-up e fluxo.",
  "p.programme.profile_unchanged": "Nada mudou; por isso nada foi enviado.",
  "p.programme.could_not_load_profile":
    "Não foi possível carregar o modelo de entrega.",
  "p.programme.could_not_save_profile": "A alteração não foi salva.",
  "p.benefits.classified": "Classificação salva.",
  "p.benefits.classify_says":
    "Salvar envia a classificação inteira: uma escolha deixada como não informada a apaga.",
  "p.benefits.could_not_load":
    "Não foi possível carregar os benefícios deste plano.",
  "p.benefits.could_not_save": "A classificação não foi salva.",
  "p.benefits.could_not_validate": "A validação não foi registrada.",
  "p.benefits.f_annual": "Valor anual ({currency})",
  "p.benefits.f_classification": "Classificação",
  "p.benefits.f_kpi": "KPI que o mede",
  "p.benefits.f_note": "Nota (opcional)",
  "p.benefits.f_recurrence": "Recorrente",
  "p.benefits.f_validated": "Valor validado até a data ({currency})",
  "p.benefits.intro":
    "Um valor realizado é uma declaração. Ele só é validado por uma validação registrada de alguém de fora do programa; cada uma informa o valor validado até a data, e vale a mais recente.",
  "p.benefits.measured_by_kpi":
    "Um benefício não financeiro é medido pelo seu KPI, não validado como um valor.",
  "p.benefits.money_line": "Meta {target}; realizado (declarado) {realized}.",
  "p.benefits.no_kpi": "Sem KPI",
  "p.benefits.non_financial": "Um benefício não financeiro.",
  "p.benefits.none": "Este plano ainda não tem benefícios.",
  "p.benefits.problem_annual_amount":
    "Escreva o valor anual como um número, não inferior a 0.",
  "p.benefits.problem_annual_financial":
    "Somente um benefício financeiro, com moeda, tem valor anual.",
  "p.benefits.problem_annual_recurring":
    "Somente um benefício recorrente tem valor anual: escolha recorrente primeiro.",
  "p.benefits.problem_validated_amount":
    "Escreva o valor validado até a data como um número, não inferior a 0.",
  "p.benefits.refused_member":
    "Um validador não pode ser membro do programa: peça a alguém de fora que valide.",
  "p.benefits.save": "Salvar a classificação",
  "p.benefits.standing_claimed": "Declarado, ainda não validado",
  "p.benefits.standing_not_claimed": "Nada declarado ainda",
  "p.benefits.standing_partly_validated": "Parcialmente validado",
  "p.benefits.standing_validated": "Validado",
  "p.benefits.title_ppm": "Benefícios — PPM",
  "p.benefits.title": "Benefícios: classificação e validação",
  "p.benefits.validate": "Registrar a validação",
  "p.benefits.validated_done": "Validação registrada.",
  "p.benefits.validated_to_date": "{amount} validados até a data",
  "p.benefits.validation_heading": "Validação",
  "p.benefits.validation_says":
    "As validações são mantidas como registradas; uma nova informa o valor validado até a data e não substitui nenhuma.",
  "p.funding.add": "Adicionar a fonte",
  "p.funding.added": "Fonte de financiamento adicionada.",
  "p.funding.choose_plan": "Escolha um plano",
  "p.funding.could_not_load_lines":
    "Não foi possível carregar as linhas de orçamento deste plano.",
  "p.funding.could_not_load_money":
    "Não foi possível carregar o dinheiro por fonte de financiamento.",
  "p.funding.could_not_load":
    "Não foi possível carregar as fontes de financiamento.",
  "p.funding.could_not_save": "A alteração não foi salva.",
  "p.funding.f_combine": "Combinar capital e despesa corrente",
  "p.funding.f_fy": "Ano fiscal (o ano em que começa; vazio para este)",
  "p.funding.f_name": "Nova fonte de financiamento",
  "p.funding.f_plan": "Plano",
  "p.funding.f_source": "Fonte de financiamento",
  "p.funding.intro":
    "De onde vem o dinheiro de cada programa. Uma linha de orçamento sem fonte de financiamento é contada e listada, nunca ocultada.",
  "p.funding.is_active": "ativa",
  "p.funding.is_retired": "desativada",
  "p.funding.line_cleared": "A linha não tem mais uma fonte de financiamento.",
  "p.funding.line_set": "A fonte de financiamento da linha foi definida.",
  "p.funding.lines_heading": "Linhas de orçamento e sua fonte de financiamento",
  "p.funding.lines_says":
    "Escolha um plano para ver suas linhas de orçamento e dar a cada uma uma fonte de financiamento.",
  "p.funding.money_heading": "Dinheiro por fonte de financiamento",
  "p.funding.no_lines": "Este plano não tem linhas de orçamento.",
  "p.funding.no_source": "Sem fonte de financiamento",
  "p.funding.no_sources":
    "Nenhuma fonte de financiamento foi adicionada ainda.",
  "p.funding.problem_fy":
    "Escreva o ano fiscal como o ano em que começa, de 2000 a 2100.",
  "p.funding.problem_name_taken":
    "Já existe uma fonte de financiamento com esse nome.",
  "p.funding.problem_name":
    "O nome de uma fonte de financiamento tem de 2 a 120 caracteres.",
  "p.funding.refused_inactive":
    "Essa fonte de financiamento está desativada; por isso não pode ser atribuída a uma linha.",
  "p.funding.restore": "Restaurar",
  "p.funding.restored": "Fonte de financiamento restaurada.",
  "p.funding.retire_says":
    "Uma fonte desativada não pode mais ser atribuída a uma linha, mas as linhas que a têm a mantêm.",
  "p.funding.retire": "Desativar",
  "p.funding.retired_option": "{name} (desativada)",
  "p.funding.retired": "Fonte de financiamento desativada.",
  "p.funding.show": "Mostrar",
  "p.funding.sources_heading": "Fontes de financiamento",
  "p.funding.title_ppm": "Financiamento — PPM",
  "p.funding.title": "Financiamento",
  "p.director_pack.actions_heading": "Revisar, emitir ou reemitir",
  "p.director_pack.aud_board": "Conselho",
  "p.director_pack.aud_escalation_board": "Comitê de escalonamento",
  "p.director_pack.aud_executive": "Executivos",
  "p.director_pack.aud_org": "Organização: {name}",
  "p.director_pack.aud_partner": "Parceiros",
  "p.director_pack.choose_audience": "Escolha um público",
  "p.director_pack.conflict_draft":
    "Este período já tem um pacote para esse público. Abra-o: emita o seu rascunho ou reemita-o com uma nota de mudança.",
  "p.director_pack.conflict_issue":
    "Este pacote não pode ser emitido agora: ele já foi emitido, ou é um rascunho agendado que o escritório de portfólio não revisou.",
  "p.director_pack.conflict_reissue":
    "Somente a última versão emitida é reemitida, e já existe uma versão posterior.",
  "p.director_pack.conflict_review":
    "Este rascunho já foi revisado, ou não é mais um rascunho.",
  "p.director_pack.could_not_act": "A alteração não foi feita.",
  "p.director_pack.could_not_draft": "O pacote não foi redigido.",
  "p.director_pack.cut_off_says":
    "Deixe vazio para usar o momento atual. O pacote lê os planos como estavam na data de corte; ela não pode estar no futuro nem ter mais de 400 dias.",
  "p.director_pack.draft_heading": "Redigir um pacote",
  "p.director_pack.draft_says":
    "Redigir lê os planos que você pode compartilhar como estão na data de corte e guarda o resultado como rascunho. Nada é enviado ao público até que alguém o emita.",
  "p.director_pack.draft": "Redigir o pacote",
  "p.director_pack.f_audience": "Público",
  "p.director_pack.f_change_note": "Nota de mudança",
  "p.director_pack.f_cut_off": "Data de corte (opcional)",
  "p.director_pack.f_period": "Período (mês)",
  "p.director_pack.from_schedule": "criado pelo agendamento mensal",
  "p.director_pack.is_reviewed": "Revisado pelo escritório de portfólio.",
  "p.director_pack.is_stale":
    "desatualizado: sua data de corte é anterior ao fim do seu período",
  "p.director_pack.issue_says":
    "Emitir congela o pacote e avisa o seu público. Não pode ser desfeito; uma mudança posterior é uma reemissão.",
  "p.director_pack.issue_waits":
    "O agendamento mensal criou este rascunho; por isso o escritório de portfólio o revisa antes que possa ser emitido.",
  "p.director_pack.issue": "Emitir o pacote",
  "p.director_pack.issued_done": "Emitido.",
  "p.director_pack.not_reviewed": "Ainda não revisado.",
  "p.director_pack.problem_audience": "Escolha um público.",
  "p.director_pack.problem_cut_off_bad":
    "Escreva a data de corte como uma data e hora.",
  "p.director_pack.problem_cut_off_future":
    "A data de corte não pode estar no futuro.",
  "p.director_pack.problem_cut_off_old":
    "A data de corte não pode ter mais de 400 dias.",
  "p.director_pack.problem_note":
    "Diga por que há uma nova versão, com pelo menos {min} caracteres.",
  "p.director_pack.problem_period": "Escolha o período como um mês.",
  "p.director_pack.refused_edit":
    "Redigir, revisar e reemitir um pacote exige acesso de editor de pacotes da diretoria.",
  "p.director_pack.refused_issue":
    "Emitir um pacote exige acesso de emissor de pacotes da diretoria.",
  "p.director_pack.reissue_says":
    "Reemitir redige a próxima versão com a sua nota; esta versão continua legível como está. Escreva pelo menos {min} caracteres.",
  "p.director_pack.reissue": "Reemitir com uma nota de mudança",
  "p.director_pack.review": "Marcar como revisado",
  "p.director_pack.reviewed_done": "Marcado como revisado.",
  "p.director_pack.add_schedule_heading": "Adicionar um agendamento",
  "p.director_pack.add_schedule": "Adicionar o agendamento",
  "p.director_pack.conflict_schedule":
    "Esse público já tem um agendamento; altere o dia dele em vez disso.",
  "p.director_pack.could_not_change_schedule":
    "O agendamento não foi alterado.",
  "p.director_pack.could_not_load_schedules":
    "Não foi possível carregar os agendamentos.",
  "p.director_pack.could_not_load_subscriptions":
    "Não foi possível carregar sobre quais pacotes você é avisado.",
  "p.director_pack.could_not_subscribe": "A alteração não foi salva.",
  "p.director_pack.day_says":
    "Um dia de 1 a 28, para que todo mês o tenha. A primeira execução é o próximo desses dias.",
  "p.director_pack.f_day": "Dia do mês",
  "p.director_pack.last_drafted":
    "Última execução {when}: redigiu o pacote de {period}.",
  "p.director_pack.last_run": "Última execução {when}.",
  "p.director_pack.last_skipped":
    "Última execução {when}: ignorou {period}, que já tinha um pacote.",
  "p.director_pack.move_day": "Mover para este dia",
  "p.director_pack.no_schedules":
    "Nenhum pacote é redigido por agendamento ainda.",
  "p.director_pack.problem_day": "Escolha um dia do mês de 1 a 28.",
  "p.director_pack.refused_subscribe":
    "Você só pode ser avisado sobre os públicos cujos pacotes pode ler.",
  "p.director_pack.schedule_added": "Agendamento adicionado.",
  "p.director_pack.schedule_off":
    "Desligado. Redigiria no dia {day} de cada mês.",
  "p.director_pack.schedule_on":
    "Redige no dia {day} de cada mês; a próxima vez em {next}.",
  "p.director_pack.schedule_saved": "Salvo.",
  "p.director_pack.schedule_switched_off": "Desligado.",
  "p.director_pack.schedule_switched_on": "Ligado.",
  "p.director_pack.schedule_unchanged":
    "Esse já é o dia dele; por isso nada foi enviado.",
  "p.director_pack.schedules_intro":
    "Cada agendamento redige o pacote de um público em um dia de cada mês. Nunca emite um: o escritório de portfólio revisa o rascunho e uma pessoa o emite.",
  "p.director_pack.schedules_link": "Agendamentos mensais",
  "p.director_pack.schedules_title_ppm": "Agendamentos mensais — PPM",
  "p.director_pack.schedules_title": "Agendamentos mensais",
  "p.director_pack.subs_heading": "Avise-me quando um pacote for emitido",
  "p.director_pack.subs_legend": "Pacotes sobre os quais ser avisado",
  "p.director_pack.subs_says":
    "Você é avisado no serviço, com um link, quando um pacote de um público que você escolher é emitido. O aviso nunca contém o próprio pacote.",
  "p.director_pack.subscribed": "Você será avisado.",
  "p.director_pack.sweep_heading": "Executar o que está pendente agora",
  "p.director_pack.sweep_result":
    "Redigidos: {drafted}; ignorados porque já existia um pacote: {skipped}.",
  "p.director_pack.sweep_says":
    "Redige todo pacote cujo dia chegou e que ainda não tem nenhum. Nada é emitido.",
  "p.director_pack.sweep": "Executar os agendamentos pendentes",
  "p.director_pack.switch_off": "Desligar",
  "p.director_pack.switch_on": "Ligar",
  "p.director_pack.unsubscribed": "Você não será mais avisado.",
  "p.plans.pid.modules_failed":
    "Não foi possível verificar quais ferramentas este plano usa; por isso todas as ferramentas são listadas.",
  "p._layout_svelte.handling_mark": "Marca de tratamento: {mark}",
  "p._layout_svelte.handling_mark_unknown":
    "Marca de tratamento: não foi possível verificar. Trate este plano como marcado até que seja verificado.",
  "l.enum.cr_status.draft": "rascunho",
  "l.enum.cr_status.submitted": "enviada",
  "l.enum.cr_status.under_review": "em revisão",
  "l.enum.cr_status.approved": "aprovada",
  "l.enum.cr_status.rejected": "rejeitada",
  "l.enum.cr_status.implemented": "implementada",
  "l.enum.cr_status.withdrawn": "retirada",
  "l.enum.cr_kind.scope": "escopo",
  "l.enum.cr_kind.schedule": "cronograma",
  "l.enum.cr_kind.cost": "custo",
  "l.enum.cr_kind.quality": "qualidade",
  "l.enum.cr_kind.other": "outro",
  "p.plan_change_requests.title_ppm": "Solicitações de mudança do plano — PPM",
  "p.plan_change_requests.title": "Solicitações de mudança do plano",
  "p.plan_change_requests.intro":
    "As solicitações de mudança deste plano, das mais recentes às mais antigas. Redija uma e edite-a enquanto for rascunho; envie-a a um aprovador nomeado, que não seja você; o aprovador a revisa e registra a decisão com um motivo. Uma solicitação aberta pode ser retirada.",
  "p.plan_change_requests.register_link":
    "Solicitações de mudança de todos os planos",
  "p.plan_change_requests.could_not_load":
    "Não foi possível carregar as solicitações de mudança do plano",
  "p.plan_change_requests.could_not_save":
    "Não foi possível salvar a solicitação de mudança",
  "p.plan_change_requests.steps": "Etapas",
  "p.plan_change_requests.draft_heading": "Redigir uma solicitação de mudança",
  "p.plan_change_requests.edit_heading": "Editar o rascunho",
  "p.plan_change_requests.f_description": "Descrição (o que vai mudar)",
  "p.plan_change_requests.f_reason": "Motivo (por que a mudança é necessária)",
  "p.plan_change_requests.f_days":
    "Impacto no cronograma em dias úteis (negativo antecipa o término; deixe vazio se não avaliado)",
  "p.plan_change_requests.f_cost":
    "Impacto no custo (deixe vazio se não avaliado)",
  "p.plan_change_requests.f_currency":
    "Moeda do custo (três letras, por exemplo EUR)",
  "p.plan_change_requests.f_summary": "Resumo do impacto",
  "p.plan_change_requests.save_draft": "Salvar o rascunho",
  "p.plan_change_requests.drafted": "A solicitação de mudança foi redigida.",
  "p.plan_change_requests.saved": "O rascunho foi salvo.",
  "p.plan_change_requests.problem_title_required":
    "Dê um título à solicitação de mudança.",
  "p.plan_change_requests.problem_description_required":
    "Diga o que vai mudar.",
  "p.plan_change_requests.problem_reason_required":
    "Diga por que a mudança é necessária.",
  "p.plan_change_requests.problem_days_not_whole":
    "O impacto no cronograma é um número inteiro de dias úteis.",
  "p.plan_change_requests.problem_days_out_of_range":
    "O impacto no cronograma deve ficar entre -3650 e 3650 dias úteis.",
  "p.plan_change_requests.problem_cost_not_amount":
    "O impacto no custo não é um valor.",
  "p.plan_change_requests.problem_cost_needs_currency":
    "Um impacto no custo precisa da sua moeda.",
  "p.plan_change_requests.problem_currency_needs_cost":
    "Uma moeda precisa de um impacto no custo.",
  "p.plan_change_requests.problem_currency_invalid":
    "A moeda é um código de três letras, por exemplo EUR, JPY ou GBP.",
  "p.plan_change_requests.steps_heading": "Etapas de {title}",
  "p.plan_change_requests.status_now": "Agora está {status}.",
  "p.plan_change_requests.decided": "Decidido por {who} em {on}: {reason}",
  "p.plan_change_requests.no_steps":
    "Nenhuma outra etapa pode ser dada nesta solicitação de mudança.",
  "p.plan_change_requests.who_note":
    "O serviço decide quem pode dar cada etapa: somente quem a redigiu a edita, envia ou retira (um administrador também pode retirá-la), e somente o aprovador nomeado a revisa ou decide.",
  "p.plan_change_requests.submit_heading": "Enviar para uma decisão",
  "p.plan_change_requests.f_approver":
    "Aprovador (uma referência person: ou worker:, não quem a redigiu)",
  "p.plan_change_requests.submit": "Enviar",
  "p.plan_change_requests.submitted": "A solicitação de mudança foi enviada.",
  "p.plan_change_requests.problem_approver_not_reference":
    "Indique o aprovador como uma referência person: ou worker:.",
  "p.plan_change_requests.problem_approver_is_requester":
    "O aprovador não pode ser quem pediu a mudança.",
  "p.plan_change_requests.review": "Iniciar a revisão",
  "p.plan_change_requests.review_started": "A revisão foi iniciada.",
  "p.plan_change_requests.decide_heading": "Registrar a decisão",
  "p.plan_change_requests.f_decision": "Decisão",
  "p.plan_change_requests.approve": "Aprovar",
  "p.plan_change_requests.reject": "Rejeitar",
  "p.plan_change_requests.f_decision_reason":
    "Motivo da decisão (pelo menos 10 caracteres)",
  "p.plan_change_requests.decide": "Registrar decisão",
  "p.plan_change_requests.decision_recorded": "A decisão foi registrada.",
  "p.plan_change_requests.problem_decision_reason":
    "Uma decisão precisa de um motivo de pelo menos 10 caracteres.",
  "p.plan_change_requests.withdraw": "Retirar a solicitação de mudança",
  "p.plan_change_requests.withdrawn": "A solicitação de mudança foi retirada.",
  "p.plan_change_requests.close_steps": "Fechar as etapas",
  "p.plan_change_requests.refused_requester":
    "Somente quem redigiu esta solicitação de mudança pode fazer isso.",
  "p.plan_change_requests.refused_withdraw":
    "Somente quem redigiu esta solicitação de mudança, ou um administrador, pode retirá-la.",
  "p.plan_change_requests.refused_approver":
    "Somente o aprovador nomeado pode revisar ou decidir esta solicitação de mudança.",
  "p.plan_change_requests.refused_state":
    "Esta solicitação de mudança avançou e essa etapa não está mais aberta para ela; a lista foi lida novamente.",
  "p.plan_change_requests.refused_invalid":
    "O serviço não aceitou o que foi informado: verifique os campos e tente novamente.",
  "p.change_requests.change_requests_ppm": "Solicitações de mudança — PPM",
  "p.change_requests.change_requests": "Solicitações de mudança",
  "p.change_requests.intro":
    "Uma solicitação de mudança é uma proposta controlada para alterar uma linha de base aprovada. Um aprovador nomeado, que não seja o solicitante, decide e dá um motivo. Os impactos são os que o solicitante declarou; uma previsão só é mostrada quando há edições hipotéticas anexadas.",
  "p.change_requests.could_not_load":
    "Não foi possível carregar as solicitações de mudança",
  "p.change_requests.by_status": "Solicitações de mudança por status",
  "p.change_requests.awaiting": "Aguardando uma decisão",
  "p.change_requests.oldest_open":
    "Aberta mais antiga: {title} ({days} dias corridos)",
  "p.change_requests.show": "Mostrar",
  "p.change_requests.all_statuses": "Todos os status",
  "p.change_requests.open_only": "Somente as abertas",
  "p.change_requests.none": "Nenhuma solicitação de mudança registrada.",
  "p.change_requests.none_match": "Nenhuma solicitação de mudança corresponde.",
  "p.change_requests.loading": "Carregando…",
  "p.change_requests.count_one":
    "{n} solicitação de mudança, a mais recente primeiro",
  "p.change_requests.count_other":
    "{n} solicitações de mudança, as mais recentes primeiro",
  "p.change_requests.title_col": "Solicitação de mudança",
  "p.change_requests.kind_col": "Tipo",
  "p.change_requests.status_col": "Situação",
  "p.change_requests.impact_col": "Impacto declarado",
  "p.change_requests.age_col": "Idade (dias corridos)",
  "p.change_requests.approver_col": "Aprovador",
  "p.change_requests.no_approver": "Ainda não nomeado",
  "p.change_requests.no_impact": "Nenhum impacto declarado",
  "p.change_requests.impact_days": "{days} dias úteis",
  "p.change_requests.plan_link": "Abrir as solicitações de mudança do plano",
  "p.director_pack.director_packs_ppm": "Pacotes da diretoria — PPM",
  "p.director_pack.director_packs": "Pacotes da diretoria",
  "p.director_pack.intro":
    "Um pacote da diretoria é o relatório do mês para os diretores, redigido uma vez, emitido por uma pessoa nomeada e depois congelado: um pacote emitido nunca muda, e uma correção é uma nova versão com uma nota. Cada número abaixo é lido do pacote emitido; nada é recalculado aqui.",
  "p.director_pack.could_not_load":
    "Não foi possível carregar os pacotes da diretoria",
  "p.director_pack.could_not_load_one":
    "Não foi possível carregar este pacote da diretoria",
  "p.director_pack.loading": "Carregando…",
  "p.director_pack.none":
    "Nenhum pacote da diretoria está disponível para você.",
  "p.director_pack.col_period": "Período",
  "p.director_pack.col_audience": "Público",
  "p.director_pack.col_version": "Versão",
  "p.director_pack.col_status": "Situação",
  "p.director_pack.col_issued": "Emitido",
  "p.director_pack.status_draft": "rascunho",
  "p.director_pack.status_issued": "emitido",
  "p.director_pack.open_pack":
    "Abrir o pacote de {period}, {audience}, versão {version}",
  "p.director_pack.all_packs": "Todos os pacotes da diretoria",
  "p.director_pack.pack_heading": "Pacote da diretoria {period} — {audience}",
  "p.director_pack.version_line":
    "Versão {version} · {status} · data de corte {cutoff}",
  "p.director_pack.issued_on": "Emitido {when}",
  "p.director_pack.draft_note":
    "Este é um rascunho. Ainda não foi emitido; por isso não está congelado.",
  "p.director_pack.change_note": "Nota de mudança: {note}",
  "p.director_pack.previous_pack":
    "Comparado com o pacote de {period}, versão {version}.",
  "p.director_pack.download_pdf": "Baixar como PDF",
  "p.director_pack.pdf_note":
    "O PDF tem os mesmos números, mas suas palavras fixas estão em inglês. Para manter outro idioma, imprima esta página.",
  "p.director_pack.basis":
    "Os registros estão como estavam quando o pacote foi redigido, com data igual ou anterior à data de corte. Problemas e solicitações de mudança não guardam histórico de status; por isso um encerrado depois da data de corte mas antes da redação aparece como encerrado. Uma seção sem nada a mostrar diz isso; a ausência nunca é mostrada como zero ou verde.",
  "p.director_pack.s_quality": "Qualidade dos dados",
  "p.director_pack.s_changed": "O que mudou",
  "p.director_pack.s_map": "Mapa do portfólio",
  "p.director_pack.s_exceptions": "Exceções",
  "p.director_pack.s_gates": "Decisões de portão",
  "p.director_pack.s_funding": "Financiamento",
  "p.director_pack.s_fever": "Gráfico de febre da reserva",
  "p.director_pack.s_evm": "Valor agregado",
  "p.director_pack.s_benefits": "Benefícios",
  "p.director_pack.s_capacity": "Capacidade",
  "p.director_pack.s_issues": "Problemas",
  "p.director_pack.s_changes": "Solicitações de mudança",
  "p.director_pack.plans_included": "Planos incluídos: {n}",
  "p.director_pack.plans_left_out":
    "Planos deixados de fora por serem privados ou marcados: {n}",
  "p.director_pack.sections_missing": "Seções ainda não construídas: {list}",
  "p.director_pack.all_sections": "Todas as seções do pacote estão presentes.",
  "p.director_pack.nothing_here": "Nada a mostrar nesta seção.",
  "p.director_pack.changed_issues": "Problemas abertos: {now}, {change}",
  "p.director_pack.changed_changes":
    "Solicitações de mudança aguardando uma decisão: {now}, {change}",
  "p.director_pack.changed_was": "era {before}",
  "p.director_pack.changed_unchanged": "sem alteração",
  "p.director_pack.changed_first": "nenhum pacote anterior para comparar",
  "p.director_pack.c_programme": "Programa",
  "p.director_pack.c_aim": "Objetivo estratégico",
  "p.director_pack.c_status": "Situação",
  "p.director_pack.c_stage": "Estágio",
  "p.director_pack.c_phase": "Fase",
  "p.director_pack.c_confidence": "Confiança de entrega",
  "p.director_pack.c_trend": "Tendência",
  "p.director_pack.self_assessed": "autoavaliado",
  "p.director_pack.stale": "desatualizado",
  "p.director_pack.no_aim": "Sem objetivo estratégico",
  "p.director_pack.c_plan": "Plano",
  "p.director_pack.c_worst": "Pior classificação",
  "p.director_pack.c_need": "Decisão necessária",
  "p.director_pack.c_owner_role": "Responsável (função)",
  "p.director_pack.c_decide_by": "Decidir até",
  "p.director_pack.c_hours_open": "Horas em aberto",
  "p.director_pack.ex_window":
    "Uma exceção que não é completada em {hours} horas após a abertura é violada.",
  "p.director_pack.ex_open": "Abertas: {n}",
  "p.director_pack.ex_breached": "Violadas: {n}",
  "p.director_pack.ex_closed": "Encerradas no período: {n}",
  "p.director_pack.g_passed": "Aprovados: {n}",
  "p.director_pack.g_failed": "Reprovados: {n}",
  "p.director_pack.g_deferred": "Adiados: {n}",
  "p.director_pack.g_paused": "Programas pausados: {n}",
  "p.director_pack.g_stopped": "Programas interrompidos: {n}",
  "p.director_pack.c_gate": "Portão",
  "p.director_pack.c_decided": "Decidido",
  "p.director_pack.c_planned": "Planejado",
  "p.director_pack.c_reason": "Motivo",
  "p.director_pack.c_days_late": "Dias de atraso",
  "p.director_pack.c_since": "Desde",
  "p.director_pack.c_allocs": "Alocações sinalizadas",
  "p.director_pack.reason_hold": "revisado como em espera",
  "p.director_pack.reason_no_review": "a data planejada passou sem uma revisão",
  "p.director_pack.fy_line":
    "Ano fiscal {label}, de {start} a {end}. Capital e despesa corrente são mostrados separadamente, e as moedas nunca são somadas.",
  "p.director_pack.c_source": "Fonte de financiamento",
  "p.director_pack.c_class": "Classe",
  "p.director_pack.c_budget": "Orçamento",
  "p.director_pack.c_spend": "Gasto até a data",
  "p.director_pack.c_variance": "Variação",
  "p.director_pack.no_funding_source": "sem fonte de financiamento",
  "p.director_pack.forecast_line":
    "Previsão de todo o ciclo de vida em {currency}: real {actual}, estimativa no término {eac}",
  "p.director_pack.unfunded_lines":
    "Linhas sem fonte de financiamento: {n}. Linhas sem datas: {dated}.",
  "p.director_pack.c_chain": "Corrente concluída (%)",
  "p.director_pack.c_buffer": "Reserva consumida (%)",
  "p.director_pack.c_zone": "Zona",
  "p.director_pack.c_slip": "Deslizamento (dias)",
  "p.director_pack.c_buffer_days": "Reserva (dias)",
  "p.director_pack.c_previous": "Pacote anterior",
  "p.director_pack.edge_below": "recuperação: desenhado na borda inferior",
  "p.director_pack.edge_above": "estouro: desenhado na borda superior",
  "p.director_pack.fever_none_no_buffer":
    "sem ponto no gráfico de febre: nenhuma reserva do projeto foi declarada",
  "p.director_pack.fever_none_no_baseline":
    "sem ponto no gráfico de febre: nenhuma linha de base de cronograma",
  "p.director_pack.fever_none_no_chain":
    "sem ponto no gráfico de febre: nenhum trabalho crítico com duração para medir",
  "p.director_pack.fever_none_no_forecast":
    "sem ponto no gráfico de febre: o cronograma não pode ser previsto agora",
  "p.director_pack.fever_lines":
    "Linhas de zona (% de reserva consumida em 0 % e em 100 % de corrente concluída): verde a âmbar {ga0} a {ga1}; âmbar a vermelho {ar0} a {ar1}.",
  "p.director_pack.c_bac": "Orçamento no término",
  "p.director_pack.c_ev": "Valor agregado",
  "p.director_pack.c_ac": "Custo real",
  "p.director_pack.c_spi": "SPI",
  "p.director_pack.c_cpi": "CPI",
  "p.director_pack.below_threshold":
    "Abaixo do limite de valor agregado: orçamento no término {bac}",
  "p.director_pack.no_baseline": "sem linha de base",
  "p.director_pack.evm_ratio_note":
    "SPI e CPI são mostrados com duas casas; em branco significa não definido, não zero.",
  "p.director_pack.c_recurrence": "Recorrência",
  "p.director_pack.c_classification": "Classificação",
  "p.director_pack.c_claimed": "Declarado",
  "p.director_pack.c_validated": "Validado",
  "p.director_pack.c_unvalidated": "Declarado, não validado",
  "p.director_pack.benefits_note":
    "Realizado mas não validado é uma declaração, não um benefício validado. Os valores recorrentes são anuais e nunca são somados a valores pontuais.",
  "p.director_pack.c_pool": "Grupo de habilidades",
  "p.director_pack.c_demand": "Demanda (FTE)",
  "p.director_pack.c_supply": "Oferta (FTE)",
  "p.director_pack.c_gap": "Lacuna (FTE)",
  "p.director_pack.c_month": "Mês",
  "p.director_pack.c_reading": "Leitura",
  "p.director_pack.no_pool": "nenhum grupo atribuído",
  "p.director_pack.capacity_note":
    "Cada célula tem uma palavra e um símbolo, além de uma cor. Nenhuma oferta registrada não é capacidade zero.",
  "p.director_pack.partners_short":
    "entrega dos parceiros abaixo do compromisso",
  "p.director_pack.open_issues": "Problemas abertos: {n}",
  "p.director_pack.awaiting":
    "Solicitações de mudança aguardando uma decisão: {n}",
  "p.director_pack.dash": "não registrado",
  "l.enum.pk_conf.green": "verde",
  "l.enum.pk_conf.amber_green": "âmbar/verde",
  "l.enum.pk_conf.amber": "âmbar",
  "l.enum.pk_conf.amber_red": "âmbar/vermelho",
  "l.enum.pk_conf.red": "vermelho",
  "l.enum.pk_conf.not_assessed": "não avaliado",
  "l.enum.pk_trend.up": "melhorando",
  "l.enum.pk_trend.down": "piorando",
  "l.enum.pk_trend.same": "sem alteração",
  "l.enum.pk_trend.new": "recém-avaliado",
  "l.enum.pk_trend.not_assessed": "não avaliado",
  "l.enum.pk_trend.no_earlier_pack": "nenhum pacote anterior",
  "l.enum.pk_status.active": "ativo",
  "l.enum.pk_status.paused": "pausado",
  "l.enum.pk_status.stopped": "interrompido",
  "l.enum.pk_zone.green": "verde",
  "l.enum.pk_zone.amber": "âmbar",
  "l.enum.pk_zone.red": "vermelho",
  "l.enum.pk_heat.shortfall": "déficit",
  "l.enum.pk_heat.tight": "apertado",
  "l.enum.pk_heat.enough": "suficiente",
  "l.enum.pk_heat.no_supply_recorded": "nenhuma oferta registrada",
  "l.enum.pk_heat.no_demand": "sem demanda",
  "l.enum.pk_heat.nothing_recorded": "nada registrado",
  "l.enum.pk_class.capex": "despesa de capital",
  "l.enum.pk_class.opex": "despesa corrente",
  "l.enum.pk_class.all": "capital e despesa corrente combinados",
  "l.enum.pk_recur.recurring": "recorrente",
  "l.enum.pk_recur.one_off": "pontual",
  "l.enum.pk_recur.not_stated": "não informado",
  "l.enum.pk_cls.cash_releasing": "liberador de caixa",
  "l.enum.pk_cls.non_cash_releasing": "não liberador de caixa",
  "l.enum.pk_cls.societal": "social",
  "l.enum.pk_cls.not_stated": "não informado",
  "l.enum.pk_gate.g0_concept": "G0 conceito",
  "l.enum.pk_gate.g1_feasibility": "G1 viabilidade",
  "l.enum.pk_gate.g2_definition": "G2 definição",
  "l.enum.pk_gate.g3_delivery": "G3 entrega",
  "l.enum.pk_gate.g4_launch": "G4 lançamento",
  "l.enum.pk_gate.g5_benefits": "G5 benefícios",
  "p.director_pack.c_currency": "Moeda",
  "p.critical_chain.chain_ppm": "Corrente crítica e reservas — PPM",
  "p.critical_chain.buffers_ppm": "Gestão de reservas — PPM",
  "p.critical_chain.pipeline_ppm": "Tambor e fila de projetos — PPM",
  "p.critical_chain.loading": "Carregando…",
  "p.critical_chain.yes": "sim",
  "p.critical_chain.no": "não",
  "p.critical_chain.zone_green": "verde",
  "p.critical_chain.zone_amber": "âmbar",
  "p.critical_chain.zone_red": "vermelho",
  "p.critical_chain.zone_none": "sem leitura",
  "p.critical_chain.outcome_as_planned": "mantém suas datas",
  "p.critical_chain.outcome_staggered": "começa mais tarde",
  "p.critical_chain.outcome_exceeds_drum": "pede mais do que o tambor tem",
  "p.critical_chain.outcome_no_slot": "nenhum início cabe",
  "p.critical_chain.advice":
    "Isto é uma recomendação. Nada foi alterado, e uma pessoa decide o que fazer.",
  "p.critical_chain.link_plan": "Plano",
  "p.critical_chain.link_buffers": "Gestão de reservas",
  "p.critical_chain.link_chain": "Corrente crítica",
  "p.critical_chain.link_critical_path": "Caminho crítico",
  "p.critical_chain.nav_pipeline": "Tambor e fila de projetos",
  "p.critical_chain.chain_h1": "Corrente crítica e reservas",
  "p.critical_chain.chain_lede":
    "A corrente crítica é a sequência mais longa de tarefas quando o tempo das pessoas é respeitado: ela segue as dependências e as esperas por uma pessoa ocupada em outra tarefa. É mostrada ao lado do caminho crítico, que segue apenas as dependências; os dois nunca são misturados.",
  "p.critical_chain.chain_h2": "A corrente crítica",
  "p.critical_chain.no_chain": "Não há corrente crítica: {reason}",
  "p.critical_chain.reason_no_assignments":
    "ninguém está atribuído a uma tarefa não concluída; por isso não há esperas por pessoas a seguir",
  "p.critical_chain.reason_no_scheduled_tasks": "nenhuma tarefa tem cronograma",
  "p.critical_chain.chain_work": "Dias úteis de trabalho na corrente: {days}",
  "p.critical_chain.chain_tasks_count": "Tarefas na corrente: {tasks}",
  "p.critical_chain.chain_finish":
    "Respeitado o tempo das pessoas, o plano termina em {finish}.",
  "p.critical_chain.chain_delay_none":
    "Isso não é posterior ao término do caminho crítico.",
  "p.critical_chain.chain_delay":
    "Dias úteis após o término do caminho crítico: {days}",
  "p.critical_chain.crosses_yes":
    "A corrente cruza entre caminhos: uma pessoa é compartilhada.",
  "p.critical_chain.crosses_no":
    "A corrente permanece em um só caminho: nenhuma pessoa é compartilhada.",
  "p.critical_chain.tied":
    "Outra corrente é igualmente longa; esta é mostrada porque vem primeiro no plano.",
  "p.critical_chain.timeline_label": "A corrente em uma linha do tempo",
  "p.critical_chain.timeline_caption":
    "Cada barra contínua é uma tarefa da corrente. Uma barra tracejada é a reserva do projeto. A tabela abaixo traz os mesmos números.",
  "p.critical_chain.col_task": "Tarefa",
  "p.critical_chain.col_days": "Dias úteis",
  "p.critical_chain.col_start": "Início",
  "p.critical_chain.col_finish": "Término",
  "p.critical_chain.col_held_by": "Retida por",
  "p.critical_chain.col_on_path": "No caminho crítico",
  "p.critical_chain.held_first": "primeira da corrente",
  "p.critical_chain.held_dep": "uma dependência",
  "p.critical_chain.held_person": "uma pessoa: {person}",
  "p.critical_chain.held_person_hidden": "uma pessoa (oculta)",
  "p.critical_chain.cp_h2": "O caminho crítico ao lado",
  "p.critical_chain.cp_finish": "O caminho crítico termina em {finish}.",
  "p.critical_chain.cp_none": "Não há caminho crítico a mostrar.",
  "p.critical_chain.buffers_h2": "Reservas",
  "p.critical_chain.buffers_lede":
    "As tarefas são agendadas com suas estimativas agressivas; a margem de segurança retirada de cada tarefa é reunida em uma reserva. Os tamanhos são em dias úteis. A regra: {rule}.",
  "p.critical_chain.rule_cut_and_paste":
    "metade da margem de segurança retirada (cortar e colar)",
  "p.critical_chain.rule_root_sum_squares":
    "a raiz quadrada da soma dos quadrados da margem de segurança retirada",
  "p.critical_chain.no_buffers": "Não há reservas a mostrar: {reason}",
  "p.critical_chain.reason_estimates_missing":
    "algumas tarefas da corrente ainda não têm estimativa agressiva e segura. Registre-as pelo serviço de estimativas; ainda não há tela para isso.",
  "p.critical_chain.missing_h3": "Tarefas da corrente sem as duas estimativas",
  "p.critical_chain.project_buffer":
    "Reserva do projeto: {days} dias úteis, protegendo uma corrente que termina em {finish}. Com a reserva, a data é {with}.",
  "p.critical_chain.project_safety":
    "Margem de segurança retirada das tarefas da corrente: {days} dias úteis.",
  "p.critical_chain.project_buffer_bar": "Reserva do projeto",
  "p.critical_chain.feeding_h3": "Reservas de alimentação",
  "p.critical_chain.feeding_none":
    "Nenhuma outra corrente de tarefas se junta à corrente crítica.",
  "p.critical_chain.col_feeding_tasks": "Tarefas alimentadoras",
  "p.critical_chain.col_joins": "Junta-se a",
  "p.critical_chain.col_buffer": "Reserva (dias úteis)",
  "p.critical_chain.col_room": "Folga antes da junção (dias úteis)",
  "p.critical_chain.col_push": "Empurraria a corrente",
  "p.critical_chain.push_yes": "sim, em {days} dias úteis",
  "p.critical_chain.unsized": "sem dimensão: falta uma estimativa",
  "p.critical_chain.resource_h3": "Reservas de recursos (alertas de prontidão)",
  "p.critical_chain.resource_lede":
    "Uma reserva de recursos não é tempo. É um lembrete de que uma pessoa deve estar pronta antes de sua tarefa na corrente começar, com {lead} dias úteis de antecedência.",
  "p.critical_chain.col_person": "Pessoa",
  "p.critical_chain.col_alert_on": "Estar pronta até",
  "p.critical_chain.col_task_starts": "A tarefa começa",
  "p.critical_chain.hidden": "(oculta)",
  "p.critical_chain.likely_h3":
    "Agendadas com a duração provável (sem estimativa agressiva)",
  "p.critical_chain.could_not_load_chain":
    "Não foi possível carregar a corrente crítica",
  "p.critical_chain.buffers_h1": "Gestão de reservas",
  "p.critical_chain.buffers_lede2":
    "Cada reserva é lida em relação ao cronograma atual: quanto da corrente está concluído e quanto da reserva o deslizamento previsto consumiu. Uma reserva no vermelho precisa de uma ação de recuperação, que uma pessoa acorda e registra.",
  "p.critical_chain.no_plan":
    "Nenhum plano de reservas foi congelado para este plano. Congele um quando a corrente e as reservas estiverem acordadas; as leituras posteriores são medidas em relação a ele.",
  "p.critical_chain.freeze_confirm":
    "Li a corrente e as reservas e quero congelá-las como o plano",
  "p.critical_chain.freeze_button": "Congelar o plano de reservas",
  "p.critical_chain.frozen_on":
    "Medido em relação ao plano de reservas congelado em {date}.",
  "p.critical_chain.alerts_h2": "Alertas",
  "p.critical_chain.alerts_none": "Nenhuma reserva está no vermelho.",
  "p.critical_chain.alert_project":
    "A reserva do projeto está no vermelho: {consumed}% usados. Corrente: {chain}.",
  "p.critical_chain.alert_feeding":
    "Uma reserva de alimentação está no vermelho: {consumed}% usados. Corrente: {chain}, juntando-se a {joins}.",
  "p.critical_chain.needs_action":
    "Nenhuma ação de recuperação aberta está registrada para: {buffers}.",
  "p.critical_chain.charts_h2": "Gráficos de febre",
  "p.critical_chain.project_label": "Reserva do projeto",
  "p.critical_chain.feeding_label": "Reserva de alimentação: {chain}",
  "p.critical_chain.axis_x": "Corrente concluída (%)",
  "p.critical_chain.axis_y": "Reserva usada (%)",
  "p.critical_chain.chart_caption":
    "{buffer}: {zone}; {consumed}% da reserva usados; {done}% da corrente concluídos.",
  "p.critical_chain.beyond_edge":
    "O valor real está fora do gráfico; por isso o ponto é desenhado na borda.",
  "p.critical_chain.table_h2": "Os mesmos números em uma tabela",
  "p.critical_chain.col_size": "Reserva (dias úteis)",
  "p.critical_chain.col_done": "Corrente concluída (%)",
  "p.critical_chain.col_used": "Reserva usada (%)",
  "p.critical_chain.col_slip": "Deslizamento previsto (dias úteis)",
  "p.critical_chain.col_zone": "Zona",
  "p.critical_chain.no_point": "Sem leitura: {reason}",
  "p.critical_chain.actions_h2": "Ações de recuperação",
  "p.critical_chain.actions_none":
    "Nenhuma ação de recuperação foi registrada.",
  "p.critical_chain.col_buffer_name": "Reserva",
  "p.critical_chain.col_action": "Ação",
  "p.critical_chain.col_owner": "Responsável",
  "p.critical_chain.col_due": "Vencimento",
  "p.critical_chain.col_status": "Situação",
  "p.critical_chain.status_open": "aberta",
  "p.critical_chain.status_done": "concluída",
  "p.critical_chain.status_dropped": "descartada",
  "p.critical_chain.mark_done": "Marcar como concluída",
  "p.critical_chain.drop_action": "Descartar",
  "p.critical_chain.add_action_h3": "Registrar uma ação de recuperação",
  "p.critical_chain.f_buffer": "Reserva",
  "p.critical_chain.f_action": "O que será feito",
  "p.critical_chain.f_owner": "Responsável (uma referência de pessoa)",
  "p.critical_chain.f_due": "Vencimento (opcional)",
  "p.critical_chain.add_button": "Registrar a ação",
  "p.critical_chain.snap_h2": "Histórico",
  "p.critical_chain.snap_button": "Registrar um instantâneo agora",
  "p.critical_chain.snap_none": "Nenhum instantâneo foi registrado.",
  "p.critical_chain.col_snap_date": "Registrado em",
  "p.critical_chain.could_not_load_buffers":
    "Não foi possível carregar a gestão de reservas",
  "p.critical_chain.could_not_save": "Não foi possível salvar isso",
  "p.critical_chain.pipeline_h1": "Tambor e fila de projetos",
  "p.critical_chain.pipeline_lede":
    "O tambor é o grupo de habilidades de que o portfólio mais depende. Os projetos que precisam dele ao mesmo tempo são escalonados: o serviço propõe inícios posteriores, com uma reserva de capacidade entre projetos, e explica cada um. Ele nunca move uma data.",
  "p.critical_chain.drum_h2": "O tambor",
  "p.critical_chain.drum_named":
    "O tambor é {name}: nomeado pelo escritório de portfólio.",
  "p.critical_chain.drum_overloaded":
    "O tambor é {name}: nenhum grupo foi nomeado, e este tem o maior déficit.",
  "p.critical_chain.no_drum": "Não há tambor: {why}",
  "p.critical_chain.set_h3": "Nomear o tambor",
  "p.critical_chain.pick_pool": "Grupo de habilidades",
  "p.critical_chain.set_button": "Nomeá-lo como o tambor",
  "p.critical_chain.clear_button": "Limpar a nomeação",
  "p.critical_chain.capacity":
    "Capacidade do tambor: {fte} FTE. Reserva de capacidade entre projetos: {days} dias.",
  "p.critical_chain.no_capacity":
    "O tambor não tem oferta registrada na janela; por isso não há nada contra o que escalonar.",
  "p.critical_chain.projects_h2": "Inícios propostos",
  "p.critical_chain.projects_none":
    "Nenhum projeto precisa do tambor nesta janela.",
  "p.critical_chain.pipeline_timeline": "Uso do tambor agora e como proposto",
  "p.critical_chain.pipeline_timeline_caption":
    "Para cada projeto, a barra superior mostra quando ele usa o tambor agora e a barra inferior quando o usaria, conforme proposto. A tabela abaixo traz as mesmas datas.",
  "p.critical_chain.bar_now": "agora",
  "p.critical_chain.bar_proposed": "proposto",
  "p.critical_chain.col_project": "Projeto",
  "p.critical_chain.col_needs": "Precisa do tambor",
  "p.critical_chain.col_peak": "Pico (FTE)",
  "p.critical_chain.col_proposal": "Proposta",
  "p.critical_chain.col_proposed": "Primeiro uso proposto",
  "p.critical_chain.col_delay": "Mais tarde em (dias)",
  "p.critical_chain.col_why": "Por quê",
  "p.critical_chain.could_not_load_pipeline":
    "Não foi possível carregar a fila de projetos",
  "p.start_queue.nav": "Fila de início",
  "p.start_queue.title": "Fila de início — PPM",
  "p.start_queue.h1": "Fila de início",
  "p.start_queue.lede":
    "Programas que pediram para iniciar a entrega e foram recusados, na ordem em que pediram. Um programa é recusado quando o limite de trabalho em andamento está cheio ou quando o grupo de restrição tem oferta insuficiente nos meses seguintes. A ordem nunca muda, e um início ainda pode prosseguir com uma substituição registrada e um motivo.",
  "p.start_queue.loading": "Carregando a fila…",
  "p.start_queue.none": "Nenhum programa está esperando para iniciar.",
  "p.start_queue.col_pos": "Posição",
  "p.start_queue.col_prog": "Programa",
  "p.start_queue.col_asked": "Pedido em",
  "p.start_queue.col_why": "Por que foi recusado",
  "p.start_queue.col_act": "Ação",
  "p.start_queue.limit":
    "Todas as {limit} vagas estão ocupadas: {in_delivery} programas estão em entrega.",
  "p.start_queue.pool":
    "O grupo de restrição {pool} tem oferta insuficiente em: {months}.",
  "p.start_queue.withdraw": "Retirar {name} da fila",
  "p.start_queue.withdraw_short": "Retirar",
  "p.start_queue.could_not_load": "Não foi possível carregar a fila de início",
  "p.start_queue.could_not_withdraw": "Não foi possível retirar da fila",
  "p.table.search": "Filtrar linhas",
  "p.table.export": "Exportar",
  "p.table.find": "Pesquisar",
  "p.table.moved": "{header} movida para a posição {position} de {count}",
  "p.table.copy": "Copiar",
  "p.table.copied": "{rows} linhas copiadas para a área de transferência",
  "p.table.copy_failed": "Não foi possível copiar para a área de transferência",
  "p.table.matches": "{matches} de {total} exibidas",
  "p.table.search_in": "Pesquisar em",
  "p.table.all_columns": "Todas as colunas",
  "p.table.pages": "Páginas",
  "p.table.export_csv": "CSV",
  "p.table.export_tsv": "TSV",
  "p.table.export_excel": "Excel",
  "p.table.exported": "{rows} linhas exportadas para {file}",
  "p.table.columns": "Colunas",
  "p.table.show_column": "Mostrar {header}",
  "p.table.resize": "Redimensionar a coluna {header}",
  "p.table.select_all": "Selecionar todas as linhas",
  "p.table.select_col": "Selecionar",
  "p.table.select_row": "Selecionar {row}",
  "p.table.prev": "Página anterior",
  "p.table.next": "Próxima página",
  "p.table.page_status": "Página {page} de {pages} ({rows} linhas)",
  "p.table.page_announce": "Página {page} de {pages}",
  "p.table.sorted": "{header} ordenada de forma {direction}",
  "p.table.asc": "crescente",
  "p.table.desc": "decrescente",
  "p.table.none": "não ordenada",
  "p.table.filtered": "{matches} de {total} linhas correspondem",
  "p.table.selected": "{n} linhas selecionadas",
  "p.table.empty": "Nada a mostrar.",
  "p.demo.tour_title": "Visita de demonstração — PPM",
  "p.demo.tour_h1": "Visita de demonstração",
  "p.demo.tour_lede":
    "Você entrou como Example Person. Tudo aqui é fictício: a Northgate Transport Authority, suas pessoas, seu dinheiro e suas datas foram inventados para esta demonstração. Escolha qualquer funcionalidade abaixo e use-a: você pode alterar coisas, e nada é real.",
  "p.demo.tour_story_h2": "A história",
  "p.demo.tour_story":
    "A Northgate Transport Authority está modernizando sua rede ferroviária: nova sinalização, estações sem degraus e um depósito digital. Dez planos, oito pessoas, quatro organizações e algumas coisas dando errado de propósito, para que as telas tenham algo a mostrar.",
  "p.demo.tour_open": "Abrir",
  "p.demo.tour_no_plan":
    "O plano North Line não foi encontrado; por isso os links para suas páginas estão ocultos. Execute a carga inicial novamente.",
  "p.demo.tour_toc": "Funcionalidades",
  "p.demo.g_overview": "O portfólio num relance",
  "p.demo.g_delivery": "Entregando um projeto (intertravamento da North Line)",
  "p.demo.g_risk": "Riscos, problemas e mudanças",
  "p.demo.g_money": "Dinheiro",
  "p.demo.g_people": "Pessoas e capacidade",
  "p.demo.g_strategy": "Estratégia e entrada de demandas",
  "p.demo.g_escalation": "Governança e escalonamento",
  "p.demo.g_chain": "Corrente crítica",
  "p.demo.g_tools": "Ferramentas",
  "p.demo.f_kanban_t": "Meu quadro kanban",
  "p.demo.f_kanban_d":
    "Suas próprias tarefas abertas, em colunas por status, de todos os projetos. Mova uma com o mouse ou pelo menu; o status da tarefa muda para todos.",
  "p.demo.f_me_t": "Meu trabalho",
  "p.demo.f_me_d":
    "As mesmas tarefas por data de vencimento no seu próprio fuso horário: em atraso, hoje, esta semana, mais tarde.",
  "p.demo.f_plans_t": "Planos",
  "p.demo.f_plans_d":
    "Dez planos fictícios: um portfólio, três programas e seis projetos da Northgate Transport Authority. Abra qualquer um.",
  "p.demo.f_dashboard_t": "Painel do portfólio",
  "p.demo.f_dashboard_d":
    "Status por tipo e estágio, com contagens em vermelho, âmbar e verde.",
  "p.demo.f_gantt_t": "Gantt",
  "p.demo.f_gantt_d":
    "O cronograma de todos os projetos em uma única linha do tempo.",
  "p.demo.f_milestones_t": "Marcos",
  "p.demo.f_milestones_d":
    "Projeto aprovado, equipamento entregue e entrada em operação de cada projeto.",
  "p.demo.f_calendar_t": "Calendário",
  "p.demo.f_calendar_d": "Marcos e datas de vencimento por mês.",
  "p.demo.f_board_t": "Quadro de tarefas",
  "p.demo.f_board_d":
    "Oito tarefas, do levantamento à entrega, em colunas por status. Uma está bloqueada pela garantia de segurança.",
  "p.demo.f_critical_path_t": "Caminho crítico",
  "p.demo.f_critical_path_d":
    "A cadeia de tarefas dependentes que define a data de término.",
  "p.demo.f_earned_value_t": "Valor agregado",
  "p.demo.f_earned_value_d":
    "Valor planejado, valor agregado e custo, com os índices de cronograma e de custo.",
  "p.demo.f_schedule_health_t": "Saúde do cronograma",
  "p.demo.f_schedule_health_d":
    "Verificações da lógica do plano: vínculos ausentes, tarefas longas, restrições.",
  "p.demo.f_wbs_t": "Estrutura analítica",
  "p.demo.f_wbs_d": "O projeto dividido em pacotes de trabalho.",
  "p.demo.f_risk_t": "Registro de riscos",
  "p.demo.f_risk_d":
    "Três riscos em cada projeto, pontuados por probabilidade e impacto.",
  "p.demo.f_issues_t": "Problemas",
  "p.demo.f_issues_d":
    "Uma rota de cabos bloqueada, uma bancada de testes com reserva duplicada e uma licença atrasada.",
  "p.demo.f_change_requests_t": "Solicitações de mudança",
  "p.demo.f_change_requests_d":
    "Uma solicitação para adiar a entrada em operação em duas semanas, com seu impacto no cronograma e no custo, aguardando o diretor do programa.",
  "p.demo.f_joint_risk_t": "Risco de custo e de cronograma em conjunto",
  "p.demo.f_joint_risk_d":
    "O que os riscos fazem com a data de término e com o custo, em conjunto.",
  "p.demo.f_financials_t": "Finanças",
  "p.demo.f_financials_d":
    "Orçamento, gasto real e variação em todo o portfólio.",
  "p.demo.f_cost_risk_t": "Risco de custo",
  "p.demo.f_cost_risk_d": "A dispersão do custo provável, com o P50 e o P80.",
  "p.demo.f_ledger_t": "Livro de financiamento",
  "p.demo.f_ledger_d":
    "Uma subvenção e uma reserva, rastreadas até as linhas de orçamento que as utilizam. O dinheiro ainda não planejado em uma linha é mostrado como não rastreado.",
  "p.demo.f_costs_t": "Custos de um projeto",
  "p.demo.f_costs_d":
    "Custos de mão de obra a partir das tabelas de taxas, e outros custos por tarefa.",
  "p.demo.f_resources_t": "Recursos",
  "p.demo.f_resources_d":
    "Oito pessoas fictícias, suas funções e em que estão alocadas.",
  "p.demo.f_capacity_t": "Mapa de calor de capacidade",
  "p.demo.f_capacity_d":
    "Oferta e demanda por grupo de habilidades e mês. Os meses em vermelho mostram onde faltam engenheiros de sinalização.",
  "p.demo.f_key_persons_t": "Pessoas-chave",
  "p.demo.f_key_persons_d":
    "Pontos únicos de falha: um engenheiro que ninguém consegue substituir ainda, e uma função com sucessor pronto.",
  "p.demo.f_objectives_t": "Objetivos",
  "p.demo.f_objectives_d":
    "Três objetivos, cada um ligado aos planos que o atendem.",
  "p.demo.f_okrs_t": "Metas e resultados-chave",
  "p.demo.f_okrs_d": "Resultados-chave com acompanhamentos e confiança.",
  "p.demo.f_kpis_t": "Medidas-chave",
  "p.demo.f_kpis_d":
    "Minutos de atraso, satisfação dos passageiros e gasto em relação ao plano, com uma leitura por mês.",
  "p.demo.f_proposals_t": "Entrada de demandas",
  "p.demo.f_proposals_d":
    "Cinco propostas em estágios diferentes, de redigida a aprovada e rejeitada.",
  "p.demo.f_ideas_t": "Ideias",
  "p.demo.f_ideas_d": "Quatro ideias com votos. Converta uma em proposta.",
  "p.demo.f_scenarios_t": "Cenários",
  "p.demo.f_scenarios_d":
    "O que cabe sob um teto de orçamento se o trabalho de sinalização vier primeiro.",
  "p.demo.f_director_packs_t": "Pacotes da diretoria",
  "p.demo.f_director_packs_d":
    "Pacotes executivos e do conselho de setembro, montados a partir dos dados atuais. Abra um para ver todas as seções.",
  "p.demo.f_organizations_t": "Organizações parceiras",
  "p.demo.f_organizations_d":
    "A autoridade, sua empresa de entrega, um parceiro e um fornecedor.",
  "p.demo.f_milestone_signoffs_t": "Aceite de marcos",
  "p.demo.f_milestone_signoffs_d":
    "A entrada em operação exige a assinatura do parceiro e do fornecedor. O fornecedor assinou; o parceiro não, então a entrada em operação foi entregue mas não cumprida.",
  "p.demo.f_cutover_t": "Decisão de virada (go/no-go)",
  "p.demo.f_cutover_d":
    "Três critérios de interrupção, um cumprido, uma cossignatura registrada. O go seria recusado, e a tela diz por quê.",
  "p.demo.f_deployment_moves_t": "Mudanças de implantação",
  "p.demo.f_deployment_moves_d":
    "Uma solicitação para adiar o início da instalação em duas semanas, com o efeito em cascata sobre outras tarefas e seu custo de mão de obra, aguardando aprovação.",
  "p.demo.f_start_queue_t": "Fila de início",
  "p.demo.f_start_queue_d":
    "Dois programas estão em entrega e o limite é dois; por isso o terceiro espera na fila, em ordem.",
  "p.demo.f_critical_chain_t": "Corrente crítica",
  "p.demo.f_critical_chain_d":
    "A corrente que passa pelas tarefas dependentes e pelas pessoas que elas compartilham, com estimativas agressivas e seguras.",
  "p.demo.f_buffers_t": "Gestão de reservas",
  "p.demo.f_buffers_d":
    "O plano de reservas congelado, os gráficos de febre e uma ação de recuperação.",
  "p.demo.f_pipeline_t": "Tambor e fila de projetos",
  "p.demo.f_pipeline_d":
    "O grupo de habilidades de que o portfólio mais depende, e inícios escalonados propostos.",
  "p.demo.f_search_t": "Pesquisa",
  "p.demo.f_search_d":
    "Digite na lupa do cabeçalho. Este link pesquisa por “signal”.",
  "p.demo.f_email_deliveries_t": "Envios de e-mail",
  "p.demo.f_email_deliveries_d":
    "Todo e-mail que o serviço tentou enviar. Nada é enviado a partir da demonstração; por isso esta lista está vazia.",
  "p.demo.f_display_t": "Configurações de exibição",
  "p.demo.f_display_d": "Tamanho do texto, temas, visão simples e idioma.",
  "p.kanban.nav": "Meu kanban",
  "p.kanban.title": "Meu quadro kanban — PPM",
  "p.kanban.h1": "Meu quadro kanban",
  "p.kanban.lede":
    "Suas tarefas abertas em todos os planos, em colunas por status. Mova um cartão com o mouse, ou pressione Enter em um cartão para escolher uma coluna em um menu. Uma mudança altera o status da tarefa para todos.",
  "p.kanban.board_label": "Suas tarefas por status",
  "p.kanban.loading": "Carregando seu quadro…",
  "p.kanban.none": "Você não tem tarefas abertas.",
  "p.kanban.todo": "A fazer",
  "p.kanban.in_progress": "Em andamento",
  "p.kanban.in_review": "Em revisão",
  "p.kanban.blocked": "Bloqueadas",
  "p.kanban.done": "Concluídas",
  "p.kanban.card_count": "{n} cartões",
  "p.kanban.move_button": "Mover {title}",
  "p.kanban.move_menu": "Mover para a coluna",
  "p.kanban.moved": "{title} movida para {column}",
  "p.kanban.due": "vence em {date}",
  "p.kanban.points": "{n} pontos",
  "p.kanban.sign_in": "Entre para ver o seu quadro.",
  "p.kanban.could_not_load": "Não foi possível carregar o seu quadro",
  "p.kanban.could_not_move":
    "Não foi possível mover a tarefa; o quadro está como estava.",
  "p.kanban.hidden":
    "{n} tarefas não são mostradas porque os planos delas não podem ser lidos por você.",
  "p.dashboard.my_home": "Minha página inicial",
  "p.demo.banner":
    "Esta é uma demonstração com dados fictícios. Nada aqui é real.",
  "p.demo.banner_tour": "Fazer a visita",
  "p.demo.signin_h2": "Demonstração",
  "p.demo.signin_lede":
    "Este site é uma demonstração. Entre com example@example.com para ver todas as funcionalidades com dados fictícios. Nenhum e-mail é enviado.",
  "p.demo.signin_button": "Entrar na demonstração",
  "p.demo.signin_wrong": "A demonstração aceita apenas example@example.com.",
  "p.search.title": "Pesquisa — PPM",
  "p.search.h1": "Pesquisar planos",
  "p.search.lede":
    "Planos cujo nome, identificadores, palavras-chave ou etiquetas correspondem à sua pesquisa. Planos que você não pode ler não são listados.",
  "p.search.none_asked": "Digite algo na caixa de pesquisa do cabeçalho.",
  "p.search.searching": "Pesquisando…",
  "p.search.results": "{n} planos correspondem a “{q}”.",
  "p.search.no_results": "Nenhum plano corresponde a “{q}”.",
  "p.search.could_not_search": "Não foi possível pesquisar",
  "p.layout.link_picker": "Páginas",
  "p.layout.search_picker": "Pesquisar planos",
  "p.layout.search_input": "Termos de pesquisa",
  "p.layout.search_submit": "Pesquisar",
  "p.layout.lp_home": "Início",
  "p.layout.lp_plans": "Planos",
  "p.layout.lp_dashboard": "Painel",
  "p.layout.lp_milestones": "Marcos",
  "p.layout.lp_proposals": "Propostas",
  "p.cutover.link": "Decisão de virada (go/no-go)",
  "p.cutover.title": "Decisão de virada (go/no-go) — PPM",
  "p.cutover.h1": "Decisão de virada (go/no-go)",
  "p.cutover.lede":
    "A decisão de virar um sistema crítico para a segurança. Os critérios de interrupção são escritos e publicados primeiro, e depois congelados: alterá-los cria uma nova versão. O go exige todos os critérios cumpridos e todas as cossignaturas necessárias. Um no-go, ou um adiamento por segurança, é uma decisão tomada com base nos critérios; não é atraso.",
  "p.cutover.loading": "Carregando as decisões…",
  "p.cutover.none": "Nenhuma decisão de virada foi redigida.",
  "p.cutover.new_h2": "Redigir uma decisão de virada",
  "p.cutover.new_title": "Título",
  "p.cutover.new_criteria":
    "Critérios de interrupção, um por linha (de 5 a 500 caracteres cada)",
  "p.cutover.draft": "Redigir a decisão",
  "p.cutover.version": "versão {n}",
  "p.cutover.status_draft": "rascunho",
  "p.cutover.status_published": "publicada, critérios congelados",
  "p.cutover.status_open": "aberta para a decisão",
  "p.cutover.status_decided": "decidida",
  "p.cutover.status_superseded": "substituída",
  "p.cutover.outcome_go": "go",
  "p.cutover.outcome_no_go": "no-go",
  "p.cutover.outcome_delay_for_safety": "adiar por segurança",
  "p.cutover.decided_line": "Decidida em {on}: {outcome}. {why}",
  "p.cutover.criteria_h3": "Critérios de interrupção",
  "p.cutover.col_n": "N.º",
  "p.cutover.col_criterion": "Critério",
  "p.cutover.col_met": "Cumprido",
  "p.cutover.col_evidence": "Evidência",
  "p.cutover.col_mark": "Marcar",
  "p.cutover.met": "cumprido",
  "p.cutover.not_met": "não cumprido",
  "p.cutover.evidence": "Evidência",
  "p.cutover.mark_met": "Marcar como cumprido: {name}",
  "p.cutover.mark_unmet": "Marcar como não cumprido: {name}",
  "p.cutover.mark_met_short": "Cumprido",
  "p.cutover.mark_unmet_short": "Não cumprido",
  "p.cutover.cosign_h3": "Cossignaturas",
  "p.cutover.cosign_signed": "Assinaram: {roles}.",
  "p.cutover.cosign_missing": "Ainda necessárias: {roles}.",
  "p.cutover.cosign_none": "nenhuma",
  "p.cutover.cosign_role": "Função com que assinar (seu token deve contê-la)",
  "p.cutover.cosign": "Cossignar",
  "p.cutover.refused_h3": "O go seria recusado porque",
  "p.cutover.publish": "Publicar e congelar os critérios",
  "p.cutover.open": "Abrir a decisão",
  "p.cutover.new_version": "Iniciar uma nova versão para alterar os critérios",
  "p.cutover.decide_h3": "Decidir",
  "p.cutover.outcome": "Resultado",
  "p.cutover.reason": "Motivo",
  "p.cutover.decide": "Registrar a decisão",
  "p.cutover.could_not_load": "Não foi possível carregar as decisões",
  "p.cutover.could_not_save": "Não foi possível salvar",
  "p.cutover.cancel_edit": "Parar de editar",
  "p.cutover.criteria_frozen":
    "Esta decisão foi publicada; por isso seus critérios estão congelados: crie uma nova versão para alterá-los.",
  "p.cutover.criteria_rules":
    "Um critério de interrupção por linha: de 1 a 30, cada um com 5 a 500 caracteres. Podem ser alterados até a decisão ser publicada.",
  "p.cutover.criteria_saved": "Os critérios de interrupção foram salvos.",
  "p.cutover.criteria_unchanged": "Nada mudou; por isso nada foi enviado.",
  "p.cutover.edit_criteria_label": "Critérios de interrupção, um por linha",
  "p.cutover.edit_criteria": "Alterar os critérios de interrupção",
  "p.cutover.problem_count":
    "Uma decisão precisa de 1 a 30 critérios de interrupção.",
  "p.cutover.problem_length":
    "O critério de interrupção {line} deve ter de 5 a 500 caracteres.",
  "p.cutover.save_criteria": "Salvar os critérios de interrupção",
  "p.moves.link": "Mudanças de implantação",
  "p.moves.title": "Mudanças de implantação — PPM",
  "p.moves.h1": "Mudanças de implantação",
  "p.moves.lede":
    "Uma solicitação para mover o início de uma tarefa. Nada muda até que o aprovador a aprove. O efeito em cascata sobre outras tarefas, e o custo de mão de obra dele, foi calculado quando a mudança foi pedida, executando o cronograma com a data movida.",
  "p.moves.loading": "Carregando as mudanças…",
  "p.moves.none": "Nenhuma mudança foi solicitada.",
  "p.moves.request_h2": "Solicitar uma mudança",
  "p.moves.task": "Tarefa",
  "p.moves.new_start": "Novo início",
  "p.moves.reason": "Motivo",
  "p.moves.approver": "Aprovador (person:<id> ou role:<name>, opcional)",
  "p.moves.request": "Solicitar a mudança",
  "p.moves.col_task": "Tarefa",
  "p.moves.col_dates": "Início",
  "p.moves.col_reason": "Motivo",
  "p.moves.col_by": "Pedido por",
  "p.moves.col_approver": "Aprovador",
  "p.moves.col_status": "Situação",
  "p.moves.col_knock": "Efeito em cascata",
  "p.moves.col_act": "Decidir",
  "p.moves.status_requested": "aguardando o aprovador",
  "p.moves.status_approved": "aprovada",
  "p.moves.status_refused": "recusada",
  "p.moves.knock": "{n} outras tarefas se movem; custo de mão de obra {cost}",
  "p.moves.no_cost": "sem alteração no custo de mão de obra",
  "p.moves.decision_reason": "Motivo da decisão",
  "p.moves.approve": "Aprovar",
  "p.moves.refuse": "Recusar",
  "p.moves.approve_for": "Aprovar a mudança de {name}",
  "p.moves.refuse_for": "Recusar a mudança de {name}",
  "p.moves.decided": "{by} em {on}: {why}",
  "p.moves.by_requester_h2": "Custo do efeito em cascata por solicitante",
  "p.moves.requester_row": "{who}: {n} solicitações; mão de obra {cost}",
  "p.moves.could_not_load": "Não foi possível carregar as mudanças",
  "p.moves.could_not_save": "Não foi possível salvar",
  "p.ledger.nav": "Livro de financiamento",
  "p.ledger.title": "Livro de financiamento — PPM",
  "p.ledger.h1": "Livro de financiamento",
  "p.ledger.lede":
    "Cada alocação de financiamento, rastreada até as linhas de orçamento que a utilizam, seus portões, os marcos cumpridos e os benefícios validados. O dinheiro alocado mas não planejado em nenhuma linha que você possa ler é não rastreado e é mostrado com o seu valor.",
  "p.ledger.scope":
    "Criar alocações, mapear códigos de custo e reconciliar uma exportação financeira são feitos pela API; esta tela lê o livro.",
  "p.ledger.loading": "Carregando o livro…",
  "p.ledger.none": "Nenhuma alocação de financiamento foi registrada.",
  "p.ledger.col_source": "Fonte de financiamento",
  "p.ledger.col_ref": "Referência",
  "p.ledger.col_on": "Alocado em",
  "p.ledger.col_amount": "Alocado",
  "p.ledger.col_planned": "Planejado em linhas",
  "p.ledger.col_untraced": "Não rastreado",
  "p.ledger.col_lines": "Linhas",
  "p.ledger.col_act": "Rastrear",
  "p.ledger.trace": "Rastrear {name}",
  "p.ledger.trace_short": "Rastrear",
  "p.ledger.trace_h2": "O que {name} comprou",
  "p.ledger.money":
    "Alocado {allocated}, planejado em linhas {planned}, gasto {spent}, não rastreado {untraced} ({currency}).",
  "p.ledger.gates": "Linhas com portão: {gated}. Liberadas: {released}.",
  "p.ledger.milestones_met": "Marcos cumpridos: {met} de {total}.",
  "p.ledger.benefits": "Benefícios registrados: {n}.",
  "p.ledger.validated": "Validado em {currency}: {amount}",
  "p.ledger.lines_h3": "Linhas de orçamento",
  "p.ledger.line_plan": "Programa",
  "p.ledger.line_desc": "Descrição",
  "p.ledger.line_planned": "Planejado",
  "p.ledger.line_actual": "Gasto",
  "p.ledger.line_gate": "Portão",
  "p.ledger.released": "liberada",
  "p.ledger.held": "retida",
  "p.ledger.no_gate": "sem portão",
  "p.ledger.could_not_load": "Não foi possível carregar o livro",
  "p.ledger.could_not_trace": "Não foi possível rastrear a alocação",
  "p.ledger.a_line":
    "uma linha de orçamento (escolha o plano dela acima para vê-la)",
  "p.ledger.choose_line": "Escolha uma linha",
  "p.ledger.choose_plan_first": "Escolha primeiro um plano acima",
  "p.ledger.choose_plan": "Escolha um plano",
  "p.ledger.choose_source": "Escolha uma fonte de financiamento",
  "p.ledger.code_mapped": "Código de custo mapeado.",
  "p.ledger.codes_heading": "Códigos de custo",
  "p.ledger.codes_says":
    "Cada código de custo financeiro corresponde a uma linha de orçamento, para que uma exportação financeira possa ser comparada com o registro. Mapear um código já mapeado o move.",
  "p.ledger.could_not_load_lines":
    "Não foi possível carregar as linhas de orçamento deste plano.",
  "p.ledger.could_not_reconcile": "Não foi possível reconciliar a exportação.",
  "p.ledger.could_not_record": "A alocação não foi registrada.",
  "p.ledger.could_not_save": "A alteração não foi salva.",
  "p.ledger.f_allocation": "Alocação que ela utiliza",
  "p.ledger.f_amount": "Valor",
  "p.ledger.f_code_line": "Linha de orçamento (do plano acima)",
  "p.ledger.f_code": "Código de custo",
  "p.ledger.f_csv": "Ou cole a exportação (CSV)",
  "p.ledger.f_currency": "Moeda",
  "p.ledger.f_file": "Arquivo de exportação financeira (CSV)",
  "p.ledger.f_on": "Alocado em",
  "p.ledger.f_plan": "Plano",
  "p.ledger.f_reference": "Referência (opcional)",
  "p.ledger.f_source": "Fonte de financiamento",
  "p.ledger.line_detached": "A linha não utiliza mais uma alocação.",
  "p.ledger.line_drawn": "A linha agora utiliza essa alocação.",
  "p.ledger.lines_heading":
    "Linhas de orçamento e as alocações que elas utilizam",
  "p.ledger.lines_says":
    "Uma linha utiliza uma alocação da sua própria fonte de financiamento e moeda; defina primeiro a fonte dela na página Financiamento.",
  "p.ledger.map_code": "Mapear o código de custo",
  "p.ledger.matched_heading": "Códigos de custo correspondentes",
  "p.ledger.matched_line":
    "na exportação {ledger}, no registro {register}, diferença {difference}",
  "p.ledger.mismatch_heading":
    "Linhas em moeda diferente da sua linha de orçamento",
  "p.ledger.mismatch_line":
    "linha em {row}, orçamento em {line}: {amount}, não somado",
  "p.ledger.needs_source": "precisa primeiro de uma fonte de financiamento",
  "p.ledger.no_allocation": "Não utiliza nenhuma alocação",
  "p.ledger.no_codes": "Nenhum código de custo foi mapeado ainda.",
  "p.ledger.no_lines": "Este plano não tem linhas de orçamento.",
  "p.ledger.no_matching_allocation":
    "Ainda não há alocação da fonte e da moeda desta linha",
  "p.ledger.no_source": "sem fonte de financiamento",
  "p.ledger.not_reconciled":
    "Não reconciliado: algo não corresponde. Nada foi gravado.",
  "p.ledger.problem_amount": "Escreva o valor como um número maior que 0.",
  "p.ledger.problem_code_line": "Escolha a linha de orçamento para o código.",
  "p.ledger.problem_code": "Um código de custo tem de 1 a 60 caracteres.",
  "p.ledger.problem_currency": "Escreva a moeda com três letras, como GBP.",
  "p.ledger.problem_date_future":
    "A data da alocação não pode estar no futuro.",
  "p.ledger.problem_date": "Escolha a data em que foi alocado.",
  "p.ledger.problem_export_empty":
    "Escolha primeiro um arquivo ou cole a exportação.",
  "p.ledger.problem_export_header":
    "A primeira linha da exportação deve nomear cost_code, amount_minor e currency.",
  "p.ledger.problem_export_large": "A exportação tem mais de 1 MB: divida-a.",
  "p.ledger.problem_reference":
    "Uma referência pode ter no máximo 120 caracteres.",
  "p.ledger.problem_source": "Escolha uma fonte de financiamento.",
  "p.ledger.reconcile_heading": "Reconciliar uma exportação financeira",
  "p.ledger.reconcile_says":
    "O código de custo de cada linha é procurado no mapa e seus valores são somados ao lado do que o registro mantém como gasto na linha. Nada é gravado.",
  "p.ledger.reconcile": "Reconciliar",
  "p.ledger.reconciled":
    "Reconciliado: cada linha correspondeu a uma linha de orçamento, na sua moeda, sem diferença.",
  "p.ledger.record_heading": "Registrar uma alocação",
  "p.ledger.record_says":
    "Dinheiro que uma fonte de financiamento alocou. As alocações são mantidas como registradas e nunca alteradas.",
  "p.ledger.record": "Registrar a alocação",
  "p.ledger.recorded": "Alocação registrada.",
  "p.ledger.refused_export":
    "Não foi possível ler a exportação: cada linha precisa de um código de custo, um valor inteiro em unidades menores e uma moeda, sem exceder o que o serviço aceita.",
  "p.ledger.refused_line":
    "A linha não pode utilizar essa alocação: elas devem ter a mesma fonte de financiamento e a mesma moeda.",
  "p.ledger.rows_read_one": "{n} linha lida.",
  "p.ledger.rows_read_other": "{n} linhas lidas.",
  "p.ledger.to_funding":
    "Adicione fontes de financiamento na página Financiamento",
  "p.ledger.unmatched_heading":
    "Códigos de custo não mapeados para uma linha que você possa ler",
  "p.signoffs.nav": "Aceite de marcos",
  "p.signoffs.title": "Aceite de marcos — PPM",
  "p.signoffs.h1": "Aceite de marcos",
  "p.signoffs.lede":
    "Alguns marcos só são cumpridos quando as organizações parceiras que eles nomeiam os aceitaram. Entregue é a organização de entrega dizendo que terminou; cumprido é entregue com todos os aceites em vigor. Um aceite retirado volta a contar como pendente.",
  "p.signoffs.loading": "Carregando os aceites…",
  "p.signoffs.none": "Nenhum marco precisa do aceite de um parceiro.",
  "p.signoffs.totals":
    "Precisam de aceite: {n}. Entregues: {delivered}. Cumpridos: {met}. Entregues mas não cumpridos: {gap}. Cumpridos, como parcela dos entregues: {share}.",
  "p.signoffs.no_share": "não medido",
  "p.signoffs.col_milestone": "Marco",
  "p.signoffs.col_due": "Vencimento",
  "p.signoffs.col_state": "Estado",
  "p.signoffs.col_delivered": "Entregue em",
  "p.signoffs.col_met": "Cumprido em",
  "p.signoffs.col_outstanding": "Aguardando",
  "p.signoffs.col_act": "Assinar ou retirar",
  "p.signoffs.state_met": "cumprido",
  "p.signoffs.state_delivered_not_met": "entregue, não cumprido",
  "p.signoffs.state_not_delivered": "não entregue",
  "p.signoffs.organization": "Organização",
  "p.signoffs.reason": "Motivo (para uma retirada)",
  "p.signoffs.sign": "Assinar",
  "p.signoffs.withdraw": "Retirar",
  "p.signoffs.sign_for": "Assinar {name} como a organização escolhida",
  "p.signoffs.withdraw_for": "Retirar o aceite de {name}",
  "p.signoffs.not_built":
    "Quais organizações um marco exige é escolhido na página de governança de cada plano; esta tela mostra e registra os aceites.",
  "p.signoffs.problem_inactive":
    "{name} não está mais ativa; por isso não pode ser exigida: desmarque-a para salvar.",
  "p.signoffs.problem_too_many":
    "Um marco pode exigir no máximo {max} organizações.",
  "p.signoffs.required_cancel": "Cancelar",
  "p.signoffs.required_change_for": "Alterar quem deve assinar {name}",
  "p.signoffs.required_change": "Alterar quem deve assinar",
  "p.signoffs.required_cleared":
    "{name} não precisa mais do aceite de um parceiro.",
  "p.signoffs.required_heading": "Aceite do parceiro",
  "p.signoffs.required_inactive": "{name} (não mais ativa)",
  "p.signoffs.required_legend":
    "Organizações que devem aceitar {name} (no máximo {max})",
  "p.signoffs.required_link": "Registrar um aceite",
  "p.signoffs.required_no_orgs": "Não há organização ativa para escolher.",
  "p.signoffs.required_none": "nenhum aceite de parceiro necessário",
  "p.signoffs.required_save": "Salvar quem deve assinar",
  "p.signoffs.required_saved": "Quem deve assinar {name} foi salvo.",
  "p.signoffs.required_says":
    "Um marco com aceite de parceiro só é cumprido quando todas as organizações abaixo assinaram; entregue e cumprido são datas separadas.",
  "p.signoffs.required_signed": "{name} assinou em {date}",
  "p.signoffs.required_unchanged": "Nada mudou; por isso nada foi enviado.",
  "p.signoffs.required_waiting": "{name} ainda não assinou",
  "p.signoffs.could_not_load": "Não foi possível carregar os aceites",
  "p.signoffs.could_not_save": "Não foi possível registrar o aceite",
  "p.key_persons.nav": "Pessoas-chave",
  "p.key_persons.title": "Pessoas-chave — PPM",
  "p.key_persons.h1": "Pessoas-chave e pontos únicos de falha",
  "p.key_persons.lede":
    "As pessoas e funções que o portfólio não pode perder sem prejuízo, quem assumiria o lugar delas e quão preparadas estão. Somente gerentes de recursos podem ler isto, porque diz respeito a pessoas nomeadas.",
  "p.key_persons.loading": "Carregando…",
  "p.key_persons.none": "Nenhuma pessoa ou função-chave foi registrada.",
  "p.key_persons.not_allowed":
    "Somente gerentes de recursos podem ler as pessoas-chave.",
  "p.key_persons.add_h2": "Registrar uma pessoa ou função-chave",
  "p.key_persons.subject_kind": "Pessoa ou função",
  "p.key_persons.kind_person": "pessoa",
  "p.key_persons.kind_role": "função",
  "p.key_persons.subject": "Nome ou função",
  "p.key_persons.successor": "Sucessor",
  "p.key_persons.readiness": "Preparo do sucessor",
  "p.key_persons.ready_ready": "pronto",
  "p.key_persons.ready_partly": "parcialmente pronto",
  "p.key_persons.ready_none": "não pronto",
  "p.key_persons.note": "Nota",
  "p.key_persons.add": "Registrar",
  "p.key_persons.col_subject": "Pessoa ou função",
  "p.key_persons.col_successor": "Sucessor",
  "p.key_persons.col_ready": "Preparo",
  "p.key_persons.col_note": "Nota",
  "p.key_persons.col_act": "Ação",
  "p.key_persons.remove": "Remover {name}",
  "p.key_persons.remove_short": "Remover",
  "p.key_persons.no_successor": "nenhum sucessor nomeado",
  "p.key_persons.contingent_h2": "Trabalhadores contingentes",
  "p.key_persons.contingent_note":
    "Pessoas marcadas como mão de obra contingente, que podem sair com pouco aviso.",
  "p.key_persons.contingent_none": "Ninguém está marcado como contingente.",
  "p.key_persons.contingent_person": "Referência da pessoa",
  "p.key_persons.contingent_unmark": "Desmarcar {name}",
  "p.key_persons.mark": "Marcar",
  "p.key_persons.unmark": "Desmarcar",
  "p.key_persons.could_not_load": "Não foi possível carregar as pessoas-chave",
  "p.key_persons.could_not_save": "Não foi possível salvar",
  "p.organizations.nav": "Organizações",
  "p.organizations.title": "Organizações — PPM",
  "p.organizations.h1": "Organizações parceiras",
  "p.organizations.lede":
    "As organizações que entregam, patrocinam, fornecem ou são parceiras no portfólio. Um pacote pode ser endereçado a uma organização, um marco pode precisar do aceite dela e a capacidade pode ser comprometida por ela. Desativar uma organização mantém o seu histórico.",
  "p.organizations.loading": "Carregando as organizações…",
  "p.organizations.none": "Nenhuma organização foi adicionada ainda.",
  "p.organizations.add_h2": "Adicionar uma organização",
  "p.organizations.name": "Nome",
  "p.organizations.kind": "Tipo",
  "p.organizations.add": "Adicionar",
  "p.organizations.col_name": "Nome",
  "p.organizations.col_kind": "Tipo",
  "p.organizations.col_state": "Estado",
  "p.organizations.col_audience": "Público do pacote",
  "p.organizations.col_act": "Ação",
  "p.organizations.kind_delivery": "entrega",
  "p.organizations.kind_partner": "parceira",
  "p.organizations.kind_sponsor": "patrocinadora",
  "p.organizations.kind_supplier": "fornecedora",
  "p.organizations.active": "ativa",
  "p.organizations.retired": "desativada",
  "p.organizations.retire": "Desativar {name}",
  "p.organizations.restore": "Restaurar {name}",
  "p.organizations.retire_short": "Desativar",
  "p.organizations.restore_short": "Restaurar",
  "p.organizations.could_not_load": "Não foi possível carregar as organizações",
  "p.organizations.could_not_save": "Não foi possível salvar a organização",
  "p.email_deliveries.nav": "Envios de e-mail",
  "p.email_deliveries.title": "Envios de e-mail — PPM",
  "p.email_deliveries.h1": "Envios de e-mail",
  "p.email_deliveries.lede":
    "Todo e-mail que o serviço tentou enviar, uma linha por destinatário: se foi enviado, se aguarda para tentar de novo ou se falhou definitivamente, quantas tentativas foram feitas e o último erro. Somente administradores podem ler isto, porque contém endereços de e-mail.",
  "p.email_deliveries.loading": "Carregando os envios…",
  "p.email_deliveries.none": "Nenhum e-mail foi enviado ou tentado.",
  "p.email_deliveries.counts":
    "Enviados: {sent}. Aguardando nova tentativa: {retry}. Falhas definitivas: {failed}.",
  "p.email_deliveries.filter": "Mostrar",
  "p.email_deliveries.all": "Todos",
  "p.email_deliveries.col_recipient": "Destinatário",
  "p.email_deliveries.col_subject": "Assunto",
  "p.email_deliveries.col_status": "Situação",
  "p.email_deliveries.col_attempts": "Tentativas",
  "p.email_deliveries.col_error": "Último erro",
  "p.email_deliveries.col_next": "Próxima tentativa",
  "p.email_deliveries.col_sent": "Enviado em",
  "p.email_deliveries.status_queued": "na fila",
  "p.email_deliveries.status_retry": "tentará de novo",
  "p.email_deliveries.status_sent": "enviado",
  "p.email_deliveries.status_failed": "falhou",
  "p.email_deliveries.not_allowed":
    "Somente administradores podem ler os envios de e-mail.",
  "p.email_deliveries.could_not_load":
    "Não foi possível carregar os envios de e-mail",
  "p.director_pack.s_signoffs": "Aceite de marcos",
  "p.director_pack.signoffs_note":
    "Alguns marcos só são cumpridos quando as organizações parceiras que eles nomeiam os aceitaram. Entregue é a organização de entrega dizendo que terminou; cumprido é entregue com todos os aceites em vigor. Um aceite retirado conta como pendente.",
  "p.director_pack.signoffs_totals":
    "Precisam de aceite: {n}. Entregues: {delivered}. Cumpridos: {met}. Entregues mas não cumpridos: {gap}. Cumpridos, como parcela dos entregues: {share}.",
  "p.director_pack.so_met": "cumprido",
  "p.director_pack.so_delivered_not_met": "entregue, não cumprido",
  "p.director_pack.so_not_delivered": "não entregue",
  "p.director_pack.so_no_share": "nenhum entregue ainda",
  "p.director_pack.c_milestone": "Marco",
  "p.director_pack.c_due": "Vencimento",
  "p.director_pack.c_delivered_on": "Entregue em",
  "p.director_pack.c_met_on": "Cumprido em",
  "p.director_pack.c_outstanding": "Aceite pendente de",
  "p.director_pack.c_delivery": "Entrega",
  "p.director_pack.dm_not_stated": "não informado",
  "p.director_pack.dm_central_product": "produto central",
  "p.director_pack.dm_central_platform_local_deployment":
    "plataforma central, implantação local",
  "p.director_pack.dm_local_programme_central_standards":
    "programa local, padrões centrais",
  "p.director_pack.dm_owner": "responsável: {name}",
  "p.director_pack.dm_not_ours": "não entregue pela organização",
  "p.director_pack.s_flow": "Burn-up e fluxo (programas de produto)",
  "p.director_pack.c_scope": "Tarefas criadas",
  "p.director_pack.c_finished": "Tarefas concluídas",
  "p.director_pack.c_open": "Tarefas abertas",
  "p.director_pack.c_throughput":
    "Concluídas por semana (últimas quatro semanas)",
  "p.director_pack.c_burnup": "Concluídas, semana a semana (da mais antiga)",
  "p.director_pack.no_flow":
    "{name}: um programa de produto ainda sem tarefas; por isso sem burn-up nem fluxo.",
  "p.director_pack.s_asks": "Pedidos externos e suas compensações",
  "p.director_pack.asks_note":
    "Um pedido do patrocinador não pode ser aprovado até nomear o que ele desloca. Isto lista os pedidos levantados no período.",
  "p.director_pack.asks_count":
    "Pedidos levantados no período: {n}. Ainda sem compensação nomeada: {without}.",
  "p.director_pack.ask_none": "nenhuma compensação nomeada ainda",
  "p.director_pack.c_ask": "Pedido",
  "p.director_pack.c_tradeoff": "O que ele desloca",
  "p.director_pack.register_line":
    "Programas com registro completo: {complete} de {n}.",
  "p.director_pack.register_enforced":
    "Uma linha de orçamento não é liberada, e nenhuma alocação firme é feita, em um programa com registro incompleto.",
  "p.director_pack.register_not_enforced":
    "Um registro incompleto é um aviso, não um impedimento: o dinheiro ainda pode circular, e o aviso é auditado.",
  "p.director_pack.s_register": "Programas com registro incompleto",
  "p.director_pack.c_score": "Completo",
  "p.director_pack.c_missing": "Faltando",
  "p.director_pack.rf_sro": "responsável sênior",
  "p.director_pack.rf_partners": "parceiros",
  "p.director_pack.rf_stage": "estágio",
  "p.director_pack.rf_baseline": "linha de base do cronograma",
  "p.director_pack.rf_target_date": "data-alvo",
  "p.director_pack.rf_budget": "orçamento",
  "p.director_pack.rf_funding_source": "fonte de financiamento",
  "p.director_pack.rf_confidence": "confiança de entrega",
  "p.director_pack.rf_top_risks": "principais riscos",
  "p.director_pack.rf_dependencies": "dependências",
  "p.director_pack.s_watermelon": "Medidas melancia",
  "p.director_pack.wm_note":
    "Verde por fora, vermelho por dentro. Estas são lidas deste pacote e do pacote emitido anterior como armazenados, não recalculadas. Uma medida sem nada a medir diz por quê; não é zero.",
  "p.director_pack.wm_gap":
    "A confiança autoavaliada e a independente divergem",
  "p.director_pack.wm_late":
    "Descoberta tardia: vermelho ou âmbar/vermelho agora, verde no pacote anterior",
  "p.director_pack.wm_red": "Vermelho sem decisão registrada no período",
  "p.director_pack.wm_signoff":
    "Marcos cumpridos com aceite, entre os entregues",
  "p.director_pack.wm_signoff_of": "{met} cumpridos de {delivered} entregues",
  "p.director_pack.wm_not_measured": "não medido",
  "p.director_pack.c_measure": "Medida",
  "p.director_pack.c_programmes": "Programas, ou por que não foi medido",
  "p.director_pack.wm_count": "Programas: {n}",
  "p.director_pack.s_keypeople": "Pessoas-chave e mão de obra contingente",
  "p.director_pack.kp_note":
    "Os pontos únicos de falha são contados aqui e nunca nomeados; o registro em si é para gerentes de recursos.",
  "p.director_pack.kp_counts":
    "Pontos únicos de falha: {n} ({people} pessoas, {roles} funções). Sem sucessor: {nosucc}. Sucessor pronto: {ready}; parcialmente pronto: {partly}; não pronto: {none}.",
  "p.director_pack.kp_no_people": "ninguém neste grupo",
  "p.director_pack.c_headcount": "Pessoas",
  "p.director_pack.c_contingent": "Contingentes",
  "p.director_pack.c_contingent_share": "Parcela contingente",
  "p.director_pack.s_triage": "Triagem e limites",
  "p.director_pack.tr_note":
    "É mostrada a última triagem de cada programa. Ultrapassar um limite abre uma exceção que exige pausa, reinício, interrupção ou uma substituição com prazo de validade; nada é pausado ou interrompido automaticamente.",
  "p.director_pack.tr_decisions":
    "Decisões neste período: pausa {pause}, reinício {reset}, interrupção {stop}, substituição {override}.",
  "p.director_pack.tr_open": "Exceções de limite aguardando uma decisão: {n}.",
  "p.director_pack.tr_override_line":
    "Substituição em vigor para {programme} até {expiry}.",
  "p.director_pack.tr_finish": "concluir",
  "p.director_pack.tr_fix": "corrigir",
  "p.director_pack.tr_pause": "pausar",
  "p.director_pack.tr_stop": "interromper",
  "p.director_pack.c_outcome": "Triagem",
  "p.director_pack.c_triaged_on": "Triado em",
  "l.enum.pk_conf.threshold": "limite ultrapassado",
  "p.director_pack.s_cutovers": "Decisões de virada",
  "p.director_pack.co_note":
    "Uma virada é decidida com base em critérios de interrupção publicados antes da decisão. Um no-go ou um adiamento por segurança é uma decisão tomada com base nesses critérios e não conta como atraso.",
  "p.director_pack.co_counts":
    "Decididas no período: go {go}, no-go {nogo}, adiar por segurança {delay}.",
  "p.director_pack.co_go": "prosseguir",
  "p.director_pack.co_no_go": "no-go",
  "p.director_pack.co_delay_for_safety": "adiar por segurança",
  "p.director_pack.c_cutover": "Virada",
  "p.director_pack.c_decision": "Decisão",
  "p.director_pack.c_decided_on": "Decidida em",
  "p.director_pack.s_outcomes": "Resultados e desescalonamento",
  "p.director_pack.oc_note":
    "O nível de escalonamento é o que o patrocinador definiu, e nunca é calculado aqui. A sequência abaixo conta trimestres concluídos em que toda medida do painel de indicadores e todo critério são cumpridos; ela é oferecida a quem decide.",
  "p.director_pack.oc_level":
    "Nível de escalonamento: {level}, definido em {on}.",
  "p.director_pack.oc_no_level":
    "Nenhum nível de escalonamento foi registrado.",
  "p.director_pack.oc_run":
    "Trimestres consecutivos cumpridos: {n}. A regra pede {need}.",
  "p.director_pack.oc_rule_yes":
    "A regra está satisfeita. Descer um nível cabe ao patrocinador.",
  "p.director_pack.oc_rule_no": "A regra ainda não está satisfeita.",
  "p.director_pack.oc_met": "cumprido",
  "p.director_pack.oc_not_met": "não cumprido",
  "p.director_pack.oc_missing": "faltando",
  "p.director_pack.c_quarter": "Trimestre",
  "p.director_pack.c_why": "Por quê",
  "p.balance.balance": "Equilíbrio do portfólio",
  "p.balance.balance_ppm": "Equilíbrio do portfólio — PPM",
  "p.balance.intro":
    "Onde o custo planejado, o valor-alvo dos benefícios e o risco aberto de cada portfólio se situam em relação aos demais, e os desequilíbrios que decorrem dos números.",
  "p.balance.loading": "Carregando…",
  "p.balance.could_not_load":
    "Não foi possível carregar o equilíbrio do portfólio",
  "p.balance.none":
    "Nenhum custo, valor de benefício ou risco foi registrado em um portfólio ainda.",
  "p.balance.currency": "Moeda",
  "p.balance.unassigned": "Sem portfólio",
  "p.balance.caption":
    "Cada bolha é um portfólio. Na horizontal: custo planejado em {currency}. Na vertical: valor-alvo dos benefícios em {currency}. Área: exposição ao risco aberto.",
  "p.balance.axis_cost": "Custo planejado ({currency})",
  "p.balance.axis_value": "Valor-alvo dos benefícios ({currency})",
  "p.balance.imbalances": "Desequilíbrios",
  "p.balance.no_imbalance":
    "Nenhuma das regras abaixo sinaliza um desequilíbrio nestes números.",
  "p.balance.rules":
    "Um alerta é emitido quando um portfólio detém {concentration}% ou mais do custo planejado ou do risco aberto (com pelo menos dois portfólios), quando sua parcela do custo planejado ou do risco está {gap} pontos percentuais ou mais acima da sua parcela do valor-alvo, ou quando tem custo planejado e nenhum valor-alvo na moeda escolhida. Um alerta é um convite a examinar, não um veredito.",
  "p.balance.flag_cost_concentration":
    "{name} detém {percent}% do custo planejado.",
  "p.balance.flag_risk_concentration":
    "{name} detém {percent}% da exposição ao risco aberto.",
  "p.balance.flag_cost_ahead_of_value":
    "{name} tem uma parcela do custo planejado bem acima da sua parcela de {percent}% do valor-alvo.",
  "p.balance.flag_risk_ahead_of_value":
    "{name} tem uma parcela do risco aberto bem acima da sua parcela de {percent}% do valor-alvo.",
  "p.balance.flag_no_value":
    "{name} tem custo planejado, mas nenhum valor-alvo registrado em {currency}.",
  "p.balance.table": "Os mesmos números em uma tabela",
  "p.balance.portfolio": "Portfólio",
  "p.balance.cost": "Custo planejado ({currency})",
  "p.balance.value": "Valor-alvo ({currency})",
  "p.balance.risk": "Exposição ao risco aberto",
  "p.balance.cost_share": "Parcela do custo",
  "p.balance.value_share": "Parcela do valor",
  "p.balance.risk_share": "Parcela do risco",
  "p.balance.flags": "Alertas",
  "p.balance.no_flag": "nenhum",
  "p.balance.notes":
    "Agrupado por portfólio, não por categoria nem por objetivo estratégico: o portfólio é o único agrupamento pelo qual custo, valor dos benefícios e risco são todos informados. O custo é o valor planejado nas linhas de orçamento, o valor é a meta dos benefícios registrados e o risco é a pontuação de exposição aberta; dinheiro em moedas diferentes nunca é somado.",
  "p.roadmap.title": "Roteiro — impacto, esforço e Agora / Próximo / Depois",
  "p.roadmap.h1": "Roteiro: impacto, esforço e Agora / Próximo / Depois",
  "p.roadmap.lede":
    "Ideias abertas e propostas ainda em decisão, posicionadas por quanto importam (impacto) e quanto trabalho exigem (esforço). O serviço não guarda impacto nem esforço de uma ideia ou proposta; por isso você avalia cada uma aqui; suas avaliações ficam somente neste navegador e não são compartilhadas nem enviadas ao serviço. Um item sem as duas avaliações não é posicionado.",
  "p.roadmap.loading": "Carregando as ideias e propostas…",
  "p.roadmap.matrix_h2": "Matriz de impacto e esforço",
  "p.roadmap.matrix_caption":
    "Impacto nas linhas, esforço nas colunas. Cada célula lista os itens avaliados assim.",
  "p.roadmap.impact": "Impacto",
  "p.roadmap.effort": "Esforço",
  "p.roadmap.impact_level": "Impacto {level}",
  "p.roadmap.effort_level": "Esforço {level}",
  "p.roadmap.level_low": "baixo",
  "p.roadmap.level_medium": "médio",
  "p.roadmap.level_high": "alto",
  "p.roadmap.rate_h2": "Avaliar os itens",
  "p.roadmap.rate_hint":
    "Esta tabela é o caminho pelo teclado até a matriz: escolha um impacto e um esforço para cada item, e ele se move na matriz e nas listas Agora / Próximo / Depois.",
  "p.roadmap.col_item": "Elemento",
  "p.roadmap.col_type": "Tipo",
  "p.roadmap.col_horizon": "Quando",
  "p.roadmap.kind_idea": "Ideia",
  "p.roadmap.kind_proposal": "Proposta",
  "p.roadmap.not_rated": "Não avaliado",
  "p.roadmap.rate_impact": "Impacto de {title}",
  "p.roadmap.rate_effort": "Esforço de {title}",
  "p.roadmap.no_items": "Não há ideias ou propostas abertas para avaliar.",
  "p.roadmap.horizons_h2": "Agora / Próximo / Depois",
  "p.roadmap.horizon_rule":
    "Agora: o impacto é maior que o esforço. Próximo: o impacto é igual ao esforço e pelo menos médio. Depois: todo o restante avaliado. Ainda não avaliado: sem impacto ou sem esforço escolhido.",
  "p.roadmap.h_now": "Agora",
  "p.roadmap.h_next": "Próximo",
  "p.roadmap.h_later": "Depois",
  "p.roadmap.h_unrated": "Ainda não avaliado",
  "p.roadmap.nothing_here": "Nada aqui.",
  "p.roadmap.see_ideas": "Ver ideias e propostas no roteiro",
  "p.people_impact.title":
    "Impacto sobre as pessoas (do serviço de força de trabalho)",
  "p.people_impact.intro":
    "Apenas contagens, do serviço de força de trabalho (WPM), consultado com o seu próprio login. Nenhuma pessoa é nomeada e nada é somado.",
  "p.people_impact.ask": "Consultar o serviço de força de trabalho",
  "p.people_impact.asking": "Consultando…",
  "p.people_impact.failed":
    "Não foi possível carregar o impacto sobre as pessoas.",
  "p.people_impact.not_known": "Não conhecido.",
  "p.people_impact.source": "Do serviço de força de trabalho, em {date}.",
  "p.people_impact.unknown": "Não conhecido: {reason}",
  "p.people_impact.end_sent":
    "Data de término enviada: {date} (a data-alvo deste plano).",
  "p.people_impact.committed": "Pessoas comprometidas, por grupo",
  "p.people_impact.ending":
    "Contratos por prazo determinado e de prestação de serviço que terminam antes do fim",
  "p.people_impact.in_no_pool": "Em nenhum grupo",
  "p.people_impact.end_not_recorded":
    "Nenhum término registrado (não contado como terminando)",
  "p.people_impact.hidden": "oculto (menos de {floor})",
  "p.people_impact.no_pools": "Nenhum grupo a mostrar.",
  "p.people_impact.part_unknown": "Não conhecido: {reason}",
  "p.people_impact.over": "Grupos-mês sobrecomprometidos",
  "p.people_impact.over_counts":
    "{over} de {considered} grupos-mês estão acima do limite; {unknown} não podem ser informados.",
  "p.people_impact.no_total":
    "Nenhum total é dado: uma pessoa pode contar em mais de um grupo.",
  "l.enum.pk_heat.supply_unknown": "oferta desconhecida",
  "p.capacity_heatmap.nav": "Mapa de calor de capacidade",
  "p.capacity_heatmap.title": "Mapa de calor de capacidade — PPM",
  "p.capacity_heatmap.h1": "Capacidade por grupo de habilidades e mês",
  "p.capacity_heatmap.lede":
    "Cada quadrado compara o trabalho pedido a um grupo de habilidades em um mês com as pessoas disponíveis para ele, em equivalentes de tempo integral (FTE).",
  "p.capacity_heatmap.bridge_note":
    "Um grupo vinculado ao serviço de força de trabalho recebe a sua oferta de lá. O que foi informado aqui para esse grupo é mostrado ao lado e nunca somado a ela. Um mês sem número do serviço de força de trabalho é mostrado como desconhecido, nunca como zero.",
  "p.capacity_heatmap.from": "Primeiro mês",
  "p.capacity_heatmap.months": "Número de meses",
  "p.capacity_heatmap.show": "Mostrar",
  "p.capacity_heatmap.loading": "Carregando o mapa de calor…",
  "p.capacity_heatmap.could_not_load":
    "Não foi possível carregar o mapa de calor de capacidade.",
  "p.capacity_heatmap.none":
    "Nenhum grupo de habilidades e nenhuma demanda a mostrar.",
  "p.capacity_heatmap.caption":
    "Demanda e oferta em equivalentes de tempo integral, por grupo de habilidades e mês",
  "p.capacity_heatmap.col_pool": "Grupo de habilidades",
  "p.capacity_heatmap.linked_to": "Vinculado ao serviço de força de trabalho",
  "p.capacity_heatmap.demand": "Demanda: {fte}",
  "p.capacity_heatmap.supply": "Oferta: {fte}",
  "p.capacity_heatmap.gap": "Lacuna: {fte}",
  "p.capacity_heatmap.unknown": "desconhecido",
  "p.capacity_heatmap.none_recorded": "nenhum registrado",
  "p.capacity_heatmap.source_wpm": "Oferta do serviço de força de trabalho",
  "p.capacity_heatmap.source_here": "Oferta informada aqui",
  "p.capacity_heatmap.peer_live": "Ao vivo, obtido {when}.",
  "p.capacity_heatmap.peer_live_no_time": "Ao vivo.",
  "p.capacity_heatmap.peer_last":
    "Último recebido {age} (obtido {when}); não está ao vivo agora: {reason}",
  "p.capacity_heatmap.peer_hidden":
    "Oculto: o serviço de força de trabalho oculta um mês formado por menos trabalhadores do que o seu mínimo.",
  "p.capacity_heatmap.peer_unknown": "Desconhecido: {reason}",
  "p.capacity_heatmap.no_reason":
    "nenhum número recebido do serviço de força de trabalho",
  "p.capacity_heatmap.entered_here":
    "Informado aqui: {fte} (mostrado ao lado, não somado)",
  "p.capacity_heatmap.entered_here_none": "Informado aqui: nenhum",
  "p.capacity_heatmap.partners_short":
    "os parceiros entregaram menos do que se comprometeram",
  "p.capacity_heatmap.no_pool": "Nenhum grupo atribuído",
  "p.capacity_heatmap.key":
    "▲ déficit · ◆ apertado · ● suficiente · ○ sem demanda · – nenhuma oferta registrada · ? oferta desconhecida. A palavra também é sempre escrita.",
  "p.capacity_heatmap.every_figure": "Todos os números em uma tabela",
  "p.capacity_heatmap.table_label": "Capacidade por grupo de habilidades e mês",
  "p.capacity_heatmap.c_pool": "Grupo de habilidades",
  "p.capacity_heatmap.c_month": "Mês",
  "p.capacity_heatmap.c_reading": "Leitura",
  "p.capacity_heatmap.c_demand": "Demanda (equivalente de tempo integral)",
  "p.capacity_heatmap.c_supply": "Oferta (equivalente de tempo integral)",
  "p.capacity_heatmap.c_source": "Oferta de",
  "p.capacity_heatmap.c_status": "Leitura do serviço de força de trabalho",
  "p.capacity_heatmap.c_entered_here": "Informado aqui",
  "p.capacity_heatmap.see_pools":
    "Vincular grupos de habilidades ao serviço de força de trabalho",
  "p.skill_pools.nav": "Grupos de habilidades",
  "p.skill_pools.title": "Grupos de habilidades — PPM",
  "p.skill_pools.h1": "Grupos de habilidades",
  "p.skill_pools.lede":
    "Um grupo de habilidades reúne as funções e habilidades que podem fazer o mesmo trabalho. Aqui você pode vincular um grupo ao grupo correspondente no serviço de força de trabalho.",
  "p.skill_pools.rule":
    "O vínculo deve ser definido nos dois lados, aqui e no serviço de força de trabalho, por uma pessoa que enxergue ambos. Nunca é feito por nome. Enquanto um grupo está vinculado, sua oferta no mapa de calor de capacidade vem do serviço de força de trabalho e sua demanda é enviada para lá.",
  "p.skill_pools.loading": "Carregando os grupos de habilidades…",
  "p.skill_pools.could_not_load":
    "Não foi possível carregar os grupos de habilidades.",
  "p.skill_pools.none": "Nenhum grupo de habilidades foi criado ainda.",
  "p.skill_pools.col_name": "Nome",
  "p.skill_pools.col_roles": "Funções",
  "p.skill_pools.col_skills": "Habilidades",
  "p.skill_pools.col_state": "Estado",
  "p.skill_pools.col_link": "Grupo do serviço de força de trabalho",
  "p.skill_pools.col_act": "Alterar",
  "p.skill_pools.active": "ativo",
  "p.skill_pools.retired": "desativado",
  "p.skill_pools.not_linked": "não vinculado",
  "p.skill_pools.change_short": "Alterar vínculo",
  "p.skill_pools.change": "Alterar o vínculo de {name}",
  "p.skill_pools.edit_h2": "Vincular {name} ao serviço de força de trabalho",
  "p.skill_pools.ref_label": "ID do grupo no serviço de força de trabalho",
  "p.skill_pools.ref_hint":
    "Copie-o do grupo no serviço de força de trabalho: 36 caracteres, em minúsculas, com hifens, como 3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b.",
  "p.skill_pools.ref_invalid":
    "Esse não é um ID de grupo do serviço de força de trabalho: deve ter 36 caracteres, em minúsculas, com hifens.",
  "p.skill_pools.save": "Salvar o vínculo",
  "p.skill_pools.clear": "Remover o vínculo",
  "p.skill_pools.cancel": "Cancelar",
  "p.skill_pools.confirm_clear":
    "Remover o vínculo entre {name} e o serviço de força de trabalho? A oferta dele no mapa de calor voltará a ser a informada aqui. Todo número de demanda já enviado ao serviço de força de trabalho para este grupo será retirado de lá (enviado como zero).",
  "p.skill_pools.saved":
    "{name} agora está vinculado ao grupo {ref} do serviço de força de trabalho.",
  "p.skill_pools.cleared":
    "{name} não está mais vinculado ao serviço de força de trabalho.",
  "p.skill_pools.taken":
    "Outro grupo aqui já está vinculado a esse grupo do serviço de força de trabalho. Remova primeiro esse vínculo: um grupo aqui para cada grupo lá.",
  "p.skill_pools.invalid":
    "O serviço não aceitou esse ID. Deve ser o ID de grupo do serviço de força de trabalho: 36 caracteres, em minúsculas, com hifens.",
  "p.skill_pools.missing":
    "Esse grupo não existe mais. A lista foi carregada novamente.",
  "p.skill_pools.could_not_save": "Não foi possível alterar o vínculo.",
  "p.skill_pools.see_heatmap": "Ver o mapa de calor de capacidade",
  "p.skill_pools.withdrawing":
    "Números de demanda sendo retirados do grupo do serviço de força de trabalho ao qual estava vinculado: {n}.",
  "p.skill_pools.add_heading": "Adicionar um grupo",
  "p.skill_pools.add_says":
    "Um grupo reúne funções de recursos e habilidades, para que o mapa de calor de capacidade conte sua demanda e oferta uma só vez. Uma função ou habilidade está em no máximo um grupo.",
  "p.skill_pools.add": "Adicionar o grupo",
  "p.skill_pools.added": "Grupo {name} adicionado.",
  "p.skill_pools.choose_pool": "Escolha um grupo",
  "p.skill_pools.clash_item": "{value} (em {pool})",
  "p.skill_pools.conflict_members":
    "Já existe um grupo com esse nome, ou uma destas funções ou habilidades já está em outro grupo.",
  "p.skill_pools.could_not_add": "O grupo não foi adicionado.",
  "p.skill_pools.f_name": "Nome do grupo",
  "p.skill_pools.f_pool": "Grupo",
  "p.skill_pools.f_roles": "Funções",
  "p.skill_pools.f_skills": "Habilidades",
  "p.skill_pools.list_says":
    "Separe as funções ou habilidades com vírgulas ou quebras de linha; uma repetição é mantida uma só vez.",
  "p.skill_pools.members_heading": "Alterar um grupo",
  "p.skill_pools.members_saved":
    "As funções e habilidades de {name} foram salvas.",
  "p.skill_pools.problem_clash": "Já em outro grupo: {list}.",
  "p.skill_pools.problem_name_taken": "Já existe um grupo com esse nome.",
  "p.skill_pools.problem_name": "O nome de um grupo tem de 2 a 120 caracteres.",
  "p.skill_pools.restore": "Restaurar o grupo",
  "p.skill_pools.restored": "Grupo {name} restaurado.",
  "p.skill_pools.retire_says":
    "Um grupo desativado permanece no registro e mantém suas funções e habilidades; restaurá-lo faz com que volte a ser contado.",
  "p.skill_pools.retire": "Desativar o grupo",
  "p.skill_pools.retired_done": "Grupo {name} desativado.",
  "p.skill_pools.save_members": "Salvar as funções e habilidades",
  "p.skill_pools.unchanged": "Nada mudou; por isso nada foi enviado.",
  "p.capacity_heatmap.set_commitment": "Definir um compromisso de capacidade",
  "p.commitments.title": "Compromissos de capacidade — PPM",
  "p.commitments.h1": "Compromissos de capacidade",
  "p.commitments.lede":
    "Registre a capacidade prometida a um programa (oferta) ou o trabalho de que um programa precisa de um grupo de habilidades (demanda), para um mês, em equivalentes de tempo integral.",
  "p.commitments.bridge_note":
    "Para um grupo vinculado ao serviço de força de trabalho, um compromisso de demanda também envia para lá a demanda total do programa para esse grupo e mês. Um total de zero retira o número de lá. Nada é tentado de novo: salve outra vez para enviar de novo.",
  "p.commitments.see_heatmap": "Ver o mapa de calor de capacidade",
  "p.commitments.from_plan": "Compromissos de capacidade deste programa",
  "p.commitments.form_h2": "Definir um compromisso",
  "p.commitments.choose": "Escolha…",
  "p.commitments.pool": "Grupo de habilidades",
  "p.commitments.pool_linked":
    "{name} (vinculado ao serviço de força de trabalho)",
  "p.commitments.pool_hint": "Somente os grupos ativos são listados.",
  "p.commitments.pool_missing": "Escolha um grupo de habilidades.",
  "p.commitments.plan": "Programa",
  "p.commitments.plan_hint":
    "O plano deve ser um programa (um plano do tipo Programa).",
  "p.commitments.plan_missing": "Escolha um programa.",
  "p.commitments.kind": "O que é comprometido",
  "p.commitments.kind_demand":
    "Demanda: trabalho de que o programa precisa do grupo",
  "p.commitments.kind_supply": "Oferta: capacidade prometida ao programa",
  "p.commitments.organization": "Organização parceira (opcional)",
  "p.commitments.organization_hint":
    "Deixe vazio para a sua própria organização.",
  "p.commitments.month": "Mês",
  "p.commitments.month_hint": "Um mês: ano e mês.",
  "p.commitments.month_invalid": "Informe um mês como ano e mês.",
  "p.commitments.committed": "Comprometido (equivalente de tempo integral)",
  "p.commitments.committed_hint":
    "Um número de 0 a 1000, escrito como você escreve números. Zero retira um número de demanda enviado ao serviço de força de trabalho.",
  "p.commitments.delivered":
    "Entregue (equivalente de tempo integral, opcional)",
  "p.commitments.delivered_hint":
    "O que foi efetivamente entregue, se conhecido.",
  "p.commitments.fte_invalid":
    "Um número de 0 a 1000, escrito como você escreve números.",
  "p.commitments.note": "Nota (opcional)",
  "p.commitments.save": "Salvar o compromisso",
  "p.commitments.saved": "Salvo: {pool}, {plan}, {month}.",
  "p.commitments.not_found":
    "Esse programa não foi encontrado, ou você não pode lê-lo.",
  "p.commitments.refused":
    "O serviço não aceitou. O plano deve ser um programa, o grupo deve estar ativo e cada número deve ficar entre 0 e 1000.",
  "p.commitments.could_not_save": "Não foi possível salvar o compromisso.",
  "p.commitments.could_not_load":
    "Não foi possível carregar os compromissos de capacidade.",
  "p.commitments.loading": "Carregando os envios recentes…",
  "p.commitments.sends_h2": "Envios recentes ao serviço de força de trabalho",
  "p.commitments.sends_note":
    "O último envio de cada programa, grupo e mês, do mais recente ao mais antigo. Um envio iniciado mostra aqui a sua resposta quando ela chega: atualize para vê-la.",
  "p.commitments.refresh": "Atualizar",
  "p.commitments.no_sends":
    "Nada foi enviado ao serviço de força de trabalho ainda.",
  "p.commitments.c_pool": "Grupo de habilidades",
  "p.commitments.c_programme": "Programa",
  "p.commitments.c_month": "Mês",
  "p.commitments.c_sent": "Enviado",
  "p.commitments.c_outcome": "O que aconteceu",
  "p.commitments.c_at": "Quando",
  "p.commitments.sent_fte": "{fte} equivalente de tempo integral",
  "p.commitments.sent_withdrawal": "uma retirada (zero)",
  "p.commitments.sent_nothing": "nada",
  "p.commitments.out_not_sent_at_all":
    "Este compromisso não é enviado ao serviço de força de trabalho: somente a demanda de um grupo vinculado a ele é.",
  "p.commitments.out_none_yet":
    "Nada foi enviado ao serviço de força de trabalho para este programa, grupo e mês ainda.",
  "p.commitments.out_started":
    "Enviando {fte} equivalente de tempo integral (o total do programa para este grupo e mês) ao serviço de força de trabalho. A resposta aparece em Envios recentes.",
  "p.commitments.out_started_zero":
    "Retirando do serviço de força de trabalho o número deste programa para este grupo e mês, porque o total agora é zero. A resposta aparece em Envios recentes.",
  "p.commitments.out_sent":
    "O serviço de força de trabalho recebeu {fte} equivalente de tempo integral.",
  "p.commitments.out_sent_zero":
    "O serviço de força de trabalho retirou o seu número para este programa, grupo e mês.",
  "p.commitments.out_duplicate":
    "O serviço de força de trabalho já tinha este número; nada mudou lá.",
  "p.commitments.out_refused":
    "O serviço de força de trabalho recusou. Nada é tentado de novo; salve outra vez para enviar de novo.",
  "p.commitments.out_unreachable":
    "Não foi possível alcançar o serviço de força de trabalho, ou ele não respondeu a tempo. Nada é tentado de novo; salve outra vez para enviar de novo.",
  "p.commitments.out_not_configured":
    "Não enviado: este serviço não está configurado para alcançar o serviço de força de trabalho.",
  "p.commitments.out_no_token":
    "Não enviado: não havia login para repassar ao serviço de força de trabalho.",
  "p.commitments.out_too_large":
    "Não enviado: o serviço de força de trabalho aceita no máximo 10 000 equivalentes de tempo integral.",
  "p.commitments.out_old_zero":
    "Não enviado: um zero registrado antes de os zeros serem enviados; por isso o serviço de força de trabalho manteve o último número. Salve outra vez para retirá-lo.",
  "p.commitments.out_other": "O serviço registrou: {outcome}.",
  "p.discussion.heading": "Discussão",
  "p.discussion.loading": "Carregando…",
  "p.discussion.could_not_load_comments":
    "Não foi possível carregar os comentários.",
  "p.discussion.could_not_load_reactions":
    "Não foi possível carregar as reações.",
  "p.discussion.could_not_load_files": "Não foi possível carregar os arquivos.",
  "p.discussion.could_not_load_watch":
    "Não foi possível ler se você segue isto.",
  "p.discussion.sign_in_to_follow":
    "Entre para seguir isto e ser avisado de novos comentários e alterações.",
  "p.discussion.you_follow":
    "Você segue isto: é avisado de novos comentários e alterações.",
  "p.discussion.you_do_not_follow": "Você não segue isto.",
  "p.discussion.followers_one": "{n} pessoa segue.",
  "p.discussion.followers_other": "{n} pessoas seguem.",
  "p.discussion.follow": "Seguir",
  "p.discussion.unfollow": "Deixar de seguir",
  "p.discussion.followed": "Agora você segue isto.",
  "p.discussion.unfollowed": "Você não segue mais isto.",
  "p.discussion.reactions_label": "Reações",
  "p.discussion.reaction_named_one": "{reaction}: {n} pessoa",
  "p.discussion.reaction_named_other": "{reaction}: {n} pessoas",
  "p.discussion.r_thumbs_up": "Joinha",
  "p.discussion.r_heart": "Coração",
  "p.discussion.r_celebrate": "Comemorar",
  "p.discussion.r_eyes": "Dando uma olhada",
  "p.discussion.r_question": "Dúvida",
  "p.discussion.comments_heading": "Comentários",
  "p.discussion.no_comments": "Ainda não há comentários.",
  "p.discussion.no_author": "alguém sem login",
  "p.discussion.by_at": "Por {who}, {when}",
  "p.discussion.edited_at": "editado {when}",
  "p.discussion.edit": "Editar",
  "p.discussion.edit_named": "Editar o comentário de {who}, {when}",
  "p.discussion.delete": "Excluir",
  "p.discussion.delete_named": "Excluir o comentário de {who}, {when}",
  "p.discussion.confirm_delete_comment":
    "Excluir este comentário? As versões anteriores são mantidas, mas ninguém mais o verá na página.",
  "p.discussion.earlier": "Versões anteriores",
  "p.discussion.no_earlier": "Nenhuma versão anterior é mantida.",
  "p.discussion.replaced_at": "Substituído {when}",
  "p.discussion.f_edit": "Alterar o comentário",
  "p.discussion.save_edit": "Salvar o comentário",
  "p.discussion.cancel": "Cancelar",
  "p.discussion.f_comment": "Seu comentário",
  "p.discussion.chars_left": "Restam {n} de {max} caracteres.",
  "p.discussion.chars_over": "{n} caracteres acima do limite de {max}.",
  "p.discussion.post": "Publicar o comentário",
  "p.discussion.posted": "O comentário foi publicado.",
  "p.discussion.edited":
    "O comentário foi alterado; a versão anterior é mantida.",
  "p.discussion.deleted": "O comentário foi excluído.",
  "p.discussion.problem_blank": "Escreva primeiro o comentário.",
  "p.discussion.problem_too_long":
    "Um comentário pode ter no máximo 4000 caracteres.",
  "p.discussion.files_heading": "Arquivos",
  "p.discussion.files_intro":
    "Os arquivos anexados aqui são salvos no seu computador quando você os abre; nunca são exibidos dentro desta página.",
  "p.discussion.no_files": "Nenhum arquivo anexado.",
  "p.discussion.file_facts": "{type}, {size}, adicionado por {who}, {when}",
  "p.discussion.remove_file": "Remover",
  "p.discussion.remove_file_named": "Remover o arquivo {name}",
  "p.discussion.confirm_remove_file":
    "Remover o arquivo {name}? Ninguém poderá baixá-lo daqui novamente.",
  "p.discussion.removed_file": "O arquivo {name} foi removido.",
  "p.discussion.f_file": "Arquivo a anexar",
  "p.discussion.files_rules":
    "Um PDF, uma imagem (PNG, JPEG, GIF ou WebP), texto simples, CSV, Markdown, JSON, ou um arquivo Word, Excel ou PowerPoint, de no máximo 10 MB. Páginas da web, desenhos SVG, arquivos compactados e programas são recusados.",
  "p.discussion.attach": "Anexar o arquivo",
  "p.discussion.attached": "O arquivo {name} foi anexado.",
  "p.discussion.problem_choose_file": "Escolha um arquivo primeiro.",
  "p.discussion.problem_empty": "Esse arquivo está vazio.",
  "p.discussion.problem_too_large":
    "Esse arquivo tem mais de 10 MB, o limite para um arquivo.",
  "p.discussion.problem_type":
    "Esse tipo de arquivo não pode ser anexado. A lista abaixo indica quais tipos podem.",
  "p.discussion.type_pdf": "documento PDF",
  "p.discussion.type_png": "imagem PNG",
  "p.discussion.type_jpeg": "imagem JPEG",
  "p.discussion.type_gif": "imagem GIF",
  "p.discussion.type_webp": "imagem WebP",
  "p.discussion.type_text": "texto simples",
  "p.discussion.type_csv": "tabela CSV",
  "p.discussion.type_markdown": "texto Markdown",
  "p.discussion.type_json": "dados JSON",
  "p.discussion.type_word": "documento Word",
  "p.discussion.type_excel": "pasta de trabalho Excel",
  "p.discussion.type_powerpoint": "slides PowerPoint",
  "p.discussion.size_b": "{n} bytes",
  "p.discussion.size_kb": "{n} KB",
  "p.discussion.size_mb": "{n} MB",
  "p.discussion.refused_sign_in":
    "Faça login primeiro: o serviço não sabe quem você é.",
  "p.discussion.refused_not_author":
    "Somente quem escreveu um comentário pode alterá-lo ou excluí-lo.",
  "p.discussion.refused_not_uploader":
    "Somente quem anexou um arquivo, ou um administrador, pode removê-lo.",
  "p.discussion.refused_forbidden": "Você não pode ler nem alterar isto.",
  "p.discussion.refused_comment_gone":
    "Esse comentário não existe mais; os comentários foram lidos novamente.",
  "p.discussion.refused_file_gone":
    "Esse arquivo já havia sido removido; os arquivos foram lidos novamente.",
  "p.discussion.refused_gone":
    "Não foi possível encontrar isto: pode ter sido removido, ou você pode não ter permissão para vê-lo.",
  "p.discussion.refused_comment_invalid":
    "O serviço não aceitou o comentário: ele não pode estar em branco e pode ter no máximo 4000 caracteres.",
  "p.discussion.refused_file_invalid":
    "O serviço não aceitou o arquivo: o tipo, o tamanho ou o conteúdo não passou nas verificações (um arquivo deve ser o que o seu tipo diz, e nenhum programa é aceito).",
  "p.discussion.refused_too_large":
    "O arquivo é grande demais para ser enviado.",
  "p.discussion.refused_scan_unavailable":
    "O arquivo não foi anexado: o verificador de malware não respondeu. Tente novamente mais tarde.",
  "p.me.timesheet_link": "Minha folha de horas",
  "p.me.timesheet_note":
    "— seu tempo por dia e por plano, semana a semana, e o envio de uma semana para aprovação.",
  "p.timesheet.title_ppm": "Minha folha de horas — PPM",
  "p.timesheet.heading": "Minha folha de horas",
  "p.timesheet.back": "← Meu trabalho",
  "p.timesheet.intro":
    "Seu tempo registrado em uma semana, por dia e por plano, em horas e minutos, com o tempo faturável e o não faturável sempre mostrados separadamente. Envie uma semana concluída para aprovação; depois de aprovada, ela é definitiva e nenhum outro tempo pode ser registrado nela.",
  "p.timesheet.loading": "Carregando…",
  "p.timesheet.week_nav": "Escolher a semana",
  "p.timesheet.previous_week": "Semana anterior",
  "p.timesheet.this_week": "Esta semana",
  "p.timesheet.next_week": "Próxima semana",
  "p.timesheet.f_week": "Semana, por exemplo 2026-W46",
  "p.timesheet.show_week": "Mostrar a semana",
  "p.timesheet.week_hint":
    "As semanas vão de segunda a domingo e são numeradas pela norma internacional, qualquer que seja o seu primeiro dia da semana. Hoje é {today} no seu fuso horário, {tz}.",
  "p.timesheet.problem_week":
    "Isso não é uma semana: escreva o ano, ‘-W’ e o número da semana, por exemplo 2026-W46.",
  "p.timesheet.mine_heading": "Sua semana {week}",
  "p.timesheet.mine_caption": "Seu tempo na semana {week}, por plano e por dia",
  "p.timesheet.status_open": "Aberta: ainda não enviada para aprovação",
  "p.timesheet.status_submitted":
    "Enviada para aprovação: aguardando uma decisão",
  "p.timesheet.status_approved":
    "Aprovada: definitiva e bloqueada para novo tempo",
  "p.timesheet.status_rejected": "Devolvida: altere e envie novamente",
  "p.timesheet.rejected_because": "o aprovador disse: {comment}",
  "p.timesheet.decision_comment": "com a observação: {comment}",
  "p.timesheet.no_entries":
    "Nenhum tempo está registrado nesta semana, portanto não há total.",
  "p.timesheet.col_plan": "Nome do plano",
  "p.timesheet.col_total": "Tempo total",
  "p.timesheet.col_billable": "Faturável",
  "p.timesheet.col_non_billable": "Não faturável",
  "p.timesheet.row_day_totals": "Cada dia",
  "p.timesheet.plan_unnamed": "Plano {id}… (nome não exibido)",
  "p.timesheet.split_line":
    "Faturável {billable}, não faturável {non_billable}: {total} no total.",
  "p.timesheet.omitted":
    "Lançamentos omitidos por estarem em planos que você não pode ler: {n}.",
  "p.timesheet.hours_one": "{n} h",
  "p.timesheet.hours_other": "{n} h",
  "p.timesheet.minutes_one": "{n} min",
  "p.timesheet.minutes_other": "{n} min",
  "p.timesheet.duration_hm": "{h} {m}",
  "p.timesheet.submit": "Enviar esta semana para aprovação",
  "p.timesheet.submit_again": "Enviar esta semana para aprovação novamente",
  "p.timesheet.submitted_notice": "A semana {week} foi enviada para aprovação.",
  "p.timesheet.nothing_to_submit":
    "Uma semana sem tempo registrado não pode ser enviada para aprovação.",
  "p.timesheet.waiting": "Esta semana aguarda a decisão de um aprovador.",
  "p.timesheet.locked":
    "Uma semana aprovada é definitiva: nenhum outro tempo pode ser registrado nela.",
  "p.timesheet.could_not_load": "Não foi possível carregar a sua semana.",
  "p.timesheet.could_not_submit":
    "Não foi possível enviar a semana para aprovação.",
  "p.timesheet.other_heading": "A semana de outra pessoa, para aprovadores",
  "p.timesheet.other_intro":
    "Se você pode aprovar folhas de horas, informe a referência da pessoa para ler a semana dela, referente à semana escolhida acima, e então aprove-a ou devolva-a. Ninguém decide sobre a própria semana, devolver uma exige um motivo e uma aprovação é definitiva.",
  "p.timesheet.f_person": "Pessoa: ‘person:’ ou ‘worker:’ e o seu id",
  "p.timesheet.show_person": "Mostrar a semana dela",
  "p.timesheet.problem_person":
    "Informe a pessoa como ‘person:’ ou ‘worker:’ seguido do seu id, sem mais nada.",
  "p.timesheet.other_week_heading": "Semana {week} de {person}",
  "p.timesheet.other_caption":
    "Tempo de {person} na semana {week}, por plano e por dia",
  "p.timesheet.decision_legend": "Sua decisão",
  "p.timesheet.approve": "Aprovar",
  "p.timesheet.reject": "Devolver",
  "p.timesheet.f_comment": "Observação, se quiser",
  "p.timesheet.f_comment_required": "Por que é devolvida (obrigatório)",
  "p.timesheet.approve_final":
    "Uma aprovação é definitiva: a semana fica bloqueada e ninguém pode devolvê-la.",
  "p.timesheet.record_decision": "Registrar a decisão",
  "p.timesheet.approved_notice": "A semana {week} de {person} foi aprovada.",
  "p.timesheet.rejected_notice": "A semana {week} de {person} foi devolvida.",
  "p.timesheet.other_not_submitted":
    "Esta semana não foi enviada para aprovação, portanto ainda não há nada a decidir.",
  "p.timesheet.other_rejected":
    "Esta semana foi devolvida; ela poderá ser decidida novamente quando a pessoa a enviar de novo.",
  "p.timesheet.problem_own_week":
    "Esta é a sua própria semana: ninguém decide sobre a própria folha de horas.",
  "p.timesheet.problem_comment": "Diga por que a semana é devolvida.",
  "p.timesheet.could_not_load_other":
    "Não foi possível carregar a semana dessa pessoa.",
  "p.timesheet.could_not_decide": "Não foi possível registrar a decisão.",
  "p.timesheet.refused_read_forbidden":
    "Somente a própria pessoa ou um aprovador de folhas de horas pode ler essa semana.",
  "p.timesheet.refused_submit_forbidden":
    "Somente a pessoa a quem a semana pertence pode enviá-la para aprovação.",
  "p.timesheet.refused_decide_forbidden":
    "Você não pode decidir sobre esta semana: é preciso ter o direito de aprovador de folhas de horas, e ninguém decide sobre a própria semana.",
  "p.timesheet.refused_submit_conflict":
    "Esta semana já foi enviada para aprovação ou aprovada; ela foi lida novamente.",
  "p.timesheet.refused_decide_conflict":
    "Esta semana não aguarda mais uma decisão; ela foi lida novamente.",
  "p.timesheet.refused_not_addressable":
    "Sua identidade de login não tem referência de pessoa, portanto não há folha de horas a mostrar para ela.",
  "p.timesheet.refused_read_invalid":
    "O serviço não aceitou essa semana ou essa pessoa.",
  "p.timesheet.refused_submit_invalid":
    "A semana não foi enviada: não há tempo registrado nela.",
  "p.timesheet.refused_decide_invalid":
    "O serviço não aceitou a decisão: devolver uma semana exige um motivo.",
  "p.security.access_link": "Revisão de acessos e acesso de emergência",
  "p.security.access_note":
    "— quem tem qual acesso, para recertificação, e o acesso de emergência com prazo limitado a um plano privado, que é registrado e auditado.",
  "p.access.title_ppm": "Revisão de acessos e acesso de emergência — PPM",
  "p.access.back": "← Segurança",
  "p.access.heading": "Revisão de acessos e acesso de emergência",
  "p.access.intro":
    "A revisão de acessos lista os acessos que as pessoas registraram como concedidos, para recertificação. O acesso de emergência abre um plano privado para você por um tempo limitado quando não há outro modo; cada uso é registrado e auditado.",
  "p.access.review_heading": "Revisão de acessos",
  "p.access.review_intro":
    "Os acessos registrados como concedidos no livro de acessos e não revogados desde então, do mais antigo ao mais recente, com quem os concedeu e quando. Mostra o que as pessoas registraram, não o que o serviço de login aplica: compare os dois. Somente para administradores; cada leitura é registrada na trilha de auditoria, portanto a revisão só é lida quando você solicita.",
  "p.access.f_review_plan":
    "Id do plano (deixe em branco para todos os planos que você pode ler)",
  "p.access.f_review_days":
    "Pendente de revisão após este número de dias corridos",
  "p.access.show_review": "Mostrar a revisão",
  "p.access.problem_review_plan":
    "Isso não é um id de plano: cole o id do plano ou deixe o campo em branco para todos os planos.",
  "p.access.problem_review_days":
    "Escreva o número de dias corridos como um número inteiro, 0 ou mais.",
  "p.access.could_not_load_review":
    "Não foi possível carregar a revisão de acessos.",
  "p.access.review_totals":
    "Em {as_of}. Concessões vigentes: {standing}. Pendentes de revisão, com {days} dias corridos ou mais: {due}.",
  "p.access.review_note":
    "A idade é contada em dias corridos, do dia em que a concessão foi registrada até o dia de hoje da implantação.",
  "p.access.download_review":
    "Baixar a revisão como planilha, com uma coluna para suas decisões de manter ou revogar",
  "p.access.review_caption":
    "Acessos vigentes hoje, do mais antigo ao mais recente",
  "p.access.review_empty":
    "Nenhum acesso registrado como concedido está vigente nos planos que você pode ler.",
  "p.access.col_person": "Pessoa",
  "p.access.col_access": "Acesso",
  "p.access.col_plan": "Nome do plano",
  "p.access.col_granted_on": "Concedido em",
  "p.access.col_granted_by": "Concedido por",
  "p.access.col_age": "Idade em dias corridos",
  "p.access.col_review": "Revisão",
  "p.access.col_note": "Observação",
  "p.access.due": "Pendente de revisão",
  "p.access.not_due": "Ainda não pendente",
  "p.access.plan_unnamed": "Plano {id}… (nome não exibido)",
  "p.access.bg_heading":
    "Acesso de emergência a um plano privado (break-glass)",
  "p.access.bg_audited_heading":
    "Cada uso do acesso de emergência é registrado e auditado.",
  "p.access.bg_audited_open":
    "Abrir um plano registra na trilha de auditoria quem você é, o plano, o seu motivo, por quanto tempo e a pessoa que você indicar.",
  "p.access.bg_audited_told":
    "A pessoa que você indicar é avisada imediatamente. Se ela não puder ser avisada, o plano não é aberto.",
  "p.access.bg_audited_each":
    "Cada requisição que somente este acesso permite é registrada na trilha de auditoria como acesso de emergência.",
  "p.access.bg_audited_visibility":
    "Ele permite que você veja o plano. O que você pode alterar continua sendo decidido pelos seus próprios direitos.",
  "p.access.bg_audited_ends":
    "Termina sozinho no horário indicado, ou antes, se você o encerrar. No máximo cinco por dia, e um de cada vez em cada plano.",
  "p.access.bg_intro":
    "Use-o somente quando os membros do plano não puderem ser contatados e o trabalho não puder esperar. Exige a elegibilidade break-glass ou o acesso de administrador; o serviço decide.",
  "p.access.problems_heading": "O plano não foi aberto:",
  "p.access.problem_plan": "Informe o id do plano privado.",
  "p.access.problem_reason_short":
    "Diga por que você precisa do acesso, com pelo menos {reason_min} caracteres.",
  "p.access.problem_reason_long":
    "O motivo tem mais de {reason_max} caracteres.",
  "p.access.problem_minutes":
    "Informe a duração em minutos inteiros, de {minutes_min} a {minutes_max}.",
  "p.access.problem_notify":
    "Indique a pessoa a avisar como ‘person:’ ou ‘worker:’ seguido do seu id, sem mais nada.",
  "p.access.problem_acknowledged":
    "Marque a caixa para dizer que você entende que isto é registrado e auditado e que a pessoa é avisada.",
  "p.access.could_not_open": "Não foi possível abrir o plano.",
  "p.access.opened_notice":
    "O plano está aberto para você até {until} (seu fuso horário, {tz}). {person} foi avisado(a).",
  "p.access.go_to_plan": "Ir para o plano",
  "p.access.f_plan": "Id do plano privado",
  "p.access.f_reason":
    "Por que você precisa do acesso (obrigatório, pelo menos {min} caracteres)",
  "p.access.reason_count": "{n} caracteres; no mínimo {min} e no máximo {max}.",
  "p.access.f_minutes": "Por quanto tempo, em minutos ({min} a {max})",
  "p.access.f_notify":
    "Pessoa a avisar: ‘person:’ ou ‘worker:’ e o seu id (não você)",
  "p.access.f_acknowledge":
    "Entendo que isto é registrado e auditado e que a pessoa que indico será avisada.",
  "p.access.open": "Abrir o plano com acesso de emergência",
  "p.access.grants_heading": "Concessões de acesso de emergência",
  "p.access.grants_intro":
    "As suas, das mais recentes às mais antigas; um administrador vê as de todos. Uma concessão nunca é excluída. Os horários estão no seu fuso horário, {tz}.",
  "p.access.grants_caption":
    "Concessões de acesso de emergência, das mais recentes às mais antigas",
  "p.access.grants_empty": "Nenhum acesso de emergência foi aberto.",
  "p.access.loading": "Carregando…",
  "p.access.could_not_load_grants":
    "Não foi possível carregar as concessões de acesso de emergência.",
  "p.access.could_not_revoke":
    "Não foi possível encerrar o acesso de emergência.",
  "p.access.revoked_notice": "O acesso de emergência a {plan} foi encerrado.",
  "p.access.revoke": "Encerrar agora",
  "p.access.revoke_for": "Encerrar agora: acesso de emergência a {plan}",
  "p.access.state_live": "Em vigor",
  "p.access.state_revoked": "Encerrado antes do prazo",
  "p.access.state_expired": "Expirado",
  "p.access.ended_by": "{when}, encerrado por {who}",
  "p.access.col_state": "Estado",
  "p.access.col_holder": "Mantido por",
  "p.access.col_opened": "Aberto",
  "p.access.col_ends": "Termina ou terminou",
  "p.access.col_reason": "Motivo informado",
  "p.access.col_told": "Pessoa avisada",
  "p.access.col_act": "Ação",
  "p.access.refused_sign_in": "Faça login primeiro.",
  "p.access.refused_review_forbidden":
    "A revisão de acessos é somente para administradores.",
  "p.access.refused_review_invalid":
    "O serviço não aceitou o plano nem o número de dias.",
  "p.access.refused_open_forbidden":
    "Você não pode usar o acesso de emergência: é preciso ter a elegibilidade break-glass ou o acesso de administrador.",
  "p.access.refused_open_not_found": "Não há nenhum plano com esse id.",
  "p.access.refused_open_conflict":
    "Você já tem acesso de emergência a este plano; a lista abaixo foi lida novamente.",
  "p.access.refused_open_invalid":
    "O plano não foi aberto. Ou ele não é privado, e portanto não há nada a abrir, ou a pessoa a avisar é você, ou essa pessoa não pode ser avisada. Nenhum acesso foi concedido.",
  "p.access.refused_open_limit":
    "Você abriu o máximo de acessos de emergência permitido em um dia (cinco, contados nas últimas 24 horas).",
  "p.access.refused_revoke_not_found":
    "Essa concessão não foi encontrada, ou não cabe a você encerrá-la.",
  "p.deescalation.nav": "Painéis de indicadores e desescalonamento",
  "p.deescalation.title": "Painéis de indicadores e desescalonamento — PPM",
  "p.deescalation.h1": "Painéis de indicadores e desescalonamento",
  "p.deescalation.lede":
    "O nível de escalonamento como o patrocinador o definiu, os critérios para reduzi-lo com as respectivas evidências a cada trimestre e os painéis de indicadores cujas medidas-chave são lidas por trimestre concluído.",
  "p.deescalation.tracker_heading": "O acompanhamento do desescalonamento",
  "p.deescalation.tracker_says":
    "O nível é registrado como o patrocinador o definiu. Ele nunca é calculado aqui, e nada nesta página o altera: a sequência de trimestres cumpridos é oferecida a quem decide.",
  "p.deescalation.loading": "Carregando…",
  "p.deescalation.could_not_load":
    "Não foi possível carregar o acompanhamento do desescalonamento.",
  "p.deescalation.could_not_save": "A alteração não foi salva.",
  "p.deescalation.level_heading": "Nível de escalonamento",
  "p.deescalation.level_now": "Nível {level}, definido em {date}.",
  "p.deescalation.level_note": "A observação do patrocinador: {note}",
  "p.deescalation.level_none":
    "Nenhum nível de escalonamento foi registrado ainda.",
  "p.deescalation.level_history": "Níveis anteriores",
  "p.deescalation.level_history_item": "{level}, definido em {date}",
  "p.deescalation.run_heading": "Trimestres cumpridos em sequência",
  "p.deescalation.run_one":
    "{n} trimestre concluído cumprido em sequência, dos {need} que a regra exige.",
  "p.deescalation.run_other":
    "{n} trimestres concluídos cumpridos em sequência, dos {need} que a regra exige.",
  "p.deescalation.rule_satisfied":
    "A regra está satisfeita. Cabe ao patrocinador avaliar; o nível não muda sozinho.",
  "p.deescalation.rule_not_satisfied": "A regra ainda não está satisfeita.",
  "p.deescalation.setting_unusable":
    "A configuração da implantação para o número de trimestres que a regra exige não pôde ser usada, portanto vale o padrão de {n}.",
  "p.deescalation.reads_scorecard":
    "O acompanhamento lê o painel de indicadores {name}.",
  "p.deescalation.reads_no_scorecard":
    "Nenhum painel de indicadores é o lido pelo acompanhamento, portanto só os critérios contam. Marque um como tal ao adicioná-lo, abaixo.",
  "p.deescalation.hidden_kpis_one":
    "{n} medida-chave deste painel de indicadores pertence a um plano que você não pode ler: ela não é exibida nem contada aqui.",
  "p.deescalation.hidden_kpis_other":
    "{n} medidas-chave deste painel de indicadores pertencem a planos que você não pode ler: elas não são exibidas nem contadas aqui.",
  "p.deescalation.quarters_label": "Cada trimestre concluído",
  "p.deescalation.col_quarter": "Trimestre",
  "p.deescalation.col_state": "Situação",
  "p.deescalation.col_reasons": "Por quê",
  "p.deescalation.state_met": "Cumprido",
  "p.deescalation.state_not_met": "Não cumprido",
  "p.deescalation.state_missing": "Nada registrado",
  "p.deescalation.state_with_value": "{state} ({value})",
  "p.deescalation.reason_all_met":
    "Todas as medidas-chave e todos os critérios foram cumpridos.",
  "p.deescalation.reason_nothing":
    "Não há nada a medir: nenhuma medida-chave e nenhum critério.",
  "p.deescalation.reason_no_kpis":
    "O painel de indicadores não tem nenhuma medida-chave que você possa ler.",
  "p.deescalation.reason_not_met": "Não cumprido: {list}.",
  "p.deescalation.reason_missing": "Nada registrado para: {list}.",
  "p.deescalation.quarter": "T{q} {year}",
  "p.deescalation.quarter_standing": "{quarter}: {state}. {reasons}",
  "p.deescalation.tracker_kpis_label":
    "O painel de indicadores do acompanhamento, por trimestre",
  "p.deescalation.set_level_heading": "Registrar o nível de escalonamento",
  "p.deescalation.set_level_says":
    "Somente o patrocinador (quem tem a função de patrocinador) ou um administrador pode registrar o nível, como o definiram. Um nível tem de {min} a {max} caracteres; a observação é opcional.",
  "p.deescalation.f_level": "Nível",
  "p.deescalation.f_level_note": "Observação (opcional)",
  "p.deescalation.save_level": "Registrar o nível",
  "p.deescalation.level_saved": "Nível {level} registrado.",
  "p.deescalation.unchanged": "Nada mudou, portanto nada foi enviado.",
  "p.deescalation.problem_level": "Um nível tem de {min} a {max} caracteres.",
  "p.deescalation.refused_level_forbidden":
    "Recusado: somente o patrocinador ou um administrador pode registrar o nível de escalonamento.",
  "p.deescalation.criteria_heading": "Critérios para reduzir o nível",
  "p.deescalation.criteria_says":
    "Um critério é adicionado e nunca editado nem aposentado. Sua evidência é registrada para um trimestre concluído, e o registro mais recente desse trimestre é o que vale.",
  "p.deescalation.criteria_none": "Nenhum critério ainda.",
  "p.deescalation.evidence_of": "Evidência de: {criterion}",
  "p.deescalation.evidence_link": "Evidência",
  "p.deescalation.add_criterion_heading": "Adicionar um critério",
  "p.deescalation.f_criterion": "Critério ({min} a {max} caracteres)",
  "p.deescalation.add_criterion": "Adicionar o critério",
  "p.deescalation.criterion_added": "Critério adicionado.",
  "p.deescalation.problem_criterion":
    "Um critério tem de {min} a {max} caracteres.",
  "p.deescalation.evidence_heading": "Registrar evidência",
  "p.deescalation.evidence_says":
    "Escolha um critério e um trimestre concluído (um trimestre ainda em andamento não pode ser registrado), diga se foi cumprido e escreva uma observação de pelo menos {min} caracteres. Um link é opcional: um endereço http ou https sem espaços, de até {max} caracteres.",
  "p.deescalation.f_ev_criterion": "Critério",
  "p.deescalation.f_ev_quarter": "Trimestre concluído",
  "p.deescalation.f_ev_met": "Foi cumprido?",
  "p.deescalation.f_ev_note": "Observação (pelo menos {min} caracteres)",
  "p.deescalation.f_ev_link": "Link para a evidência (opcional)",
  "p.deescalation.save_evidence": "Registrar a evidência",
  "p.deescalation.evidence_saved": "Evidência registrada para {quarter}.",
  "p.deescalation.problem_criterion_choose":
    "Adicione um critério primeiro e depois registre a sua evidência.",
  "p.deescalation.problem_quarter": "Escolha um trimestre concluído.",
  "p.deescalation.problem_quarter_unfinished":
    "Esse trimestre não terminou, portanto ainda não pode ser registrado.",
  "p.deescalation.problem_met": "Diga se o critério foi cumprido.",
  "p.deescalation.problem_note":
    "É necessária uma observação de pelo menos {min} caracteres.",
  "p.deescalation.problem_link":
    "Um link é um endereço http ou https sem espaços, de até {max} caracteres.",
  "p.deescalation.refused_evidence":
    "Recusado: o trimestre deve ter terminado, a observação deve ter pelo menos cinco caracteres e um link deve ser um endereço http ou https sem espaços.",
  "p.deescalation.refused_gone":
    "Esse registro não existe mais. Recarregue a página.",
  "p.deescalation.scorecards_heading": "Painéis de indicadores",
  "p.deescalation.scorecards_says":
    "Um painel de indicadores é um conjunto nomeado de medidas-chave, lidas por trimestre concluído. Uma medida-chave é cumprida em um trimestre quando sua última leitura nesse trimestre atinge ou supera a meta; um trimestre sem leitura é ausente, o que não é o mesmo que não cumprido.",
  "p.deescalation.scorecards_none": "Nenhum painel de indicadores ainda.",
  "p.deescalation.could_not_load_scorecards":
    "Não foi possível carregar os painéis de indicadores.",
  "p.deescalation.col_scorecard": "Painel de indicadores",
  "p.deescalation.col_tracker": "Lido pelo acompanhamento",
  "p.deescalation.is_tracker": "Sim, o lido pelo acompanhamento",
  "p.deescalation.not_tracker": "Não",
  "p.deescalation.add_scorecard_heading": "Adicionar um painel de indicadores",
  "p.deescalation.add_scorecard_says":
    "Um nome de {min} a {max} caracteres, ainda não usado (em qualquer caixa). No máximo um painel de indicadores é o lido pelo acompanhamento.",
  "p.deescalation.f_name": "Nome",
  "p.deescalation.f_for_tracker": "O lido pelo acompanhamento",
  "p.deescalation.add_scorecard": "Adicionar o painel de indicadores",
  "p.deescalation.scorecard_added": "Painel de indicadores {name} adicionado.",
  "p.deescalation.problem_name":
    "O nome de um painel de indicadores tem de {min} a {max} caracteres.",
  "p.deescalation.problem_name_taken":
    "Já existe um painel de indicadores com esse nome.",
  "p.deescalation.problem_tracker_taken":
    "O painel de indicadores {name} já é o lido pelo acompanhamento.",
  "p.deescalation.scorecard_heading": "Um painel de indicadores por trimestre",
  "p.deescalation.f_scorecard": "Painel de indicadores",
  "p.deescalation.choose_scorecard": "Escolha um painel de indicadores",
  "p.deescalation.could_not_load_scorecard":
    "Não foi possível carregar o painel de indicadores.",
  "p.deescalation.scorecard_says":
    "Cada medida-chave por trimestre concluído: cumprida, não cumprida ou nada registrado.",
  "p.deescalation.scorecard_kpis_label":
    "Medidas-chave de {name}, por trimestre",
  "p.deescalation.scorecard_quarters_label":
    "Cada trimestre do painel de indicadores",
  "p.deescalation.col_kpi": "Medida-chave",
  "p.deescalation.col_target": "Meta",
  "p.deescalation.target_at_least": "pelo menos {value} {unit}",
  "p.deescalation.target_at_most": "no máximo {value} {unit}",
  "p.deescalation.no_kpis":
    "Este painel de indicadores não tem nenhuma medida-chave que você possa ler.",
  "p.deescalation.unit_percent": "por cento",
  "p.deescalation.unit_count": "uma contagem",
  "p.deescalation.kpis_heading": "Escolha as suas medidas-chave (KPIs)",
  "p.deescalation.kpis_says":
    "Marque as medidas-chave que este painel de indicadores contém, no máximo {max}. Salvar substitui o conjunto; uma medida de um plano que você não pode ler permanece no painel.",
  "p.deescalation.f_kpis": "Medidas-chave do painel de indicadores",
  "p.deescalation.kpis_none":
    "Não há medidas-chave para escolher. Adicione-as na página de medidas-chave ou a partir do modelo abaixo.",
  "p.deescalation.save_kpis": "Salvar as medidas-chave",
  "p.deescalation.kpis_saved_one":
    "O painel de indicadores agora contém {n} medida-chave.",
  "p.deescalation.kpis_saved_other":
    "O painel de indicadores agora contém {n} medidas-chave.",
  "p.deescalation.problem_kpis_max":
    "Um painel de indicadores contém no máximo {max} medidas-chave.",
  "p.deescalation.refused_kpis":
    "Recusado: cada uma deve ser uma medida-chave que você possa ler, e um painel de indicadores contém no máximo 60.",
  "p.deescalation.template_heading": "Adicionar medidas a partir do modelo",
  "p.deescalation.template_says":
    "Um conjunto inicial, oferecido e não aplicado. Ele não tem metas: você define cada meta e cada linha vermelha, e as medidas que escolher se tornam medidas-chave deste painel de indicadores.",
  "p.deescalation.template_rule":
    "Onde mais alto é melhor, a linha vermelha deve ficar abaixo da meta; onde mais baixo é melhor, acima dela. Metas e linhas vermelhas são números inteiros.",
  "p.deescalation.could_not_load_template":
    "Não foi possível carregar o modelo.",
  "p.deescalation.higher_better": "Mais alto é melhor",
  "p.deescalation.lower_better": "Mais baixo é melhor",
  "p.deescalation.unit": "medido como {unit}",
  "p.deescalation.f_pick": "Adicionar {measure}",
  "p.deescalation.f_target": "Meta",
  "p.deescalation.f_red": "Linha vermelha",
  "p.deescalation.add_measures": "Adicionar as medidas",
  "p.deescalation.measures_added_one":
    "{n} medida adicionada ao painel de indicadores.",
  "p.deescalation.measures_added_other":
    "{n} medidas adicionadas ao painel de indicadores.",
  "p.deescalation.problem_pick_one": "Escolha pelo menos uma medida.",
  "p.deescalation.problem_whole":
    "{measure}: a meta e a linha vermelha são números inteiros.",
  "p.deescalation.problem_red_higher":
    "{measure}: mais alto é melhor, portanto a linha vermelha deve ficar abaixo da meta.",
  "p.deescalation.problem_red_lower":
    "{measure}: mais baixo é melhor, portanto a linha vermelha deve ficar acima da meta.",
  "p.deescalation.problem_not_in_template": "{measure} não está no modelo.",
  "p.deescalation.refused_template":
    "Recusado: confira cada meta e cada linha vermelha com a regra de direção.",
  "p.deescalation.measure_programmes_with_independent_assurance":
    "Programas com garantia independente no último trimestre",
  "p.deescalation.measure_milestones_met_on_time":
    "Marcos cumpridos na data ou antes dela",
  "p.deescalation.measure_benefits_validated":
    "Benefícios planejados validados por alguém de fora do programa",
  "p.deescalation.measure_red_without_decision":
    "Programas em vermelho sem decisão registrada",
  "p.deescalation.measure_open_critical_issues":
    "Problemas abertos de severidade crítica",
  "p.deescalation.measure_change_requests_waiting":
    "Solicitações de mudança aguardando uma decisão há mais de 30 dias",
  "p.phase.name_initiating": "Iniciação",
  "p.phase.name_planning": "Planejamento",
  "p.phase.name_executing": "Execução",
  "p.phase.name_controlling": "Controle",
  "p.phase.name_closing": "Encerramento",
  "p.phase.heading": "Fase",
  "p.phase.heading_programme": "Fase e início",
  "p.phase.says":
    "Um plano avança uma fase de cada vez; pode voltar a uma fase anterior com um motivo, que é registrado junto com a mudança. A fase nunca impede que o trabalho seja registrado.",
  "p.phase.could_not_load": "Não foi possível carregar a fase do plano",
  "p.phase.now": "O plano está na fase {phase}.",
  "p.phase.none":
    "Nenhuma fase está registrada ainda: o primeiro passo é para Iniciação.",
  "p.phase.time_heading": "Tempo em cada fase",
  "p.phase.days_in_one": "{phase}: {days} dia",
  "p.phase.days_in_other": "{phase}: {days} dias",
  "p.phase.visits_one": "entrou {visits} vez",
  "p.phase.visits_other": "entrou {visits} vezes",
  "p.phase.current": "agora",
  "p.phase.check_heading": "Verificação de início",
  "p.phase.check_says":
    "Um programa começa quando sua fase passa para Execução. O início é recusado quando todas as vagas para programas em entrega ativa estão ocupadas (programas pausados e interrompidos não usam nenhuma) ou quando o grupo restritivo tem oferta insuficiente em qualquer um dos próximos seis meses. Um programa recusado entra na fila de início e mantém o seu lugar; ele pode começar mesmo assim com um motivo de exceção de {min} ou mais caracteres, que é registrado e auditado. Ler a verificação não move nada.",
  "p.phase.could_not_check": "Não foi possível ler a verificação de início",
  "p.phase.places":
    "{in_delivery} de {limit} vagas para programas em entrega ativa estão ocupadas.",
  "p.phase.no_limit":
    "Nenhum limite de programas em entrega ativa está definido.",
  "p.phase.limit_unusable":
    "O limite da implantação para programas em entrega não pôde ser usado, portanto nenhum limite se aplica.",
  "p.phase.pool_is": "O grupo restritivo é {pool}.",
  "p.phase.no_pool": "Nenhum grupo restritivo está identificado.",
  "p.phase.would_refuse": "Um início agora seria recusado:",
  "p.phase.would_start": "Um início agora seria concluído.",
  "p.phase.in_queue": "Este programa é o número {n} na fila de início.",
  "p.phase.see_queue": "Ver a fila de início",
  "p.phase.refusal_limit":
    "Todas as {limit} vagas para programas em entrega ativa estão ocupadas ({in_delivery} em entrega).",
  "p.phase.refusal_pool":
    "O grupo restritivo {pool} tem oferta insuficiente em {months}.",
  "p.phase.demand_heading": "A demanda firme deste programa",
  "p.phase.demand_caption":
    "Equivalentes de tempo integral por grupo de habilidades e mês, a partir das alocações firmes neste programa e nos planos abaixo dele",
  "p.phase.no_demand":
    "Nenhuma demanda firme neste programa em um grupo de habilidades nos próximos seis meses.",
  "p.phase.col_pool": "Grupo de habilidades",
  "p.phase.check_again": "Verificar novamente",
  "p.phase.move_heading": "Mudar a fase",
  "p.phase.f_to": "Passar para",
  "p.phase.forward_to": "Avançar para {phase}",
  "p.phase.back_to": "Voltar para {phase}",
  "p.phase.back_needs_reason": "Voltar exige um motivo.",
  "p.phase.f_reason":
    "Motivo (necessário para voltar; registrado junto com a mudança)",
  "p.phase.start_may_queue":
    "Passar para Execução é o início deste programa: se a verificação não encontrar espaço quando for enviada, o início é recusado e o programa entra na fila de início.",
  "p.phase.start_would_queue":
    "Este início seria recusado. Enviado sem um motivo de exceção, o programa entra na fila de início (ou mantém o seu lugar nela).",
  "p.phase.f_override":
    "Motivo de exceção, para começar mesmo assim ({min} ou mais caracteres; registrado e auditado)",
  "p.phase.refused_why": "Por que o início foi recusado",
  "p.phase.move": "Mudar a fase",
  "p.phase.moved": "O plano agora está na fase {phase}.",
  "p.phase.started_override":
    "O programa começou com uma exceção: agora está na fase {phase}, e a exceção está registrada.",
  "p.phase.start_refused_place":
    "O início foi recusado, portanto o programa é o número {n} na fila de início. Para começar mesmo assim, informe um motivo de exceção.",
  "p.phase.start_refused":
    "O início foi recusado, e o programa entrou na fila de início. Para começar mesmo assim, informe um motivo de exceção.",
  "p.phase.could_not_move": "Não foi possível mudar a fase",
  "p.phase.problem_choose": "Escolha a fase para a qual passar.",
  "p.phase.problem_same":
    "O plano já está nessa fase, portanto nada foi enviado.",
  "p.phase.problem_skip": "Avançar é uma fase de cada vez.",
  "p.phase.problem_reason": "Voltar exige um motivo.",
  "p.phase.problem_override":
    "Um motivo de exceção tem {min} ou mais caracteres.",
  "p.phase.refused_move":
    "Recusado: avançar é uma fase de cada vez, voltar exige um motivo e o plano não pode já estar nessa fase.",
  "p.phase.refused_gone": "Esse plano não existe mais.",
  "p.proposals.act_submit": "Enviar",
  "p.proposals.act_review": "Iniciar revisão",
  "p.proposals.act_approve": "Aprovar",
  "p.proposals.act_reject": "Rejeitar",
  "p.proposals.act_submit_named": "Enviar {title}",
  "p.proposals.act_review_named": "Iniciar revisão de {title}",
  "p.proposals.act_approve_named": "Aprovar {title}",
  "p.proposals.act_reject_named": "Rejeitar {title}",
  "p.proposals.ask_rule":
    "Um pedido externo do patrocinador deve indicar o que ele desloca (uma contrapartida de {min} ou mais caracteres) antes de poder ser aprovado.",
  "p.proposals.new_form": "Nova proposta",
  "p.proposals.f_external_ask": "Um pedido externo do patrocinador",
  "p.proposals.f_new_tradeoff":
    "O que ele desloca (a contrapartida, de {min} a {max} caracteres; pode ser indicada depois, antes da aprovação)",
  "p.proposals.col_tradeoff": "Pedido externo e contrapartida",
  "p.proposals.not_ask": "Não é um pedido externo",
  "p.proposals.ask_with": "Pedido externo. Contrapartida: {tradeoff}",
  "p.proposals.ask_without":
    "Pedido externo. Nenhuma contrapartida indicada ainda.",
  "p.proposals.tradeoff_name": "Indicar a contrapartida",
  "p.proposals.tradeoff_change": "Alterar a contrapartida",
  "p.proposals.tradeoff_name_named": "Indicar a contrapartida de {title}",
  "p.proposals.tradeoff_change_named": "Alterar a contrapartida de {title}",
  "p.proposals.tradeoff_heading": "A contrapartida de {title}",
  "p.proposals.tradeoff_says":
    "Indique o que este pedido desloca, em {min} a {max} caracteres. Ela pode ser indicada ou alterada até que a proposta seja aprovada ou rejeitada, e cada alteração é registrada.",
  "p.proposals.f_tradeoff": "O que ele desloca ({min} a {max} caracteres)",
  "p.proposals.tradeoff_save": "Salvar a contrapartida",
  "p.proposals.tradeoff_close": "Fechar o formulário da contrapartida",
  "p.proposals.tradeoff_unchanged": "Nada mudou, portanto nada foi enviado.",
  "p.proposals.tradeoff_saved": "Contrapartida salva para {title}.",
  "p.proposals.problem_title": "Uma proposta precisa de um título.",
  "p.proposals.problem_tradeoff":
    "Uma contrapartida tem de {min} a {max} caracteres.",
  "p.proposals.problem_not_ask": "Somente um pedido externo tem contrapartida.",
  "p.proposals.problem_decided":
    "A proposta já foi decidida, portanto sua contrapartida não pode mais mudar.",
  "p.proposals.problem_approve":
    "{title} é um pedido externo: indique a sua contrapartida ({min} ou mais caracteres) antes de aprová-lo.",
  "p.proposals.refused_create":
    "Recusado: uma proposta precisa de um título, um valor precisa de uma moeda e uma contrapartida tem no máximo {max} caracteres.",
  "p.proposals.refused_tradeoff":
    "Recusado: uma contrapartida tem de {min} a {max} caracteres, em um pedido externo que ainda não foi decidido.",
  "p.proposals.refused_step":
    "Recusado: esse passo não decorre do status da proposta.",
  "p.proposals.refused_gone": "Essa proposta não existe mais.",
  "p.thresholds.opened_by_kind":
    "Aberto porque o programa ultrapassou o seu limite: {kind}.",
  "p.thresholds.joined_by_kind":
    "Enquanto este estava aberto, o programa também ultrapassou um limite: {kind}.",
  "p.deescalation.refused_tracker_taken":
    "Recusado: outro painel de indicadores já é o lido pelo acompanhamento.",
  "p.deescalation.level_now_by": "Nível {level}, definido em {date} por {who}.",
  "p.deescalation.level_history_item_by":
    "{level}, definido em {date} por {who}",
  "p.deescalation.evidence_recorded_by": "registrado por {who} em {date}",
  "p.deescalation.evidence_recorded_on": "registrado em {date}",
  "p.proposals.refused_approve_status":
    "Recusado: somente uma proposta em revisão pode ser aprovada.",
  "p.proposals.refused_tradeoff_required":
    "Recusado: um pedido externo precisa de uma contrapartida indicada, de {min} ou mais caracteres, antes de poder ser aprovado.",
  "p.ledger.untraced_no_readable_line":
    "Nenhuma linha de orçamento que você possa ler recorre a esta alocação, portanto não é possível rastrear aqui para onde vai o seu dinheiro.",
  "p.ledger.untraced_not_on_a_line":
    "Parte desta alocação ainda não está planejada em nenhuma linha de orçamento.",
  "p.ledger.untraced_lines_over_plan":
    "As linhas de orçamento planejam mais do que esta alocação contém.",
  "p.ledger.untraced_other":
    "Parte do dinheiro desta alocação não pode ser rastreada.",
  "p.status.title_ppm": "Atualizações de status — PPM",
  "p.status.title": "Atualizações de status",
  "p.status.link": "Atualizações de status e suas evidências",
  "p.status.intro":
    "Uma atualização de status é a avaliação de uma pessoa sobre como o plano está indo (no prazo, em risco ou fora do prazo), com o que está acontecendo em palavras. Nada aqui calcula uma pontuação.",
  "p.status.could_not_load": "Não foi possível carregar o status do plano",
  "p.status.now_heading": "Em que ponto o plano está",
  "p.status.no_update": "Nenhuma atualização de status foi publicada ainda.",
  "p.status.cadence_is_one": "Espera-se uma atualização por dia.",
  "p.status.cadence_is_other": "Espera-se uma atualização a cada {n} dias.",
  "p.status.standing_overdue_one":
    "A próxima atualização era devida em {date}: está atrasada {n} dia.",
  "p.status.standing_overdue_other":
    "A próxima atualização era devida em {date}: está atrasada {n} dias.",
  "p.status.standing_due": "Uma atualização é devida hoje, {date}.",
  "p.status.standing_next": "A próxima atualização é devida em {date}.",
  "p.status.no_cadence":
    "Nenhuma periodicidade está definida, portanto nenhuma atualização é devida nem está atrasada.",
  "p.status.as_of": "Em {date}, no fuso horário da implantação.",
  "p.status.loading": "Carregando…",
  "p.status.posted_on_by": "publicada em {date} por {who}",
  "p.status.posted_on": "publicada em {date}",
  "p.status.claims_of": "Afirmações na atualização de {date}",
  "p.status.source_metric": "{metric}: {value} agora",
  "p.status.source_metric_unknown": "{metric}: desconhecido (não zero)",
  "p.status.source_unavailable": "Uma fonte que não está mais disponível",
  "p.status.kind_task": "Tarefa: {title}",
  "p.status.kind_risk": "Risco: {title}",
  "p.status.kind_milestone": "Marco: {title}",
  "p.status.kind_issue": "Problema: {title}",
  "p.status.kind_other": "Registro: {title}",
  "p.status.metric_overdue": "Tarefas atrasadas",
  "p.status.metric_due_soon": "Tarefas com vencimento próximo",
  "p.status.metric_blocked": "Tarefas bloqueadas",
  "p.status.metric_no_due_date": "Tarefas sem data de vencimento",
  "p.status.metric_def_overdue":
    "Tarefas abertas cuja data de vencimento é anterior a hoje. Uma tarefa com vencimento hoje não está atrasada. Desconhecido quando há tarefas abertas e nenhuma tem data de vencimento.",
  "p.status.metric_def_due_soon":
    "Tarefas abertas com vencimento hoje ou nos próximos {days} dias. Desconhecido quando há tarefas abertas e nenhuma tem data de vencimento.",
  "p.status.metric_def_blocked": "Tarefas abertas cujo status é bloqueada.",
  "p.status.metric_def_no_due_date":
    "Tarefas abertas sem data de vencimento: as tarefas que os outros números não enxergam.",
  "p.status.metrics_heading": "Como os números são contados",
  "p.status.metrics_says":
    "Sobre as tarefas abertas do plano (uma tarefa cujo estado não é concluída), contadas quando uma atualização é lida, não quando foi publicada.",
  "p.status.cadence_heading": "Com que frequência uma atualização é esperada",
  "p.status.cadence_says":
    "A cada {min} a {max} dias. Defini-la reinicia a contagem a partir de hoje, de modo que a mesma periodicidade não seja enviada de novo. Uma periodicidade não pode ser removida depois de definida.",
  "p.status.cadence_set_by": "Definida em {date} por {who}.",
  "p.status.cadence_set_on": "Definida em {date}.",
  "p.status.f_cadence": "Dias entre atualizações ({min} a {max})",
  "p.status.save_cadence": "Salvar a periodicidade",
  "p.status.unchanged": "Nada mudou, portanto nada foi enviado.",
  "p.status.problem_cadence":
    "Uma periodicidade é um número inteiro de dias, de {min} a {max}.",
  "p.status.cadence_saved_one":
    "Agora espera-se uma atualização por dia; a contagem começa hoje.",
  "p.status.cadence_saved_other":
    "Agora espera-se uma atualização a cada {n} dias; a contagem começa hoje.",
  "p.status.could_not_save": "Não foi possível salvar a periodicidade",
  "p.status.refused_gone": "Esse plano não existe mais.",
  "p.status.history_heading":
    "Todas as atualizações, das mais recentes às mais antigas",
  "p.status.history_says":
    "As atualizações nunca são alteradas nem removidas. Cada fonte é mostrada como está agora: um registro que desapareceu não está mais disponível, e cada número é o seu valor hoje.",
  "p.status.history_none": "Nenhuma atualização ainda.",
  "p.status.post_heading": "Publicar uma atualização",
  "p.status.post_says":
    "Escolha a avaliação e diga o que está acontecendo (até {max} caracteres). Uma atualização não pode ser alterada nem removida depois de publicada, e publicar uma reinicia a contagem da periodicidade.",
  "p.status.claims_rule":
    "Você pode adicionar até {claims} afirmações, cada uma uma frase de até {chars} caracteres que cita até {sources} fontes: os números abaixo, ou as tarefas, os riscos, os marcos e os problemas do plano. Cada fonte é verificada quando a atualização é publicada.",
  "p.status.f_call": "A avaliação",
  "p.status.call_on_track": "No prazo",
  "p.status.call_at_risk": "Em risco",
  "p.status.call_off_track": "Fora do prazo",
  "p.status.f_narrative": "O que está acontecendo (até {max} caracteres)",
  "p.status.claim_legend": "Afirmação {n}",
  "p.status.f_claim_text": "A afirmação (até {chars} caracteres)",
  "p.status.f_metrics": "Números que ela cita",
  "p.status.f_record": "Um registro do plano que a afirmação {n} cita",
  "p.status.choose_record": "Escolha um registro",
  "p.status.add_record": "Adicionar o registro à afirmação {n}",
  "p.status.claim_records": "Registros que a afirmação {n} cita",
  "p.status.remove": "Remover",
  "p.status.remove_source": "Remover {source} da afirmação {n}",
  "p.status.remove_claim": "Remover a afirmação {n}",
  "p.status.add_claim": "Adicionar uma afirmação",
  "p.status.post": "Publicar a atualização",
  "p.status.posted": "Atualização publicada: {call}.",
  "p.status.could_not_post": "Não foi possível publicar a atualização",
  "p.status.problem_call": "Escolha no prazo, em risco ou fora do prazo.",
  "p.status.problem_narrative":
    "Diga o que está acontecendo, em até {max} caracteres.",
  "p.status.problem_claims":
    "Uma atualização contém no máximo {claims} afirmações.",
  "p.status.problem_claim_text":
    "Uma afirmação é uma frase de até {chars} caracteres.",
  "p.status.problem_sources": "Uma afirmação cita no máximo {sources} fontes.",
  "p.status.problem_in_claim": "Afirmação {n}: {problem}",
  "p.status.refused_update":
    "Recusado: uma fonte pode não ser mais um registro deste plano. Nada foi publicado; confira as afirmações e publique novamente.",
  "p.status.due_heading": "Atualizações de status devidas",
  "p.status.due_none":
    "Nenhum plano que você possa ler tem uma atualização de status devida.",
  "p.status.due_could_not_load":
    "Não foi possível carregar as atualizações de status devidas",
  "p.status.a_plan": "Um plano",
  "p.status.due_last": "Última atualização: {call}, em {date}.",
  "p.status.due_never": "Nenhuma atualização ainda.",
  "p.status.group_task": "Tarefas",
  "p.status.group_risk": "Riscos",
  "p.status.group_milestone": "Marcos",
  "p.status.group_issue": "Problemas",
  "l.enum.risk_status.open": "Aberto",
  "l.enum.risk_status.mitigating": "Em mitigação",
  "l.enum.risk_status.closed": "Fechado",
  "l.enum.risk_status.materialized": "Concretizado",
  "p.risk_edit.change": "Alterar",
  "p.risk_edit.change_named": "Alterar: {title}",
  "p.risk_edit.heading": "Alterar o risco “{title}”",
  "p.risk_edit.says":
    "A probabilidade e o impacto são números inteiros de {min} a {max}; a exposição é o produto deles. A mitigação tem no máximo {max_bytes} bytes (menos caracteres em alguns alfabetos). Uma data de revisão pode ser alterada, mas não removida. Somente o que você alterar é enviado.",
  "p.risk_edit.says_escalate":
    "Para registrar que um risco ocorreu, use Escalonar na tabela, que também registra quando.",
  "p.risk_edit.problem_status": "Escolha um status.",
  "p.risk_edit.problem_score":
    "A probabilidade e o impacto devem ser, cada um, um número inteiro de {min} a {max}.",
  "p.risk_edit.problem_mitigation":
    "A mitigação tem mais de {max_bytes} bytes.",
  "p.risk_edit.problem_review_cleared":
    "Uma data de revisão não pode ser removida depois de definida; escolha outra data.",
  "p.risk_edit.problem_review_date": "A data de revisão não é uma data.",
  "p.risk_edit.unchanged": "Nada mudou, portanto nada foi enviado.",
  "p.risk_edit.saved": "O risco “{title}” foi salvo.",
  "p.risk_edit.could_not_save": "Não foi possível salvar o risco",
  "p.risk_edit.refused_gone":
    "Esse risco não está mais neste plano; a lista foi lida novamente.",
  "p.risk_edit.refused_invalid":
    "Recusado: o serviço não aceitou o status, as pontuações ou a mitigação. Nada foi alterado.",
  "p.risk_edit.f_status": "Status do risco",
  "p.risk_edit.f_probability": "Probabilidade ({min} a {max})",
  "p.risk_edit.f_impact": "Impacto ({min} a {max})",
  "p.risk_edit.f_mitigation": "Mitigação",
  "p.risk_edit.f_review_date": "Data de revisão",
  "p.risk_edit.save": "Salvar o risco",
  "p.risk_edit.close": "Fechar o formulário",
  "p.plan_issues.refused_gone":
    "Esse problema não está mais neste plano; a lista foi lida novamente.",
  "p.plan_issues.refused_remove":
    "Recusado: você não pode remover problemas neste plano. Nada foi removido.",
  "p.plan_issues.refused_remove_admin":
    "Recusado: somente um administrador pode remover um problema sob o fluxo de trabalho deste tipo. Nada foi removido.",
  "p.plan_issues.remove_admin_only":
    "Somente um administrador pode remover “{title}”: o fluxo de trabalho do seu tipo controla suas mudanças ou campos. Nada foi enviado.",
  "p.plan_issues.confirm_remove":
    "Remover o problema “{title}”? Ele sai de todas as listas; o registro dele é mantido para a auditoria.",
  "p.plan_issues.removed": "O problema “{title}” foi removido.",
  "p.plan_issues.remove": "Remover",
  "p.plan_issues.remove_named": "Remover: {title}",
  "p.plan_issues.remove_says":
    "Remover um problema o retira de todas as listas e não pode ser desfeito aqui; a auditoria mantém um registro. Sob um fluxo de trabalho que controla mudanças ou campos, somente um administrador pode remover um.",
  "p.plan_issues.raised_on_by": "Levantado em {date} por {who}.",
  "p.plan_issues.raised_on": "Levantado em {date}.",
  "p.plan_issues.from_risk_named": "Levantado a partir do risco “{title}”.",
  "p.plan_issues.from_a_risk":
    "Levantado a partir de um risco que não está mais na lista.",
  "p.plan_issues.from_a_task":
    "Levantado a partir de uma tarefa bloqueada (abra a tarefa)",
  "p.plan_issues.answered_by_change":
    "Atendido por uma solicitação de mudança (veja as solicitações de mudança do plano)",
  "p.plan_issues.resolved_on": "Resolvido em {date}.",
  "p.task.issue_heading": "Um problema a partir desta tarefa",
  "p.task.issue_says":
    "Somente uma tarefa bloqueada pode ser levantada como problema, e apenas uma vez enquanto esse problema estiver aberto. O problema é um impedimento de severidade média, de responsabilidade do responsável pela tarefa; a própria tarefa não é alterada.",
  "p.task.issue_raised": "O problema foi levantado.",
  "p.task.issue_open_list": "Ver os problemas do plano",
  "p.task.issue_exists":
    "Um problema aberto foi levantado a partir desta tarefa: “{title}”.",
  "p.task.issue_not_blocked":
    "Esta tarefa não está bloqueada, portanto não pode ser levantada como problema. Marque-a como bloqueada no quadro primeiro.",
  "p.task.issue_raise": "Levantar um problema a partir desta tarefa",
  "p.task.refused_from_task_not_blocked":
    "Recusado: somente uma tarefa bloqueada pode ser levantada como problema, e esta não está mais bloqueada. A tarefa foi lida novamente.",
  "p.task.refused_from_task_exists":
    "Recusado: um problema aberto já foi levantado a partir desta tarefa. Os problemas foram lidos novamente.",
  "p.task.refused_from_task_forbidden":
    "Recusado: você não pode levantar problemas neste plano.",
  "p.task.refused_from_task_gone": "Essa tarefa não está mais neste plano.",
  "p.plan_change_requests.refused_gone":
    "Essa solicitação de mudança não está mais neste plano; a lista foi lida novamente.",
  "p.plan_change_requests.refused_links_conflict":
    "Recusado: a solicitação foi encerrada, ou um dos problemas agora atende a outra solicitação de mudança. A solicitação e os problemas do plano foram lidos novamente.",
  "p.plan_change_requests.refused_links_invalid":
    "Recusado: um problema ou risco escolhido não está mais neste plano. Os problemas e riscos do plano foram lidos novamente.",
  "p.plan_change_requests.refused_preview":
    "Recusado: esta solicitação não traz edições hipotéticas que o agendador possa ler, portanto não há previsão.",
  "p.plan_change_requests.refused_baseline":
    "Recusado: uma linha de base já foi criada para esta mudança, ou ela não está mais aprovada. A solicitação foi lida novamente.",
  "p.plan_change_requests.problem_links_too_many":
    "Escolha no máximo {max} problemas e {max} riscos.",
  "p.plan_change_requests.problem_links_elsewhere":
    "“{title}” já atende a outra solicitação de mudança; desmarque-o.",
  "p.plan_change_requests.links_unchanged":
    "Nada mudou, portanto nada foi enviado.",
  "p.plan_change_requests.links_saved":
    "Os problemas e riscos que esta mudança atende foram salvos.",
  "p.plan_change_requests.confirm_baseline":
    "Criar uma linha de base do cronograma por causa de “{title}”? Somente uma pode ser criada para esta mudança, e isso não pode ser desfeito.",
  "p.plan_change_requests.baseline_taken":
    "A versão {version} da linha de base foi criada por causa desta mudança.",
  "p.plan_change_requests.stated_impact":
    "Impacto informado por quem solicitou: {impact}.",
  "p.plan_change_requests.implemented_on": "Implementada em {date}.",
  "p.plan_change_requests.implement": "Marcar como implementada",
  "p.plan_change_requests.implemented":
    "A mudança foi marcada como implementada.",
  "p.plan_change_requests.links_heading":
    "Problemas e riscos que esta mudança atende",
  "p.plan_change_requests.links_none":
    "Ela ainda não atende a nenhum problema nem risco.",
  "p.plan_change_requests.answers_issue": "Problema: {title}",
  "p.plan_change_requests.answers_risk": "Risco: {title}",
  "p.plan_change_requests.an_issue": "um problema que não está mais na lista",
  "p.plan_change_requests.a_risk": "um risco que não está mais na lista",
  "p.plan_change_requests.links_says":
    "Escolhidos enquanto a solicitação está aberta (rascunho, enviada, em revisão ou aprovada). Um problema atende a no máximo uma solicitação de mudança; até {max} problemas e {max} riscos. Salvar substitui o conjunto inteiro.",
  "p.plan_change_requests.links_issues": "Problemas deste plano",
  "p.plan_change_requests.issue_elsewhere":
    "{title} (atende a outra solicitação de mudança)",
  "p.plan_change_requests.links_no_issues": "Este plano não tem problemas.",
  "p.plan_change_requests.links_risks": "Riscos deste plano",
  "p.plan_change_requests.links_no_risks": "Este plano não tem riscos.",
  "p.plan_change_requests.links_save": "Salvar o que ela atende",
  "p.plan_change_requests.links_closed":
    "Uma solicitação encerrada não pode ser vinculada novamente.",
  "p.plan_change_requests.forecast_heading": "Previsão hipotética",
  "p.plan_change_requests.forecast_says":
    "Uma previsão a partir do cronograma atual com as edições hipotéticas desta solicitação aplicadas. Lê-la não altera nada; o impacto informado acima é o valor do próprio solicitante.",
  "p.plan_change_requests.forecast_show": "Mostrar a previsão",
  "p.plan_change_requests.forecast_label": "Previsão — nada foi alterado",
  "p.plan_change_requests.forecast_finish":
    "Término do plano: {before} agora, {after} com a mudança.",
  "p.plan_change_requests.forecast_finish_unknown":
    "O término do plano não pode ser previsto.",
  "p.plan_change_requests.forecast_moved_one":
    "1 tarefa terminaria em um dia diferente.",
  "p.plan_change_requests.forecast_moved_other":
    "{n} tarefas terminariam em um dia diferente.",
  "p.plan_change_requests.forecast_later_one":
    "{title}: termina 1 dia útil depois",
  "p.plan_change_requests.forecast_later_other":
    "{title}: termina {n} dias úteis depois",
  "p.plan_change_requests.forecast_earlier_one":
    "{title}: termina 1 dia útil antes",
  "p.plan_change_requests.forecast_earlier_other":
    "{title}: termina {n} dias úteis antes",
  "p.plan_change_requests.a_task": "Uma tarefa",
  "p.plan_change_requests.forecast_labour":
    "O custo de mão de obra planejado mudaria em {amount}.",
  "p.plan_change_requests.forecast_labour_hidden":
    "A previsão do custo de mão de obra é mostrada apenas a quem pode ver as taxas de custo.",
  "p.plan_change_requests.forecast_labour_none":
    "Nenhum trabalho com custo, portanto nenhuma previsão de custo de mão de obra.",
  "p.plan_change_requests.forecast_labour_unknown":
    "Nenhuma previsão de custo de mão de obra.",
  "p.plan_change_requests.forecast_no_edits":
    "Esta solicitação não traz edições hipotéticas, portanto não há previsão; o impacto informado é o valor do próprio solicitante.",
  "p.plan_change_requests.baseline_heading":
    "Uma linha de base por causa desta mudança",
  "p.plan_change_requests.baseline_done":
    "Uma linha de base do cronograma foi criada por causa desta mudança.",
  "p.plan_change_requests.baseline_says":
    "Criar uma é uma escolha de uma pessoa; nada é rebaseado automaticamente. Uma linha de base por mudança, com o motivo citando a solicitação, e não pode ser desfeita.",
  "p.plan_change_requests.baseline_take": "Criar uma linha de base",
  "p.plan_change_requests.baseline_not_yet":
    "Uma linha de base pode ser criada depois que a mudança for aprovada.",
  "p.escalations.problem_outcome":
    "Escolha um resultado: decidido, orientação dada ou rejeitado.",
  "p.escalations.problem_response":
    "Escreva uma resposta de {min} a {max} caracteres.",
  "p.escalations.refused_ack_right":
    "Recusado: reconhecer um escalonamento exige o direito de reconhecer escalonamentos (um administrador o tem). Nada foi registrado.",
  "p.escalations.refused_resolve_right":
    "Recusado: resolver um escalonamento exige o direito de resolver escalonamentos (um administrador o tem). Nada foi registrado.",
  "p.escalations.refused_ack_state":
    "Recusado: este escalonamento foi resolvido ou retirado nesse meio-tempo, portanto não pode ser reconhecido. Ele foi lido novamente.",
  "p.escalations.refused_resolve_state":
    "Recusado: este escalonamento foi retirado, ou já está resolvido com outro resultado ou outra resposta; uma resolução nunca é sobrescrita. Ele foi lido novamente.",
  "p.escalations.refused_resolve_invalid":
    "Recusado: o resultado ou a resposta não foi aceito. Confira as regras acima e tente novamente.",
  "p.escalations.refused_gone":
    "Este escalonamento não existe mais, ou você não pode mais vê-lo. A lista foi lida novamente.",
  "p.escalations.could_not_save": "Isso não foi registrado.",
  "p.escalations.by_unrecorded": "alguém não registrado",
  "p.escalations.ack_record": "Reconhecido no nível {level} por {by}, {at}.",
  "p.escalations.resolution_record": "Resolvido {at} por {by}: {outcome}.",
  "p.escalations.outcome_unrecorded": "nenhum resultado registrado",
  "p.escalations.records_heading": "Reconhecimento e resolução",
  "p.escalations.records_failed":
    "O reconhecimento ou a resolução não pôde ser lido; o histórico abaixo ainda mostra o que aconteceu.",
  "p.escalations.ack_note_line": "Observação: {note}",
  "p.escalations.ack_none": "Não reconhecido no nível atual.",
  "p.escalations.resolution_kept":
    "Uma resolução é um registro: ela não é alterada nem sobrescrita.",
  "p.escalations.records_rules":
    "Reconhecer exige o direito de reconhecer escalonamentos, e resolver exige o direito de resolvê-los; um administrador tem ambos, e esta página não pode saber de antemão se você os tem. Um reconhecimento vale no nível atual: subir um nível o apaga. Uma resolução é um registro e nunca é sobrescrita.",
  "p.escalations.ack_heading": "Reconhecer este escalonamento",
  "p.escalations.ack_note": "Observação (opcional)",
  "p.escalations.ack_done": "Reconhecido; o registro é mostrado abaixo.",
  "p.escalations.resolve_heading": "Resolver este escalonamento",
  "p.escalations.resolve_says":
    "A resposta tem de {min} a {max} caracteres. Pede-se confirmação antes de registrá-la, e ela não pode ser alterada depois.",
  "p.escalations.confirm_resolve":
    "Registrar esta resolução de “{title}”? Ela não poderá ser alterada nem sobrescrita depois.",
  "p.escalations.resolution_done": "Resolvido; o registro é mostrado abaixo.",
  "p.register_completeness.link": "Completude do registro",
  "p.register_completeness.title": "Completude do registro",
  "p.register_completeness.title_ppm": "Completude do registro — PPM",
  "p.register_completeness.intro":
    "Quão completo está o registro de cada programa: do que um programa deveria ter registrado, a parcela que ele tem, com cada item ausente nomeado. Somente os programas (planos do tipo Programa) que você pode ler são listados.",
  "p.register_completeness.rules":
    "Os dez: um responsável sênior (o líder do programa representa um), parceiros, uma etapa, uma linha de base do cronograma, uma data-alvo, um orçamento, uma fonte de financiamento em cada linha de orçamento, uma avaliação da confiança de entrega, riscos e dependências. Os parceiros só são exigidos onde o modelo de entrega tem parceiros, e a fonte de financiamento só onde há uma linha de orçamento. O programa e os planos abaixo dele contam em conjunto.",
  "p.register_completeness.switch_on":
    "“Sem registro, sem dinheiro” está ativado: liberar uma linha de orçamento ou reservar uma alocação firme em um programa incompleto é recusado, indicando o que falta.",
  "p.register_completeness.switch_off":
    "“Sem registro, sem dinheiro” está desativado: liberar uma linha de orçamento ou reservar uma alocação firme em um programa incompleto prossegue, e o aviso é mantido na trilha de auditoria.",
  "p.register_completeness.count_one":
    "{complete} de {n} programa está completo.",
  "p.register_completeness.count_other":
    "{complete} de {n} programas estão completos.",
  "p.register_completeness.none": "Não há nenhum programa que você possa ler.",
  "p.register_completeness.loading": "Carregando…",
  "p.register_completeness.could_not_load":
    "Não foi possível carregar a completude do registro.",
  "p.register_completeness.table": "Programas e seus registros",
  "p.register_completeness.col_programme": "Programa",
  "p.register_completeness.col_state": "Estado",
  "p.register_completeness.col_score": "Parcela registrada",
  "p.register_completeness.col_on_record": "Campos registrados",
  "p.register_completeness.col_missing": "Ausentes",
  "p.register_completeness.col_not_asked": "Não exigidos",
  "p.register_completeness.of": "{present} de {applicable}",
  "p.register_completeness.state_complete": "completo",
  "p.register_completeness.state_incomplete": "incompleto",
  "p.register_completeness.field_other": "outro campo",
  "p.register_completeness.none_listed": "nenhum",
  "p.meetings.title": "Reuniões",
  "p.meetings.title_ppm": "Reuniões — PPM",
  "p.meetings.link": "Reuniões, atas e ações",
  "p.meetings.intro":
    "Uma reunião é planejada com um título, uma data e uma pauta. Depois, ela é registrada uma única vez como realizada, com a sua ata e os seus resultados (decisões, ações e anotações), ou é cancelada. Uma reunião realizada ou cancelada é um registro: a ata de uma reunião realizada não pode ser corrigida e uma reunião cancelada não pode ser restaurada. Os participantes não são registrados.",
  "p.meetings.plan_heading": "Planejar uma reunião",
  "p.meetings.plan_says":
    "Um título de até {title} caracteres e uma data. A pauta contém até {items} itens de até {chars} caracteres cada; um item pode apontar para uma tarefa, um risco, um marco ou um problema deste plano.",
  "p.meetings.f_title": "Título (até {max} caracteres)",
  "p.meetings.f_date": "Data para a qual está planejada",
  "p.meetings.agenda_item": "Item de pauta {n}",
  "p.meetings.f_item_text":
    "Do que trata o item de pauta {n} (até {max} caracteres)",
  "p.meetings.f_item_link": "O registro a que o item de pauta {n} se refere",
  "p.meetings.no_record": "Nenhum registro",
  "p.meetings.records_loading": "Carregando os registros do plano…",
  "p.meetings.remove_item": "Remover o item de pauta {n}",
  "p.meetings.add_item": "Adicionar um item de pauta",
  "p.meetings.plan": "Planejar a reunião",
  "p.meetings.planned": "Planejada: {title}, em {date}.",
  "p.meetings.list_heading":
    "As reuniões do plano, das mais recentes às mais antigas",
  "p.meetings.pdf_says":
    "Cada reunião é baixada como um documento PDF com o que esta página mostra; suas palavras fixas (os títulos e o estado) estão em inglês.",
  "p.meetings.loading": "Carregando as reuniões…",
  "p.meetings.none": "Nenhuma reunião ainda.",
  "p.meetings.status_planned": "planejada",
  "p.meetings.status_held": "realizada",
  "p.meetings.status_cancelled": "cancelada",
  "p.meetings.held_on": "realizada em {date}",
  "p.meetings.scheduled_on": "planejada para {date}",
  "p.meetings.show": "Abrir a reunião {title}",
  "p.meetings.hide": "Fechar a reunião {title}",
  "p.meetings.pdf": "Baixar como PDF",
  "p.meetings.pdf_named": "Baixar a reunião {title} como PDF",
  "p.meetings.agenda_heading": "Pauta",
  "p.meetings.agenda_none": "Nenhum item de pauta.",
  "p.meetings.minutes_heading": "Ata",
  "p.meetings.outcomes_heading": "Resultados",
  "p.meetings.outcomes_none": "Nenhum resultado foi registrado.",
  "p.meetings.outcome_decision": "Decisão",
  "p.meetings.outcome_action": "Ação",
  "p.meetings.outcome_note": "Anotação",
  "p.meetings.about": "sobre {record}",
  "p.meetings.link_gone": "sobre um registro que não existe mais",
  "p.meetings.task_made": "a tarefa criada a partir do resultado {n}",
  "p.meetings.held_kept":
    "Esta reunião é um registro: sua ata e seus resultados não são alterados.",
  "p.meetings.cancelled_kept":
    "Esta reunião foi cancelada. É um registro e não pode ser restaurada.",
  "p.meetings.held_heading": "Registrar {title} como realizada",
  "p.meetings.held_says":
    "Uma ata de até {minutes} caracteres e até {items} resultados de até {chars} caracteres cada: cada um uma decisão, uma ação ou uma anotação, e cada um pode apontar para um registro deste plano. A data de realização é a data planejada, a menos que você informe outra. Isso é feito uma única vez: a ata não pode ser corrigida depois.",
  "p.meetings.f_minutes": "Ata (até {max} caracteres)",
  "p.meetings.f_held_on":
    "Data em que foi realizada (em branco para a data planejada)",
  "p.meetings.outcome_n": "Resultado {n}",
  "p.meetings.f_outcome_kind": "O que é o resultado {n}",
  "p.meetings.choose_kind": "Escolher…",
  "p.meetings.f_outcome_text":
    "O resultado {n} em palavras (até {max} caracteres)",
  "p.meetings.f_outcome_link": "O registro a que o resultado {n} se refere",
  "p.meetings.remove_outcome": "Remover o resultado {n}",
  "p.meetings.add_outcome": "Adicionar um resultado",
  "p.meetings.record_held": "Registrar como realizada",
  "p.meetings.cancel_meeting": "Cancelar a reunião {title}",
  "p.meetings.confirm_held":
    "Registrar {title} como realizada? Sua ata e seus resultados não poderão ser corrigidos depois.",
  "p.meetings.confirm_cancel":
    "Cancelar {title}? Uma reunião cancelada não pode ser restaurada.",
  "p.meetings.held_done": "Registrada como realizada: {title}.",
  "p.meetings.cancelled_done": "Cancelada: {title}.",
  "p.meetings.problem_title": "Informe um título de até {title} caracteres.",
  "p.meetings.problem_date": "Informe uma data.",
  "p.meetings.problem_agenda_count":
    "Uma pauta contém no máximo {items} itens.",
  "p.meetings.problem_outcome_count":
    "Uma reunião contém no máximo {items} resultados.",
  "p.meetings.problem_item_text": "Escreva em até {chars} caracteres.",
  "p.meetings.problem_minutes": "Escreva a ata, em até {minutes} caracteres.",
  "p.meetings.problem_outcome_kind":
    "Escolha uma decisão, uma ação ou uma anotação.",
  "p.meetings.problem_on_line": "Para {line}: {problem}",
  "p.meetings.could_not_load": "Não foi possível carregar as reuniões.",
  "p.meetings.could_not_save": "A reunião não foi alterada.",
  "p.meetings.refused_gone":
    "Recusado: esta reunião, ou o plano, não existe mais. A lista foi lida novamente.",
  "p.meetings.refused_settled":
    "Recusado: esta reunião já foi realizada ou cancelada, portanto é um registro e não é alterada. Ela foi lida novamente.",
  "p.meetings.refused_link":
    "Recusado: um registro para o qual um item aponta não é mais um registro deste plano. Nada foi salvo; escolha os registros novamente.",
  "p.meetings.refused_not_held":
    "Recusado: tarefas só são propostas a partir de uma reunião que foi realizada e tem ata.",
  "p.meetings.refused_commit_state":
    "Recusado: a reunião não foi realizada, ou uma destas ações passou a ter uma tarefa nesse meio-tempo. Nada foi criado; a reunião foi lida novamente, portanto confira de novo.",
  "p.meetings.refused_commit_invalid":
    "Recusado: uma proposta não está mais pronta. Nada foi criado; confira as propostas novamente.",
  "p.meetings.refused_proposals": "Recusado: envie de 1 a {count} propostas.",
  "p.meetings.tasks_heading": "Tarefas a partir das ações",
  "p.meetings.tasks_says":
    "Para cada ação ainda sem tarefa, você pode propor uma: um título de até {max} caracteres e um trecho da ata que a sustente, de pelo menos {min} caracteres (maiúsculas e espaçamento são ignorados). Um responsável é opcional e já deve constar nas tarefas deste plano ou em sua matriz de responsabilidades, como uma referência do tipo person:…; uma data de vencimento é opcional, de {first} a {last}; os pontos vão de 0 a {points}. Até {count} por vez. A verificação não grava nada; as tarefas são então criadas todas ou nenhuma, e uma ação recebe uma única tarefa.",
  "p.meetings.proposal_legend": "Ação {n}: {text}",
  "p.meetings.f_propose": "Propor uma tarefa para a ação {n}",
  "p.meetings.f_task_title":
    "Título da tarefa para a ação {n} (até {max} caracteres)",
  "p.meetings.f_excerpt":
    "Trecho da ata para a ação {n} (pelo menos {min} caracteres)",
  "p.meetings.f_assignee": "Responsável pela ação {n} (referência opcional)",
  "p.meetings.f_due": "Data de vencimento para a ação {n} (opcional)",
  "p.meetings.f_points": "Pontos para a ação {n} (opcional, de 0 a {max})",
  "p.meetings.check": "Verificar as propostas",
  "p.meetings.create_tasks": "Criar as tarefas",
  "p.meetings.none_chosen":
    "Nenhuma proposta foi escolhida, portanto nada foi enviado.",
  "p.meetings.checked_one":
    "Verificado, nada gravado: {n} proposta está pronta, {dropped} não.",
  "p.meetings.checked_other":
    "Verificado, nada gravado: {n} propostas estão prontas, {dropped} não.",
  "p.meetings.ready": "Pronta, sem responsável.",
  "p.meetings.ready_for": "Pronta, para {who}.",
  "p.meetings.not_all_ready":
    "Toda proposta escolhida deve estar pronta antes que as tarefas possam ser criadas: altere ou desmarque as que não estão e verifique novamente.",
  "p.meetings.check_again":
    "As propostas mudaram desde que foram verificadas: verifique-as novamente.",
  "p.meetings.confirm_tasks_one":
    "Criar {n} tarefa a partir das ações de {title}? Uma ação recebe uma única tarefa.",
  "p.meetings.confirm_tasks_other":
    "Criar {n} tarefas a partir das ações de {title}? Uma ação recebe uma única tarefa.",
  "p.meetings.tasks_made_one":
    "{n} tarefa criada; ela está vinculada abaixo da sua ação.",
  "p.meetings.tasks_made_other":
    "{n} tarefas criadas; cada uma está vinculada abaixo da sua ação.",
  "p.meetings.could_not_check": "Não foi possível verificar as propostas.",
  "p.meetings.could_not_create": "As tarefas não foram criadas.",
  "p.meetings.dropped_title_invalid":
    "Não pronta: o título está em branco ou é longo demais.",
  "p.meetings.dropped_excerpt_not_in_minutes":
    "Não pronta: o trecho não foi encontrado na ata, ou é curto demais.",
  "p.meetings.dropped_outcome_not_found":
    "Não pronta: a reunião não tem esse resultado.",
  "p.meetings.dropped_outcome_not_an_action":
    "Não pronta: esse resultado não é uma ação.",
  "p.meetings.dropped_outcome_already_has_a_task":
    "Não pronta: essa ação já tem uma tarefa.",
  "p.meetings.dropped_due_outside_window":
    "Não pronta: a data de vencimento está fora das datas permitidas.",
  "p.meetings.dropped_points_invalid":
    "Não pronta: os pontos estão fora do intervalo permitido.",
  "p.meetings.dropped_assignee_not_in_plan":
    "Não pronta: o responsável não é uma das pessoas deste plano (em suas tarefas ou em sua matriz de responsabilidades).",
  "p.meetings.dropped_outcome_proposed_twice":
    "Não pronta: a mesma ação é proposta duas vezes.",
  "p.meetings.dropped_other":
    "Não pronta, por um motivo que esta página desconhece.",
  "p.meetings.problem_proposals_count":
    "Proponha no máximo {count} tarefas por vez.",
  "p.meetings.problem_task_title":
    "Dê à tarefa um título de até {max} caracteres.",
  "p.meetings.problem_excerpt":
    "Cite um trecho de pelo menos {min} caracteres que conste da ata.",
  "p.meetings.problem_due":
    "A data de vencimento deve ficar entre uma semana antes da reunião e um ano depois dela.",
  "p.meetings.problem_points":
    "Os pontos são um número inteiro de 0 a {points}.",
  "p.lessons.title": "Lições aprendidas",
  "p.lessons.title_ppm": "Lições aprendidas — PPM",
  "p.lessons.link": "Lições aprendidas",
  "p.lessons.intro":
    "Uma lição é registrada em um portão de fase ou no encerramento do plano. Ela nunca é alterada: só pode ser retirada, e uma lição retirada deixa de ser mostrada (ela permanece na trilha de auditoria). Uma nova proposta cujo título se assemelha ao deste plano mostra as lições dele.",
  "p.lessons.record_heading": "Registrar uma lição",
  "p.lessons.record_says":
    "Quando (um portão ou o encerramento), o que é, um resumo de até {max} caracteres e, se quiser, um detalhe de até {bytes} bytes (uma letra acentuada ou de outro alfabeto ocupa mais de um) e a tarefa, o risco, o marco ou o problema deste plano sobre o qual ela trata.",
  "p.lessons.f_occasion": "Quando",
  "p.lessons.choose_occasion": "Escolha um portão ou o encerramento…",
  "p.lessons.closure": "Encerramento",
  "p.lessons.occasion_other": "Outra ocasião",
  "p.lessons.f_category": "O que é",
  "p.lessons.category_went_well": "Deu certo",
  "p.lessons.category_went_badly": "Deu errado",
  "p.lessons.category_recommendation": "Recomendação",
  "p.lessons.category_other": "Outro tipo",
  "p.lessons.f_summary": "Resumo (até {max} caracteres)",
  "p.lessons.f_detail": "Detalhe (opcional, até {bytes} bytes)",
  "p.lessons.f_link": "O registro sobre o qual trata (opcional)",
  "p.lessons.record": "Registrar a lição",
  "p.lessons.recorded": "Registrada: {summary}",
  "p.lessons.problem_occasion": "Escolha um portão ou o encerramento.",
  "p.lessons.problem_category":
    "Escolha deu certo, deu errado ou recomendação.",
  "p.lessons.problem_summary": "Escreva um resumo de até {max} caracteres.",
  "p.lessons.problem_detail": "O detalhe tem mais de {bytes} bytes.",
  "p.lessons.list_heading":
    "As lições do plano, das mais antigas às mais recentes",
  "p.lessons.loading": "Carregando as lições…",
  "p.lessons.none": "Nenhuma lição ainda.",
  "p.lessons.count_one": "{n} lição.",
  "p.lessons.count_other": "{n} lições.",
  "p.lessons.recorded_on": "registrada em {date}",
  "p.lessons.recorded_on_by": "registrada em {date} por {who}",
  "p.lessons.link_unlisted":
    "sobre um registro que não está entre as tarefas, os riscos, os marcos e os problemas do plano listados aqui",
  "p.lessons.withdraw": "Retirar a lição {summary}",
  "p.lessons.withdraw_short": "Retirar",
  "p.lessons.confirm_withdraw":
    "Retirar a lição “{summary}”? Ela deixará de ser mostrada, aqui e em novas propostas; permanece na trilha de auditoria.",
  "p.lessons.withdrawn": "Retirada: {summary}",
  "p.lessons.could_not_load": "Não foi possível carregar as lições.",
  "p.lessons.could_not_save": "A lição não foi alterada.",
  "p.lessons.refused_gone": "Recusado: o plano não existe mais.",
  "p.lessons.refused_withdrawn":
    "Recusado: essa lição já foi retirada. A lista foi lida novamente.",
  "p.lessons.refused_link":
    "Recusado: o registro escolhido não é mais um registro deste plano. Nada foi registrado; escolha-o novamente.",
  "p.lessons.refs_heading": "O que faz referência a um registro",
  "p.lessons.refs_says":
    "Escolha uma das tarefas, riscos, marcos ou problemas do plano para listar os itens de reunião e as lições que apontam para ele (até 100 de cada).",
  "p.lessons.f_target": "O registro",
  "p.lessons.choose_target": "Escolha um registro…",
  "p.lessons.show_refs": "Mostrar o que faz referência a ele",
  "p.lessons.problem_target": "Escolha um registro primeiro.",
  "p.lessons.refs_count_one": "{n} item faz referência a ele.",
  "p.lessons.refs_count_other": "{n} itens fazem referência a ele.",
  "p.lessons.refs_meetings": "Itens de reunião",
  "p.lessons.refs_lessons": "Lições que apontam para ele",
  "p.lessons.refs_agenda": "Pauta",
  "p.lessons.refused_no_record":
    "Esse registro não é mais um registro deste plano, ou você não pode lê-lo.",
  "p.lessons.could_not_read_refs":
    "Não foi possível ler o que faz referência a ele.",
  "p.lessons.similar": "Lições de planos semelhantes",
  "p.lessons.similar_named": "Lições de planos como {title}",
  "p.lessons.similar_heading": "Lições de planos como {title}",
  "p.lessons.similar_says":
    "Correspondência apenas pelo título, com todos os planos que têm lições, encerrados ou não, da melhor correspondência para a pior. A pontuação e a confiança são do comparador.",
  "p.lessons.similar_none": "Nenhum plano semelhante tem lições.",
  "p.lessons.match": "correspondência {score}, confiança {confidence}.",
  "p.lessons.match_title": "Títulos {score} parecidos.",
  "p.lessons.confidence_high": "alta",
  "p.lessons.confidence_medium": "média",
  "p.lessons.confidence_low": "baixa",
  "p.lessons.refused_proposal_gone":
    "Essa proposta não existe mais, ou você não pode lê-la.",
  "p.lessons.could_not_read_similar":
    "Não foi possível ler as lições de planos semelhantes.",
  "p.director_pack.open_by_severity": "Abertos por severidade: {list}",
  "p.director_pack.issues_overdue": "Problemas atrasados: {n}",
  "p.director_pack.issues_in_window":
    "No período: {raised} levantados, {resolved} resolvidos",
  "p.director_pack.oldest_issue":
    "Problema aberto mais antigo: {title} (dias corridos em aberto: {n})",
  "p.director_pack.approved_not_implemented":
    "Aprovadas mas ainda não implementadas: {n}",
  "p.director_pack.decided_in_window": "Decididas no período: {n}",
  "p.director_pack.changes_by_status": "Por status: {list}",
  "p.director_pack.oldest_change":
    "Solicitação de mudança aberta mais antiga: {title} (dias corridos em aberto: {n})",
  "p.live_pack.heading": "Leituras dos programas, agora",
  "p.live_pack.says":
    "Lidas agora a partir dos planos que você pode ler, desenhadas como o pacote da diretoria as desenha. Um pacote emitido mantém os números com que foi emitido; o ponto anterior do gráfico de febre vem somente de um pacote emitido.",
  "p.live_pack.f_ac": "Custo real para o valor agregado",
  "p.live_pack.ac_budget_lines": "Linhas de orçamento (o padrão)",
  "p.live_pack.ac_labour": "Custo de mão de obra registrado",
  "p.live_pack.ac_both": "Linhas de orçamento e custo de mão de obra",
  "p.live_pack.f_window":
    "Problemas e solicitações de mudança nos últimos dias ({min} a {max}; vazio para 30)",
  "p.live_pack.read_again": "Ler novamente",
  "p.live_pack.problem_window":
    "O número de dias deve ser um número inteiro de {min} a {max}, ou vazio para 30.",
  "p.live_pack.could_not_read_fever":
    "Não foi possível ler o gráfico de febre dos pulmões.",
  "p.live_pack.could_not_read_evm": "Não foi possível ler o valor agregado.",
  "p.live_pack.could_not_read_benefits": "Não foi possível ler os benefícios.",
  "p.live_pack.could_not_read_issues":
    "Não foi possível ler os problemas e as solicitações de mudança.",
  "p.live_pack.issues_heading": "Problemas e solicitações de mudança",
  "p.live_pack.issues_none":
    "Você não pode ler nenhum plano com problemas ou solicitações de mudança.",
  "p.live_pack.nothing": "Nada a mostrar.",
  "p.forecast.heading": "Previsão no término",
  "p.forecast.says":
    "A estimativa no término é o custo real até agora mais a estimativa para terminar. A estimativa para terminar é o último valor total de controle do projeto na moeda da previsão, ou, na falta dele, o que a última linha de base ainda planeja. Lida agora; nada é armazenado.",
  "p.forecast.absent_no_currency":
    "Sem previsão: o plano não tem linha de base de orçamento nem valor total de controle do projeto, portanto não há moeda em que prever.",
  "p.forecast.c_actual": "Custo real até agora",
  "p.forecast.c_etc": "Estimativa para terminar",
  "p.forecast.c_eac": "Estimativa no término",
  "p.forecast.c_plans": "Planos",
  "p.forecast.not_known": "desconhecido",
  "p.forecast.etc_tpc": "a partir do último valor total de controle do projeto",
  "p.forecast.etc_baseline": "a partir do que a linha de base ainda planeja",
  "p.forecast.no_baseline_used":
    "Nenhuma linha de base de orçamento foi usada.",
  "p.forecast.baseline_used":
    "Usa a versão {version} da linha de base de orçamento.",
  "p.forecast.as_of": "Lida em {date}.",
  "p.forecast.other_currency":
    "O custo real registrado em uma moeda diferente de {currency} não é somado.",
  "p.forecast.baselines_heading": "Linhas de base de orçamento",
  "p.forecast.baselines_says":
    "Uma linha de base é o orçamento aprovado por período, em uma moeda. Depois de aprovada, ela é congelada e nunca editada; uma nova linha de base é uma nova versão, e toda versão após a primeira exige um motivo. Uma linha de base contém de 1 a {max} períodos, cada um com um fim que não antecede o início e um valor planejado não inferior a zero.",
  "p.forecast.no_baselines": "Nenhuma linha de base de orçamento foi aprovada.",
  "p.forecast.version_line":
    "Versão {version} · {currency} · aprovada em {date}",
  "p.forecast.period_line": "{start} a {end}: {amount}",
  "p.forecast.first_baseline_heading": "Aprovar a primeira linha de base",
  "p.forecast.rebaseline_heading": "Aprovar a versão {version}",
  "p.forecast.f_currency": "Moeda (três letras)",
  "p.forecast.periods_legend": "Períodos",
  "p.forecast.f_start": "O período {n} começa em",
  "p.forecast.f_end": "O período {n} termina em",
  "p.forecast.f_amount": "Valor planejado do período {n}",
  "p.forecast.remove_period": "Remover o período {n}",
  "p.forecast.add_period": "Adicionar um período",
  "p.forecast.f_reason": "Motivo (opcional para a primeira)",
  "p.forecast.f_reason_needed": "Por que o orçamento é rebaseado (obrigatório)",
  "p.forecast.approve": "Aprovar a linha de base",
  "p.forecast.confirm_baseline":
    "Aprovar a versão {version} da linha de base de orçamento? Depois de aprovada, ela é congelada e não pode ser alterada nem removida.",
  "p.forecast.baseline_approved": "Versão {version} aprovada.",
  "p.forecast.problem_currency":
    "A moeda deve ser um código de três letras, como GBP ou EUR.",
  "p.forecast.problem_no_periods": "Informe pelo menos um período.",
  "p.forecast.problem_too_many":
    "Uma linha de base contém no máximo {max} períodos.",
  "p.forecast.problem_dates":
    "Cada período precisa de uma data de início e uma data de término.",
  "p.forecast.problem_order": "Um período não pode terminar antes de começar.",
  "p.forecast.problem_amount":
    "Cada valor planejado deve ser um número, não inferior a zero.",
  "p.forecast.problem_reason": "Uma nova linha de base exige um motivo.",
  "p.forecast.problem_reason_long":
    "O motivo é longo demais: no máximo {bytes} bytes.",
  "p.forecast.problem_same": "Isto é igual à última versão; nada foi enviado.",
  "p.forecast.refused_gone":
    "Este plano não existe mais, ou você não pode lê-lo.",
  "p.forecast.refused_baseline":
    "A linha de base foi recusada: outra versão pode ter sido aprovada nesse meio-tempo, o que torna um motivo necessário. A lista foi lida novamente.",
  "p.forecast.could_not_approve": "Não foi possível aprovar a linha de base.",
  "p.forecast.could_not_read": "Não foi possível ler a previsão.",
  "p.forecast.could_not_read_baselines":
    "Não foi possível ler as linhas de base de orçamento.",
  "p.forecast.could_not_read_lines":
    "Não foi possível ler as linhas de orçamento.",
  "p.forecast.tranches_heading": "Financiamento retido até um portão",
  "p.forecast.tranches_says":
    "Uma linha de orçamento com um portão de financiamento fica retida até que a etapa do plano chegue a esse portão; então ela pode ser liberada, uma única vez, e somente depois disso o custo real pode ser registrado nela. Uma liberação não pode ser desfeita.",
  "p.forecast.stage_now": "A etapa do plano: {stage}.",
  "p.forecast.stage_none": "O plano ainda não passou por nenhum portão.",
  "p.forecast.no_tranches":
    "Nenhuma linha de orçamento está retida até um portão.",
  "p.forecast.gate_line": "retida até {gate}",
  "p.forecast.state_released": "liberada em {date}",
  "p.forecast.state_held": "ainda retida: o portão não foi ultrapassado",
  "p.forecast.release": "Liberar {description}",
  "p.forecast.confirm_release":
    "Liberar {description}, retida até {gate}? Uma liberação não pode ser desfeita.",
  "p.forecast.released": "{description} liberada.",
  "p.forecast.refused_line_gone":
    "Essa linha de orçamento ou plano não existe mais.",
  "p.forecast.refused_register":
    "Recusado: o registro do programa está incompleto. Complete-o primeiro na página de completude do registro.",
  "p.forecast.refused_release":
    "Recusado: a linha não está mais retida, ou o seu portão não foi ultrapassado. As linhas foram lidas novamente.",
  "p.forecast.could_not_release": "Não foi possível liberar a linha.",
  "p.forecast.rollup_heading": "Previsão no término, consolidada",
  "p.forecast.rollup_says":
    "A partir de um plano, sobre os planos abaixo dele, até {max} níveis de profundidade, uma linha por moeda: moedas nunca são somadas entre si. Os planos sem moeda em que prever são contados, não somados.",
  "p.forecast.f_plan": "Plano",
  "p.forecast.choose_plan": "Escolha um plano",
  "p.forecast.f_depth": "Níveis de profundidade (1 a {max}; vazio para todos)",
  "p.forecast.show_rollup": "Mostrar a previsão",
  "p.forecast.problem_plan": "Escolha um plano.",
  "p.forecast.problem_depth":
    "Os níveis de profundidade devem ser um número inteiro de 1 a {max}, ou vazio.",
  "p.forecast.refused_plan_gone":
    "Esse plano não existe mais, ou você não pode lê-lo.",
  "p.forecast.could_not_read_rollup":
    "Não foi possível ler a previsão consolidada.",
  "p.forecast.rollup_line_one": "A partir de {name}: {n} plano lido, {date}.",
  "p.forecast.rollup_line_other":
    "A partir de {name}: {n} planos lidos, {date}.",
  "p.forecast.rollup_truncated":
    "A árvore é maior do que o serviço percorre; os números cobrem parte dela.",
  "p.forecast.rollup_revisits_one":
    "{n} plano foi alcançado duas vezes e contado uma só.",
  "p.forecast.rollup_revisits_other":
    "{n} planos foram alcançados duas vezes e contados uma só.",
  "p.forecast.rollup_no_currency_one":
    "{n} plano não tem moeda em que prever e não é somado.",
  "p.forecast.rollup_no_currency_other":
    "{n} planos não têm moeda em que prever e não são somados.",
  "p.forecast.rollup_empty": "Nenhum plano aqui tem previsão.",
  "p.value.heading": "Valor realizado",
  "p.value.says":
    "Lido agora a partir do valor registrado; nada é armazenado. O retorno sobre o investimento compara cada ponto de valor com o custo real registrado, e é retido quando as linhas de orçamento usam mais de uma moeda, pois nenhuma moeda é convertida. Um número que não pode ser informado diz por quê, nunca zero.",
  "p.value.roi": "Retorno sobre o investimento: {pct}",
  "p.value.roi_absent": "Retorno sobre o investimento não informado: {reason}",
  "p.value.measured_share": "evidência medida: {pct} dos pontos de valor",
  "p.value.ttv_one":
    "Tempo até o valor: mediana de {p50} dias corridos, percentil 85 de {p85} ({n} observação)",
  "p.value.ttv_other":
    "Tempo até o valor: mediana de {p50} dias corridos, percentil 85 de {p85} ({n} observações)",
  "p.value.ttv_absent": "Tempo até o valor não informado: {reason}",
  "p.value.adoption":
    "Adoção: {pct} ({active} de {target} usuários ativos em {days} dias)",
  "p.value.adoption_definition": "um usuário ativo é: {definition}",
  "p.value.adoption_absent": "Adoção não informada: {reason}",
  "p.value.adoption_none": "Nenhuma leitura de adoção está registrada.",
  "p.value.absent_unrealized": "nenhum valor foi registrado",
  "p.value.absent_no_denominator":
    "não há nada com que compará-lo (nenhum custo real, ou nenhuma meta)",
  "p.value.absent_mixed_currency":
    "as linhas de orçamento usam mais de uma moeda",
  "p.value.absent_no_baseline": "não há linha de base de orçamento",
  "p.value.absent_no_responses": "ninguém respondeu",
  "p.value.absent_overflow": "o número é grande demais para ser calculado",
  "p.value.absent_other": "o serviço não disse por quê",
  "p.value.case_heading": "Metas do business case",
  "p.value.no_targets": "Nenhuma meta do business case está registrada.",
  "p.value.target_line":
    "{metric}: de {baseline} para {target} · {source} · aprovada em {date}",
  "p.value.source_charter": "o termo de abertura",
  "p.value.source_gate_review": "uma revisão de portão",
  "p.value.source_other": "outra fonte",
  "p.value.to_costs": "Registrar uma meta na página de custos",
  "p.value.point_heading": "Registrar um valor observado",
  "p.value.point_says":
    "Um valor é medido, estimado ou afirmado. Com uma moeda, é uma quantia em dinheiro; sem ela, um número inteiro na unidade da própria medida. Um ponto de valor não pode ser alterado aqui depois de registrado. Um ponto de valor por plano pode ser o primeiro valor mensurável: ele interrompe o relógio do tempo até o valor, que a primeira meta do business case iniciou, e não pode ser movido.",
  "p.value.f_value": "Valor",
  "p.value.f_currency": "Moeda (três letras, ou vazio)",
  "p.value.f_method": "Como foi obtido",
  "p.value.method_measured": "medido",
  "p.value.method_estimated": "estimado",
  "p.value.method_asserted": "afirmado",
  "p.value.f_evidence": "Evidência (um link ou referência, opcional)",
  "p.value.f_first": "Este é o primeiro valor mensurável",
  "p.value.record_point": "Registrar o valor",
  "p.value.confirm_first":
    "Registrar este como o primeiro valor mensurável do plano? Isso interrompe o relógio do tempo até o valor e não pode ser movido.",
  "p.value.point_recorded": "Valor registrado.",
  "p.value.problem_currency":
    "A moeda deve ser um código de três letras, ou vazio.",
  "p.value.problem_amount": "O valor deve ser uma quantia em dinheiro.",
  "p.value.problem_whole": "Sem moeda, o valor deve ser um número inteiro.",
  "p.value.problem_method": "Escolha como o valor foi obtido.",
  "p.value.refused_first_taken":
    "Recusado: este plano já tem um primeiro valor mensurável; o relógio é interrompido uma única vez.",
  "p.value.refused_value": "O valor foi recusado.",
  "p.value.refused_gone": "Este plano não existe mais, ou você não pode lê-lo.",
  "p.value.could_not_read": "Não foi possível ler o valor realizado.",
  "p.value.could_not_record_point": "Não foi possível registrar o valor.",
  "p.value.case_says":
    "Uma meta é uma medida prometida no termo de abertura ou em uma revisão de portão, com o seu valor na aprovação e o valor prometido, como números inteiros na unidade da medida. A data em que é aprovada inicia o relógio do tempo até o valor e não pode ser alterada, e uma meta não pode ser alterada nem removida aqui.",
  "p.value.to_benefits": "O valor realizado está na página de benefícios.",
  "p.value.f_metric": "Medida",
  "p.value.f_baseline": "Valor na aprovação",
  "p.value.f_target": "Valor prometido",
  "p.value.f_unit": "Unidade (opcional)",
  "p.value.f_source": "Prometida em",
  "p.value.record_target": "Registrar a meta",
  "p.value.confirm_target":
    "Registrar a meta {metric}? Ela não pode ser alterada nem removida aqui.",
  "p.value.target_recorded": "Meta {metric} registrada.",
  "p.value.problem_metric": "Dê um nome à medida.",
  "p.value.problem_target_values":
    "O valor na aprovação e o valor prometido devem ser números inteiros.",
  "p.value.problem_source": "Escolha onde a meta foi prometida.",
  "p.value.refused_target": "A meta foi recusada.",
  "p.value.could_not_record_target": "Não foi possível registrar a meta.",
  "p.scoring.rank_by": "Classificar por",
  "p.scoring.model_smart": "Smart Score (a partir de evidências)",
  "p.scoring.model_wsjf": "WSJF (trabalho mais curto ponderado primeiro)",
  "p.scoring.model_rice": "RICE (alcance, impacto, confiança, esforço)",
  "p.scoring.models_say":
    "O Smart Score é obtido das evidências que o serviço guarda; WSJF e RICE são pontuados a partir de estimativas de uma pessoa, definidas para cada plano no botão Estimativas. Um plano sem todas as entradas de que um modelo precisa fica sem pontuação e é classificado por último, nunca com pontuação zero. A faixa é a do Smart Score.",
  "p.scoring.of_plans_one":
    "{length} de {scored_of} plano, classificado por {model}",
  "p.scoring.of_plans_other":
    "{length} de {scored_of} planos, classificados por {model}",
  "p.scoring.ranked_by": "Planos classificados por {model}",
  "p.scoring.waiting_for": "Aguardando",
  "p.scoring.open": "Estimativas",
  "p.scoring.open_named": "Abrir as estimativas de {name}",
  "p.scoring.heading": "Estimativas de {name}",
  "p.scoring.form": "As estimativas de {name}",
  "p.scoring.says":
    "WSJF e RICE são pontuados a partir destas estimativas. Uma pontuação com qualquer entrada ausente não é dada, nunca é adivinhada, e um zero informado não é uma entrada ausente. Salvar substitui todas as estimativas: um campo vazio a apaga.",
  "p.scoring.last_set": "Definidas pela última vez {when}.",
  "p.scoring.none_set": "Nenhuma estimativa foi definida.",
  "p.scoring.wsjf_inputs":
    "Entradas do WSJF, cada uma um número inteiro de {min} a {max}",
  "p.scoring.rice_inputs": "Entradas do RICE",
  "p.scoring.input_business_value": "Valor de negócio",
  "p.scoring.input_time_criticality": "Criticidade de tempo",
  "p.scoring.input_risk_reduction":
    "Redução de risco ou viabilização de oportunidade",
  "p.scoring.input_job_size": "Tamanho do trabalho",
  "p.scoring.input_reach": "Alcance (pessoas ou eventos em um período)",
  "p.scoring.input_impact": "Impacto",
  "p.scoring.input_confidence": "Confiança (porcentagem)",
  "p.scoring.input_effort": "Esforço (pessoas-mês)",
  "p.scoring.input_other": "outra entrada",
  "p.scoring.component_cost_of_delay": "Custo do atraso",
  "p.scoring.impact_minimal": "mínimo",
  "p.scoring.impact_low": "baixo",
  "p.scoring.impact_medium": "médio",
  "p.scoring.impact_high": "alto",
  "p.scoring.impact_massive": "enorme",
  "p.scoring.impact_value": "{value} (impacto {word})",
  "p.scoring.not_estimated": "não estimado",
  "p.scoring.save": "Salvar as estimativas",
  "p.scoring.close": "Fechar as estimativas",
  "p.scoring.unchanged":
    "As estimativas estão como armazenadas, portanto nada foi enviado.",
  "p.scoring.saved": "Estimativas de {name} salvas.",
  "p.scoring.could_not_read": "Não foi possível ler as estimativas.",
  "p.scoring.could_not_save": "Não foi possível salvar as estimativas.",
  "p.scoring.refused_gone":
    "Esse plano não existe mais, ou você não pode lê-lo.",
  "p.scoring.refused_inputs":
    "As estimativas foram recusadas: uma entrada está fora do seu intervalo.",
  "p.scoring.problem_wsjf":
    "{input} deve ser um número inteiro de {min} a {max}.",
  "p.scoring.problem_reach":
    "{input} deve ser um número inteiro, não inferior a zero.",
  "p.scoring.problem_impact": "{input} deve ser 0,25, 0,5, 1, 2 ou 3.",
  "p.scoring.problem_confidence":
    "{input} deve ser um número inteiro de 0 a 100.",
  "p.scoring.problem_effort": "{input} deve ser maior que zero.",
  "p.scoring.problem_number": "{input} deve ser um número.",
  "p.scoring.formula_wsjf":
    "WSJF é (valor de negócio + criticidade de tempo + redução de risco ou viabilização de oportunidade) ÷ tamanho do trabalho.",
  "p.scoring.formula_rice": "RICE é alcance × impacto × confiança ÷ esforço.",
  "p.scoring.score_is": "Pontuação: {score}",
  "p.scoring.no_score": "Sem pontuação: ela aguarda estas entradas.",
  "p.scoring.needs_delay_part": "faz parte do custo do atraso",
  "p.scoring.needs_divisor":
    "é o divisor: sem ele não há valor por unidade de trabalho",
  "p.scoring.needs_reach":
    "diz quantas pessoas o trabalho atinge em um período",
  "p.scoring.needs_impact":
    "diz quanto o trabalho importa para cada pessoa atingida",
  "p.scoring.needs_confidence":
    "desconta as estimativas conforme o grau de certeza",
  "p.scoring.needs_other": "a pontuação precisa dela",
  "p.scoring.means_business_value":
    "valor relativo para o usuário ou para o negócio, de 1 a 20",
  "p.scoring.means_time_criticality":
    "com que rapidez o valor se perde se houver atraso, de 1 a 20",
  "p.scoring.means_risk_reduction":
    "o risco que elimina ou a oportunidade que abre, de 1 a 20",
  "p.scoring.means_cost_of_delay":
    "a soma dos três acima: o que um atraso custa",
  "p.scoring.means_job_size":
    "o tamanho relativo do trabalho, de 1 a 20; o divisor",
  "p.scoring.means_reach": "pessoas ou eventos atingidos em um período",
  "p.scoring.means_impact": "0,25 mínimo, 0,5 baixo, 1 médio, 2 alto, 3 enorme",
  "p.scoring.means_confidence": "quão certas são as estimativas",
  "p.scoring.means_effort": "pessoas-mês de trabalho; o divisor",
  "p.scoring.means_other": "um componente que esta página desconhece",
  "p.frontier.heading": "Escolher um portfólio",
  "p.frontier.says":
    "O gerador pontua cada plano que você pode ler pelo seu Smart Score e o precifica pelas suas linhas de orçamento; depois toma planos pela pontuação por unidade de custo até o limite se esgotar. Os planos que devem ser incluídos são tomados primeiro, qualquer que seja o custo. Um plano sem Smart Score é deixado de fora; um plano sem linhas de orçamento não custa nada. A escolha nem sempre é a melhor possível, por isso um limite que não compra mais valor do que um mais barato é marcado, não escondido.",
  "p.frontier.must_include": "Planos que toda escolha deve incluir (até {max})",
  "p.frontier.plans_loading": "Lendo os planos…",
  "p.frontier.plans_none": "Não há planos que você possa ler.",
  "p.frontier.frontier_heading": "Fronteira eficiente",
  "p.frontier.frontier_says":
    "O gerador é executado uma vez em cada limite, e nada é salvo. Distribua de {min} a {max} limites uniformemente, de zero até o custo de todos os planos que podem ser precificados (vazio para {fallback}), ou indique você mesmo até {list} limites, cada um um valor não inferior a zero.",
  "p.frontier.f_currency": "Moeda dos custos (três letras)",
  "p.frontier.f_how": "Quais limites de orçamento",
  "p.frontier.how_steps": "Distribuídos uniformemente",
  "p.frontier.how_caps": "Limites que eu indicar",
  "p.frontier.f_steps": "Quantos limites (vazio para {fallback})",
  "p.frontier.f_caps": "Limites de orçamento, um por linha (até {list})",
  "p.frontier.run": "Traçar a fronteira",
  "p.frontier.value_is":
    "O valor é a soma dos Smart Scores dos planos escolhidos.",
  "p.frontier.table": "A fronteira, ponto a ponto",
  "p.frontier.col_cap": "Limite de orçamento",
  "p.frontier.col_cost": "Custo da escolha",
  "p.frontier.col_value": "Valor",
  "p.frontier.col_included": "Planos escolhidos",
  "p.frontier.col_more": "Compra mais valor",
  "p.frontier.more_yes": "Sim",
  "p.frontier.more_no": "Não: um limite mais barato alcança o mesmo",
  "p.frontier.show_choice": "Mostrar a escolha",
  "p.frontier.show_choice_named": "Mostrar a escolha em {cap}",
  "p.frontier.save_point": "Salvar como rascunho",
  "p.frontier.save_point_named": "Salvar a escolha em {cap} como rascunho",
  "p.frontier.point_name": "Fronteira em {cap}",
  "p.frontier.axis_cost": "Custo dos planos escolhidos",
  "p.frontier.axis_value": "Valor (Smart Score somado)",
  "p.frontier.chart_points_one": "{n} limite de orçamento.",
  "p.frontier.chart_points_other": "{n} limites de orçamento.",
  "p.frontier.chart_best": "O maior valor, {value}, custa {cost}.",
  "p.frontier.chart_dominated_one":
    "{n} limite não compra mais valor do que um mais barato.",
  "p.frontier.chart_dominated_other":
    "{n} limites não compram mais valor do que um mais barato.",
  "p.frontier.legend_efficient": "compra mais valor",
  "p.frontier.legend_dominated":
    "não compra mais valor do que um limite mais barato",
  "p.frontier.no_points": "A fronteira não tem pontos.",
  "p.frontier.generate_heading": "Gerar um rascunho",
  "p.frontier.generate_says":
    "Salvo como um cenário em rascunho, para avaliar, comparar e confirmar como qualquer outro. Um nome é necessário, de no máximo {bytes} bytes. Com uma moeda, cada plano é precificado nela; um limite precisa da sua moeda.",
  "p.frontier.g_name": "Nome do rascunho",
  "p.frontier.g_cap": "Limite de orçamento (opcional)",
  "p.frontier.g_currency": "Moeda (necessária com um limite)",
  "p.frontier.generate": "Gerar o rascunho",
  "p.frontier.draft_saved": "Rascunho {name} salvo; ele está na lista acima.",
  "p.frontier.choice_at": "O que um limite de {cap} escolhe",
  "p.frontier.choice_of": "O que {name} escolheu",
  "p.frontier.chosen_count_one": "{n} plano escolhido de {of}.",
  "p.frontier.chosen_count_other": "{n} planos escolhidos de {of}.",
  "p.frontier.col_plan": "Plano na escolha",
  "p.frontier.col_chosen": "Escolhido ou não",
  "p.frontier.col_score": "Seu Smart Score",
  "p.frontier.col_plan_cost": "Seu custo",
  "p.frontier.col_why": "Por que foi deixado de fora",
  "p.frontier.chosen": "escolhido",
  "p.frontier.left_out": "deixado de fora",
  "p.frontier.no_score": "sem Smart Score",
  "p.frontier.plan_unknown":
    "Um plano que você não pode ler, ou que não existe mais",
  "p.frontier.reason_no_score":
    "não tem Smart Score, o que não é uma pontuação zero",
  "p.frontier.reason_foreign_currency":
    "suas linhas de orçamento estão todas em outra moeda, portanto não pode ser precificado",
  "p.frontier.reason_over_cap": "o limite se esgotou antes dele",
  "p.frontier.reason_must_include_conflict":
    "deve ser incluído, mas não é um plano que você possa ler",
  "p.frontier.reason_other":
    "deixado de fora por um motivo que esta página desconhece",
  "p.frontier.problem_currency": "Informe a moeda com três letras.",
  "p.frontier.problem_steps":
    "O número de limites deve ser um número inteiro de {min} a {max}.",
  "p.frontier.problem_caps_none": "Indique pelo menos um limite.",
  "p.frontier.problem_caps_many": "Indique no máximo {list} limites.",
  "p.frontier.problem_caps_amount":
    "Cada limite deve ser um valor não inferior a zero.",
  "p.frontier.problem_must_include":
    "Escolha no máximo {list} planos a incluir.",
  "p.frontier.problem_name": "Dê um nome ao rascunho.",
  "p.frontier.problem_name_long": "O nome tem mais de {bytes} bytes.",
  "p.frontier.problem_cap_currency": "Um limite precisa da sua moeda.",
  "p.frontier.problem_cap_amount":
    "O limite deve ser um valor não inferior a zero.",
  "p.frontier.refused_frontier": "A fronteira foi recusada.",
  "p.frontier.refused_generate": "O rascunho foi recusado.",
  "p.frontier.could_not_read": "Não foi possível traçar a fronteira.",
  "p.frontier.could_not_generate": "Não foi possível gerar o rascunho.",
  "p.frontier.rollback": "Reverter",
  "p.frontier.rollback_named": "Reverter {name}",
  "p.frontier.rollback_says":
    "Um cenário confirmado pode ser revertido a rascunho, após uma pergunta; isso não altera nada além do cenário.",
  "p.frontier.confirm_rollback":
    "Reverter {name} a rascunho? Ele deixa de ser o portfólio confirmado; a confirmação permanece na trilha de auditoria.",
  "p.frontier.rolled_back": "{name} voltou a ser um rascunho.",
  "p.frontier.refused_rollback_status":
    "{name} não está mais confirmado, portanto não há nada a reverter. A lista foi lida novamente.",
  "p.frontier.refused_rollback_gone":
    "{name} não existe mais, ou você não pode lê-lo. A lista foi lida novamente.",
  "p.frontier.could_not_roll_back": "Não foi possível reverter o cenário.",
  "p.frontier.status_draft": "rascunho",
  "p.frontier.status_committed": "confirmado",
  "p.frontier.violations_one": "{n} restrição violada",
  "p.frontier.violations_other": "{n} restrições violadas",
  "p.frontier.compare_note":
    "B menos A, uma moeda por vez; moedas nunca são somadas entre si.",
  "p.intake.edit": "Editar",
  "p.intake.edit_named": "Editar o rascunho {title}",
  "p.intake.edit_heading": "Editar {title}",
  "p.intake.edit_heading_loading": "Lendo o rascunho…",
  "p.intake.edit_form": "Os campos do rascunho",
  "p.intake.edit_says":
    "Somente um rascunho pode ser editado, e é também nele que uma proposta é marcada como pedido externo. Um título é obrigatório; o título e o resumo têm no máximo {bytes} bytes cada; um valor precisa da sua moeda; uma contrapartida indicada tem de {min} a {max} caracteres. Salvar substitui todos os campos.",
  "p.intake.save": "Salvar o rascunho",
  "p.intake.saved": "{title} salvo.",
  "p.intake.unchanged": "Nada mudou, portanto nada foi enviado.",
  "p.intake.could_not_read_draft": "Não foi possível ler o rascunho.",
  "p.intake.problem_not_draft": "Somente um rascunho pode ser editado.",
  "p.intake.problem_title_long": "O título tem mais de {bytes} bytes.",
  "p.intake.problem_summary_long": "O resumo tem mais de {bytes} bytes.",
  "p.intake.refused_not_draft":
    "A edição foi recusada: a proposta não é mais um rascunho. A lista foi lida novamente.",
  "p.intake.refused_edit":
    "A edição foi recusada: um campo está fora do que o serviço aceita.",
  "p.intake.forecast_heading": "Quantas serão aprovadas",
  "p.intake.forecast_says":
    "Obtida por simulação a partir de quantas propostas foram aprovadas em cada período passado; as propostas que chegam não a alimentam. Uma previsão precisa de pelo menos {needed} períodos de histórico.",
  "p.intake.f_periods": "Períodos à frente (1 a {max}; vazio para {fallback})",
  "p.intake.f_period_days":
    "Dias em um período (1 a {max}; vazio para {fallback})",
  "p.intake.f_history":
    "Períodos de histórico (1 a {max}; vazio para {fallback})",
  "p.intake.forecast_run": "Prever",
  "p.intake.problem_periods":
    "Os períodos à frente devem ser um número inteiro de 1 a {periods}.",
  "p.intake.problem_period_days":
    "Um período deve ser um número inteiro de dias, de 1 a {days}.",
  "p.intake.problem_history":
    "O histórico deve ser um número inteiro de períodos, de 1 a {history}.",
  "p.intake.refused_forecast": "A previsão foi recusada.",
  "p.intake.could_not_forecast": "Não foi possível obter a previsão.",
  "p.intake.window": "Histórico de {from} a {to}, em períodos de {days} dias.",
  "p.intake.at_least_label_one":
    "No próximo período de {n} ({days} dias), pelo menos",
  "p.intake.at_least_label_other":
    "Nos próximos {n} períodos ({days} dias), pelo menos",
  "p.intake.at_least_means": "com 85% de confiança: o número em que se basear",
  "p.intake.median_label": "Tipicamente",
  "p.intake.median_means":
    "metade das simulações aprova mais, metade menos: cara ou coroa, não um compromisso",
  "p.intake.at_most_label": "No máximo",
  "p.intake.at_most_means":
    "o extremo otimista: 85% das simulações não aprovam mais que isso",
  "p.intake.proposals_n_one": "{n} proposta",
  "p.intake.proposals_n_other": "{n} propostas",
  "p.intake.absent_history":
    "Sem previsão: ela precisa de pelo menos {needed} períodos de histórico, e há {n}.",
  "p.intake.absent_other":
    "Nenhuma previsão pôde ser obtida a partir deste histórico.",
  "p.intake.approvals_history":
    "Aprovadas por período, da mais antiga à mais recente: {list}",
  "p.intake.arrivals_history":
    "Propostas que chegam por período, da mais antiga à mais recente (mostradas ao lado; elas não alimentam a previsão): {list}",
  "p.planner.heading": "Tempo planejado",
  "p.planner.says":
    "Reserve tempo para uma das suas tarefas abertas em um dia. Um bloco é só seu e nunca altera a data de vencimento da tarefa; ele termina depois de começar e dura no máximo 12 horas. Os blocos podem se sobrepor: uma sobreposição é mostrada como conflito, e um bloco que termina quando o seguinte começa não é um conflito. Os horários estão no seu fuso horário, {tz}.",
  "p.planner.day_nav": "Escolher o dia",
  "p.planner.previous_day": "Dia anterior",
  "p.planner.back_to_today": "Voltar para hoje",
  "p.planner.next_day": "Próximo dia",
  "p.planner.f_day": "Dia",
  "p.planner.show_day": "Mostrar o dia",
  "p.planner.problem_day": "Informe o dia como uma data.",
  "p.planner.day_heading": "Planejado em {day}",
  "p.planner.day_hours":
    "Este dia tem {hours} horas, por causa da mudança de horário.",
  "p.planner.nothing_planned": "Nada está planejado neste dia.",
  "p.planner.clashes_one": "Conflita com {n} outro bloco",
  "p.planner.clashes_other": "Conflita com {n} outros blocos",
  "p.planner.remove": "Remover",
  "p.planner.remove_named": "Remover o bloco de {title}, {time}",
  "p.planner.confirm_remove":
    "Remover o bloco de {title}, {time}? A tarefa em si não é alterada.",
  "p.planner.removed": "Bloco de {title}, {time}, removido.",
  "p.planner.omitted_one":
    "{n} bloco em uma tarefa que você não pode mais ler não é mostrado.",
  "p.planner.omitted_other":
    "{n} blocos em tarefas que você não pode mais ler não são mostrados.",
  "p.planner.due_heading": "Com vencimento neste dia",
  "p.planner.nothing_due": "Nada atribuído a você vence neste dia.",
  "p.planner.loading": "Lendo o dia…",
  "p.planner.form": "Planejar tempo para uma tarefa",
  "p.planner.form_says":
    "O bloco é em {day}, no seu fuso horário. Um que passe da meia-noite é planejado como dois.",
  "p.planner.f_task": "Tarefa",
  "p.planner.choose_task": "Escolha uma das suas tarefas abertas",
  "p.planner.loading_tasks": "Lendo as suas tarefas…",
  "p.planner.task_option": "{title}, em {plan}",
  "p.planner.no_tasks":
    "Nada está atribuído a você, portanto não há tarefa para a qual planejar tempo.",
  "p.planner.f_start": "Começa",
  "p.planner.f_end": "Termina",
  "p.planner.plan_time": "Planejar este tempo",
  "p.planner.planned": "Planejado {title}, {time}.",
  "p.planner.problem_task": "Escolha uma tarefa.",
  "p.planner.problem_start": "Informe a hora em que começa.",
  "p.planner.problem_end": "Informe a hora em que termina.",
  "p.planner.problem_start_skipped":
    "O relógio pula a hora de início neste dia; escolha outra.",
  "p.planner.problem_end_skipped":
    "O relógio pula a hora de término neste dia; escolha outra.",
  "p.planner.problem_order": "Um bloco deve terminar depois de começar.",
  "p.planner.problem_too_long": "Um bloco dura no máximo 12 horas.",
  "p.planner.refused_zone":
    "O fuso horário do seu navegador não foi aceito, portanto o dia não pode ser mostrado.",
  "p.planner.refused_slot":
    "O bloco não foi aceito: ele deve terminar depois de começar e durar no máximo 12 horas, e uma pessoa tem no máximo {max} blocos.",
  "p.planner.refused_task":
    "Essa tarefa não existe mais, ou está em um plano que você não pode ler.",
  "p.planner.refused_gone":
    "Esse bloco já não existe. O dia foi lido novamente.",
  "p.planner.could_not_load": "Não foi possível ler o dia.",
  "p.planner.could_not_plan": "Não foi possível planejar o tempo.",
  "p.planner.could_not_remove": "Não foi possível remover o bloco.",
  "p.feed.heading": "Feed de calendário",
  "p.feed.says":
    "Um endereço privado que um aplicativo de calendário pode assinar: as tarefas abertas atribuídas a você que têm data de vencimento, os marcos e as sprints dos seus planos e o tempo que você planejou. Um segundo endereço é um feed das suas notificações para um leitor de feeds. Quem tiver um endereço pode ler o que ele mostra, portanto mantenha-o só para você.",
  "p.feed.rules":
    "Um endereço é mostrado uma única vez, aqui, e esta página não o guarda; ele dura 365 dias. Criar um novo interrompe imediatamente qualquer anterior.",
  "p.feed.issue": "Criar um endereço de feed",
  "p.feed.confirm_issue":
    "Criar um novo endereço de feed? Qualquer endereço criado antes deixa de funcionar imediatamente.",
  "p.feed.issued":
    "Seus novos endereços estão abaixo. Copie-os agora: eles não são mostrados novamente.",
  "p.feed.calendar_address": "Endereço do calendário (iCalendar)",
  "p.feed.atom_address": "Endereço do feed de notificações (Atom)",
  "p.feed.revoke": "Interromper todos os endereços de feed",
  "p.feed.confirm_revoke":
    "Interromper todos os endereços de feed que você criou? Um aplicativo de calendário que use um deles deixa de atualizar imediatamente.",
  "p.feed.revoked":
    "Todos os endereços de feed que você criou deixaram de funcionar.",
  "p.feed.no_address":
    "A resposta não trouxe nenhum endereço que esta página possa mostrar, portanto nenhum é mostrado. Qualquer endereço anterior foi interrompido; crie outro se precisar.",
  "p.feed.refused_forbidden": "Você não pode gerenciar um endereço de feed.",
  "p.feed.could_not_issue": "Não foi possível criar um endereço de feed.",
  "p.feed.could_not_revoke":
    "Não foi possível interromper os endereços de feed.",
  "p.watching.heading": "O que você segue",
  "p.watching.says":
    "Você é avisado quando algo que segue muda. Você segue uma tarefa quando a cria, comenta nela ou a recebe, a menos que tenha desativado isso nas suas opções de notificação. Ninguém mais é avisado de quem segue. Um plano é mostrado pelo nome; qualquer outra coisa é mostrada pelo seu tipo e pelo início da sua referência.",
  "p.watching.count_one": "Você segue {n} item.",
  "p.watching.count_other": "Você segue {n} itens.",
  "p.watching.capped": "Somente os {max} mais recentes são mostrados.",
  "p.watching.none": "No momento, você não segue nada.",
  "p.watching.named_plan": "Plano {name}",
  "p.watching.unnamed": "{kind}, referência {ref}",
  "p.watching.kind_plan": "Plano",
  "p.watching.kind_task": "Tarefa",
  "p.watching.kind_risk": "Risco",
  "p.watching.kind_proposal": "Proposta",
  "p.watching.kind_idea": "Ideia",
  "p.watching.kind_canvas_item": "Item de canvas",
  "p.watching.kind_budget_line": "Linha de orçamento",
  "p.watching.kind_scenario": "Cenário",
  "p.watching.kind_document_version": "Versão de documento",
  "p.watching.kind_other": "Outra coisa",
  "p.watching.stop": "Deixar de seguir",
  "p.watching.stop_named": "Deixar de seguir {what}",
  "p.watching.confirm_stop":
    "Deixar de seguir {what}? Você não será mais avisado quando mudar.",
  "p.watching.stopped": "Você não segue mais {what}.",
  "p.watching.refused": "Deixar de seguir isso não foi aceito.",
  "p.watching.loading": "Lendo o que você segue…",
  "p.watching.could_not_load": "Não foi possível ler o que você segue.",
  "p.watching.could_not_stop": "Não foi possível deixar de seguir.",
  "p.workprofile.heading": "Trabalho ao longo do tempo",
  "p.workprofile.says":
    "O trabalho levado pelas tarefas de todos os planos que você pode ler, distribuído pelos dias úteis de cada tarefa conforme o seu contorno, em minutos inteiros. O trabalho que não pode ser posicionado (uma tarefa ainda não agendada, ou uma sem dia útil) é contado à parte, nunca descartado.",
  "p.workprofile.rules":
    "As semanas vão de segunda a domingo. Deixe as datas vazias para cobrir todo o trabalho posicionado. Uma janela abrange no máximo dez anos e {max} períodos.",
  "p.workprofile.form": "Mostrar o trabalho ao longo do tempo",
  "p.workprofile.f_period": "Períodos",
  "p.workprofile.period_week": "Semanas",
  "p.workprofile.period_day": "Dias",
  "p.workprofile.period_month": "Meses",
  "p.workprofile.f_from": "De (vazio para o primeiro dia posicionado)",
  "p.workprofile.f_to": "Até (vazio para o último dia posicionado)",
  "p.workprofile.show": "Mostrar o trabalho",
  "p.workprofile.problem_from": "Informe o primeiro dia como uma data.",
  "p.workprofile.problem_to": "Informe o último dia como uma data.",
  "p.workprofile.problem_order":
    "O primeiro dia não pode ser posterior ao último.",
  "p.workprofile.problem_too_long": "Uma janela abrange no máximo dez anos.",
  "p.workprofile.problem_periods":
    "Essa janela contém mais de {max} períodos; escolha períodos mais longos ou uma janela mais curta.",
  "p.workprofile.refused":
    "Essa janela não foi aceita: sem trabalho posicionado ainda, informe as duas datas e fique dentro dos períodos indicados acima.",
  "p.workprofile.none":
    "Nenhum plano que você possa ler tem uma tarefa com trabalho.",
  "p.workprofile.summary":
    "{range}: {work} de trabalho, {unplaced} dele não posicionado.",
  "p.workprofile.plans_table": "Trabalho por plano",
  "p.workprofile.periods_table": "Trabalho por período",
  "p.workprofile.col_plan": "Plano",
  "p.workprofile.col_work": "Trabalho",
  "p.workprofile.col_unplaced": "Não posicionado",
  "p.workprofile.col_period": "Período",
  "p.workprofile.plan_unnamed": "Um plano sem nome",
  "p.workprofile.could_not_load":
    "Não foi possível ler o trabalho ao longo do tempo.",
  "p.timeentry.heading": "Registrar tempo",
  "p.timeentry.says":
    "O tempo é registrado em seu nome, em minutos inteiros, de 1 minuto a {max} em uma data, em um plano e, se quiser, em uma de suas tarefas. Uma semana aprovada não aceita mais tempo. Um lançamento não pode ser alterado nem removido depois de registrado.",
  "p.timeentry.open": "Registrar algum tempo",
  "p.timeentry.form": "Registrar tempo em um plano",
  "p.timeentry.f_plan": "Plano",
  "p.timeentry.choose_plan": "Escolha um plano",
  "p.timeentry.loading": "Lendo os planos…",
  "p.timeentry.f_task": "Tarefa (opcional)",
  "p.timeentry.no_task": "Nenhuma tarefa: o próprio plano",
  "p.timeentry.task_unnamed": "Uma tarefa que não está mais neste plano",
  "p.timeentry.f_date": "Data",
  "p.timeentry.f_duration": "Tempo gasto",
  "p.timeentry.f_hours": "Horas",
  "p.timeentry.f_minutes": "Minutos",
  "p.timeentry.typed": "Isso é {time}.",
  "p.timeentry.f_category": "Categoria",
  "p.timeentry.category_unclassified": "Não classificado",
  "p.timeentry.category_capex": "Gasto de capital (capex)",
  "p.timeentry.category_opex": "Gasto operacional (opex)",
  "p.timeentry.f_billable": "Faturável",
  "p.timeentry.f_note": "Observação (opcional)",
  "p.timeentry.record": "Registrar este tempo",
  "p.timeentry.confirm":
    "Registrar {time} em {day} em {plan}? Um lançamento não pode ser alterado nem removido depois de registrado.",
  "p.timeentry.recorded": "Registrado {time} em {day} em {plan}.",
  "p.timeentry.problem_plan": "Escolha um plano.",
  "p.timeentry.problem_date": "Informe a data em que o tempo foi gasto.",
  "p.timeentry.problem_locked":
    "Essa semana está aprovada e bloqueada, portanto não aceita mais tempo.",
  "p.timeentry.problem_duration":
    "Informe o tempo gasto em horas inteiras (0 a 24) e minutos (0 a 59), de 1 minuto a {max} minutos no total.",
  "p.timeentry.refused_locked":
    "Essa semana está aprovada e bloqueada, portanto não aceita mais tempo. Nada foi registrado.",
  "p.timeentry.refused_invalid":
    "O lançamento não foi aceito: o seu tempo, ou a sua tarefa, não é algo que o plano aceite. Nada foi registrado.",
  "p.timeentry.refused_plan":
    "Esse plano não existe mais, ou você não pode lê-lo.",
  "p.timeentry.refused_forbidden": "Você não pode registrar tempo neste plano.",
  "p.timeentry.could_not_load_plans": "Não foi possível ler os planos.",
  "p.timeentry.could_not_load_plan":
    "Não foi possível ler as tarefas e o tempo desse plano.",
  "p.timeentry.could_not_record": "Não foi possível registrar o tempo.",
  "p.timeentry.yours_heading": "Seu tempo em {plan}",
  "p.timeentry.yours_none": "Você não registrou tempo neste plano.",
  "p.timeentry.yours_table":
    "Seus lançamentos de tempo em {plan}, do mais recente ao mais antigo",
  "p.timeentry.col_date": "Data",
  "p.timeentry.col_time": "Tempo",
  "p.timeentry.col_task": "Tarefa",
  "p.timeentry.col_category": "Categoria",
  "p.timeentry.col_billable": "Faturável",
  "p.timeentry.col_note": "Observação",
  "p.timeentry.billable_yes": "Faturável",
  "p.timeentry.billable_no": "Não faturável",
  "p.timereport.heading": "Relatório de tempo",
  "p.timereport.says":
    "O tempo registrado nos planos que você pode ler, agrupado como você escolher, com o faturável e o não faturável sempre separados. Deixe as datas vazias para os últimos 90 dias até hoje. Uma pessoa é uma referência person: ou worker:; deixe vazio para todos.",
  "p.timereport.form": "Solicitar um relatório de tempo",
  "p.timereport.f_group_by": "Agrupar por",
  "p.timereport.by_plan": "Plano",
  "p.timereport.by_person": "Pessoa",
  "p.timereport.by_category": "Categoria",
  "p.timereport.by_billable": "Faturável ou não",
  "p.timereport.by_month": "Mês",
  "p.timereport.f_from": "De",
  "p.timereport.f_to": "Até",
  "p.timereport.f_plan": "Plano",
  "p.timereport.every_plan": "Todos os planos que você pode ler",
  "p.timereport.loading_plans": "Lendo os planos…",
  "p.timereport.f_person": "Pessoa (uma referência)",
  "p.timereport.show": "Mostrar o relatório de tempo",
  "p.timereport.problem_from": "Informe o primeiro dia como uma data.",
  "p.timereport.problem_to": "Informe o último dia como uma data.",
  "p.timereport.problem_order":
    "O primeiro dia não pode ser posterior ao último.",
  "p.timereport.problem_person":
    "Uma pessoa é person: ou worker: e um identificador, como person:6f1c2a3b-0000-4000-8000-000000000009.",
  "p.timereport.refused":
    "O relatório de tempo não foi aceito para essas escolhas.",
  "p.timereport.window": "Tempo registrado {range}.",
  "p.timereport.no_entries":
    "Nenhum tempo foi registrado nestes planos nessa janela.",
  "p.timereport.table": "Tempo por grupo",
  "p.timereport.col_entries": "Lançamentos",
  "p.timereport.col_billable": "Faturável",
  "p.timereport.col_not_billable": "Não faturável",
  "p.timereport.col_total": "Total geral",
  "p.timereport.total":
    "No total: {billable} faturável, {not_billable} não faturável, {total} juntos.",
  "p.timereport.omitted_one":
    "{n} lançamento em um plano que você não pode ler foi deixado de fora.",
  "p.timereport.omitted_other":
    "{n} lançamentos em planos que você não pode ler foram deixados de fora.",
  "p.timereport.truncated":
    "Somente os primeiros {max} lançamentos foram lidos, portanto o relatório está incompleto.",
  "p.timereport.csv": "Baixar este relatório de tempo como CSV",
  "p.timereport.billable": "Faturável",
  "p.timereport.not_billable": "Não faturável",
  "p.timereport.plan_unnamed": "Um plano sem nome aqui ({ref})",
  "p.timereport.could_not_load": "Não foi possível ler o relatório de tempo.",
  "p.timereport.could_not_load_plans":
    "Não foi possível ler os planos para escolher.",
  "p.hammock.heading": "Hammocks: tarefas-resumo que abrangem outras",
  "p.hammock.says":
    "Um hammock é um resumo cujas datas vêm das tarefas deste plano às quais ele se vincula, do início mais cedo ao término mais tarde. Ele não está na rede de dependências, portanto nunca altera a folga de nenhuma tarefa nem o término. Uma tarefa só se torna um quando não tem duração, trabalho, restrição nem dependência, e o seu início planejado é apagado; torná-la uma tarefa de novo remove todos os seus vínculos. Um hammock se vincula a uma tarefa ou a outro hammock deste plano, não a si mesmo, não duas vezes e não em um ciclo.",
  "p.hammock.none": "Este plano não tem hammocks.",
  "p.hammock.spans": "Abrange {range}.",
  "p.hammock.no_links": "Ainda sem datas: não se vincula a nenhuma tarefa.",
  "p.hammock.members_unscheduled":
    "Ainda sem datas: nenhuma das tarefas às quais se vincula tem datas.",
  "p.hammock.untitled": "Uma tarefa sem título",
  "p.hammock.member_hammock": "{title} (um hammock)",
  "p.hammock.unmake_named": "Tornar {title} uma tarefa de novo",
  "p.hammock.unlink": "Remover o vínculo",
  "p.hammock.unlink_named": "Remover o vínculo de {hammock} com {title}",
  "p.hammock.make_form": "Tornar uma tarefa um hammock",
  "p.hammock.f_task": "Tarefa",
  "p.hammock.choose_task": "Escolha uma tarefa",
  "p.hammock.make": "Torná-la um hammock",
  "p.hammock.link_form": "Vincular um hammock a uma tarefa",
  "p.hammock.f_hammock": "Hammock (resumo)",
  "p.hammock.choose_hammock": "Escolha um hammock",
  "p.hammock.f_member": "Tarefa que ele abrange",
  "p.hammock.choose_member": "Escolha uma tarefa a abranger",
  "p.hammock.link": "Adicionar o vínculo",
  "p.hammock.confirm_make":
    "Tornar {title} um hammock? O seu início planejado é apagado, e as suas datas passam a vir das tarefas às quais ele se vincula.",
  "p.hammock.confirm_unmake_one":
    "Tornar {title} uma tarefa de novo? O seu {n} vínculo é removido.",
  "p.hammock.confirm_unmake_other":
    "Tornar {title} uma tarefa de novo? Os seus {n} vínculos são removidos.",
  "p.hammock.confirm_unlink":
    "Remover o vínculo de {hammock} com {title}? A tarefa em si não é alterada.",
  "p.hammock.made": "{title} agora é um hammock.",
  "p.hammock.unmade": "{title} é uma tarefa de novo.",
  "p.hammock.linked": "{hammock} agora abrange {title}.",
  "p.hammock.unlinked": "{hammock} não abrange mais {title}.",
  "p.hammock.problem_choose_task": "Escolha uma tarefa para tornar um hammock.",
  "p.hammock.problem_carries_work":
    "Esta tarefa tem duração, trabalho ou restrição: apague-os primeiro no cronograma.",
  "p.hammock.problem_has_dependencies":
    "Esta tarefa tem dependências: remova-as primeiro, pois um hammock não aceita nenhuma.",
  "p.hammock.problem_choose_hammock": "Escolha um hammock.",
  "p.hammock.problem_choose_member":
    "Escolha uma tarefa para o hammock abranger.",
  "p.hammock.problem_self": "Um hammock não pode abranger a si mesmo.",
  "p.hammock.problem_linked": "Este hammock já abrange essa tarefa.",
  "p.hammock.problem_loop":
    "Esse hammock já abrange este, portanto o vínculo criaria um ciclo.",
  "p.hammock.refused_make":
    "O serviço não tornou esta tarefa um hammock: ela ainda tem uma duração, trabalho, uma restrição ou uma dependência.",
  "p.hammock.refused_link":
    "O serviço não adicionou o vínculo: a tarefa pode ter saído deste plano, já ser abrangida ou fazer parte de um ciclo.",
  "p.hammock.refused_gone":
    "A tarefa ou o hammock não existe mais, ou você não pode mais lê-lo.",
  "p.hammock.refused_link_gone": "Esse vínculo já não existe.",
  "p.hammock.could_not_load": "Não foi possível ler os hammocks.",
  "p.hammock.could_not_save": "Não foi possível alterar o hammock.",
  "p.crossplan.heading": "Tarefas de planos vinculados",
  "p.crossplan.says":
    "Estas são as tarefas de outros planos que você pode ler, às quais as tarefas deste plano estão vinculadas, agendadas com o calendário do respectivo plano. Uma barra tracejada é uma tarefa fora do caminho crítico; uma barra cheia e um losango marcam uma que está nele.",
  "p.crossplan.network_finish":
    "Com os planos vinculados, a rede termina em {date}.",
  "p.crossplan.truncated":
    "Nem todo plano vinculado é mostrado: a rede atingiu o seu limite de planos ou tarefas.",
  "p.crossplan.links_heading": "Vínculos que cruzam para outro plano",
  "p.crossplan.lag": "defasagem de {days} dias úteis",
  "p.crossplan.timeline": "Tarefas de {plan} em uma linha do tempo",
  "p.crossplan.table": "Datas das tarefas de planos vinculados",
  "p.crossplan.col_task": "Tarefa",
  "p.crossplan.col_plan": "Plano",
  "p.crossplan.col_start": "Início mais cedo",
  "p.crossplan.col_finish": "Término mais cedo",
  "p.crossplan.col_critical": "No caminho crítico",
  "p.crossplan.yes": "Sim",
  "p.crossplan.no": "Não",
  "p.crossplan.untitled": "Uma tarefa sem título",
  "p.crossplan.kind_fs": "término para início",
  "p.crossplan.kind_ss": "início para início",
  "p.crossplan.kind_ff": "término para término",
  "p.crossplan.kind_sf": "início para término",
  "p.crossplan.task_heading": "Dependências, neste plano ou em outro",
  "p.crossplan.task_says":
    "Uma dependência liga esta tarefa a uma de outro plano que você pode ler. Seu tipo é término para início, a menos que seja escolhido outro; sua defasagem é um número inteiro de dias úteis de menos {max} a {max}, e uma defasagem negativa é uma antecipação. Uma dependência não é criada duas vezes do mesmo modo, nem em um ciclo entre planos, nem com um hammock. Uma cuja outra ponta você não pode ler é contada, nunca nomeada, e fica fora do cronograma.",
  "p.crossplan.no_links": "Esta tarefa ainda não tem dependências.",
  "p.crossplan.this_before": "Esta tarefa vem antes de {other}",
  "p.crossplan.this_after": "Esta tarefa vem depois de {other}",
  "p.crossplan.end_own": "{title}, neste plano",
  "p.crossplan.end_external": "{title}, em {plan}",
  "p.crossplan.end_unnamed": "uma tarefa sem nome aqui ({ref})",
  "p.crossplan.remove": "Remover",
  "p.crossplan.remove_named": "Remover a dependência de {other}",
  "p.crossplan.restricted_one":
    "Mais {n} dependência liga uma tarefa que você não pode ler.",
  "p.crossplan.restricted_other":
    "Mais {n} dependências ligam tarefas que você não pode ler.",
  "p.crossplan.see_network": "Ver o cronograma entre planos",
  "p.crossplan.form": "Adicionar uma dependência de uma tarefa de outro plano",
  "p.crossplan.f_order": "Ordem",
  "p.crossplan.order_before": "Esta tarefa vem primeiro",
  "p.crossplan.order_after": "A outra tarefa vem primeiro",
  "p.crossplan.f_plan": "Outro plano",
  "p.crossplan.choose_plan": "Escolha um plano",
  "p.crossplan.f_task": "Tarefa nesse plano",
  "p.crossplan.choose_task": "Escolha uma tarefa",
  "p.crossplan.f_kind": "Como as duas se ligam",
  "p.crossplan.f_lag": "Defasagem em dias úteis (0 se vazio)",
  "p.crossplan.add": "Adicionar a dependência",
  "p.crossplan.added": "Dependência de {title} adicionada.",
  "p.crossplan.removed": "Dependência removida.",
  "p.crossplan.confirm_remove_before":
    "Remover a dependência: {task} vem antes de {other}? Nenhuma das tarefas é alterada.",
  "p.crossplan.confirm_remove_after":
    "Remover a dependência: {task} vem depois de {other}? Nenhuma das tarefas é alterada.",
  "p.crossplan.problem_plan": "Escolha o outro plano.",
  "p.crossplan.problem_task": "Escolha uma tarefa nesse plano.",
  "p.crossplan.problem_lag":
    "Uma defasagem é um número inteiro de dias úteis de menos {max} a {max}.",
  "p.crossplan.problem_self": "Uma tarefa não pode depender de si mesma.",
  "p.crossplan.problem_exists": "Esta dependência já existe.",
  "p.crossplan.refused_unreadable":
    "Uma das duas tarefas não existe mais, ou você não pode lê-la.",
  "p.crossplan.refused_exists": "Esta dependência já existe.",
  "p.crossplan.refused_add":
    "O serviço não adicionou a dependência: ela pode já existir, criar um ciclo ou ligar um hammock.",
  "p.crossplan.refused_gone":
    "Essa dependência já não existe, ou você não pode mais ler as duas tarefas.",
  "p.crossplan.could_not_load":
    "Não foi possível ler as dependências desta tarefa.",
  "p.crossplan.could_not_load_plans":
    "Não foi possível ler os planos para escolher.",
  "p.crossplan.could_not_load_tasks":
    "Não foi possível ler as tarefas desse plano.",
  "p.crossplan.could_not_save": "Não foi possível alterar a dependência.",
  "p.taskrollup.heading": "Subtarefas consolidadas",
  "p.taskrollup.says":
    "Calculado a cada vez a partir das subtarefas desta tarefa, nunca armazenado: a data de vencimento mais tardia entre elas, os seus pontos somados, o progresso ponderado apenas pelas estimadas e o tempo registrado em todas.",
  "p.taskrollup.subtasks": "Subtarefas",
  "p.taskrollup.done_of_one": "{done} de {n} concluída",
  "p.taskrollup.done_of_other": "{done} de {n} concluídas",
  "p.taskrollup.due": "Data de vencimento mais tardia",
  "p.taskrollup.points": "Pontos somados",
  "p.taskrollup.progress": "Progresso",
  "p.taskrollup.spent": "Tempo registrado",
  "p.taskrollup.no_children": "Esta tarefa não tem subtarefas.",
  "p.taskrollup.no_dated_children":
    "Nenhuma de suas subtarefas tem data de vencimento.",
  "p.taskrollup.no_estimates": "Nenhuma de suas subtarefas está estimada.",
  "p.taskrollup.absent": "Desconhecido.",
  "p.taskrollup.unestimated_one":
    "{n} subtarefa não tem estimativa, portanto fica de fora do progresso (não é contada como zero):",
  "p.taskrollup.unestimated_other":
    "{n} subtarefas não têm estimativa, portanto ficam de fora do progresso (não são contadas como zero):",
  "p.taskrollup.unnamed": "Uma subtarefa sem nome aqui ({ref})",
  "p.taskrollup.refused_gone":
    "Esta tarefa não existe mais, ou você não pode mais lê-la.",
  "p.taskrollup.could_not_load":
    "Não foi possível ler as subtarefas consolidadas.",
  "p.readings.heading": "Mais sobre este plano",
  "p.readings.says": "Cada um destes é lido somente quando você solicita.",
  "p.readings.read": "Ler",
  "p.readings.could_not_read": "Não foi possível ler isto.",
  "p.readings.not_measured": "não medido",
  "p.readings.days": "{n} dias",
  "p.readings.rollup_heading": "Fluxo nos planos que ele contém",
  "p.readings.rollup_says":
    "As tarefas deste plano e de todos os planos abaixo dele, contadas em conjunto, não em média; até {max} níveis abaixo quando deixado vazio.",
  "p.readings.f_depth": "Níveis abaixo",
  "p.readings.rollup_plans_one": "{n} plano.",
  "p.readings.rollup_plans_other": "{n} planos.",
  "p.readings.rollup_tasks_one":
    "{n} tarefa: {finished} concluídas, {wip} em andamento, {not_started} não iniciadas.",
  "p.readings.rollup_tasks_other":
    "{n} tarefas: {finished} concluídas, {wip} em andamento, {not_started} não iniciadas.",
  "p.readings.rollup_times":
    "A maior parte do trabalho (85 em 100) terminou em até {cycle} após o início e {lead} após ser solicitada; tempo gasto trabalhando em vez de esperando: {efficiency}.",
  "p.readings.rollup_truncated":
    "Nem todos os planos abaixo foram contados: a varredura atingiu o seu limite.",
  "p.readings.rollup_revisits_one":
    "{n} plano é alcançado duas vezes e contado uma só.",
  "p.readings.rollup_revisits_other":
    "{n} planos são alcançados duas vezes e contados uma só.",
  "p.readings.rollup_table": "Fluxo por plano",
  "p.readings.col_plan": "Plano",
  "p.readings.col_depth": "Nível",
  "p.readings.col_tasks": "Tarefas",
  "p.readings.col_finished": "Concluídas",
  "p.readings.col_wip": "Em andamento",
  "p.readings.col_efficiency": "Tempo trabalhando em vez de esperando",
  "p.readings.effort_heading": "Tempo registrado",
  "p.readings.effort_says":
    "O tempo que as pessoas registraram neste plano, como o informaram; nada o verifica.",
  "p.readings.read_effort": "Ler o tempo registrado",
  "p.readings.effort_none": "Nenhum tempo está registrado neste plano.",
  "p.readings.effort_total":
    "{total} registrado, dos quais {billable} faturável.",
  "p.readings.effort_split":
    "Gasto de capital {capex}, gasto operacional {opex}, não classificado {unclassified}.",
  "p.readings.effort_by_person": "Tempo por pessoa",
  "p.readings.effort_by_task": "Tempo por tarefa",
  "p.readings.col_person": "Pessoa",
  "p.readings.col_time": "Tempo",
  "p.readings.col_billable": "Faturável",
  "p.readings.col_task": "Tarefa",
  "p.readings.task_unnamed": "Uma tarefa sem nome aqui ({ref})",
  "p.readings.forecast_heading": "Previsão de entrega",
  "p.readings.forecast_says":
    "Dez mil execuções obtidas a partir de quanto este plano concluiu em cada período passado: quanto tempo o trabalho aberto pode levar e quanto pode ser feito em alguns períodos. Um campo vazio usa o padrão; são necessários pelo menos seis períodos de histórico.",
  "p.readings.f_items": "Itens a concluir (os abertos, se vazio)",
  "p.readings.f_periods": "Períodos à frente ({n} se vazio)",
  "p.readings.f_period_days": "Dias em um período ({n} se vazio)",
  "p.readings.f_history": "Períodos de histórico ({n} se vazio)",
  "p.readings.read_forecast": "Ler a previsão",
  "p.readings.forecast_history_one": "Obtida de {n} período de {days} dias.",
  "p.readings.forecast_history_other": "Obtida de {n} períodos de {days} dias.",
  "p.readings.forecast_batch_one":
    "{n} item: metade das execuções termina em até {p50} dias, 85 em 100 em até {p85} dias e 95 em 100 em até {p95} dias.",
  "p.readings.forecast_batch_other":
    "{n} itens: metade das execuções termina em até {p50} dias, 85 em 100 em até {p85} dias e 95 em 100 em até {p95} dias.",
  "p.readings.forecast_no_batch":
    "Não é possível prever quanto tempo o trabalho pode levar: há histórico insuficiente, ou nada foi concluído nele.",
  "p.readings.forecast_horizon":
    "Nos próximos {days} dias, cerca de {median} itens podem ser concluídos: pelo menos {least} em 85 execuções de 100, e no máximo {most}.",
  "p.readings.forecast_no_horizon":
    "Não é possível prever quanto pode ser feito: há histórico insuficiente, ou nada foi concluído nele.",
  "p.readings.performance_heading": "Cronograma, custo e partes interessadas",
  "p.readings.performance_says":
    "O desempenho do cronograma é o trabalho feito em relação ao trabalho planejado até agora, e o desempenho de custo é o trabalho feito em relação ao que custou; acima de 100% é melhor que o plano. A pontuação das partes interessadas vai de menos 100 a 100.",
  "p.readings.read_performance": "Ler o desempenho",
  "p.readings.schedule": "Desempenho do cronograma",
  "p.readings.cost": "Desempenho de custo",
  "p.readings.stakeholders": "Pontuação das partes interessadas",
  "p.readings.nps_one":
    "{score}, de {n} resposta ({promoters} promotores, {detractors} detratores)",
  "p.readings.nps_other":
    "{score}, de {n} respostas ({promoters} promotores, {detractors} detratores)",
  "p.readings.nps_none": "Nenhuma resposta ainda.",
  "p.readings.earned_value_link": "Ver o valor agregado completo",
  "p.readings.absent_no_baseline":
    "Não medido: o plano não tem linha de base de orçamento.",
  "p.readings.absent_no_weighted_tasks": "Não medido: nenhuma tarefa tem peso.",
  "p.readings.absent_not_yet_defined":
    "Ainda não medido: nada foi planejado nem gasto até agora.",
  "p.readings.absent_several_currencies":
    "Não é um único número: o plano tem orçamentos em várias moedas; veja o valor agregado.",
  "p.readings.absent_other": "Não medido.",
  "p.readings.workflow_heading": "Fluxo de trabalho das tarefas",
  "p.readings.read_workflow": "Ler o fluxo de trabalho das tarefas",
  "p.readings.workflow_plan":
    "Este plano tem o seu próprio fluxo de trabalho de tarefas.",
  "p.readings.workflow_built_in":
    "Este plano usa o fluxo de trabalho de tarefas integrado.",
  "p.readings.workflow_constrained":
    "Somente as mudanças listadas são permitidas.",
  "p.readings.workflow_free":
    "Uma tarefa pode passar de qualquer estado para qualquer outro.",
  "p.readings.cat_todo": "não iniciada",
  "p.readings.cat_active": "em andamento",
  "p.readings.cat_waiting": "em espera",
  "p.readings.cat_done": "concluída",
  "p.readings.cat_other": "outro tipo",
  "p.readings.initial": "onde uma tarefa começa",
  "p.readings.terminal": "onde uma tarefa termina",
  "p.readings.wip_limit": "no máximo {n} ao mesmo tempo",
  "p.readings.anyone": "qualquer pessoa",
  "p.readings.roles": "funções: {roles}",
  "p.readings.problem_depth": "Níveis abaixo é um número inteiro de 1 a {max}.",
  "p.readings.problem_depth_zero":
    "Níveis abaixo é um número inteiro de 0 a {max}.",
  "p.readings.problem_items":
    "Itens a concluir é um número inteiro de pelo menos 1.",
  "p.readings.problem_periods":
    "Períodos à frente é um número inteiro de 1 a 520.",
  "p.readings.problem_period_days":
    "Dias em um período é um número inteiro de 1 a 90.",
  "p.readings.problem_history":
    "Períodos de histórico é um número inteiro de 6 a 260.",
  "p.readings.problem_window":
    "A janela é um número inteiro de dias, de 1 a 3650.",
  "p.readings.refused_query": "O serviço não aceitou essa pergunta.",
  "p.readings.refused_module": "Esta parte do plano está desativada.",
  "p.readings.refused_gone":
    "Este plano não existe mais, ou você não pode mais lê-lo.",
  "p.readings.flow_feature": "Novas funcionalidades",
  "p.readings.flow_defect": "Defeitos",
  "p.readings.flow_risk": "Trabalho de risco",
  "p.readings.flow_debt": "Dívida técnica",
  "p.readings.flow_unclassified": "Não classificado",
  "p.readings.flow_other": "Outro tipo",
  "p.flowmix.heading": "A composição do trabalho concluído",
  "p.flowmix.says":
    "Como o trabalho concluído em uma janela se divide entre novas funcionalidades, defeitos, trabalho de risco e dívida técnica, em comparação com a composição que o plano pretende, quando declarou uma. A janela é de {n} dias se deixada vazia.",
  "p.flowmix.f_window": "Dias para trás ({n} se vazio)",
  "p.flowmix.f_rollup": "Incluir os planos que ele contém",
  "p.flowmix.f_depth": "Níveis abaixo ({max} se vazio)",
  "p.flowmix.read": "Ler a composição",
  "p.flowmix.window_one": "Trabalho concluído no último {n} dia.",
  "p.flowmix.window_other": "Trabalho concluído nos últimos {n} dias.",
  "p.flowmix.plans_one": "{n} plano contado.",
  "p.flowmix.plans_other": "{n} planos contados.",
  "p.flowmix.truncated":
    "Nem todos os planos abaixo foram contados: a varredura atingiu o seu limite.",
  "p.flowmix.none": "Nenhum trabalho foi concluído nessa janela.",
  "p.flowmix.no_intent":
    "O plano não declara nenhuma composição pretendida, portanto não há com o que comparar.",
  "p.flowmix.table": "Trabalho concluído por tipo",
  "p.flowmix.col_kind": "Tipo de trabalho",
  "p.flowmix.col_count": "Concluídos",
  "p.flowmix.col_share": "Participação",
  "p.flowmix.col_intended": "Participação pretendida",
  "p.flowmix.col_gap": "Diferença",
  "p.ccpm.heading": "Estimativas agressivas e seguras",
  "p.ccpm.says":
    "Para a corrente crítica, cada tarefa precisa de uma estimativa agressiva (cerca de 50% de chance) e de uma segura, em dias úteis inteiros de 1 a {max}, a agressiva não superior à segura. O número mostrado é o registrado; na falta dele, o da faixa de risco; na falta desta, um que a regra da implantação deriva, e cada um diz qual é. Um marco e uma tarefa sem duração não são estimados. Apagar as duas remove o que está registrado.",
  "p.ccpm.rule_explicit_only":
    "Regra: somente as estimativas informadas são usadas; nada é derivado.",
  "p.ccpm.rule_half_of_safe":
    "Regra: uma estimativa segura ausente é a duração da tarefa, e uma agressiva ausente é metade da segura.",
  "p.ccpm.rule_other": "Regra: uma que esta página desconhece.",
  "p.ccpm.rule_unusable":
    "A configuração da regra da implantação não pôde ser lida, portanto somente as estimativas informadas são usadas.",
  "p.ccpm.attention_one": "{n} tarefa precisa das suas estimativas.",
  "p.ccpm.attention_other": "{n} tarefas precisam das suas estimativas.",
  "p.ccpm.more": "As primeiras {shown} de {total} tarefas são mostradas.",
  "p.ccpm.no_tasks": "Este plano não tem tarefas a estimar.",
  "p.ccpm.table": "Estimativas por tarefa",
  "p.ccpm.col_task": "Tarefa",
  "p.ccpm.col_likely": "Duração em dias úteis",
  "p.ccpm.col_aggressive": "Agressiva",
  "p.ccpm.col_safe": "Segura",
  "p.ccpm.col_state": "Situação",
  "p.ccpm.figure": "{days} dias, {source}",
  "p.ccpm.none": "nenhuma",
  "p.ccpm.source_recorded": "registrada",
  "p.ccpm.source_risk_range": "da faixa de risco",
  "p.ccpm.source_rule": "pela regra",
  "p.ccpm.state_complete": "Ambas informadas",
  "p.ccpm.state_derived": "Parcialmente derivada",
  "p.ccpm.state_missing": "Falta uma estimativa",
  "p.ccpm.state_inconsistent": "Agressiva acima da segura",
  "p.ccpm.state_milestone": "Um marco, não estimado",
  "p.ccpm.state_no_duration": "Sem duração, não estimada",
  "p.ccpm.state_other": "Outra situação",
  "p.ccpm.form": "Registrar as estimativas de uma tarefa",
  "p.ccpm.f_task": "Tarefa",
  "p.ccpm.choose_task": "Escolha uma tarefa",
  "p.ccpm.f_aggressive": "Agressiva, em dias úteis",
  "p.ccpm.f_safe": "Segura, em dias úteis",
  "p.ccpm.save": "Salvar as estimativas",
  "p.ccpm.unchanged": "Nada mudou, portanto nada foi enviado.",
  "p.ccpm.saved": "Estimativas de {title} salvas.",
  "p.ccpm.cleared": "Estimativas registradas de {title} apagadas.",
  "p.ccpm.confirm_clear":
    "Apagar as estimativas registradas de {title}? Um número da faixa de risco ou da regra ainda pode ser mostrado.",
  "p.ccpm.problem_task": "Escolha uma tarefa.",
  "p.ccpm.problem_aggressive":
    "A estimativa agressiva é um número inteiro de dias úteis de 1 a {max}.",
  "p.ccpm.problem_safe":
    "A estimativa segura é um número inteiro de dias úteis de 1 a {max}.",
  "p.ccpm.problem_order":
    "A estimativa agressiva não pode ser superior à segura.",
  "p.ccpm.refused_invalid":
    "O serviço não aceitou estas estimativas: a tarefa pode ter perdido a duração ou se tornado um marco.",
  "p.ccpm.refused_gone":
    "Esta tarefa não existe mais, ou você não pode mais lê-la.",
  "p.ccpm.could_not_load": "Não foi possível ler as estimativas.",
  "p.ccpm.could_not_save": "Não foi possível salvar as estimativas.",
  "p.frozen.heading": "O plano de pulmões congelado",
  "p.frozen.none": "Nenhum plano de pulmões foi congelado ainda.",
  "p.frozen.freeze_link": "Congele um na gestão de pulmões",
  "p.frozen.taken": "Congelado em {date}; {rule}",
  "p.frozen.table": "Pulmões como congelados",
  "p.frozen.col_buffer": "Pulmão",
  "p.frozen.col_days": "Dias úteis",
  "p.frozen.col_chain": "Corrente",
  "p.frozen.col_finish": "Término planejado",
  "p.frozen.project": "Pulmão do projeto",
  "p.frozen.feeding": "Pulmão de alimentação em {joins}",
  "p.frozen.unsized": "não dimensionado",
  "p.frozen.untitled": "Uma tarefa sem título",
  "p.frozen.history_one": "{n} plano de pulmões congelado: {dates}.",
  "p.frozen.history_other":
    "{n} planos de pulmões congelados, do mais recente ao mais antigo: {dates}.",
  "p.frozen.could_not_load":
    "Não foi possível ler o plano de pulmões congelado.",
  "p.workingtime.heading": "O dia e a semana de trabalho",
  "p.workingtime.says":
    "Quanto dura um dia de trabalho (de 1 minuto a 24 horas) e quantos dias por semana se trabalha (de 1 a 7), para este plano ou como padrão da organização. Cada um é um novo registro que não pode ser alterado nem removido aqui, por isso você é consultado antes; um segundo para este plano é recusado.",
  "p.workingtime.form": "Definir o dia e a semana de trabalho",
  "p.workingtime.f_scope": "Para",
  "p.workingtime.scope_plan": "Este plano",
  "p.workingtime.scope_default": "O padrão da organização",
  "p.workingtime.f_hours": "Horas em um dia",
  "p.workingtime.f_minutes": "E minutos",
  "p.workingtime.f_days": "Dias em uma semana",
  "p.workingtime.save": "Definir o dia de trabalho",
  "p.workingtime.confirm_plan_one":
    "Definir o dia de trabalho deste plano como {day}, {days} dia por semana? Ele não poderá ser alterado aqui depois.",
  "p.workingtime.confirm_plan_other":
    "Definir o dia de trabalho deste plano como {day}, {days} dias por semana? Ele não poderá ser alterado aqui depois.",
  "p.workingtime.confirm_default_one":
    "Definir o dia de trabalho padrão da organização como {day}, {days} dia por semana? Ele não poderá ser alterado aqui depois.",
  "p.workingtime.confirm_default_other":
    "Definir o dia de trabalho padrão da organização como {day}, {days} dias por semana? Ele não poderá ser alterado aqui depois.",
  "p.workingtime.saved_one":
    "Dia de trabalho definido como {day}, {days} dia por semana.",
  "p.workingtime.saved_other":
    "Dia de trabalho definido como {day}, {days} dias por semana.",
  "p.workingtime.away_heading": "Tempo fora do trabalho do projeto",
  "p.workingtime.away_says":
    "Licença, férias, licença para estudos ou uma atribuição fora do projeto, para uma pessoa de um primeiro a um último dia, de modo que esse tempo não seja lido como ocioso. A pessoa é uma referência como person: e um id. Não pode ser alterado nem removido depois de registrado, por isso você é consultado antes; uma observação em branco não é enviada.",
  "p.workingtime.f_person": "Referência da pessoa",
  "p.workingtime.f_starts": "Primeiro dia",
  "p.workingtime.f_ends": "Último dia",
  "p.workingtime.f_kind": "Tipo",
  "p.workingtime.f_note": "Observação (opcional)",
  "p.workingtime.record": "Registrar o tempo fora",
  "p.workingtime.confirm_away":
    "Registrar {kind} para {person}, {range}? Não poderá ser alterado nem removido depois.",
  "p.workingtime.recorded": "Registrado {kind} para {person}, {range}.",
  "p.workingtime.kind_leave": "licença",
  "p.workingtime.kind_holiday": "férias",
  "p.workingtime.kind_study_leave": "licença para estudos",
  "p.workingtime.kind_non_project_duty": "uma atribuição fora do projeto",
  "p.workingtime.problem_hours": "As horas são um número inteiro de 0 a 24.",
  "p.workingtime.problem_minutes":
    "Os minutos são um número inteiro de 0 a 59.",
  "p.workingtime.problem_day": "Um dia de trabalho vai de 1 minuto a 24 horas.",
  "p.workingtime.problem_week":
    "Os dias em uma semana são um número inteiro de 1 a 7.",
  "p.workingtime.problem_person":
    "Informe a pessoa como person: ou worker: e um id.",
  "p.workingtime.problem_starts": "Informe o primeiro dia.",
  "p.workingtime.problem_ends": "Informe o último dia.",
  "p.workingtime.problem_order": "O último dia não pode vir antes do primeiro.",
  "p.workingtime.refused_exists":
    "Este plano já tem um dia de trabalho definido, e ele não pode ser substituído aqui.",
  "p.workingtime.refused_working":
    "O serviço não aceitou este dia de trabalho.",
  "p.workingtime.refused_non_working": "O serviço não aceitou este tempo fora.",
  "p.workingtime.refused_forbidden": "Você não pode registrar isto.",
  "p.workingtime.could_not_save": "Não foi possível registrar isto.",
  "p.task.is_hammock":
    "Esta tarefa é um hammock: suas datas vêm das tarefas que ela abrange.",
  "p.task.hammock_link": "Ver a sua abrangência no caminho crítico",
  "p.adoption.could_not": "Não foi possível registrar a adoção.",
  "p.adoption.f_active": "Usuários ativos",
  "p.adoption.f_definition": "O que contou como usuário ativo",
  "p.adoption.f_target": "Usuários a que se destina",
  "p.adoption.f_window": "Em quantos dias",
  "p.adoption.form": "Registrar a adoção",
  "p.adoption.heading": "Quantas pessoas o usam",
  "p.adoption.problem_active":
    "Informe os usuários ativos como um número inteiro, 0 ou mais.",
  "p.adoption.problem_definition":
    "Diga o que contou como usuário ativo, para que as leituras posteriores sejam comparáveis.",
  "p.adoption.problem_target":
    "Informe os usuários a que se destina como um número inteiro maior que 0.",
  "p.adoption.problem_window":
    "Informe a janela como um número inteiro de dias, 1 ou mais.",
  "p.adoption.record": "Registrar a adoção",
  "p.adoption.refused": "O serviço não aceitou estes números.",
  "p.adoption.refused_forbidden":
    "Você não pode registrar a adoção deste plano.",
  "p.adoption.refused_gone": "Este plano não existe mais.",
  "p.adoption.saved": "Adoção registrada.",
  "p.adoption.says":
    "Usuários ativos em relação aos usuários a que se destina, ao longo de uma janela de dias. O que contou como ativo é guardado junto ao número, para que uma leitura posterior possa ser comparada com ele.",
  "p.bulk.apply": "Definir nas tarefas escolhidas",
  "p.bulk.choose_all": "Escolher todas as tarefas",
  "p.bulk.choose_none": "Não escolher nenhuma",
  "p.bulk.choose_plan": "Escolha um plano",
  "p.bulk.chosen_one": "{n} tarefa escolhida",
  "p.bulk.chosen_other": "{n} tarefas escolhidas",
  "p.bulk.confirm_delete_one":
    "Excluir {n} tarefa? Ela sai do quadro e de todas as visões do plano.",
  "p.bulk.confirm_delete_other":
    "Excluir {n} tarefas? Elas saem do quadro e de todas as visões do plano.",
  "p.bulk.confirm_move_one":
    "Mover {n} tarefa para {plan}? Sua sprint, seu componente e sua tarefa-mãe são apagados, e um status que esse plano não tem passa a ser o primeiro dele.",
  "p.bulk.confirm_move_other":
    "Mover {n} tarefas para {plan}? Suas sprints, seus componentes e suas tarefas-mãe são apagados, e um status que esse plano não tem passa a ser o primeiro dele.",
  "p.bulk.copy": "Copiar para o plano",
  "p.bulk.could_not": "Não foi possível alterar as tarefas.",
  "p.bulk.could_not_load_plans": "Não foi possível ler os planos.",
  "p.bulk.delete": "Excluir as tarefas escolhidas",
  "p.bulk.delete_failed": "Não excluídas: {titles}.",
  "p.bulk.deleted_gone_one": "{n} tarefa já não existia.",
  "p.bulk.deleted_gone_other": "{n} tarefas já não existiam.",
  "p.bulk.deleted_one": "{n} tarefa excluída.",
  "p.bulk.deleted_other": "{n} tarefas excluídas.",
  "p.bulk.done_one": "Concluído para {done} de {requested} tarefa.",
  "p.bulk.done_other": "Concluído para {done} de {requested} tarefas.",
  "p.bulk.edit_form": "Definir campos nas tarefas escolhidas",
  "p.bulk.f_assignee": "Responsável",
  "p.bulk.f_due": "Data de vencimento",
  "p.bulk.f_plan": "Plano",
  "p.bulk.f_start": "Data de início",
  "p.bulk.f_value": "Novo valor de {field}",
  "p.bulk.heading": "Alterar várias tarefas de uma vez",
  "p.bulk.keep_assignee": "Manter o responsável ao copiar",
  "p.bulk.mapped_one":
    "{n} tarefa tinha um status que esse plano não tem, portanto ela começa no primeiro estado desse plano.",
  "p.bulk.mapped_other":
    "{n} tarefas tinham um status que esse plano não tem, portanto elas começam no primeiro estado desse plano.",
  "p.bulk.mode_clear": "Apagar",
  "p.bulk.mode_keep": "Deixar como está",
  "p.bulk.mode_set": "Definir como",
  "p.bulk.move": "Mover para o plano",
  "p.bulk.no_tasks": "Este plano não tem tarefas.",
  "p.bulk.problem_assignee":
    "Informe o responsável como person: ou worker: e um id.",
  "p.bulk.problem_due": "Informe a data de vencimento como uma data real.",
  "p.bulk.problem_no_field": "Escolha pelo menos um campo a definir ou apagar.",
  "p.bulk.problem_none": "Escolha pelo menos uma tarefa.",
  "p.bulk.problem_plan": "Escolha o plano para o qual copiar ou mover.",
  "p.bulk.problem_same_plan":
    "As tarefas já estão neste plano: escolha outro para o qual movê-las.",
  "p.bulk.problem_start":
    "Informe a data de início como uma data real de {min} a {latest}.",
  "p.bulk.problem_too_many": "Escolha no máximo {max} tarefas por vez.",
  "p.bulk.refused_edit": "O serviço não aceitou estes valores.",
  "p.bulk.refused_forbidden": "Você não pode alterar estas tarefas.",
  "p.bulk.refused_plan_gone":
    "Esse plano não existe mais ou não é um que você possa ver.",
  "p.bulk.refused_titles":
    "Recusado, por um motivo que o serviço informa somente em inglês: {titles}.",
  "p.bulk.refused_transfer":
    "Esse plano não é um que você possa ver, ou o serviço não aceitou a lista.",
  "p.bulk.says":
    "Escolha tarefas e então defina o mesmo responsável, a mesma data de vencimento ou a mesma data de início em todas elas, copie-as para um plano ou mova-as para lá, ou exclua-as. No máximo {max} por vez; a data de início vai de {min} a {latest}. O status é alterado no quadro, um cartão por vez. Uma cópia começa no primeiro estado desse plano e mantém o responsável somente se solicitado; uma mudança apaga a sprint, o componente e a tarefa-mãe, e uma tarefa com subtarefas não é movida.",
  "p.bulk.this_plan": "{name} (este plano)",
  "p.bulk.transfer_form": "Copiar ou mover as tarefas escolhidas",
  "p.bulk.unchanged":
    "As tarefas escolhidas já têm estes valores, portanto nada foi enviado.",
  "p.ceremony.choose_kind": "Escolha uma cerimônia",
  "p.ceremony.could_not_load": "Não foi possível ler as cerimônias da sprint.",
  "p.ceremony.could_not_save": "Não foi possível registrar a cerimônia.",
  "p.ceremony.counts": "Cerimônias realizadas",
  "p.ceremony.f_facilitator":
    "Facilitador (person: ou worker: e um id, opcional)",
  "p.ceremony.f_kind": "Cerimônia",
  "p.ceremony.f_summary": "O que foi decidido (opcional)",
  "p.ceremony.facilitated_by": "facilitada por {ref}",
  "p.ceremony.form": "Registrar uma cerimônia",
  "p.ceremony.heading": "Cerimônias de {name}",
  "p.ceremony.held_count_one": "{kind}: realizada {n} vez",
  "p.ceremony.held_count_other": "{kind}: realizada {n} vezes",
  "p.ceremony.held_saved": "Registrada: {kind}.",
  "p.ceremony.kind_daily": "Reunião diária",
  "p.ceremony.kind_other": "Outra cerimônia",
  "p.ceremony.kind_planning": "Planejamento",
  "p.ceremony.kind_retrospective": "Retrospectiva",
  "p.ceremony.kind_review": "Revisão",
  "p.ceremony.none_held": "{kind}: não realizada",
  "p.ceremony.problem_facilitator":
    "Informe o facilitador como person: ou worker: e um id, ou deixe em branco.",
  "p.ceremony.problem_kind": "Escolha a cerimônia.",
  "p.ceremony.problem_once":
    "Esta sprint já realizou o seu(sua) {kind}: uma segunda vez é um novo planejamento, que é uma nova sprint.",
  "p.ceremony.record": "Registrar a cerimônia",
  "p.ceremony.refused_ceremony":
    "O serviço a recusou: esta sprint pode já ter realizado essa cerimônia.",
  "p.ceremony.refused_forbidden": "Você não pode registrar isto para a sprint.",
  "p.ceremony.refused_gone": "Esta sprint não existe mais.",
  "p.ceremony.says":
    "Uma sprint tem um planejamento e uma revisão, e qualquer número de reuniões diárias e retrospectivas. Todos os tipos são contados, inclusive os não realizados.",
  "p.checkins.heading": "Acompanhamentos de {title}",
  "p.checkins.none": "Nenhum acompanhamento ainda.",
  "p.checkins.row": "{date}: valor {value}",
  "p.commitment.added_one": "{n} tarefa adicionada desde então: {titles}.",
  "p.commitment.added_other": "{n} tarefas adicionadas desde então: {titles}.",
  "p.commitment.commit": "Confirmar as tarefas da sprint",
  "p.commitment.confirm_one":
    "Confirmar agora a {n} tarefa de {name}? Isso é feito uma única vez e não pode ser alterado; tarefas adicionadas ou removidas depois aparecem como uma mudança.",
  "p.commitment.confirm_other":
    "Confirmar agora as {n} tarefas de {name}? Isso é feito uma única vez e não pode ser alterado; tarefas adicionadas ou removidas depois aparecem como uma mudança.",
  "p.commitment.could_not": "Não foi possível confirmar a sprint.",
  "p.commitment.heading": "O que a sprint assumiu como compromisso",
  "p.commitment.not_yet_one":
    "Ainda não confirmada; {n} tarefa está na sprint agora.",
  "p.commitment.not_yet_other":
    "Ainda não confirmada; {n} tarefas estão na sprint agora.",
  "p.commitment.problem_done":
    "Esta sprint já está confirmada: um compromisso nunca é reescrito.",
  "p.commitment.problem_empty": "Esta sprint não tem tarefas a confirmar.",
  "p.commitment.problem_unread": "O compromisso da sprint ainda não foi lido.",
  "p.commitment.refused_commit":
    "O serviço a recusou: esta sprint já está confirmada.",
  "p.commitment.removed_one": "{n} tarefa removida desde então: {titles}.",
  "p.commitment.removed_other": "{n} tarefas removidas desde então: {titles}.",
  "p.commitment.saved_one": "{n} tarefa confirmada.",
  "p.commitment.saved_other": "{n} tarefas confirmadas.",
  "p.commitment.says":
    "As tarefas da sprint como estão quando confirmadas, registradas uma única vez, de modo que uma tarefa adicionada ou removida depois seja lida como uma mudança, e não como se sempre tivesse feito parte do plano.",
  "p.commitment.state": "{committed} confirmadas; {current} na sprint agora.",
  "p.commitment.unchanged": "Nada adicionado nem removido desde então.",
  "p.hours.col_actual": "Registradas",
  "p.hours.col_placeholder": "Das quais não atribuídas",
  "p.hours.col_plan": "Plano",
  "p.hours.col_planned": "Planejadas",
  "p.hours.col_variance": "Registradas menos planejadas",
  "p.hours.could_not": "Não foi possível ler as horas.",
  "p.hours.f_from": "De",
  "p.hours.f_to": "Até",
  "p.hours.form": "Horas planejadas e registradas",
  "p.hours.fte_says":
    "Os equivalentes de tempo integral são os percentuais-dia divididos por 100 vezes os dias do período; uma lacuna é oferta menos demanda (abaixo de zero é déficit). As reservas propostas são uma série à parte.",
  "p.hours.h": "{h} h",
  "p.hours.heading": "Horas planejadas e registradas",
  "p.hours.masked": "As pessoas não são mostradas: você não pode gerenciá-las.",
  "p.hours.none":
    "Nenhum plano teve horas planejadas ou registradas nesta janela.",
  "p.hours.nothing_recorded": "nada registrado",
  "p.hours.problem_from":
    "Informe o primeiro dia como uma data real, ou deixe em branco.",
  "p.hours.problem_order": "O último dia não pode ser anterior ao primeiro.",
  "p.hours.problem_span": "A janela pode ter no máximo {max} dias.",
  "p.hours.problem_to":
    "Informe o último dia como uma data real, ou deixe em branco.",
  "p.hours.read": "Ler as horas",
  "p.hours.refused": "O serviço não aceitou esta janela.",
  "p.hours.says":
    "Planejado é a parcela de cada dia útil de uma reserva nomeada e confirmada; registrado é o tempo que as pessoas lançaram, aprovado ou não. Deixe os dias em branco para as quatro semanas até hoje; no máximo {max} dias.",
  "p.hours.table": "Horas por plano",
  "p.hours.totals": "Planejadas {planned}; registradas {actual}.",
  "p.hours.truncated":
    "Somente os primeiros lançamentos foram lidos; os totais podem estar baixos.",
  "p.hours.unplanned": "{plan}: {hours} registradas",
  "p.hours.unplanned_heading":
    "Tempo registrado por pessoas sem reserva no plano",
  "p.hours.unplanned_people_one": "({n} pessoa)",
  "p.hours.unplanned_people_other": "({n} pessoas)",
  "p.hours.window": "De {from} a {to}.",
  "p.kpireadings.col_standing": "Situação",
  "p.kpireadings.counted": "contada",
  "p.kpireadings.replaced":
    "substituída por uma leitura posterior no mesmo dia",
  "p.kpireadings.says":
    "Todas as leituras, das mais recentes às mais antigas. Uma leitura posterior no mesmo dia substitui uma anterior, que é mantida.",
  "p.live.burndown_says":
    "Tarefas restantes a cada dia, a partir das conclusões registradas; nenhuma linha ideal é traçada e nada é preenchido.",
  "p.live.event_comment_added": "um comentário adicionado",
  "p.live.event_other": "uma mudança",
  "p.live.event_raci_updated": "mudou quem faz o quê",
  "p.live.event_resync": "o quadro foi lido novamente por inteiro",
  "p.live.event_task_created": "uma tarefa adicionada",
  "p.live.event_task_moved": "uma tarefa movida",
  "p.live.event_task_updated": "uma tarefa alterada",
  "p.live.follow": "Acompanhar as mudanças à medida que acontecem",
  "p.live.heading": "Mudanças à medida que acontecem",
  "p.live.last_change": "Última: {change} às {time}.",
  "p.live.says":
    "Enquanto acompanhado, o quadro é lido novamente sempre que alguém adiciona, move ou altera uma tarefa aqui. Isso termina quando você sai da página.",
  "p.live.state_connecting": "Conectando…",
  "p.live.state_off": "Não acompanhando.",
  "p.live.state_on": "Acompanhando.",
  "p.live.state_retrying": "Conexão perdida; tentando novamente.",
  "p.live.state_stopped": "Interrompido: o serviço não quis enviar mudanças.",
  "p.live.stop": "Deixar de acompanhar",
  "p.live.velocity_says":
    "Tarefas e pontos concluídos em cada sprint, contados a partir do quadro; isto não é uma medida de fluxo.",
  "p.placeholder.choose": "Escolha um recurso genérico",
  "p.placeholder.confirm_over":
    "Atribuir isto a {person} o deixaria acima da capacidade em alguns dias. Fazer mesmo assim?",
  "p.placeholder.confirm_remove":
    "Remover este recurso genérico: {what}? A demanda que ele representava vai junto.",
  "p.placeholder.could_not": "Não foi possível alterar o recurso genérico.",
  "p.placeholder.f_person": "Pessoa (person: ou worker: e um id)",
  "p.placeholder.f_placeholder": "Recurso genérico",
  "p.placeholder.form": "Atribuir um recurso genérico a uma pessoa",
  "p.placeholder.give": "Atribuir à pessoa",
  "p.placeholder.given": "Atribuído a {person}.",
  "p.placeholder.heading":
    "Atribuir um recurso genérico a uma pessoa, ou removê-lo",
  "p.placeholder.not_given": "Não atribuído: nada foi alterado.",
  "p.placeholder.open": "aberto",
  "p.placeholder.option": "{role} em {plan}, {percent}, de {from} a {to}",
  "p.placeholder.problem_choose": "Escolha um recurso genérico.",
  "p.placeholder.problem_person":
    "Informe a pessoa como person: ou worker: e um id.",
  "p.placeholder.refused_forbidden":
    "Somente um administrador ou um gerente de recursos pode fazer isto.",
  "p.placeholder.refused_gone": "Essa reserva não existe mais.",
  "p.placeholder.refused_named":
    "Esta reserva já indica uma pessoa; a lista foi lida novamente.",
  "p.placeholder.refused_person": "O serviço não aceitou essa pessoa.",
  "p.placeholder.remove": "Remover o recurso genérico",
  "p.placeholder.removed": "Recurso genérico removido.",
  "p.placeholder.says":
    "Um recurso genérico é uma função solicitada sem pessoa ainda. Atribuí-lo a alguém é uma decisão do gerente de recursos; se isso deixar a pessoa acima da capacidade, você é consultado antes.",
  "p.planokr.as_of": "calculado {at}",
  "p.planokr.choose_plan": "Escolha um plano",
  "p.planokr.could_not": "Não foi possível ler os objetivos do plano.",
  "p.planokr.could_not_load_plans": "Não foi possível ler os planos.",
  "p.planokr.currencies": "resultados-chave em mais de uma moeda",
  "p.planokr.f_plan": "Plano",
  "p.planokr.form": "A pontuação de objetivos de um plano",
  "p.planokr.heading": "A pontuação de objetivos de um plano",
  "p.planokr.its_key_results": "Resultados-chave de {title}",
  "p.planokr.key_results_one": "{n} resultado-chave",
  "p.planokr.key_results_other": "{n} resultados-chave",
  "p.planokr.left_out_one":
    "{n} objetivo não é medido e fica de fora da pontuação.",
  "p.planokr.left_out_other":
    "{n} objetivos não são medidos e ficam de fora da pontuação.",
  "p.planokr.no_objectives": "Nenhum objetivo está alinhado a este plano.",
  "p.planokr.not_measured":
    "Não medido: nenhum objetivo alinhado a ele pode ser medido ainda.",
  "p.planokr.objective_not_measured": "não medido",
  "p.planokr.problem_plan": "Escolha um plano.",
  "p.planokr.read": "Ler a pontuação",
  "p.planokr.refused_gone": "Esse plano ou objetivo não existe mais.",
  "p.planokr.says":
    "A pontuação de cada objetivo alinhado ao plano, ponderada pela intensidade do alinhamento. Um objetivo que nada pode medir fica de fora, nunca contado como 0%.",
  "p.planokr.score": "Pontuação {score}",
  "p.planokr.unmeasurable_no_range":
    "sem progresso: o seu início e a sua meta são iguais",
  "p.planokr.unmeasurable_no_tolerance":
    "sem progresso: é mantido estável, sem tolerância",
  "p.planokr.unmeasurable_other": "nenhum progresso pode ser calculado",
  "p.planokr.unmeasurable_overflow":
    "sem progresso: os números são grandes demais",
  "p.planokr.unnamed": "Um objetivo ({id})",
  "p.planokr.weight": "peso {weight}",
  "p.rate.answer_person":
    "{rate} por hora em {date}, da tabela da própria pessoa, em vigor desde {from}.",
  "p.rate.answer_role":
    "{rate} por hora em {date}, da tabela da função dela, em vigor desde {from}.",
  "p.rate.could_not": "Não foi possível ler a taxa.",
  "p.rate.f_date": "Em",
  "p.rate.f_person": "Pessoa",
  "p.rate.f_table": "Tabela",
  "p.rate.form": "A taxa de custo em vigor",
  "p.rate.heading": "A taxa de custo em vigor",
  "p.rate.none": "Nenhuma taxa está em vigor para esta pessoa em {date}.",
  "p.rate.problem_date": "Informe o dia como uma data real.",
  "p.rate.problem_person": "Informe a referência da pessoa.",
  "p.rate.read": "Ler a taxa",
  "p.rate.refused_hidden":
    "Nenhuma taxa que você possa ver: as taxas são mostradas somente às finanças.",
  "p.rate.refused_query": "O serviço não aceitou esta pergunta.",
  "p.rate.says":
    "A taxa horária em vigor para uma pessoa em um dia: a da tabela dela ou, na falta dela, a da sua função. A tabela padrão, a menos que você indique outra.",
  "p.skillgap.choose_plan": "Escolha um plano",
  "p.skillgap.col_person": "Pessoa",
  "p.skillgap.col_skill": "Habilidade",
  "p.skillgap.col_status": "Situação",
  "p.skillgap.could_not": "Não foi possível ler as lacunas de habilidades.",
  "p.skillgap.count_covered_one": "{n} habilidade possuída;",
  "p.skillgap.count_covered_other": "{n} habilidades possuídas;",
  "p.skillgap.count_missing_one": "{n} habilidade ausente;",
  "p.skillgap.count_missing_other": "{n} habilidades ausentes;",
  "p.skillgap.count_unknown_one": "{n} desconhecida.",
  "p.skillgap.count_unknown_other": "{n} desconhecidas.",
  "p.skillgap.f_plan": "Plano",
  "p.skillgap.form": "As lacunas de habilidades de um plano",
  "p.skillgap.heading": "Habilidades que as reservas pedem",
  "p.skillgap.none": "Nenhuma reserva neste plano pede uma habilidade.",
  "p.skillgap.problem_plan": "Escolha um plano.",
  "p.skillgap.read": "Ler as lacunas de habilidades",
  "p.skillgap.says":
    "Cada habilidade que uma reserva do plano pede, em relação ao que a pessoa possui no serviço de força de trabalho. Desconhecida significa que as habilidades dela não puderam ser lidas; um recurso genérico ainda não tem pessoa e fica de fora.",
  "p.skillgap.status_covered": "possuída",
  "p.skillgap.status_missing": "ausente",
  "p.skillgap.status_unknown": "desconhecida",
  "p.skillgap.table": "Habilidades por reserva",
  "p.util.by_person": "Por pessoa",
  "p.util.by_team": "A equipe",
  "p.util.col_available": "Disponível",
  "p.util.col_away": "Ausente",
  "p.util.col_effort": "Esforço",
  "p.util.col_figure": "Uso da capacidade",
  "p.util.col_person": "Pessoa",
  "p.util.could_not": "Não foi possível ler o uso da capacidade.",
  "p.util.every_plan": "Todos os planos",
  "p.util.f_by": "Mostrar",
  "p.util.f_plan": "Plano",
  "p.util.f_window": "Dias para trás a partir de hoje",
  "p.util.form": "Esforço em relação à capacidade",
  "p.util.heading": "Esforço em relação à capacidade",
  "p.util.none": "Ninguém registrou esforço ou tempo ausente nesta janela.",
  "p.util.over": "na capacidade ou acima dela: um aviso, não uma conquista",
  "p.util.people_counted_one": "{n} pessoa contada.",
  "p.util.people_counted_other": "{n} pessoas contadas.",
  "p.util.problem_window":
    "Informe a janela como um número inteiro de dias, de 1 a {max}.",
  "p.util.read": "Ler o uso da capacidade",
  "p.util.refused": "O serviço não aceitou esta pergunta.",
  "p.util.says":
    "O esforço registrado sobre a capacidade declarada, menos o tempo ausente, nos {days} dias até hoje, a menos que você escolha de 1 a {max}. Na capacidade ou acima dela é um aviso sobre a fila. Grupos pequenos e números pequenos são omitidos.",
  "p.util.suppression":
    "Omitido: um grupo com menos de {people} pessoas, ou um número abaixo de {hours}.",
  "p.util.table": "Uso da capacidade por pessoa",
  "p.util.team_detail": "{effort} de {available} disponíveis, {away} ausentes",
  "p.util.unavailable_all_away": "ausente durante toda a janela, não 0%",
  "p.util.unavailable_below_floor": "pouco demais para informar",
  "p.util.unavailable_no_capacity": "nenhuma capacidade declarada",
  "p.util.unavailable_other": "não informável",
  "p.util.unavailable_small_group": "pessoas de menos para mostrar",
  "p.util.window_one": "No último {n} dia.",
  "p.util.window_other": "Nos últimos {n} dias.",
};
