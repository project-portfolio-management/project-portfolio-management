// Russian (ru-001) for the page and component catalogue (WW-1.1). Typed as a complete record of
// the English keys, so a missing string is a compile error.
//
// STATUS: MACHINE DRAFT. Written by Claude (the AI assistant) on 2026-11-21 and NOT reviewed by a
// speaker or a professional translator. The review is recorded in
// `governance/translation-reviews/ru-001.md`; update the status there, not here.

import type { PageKey } from "./i18n-pages";

export const PAGES_RU: Record<PageKey, string> = {
  "c.CalendarMonth.next_month": "Следующий месяц",
  "p.calendar.day": "День",
  "p.calendar.agenda": "Повестка",
  "p.calendar.status": "Статус",
  "p.calendar.status_done": "выполнено",
  "p.calendar.status_open": "открыто",
  "p.calendar.item": "Рабочий элемент",
  "p.calendar.showing": "Показано: {view}, с {start} по {end}",
  "p.calendar.more": "+ ещё {n}",
  "p.calendar.milestone": "Веха",
  "p.calendar.due": "Срок",
  "p.plans.pid.board.assignee": "Исполнитель",
  "p.plans.pid.board.moving": "Перемещение «{title}» в «{column}»…",
  "p.plans.pid.board.move_failed":
    "Не удалось переместить «{title}» в «{column}»; карточка возвращена на прежнее место.",
  "p.plans.pid.board.empty_column": "Задач нет",
  "p.plans.pid.board.search": "Поиск по доске",
  "p.plans.pid.board.task": "Задача",
  "p.plans.pid.board.status": "Статус",
  "c.GanttView.holiday": "Праздник",
  "c.GanttView.shaded":
    "Затенённые столбцы — нерабочие дни: выходные и праздники в рабочем календаре этого плана.",
  "c.GanttView.kind": "Вид",
  "c.GanttView.stage": "Этап",
  "c.GanttView.task": "Задача",
  "c.GanttView.start": "Начало",
  "c.GanttView.end": "Окончание",
  "c.GanttView.predecessors": "Зависит от",
  "c.GanttView.depends_on": "Зависит от: {names}",
  "c.GanttView.critical": "{name} (критический путь)",
  "c.GanttView.zoom_in": "Более мелкая шкала времени",
  "c.GanttView.zoom_out": "Более крупная шкала времени",
  "c.GanttView.zoomed": "Шкала времени: {unit}",
  "c.GanttView.unit_day": "день",
  "c.GanttView.unit_week": "неделя",
  "c.GanttView.unit_month": "месяц",
  "c.GanttView.export": "Экспортировать расписание",
  "c.GanttView.csv": "CSV",
  "c.GanttView.excel": "Excel",
  "c.GanttView.exported": "Экспортировано задач: {n}, файл {file}",
  "c.CalendarMonth.week": "Неделя",
  "c.CalendarMonth.four_weeks": "4 недели",
  "c.CalendarMonth.month": "Месяц",
  "c.CalendarMonth.today": "Сегодня",
  "c.CalendarMonth.previous": "Предыдущий период: {view}",
  "c.CalendarMonth.next": "Следующий период: {view}",
  "c.CalendarMonth.working_calendar": "Рабочий календарь",
  "c.CalendarMonth.previous_month": "Предыдущий месяц",
  "c.CapacityMeter.days_over_capacity": "Дней сверх мощности",
  "c.CostRiskView.adds": "{risk}: добавляет {money} — {eventHitText}",
  "c.CostRiskView.already_spent": "Уже потрачено",
  "c.CostRiskView.assumptions": "Допущения",
  "c.CostRiskView.budget": "бюджет",
  "c.CostRiskView.budget_2": "Бюджет",
  "c.CostRiskView.budget_sum_of_planned_amounts":
    "Бюджет (сумма плановых величин)",
  "c.CostRiskView.by_plan": "По планам",
  "c.CostRiskView.cost": "Стоимость",
  "c.CostRiskView.cost_at_each_percentile":
    "Стоимость ({currency}) на каждом процентиле",
  "c.CostRiskView.cost_from": "Стоимость от",
  "c.CostRiskView.each_bar_is_a_cost":
    "Каждый столбец — диапазон стоимости; чем он выше, тем больше симуляций завершилось в нём. Сплошные линии — P50, P80 и P90 (подписаны); штриховая линия — бюджет.",
  "c.CostRiskView.each_plan_s_budget_and":
    "Бюджет каждого плана и средняя смоделированная стоимость в {currency}",
  "c.CostRiskView.estimate": "Оценка",
  "c.CostRiskView.independence": "Независимость.",
  "c.CostRiskView.left_out": "Исключено:",
  "c.CostRiskView.line": "Статья",
  "c.CostRiskView.low_planned_high": "Низкая / плановая / высокая",
  "c.CostRiskView.mean_simulated_cost": "Средняя смоделированная стоимость",
  "c.CostRiskView.no_register_risk_was_used":
    "Ни один риск из реестра не использован как событие стоимости.",
  "c.CostRiskView.outcome": "Результат",
  "c.CostRiskView.per_line_estimate_and_sensitivity":
    "Оценка и чувствительность по статьям в {currency}",
  "c.CostRiskView.plan": "План",
  "c.CostRiskView.risks_that_added_cost": "Риски, увеличившие стоимость",
  "c.CostRiskView.seed_the_same_seed_gives":
    "Начальное число {seed}: то же начальное число даёт тот же результат. Не более {max_iterations} итераций.",
  "c.CostRiskView.sensitivity": "Чувствительность",
  "c.CostRiskView.simulations": "Симуляции",
  "c.CostRiskView.spent": "Потрачено",
  "c.CostRiskView.the_same_figures_as_a": "Те же данные в виде таблицы",
  "c.CostRiskView.to": "до",
  "c.CostRiskView.to_of": "от {money} до {money2}: {count} из {iterations}",
  "c.CostRiskView.what_drives_it": "Что влияет",
  "c.CpmGantt.cannot_meet_a_constraint_or":
    "не укладывается в ограничение или крайний срок (линия отмечает предел)",
  "c.CpmGantt.critical_any_slip_moves_the":
    "критическая: любой сдвиг переносит окончание",
  "c.CpmGantt.has_float_the_hatched_tail": "есть резерв (заштрихованная часть)",
  "c.CpmGantt.legend": "Легенда",
  "c.CpmGantt.near_critical": "околокритическая",
  "c.CpmGantt.no_scheduled_tasks_to_draw":
    "Запланированных задач для отображения пока нет. Задайте задаче длительность (и дату начала или предшественника).",
  "c.CpmGantt.the_outline_under_a_bar":
    "контур под полосой показывает, где она была в базовом плане",
  "c.CpmGantt.to_float_is_counted_in":
    "С {from} по {to}. Резерв считается в рабочих днях плана; полосы нарисованы в календарных днях.",
  "c.CpmTable.calendar": "Календарь",
  "c.CpmTable.constraint": "Ограничение",
  "c.CpmTable.deadline": "Крайний срок",
  "c.CpmTable.finishes": "Окончание",
  "c.CpmTable.free_float": "Свободный резерв",
  "c.CpmTable.overrides_its_predecessors": "(переопределяет предшественников)",
  "c.CpmTable.plan": "план",
  "c.CpmTable.plan_assignee": "план + исполнитель",
  "c.CpmTable.starts": "Начало",
  "c.CpmTable.state": "Состояние",
  "c.CpmTable.task": "Задача",
  "c.CpmTable.task_schedule_with_float_in":
    "Расписание задач с резервом в рабочих днях плана",
  "c.CpmTable.total_float": "Общий резерв",
  "c.CpmTable.untitled": "(без названия)",
  "c.CumulativeFlow.cumulative_flow": "Накопительный поток",
  "c.CumulativeFlow.day": "День",
  "c.CumulativeFlow.every_value_in_the_chart": "Все значения диаграммы. {note}",
  "c.CumulativeFlow.no_samples_in_this_window": "В этом окне нет замеров.",
  "c.CumulativeFlow.the_vertical_gap_above_done":
    "вертикальный промежуток над «Выполнено» — это незавершённая работа; горизонтальный промежуток до неё — примерно время цикла",
  "c.CumulativeFlow.total": "Итого",
  "c.CumulativeFlow.total_in_progress_done":
    "всего {total} · в работе {work_in_progress} · выполнено {done}",
  "c.EvBars.the_double_line_is_the":
    "Двойная линия — бюджет по завершении: {money}.",
  "c.EvBars.unknown": "неизвестно",
  "c.KpiCard.add_a_reading": "Добавить показание",
  "c.KpiCard.cadence": "Периодичность",
  "c.KpiCard.daily": "Ежедневно",
  "c.KpiCard.delete": "Удалить",
  "c.KpiCard.edit_or_delete": "Изменить или удалить",
  "c.KpiCard.history": "История ({readings})",
  "c.KpiCard.loading": "Загрузка…",
  "c.KpiCard.monthly": "Ежемесячно",
  "c.KpiCard.name": "Название",
  "c.KpiCard.no_reading_yet": "Показаний пока нет.",
  "c.KpiCard.no_readings": "Показаний нет.",
  "c.KpiCard.none": "Нет",
  "c.KpiCard.note": "Примечание",
  "c.KpiCard.observed": "Измерено",
  "c.KpiCard.observed_on": "Дата измерения",
  "c.KpiCard.on": "{valueText}, {observed_on}",
  "c.KpiCard.quarterly": "Ежеквартально",
  "c.KpiCard.read": "· показания: {cadence}",
  "c.KpiCard.record": "Записать",
  "c.KpiCard.red_line": "Красная линия",
  "c.KpiCard.save": "Сохранить",
  "c.KpiCard.target": "Целевое значение",
  "c.KpiCard.unit": "Единица",
  "c.KpiCard.value": "Значение",
  "c.KpiCard.weekly": "Еженедельно",
  "c.KpiCard.which_way_is_good_cannot":
    "Направление улучшения ({v}) изменить нельзя: это перекрасило бы все прошлые показания.",
  "c.LilyGanttTable.critical_path": "критический путь",
  "c.LilyGanttTable.day": "День",
  "c.LilyGanttTable.month": "Месяц",
  "c.LilyGanttTable.no_dated_tasks_to_show": "Задач с датами для показа нет.",
  "c.LilyGanttTable.no_usable_dates": "Пригодных дат нет:",
  "c.LilyGanttTable.task": "Задача",
  "c.LilyGanttTable.that_range_is_too_long":
    "Этот диапазон слишком длинный для одного столбца на каждое значение «{unit}»; показан один столбец на «{unit2}».",
  "c.LilyGanttTable.time_scale": "Шкала времени",
  "c.LilyGanttTable.week": "Неделя",
  "c.LilyKanbanTable.move_to": "Переместить «{title}» в",
  "c.LilyKanbanTable.no_tasks_yet": "Задач пока нет.",
  "c.LilyKanbanTable.unknown_status": "Неизвестный статус:",
  "c.LineRange.high": "Высокая",
  "c.LineRange.low": "Низкая",
  "c.LineRange.planned_is_the_likely_cost":
    "Плановая стоимость — {money}, наиболее вероятная. Оставьте поле пустым, чтобы очистить его.",
  "c.LineRange.save_range": "Сохранить диапазон",
  "c.MilestoneCard.by": "Кем",
  "c.MilestoneCard.by_hand_only": "Только вручную",
  "c.MilestoneCard.complete": "Завершить",
  "c.MilestoneCard.completes": "Завершается",
  "c.MilestoneCard.done": "Выполнено",
  "c.MilestoneCard.due": "Срок {due} · {kind}",
  "c.MilestoneCard.every_move_is_kept_in":
    "Каждый перенос сохраняется в истории: прежняя дата, новая дата, причина и кто перенёс.",
  "c.MilestoneCard.forecast": "· прогноз {early_finish}",
  "c.MilestoneCard.from": "С",
  "c.MilestoneCard.loading": "Загрузка…",
  "c.MilestoneCard.mark_done": "Отметить выполненной",
  "c.MilestoneCard.move": "Перенести",
  "c.MilestoneCard.never_rescheduled": "Не переносилась.",
  "c.MilestoneCard.new_due_date": "Новый срок",
  "c.MilestoneCard.no_reason_given": "причина не указана",
  "c.MilestoneCard.open": "Открыта",
  "c.MilestoneCard.pick_the_tasks_that_must":
    "Выберите задачи, которые должны быть выполнены",
  "c.MilestoneCard.reason_required": "Причина (обязательно)",
  "c.MilestoneCard.reopen": "Открыть снова",
  "c.MilestoneCard.reschedule": "Перенести срок",
  "c.MilestoneCard.save": "Сохранить",
  "c.MilestoneCard.slip_history": "История сдвигов ({reschedules})",
  "c.MilestoneCard.tasks_it_waits_for": "Задачи, которых она ждёт",
  "c.MilestoneCard.tasks_this_milestone_waits_for":
    "Задачи, которых ждёт эта веха",
  "c.MilestoneCard.this_plan_has_no_tasks": "В этом плане пока нет задач.",
  "c.MilestoneCard.to": "На",
  "c.MilestoneCard.view_plan": "Открыть план",
  "c.MilestoneCard.when_all_those_tasks_are": "Когда все эти задачи выполнены",
  "c.MilestoneCard.why": "Почему",
  "c.NetworkDiagram.float": "{slice} → {slice2} · резерв {total_float}",
  "c.NetworkDiagram.no_dependencies": "Зависимостей нет.",
  "c.NetworkDiagram.no_scheduled_tasks_to_draw":
    "Запланированных задач для отображения пока нет.",
  "c.NetworkDiagram.task_network_boxes_are_tasks":
    "Сетевая диаграмма задач: прямоугольники — задачи, стрелки — зависимости",
  "c.NetworkDiagram.the_same_network_as_a": "Та же сеть в виде списка",
  "c.ObjectiveCard.add_a_key_result": "Добавить ключевой результат",
  "c.ObjectiveCard.add_key_result": "Добавить ключевой результат",
  "c.ObjectiveCard.check_in": "Отметить прогресс",
  "c.ObjectiveCard.confidence_0_100": "Уверенность (0–100)",
  "c.ObjectiveCard.currency": "Валюта",
  "c.ObjectiveCard.currency_e_g_gbp": "Валюта (например, EUR, JPY или GBP)",
  "c.ObjectiveCard.decrease": "Уменьшение",
  "c.CommandPalette.cmd_dashboard": "Панель",
  "c.CommandPalette.cmd_escalations": "Эскалации",
  "c.CommandPalette.cmd_my_work": "Моя работа",
  "c.CommandPalette.cmd_new_plan": "Новый план",
  "c.CommandPalette.count_one": "{n} результат",
  "c.CommandPalette.count_other": "{n} результатов",
  "c.CommandPalette.failed":
    "Поиск планов сейчас недоступен. Команды по-прежнему работают.",
  "c.CommandPalette.kind_command": "Команда",
  "c.CommandPalette.kind_plan": "План",
  "c.CommandPalette.kind_recent": "Недавнее",
  "c.CommandPalette.label": "Поиск планов и команд",
  "c.CommandPalette.none": "Ничего не найдено",
  "c.CommandPalette.open": "Поиск и команды",
  "c.CommandPalette.placeholder": "Введите название плана или команду",
  "c.CommandPalette.title": "Поиск и команды",
  "c.NavItems.portfolios": "Портфели",
  "c.NavItems.projects": "Проекты",
  "c.NavItems.products": "Продукты",
  "c.NavItems.programs": "Программы",
  "c.NavItems.practices": "Практики",
  "c.NavItems.processes": "Процессы",
  "c.NavItems.purposes": "Цели",
  "c.NavItems.pathways": "Пути",
  "c.PasteTasks.check": "Проверить на сервере",
  "c.PasteTasks.clear": "Очистить",
  "c.PasteTasks.col_due": "Крайний срок",
  "c.PasteTasks.col_line": "Строка",
  "c.PasteTasks.col_notes": "Примечания",
  "c.PasteTasks.col_status": "Статус",
  "c.PasteTasks.col_title": "Название",
  "c.PasteTasks.create_one": "Создать 1 задачу",
  "c.PasteTasks.create_other": "Создать задачи: {n}",
  "c.PasteTasks.done": "Готово",
  "c.PasteTasks.failed": "Не удалось вставить данные.",
  "c.PasteTasks.help":
    "Скопируйте ячейки из электронной таблицы и вставьте их сюда: одна задача на строку, столбцы по порядку — «Название», «Крайний срок» (гггг-мм-дд) и «Примечания». Дата становится крайним сроком задачи. Первая строка с названиями столбцов пропускается. Строка с ошибкой показывается с причиной и исключается.",
  "c.PasteTasks.label": "Вставленные строки",
  "c.PasteTasks.left_out": "Исключено: {why}",
  "c.PasteTasks.problem_beyond_limit": "больше {max} строк",
  "c.PasteTasks.problem_date_bad":
    "«{value}» не является датой; запишите её в виде гггг-мм-дд",
  "c.PasteTasks.problem_no_title": "нет названия",
  "c.PasteTasks.problem_notes_long": "примечания длиннее {max} символов",
  "c.PasteTasks.problem_same_as": "совпадает со строкой {line}",
  "c.PasteTasks.problem_title_long": "название длиннее {max} символов",
  "c.PasteTasks.ready": "Готово к созданию",
  "c.PasteTasks.tally": "готово: {good}, исключено: {left}",
  "c.PasteTasks.title": "Вставка строк из электронной таблицы",
  "c.PasteTasks.would": "Что произойдёт",
  "c.ConfirmHost.cancel": "Отмена",
  "c.ConfirmHost.ok": "OK",
  "c.ConfirmHost.title": "Подтвердите действие",
  "c.ObjectiveCard.delete": "Удалить",
  "c.ObjectiveCard.direction": "Направление",
  "c.ObjectiveCard.edit_or_delete": "Изменить или удалить",
  "c.ObjectiveCard.edit_or_delete_this_objective":
    "Изменить или удалить эту цель",
  "c.ObjectiveCard.from": "{valueText} (было {start_value})",
  "c.ObjectiveCard.increase": "Увеличение",
  "c.ObjectiveCard.it_has_no_measurable_key":
    "— у неё пока нет измеримого ключевого результата, поэтому оценки нет (это не ноль).",
  "c.ObjectiveCard.key_results_in_more_than":
    "Ключевые результаты в нескольких валютах оцениваются отдельно; валюты никогда не пересчитываются.",
  "c.ObjectiveCard.last_check_in": "· последняя отметка прогресса {slice}",
  "c.ObjectiveCard.maintain": "Поддержание",
  "c.ObjectiveCard.metric": "Метрика",
  "c.ObjectiveCard.no_key_results_yet": "Ключевых результатов пока нет.",
  "c.ObjectiveCard.note": "Примечание",
  "c.ObjectiveCard.number": "Число",
  "c.ObjectiveCard.percent": "Процент",
  "c.ObjectiveCard.period": "Период",
  "c.ObjectiveCard.record": "Записать",
  "c.ObjectiveCard.save": "Сохранить",
  "c.ObjectiveCard.start": "Начало",
  "c.ObjectiveCard.target": "Целевое значение",
  "c.ObjectiveCard.the_baseline_metric_and_direction":
    "Исходное значение, метрику и направление изменить нельзя: прогресс от смещающегося исходного значения не является прогрессом.",
  "c.ObjectiveCard.title": "Название",
  "c.ObjectiveCard.tolerance": "· допуск ±{tolerance}",
  "c.ObjectiveCard.tolerance_2": "Допуск",
  "c.ObjectiveCard.unit": "Единица",
  "c.ObjectiveCard.value": "Значение",
  "c.ObjectiveCard.yes_no": "Да / нет",
  "c.PlanForm.organization": "organization:…",
  "c.PlanForm.parent_plan_pid_uuid": "pid родительского плана (UUID)",
  "c.PlanForm.proj_2026": "ПРОЕКТ-2026",
  "c.RaciMatrix.complete": "заполнено",
  "c.RaciMatrix.gaps": "Пробелы",
  "c.RaciMatrix.task": "Задача",
  "c.RaciMatrix.who_does_what_raci_matrix":
    "Кто что делает (матрица RACI): кто Ответственный, Подотчётный, Консультирующий и Информируемый по каждой задаче",
  "c.RiskInputs.adds_cost": "Увеличивает стоимость",
  "c.RiskInputs.by_business_days": "на (рабочих дней)",
  "c.RiskInputs.currency": "Валюта",
  "c.RiskInputs.delays_task": "Задерживает задачу",
  "c.RiskInputs.gbp": "EUR",
  "c.RiskInputs.none": "Нет",
  "c.RiskInputs.one_probability_covers_both_effects":
    "Одна вероятность охватывает оба последствия: риск, который задерживает задачу и увеличивает стоимость, наступает один раз и вызывает и то и другое. Оставьте часть пустой, чтобы исключить её.",
  "c.RiskInputs.probability": "Вероятность (%)",
  "c.RiskInputs.save": "Сохранить",
  "c.RiskInputs.saving": "Сохранение…",
  "c.RiskInputs.simulation_inputs": "Входные данные симуляции",
  "c.SCurve.actual": "Фактически",
  "c.SCurve.date": "Дата",
  "c.SCurve.earned": "Освоено",
  "c.SCurve.line_planned_value_to_earned":
    "Линия: плановый объём (с {from} по {to}). ● освоенный объём и ■ фактическая стоимость, только за записанные дни (пока {snapshots}). Штриховая линия: бюджет по завершении, {money}.",
  "c.SCurve.no_curve_to_draw": "Нет кривой для отображения.",
  "c.SCurve.not_recorded": "не записано",
  "c.SCurve.planned": "План",
  "c.SCurve.s_curve_planned_value_over":
    "S-кривая: плановый объём во времени с записанными освоенным объёмом и фактической стоимостью",
  "c.SCurve.the_same_figures_as_a": "Те же данные в виде таблицы",
  "c.VarianceTable.baselined": "Базовый план",
  "c.VarianceTable.duration": "Длительность Δ",
  "c.VarianceTable.finish": "Окончание Δ",
  "c.VarianceTable.now": "Сейчас",
  "c.VarianceTable.schedule_variance_against_the_baseli":
    "Отклонение расписания от базового плана в рабочих днях",
  "c.VarianceTable.standing": "Положение",
  "c.VarianceTable.start": "Начало Δ",
  "c.VarianceTable.task": "Задача",
  "c.VarianceTable.the_plan_s_finish_is": "Окончание плана не изменилось.",
  "c.VarianceTable.the_plan_s_own_finish":
    "Собственное окончание плана сравнить нельзя.",
  "p.announcements.announcements": "Объявления",
  "p.announcements.announcements_ppm": "Объявления — PPM",
  "p.announcements.expires_optional": "Действует до (необязательно)",
  "p.announcements.important": "Важное",
  "p.announcements.info": "Информация",
  "p.announcements.loading": "Загрузка…",
  "p.announcements.message": "Сообщение",
  "p.announcements.nothing_has_been_announced": "Объявлений нет.",
  "p.announcements.pin_to_the_top": "Закрепить вверху",
  "p.announcements.pinned": "· Закреплено",
  "p.announcements.publish": "Опубликовать",
  "p.announcements.publish_an_announcement": "Опубликовать объявление",
  "p.announcements.publishing_needs_the_announcements_p":
    "Для публикации нужно разрешение на объявления.",
  "p.announcements.retract": "Отозвать",
  "p.announcements.severity": "Важность",
  "p.announcements.title": "Заголовок",
  "p.announcements.urgent": "Срочное",
  "p.auditor.action": "Действие",
  "p.auditor.actor": "Исполнитель",
  "p.auditor.actor_less_recorded_actions":
    "Записанные действия без исполнителя:",
  "p.auditor.audit_trail": "Журнал аудита",
  "p.auditor.bearer_sub": "sub предъявителя",
  "p.auditor.detail": "Подробности",
  "p.auditor.e_g_merged": "например, merged",
  "p.auditor.entity": "Сущность",
  "p.auditor.evidence_pack_csv": "Пакет доказательств (CSV)",
  "p.auditor.filter": "Фильтр",
  "p.auditor.findings": "Выводы",
  "p.auditor.no_audit_rows_match": "Подходящих записей аудита нет.",
  "p.auditor.no_findings": "Выводов нет.",
  "p.auditor.ppm": "{t} — PPM",
  "p.auditor.rows_distinct_actors_actor_less":
    "строк: {returned} · различных исполнителей: {distinct_actors} · без исполнителя: {actorless}",
  "p.auditor.rule": "Правило",
  "p.auditor.submitted_without_a_verified_token":
    "(отправлено без проверенного токена)",
  "p.auditor.when": "Когда",
  "p.automations.action": "действие",
  "p.automations.action_2": "Действие",
  "p.automations.add_label": "добавить метку",
  "p.automations.any": "любой",
  "p.automations.assign": "назначить",
  "p.automations.assignee": "исполнитель",
  "p.automations.automation": "автоматизация",
  "p.automations.automations_ppm": "Автоматизации — PPM",
  "p.automations.blank_every_plan": "пусто = все планы",
  "p.automations.cancel": "Отмена",
  "p.automations.claim_based_so_a_deadline":
    "Основано на захвате, поэтому срок срабатывает ровно один раз, даже если таймер и ручной обход выполняются одновременно.",
  "p.automations.configure_a_rule_once_as":
    "Настройте правило один раз; когда элементы перемещаются по доске, платформа применяет его. Правила никогда не блокируют перемещение, которое их запустило: сбой записывается как неудачный запуск, а не как откат.",
  "p.automations.delete": "Удалить",
  "p.automations.detail": "Подробности",
  "p.automations.disable": "Отключить",
  "p.automations.due": "Срок",
  "p.automations.enable": "Включить",
  "p.automations.every_plan": "все планы",
  "p.automations.expire_review": "срок действия рецензии истёк",
  "p.automations.from": "из",
  "p.automations.in_calendar_days": "в календарных днях",
  "p.automations.label": "метка",
  "p.automations.message": "сообщение",
  "p.automations.name": "название",
  "p.automations.name_2": "Название",
  "p.automations.no_rules_configured": "Правила не настроены.",
  "p.automations.nothing_has_fired_yet": "Пока ничего не сработало.",
  "p.automations.nothing_pending": "Ожидающих нет.",
  "p.automations.notify": "уведомить",
  "p.automations.off": "выкл.",
  "p.automations.on": "вкл.",
  "p.automations.operator": "оператор",
  "p.automations.outcome": "Результат",
  "p.automations.person_uuid": "person:<uuid>",
  "p.automations.plan_pid": "pid плана",
  "p.automations.plan_stage_changed": "этап плана изменился",
  "p.automations.recipient": "получатель",
  "p.automations.review_submitted": "рецензия отправлена",
  "p.automations.rules": "Правила",
  "p.automations.runs": "Запуски",
  "p.automations.save_rule": "Сохранить правило",
  "p.automations.schedule_action": "запланировать действие",
  "p.automations.scope": "Область",
  "p.automations.set_and_forget": "Настроить и забыть",
  "p.automations.set_task_status": "задать статус задачи",
  "p.automations.source": "Источник",
  "p.automations.state": "Состояние",
  "p.automations.status": "статус",
  "p.automations.subject": "Тема",
  "p.automations.sweep_due_actions": "Выполнить наступившие действия",
  "p.automations.task_moved": "задача перемещена",
  "p.automations.then": "тогда",
  "p.automations.to": "в",
  "p.automations.trigger": "триггер",
  "p.automations.trigger_2": "Триггер",
  "p.automations.when": "Когда",
  "p.automations.workflow_automation": "Автоматизация рабочих процессов",
  "p.board.amber": "жёлтый",
  "p.board.amount": "Сумма",
  "p.board.benefit_realizations": "реализации выгод",
  "p.board.board_pack": "Пакет для совета директоров",
  "p.board.decision": "Решение",
  "p.board.decisions_in_window": "Решения в окне",
  "p.board.green": "зелёный",
  "p.board.investment_decisions": "Инвестиционные решения",
  "p.board.kind": "Вид",
  "p.board.milestones_completed": "вех завершено",
  "p.board.no_decisions_in_the_window": "В этом окне решений нет.",
  "p.board.no_investment_decisions_recorded":
    "Инвестиционных решений не записано.",
  "p.board.no_snapshots_captured_yet": "Снимков пока нет.",
  "p.board.open_exposure": "Открытая подверженность рискам",
  "p.board.portfolios": "Портфели",
  "p.board.ppm": "{t} — PPM",
  "p.board.realized_in_window": "Реализовано в окне:",
  "p.board.red_portfolios": "красных портфелей",
  "p.board.subject": "Тема",
  "p.board.take_snapshot_now": "Сделать снимок сейчас",
  "p.board.taken": "Сделан",
  "p.board.tranches_released": "траншей выделено",
  "p.board.trends": "Тенденции",
  "p.board.unattributed_events": "(событий без автора: {unattributed_events})",
  "p.board.what": "Что",
  "p.board.when": "Когда",
  "p.board.window": "окно {from} → {to}",
  "p.board.work_items": "Рабочие элементы",
  "p.calendar.all": "(все)",
  "p.calendar.done": "· выполнено",
  "p.calendar.kind": "Вид",
  "p.calendar.no_milestones_match": "Подходящих вех нет.",
  "p.calendar.ppm": "{t} — PPM",
  "p.capacity.capacity_ppm": "Мощности — PPM",
  "p.capacity.from": "с",
  "p.capacity.hatched_extension_proposed_not_count":
    "штриховое продолжение: предложено, не учитывается",
  "p.capacity.legend": "Легенда",
  "p.capacity.over": "превышение",
  "p.capacity.over_capacity_solid_bar_firm":
    "превышение мощности (сплошная полоса: твёрдые бронирования; линия: действующая мощность)",
  "p.capacity.proposed": "Предложено",
  "p.capacity.to": "по",
  "p.capacity.window": "окно: {from} → {to}",
  "p.capacity.within_capacity": "в пределах мощности",
  "p.capacity.within_capacity_but_committing_the":
    "в пределах мощности, но утверждение предложенных бронирований её превысит",
  "p.ceo.all_announcements": "Все объявления",
  "p.ceo.all_key_measures": "Все ключевые показатели",
  "p.ceo.all_milestones": "Все вехи",
  "p.ceo.and_more": "и ещё {more}",
  "p.ceo.announcements": "Объявления",
  "p.ceo.ceo_dashboard": "Панель генерального директора",
  "p.ceo.ceo_dashboard_ppm": "Панель генерального директора — PPM",
  "p.ceo.decision_log": "Журнал решений",
  "p.ceo.escalated_risks": "эскалированных рисков",
  "p.ceo.every_kpi_is_on_target": "● Все KPI достигают целевых значений.",
  "p.ceo.executive_detail": "Подробности для руководства",
  "p.ceo.key_measures_needing_a_look":
    "Ключевые показатели, требующие внимания",
  "p.ceo.loading": "Загрузка…",
  "p.ceo.milestones_to_watch": "Вехи под наблюдением",
  "p.ceo.no_data_yet": "Данных пока нет",
  "p.ceo.no_decisions_recorded_yet": "Решений пока не записано.",
  "p.ceo.no_kpis_defined_yet": "KPI пока не определены.",
  "p.ceo.no_open_milestones": "Открытых вех нет.",
  "p.ceo.no_portfolio_is_over_budget": "● Ни один портфель не превышает бюджет",
  "p.ceo.nothing_has_been_announced": "Объявлений нет.",
  "p.ceo.over_budget": "▲ Превышение бюджета: {join}",
  "p.ceo.overdue_milestones": "просроченных вех: {overdueMilestones}",
  "p.ceo.portfolio_health": "Состояние портфеля",
  "p.ceo.recent_decisions": "Недавние решения",
  "p.ceo.red_amber_green":
    "■ красных: {red} · ▲ жёлтых: {amber} · ● зелёных: {green}",
  "p.ceo.risk_and_money": "Риски и деньги",
  "p.ceo.risk_register": "Реестр рисков",
  "p.compliance.compliance_risk_register":
    "Реестр рисков соответствия требованиям",
  "p.compliance.conformance_findings": "Выводы о соответствии",
  "p.compliance.detail": "Подробности",
  "p.compliance.exposure": "Подверженность риску",
  "p.compliance.gate_review_recency_threshold_calend":
    "порог давности рассмотрения контрольной точки: {review_days} календарных дней",
  "p.compliance.item": "Элемент",
  "p.compliance.no_compliance_risks_recorded":
    "Рисков соответствия требованиям не записано.",
  "p.compliance.no_conformance_findings": "Выводов о соответствии нет.",
  "p.compliance.open_exposure":
    "{note} · открытая подверженность {open_exposure}",
  "p.compliance.owner": "Владелец",
  "p.compliance.ppm": "{t} — PPM",
  "p.compliance.risk": "Риск",
  "p.compliance.rule": "Правило",
  "p.compliance.status": "Статус",
  "p.cost_rates.a_person": "Сотрудник",
  "p.cost_rates.a_role": "Роль",
  "p.cost_rates.add_a_rate_table": "Добавить таблицу ставок",
  "p.cost_rates.add_rate": "Добавить ставку",
  "p.cost_rates.add_table": "Добавить таблицу",
  "p.cost_rates.cost_rates": "Ставки стоимости",
  "p.cost_rates.cost_rates_are_finance_data":
    "Ставки стоимости — финансовые данные. У вас нет доступа к ним.",
  "p.cost_rates.cost_rates_ppm": "Ставки стоимости — PPM",
  "p.cost_rates.currency": "Валюта",
  "p.cost_rates.effective_from": "Действует с",
  "p.cost_rates.for": "Для",
  "p.cost_rates.from": "С",
  "p.cost_rates.name": "Название",
  "p.cost_rates.no_rate_tables_yet": "Таблиц ставок пока нет.",
  "p.cost_rates.no_rate_yet_work_by":
    "Ставки пока нет: работа этого объекта ({v}) не оценивается в стоимости.",
  "p.cost_rates.per_hour": "В час",
  "p.cost_rates.person_person_reference": "Сотрудник (ссылка person:…)",
  "p.cost_rates.role": "Роль",
  "p.cost_risk.band_for_lines_with_no": "Диапазон для статей без оценки (± %)",
  "p.cost_risk.distribution": "Распределение",
  "p.cost_risk.iterations": "Итерации",
  "p.cost_risk.pert": "PERT",
  "p.cost_risk.portfolio_cost_risk": "Стоимостные риски портфеля",
  "p.cost_risk.portfolio_cost_risk_ppm": "Стоимостные риски портфеля — PPM",
  "p.cost_risk.resources": "Ресурсы",
  "p.cost_risk.run": "Запустить",
  "p.cost_risk.running": "Выполнение…",
  "p.cost_risk.seed": "Начальное число",
  "p.cost_risk.shows_people_instead": "вместо этого показывает сотрудников.",
  "p.cost_risk.triangular": "Треугольное",
  "p.cost_risk.what_all_your_plans_together":
    "Сколько могут стоить все ваши планы вместе с учётом диапазона каждой статьи бюджета и рисков, увеличивающих стоимость. Каждая валюта считается отдельно; они никогда не складываются.",
  "p.dashboard.dashboard_ppm": "Панель — PPM",
  "p.display.display": "Отображение",
  "p.display.display_ppm": "Отображение — PPM",
  "p.display.dyslexia_friendly_font": "Шрифт, удобный при дислексии",
  "p.display.go_back_to_the_default":
    "Вернуть значение по умолчанию для моей роли",
  "p.display.how_the_app_looks_and":
    "Как приложение выглядит и читается. Эти настройки никогда не меняют ваших возможностей.",
  "p.display.keeps_the_everyday_pages_in":
    "Оставляет на виду повседневные страницы (Моя работа, Планы, Доска, Календарь, Обзор), а остальные прячет за ссылкой «Ещё». Все страницы по-прежнему доступны и работают.",
  "p.display.sign_in_to_keep_these":
    "Войдите, чтобы сохранить эти настройки и в вашей учётной записи, а не только на этом устройстве.",
  "p.display.simple_view": "Простой вид",
  "p.display.this_is_the_default_for":
    "Это значение по умолчанию для вашей роли: {v}.",
  "p.display.use_simple_view": "Использовать простой вид",
  "p.display.sections_title": "Разделы в верхней панели",
  "p.display.sections_help":
    "Снимите отметку с раздела, чтобы скрыть его из верхней панели. Он останется доступен по своему адресу, и здесь его можно показать снова. Ничего не сохраняется, пока вы не скроете раздел, поэтому добавленные позже разделы будут вам доступны.",
  "p.display.sections_legend": "Показывать эти разделы",
  "p.display.sections_hidden_count_one": "Скрыт {n} раздел",
  "p.display.sections_hidden_count_other": "Скрыто разделов: {n}",
  "p.display.sections_none_hidden": "Скрытых разделов нет.",
  "p.display.sections_show_all": "Показать все разделы",
  "p.display.week_start_title": "Первый день недели",
  "p.display.week_start_help":
    "С какого дня начинается неделя в календарях. По умолчанию определяется выбранным языком; номера недель остаются номерами по ISO и начинаются с понедельника.",
  "p.display.week_start_label": "Неделя начинается в",
  "p.display.week_start_locale": "Обычный день для этого языка",
  "p.display.week_start_mon": "Понедельник",
  "p.display.week_start_sun": "Воскресенье",
  "p.display.week_start_sat": "Суббота",
  "p.display.use_the_dyslexia_friendly_font":
    "Использовать шрифт, удобный при дислексии",
  "p.display.uses_the_opendyslexic_typeface_with":
    "Использует шрифт OpenDyslexic с увеличенными интервалами между буквами, словами и строками.",
  "p.display.you_chose_this": "Вы выбрали это.",
  "p.documents.controlled_documents": "Контролируемые документы",
  "p.documents.create_document": "Создать документ",
  "p.documents.document": "Документ",
  "p.documents.documents": "Документы",
  "p.documents.documents_ppm": "Документы — PPM",
  "p.documents.first_draft": "Первый черновик",
  "p.documents.in_force": "Действует",
  "p.documents.loading": "Загрузка…",
  "p.documents.new_document": "Новый документ",
  "p.documents.no_documents_yet": "Документов пока нет.",
  "p.documents.pid.approver_s_id": "Идентификатор утверждающего",
  "p.documents.pid.date": "Дата",
  "p.documents.pid.document": "Документ",
  "p.documents.pid.documents": "← Документы",
  "p.documents.pid.look_up": "Найти",
  "p.documents.pid.nothing_was_in_force_on": "На {on} ничего не действовало.",
  "p.documents.pid.person_id": "идентификатор сотрудника",
  "p.documents.pid.save_draft": "Сохранить черновик",
  "p.documents.pid.send_back": "Вернуть",
  "p.documents.pid.text": "Текст",
  "p.documents.pid.this_version_s_text_is":
    "Текст этой версии окончателен. Изменение вносится в следующий черновик.",
  "p.documents.pid.version": "Версия {version}",
  "p.documents.pid.version_was_in_force_on":
    "Версия {version} действовала на {on}.",
  "p.documents.pid.versions": "Версии",
  "p.documents.pid.what_changed": "Что изменилось",
  "p.documents.pid.what_was_in_force_on": "Что действовало на дату",
  "p.documents.policies_and_procedures_a_new":
    "Политики и процедуры. Новая версия проверяется и утверждается до вступления в силу, а действующая версия никогда не меняется.",
  "p.documents.title": "Название",
  "p.documents.versions": "Версии",
  "p.engineering.assignee": "Исполнитель",
  "p.engineering.blocked_calendar_days": "Заблокировано (календарных дней)",
  "p.engineering.blocked_work": "Заблокированная работа",
  "p.engineering.declared_cause_failure_rate":
    "доля сбоев с объявленной причиной",
  "p.engineering.delivery_links": "Связи с поставкой",
  "p.engineering.deploys_mo": "развёртываний ({window_months} мес.)",
  "p.engineering.devops_metrics": "Метрики DevOps",
  "p.engineering.environment": "Среда",
  "p.engineering.incidents": "инцидентов",
  "p.engineering.incidents_unresolved_counted_never_t":
    "Нерешённых инцидентов: {unresolved_incidents} (учтены, но время не измерялось).",
  "p.engineering.item": "Элемент",
  "p.engineering.items_carry_no_moscow_tag":
    "Элементов без тега MoSCoW: {untagged}.",
  "p.engineering.median_recovery_h": "медианное время восстановления (ч)",
  "p.engineering.moscow_scope": "Объём по MoSCoW",
  "p.engineering.no_deploy_events_ingested_yet":
    "События развёртывания пока не загружены.",
  "p.engineering.no_externally_tracked_items":
    "Элементов, отслеживаемых во внешних системах, нет.",
  "p.engineering.nothing_blocked_ship_it":
    "Ничего не заблокировано. Выпускайте.",
  "p.engineering.ppm": "{t} — PPM",
  "p.engineering.releases": "Выпуски",
  "p.engineering.task": "Задача",
  "p.engineering.trackers": "Трекеры",
  "p.engineering.untracked": "Не отслеживается:",
  "p.engineering.version": "Версия",
  "p.engineering.when": "Когда",
  "p.executive.actor": "Исполнитель",
  "p.executive.aligned": "Согласовано",
  "p.executive.as_of": "по состоянию на {as_of} · {derivation}",
  "p.executive.benefits": "Выгоды",
  "p.executive.benefits_realization": "Реализация выгод",
  "p.executive.calendar_days_since_update": "Календарных дней с обновления",
  "p.executive.collection": "Набор",
  "p.executive.currency": "Валюта",
  "p.executive.decision": "Решение",
  "p.executive.decision_log": "Журнал решений",
  "p.executive.escalated_risks": "Эскалированные риски",
  "p.executive.kind": "Вид",
  "p.executive.largest_unaligned_items": "Крупнейшие несогласованные элементы",
  "p.executive.members": "Участники",
  "p.executive.no_decisions_recorded_yet": "Решений пока не записано.",
  "p.executive.no_target": "нет цели",
  "p.executive.non_financial_only": "только нефинансовые ({non_financial})",
  "p.executive.open_exposure": "Открытая подверженность рискам",
  "p.executive.overdue_milestones": "Просроченные вехи",
  "p.executive.overrun_currencies": "Валюты с перерасходом",
  "p.executive.planned_with_no_objective": "запланировано без цели",
  "p.executive.portfolio": "Портфель",
  "p.executive.portfolio_health": "Состояние портфеля",
  "p.executive.ppm": "{t} — PPM",
  "p.executive.realization": "Реализация",
  "p.executive.realized": "Реализовано",
  "p.executive.status": "Статус",
  "p.executive.strategic_alignment": "Стратегическое соответствие",
  "p.executive.subject": "Тема",
  "p.executive.target": "Цель",
  "p.executive.total": "Итого",
  "p.executive.unaligned": "Не согласовано",
  "p.executive.unaligned_spend": "Расходы без согласования",
  "p.executive.when": "Когда",
  "p.financials.actual": "Фактически",
  "p.financials.as_of": "по состоянию на {as_of} · {note}",
  "p.financials.currency": "Валюта",
  "p.financials.currency_exposure": "Валютная подверженность",
  "p.financials.lines": "Статьи",
  "p.financials.no_budget_lines_recorded_yet":
    "Статей бюджета пока не записано.",
  "p.financials.planned": "План",
  "p.financials.ppm": "{t} — PPM",
  "p.financials.remaining": "Остаток",
  "p.financials.work_items": "Рабочие элементы",
  "p.gantt.main_x": "{t} — Main X",
  "p.ideas.convert_target": "Преобразовать в",
  "p.ideas.ideas_ppm": "Идеи — PPM",
  "p.kpis.all": "Все",
  "p.kpis.blank_for_the_whole_portfolio": "пусто для всего портфеля",
  "p.kpis.daily": "Ежедневно",
  "p.kpis.define": "Определить",
  "p.kpis.define_a_key_measure_kpi": "Определить ключевой показатель (KPI)",
  "p.kpis.higher_is_better": "Чем выше, тем лучше",
  "p.kpis.key_measures_kpis": "Ключевые показатели (KPI)",
  "p.kpis.kpis_by_status": "KPI по статусу",
  "p.kpis.kpis_ppm": "KPI — PPM",
  "p.kpis.loading": "Загрузка…",
  "p.kpis.lower_is_better": "Чем ниже, тем лучше",
  "p.kpis.monthly": "Ежемесячно",
  "p.kpis.name": "Название",
  "p.kpis.no_data_yet": "Данных пока нет",
  "p.kpis.not_set": "Не задано",
  "p.kpis.off_target": "Вне цели",
  "p.kpis.on_target": "В пределах цели",
  "p.kpis.percent_defects_gbp": "процент, дефекты, EUR…",
  "p.kpis.plan_optional_pid": "План (необязательный pid)",
  "p.kpis.quarterly": "Ежеквартально",
  "p.kpis.read_how_often": "Как часто снимать показания",
  "p.kpis.red_line": "Красная линия",
  "p.kpis.show": "Показать",
  "p.kpis.something_you_track_over_time":
    "То, что вы отслеживаете во времени относительно цели, чтобы заранее видеть, идёте ли вы по курсу.",
  "p.kpis.standing_measures_green_is_at":
    "Постоянные показатели. Зелёный — на уровне цели или лучше; красный — на уровне красной линии или хуже; жёлтый — между ними. KPI, по которому ещё нет показаний, отображается как «Данных пока нет».",
  "p.kpis.target": "Целевое значение",
  "p.kpis.the_red_line_must_be":
    "Красная линия должна лежать по худшую сторону от цели. Приводите значения к целым числам (базисные пункты, пенсы).",
  "p.kpis.unit": "Единица",
  "p.kpis.watch": "Наблюдать",
  "p.kpis.weekly": "Еженедельно",
  "p.kpis.which_way_is_good": "Какое направление лучше",
  "p.layout.display": "Отображение",
  "p.layout.more": "Ещё",
  "p.layout.simple_view": "Простой вид: {v}",
  "p.lifecycle.as_of": "по состоянию на {as_of}",
  "p.lifecycle.in_an_unrecognized_phase": "в нераспознанной фазе",
  "p.lifecycle.lifecycle": "Жизненный цикл",
  "p.lifecycle.lifecycle_ppm": "Жизненный цикл — PPM",
  "p.lifecycle.live": "Активные",
  "p.lifecycle.live_items_one":
    "{total} активный элемент · порог застоя {stall}",
  "p.lifecycle.live_items_other":
    "Активных элементов: {total} · порог застоя {stall}",
  "p.lifecycle.phase": "Фаза",
  "p.lifecycle.share": "Доля",
  "p.lifecycle.stalled": "Застряли",
  "p.lifecycle.where_everything_sits_across_the":
    "Где что находится в жизненном цикле, чтобы следующая фаза никогда не была неожиданностью. В «Застряли» считаются элементы, находящиеся в фазе дольше порога.",
  "p.me.calendar.add": "Добавить",
  "p.me.calendar.at_least_one_weekday_must":
    "Хотя бы один день недели должен быть рабочим.",
  "p.me.calendar.from": "С",
  "p.me.calendar.loading": "Загрузка…",
  "p.me.calendar.most_i_can_commit": "Максимум, который я могу выделить (%)",
  "p.me.calendar.my_calendar": "Мой календарь",
  "p.me.calendar.my_calendar_ppm": "Мой календарь — PPM",
  "p.me.calendar.my_days_off": "Мои выходные дни",
  "p.me.calendar.my_work": "← Моя работа",
  "p.me.calendar.none_set": "Не задано.",
  "p.me.calendar.note": "Примечание",
  "p.me.calendar.part_time_spells_and_leave":
    "Периоды неполной занятости и отпуска",
  "p.me.calendar.remove": "Удалить",
  "p.me.calendar.save": "Сохранить",
  "p.me.calendar.saved": "Сохранено.",
  "p.me.calendar.the_days_i_work": "Дни, в которые я работаю",
  "p.me.calendar.the_most_of_your_time":
    "Максимум вашего времени, который вы можете выделить между двумя датами: 60 для периода неполной занятости, 0 для отпуска. Вне этих дат значение равно 100. Мощность считается по дням, поэтому короткий период не усредняется.",
  "p.me.calendar.to": "с {from} по {to}: {max_percent}%{v}",
  "p.me.calendar.to_2": "По",
  "p.me.due": "· срок {due_date}",
  "p.me.in_plans_you_cannot_read":
    "· {omitted_unreadable} в планах, которые вы не можете просматривать",
  "p.me.loading": "Загрузка…",
  "p.me.my_calendar": "Мой календарь",
  "p.me.my_work": "Моя работа",
  "p.me.my_work_ppm": "Моя работа — PPM",
  "p.me.nothing_assigned_to_you_enjoy": "Вам ничего не назначено. Отдыхайте.",
  "p.me.open": "{today} ({tz}) · открыто: {total}",
  "p.me.showing_the_first_500": "· показаны первые 500",
  "p.me.the_days_you_work_so":
    "— дни, в которые вы работаете, чтобы ваши даты были точными.",
  "p.me.your_sign_in_identity_cannot":
    "Вашу учётную запись входа не удаётся сопоставить с назначениями задач, поэтому пока нечего показать.",
  "p.milestones.a_milestone_is_a_dated":
    "Веха — это обещание с датой. Свяжите её с задачами, которых она ждёт, и она будет достигнута, когда они выполнены, а прогноз возьмётся из расписания; не связывайте её, и человек отметит её выполненной вручную.",
  "p.milestones.achieved": "Достигнута",
  "p.milestones.all": "Все",
  "p.milestones.due_soon": "Скоро срок",
  "p.milestones.forecast": "Прогноз:",
  "p.milestones.late": "С опозданием",
  "p.milestones.loading": "Загрузка…",
  "p.milestones.milestones": "Вехи",
  "p.milestones.milestones_by_status": "Вехи по статусу",
  "p.milestones.milestones_ppm": "Вехи — PPM",
  "p.milestones.show": "Показать",
  "p.milestones.upcoming": "Предстоящие",
  "p.objectives.objectives_ppm": "Цели — PPM",
  "p.objectives.period_e_g_2026_h2": "Период (например, 2026-H2)",
  "p.okrs.2026_q4_blank_for_all": "2026-Q4 (пусто для всех)",
  "p.okrs.a_goal_with_a_few":
    "Цель с несколькими измеримыми результатами, показывающими, достигнута ли она.",
  "p.okrs.create": "Создать",
  "p.okrs.goals_and_key_results_okrs": "Цели и ключевые результаты (OKR)",
  "p.okrs.loading": "Загрузка…",
  "p.okrs.new_objective": "Новая цель",
  "p.okrs.objectives_and_key_results_scores":
    "Цели и ключевые результаты. Оценки рассчитываются по отметкам прогресса при каждом просмотре; ничего не хранится, что могло бы устареть.",
  "p.okrs.okrs_ppm": "OKR — PPM",
  "p.okrs.period": "Период",
  "p.okrs.show": "Показать",
  "p.okrs.title": "Название",
  "p.onboarding.a_role_by_role_first":
    "Пошаговое знакомство на первый час для каждой роли. Каждый шаг называет страницу, на которую он ведёт. Это рекомендуемый порядок, а не измеренное утверждение вроде «вы начнёте работать продуктивно за N минут» — время никто не замерял.",
  "p.onboarding.onboarding_ppm": "Введение в работу — PPM",
  "p.onboarding.operator_onboarding": "Введение в работу для оператора",
  "p.plans.main_x": "{t} — Main X",
  "p.plans.merge.ppm": "{t} — PPM",
  "p.plans.new.main_x": "{t} — Main X",
  "p.plans.pid.board.action": "действие",
  "p.plans.pid.board.add_note": "Добавить заметку",
  "p.plans.pid.board.add_sprint": "Добавить спринт",
  "p.plans.pid.board.add_task": "Добавить задачу",
  "p.plans.pid.board.board": "Доска",
  "p.plans.pid.board.burndown": "Диаграмма сгорания — {name}",
  "p.plans.pid.board.convert_to_task": "Преобразовать в задачу",
  "p.plans.pid.board.created_moves_blocked_now":
    "создано: {length} · перемещений: {length2} · заблокировано сейчас: {length3}",
  "p.plans.pid.board.date": "Дата",
  "p.plans.pid.board.feedback": "обратная связь",
  "p.plans.pid.board.improve": "улучшить",
  "p.plans.pid.board.new_task_title": "Название новой задачи",
  "p.plans.pid.board.no_notes_for_this_sprint":
    "Заметок для этого спринта пока нет.",
  "p.plans.pid.board.none": "(нет)",
  "p.plans.pid.board.note": "Заметка",
  "p.plans.pid.board.note_body": "Текст заметки",
  "p.plans.pid.board.note_category": "Категория заметки",
  "p.plans.pid.board.points_done": "Выполнено пунктов",
  "p.plans.pid.board.ppm": "{t} — PPM",
  "p.plans.pid.board.pts": "пункт.",
  "p.plans.pid.board.remaining_of": "Осталось из {total_tasks}",
  "p.plans.pid.board.retro_feedback_notes":
    "Заметки ретроспективы и обратной связи",
  "p.plans.pid.board.sprint": "Спринт",
  "p.plans.pid.board.sprint_end": "Окончание спринта",
  "p.plans.pid.board.sprint_name": "Название спринта",
  "p.plans.pid.board.sprint_start": "Начало спринта",
  "p.plans.pid.board.sprints": "Спринты",
  "p.plans.pid.board.standup_last_24h": "Планёрка (последние 24 ч)",
  "p.plans.pid.board.story_points": "Сторипоинты",
  "p.plans.pid.board.task_created": "→ задача создана",
  "p.plans.pid.board.tasks_done": "Выполнено задач",
  "p.plans.pid.board.unpointed": "Без оценки",
  "p.plans.pid.board.velocity": "Скорость",
  "p.plans.pid.board.went_well": "прошло хорошо",
  "p.plans.pid.calendar.at_least_one_weekday_must":
    "Хотя бы один день недели должен быть рабочим.",
  "p.plans.pid.calendar.critical_path": "← Критический путь",
  "p.plans.pid.calendar.holidays": "Праздники",
  "p.plans.pid.calendar.loading": "Загрузка…",
  "p.plans.pid.calendar.my_calendar": "Мой календарь",
  "p.plans.pid.calendar.save_calendar": "Сохранить календарь",
  "p.plans.pid.calendar.saved": "Сохранено.",
  "p.plans.pid.calendar.their_work_then_happens_only":
    "); их работа тогда выполняется только в дни, рабочие в обоих календарях.",
  "p.plans.pid.calendar.working_calendar": "Рабочий календарь",
  "p.plans.pid.calendar.working_calendar_ppm": "Рабочий календарь — PPM",
  "p.plans.pid.calendar.working_weekdays": "Рабочие дни недели",
  "p.plans.pid.cost_risk.band_for_lines_with_no":
    "Диапазон для статей без оценки (± %)",
  "p.plans.pid.cost_risk.cost_and_schedule_together":
    "Стоимость и расписание вместе",
  "p.plans.pid.cost_risk.cost_risk": "Стоимостные риски",
  "p.plans.pid.cost_risk.cost_risk_ppm": "Стоимостные риски — PPM",
  "p.plans.pid.cost_risk.distribution": "Распределение",
  "p.plans.pid.cost_risk.iterations": "Итерации",
  "p.plans.pid.cost_risk.many_what_if_runs_of":
    "Множество прогонов бюджета по принципу «что, если» показывают диапазон итоговой стоимости и вероятность каждого значения.",
  "p.plans.pid.cost_risk.pert": "PERT",
  "p.plans.pid.cost_risk.plan": "← План",
  "p.plans.pid.cost_risk.run": "Запустить",
  "p.plans.pid.cost_risk.running": "Выполнение…",
  "p.plans.pid.cost_risk.seed": "Начальное число",
  "p.plans.pid.cost_risk.triangular": "Треугольное",
  "p.plans.pid.cost_risk.whole_portfolio": "Весь портфель",
  "p.plans.pid.costs.a_recorded_cost_keeps_the":
    "Записанная стоимость сохраняет ставку, по которой она была оценена; последующее изменение ставки её не перезаписывает.",
  "p.plans.pid.costs.add_an_expense": "Добавить расход",
  "p.plans.pid.costs.add_expense": "Добавить расход",
  "p.plans.pid.costs.all_at_the_end": "Всё в конце",
  "p.plans.pid.costs.all_at_the_start": "Всё в начале",
  "p.plans.pid.costs.amount": "Сумма",
  "p.plans.pid.costs.an_expense_of_so_it":
    "▲ Расход {money} {reason}, поэтому его нет в периодах.",
  "p.plans.pid.costs.choose": "Выберите…",
  "p.plans.pid.costs.cost_rates": "Ставки стоимости",
  "p.plans.pid.costs.costs": "Затраты",
  "p.plans.pid.costs.costs_and_rates_are_finance":
    "Затраты и ставки — финансовые данные. У вас нет доступа к ним.",
  "p.plans.pid.costs.costs_ppm": "Затраты — PPM",
  "p.plans.pid.costs.currency": "Валюта",
  "p.plans.pid.costs.description": "Описание",
  "p.plans.pid.costs.evenly_across_the_task": "Равномерно по задаче",
  "p.plans.pid.costs.falls": "Приходится на",
  "p.plans.pid.costs.fixed_cost": "Фиксированная стоимость",
  "p.plans.pid.costs.kind": "Вид",
  "p.plans.pid.costs.material": "Материалы",
  "p.plans.pid.costs.no_time_has_been_costed":
    "Время пока не оценено в стоимости.",
  "p.plans.pid.costs.nothing_could_be_costed_yet":
    "Пока нечего оценить в стоимости: добавьте ставки для людей, выполняющих работу, или добавьте расход.",
  "p.plans.pid.costs.per_period": "{currency}, за период",
  "p.plans.pid.costs.plan": "← План",
  "p.plans.pid.costs.planned_cost": "Плановая стоимость",
  "p.plans.pid.costs.record_labour_costs": "Записать затраты на труд",
  "p.plans.pid.costs.recorded_labour_cost": "Записанные затраты на труд",
  "p.plans.pid.costs.so_its_labour_is_not":
    "▲ {title} ({hoursText}): {reason}, поэтому затраты на труд не оценены.",
  "p.plans.pid.costs.task": "Задача",
  "p.plans.pid.costs.time_entries_are_not_costed":
    "▲ Записей времени ({hoursText}): {entries} — не оценены в стоимости: ещё не записаны или нет ставки. Их стоимость неизвестна, а не равна нулю.",
  "p.plans.pid.costs.total": "Итого",
  "p.plans.pid.costs.work": "Работа",
  "p.plans.pid.critical_path.a_baseline_is_a_numbered":
    "Базовый план — это пронумерованный снимок расписания, сохраняемый навсегда: повторное утверждение базового плана создаёт следующую версию и никогда не заменяет прежнюю.",
  "p.plans.pid.critical_path.baselines": "Базовые планы",
  "p.plans.pid.critical_path.business_days": "рабочих дней",
  "p.plans.pid.critical_path.compare_with": "Сравнить с",
  "p.plans.pid.critical_path.constraint": "Ограничение",
  "p.plans.pid.critical_path.constraint_date": "Дата ограничения",
  "p.plans.pid.critical_path.critical_chains":
    "Критических цепочек: {count}{more}.",
  "p.plans.pid.critical_path.critical_path": "Критический путь",
  "p.plans.pid.critical_path.critical_path_ppm": "Критический путь — PPM",
  "p.plans.pid.critical_path.deadline_a_goal_it_shows":
    "Крайний срок (цель: отображается как отрицательный резерв, задачу не сдвигает)",
  "p.plans.pid.critical_path.duration_business_days_0_is":
    "Длительность (рабочих дней; 0 — веха)",
  "p.plans.pid.critical_path.finish_no_earlier_than": "Окончание не раньше",
  "p.plans.pid.critical_path.finish_no_later_than": "Окончание не позже",
  "p.plans.pid.critical_path.gantt": "Диаграмма Ганта",
  "p.plans.pid.critical_path.loading": "Загрузка…",
  "p.plans.pid.critical_path.must_finish_on_overrides_logic":
    "Должна закончиться в (переопределяет логику)",
  "p.plans.pid.critical_path.must_start_on_overrides_logic":
    "Должна начаться в (переопределяет логику)",
  "p.plans.pid.critical_path.near_critical_within":
    "Околокритические в пределах",
  "p.plans.pid.critical_path.network": "Сеть",
  "p.plans.pid.critical_path.no_baseline_yet_so_nothing":
    "Базового плана пока нет, поэтому о сдвигах ничего сказать нельзя.",
  "p.plans.pid.critical_path.none_as_soon_as_possible":
    "Нет (как можно раньше)",
  "p.plans.pid.critical_path.not_scheduled": "Не запланировано",
  "p.plans.pid.critical_path.optimistic_business_days":
    "Оптимистичная (рабочих дней)",
  "p.plans.pid.critical_path.pessimistic_business_days":
    "Пессимистичная (рабочих дней)",
  "p.plans.pid.critical_path.plan": "← План",
  "p.plans.pid.critical_path.planned_start": "Плановое начало",
  "p.plans.pid.critical_path.range_for_schedule_risk":
    "Диапазон для риска расписания",
  "p.plans.pid.critical_path.range_saved": "Диапазон сохранён.",
  "p.plans.pid.critical_path.reason_required_this_overrides_the":
    "Причина (обязательно: это переопределяет логику расписания)",
  "p.plans.pid.critical_path.save_range": "Сохранить диапазон",
  "p.plans.pid.critical_path.save_schedule": "Сохранить расписание",
  "p.plans.pid.critical_path.saved_moved": "Сохранено. Сдвинуто:",
  "p.plans.pid.critical_path.saved_nothing_else_moved":
    "Сохранено. Больше ничего не сдвинулось.",
  "p.plans.pid.critical_path.saving": "Сохранение…",
  "p.plans.pid.critical_path.show_the_baseline_under_each":
    "Показывать базовый план под каждой полосой",
  "p.plans.pid.critical_path.start_no_earlier_than": "Начало не раньше",
  "p.plans.pid.critical_path.start_no_later_than": "Начало не позже",
  "p.plans.pid.critical_path.table": "Таблица",
  "p.plans.pid.critical_path.take_baseline": "Зафиксировать базовый план",
  "p.plans.pid.critical_path.task": "Задача",
  "p.plans.pid.critical_path.the_duration_above_is_the":
    "{taskRangeText}. Длительность выше — наиболее вероятное значение.",
  "p.plans.pid.critical_path.the_longest_chain_of_tasks":
    "Самая длинная цепочка задач, определяющая дату окончания. Задержка любой задачи в ней задерживает весь план.",
  "p.plans.pid.critical_path.these_assignees_share_no_business":
    "У этих исполнителей нет общих рабочих дней с календарём плана, поэтому для них использован календарь плана: {join}.",
  "p.plans.pid.critical_path.this_network_has_tasks_too":
    "В этой сети задач: {length} — слишком много для наглядного рисунка. Таблица ниже содержит то же самое.",
  "p.plans.pid.critical_path.untitled": "(без названия)",
  "p.plans.pid.critical_path.version": "Версия {version}: {reason} ({slice})",
  "p.plans.pid.critical_path.view": "Вид",
  "p.plans.pid.critical_path.why_a_baseline_now": "Почему базовый план сейчас?",
  "p.plans.pid.critical_path.working_calendar": "Рабочий календарь",
  "p.plans.pid.earned_value.a_percent_i_enter_flagged":
    "Процент, который я ввожу (с пометкой)",
  "p.plans.pid.earned_value.actual_cost": "Фактическая стоимость",
  "p.plans.pid.earned_value.against_the_budget_vac":
    "Относительно бюджета (VAC)",
  "p.plans.pid.earned_value.baseline_version":
    "версия базового плана {baseline_version}",
  "p.plans.pid.earned_value.budget_at_completion": "Бюджет по завершении",
  "p.plans.pid.earned_value.budget_lines_in_have_no":
    "У статей бюджета в {join} нет базового плана, поэтому они не входят ни в одну из величин выше и никогда не суммируются с другой валютой.",
  "p.plans.pid.earned_value.by_story_points_when_finished":
    "по сторипоинтам, когда завершено",
  "p.plans.pid.earned_value.cancel": "Отмена",
  "p.plans.pid.earned_value.change": "Изменить",
  "p.plans.pid.earned_value.complete": "{round}% выполнено",
  "p.plans.pid.earned_value.cost_index": "Индекс стоимости (",
  "p.plans.pid.earned_value.cost_is": "Стоимость: {word}.",
  "p.plans.pid.earned_value.cost_variance_ev_ac":
    "Отклонение по стоимости (EV − AC)",
  "p.plans.pid.earned_value.count_a_task_another_way": "Считать задачу иначе",
  "p.plans.pid.earned_value.count_tasks": "Считать задачи",
  "p.plans.pid.earned_value.critical_path": "Критический путь",
  "p.plans.pid.earned_value.earned_value": "Освоенный объём",
  "p.plans.pid.earned_value.earned_value_figures":
    "Показатели освоенного объёма в {currency}",
  "p.plans.pid.earned_value.earned_value_ppm": "Освоенный объём — PPM",
  "p.plans.pid.earned_value.final_cost_eac": "Итоговая стоимость (EAC)",
  "p.plans.pid.earned_value.finished_only": "только завершённые",
  "p.plans.pid.earned_value.forecast": "Прогноз",
  "p.plans.pid.earned_value.forecast_final_cost":
    "· прогноз итоговой стоимости {money}",
  "p.plans.pid.earned_value.forecasts_of_the_final_cost":
    "Прогнозы итоговой стоимости в {currency}, каждый по своей формуле",
  "p.plans.pid.earned_value.governance": "← Управление",
  "p.plans.pid.earned_value.half_for_started": "половина за начатые",
  "p.plans.pid.earned_value.is_the_work_finished_worth":
    "Стоит ли выполненная работа столько, сколько было запланировано к этому моменту, и сколько она стоила? Индекс выше 1,00 лучше плана.",
  "p.plans.pid.earned_value.leading_with_that_one_because":
    "Этот прогноз показан первым, потому что {why}. Остальные показаны рядом: какой из них верен, зависит от того, продолжатся ли прошлые перерасходы.",
  "p.plans.pid.earned_value.loading": "Загрузка…",
  "p.plans.pid.earned_value.manual": "вручную",
  "p.plans.pid.earned_value.measured_as_of_percent_complete":
    "Измерено по состоянию на {as_of}. Процент выполнения — это выполненная работа, а не прошедшее время.",
  "p.plans.pid.earned_value.method": "Метод",
  "p.plans.pid.earned_value.only_when_finished": "только по завершении",
  "p.plans.pid.earned_value.only_when_finished_2": "Только по завершении",
  "p.plans.pid.earned_value.over_time": "Во времени",
  "p.plans.pid.earned_value.percent_complete": "Процент выполнения",
  "p.plans.pid.earned_value.planned_value": "Плановый объём",
  "p.plans.pid.earned_value.plans": "планов: {plans}",
  "p.plans.pid.earned_value.progress_against_budget_earned_value":
    "Прогресс относительно бюджета (освоенный объём)",
  "p.plans.pid.earned_value.record_today_s_figures_now":
    "Записать сегодняшние показатели сейчас",
  "p.plans.pid.earned_value.recorded": "Записано.",
  "p.plans.pid.earned_value.save": "Сохранить",
  "p.plans.pid.earned_value.schedule_index": "Индекс расписания (",
  "p.plans.pid.earned_value.schedule_index_by_time":
    "Индекс расписания по времени (",
  "p.plans.pid.earned_value.schedule_is": "Расписание: {word}.",
  "p.plans.pid.earned_value.schedule_variance_ev_pv":
    "Отклонение по расписанию (EV − PV)",
  "p.plans.pid.earned_value.spi": "SPI {ratioOrReason} ·",
  "p.plans.pid.earned_value.started_work_counts_half":
    "начатая работа считается наполовину",
  "p.plans.pid.earned_value.started_work_counts_half_2":
    "Начатая работа считается наполовину",
  "p.plans.pid.earned_value.still_to_spend_etc": "Осталось потратить (ETC)",
  "p.plans.pid.earned_value.task": "(задача)",
  "p.plans.pid.earned_value.task_2": "Задача",
  "p.plans.pid.earned_value.the_one_to_lead_with":
    "— его стоит показывать первым",
  "p.plans.pid.earned_value.the_scheduler_also_records_them":
    "Планировщик также записывает их ежедневно.",
  "p.plans.pid.earned_value.this_plan_and_everything_beneath":
    "Этот план и всё, что под ним",
  "p.plans.pid.earned_value.unknown": "неизвестно",
  "p.plans.pid.earned_value.use_the_plan_s_method": "Использовать метод плана",
  "p.plans.pid.earned_value.where_it_is_heading": "Куда всё движется",
  "p.plans.pid.earned_value.why_required": "Почему (обязательно)",
  "p.plans.pid.edit.main_x": "{t} — Main X",
  "p.plans.pid.flow.180_calendar_days": "180 календарных дней",
  "p.plans.pid.flow.30_calendar_days": "30 календарных дней",
  "p.plans.pid.flow.60_calendar_days": "60 календарных дней",
  "p.plans.pid.flow.90_calendar_days": "90 календарных дней",
  "p.plans.pid.flow.age": "Возраст",
  "p.plans.pid.flow.aging_work_in_progress": "Старение незавершённой работы",
  "p.plans.pid.flow.arrival_rate": "Интенсивность поступления (λ)",
  "p.plans.pid.flow.assignee": "Исполнитель",
  "p.plans.pid.flow.blocked": "Заблокировано",
  "p.plans.pid.flow.calendar_day": "{v} / календарный день",
  "p.plans.pid.flow.column": "Столбец",
  "p.plans.pid.flow.columns": "Столбцы",
  "p.plans.pid.flow.concentrated": "сосредоточено",
  "p.plans.pid.flow.constraints": "Ограничения",
  "p.plans.pid.flow.count": "Количество",
  "p.plans.pid.flow.cumulative_flow": "Накопительный поток",
  "p.plans.pid.flow.cycle_time_is_work_in":
    "Время цикла — это объём незавершённой работы, делённый на пропускную способность, поэтому снижение лимита сокращает его, и никому не нужно работать быстрее.",
  "p.plans.pid.flow.detail": "Подробности",
  "p.plans.pid.flow.distributions": "Распределения",
  "p.plans.pid.flow.first_pass_yield": "Выход с первого раза",
  "p.plans.pid.flow.flow": "Поток",
  "p.plans.pid.flow.flow_efficiency": "Эффективность потока",
  "p.plans.pid.flow.flow_ppm": "Поток — PPM",
  "p.plans.pid.flow.how_long_work_takes": "Сколько длится работа",
  "p.plans.pid.flow.how_long_work_takes_to":
    "Сколько времени занимает выполнение работы и сколько её находится в процессе одновременно.",
  "p.plans.pid.flow.implied_cycle_time": "Расчётное время цикла",
  "p.plans.pid.flow.item": "Элемент",
  "p.plans.pid.flow.limit": "Лимит",
  "p.plans.pid.flow.max": "макс.",
  "p.plans.pid.flow.mean": "среднее",
  "p.plans.pid.flow.measure": "Показатель",
  "p.plans.pid.flow.min": "мин.",
  "p.plans.pid.flow.no_cap": "без ограничения",
  "p.plans.pid.flow.no_expectation_yet": "ожидания пока нет",
  "p.plans.pid.flow.no_findings_yet": "Выводов пока нет.",
  "p.plans.pid.flow.no_finished_work_to_measure":
    "Завершённой работы для измерения пока нет.",
  "p.plans.pid.flow.not_enough_finished_items_yet":
    "Завершённых элементов пока недостаточно.",
  "p.plans.pid.flow.not_started_finished_of":
    "не начато: {not_started} · завершено: {finished} из {tasks}",
  "p.plans.pid.flow.nothing_in_progress": "В работе ничего нет.",
  "p.plans.pid.flow.observed_median": "Наблюдаемая медиана",
  "p.plans.pid.flow.of_the_calendar_time_these":
    "Какую часть календарного времени, затраченного на эти элементы, над ними кто-то работал? Эффективность потока в интеллектуальной работе обычно составляет 5–15 %, поэтому низкое значение нормально: вывод в том, что есть очередь, а не в усилиях.",
  "p.plans.pid.flow.of_the_transitions_behind_these":
    "{percent} переходов, лежащих в основе этих показателей, были синтезированы при введении журнала, а не наблюдались. По мере накопления реальных перемещений по доске показатели становятся надёжнее.",
  "p.plans.pid.flow.percentiles_are_nearest_rank_so":
    "Процентили вычисляются по методу ближайшего ранга, поэтому каждый из них — реальный элемент. Время цикла контролирует команда; время выполнения — это то, сколько ждёт заказчик, а разница между ними — сколько работа пролежала в резерве задач.",
  "p.plans.pid.flow.recoverable": "Можно вернуть",
  "p.plans.pid.flow.rework": "Доработка",
  "p.plans.pid.flow.rule": "Правило",
  "p.plans.pid.flow.service_level_expectation":
    "Ожидаемый уровень обслуживания",
  "p.plans.pid.flow.status": "Статус",
  "p.plans.pid.flow.strong_against_the_5_15":
    "Высокий результат по сравнению с нормой 5–15 %.",
  "p.plans.pid.flow.subject": "Тема",
  "p.plans.pid.flow.the_mean_is_shown_for":
    "Среднее показано для полноты и не описывает ни один реальный элемент — ссылайтесь на p85.",
  "p.plans.pid.flow.the_plan_s_overall_ratio":
    ": общий показатель плана ({percent}) сильно отличается от показателя типичного элемента ({percent2}). Большинство элементов движется, меньшинство застряло. Это требует иного решения, чем равномерно медленная поставка: найдите эти элементы, а не перестраивайте процесс.",
  "p.plans.pid.flow.the_waste_is": "Потери составляют",
  "p.plans.pid.flow.throughput": "Пропускная способность (μ)",
  "p.plans.pid.flow.typical_median_item":
    "Типично. Медианный элемент: {percent}.",
  "p.plans.pid.flow.unassigned": "не назначено",
  "p.plans.pid.flow.unusually_high_this_usually_means":
    "Необычно высоко — обычно это означает, что доску не обновляют, а не что очередь исчезла.",
  "p.plans.pid.flow.utilization": "Загрузка (ρ)",
  "p.plans.pid.flow.vs_expectation": "относительно ожидания",
  "p.plans.pid.flow.window": "Окно",
  "p.plans.pid.flow.work_in_progress": "Незавершённая работа",
  "p.plans.pid.flow.work_in_progress_2": "Незавершённая работа (κ)",
  "p.plans.pid.governance.actual": "Фактически",
  "p.plans.pid.governance.approved": "утверждено",
  "p.plans.pid.governance.approved_with_conditions": "утверждено с условиями",
  "p.plans.pid.governance.benefit": "Выгода",
  "p.plans.pid.governance.benefit_category": "Категория выгоды",
  "p.plans.pid.governance.capex": "капитальные затраты",
  "p.plans.pid.governance.category": "Категория",
  "p.plans.pid.governance.choose_objective": "выберите цель…",
  "p.plans.pid.governance.commit": "Зафиксировать",
  "p.plans.pid.governance.commit_anyway": "Всё равно зафиксировать",
  "p.plans.pid.governance.committing_this_puts_them_over":
    "Фиксация этого превысит их мощность.",
  "p.plans.pid.governance.compliance": "соответствие требованиям",
  "p.plans.pid.governance.cost_saving": "экономия затрат",
  "p.plans.pid.governance.currency": "Валюта",
  "p.plans.pid.governance.decision": "Решение",
  "p.plans.pid.governance.governance_ppm": "Управление — PPM",
  "p.plans.pid.governance.hold": "приостановлено",
  "p.plans.pid.governance.line": "Статья",
  "p.plans.pid.governance.materialized": "наступило: {materialized}",
  "p.plans.pid.governance.max_exposure":
    "максимальная подверженность: {max_exposure}",
  "p.plans.pid.governance.new_benefit": "Новая выгода",
  "p.plans.pid.governance.new_budget_line": "Новая статья бюджета",
  "p.plans.pid.governance.new_milestone": "Новая веха",
  "p.plans.pid.governance.new_milestone_due": "Срок новой вехи",
  "p.plans.pid.governance.new_risk": "Новый риск",
  "p.plans.pid.governance.next": "далее: {next_gate}",
  "p.plans.pid.governance.objective": "Цель",
  "p.plans.pid.governance.open_risks": "открытых рисков: {open}",
  "p.plans.pid.governance.opex": "операционные затраты",
  "p.plans.pid.governance.other": "прочее",
  "p.plans.pid.governance.planned": "План",
  "p.plans.pid.governance.planned_actual_variance":
    ": план {money} · факт {money2} · отклонение {money3}",
  "p.plans.pid.governance.planned_major": "План (в основных единицах)",
  "p.plans.pid.governance.proposed_does_not_count_until":
    "предложено (не учитывается до фиксации)",
  "p.plans.pid.governance.quality": "качество",
  "p.plans.pid.governance.range_for_cost_risk":
    "Диапазон для стоимостного риска",
  "p.plans.pid.governance.realized": "Реализовано",
  "p.plans.pid.governance.record_actual": "Записать факт",
  "p.plans.pid.governance.rejected": "отклонено",
  "p.plans.pid.governance.revenue": "доход",
  "p.plans.pid.governance.review": "Рассмотрение",
  "p.plans.pid.governance.risk": "Риск",
  "p.plans.pid.governance.risk_reduction": "снижение рисков",
  "p.plans.pid.governance.roi": "· ROI {toFixed}%",
  "p.plans.pid.governance.stage": "этап: {v}",
  "p.plans.pid.governance.status": "Статус",
  "p.plans.pid.governance.target": "Цель",
  "p.plans.pid.governance.target_major": "Цель (в основных единицах)",
  "p.plans.pid.governance.target_realized_spend":
    ": цель {money} · реализовано {money2} · расходы {money3}",
  "p.plans.pid.governance.weight": "вес",
  "p.plans.pid.governance.worker_uuid": "worker:<uuid>",
  "p.plans.pid.handover.discard": "Отменить",
  "p.plans.pid.handover.from": "От",
  "p.plans.pid.handover.hand_over": "Передать",
  "p.plans.pid.handover.handover": "Передача дел",
  "p.plans.pid.handover.handover_ppm": "Передача дел — PPM",
  "p.plans.pid.handover.kind": "Вид",
  "p.plans.pid.handover.onboarding_and_offboarding":
    "Приём и увольнение сотрудников",
  "p.plans.pid.handover.person": "person:…",
  "p.plans.pid.handover.person_empty_releases_the_work":
    "person:… (пусто — освободить работу)",
  "p.plans.pid.handover.plan": "← План",
  "p.plans.pid.handover.preview": "Предпросмотр",
  "p.plans.pid.handover.reason_recorded_with_each_item":
    "Причина (записывается с каждым элементом)",
  "p.plans.pid.handover.release": "Освободить",
  "p.plans.pid.handover.the_new_person_s_capacity":
    "Мощность нового сотрудника здесь не проверяется.",
  "p.plans.pid.handover.to": "Кому",
  "p.plans.pid.handover.what": "Что",
  "p.plans.pid.handover.what_holds_that_is_still":
    "Что ещё актуально у сотрудника {from_person}",
  "p.plans.pid.handover.who_is_handing_over": "Кто передаёт дела?",
  "p.plans.pid.import.bring_tasks_in_from_jira":
    "Импорт задач из Jira, Asana или Microsoft Project. Сначала вы увидите, что произойдёт; ничего не записывается, пока вы не подтвердите.",
  "p.plans.pid.import.choose_another_file": "Выбрать другой файл",
  "p.plans.pid.import.file": "Файл",
  "p.plans.pid.import.import": "Импортировать",
  "p.plans.pid.import.import_ppm": "Импорт — PPM",
  "p.plans.pid.import.imported": "Импортировано",
  "p.plans.pid.import.plan": "← План",
  "p.plans.pid.import.preview": "Предпросмотр",
  "p.plans.pid.import.tracker": "Трекер",
  "p.plans.pid.import.where_from": "Откуда",
  "p.plans.pid.joint_risk.a_late_project_usually_costs":
    "Отстающий проект обычно стоит и дороже, поэтому здесь стоимость и расписание рассчитываются вместе.",
  "p.plans.pid.joint_risk.assumptions": "Допущения",
  "p.plans.pid.joint_risk.band_when_there_is_no":
    "Диапазон при отсутствии оценки (± %)",
  "p.plans.pid.joint_risk.both_at_once": "Оба сразу",
  "p.plans.pid.joint_risk.budget": "бюджет",
  "p.plans.pid.joint_risk.budget_2": "Бюджет",
  "p.plans.pid.joint_risk.budget_lines_in_are_not":
    "Статьи бюджета в {join} не входят в этот расчёт; валюты никогда не суммируются.",
  "p.plans.pid.joint_risk.business_days_late":
    "опоздание {days} рабочих дней, {money}",
  "p.plans.pid.joint_risk.business_days_late_2": "рабочих дней опоздания →",
  "p.plans.pid.joint_risk.cost": "Стоимость",
  "p.plans.pid.joint_risk.cost_2": "Стоимость — {title}: {costSkippedText}",
  "p.plans.pid.joint_risk.cost_and_schedule_risk_ppm":
    "Риски стоимости и расписания — PPM",
  "p.plans.pid.joint_risk.cost_and_schedule_together":
    "Стоимость и расписание вместе",
  "p.plans.pid.joint_risk.cost_lines": "Статьи стоимости",
  "p.plans.pid.joint_risk.cost_of_each_business_day":
    "Стоимость каждого рабочего дня опоздания (в основных единицах)",
  "p.plans.pid.joint_risk.cost_risk": "Стоимостной риск",
  "p.plans.pid.joint_risk.critical_path_finish_the_plan":
    "Окончание по критическому пути (собственная дата плана)",
  "p.plans.pid.joint_risk.currency_if_the_plan_has":
    "Валюта (если в плане их несколько)",
  "p.plans.pid.joint_risk.distribution": "Распределение",
  "p.plans.pid.joint_risk.each_dot_is_one_simulation":
    "Каждая точка — одна симуляция (прореженная выборка). Справа от линии «вовремя» — опоздание; выше линии бюджета — превышение бюджета. Полные значения приведены в таблице выше.",
  "p.plans.pid.joint_risk.estimate": "Оценка",
  "p.plans.pid.joint_risk.finish": "Окончание",
  "p.plans.pid.joint_risk.finish_date_at_each_percentile":
    "Дата окончания на каждом процентиле",
  "p.plans.pid.joint_risk.how_the_simulated_outcomes_fall":
    "Как смоделированные результаты распределяются относительно бюджета и окончания по критическому пути",
  "p.plans.pid.joint_risk.independence": "Независимость.",
  "p.plans.pid.joint_risk.iterations": "Итерации",
  "p.plans.pid.joint_risk.line": "Статья",
  "p.plans.pid.joint_risk.low_planned_high": "Низкая / плановая / высокая",
  "p.plans.pid.joint_risk.mean_delay_cost_included":
    "Среднее (с учётом стоимости задержки)",
  "p.plans.pid.joint_risk.no_register_risk_was_used":
    "В этом расчёте не использован ни один риск из реестра.",
  "p.plans.pid.joint_risk.not_enough_data": "недостаточно данных",
  "p.plans.pid.joint_risk.of_which_delay_cost_on":
    "Из них стоимость задержки в среднем",
  "p.plans.pid.joint_risk.on_time": "вовремя",
  "p.plans.pid.joint_risk.outcome": "Результат",
  "p.plans.pid.joint_risk.per_line_estimate_in":
    "Оценка по статьям в {currency}",
  "p.plans.pid.joint_risk.pert": "PERT",
  "p.plans.pid.joint_risk.plan": "← План",
  "p.plans.pid.joint_risk.risks": "Риски",
  "p.plans.pid.joint_risk.run": "Запустить",
  "p.plans.pid.joint_risk.running": "Выполнение…",
  "p.plans.pid.joint_risk.scatter_of_simulated_total_cost":
    "Диаграмма рассеяния смоделированной общей стоимости относительно рабочих дней опоздания с бюджетом и линией «вовремя»",
  "p.plans.pid.joint_risk.schedule":
    "Расписание — {title}: {skippedReasonText}",
  "p.plans.pid.joint_risk.schedule_risk": "Риск расписания",
  "p.plans.pid.joint_risk.seed": "Начальное число",
  "p.plans.pid.joint_risk.seed_the_same_seed_gives":
    "Начальное число {seed}: то же начальное число даёт тот же результат.",
  "p.plans.pid.joint_risk.share": "Доля",
  "p.plans.pid.joint_risk.simulations": "Симуляции",
  "p.plans.pid.joint_risk.total_cost_delay_cost_included":
    "Общая стоимость, включая стоимость задержки",
  "p.plans.pid.joint_risk.triangular": "Треугольное",
  "p.plans.pid.leveling.allow_critical_tasks_to_move":
    "Разрешить перемещать критические задачи (это может сдвинуть окончание)",
  "p.plans.pid.leveling.apply": "Применить",
  "p.plans.pid.leveling.apply_the_ticked_moves":
    "Применить отмеченные переносы",
  "p.plans.pid.leveling.balance_workloads_resource_leveling":
    "Выровнять загрузку (выравнивание ресурсов)",
  "p.plans.pid.leveling.could_not_be_fixed": "Не удалось исправить",
  "p.plans.pid.leveling.critical": "критическая",
  "p.plans.pid.leveling.each_start_date_the_proposal":
    "Каждая дата начала, которую изменит предложение",
  "p.plans.pid.leveling.finish": "Окончание",
  "p.plans.pid.leveling.float": "Резерв",
  "p.plans.pid.leveling.i_have_read_the_proposal":
    "Я прочитал(а) предложение и хочу изменить эти даты начала",
  "p.plans.pid.leveling.later_by": "Позже на",
  "p.plans.pid.leveling.load_per_task_of_a":
    "Загрузка на задачу (% рабочего дня сотрудника)",
  "p.plans.pid.leveling.loading": "Загрузка…",
  "p.plans.pid.leveling.no_start_date_would_change":
    "Ни одна дата начала не изменится.",
  "p.plans.pid.leveling.nobody_is_over_allocated_so":
    "Перегруженных нет, поэтому выравнивать нечего.",
  "p.plans.pid.leveling.over_allocated_now": "Перегружены сейчас",
  "p.plans.pid.leveling.person": "Сотрудник",
  "p.plans.pid.leveling.plan": "← План",
  "p.plans.pid.leveling.propose_again": "Предложить снова",
  "p.plans.pid.leveling.proposed_start_dates": "Предлагаемые даты начала",
  "p.plans.pid.leveling.refused": "Отказано: {note}",
  "p.plans.pid.leveling.resource_leveling_ppm": "Выравнивание ресурсов — PPM",
  "p.plans.pid.leveling.start": "Начало",
  "p.plans.pid.leveling.start_finish_business_days":
    "{pid}: начало {v}{start_days}, окончание {v2}{finish_days} рабочих дней",
  "p.plans.pid.leveling.suggests_moving_tasks_so_that":
    "Предлагает перенести задачи так, чтобы никто не был занят больше, чем может. Ничего не меняется, пока вы не подтвердите.",
  "p.plans.pid.leveling.task": "Задача",
  "p.plans.pid.leveling.the_search_stopped_at_its":
    "поиск остановился на своём пределе",
  "p.plans.pid.leveling.this_is_advice_nothing_changes":
    "Это рекомендация. Ничего не меняется, пока вы не выберете переносы ниже и не примените их.",
  "p.plans.pid.leveling.why_kept_with_each_audit":
    "Почему (сохраняется с каждой записью аудита)",
  "p.plans.pid.leveling.working_calendar": "Рабочий календарь",
  "p.plans.pid.more_tools": "Другие инструменты",
  "p.plans.pid.onboarding.access": "Доступ",
  "p.plans.pid.onboarding.access_record": "Запись о доступе",
  "p.plans.pid.onboarding.action": "Действие",
  "p.plans.pid.onboarding.cancel_checklist": "Отменить чек-лист",
  "p.plans.pid.onboarding.checklists": "Чек-листы",
  "p.plans.pid.onboarding.complete_checklist": "Завершить чек-лист",
  "p.plans.pid.onboarding.done": "Выполнено",
  "p.plans.pid.onboarding.effective":
    "{status}{v} · вступает в силу {effective_date} · {progressText}",
  "p.plans.pid.onboarding.from":
    "{status} · с {effective_date} · {progressText}",
  "p.plans.pid.onboarding.granted": "Предоставлен",
  "p.plans.pid.onboarding.hand_over_someone_s_work": "Передать чью-то работу",
  "p.plans.pid.onboarding.no_access_is_outstanding":
    "● Незакрытых доступов нет.",
  "p.plans.pid.onboarding.none_yet": "Пока нет.",
  "p.plans.pid.onboarding.onboarding_and_offboarding":
    "Приём и увольнение сотрудников",
  "p.plans.pid.onboarding.onboarding_and_offboarding_ppm":
    "Приём и увольнение сотрудников — PPM",
  "p.plans.pid.onboarding.person": "person:…",
  "p.plans.pid.onboarding.person_2": "Сотрудник",
  "p.plans.pid.onboarding.plan": "← План",
  "p.plans.pid.onboarding.ready_to_close": "Готово к закрытию?",
  "p.plans.pid.onboarding.reason": "Причина",
  "p.plans.pid.onboarding.reason_for_cancelling": "Причина отмены",
  "p.plans.pid.onboarding.record": "Записать",
  "p.plans.pid.onboarding.record_access": "Записать доступ",
  "p.plans.pid.onboarding.reopen": "Открыть снова",
  "p.plans.pid.onboarding.repo_write": "запись в репозиторий",
  "p.plans.pid.onboarding.revoked": "Отозван",
  "p.plans.pid.onboarding.save": "Сохранить",
  "p.plans.pid.onboarding.since": "▲ {person_ref}: {access} (с {granted_on})",
  "p.plans.pid.onboarding.skip": "Пропустить",
  "p.plans.pid.onboarding.start": "Начать",
  "p.plans.pid.onboarding.start_a_checklist": "Начать чек-лист",
  "p.plans.pid.onboarding.takes_effect": "Вступает в силу",
  "p.plans.pid.onboarding.what": "Что",
  "p.plans.pid.onboarding.who_holds_what_access_is":
    "Кто какой доступ имеет — вопрос о людях, поэтому нужно разрешение на управление людьми.",
  "p.plans.pid.onboarding.why_skip": "Зачем пропускать?",
  "p.plans.pid.raci.add_a_person_person_or":
    "Добавить человека (person:… или worker:…)",
  "p.plans.pid.raci.add_a_person_to_start":
    "Добавьте человека, чтобы начать назначать буквы.",
  "p.plans.pid.raci.add_column": "Добавить столбец",
  "p.plans.pid.raci.export_to_a_spreadsheet": "Экспорт в электронную таблицу",
  "p.plans.pid.raci.for_each_task_who_does":
    "Для каждой задачи: кто выполняет, кто принимает, кого спрашивают и кого информируют.",
  "p.plans.pid.raci.have_nobody_accountable":
    "{missing_accountable}: нет ответственного за результат.",
  "p.plans.pid.raci.have_nobody_responsible":
    "{missing_responsible}: нет исполнителя.",
  "p.plans.pid.raci.how_many_tasks_each_person":
    "Сколько задач у каждого человека по каждой букве",
  "p.plans.pid.raci.legend": "Обозначения",
  "p.plans.pid.raci.load_by_person": "Загрузка по людям",
  "p.plans.pid.raci.loading": "Загрузка…",
  "p.plans.pid.raci.of_tasks_are_complete_one":
    "Полностью назначены {complete} из {tasks} задач (один ответственный за результат, не менее одного исполнителя).",
  "p.plans.pid.raci.person": "Человек",
  "p.plans.pid.raci.plan": "← План",
  "p.plans.pid.raci.raci_ppm": "Матрица RACI — PPM",
  "p.plans.pid.raci.this_plan_has_no_tasks": "В этом плане пока нет задач.",
  "p.plans.pid.raci.who_does_what_raci_matrix": "Кто что делает (матрица RACI)",
  "p.plans.pid.schedule.dependency_violated_successor_starts":
    "Зависимость нарушена: последующая задача начинается {actual_start}, но не может начаться раньше {earliest_start}.",
  "p.plans.pid.schedule.schedule_ppm": "Расписание — PPM",
  "p.plans.pid.schedule_health.as_of": "По состоянию на {as_of} ·",
  "p.plans.pid.schedule_health.baseline_version":
    "версия базового плана {version}",
  "p.plans.pid.schedule_health.check": "Проверить",
  "p.plans.pid.schedule_health.checks_the_schedule_for_common":
    "Проверяет расписание на типичные проблемы, например задачи без связей или без оценки.",
  "p.plans.pid.schedule_health.critical_path": "Критический путь",
  "p.plans.pid.schedule_health.held_to": "Соответствует",
  "p.plans.pid.schedule_health.loading": "Загрузка…",
  "p.plans.pid.schedule_health.no_baseline_taken":
    "базовый план не зафиксирован",
  "p.plans.pid.schedule_health.plan": "← План",
  "p.plans.pid.schedule_health.re_run": "Запустить снова",
  "p.plans.pid.schedule_health.result": "Результат",
  "p.plans.pid.schedule_health.schedule_health": "Состояние расписания",
  "p.plans.pid.schedule_health.schedule_health_ppm":
    "Состояние расписания — PPM",
  "p.plans.pid.schedule_health.see_the_note": "См. примечание",
  "p.plans.pid.schedule_health.status_date": "Дата статуса",
  "p.plans.pid.schedule_health.the_fourteen_schedule_health_checks":
    "Четырнадцать проверок состояния расписания",
  "p.plans.pid.schedule_health.to_fix": "Что исправить",
  "p.plans.pid.schedule_health.what_was_found": "Что обнаружено",
  "p.plans.pid.schedule_risk.assumptions": "Допущения",
  "p.plans.pid.schedule_risk.band_for_tasks_with_no":
    "Диапазон для задач без оценки (± %)",
  "p.plans.pid.schedule_risk.critical_in": "Критична в",
  "p.plans.pid.schedule_risk.critical_path": "Критический путь",
  "p.plans.pid.schedule_risk.critical_path_finish_the_plan":
    "Завершение по критическому пути (собственная дата плана)",
  "p.plans.pid.schedule_risk.distribution": "Распределение",
  "p.plans.pid.schedule_risk.each_bar_is_a_finish":
    "Каждый столбец — дата завершения; чем он выше, тем больше симуляций закончилось в эту дату. Сплошные линии — P50, P80 и P90 (подписаны); пунктирная линия — собственная дата плана.",
  "p.plans.pid.schedule_risk.earliest_latest":
    "{join}. Самая ранняя дата — {earliest}, самая поздняя — {latest}.",
  "p.plans.pid.schedule_risk.estimate": "Оценка",
  "p.plans.pid.schedule_risk.finish": "Завершение",
  "p.plans.pid.schedule_risk.finish_date": "Дата завершения",
  "p.plans.pid.schedule_risk.finish_date_at_each_percentile":
    "Дата завершения для каждого процентиля",
  "p.plans.pid.schedule_risk.histogram_of_simulated_finish_dates":
    "Гистограмма смоделированных дат завершения с отметками P50, P80 и P90",
  "p.plans.pid.schedule_risk.independence": "Независимость.",
  "p.plans.pid.schedule_risk.iterations": "Итерации",
  "p.plans.pid.schedule_risk.left_out": "Исключены:",
  "p.plans.pid.schedule_risk.many_what_if_runs_of":
    "Множество прогонов расписания по принципу «что, если» показывают диапазон дат завершения и вероятность каждой из них.",
  "p.plans.pid.schedule_risk.no_register_risk_was_used":
    "Ни один риск из реестра не использован как событие расписания.",
  "p.plans.pid.schedule_risk.not_enough_data": "недостаточно данных",
  "p.plans.pid.schedule_risk.not_simulated_because_the_schedule":
    "Не смоделированы, так как расписание не может их разместить: {join}.",
  "p.plans.pid.schedule_risk.of": "{date}: {count} из {iterations}",
  "p.plans.pid.schedule_risk.of_runs": "{criticalityText} прогонов",
  "p.plans.pid.schedule_risk.optimistic_likely_pessimistic_busine":
    "Оптимистичная / вероятная / пессимистичная (рабочие дни)",
  "p.plans.pid.schedule_risk.outcome": "Исход",
  "p.plans.pid.schedule_risk.per_task_estimate_criticality_and":
    "Оценка, критичность и чувствительность по задачам",
  "p.plans.pid.schedule_risk.plan": "← План",
  "p.plans.pid.schedule_risk.plan_2": "план",
  "p.plans.pid.schedule_risk.risks_used": "Использованные риски",
  "p.plans.pid.schedule_risk.run": "Запустить",
  "p.plans.pid.schedule_risk.run_on_against_the_plan":
    "Запуск {ranOn} для плана в его текущем виде · начальное значение {seed} · итераций: {iterations} · {distributionText} · календарь {calendar}",
  "p.plans.pid.schedule_risk.running": "Выполняется…",
  "p.plans.pid.schedule_risk.schedule_risk": "Риск расписания",
  "p.plans.pid.schedule_risk.schedule_risk_ppm": "Риск расписания — PPM",
  "p.plans.pid.schedule_risk.seed": "Начальное значение",
  "p.plans.pid.schedule_risk.seed_the_same_seed_gives":
    "Начальное значение {seed}: одно и то же начальное значение даёт один и тот же результат. Не более {max_iterations} итераций.",
  "p.plans.pid.schedule_risk.sensitivity": "Чувствительность",
  "p.plans.pid.schedule_risk.share": "Доля",
  "p.plans.pid.schedule_risk.simulations": "Симуляции",
  "p.plans.pid.schedule_risk.task": "Задача",
  "p.plans.pid.schedule_risk.the_same_figures_as_a":
    "Те же значения в виде таблицы",
  "p.plans.pid.schedule_risk.triangular": "Треугольное",
  "p.plans.pid.schedule_risk.weighted_towards_the_likely_value":
    "С весом в сторону вероятного значения (PERT)",
  "p.plans.pid.schedule_risk.what_drives_it": "Что на это влияет",
  "p.plans.pid.schedule_risk.when_it_finishes": "Когда завершится",
  "p.plans.pid.wbs.code": "Код",
  "p.plans.pid.wbs.level": "Уровень",
  "p.plans.pid.wbs.no_tasks_yet": "Задач пока нет.",
  "p.plans.pid.wbs.plan": "← План",
  "p.plans.pid.wbs.sub_tasks": "Подзадачи",
  "p.plans.pid.wbs.task": "Задача",
  "p.plans.pid.wbs.tasks_is_too_many_to":
    "Задач: {length} — слишком много для удобной вложенности; таблица показывает их все.",
  "p.plans.pid.wbs.the_plan_s_tasks_each":
    "Задачи плана, каждая разбита на подзадачи. Код в каждой строке нужен для чтения: он меняется при добавлении или перемещении работ.",
  "p.plans.pid.wbs.work_breakdown": "Иерархическая структура работ",
  "p.plans.pid.wbs.work_breakdown_as_a_table":
    "Иерархическая структура работ в виде таблицы",
  "p.plans.pid.wbs.work_breakdown_ppm": "Иерархическая структура работ — PPM",
  "p.plans.pid.wbs.work_breakdown_tree": "Дерево иерархической структуры работ",
  "p.plans.pid.what_if.after": "После",
  "p.plans.pid.what_if.before": "До",
  "p.plans.pid.what_if.business_days": "Рабочие дни",
  "p.plans.pid.what_if.choose": "Выберите…",
  "p.plans.pid.what_if.commit_these_changes": "Зафиксировать эти изменения",
  "p.plans.pid.what_if.constraint": "Ограничение",
  "p.plans.pid.what_if.critical": "■ критическая",
  "p.plans.pid.what_if.critical_path": "Критический путь",
  "p.plans.pid.what_if.date": "Дата",
  "p.plans.pid.what_if.discard": "Отбросить",
  "p.plans.pid.what_if.edit": "Изменить",
  "p.plans.pid.what_if.finish_to_finish": "Окончание — окончание",
  "p.plans.pid.what_if.finish_to_start": "Окончание — начало",
  "p.plans.pid.what_if.float": "Резерв времени",
  "p.plans.pid.what_if.follows": "{name} следует за {name2} ({toUpperCase})",
  "p.plans.pid.what_if.joins_the_critical_path":
    "Попадают на критический путь: {join}.",
  "p.plans.pid.what_if.lag_business_days": "Запаздывание (рабочие дни)",
  "p.plans.pid.what_if.leaves_it": "Выходят с него: {join}.",
  "p.plans.pid.what_if.link": "Связь",
  "p.plans.pid.what_if.new_start": "Новое начало",
  "p.plans.pid.what_if.no_task_s_dates_or":
    "Даты и резерв времени ни одной задачи не меняются.",
  "p.plans.pid.what_if.none_clear_it": "Нет (очистить)",
  "p.plans.pid.what_if.none_yet": "Пока нет.",
  "p.plans.pid.what_if.now_takes_of_the_day":
    "«{name}» теперь занимает {units_percent}% дня на {hoursText} работы (рассчитано по её правилу планирования).",
  "p.plans.pid.what_if.of_work_has_no_rate":
    "▲ Для {hoursText} работы не задана ставка, поэтому стоимость не рассчитана.",
  "p.plans.pid.what_if.plan": "← План",
  "p.plans.pid.what_if.planned_labour_cost":
    "Плановая стоимость труда: {deltaText} ({deltaText2} → {deltaText3}).",
  "p.plans.pid.what_if.predecessor": "Предшественник",
  "p.plans.pid.what_if.preview": "Предпросмотр",
  "p.plans.pid.what_if.reason": "Причина",
  "p.plans.pid.what_if.reason_recorded_with_each_change":
    "Причина (записывается с каждым изменением)",
  "p.plans.pid.what_if.remove": "Удалить",
  "p.plans.pid.what_if.stage_an_edit": "Подготовить изменение",
  "p.plans.pid.what_if.stage_edit": "Подготовить изменение",
  "p.plans.pid.what_if.staged_edits": "Подготовленные изменения",
  "p.plans.pid.what_if.start_to_finish": "Начало — окончание",
  "p.plans.pid.what_if.start_to_start": "Начало — начало",
  "p.plans.pid.what_if.successor": "Последователь",
  "p.plans.pid.what_if.task": "Задача",
  "p.plans.pid.what_if.tasks_whose_dates_float_or":
    "Задачи, у которых меняются даты, резерв времени или статус критичности",
  "p.plans.pid.what_if.try_changes_on_a_copy":
    "Пробуйте изменения на копии. Ничего не сохраняется, пока вы не зафиксируете изменения.",
  "p.plans.pid.what_if.type": "Тип",
  "p.plans.pid.what_if.what_if": "Что, если",
  "p.plans.pid.what_if.what_if_ppm": "Что, если — PPM",
  "p.plans.pid.work.business_days_of_a_day":
    "{duration_days} рабочих дней · {units_percent}% дня · {hoursText} работы · {v}",
  "p.plans.pid.work.change": "Изменить",
  "p.plans.pid.work.costs": "Затраты",
  "p.plans.pid.work.day": "День",
  "p.plans.pid.work.duration_business_days": "Длительность (рабочие дни)",
  "p.plans.pid.work.edit_assignment": "Изменить назначение",
  "p.plans.pid.work.falls_outside_this_window":
    "{hoursText} выходит за пределы этого окна.",
  "p.plans.pid.work.hours_per_each_row_adds":
    "Часы по периодам ({grain}); каждая строка в сумме даёт объём работы задачи, последняя строка — объём работы плана.",
  "p.plans.pid.work.how_long_each_task_takes":
    "Сколько длится каждая задача, какую часть дня человека она занимает и какой объём работы получается в сумме.",
  "p.plans.pid.work.month": "Месяц",
  "p.plans.pid.work.no_assignment_yet_give_a":
    "Назначения пока нет. Укажите длительность и единицы, и объём работы будет рассчитан.",
  "p.plans.pid.work.no_task_carries_work_yet":
    "Ни у одной задачи пока нет объёма работы, поэтому распределять нечего.",
  "p.plans.pid.work.not_placed": "(не размещено)",
  "p.plans.pid.work.percentages": "Проценты",
  "p.plans.pid.work.period": "Период",
  "p.plans.pid.work.plan": "← План",
  "p.plans.pid.work.plan_2": "План",
  "p.plans.pid.work.rule": "Правило",
  "p.plans.pid.work.save_assignment": "Сохранить назначение",
  "p.plans.pid.work.save_contour": "Сохранить профиль",
  "p.plans.pid.work.so_its_work_is_not":
    "▲ {title} ({hoursText}) {reason}, поэтому её работа не входит в периоды.",
  "p.plans.pid.work.spread": "Распределение",
  "p.plans.pid.work.task": "Задача",
  "p.plans.pid.work.tasks": "Задачи",
  "p.plans.pid.work.this_plan_has_no_scheduled":
    "В этом плане пока нет запланированных задач.",
  "p.plans.pid.work.total": "Итого",
  "p.plans.pid.work.units": "Единицы (%)",
  "p.plans.pid.work.units_of_the_day": "Единицы (% дня)",
  "p.plans.pid.work.week": "Неделя",
  "p.plans.pid.work.work_and_assignments": "Работа и назначения",
  "p.plans.pid.work.work_and_assignments_ppm": "Работа и назначения — PPM",
  "p.plans.pid.work.work_hours": "Работа (часы)",
  "p.plans.pid.work.work_over_time": "Работа по времени",
  "p.prioritization.all": "все",
  "p.prioritization.apply": "Применить",
  "p.prioritization.band": "диапазон",
  "p.prioritization.band_2": "Диапазон",
  "p.prioritization.component": "Компонент",
  "p.prioritization.coverage": "охват",
  "p.prioritization.coverage_2": "Охват",
  "p.prioritization.explain": "Пояснить",
  "p.prioritization.hide": "Скрыть",
  "p.prioritization.high": "высокий",
  "p.prioritization.loading_the_breakdown": "Загрузка разбивки…",
  "p.prioritization.low": "низкий",
  "p.prioritization.medium": "средний",
  "p.prioritization.missing_evidence": "Отсутствующие данные",
  "p.prioritization.no_evidence_for_any_component":
    "Нет данных ни по одному компоненту — этот план не оценён.",
  "p.prioritization.no_plans_match_that_filter":
    "Нет планов, подходящих под этот фильтр.",
  "p.prioritization.none": "нет",
  "p.prioritization.of_plans_one": "{length} из {scored_of} плана",
  "p.prioritization.of_plans_other": "{length} из {scored_of} планов",
  "p.prioritization.plan": "План",
  "p.prioritization.points": "Баллы",
  "p.prioritization.prioritization": "Приоритизация",
  "p.prioritization.prioritization_ppm": "Приоритизация — PPM",
  "p.prioritization.score": "Оценка",
  "p.prioritization.shows_how_much_of_the":
    "показывает, какая часть оценки действительно подтверждена данными.",
  "p.prioritization.smart_score_over_roi_strategic":
    "Умная оценка по ROI, стратегическому соответствию, экспертной оценке, риску, спросу, приоритету MoSCoW и динамике. Компоненты без данных отбрасываются, а остальные нормируются заново —",
  "p.prioritization.unscored": "не оценён",
  "p.prioritization.value": "Ценность",
  "p.prioritization.weight": "Вес",
  "p.proposals.actions": "Действия",
  "p.proposals.currency": "Валюта",
  "p.proposals.live": "действующая:",
  "p.proposals.promote": "перевести",
  "p.proposals.proposal": "предложение:",
  "p.proposals.proposals_ppm": "Предложения — PPM",
  "p.proposals.requested": "Запрошено",
  "p.proposals.status": "Статус",
  "p.proposals.status_filter": "Фильтр по статусу",
  "p.proposals.target": "Цель",
  "p.proposals.target_collection": "Целевая коллекция",
  "p.proposals.title": "Название",
  "p.regulator.actual": "Фактически",
  "p.regulator.benefits": "Выгоды:",
  "p.regulator.currency": "Валюта",
  "p.regulator.gate_decisions": "· Решения шлюзовых проверок:",
  "p.regulator.members": "Участники:",
  "p.regulator.names_withheld_abac_mask_obligation":
    "Имена скрыты (обязательство маскирования ABAC).",
  "p.regulator.no_budget_lines": "Статей бюджета нет.",
  "p.regulator.no_portfolios_recorded": "Портфели не записаны.",
  "p.regulator.none": "нет",
  "p.regulator.planned": "Запланировано",
  "p.regulator.ppm": "{t} — PPM",
  "p.regulator.stage": "этап: {v}",
  "p.regulator.target_realized": "цель {money} / достигнуто {money2}",
  "p.reports.collection": "Коллекция",
  "p.reports.csv": "CSV",
  "p.reports.fields_comma_separated": "поля (через запятую)",
  "p.reports.name_contains": "название содержит…",
  "p.reports.reports_ppm": "Отчёты — PPM",
  "p.resources.a_multi_skill_booking_counts":
    "Бронирование с несколькими навыками полностью учитывается для каждого из навыков, поэтому строки навыков пересекаются и не дают в сумме общий итог.",
  "p.resources.by": "По",
  "p.resources.demand_fte": "Спрос, FTE",
  "p.resources.demand_supply_and_gap_by":
    "Спрос, предложение и разрыв по периодам и параметру {group_by}",
  "p.resources.every_figure_as_a_table": "Все значения в виде таблицы",
  "p.resources.from": "С",
  "p.resources.gap_fte": "Разрыв, FTE",
  "p.resources.gap_per_in_fte_supply":
    "Разрыв за период ({period}) в FTE (предложение минус спрос)",
  "p.resources.loading": "Загрузка…",
  "p.resources.month": "Месяц",
  "p.resources.no_demand_or_supply_in":
    "В этом окне нет ни спроса, ни предложения.",
  "p.resources.not_known": "неизвестно",
  "p.resources.of_which_unassigned": "Из них не назначено",
  "p.resources.people": "Люди",
  "p.resources.per": "За",
  "p.resources.per_by_scope":
    "{periodLabel} · за период ({period}) · по параметру {group_by} · область: {kind}",
  "p.resources.period": "Период",
  "p.resources.proposed_fte": "Предложено, FTE",
  "l.processes.name_problem":
    "Дайте процессу название длиной от 1 до 120 символов.",
  "l.processes.steps_problem": "В процессе от 1 до {max} шагов.",
  "l.processes.title_problem":
    "Шаг {n}: дайте ему название длиной от 1 до 200 символов.",
  "l.processes.person_problem":
    "Шаг {n}: человек должен быть указан ссылкой person: или worker:.",
  "l.processes.kind_approve": "Утвердить",
  "l.processes.kind_todo": "Выполнить",
  "l.processes.kind_notify": "Уведомить",
  "l.processes.state_pending": "не достигнут",
  "l.processes.state_waiting": "ожидает",
  "l.processes.state_completed": "выполнен",
  "l.processes.state_skipped": "пропущен",
  "l.processes.state_rejected": "отклонён",
  "l.processes.state_failed": "сбой",
  "l.processes.run_running": "выполняется",
  "l.processes.run_completed": "завершён",
  "l.processes.run_rejected": "отклонён",
  "l.processes.run_cancelled": "отменён",
  "l.processes.run_failed": "сбой",
  "p.processes.processes": "Процессы утверждения",
  "p.processes.processes_ppm": "Процессы утверждения — PPM",
  "p.processes.intro":
    "Процесс — это список шагов, которые выполняются один за другим: попросить кого-то утвердить, поручить кому-то задачу или уведомить кого-то. Отклонение завершает запуск и пропускает следующие за ним шаги. Изменение процесса никогда не затрагивает уже выполняющийся запуск.",
  "p.processes.definitions": "Процессы",
  "p.processes.none_defined": "Процессы пока не определены.",
  "p.processes.version": "версия {v}",
  "p.processes.new_process": "Определить процесс",
  "p.processes.name": "Название",
  "p.processes.step_n": "Шаг {n}",
  "p.processes.kind": "Что он делает",
  "p.processes.person": "Человек (person:… или worker:…)",
  "p.processes.step_title": "Текст шага",
  "p.processes.add_step": "Добавить шаг",
  "p.processes.remove_step": "Удалить шаг {n}",
  "p.processes.create": "Определить процесс",
  "p.processes.defined": "Определён процесс {name}.",
  "p.processes.start_heading": "Начать запуск процесса {name}",
  "p.processes.subject_kind": "К чему он относится",
  "p.processes.subject_pid": "Его идентификатор",
  "p.processes.start": "Начать запуск",
  "p.processes.started": "Запуск процесса {name} начат.",
  "p.processes.runs": "Ваши запуски",
  "p.processes.none_runs": "Вы не начали ни одного запуска.",
  "p.processes.run_of": "{name} для объекта {kind} {pid}",
  "p.processes.status": "Статус",
  "p.processes.show_run": "Показать: {name}",
  "p.processes.hide_run": "Скрыть: {name}",
  "p.processes.steps": "Шаги",
  "p.processes.log": "Что произошло",
  "p.processes.complete_step": "Отметить шаг {n} выполненным",
  "p.processes.cancel": "Отменить запуск",
  "p.processes.resume": "Проверить утверждение снова",
  "p.processes.done": "Готово.",
  "p.processes.could_not": "Не получилось: {reason}",
  "p.processes.could_not_load": "Не удалось загрузить процессы",
  "l.tolerance.green": "в пределах допуска",
  "l.tolerance.amber": "близко к пределу",
  "l.tolerance.red": "допуск превышен",
  "l.tolerance.unknown": "измерить невозможно",
  "l.tolerance.not_set": "предел не задан",
  "l.tolerance.time": "Время",
  "l.tolerance.schedule": "Расписание относительно базового плана",
  "l.tolerance.cost": "Стоимость",
  "l.tolerance.risk": "Риск",
  "l.tolerance.resources": "Ресурсы",
  "l.tolerance.day_one": "{n} день",
  "l.tolerance.day_other": "{n} дней",
  "l.tolerance.score": "оценка {n}",
  "l.tolerance.points": "{n} п. п. сверх 100%",
  "l.tolerance.field_time": "Предел (календарных дней опоздания)",
  "l.tolerance.field_schedule":
    "Предел (календарных дней позже базового плана)",
  "l.tolerance.field_cost": "Предел (процент сверх утверждённого бюджета)",
  "l.tolerance.field_risk":
    "Предел (наибольшая допустимая оценка риска, от 1 до 25)",
  "l.tolerance.field_resources": "Предел (процентных пунктов перегрузки)",
  "l.tolerance.no_limit": "Здесь и выше предел не задан.",
  "l.tolerance.could_not_measure": "Измерить не удалось.",
  "l.tolerance.zero_tolerance":
    "Нулевой допуск: любое отклонение превышает предел.",
  "l.tolerance.used": "Использовано {percent}% предела",
  "l.tolerance.own": "Задан в этом плане",
  "l.tolerance.inherited": "Унаследован от плана выше",
  "p.issue_settings.title": "Типы, статусы и роли проблем",
  "p.issue_settings.title_ppm": "Типы, статусы и роли проблем — PPM",
  "p.issue_settings.intro":
    "Собственный словарь проблем организации. Изменить его может только администратор. Если ничего не настроено, проблемы используют встроенные типы и статусы.",
  "p.issue_settings.types_heading": "Типы проблем",
  "p.issue_settings.types_intro":
    "Выведенный из использования тип больше нельзя выбрать. Проблемы, у которых он есть, сохраняют его.",
  "p.issue_settings.key": "Ключ",
  "p.issue_settings.label": "Название",
  "p.issue_settings.state": "Состояние",
  "p.issue_settings.in_use": "Используется",
  "p.issue_settings.retired": "Выведен из использования",
  "p.issue_settings.retire": "Вывести из использования",
  "p.issue_settings.restore": "Восстановить",
  "p.issue_settings.actions": "Действия",
  "p.issue_settings.add_type": "Добавить тип",
  "p.issue_settings.roles_heading": "Роли",
  "p.issue_settings.roles_intro":
    "Роли, которые признаёт эта организация, и что разрешает каждая из них в плане с участниками по ролям. Роль может только сузить то, что уже разрешает вход человека. Рабочий процесс может назвать только роль из списка, чтобы переход нельзя было ограничить ролью с опечаткой.",
  "p.issue_settings.description": "Описание",
  "p.issue_settings.add_role": "Добавить роль",
  "p.issue_settings.workflows_heading": "Статусы и переходы",
  "p.issue_settings.workflows_intro":
    "Рабочий процесс задаёт статусы типа проблемы, кто может выполнять каждый переход и какие поля каждый статус требует или блокирует. Рабочий процесс для одного типа имеет приоритет над процессом для всех типов. Проблемы в удалённом статусе сохраняют его.",
  "p.issue_settings.none_workflows":
    "Ни один рабочий процесс для проблем не зарегистрирован, поэтому проблемы используют встроенные статусы.",
  "p.issue_settings.wf_name": "Название рабочего процесса",
  "p.issue_settings.wf_type": "Применяется к",
  "p.issue_settings.wf_all_types":
    "Все типы без собственного рабочего процесса",
  "p.issue_settings.wf_plan": "Один план",
  "p.issue_settings.statuses": "Статусы",
  "p.issue_settings.category": "Значение",
  "p.issue_settings.initial": "Начальный",
  "p.issue_settings.terminal": "Требует решения",
  "p.issue_settings.add_status": "Добавить статус",
  "p.issue_settings.moves": "Разрешённые переходы",
  "p.issue_settings.moves_intro":
    "Если переходы не перечислены, разрешён любой переход. Как только один перечислен, разрешены только перечисленные.",
  "p.issue_settings.from": "Из",
  "p.issue_settings.to": "В",
  "p.issue_settings.move_roles": "Роли (через запятую; пусто — для всех)",
  "p.issue_settings.add_move": "Добавить переход",
  "p.issue_settings.remove": "Удалить",
  "p.issue_settings.register": "Зарегистрировать рабочий процесс",
  "p.issue_settings.withdraw": "Отозвать",
  "p.issue_settings.saved": "Сохранено.",
  "p.issue_settings.could_not_load": "Не удалось загрузить настройки проблем.",
  "p.issue_settings.could_not_save":
    "Не удалось сохранить. Изменять эти настройки может только администратор, а каждый ключ должен быть новым и правильно сформированным.",
  "p.issue_settings.cat_todo": "Не начата",
  "p.issue_settings.cat_active": "В работе",
  "p.issue_settings.cat_waiting": "Ожидает",
  "p.issue_settings.cat_done": "Выполнена",
  "p.issue_settings.translations":
    "Переводы (язык=название, через точку с запятой)",
  "p.issue_settings.bad_translations":
    "Запишите каждый перевод в виде язык=название, например cy=Risg.",
  "p.issue_settings.save": "Сохранить",
  "p.issue_settings.permissions": "Разрешения",
  "p.issue_settings.perm_issue_add": "Регистрировать проблемы",
  "p.issue_settings.perm_issue_edit": "Изменять проблемы",
  "p.issue_settings.perm_issue_move": "Переводить проблемы между статусами",
  "p.issue_settings.perm_issue_delete": "Удалять проблемы",
  "p.issue_settings.edit": "Изменить",
  "p.issue_settings.save_workflow": "Сохранить рабочий процесс",
  "p.issue_settings.cancel": "Отмена",
  "p.issue_settings.editing": "Изменение: {name}",
  "p.issue_settings.rule_field": "Поле",
  "p.issue_settings.rule_kind": "Правило",
  "p.issue_settings.rule_required": "Обязательно",
  "p.issue_settings.rule_read_only": "Только чтение",
  "p.issue_settings.rule_roles": "Роли (через запятую; пусто — для всех)",
  "p.issue_settings.add_rule": "Добавить правило поля",
  "p.issue_settings.field_title": "Название",
  "p.issue_settings.field_description": "Описание",
  "p.issue_settings.field_severity": "Серьёзность",
  "p.issue_settings.field_priority": "Приоритет",
  "p.issue_settings.field_owner_ref": "Владелец",
  "p.issue_settings.field_due_date": "Срок",
  "p.issue_settings.field_resolution": "Решение",
  "p.plan_settings.title": "Настройки проблем для этого плана",
  "p.plan_settings.title_ppm": "Настройки проблем для этого плана — PPM",
  "p.plan_settings.intro":
    "Кто какую роль здесь имеет, какие типы проблем предлагает этот план и его собственные рабочие процессы. Эти изменения вносит администратор.",
  "p.plan_settings.back": "К проблемам этого плана",
  "p.plan_settings.all_settings": "Настройки для всей организации",
  "p.plan_settings.members_heading": "Люди и роли",
  "p.plan_settings.members_intro":
    "Если в плане указан хотя бы один человек, для регистрации, изменения, перевода между статусами и удаления проблем в нём нужна роль, которая это разрешает. Роль не может дать человеку больше, чем разрешает его вход, и не открывает закрытый план.",
  "p.plan_settings.enforced":
    "Для действий с проблемами в этом плане нужна роль.",
  "p.plan_settings.not_enforced":
    "Любой, кто может записывать, может действовать с проблемами здесь.",
  "p.plan_settings.person": "Идентификатор пользователя",
  "p.plan_settings.role": "Роль",
  "p.plan_settings.choose_role": "Выберите роль",
  "p.plan_settings.add_member": "Добавить человека",
  "p.plan_settings.types_heading": "Типы проблем в этом плане",
  "p.plan_settings.types_intro":
    "Отметьте типы, которые предлагает этот план. Если ничего не отмечено, предлагаются все типы. Проблема сохраняет тип, который вы исключили.",
  "p.plan_settings.save_types": "Сохранить типы",
  "p.plan_settings.workflows_intro":
    "Рабочий процесс здесь применяется только к этому плану и имеет приоритет над процессом организации. Рабочий процесс для одного типа имеет приоритет над процессом для всех типов. Проблемы в удалённом статусе сохраняют его.",
  "p.plan_issues.settings_link": "Настройки проблем для этого плана",
  "p.plan_issues.problem_fields_required": "Для этого статуса нужно: {fields}.",
  "p.tolerances.tolerances": "Допуски",
  "p.tolerances.tolerances_ppm": "Допуски — PPM",
  "p.tolerances.plan_link": "Допуски",
  "p.tolerances.intro":
    "Насколько каждый проект может отклониться по времени, стоимости, риску и ресурсам, прежде чем вопрос нужно передать выше, и какая часть каждого предела использована. В пределах, но близко к нему — жёлтый; за пределом — красный; предел, который невозможно измерить, показывается как неизмеримый, но никогда не как зелёный.",
  "p.tolerances.board_intro":
    "Все доступные вам для чтения планы, у которых есть допуск, сначала худшие.",
  "p.tolerances.none": "Ни у одного плана пока нет допуска.",
  "p.tolerances.none_match": "Нет подходящих планов.",
  "p.tolerances.show": "Показать",
  "p.tolerances.all_statuses": "Все цвета",
  "p.tolerances.plan": "План",
  "p.tolerances.overall": "В целом",
  "p.tolerances.truncated":
    "Измерены только первые {n} планов, поэтому список может быть неполным.",
  "p.tolerances.loading": "Загрузка…",
  "p.tolerances.could_not_load": "Не удалось загрузить допуски",
  "p.tolerances.back": "← План",
  "p.tolerances.board_link": "Все допуски",
  "p.tolerances.limit": "Предел",
  "p.tolerances.measured": "Измерено",
  "p.tolerances.how": "Как это измеряется",
  "p.tolerances.amber_from": "Жёлтый начиная с (процент предела)",
  "p.tolerances.note": "Примечание",
  "p.tolerances.save": "Задать предел",
  "p.tolerances.remove": "Удалить собственный предел этого плана",
  "p.tolerances.saved": "Предел «{dimension}» сохранён.",
  "p.tolerances.removed": "Предел «{dimension}» удалён.",
  "p.tolerances.could_not": "Не получилось: {reason}",
  "p.tolerances.limit_problem": "Укажите предел числом.",
  "l.escalation.open": "открыта",
  "l.escalation.acknowledged": "принята к сведению",
  "l.escalation.resolved": "решена",
  "l.escalation.withdrawn": "отозвана",
  "l.escalation.reason_tolerance": "Превышен допуск",
  "l.escalation.reason_risk": "Риск",
  "l.escalation.reason_issue": "Проблема",
  "l.escalation.reason_dependency": "Зависимость",
  "l.escalation.reason_decision": "Нужно решение",
  "l.escalation.reason_resource": "Ресурсы",
  "l.escalation.reason_other": "Другое",
  "l.escalation.outcome_decided": "Решение принято",
  "l.escalation.outcome_guidance_given": "Даны указания",
  "l.escalation.outcome_rejected": "Отклонено",
  "l.escalation.event_raised": "Поднята",
  "l.escalation.event_acknowledged": "Принята к сведению",
  "l.escalation.event_level_raised": "Передана на уровень выше",
  "l.escalation.event_resolved": "Решена",
  "l.escalation.event_withdrawn": "Отозвана",
  "l.escalation.title_problem":
    "Дайте ей название длиной от 1 до 200 символов.",
  "l.escalation.dimension_problem": "Укажите, какого допуска она касается.",
  "l.escalation.level_problem": "Уровень — целое число от 1 до 9.",
  "l.escalation.owner_problem":
    "Человек должен быть указан ссылкой person: или worker:.",
  "l.escalation.level_to": "Уровень {level}: {to}",
  "l.escalation.level": "Уровень {level}",
  "l.escalation.open_for": "Открыта {n} дн.",
  "l.escalation.took": "Заняла {n} дн.",
  "l.escalation.overdue": "просрочена",
  "l.escalation.automatic": "Поднята автоматически",
  "p.escalations.escalations": "Эскалации",
  "p.escalations.escalations_ppm": "Эскалации — PPM",
  "p.escalations.plan_link": "Эскалации",
  "p.task.task": "Задача",
  "p.task.task_ppm": "Задача — PPM",
  "p.task.back": "К иерархической структуре работ",
  "p.task.could_not_load": "Не удалось загрузить задачу.",
  "p.task.loading": "Загрузка…",
  "p.task.status": "Статус",
  "p.task.due": "Срок",
  "p.task.assignee": "Исполнитель",
  "p.task.description": "Описание",
  "p.task.s_todo": "К выполнению",
  "p.task.s_in_progress": "В работе",
  "p.task.s_in_review": "На проверке",
  "p.task.s_done": "Выполнена",
  "p.task.s_blocked": "Заблокирована",
  "p.task.links_intro":
    "Где эта задача находится в других инструментах, например задача Jira или Linear, под которой она отслеживается. Адрес сохраняется и показывается; он никогда не загружается, и ничего не синхронизируется.",
  "p.task.back_board": "К доске",
  "p.task.could_not_load_links":
    "Не удалось загрузить связи задачи с другими задачами.",
  "p.task.could_not_load_tasks":
    "Не удалось загрузить задачи плана для связывания.",
  "p.task.edit_heading": "Изменить задачу",
  "p.task.edit_note":
    "Статус меняется на доске. Спринт, компонент и родительская задача остаются как есть.",
  "p.task.f_title": "Название",
  "p.task.f_assignee":
    "Исполнитель (ссылка person: или worker:; пусто — никто)",
  "p.task.f_points": "Story points (целое число от 0 до 100; пусто — нет)",
  "p.task.f_due": "Срок (пусто — нет)",
  "p.task.save": "Сохранить задачу",
  "p.task.saved": "Задача сохранена.",
  "p.task.problem_title_required": "Укажите название задачи.",
  "p.task.problem_title_too_long": "Название слишком длинное.",
  "p.task.problem_assignee":
    "Укажите исполнителя ссылкой person: или worker: либо оставьте поле пустым.",
  "p.task.problem_points_whole": "Story points — целое число.",
  "p.task.problem_points_range": "Story points должны быть от 0 до 100.",
  "p.task.problem_due": "Срок не является датой.",
  "p.task.relations_heading": "Связи с другими задачами",
  "p.task.relations_intro":
    "«Блокирует» не даёт завершить другую задачу, пока эта открыта. «Предшествует» требует, чтобы срок другой задачи был позже, чем у этой. Когда задача завершена, её дубликат тоже завершается. «Связана с» и «Скопирована из» носят справочный характер.",
  "p.task.no_relations": "У этой задачи нет связей с другими задачами.",
  "p.task.rel_related": "Связана с",
  "p.task.rel_blocks": "Блокирует",
  "p.task.rel_blocked_by": "Заблокирована задачей",
  "p.task.rel_precedes": "Предшествует",
  "p.task.rel_follows": "Следует за",
  "p.task.rel_duplicates": "Дубликат",
  "p.task.rel_duplicated_by": "Дублируется задачей",
  "p.task.rel_copied_from": "Скопирована из",
  "p.task.rel_copied_to": "Скопирована в",
  "p.task.other_state": "(сейчас: {status})",
  "p.task.other_state_due": "(сейчас: {status}, срок {date})",
  "p.task.remove_link": "Удалить",
  "p.task.remove_link_named": "Удалить связь: {link} {title}",
  "p.task.hidden_links_one":
    "Ещё {n} связь ведёт к задаче в плане, который вы не видите.",
  "p.task.hidden_links_other":
    "Ещё связей, ведущих к задачам в планах, которые вы не видите: {n}.",
  "p.task.add_link_heading": "Связать с другой задачей",
  "p.task.f_kind": "Эта задача",
  "p.task.f_other": "Другая задача",
  "p.task.choose_task": "Выберите задачу",
  "p.task.add_link": "Связать задачи",
  "p.task.link_added": "Связь добавлена.",
  "p.task.link_removed": "Связь удалена.",
  "p.task.problem_choose_task": "Выберите другую задачу.",
  "p.task.problem_self": "Задачу нельзя связать с ней самой.",
  "p.task.problem_exists": "У этой задачи уже есть такая связь.",
  "p.task.problem_cycle":
    "У другой задачи уже есть такая связь с этой; две задачи ждали бы друг друга.",
  "p.task.problem_precedes_dates":
    "«Предшествует» требует, чтобы срок другой задачи был позже, чем у этой: сначала измените срок.",
  "p.task.refused_forbidden": "Вам нельзя изменять задачи этого плана.",
  "p.task.refused_gone":
    "Этой задачи больше нет, или вы её не видите; данные прочитаны заново.",
  "p.task.refused_link_gone":
    "Эта связь уже была удалена; связи прочитаны заново.",
  "p.task.refused_link_exists":
    "Такая связь уже существует; связи прочитаны заново.",
  "p.task.refused_edit_invalid":
    "Сервис не принял изменение: проверьте поля. Новый срок также должен сохранять порядок всех связей «Предшествует».",
  "p.task.refused_link_invalid":
    "Сервис не принял связь: возможно, другой задачи больше нет, связи «Блокирует» или «Предшествует» не могут образовывать цикл, а «Предшествует» требует, чтобы срок другой задачи был позже.",
  "p.me.notifications_link": "Мои уведомления",
  "p.me.notifications_note":
    "— ваши входящие, то, о чём вас уведомляют, и куда направляются копии.",
  "p.notifications.title_ppm": "Мои уведомления — PPM",
  "p.notifications.heading": "Мои уведомления",
  "p.notifications.back": "← Моя работа",
  "p.notifications.intro":
    "Всё здесь принадлежит только вам: никто другой не может прочитать ваши входящие, ваш выбор или ваши адресаты.",
  "p.notifications.loading": "Загрузка…",
  "p.notifications.inbox_heading": "Входящие",
  "p.notifications.unread_only": "Показывать только непрочитанные",
  "p.notifications.unread_count_one": "{n} непрочитанное уведомление.",
  "p.notifications.unread_count_other": "Непрочитанных уведомлений: {n}.",
  "p.notifications.inbox_tz": "Дни считаются в вашем часовом поясе: {tz}.",
  "p.notifications.inbox_empty": "Во входящих ничего нет.",
  "p.notifications.inbox_unread_empty": "Непрочитанных нет.",
  "p.notifications.status_unread": "Не прочитано",
  "p.notifications.status_read": "Прочитано",
  "p.notifications.digest_count_one": "ежедневная сводка из {n} уведомления",
  "p.notifications.digest_count_other": "ежедневная сводка из {n} уведомлений",
  "p.notifications.open_subject": "Открыть план",
  "p.notifications.mark_read": "Отметить прочитанным",
  "p.notifications.mark_read_named": "Отметить прочитанным: {message}",
  "p.notifications.marked_read": "Отмечено прочитанным.",
  "p.notifications.could_not_load_inbox": "Не удалось загрузить ваши входящие.",
  "p.notifications.could_not_mark":
    "Не удалось отметить уведомление прочитанным.",
  "p.notifications.prefs_heading": "О чём вас уведомляют",
  "p.notifications.prefs_intro":
    "Включайте или отключайте каждый вид уведомлений для входящих и для каждого вида адресатов. Копия отправляется на вебхук, в чат или по электронной почте, только если ниже вы добавили адресата такого вида. Ежедневная сводка собирает уведомления за день об одном объекте в одну строку входящих; ничего не задерживается и не отбрасывается.",
  "p.notifications.pref_daily": "Ежедневная сводка во входящих",
  "p.notifications.pref_saved": "Ваш выбор сохранён.",
  "p.notifications.pref_reset_done":
    "Возвращено значение по умолчанию: включено, по мере поступления.",
  "p.notifications.reset": "Вернуть значение по умолчанию",
  "p.notifications.reset_named": "Вернуть значение по умолчанию: {kind}",
  "p.notifications.could_not_load_prefs": "Не удалось загрузить ваш выбор.",
  "p.notifications.could_not_save_pref": "Не удалось сохранить выбор.",
  "p.notifications.sinks_heading": "Куда направляются копии",
  "p.notifications.sinks_intro":
    "Адресат получает копию каждого вашего уведомления, содержащую только собственный текст уведомления. Адрес является секретом: после сохранения он показывается только в сокращённом виде. Медленный или неисправный адресат никогда не задерживает входящие; неудавшаяся отправка повторяется ещё четыре раза примерно за два с половиной часа, после чего прекращается.",
  "p.notifications.no_sinks":
    "У вас нет адресатов; ваши уведомления поступают только во входящие.",
  "p.notifications.chat_named": "Чат {name}",
  "p.notifications.sink_sending": "Отправка",
  "p.notifications.sink_paused": "Приостановлено",
  "p.notifications.show_deliveries": "Последние доставки",
  "p.notifications.deliveries_named": "Последние доставки: {destination}",
  "p.notifications.remove_sink": "Удалить",
  "p.notifications.remove_sink_named": "Удалить: {destination}",
  "p.notifications.sink_removed": "Адресат удалён.",
  "p.notifications.deliveries_heading": "Последние доставки для {destination}",
  "p.notifications.deliveries_note":
    "Причина сбоя отправки здесь не показывается, так как она может содержать секретный адрес.",
  "p.notifications.no_deliveries":
    "Этому адресату пока ничего не отправлялось.",
  "p.notifications.attempts_one": "{n} попытка",
  "p.notifications.attempts_other": "{n} попыток",
  "p.notifications.delivered_at": "отправлено {date}",
  "p.notifications.next_attempt": "следующая попытка {date}",
  "p.notifications.close_deliveries": "Закрыть доставки",
  "p.notifications.add_heading": "Добавить адресата",
  "p.notifications.f_kind": "Вид",
  "p.notifications.f_destination_email": "Адрес электронной почты",
  "p.notifications.f_destination_url": "Веб-адрес (https, на публичный хост)",
  "p.notifications.f_chat_id": "Чат Telegram (его номер или @ и его имя)",
  "p.notifications.add_sink": "Добавить адресата",
  "p.notifications.sink_added": "Адресат добавлен: {destination}.",
  "p.notifications.could_not_load_sinks":
    "Не удалось загрузить ваших адресатов.",
  "p.notifications.could_not_load_deliveries": "Не удалось загрузить доставки.",
  "p.notifications.could_not_save_sink": "Не удалось добавить адресата.",
  "p.notifications.could_not_remove_sink": "Не удалось удалить адресата.",
  "p.notifications.problem_too_many":
    "У вас уже десять адресатов — это максимум: сначала удалите одного.",
  "p.notifications.problem_destination_required": "Укажите адрес для отправки.",
  "p.notifications.problem_email_invalid": "Это не адрес электронной почты.",
  "p.notifications.problem_url_invalid": "Это не веб-адрес.",
  "p.notifications.problem_url_not_https":
    "Веб-адрес должен начинаться с https://.",
  "p.notifications.problem_url_credentials":
    "Веб-адрес не должен содержать имя пользователя или пароль.",
  "p.notifications.problem_url_private":
    "Веб-адрес должен указывать на публичный хост, а не на локальный, частный или внутренний.",
  "p.notifications.problem_url_too_long": "Веб-адрес слишком длинный.",
  "p.notifications.problem_chat_id_required":
    "Для адресата Telegram нужен чат, в который отправляются сообщения.",
  "p.notifications.refused_forbidden":
    "Вам нельзя изменять эти настройки уведомлений.",
  "p.notifications.refused_note_gone":
    "Этого уведомления больше нет, или оно не ваше; входящие прочитаны заново.",
  "p.notifications.refused_sink_gone":
    "Этот адресат уже был удалён; ваши адресаты прочитаны заново.",
  "p.notifications.refused_not_addressable":
    "Ваша учётная запись входа не относится к тем, которым можно адресовать уведомления, поэтому показывать здесь нечего.",
  "p.notifications.refused_pref_invalid":
    "Сервис не принял этот выбор; ваш выбор прочитан заново.",
  "p.notifications.refused_sink_invalid":
    "Сервис не принял адресата: это должен быть адрес электронной почты или веб-адрес https на публичный хост; для Telegram нужен его чат; всего может быть не более десяти.",
  "l.enum.notify_kind.approval_decided":
    "По запрошенному вами утверждению принято решение",
  "l.enum.notify_kind.approval_requested": "Вас просят что-то утвердить",
  "l.enum.notify_kind.auto_watch":
    "Автоматическое отслеживание того, над чем вы работаете",
  "l.enum.notify_kind.automation": "Вас уведомляет правило",
  "l.enum.notify_kind.comment": "Комментарии",
  "l.enum.notify_kind.escalation": "Эскалации",
  "l.enum.notify_kind.exceptions_digest": "Еженедельная сводка исключений",
  "l.enum.notify_kind.reschedule_proposed": "Предложена новая дата",
  "l.enum.notify_kind.review_invited": "Вас приглашают на рецензирование",
  "l.enum.notify_kind.review_submitted": "Рецензия представлена",
  "l.enum.notify_kind.scheduled": "Запланированное напоминание",
  "l.enum.notify_kind.status_update_due": "Подходит срок обновления статуса",
  "l.enum.notify_kind.task_assigned": "Вам назначена задача",
  "l.enum.notify_kind.watch_update": "Изменилось то, что вы отслеживаете",
  "l.enum.notify_kind.process_todo": "Шаг процесса, который выполняете вы",
  "l.enum.notify_kind.process_notice": "Шаг процесса, о котором вас уведомляют",
  "l.enum.notify_kind.break_glass": "Закрытый план открыт в экстренном порядке",
  "l.enum.notify_channel.in_app": "Во входящих",
  "l.enum.notify_channel.webhook": "Через подписанный вебхук",
  "l.enum.notify_channel.chat": "В чате",
  "l.enum.notify_channel.email": "По электронной почте",
  "l.enum.sink_kind.email": "Электронная почта",
  "l.enum.sink_kind.webhook": "Подписанный вебхук",
  "l.enum.delivery_status.pending": "Ожидает отправки",
  "l.enum.delivery_status.sent": "Отправлено",
  "l.enum.delivery_status.failed": "Попытки прекращены",
  "l.enum.delivery_status.skipped":
    "Не отправлено: адресата или уведомления больше нет",
  "l.link.kind_docs": "Документы",
  "l.link.kind_repo": "Репозиторий кода",
  "l.link.kind_ticket": "Тикет",
  "l.link.kind_design": "Дизайн",
  "l.link.kind_chat": "Чат",
  "l.link.kind_tracker": "Трекер задач",
  "l.link.kind_other": "Другое",
  "l.link.system_other": "Другой инструмент",
  "l.fw.method_waterfall": "Водопад",
  "l.fw.method_hybrid": "Гибридный",
  "l.fw.method_other": "Другой",
  "l.fw.model_sponsor": "Решает спонсор",
  "l.fw.model_steering_committee": "Руководящий комитет",
  "l.fw.model_product_owner": "Владелец продукта",
  "l.fw.model_delegated": "Делегированное решение",
  "l.fw.model_consensus": "Консенсус",
  "l.fw.model_other": "Другая",
  "p.frameworks.wow": "Способы работы",
  "p.frameworks.wow_ppm": "Способы работы — PPM",
  "p.frameworks.gov": "Рамки управления",
  "p.frameworks.gov_ppm": "Рамки управления — PPM",
  "p.frameworks.intro_wow":
    "Как работает команда: метод, длительность итерации, лимит незавершённой работы. План следует одному из способов — общему или собственному; если способа нет, он наследует способ ближайшего плана выше.",
  "p.frameworks.intro_gov":
    "Как управляется работа: кто принимает решения, как часто проводится проверка и путь эскалации, по которому идут эскалации. План следует одним рамкам — общим или собственным; если рамок нет, он наследует рамки ближайшего плана выше.",
  "p.frameworks.back": "К плану",
  "p.frameworks.loading": "Загрузка…",
  "p.frameworks.could_not_load": "Не удалось загрузить.",
  "p.frameworks.could_not": "Не удалось выполнить: {reason}",
  "p.frameworks.in_force": "Действует для этого плана",
  "p.frameworks.source_own": "задано в этом плане",
  "p.frameworks.source_inherited": "унаследовано от плана выше",
  "p.frameworks.source_none": "нет",
  "p.frameworks.from_plan": "см. этот план",
  "p.frameworks.scope_plan": "написано для одного плана",
  "p.frameworks.scope_org": "общее для организации",
  "p.frameworks.none": "Ничего не действует.",
  "p.frameworks.edit": "Изменить",
  "p.frameworks.unfollow": "Перестать следовать (снова наследовать)",
  "p.frameworks.follow_heading": "Следовать другому",
  "p.frameworks.no_others": "Других общих вариантов для следования нет.",
  "p.frameworks.choose": "Общий или существующий",
  "p.frameworks.follow": "Следовать этому",
  "p.frameworks.write_new": "Написать новый для этого плана",
  "p.frameworks.followed": "Теперь план следует ему.",
  "p.frameworks.unfollowed": "Следование прекращено; план снова наследует.",
  "p.frameworks.saved": "Сохранено.",
  "p.frameworks.new_heading": "Новый вариант для этого плана",
  "p.frameworks.edit_heading": "Изменить этот вариант",
  "p.frameworks.f_name": "Название",
  "p.frameworks.f_description": "Описание",
  "p.frameworks.save": "Сохранить",
  "p.frameworks.cancel": "Отмена",
  "p.frameworks.methodology": "Метод",
  "p.frameworks.cadence_days": "Длительность итерации (дни)",
  "p.frameworks.wip_limit": "Лимит незавершённой работы",
  "p.frameworks.ready": "Критерии готовности к работе",
  "p.frameworks.done": "Критерии завершённости",
  "p.frameworks.ceremonies": "Церемонии",
  "p.frameworks.decision_model": "Модель принятия решений",
  "p.frameworks.review_cadence": "Проверка каждые (дни)",
  "p.frameworks.reporting_cadence": "Отчёт вышестоящим каждые (дни)",
  "p.frameworks.path": "Путь эскалации",
  "p.frameworks.rights": "Права на решения",
  "p.frameworks.forums": "Форумы",
  "p.frameworks.days": "{n} дн.",
  "p.frameworks.path_level": "Уровень",
  "p.frameworks.path_body": "Кто рассматривает",
  "p.frameworks.path_trigger": "Что направляет туда",
  "p.frameworks.path_add": "Добавить уровень",
  "p.frameworks.path_remove": "Удалить уровень {n}",
  "p.frameworks.name_problem": "Дайте название длиной от 1 до 120 символов.",
  "p.frameworks.number_problem":
    "Дни — целые числа от 1 до 366, а лимит — от 1 до 1000.",
  "p.frameworks.path_problem":
    "Каждый уровень эскалации — разное целое число от 1 до 9, с указанием, кто рассматривает и что направляет туда.",
  "p.frameworks.confirm_shared_one":
    "Это общий вариант: сохранение изменит его ещё для {n} другого плана.",
  "p.frameworks.confirm_shared_other":
    "Это общий вариант: сохранение изменит его ещё для других планов: {n}.",
  "p.frameworks.confirm_problem_one":
    "Отметьте флажок, чтобы подтвердить изменение для {n} другого плана.",
  "p.frameworks.confirm_problem_other":
    "Отметьте флажок, чтобы подтвердить изменение для других планов: {n}.",
  "p.frameworks.deleted": "Удалено: {name}.",
  "p.frameworks.delete": "Удалить {name} (никто ему не следует)",
  "p.frameworks.text_problem": "Определение содержит не более 10000 символов.",
  "p.frameworks.practice_problem":
    "Каждой церемонии или каждому форуму нужны название и периодичность, и всего их не более 20.",
  "p.frameworks.rights_problem":
    "Каждому праву на решение нужны решение и тот, кто его принимает, и всего их не более 20.",
  "p.frameworks.practice_name": "Название",
  "p.frameworks.practice_cadence": "Как часто",
  "p.frameworks.practice_purpose": "Для чего",
  "p.frameworks.row_remove": "Удалить строку {n}",
  "p.frameworks.ceremony_add": "Добавить церемонию",
  "p.frameworks.forum_add": "Добавить форум",
  "p.frameworks.right_decision": "Решение",
  "p.frameworks.right_decides": "Кто решает",
  "p.frameworks.right_consulted": "Кого спрашивают сначала",
  "p.frameworks.right_add": "Добавить право на решение",
  "p.links.links": "Ссылки",
  "p.links.links_ppm": "Ссылки — PPM",
  "p.links.intro":
    "Где находятся документы и работа этого плана, включая его доску Jira, Trello, Asana, Linear или Monday. Адрес сохраняется и показывается; он никогда не загружается.",
  "p.links.back": "К плану",
  "p.links.loading": "Загрузка…",
  "p.links.could_not_load": "Не удалось загрузить ссылки.",
  "p.links.could_not": "Не удалось выполнить: {reason}",
  "p.links.none": "Ссылок пока нет.",
  "p.links.opens_new": "откроется в новой вкладке",
  "p.links.remove": "Удалить {label}",
  "p.links.added": "Добавлено: {label}.",
  "p.links.removed": "Удалено: {label}.",
  "p.links.add_heading": "Добавить ссылку",
  "p.links.add": "Добавить ссылку",
  "p.links.f_label": "Подпись",
  "p.links.f_url": "Адрес (http или https)",
  "p.links.f_kind": "Вид",
  "p.links.f_system": "Внешний инструмент",
  "p.links.f_id": "Его идентификатор там",
  "p.links.url_problem":
    "Адрес должен начинаться с http:// или https:// и содержать имя хоста.",
  "p.links.label_problem": "Укажите подпись.",
  "p.links.system_problem": "Ссылка на трекер должна называть свой инструмент.",
  "p.links.system_kind_problem":
    "Инструмент называет только ссылка на трекер или тикет.",
  "p.links.id_problem":
    "Идентификатору нужен инструмент, и он не содержит пробелов.",
  "p.links.domain_problem": "Ссылки этого инструмента находятся на {domains}.",
  "p.escalations.intro":
    "Передайте вопрос выше, когда проект не может или не должен решить его сам. Эскалация поднимается на одном из уровней пути эскалации в рамках управления проекта, принимается к сведению, передаётся на уровень выше, если никто не отвечает, и завершается решением или отзывом. Назначенный ответственным человек и тот, кто её поднял, получают уведомление во входящие (электронная почта не отправляется).",
  "p.escalations.register_intro":
    "Все эскалации по планам, которые вы можете читать.",
  "p.escalations.none": "Эскалаций пока нет.",
  "p.escalations.none_match": "Нет подходящих эскалаций.",
  "p.escalations.raise_heading": "Поднять эскалацию",
  "p.escalations.f_title": "Название",
  "p.escalations.f_reason": "Почему",
  "p.escalations.f_dimension": "Какой допуск",
  "p.escalations.f_level": "Уровень (пусто — первый уровень пути)",
  "p.escalations.f_to": "Кому эскалировать (пусто — по пути)",
  "p.escalations.f_owner":
    "Человек, для которого она предназначена (person:… или worker:…)",
  "p.escalations.f_respond": "Ответ нужен к",
  "p.escalations.f_description": "Что и почему",
  "p.escalations.raise": "Поднять",
  "p.escalations.raised": "Поднята эскалация: {title}.",
  "p.escalations.acknowledge": "Принять к сведению",
  "p.escalations.take_up": "Передать на уровень выше",
  "p.escalations.take_up_note": "Почему она поднимается выше",
  "p.escalations.resolve": "Решить",
  "p.escalations.outcome": "Исход",
  "p.escalations.response": "Ответ",
  "p.escalations.withdraw": "Отозвать",
  "p.escalations.done": "Готово.",
  "p.escalations.could_not": "Не получилось: {reason}",
  "p.error.status_401": "Вы не вошли в систему, или ваш сеанс завершён.",
  "p.error.status_403": "Вам это не разрешено.",
  "p.error.status_404": "Не найдено, или вы этого не видите.",
  "p.error.status_409":
    "Это противоречит текущему состоянию: кто-то другой изменил или решил это, или это уже выполнено.",
  "p.error.status_412":
    "С момента загрузки это изменилось: перезагрузите и повторите попытку.",
  "p.error.status_413": "Слишком большой объём.",
  "p.error.status_422": "Введённое значение не принято.",
  "p.error.status_429":
    "Слишком много запросов: подождите немного и повторите попытку.",
  "p.bridge.open_workforce": "Открыть {name} в планировании персонала",
  "p.bridge.open_contacts": "Открыть {name} в контактах",
  "p.error.client": "Запрос отклонён.",
  "p.error.server": "В сервисе возникла проблема: повторите попытку.",
  "p.escalations.note_problem": "Напишите не менее 10 символов.",
  "p.escalations.show": "Показать: {title}",
  "p.escalations.hide": "Скрыть: {title}",
  "p.escalations.history": "История",
  "p.escalations.status": "Статус",
  "p.escalations.plan": "План",
  "p.escalations.level_col": "Уровень",
  "p.escalations.age": "Возраст",
  "p.escalations.respond_by": "Ответ нужен к",
  "p.escalations.filter": "Показать",
  "p.escalations.all": "Все",
  "p.escalations.live": "Ещё открытые",
  "p.escalations.overdue_count": "Просрочено: {n}",
  "p.escalations.loading": "Загрузка…",
  "p.escalations.could_not_load": "Не удалось загрузить эскалации",
  "p.escalations.back": "← План",
  "p.escalations.register_link": "Все эскалации",
  "p.tolerances.needs_escalation":
    "Допуск превышен, и никто не поднял эскалацию.",
  "p.tolerances.raise_one": "Поднять эскалацию",
  "p.tolerances.escalate_all": "Эскалировать каждый красный допуск",
  "p.tolerances.escalate_all_intro":
    "Для каждого показателя, у которого допуск превышен и эскалация ещё не поднята, создаётся эскалация на первом уровне пути эскалации. Сами по себе эскалации не поднимаются.",
  "p.tolerances.escalated_one": "Поднята {n} эскалация.",
  "p.tolerances.escalated_other": "Поднято эскалаций: {n}.",
  "p.tolerances.escalations_one": "{n} открытая эскалация",
  "p.tolerances.escalations_other": "Открытых эскалаций: {n}",
  "p.resources.rbs.by_role": "По ролям",
  "p.resources.rbs.show_tasks": "Показать открытые задачи роли «{role}»",
  "p.resources.rbs.hide_tasks": "Скрыть открытые задачи роли «{role}»",
  "p.resources.rbs.no_open_tasks": "нет открытых задач",
  "p.resources.rbs.tasks_in_plans_you_cannot_read":
    "Не показано: {count} в планах, которые вы не можете читать.",
  "p.resources.rbs.only_the_first_500_tasks":
    "Прочитаны только первые 500 задач, поэтому список может быть неполным.",
  "p.resources.rbs.could_not_load_tasks":
    "Не удалось загрузить задачи: {reason}",
  "p.resources.rbs.by_skill": "По навыкам",
  "p.resources.rbs.demand_by_skill": "Спрос по навыкам",
  "p.resources.rbs.demand_fte": "Спрос, FTE",
  "p.resources.rbs.no_roles_in_this_window": "В этом окне нет ролей.",
  "p.resources.rbs.no_skill_demand_in_this":
    "В этом окне нет спроса на навыки.",
  "p.resources.rbs.not_known": "неизвестно",
  "p.resources.rbs.people": "Люди",
  "p.resources.rbs.resource_breakdown": "Структура ресурсов",
  "p.resources.rbs.resource_breakdown_ppm": "Структура ресурсов — PPM",
  "p.resources.rbs.role": "Роль",
  "p.resources.rbs.roles_with_people_demand_and":
    "Роли с людьми, спросом и предложением",
  "p.resources.rbs.skill": "Навык",
  "p.resources.rbs.supply_fte": "Предложение, FTE",
  "p.resources.rbs.you_can_see_headcounts_only":
    "Вы видите только численность, а не имена.",
  "p.resources.resources": "Ресурсы",
  "p.resources.resources_ppm": "Ресурсы — PPM",
  "p.resources.role": "Роль",
  "p.resources.roles_and_totals_only": "только роли и итоги",
  "p.resources.roles_asked_for_with_no":
    "Запрошенные роли, на которые пока нет человека",
  "p.resources.short_spare_balanced_supply_not":
    "▼ нехватка · ▲ избыток · ● баланс · ? предложение неизвестно. Затенение показывает только размер разрыва.",
  "p.resources.show": "Показать",
  "p.resources.skill": "Навык",
  "p.resources.skill_demand_only": "Навык (только спрос)",
  "p.resources.supply_fte": "Предложение, FTE",
  "p.resources.to": "По",
  "p.resources.unclassified": "не классифицировано",
  "p.resources.week": "Неделя",
  "p.resources.what_the_plans_ask_for":
    "Что запрашивают планы и что могут дать люди в каждой роли, в полных эквивалентах ставки. Предлагаемые бронирования хранятся отдельно и никогда не учитываются.",
  "p.reviews.accept": "Принять",
  "p.reviews.accepted": "принято",
  "p.reviews.actions": "Действия",
  "p.reviews.advance": "Продвинуть",
  "p.reviews.all": "все",
  "p.reviews.collaborative_review": "Совместная рецензия",
  "p.reviews.consensus": "Консенсус",
  "p.reviews.consensus_for_this_subject": "Консенсус по этому объекту",
  "p.reviews.decline": "Отказаться",
  "p.reviews.declined": "отказано",
  "p.reviews.delegate": "Делегировать",
  "p.reviews.delegate_a_specific_item_to":
    "Делегируйте конкретный пункт подходящему эксперту. Вердикт может представить только рецензент, принявший приглашение, поэтому приглашение без ответа никогда не становится доказательством.",
  "p.reviews.due": "срок",
  "p.reviews.e_g_clinical_safety": "например, клиническая безопасность",
  "p.reviews.expertise": "экспертиза",
  "p.reviews.expertise_2": "Экспертиза",
  "p.reviews.expired": "истекло",
  "p.reviews.external": "внешний",
  "p.reviews.hold": "Приостановить",
  "p.reviews.idea": "идея",
  "p.reviews.internal": "внутренний",
  "p.reviews.invited": "приглашён",
  "p.reviews.no_review_invitations_yet":
    "Приглашений на рецензирование пока нет.",
  "p.reviews.no_score": "нет оценки",
  "p.reviews.person_uuid": "person:<uuid> (человек)",
  "p.reviews.plan": "план",
  "p.reviews.proposal": "предложение",
  "p.reviews.reject": "Отклонить",
  "p.reviews.reviewer": "рецензент",
  "p.reviews.reviewer_2": "Рецензент",
  "p.reviews.reviews_ppm": "Рецензии — PPM",
  "p.reviews.scope": "область",
  "p.reviews.scope_2": "Область",
  "p.reviews.status": "статус",
  "p.reviews.status_2": "Статус",
  "p.reviews.subject": "объект",
  "p.reviews.subject_2": "Объект",
  "p.reviews.subject_pid": "идентификатор объекта",
  "p.reviews.submitted": "представлено",
  "p.reviews.submitted_outstanding_declined_exter":
    "представлено: {submitted} · ожидается: {outstanding} · отказов: {declined} · внешних: {external_submitted}",
  "p.reviews.uuid": "uuid (идентификатор)",
  "p.reviews.verdict": "Вердикт",
  "p.reviews.withdraw": "Отозвать",
  "p.risk.category": "Категория",
  "p.risk.concentration": "Концентрация",
  "p.risk.escalated": "Эскалирован",
  "p.risk.exposure": "Подверженность",
  "p.risk.exposure_25_of_estate":
    "— подверженность {exposure} (≥ 25% всей совокупности)",
  "p.risk.heatmap": "Тепловая карта",
  "p.risk.item": "Элемент",
  "p.risk.materialized": "Реализовался",
  "p.risk.max_estate_max_single_risk":
    "— максимум по совокупности {v}, максимум по одному риску {v2}",
  "p.risk.no_open_risks": "Открытых рисков нет.",
  "p.risk.open_exposure": "Открытая подверженность",
  "p.risk.open_risks_estate_open_exposure":
    "открытых рисков · открытая подверженность по совокупности",
  "p.risk.overdue_reviews": "Просроченные проверки",
  "p.risk.portfolio": "Портфель",
  "p.risk.posture_by_portfolio": "Состояние по портфелям",
  "p.risk.ppm": "{t} — PPM",
  "p.risk.probability_impact": "вероятность ↓ / влияние →",
  "p.risk.review_due": "(проверка к {review_date})",
  "p.risk.risk": "Риск",
  "p.risk.top_risks": "Главные риски",
  "p.risk.unassigned": "(не назначено)",
  "p.scenarios.alignment": "Соответствие",
  "p.scenarios.cap": "Лимит",
  "p.scenarios.compare": "Сравнить",
  "p.scenarios.currency": "Валюта",
  "p.scenarios.delta_b_a": "Разница (b − a)",
  "p.scenarios.evaluation": "Оценка",
  "p.scenarios.feasible": "Выполним",
  "p.scenarios.no": "нет",
  "p.scenarios.open_exposure": "Открытая подверженность",
  "p.scenarios.pick": "выберите…",
  "p.scenarios.planned": "Запланировано, {currency}",
  "p.scenarios.scenario": "Сценарий",
  "p.scenarios.scenarios_ppm": "Сценарии — PPM",
  "p.scenarios.status": "Статус",
  "p.scenarios.work_item_pids_space_comma":
    "идентификаторы рабочих элементов (через пробел или запятую)",
  "p.scenarios.yes": "да",
  "p.security.exposure": "Подверженность",
  "p.security.item": "Элемент",
  "p.security.late_stage_items_with_no":
    "Элементы на поздней стадии без записанного риска безопасности",
  "p.security.no_security_risks_recorded": "Рисков безопасности не записано.",
  "p.security.none_every_late_stage_item":
    "Нет — у каждого элемента на поздней стадии есть записанный риск безопасности.",
  "p.security.open_exposure":
    "{note} · открытая подверженность {open_exposure}",
  "p.security.owner": "Владелец",
  "p.security.ppm": "{t} — PPM",
  "p.security.risk": "Риск",
  "p.security.security_risk_register": "Реестр рисков безопасности",
  "p.security.status": "Статус",
  "p.signin.check_your_email_for_a":
    "Проверьте почту: вам отправлена ссылка для входа.",
  "p.signin.could_not_send_the_sign":
    "Не удалось отправить ссылку для входа. Повторите попытку.",
  "p.signin.email": "Электронная почта",
  "p.signin.send_magic_link": "Отправить волшебную ссылку",
  "p.signin.sign_in": "Войти",
  "p.technology.as_of": "по состоянию на {as_of} · {convention}",
  "p.technology.as_of_edges": "по состоянию на {as_of} · связей: {edges}",
  "p.technology.calendar_days": "{median_lead_days} календарных дней",
  "p.technology.completed_before_timing_existed_unti":
    "· {undated_completions} завершено до появления учёта времени (без времени)",
  "p.technology.cross_portfolio_edges": "Связи между портфелями",
  "p.technology.delivery_flow": "Поток поставки",
  "p.technology.dependency_risk": "Риск зависимостей",
  "p.technology.dependents": "Зависимые",
  "p.technology.edges_importing_risk_from_a":
    "Связи, переносящие риск от красного предшественника",
  "p.technology.exposure": "Подверженность",
  "p.technology.item": "Элемент",
  "p.technology.kind": "Вид",
  "p.technology.median_lead": "Медианное время выполнения:",
  "p.technology.milestones_completed": "Завершённые вехи",
  "p.technology.month": "Месяц",
  "p.technology.most_depended_on_single_point":
    "Больше всего зависимостей (кандидаты на единую точку отказа)",
  "p.technology.no_dependencies_recorded_yet": "Зависимости пока не записаны.",
  "p.technology.no_tech_debt_risks_recorded": "Рисков tech_debt не записано.",
  "p.technology.no_tech_tags_recorded_yet": "Тегов `tech:*` пока не записано.",
  "p.technology.no_timed_completions": "нет завершений со временем",
  "p.technology.no_timed_completions_in_the":
    "В этом окне нет завершений со временем.",
  "p.technology.none_all_edges_stay_within":
    "Нет — все связи остаются в пределах одного портфеля.",
  "p.technology.none_no_red_predecessors":
    "Нет — красных предшественников нет.",
  "p.technology.open_exposure":
    "{note} · открытая подверженность {open_exposure}",
  "p.technology.over_timed_completions":
    "по завершениям со временем: {timed_completions}",
  "p.technology.owner": "Владелец",
  "p.technology.ppm": "{t} — PPM",
  "p.technology.predecessor": "Предшественник",
  "p.technology.predecessor_red": "Предшественник (красный)",
  "p.technology.ring_votes": "(голосов по кольцам: {ring_votes})",
  "p.technology.risk": "Риск",
  "p.technology.status": "Статус (",
  "p.technology.status_2": "Статус",
  "p.technology.successor": "Последователь",
  "p.technology.successor_at_risk": "Последователь под риском",
  "p.technology.technical_debt_register": "Реестр технического долга",
  "p.technology.technology_radar": "Технологический радар",
  "p.verify.request_a_new_link": "Запросить новую ссылку",
  "p.verify.sign_in_link": "Ссылка для входа",
  "p.milestones.milestone_by_due_date_one": "{length} веха, по сроку",
  "p.milestones.milestone_by_due_date_other": "{length} вех, по сроку",
  "p.okrs.objective_least_progressed_first_one":
    "{length} цель, сначала с наименьшим прогрессом",
  "p.okrs.objective_least_progressed_first_other":
    "{length} целей, сначала с наименьшим прогрессом",
  "p.plans.pid.board.blocked_calendar_day_one":
    "заблокирована {blocked_days} календарный день",
  "p.plans.pid.board.blocked_calendar_day_other":
    "заблокирована {blocked_days} календарных дней",
  "p.plans.pid.calendar.holiday_one": "{length} праздник: {join}",
  "p.plans.pid.calendar.holiday_other": "{length} праздников: {join}",
  "p.plans.pid.costs.for_across_one":
    "{money} за {hoursText} по {entries} записи.",
  "p.plans.pid.costs.for_across_other":
    "{money} за {hoursText} по {entries} записям.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_one":
    "У {plans_without_figures} плана ниже пока нет показателей (нет базового плана или нечего измерять); они не угадываются.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_other":
    "У {plans_without_figures} планов ниже пока нет показателей (нет базового плана или нечего измерять); они не угадываются.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_one":
    "завершено без возвратов назад (возвратов назад: {rework_count}). Читайте это рядом с пропускной способностью: рост пропускной способности при падении доли без возвратов означает, что работа возвращается сама к себе.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_other":
    "завершено без возвратов назад (возвратов назад: {rework_count}). Читайте это рядом с пропускной способностью: рост пропускной способности при падении доли без возвратов означает, что работа возвращается сама к себе.",
  "p.plans.pid.flow.of_items_finish_within_this_one":
    "{percent} элементов завершаются в этот срок, по выборке из {sample} завершённого элемента",
  "p.plans.pid.flow.of_items_finish_within_this_other":
    "{percent} элементов завершаются в этот срок, по выборке из {sample} завершённых элементов",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_one":
    "{length} риск влияет и на расписание, и на стоимость: каждый срабатывает один раз и влияет на оба показателя.",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_other":
    "{length} рисков влияют и на расписание, и на стоимость: каждый срабатывает один раз и влияет на оба показателя.",
  "p.plans.pid.leveling.apply_move_one": "Применить {length} перенос",
  "p.plans.pid.leveling.apply_move_other": "Применить переносов: {length}",
  "p.plans.pid.leveling.business_day_one": "{delay_days} рабочий день",
  "p.plans.pid.leveling.business_day_other": "{delay_days} рабочих дней",
  "p.plans.pid.leveling.other_task_would_be_pushed_one":
    "В результате будет сдвинута {length} другая задача",
  "p.plans.pid.leveling.other_task_would_be_pushed_other":
    "В результате будет сдвинуто других задач: {length}",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_one":
    "{hidden} контрольный список о людях вам не показывается.",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_other":
    "Контрольных списков о людях, которые вам не показываются: {hidden}.",
  "p.plans.pid.schedule_health.preset_thresholds_task_one":
    "· набор {preset} (пороги: {source}) · {tasks} задача",
  "p.plans.pid.schedule_health.preset_thresholds_task_other":
    "· набор {preset} (пороги: {source}) · задач: {tasks}",
  "p.plans.pid.schedule_health.task_one": "{length} задача",
  "p.plans.pid.schedule_health.task_other": "задач: {length}",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_one":
    "{title}: вероятность {probability_percent}% добавить {impact_days} рабочий день",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_other":
    "{title}: вероятность {probability_percent}% добавить {impact_days} рабочих дней",
  "p.plans.pid.wbs.move_up": "Переместить выше: {title}",
  "p.plans.pid.wbs.move_down": "Переместить ниже: {title}",
  "p.plans.pid.wbs.moved": "Перемещено: {title}.",
  "p.plans.pid.wbs.truncated":
    "Показано {shown} из {total} задач: этот план слишком велик, чтобы показать его полностью, поэтому структура ниже неполная.",
  "p.plans.pid.critical_path.too_large":
    "Этот план слишком велик, чтобы рассчитать его расписание целиком за один ответ.",
  "p.plans.pid.critical_path.show_critical_only":
    "Показать только критическую часть",
  "p.plans.pid.critical_path.critical_only_notice":
    "Показаны только критические и почти критические задачи; остальная часть плана опущена.",
  "p.plans.pid.earned_value.names_truncated":
    "Этот план очень велик: показатели охватывают каждую задачу, но некоторые названия задач ниже могут отсутствовать.",
  "p.plans.pid.wbs.could_not_move": "Не удалось переместить задачу: {reason}",
  "p.plans.pid.wbs.sub_task_one": "({children} подзадача)",
  "p.plans.pid.wbs.sub_task_other": "(подзадач: {children})",
  "p.plans.pid.wbs.links": "Связи",
  "p.plans.pid.wbs.link_one": "· {links} связь",
  "p.plans.pid.wbs.link_other": "· связей: {links}",
  "p.common.on": "вкл.",
  "p.common.off": "выкл.",
  "p.common.pre_gate": "до шлюза",
  "p.common.role_word": "роль",
  "p.common.person_word": "человек",
  "c.NetworkDiagram.task_fallback": "задача",
  "c.NetworkDiagram.task_summary":
    "{title}, с {early_start} по {early_finish}, резерв времени: {total_float} рабочих дней",
  "c.NetworkDiagram.task_summary_critical":
    "{title}, с {early_start} по {early_finish}, резерв времени: {total_float} рабочих дней, на критическом пути",
  "c.NetworkDiagram.lag_one": ", запаздывание {lag_days} рабочий день",
  "c.NetworkDiagram.lag_other": ", запаздывание {lag_days} рабочих дней",
  "c.KpiCard.direction_higher": "выше",
  "c.KpiCard.direction_lower": "ниже",
  "c.CalendarMonth.legend":
    "– нерабочий день   ✕ праздник   A записанное отсутствие",
  "c.CalendarMonth.legend_editable":
    " · выберите рабочий день, чтобы отметить или снять праздник",
  "c.CostRiskView.run_summary":
    "начальное значение {seed} · итераций: {iterations} · {distribution}",
  "c.CostRiskView.dist_pert": "Распределение PERT",
  "c.CostRiskView.dist_triangular": "Треугольное",
  "c.CostRiskView.risks_without_cost_one":
    "У {count} риска из реестра нет полей стоимости, и он не входит в расчёт.",
  "c.CostRiskView.risks_without_cost_other":
    "У рисков из реестра ({count}) нет полей стоимости, и они не входят в расчёт.",
  "c.CumulativeFlow.hide_the_numbers": "Скрыть числа",
  "c.CumulativeFlow.show_the_numbers": "Показать числа",
  "c.CostRiskView.scope_all_one": "Все {plans} доступный вам для чтения план",
  "c.CostRiskView.scope_all_other":
    "Все доступные вам для чтения планы: {plans}",
  "c.CostRiskView.scope_this_plan": "Этот план",
  "c.MilestoneCard.reason": "Причина",
  "c.MilestoneCard.reason_required_open":
    " (обязательно: задачи, которых она ждёт, ещё открыты)",
  "c.MilestoneCard.reason_optional": " (необязательно)",
  "c.VarianceTable.matched_of_baselined":
    "{matched} из {baseline_tasks} задач базового плана всё ещё в плане:",
  "c.VarianceTable.slipped_pulled_on":
    "сдвинуто назад: {slipped}, сдвинуто вперёд: {pulled_in}, по базовому плану: {on_baseline}{unmeasurable}.",
  "c.VarianceTable.cannot_be_measured":
    ", не поддаются измерению: {unmeasurable}",
  "c.VarianceTable.added_removed":
    "С тех пор добавлено: {added}, удалено: {removed}.",
  "c.VarianceTable.finish_later_one":
    "Завершение плана позже базового на {days} рабочий день{note}.",
  "c.VarianceTable.finish_later_other":
    "Завершение плана позже базового на {days} рабочих дней{note}.",
  "c.VarianceTable.finish_earlier_one":
    "Завершение плана раньше базового на {days} рабочий день{note}.",
  "c.VarianceTable.finish_earlier_other":
    "Завершение плана раньше базового на {days} рабочих дней{note}.",
  "c.VarianceTable.check_removed":
    " (проверьте удалённые задачи: удаление работы не является улучшением)",
  "p.plans.pid.critical_path.finishes": "Завершается {date}.",
  "p.plans.pid.critical_path.nothing_scheduled":
    "Пока ничего не запланировано.",
  "p.plans.pid.critical_path.calendar_label": "Календарь: {calendar}.",
  "p.plans.pid.critical_path.calendar_plan": "этого плана",
  "p.plans.pid.critical_path.calendar_default":
    "по умолчанию (работает каждый день)",
  "p.plans.pid.critical_path.more_not_shown": " (остальное не показано)",
  "p.plans.pid.critical_path.pushed_later":
    "Её предшественники сдвигают её на {days} рабочих дней позже плана.",
  "p.plans.pid.earned_value.manual_percent_banner":
    "Часть этого показателя опирается на процент, введённый человеком, а не на завершённую работу: {tasks}.",
  "p.plans.pid.earned_value.a_task": "задача",
  "p.plans.pid.earned_value.overrun": " (перерасход)",
  "p.plans.pid.earned_value.target_budget": "бюджет",
  "p.plans.pid.earned_value.target_forecast": "прогноз",
  "p.plans.pid.earned_value.lights_use":
    "Индикаторы используют {which}: {values} (базисные пункты).",
  "p.plans.pid.earned_value.thresholds_default": "пороги по умолчанию",
  "p.plans.pid.earned_value.thresholds_deployment": "пороги этой установки",
  "p.plans.pid.earned_value.rollup_money":
    "Бюджет {bac}; запланировано {pv}; освоено {ev}; потрачено {ac}{unknown}.",
  "p.plans.pid.earned_value.cost_unknown_for":
    " (стоимость неизвестна для планов: {plans})",
  "p.plans.pid.earned_value.cpi_value": "CPI {value}",
  "p.plans.pid.earned_value.cpi_not_stated":
    "не указан: для некоторых планов стоимость неизвестна",
  "p.plans.pid.earned_value.weight_points": ", баллов веса: {weight}",
  "p.plans.pid.critical_path.moved_line":
    "{title}: {moved}{finish}{start} рабочих дней",
  "p.plans.pid.critical_path.moved_both": "начало и завершение",
  "p.plans.pid.critical_path.moved_start": "начало",
  "p.plans.pid.critical_path.moved_finish": "завершение",
  "p.plans.pid.critical_path.moved_finish_by": " завершение {days}",
  "p.plans.pid.critical_path.moved_start_by": " начало {days}",
  "p.plans.pid.flow.overdue_one":
    "{count} элемент уже вышел за ожидаемый срок. Стареющий элемент — единственное на этой странице, исход чего ещё можно изменить.",
  "p.plans.pid.flow.overdue_other":
    "Элементов, уже вышедших за ожидаемый срок: {count}. Стареющий элемент — единственное на этой странице, исход чего ещё можно изменить.",
  "p.plans.pid.flow.classified_with": "Классифицировано по {map}:",
  "p.plans.pid.flow.map_override": "переопределение установки",
  "p.plans.pid.flow.map_default": "карте по умолчанию",
  "p.plans.pid.governance.proposed_not_counted":
    "предложено{requested} — не учитывается",
  "p.plans.pid.governance.requested": ", запрошено",
  "p.plans.pid.joint_risk.run_summary":
    "{currency} · начальное значение {seed} · итераций: {iterations} · {distribution} · календарь {calendar}",
  "p.plans.pid.joint_risk.dist_pert": "Распределение PERT",
  "p.plans.pid.joint_risk.dist_triangular": "Треугольное",
  "p.plans.pid.leveling.applied_one":
    "Применён {ok} из {requested} переноса{refused}.",
  "p.plans.pid.leveling.applied_other":
    "Применено {ok} из {requested} переносов{refused}.",
  "p.plans.pid.leveling.applied_refused": "; отклонено: {refused}",
  "p.plans.pid.leveling.proposal_summary":
    "Календарь: {calendar} · загрузка {load}% на задачу · критических задач перемещается: {critical}",
  "p.plans.pid.leveling.may": "может",
  "p.plans.pid.leveling.may_not": "не может",
  "p.plans.pid.onboarding.due": ", срок {date}",
  "p.plans.pid.schedule_risk.risks_without_schedule_one":
    "У {count} риска из реестра нет полей расписания, и он не входит в расчёт.",
  "p.plans.pid.schedule_risk.risks_without_schedule_other":
    "У рисков из реестра ({count}) нет полей расписания, и они не входят в расчёт.",
  "p.resources.from_date": "с {date}",
  "p.resources.to_date": "по {date}",
  "p.kpis.none": "KPI пока нет.",
  "p.kpis.none_with_status": "KPI с таким статусом пока нет.",
  "p.milestones.none":
    "Вех пока нет. Добавьте их на странице управления плана.",
  "p.milestones.none_with_status":
    "Вех с таким статусом пока нет. Добавьте их на странице управления плана.",
  "p.okrs.none": "Целей пока нет.",
  "p.okrs.none_for_period": "Целей на период {period} пока нет.",
  "p.me.calendar.own_set": "Ваш собственный календарь задан.",
  "p.me.calendar.plan_applies":
    "Личного календаря нет: к вам применяется календарь плана.",
  "p.me.calendar.leave_counts":
    "Записанный для вас отпуск уже учитывается как выходные дни.",
  "p.documents.pid.page_title": "{title} — PPM",
  "p.documents.pid.document_fallback": "Документ",
  "p.plans.pid.page_title": "План: {name} — Main X",
  "p.plans.pid.plan_fallback": "План",
  "p.plans.pid.custom_scheme": "Своя схема ({name})",
  "p.plans.pid.calendar.own_calendar":
    "У этого плана есть собственный календарь.",
  "p.plans.pid.calendar.no_calendar":
    "Календарь не задан: работает каждый день.",
  "p.plans.pid.calendar.each_person_can_set":
    "Каждый человек может также задать свой собственный (",
  "p.reviews.consensus_line":
    "средняя оценка: {mean} · большинство: {majority} · {state}",
  "p.reviews.no_majority": "нет большинства",
  "p.reviews.everyone_answered": "ответили все",
  "p.reviews.still_waiting": "ещё ожидается",
  "l.announcement_view.just_now": "только что",
  "l.announcement_view.min_ago": "{minutes} мин назад",
  "l.announcement_view.h_ago": "{hours} ч назад",
  "l.announcement_view.calendar_days_ago": "{days} календарных дней назад",
  "l.announcement_view.scheduled": "запланировано",
  "l.announcement_view.expired": "Истекло",
  "l.api_ppm.task": "задача {uid}",
  "l.api_ppm.the_project_calendar_comes_across":
    " Календарь проекта переносится.",
  "l.api_tba.calendar_days": "{toFixed} календарных дней",
  "l.api_tba.work_in_progress_is_growing": "Незавершённая работа растёт",
  "l.api_tba.steady_state": "Устойчивое состояние",
  "l.api_tba.queue_draining": "Очередь сокращается",
  "l.api_tba.not_enough_data": "Недостаточно данных",
  "l.capacity_view.committed_against_a_capacity_as":
    "Задействовано {committed}% при ёмкости до {capacity}%",
  "l.capacity_view.committed": "Задействовано {committed}%",
  "l.capacity_view.to": " по {to}",
  "l.capacity_view.over_capacity_from_asked_available":
    "превышение ёмкости с {from}{v} (запрошено {demand}%, доступно {capacity}%)",
  "l.capacity_view.over_capacity": "превышение ёмкости",
  "l.capacity_view.committing_it_would_put_them":
    "; при закреплении будет превышена ёмкость",
  "l.capacity_view.proposed_not_counted":
    "{percent}% предложено, не учитывается{v}",
  "l.capacity_view.asked_available":
    "{from}{v}: запрошено {demand}%, доступно {capacity}%",
  "l.ceo_view.no_portfolios_to_report_on": "Портфелей для отчёта пока нет.",
  "l.ceo_view.portfolios_exist_but_hold_no":
    "Портфели есть, но планов в них пока нет.",
  "l.ceo_view.of_plans_are_red_amber":
    "Из {n} планов красных: {red}, жёлтых: {amber}, зелёных: {green}.",
  "c.CalendarMonth.not_a_business_day": "нерабочий день",
  "c.CalendarMonth.business_day": "рабочий день",
  "c.KpiCard.that_did_not_work": "Не получилось",
  "c.KpiCard.could_not_load_the_history": "Не удалось загрузить историю",
  "c.KpiCard.delete_the_kpi_its_readings":
    "Удалить KPI «{name}»? Его показания сохраняются.",
  "c.LineRange.could_not_save": "Не удалось сохранить",
  "c.MilestoneCard.that_did_not_work": "Не получилось",
  "c.MilestoneCard.could_not_load_the_plan":
    "Не удалось загрузить задачи плана",
  "c.MilestoneCard.could_not_load_the_history": "Не удалось загрузить историю",
  "c.ObjectiveCard.that_did_not_work": "Не получилось",
  "c.ObjectiveCard.delete_the_objective_and_its":
    "Удалить цель «{title}» и её ключевые результаты?",
  "c.ObjectiveCard.delete_the_key_result":
    "Удалить ключевой результат «{title}»?",
  "c.RaciMatrix.accountable_moved_from_to_a":
    "Ответственный за результат перенесён с {shortRef} на {shortRef2}: у задачи он один.",
  "c.RiskInputs.could_not_load_the_plan": "Не удалось загрузить задачи плана",
  "c.RiskInputs.could_not_save": "Не удалось сохранить",
  "c.RiskInputs.chance": "вероятность {probability_percent}%",
  "c.RiskInputs.adds_business_days_to_a":
    "добавляет задаче {schedule_impact_days} рабочих дней",
  "c.RiskInputs.adds": "добавляет {toFixed} {cost_currency}",
  "c.RiskInputs.not_used_in_the_simulations": "Не используется в симуляциях",
  "l.cpm_view.cannot_meet_its_constraint_or":
    "Не может уложиться в ограничение или крайний срок: не хватает {plural}",
  "l.cpm_view.on_the_critical_path_any":
    "На критическом пути: любой сдвиг переносит завершение",
  "l.cpm_view.of_float_near_critical": "резерв времени {d} (почти критическая)",
  "l.cpm_view.of_float": "резерв времени {d}",
  "l.cpm_view.baselined_to": ". Базовый план: с {start} по {finish}",
  "l.cpm_view.to": "{v}: с {early_start} по {early_finish}. {floatLabel}{v2}",
  "l.cpm_view.has_no_duration_yet": "длительность пока не задана",
  "l.cpm_view.has_no_start_date_and":
    "нет даты начала, и ничто перед ней не запланировано",
  "l.cpm_view.has_a_date_outside_the":
    "дата выходит за поддерживаемый диапазон",
  "l.cpm_view.slipped": "Сдвинуто назад",
  "l.cpm_view.finishes_later_than_baselined":
    "завершается позже базового плана на {d}",
  "l.cpm_view.pulled_in": "Сдвинуто вперёд",
  "l.cpm_view.finishes_earlier_than_baselined":
    "завершается раньше базового плана на {d}",
  "l.cpm_view.on_baseline": "По базовому плану",
  "l.cpm_view.finishes_when_it_was_baselined":
    "завершается тогда, когда было зафиксировано в базовом плане",
  "l.cpm_view.added": "Добавлено",
  "l.cpm_view.not_in_the_baseline_so":
    "нет в базовом плане, поэтому сдвига нет",
  "l.cpm_view.removed": "Удалено",
  "l.cpm_view.was_in_the_baseline_and":
    "была в базовом плане и удалена; её базовые даты сохраняются",
  "l.cpm_view.cannot_say": "Определить невозможно",
  "l.cpm_view.it_had_no_schedule_when":
    "при фиксации базового плана у неё не было расписания",
  "l.cpm_view.it_has_no_schedule_now": "сейчас у неё нет расписания",
  "l.document_view.version": "Версия {version} — {v}",
  "l.document_view.since": "{base} с {effective_from}",
  "l.document_view.by_version": " версией {version}",
  "l.document_view.on": "{base} {superseded_on}{v}",
  "l.document_view.nothing_in_force_yet": "Пока ничего не действует",
  "l.document_view.version_in_force": "Действует версия {in_force_today}",
  "l.ev_view.nothing_was_planned_to_be":
    "пока ничего не планировалось выполнять",
  "l.ev_view.no_cost_has_been_recorded": "стоимость пока не записана",
  "l.ev_view.the_baseline_plans_no_value":
    "базовый план не предусматривает ценности",
  "l.ev_view.not_available": "недоступно",
  "l.ev_view.schedule_no_ratio": "Расписание: коэффициента нет, {absentText}.",
  "l.ev_view.ahead_of_schedule": "Опережение расписания",
  "l.ev_view.behind_schedule": "Отставание от расписания",
  "l.ev_view.on_schedule": "По расписанию",
  "l.ev_view.spi": "{word}: SPI {ratioText}, {gap}.",
  "l.ev_view.cost_no_ratio": "Стоимость: коэффициента нет, {absentText}.",
  "l.ev_view.under_budget": "Ниже бюджета",
  "l.ev_view.over_budget": "Превышение бюджета",
  "l.ev_view.on_budget": "В рамках бюджета",
  "l.ev_view.earned_schedule_no_ratio":
    "Освоенное расписание: коэффициента нет, {absentText}.",
  "l.ev_view.by_earned_schedule_the_work":
    "По освоенному расписанию работа: {word}; за {days} выполнено то, что по плану ожидалось за {days2} (SPI(t) {ratioText}).",
  "l.ev_view.finished_work_counts_for_its":
    "завершённая работа учитывается по своим story points",
  "l.ev_view.only_finished_work_counts":
    "учитывается только завершённая работа",
  "l.ev_view.by_story_points": "по story points",
  "l.ev_view.counted_by_a_person_entered":
    "{length} учтено по проценту, введённому человеком (вручную)",
  "l.ev_view.with_no_points_carry_no": "{length} без баллов не имеют веса",
  "l.ev_view.no_actual_cost_is_recorded": "фактическая стоимость не записана",
  "l.ev_view.the_ratio_it_divides_by":
    "коэффициент, на который он делится, пока не определён",
  "l.ev_view.no_estimate_to_complete_has": "оценка до завершения не введена",
  "l.ev_view.the_budget_is_already_spent": "бюджет уже потрачен",
  "l.ev_view.no_cost_remains": "стоимости не осталось",
  "l.ev_view.against_the_money_is_already":
    "Относительно {target}: деньги уже потрачены, поэтому никакая эффективность не позволит выполнить оставшуюся работу в их пределах.",
  "l.ev_view.against": "Относительно {target}: {amountAbsentText}.",
  "l.ev_view.against_the_remaining_work_must":
    "Относительно {target}: оставшуюся работу нужно выполнять в {ratioText} раза эффективнее, чем работу до сих пор (TCPI {ratioText2}), для чего нужно повысить эффективность.",
  "l.ev_view.against_the_remaining_work_can":
    "Относительно {target}: оставшуюся работу можно выполнить при {ratioText} от эффективности затрат на работу до сих пор (TCPI {ratioText2}).",
  "l.ev_view.earned": "{date}: освоено {ev_minor}",
  "l.ev_view.spent": "{date}: потрачено {ac_minor}",
  "l.import_view.would_create": "Было бы создано",
  "l.import_view.created": "Создано",
  "l.import_view.and":
    "{verb} {plural}, обновлено {upd} {tasks_updated}{tail}.",
  "l.import_view.preview_the_import": "Предпросмотр импорта",
  "l.import_view.import": "Импортировать {plural}",
  "l.kpi_view.target_or_more_red_at":
    "Цель {target_value} или больше; красный при {red_value} или меньше{u}",
  "l.kpi_view.target_or_less_red_at":
    "Цель {target_value} или меньше; красный при {red_value} или больше{u}",
  "l.kpi_view.no_reading_within_twice_its":
    "Нет показаний в пределах удвоенной периодичности ({cadence})",
  "l.kpi_view.no_reading_yet": "показаний пока нет",
  "l.milestone_view.there_is_no_forecast": "Прогноза нет.",
  "l.milestone_view.some_open_tasks_cannot_be":
    " (некоторые открытые задачи невозможно разместить, поэтому дата может быть слишком ранней)",
  "l.milestone_view.forecast_to_finish_after_it":
    "Прогноз завершения: {forecast}, на {businessDays} позже срока ({due}){partial}.",
  "l.milestone_view.forecast_to_finish_on_the":
    "Прогноз завершения в срок, {due}{partial}.",
  "l.milestone_view.forecast_to_finish_before_it":
    "Прогноз завершения: {forecast}, на {businessDays} раньше срока ({due}){partial}.",
  "l.milestone_view.of_them_later": ", из них позже: {later_moves}",
  "l.milestone_view.back_on_its_original_date": "снова на исходной дате",
  "l.milestone_view.later_than_the_original":
    "на {calendarDays} позже исходной даты {original_due}",
  "l.milestone_view.earlier_than_the_original":
    "на {calendarDays} раньше исходной даты {original_due}",
  "l.milestone_view.originally_due_now":
    "Исходный срок {original_due}; {times}{later}; сейчас {net}.",
  "l.milestone_view.on": " {achieved_on}",
  "l.milestone_view.achieved_every_task_it_waits":
    "Достигнута{on}: выполнена каждая задача, которой она ждёт.",
  "l.milestone_view.reason": " Причина: {done_reason}",
  "l.milestone_view.marked_done": "Отмечена выполненной{on}.{why}",
  "l.milestone_view.due": "{name}: {toLowerCase}, срок {due}{state}",
  "l.my_work_view.this_is_already_late_for":
    "Это уже опаздывает относительно плана (отставание {days}).",
  "l.my_work_view.on_the_critical_path": "На критическом пути",
  "l.my_work_view.of_float": "резерв времени {days}",
  "l.okr_view.not_measured": "Не измерено",
  "l.okr_view.not_measurable": "Измерить невозможно ({reason}).",
  "l.okr_view.of": "{current} из {target}{u}",
  "l.okr_view.confidence_not_part_of_the":
    "Уверенность {confidence}% (не входит в оценку)",
  "l.onboarding_view.give_the_date_it_takes": "Укажите дату вступления в силу.",
  "l.onboarding_view.give_the_person_as_person":
    "Укажите человека в виде person:… (или worker:…).",
  "l.onboarding_view.say_what_access_in_words":
    "Опишите словами, какой доступ.",
  "l.onboarding_view.choose_granted_or_revoked":
    "Выберите: предоставлен или отозван.",
  "l.onboarding_view.give_who_is_handing_over":
    "Укажите, кто передаёт дела, в виде person:…",
  "l.onboarding_view.give_who_takes_over_as":
    "Укажите, кто принимает дела, в виде person:… (или оставьте пустым, чтобы освободить работу).",
  "l.onboarding_view.hand_over_to_someone_else":
    "Передайте дела другому человеку.",
  "l.onboarding_view.no_items": "Нет пунктов",
  "l.onboarding_view.of_done": "Выполнено {done} из {total}",
  "l.onboarding_view.skipped": "пропущено: {skipped}",
  "l.onboarding_view.overdue": "просрочено: {overdue}",
  "l.onboarding_view.skipped_2": "Пропущено",
  "l.onboarding_view.overdue_2": "Просрочено",
  "l.onboarding_view.nothing_is_left_open_this":
    "Ничего не осталось открытым. Этот план готов к закрытию.",
  "l.onboarding_view.not_ready_to_close": "Не готов к закрытию: {join}.",
  "l.onboarding_view.this_person_holds_nothing_live":
    "У этого человека нет ничего действующего в этом плане.",
  "l.raci_view.nobody_is_accountable": "Никто не отвечает за результат",
  "l.raci_view.nobody_is_responsible": "Никто не исполняет",
  "l.raci_view.more_than_one_person_is":
    "За результат отвечает более одного человека",
  "l.raci_view.has_no_role": "{task}: у {who} нет роли",
  "l.raci_view.and": " и ",
  "l.raci_view.is": "{task}: {who} — {join}",
  "l.rbs.a_booking_that_needs_several":
    "Бронирование, требующее нескольких навыков, полностью учитывается для каждого из них, поэтому навыки пересекаются и не дают в сумме общий итог. Предложение и люди для навыков неизвестны.",
  "l.rbs.named": "названо: {length}",
  "l.rbs.supply_not_known": "предложение неизвестно",
  "l.rbs.supply_fte": "предложение {supply_fte} FTE",
  "l.rbs.demand_fte": "{role}: {people}, спрос {demand_fte} FTE, {supply}",
  "l.scheduling_view.pass": "Пройдена",
  "l.scheduling_view.fail": "Не пройдена",
  "l.scheduling_view.not_computable": "Не вычисляется",
  "l.scheduling_view.not_enough_data": "Недостаточно данных.",
  "l.scheduling_view.not_enough_data_2": "Недостаточно данных: {reason}",
  "l.scheduling_view.must_pass_a_business_day":
    "Обязательна: задержка на {v} рабочих дней должна сдвигать завершение",
  "l.scheduling_view.at_least_business_days": "Не менее {limit} рабочих дней",
  "l.scheduling_view.at_most": "не более",
  "l.scheduling_view.at_least": "не менее",
  "l.scheduling_view.no_figure": "Нет значения",
  "l.scheduling_view.finish_moved_of_business_days":
    "Завершение сдвинулось в {count} из {of} рабочих дней",
  "l.scheduling_view.of": "{toFixed} ({count} из {of})",
  "l.scheduling_view.of_2": "{value}% ({count} из {of} {of_what})",
  "l.scheduling_view.passed_failed_not_computable":
    "пройдено: {passed}, не пройдено: {failed}, не вычисляется: {not_computable}",
  "l.scheduling_view.not_resolved": "Не решено.",
  "l.scheduling_view.it_is_on_the_critical":
    " Она на критическом пути, поэтому завершение проекта сдвигается.",
  "l.scheduling_view.starts_later_finishes_instead_of":
    "{pid} ({who}) начинается {from_start} → {scheduled_start}, на {days} позже; завершается {to_finish} вместо {from_finish}.{crit}",
  "l.scheduling_view.to": "с {from} по {to}",
  "l.scheduling_view.is_over_capacity_on":
    "У {who} превышена ёмкость в период {span}.",
  "l.scheduling_view.is_asked_for_on_and":
    "От {who} требуется {demand}% в период {span}, а может быть выделено {capacity}%.",
  "l.scheduling_view.no_project_finish_can_be":
    "Завершение проекта рассчитать невозможно.",
  "l.scheduling_view.the_project_finish_stays":
    "Завершение проекта остаётся {before}.",
  "l.scheduling_view.the_project_finish_moves_from":
    "Завершение проекта сдвигается с {before} на {after} (на {days} позже).",
  "l.scheduling_view.no_demand_or_supply": "Нет ни спроса, ни предложения",
  "l.scheduling_view.demand_fte_supply_not_known":
    "Спрос {demand_fte} FTE; предложение неизвестно",
  "l.scheduling_view.short_fte": "Нехватка {abs} FTE",
  "l.scheduling_view.spare_fte": "Избыток {gap_fte} FTE",
  "l.scheduling_view.balanced": "Баланс",
  "l.scheduling_view.the_critical_path_finish":
    "завершение по критическому пути",
  "l.scheduling_view.of_the_simulated_outcomes_finish":
    "{pct}% смоделированных исходов завершаются {date} или раньше.",
  "l.scheduling_view.no_variation": "Нет разброса",
  "l.scheduling_view.no_estimate_using_the_fallback":
    "Нет оценки: используется запасной диапазон",
  "l.scheduling_view.milestone_no_spread": "Веха: без разброса",
  "l.scheduling_view.estimated_widened_to_include_the":
    "Оценено (расширено, чтобы включить вероятное значение)",
  "l.scheduling_view.estimated": "Оценено",
  "l.scheduling_view.pert_favours_the_likely_value":
    "PERT (отдаёт предпочтение вероятному значению)",
  "l.scheduling_view.of_the_simulated_outcomes_cost":
    "{round}% смоделированных исходов стоят не больше бюджета ({budgetText}).",
  "l.scheduling_view.estimated_widened_to_include_the_2":
    "Оценено (расширено, чтобы включить плановую сумму)",
  "l.scheduling_view.stated_struck_in_of_runs":
    "заявлено {round}%, сработало в {toFixed}% прогонов",
  "l.scheduling_view.within_budget_and_on_time": "В рамках бюджета и в срок",
  "l.scheduling_view.within_budget_but_late":
    "В рамках бюджета, но с опозданием",
  "l.scheduling_view.on_time_but_over_budget":
    "В срок, но с превышением бюджета",
  "l.scheduling_view.over_budget_and_late": "Превышение бюджета и опоздание",
  "l.scheduling_view.both_within_budget_and_on":
    "И в рамках бюджета, и в срок: {pct} (только в рамках бюджета {pct2}; только в срок {pct3}).",
  "l.scheduling_view.a_late_finish_costs_nothing":
    "Опоздание с завершением в этом запуске ничего не стоит дополнительно (стоимость задержки за рабочий день равна 0).",
  "l.scheduling_view.each_business_day_late_adds":
    "Каждый рабочий день опоздания добавляет {format}; досрочное завершение ничего не возвращает.",
  "l.simulation_inputs.the_probability_must_be_a":
    "Вероятность должна быть целым числом от 0 до 100.",
  "l.simulation_inputs.name_the_task_and_the":
    "Укажите задачу и рабочие дни, которые риск добавляет, вместе либо оставьте оба поля пустыми.",
  "l.simulation_inputs.the_business_days_a_risk":
    "Рабочие дни, которые добавляет риск, должны быть целым числом от 1 до 365.",
  "l.simulation_inputs.give_the_cost_and_its":
    "Укажите стоимость и её валюту вместе либо оставьте оба поля пустыми.",
  "l.simulation_inputs.the_cost_must_be_an":
    "Стоимость должна быть суммой больше нуля.",
  "l.simulation_inputs.the_currency_must_be_a":
    "Валюта должна быть трёхбуквенным кодом, например EUR, JPY или GBP.",
  "l.simulation_inputs.a_risk_that_changes_the":
    "Риску, который меняет расписание или стоимость, нужна вероятность.",
  "l.simulation_inputs.amounts_must_be_numbers_zero":
    "Суммы должны быть числами, равными нулю или больше.",
  "l.simulation_inputs.the_low_amount_cannot_be":
    "Нижняя сумма не может быть больше плановой.",
  "l.simulation_inputs.the_high_amount_cannot_be":
    "Верхняя сумма не может быть меньше плановой.",
  "l.simulation_inputs.business_days_must_be_whole":
    "Рабочие дни должны быть целыми числами от 0 до 3650.",
  "l.simulation_inputs.give_the_task_a_duration":
    "Сначала задайте задаче длительность: это её вероятное значение.",
  "l.simulation_inputs.optimistic_cannot_be_above_the":
    "Оптимистичная оценка не может быть больше вероятной длительности ({likely}).",
  "l.simulation_inputs.pessimistic_cannot_be_below_the":
    "Пессимистичная оценка не может быть меньше вероятной длительности ({likely}).",
  "l.simulation_inputs.no_range_the_simulation_uses":
    "Диапазона нет: симуляция использует запасной диапазон",
  "l.simulation_inputs.to": "от {fmt} до {fmt2}",
  "l.wbs.restricted_item": "Элемент с ограниченным доступом",
  "l.whatif_view.choose_a_task": "Выберите задачу.",
  "l.whatif_view.give_the_new_start_date": "Укажите новую дату начала.",
  "l.whatif_view.duration_must_be_a_whole":
    "Длительность должна быть целым числом рабочих дней от 0 до 3650.",
  "l.whatif_view.choose_a_constraint": "Выберите ограничение.",
  "l.whatif_view.a_constraint_needs_a_date": "Ограничению нужна дата.",
  "l.whatif_view.a_hard_constraint_needs_a":
    "Жёсткому ограничению нужна причина.",
  "l.whatif_view.choose_both_tasks": "Выберите обе задачи.",
  "l.whatif_view.a_task_cannot_depend_on":
    "Задача не может зависеть от самой себя.",
  "l.whatif_view.lag_must_be_a_whole":
    "Запаздывание должно быть целым числом рабочих дней (отрицательное означает опережение).",
  "l.whatif_view.choose_a_link": "Выберите связь.",
  "l.whatif_view.start_on": "Начать «{name}» {start_date}",
  "l.whatif_view.make_a_milestone": "Сделать «{name}» вехой",
  "l.whatif_view.for": "{constraint_kind} {constraint_date} для «{name}»",
  "l.whatif_view.clear_the_constraint_on": "Снять ограничение с «{name}»",
  "l.whatif_view.lag": ", запаздывание {lag_days}",
  "l.whatif_view.follows": "«{name}» следует за «{name2}» ({toUpperCase}{v})",
  "l.whatif_view.remove_the_link": "Удалить связь {slice}",
  "l.whatif_view.the_plan_has_no_finish": "У плана пока нет даты завершения.",
  "l.whatif_view.the_finish_stays": "Завершение остаётся {after}.",
  "l.whatif_view.the_finish_becomes": "Завершение становится {after}.",
  "l.whatif_view.the_plan_no_longer_has":
    "У плана больше нет даты завершения (было {before}).",
  "l.whatif_view.the_finish_moves_from_to":
    "Завершение сдвигается с {before} на {after}.",
  "l.whatif_view.cost_impact_is_finance_data":
    "Влияние на стоимость — финансовые данные, поэтому оно вам не показывается.",
  "l.whatif_view.no_task_with_work_has":
    "Ни у одной задачи с работой нет ставки, поэтому сравнивать стоимость нечего.",
  "l.work_view.work_must_be_a_number":
    "Работа должна быть числом часов больше нуля.",
  "l.work_view.duration_must_be_a_whole":
    "Длительность должна быть целым числом рабочих дней от 1 до 3650.",
  "l.work_view.units_must_be_a_whole":
    "Единицы должны быть целым процентом дня от 1 до 1000.",
  "l.work_view.give_the_units_too": "Укажите также единицы.",
  "l.work_view.business_days_at_is":
    "{duration_days} рабочих дней при {units_percent}% — это {hoursText}",
  "l.work_view.the_duration_was_worked_out": "Длительность рассчитана.",
  "l.work_view.the_share_of_the_day": "Доля дня рассчитана.",
  "l.work_view.the_work_was_worked_out": "Объём работы рассчитан.",
  "l.work_view.rounded_the_three_do_not":
    " Округлено: три значения не совпадают с точностью до минуты.",
  "l.work_view.capacity_not_checked_the_task":
    "Ёмкость не проверена: у задачи пока нет дат.",
  "l.work_view.within_the_person_s_capacity": "В пределах ёмкости человека.",
  "l.work_view.give_1_to_40_whole":
    "Укажите от 1 до 40 целых процентов через запятую.",
  "l.work_view.the_percentages_add_up_to":
    "Проценты в сумме дают {sum}, а не 100.",
  "p.home.main_x_plans": "Планы — Main X",
  "p.announcements.could_not_load_the_announcements":
    "Не удалось загрузить объявления",
  "p.announcements.could_not_publish": "Не удалось опубликовать",
  "p.announcements.could_not_retract": "Не удалось отозвать",
  "p.auditor.auditor_ppm": "Аудитор — PPM",
  "p.automations.could_not_load_automations":
    "Не удалось загрузить автоматизации",
  "p.automations.rule_saved_it_fires_from":
    "Правило сохранено — оно сработает со следующего подходящего перехода",
  "p.automations.could_not_save_the_rule": "Не удалось сохранить правило",
  "p.automations.that_action_was_refused": "Это действие отклонено",
  "p.automations.capped_at_run_again_for":
    " (ограничено {cap} — запустите снова для остального)",
  "p.automations.swept_fired_already_claimed":
    "Проверка выполнена: сработало {fired}, уже занято {skipped_already_claimed}{v}",
  "p.automations.the_sweep_failed": "Проверка завершилась сбоем",
  "p.board.board_ppm": "Доска — PPM",
  "p.calendar.calendar_ppm": "Календарь — PPM",
  "p.ceo.could_not_load": "Не удалось загрузить",
  "p.compliance.compliance_ppm": "Соответствие требованиям — PPM",
  "p.cost_rates.could_not_load_the_rate": "Не удалось загрузить таблицы ставок",
  "p.cost_rates.rate_table_added": "Таблица ставок добавлена.",
  "p.cost_rates.could_not_add_the_table": "Не удалось добавить таблицу",
  "p.cost_rates.give_a_date_and_an":
    "Укажите дату и почасовую ставку, равную нулю или больше.",
  "p.cost_rates.rate_added": "Ставка добавлена.",
  "p.cost_rates.could_not_add_the_rate": "Не удалось добавить ставку",
  "p.cost_risk.could_not_run_the_simulation": "Не удалось запустить симуляцию",
  "p.documents.could_not_load_the_documents": "Не удалось загрузить документы",
  "p.documents.could_not_create_the_document": "Не удалось создать документ",
  "p.documents.pid.could_not_load_the_document":
    "Не удалось загрузить документ",
  "p.documents.pid.that_did_not_work": "Не получилось",
  "p.documents.pid.enter_the_approver_s_id":
    "Введите идентификатор утверждающего (идентификатор человека, не ваш).",
  "p.documents.pid.sent_for_review": "Отправлено на проверку.",
  "p.documents.pid.needs_changes": "Требуются изменения",
  "p.documents.pid.approved": "Утверждено.",
  "p.documents.pid.sent_back_to_draft": "Возвращено в черновик.",
  "p.documents.pid.in_force_from_today": "Действует с сегодняшнего дня.",
  "p.documents.pid.next_draft_started": "Начат следующий черновик.",
  "p.documents.pid.could_not_look_that_up": "Не удалось найти",
  "p.engineering.engineering_ppm": "Инженерия — PPM",
  "p.executive.executive_ppm": "Руководство — PPM",
  "p.financials.financials_ppm": "Финансы — PPM",
  "p.gantt.gantt_main_x": "Диаграмма Ганта — Main X",
  "p.gantt.calendar_failed":
    "Не удалось загрузить рабочий календарь, поэтому нерабочие дни не затенены.",
  "p.kpis.could_not_load_the_kpis": "Не удалось загрузить KPI",
  "p.kpis.could_not_define_the_kpi": "Не удалось определить KPI",
  "p.lifecycle.could_not_load_the_funnel": "Не удалось загрузить воронку",
  "p.me.could_not_load_your_tasks": "Не удалось загрузить ваши задачи",
  "p.me.calendar.could_not_load_your_calendar":
    "Не удалось загрузить ваш календарь",
  "p.me.calendar.could_not_save": "Не удалось сохранить",
  "p.me.calendar.could_not_remove": "Не удалось удалить",
  "p.milestones.could_not_load_the_milestones": "Не удалось загрузить вехи",
  "p.objectives.create_failed": "не удалось создать",
  "p.objectives.alignment_failed": "не удалось согласовать",
  "p.okrs.could_not_load_the_okrs": "Не удалось загрузить OKR",
  "p.okrs.could_not_create_the_objective": "Не удалось создать цель",
  "p.plans.plans_main_x": "Планы — Main X",
  "p.plans.pid.main_x": "Страница «{v}» — Main X",
  "p.plans.pid.board.pt": " · {points} очк.",
  "p.plans.pid.board.board_ppm": "Доска — PPM",
  "p.plans.pid.calendar.could_not_load_the_calendar":
    "Не удалось загрузить календарь",
  "p.plans.pid.calendar.could_not_save": "Не удалось сохранить",
  "p.plans.pid.cost_risk.could_not_run_the_simulation":
    "Не удалось запустить симуляцию",
  "p.plans.pid.costs.could_not_load_the_costs": "Не удалось загрузить затраты",
  "p.plans.pid.costs.choose_a_task": "Выберите задачу.",
  "p.plans.pid.costs.the_amount_must_be_above":
    "Сумма должна быть больше нуля.",
  "p.plans.pid.costs.the_currency_must_be_a":
    "Валюта должна быть трёхбуквенным кодом, например EUR, JPY или GBP.",
  "p.money.currency_hint": "например, EUR, JPY или GBP",
  "p.money.amount_or_currency":
    "Укажите сумму числом, а валюту — трёхбуквенным кодом, например EUR, JPY или GBP.",
  "p.plans.pid.costs.expense_added": "Расход добавлен.",
  "p.plans.pid.costs.could_not_add_the_expense": "Не удалось добавить расход",
  "p.plans.pid.costs.could_not_record": "Не удалось записать",
  "p.plans.pid.critical_path.could_not_load_the_baselines":
    "Не удалось загрузить базовые планы",
  "p.plans.pid.critical_path.could_not_take_the_baseline":
    "Не удалось зафиксировать базовый план",
  "p.plans.pid.critical_path.could_not_load_the_schedule":
    "Не удалось загрузить расписание",
  "p.plans.pid.critical_path.could_not_save_the_estimate":
    "Не удалось сохранить оценку",
  "p.plans.pid.critical_path.could_not_save": "Не удалось сохранить",
  "p.plans.pid.earned_value.could_not_load_earned_value":
    "Не удалось загрузить освоенный объём",
  "p.plans.pid.earned_value.could_not_save": "Не удалось сохранить",
  "p.plans.pid.earned_value.could_not_record": "Не удалось записать",
  "p.plans.pid.edit.edit_case_main_x": "Изменение обоснования — Main X",
  "p.plans.pid.flow.could_not_load_flow_data":
    "Не удалось загрузить данные потока.",
  "p.plans.pid.handover.handover_is_about_people_so":
    "Передача дел касается людей, поэтому нужно разрешение на управление людьми.",
  "p.plans.pid.handover.could_not_preview": "Не удалось выполнить предпросмотр",
  "p.plans.pid.handover.what_this_person_holds_changed":
    "То, чем занимается этот человек, изменилось после предпросмотра, поэтому ничего не изменено. Выполните предпросмотр снова, затем зафиксируйте.",
  "p.plans.pid.handover.could_not_commit": "Не удалось зафиксировать",
  "p.plans.pid.handover.discarded_nothing_was_changed":
    "Отброшено. Ничего не изменено.",
  "p.plans.pid.import.choose_the_file_first": "Сначала выберите файл.",
  "p.plans.pid.import.the_import_did_not_work": "Импорт не удался",
  "p.plans.pid.joint_risk.could_not_run_the_simulation":
    "Не удалось запустить симуляцию",
  "p.plans.pid.leveling.could_not_propose_a_leveling":
    "Не удалось предложить выравнивание",
  "p.plans.pid.leveling.could_not_apply_the_moves":
    "Не удалось применить переносы",
  "p.plans.pid.onboarding.could_not_load": "Не удалось загрузить",
  "p.plans.pid.onboarding.could_not_open_the_checklist":
    "Не удалось открыть контрольный список",
  "p.plans.pid.onboarding.checklist_started": "Контрольный список начат.",
  "p.plans.pid.onboarding.could_not_start": "Не удалось начать",
  "p.plans.pid.onboarding.could_not_save": "Не удалось сохранить",
  "p.plans.pid.onboarding.checklist_completed": "Контрольный список завершён.",
  "p.plans.pid.onboarding.could_not_complete": "Не удалось завершить",
  "p.plans.pid.onboarding.say_why_you_are_cancelling":
    "Укажите, почему вы отменяете.",
  "p.plans.pid.onboarding.checklist_cancelled": "Контрольный список отменён.",
  "p.plans.pid.onboarding.could_not_cancel": "Не удалось отменить",
  "p.plans.pid.onboarding.could_not_record": "Не удалось записать",
  "p.plans.pid.raci.could_not_load_who_does":
    "Не удалось загрузить, кто что делает",
  "p.plans.pid.raci.could_not_save": "Не удалось сохранить",
  "p.plans.pid.raci.use_a_person_or_worker":
    "Используйте ссылку person: или worker: с UUID.",
  "p.plans.pid.schedule_health.could_not_run_the_schedule":
    "Не удалось выполнить проверки состояния расписания",
  "p.plans.pid.schedule_risk.could_not_run_the_simulation":
    "Не удалось запустить симуляцию",
  "p.plans.pid.wbs.could_not_load_the_tasks": "Не удалось загрузить задачи",
  "p.plans.pid.wbs.link_counts_failed":
    "Не удалось загрузить количество связей у каждой задачи, поэтому количества не показываются.",
  "p.plans.pid.what_if.could_not_load_the_plan": "Не удалось загрузить план",
  "p.plans.pid.what_if.at_most_edits_at_a":
    "Не более {MAX_EDITS} изменений за раз.",
  "p.plans.pid.what_if.edit_staged_preview_to_see":
    "Изменение подготовлено. Выполните предпросмотр, чтобы увидеть результат.",
  "p.plans.pid.what_if.discarded_nothing_was_changed":
    "Отброшено. Ничего не изменено.",
  "p.plans.pid.what_if.could_not_preview": "Не удалось выполнить предпросмотр",
  "p.plans.pid.what_if.the_plan_changed_since_this":
    "План изменился после этого предпросмотра, поэтому ничего не применено. Выполните предпросмотр снова, затем зафиксируйте.",
  "p.plans.pid.what_if.could_not_commit": "Не удалось зафиксировать",
  "p.plans.pid.what_if.to": "с {early_start} по {early_finish}",
  "p.plans.pid.what_if.not_scheduled": "не запланировано",
  "p.plans.pid.work.could_not_load_the_plan": "Не удалось загрузить план",
  "p.plans.pid.work.could_not_load_the_work":
    "Не удалось загрузить профиль работы",
  "p.plans.pid.work.could_not_save": "Не удалось сохранить",
  "p.plans.pid.work.contour_saved": "Профиль сохранён.",
  "p.plans.merge.merge_plans_ppm": "Объединение планов — PPM",
  "p.plans.new.new_case_main_x": "Новое обоснование — Main X",
  "p.prioritization.could_not_load_the_queue": "Не удалось загрузить очередь",
  "p.prioritization.could_not_load_the_score": "Не удалось загрузить оценку",
  "p.proposals.duplicate_check_failed": "не удалось проверить на дубликаты",
  "p.regulator.regulator_ppm": "Регулятор — PPM",
  "p.reports.save_failed": "не удалось сохранить",
  "p.reports.run_failed": "не удалось выполнить",
  "p.resources.could_not_load_the_resource":
    "Не удалось загрузить план ресурсов",
  "p.resources.rbs.could_not_load_resources": "Не удалось загрузить ресурсы",
  "p.reviews.could_not_load_reviews": "Не удалось загрузить рецензии",
  "p.reviews.invitation_sent": "Приглашение отправлено",
  "p.reviews.could_not_delegate": "Не удалось делегировать",
  "p.reviews.score_0_100_leave_blank":
    "Оценка от 0 до 100 (пусто — без оценки)",
  "p.reviews.score_must_be_between_0": "Оценка должна быть от 0 до 100",
  "p.reviews.that_action_was_refused": "Это действие отклонено",
  "p.reviews.could_not_load_consensus": "Не удалось загрузить консенсус",
  "p.risk.risk_ppm": "Риск — PPM",
  "p.scenarios.evaluate_failed": "не удалось оценить",
  "p.security.security_ppm": "Безопасность — PPM",
  "p.technology.technology_ppm": "Технологии — PPM",
  "p.tour.tour_main_x_plans": "Обзор · Main X Plans",
  "p.verify.this_sign_in_link_is": "В этой ссылке для входа отсутствует токен.",
  "p.verify.we_could_not_reach_the":
    "Не удалось связаться со службой входа. Повторите попытку через минуту.",
  "p.verify.this_sign_in_link_is_2":
    "Эта ссылка для входа недействительна или срок её действия истёк.",
  "l.ceo_view.gate_review": "Шлюзовая проверка",
  "l.ceo_view.scenario_committed": "Сценарий закреплён",
  "l.ceo_view.proposal": "Предложение",
  "l.ceo_view.plans_merged": "Планы объединены",
  "c.CumulativeFlow.in_review": "На проверке",
  "c.CumulativeFlow.in_progress": "В работе",
  "c.CumulativeFlow.todo": "К выполнению",
  "l.document_view.draft": "Черновик",
  "l.document_view.in_review": "На проверке",
  "l.document_view.approved_not_yet_in_force": "Утверждено, ещё не действует",
  "l.document_view.replaced": "Заменено",
  "l.document_view.send_for_review": "Отправить на проверку",
  "l.document_view.approve": "Утвердить",
  "l.document_view.bring_into_force_today": "Ввести в действие сегодня",
  "l.document_view.start_the_next_draft": "Начать следующий черновик",
  "l.document_view.waiting_for_the_approver": "Ожидается утверждающий",
  "l.document_view.approved": "Утверждено",
  "l.document_view.change_this_document": "Изменить этот документ",
  "l.document_view.someone_else_approves_it_you":
    "Утверждает кто-то другой: вы не можете утвердить собственный документ.",
  "l.document_view.only_the_person_asked_can":
    "Решение может принять только тот, кого попросили.",
  "l.document_view.it_comes_into_force_today":
    "Он вступает в силу сегодня и заменяет действующую сейчас версию.",
  "l.document_view.the_next_draft_starts_as":
    "Следующий черновик начинается как копия последнего текста.",
  "l.ev_view.on_track": "по плану",
  "l.ev_view.off_plan": "вне плана",
  "l.ev_view.not_yet_known": "пока неизвестно",
  "l.glossary.schedule_performance_index_work_done":
    "Индекс выполнения расписания: выполненная работа по сравнению с работой, запланированной к текущему моменту. Значение выше 1,00 означает опережение плана.",
  "l.glossary.cost_performance_index_value_of":
    "Индекс выполнения стоимости: ценность выполненной работы по сравнению с её стоимостью. Значение выше 1,00 означает, что затраты ниже бюджета.",
  "l.glossary.schedule_index_by_time_how":
    "Индекс расписания по времени: насколько мы продвинулись по плану во времени; остаётся достоверным, даже когда проект завершается с опозданием.",
  "l.glossary.earned_value_the_budget_for":
    "Освоенный объём: бюджет фактически завершённой работы.",
  "l.glossary.planned_value_the_budget_for":
    "Плановый объём: бюджет работы, которая должна быть завершена к текущему моменту.",
  "l.glossary.actual_cost_what_has_really":
    "Фактическая стоимость: сколько действительно потрачено.",
  "l.glossary.estimate_at_completion_the_expected":
    "Оценка по завершении: ожидаемая итоговая стоимость.",
  "l.glossary.estimate_to_complete_what_is":
    "Оценка до завершения: сколько ещё предполагается потратить.",
  "l.glossary.variance_at_completion_the_expected":
    "Отклонение по завершении: ожидаемая итоговая стоимость относительно бюджета.",
  "l.glossary.red_amber_or_green_a":
    "Красный, жёлтый или зелёный: индикатор статуса.",
  "l.glossary.responsible_accountable_consulted_in":
    "Исполнитель, ответственный за результат, консультант, информируемый: кто выполняет работу, кто принимает её, кого спрашивают и кого информируют.",
  "l.glossary.a_way_of_weighting_the":
    "Способ придать вероятной оценке больший вес, чем лучшему и худшему случаям.",
  "l.glossary.key_performance_indicator_a_measure":
    "Ключевой показатель эффективности: показатель, который отслеживается во времени относительно цели.",
  "l.glossary.key_performance_indicators_measures_":
    "Ключевые показатели эффективности: показатели, которые отслеживаются во времени относительно цели.",
  "l.glossary.objective_and_key_results_a":
    "Цель и ключевые результаты: цель с несколькими измеримыми результатами.",
  "l.glossary.objectives_and_key_results_goals":
    "Цели и ключевые результаты: цели с несколькими измеримыми результатами.",
  "l.import_view.jira_csv_export": "Jira (экспорт CSV)",
  "l.import_view.asana_csv_export": "Asana (экспорт CSV)",
  "l.import_view.microsoft_project_xml_file": "Microsoft Project (файл XML)",
  "l.import_view.columns_not_imported": "Столбцы не импортированы",
  "l.import_view.rows_skipped": "Строки пропущены",
  "l.import_view.links_not_imported": "Связи не импортированы",
  "l.import_view.parents_not_used": "Родители не использованы",
  "l.import_view.dates_not_understood": "Даты не распознаны",
  "l.import_view.fields_shortened": "Поля сокращены",
  "l.import_view.status_notes": "Примечания к статусам",
  "l.import_view.resources_not_matched_to_people":
    "Ресурсы не сопоставлены с людьми",
  "l.import_view.costs_not_imported": "Затраты не импортированы",
  "l.import_view.schedules_not_imported": "Расписания не импортированы",
  "l.import_view.constraints_not_imported": "Ограничения не импортированы",
  "l.import_view.calendar_notes": "Примечания к календарю",
  "l.kpi_view.improving": "Улучшается",
  "l.kpi_view.getting_worse": "Ухудшается",
  "l.kpi_view.no_change": "Без изменений",
  "l.milestone_view.on_track": "По плану",
  "l.milestone_view.at_risk": "Под риском",
  "l.milestone_view.forecast_late": "Прогноз: опоздание",
  "l.milestone_view.no_forecast": "Нет прогноза",
  "l.milestone_view.it_is_not_linked_to":
    "Она не связана ни с одной задачей, поэтому прогнозировать не из чего.",
  "l.milestone_view.its_open_tasks_have_no":
    "У её открытых задач нет дат, которые расписание могло бы разместить (задайте им длительность и начало или предшественника).",
  "l.milestone_view.the_plan_s_dependencies_contain":
    "Зависимости плана содержат цикл, поэтому прогноз построить невозможно.",
  "l.okr_view.not_measurable_the_start_and":
    "Измерить невозможно: начальное значение и цель совпадают, поэтому нет расстояния, которое нужно пройти.",
  "l.okr_view.not_measurable_a_maintain_key":
    "Измерить невозможно: ключевому результату «поддерживать» нужен диапазон допуска.",
  "l.okr_view.not_measurable_the_numbers_are":
    "Измерить невозможно: числа слишком велики для оценки.",
  "l.onboarding_view.someone_joins_this_plan":
    "Кто-то присоединяется к этому плану",
  "l.onboarding_view.someone_leaves_this_plan": "Кто-то покидает этот план",
  "l.onboarding_view.take_this_plan_in": "Принять этот план",
  "l.onboarding_view.close_this_plan_out": "Закрыть этот план",
  "l.onboarding_view.booking": "Бронирование",
  "l.onboarding.executive": "Руководитель",
  "l.onboarding.outcomes_and_exposure_across_the":
    "Результаты и подверженность по всей совокупности, а не по одному плану.",
  "l.onboarding.read_the_executive_summary":
    "Прочитайте краткое резюме для руководства",
  "l.onboarding.health_decisions_benefits_realized_a":
    "Состояние, решения, полученные выгоды и стратегическое соответствие в одном представлении.",
  "l.onboarding.open_the_at_a_glance": "Откройте обзорную панель",
  "l.onboarding.the_whole_estate_s_rag":
    "Статус RAG всей совокупности и нарушения расписания одновременно.",
  "l.onboarding.review_the_board_pack":
    "Просмотрите пакет для совета директоров",
  "l.onboarding.the_pack_a_board_meeting":
    "Пакет, который действительно вручили бы заседанию совета директоров: инвестиции и тенденции.",
  "l.onboarding.check_budget_variance_and_exposure":
    "Проверьте отклонение бюджета и подверженность",
  "l.onboarding.where_spend_is_diverging_from":
    "Где расходы расходятся с планом, по каждой валюте — валюты никогда не суммируются.",
  "l.onboarding.compare_two_candidate_portfolios":
    "Сравните два кандидата в портфели",
  "l.onboarding.what_if_scenarios_side_by":
    "Сценарии «что, если» рядом друг с другом, прежде чем направлять финансирование в любой из них.",
  "l.onboarding.see_what_s_ranked_highest":
    "Посмотрите, что стоит выше всего, и почему",
  "l.onboarding.the_smart_score_queue_every":
    "Очередь умной оценки — каждая оценка показывает подтверждающие данные или сообщает, что их нет.",
  "l.onboarding.pmo_portfolio_manager": "PMO / менеджер портфеля",
  "l.onboarding.intake_governance_and_the_schedule":
    "Приём работ, управление и расписание по всем планам.",
  "l.onboarding.open_the_intake_pipeline": "Откройте конвейер приёма работ",
  "l.onboarding.every_proposal_s_stage_from":
    "Стадия каждого предложения, от черновика до утверждения.",
  "l.onboarding.register_a_plan": "Зарегистрируйте план",
  "l.onboarding.the_first_plan_you_create":
    "Первый созданный вами план — хороший способ увидеть всю структуру данных.",
  "l.onboarding.browse_the_plan_registry": "Просмотрите реестр планов",
  "l.onboarding.one_recursive_collection_kind_is":
    "Одна рекурсивная коллекция — `kind` — это метка, а не ограничение.",
  "l.onboarding.open_a_plan_s_governance": "Откройте вкладку управления плана",
  "l.onboarding.gate_reviews_risks_and_budget":
    "Шлюзовые проверки, риски и статьи бюджета для одного этого плана.",
  "l.onboarding.look_at_the_cross_plan": "Посмотрите расписание по всем планам",
  "l.onboarding.dependencies_the_critical_path_and":
    "Зависимости, критический путь и любой отстающий последователь.",
  "l.onboarding.check_the_risk_register": "Проверьте реестр рисков",
  "l.onboarding.everything_open_or_mitigating_ranked":
    "Всё открытое или снижаемое, ранжированное по подверженности.",
  "l.onboarding.review_the_compliance_register":
    "Просмотрите реестр соответствия требованиям",
  "l.onboarding.standing_findings_across_the_estate":
    "Действующие замечания по всей совокупности, а не только по одному плану.",
  "l.onboarding.run_a_saved_report": "Запустите сохранённый отчёт",
  "l.onboarding.a_filter_field_projection_saved":
    "Фильтр и выбор полей, сохранённые для повторного использования и запускаемые по требованию.",
  "l.onboarding.resource_manager": "Менеджер ресурсов",
  "l.onboarding.who_is_allocated_where_and":
    "Кто где назначен и как на самом деле идёт поставка.",
  "l.onboarding.check_the_capacity_rollup": "Проверьте сводку ёмкости",
  "l.onboarding.who_is_over_allocated_this":
    "Кто перегружен в этом окне, с суммированием по всем планам, в которых он участвует.",
  "l.onboarding.open_a_plan_s_task": "Откройте доску задач плана",
  "l.onboarding.the_kanban_board_that_task":
    "Доска Канбан, по которой на самом деле движется работа на уровне задач.",
  "l.onboarding.read_that_plan_s_flow":
    "Прочитайте показатели потока этого плана",
  "l.onboarding.cycle_time_lead_time_and":
    "Время цикла, время выполнения и эффективность потока — вычисляются из собственной истории доски.",
  "l.onboarding.scan_blocked_items_and_the":
    "Просмотрите заблокированные элементы и состав поставки",
  "l.onboarding.what_s_stuck_the_moscow":
    "Что застряло, состав MoSCoW и календарь вех в одном представлении.",
  "l.onboarding.look_at_the_milestone_calendar": "Посмотрите календарь вех",
  "l.onboarding.every_milestone_across_every_plan":
    "Каждая веха по каждому плану, включая сроки.",
  "l.onboarding.see_what_automations_are_configured":
    "Посмотрите, какие автоматизации настроены",
  "l.onboarding.what_fires_on_a_field":
    "Что срабатывает при изменении поля и что оно делает, когда срабатывает.",
  "l.onboarding.open_collaborative_reviews": "Откройте совместные рецензии",
  "l.onboarding.where_a_plan_gets_a":
    "Где план получает второе мнение до принятия по нему решения.",
  "l.raci_view.responsible": "Исполнитель",
  "l.raci_view.accountable": "Ответственный за результат",
  "l.raci_view.consulted": "Консультант",
  "l.raci_view.informed": "Информируемый",
  "l.raci_view.does_the_work": "выполняет работу",
  "l.raci_view.answers_for_it_and_signs":
    "отвечает за неё и принимает её (один на задачу)",
  "l.raci_view.is_asked_before": "спрашивается до этого",
  "l.raci_view.is_told_after": "информируется после этого",
  "l.scheduling_view.no_schedule_baseline_has_been":
    "Для этого плана не зафиксирован базовый план расписания.",
  "l.scheduling_view.nothing_was_baselined_to_finish":
    "К дате статуса по базовому плану ничего не должно было завершиться.",
  "l.scheduling_view.the_plan_s_dependencies_contain":
    "Зависимости плана содержат цикл.",
  "l.scheduling_view.there_are_no_incomplete_tasks":
    "Нет незавершённых задач для измерения.",
  "l.scheduling_view.there_are_no_relationships_between":
    "Нет связей между задачами для измерения.",
  "l.scheduling_view.no_incomplete_task_has_dates":
    "Ни у одной незавершённой задачи нет дат, которые расписание могло бы разместить.",
  "l.scheduling_view.no_incomplete_task_has_a":
    "Ни у одной незавершённой задачи нет длительности.",
  "l.scheduling_view.the_plan_has_no_tasks": "В плане нет задач.",
  "l.scheduling_view.there_is_no_critical_task":
    "Нет критической задачи для проверки.",
  "l.scheduling_view.no_path_remains_to_measure":
    "Не осталось путей для измерения.",
  "l.scheduling_view.every_task_involved_has_already":
    "Каждая задействованная задача уже началась, поэтому ни одну нельзя сдвигать.",
  "l.scheduling_view.the_tasks_involved_are_critical":
    "Задействованные задачи критические; сдвиг одной из них сдвинул бы завершение. Разрешите критические задачи, чтобы увидеть эту стоимость.",
  "l.scheduling_view.delaying_a_task_enough_would":
    "Достаточная задержка задачи нарушила бы расписание (отрицательный резерв времени или более позднее завершение).",
  "l.scheduling_view.a_task_involved_has_a":
    "У задействованной задачи жёсткое ограничение, и её нельзя сдвинуть.",
  "l.scheduling_view.the_schedule_would_not_let":
    "Расписание не позволит задаче начаться позже.",
  "l.scheduling_view.the_search_stopped_at_its":
    "Поиск остановился на своём пределе, не разрешив этот вопрос.",
  "l.scheduling_view.no_probability": "нет вероятности",
  "l.scheduling_view.no_task_named": "задача не названа",
  "l.scheduling_view.no_impact_in_business_days": "нет влияния в рабочих днях",
  "l.scheduling_view.the_risk_is_not_open": "риск не открыт",
  "l.scheduling_view.its_task_is_not_on": "её задачи нет в расписании",
  "l.scheduling_view.the_plan_has_no_budget":
    "у плана нет статьи бюджета в её валюте",
  "l.whatif_view.move_a_task_set_its": "Переместить задачу (задать её начало)",
  "l.whatif_view.change_a_task_s_duration": "Изменить длительность задачи",
  "l.whatif_view.change_a_task_s_constraint": "Изменить ограничение задачи",
  "l.whatif_view.add_a_link": "Добавить связь",
  "l.whatif_view.remove_a_link": "Удалить связь",
  "l.whatif_view.must_start_on_hard": "Должна начаться в (жёсткое)",
  "l.whatif_view.must_finish_on_hard": "Должна завершиться в (жёсткое)",
  "l.work_view.fixed_units": "Фиксированные единицы",
  "l.work_view.fixed_duration": "Фиксированная длительность",
  "l.work_view.fixed_work": "Фиксированный объём работы",
  "l.work_view.changing_the_work_changes_how":
    "Изменение объёма работы меняет длительность; доля дня остаётся.",
  "l.work_view.changing_the_work_changes_the":
    "Изменение объёма работы меняет долю дня; длительность остаётся.",
  "l.work_view.changing_the_duration_or_the":
    "Изменение длительности или доли дня меняет другой показатель; объём работы остаётся.",
  "l.work_view.flat_the_same_each_day": "Равномерно: одинаково каждый день",
  "l.work_view.front_loaded_more_at_the": "С опережением: больше в начале",
  "l.work_view.back_loaded_more_at_the": "С отставанием: больше в конце",
  "l.work_view.bell_most_in_the_middle": "Колокол: больше всего в середине",
  "l.work_view.turtle_ramps_up_holds_ramps":
    "Черепаха: нарастание, удержание, спад",
  "l.work_view.custom_percentages": "Свои проценты",
  "l.work_view.has_no_dates_yet": "дат пока нет",
  "l.work_view.has_no_business_day_to":
    "нет рабочего дня, на который её можно поместить",
  "l.work_view.has_a_contour_that_gives": "профиль не даёт ей веса",
  "l.work_view.no_rate_applies": "ставка не применяется",
  "l.work_view.no_rate_applied_on_some":
    "в некоторые дни ставка не применялась",
  "l.work_view.has_nobody_assigned": "никто не назначен",
  "p._layout_svelte.reviews": "Рецензии",
  "p._layout_svelte.automations": "Автоматизации",
  "p._layout_svelte.goals_okrs": "Цели (OKR)",
  "p._layout_svelte.onboarding": "Ввод в работу",
  "p.ceo.red": "Красный",
  "p.ceo.amber": "Жёлтый",
  "p.ceo.green": "Зелёный",
  "p.display.saved_on_this_device": "Сохранено на этом устройстве.",
  "p.display.saved_to_your_account_so":
    "Сохранено в вашей учётной записи, поэтому действует и в других браузерах.",
  "p.display.could_not_save_to_your":
    "Сейчас не удалось сохранить в вашей учётной записи; значение хранится на этом устройстве.",
  "p.me.today": "Сегодня",
  "p.me.this_week": "На этой неделе",
  "p.me.later": "Позже",
  "p.me.no_due_date": "Без срока",
  "p.plans.pid.leveling": "Выравнивание",
  "p.plans.pid.cost_and_schedule_risk": "Риск стоимости и сроков",
  "p.plans.pid.board.todo": "К выполнению",
  "p.plans.pid.board.in_progress": "В работе",
  "p.plans.pid.board.in_review": "На проверке",
  "p.common.business_day_one": "{n} рабочий день",
  "p.common.business_day_other": "{n} рабочих дней",
  "p.common.calendar_day_one": "{n} календарный день",
  "p.common.calendar_day_other": "{n} календарных дней",
  "l.milestone_view.rescheduled_one": "перенесена {count} раз",
  "l.milestone_view.rescheduled_other": "перенесена раз: {count}",
  "l.milestone_view.open_of_linked_one":
    "{open_tasks} из {linked_tasks} задачи ещё открыта",
  "l.milestone_view.open_of_linked_other":
    "{open_tasks} из {linked_tasks} задач ещё открыто",
  "l.cpm_view.restricted_one":
    "{n} зависимость от плана, который вы не видите, не учтена в этих датах.",
  "l.cpm_view.restricted_other":
    "Зависимости от плана, который вы не видите ({n}), не учтены в этих датах.",
  "l.cpm_view.monday": "Понедельник",
  "l.cpm_view.tuesday": "Вторник",
  "l.cpm_view.wednesday": "Среда",
  "l.cpm_view.thursday": "Четверг",
  "l.cpm_view.friday": "Пятница",
  "l.cpm_view.saturday": "Суббота",
  "l.cpm_view.sunday": "Воскресенье",
  "l.ev_view.gap_more":
    "{money} — освоено больше ценности, чем планировалось к текущему моменту",
  "l.ev_view.gap_less":
    "{money} — освоено меньше ценности, чем планировалось к текущему моменту",
  "l.ev_view.cost_line_less":
    "{word}: CPI {ratio}, потрачено на {money} меньше, чем освоила работа.",
  "l.ev_view.cost_line_more":
    "{word}: CPI {ratio}, потрачено на {money} больше, чем освоила работа.",
  "l.ev_view.tasks_counted_one":
    "Учтена {count} задача, вес: {weighting}; {how}",
  "l.ev_view.tasks_counted_other":
    "Учтено задач: {count}, вес: {weighting}; {how}",
  "l.ev_view.equally": "поровну",
  "l.onboarding_view.would_move_one": "{n} элемент будет перемещён: {to}.",
  "l.onboarding_view.would_move_other":
    "Элементов, которые будут перемещены: {n}. Куда: {to}.",
  "l.onboarding_view.would_release_one":
    "{n} элемент будет освобождён: задачи и риски без назначения, бронирования и роли удалены.",
  "l.onboarding_view.would_release_other":
    "Элементов, которые будут освобождены: {n}. Задачи и риски без назначения, бронирования и роли удалены.",
  "l.whatif_view.set_duration_one":
    "Установить для «{name}» {duration_days} рабочий день",
  "l.whatif_view.set_duration_other":
    "Установить для «{name}» {duration_days} рабочих дней",
  "l.work_view.over_capacity_one":
    "Превышение ёмкости на {days_over} календарный день (с {from} по {to}): это требует больше, чем человек может дать.",
  "l.work_view.over_capacity_other":
    "Превышение ёмкости на {days_over} календарных дней (с {from} по {to}): это требует больше, чем человек может дать.",
  "p.plans.pid.handover.done_one": "Готово: передан и записан {count} элемент.",
  "p.plans.pid.handover.done_other":
    "Готово: передано и записано элементов: {count}.",
  "p.plans.pid.what_if.committed_one":
    "Зафиксировано: записано {count} изменение.",
  "p.plans.pid.what_if.committed_other":
    "Зафиксировано: записано изменений: {count}.",
  "l.import_view.mspdi_hint":
    "В Microsoft Project выберите «Файл» → «Сохранить как» → «Формат XML». Файл .mpp прочитать невозможно.",
  "c.CalendarMonth.january": "Январь",
  "c.CalendarMonth.february": "Февраль",
  "c.CalendarMonth.march": "Март",
  "c.CalendarMonth.april": "Апрель",
  "c.CalendarMonth.may": "Май",
  "c.CalendarMonth.june": "Июнь",
  "c.CalendarMonth.july": "Июль",
  "c.CalendarMonth.august": "Август",
  "c.CalendarMonth.september": "Сентябрь",
  "c.CalendarMonth.october": "Октябрь",
  "c.CalendarMonth.november": "Ноябрь",
  "c.CalendarMonth.december": "Декабрь",
  "c.LilyGanttTable.schedule": "Расписание",
  "c.LilyKanbanTable.board": "Доска",
  "l.announcement_view.yesterday": "вчера",
  "c.CalendarMonth.holiday_state": "праздник",
  "c.CalendarMonth.absent_state": "отсутствует",
  "l.ev_view.word_ahead": "с опережением",
  "l.ev_view.word_behind": "с отставанием",
  "l.ev_view.slipping": "отставание растёт",
  "l.import_view.task_one": "{n} задача",
  "l.import_view.task_other": "{n} задач",
  "l.import_view.dependency_one": "{n} зависимость",
  "l.import_view.dependency_other": "{n} зависимостей",
  "l.import_view.word_update": "обновить",
  "l.import_view.word_updated": "обновлено",
  "l.rbs.headcount_one": "{headcount} человек",
  "l.rbs.headcount_other": "{headcount} человек",
  "l.scheduling_view.exactly": "ровно",
  "l.scheduling_view.strength_strong": "сильная",
  "l.scheduling_view.strength_moderate": "умеренная",
  "l.scheduling_view.strength_weak": "слабая",
  "l.simulation_inputs.planned_word": "запланировано",
  "p.plans.pid.board.unassigned": "не назначено",
  "p.plans.pid.costs.recorded_one":
    "Записана {recorded} запись учёта времени; ранее уже записано: {already_recorded}; без ставки: {length}.",
  "p.plans.pid.costs.recorded_other":
    "Записано записей учёта времени: {recorded}; ранее уже записано: {already_recorded}; без ставки: {length}.",
  "c.CostRiskView.histogram_label":
    "Гистограмма смоделированной стоимости в {currency} с отметками P50, P80, P90 и бюджета",
  "c.CumulativeFlow.chart_label":
    "Кумулятивный поток за {days} календарных дн., максимум — {peak} задач",
  "p.plans.pid.onboarding.owner_of": "Ответственный за «{title}»",
  "p.plans.pid.onboarding.due_date_of": "Срок для «{title}»",
  "p.plans.pid.onboarding.why_skip_title": "Почему пропустить «{title}»",
  "p.plans.pid.what_if.remove_edit": "Удалить изменение {n}",
  "c.LilyGanttTable.cell_active": "{name}: с {start} по {end}{ends}",
  "c.LilyGanttTable.ends_here": " (здесь заканчивается)",
  "c.LilyGanttTable.cell_not_scheduled": "{name}: не запланировано",
  "l.simulation_inputs.range_business_days":
    "{optimistic} / {likely} / {pessimistic} рабочих дн. (оптимистично / вероятно / пессимистично)",
  "l.enum.task_status.todo": "к выполнению",
  "l.enum.task_status.in_progress": "в работе",
  "l.enum.task_status.in_review": "на проверке",
  "l.enum.task_status.done": "выполнено",
  "l.enum.task_status.blocked": "заблокировано",
  "l.enum.kind.Portfolio": "Портфель",
  "l.enum.kind.Project": "Проект",
  "l.enum.kind.Product": "Продукт",
  "l.enum.kind.Program": "Программа",
  "l.enum.kind.Practice": "Практика",
  "l.enum.kind.Process": "Процесс",
  "l.enum.kind.Purpose": "Назначение",
  "l.enum.kind.Pathway": "Путь",
  "l.enum.kind.Proposal": "Предложение",
  "l.enum.plan_status.Proposed": "Предложен",
  "l.enum.plan_status.Active": "Активен",
  "l.enum.plan_status.OnHold": "Приостановлен",
  "l.enum.plan_status.Completed": "Завершён",
  "l.enum.plan_status.Cancelled": "Отменён",
  "l.enum.collection.portfolios": "портфели",
  "l.enum.collection.projects": "проекты",
  "l.enum.collection.products": "продукты",
  "l.enum.collection.programs": "программы",
  "l.enum.collection.practices": "практики",
  "l.enum.collection.processes": "процессы",
  "l.enum.collection.purposes": "назначения",
  "l.enum.collection.pathways": "пути",
  "l.enum.proposal_status.draft": "черновик",
  "l.enum.proposal_status.submitted": "подано",
  "l.enum.proposal_status.in_review": "на рассмотрении",
  "l.enum.proposal_status.approved": "утверждено",
  "l.enum.proposal_status.rejected": "отклонено",
  "l.enum.proposal_status.promoted": "переведено в план",
  "l.enum.band.high": "высокий",
  "l.enum.band.medium": "средний",
  "l.enum.band.low": "низкий",
  "l.enum.band.unscored": "не оценено",
  "l.enum.api.unauthorized": "не авторизован",
  "l.enum.api.forbidden": "доступ запрещён",
  "l.enum.api.not_found": "не найдено",
  "l.enum.api.internal_error": "Внутренняя ошибка",
  "l.enum.api.rate_limited": "превышен лимит запросов",
  "l.enum.api.http_status": "Код HTTP {status}",
  "p.error.not_found": "Такой страницы не существует.",
  "p.error.not_allowed": "У вас нет доступа к этой странице.",
  "p.error.went_wrong": "Что-то пошло не так. Повторите попытку.",
  "p.error.title": "Ошибка {status} — PPM",
  "p.error.back": "Вернуться на главную",
  "p.issues.issues_ppm": "Реестр проблем — PPM",
  "p.issues.issues": "Реестр проблем",
  "p.issues.intro":
    "Проблема — это то, что уже произошло и требует управления; риск — то, что может произойти. Сначала самые серьёзные, затем самые давние. Возраст указан в календарных днях с момента регистрации проблемы.",
  "p.issues.could_not_load": "Не удалось загрузить проблемы",
  "p.issues.by_severity": "Проблемы по серьёзности",
  "p.issues.overdue_count": "Просрочено",
  "p.issues.oldest_open":
    "Самая давняя открытая: {title} ({days} календарных дн.)",
  "p.issues.show": "Показать",
  "p.issues.all_statuses": "Все статусы",
  "p.issues.open_only": "Только открытые и в работе",
  "p.issues.severity_filter": "Серьёзность",
  "p.issues.all_severities": "Все уровни серьёзности",
  "p.issues.older_than": "Возраст не менее (календарных дней)",
  "p.issues.none": "Проблем не зарегистрировано.",
  "p.issues.none_match": "Подходящих проблем нет.",
  "p.issues.loading": "Загрузка…",
  "p.issues.count_one": "{n} проблема, сначала самые серьёзные",
  "p.issues.count_other": "Проблем: {n}, сначала самые серьёзные",
  "p.issues.truncated":
    "Показаны первые {n}; чтобы увидеть остальные, сузьте фильтры.",
  "p.issues.title_col": "Проблема",
  "p.issues.kind_col": "Вид",
  "p.issues.severity_col": "Серьёзность",
  "p.issues.status_col": "Статус",
  "p.issues.owner_col": "Ответственный",
  "p.issues.age_col": "Возраст (календарных дней)",
  "p.issues.due_col": "Срок",
  "p.issues.overdue_flag": "Просрочено",
  "p.issues.no_owner": "Нет ответственного",
  "p.issues.no_due": "Срок не задан",
  "p.issues.plan_link": "Открыть проблемы плана",
  "p.issues.plan_issues_link": "Проблемы",
  "p.plan_issues.title_ppm": "Проблемы плана — PPM",
  "p.plan_issues.title": "Проблемы плана",
  "p.plan_issues.intro":
    "Проблемы, зарегистрированные по этому плану, сначала самые серьёзные. Проблема — это то, что уже произошло; риск — то, что может произойти. Можно зарегистрировать проблему, изменить её или создать проблему из риска, который наступил.",
  "p.plan_issues.back": "Назад к плану",
  "p.plan_issues.register_link": "Проблемы по всем планам",
  "p.plan_issues.could_not_load": "Не удалось загрузить проблемы плана",
  "p.plan_issues.could_not_save": "Не удалось сохранить проблему",
  "p.plan_issues.counts":
    "Всего: {total}, открытых или в работе: {open}, просроченных: {overdue}",
  "p.plan_issues.actions_col": "Действия",
  "p.plan_issues.edit": "Изменить",
  "p.plan_issues.raise_heading": "Зарегистрировать проблему",
  "p.plan_issues.edit_heading": "Изменить проблему",
  "p.plan_issues.f_title": "Название",
  "p.plan_issues.f_description": "Описание",
  "p.plan_issues.f_owner": "Ответственный (ссылка вида person: или worker:)",
  "p.plan_issues.f_raised_on":
    "Дата регистрации (оставьте пустым для сегодняшней)",
  "p.plan_issues.f_resolution": "Решение (требуется для закрытия проблемы)",
  "p.plan_issues.raise": "Зарегистрировать проблему",
  "p.plan_issues.save": "Сохранить изменения",
  "p.plan_issues.cancel_edit": "Отменить изменение",
  "p.plan_issues.raised": "Проблема зарегистрирована.",
  "p.plan_issues.saved": "Проблема сохранена.",
  "p.plan_issues.problem_title_required": "Укажите название проблемы.",
  "p.plan_issues.problem_resolution_required":
    "Для закрытой проблемы нужно решение: укажите, как она была урегулирована.",
  "p.plan_issues.problem_due_before_raised":
    "Срок не может быть раньше дня регистрации проблемы.",
  "p.plan_issues.from_risk_heading": "Создать проблему из риска",
  "p.plan_issues.from_risk_intro":
    "Для риска, который наступил. Проблема получает название, описание и ответственного риска, а серьёзность — по его уровню воздействия; сам риск не меняется. У риска может быть только одна открытая проблема одновременно.",
  "p.plan_issues.f_risk": "Риск",
  "p.plan_issues.choose_risk": "Выберите риск",
  "p.plan_issues.raise_from_risk": "Создать проблему из этого риска",
  "p.plan_issues.raised_from_risk": "Проблема создана из риска.",
  "l.enum.severity.low": "низкая",
  "l.enum.severity.med": "средняя",
  "l.enum.severity.high": "высокая",
  "l.enum.severity.critical": "критическая",
  "l.enum.issue_status.open": "открыта",
  "l.enum.issue_status.in_progress": "в работе",
  "l.enum.issue_status.resolved": "решена",
  "l.enum.issue_status.closed": "закрыта",
  "l.enum.issue_kind.bug": "дефект",
  "l.enum.issue_kind.risk": "риск",
  "l.enum.issue_kind.blocker": "блокер",
  "l.enum.issue_kind.question": "вопрос",
  "l.enum.issue_kind.improvement": "улучшение",
  "l.enum.issue_kind.concern": "замечание",
  "p.signin.oidc_idle":
    "Вы вышли из системы из-за длительного бездействия. Войдите снова.",
  "p.plans.pid.board.only_first_tasks":
    "В этом плане задач: {total}; на доске показаны первые {shown}. Сузьте выбор по компоненту или прочитайте остальные через API.",
  "p.plan_modules.title": "Инструменты этого плана",
  "p.plan_modules.title_ppm": "Инструменты этого плана — PPM",
  "p.plan_modules.back": "← Назад к плану",
  "p.plan_modules.intro":
    "Выберите, какие инструменты показывает этот план. Отключённый инструмент скрыт на странице плана, а его страницы отвечают «не найдено». Ничего не удаляется: при повторном включении инструмента снова виден его прежний набор данных.",
  "p.plan_modules.source_plan": "У этого плана собственный набор инструментов.",
  "p.plan_modules.source_default":
    "Этот план использует инструменты по умолчанию для развёртывания.",
  "p.plan_modules.legend": "Инструменты, которые показывает план",
  "p.plan_modules.module_tasks": "Задачи, доска и анализ расписания",
  "p.plan_modules.module_time": "Учёт времени",
  "p.plan_modules.module_risks": "Риски и проблемы",
  "p.plan_modules.module_financials": "Бюджеты, затраты и освоенный объём",
  "p.plan_modules.module_milestones": "Вехи, расписание и календарь",
  "p.plan_modules.module_governance":
    "Управление, контрольные точки и запросы на изменение",
  "p.plan_modules.module_roadmap": "Публичная дорожная карта продукта",
  "p.plan_modules.none_chosen":
    "Если ни один инструмент не выбран, страница плана показывает его запись без инструментов.",
  "p.plan_modules.save": "Сохранить инструменты",
  "p.plan_modules.presets_heading": "Готовые наборы",
  "p.plan_modules.personal_is":
    "Личный набор для работы одного человека: {names}.",
  "p.plan_modules.use_personal": "Использовать личный набор",
  "p.plan_modules.default_is": "Набор по умолчанию для развёртывания: {names}.",
  "p.plan_modules.use_default":
    "Следовать набору по умолчанию для развёртывания",
  "p.plan_modules.no_tools": "нет инструментов",
  "p.plan_modules.unchanged":
    "Сохранять нечего: этот план уже следует набору по умолчанию, который совпадает с этим выбором.",
  "p.plan_modules.saved": "Сохранено.",
  "p.plan_modules.could_not_load":
    "Не удалось загрузить инструменты этого плана.",
  "p.plan_modules.could_not_save": "Изменение не сохранено.",
  "p.canvas.title": "Стратегические канвасы",
  "p.canvas.title_ppm": "Стратегические канвасы — PPM",
  "p.canvas.intro":
    "Канвас — это одностраничная доска из блоков, например Lean Canvas или SWOT-анализ. Элемент канваса может стать идеей, целью или, в канвасе плана, риском.",
  "p.canvas.plan_title": "Канвасы этого плана",
  "p.canvas.plan_title_ppm": "Канвасы этого плана — PPM",
  "p.canvas.plan_intro":
    "Стратегические канвасы этого плана. Элемент любого из них может стать риском в реестре рисков этого плана.",
  "p.canvas.one_title": "Стратегический канвас",
  "p.canvas.one_title_ppm": "Стратегический канвас — PPM",
  "p.canvas.back": "← Назад к канвасам",
  "p.canvas.list_heading": "Канвасы",
  "p.canvas.filter": "Показать",
  "p.canvas.filter_all": "Все виды",
  "p.canvas.none_yet": "Канвасов пока нет.",
  "p.canvas.none_of_kind": "Канвасов этого вида нет.",
  "p.canvas.free_standing": "не привязан к плану",
  "p.canvas.on_plan": "привязан к плану",
  "p.canvas.new_heading": "Создать канвас",
  "p.canvas.new_on_plan": "Новый канвас относится к этому плану.",
  "p.canvas.new_free_standing":
    "Новый канвас не привязан к плану, поэтому его элементы могут стать идеями или целями, но не рисками.",
  "p.canvas.f_template": "Вид канваса",
  "p.canvas.f_title": "Название",
  "p.canvas.f_description": "Описание (необязательно)",
  "p.canvas.create": "Создать канвас",
  "p.canvas.could_not_load_list": "Не удалось загрузить канвасы.",
  "p.canvas.could_not_load_templates":
    "Не удалось загрузить виды канвасов, поэтому создать новый канвас нельзя.",
  "p.canvas.could_not_create": "Канвас не создан.",
  "p.canvas.problem_title_blank": "Укажите название канваса.",
  "p.canvas.problem_title_long":
    "Название может содержать не более 200 символов.",
  "p.canvas.problem_description_long":
    "Описание может содержать не более 2000 символов.",
  "p.canvas.template_lean": "Бережливый канвас (Lean Canvas)",
  "p.canvas.template_business_model":
    "Канвас бизнес-модели (Business Model Canvas)",
  "p.canvas.template_swot": "SWOT-анализ",
  "p.canvas.template_empathy": "Карта эмпатии",
  "p.canvas.template_logic_model": "Логическая модель",
  "p.canvas.template_risk": "Канвас рисков",
  "p.canvas.template_goal": "Канвас цели",
  "p.canvas.box_problem": "Проблема",
  "p.canvas.box_solution": "Решение",
  "p.canvas.box_key_metrics": "Ключевые метрики",
  "p.canvas.box_unique_value_proposition": "Уникальное ценностное предложение",
  "p.canvas.box_unfair_advantage": "Неочевидное преимущество",
  "p.canvas.box_channels": "Каналы",
  "p.canvas.box_customer_segments": "Сегменты клиентов",
  "p.canvas.box_cost_structure": "Структура затрат",
  "p.canvas.box_revenue_streams": "Потоки доходов",
  "p.canvas.box_key_partners": "Ключевые партнёры",
  "p.canvas.box_key_activities": "Ключевые виды деятельности",
  "p.canvas.box_key_resources": "Ключевые ресурсы",
  "p.canvas.box_value_propositions": "Ценностные предложения",
  "p.canvas.box_customer_relationships": "Отношения с клиентами",
  "p.canvas.box_strengths": "Сильные стороны",
  "p.canvas.box_weaknesses": "Слабые стороны",
  "p.canvas.box_opportunities": "Возможности",
  "p.canvas.box_threats": "Угрозы",
  "p.canvas.box_says": "Говорит",
  "p.canvas.box_thinks": "Думает",
  "p.canvas.box_does": "Делает",
  "p.canvas.box_feels": "Чувствует",
  "p.canvas.box_pains": "Проблемы",
  "p.canvas.box_gains": "Выгоды",
  "p.canvas.box_inputs": "Входные данные",
  "p.canvas.box_activities": "Мероприятия",
  "p.canvas.box_outputs": "Выходные результаты",
  "p.canvas.box_outcomes": "Итоги",
  "p.canvas.box_impact": "Воздействие",
  "p.canvas.box_vulnerabilities": "Уязвимости",
  "p.canvas.box_existing_controls": "Действующие меры контроля",
  "p.canvas.box_residual_risks": "Остаточные риски",
  "p.canvas.box_goal": "Цель",
  "p.canvas.box_success_measures": "Показатели успеха",
  "p.canvas.box_actions": "Действия",
  "p.canvas.box_obstacles": "Препятствия",
  "p.canvas.box_resources": "Ресурсы",
  "p.canvas.box_empty": "Пока ничего нет.",
  "p.canvas.f_new_item": "Новый элемент в блоке «{box}»",
  "p.canvas.add": "Добавить",
  "p.canvas.added": "Добавлено.",
  "p.canvas.f_text": "Текст",
  "p.canvas.f_box": "Блок",
  "p.canvas.save": "Сохранить",
  "p.canvas.cancel": "Отмена",
  "p.canvas.saved": "Сохранено.",
  "p.canvas.unchanged": "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.canvas.edit": "Изменить",
  "p.canvas.remove": "Удалить",
  "p.canvas.removed": "Удалено.",
  "p.canvas.confirm_remove_item": "Удалить «{text}» из канваса?",
  "p.canvas.promote": "Преобразовать в…",
  "p.canvas.promote_legend": "Преобразовать этот элемент в",
  "p.canvas.target_idea": "идею",
  "p.canvas.target_objective": "цель",
  "p.canvas.target_risk": "риск этого плана",
  "p.canvas.promoted": "Преобразовано: {kind}.",
  "p.canvas.promote_once": "Элемент можно преобразовать только один раз.",
  "p.canvas.promote_go": "Преобразовать в это",
  "p.canvas.no_risk_free_standing":
    "Этот канвас не привязан к плану, поэтому элемент не может стать риском.",
  "p.canvas.f_probability": "Вероятность (от 1 до 5)",
  "p.canvas.f_impact": "Воздействие (от 1 до 5)",
  "p.canvas.problem_risk_scores":
    "Для риска нужны вероятность и воздействие — целые числа от 1 до 5.",
  "p.canvas.problem_text_blank": "Сначала введите текст элемента.",
  "p.canvas.problem_text_long":
    "Элемент может содержать не более 2000 символов.",
  "p.canvas.became_idea": "Стал идеей",
  "p.canvas.became_risk": "Стал риском этого плана",
  "p.canvas.became_objective": "Стал целью",
  "p.canvas.owner": "Ответственный: {owner}",
  "p.canvas.full":
    "В этом канвасе максимально возможное число элементов ({max}); удалите один, чтобы добавить другой.",
  "p.canvas.could_not_load": "Не удалось загрузить этот канвас.",
  "p.canvas.could_not_add": "Элемент не добавлен.",
  "p.canvas.could_not_save": "Изменение не сохранено.",
  "p.canvas.could_not_remove": "Элемент не удалён.",
  "p.canvas.could_not_promote": "Элемент не преобразован.",
  "p.canvas.remove_canvas_heading": "Удалить канвас",
  "p.canvas.remove_canvas_says":
    "При удалении канваса удаляются его элементы. Идеи, цели и риски, которыми стали его элементы, остаются.",
  "p.canvas.remove_canvas": "Удалить этот канвас",
  "p.canvas.confirm_remove_canvas": "Удалить канвас «{title}» и его элементы?",
  "p.canvas.could_not_remove_canvas": "Канвас не удалён.",
  "p.plan_templates.title": "Шаблоны планов",
  "p.plan_templates.title_ppm": "Шаблоны планов — PPM",
  "p.plan_templates.intro":
    "Шаблон — это план и вложенные в него планы, сохранённые без людей и с каждой датой в виде числа дней после начала, чтобы новые планы можно было создавать на его основе.",
  "p.plan_templates.saved_heading": "Сохранённые шаблоны",
  "p.plan_templates.starters_heading": "Встроенные заготовки",
  "p.plan_templates.none_saved":
    "Ни один шаблон ещё не сохранён. Сохраните его на странице плана.",
  "p.plan_templates.none_starters": "Встроенных заготовок нет.",
  "p.plan_templates.plans_count_one": "{count} план",
  "p.plan_templates.plans_count_other": "{count} планов",
  "p.plan_templates.needs_start": "нужна дата начала",
  "p.plan_templates.could_not_load_list": "Не удалось загрузить шаблоны.",
  "p.plan_templates.one_title": "Шаблон плана",
  "p.plan_templates.one_title_ppm": "Шаблон плана — PPM",
  "p.plan_templates.back": "← Назад к шаблонам",
  "p.plan_templates.is_starter":
    "Встроенная заготовка. Её нельзя изменить или удалить; создайте на её основе план и сохраните его под своим именем.",
  "p.plan_templates.is_saved": "Сохранённый шаблон, версия {version}.",
  "p.plan_templates.holds_heading": "Что он содержит",
  "p.plan_templates.holds_says":
    "Каждый план, число его задач и его вехи в днях после начала. Люди не сохраняются.",
  "p.plan_templates.tasks_count_one": "{count} задача",
  "p.plan_templates.tasks_count_other": "{count} задач",
  "p.plan_templates.milestone_at": "{name}: день {days}",
  "p.plan_templates.start_heading": "Создать планы на его основе",
  "p.plan_templates.start_says":
    "Новые планы создаются из шаблона, а даты отсчитываются от даты начала. Задачи начинаются с первого шага.",
  "p.plan_templates.f_plan_name":
    "Название плана верхнего уровня (необязательно)",
  "p.plan_templates.f_start": "Дата начала",
  "p.plan_templates.no_dates":
    "В этом шаблоне нет дат, поэтому дата начала не нужна.",
  "p.plan_templates.f_parent": "Поместить в",
  "p.plan_templates.no_parent": "Без плана: новый план верхнего уровня",
  "p.plan_templates.start": "Создать планы",
  "p.plan_templates.started":
    "Создано. Планов: {plans}; задач: {tasks}; вех: {milestones}.",
  "p.plan_templates.open_new": "Открыть новый план",
  "p.plan_templates.could_not_load": "Не удалось загрузить этот шаблон.",
  "p.plan_templates.could_not_start": "Планы не созданы.",
  "p.plan_templates.problem_plan_name_long":
    "Название плана может содержать не более 1024 символов.",
  "p.plan_templates.problem_start_needed":
    "Выберите дату начала: в этом шаблоне есть даты.",
  "p.plan_templates.problem_start_bad":
    "Укажите дату начала в виде дня, например 2027-01-04.",
  "p.plan_templates.remove_heading": "Удалить шаблон",
  "p.plan_templates.remove_says":
    "Удаление шаблона не меняет планы, уже созданные на его основе.",
  "p.plan_templates.remove": "Удалить этот шаблон",
  "p.plan_templates.confirm_remove": "Удалить шаблон «{name}»?",
  "p.plan_templates.could_not_remove": "Шаблон не удалён.",
  "p.plan_templates.save_title": "Сохранить как шаблон",
  "p.plan_templates.save_title_ppm": "Сохранить как шаблон — PPM",
  "p.plan_templates.save_says":
    "Сохраняет этот план и вложенные в него планы, доступные вам для чтения, с их задачами, компонентами, вехами и рабочими процессами. Люди не сохраняются, а каждая дата становится числом дней после начала. Последующие изменения плана не меняют шаблон.",
  "p.plan_templates.f_name": "Название шаблона",
  "p.plan_templates.f_description": "Описание (необязательно)",
  "p.plan_templates.save": "Сохранить шаблон",
  "p.plan_templates.saved":
    "Сохранено. Планов: {plans}; задач: {tasks}; вех: {milestones}.",
  "p.plan_templates.omitted_one":
    "{count} вложенный план пропущен, потому что вы не можете его прочитать.",
  "p.plan_templates.omitted_other":
    "Вложенных планов пропущено: {count}, потому что вы не можете их прочитать.",
  "p.plan_templates.open_template": "Открыть шаблон",
  "p.plan_templates.name_taken":
    "Шаблон с таким названием уже существует. Выберите другое название.",
  "p.plan_templates.could_not_save": "Шаблон не сохранён.",
  "p.plan_templates.problem_name_blank": "Укажите название шаблона.",
  "p.plan_templates.problem_name_long":
    "Название шаблона может содержать не более 120 символов.",
  "p.plan_export.title": "Экспорт в Microsoft Project",
  "p.plan_export.title_ppm": "Экспорт в Microsoft Project — PPM",
  "p.plan_export.intro":
    "Сохраните задачи этого плана в файл MSPDI XML — формат, который открывают Microsoft Project и другие средства планирования.",
  "p.plan_export.carried_heading": "Что содержит файл",
  "p.plan_export.carried":
    "Каждая задача с её структурой под родительской задачей, дата начала, длительность в рабочих днях, вехи, ограничения, крайний срок и заметки; признак выполнения; связи между задачами с их типом и запаздыванием; рабочий календарь плана.",
  "p.plan_export.not_carried_heading": "Что в файл не попадает",
  "p.plan_export.not_carried":
    "Назначенные люди и ресурсы; любой статус, кроме «выполнено» (задача выполнена на 0 или 100 процентов); спринты и компоненты; трудозатраты; пользовательские поля; комментарии; базовые планы; связи между задачами, кроме «предшествует»; и запаздывания в календарном времени (все запаздывания указываются в рабочих днях). Файл сообщает об этом и сам.",
  "p.plan_export.limit":
    "План, в котором больше 5000 задач, экспортировать нельзя.",
  "p.plan_export.not_checked":
    "Файл прочитан обратно собственным импортёром этой службы, но ещё не открывался в самом Microsoft Project.",
  "p.plan_export.download": "Скачать файл MSPDI",
  "p.plan_export.to_import": "Вместо этого загрузить файл MSPDI в этот план",
  "p.confidence.assess_heading": "Добавить оценку",
  "p.confidence.assess_says":
    "Заявленное значение — это мнение человека о том, выполнит ли план свои обязательства. Оно добавляется в историю и никогда не заменяет предыдущее.",
  "p.confidence.choose_value": "Выберите значение",
  "p.confidence.could_not_load":
    "Не удалось загрузить уверенность в поставке этого плана.",
  "p.confidence.could_not_save": "Оценка не добавлена.",
  "p.confidence.f_assessed_on":
    "Дата оценки (необязательно; если пусто, то сегодня)",
  "p.confidence.f_kind": "Кто даёт оценку",
  "p.confidence.f_rationale": "Обоснование (не менее {min} символов)",
  "p.confidence.f_value": "Уверенность в поставке",
  "p.confidence.history_heading": "История, сначала новые",
  "p.confidence.intro":
    "Заявленное мнение человека о том, выполнит ли этот план свои обязательства. Оно хранится рядом с RAG, рассчитанным по данным плана, и никогда с ним не смешивается.",
  "p.confidence.kind_independent": "Независимая оценка",
  "p.confidence.kind_self": "Собственная команда плана (самооценка)",
  "p.confidence.no_history": "Ни одной оценки пока не дано.",
  "p.confidence.none_yet":
    "Не оценено: никто ещё не высказал мнение. Это не то же самое, что зелёный.",
  "p.confidence.opens_exception":
    "Янтарный/красный или красный открывает исключение по этому плану либо усугубляет уже открытое. Затем кто-то должен его заполнить.",
  "p.confidence.problem_future": "Дата оценки не может быть в будущем.",
  "p.confidence.problem_kind": "Выберите, кто даёт оценку.",
  "p.confidence.problem_rationale":
    "Укажите причину не менее чем в {min} символов.",
  "p.confidence.problem_value": "Выберите значение.",
  "p.confidence.refused_independent":
    "Для независимой оценки нужно право независимого оценщика. Вы можете добавить самооценку.",
  "p.confidence.save": "Добавить оценку",
  "p.confidence.saved_exception_one":
    "Добавлено. Теперь у этого плана есть открытое исключение: его нужно заполнить в течение {hours} ч.",
  "p.confidence.saved_exception_other":
    "Добавлено. Теперь у этого плана есть открытое исключение: его нужно заполнить в течение {hours} ч.",
  "p.confidence.saved": "Добавлено.",
  "p.confidence.shown_heading": "Что показывает карта портфеля",
  "p.confidence.shown_independent": "Независимая оценка от {when}.",
  "p.confidence.shown_self":
    "Самооценка от {when}; вместо неё будет показана независимая оценка, когда она появится.",
  "p.confidence.stale": "устарела: старше этого месяца",
  "p.confidence.title_ppm": "Уверенность в поставке — PPM",
  "p.confidence.title": "Уверенность в поставке",
  "p.exceptions.a_plan": "План",
  "p.exceptions.breached_one":
    "Нарушен срок: не заполнено в течение {hours} ч после открытия.",
  "p.exceptions.breached_other":
    "Нарушен срок: не заполнено в течение {hours} ч после открытия.",
  "p.exceptions.cancel": "Отмена",
  "p.thresholds.could_not_load": "Не удалось загрузить триаж и пороги.",
  "p.thresholds.could_not_save": "Изменение не сохранено.",
  "p.thresholds.could_not_sweep": "Не удалось проверить пороги.",
  "p.thresholds.decide_says":
    "Запись решения закрывает исключение. Она фиксирует принятое решение; она не приостанавливает, не сбрасывает и не останавливает программу — это делает человек через статус программы.",
  "p.thresholds.decide": "Записать решение",
  "p.thresholds.decided_done": "Решение записано: {action}.",
  "p.thresholds.f_action": "Решение",
  "p.thresholds.f_amount": "Предельная сумма",
  "p.thresholds.f_count":
    "Выпущенных пакетов с красным подряд (от {min} до {max})",
  "p.thresholds.f_currency": "Валюта (три буквы)",
  "p.thresholds.f_date":
    "Дата, позже которой не должно быть прогнозируемое окончание",
  "p.thresholds.f_expiry":
    "Переопределение действует до (в будущем, не более чем через {days} дн.)",
  "p.thresholds.f_outcome": "Исход",
  "p.thresholds.f_reason": "Причина (не менее {min} символов)",
  "p.thresholds.history_item": "{date}: {outcome}. {reason}",
  "p.thresholds.history": "Прежние триажи",
  "p.thresholds.not_set": "Не задан",
  "p.thresholds.now_cap": "Пересекается, когда стоимость P80 выше {amount}.",
  "p.thresholds.now_forecast":
    "Пересекается, когда прогнозируемое окончание позже {date}.",
  "p.thresholds.now_red_packs_one":
    "Пересекается, когда уверенность красная в {n} выпущенном пакете подряд.",
  "p.thresholds.now_red_packs_other":
    "Пересекается, когда уверенность красная в выпущенных пакетах подряд: {n}.",
  "p.thresholds.opened_by_threshold":
    "Открыто, потому что пересечён порог программы.",
  "p.thresholds.problem_action":
    "Выберите приостановку, сброс, остановку или переопределение.",
  "p.thresholds.problem_cap": "Предельная сумма должна быть больше нуля.",
  "p.thresholds.problem_count":
    "Число пакетов должно быть целым числом от {min} до {max}.",
  "p.thresholds.problem_currency":
    "Валюта должна быть задана трёхбуквенным кодом ISO 4217.",
  "p.thresholds.problem_date": "Выберите дату.",
  "p.thresholds.problem_expiry_long":
    "Переопределение может действовать не более {days} дн.",
  "p.thresholds.problem_expiry_needed":
    "Для переопределения нужна дата, до которой оно действует.",
  "p.thresholds.problem_expiry_past":
    "Дата переопределения должна быть в будущем.",
  "p.thresholds.problem_outcome":
    "Выберите завершение, исправление, приостановку или остановку.",
  "p.thresholds.problem_reason":
    "Причина должна содержать не менее {min} символов.",
  "p.thresholds.remove_threshold": "Удалить порог",
  "p.thresholds.save_threshold": "Сохранить порог",
  "p.thresholds.save_triage": "Записать триаж",
  "p.thresholds.sweep_done_one":
    "Проверено порогов: {checked}; открыто исключение: {n}.",
  "p.thresholds.sweep_done_other":
    "Проверено порогов: {checked}; открыто исключений: {n}.",
  "p.thresholds.sweep_item": "{programme}: {kind} — {outcome}",
  "p.thresholds.sweep_list": "Пересечённые пороги",
  "p.thresholds.sweep_says":
    "Проверка открывает исключение по поставке для каждого пересечённого порога или дополняет уже открытое; она ничего не приостанавливает, не сбрасывает и не останавливает.",
  "p.thresholds.sweep": "Проверить пороги сейчас",
  "p.thresholds.threshold_removed": "Порог удалён: {kind}.",
  "p.thresholds.threshold_saved": "Порог сохранён: {kind}.",
  "p.thresholds.thresholds_heading": "Пороги",
  "p.thresholds.thresholds_says":
    "Пересечение порога открывает исключение по поставке, на которое человек отвечает приостановкой, сбросом, остановкой или переопределением со сроком действия. Автоматически ничего не приостанавливается, не сбрасывается и не останавливается.",
  "p.thresholds.triage_heading": "Триаж",
  "p.thresholds.triage_latest": "Последний триаж, {date}: {outcome}. {reason}",
  "p.thresholds.triage_none": "Триаж ещё не записан.",
  "p.thresholds.triage_saved": "Триаж записан.",
  "p.thresholds.triage_says":
    "Триаж — это датированное мнение о том, куда должна двигаться программа, с указанием причины. Он фиксирует мнение; в самой программе он ничего не меняет.",
  "p.thresholds.unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.thresholds.action_pause": "Приостановить",
  "p.thresholds.action_reset": "Сбросить",
  "p.thresholds.action_stop": "Остановить",
  "p.thresholds.action_override": "Переопределить",
  "p.thresholds.kind_confidence_red_packs":
    "Красная уверенность в нескольких пакетах",
  "p.thresholds.kind_forecast_past": "Прогнозируемое окончание позже даты",
  "p.thresholds.kind_p80_cap": "Стоимость P80 выше предела",
  "p.thresholds.outcome_finish": "Завершить",
  "p.thresholds.outcome_fix": "Исправить",
  "p.thresholds.outcome_pause": "Приостановить",
  "p.thresholds.outcome_stop": "Остановить",
  "p.thresholds.sweep_opened": "исключение открыто",
  "p.thresholds.sweep_already_open": "исключение уже открыто",
  "p.thresholds.sweep_attached_to_open_exception":
    "добавлено в открытое исключение",
  "p.thresholds.sweep_suppressed_by_override":
    "не открыто: действует переопределение",
  "p.exceptions.closed_done": "Решение записано; исключение закрыто.",
  "p.exceptions.complete_it": "Заполнить",
  "p.exceptions.complete_says":
    "Исключение заполнено, когда указаны его причина, необходимое решение, ответственный и срок принятия решения. Оставленная пустой часть сохраняет прежнее значение.",
  "p.exceptions.complete":
    "Заполнено: указаны причина, необходимое решение, ответственный и срок принятия решения.",
  "p.exceptions.conflict_closed": "Это исключение уже закрыто.",
  "p.exceptions.could_not_load": "Не удалось загрузить исключения.",
  "p.exceptions.could_not_save": "Изменение не сохранено.",
  "p.exceptions.decide_it": "Записать решение",
  "p.exceptions.decide_says":
    "Запись решения закрывает исключение. Повышение уверенности само по себе его никогда не закрывает.",
  "p.exceptions.decide": "Записать решение и закрыть",
  "p.exceptions.f_breached": "Только с нарушенным сроком",
  "p.exceptions.f_decision": "Решение (не менее {min} символов)",
  "p.exceptions.f_owner_ref": "Ответственный (ссылка на человека)",
  "p.exceptions.f_owner_role": "Роль ответственного (то, что показывает пакет)",
  "p.exceptions.f_status": "Показать",
  "p.exceptions.intro_one":
    "Исключение открывается, когда уверенность в поставке плана достигает янтарного/красного или красного. Его нужно заполнить в течение {hours} ч, а закрыть его может только записанное решение.",
  "p.exceptions.intro_other":
    "Исключение открывается, когда уверенность в поставке плана достигает янтарного/красного или красного. Его нужно заполнить в течение {hours} ч, а закрыть его может только записанное решение.",
  "p.exceptions.is_closed": "закрыто {when}",
  "p.exceptions.is_open": "открыто",
  "p.exceptions.missing": "Ещё требуется: {parts}.",
  "p.exceptions.none": "Подходящих исключений нет.",
  "p.exceptions.opened": "Открыто {when}",
  "p.exceptions.part_cause": "Причина",
  "p.exceptions.part_decide_by": "Принять решение до",
  "p.exceptions.part_decision_needed": "Необходимое решение",
  "p.exceptions.part_decision": "Решение",
  "p.exceptions.part_owner": "Ответственный",
  "p.exceptions.problem_decision":
    "Запишите решение не менее чем в {min} символов.",
  "p.exceptions.problem_part_short":
    "Чтобы причина, необходимое решение или ответственный засчитывались, нужно не менее {min} символов.",
  "p.exceptions.refused_threshold":
    "Это исключение открыто из-за порога, поэтому оно закрывается приостановкой, сбросом, остановкой или записанным переопределением, а не решением здесь.",
  "p.exceptions.save": "Сохранить",
  "p.exceptions.saved": "Сохранено.",
  "p.exceptions.see_register": "Открыть реестр исключений",
  "p.exceptions.status_all": "Все исключения",
  "p.exceptions.status_closed": "Закрытые",
  "p.exceptions.status_open": "Открытые",
  "p.exceptions.title_ppm": "Исключения — PPM",
  "p.exceptions.title": "Исключения",
  "p.exceptions.unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.exceptions.worsened_one": "Ухудшалось {count} раз с момента открытия",
  "p.exceptions.worsened_other": "Ухудшалось {count} раз с момента открытия",
  "p.exceptions.worst": "Худшая уверенность: {value}",
  "p.aims.aims_heading": "Стратегические цели",
  "p.aims.aims_legend": "Задачи, которые являются стратегическими целями",
  "p.aims.aims_says":
    "Стратегическая цель — это цель, вокруг которой выстроен портфель. Снятие отметки также лишает её статуса основной цели любой программы.",
  "p.aims.choose_aim": "Выберите цель",
  "p.aims.choose_programme": "Выберите программу",
  "p.aims.could_not_load": "Не удалось загрузить карту портфеля.",
  "p.aims.could_not_save": "Изменение не сохранено.",
  "p.aims.f_aim": "Основная цель",
  "p.aims.f_period": "Месяц (если пусто, то текущий)",
  "p.aims.f_programme": "Программа",
  "p.aims.intro":
    "Доступные вам для чтения программы, каждая по одному разу под своей основной стратегической целью. Уверенность в поставке — это заявленное значение; оно показано рядом с RAG, рассчитанным по данным, и никогда с ним не смешивается; «не оценено» — это не зелёный.",
  "p.aims.map_heading": "Карта портфеля, {period}",
  "p.aims.marked": "Отмечено как стратегическая цель.",
  "p.aims.no_aims":
    "Ни одна цель пока не является стратегической; сначала отметьте одну выше.",
  "p.aims.no_objectives": "Целей пока нет.",
  "p.aims.no_programmes":
    "На карте нет программ: программа — это план вида Program.",
  "p.aims.primary_heading": "Основная цель программы",
  "p.aims.primary_says":
    "Программа показывается на карте под единственной основной целью; другие цели, которым она служит, указаны рядом. Выбор новой основной цели превращает прежнюю в обычную связь.",
  "p.aims.primary_set": "Основная цель задана.",
  "p.aims.problem_already_primary": "Это уже основная цель этой программы.",
  "p.aims.problem_choose": "Выберите программу и цель.",
  "p.aims.problem_period":
    "Укажите месяц в виде ГГГГ-ММ или оставьте пустым для текущего месяца.",
  "p.aims.set_primary": "Задать основную цель",
  "p.aims.show": "Показать",
  "p.aims.title_ppm": "Карта портфеля — PPM",
  "p.aims.title": "Карта портфеля",
  "p.aims.to_objectives": "Открыть цели",
  "p.aims.unmarked": "Больше не является стратегической целью.",
  "p.programme.cited_exception":
    "Оно вытекало из исключения по поставке этой программы.",
  "p.programme.cited_review":
    "Оно вытекало из проверки контрольной точки этой программы.",
  "p.programme.cites_none": "Ни то, ни другое",
  "p.programme.conflict_status":
    "У программы уже такой статус, либо она остановлена и остаётся остановленной.",
  "p.programme.could_not_decide": "Решение не записано.",
  "p.programme.could_not_load":
    "Не удалось загрузить контрольные точки и статус этого плана.",
  "p.programme.could_not_save_gates":
    "Сохранены не все даты контрольных точек; показаны даты, которые есть в службе.",
  "p.programme.decide": "Записать решение",
  "p.programme.decided": "Записано: теперь программа — {status}.",
  "p.programme.do_pause": "Приостановить программу",
  "p.programme.do_resume": "Возобновить программу",
  "p.programme.do_stop": "Остановить программу",
  "p.programme.f_change": "Перевести программу в состояние",
  "p.programme.f_exception": "Вытекает из исключения (необязательно)",
  "p.programme.f_reason": "Причина (не менее {min} символов)",
  "p.programme.f_review":
    "Вытекает из проверки контрольной точки (необязательно)",
  "p.programme.gates_heading": "Плановые даты контрольных точек",
  "p.programme.gates_says":
    "Дата, на которую этот план ожидает проверку на каждой контрольной точке. Заданную дату нельзя очистить, можно только перенести.",
  "p.programme.gates_saved": "Даты контрольных точек сохранены.",
  "p.programme.gates_unchanged":
    "Ни одна дата контрольной точки не изменилась, поэтому ничего не отправлено.",
  "p.programme.held_allocations_one":
    "{count} распределение помечено, а не удалено.",
  "p.programme.held_allocations_other":
    "Распределений помечено, а не удалено: {count}.",
  "p.programme.held_benefits_one": "{count} выгода сохранена в записи.",
  "p.programme.held_benefits_other": "Выгод сохранено в записи: {count}.",
  "p.programme.held_heading": "Что всё ещё удерживает программа",
  "p.programme.held_money":
    "Деньги: запланировано {planned}, фактически {actual}.",
  "p.programme.history_heading": "Решения, сначала новые",
  "p.programme.no_history": "Решений пока нет: программа активна.",
  "p.programme.not_a_programme":
    "Статус программы есть только у плана вида Program. Даты его контрольных точек всё равно можно запланировать выше.",
  "p.programme.problem_reason":
    "Укажите причину не менее чем в {min} символов.",
  "p.programme.problem_status": "Выберите изменение.",
  "p.programme.refused_decider":
    "Для приостановки, возобновления или остановки программы нужно право принимающего решения по программе.",
  "p.programme.refused_not_programme":
    "Статус есть только у плана вида Program, а указанная проверка или исключение должны относиться к этой программе.",
  "p.programme.save_gates": "Сохранить даты контрольных точек",
  "p.programme.status_heading": "Статус программы",
  "p.programme.status_is": "Статус: {status}",
  "p.programme.status_says":
    "Программа активна, пока записанное решение не приостановит или не остановит её. Приостановка помечает её распределения и ничего не удаляет.",
  "p.programme.stop_warns":
    "Остановка окончательна: программа остаётся остановленной и сохраняет свою запись.",
  "p.programme.stopped_final":
    "Эта программа остановлена. Это окончательно; её запись сохраняется.",
  "p.programme.title_ppm": "Контрольные точки и статус программы — PPM",
  "p.programme.title": "Контрольные точки и статус программы",
  "p.programme.buffer_by_days": "Число рабочих дней",
  "p.programme.buffer_by_rule":
    "Половина критической работы по базовому плану, с округлением вверх",
  "p.programme.buffer_declared_one": "Объявлено: буфер {days} рабочий день.",
  "p.programme.buffer_declared_other": "Объявлено: буфер {days} рабочих дн.",
  "p.programme.buffer_heading": "Буфер проекта",
  "p.programme.buffer_says":
    "Время, удерживаемое в конце программы для поглощения задержек. Диаграмма показывает, какая часть буфера израсходована по сравнению с долей выполненной критической работы. Объявление никогда не заменяет предыдущее; буфером считается последнее. Если есть план буфера критической цепи, используется он.",
  "p.programme.buffer_history": "Объявления, сначала новые",
  "p.programme.buffer_was_days_one": "{days} рабочий день, объявлено",
  "p.programme.buffer_was_days_other": "{days} рабочих дн., объявлено",
  "p.programme.buffer_was_rule_one": "{days} рабочий день, по правилу",
  "p.programme.buffer_was_rule_other": "{days} рабочих дн., по правилу",
  "p.programme.could_not_declare_buffer": "Буфер не объявлен.",
  "p.programme.could_not_load_buffer": "Не удалось прочитать буфер проекта.",
  "p.programme.declare_buffer": "Объявить буфер",
  "p.programme.f_buffer_days": "Рабочие дни (от 1 до {max})",
  "p.programme.f_buffer_how": "Как задаётся длина буфера",
  "p.programme.f_buffer_note": "Примечание (необязательно)",
  "p.programme.problem_buffer_days":
    "Укажите буфер целым числом рабочих дней от 1 до 2000.",
  "p.programme.refused_buffer_rule":
    "Для правила нужен базовый план расписания с критической работой, имеющей длительность; сначала задайте базовый план или укажите число дней.",
  "p.programme.refused_not_programme_buffer":
    "Буфер проекта есть только у плана вида Program.",
  "p.programme.profile_heading": "Модель поставки и отчётность",
  "p.programme.f_model": "Модель поставки",
  "p.programme.f_owner": "Организация-владелец",
  "p.programme.no_owner": "Нет организации-владельца",
  "p.programme.model_says":
    "Модель поставки определяет, кто создаёт и кто развёртывает; пакет для директоров показывает её рядом с программой. Модель и владелец сохраняются вместе.",
  "p.programme.save_model": "Сохранить модель поставки",
  "p.programme.model_saved": "Модель поставки сохранена.",
  "p.programme.f_method": "Как программа отчитывается о ходе работ",
  "p.programme.method_evm": "Освоенный объём (показатели стоимости и сроков)",
  "p.programme.method_product":
    "Диаграмма сгорания и поток (продуктовая программа)",
  "p.programme.save_method": "Сохранить способ отчётности",
  "p.programme.method_saved_evm":
    "Теперь программа отчитывается по освоенному объёму.",
  "p.programme.method_saved_product":
    "Теперь программа отчитывается по диаграмме сгорания и потоку.",
  "p.programme.profile_unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.programme.could_not_load_profile": "Не удалось загрузить модель поставки.",
  "p.programme.could_not_save_profile": "Изменение не сохранено.",
  "p.benefits.classified": "Классификация сохранена.",
  "p.benefits.classify_says":
    "Сохранение отправляет всю классификацию целиком: выбор, оставленный как «не указано», очищает значение.",
  "p.benefits.could_not_load": "Не удалось загрузить выгоды этого плана.",
  "p.benefits.could_not_save": "Классификация не сохранена.",
  "p.benefits.could_not_validate": "Подтверждение не записано.",
  "p.benefits.f_annual": "Сумма в год ({currency})",
  "p.benefits.f_classification": "Классификация",
  "p.benefits.f_kpi": "KPI, который её измеряет",
  "p.benefits.f_note": "Примечание (необязательно)",
  "p.benefits.f_recurrence": "Повторяемость",
  "p.benefits.f_validated": "Подтверждённая на сегодня сумма ({currency})",
  "p.benefits.intro":
    "Реализованная сумма — это заявление. Она считается подтверждённой только при наличии записанного подтверждения от человека вне программы; в каждом указана подтверждённая на сегодня сумма, и учитывается последнее.",
  "p.benefits.measured_by_kpi":
    "Нефинансовая выгода измеряется своим KPI, а не подтверждается как сумма.",
  "p.benefits.money_line": "Цель {target}; реализовано (заявлено) {realized}.",
  "p.benefits.no_kpi": "Нет KPI",
  "p.benefits.non_financial": "Нефинансовая выгода.",
  "p.benefits.none": "У этого плана пока нет выгод.",
  "p.benefits.problem_annual_amount": "Укажите сумму в год числом не меньше 0.",
  "p.benefits.problem_annual_financial":
    "Сумма в год есть только у финансовой выгоды, то есть имеющей валюту.",
  "p.benefits.problem_annual_recurring":
    "Сумма в год есть только у повторяющейся выгоды: сначала выберите «повторяющаяся».",
  "p.benefits.problem_validated_amount":
    "Укажите подтверждённую на сегодня сумму числом не меньше 0.",
  "p.benefits.refused_member":
    "Подтверждающий не может быть участником программы: попросите подтвердить кого-то вне её.",
  "p.benefits.save": "Сохранить классификацию",
  "p.benefits.standing_claimed": "Заявлено, ещё не подтверждено",
  "p.benefits.standing_not_claimed": "Пока ничего не заявлено",
  "p.benefits.standing_partly_validated": "Подтверждено частично",
  "p.benefits.standing_validated": "Подтверждено",
  "p.benefits.title_ppm": "Выгоды — PPM",
  "p.benefits.title": "Выгоды: классификация и подтверждение",
  "p.benefits.validate": "Записать подтверждение",
  "p.benefits.validated_done": "Подтверждение записано.",
  "p.benefits.validated_to_date": "Подтверждено на сегодня: {amount}",
  "p.benefits.validation_heading": "Подтверждение",
  "p.benefits.validation_says":
    "Подтверждения хранятся в том виде, как записаны; новое указывает подтверждённую на сегодня сумму и ничего не заменяет.",
  "p.funding.add": "Добавить источник",
  "p.funding.added": "Источник финансирования добавлен.",
  "p.funding.choose_plan": "Выберите план",
  "p.funding.could_not_load_lines":
    "Не удалось загрузить бюджетные статьи этого плана.",
  "p.funding.could_not_load_money":
    "Не удалось загрузить деньги по источникам финансирования.",
  "p.funding.could_not_load": "Не удалось загрузить источники финансирования.",
  "p.funding.could_not_save": "Изменение не сохранено.",
  "p.funding.f_combine": "Объединить капитальные и операционные расходы",
  "p.funding.f_fy": "Финансовый год (год начала; если пусто, то текущий)",
  "p.funding.f_name": "Новый источник финансирования",
  "p.funding.f_plan": "План",
  "p.funding.f_source": "Источник финансирования",
  "p.funding.intro":
    "Откуда берутся деньги каждой программы. Бюджетная статья без источника финансирования учитывается и перечисляется, а не скрывается.",
  "p.funding.is_active": "действует",
  "p.funding.is_retired": "выведен из использования",
  "p.funding.line_cleared": "У статьи больше нет источника финансирования.",
  "p.funding.line_set": "Источник финансирования статьи задан.",
  "p.funding.lines_heading": "Бюджетные статьи и их источник финансирования",
  "p.funding.lines_says":
    "Выберите план, чтобы увидеть его бюджетные статьи и задать для каждой источник финансирования.",
  "p.funding.money_heading": "Деньги по источникам финансирования",
  "p.funding.no_lines": "У этого плана нет бюджетных статей.",
  "p.funding.no_source": "Нет источника финансирования",
  "p.funding.no_sources": "Ни один источник финансирования пока не добавлен.",
  "p.funding.problem_fy":
    "Укажите финансовый год как год его начала, от 2000 до 2100.",
  "p.funding.problem_name_taken":
    "Источник финансирования с таким названием уже существует.",
  "p.funding.problem_name":
    "Название источника финансирования содержит от 2 до 120 символов.",
  "p.funding.refused_inactive":
    "Этот источник финансирования выведен из использования, поэтому назначить его статье нельзя.",
  "p.funding.restore": "Восстановить",
  "p.funding.restored": "Источник финансирования восстановлен.",
  "p.funding.retire_says":
    "Выведенный из использования источник нельзя назначить статье, но статьи, у которых он уже есть, сохраняют его.",
  "p.funding.retire": "Вывести из использования",
  "p.funding.retired_option": "{name} (выведен из использования)",
  "p.funding.retired": "Источник финансирования выведен из использования.",
  "p.funding.show": "Показать",
  "p.funding.sources_heading": "Источники финансирования",
  "p.funding.title_ppm": "Финансирование — PPM",
  "p.funding.title": "Финансирование",
  "p.director_pack.actions_heading":
    "Проверить, выпустить или выпустить повторно",
  "p.director_pack.aud_board": "Совет",
  "p.director_pack.aud_escalation_board": "Совет по эскалации",
  "p.director_pack.aud_executive": "Руководство",
  "p.director_pack.aud_org": "Организация: {name}",
  "p.director_pack.aud_partner": "Партнёры",
  "p.director_pack.choose_audience": "Выберите аудиторию",
  "p.director_pack.conflict_draft":
    "За этот период для этой аудитории пакет уже существует. Откройте его: выпустите его черновик или выпустите повторно с примечанием об изменении.",
  "p.director_pack.conflict_issue":
    "Этот пакет сейчас нельзя выпустить: он уже выпущен либо это черновик по расписанию, который офис портфеля не проверил.",
  "p.director_pack.conflict_reissue":
    "Повторно выпускается только последняя выпущенная версия, а более поздняя версия уже существует.",
  "p.director_pack.conflict_review":
    "Этот черновик уже проверен либо больше не является черновиком.",
  "p.director_pack.could_not_act": "Изменение не внесено.",
  "p.director_pack.could_not_draft": "Пакет не подготовлен в виде черновика.",
  "p.director_pack.cut_off_says":
    "Оставьте пустым, чтобы использовать текущий момент. Пакет читает планы в состоянии на момент отсечки; он не может быть в будущем или раньше чем 400 дней назад.",
  "p.director_pack.draft_heading": "Подготовить черновик пакета",
  "p.director_pack.draft_says":
    "Подготовка читает планы, которыми вы можете делиться, в состоянии на момент отсечки и сохраняет результат как черновик. Ничего не отправляется аудитории, пока кто-то не выпустит пакет.",
  "p.director_pack.draft": "Подготовить черновик пакета",
  "p.director_pack.f_audience": "Аудитория",
  "p.director_pack.f_change_note": "Примечание об изменении",
  "p.director_pack.f_cut_off": "Отсечка (необязательно)",
  "p.director_pack.f_period": "Период (месяц)",
  "p.director_pack.from_schedule": "создан по ежемесячному расписанию",
  "p.director_pack.is_reviewed": "Проверен офисом портфеля.",
  "p.director_pack.is_stale": "устарел: его отсечка раньше конца его периода",
  "p.director_pack.issue_says":
    "Выпуск фиксирует пакет и уведомляет его аудиторию. Это нельзя отменить; последующее изменение — это повторный выпуск.",
  "p.director_pack.issue_waits":
    "Этот черновик создан по ежемесячному расписанию, поэтому перед выпуском его проверяет офис портфеля.",
  "p.director_pack.issue": "Выпустить пакет",
  "p.director_pack.issued_done": "Выпущено.",
  "p.director_pack.not_reviewed": "Ещё не проверен.",
  "p.director_pack.problem_audience": "Выберите аудиторию.",
  "p.director_pack.problem_cut_off_bad": "Укажите отсечку как дату и время.",
  "p.director_pack.problem_cut_off_future": "Отсечка не может быть в будущем.",
  "p.director_pack.problem_cut_off_old":
    "Отсечка не может быть раньше чем 400 дней назад.",
  "p.director_pack.problem_note":
    "Объясните, почему появилась новая версия, не менее чем в {min} символов.",
  "p.director_pack.problem_period": "Выберите период в виде месяца.",
  "p.director_pack.refused_edit":
    "Для подготовки, проверки и повторного выпуска пакета нужен доступ редактора пакетов для директоров.",
  "p.director_pack.refused_issue":
    "Для выпуска пакета нужен доступ выпускающего пакеты для директоров.",
  "p.director_pack.reissue_says":
    "Повторный выпуск готовит следующую версию с вашим примечанием; эта версия остаётся доступной для чтения как есть. Напишите не менее {min} символов.",
  "p.director_pack.reissue": "Выпустить повторно с примечанием об изменении",
  "p.director_pack.review": "Отметить как проверенный",
  "p.director_pack.reviewed_done": "Отмечено как проверенное.",
  "p.director_pack.add_schedule_heading": "Добавить расписание",
  "p.director_pack.add_schedule": "Добавить расписание",
  "p.director_pack.conflict_schedule":
    "У этой аудитории уже есть расписание; измените его день.",
  "p.director_pack.could_not_change_schedule": "Расписание не изменено.",
  "p.director_pack.could_not_load_schedules":
    "Не удалось загрузить расписания.",
  "p.director_pack.could_not_load_subscriptions":
    "Не удалось загрузить сведения о том, о каких пакетах вы получаете уведомления.",
  "p.director_pack.could_not_subscribe": "Изменение не сохранено.",
  "p.director_pack.day_says":
    "День от 1 до 28, чтобы он был в каждом месяце. Первый запуск — в ближайший такой день.",
  "p.director_pack.f_day": "День месяца",
  "p.director_pack.last_drafted":
    "Последний запуск {when}: подготовлен черновик пакета за {period}.",
  "p.director_pack.last_run": "Последний запуск {when}.",
  "p.director_pack.last_skipped":
    "Последний запуск {when}: пропущен {period}, для которого пакет уже существовал.",
  "p.director_pack.move_day": "Перенести на этот день",
  "p.director_pack.no_schedules":
    "Ни один пакет пока не готовится по расписанию.",
  "p.director_pack.problem_day": "Выберите день месяца от 1 до 28.",
  "p.director_pack.refused_subscribe":
    "Уведомления можно получать только для тех аудиторий, пакеты которых вы вправе читать.",
  "p.director_pack.schedule_added": "Расписание добавлено.",
  "p.director_pack.schedule_off":
    "Выключено. Черновик готовился бы в день {day} каждого месяца.",
  "p.director_pack.schedule_on":
    "Черновик готовится в день {day} каждого месяца; следующий раз — {next}.",
  "p.director_pack.schedule_saved": "Сохранено.",
  "p.director_pack.schedule_switched_off": "Выключено.",
  "p.director_pack.schedule_switched_on": "Включено.",
  "p.director_pack.schedule_unchanged":
    "Это уже его день, поэтому ничего не отправлено.",
  "p.director_pack.schedules_intro":
    "Каждое расписание готовит черновик пакета для аудитории в определённый день каждого месяца. Оно никогда не выпускает пакет: черновик проверяет офис портфеля, а выпускает человек.",
  "p.director_pack.schedules_link": "Ежемесячные расписания",
  "p.director_pack.schedules_title_ppm": "Ежемесячные расписания — PPM",
  "p.director_pack.schedules_title": "Ежемесячные расписания",
  "p.director_pack.subs_heading": "Уведомлять меня о выпуске пакета",
  "p.director_pack.subs_legend": "Пакеты, о которых уведомлять",
  "p.director_pack.subs_says":
    "Вы получаете уведомление в службе со ссылкой, когда выпускается пакет для выбранной вами аудитории. Само уведомление никогда не содержит пакет.",
  "p.director_pack.subscribed": "Вы будете получать уведомления.",
  "p.director_pack.sweep_heading": "Выполнить то, что подошло по сроку",
  "p.director_pack.sweep_result":
    "Подготовлено черновиков: {drafted}; пропущено, потому что пакет уже существовал: {skipped}.",
  "p.director_pack.sweep_says":
    "Готовит черновики всех пакетов, чей день наступил и которых ещё нет. Ничего не выпускается.",
  "p.director_pack.sweep": "Выполнить расписания, срок которых наступил",
  "p.director_pack.switch_off": "Выключить",
  "p.director_pack.switch_on": "Включить",
  "p.director_pack.unsubscribed": "Вы больше не будете получать уведомления.",
  "p.plans.pid.modules_failed":
    "Не удалось проверить, какие инструменты использует этот план, поэтому перечислены все инструменты.",
  "p._layout_svelte.handling_mark": "Гриф обращения: {mark}",
  "p._layout_svelte.handling_mark_unknown":
    "Гриф обращения: не удалось проверить. Считайте этот план имеющим гриф, пока проверка не выполнена.",
  "l.enum.cr_status.draft": "черновик",
  "l.enum.cr_status.submitted": "подан",
  "l.enum.cr_status.under_review": "на рассмотрении",
  "l.enum.cr_status.approved": "утверждён",
  "l.enum.cr_status.rejected": "отклонён",
  "l.enum.cr_status.implemented": "внедрён",
  "l.enum.cr_status.withdrawn": "отозван",
  "l.enum.cr_kind.scope": "содержание",
  "l.enum.cr_kind.schedule": "расписание",
  "l.enum.cr_kind.cost": "стоимость",
  "l.enum.cr_kind.quality": "качество",
  "l.enum.cr_kind.other": "другое",
  "p.plan_change_requests.title_ppm": "Запросы на изменение плана — PPM",
  "p.plan_change_requests.title": "Запросы на изменение плана",
  "p.plan_change_requests.intro":
    "Запросы на изменение этого плана, сначала новые. Составьте запрос и изменяйте его, пока он черновик; подайте его названному утверждающему, не вам; утверждающий рассматривает его и записывает решение с причиной. Открытый запрос можно отозвать.",
  "p.plan_change_requests.register_link": "Запросы на изменение по всем планам",
  "p.plan_change_requests.could_not_load":
    "Не удалось загрузить запросы на изменение плана",
  "p.plan_change_requests.could_not_save":
    "Не удалось сохранить запрос на изменение",
  "p.plan_change_requests.steps": "Шаги",
  "p.plan_change_requests.draft_heading": "Составить запрос на изменение",
  "p.plan_change_requests.edit_heading": "Изменить черновик",
  "p.plan_change_requests.f_description": "Описание (что должно измениться)",
  "p.plan_change_requests.f_reason": "Причина (зачем нужно изменение)",
  "p.plan_change_requests.f_days":
    "Влияние на расписание в рабочих днях (отрицательное значение приближает окончание; если не оценено, оставьте пустым)",
  "p.plan_change_requests.f_cost":
    "Влияние на стоимость (если не оценено, оставьте пустым)",
  "p.plan_change_requests.f_currency":
    "Валюта стоимости (три буквы, например EUR)",
  "p.plan_change_requests.f_summary": "Краткое описание влияния",
  "p.plan_change_requests.save_draft": "Сохранить черновик",
  "p.plan_change_requests.drafted": "Запрос на изменение составлен.",
  "p.plan_change_requests.saved": "Черновик сохранён.",
  "p.plan_change_requests.problem_title_required":
    "Укажите название запроса на изменение.",
  "p.plan_change_requests.problem_description_required":
    "Опишите, что должно измениться.",
  "p.plan_change_requests.problem_reason_required":
    "Объясните, зачем нужно изменение.",
  "p.plan_change_requests.problem_days_not_whole":
    "Влияние на расписание — целое число рабочих дней.",
  "p.plan_change_requests.problem_days_out_of_range":
    "Влияние на расписание должно быть от -3650 до 3650 рабочих дней.",
  "p.plan_change_requests.problem_cost_not_amount":
    "Влияние на стоимость не является суммой.",
  "p.plan_change_requests.problem_cost_needs_currency":
    "Для влияния на стоимость нужна валюта.",
  "p.plan_change_requests.problem_currency_needs_cost":
    "Для валюты нужно влияние на стоимость.",
  "p.plan_change_requests.problem_currency_invalid":
    "Валюта — трёхбуквенный код, например EUR, JPY или GBP.",
  "p.plan_change_requests.steps_heading": "Шаги для «{title}»",
  "p.plan_change_requests.status_now": "Сейчас статус: {status}.",
  "p.plan_change_requests.decided": "Решил {who}, {on}: {reason}",
  "p.plan_change_requests.no_steps":
    "По этому запросу на изменение больше нельзя выполнить ни одного шага.",
  "p.plan_change_requests.who_note":
    "Служба определяет, кто может выполнить каждый шаг: изменять, подавать или отзывать запрос может только составивший его (отозвать может и администратор), а рассматривать или решать — только названный утверждающий.",
  "p.plan_change_requests.submit_heading": "Подать на решение",
  "p.plan_change_requests.f_approver":
    "Утверждающий (ссылка вида person: или worker:, не составивший запрос)",
  "p.plan_change_requests.submit": "Подать",
  "p.plan_change_requests.submitted": "Запрос на изменение подан.",
  "p.plan_change_requests.problem_approver_not_reference":
    "Укажите утверждающего ссылкой вида person: или worker:.",
  "p.plan_change_requests.problem_approver_is_requester":
    "Утверждающим не может быть тот, кто запросил изменение.",
  "p.plan_change_requests.review": "Начать рассмотрение",
  "p.plan_change_requests.review_started": "Рассмотрение начато.",
  "p.plan_change_requests.decide_heading": "Записать решение",
  "p.plan_change_requests.f_decision": "Решение",
  "p.plan_change_requests.approve": "Утвердить",
  "p.plan_change_requests.reject": "Отклонить",
  "p.plan_change_requests.f_decision_reason":
    "Причина решения (не менее 10 символов)",
  "p.plan_change_requests.decide": "Записать решение",
  "p.plan_change_requests.decision_recorded": "Решение записано.",
  "p.plan_change_requests.problem_decision_reason":
    "Для решения нужна причина не менее чем в 10 символов.",
  "p.plan_change_requests.withdraw": "Отозвать запрос на изменение",
  "p.plan_change_requests.withdrawn": "Запрос на изменение отозван.",
  "p.plan_change_requests.close_steps": "Закрыть шаги",
  "p.plan_change_requests.refused_requester":
    "Это может сделать только составивший этот запрос на изменение.",
  "p.plan_change_requests.refused_withdraw":
    "Отозвать этот запрос на изменение может только составивший его или администратор.",
  "p.plan_change_requests.refused_approver":
    "Рассматривать или решать этот запрос на изменение может только названный утверждающий.",
  "p.plan_change_requests.refused_state":
    "Этот запрос на изменение продвинулся дальше, и этот шаг для него больше недоступен; список прочитан заново.",
  "p.plan_change_requests.refused_invalid":
    "Служба не приняла введённое: проверьте поля и повторите попытку.",
  "p.change_requests.change_requests_ppm": "Запросы на изменение — PPM",
  "p.change_requests.change_requests": "Запросы на изменение",
  "p.change_requests.intro":
    "Запрос на изменение — это контролируемое предложение изменить утверждённый базовый план. Один названный утверждающий, не являющийся автором запроса, принимает решение и указывает причину. Влияние — это то, что указал автор запроса; прогноз показывается только при приложенных правках «что если».",
  "p.change_requests.could_not_load":
    "Не удалось загрузить запросы на изменение",
  "p.change_requests.by_status": "Запросы на изменение по статусам",
  "p.change_requests.awaiting": "Ожидают решения",
  "p.change_requests.oldest_open":
    "Самый давний открытый: {title} ({days} календарных дн.)",
  "p.change_requests.show": "Показать",
  "p.change_requests.all_statuses": "Все статусы",
  "p.change_requests.open_only": "Только открытые",
  "p.change_requests.none": "Запросов на изменение не зарегистрировано.",
  "p.change_requests.none_match": "Подходящих запросов на изменение нет.",
  "p.change_requests.loading": "Загрузка…",
  "p.change_requests.count_one": "{n} запрос на изменение, сначала новые",
  "p.change_requests.count_other": "Запросов на изменение: {n}, сначала новые",
  "p.change_requests.title_col": "Запрос на изменение",
  "p.change_requests.kind_col": "Вид",
  "p.change_requests.status_col": "Статус",
  "p.change_requests.impact_col": "Заявленное влияние",
  "p.change_requests.age_col": "Возраст (календарных дней)",
  "p.change_requests.approver_col": "Утверждающий",
  "p.change_requests.no_approver": "Пока не назван",
  "p.change_requests.no_impact": "Влияние не указано",
  "p.change_requests.impact_days": "{days} рабочих дн.",
  "p.change_requests.plan_link": "Открыть запросы на изменение плана",
  "p.director_pack.director_packs_ppm": "Пакеты для директоров — PPM",
  "p.director_pack.director_packs": "Пакеты для директоров",
  "p.director_pack.intro":
    "Пакет для директоров — это месячный отчёт директорам: он готовится один раз, выпускается названным человеком и затем фиксируется. Выпущенный пакет никогда не меняется, а исправление — это новая версия с примечанием. Каждая цифра ниже прочитана из выпущенного пакета; здесь ничего не пересчитывается.",
  "p.director_pack.could_not_load":
    "Не удалось загрузить пакеты для директоров",
  "p.director_pack.could_not_load_one":
    "Не удалось загрузить этот пакет для директоров",
  "p.director_pack.loading": "Загрузка…",
  "p.director_pack.none": "Вам не доступен ни один пакет для директоров.",
  "p.director_pack.col_period": "Период",
  "p.director_pack.col_audience": "Аудитория",
  "p.director_pack.col_version": "Версия",
  "p.director_pack.col_status": "Статус",
  "p.director_pack.col_issued": "Выпущен",
  "p.director_pack.status_draft": "черновик",
  "p.director_pack.status_issued": "выпущен",
  "p.director_pack.open_pack":
    "Открыть пакет за {period}, {audience}, версия {version}",
  "p.director_pack.all_packs": "Все пакеты для директоров",
  "p.director_pack.pack_heading": "Пакет для директоров {period} — {audience}",
  "p.director_pack.version_line":
    "Версия {version} · {status} · отсечка {cutoff}",
  "p.director_pack.issued_on": "Выпущен {when}",
  "p.director_pack.draft_note":
    "Это черновик. Он ещё не выпущен, поэтому не зафиксирован.",
  "p.director_pack.change_note": "Примечание об изменении: {note}",
  "p.director_pack.previous_pack":
    "В сравнении с пакетом за {period}, версия {version}.",
  "p.director_pack.download_pdf": "Скачать в формате PDF",
  "p.director_pack.pdf_note":
    "В PDF те же цифры, но его постоянные слова — на английском. Чтобы сохранить другой язык, распечатайте эту страницу.",
  "p.director_pack.basis":
    "Записи приведены в том состоянии, в каком они были при подготовке пакета, с датами не позже отсечки. У проблем и запросов на изменение нет истории статусов, поэтому закрытые после отсечки, но до подготовки показаны как закрытые. Раздел, в котором нечего показать, сообщает об этом; отсутствие никогда не показывается как ноль или зелёный.",
  "p.director_pack.s_quality": "Качество данных",
  "p.director_pack.s_changed": "Что изменилось",
  "p.director_pack.s_map": "Карта портфеля",
  "p.director_pack.s_exceptions": "Исключения",
  "p.director_pack.s_gates": "Решения по контрольным точкам",
  "p.director_pack.s_funding": "Финансирование",
  "p.director_pack.s_fever": "Диаграмма «лихорадки» буфера",
  "p.director_pack.s_evm": "Освоенный объём",
  "p.director_pack.s_benefits": "Выгоды",
  "p.director_pack.s_capacity": "Мощность",
  "p.director_pack.s_issues": "Проблемы",
  "p.director_pack.s_changes": "Запросы на изменение",
  "p.director_pack.plans_included": "Включено планов: {n}",
  "p.director_pack.plans_left_out":
    "Планов пропущено, так как они закрытые или с грифом: {n}",
  "p.director_pack.sections_missing": "Разделы, которые ещё не созданы: {list}",
  "p.director_pack.all_sections": "Все разделы пакета присутствуют.",
  "p.director_pack.nothing_here": "В этом разделе нечего показать.",
  "p.director_pack.changed_issues": "Открытые проблемы: {now}, {change}",
  "p.director_pack.changed_changes":
    "Запросы на изменение, ожидающие решения: {now}, {change}",
  "p.director_pack.changed_was": "было {before}",
  "p.director_pack.changed_unchanged": "без изменений",
  "p.director_pack.changed_first": "нет более раннего пакета для сравнения",
  "p.director_pack.c_programme": "Программа",
  "p.director_pack.c_aim": "Стратегическая цель",
  "p.director_pack.c_status": "Статус",
  "p.director_pack.c_stage": "Этап",
  "p.director_pack.c_phase": "Фаза",
  "p.director_pack.c_confidence": "Уверенность в поставке",
  "p.director_pack.c_trend": "Тенденция",
  "p.director_pack.self_assessed": "самооценка",
  "p.director_pack.stale": "устарело",
  "p.director_pack.no_aim": "Нет стратегической цели",
  "p.director_pack.c_plan": "План",
  "p.director_pack.c_worst": "Худшая оценка",
  "p.director_pack.c_need": "Необходимое решение",
  "p.director_pack.c_owner_role": "Ответственный (роль)",
  "p.director_pack.c_decide_by": "Принять решение до",
  "p.director_pack.c_hours_open": "Часов открыто",
  "p.director_pack.ex_window":
    "Исключение, не заполненное в течение {hours} ч после открытия, считается нарушившим срок.",
  "p.director_pack.ex_open": "Открыто: {n}",
  "p.director_pack.ex_breached": "Срок нарушен: {n}",
  "p.director_pack.ex_closed": "Закрыто за период: {n}",
  "p.director_pack.g_passed": "Пройдено: {n}",
  "p.director_pack.g_failed": "Не пройдено: {n}",
  "p.director_pack.g_deferred": "Отложено: {n}",
  "p.director_pack.g_paused": "Приостановленные программы: {n}",
  "p.director_pack.g_stopped": "Остановленные программы: {n}",
  "p.director_pack.c_gate": "Контрольная точка",
  "p.director_pack.c_decided": "Решено",
  "p.director_pack.c_planned": "Запланировано",
  "p.director_pack.c_reason": "Причина",
  "p.director_pack.c_days_late": "Дней опоздания",
  "p.director_pack.c_since": "С",
  "p.director_pack.c_allocs": "Помечено распределений",
  "p.director_pack.reason_hold": "проверено как «удержание»",
  "p.director_pack.reason_no_review": "плановая дата прошла без проверки",
  "p.director_pack.fy_line":
    "Финансовый год {label}, с {start} по {end}. Капитальные и операционные расходы показаны отдельно, а валюты никогда не суммируются.",
  "p.director_pack.c_source": "Источник финансирования",
  "p.director_pack.c_class": "Класс",
  "p.director_pack.c_budget": "Бюджет",
  "p.director_pack.c_spend": "Расходы на сегодня",
  "p.director_pack.c_variance": "Отклонение",
  "p.director_pack.no_funding_source": "нет источника финансирования",
  "p.director_pack.forecast_line":
    "Прогноз на весь жизненный цикл в {currency}: факт {actual}, оценка по завершении {eac}",
  "p.director_pack.unfunded_lines":
    "Статей без источника финансирования: {n}. Статей без дат: {dated}.",
  "p.director_pack.c_chain": "Цепь выполнена (%)",
  "p.director_pack.c_buffer": "Буфер израсходован (%)",
  "p.director_pack.c_zone": "Зона",
  "p.director_pack.c_slip": "Сдвиг (дней)",
  "p.director_pack.c_buffer_days": "Буфер (дней)",
  "p.director_pack.c_previous": "Предыдущий пакет",
  "p.director_pack.edge_below": "восстановление: нарисовано на нижней границе",
  "p.director_pack.edge_above": "перерасход: нарисовано на верхней границе",
  "p.director_pack.fever_none_no_buffer":
    "нет точки на диаграмме «лихорадки»: буфер проекта не объявлен",
  "p.director_pack.fever_none_no_baseline":
    "нет точки на диаграмме «лихорадки»: нет базового плана расписания",
  "p.director_pack.fever_none_no_chain":
    "нет точки на диаграмме «лихорадки»: нет критической работы с длительностью, которую можно измерить",
  "p.director_pack.fever_none_no_forecast":
    "нет точки на диаграмме «лихорадки»: сейчас расписание нельзя спрогнозировать",
  "p.director_pack.fever_lines":
    "Линии зон (израсходованный буфер, % при выполнении цепи на 0 % и на 100 %): от зелёной к янтарной {ga0}–{ga1}; от янтарной к красной {ar0}–{ar1}.",
  "p.director_pack.c_bac": "Бюджет по завершении",
  "p.director_pack.c_ev": "Освоенный объём",
  "p.director_pack.c_ac": "Фактическая стоимость",
  "p.director_pack.c_spi": "SPI",
  "p.director_pack.c_cpi": "CPI",
  "p.director_pack.below_threshold":
    "Ниже порога освоенного объёма: бюджет по завершении {bac}",
  "p.director_pack.no_baseline": "нет базового плана",
  "p.director_pack.evm_ratio_note":
    "SPI и CPI показаны с двумя знаками; пусто означает «не определено», а не ноль.",
  "p.director_pack.c_recurrence": "Повторяемость",
  "p.director_pack.c_classification": "Классификация",
  "p.director_pack.c_claimed": "Заявлено",
  "p.director_pack.c_validated": "Подтверждено",
  "p.director_pack.c_unvalidated": "Заявлено, не подтверждено",
  "p.director_pack.benefits_note":
    "Реализованная, но не подтверждённая выгода — это заявление, а не подтверждённая выгода. Повторяющиеся суммы — годовые и никогда не складываются с разовыми.",
  "p.director_pack.c_pool": "Пул навыков",
  "p.director_pack.c_demand": "Спрос (FTE)",
  "p.director_pack.c_supply": "Предложение (FTE)",
  "p.director_pack.c_gap": "Разрыв (FTE)",
  "p.director_pack.c_month": "Месяц",
  "p.director_pack.c_reading": "Показание",
  "p.director_pack.no_pool": "пул не назначен",
  "p.director_pack.capacity_note":
    "В каждой ячейке есть слово и символ, а не только цвет. Отсутствие записанного предложения — это не нулевая мощность.",
  "p.director_pack.partners_short": "поставки партнёров ниже обязательств",
  "p.director_pack.open_issues": "Открытые проблемы: {n}",
  "p.director_pack.awaiting": "Запросы на изменение, ожидающие решения: {n}",
  "p.director_pack.dash": "не записано",
  "l.enum.pk_conf.green": "зелёный",
  "l.enum.pk_conf.amber_green": "янтарный/зелёный",
  "l.enum.pk_conf.amber": "янтарный",
  "l.enum.pk_conf.amber_red": "янтарный/красный",
  "l.enum.pk_conf.red": "красный",
  "l.enum.pk_conf.not_assessed": "не оценено",
  "l.enum.pk_trend.up": "улучшается",
  "l.enum.pk_trend.down": "ухудшается",
  "l.enum.pk_trend.same": "без изменений",
  "l.enum.pk_trend.new": "оценено впервые",
  "l.enum.pk_trend.not_assessed": "не оценено",
  "l.enum.pk_trend.no_earlier_pack": "нет более раннего пакета",
  "l.enum.pk_status.active": "активна",
  "l.enum.pk_status.paused": "приостановлена",
  "l.enum.pk_status.stopped": "остановлена",
  "l.enum.pk_zone.green": "зелёная",
  "l.enum.pk_zone.amber": "янтарная",
  "l.enum.pk_zone.red": "красная",
  "l.enum.pk_heat.shortfall": "нехватка",
  "l.enum.pk_heat.tight": "впритык",
  "l.enum.pk_heat.enough": "достаточно",
  "l.enum.pk_heat.no_supply_recorded": "предложение не записано",
  "l.enum.pk_heat.no_demand": "нет спроса",
  "l.enum.pk_heat.nothing_recorded": "ничего не записано",
  "l.enum.pk_class.capex": "капитальные",
  "l.enum.pk_class.opex": "операционные",
  "l.enum.pk_class.all": "капитальные и операционные вместе",
  "l.enum.pk_recur.recurring": "повторяющаяся",
  "l.enum.pk_recur.one_off": "разовая",
  "l.enum.pk_recur.not_stated": "не указано",
  "l.enum.pk_cls.cash_releasing": "высвобождающая денежные средства",
  "l.enum.pk_cls.non_cash_releasing": "не высвобождающая денежные средства",
  "l.enum.pk_cls.societal": "общественная",
  "l.enum.pk_cls.not_stated": "не указано",
  "l.enum.pk_gate.g0_concept": "G0 концепция",
  "l.enum.pk_gate.g1_feasibility": "G1 технико-экономическое обоснование",
  "l.enum.pk_gate.g2_definition": "G2 определение",
  "l.enum.pk_gate.g3_delivery": "G3 поставка",
  "l.enum.pk_gate.g4_launch": "G4 запуск",
  "l.enum.pk_gate.g5_benefits": "G5 выгоды",
  "p.director_pack.c_currency": "Валюта",
  "p.critical_chain.chain_ppm": "Критическая цепь и буферы — PPM",
  "p.critical_chain.buffers_ppm": "Управление буферами — PPM",
  "p.critical_chain.pipeline_ppm": "Барабан и конвейер — PPM",
  "p.critical_chain.loading": "Загрузка…",
  "p.critical_chain.yes": "да",
  "p.critical_chain.no": "нет",
  "p.critical_chain.zone_green": "зелёная",
  "p.critical_chain.zone_amber": "янтарная",
  "p.critical_chain.zone_red": "красная",
  "p.critical_chain.zone_none": "нет показания",
  "p.critical_chain.outcome_as_planned": "сохраняет свои даты",
  "p.critical_chain.outcome_staggered": "начать позже",
  "p.critical_chain.outcome_exceeds_drum":
    "требует больше, чем есть у барабана",
  "p.critical_chain.outcome_no_slot": "ни один старт не подходит",
  "p.critical_chain.advice":
    "Это рекомендация. Ничего не изменено, а решение о действиях принимает человек.",
  "p.critical_chain.link_plan": "План",
  "p.critical_chain.link_buffers": "Управление буферами",
  "p.critical_chain.link_chain": "Критическая цепь",
  "p.critical_chain.link_critical_path": "Критический путь",
  "p.critical_chain.nav_pipeline": "Барабан и конвейер",
  "p.critical_chain.chain_h1": "Критическая цепь и буферы",
  "p.critical_chain.chain_lede":
    "Критическая цепь — это самая длинная последовательность задач с учётом времени людей: она следует зависимостям и ожиданию человека, занятого другой задачей. Она показана рядом с критическим путём, который следует только зависимостям; они никогда не смешиваются.",
  "p.critical_chain.chain_h2": "Критическая цепь",
  "p.critical_chain.no_chain": "Критической цепи нет: {reason}",
  "p.critical_chain.reason_no_assignments":
    "никто не назначен на незавершённую задачу, поэтому нет ожиданий людей, которым можно следовать",
  "p.critical_chain.reason_no_scheduled_tasks":
    "ни у одной задачи нет расписания",
  "p.critical_chain.chain_work": "Рабочих дней работы в цепи: {days}",
  "p.critical_chain.chain_tasks_count": "Задач в цепи: {tasks}",
  "p.critical_chain.chain_finish":
    "С учётом времени людей план завершится {finish}.",
  "p.critical_chain.chain_delay_none":
    "Это не позже окончания по критическому пути.",
  "p.critical_chain.chain_delay":
    "Рабочих дней позже окончания по критическому пути: {days}",
  "p.critical_chain.crosses_yes": "Цепь переходит между путями: человек общий.",
  "p.critical_chain.crosses_no":
    "Цепь остаётся на одном пути: общих людей нет.",
  "p.critical_chain.tied":
    "Другая цепь такой же длины; показана эта, потому что она идёт первой в плане.",
  "p.critical_chain.timeline_label": "Цепь на временной шкале",
  "p.critical_chain.timeline_caption":
    "Каждая сплошная полоса — задача цепи. Пунктирная полоса — буфер проекта. Таблица ниже содержит те же цифры.",
  "p.critical_chain.col_task": "Задача",
  "p.critical_chain.col_days": "Рабочих дней",
  "p.critical_chain.col_start": "Начало",
  "p.critical_chain.col_finish": "Окончание",
  "p.critical_chain.col_held_by": "Удерживается",
  "p.critical_chain.col_on_path": "На критическом пути",
  "p.critical_chain.held_first": "первая в цепи",
  "p.critical_chain.held_dep": "зависимостью",
  "p.critical_chain.held_person": "человеком: {person}",
  "p.critical_chain.held_person_hidden": "человеком (скрыт)",
  "p.critical_chain.cp_h2": "Рядом — критический путь",
  "p.critical_chain.cp_finish": "Критический путь заканчивается {finish}.",
  "p.critical_chain.cp_none": "Критического пути для показа нет.",
  "p.critical_chain.buffers_h2": "Буферы",
  "p.critical_chain.buffers_lede":
    "Задачи планируются по агрессивным оценкам; запас, вынутый из каждой задачи, объединяется в буфер. Размеры указаны в рабочих днях. Правило: {rule}.",
  "p.critical_chain.rule_cut_and_paste":
    "половина удалённого запаса (вырезать и вставить)",
  "p.critical_chain.rule_root_sum_squares":
    "квадратный корень из суммы квадратов удалённого запаса",
  "p.critical_chain.no_buffers": "Буферов для показа нет: {reason}",
  "p.critical_chain.reason_estimates_missing":
    "у некоторых задач цепи пока нет агрессивной и безопасной оценки. Запишите их через службу оценок; экрана для этого пока нет.",
  "p.critical_chain.missing_h3": "Задачи цепи без обеих оценок",
  "p.critical_chain.project_buffer":
    "Буфер проекта: {days} рабочих дн., защищает цепь, заканчивающуюся {finish}. С буфером дата — {with}.",
  "p.critical_chain.project_safety":
    "Запас, вынутый из задач цепи: {days} рабочих дн.",
  "p.critical_chain.project_buffer_bar": "Буфер проекта",
  "p.critical_chain.feeding_h3": "Подпитывающие буферы",
  "p.critical_chain.feeding_none":
    "Никакая другая цепь задач не присоединяется к критической цепи.",
  "p.critical_chain.col_feeding_tasks": "Подпитывающие задачи",
  "p.critical_chain.col_joins": "Присоединяется к",
  "p.critical_chain.col_buffer": "Буфер (рабочих дней)",
  "p.critical_chain.col_room": "Запас до присоединения (рабочих дней)",
  "p.critical_chain.col_push": "Сдвинет цепь",
  "p.critical_chain.push_yes": "да, на {days} рабочих дн.",
  "p.critical_chain.unsized": "размер не задан: не хватает оценки",
  "p.critical_chain.resource_h3": "Ресурсные буферы (оповещения о готовности)",
  "p.critical_chain.resource_lede":
    "Ресурсный буфер — это не время. Это напоминание о том, что человек должен быть готов до начала его задачи в цепи, за {lead} рабочих дн.",
  "p.critical_chain.col_person": "Человек",
  "p.critical_chain.col_alert_on": "Быть готовым к",
  "p.critical_chain.col_task_starts": "Задача начинается",
  "p.critical_chain.hidden": "(скрыто)",
  "p.critical_chain.likely_h3":
    "Запланированы по вероятной длительности (нет агрессивной оценки)",
  "p.critical_chain.could_not_load_chain":
    "Не удалось загрузить критическую цепь",
  "p.critical_chain.buffers_h1": "Управление буферами",
  "p.critical_chain.buffers_lede2":
    "Каждый буфер сопоставляется с текущим расписанием: какая часть цепи выполнена и какую часть буфера израсходовал прогнозируемый сдвиг. Буфер в красной зоне требует корректирующего действия, которое человек согласует и записывает.",
  "p.critical_chain.no_plan":
    "Для этого плана не зафиксирован план буферов. Зафиксируйте его, когда цепь и буферы согласованы; последующие показания измеряются относительно него.",
  "p.critical_chain.freeze_confirm":
    "Я прочитал цепь и буферы и хочу зафиксировать их как план",
  "p.critical_chain.freeze_button": "Зафиксировать план буферов",
  "p.critical_chain.frozen_on":
    "Измерено относительно плана буферов, зафиксированного {date}.",
  "p.critical_chain.alerts_h2": "Оповещения",
  "p.critical_chain.alerts_none": "Ни один буфер не в красной зоне.",
  "p.critical_chain.alert_project":
    "Буфер проекта в красной зоне: израсходовано {consumed}%. Цепь: {chain}.",
  "p.critical_chain.alert_feeding":
    "Подпитывающий буфер в красной зоне: израсходовано {consumed}%. Цепь: {chain}, присоединяется к {joins}.",
  "p.critical_chain.needs_action":
    "Открытого корректирующего действия не записано для: {buffers}.",
  "p.critical_chain.charts_h2": "Диаграммы «лихорадки»",
  "p.critical_chain.project_label": "Буфер проекта",
  "p.critical_chain.feeding_label": "Подпитывающий буфер: {chain}",
  "p.critical_chain.axis_x": "Цепь выполнена (%)",
  "p.critical_chain.axis_y": "Буфер израсходован (%)",
  "p.critical_chain.chart_caption":
    "{buffer}: {zone}; израсходовано {consumed}% буфера; выполнено {done}% цепи.",
  "p.critical_chain.beyond_edge":
    "Истинное значение за пределами диаграммы, поэтому точка нарисована на границе.",
  "p.critical_chain.table_h2": "Те же цифры в виде таблицы",
  "p.critical_chain.col_size": "Буфер (рабочих дней)",
  "p.critical_chain.col_done": "Цепь выполнена (%)",
  "p.critical_chain.col_used": "Буфер израсходован (%)",
  "p.critical_chain.col_slip": "Прогнозируемый сдвиг (рабочих дней)",
  "p.critical_chain.col_zone": "Зона",
  "p.critical_chain.no_point": "Нет показания: {reason}",
  "p.critical_chain.actions_h2": "Корректирующие действия",
  "p.critical_chain.actions_none":
    "Ни одного корректирующего действия не записано.",
  "p.critical_chain.col_buffer_name": "Буфер",
  "p.critical_chain.col_action": "Действие",
  "p.critical_chain.col_owner": "Ответственный",
  "p.critical_chain.col_due": "Срок",
  "p.critical_chain.col_status": "Статус",
  "p.critical_chain.status_open": "открыто",
  "p.critical_chain.status_done": "выполнено",
  "p.critical_chain.status_dropped": "отброшено",
  "p.critical_chain.mark_done": "Отметить выполненным",
  "p.critical_chain.drop_action": "Отбросить",
  "p.critical_chain.add_action_h3": "Записать корректирующее действие",
  "p.critical_chain.f_buffer": "Буфер",
  "p.critical_chain.f_action": "Что будет сделано",
  "p.critical_chain.f_owner": "Ответственный (ссылка на человека)",
  "p.critical_chain.f_due": "Срок (необязательно)",
  "p.critical_chain.add_button": "Записать действие",
  "p.critical_chain.snap_h2": "История",
  "p.critical_chain.snap_button": "Записать снимок сейчас",
  "p.critical_chain.snap_none": "Ни одного снимка не записано.",
  "p.critical_chain.col_snap_date": "Записан",
  "p.critical_chain.could_not_load_buffers":
    "Не удалось загрузить управление буферами",
  "p.critical_chain.could_not_save": "Не удалось сохранить",
  "p.critical_chain.pipeline_h1": "Барабан и конвейер",
  "p.critical_chain.pipeline_lede":
    "Барабан — это пул навыков, от которого портфель зависит больше всего. Проекты, которым он нужен одновременно, разносятся по времени: служба предлагает более поздние старты с буфером мощности между проектами и объясняет каждый. Она никогда не переносит дату.",
  "p.critical_chain.drum_h2": "Барабан",
  "p.critical_chain.drum_named": "Барабан — {name}: назван офисом портфеля.",
  "p.critical_chain.drum_overloaded":
    "Барабан — {name}: пул не назван, а у этого самая большая нехватка.",
  "p.critical_chain.no_drum": "Барабана нет: {why}",
  "p.critical_chain.set_h3": "Назвать барабан",
  "p.critical_chain.pick_pool": "Пул навыков",
  "p.critical_chain.set_button": "Назвать его барабаном",
  "p.critical_chain.clear_button": "Снять назначение",
  "p.critical_chain.capacity":
    "Мощность барабана: {fte} FTE. Буфер мощности между проектами: {days} дн.",
  "p.critical_chain.no_capacity":
    "У барабана в этом окне нет записанного предложения, поэтому разносить не по чему.",
  "p.critical_chain.projects_h2": "Предлагаемые старты",
  "p.critical_chain.projects_none":
    "В этом окне ни одному проекту барабан не нужен.",
  "p.critical_chain.pipeline_timeline":
    "Использование барабана сейчас и по предложению",
  "p.critical_chain.pipeline_timeline_caption":
    "Для каждого проекта верхняя полоса показывает, когда он использует барабан сейчас, а нижняя — когда использовал бы по предложению. Таблица ниже содержит те же даты.",
  "p.critical_chain.bar_now": "сейчас",
  "p.critical_chain.bar_proposed": "предложено",
  "p.critical_chain.col_project": "Проект",
  "p.critical_chain.col_needs": "Нужен барабан",
  "p.critical_chain.col_peak": "Пик (FTE)",
  "p.critical_chain.col_proposal": "Предложение",
  "p.critical_chain.col_proposed": "Предлагаемое первое использование",
  "p.critical_chain.col_delay": "Позже на (дней)",
  "p.critical_chain.col_why": "Почему",
  "p.critical_chain.could_not_load_pipeline": "Не удалось загрузить конвейер",
  "p.start_queue.nav": "Очередь запуска",
  "p.start_queue.title": "Очередь запуска — PPM",
  "p.start_queue.h1": "Очередь запуска",
  "p.start_queue.lede":
    "Программы, которые запросили начало поставки и получили отказ, в порядке запроса. Программе отказывают, когда лимит незавершённой работы заполнен или когда в пуле ограничений в ближайшие месяцы не хватает предложения. Порядок никогда не меняется, а старт всё равно можно выполнить с записанным переопределением и причиной.",
  "p.start_queue.loading": "Загрузка очереди…",
  "p.start_queue.none": "Ни одна программа не ждёт запуска.",
  "p.start_queue.col_pos": "Место",
  "p.start_queue.col_prog": "Программа",
  "p.start_queue.col_asked": "Запрошено",
  "p.start_queue.col_why": "Почему отказано",
  "p.start_queue.col_act": "Действие",
  "p.start_queue.limit":
    "Все места ({limit}) заняты: программ в поставке — {in_delivery}.",
  "p.start_queue.pool":
    "В пуле ограничений {pool} не хватает предложения в: {months}.",
  "p.start_queue.withdraw": "Снять {name} с очереди",
  "p.start_queue.withdraw_short": "Снять",
  "p.start_queue.could_not_load": "Не удалось загрузить очередь запуска",
  "p.start_queue.could_not_withdraw": "Не удалось снять с очереди",
  "p.table.search": "Фильтр строк",
  "p.table.export": "Экспорт",
  "p.table.find": "Поиск",
  "p.table.moved": "«{header}» перемещён на позицию {position} из {count}",
  "p.table.copy": "Копировать",
  "p.table.copied": "Строк скопировано в буфер обмена: {rows}",
  "p.table.copy_failed": "Не удалось скопировать в буфер обмена",
  "p.table.matches": "Показано {matches} из {total}",
  "p.table.search_in": "Искать в",
  "p.table.all_columns": "Все столбцы",
  "p.table.pages": "Страницы",
  "p.table.export_csv": "CSV",
  "p.table.export_tsv": "TSV",
  "p.table.export_excel": "Таблица Excel",
  "p.table.exported": "Строк экспортировано в {file}: {rows}",
  "p.table.columns": "Столбцы",
  "p.table.show_column": "Показать «{header}»",
  "p.table.resize": "Изменить ширину столбца «{header}»",
  "p.table.select_all": "Выбрать все строки",
  "p.table.select_col": "Выбор",
  "p.table.select_row": "Выбрать {row}",
  "p.table.prev": "Предыдущая страница",
  "p.table.next": "Следующая страница",
  "p.table.page_status": "Страница {page} из {pages} (строк: {rows})",
  "p.table.page_announce": "Страница {page} из {pages}",
  "p.table.sorted": "«{header}»: сортировка {direction}",
  "p.table.asc": "по возрастанию",
  "p.table.desc": "по убыванию",
  "p.table.none": "без сортировки",
  "p.table.filtered": "Подходящих строк: {matches} из {total}",
  "p.table.selected": "Выбрано строк: {n}",
  "p.table.empty": "Нечего показать.",
  "p.demo.tour_title": "Демонстрационный обзор — PPM",
  "p.demo.tour_h1": "Демонстрационный обзор",
  "p.demo.tour_lede":
    "Вы вошли как Example Person. Всё здесь вымышлено: Транспортное управление Нортгейта, его сотрудники, деньги и даты придуманы для этой демонстрации. Выберите любую возможность ниже и воспользуйтесь ею: можно что-то менять, ничто здесь не настоящее.",
  "p.demo.tour_story_h2": "История",
  "p.demo.tour_story":
    "Транспортное управление Нортгейта модернизирует железнодорожную сеть: новая сигнализация, станции без ступенек и цифровое депо. Десять планов, восемь человек, четыре организации и несколько намеренно возникающих неполадок, чтобы экранам было что показать.",
  "p.demo.tour_open": "Открыть",
  "p.demo.tour_no_plan":
    "План «Северная линия» не найден, поэтому ссылки на его страницы скрыты. Запустите заполнение данными снова.",
  "p.demo.tour_toc": "Возможности",
  "p.demo.g_overview": "Портфель с первого взгляда",
  "p.demo.g_delivery":
    "Реализация одного проекта (взаимная блокировка Северной линии)",
  "p.demo.g_risk": "Риски, проблемы и изменения",
  "p.demo.g_money": "Деньги",
  "p.demo.g_people": "Люди и мощность",
  "p.demo.g_strategy": "Стратегия и приём работ",
  "p.demo.g_escalation": "Управление и эскалация",
  "p.demo.g_chain": "Критическая цепь",
  "p.demo.g_tools": "Инструменты",
  "p.demo.f_kanban_t": "Моя канбан-доска",
  "p.demo.f_kanban_d":
    "Ваши открытые задачи из всех проектов в столбцах по статусам. Перемещайте задачу мышью или через её меню; статус задачи изменится для всех.",
  "p.demo.f_me_t": "Моя работа",
  "p.demo.f_me_d":
    "Те же задачи по сроку в вашем часовом поясе: просроченные, сегодня, на этой неделе, позже.",
  "p.demo.f_plans_t": "Планы",
  "p.demo.f_plans_d":
    "Десять вымышленных планов: один портфель, три программы и шесть проектов Транспортного управления Нортгейта. Откройте любой.",
  "p.demo.f_dashboard_t": "Панель портфеля",
  "p.demo.f_dashboard_d":
    "Статус по видам и этапам с числом красных, янтарных и зелёных.",
  "p.demo.f_gantt_t": "Диаграмма Ганта",
  "p.demo.f_gantt_d": "Расписание каждого проекта на одной временной шкале.",
  "p.demo.f_milestones_t": "Вехи",
  "p.demo.f_milestones_d":
    "Утверждение проекта, поставка оборудования и запуск для каждого проекта.",
  "p.demo.f_calendar_t": "Календарь",
  "p.demo.f_calendar_d": "Вехи и сроки по месяцам.",
  "p.demo.f_board_t": "Доска задач",
  "p.demo.f_board_d":
    "Восемь задач от обследования до передачи в столбцах по статусам. Одна заблокирована из-за подтверждения безопасности.",
  "p.demo.f_critical_path_t": "Критический путь",
  "p.demo.f_critical_path_d":
    "Цепь зависимых задач, определяющая дату окончания.",
  "p.demo.f_earned_value_t": "Освоенный объём",
  "p.demo.f_earned_value_d":
    "Плановый объём, освоенный объём и стоимость с индексами расписания и стоимости.",
  "p.demo.f_schedule_health_t": "Состояние расписания",
  "p.demo.f_schedule_health_d":
    "Проверки логики плана: отсутствующие связи, длинные задачи, ограничения.",
  "p.demo.f_wbs_t": "Структура декомпозиции работ",
  "p.demo.f_wbs_d": "Проект, разбитый на пакеты работ.",
  "p.demo.f_risk_t": "Реестр рисков",
  "p.demo.f_risk_d":
    "Три риска по каждому проекту с оценкой вероятности и воздействия.",
  "p.demo.f_issues_t": "Проблемы",
  "p.demo.f_issues_d":
    "Заблокированная трасса кабеля, дважды забронированный испытательный стенд и задержанное разрешение.",
  "p.demo.f_change_requests_t": "Запросы на изменение",
  "p.demo.f_change_requests_d":
    "Запрос на перенос запуска на две недели с влиянием на расписание и стоимость, ожидающий директора программы.",
  "p.demo.f_joint_risk_t": "Риск стоимости и расписания вместе",
  "p.demo.f_joint_risk_d":
    "Как риски влияют вместе на дату окончания и на стоимость.",
  "p.demo.f_financials_t": "Финансы",
  "p.demo.f_financials_d":
    "Бюджет, фактические расходы и отклонение по портфелю.",
  "p.demo.f_cost_risk_t": "Риск стоимости",
  "p.demo.f_cost_risk_d": "Разброс вероятной стоимости с P50 и P80.",
  "p.demo.f_ledger_t": "Книга финансирования",
  "p.demo.f_ledger_d":
    "Грант и резерв, прослеженные до бюджетных статей, которые за их счёт финансируются. Деньги, ещё не запланированные в статье, показаны как непрослеженные.",
  "p.demo.f_costs_t": "Затраты по проекту",
  "p.demo.f_costs_d":
    "Затраты на труд по таблицам ставок и прочие затраты по задачам.",
  "p.demo.f_resources_t": "Ресурсы",
  "p.demo.f_resources_d":
    "Восемь вымышленных людей, их роли и то, на что они заняты.",
  "p.demo.f_capacity_t": "Тепловая карта мощности",
  "p.demo.f_capacity_d":
    "Предложение и спрос по пулам навыков и месяцам. Красные месяцы показывают, где не хватает инженеров по сигнализации.",
  "p.demo.f_key_persons_t": "Ключевые сотрудники",
  "p.demo.f_key_persons_d":
    "Единые точки отказа: один инженер, которого пока никто не может заменить, и роль с готовым преемником.",
  "p.demo.f_objectives_t": "Цели",
  "p.demo.f_objectives_d":
    "Три цели, каждая связана с планами, которые ей служат.",
  "p.demo.f_okrs_t": "Цели и ключевые результаты",
  "p.demo.f_okrs_d": "Ключевые результаты с отметками о ходе и уверенностью.",
  "p.demo.f_kpis_t": "Ключевые показатели",
  "p.demo.f_kpis_d":
    "Минуты задержек, удовлетворённость пассажиров и расходы в сравнении с планом, с показанием за каждый месяц.",
  "p.demo.f_proposals_t": "Приём работ",
  "p.demo.f_proposals_d":
    "Пять предложений на разных стадиях: от составленных до утверждённых и отклонённых.",
  "p.demo.f_ideas_t": "Идеи",
  "p.demo.f_ideas_d":
    "Четыре идеи с голосами. Преобразуйте одну из них в предложение.",
  "p.demo.f_scenarios_t": "Сценарии",
  "p.demo.f_scenarios_d":
    "Что помещается в предел бюджета, если сначала выполнять работы по сигнализации.",
  "p.demo.f_director_packs_t": "Пакеты для директоров",
  "p.demo.f_director_packs_d":
    "Пакеты для руководства и совета за сентябрь, собранные из актуальных данных. Откройте один, чтобы увидеть все разделы.",
  "p.demo.f_organizations_t": "Организации-партнёры",
  "p.demo.f_organizations_d":
    "Управление, его исполнитель, партнёр и поставщик.",
  "p.demo.f_milestone_signoffs_t": "Подписание вехи",
  "p.demo.f_milestone_signoffs_d":
    "Для запуска нужны подписи партнёра и поставщика. Поставщик подписал, партнёр нет, поэтому запуск выполнен, но не достигнут.",
  "p.demo.f_cutover_t": "Решение о переходе (go/no-go)",
  "p.demo.f_cutover_d":
    "Три критерия остановки, один выполнен, одна сопутствующая подпись есть. В решении «go» было бы отказано, и экран объясняет почему.",
  "p.demo.f_deployment_moves_t": "Переносы развёртывания",
  "p.demo.f_deployment_moves_d":
    "Запрос на перенос начала установки на две недели с последствиями для других задач и затратами на труд, ожидающий утверждения.",
  "p.demo.f_start_queue_t": "Очередь запуска",
  "p.demo.f_start_queue_d":
    "Две программы в поставке, а лимит — две, поэтому третья ждёт в очереди по порядку.",
  "p.demo.f_critical_chain_t": "Критическая цепь",
  "p.demo.f_critical_chain_d":
    "Цепь через зависимые задачи и общих для них людей с агрессивными и безопасными оценками.",
  "p.demo.f_buffers_t": "Управление буферами",
  "p.demo.f_buffers_d":
    "Зафиксированный план буферов, диаграммы «лихорадки» и корректирующее действие.",
  "p.demo.f_pipeline_t": "Барабан и конвейер",
  "p.demo.f_pipeline_d":
    "Пул навыков, от которого портфель зависит больше всего, и предлагаемые разнесённые старты.",
  "p.demo.f_search_t": "Поиск",
  "p.demo.f_search_d":
    "Введите запрос в значке лупы в заголовке. Эта ссылка ищет «signal».",
  "p.demo.f_email_deliveries_t": "Доставка писем",
  "p.demo.f_email_deliveries_d":
    "Каждое письмо, которое служба пыталась отправить. Из демонстрации ничего не отправляется, поэтому список пуст.",
  "p.demo.f_display_t": "Настройки отображения",
  "p.demo.f_display_d": "Размер текста, темы, простое представление и язык.",
  "p.kanban.nav": "Моя канбан-доска",
  "p.kanban.title": "Моя канбан-доска — PPM",
  "p.kanban.h1": "Моя канбан-доска",
  "p.kanban.lede":
    "Ваши открытые задачи из всех планов в столбцах по статусам. Перемещайте карточку мышью или нажмите Enter на карточке, чтобы выбрать столбец в меню. Перемещение меняет статус задачи для всех.",
  "p.kanban.board_label": "Ваши задачи по статусам",
  "p.kanban.loading": "Загрузка вашей доски…",
  "p.kanban.none": "У вас нет открытых задач.",
  "p.kanban.todo": "К выполнению",
  "p.kanban.in_progress": "В работе",
  "p.kanban.in_review": "На проверке",
  "p.kanban.blocked": "Заблокировано",
  "p.kanban.done": "Выполнено",
  "p.kanban.card_count": "Карточек: {n}",
  "p.kanban.move_button": "Переместить «{title}»",
  "p.kanban.move_menu": "Переместить в столбец",
  "p.kanban.moved": "«{title}» перемещена в «{column}»",
  "p.kanban.due": "срок {date}",
  "p.kanban.points": "Баллов: {n}",
  "p.kanban.sign_in": "Войдите, чтобы увидеть свою доску.",
  "p.kanban.could_not_load": "Не удалось загрузить вашу доску",
  "p.kanban.could_not_move":
    "Не удалось переместить задачу; доска осталась прежней.",
  "p.kanban.hidden":
    "Задач не показано, так как их планы недоступны вам для чтения: {n}.",
  "p.dashboard.my_home": "Моя домашняя страница",
  "p.demo.banner":
    "Это демонстрация с вымышленными данными. Ничто здесь не настоящее.",
  "p.demo.banner_tour": "Пройти обзор",
  "p.demo.signin_h2": "Демонстрация",
  "p.demo.signin_lede":
    "Этот сайт — демонстрация. Войдите с адресом example@example.com, чтобы увидеть все возможности на вымышленных данных. Письма не отправляются.",
  "p.demo.signin_button": "Войти в демонстрацию",
  "p.demo.signin_wrong": "Демонстрация принимает только example@example.com.",
  "p.search.title": "Поиск — PPM",
  "p.search.h1": "Поиск планов",
  "p.search.lede":
    "Планы, название, идентификаторы, ключевые слова или теги которых соответствуют запросу. Планы, которые вы не вправе читать, не перечисляются.",
  "p.search.none_asked": "Введите запрос в поле поиска в заголовке.",
  "p.search.searching": "Поиск…",
  "p.search.results": "Планов по запросу «{q}»: {n}.",
  "p.search.no_results": "Ни один план не соответствует запросу «{q}».",
  "p.search.could_not_search": "Не удалось выполнить поиск",
  "p.layout.link_picker": "Страницы",
  "p.layout.search_picker": "Поиск планов",
  "p.layout.search_input": "Поисковый запрос",
  "p.layout.search_submit": "Найти",
  "p.layout.lp_home": "Главная",
  "p.layout.lp_plans": "Планы",
  "p.layout.lp_dashboard": "Панель",
  "p.layout.lp_milestones": "Вехи",
  "p.layout.lp_proposals": "Предложения",
  "p.cutover.link": "Решение о переходе (go/no-go)",
  "p.cutover.title": "Решение о переходе (go/no-go) — PPM",
  "p.cutover.h1": "Решение о переходе (go/no-go)",
  "p.cutover.lede":
    "Решение о переходе критичной для безопасности системы. Критерии остановки сначала записываются и публикуются, а затем фиксируются: их изменение создаёт новую версию. Для «go» нужны выполнение каждого критерия и каждая требуемая сопутствующая подпись. «No-go» или отсрочка по соображениям безопасности — это решение, принятое по критериям; это не срыв сроков.",
  "p.cutover.loading": "Загрузка решений…",
  "p.cutover.none": "Ни одно решение о переходе не составлено.",
  "p.cutover.new_h2": "Составить решение о переходе",
  "p.cutover.new_title": "Название",
  "p.cutover.new_criteria":
    "Критерии остановки, по одному в строке (каждый от 5 до 500 символов)",
  "p.cutover.draft": "Составить решение",
  "p.cutover.version": "версия {n}",
  "p.cutover.status_draft": "черновик",
  "p.cutover.status_published": "опубликовано, критерии зафиксированы",
  "p.cutover.status_open": "открыто для решения",
  "p.cutover.status_decided": "решено",
  "p.cutover.status_superseded": "заменено",
  "p.cutover.outcome_go": "go",
  "p.cutover.outcome_no_go": "no-go",
  "p.cutover.outcome_delay_for_safety": "отсрочка по соображениям безопасности",
  "p.cutover.decided_line": "Решено {on}: {outcome}. {why}",
  "p.cutover.criteria_h3": "Критерии остановки",
  "p.cutover.col_n": "№",
  "p.cutover.col_criterion": "Критерий",
  "p.cutover.col_met": "Выполнен",
  "p.cutover.col_evidence": "Свидетельство",
  "p.cutover.col_mark": "Отметка",
  "p.cutover.met": "выполнен",
  "p.cutover.not_met": "не выполнен",
  "p.cutover.evidence": "Свидетельство",
  "p.cutover.mark_met": "Отметить выполненным: {name}",
  "p.cutover.mark_unmet": "Отметить невыполненным: {name}",
  "p.cutover.mark_met_short": "Выполнен",
  "p.cutover.mark_unmet_short": "Не выполнен",
  "p.cutover.cosign_h3": "Сопутствующие подписи",
  "p.cutover.cosign_signed": "Подписали: {roles}.",
  "p.cutover.cosign_missing": "Ещё требуются: {roles}.",
  "p.cutover.cosign_stale":
    "Подписано для прежних доказательств, поэтому нужна новая подпись: {roles}.",
  "p.cutover.cosign_none": "нет",
  "p.cutover.cosign_role":
    "Роль, в которой подписать (она должна быть в вашем токене)",
  "p.cutover.cosign": "Подписать",
  "p.cutover.refused_h3": "В «go» было бы отказано, потому что",
  "p.cutover.publish": "Опубликовать и зафиксировать критерии",
  "p.cutover.open": "Открыть решение",
  "p.cutover.new_version": "Начать новую версию, чтобы изменить критерии",
  "p.cutover.decide_h3": "Принять решение",
  "p.cutover.outcome": "Исход",
  "p.cutover.reason": "Причина",
  "p.cutover.decide": "Записать решение",
  "p.cutover.could_not_load": "Не удалось загрузить решения",
  "p.cutover.could_not_save": "Не удалось сохранить",
  "p.cutover.cancel_edit": "Прекратить редактирование",
  "p.cutover.criteria_frozen":
    "Это решение опубликовано, поэтому его критерии зафиксированы: чтобы изменить их, создайте новую версию.",
  "p.cutover.criteria_rules":
    "Один критерий остановки в строке: от 1 до 30, каждый от 5 до 500 символов. Их можно менять, пока решение не опубликовано.",
  "p.cutover.criteria_saved": "Критерии остановки сохранены.",
  "p.cutover.criteria_unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.cutover.edit_criteria_label": "Критерии остановки, по одному в строке",
  "p.cutover.edit_criteria": "Изменить критерии остановки",
  "p.cutover.problem_count":
    "Для решения нужно от 1 до 30 критериев остановки.",
  "p.cutover.problem_length":
    "Критерий остановки {line} должен содержать от 5 до 500 символов.",
  "p.cutover.save_criteria": "Сохранить критерии остановки",
  "p.moves.link": "Переносы развёртывания",
  "p.moves.title": "Переносы развёртывания — PPM",
  "p.moves.h1": "Переносы развёртывания",
  "p.moves.lede":
    "Запрос на перенос начала задачи. Ничего не меняется, пока утверждающий его не утвердит. Последствия для других задач и затраты на труд были рассчитаны в момент запроса переноса путём прогона расписания с перенесённой датой.",
  "p.moves.loading": "Загрузка переносов…",
  "p.moves.none": "Ни один перенос не запрошен.",
  "p.moves.request_h2": "Запросить перенос",
  "p.moves.task": "Задача",
  "p.moves.new_start": "Новое начало",
  "p.moves.reason": "Причина",
  "p.moves.approver":
    "Утверждающий (person:<id> или role:<name>, необязательно)",
  "p.moves.request": "Запросить перенос",
  "p.moves.col_task": "Задача",
  "p.moves.col_dates": "Начало",
  "p.moves.col_reason": "Причина",
  "p.moves.col_by": "Запросил",
  "p.moves.col_approver": "Утверждающий",
  "p.moves.col_status": "Статус",
  "p.moves.col_knock": "Последствия",
  "p.moves.col_act": "Решить",
  "p.moves.status_requested": "ожидает утверждающего",
  "p.moves.status_approved": "утверждён",
  "p.moves.status_refused": "отклонён",
  "p.moves.knock": "Других задач сдвигается: {n}; затраты на труд {cost}",
  "p.moves.no_cost": "затраты на труд не меняются",
  "p.moves.decision_reason": "Причина решения",
  "p.moves.approve": "Утвердить",
  "p.moves.refuse": "Отклонить",
  "p.moves.approve_for": "Утвердить перенос задачи «{name}»",
  "p.moves.refuse_for": "Отклонить перенос задачи «{name}»",
  "p.moves.decided": "{by}, {on}: {why}",
  "p.moves.by_requester_h2": "Стоимость последствий по запросившим",
  "p.moves.requester_row": "{who}: запросов {n}; труд {cost}",
  "p.moves.could_not_load": "Не удалось загрузить переносы",
  "p.moves.could_not_save": "Не удалось сохранить",
  "p.ledger.nav": "Книга финансирования",
  "p.ledger.title": "Книга финансирования — PPM",
  "p.ledger.h1": "Книга финансирования",
  "p.ledger.lede":
    "Каждое выделение финансирования, прослеженное до бюджетных статей, которые за его счёт финансируются, их контрольных точек, достигнутых вех и подтверждённых выгод. Деньги, выделенные, но не запланированные ни в одной доступной вам для чтения статье, непрослеженные и показаны с суммой.",
  "p.ledger.scope":
    "Создание выделений, сопоставление кодов затрат и сверка финансовой выгрузки выполняются через API; этот экран читает книгу.",
  "p.ledger.loading": "Загрузка книги…",
  "p.ledger.none": "Ни одно выделение финансирования не записано.",
  "p.ledger.col_source": "Источник финансирования",
  "p.ledger.col_ref": "Ссылка",
  "p.ledger.col_on": "Выделено",
  "p.ledger.col_amount": "Выделено (сумма)",
  "p.ledger.col_planned": "Запланировано в статьях",
  "p.ledger.col_untraced": "Непрослежено",
  "p.ledger.col_lines": "Статьи",
  "p.ledger.col_act": "Проследить",
  "p.ledger.trace": "Проследить «{name}»",
  "p.ledger.trace_short": "Проследить",
  "p.ledger.trace_h2": "Что приобретено за счёт «{name}»",
  "p.ledger.money":
    "Выделено {allocated}, запланировано в статьях {planned}, израсходовано {spent}, непрослежено {untraced} ({currency}).",
  "p.ledger.gates":
    "Статей с контрольной точкой: {gated}. Освобождено: {released}.",
  "p.ledger.milestones_met": "Достигнуто вех: {met} из {total}.",
  "p.ledger.benefits": "Записано выгод: {n}.",
  "p.ledger.validated": "Подтверждено в {currency}: {amount}",
  "p.ledger.lines_h3": "Бюджетные статьи",
  "p.ledger.line_plan": "Программа",
  "p.ledger.line_desc": "Описание",
  "p.ledger.line_planned": "Запланировано",
  "p.ledger.line_actual": "Израсходовано",
  "p.ledger.line_gate": "Контрольная точка",
  "p.ledger.released": "освобождено",
  "p.ledger.held": "удерживается",
  "p.ledger.no_gate": "нет контрольной точки",
  "p.ledger.could_not_load": "Не удалось загрузить книгу",
  "p.ledger.could_not_trace": "Не удалось проследить выделение",
  "p.ledger.a_line":
    "бюджетная статья (выберите её план выше, чтобы увидеть её)",
  "p.ledger.choose_line": "Выберите статью",
  "p.ledger.choose_plan_first": "Сначала выберите план выше",
  "p.ledger.choose_plan": "Выберите план",
  "p.ledger.choose_source": "Выберите источник финансирования",
  "p.ledger.code_mapped": "Код затрат сопоставлен.",
  "p.ledger.codes_heading": "Коды затрат",
  "p.ledger.codes_says":
    "Каждый финансовый код затрат сопоставляется с одной бюджетной статьёй, чтобы финансовую выгрузку можно было сравнить с реестром. Сопоставление уже сопоставленного кода переносит его.",
  "p.ledger.could_not_load_lines":
    "Не удалось загрузить бюджетные статьи этого плана.",
  "p.ledger.could_not_reconcile": "Не удалось выполнить сверку выгрузки.",
  "p.ledger.could_not_record": "Выделение не записано.",
  "p.ledger.could_not_save": "Изменение не сохранено.",
  "p.ledger.f_allocation": "Выделение, за счёт которого финансируется",
  "p.ledger.f_amount": "Сумма",
  "p.ledger.f_code_line": "Бюджетная статья (из плана выше)",
  "p.ledger.f_code": "Код затрат",
  "p.ledger.f_csv": "Или вставьте выгрузку (CSV)",
  "p.ledger.f_currency": "Валюта",
  "p.ledger.f_file": "Файл финансовой выгрузки (CSV)",
  "p.ledger.f_on": "Дата выделения",
  "p.ledger.f_plan": "План",
  "p.ledger.f_reference": "Ссылка (необязательно)",
  "p.ledger.f_source": "Источник финансирования",
  "p.ledger.line_detached": "Статья больше не финансируется за счёт выделения.",
  "p.ledger.line_drawn": "Теперь статья финансируется за счёт этого выделения.",
  "p.ledger.lines_heading":
    "Бюджетные статьи и выделения, за счёт которых они финансируются",
  "p.ledger.lines_says":
    "Статья финансируется за счёт выделения своего источника финансирования и своей валюты; сначала задайте её источник на странице «Финансирование».",
  "p.ledger.map_code": "Сопоставить код затрат",
  "p.ledger.matched_heading": "Сопоставленные коды затрат",
  "p.ledger.matched_line":
    "в выгрузке {ledger}, в реестре {register}, разница {difference}",
  "p.ledger.mismatch_heading": "Строки в другой валюте, чем их статья",
  "p.ledger.mismatch_line":
    "строка в {row}, статья в {line}: {amount}, не включено в сумму",
  "p.ledger.needs_source": "сначала нужен источник финансирования",
  "p.ledger.no_allocation": "Не финансируется ни за счёт какого выделения",
  "p.ledger.no_codes": "Ни один код затрат пока не сопоставлен.",
  "p.ledger.no_lines": "У этого плана нет бюджетных статей.",
  "p.ledger.no_matching_allocation":
    "Пока нет выделения с источником и валютой этой статьи",
  "p.ledger.no_source": "нет источника финансирования",
  "p.ledger.not_reconciled":
    "Сверка не выполнена: что-то не совпадает. Ничего не записано.",
  "p.ledger.problem_amount": "Укажите сумму числом больше 0.",
  "p.ledger.problem_code_line": "Выберите бюджетную статью для кода.",
  "p.ledger.problem_code": "Код затрат содержит от 1 до 60 символов.",
  "p.ledger.problem_currency": "Укажите валюту тремя буквами, например GBP.",
  "p.ledger.problem_date_future": "Дата выделения не может быть в будущем.",
  "p.ledger.problem_date": "Выберите дату выделения.",
  "p.ledger.problem_export_empty":
    "Сначала выберите файл или вставьте выгрузку.",
  "p.ledger.problem_export_header":
    "В первой строке выгрузки должны быть названы cost_code, amount_minor и currency.",
  "p.ledger.problem_export_large": "Выгрузка больше 1 МБ: разделите её.",
  "p.ledger.problem_reference": "Ссылка может содержать не более 120 символов.",
  "p.ledger.problem_source": "Выберите источник финансирования.",
  "p.ledger.reconcile_heading": "Сверить финансовую выгрузку",
  "p.ledger.reconcile_says":
    "Код затрат каждой строки ищется в сопоставлении, а её суммы складываются и сравниваются с тем, что реестр хранит как израсходованное по статье. Ничего не записывается.",
  "p.ledger.reconcile": "Сверить",
  "p.ledger.reconciled":
    "Сверка выполнена: каждая строка совпала со статьёй, в её валюте, без разницы.",
  "p.ledger.record_heading": "Записать выделение",
  "p.ledger.record_says":
    "Деньги, выделенные источником финансирования. Выделения хранятся в записанном виде и никогда не изменяются.",
  "p.ledger.record": "Записать выделение",
  "p.ledger.recorded": "Выделение записано.",
  "p.ledger.refused_export":
    "Не удалось прочитать выгрузку: в каждой строке нужны код затрат, целая сумма в минимальных единицах и валюта, не больше, чем принимает служба.",
  "p.ledger.refused_line":
    "Статья не может финансироваться за счёт этого выделения: у них должны быть одинаковые источник финансирования и валюта.",
  "p.ledger.rows_read_one": "Прочитана {n} строка.",
  "p.ledger.rows_read_other": "Прочитано строк: {n}.",
  "p.ledger.to_funding":
    "Добавьте источники финансирования на странице «Финансирование»",
  "p.ledger.unmatched_heading":
    "Коды затрат, не сопоставленные со статьёй, доступной вам для чтения",
  "p.signoffs.nav": "Подписание вех",
  "p.signoffs.title": "Подписание вех — PPM",
  "p.signoffs.h1": "Подписание вех",
  "p.signoffs.lede":
    "Некоторые вехи считаются достигнутыми только после того, как названные организации-партнёры их подпишут. «Поставлено» — это заявление организации-исполнителя о завершении; «достигнуто» — это поставлено при действующих всех подписях. Отозванная подпись снова считается недостающей.",
  "p.signoffs.loading": "Загрузка подписаний…",
  "p.signoffs.none": "Ни одна веха не требует подписи партнёра.",
  "p.signoffs.totals":
    "Требуют подписи: {n}. Поставлено: {delivered}. Достигнуто: {met}. Поставлено, но не достигнуто: {gap}. Достигнуто, как доля поставленных: {share}.",
  "p.signoffs.no_share": "не измерено",
  "p.signoffs.col_milestone": "Веха",
  "p.signoffs.col_due": "Срок",
  "p.signoffs.col_state": "Состояние",
  "p.signoffs.col_delivered": "Поставлено",
  "p.signoffs.col_met": "Достигнуто",
  "p.signoffs.col_outstanding": "Ожидается от",
  "p.signoffs.col_act": "Подписать или отозвать",
  "p.signoffs.state_met": "достигнута",
  "p.signoffs.state_delivered_not_met": "поставлена, не достигнута",
  "p.signoffs.state_not_delivered": "не поставлена",
  "p.signoffs.organization": "Организация",
  "p.signoffs.reason": "Причина (для отзыва)",
  "p.signoffs.sign": "Подписать",
  "p.signoffs.withdraw": "Отозвать",
  "p.signoffs.sign_for": "Подписать «{name}» от имени выбранной организации",
  "p.signoffs.withdraw_for": "Отозвать подпись для «{name}»",
  "p.signoffs.not_built":
    "Какие организации нужны для вехи, выбирается на странице управления каждого плана; этот экран показывает и записывает подписи.",
  "p.signoffs.problem_inactive":
    "«{name}» больше не активна, поэтому её нельзя требовать: снимите отметку, чтобы сохранить.",
  "p.signoffs.problem_too_many":
    "Веха может требовать не более {max} организаций.",
  "p.signoffs.required_cancel": "Отмена",
  "p.signoffs.required_change_for": "Изменить, кто должен подписать «{name}»",
  "p.signoffs.required_change": "Изменить, кто должен подписать",
  "p.signoffs.required_cleared": "«{name}» больше не требует подписи партнёра.",
  "p.signoffs.required_heading": "Подпись партнёра",
  "p.signoffs.required_inactive": "{name} (больше не активна)",
  "p.signoffs.required_legend":
    "Организации, которые должны подписать «{name}» (не более {max})",
  "p.signoffs.required_link": "Записать подпись",
  "p.signoffs.required_no_orgs": "Нет активной организации для выбора.",
  "p.signoffs.required_none": "подпись партнёра не нужна",
  "p.signoffs.required_save": "Сохранить, кто должен подписать",
  "p.signoffs.required_saved": "Сохранено, кто должен подписать «{name}».",
  "p.signoffs.required_says":
    "Веха с подписью партнёра достигается только тогда, когда подписала каждая организация ниже; «поставлено» и «достигнуто» — разные даты.",
  "p.signoffs.required_signed": "{name} подписала {date}",
  "p.signoffs.required_unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.signoffs.required_waiting": "{name} ещё не подписала",
  "p.signoffs.could_not_load": "Не удалось загрузить подписания",
  "p.signoffs.could_not_save": "Не удалось записать подпись",
  "p.key_persons.nav": "Ключевые сотрудники",
  "p.key_persons.title": "Ключевые сотрудники — PPM",
  "p.key_persons.h1": "Ключевые сотрудники и единые точки отказа",
  "p.key_persons.lede":
    "Люди и роли, потеря которых вредна для портфеля, кто их заменит и насколько он готов. Читать это могут только менеджеры ресурсов, потому что речь идёт о названных людях.",
  "p.key_persons.loading": "Загрузка…",
  "p.key_persons.none": "Ни один ключевой сотрудник или роль не записаны.",
  "p.key_persons.not_allowed":
    "Читать ключевых сотрудников могут только менеджеры ресурсов.",
  "p.key_persons.add_h2": "Записать ключевого сотрудника или роль",
  "p.key_persons.subject_kind": "Человек или роль",
  "p.key_persons.kind_person": "человек",
  "p.key_persons.kind_role": "роль",
  "p.key_persons.subject": "Имя или роль",
  "p.key_persons.successor": "Преемник",
  "p.key_persons.readiness": "Готовность преемника",
  "p.key_persons.ready_ready": "готов",
  "p.key_persons.ready_partly": "частично готов",
  "p.key_persons.ready_none": "не готов",
  "p.key_persons.note": "Примечание",
  "p.key_persons.add": "Записать",
  "p.key_persons.col_subject": "Человек или роль",
  "p.key_persons.col_successor": "Преемник",
  "p.key_persons.col_ready": "Готовность",
  "p.key_persons.col_note": "Примечание",
  "p.key_persons.col_act": "Действие",
  "p.key_persons.remove": "Удалить «{name}»",
  "p.key_persons.remove_short": "Удалить",
  "p.key_persons.no_successor": "преемник не назван",
  "p.key_persons.contingent_h2": "Привлечённые работники",
  "p.key_persons.contingent_note":
    "Люди, отмеченные как привлечённая рабочая сила, которые могут уйти с коротким уведомлением.",
  "p.key_persons.contingent_none": "Никто не отмечен как привлечённый.",
  "p.key_persons.contingent_person": "Ссылка на человека",
  "p.key_persons.contingent_unmark": "Снять отметку с «{name}»",
  "p.key_persons.mark": "Отметить",
  "p.key_persons.unmark": "Снять отметку",
  "p.key_persons.could_not_load": "Не удалось загрузить ключевых сотрудников",
  "p.key_persons.could_not_save": "Не удалось сохранить",
  "p.organizations.nav": "Организации",
  "p.organizations.title": "Организации — PPM",
  "p.organizations.h1": "Организации-партнёры",
  "p.organizations.lede":
    "Организации, которые реализуют, спонсируют, поставляют или выступают партнёрами портфеля. Пакет можно адресовать организации, для вехи может потребоваться её подпись, а мощность может быть обязана ею. Вывод организации из использования сохраняет её историю.",
  "p.organizations.loading": "Загрузка организаций…",
  "p.organizations.none": "Ни одна организация пока не добавлена.",
  "p.organizations.add_h2": "Добавить организацию",
  "p.organizations.name": "Название",
  "p.organizations.kind": "Вид",
  "p.organizations.add": "Добавить",
  "p.organizations.col_name": "Название",
  "p.organizations.col_kind": "Вид",
  "p.organizations.col_state": "Состояние",
  "p.organizations.col_audience": "Аудитория пакета",
  "p.organizations.col_act": "Действие",
  "p.organizations.kind_delivery": "исполнитель",
  "p.organizations.kind_partner": "партнёр",
  "p.organizations.kind_sponsor": "спонсор",
  "p.organizations.kind_supplier": "поставщик",
  "p.organizations.active": "активна",
  "p.organizations.retired": "выведена из использования",
  "p.organizations.retire": "Вывести «{name}» из использования",
  "p.organizations.restore": "Восстановить «{name}»",
  "p.organizations.retire_short": "Вывести из использования",
  "p.organizations.restore_short": "Восстановить",
  "p.organizations.could_not_load": "Не удалось загрузить организации",
  "p.organizations.could_not_save": "Не удалось сохранить организацию",
  "p.email_deliveries.nav": "Доставка писем",
  "p.email_deliveries.title": "Доставка писем — PPM",
  "p.email_deliveries.h1": "Доставка писем",
  "p.email_deliveries.lede":
    "Каждое письмо, которое служба пыталась отправить, по одной строке на получателя: отправлено ли оно, ждёт повторной попытки или окончательно не удалось, сколько было попыток и последняя ошибка. Читать это могут только администраторы, потому что здесь адреса электронной почты.",
  "p.email_deliveries.loading": "Загрузка доставок…",
  "p.email_deliveries.none":
    "Ни одно письмо не отправлено и не предпринималось попыток отправки.",
  "p.email_deliveries.counts":
    "Отправлено: {sent}. Ждут повторной попытки: {retry}. Окончательно не удалось: {failed}.",
  "p.email_deliveries.filter": "Показать",
  "p.email_deliveries.all": "Все",
  "p.email_deliveries.col_recipient": "Получатель",
  "p.email_deliveries.col_subject": "Тема",
  "p.email_deliveries.col_status": "Статус",
  "p.email_deliveries.col_attempts": "Попытки",
  "p.email_deliveries.col_error": "Последняя ошибка",
  "p.email_deliveries.col_next": "Следующая попытка",
  "p.email_deliveries.col_sent": "Отправлено",
  "p.email_deliveries.status_queued": "в очереди",
  "p.email_deliveries.status_retry": "будет повтор",
  "p.email_deliveries.status_sent": "отправлено",
  "p.email_deliveries.status_failed": "не удалось",
  "p.email_deliveries.not_allowed":
    "Читать доставку писем могут только администраторы.",
  "p.email_deliveries.could_not_load": "Не удалось загрузить доставку писем",
  "p.director_pack.s_signoffs": "Подписание вех",
  "p.director_pack.signoffs_note":
    "Некоторые вехи считаются достигнутыми только после того, как названные организации-партнёры их подпишут. «Поставлено» — это заявление организации-исполнителя о завершении; «достигнуто» — это поставлено при действующих всех подписях. Отозванная подпись считается недостающей.",
  "p.director_pack.signoffs_totals":
    "Требуют подписи: {n}. Поставлено: {delivered}. Достигнуто: {met}. Поставлено, но не достигнуто: {gap}. Достигнуто, как доля поставленных: {share}.",
  "p.director_pack.so_met": "достигнута",
  "p.director_pack.so_delivered_not_met": "поставлена, не достигнута",
  "p.director_pack.so_not_delivered": "не поставлена",
  "p.director_pack.so_no_share": "пока ничего не поставлено",
  "p.director_pack.c_milestone": "Веха",
  "p.director_pack.c_due": "Срок",
  "p.director_pack.c_delivered_on": "Поставлено",
  "p.director_pack.c_met_on": "Достигнуто",
  "p.director_pack.c_outstanding": "Недостающая подпись от",
  "p.director_pack.c_delivery": "Поставка",
  "p.director_pack.dm_not_stated": "не указано",
  "p.director_pack.dm_central_product": "центральный продукт",
  "p.director_pack.dm_central_platform_local_deployment":
    "центральная платформа, локальное развёртывание",
  "p.director_pack.dm_local_programme_central_standards":
    "локальная программа, центральные стандарты",
  "p.director_pack.dm_owner": "владелец: {name}",
  "p.director_pack.dm_not_ours": "поставляется не организацией",
  "p.director_pack.s_flow":
    "Диаграмма сгорания и поток (продуктовые программы)",
  "p.director_pack.c_scope": "Создано задач",
  "p.director_pack.c_finished": "Завершено задач",
  "p.director_pack.c_open": "Открыто задач",
  "p.director_pack.c_throughput":
    "Завершено в неделю (последние четыре недели)",
  "p.director_pack.c_burnup": "Завершено по неделям (сначала старые)",
  "p.director_pack.no_flow":
    "{name}: продуктовая программа, в которой пока нет задач, поэтому нет сгорания и потока.",
  "p.director_pack.s_asks": "Внешние запросы и компромиссы",
  "p.director_pack.asks_note":
    "Запрос от спонсора нельзя утвердить, пока в нём не названо, что он вытесняет. Здесь перечислены запросы, поданные за период.",
  "p.director_pack.asks_count":
    "Запросов, поданных за период: {n}. Пока без названного компромисса: {without}.",
  "p.director_pack.ask_none": "компромисс пока не назван",
  "p.director_pack.c_ask": "Запрос",
  "p.director_pack.c_tradeoff": "Что он вытесняет",
  "p.director_pack.register_line":
    "Программ с полным реестром: {complete} из {n}.",
  "p.director_pack.register_enforced":
    "Бюджетная статья не освобождается и твёрдое выделение не записывается для программы с неполным реестром.",
  "p.director_pack.register_not_enforced":
    "Неполный реестр — это предупреждение, а не запрет: деньги по-прежнему можно перемещать, а предупреждение попадает в аудит.",
  "p.director_pack.s_register": "Программы с неполным реестром",
  "p.director_pack.c_score": "Полнота",
  "p.director_pack.c_missing": "Отсутствует",
  "p.director_pack.rf_sro": "старший ответственный владелец",
  "p.director_pack.rf_partners": "партнёры",
  "p.director_pack.rf_stage": "этап",
  "p.director_pack.rf_baseline": "базовый план расписания",
  "p.director_pack.rf_target_date": "целевая дата",
  "p.director_pack.rf_budget": "бюджет",
  "p.director_pack.rf_funding_source": "источник финансирования",
  "p.director_pack.rf_confidence": "уверенность в поставке",
  "p.director_pack.rf_top_risks": "главные риски",
  "p.director_pack.rf_dependencies": "зависимости",
  "p.director_pack.s_watermelon": "Показатели «арбуза»",
  "p.director_pack.wm_note":
    "Снаружи зелёный, внутри красный. Они читаются из этого пакета и предыдущего выпущенного пакета в сохранённом виде, а не пересчитываются. Показатель, для которого нечего измерять, объясняет почему; это не ноль.",
  "p.director_pack.wm_gap":
    "Самооценка и независимая оценка уверенности расходятся",
  "p.director_pack.wm_late":
    "Позднее обнаружение: сейчас красный или янтарный/красный, в предыдущем пакете зелёный",
  "p.director_pack.wm_red": "Красный без записанного за период решения",
  "p.director_pack.wm_signoff": "Вехи, достигнутые с подписью, из поставленных",
  "p.director_pack.wm_signoff_of":
    "достигнуто {met} из поставленных {delivered}",
  "p.director_pack.wm_not_measured": "не измерено",
  "p.director_pack.c_measure": "Показатель",
  "p.director_pack.c_programmes": "Программы или почему не измерено",
  "p.director_pack.wm_count": "Программ: {n}",
  "p.director_pack.s_keypeople":
    "Ключевые сотрудники и привлечённая рабочая сила",
  "p.director_pack.kp_note":
    "Единые точки отказа здесь подсчитываются и никогда не называются; сам реестр предназначен для менеджеров ресурсов.",
  "p.director_pack.kp_counts":
    "Единых точек отказа: {n} (людей {people}, ролей {roles}). Без преемника: {nosucc}. Преемник готов: {ready}; частично готов: {partly}; не готов: {none}.",
  "p.director_pack.kp_no_people": "в этом пуле никого нет",
  "p.director_pack.c_headcount": "Людей",
  "p.director_pack.c_contingent": "Привлечённые",
  "p.director_pack.c_contingent_share": "Доля привлечённых",
  "p.director_pack.s_triage": "Триаж и пороги",
  "p.director_pack.tr_note":
    "Показан последний триаж каждой программы. Пересечение порога открывает исключение, требующее приостановки, сброса, остановки или переопределения со сроком действия; автоматически ничего не приостанавливается и не останавливается.",
  "p.director_pack.tr_decisions":
    "Решения за этот период: приостановка {pause}, сброс {reset}, остановка {stop}, переопределение {override}.",
  "p.director_pack.tr_open": "Исключений по порогам, ожидающих решения: {n}.",
  "p.director_pack.tr_override_line":
    "Переопределение для {programme} действует до {expiry}.",
  "p.director_pack.tr_finish": "завершить",
  "p.director_pack.tr_fix": "исправить",
  "p.director_pack.tr_pause": "приостановить",
  "p.director_pack.tr_stop": "остановить",
  "p.director_pack.c_outcome": "Триаж",
  "p.director_pack.c_triaged_on": "Триаж выполнен",
  "l.enum.pk_conf.threshold": "порог пересечён",
  "p.director_pack.s_cutovers": "Решения о переходе",
  "p.director_pack.co_note":
    "Решение о переходе принимается по критериям остановки, опубликованным до решения. «No-go» или отсрочка по соображениям безопасности — это решение, принятое по этим критериям, и оно не считается срывом сроков.",
  "p.director_pack.co_counts":
    "Решено за период: go {go}, no-go {nogo}, отсрочка по соображениям безопасности {delay}.",
  "p.director_pack.co_go": "go (переход разрешён)",
  "p.director_pack.co_no_go": "no-go",
  "p.director_pack.co_delay_for_safety":
    "отсрочка по соображениям безопасности",
  "p.director_pack.c_cutover": "Переход",
  "p.director_pack.c_decision": "Решение",
  "p.director_pack.c_decided_on": "Решено",
  "p.director_pack.s_outcomes": "Результаты и деэскалация",
  "p.director_pack.oc_note":
    "Уровень эскалации задаёт спонсор, и здесь он никогда не рассчитывается. Серия ниже считает завершённые кварталы, в которых выполнены все показатели оценочной карты и все критерии; она предлагается тому, кто принимает решение.",
  "p.director_pack.oc_level": "Уровень эскалации: {level}, задан {on}.",
  "p.director_pack.oc_no_level": "Уровень эскалации не записан.",
  "p.director_pack.oc_run":
    "Последовательных кварталов с выполнением: {n}. Правило требует {need}.",
  "p.director_pack.oc_rule_yes":
    "Правило выполнено. Снижать ли уровень — решает спонсор.",
  "p.director_pack.oc_rule_no": "Правило пока не выполнено.",
  "p.director_pack.oc_met": "выполнено",
  "p.director_pack.oc_not_met": "не выполнено",
  "p.director_pack.oc_missing": "отсутствует",
  "p.director_pack.c_quarter": "Квартал",
  "p.director_pack.c_why": "Почему",
  "p.balance.balance": "Баланс портфеля",
  "p.balance.balance_ppm": "Баланс портфеля — PPM",
  "p.balance.intro":
    "Как плановая стоимость, целевая ценность выгод и открытый риск каждого портфеля соотносятся с остальными, и вытекающие из этих цифр дисбалансы.",
  "p.balance.loading": "Загрузка…",
  "p.balance.could_not_load": "Не удалось загрузить баланс портфеля",
  "p.balance.none":
    "Ни стоимость, ни ценность выгод, ни риск пока не записаны ни по одному портфелю.",
  "p.balance.currency": "Валюта",
  "p.balance.unassigned": "Нет портфеля",
  "p.balance.caption":
    "Каждый пузырь — портфель. По горизонтали: плановая стоимость в {currency}. По вертикали: целевая ценность выгод в {currency}. Площадь: открытое воздействие риска.",
  "p.balance.axis_cost": "Плановая стоимость ({currency})",
  "p.balance.axis_value": "Целевая ценность выгод ({currency})",
  "p.balance.imbalances": "Дисбалансы",
  "p.balance.no_imbalance":
    "Ни одно из правил ниже не выявляет дисбаланс в этих цифрах.",
  "p.balance.rules":
    "Отметка ставится, когда портфель содержит {concentration}% или более плановой стоимости либо открытого риска (при наличии не менее двух портфелей), когда его доля плановой стоимости или риска на {gap} процентных пункта и более превышает его долю целевой ценности, либо когда у него есть плановая стоимость, но нет целевой ценности в выбранной валюте. Отметка — повод присмотреться, а не вердикт.",
  "p.balance.flag_cost_concentration":
    "{name} содержит {percent}% плановой стоимости.",
  "p.balance.flag_risk_concentration":
    "{name} содержит {percent}% открытого воздействия риска.",
  "p.balance.flag_cost_ahead_of_value":
    "У «{name}» доля плановой стоимости заметно выше его доли целевой ценности ({percent}%).",
  "p.balance.flag_risk_ahead_of_value":
    "У «{name}» доля открытого риска заметно выше его доли целевой ценности ({percent}%).",
  "p.balance.flag_no_value":
    "У «{name}» есть плановая стоимость, но целевая ценность в {currency} не записана.",
  "p.balance.table": "Те же цифры в виде таблицы",
  "p.balance.portfolio": "Портфель",
  "p.balance.cost": "Плановая стоимость ({currency})",
  "p.balance.value": "Целевая ценность ({currency})",
  "p.balance.risk": "Открытое воздействие риска",
  "p.balance.cost_share": "Доля стоимости",
  "p.balance.value_share": "Доля ценности",
  "p.balance.risk_share": "Доля риска",
  "p.balance.flags": "Отметки",
  "p.balance.no_flag": "нет",
  "p.balance.notes":
    "Группировка по портфелям, а не по категориям или стратегическим целям: портфель — единственная группировка, по которой отчитываются и стоимость, и ценность выгод, и риск. Стоимость — это плановая сумма по бюджетным статьям, ценность — цель записанных выгод, а риск — оценка открытого воздействия; деньги в разных валютах никогда не складываются.",
  "p.roadmap.title":
    "Дорожная карта — влияние, трудозатраты и Сейчас / Далее / Позже",
  "p.roadmap.h1":
    "Дорожная карта: влияние, трудозатраты и Сейчас / Далее / Позже",
  "p.roadmap.lede":
    "Открытые идеи и предложения, по которым ещё не принято решение, размещённые по тому, насколько они важны (влияние) и сколько работы требуют (трудозатраты). Служба не хранит влияние и трудозатраты идеи или предложения, поэтому вы оцениваете каждый элемент здесь; ваши оценки хранятся только в этом браузере и не передаются службе. Элемент без обеих оценок не размещается.",
  "p.roadmap.loading": "Загрузка идей и предложений…",
  "p.roadmap.matrix_h2": "Матрица влияния и трудозатрат",
  "p.roadmap.matrix_caption":
    "Влияние по строкам, трудозатраты по столбцам. В каждой ячейке перечислены элементы с такой оценкой.",
  "p.roadmap.impact": "Влияние",
  "p.roadmap.effort": "Трудозатраты",
  "p.roadmap.impact_level": "Влияние: {level}",
  "p.roadmap.effort_level": "Трудозатраты: {level}",
  "p.roadmap.level_low": "Низкое",
  "p.roadmap.level_medium": "Среднее",
  "p.roadmap.level_high": "Высокое",
  "p.roadmap.rate_h2": "Оценить элементы",
  "p.roadmap.rate_hint":
    "Эта таблица — способ работы с матрицей с клавиатуры: выберите влияние и трудозатраты для каждого элемента, и он переместится в матрице и в списках «Сейчас / Далее / Позже».",
  "p.roadmap.col_item": "Элемент",
  "p.roadmap.col_type": "Тип",
  "p.roadmap.col_horizon": "Когда",
  "p.roadmap.kind_idea": "Идея",
  "p.roadmap.kind_proposal": "Предложение",
  "p.roadmap.not_rated": "Не оценено",
  "p.roadmap.rate_impact": "Влияние «{title}»",
  "p.roadmap.rate_effort": "Трудозатраты «{title}»",
  "p.roadmap.no_items": "Нет открытых идей или предложений для оценки.",
  "p.roadmap.horizons_h2": "Сейчас / Далее / Позже",
  "p.roadmap.horizon_rule":
    "Сейчас: влияние выше трудозатрат. Далее: влияние равно трудозатратам и не ниже среднего. Позже: всё остальное оценённое. Ещё не оценено: не выбрано влияние или трудозатраты.",
  "p.roadmap.h_now": "Сейчас",
  "p.roadmap.h_next": "Далее",
  "p.roadmap.h_later": "Позже",
  "p.roadmap.h_unrated": "Ещё не оценено",
  "p.roadmap.nothing_here": "Здесь ничего нет.",
  "p.roadmap.see_ideas": "Посмотреть идеи и предложения на дорожной карте",
  "p.people_impact.title": "Влияние на людей (из службы кадровых ресурсов)",
  "p.people_impact.intro":
    "Только числа из службы кадровых ресурсов (WPM), запрошенные под вашей учётной записью. Люди не называются, и ничего не суммируется.",
  "p.people_impact.ask": "Запросить службу кадровых ресурсов",
  "p.people_impact.asking": "Запрос…",
  "p.people_impact.failed": "Не удалось загрузить влияние на людей.",
  "p.people_impact.not_known": "Неизвестно.",
  "p.people_impact.source":
    "Из службы кадровых ресурсов по состоянию на {date}.",
  "p.people_impact.unknown": "Неизвестно: {reason}",
  "p.people_impact.end_sent":
    "Отправлена дата окончания: {date} (целевая дата этого плана).",
  "p.people_impact.committed": "Занятые люди по пулам",
  "p.people_impact.ending":
    "Срочные и контрактные привлечения, заканчивающиеся до окончания",
  "p.people_impact.in_no_pool": "Не в пуле",
  "p.people_impact.end_not_recorded":
    "Окончание не записано (не считается заканчивающимся)",
  "p.people_impact.hidden": "скрыто (меньше {floor})",
  "p.people_impact.no_pools": "Нет пула для показа.",
  "p.people_impact.part_unknown": "Неизвестно: {reason}",
  "p.people_impact.over": "Месяцы пулов с превышением нагрузки",
  "p.people_impact.over_counts":
    "Месяцев пулов с превышением: {over} из {considered}; невозможно сказать: {unknown}.",
  "p.people_impact.no_total":
    "Итог не указан: человек может входить в несколько пулов.",
  "l.enum.pk_heat.supply_unknown": "предложение неизвестно",
  "p.capacity_heatmap.nav": "Тепловая карта мощности",
  "p.capacity_heatmap.title": "Тепловая карта мощности — PPM",
  "p.capacity_heatmap.h1": "Мощность по пулам навыков и месяцам",
  "p.capacity_heatmap.lede":
    "Каждый квадрат сравнивает работу, запрошенную у пула навыков за месяц, с доступными для неё людьми в эквиваленте полной занятости (FTE).",
  "p.capacity_heatmap.bridge_note":
    "Пул, связанный со службой кадровых ресурсов, берёт предложение оттуда. То, что введено здесь для этого пула, показано рядом и никогда к нему не добавляется. Месяц без значения от службы кадровых ресурсов показан как неизвестный, а не как ноль.",
  "p.capacity_heatmap.from": "Первый месяц",
  "p.capacity_heatmap.months": "Число месяцев",
  "p.capacity_heatmap.show": "Показать",
  "p.capacity_heatmap.loading": "Загрузка тепловой карты…",
  "p.capacity_heatmap.could_not_load":
    "Не удалось загрузить тепловую карту мощности.",
  "p.capacity_heatmap.none": "Нет ни пула навыков, ни спроса для показа.",
  "p.capacity_heatmap.caption":
    "Спрос и предложение в эквиваленте полной занятости по пулам навыков и месяцам",
  "p.capacity_heatmap.col_pool": "Пул навыков",
  "p.capacity_heatmap.linked_to": "Связан со службой кадровых ресурсов",
  "p.capacity_heatmap.demand": "Спрос: {fte}",
  "p.capacity_heatmap.supply": "Предложение: {fte}",
  "p.capacity_heatmap.gap": "Разрыв: {fte}",
  "p.capacity_heatmap.unknown": "неизвестно",
  "p.capacity_heatmap.none_recorded": "не записано",
  "p.capacity_heatmap.source_wpm": "Предложение из службы кадровых ресурсов",
  "p.capacity_heatmap.source_here": "Предложение, введённое здесь",
  "p.capacity_heatmap.peer_live": "Актуально, получено {when}.",
  "p.capacity_heatmap.peer_live_no_time": "Актуально.",
  "p.capacity_heatmap.peer_last":
    "Последнее получено {age} (взято {when}); сейчас не актуально: {reason}",
  "p.capacity_heatmap.peer_hidden":
    "Скрыто: служба кадровых ресурсов скрывает месяц, состоящий из меньшего числа работников, чем её порог.",
  "p.capacity_heatmap.peer_unknown": "Неизвестно: {reason}",
  "p.capacity_heatmap.no_reason":
    "от службы кадровых ресурсов не получено значение",
  "p.capacity_heatmap.entered_here":
    "Введено здесь: {fte} (показано рядом, не добавлено)",
  "p.capacity_heatmap.entered_here_none": "Введено здесь: нет",
  "p.capacity_heatmap.partners_short":
    "партнёры поставили меньше, чем обязались",
  "p.capacity_heatmap.no_pool": "Пул не назначен",
  "p.capacity_heatmap.key":
    "▲ нехватка · ◆ впритык · ● достаточно · ○ нет спроса · – предложение не записано · ? предложение неизвестно. Слово всегда записывается тоже.",
  "p.capacity_heatmap.every_figure": "Все значения в виде таблицы",
  "p.capacity_heatmap.table_label": "Мощность по пулам навыков и месяцам",
  "p.capacity_heatmap.c_pool": "Пул навыков",
  "p.capacity_heatmap.c_month": "Месяц",
  "p.capacity_heatmap.c_reading": "Показание",
  "p.capacity_heatmap.c_demand": "Спрос (эквивалент полной занятости)",
  "p.capacity_heatmap.c_supply": "Предложение (эквивалент полной занятости)",
  "p.capacity_heatmap.c_source": "Предложение из",
  "p.capacity_heatmap.c_status": "Показание службы кадровых ресурсов",
  "p.capacity_heatmap.c_entered_here": "Введено здесь",
  "p.capacity_heatmap.see_pools":
    "Связать пулы навыков со службой кадровых ресурсов",
  "p.skill_pools.nav": "Пулы навыков",
  "p.skill_pools.title": "Пулы навыков — PPM",
  "p.skill_pools.h1": "Пулы навыков",
  "p.skill_pools.lede":
    "Пул навыков объединяет роли и навыки, которые могут выполнять одну и ту же работу. Здесь можно связать пул с соответствующим пулом в службе кадровых ресурсов.",
  "p.skill_pools.rule":
    "Связь должна быть задана с обеих сторон — здесь и в службе кадровых ресурсов — человеком, который видит обе. По названию пулы никогда не сопоставляются. Пока пул связан, его предложение на тепловой карте мощности берётся из службы кадровых ресурсов, а его спрос отправляется туда.",
  "p.skill_pools.loading": "Загрузка пулов навыков…",
  "p.skill_pools.could_not_load": "Не удалось загрузить пулы навыков.",
  "p.skill_pools.none": "Ни один пул навыков пока не создан.",
  "p.skill_pools.col_name": "Название",
  "p.skill_pools.col_roles": "Роли",
  "p.skill_pools.col_skills": "Навыки",
  "p.skill_pools.col_state": "Состояние",
  "p.skill_pools.col_link": "Пул службы кадровых ресурсов",
  "p.skill_pools.col_act": "Изменить",
  "p.skill_pools.active": "активен",
  "p.skill_pools.retired": "выведен из использования",
  "p.skill_pools.not_linked": "не связан",
  "p.skill_pools.change_short": "Изменить связь",
  "p.skill_pools.change": "Изменить связь для «{name}»",
  "p.skill_pools.edit_h2": "Связать «{name}» со службой кадровых ресурсов",
  "p.skill_pools.ref_label": "Идентификатор пула службы кадровых ресурсов",
  "p.skill_pools.ref_hint":
    "Скопируйте его из пула в службе кадровых ресурсов: 36 символов, строчными буквами, с дефисами, например 3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b.",
  "p.skill_pools.ref_invalid":
    "Это не идентификатор пула службы кадровых ресурсов: он должен содержать 36 символов, строчными буквами, с дефисами.",
  "p.skill_pools.save": "Сохранить связь",
  "p.skill_pools.clear": "Удалить связь",
  "p.skill_pools.cancel": "Отмена",
  "p.skill_pools.confirm_clear":
    "Удалить связь между «{name}» и службой кадровых ресурсов? Его предложение на тепловой карте снова будет таким, как введено здесь. Каждое значение спроса, уже отправленное в службу кадровых ресурсов для этого пула, будет там отозвано (отправлено как ноль).",
  "p.skill_pools.saved":
    "«{name}» теперь связан с пулом службы кадровых ресурсов {ref}.",
  "p.skill_pools.cleared":
    "«{name}» больше не связан со службой кадровых ресурсов.",
  "p.skill_pools.taken":
    "Другой пул здесь уже связан с этим пулом службы кадровых ресурсов. Сначала удалите ту связь: один пул здесь на один пул там.",
  "p.skill_pools.invalid":
    "Служба не приняла этот идентификатор. Это должен быть идентификатор пула службы кадровых ресурсов: 36 символов, строчными буквами, с дефисами.",
  "p.skill_pools.missing": "Этого пула больше нет. Список загружен заново.",
  "p.skill_pools.could_not_save": "Не удалось изменить связь.",
  "p.skill_pools.see_heatmap": "Открыть тепловую карту мощности",
  "p.skill_pools.withdrawing":
    "Значений спроса, отзываемых из пула службы кадровых ресурсов, с которым он был связан: {n}.",
  "p.skill_pools.add_heading": "Добавить пул",
  "p.skill_pools.add_says":
    "Пул объединяет роли ресурсов и навыки, чтобы тепловая карта мощности считала их спрос и предложение один раз. Роль или навык может входить не более чем в один пул.",
  "p.skill_pools.add": "Добавить пул",
  "p.skill_pools.added": "Пул «{name}» добавлен.",
  "p.skill_pools.choose_pool": "Выберите пул",
  "p.skill_pools.clash_item": "{value} (в «{pool}»)",
  "p.skill_pools.conflict_members":
    "Пул с таким названием уже существует, либо одна из этих ролей или навыков уже входит в другой пул.",
  "p.skill_pools.could_not_add": "Пул не добавлен.",
  "p.skill_pools.f_name": "Название пула",
  "p.skill_pools.f_pool": "Пул",
  "p.skill_pools.f_roles": "Роли",
  "p.skill_pools.f_skills": "Навыки",
  "p.skill_pools.list_says":
    "Разделяйте роли или навыки запятыми или переводами строки; повтор сохраняется один раз.",
  "p.skill_pools.members_heading": "Изменить пул",
  "p.skill_pools.members_saved": "Роли и навыки «{name}» сохранены.",
  "p.skill_pools.problem_clash": "Уже входит в другой пул: {list}.",
  "p.skill_pools.problem_name_taken": "Пул с таким названием уже существует.",
  "p.skill_pools.problem_name": "Название пула содержит от 2 до 120 символов.",
  "p.skill_pools.restore": "Восстановить пул",
  "p.skill_pools.restored": "Пул «{name}» восстановлен.",
  "p.skill_pools.retire_says":
    "Выведенный из использования пул остаётся в записи и сохраняет свои роли и навыки; после восстановления он снова учитывается.",
  "p.skill_pools.retire": "Вывести пул из использования",
  "p.skill_pools.retired_done": "Пул «{name}» выведен из использования.",
  "p.skill_pools.save_members": "Сохранить роли и навыки",
  "p.skill_pools.unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.capacity_heatmap.set_commitment": "Задать обязательство по мощности",
  "p.commitments.title": "Обязательства по мощности — PPM",
  "p.commitments.h1": "Обязательства по мощности",
  "p.commitments.lede":
    "Запишите мощность, обещанную программе (предложение), или работу, которая нужна программе от пула навыков (спрос), за один месяц в эквиваленте полной занятости.",
  "p.commitments.bridge_note":
    "Для пула, связанного со службой кадровых ресурсов, обязательство по спросу также отправляет туда общий спрос программы на этот пул и месяц. Итог ноль отзывает там значение. Повторных попыток нет: сохраните снова, чтобы отправить снова.",
  "p.commitments.see_heatmap": "Открыть тепловую карту мощности",
  "p.commitments.from_plan": "Обязательства по мощности для этой программы",
  "p.commitments.form_h2": "Задать обязательство",
  "p.commitments.choose": "Выберите…",
  "p.commitments.pool": "Пул навыков",
  "p.commitments.pool_linked": "{name} (связан со службой кадровых ресурсов)",
  "p.commitments.pool_hint": "Перечислены только активные пулы.",
  "p.commitments.pool_missing": "Выберите пул навыков.",
  "p.commitments.plan": "Программа",
  "p.commitments.plan_hint":
    "План должен быть программой (планом вида Program).",
  "p.commitments.plan_missing": "Выберите программу.",
  "p.commitments.kind": "Что обещано",
  "p.commitments.kind_demand": "Спрос: работа, которая нужна программе от пула",
  "p.commitments.kind_supply": "Предложение: мощность, обещанная программе",
  "p.commitments.organization": "Организация-партнёр (необязательно)",
  "p.commitments.organization_hint":
    "Оставьте пустым для собственной организации.",
  "p.commitments.month": "Месяц",
  "p.commitments.month_hint": "Один месяц: год и месяц.",
  "p.commitments.month_invalid": "Укажите месяц как год и месяц.",
  "p.commitments.committed": "Обещано (эквивалент полной занятости)",
  "p.commitments.committed_hint":
    "Число от 0 до 1000, записанное так, как вы записываете числа. Ноль отзывает значение спроса, отправленное в службу кадровых ресурсов.",
  "p.commitments.delivered":
    "Поставлено (эквивалент полной занятости, необязательно)",
  "p.commitments.delivered_hint":
    "Что было фактически поставлено, если известно.",
  "p.commitments.fte_invalid":
    "Число от 0 до 1000, записанное так, как вы записываете числа.",
  "p.commitments.note": "Примечание (необязательно)",
  "p.commitments.save": "Сохранить обязательство",
  "p.commitments.saved": "Сохранено: {pool}, {plan}, {month}.",
  "p.commitments.not_found":
    "Эта программа не найдена, либо у вас нет права её читать.",
  "p.commitments.refused":
    "Служба не приняла это. План должен быть программой, пул — активным, а каждое значение — от 0 до 1000.",
  "p.commitments.could_not_save": "Не удалось сохранить обязательство.",
  "p.commitments.could_not_load":
    "Не удалось загрузить обязательства по мощности.",
  "p.commitments.loading": "Загрузка недавних отправок…",
  "p.commitments.sends_h2": "Недавние отправки в службу кадровых ресурсов",
  "p.commitments.sends_note":
    "Последняя отправка для каждой программы, пула и месяца, сначала новые. У начатой отправки здесь появится ответ, когда он придёт: обновите страницу, чтобы увидеть его.",
  "p.commitments.refresh": "Обновить",
  "p.commitments.no_sends":
    "В службу кадровых ресурсов пока ничего не отправлено.",
  "p.commitments.c_pool": "Пул навыков",
  "p.commitments.c_programme": "Программа",
  "p.commitments.c_month": "Месяц",
  "p.commitments.c_sent": "Отправлено",
  "p.commitments.c_outcome": "Что произошло",
  "p.commitments.c_at": "Когда",
  "p.commitments.sent_fte": "{fte} эквивалента полной занятости",
  "p.commitments.sent_withdrawal": "отзыв (ноль)",
  "p.commitments.sent_nothing": "ничего",
  "p.commitments.out_not_sent_at_all":
    "Это обязательство не отправляется в службу кадровых ресурсов: туда отправляется только спрос по связанному с ней пулу.",
  "p.commitments.out_none_yet":
    "Для этой программы, пула и месяца в службу кадровых ресурсов пока ничего не отправлено.",
  "p.commitments.out_started":
    "Отправка {fte} эквивалента полной занятости (общий итог программы для этого пула и месяца) в службу кадровых ресурсов. Её ответ появится в разделе «Недавние отправки».",
  "p.commitments.out_started_zero":
    "Отзыв значения этой программы для этого пула и месяца из службы кадровых ресурсов, потому что итог теперь равен нулю. Её ответ появится в разделе «Недавние отправки».",
  "p.commitments.out_sent":
    "Служба кадровых ресурсов приняла {fte} эквивалента полной занятости.",
  "p.commitments.out_sent_zero":
    "Служба кадровых ресурсов отозвала своё значение для этой программы, пула и месяца.",
  "p.commitments.out_duplicate":
    "У службы кадровых ресурсов уже было это значение; там ничего не изменилось.",
  "p.commitments.out_refused":
    "Служба кадровых ресурсов отклонила это. Повторных попыток нет; сохраните снова, чтобы отправить снова.",
  "p.commitments.out_unreachable":
    "Служба кадровых ресурсов недоступна или не ответила вовремя. Повторных попыток нет; сохраните снова, чтобы отправить снова.",
  "p.commitments.out_not_configured":
    "Не отправлено: эта служба не настроена на связь со службой кадровых ресурсов.",
  "p.commitments.out_no_token":
    "Не отправлено: не было входа в систему, который можно передать службе кадровых ресурсов.",
  "p.commitments.out_too_large":
    "Не отправлено: служба кадровых ресурсов принимает не более 10 000 эквивалентов полной занятости.",
  "p.commitments.out_old_zero":
    "Не отправлено: ноль записан до того, как нули стали отправляться, поэтому служба кадровых ресурсов сохранила своё последнее значение. Сохраните снова, чтобы отозвать его.",
  "p.commitments.out_other": "Служба записала: {outcome}.",
  "p.discussion.heading": "Обсуждение",
  "p.discussion.loading": "Загрузка…",
  "p.discussion.could_not_load_comments": "Не удалось загрузить комментарии.",
  "p.discussion.could_not_load_reactions": "Не удалось загрузить реакции.",
  "p.discussion.could_not_load_files": "Не удалось загрузить файлы.",
  "p.discussion.could_not_load_watch":
    "Не удалось определить, следите ли вы за этим.",
  "p.discussion.sign_in_to_follow":
    "Войдите, чтобы следить за этим и получать уведомления о новых комментариях и изменениях.",
  "p.discussion.you_follow":
    "Вы следите за этим: вы получаете уведомления о новых комментариях и изменениях.",
  "p.discussion.you_do_not_follow": "Вы не следите за этим.",
  "p.discussion.followers_one": "Следит {n} человек.",
  "p.discussion.followers_other": "Следят: {n}.",
  "p.discussion.follow": "Следить",
  "p.discussion.unfollow": "Перестать следить",
  "p.discussion.followed": "Теперь вы следите за этим.",
  "p.discussion.unfollowed": "Вы больше не следите за этим.",
  "p.discussion.reactions_label": "Реакции",
  "p.discussion.reaction_named_one": "{reaction}: {n} человек",
  "p.discussion.reaction_named_other": "{reaction}: {n} человек",
  "p.discussion.r_thumbs_up": "Палец вверх",
  "p.discussion.r_heart": "Сердце",
  "p.discussion.r_celebrate": "Поздравляю",
  "p.discussion.r_eyes": "Смотрю",
  "p.discussion.r_question": "Вопрос",
  "p.discussion.comments_heading": "Комментарии",
  "p.discussion.no_comments": "Комментариев пока нет.",
  "p.discussion.no_author": "кто-то без входа в систему",
  "p.discussion.by_at": "Автор: {who}, {when}",
  "p.discussion.edited_at": "изменён {when}",
  "p.discussion.edit": "Изменить",
  "p.discussion.edit_named": "Изменить комментарий автора {who}, {when}",
  "p.discussion.delete": "Удалить",
  "p.discussion.delete_named": "Удалить комментарий автора {who}, {when}",
  "p.discussion.confirm_delete_comment":
    "Удалить этот комментарий? Его прежние версии сохраняются, но на странице его больше никто не увидит.",
  "p.discussion.earlier": "Прежние версии",
  "p.discussion.no_earlier": "Прежних версий не сохранено.",
  "p.discussion.replaced_at": "Заменена {when}",
  "p.discussion.f_edit": "Изменить комментарий",
  "p.discussion.save_edit": "Сохранить комментарий",
  "p.discussion.cancel": "Отмена",
  "p.discussion.f_comment": "Ваш комментарий",
  "p.discussion.chars_left": "Осталось символов: {n} из {max}.",
  "p.discussion.chars_over": "Превышение лимита {max} на {n} символов.",
  "p.discussion.post": "Опубликовать комментарий",
  "p.discussion.posted": "Комментарий опубликован.",
  "p.discussion.edited": "Комментарий изменён; его прежняя версия сохранена.",
  "p.discussion.deleted": "Комментарий удалён.",
  "p.discussion.problem_blank": "Сначала напишите комментарий.",
  "p.discussion.problem_too_long":
    "Комментарий может содержать не более 4000 символов.",
  "p.discussion.files_heading": "Файлы",
  "p.discussion.files_intro":
    "Прикреплённые здесь файлы сохраняются на ваш компьютер при открытии; внутри этой страницы они никогда не показываются.",
  "p.discussion.no_files": "Файлов не прикреплено.",
  "p.discussion.file_facts": "{type}, {size}, добавил {who}, {when}",
  "p.discussion.remove_file": "Удалить",
  "p.discussion.remove_file_named": "Удалить файл {name}",
  "p.discussion.confirm_remove_file":
    "Удалить файл {name}? Отсюда его больше никто не сможет скачать.",
  "p.discussion.removed_file": "Файл {name} удалён.",
  "p.discussion.f_file": "Файл для прикрепления",
  "p.discussion.files_rules":
    "PDF, изображение (PNG, JPEG, GIF или WebP), обычный текст, CSV, Markdown, JSON либо файл Word, Excel или PowerPoint размером не более 10 МБ. Веб-страницы, рисунки SVG, архивы и программы не принимаются.",
  "p.discussion.attach": "Прикрепить файл",
  "p.discussion.attached": "Файл {name} прикреплён.",
  "p.discussion.problem_choose_file": "Сначала выберите файл.",
  "p.discussion.problem_empty": "Этот файл пуст.",
  "p.discussion.problem_too_large":
    "Этот файл больше 10 МБ — предельного размера одного файла.",
  "p.discussion.problem_type":
    "Файл такого вида прикрепить нельзя. Допустимые виды перечислены в списке ниже.",
  "p.discussion.type_pdf": "документ PDF",
  "p.discussion.type_png": "изображение PNG",
  "p.discussion.type_jpeg": "изображение JPEG",
  "p.discussion.type_gif": "изображение GIF",
  "p.discussion.type_webp": "изображение WebP",
  "p.discussion.type_text": "обычный текст",
  "p.discussion.type_csv": "таблица CSV",
  "p.discussion.type_markdown": "текст Markdown",
  "p.discussion.type_json": "данные JSON",
  "p.discussion.type_word": "документ Word",
  "p.discussion.type_excel": "книга Excel",
  "p.discussion.type_powerpoint": "слайды PowerPoint",
  "p.discussion.size_b": "{n} байт",
  "p.discussion.size_kb": "{n} КБ",
  "p.discussion.size_mb": "{n} МБ",
  "p.discussion.refused_sign_in":
    "Сначала войдите в систему: сервис не знает, кто вы.",
  "p.discussion.refused_not_author":
    "Изменить или удалить комментарий может только его автор.",
  "p.discussion.refused_not_uploader":
    "Удалить файл может только тот, кто его прикрепил, или администратор.",
  "p.discussion.refused_forbidden": "У вас нет права читать или изменять это.",
  "p.discussion.refused_comment_gone":
    "Этого комментария больше нет; комментарии перечитаны.",
  "p.discussion.refused_file_gone": "Этот файл уже удалён; файлы перечитаны.",
  "p.discussion.refused_gone":
    "Не удалось найти это: возможно, оно удалено или вам недоступно.",
  "p.discussion.refused_comment_invalid":
    "Сервис не принял комментарий: он не должен быть пустым и может содержать не более 4000 символов.",
  "p.discussion.refused_file_invalid":
    "Сервис не принял файл: его вид, размер или содержимое не прошли проверку (файл должен соответствовать своему виду, программы не принимаются).",
  "p.discussion.refused_too_large": "Файл слишком велик для отправки.",
  "p.discussion.refused_scan_unavailable":
    "Файл не прикреплён: сканер вредоносных программ не ответил. Повторите попытку позже.",
  "p.me.timesheet_link": "Мой табель",
  "p.me.timesheet_note":
    "— ваше время по дням и планам, неделя за неделей, и отправка недели на утверждение.",
  "p.timesheet.title_ppm": "Мой табель — PPM",
  "p.timesheet.heading": "Мой табель",
  "p.timesheet.back": "← Моя работа",
  "p.timesheet.intro":
    "Учтённое время за одну неделю по дням и планам, в часах и минутах; оплачиваемое и неоплачиваемое время всегда показано отдельно. Завершённую неделю отправьте на утверждение; после утверждения она окончательна, и учитывать в ней время больше нельзя.",
  "p.timesheet.loading": "Загрузка…",
  "p.timesheet.week_nav": "Выбор недели",
  "p.timesheet.previous_week": "Предыдущая неделя",
  "p.timesheet.this_week": "Эта неделя",
  "p.timesheet.next_week": "Следующая неделя",
  "p.timesheet.f_week": "Неделя, например 2026-W46",
  "p.timesheet.show_week": "Показать неделю",
  "p.timesheet.week_hint":
    "Недели идут с понедельника по воскресенье и нумеруются по международному стандарту независимо от того, какой день у вас считается первым. Сегодня {today} в вашем часовом поясе, {tz}.",
  "p.timesheet.problem_week":
    "Это не неделя: укажите год, «-W» и номер недели, например 2026-W46.",
  "p.timesheet.mine_heading": "Ваша неделя {week}",
  "p.timesheet.mine_caption": "Ваше время за неделю {week} по планам и дням",
  "p.timesheet.status_open": "Открыта: ещё не отправлена на утверждение",
  "p.timesheet.status_submitted": "Отправлена на утверждение: ожидает решения",
  "p.timesheet.status_approved":
    "Утверждена: окончательна, новое время учитывать нельзя",
  "p.timesheet.status_rejected": "Возвращена: измените и отправьте снова",
  "p.timesheet.rejected_because": "утверждающий написал: {comment}",
  "p.timesheet.decision_comment": "с примечанием: {comment}",
  "p.timesheet.no_entries": "За эту неделю время не учтено, поэтому итога нет.",
  "p.timesheet.col_plan": "Название плана",
  "p.timesheet.col_total": "Итого",
  "p.timesheet.col_billable": "Оплачиваемое",
  "p.timesheet.col_non_billable": "Неоплачиваемое",
  "p.timesheet.row_day_totals": "Каждый день",
  "p.timesheet.plan_unnamed": "План {id}… (название не показано)",
  "p.timesheet.split_line":
    "Оплачиваемое {billable}, неоплачиваемое {non_billable}: всего {total}.",
  "p.timesheet.omitted":
    "Записи, пропущенные из-за планов, которые вам нельзя читать: {n}.",
  "p.timesheet.hours_one": "{n} ч",
  "p.timesheet.hours_other": "{n} ч",
  "p.timesheet.minutes_one": "{n} мин",
  "p.timesheet.minutes_other": "{n} мин",
  "p.timesheet.duration_hm": "{h} {m}",
  "p.timesheet.submit": "Отправить эту неделю на утверждение",
  "p.timesheet.submit_again": "Отправить эту неделю на утверждение снова",
  "p.timesheet.submitted_notice": "Неделя {week} отправлена на утверждение.",
  "p.timesheet.nothing_to_submit":
    "Неделю без учтённого времени нельзя отправить на утверждение.",
  "p.timesheet.waiting": "Эта неделя ожидает решения утверждающего.",
  "p.timesheet.locked":
    "Утверждённая неделя окончательна: учитывать в ней время больше нельзя.",
  "p.timesheet.could_not_load": "Не удалось загрузить вашу неделю.",
  "p.timesheet.could_not_submit": "Не удалось отправить неделю на утверждение.",
  "p.timesheet.other_heading": "Неделя другого сотрудника, для утверждающих",
  "p.timesheet.other_intro":
    "Если вы вправе утверждать табели, укажите ссылку на человека, чтобы прочитать его неделю за выбранную выше неделю, затем утвердите её или верните. Никто не принимает решение по собственной неделе, для возврата нужна причина, а утверждение окончательно.",
  "p.timesheet.f_person":
    "Человек: «person:» или «worker:» и его идентификатор",
  "p.timesheet.show_person": "Показать его неделю",
  "p.timesheet.problem_person":
    "Укажите человека как «person:» или «worker:» и его идентификатор, ничего больше.",
  "p.timesheet.other_week_heading": "Неделя {week}, {person}",
  "p.timesheet.other_caption":
    "Время {person} за неделю {week} по планам и дням",
  "p.timesheet.decision_legend": "Ваше решение",
  "p.timesheet.approve": "Утвердить",
  "p.timesheet.reject": "Вернуть",
  "p.timesheet.f_comment": "Примечание, если нужно",
  "p.timesheet.f_comment_required": "Причина возврата (обязательно)",
  "p.timesheet.approve_final":
    "Утверждение окончательно: неделя блокируется, и вернуть её уже никто не сможет.",
  "p.timesheet.record_decision": "Записать решение",
  "p.timesheet.approved_notice": "Неделя {week}, {person}, утверждена.",
  "p.timesheet.rejected_notice": "Неделя {week}, {person}, возвращена.",
  "p.timesheet.other_not_submitted":
    "Эта неделя не отправлена на утверждение, поэтому решать пока нечего.",
  "p.timesheet.other_rejected":
    "Эта неделя возвращена; решение по ней можно принять снова после повторной отправки.",
  "p.timesheet.problem_own_week":
    "Это ваша собственная неделя: никто не принимает решение по собственному табелю.",
  "p.timesheet.problem_comment": "Укажите, почему неделя возвращается.",
  "p.timesheet.could_not_load_other":
    "Не удалось загрузить неделю этого человека.",
  "p.timesheet.could_not_decide": "Не удалось записать решение.",
  "p.timesheet.refused_read_forbidden":
    "Читать эту неделю может только сам человек или утверждающий табели.",
  "p.timesheet.refused_submit_forbidden":
    "Отправить неделю на утверждение может только тот, кому она принадлежит.",
  "p.timesheet.refused_decide_forbidden":
    "Вы не можете принять решение по этой неделе: нужно право утверждающего табели, и никто не решает по собственной неделе.",
  "p.timesheet.refused_submit_conflict":
    "Эта неделя уже отправлена на утверждение или утверждена; данные перечитаны.",
  "p.timesheet.refused_decide_conflict":
    "Эта неделя больше не ожидает решения; данные перечитаны.",
  "p.timesheet.refused_not_addressable":
    "У вашей учётной записи нет ссылки на человека, поэтому показать табель нельзя.",
  "p.timesheet.refused_read_invalid":
    "Сервис не принял эту неделю или этого человека.",
  "p.timesheet.refused_submit_invalid":
    "Неделя не отправлена: в ней нет учтённого времени.",
  "p.timesheet.refused_decide_invalid":
    "Сервис не принял решение: для возврата недели нужна причина.",
  "p.security.access_link": "Проверка доступа и экстренный доступ",
  "p.security.access_note":
    "— кто какой доступ имеет, для повторной сертификации, и ограниченный по времени экстренный доступ к закрытому плану, который фиксируется и подлежит аудиту.",
  "p.access.title_ppm": "Проверка доступа и экстренный доступ — PPM",
  "p.access.back": "← Безопасность",
  "p.access.heading": "Проверка доступа и экстренный доступ",
  "p.access.intro":
    "Проверка доступа перечисляет доступ, записанный как предоставленный, для повторной сертификации. Экстренный доступ на ограниченное время открывает вам закрытый план, когда другого пути нет; каждое использование фиксируется и подлежит аудиту.",
  "p.access.review_heading": "Проверка доступа",
  "p.access.review_intro":
    "Доступ, записанный в журнале доступа как предоставленный и не отозванный, сначала самый давний, с указанием, кто и когда его предоставил. Показано то, что записали люди, а не то, что обеспечивает служба входа: сравните их. Только для администраторов; каждое чтение фиксируется в журнале аудита, поэтому проверка читается только по вашему запросу.",
  "p.access.f_review_plan":
    "Идентификатор плана (оставьте пустым для всех планов, которые вам можно читать)",
  "p.access.f_review_days":
    "Нужна проверка по истечении стольких календарных дней",
  "p.access.show_review": "Показать проверку",
  "p.access.problem_review_plan":
    "Это не идентификатор плана: вставьте идентификатор плана или оставьте поле пустым для всех планов.",
  "p.access.problem_review_days":
    "Укажите число календарных дней целым числом, 0 или больше.",
  "p.access.could_not_load_review": "Не удалось загрузить проверку доступа.",
  "p.access.review_totals":
    "По состоянию на {as_of}. Действующих предоставлений: {standing}. Нужна проверка при возрасте {days} календарных дней и более: {due}.",
  "p.access.review_note":
    "Возраст считается в календарных днях от дня записи предоставления до сегодняшнего дня развёртывания.",
  "p.access.download_review":
    "Скачать проверку как электронную таблицу со столбцом для решений «оставить» или «отозвать»",
  "p.access.review_caption":
    "Доступ, действующий сегодня, сначала самый давний",
  "p.access.review_empty":
    "В планах, которые вам можно читать, нет действующего доступа, записанного как предоставленный.",
  "p.access.col_person": "Человек",
  "p.access.col_access": "Доступ",
  "p.access.col_plan": "Название плана",
  "p.access.col_granted_on": "Дата предоставления",
  "p.access.col_granted_by": "Кем предоставлен",
  "p.access.col_age": "Возраст в календарных днях",
  "p.access.col_review": "Проверка",
  "p.access.col_note": "Примечание",
  "p.access.due": "Нужна проверка",
  "p.access.not_due": "Срок ещё не наступил",
  "p.access.plan_unnamed": "План {id}… (название не показано)",
  "p.access.bg_heading": "Экстренный доступ к закрытому плану (break-glass)",
  "p.access.bg_audited_heading":
    "Каждое использование экстренного доступа фиксируется и подлежит аудиту.",
  "p.access.bg_audited_open":
    "При открытии плана в журнале аудита записываются ваши данные, план, ваша причина, срок и человек, которого вы назвали.",
  "p.access.bg_audited_told":
    "Названный вами человек получает уведомление сразу. Если уведомить его нельзя, план не открывается.",
  "p.access.bg_audited_each":
    "Каждый запрос, который проходит только благодаря этому доступу, записывается в журнале аудита как экстренный доступ.",
  "p.access.bg_audited_visibility":
    "Он позволяет видеть план. Что вы можете изменять, по-прежнему определяют ваши собственные права.",
  "p.access.bg_audited_ends":
    "Он прекращается сам в указанное время или раньше, если вы его завершите. Не более пяти в сутки и по одному одновременно на каждый план.",
  "p.access.bg_intro":
    "Используйте его, только когда участников плана не удаётся найти, а работа не терпит отлагательства. Нужно право экстренного доступа (break-glass) или доступ администратора; решение принимает сервис.",
  "p.access.problems_heading": "План не открыт:",
  "p.access.problem_plan": "Укажите идентификатор закрытого плана.",
  "p.access.problem_reason_short":
    "Укажите, зачем вам нужен доступ, не менее {reason_min} символов.",
  "p.access.problem_reason_long": "Причина длиннее {reason_max} символов.",
  "p.access.problem_minutes":
    "Укажите срок целым числом минут, от {minutes_min} до {minutes_max}.",
  "p.access.problem_notify":
    "Укажите человека для уведомления как «person:» или «worker:» и его идентификатор, ничего больше.",
  "p.access.problem_acknowledged":
    "Отметьте флажок, подтверждая, что вам понятно: это фиксируется, подлежит аудиту, а человека уведомят.",
  "p.access.could_not_open": "Не удалось открыть план.",
  "p.access.opened_notice":
    "План открыт для вас до {until} (ваш часовой пояс, {tz}). {person} уведомлён.",
  "p.access.go_to_plan": "Перейти к плану",
  "p.access.f_plan": "Идентификатор закрытого плана",
  "p.access.f_reason":
    "Зачем вам нужен доступ (обязательно, не менее {min} символов)",
  "p.access.reason_count": "Символов: {n}; не менее {min} и не более {max}.",
  "p.access.f_minutes": "На какой срок, в минутах (от {min} до {max})",
  "p.access.f_notify":
    "Кого уведомить: «person:» или «worker:» и идентификатор (не вы)",
  "p.access.f_acknowledge":
    "Мне понятно, что это фиксируется и подлежит аудиту и что названный мной человек будет уведомлён.",
  "p.access.open": "Открыть план по экстренному доступу",
  "p.access.grants_heading": "Предоставления экстренного доступа",
  "p.access.grants_intro":
    "Ваши, сначала новые; администратор видит все. Предоставление никогда не удаляется. Время указано в вашем часовом поясе, {tz}.",
  "p.access.grants_caption":
    "Предоставления экстренного доступа, сначала новые",
  "p.access.grants_empty": "Экстренный доступ не открывался.",
  "p.access.loading": "Загрузка…",
  "p.access.could_not_load_grants":
    "Не удалось загрузить предоставления экстренного доступа.",
  "p.access.could_not_revoke": "Не удалось завершить экстренный доступ.",
  "p.access.revoked_notice": "Экстренный доступ к {plan} завершён.",
  "p.access.revoke": "Завершить сейчас",
  "p.access.revoke_for": "Завершить сейчас: экстренный доступ к {plan}",
  "p.access.state_live": "Действует",
  "p.access.state_revoked": "Завершён досрочно",
  "p.access.state_expired": "Истёк",
  "p.access.ended_by": "{when}, завершил {who}",
  "p.access.col_state": "Состояние",
  "p.access.col_holder": "Кем удерживается",
  "p.access.col_opened": "Открыт",
  "p.access.col_ends": "Завершается или завершился",
  "p.access.col_reason": "Указанная причина",
  "p.access.col_told": "Уведомлённый человек",
  "p.access.col_act": "Действие",
  "p.access.refused_sign_in": "Сначала войдите в систему.",
  "p.access.refused_review_forbidden":
    "Проверка доступа предназначена только для администраторов.",
  "p.access.refused_review_invalid": "Сервис не принял план или число дней.",
  "p.access.refused_open_forbidden":
    "Вы не можете использовать экстренный доступ: нужно право экстренного доступа (break-glass) или доступ администратора.",
  "p.access.refused_open_not_found": "Плана с таким идентификатором нет.",
  "p.access.refused_open_conflict":
    "У вас уже есть экстренный доступ к этому плану; список ниже перечитан.",
  "p.access.refused_open_invalid":
    "План не открыт. Либо он не закрытый, и открывать нечего, либо уведомляемый человек — это вы, либо этого человека нельзя уведомить. Доступ не предоставлен.",
  "p.access.refused_open_limit":
    "Вы открыли максимально допустимое за сутки число экстренных доступов (пять, считая за последние 24 часа).",
  "p.access.refused_revoke_not_found":
    "Это предоставление не найдено, или завершить его вы не вправе.",
  "p.deescalation.nav": "Сводные карты показателей и снижение эскалации",
  "p.deescalation.title":
    "Сводные карты показателей и снижение эскалации — PPM",
  "p.deescalation.h1": "Сводные карты показателей и снижение эскалации",
  "p.deescalation.lede":
    "Уровень эскалации, как его задал спонсор, критерии снижения с доказательствами за каждый квартал и сводные карты показателей, ключевые показатели которых читаются по завершённым кварталам.",
  "p.deescalation.tracker_heading": "Трекер снижения эскалации",
  "p.deescalation.tracker_says":
    "Уровень записывается так, как его задал спонсор. Здесь он никогда не вычисляется, и ничто на этой странице его не меняет: серия выполненных кварталов предлагается тому, кто принимает решение.",
  "p.deescalation.loading": "Загрузка…",
  "p.deescalation.could_not_load":
    "Не удалось загрузить трекер снижения эскалации.",
  "p.deescalation.could_not_save": "Изменение не сохранено.",
  "p.deescalation.level_heading": "Уровень эскалации",
  "p.deescalation.level_now": "Уровень {level}, задан {date}.",
  "p.deescalation.level_note": "Примечание спонсора: {note}",
  "p.deescalation.level_none": "Уровень эскалации ещё не записан.",
  "p.deescalation.level_history": "Прежние уровни",
  "p.deescalation.level_history_item": "{level}, задан {date}",
  "p.deescalation.run_heading": "Кварталы подряд с выполнением",
  "p.deescalation.run_one":
    "Завершённых кварталов с выполнением подряд: {n} из {need}, которые требует правило.",
  "p.deescalation.run_other":
    "Завершённых кварталов с выполнением подряд: {n} из {need}, которые требует правило.",
  "p.deescalation.rule_satisfied":
    "Правило выполнено. Это на усмотрение спонсора; уровень сам не меняется.",
  "p.deescalation.rule_not_satisfied": "Правило пока не выполнено.",
  "p.deescalation.setting_unusable":
    "Настройку развёртывания о том, сколько кварталов требует правило, использовать не удалось, поэтому действует значение по умолчанию: {n}.",
  "p.deescalation.reads_scorecard":
    "Трекер читает сводную карту показателей {name}.",
  "p.deescalation.reads_no_scorecard":
    "Ни одна сводная карта не назначена читаемой трекером, поэтому учитываются только критерии. Отметьте одну при добавлении ниже.",
  "p.deescalation.hidden_kpis_one":
    "{n} ключевой показатель этой сводной карты относится к плану, который вам нельзя читать: он здесь не показан и не учитывается.",
  "p.deescalation.hidden_kpis_other":
    "Ключевых показателей этой сводной карты, относящихся к планам, которые вам нельзя читать: {n}. Они здесь не показаны и не учитываются.",
  "p.deescalation.quarters_label": "Каждый завершённый квартал",
  "p.deescalation.col_quarter": "Квартал",
  "p.deescalation.col_state": "Положение",
  "p.deescalation.col_reasons": "Почему",
  "p.deescalation.state_met": "Выполнено",
  "p.deescalation.state_not_met": "Не выполнено",
  "p.deescalation.state_missing": "Ничего не записано",
  "p.deescalation.state_with_value": "{state} ({value})",
  "p.deescalation.reason_all_met":
    "Выполнены все ключевые показатели и все критерии.",
  "p.deescalation.reason_nothing":
    "Измерять нечего: нет ни ключевых показателей, ни критериев.",
  "p.deescalation.reason_no_kpis":
    "В сводной карте нет ключевых показателей, которые вам можно читать.",
  "p.deescalation.reason_not_met": "Не выполнено: {list}.",
  "p.deescalation.reason_missing": "Ничего не записано по: {list}.",
  "p.deescalation.quarter": "{q}-й кв. {year}",
  "p.deescalation.quarter_standing": "{quarter}: {state}. {reasons}",
  "p.deescalation.tracker_kpis_label": "Сводная карта трекера по кварталам",
  "p.deescalation.set_level_heading": "Записать уровень эскалации",
  "p.deescalation.set_level_says":
    "Записать уровень так, как его задал спонсор, может только спонсор (человек с ролью спонсора) или администратор. Уровень — от {min} до {max} символов; примечание необязательно.",
  "p.deescalation.f_level": "Уровень",
  "p.deescalation.f_level_note": "Примечание (необязательно)",
  "p.deescalation.save_level": "Записать уровень",
  "p.deescalation.level_saved": "Уровень {level} записан.",
  "p.deescalation.unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.deescalation.problem_level": "Уровень — от {min} до {max} символов.",
  "p.deescalation.refused_level_forbidden":
    "Отказано: записать уровень эскалации может только спонсор или администратор.",
  "p.deescalation.criteria_heading": "Критерии снижения",
  "p.deescalation.criteria_says":
    "Критерий добавляется и никогда не редактируется и не выводится из употребления. Его доказательства записываются за завершённый квартал, и учитывается последняя запись за этот квартал.",
  "p.deescalation.criteria_none": "Критериев пока нет.",
  "p.deescalation.evidence_of": "Доказательства для: {criterion}",
  "p.deescalation.evidence_link": "Доказательства",
  "p.deescalation.add_criterion_heading": "Добавить критерий",
  "p.deescalation.f_criterion": "Критерий (от {min} до {max} символов)",
  "p.deescalation.add_criterion": "Добавить критерий",
  "p.deescalation.criterion_added": "Критерий добавлен.",
  "p.deescalation.problem_criterion": "Критерий — от {min} до {max} символов.",
  "p.deescalation.evidence_heading": "Записать доказательства",
  "p.deescalation.evidence_says":
    "Выберите критерий и завершённый квартал (идущий квартал записать нельзя), укажите, выполнен ли он, и напишите примечание не короче {min} символов. Ссылка необязательна: адрес http или https без пробелов, до {max} символов.",
  "p.deescalation.f_ev_criterion": "Критерий",
  "p.deescalation.f_ev_quarter": "Завершённый квартал",
  "p.deescalation.f_ev_met": "Выполнен ли он?",
  "p.deescalation.f_ev_note": "Примечание (не менее {min} символов)",
  "p.deescalation.f_ev_link": "Ссылка на доказательства (необязательно)",
  "p.deescalation.save_evidence": "Записать доказательства",
  "p.deescalation.evidence_saved": "Доказательства записаны за {quarter}.",
  "p.deescalation.problem_criterion_choose":
    "Сначала добавьте критерий, затем записывайте его доказательства.",
  "p.deescalation.problem_quarter": "Выберите завершённый квартал.",
  "p.deescalation.problem_quarter_unfinished":
    "Этот квартал не завершён, поэтому записать его пока нельзя.",
  "p.deescalation.problem_met": "Укажите, выполнен ли критерий.",
  "p.deescalation.problem_note": "Нужно примечание не короче {min} символов.",
  "p.deescalation.problem_link":
    "Ссылка — это адрес http или https без пробелов, до {max} символов.",
  "p.deescalation.refused_evidence":
    "Отказано: квартал должен быть завершён, примечание должно содержать не менее пяти символов, а ссылка должна быть адресом http или https без пробелов.",
  "p.deescalation.refused_gone":
    "Этой записи больше нет. Перезагрузите страницу.",
  "p.deescalation.scorecards_heading": "Сводные карты показателей",
  "p.deescalation.scorecards_says":
    "Сводная карта показателей — именованный набор ключевых показателей, читаемый по завершённым кварталам. Ключевой показатель выполнен в квартале, если его последнее значение за этот квартал не хуже цели; квартал без значения считается отсутствующим, а это не то же самое, что не выполненным.",
  "p.deescalation.scorecards_none": "Сводных карт показателей пока нет.",
  "p.deescalation.could_not_load_scorecards":
    "Не удалось загрузить сводные карты показателей.",
  "p.deescalation.col_scorecard": "Сводная карта показателей",
  "p.deescalation.col_tracker": "Читается трекером",
  "p.deescalation.is_tracker": "Да, читается трекером",
  "p.deescalation.not_tracker": "Нет",
  "p.deescalation.add_scorecard_heading": "Добавить сводную карту показателей",
  "p.deescalation.add_scorecard_says":
    "Название от {min} до {max} символов, ещё не использованное (без учёта регистра). Не более одной сводной карты читается трекером.",
  "p.deescalation.f_name": "Название",
  "p.deescalation.f_for_tracker": "Читается трекером",
  "p.deescalation.add_scorecard": "Добавить сводную карту",
  "p.deescalation.scorecard_added": "Сводная карта {name} добавлена.",
  "p.deescalation.problem_name":
    "Название сводной карты — от {min} до {max} символов.",
  "p.deescalation.problem_name_taken":
    "Сводная карта с таким названием уже есть.",
  "p.deescalation.problem_tracker_taken":
    "Сводная карта {name} уже читается трекером.",
  "p.deescalation.scorecard_heading": "Сводная карта по кварталам",
  "p.deescalation.f_scorecard": "Сводная карта показателей",
  "p.deescalation.choose_scorecard": "Выберите сводную карту",
  "p.deescalation.could_not_load_scorecard":
    "Не удалось загрузить сводную карту.",
  "p.deescalation.scorecard_says":
    "Каждый ключевой показатель по завершённым кварталам: выполнен, не выполнен или ничего не записано.",
  "p.deescalation.scorecard_kpis_label":
    "Ключевые показатели: {name}, по кварталам",
  "p.deescalation.scorecard_quarters_label": "Каждый квартал сводной карты",
  "p.deescalation.col_kpi": "Ключевой показатель",
  "p.deescalation.col_target": "Цель",
  "p.deescalation.target_at_least": "не менее {value} {unit}",
  "p.deescalation.target_at_most": "не более {value} {unit}",
  "p.deescalation.no_kpis":
    "В этой сводной карте нет ключевых показателей, которые вам можно читать.",
  "p.deescalation.unit_percent": "процентов",
  "p.deescalation.unit_count": "штук",
  "p.deescalation.kpis_heading": "Выбор ключевых показателей (KPI)",
  "p.deescalation.kpis_says":
    "Отметьте ключевые показатели этой сводной карты, не более {max}. Сохранение заменяет набор; показатель по плану, который вам нельзя читать, остаётся в карте.",
  "p.deescalation.f_kpis": "Ключевые показатели в сводной карте",
  "p.deescalation.kpis_none":
    "Выбирать нечего. Добавьте ключевые показатели на странице ключевых показателей или из шаблона ниже.",
  "p.deescalation.save_kpis": "Сохранить ключевые показатели",
  "p.deescalation.kpis_saved_one":
    "Теперь в сводной карте {n} ключевой показатель.",
  "p.deescalation.kpis_saved_other":
    "Ключевых показателей в сводной карте теперь: {n}.",
  "p.deescalation.problem_kpis_max":
    "В сводной карте не более {max} ключевых показателей.",
  "p.deescalation.refused_kpis":
    "Отказано: каждый должен быть ключевым показателем, который вам можно читать, а в сводной карте не более 60.",
  "p.deescalation.template_heading": "Добавить показатели из шаблона",
  "p.deescalation.template_says":
    "Стартовый набор: он предлагается, но не применяется. В нём нет целевых значений: цель и красную линию вы задаёте сами, а выбранные показатели становятся ключевыми показателями этой сводной карты.",
  "p.deescalation.template_rule":
    "Если лучше большее значение, красная линия должна быть ниже цели; если лучше меньшее — выше. Цели и красные линии — целые числа.",
  "p.deescalation.could_not_load_template": "Не удалось загрузить шаблон.",
  "p.deescalation.higher_better": "Чем больше, тем лучше",
  "p.deescalation.lower_better": "Чем меньше, тем лучше",
  "p.deescalation.unit": "единица измерения: {unit}",
  "p.deescalation.f_pick": "Добавить: {measure}",
  "p.deescalation.f_target": "Цель",
  "p.deescalation.f_red": "Красная линия",
  "p.deescalation.add_measures": "Добавить показатели",
  "p.deescalation.measures_added_one":
    "В сводную карту добавлен {n} показатель.",
  "p.deescalation.measures_added_other":
    "Показателей добавлено в сводную карту: {n}.",
  "p.deescalation.problem_pick_one": "Выберите хотя бы один показатель.",
  "p.deescalation.problem_whole":
    "{measure}: цель и красная линия — целые числа.",
  "p.deescalation.problem_red_higher":
    "{measure}: лучше большее значение, поэтому красная линия должна быть ниже цели.",
  "p.deescalation.problem_red_lower":
    "{measure}: лучше меньшее значение, поэтому красная линия должна быть выше цели.",
  "p.deescalation.problem_not_in_template": "{measure} нет в шаблоне.",
  "p.deescalation.refused_template":
    "Отказано: проверьте каждую цель и красную линию по правилу направления.",
  "p.deescalation.measure_programmes_with_independent_assurance":
    "Программы с независимой гарантией качества за последний квартал",
  "p.deescalation.measure_milestones_met_on_time":
    "Вехи, достигнутые в срок или раньше",
  "p.deescalation.measure_benefits_validated":
    "Плановые выгоды, подтверждённые кем-либо вне программы",
  "p.deescalation.measure_red_without_decision":
    "Программы в красном статусе без записанного решения",
  "p.deescalation.measure_open_critical_issues":
    "Открытые проблемы критической серьёзности",
  "p.deescalation.measure_change_requests_waiting":
    "Запросы на изменение, ожидающие решения более 30 дней",
  "p.phase.name_initiating": "Инициация",
  "p.phase.name_planning": "Планирование",
  "p.phase.name_executing": "Исполнение",
  "p.phase.name_controlling": "Контроль",
  "p.phase.name_closing": "Закрытие",
  "p.phase.heading": "Фаза",
  "p.phase.heading_programme": "Фаза и запуск",
  "p.phase.says":
    "План движется вперёд по одной фазе; он может вернуться к более ранней фазе с указанием причины, которая записывается вместе с переходом. Фаза никогда не мешает записывать работу.",
  "p.phase.could_not_load": "Не удалось загрузить фазу плана",
  "p.phase.now": "План находится в фазе «{phase}».",
  "p.phase.none": "Фаза ещё не записана: первый переход — в «Инициацию».",
  "p.phase.time_heading": "Время в каждой фазе",
  "p.phase.days_in_one": "{phase}: {days} день",
  "p.phase.days_in_other": "{phase}: дней — {days}",
  "p.phase.visits_one": "вход: {visits} раз",
  "p.phase.visits_other": "входов: {visits}",
  "p.phase.current": "сейчас",
  "p.phase.check_heading": "Проверка запуска",
  "p.phase.check_says":
    "Программа запускается, когда её фаза переходит в «Исполнение». В запуске отказывается, если заняты все места для программ в активной реализации (приостановленные и остановленные программы мест не занимают) или если в пуле ограничения не хватает предложения в любом из ближайших шести месяцев. Программа, получившая отказ, попадает в очередь запуска и сохраняет в ней своё место; её можно запустить всё равно, указав причину обхода не короче {min} символов, которая записывается и подлежит аудиту. Чтение проверки ничего не меняет.",
  "p.phase.could_not_check": "Не удалось прочитать проверку запуска",
  "p.phase.places":
    "Занято мест для программ в активной реализации: {in_delivery} из {limit}.",
  "p.phase.no_limit": "Предел числа программ в активной реализации не задан.",
  "p.phase.limit_unusable":
    "Предел числа программ в реализации, заданный в развёртывании, использовать не удалось, поэтому предела нет.",
  "p.phase.pool_is": "Пул ограничения: {pool}.",
  "p.phase.no_pool": "Пул ограничения не определён.",
  "p.phase.would_refuse": "Запуск сейчас был бы отклонён:",
  "p.phase.would_start": "Запуск сейчас состоялся бы.",
  "p.phase.in_queue": "Эта программа в очереди запуска под номером {n}.",
  "p.phase.see_queue": "Открыть очередь запуска",
  "p.phase.refusal_limit":
    "Заняты все {limit} мест для программ в активной реализации (в реализации: {in_delivery}).",
  "p.phase.refusal_pool":
    "В пуле ограничения {pool} не хватает предложения: {months}.",
  "p.phase.demand_heading": "Твёрдый спрос этой программы",
  "p.phase.demand_caption":
    "Эквиваленты полной занятости по пулам навыков и месяцам, по твёрдым назначениям в этой программе и подчинённых ей планах",
  "p.phase.no_demand":
    "В ближайшие шесть месяцев у этой программы нет твёрдого спроса ни в одном пуле навыков.",
  "p.phase.col_pool": "Пул навыков",
  "p.phase.check_again": "Проверить снова",
  "p.phase.move_heading": "Изменить фазу",
  "p.phase.f_to": "Перейти в",
  "p.phase.forward_to": "Вперёд, в «{phase}»",
  "p.phase.back_to": "Назад, в «{phase}»",
  "p.phase.back_needs_reason": "Для возврата назад нужна причина.",
  "p.phase.f_reason":
    "Причина (нужна для возврата назад; записывается вместе с переходом)",
  "p.phase.start_may_queue":
    "Переход в «Исполнение» — это запуск программы: если при отправке проверка не найдёт места, в запуске будет отказано, а программа попадёт в очередь запуска.",
  "p.phase.start_would_queue":
    "В этом запуске было бы отказано. Если отправить без причины обхода, программа попадёт в очередь запуска (или сохранит там своё место).",
  "p.phase.f_override":
    "Причина обхода, чтобы всё равно запустить (не менее {min} символов; записывается и подлежит аудиту)",
  "p.phase.refused_why": "Почему в запуске отказано",
  "p.phase.move": "Сменить фазу",
  "p.phase.moved": "Теперь план находится в фазе «{phase}».",
  "p.phase.started_override":
    "Программа запущена в обход ограничения: теперь она в фазе «{phase}», обход записан.",
  "p.phase.start_refused_place":
    "В запуске отказано, поэтому программа в очереди запуска под номером {n}. Чтобы запустить её всё равно, укажите причину обхода.",
  "p.phase.start_refused":
    "В запуске отказано, программа попала в очередь запуска. Чтобы запустить её всё равно, укажите причину обхода.",
  "p.phase.could_not_move": "Не удалось сменить фазу",
  "p.phase.problem_choose": "Выберите фазу, в которую нужно перейти.",
  "p.phase.problem_same":
    "План уже находится в этой фазе, поэтому ничего не отправлено.",
  "p.phase.problem_skip": "Переход вперёд выполняется по одной фазе.",
  "p.phase.problem_reason": "Для возврата назад нужна причина.",
  "p.phase.problem_override": "Причина обхода — не менее {min} символов.",
  "p.phase.refused_move":
    "Отказано: переход вперёд выполняется по одной фазе, для возврата назад нужна причина, а план не должен уже находиться в этой фазе.",
  "p.phase.refused_gone": "Этого плана больше нет.",
  "p.proposals.act_submit": "Подать",
  "p.proposals.act_review": "Начать рассмотрение",
  "p.proposals.act_approve": "Утвердить",
  "p.proposals.act_reject": "Отклонить",
  "p.proposals.act_submit_named": "Подать: {title}",
  "p.proposals.act_review_named": "Начать рассмотрение: {title}",
  "p.proposals.act_approve_named": "Утвердить: {title}",
  "p.proposals.act_reject_named": "Отклонить: {title}",
  "p.proposals.ask_rule":
    "Внешняя просьба спонсора должна указывать, что она вытесняет (компромисс не короче {min} символов), прежде чем её можно будет утвердить.",
  "p.proposals.new_form": "Новое предложение",
  "p.proposals.f_external_ask": "Внешняя просьба спонсора",
  "p.proposals.f_new_tradeoff":
    "Что она вытесняет (компромисс, от {min} до {max} символов; можно указать позже, до утверждения)",
  "p.proposals.col_tradeoff": "Внешняя просьба и компромисс",
  "p.proposals.not_ask": "Не внешняя просьба",
  "p.proposals.ask_with": "Внешняя просьба. Компромисс: {tradeoff}",
  "p.proposals.ask_without": "Внешняя просьба. Компромисс пока не указан.",
  "p.proposals.tradeoff_name": "Указать компромисс",
  "p.proposals.tradeoff_change": "Изменить компромисс",
  "p.proposals.tradeoff_name_named": "Указать компромисс: {title}",
  "p.proposals.tradeoff_change_named": "Изменить компромисс: {title}",
  "p.proposals.tradeoff_heading": "Компромисс: {title}",
  "p.proposals.tradeoff_says":
    "Укажите, что вытесняет эта просьба, от {min} до {max} символов. Его можно указать или изменить, пока предложение не утверждено и не отклонено; каждое изменение записывается.",
  "p.proposals.f_tradeoff": "Что она вытесняет (от {min} до {max} символов)",
  "p.proposals.tradeoff_save": "Сохранить компромисс",
  "p.proposals.tradeoff_close": "Закрыть форму компромисса",
  "p.proposals.tradeoff_unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.proposals.tradeoff_saved": "Компромисс сохранён: {title}.",
  "p.proposals.problem_title": "У предложения должно быть название.",
  "p.proposals.problem_tradeoff": "Компромисс — от {min} до {max} символов.",
  "p.proposals.problem_not_ask": "Компромисс есть только у внешней просьбы.",
  "p.proposals.problem_decided":
    "По предложению уже принято решение, поэтому его компромисс больше нельзя изменить.",
  "p.proposals.problem_approve":
    "{title} — внешняя просьба: перед утверждением укажите её компромисс (не менее {min} символов).",
  "p.proposals.refused_create":
    "Отказано: у предложения должно быть название, у суммы — валюта, а компромисс — не более {max} символов.",
  "p.proposals.refused_tradeoff":
    "Отказано: компромисс — от {min} до {max} символов и допустим только у внешней просьбы, по которой ещё нет решения.",
  "p.proposals.refused_step":
    "Отказано: этот шаг не следует из статуса предложения.",
  "p.proposals.refused_gone": "Этого предложения больше нет.",
  "p.thresholds.opened_by_kind":
    "Открыто, потому что программа пересекла свой порог: {kind}.",
  "p.thresholds.joined_by_kind":
    "Пока это было открыто, программа пересекла ещё один порог: {kind}.",
  "p.deescalation.refused_tracker_taken":
    "Отказано: другая сводная карта уже читается трекером.",
  "p.deescalation.level_now_by": "Уровень {level}, задан {date}, автор: {who}.",
  "p.deescalation.level_history_item_by": "{level}, задан {date}, автор: {who}",
  "p.deescalation.evidence_recorded_by": "записано: {who}, {date}",
  "p.deescalation.evidence_recorded_on": "записано {date}",
  "p.proposals.refused_approve_status":
    "Отказано: утвердить можно только предложение, находящееся на рассмотрении.",
  "p.proposals.refused_tradeoff_required":
    "Отказано: внешней просьбе перед утверждением нужен указанный компромисс не короче {min} символов.",
  "p.ledger.untraced_no_readable_line":
    "Ни одна строка бюджета, которую вам можно читать, не опирается на это назначение, поэтому здесь нельзя проследить, куда идут его деньги.",
  "p.ledger.untraced_not_on_a_line":
    "Часть этого назначения ещё не запланирована ни по одной строке бюджета.",
  "p.ledger.untraced_lines_over_plan":
    "Строки бюджета планируют больше, чем содержит это назначение.",
  "p.ledger.untraced_other": "Часть денег этого назначения проследить нельзя.",
  "p.status.title_ppm": "Обновления статуса — PPM",
  "p.status.title": "Обновления статуса",
  "p.status.link": "Обновления статуса и их подтверждения",
  "p.status.intro":
    "Обновление статуса — это оценка человека, как идёт план (по плану, под угрозой или вне плана), с описанием происходящего словами. Здесь ничего не вычисляется как оценка.",
  "p.status.could_not_load": "Не удалось загрузить статус плана",
  "p.status.now_heading": "Положение плана",
  "p.status.no_update": "Обновлений статуса пока не опубликовано.",
  "p.status.cadence_is_one": "Обновление ожидается каждый день.",
  "p.status.cadence_is_other": "Обновление ожидается каждые {n} дней.",
  "p.status.standing_overdue_one":
    "Следующее обновление ожидалось {date}: просрочка {n} день.",
  "p.status.standing_overdue_other":
    "Следующее обновление ожидалось {date}: просрочка в днях — {n}.",
  "p.status.standing_due": "Обновление ожидается сегодня, {date}.",
  "p.status.standing_next": "Следующее обновление ожидается {date}.",
  "p.status.no_cadence":
    "Периодичность не задана, поэтому обновление никогда не ожидается и не просрочено.",
  "p.status.as_of": "По состоянию на {date}, в часовом поясе развёртывания.",
  "p.status.loading": "Загрузка…",
  "p.status.posted_on_by": "опубликовано {date}, автор: {who}",
  "p.status.posted_on": "опубликовано {date}",
  "p.status.claims_of": "Тезисы в обновлении от {date}",
  "p.status.source_metric": "{metric}: сейчас {value}",
  "p.status.source_metric_unknown": "{metric}: неизвестно (не ноль)",
  "p.status.source_unavailable": "Источник, который больше недоступен",
  "p.status.kind_task": "Задача: {title}",
  "p.status.kind_risk": "Риск: {title}",
  "p.status.kind_milestone": "Веха: {title}",
  "p.status.kind_issue": "Проблема: {title}",
  "p.status.kind_other": "Запись: {title}",
  "p.status.metric_overdue": "Просроченные задачи",
  "p.status.metric_due_soon": "Задачи со скорым сроком",
  "p.status.metric_blocked": "Заблокированные задачи",
  "p.status.metric_no_due_date": "Задачи без срока",
  "p.status.metric_def_overdue":
    "Открытые задачи, срок которых раньше сегодняшнего дня. Задача со сроком сегодня не просрочена. Неизвестно, если есть открытые задачи и ни у одной нет срока.",
  "p.status.metric_def_due_soon":
    "Открытые задачи со сроком сегодня или в ближайшие {days} дн. Неизвестно, если есть открытые задачи и ни у одной нет срока.",
  "p.status.metric_def_blocked": "Открытые задачи со статусом «заблокирована».",
  "p.status.metric_def_no_due_date":
    "Открытые задачи без срока: задачи, которых другие числа не видят.",
  "p.status.metrics_heading": "Как считаются числа",
  "p.status.metrics_says":
    "По открытым задачам плана (задачам, состояние которых не «выполнена»); считается при чтении обновления, а не при его публикации.",
  "p.status.cadence_heading": "Как часто ожидается обновление",
  "p.status.cadence_says":
    "Каждые от {min} до {max} дней. При установке отсчёт начинается заново с сегодняшнего дня, поэтому одна и та же периодичность повторно не отправляется. Установленную периодичность удалить нельзя.",
  "p.status.cadence_set_by": "Задана {date}, автор: {who}.",
  "p.status.cadence_set_on": "Задана {date}.",
  "p.status.f_cadence": "Дней между обновлениями (от {min} до {max})",
  "p.status.save_cadence": "Сохранить периодичность",
  "p.status.unchanged": "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.status.problem_cadence":
    "Периодичность — целое число дней от {min} до {max}.",
  "p.status.cadence_saved_one":
    "Теперь обновление ожидается каждый день; отсчёт начинается сегодня.",
  "p.status.cadence_saved_other":
    "Теперь обновление ожидается каждые {n} дней; отсчёт начинается сегодня.",
  "p.status.could_not_save": "Не удалось сохранить периодичность",
  "p.status.refused_gone": "Этого плана больше нет.",
  "p.status.history_heading": "Все обновления, сначала новые",
  "p.status.history_says":
    "Обновления никогда не изменяются и не удаляются. Каждый источник показан таким, какой он сейчас: удалённая запись больше недоступна, а каждое число — это его значение сегодня.",
  "p.status.history_none": "Обновлений пока нет.",
  "p.status.post_heading": "Опубликовать обновление",
  "p.status.post_says":
    "Выберите оценку и опишите происходящее (до {max} символов). Опубликованное обновление нельзя изменить или удалить, а его публикация заново запускает отсчёт периодичности.",
  "p.status.claims_rule":
    "Можно добавить до {claims} тезисов; каждый — предложение до {chars} символов со ссылкой не более чем на {sources} источников: числа ниже либо задачи, риски, вехи и проблемы плана. Каждый источник проверяется при публикации обновления.",
  "p.status.f_call": "Оценка",
  "p.status.call_on_track": "По плану",
  "p.status.call_at_risk": "Под угрозой",
  "p.status.call_off_track": "Вне плана",
  "p.status.f_narrative": "Что происходит (до {max} символов)",
  "p.status.claim_legend": "Тезис {n}",
  "p.status.f_claim_text": "Тезис (до {chars} символов)",
  "p.status.f_metrics": "Числа, на которые он ссылается",
  "p.status.f_record": "Запись плана, на которую ссылается тезис {n}",
  "p.status.choose_record": "Выберите запись",
  "p.status.add_record": "Добавить запись к тезису {n}",
  "p.status.claim_records": "Записи, на которые ссылается тезис {n}",
  "p.status.remove": "Удалить",
  "p.status.remove_source": "Удалить {source} из тезиса {n}",
  "p.status.remove_claim": "Удалить тезис {n}",
  "p.status.add_claim": "Добавить тезис",
  "p.status.post": "Опубликовать обновление",
  "p.status.posted": "Обновление опубликовано: {call}.",
  "p.status.could_not_post": "Не удалось опубликовать обновление",
  "p.status.problem_call":
    "Выберите «по плану», «под угрозой» или «вне плана».",
  "p.status.problem_narrative":
    "Опишите происходящее не более чем в {max} символах.",
  "p.status.problem_claims": "В обновлении не более {claims} тезисов.",
  "p.status.problem_claim_text": "Тезис — это предложение до {chars} символов.",
  "p.status.problem_sources":
    "Тезис ссылается не более чем на {sources} источников.",
  "p.status.problem_in_claim": "Тезис {n}: {problem}",
  "p.status.refused_update":
    "Отказано: источник, возможно, больше не является записью этого плана. Ничего не опубликовано; проверьте тезисы и опубликуйте снова.",
  "p.status.due_heading": "Ожидаемые обновления статуса",
  "p.status.due_none":
    "Ни у одного плана, который вам можно читать, нет ожидаемого обновления статуса.",
  "p.status.due_could_not_load":
    "Не удалось загрузить ожидаемые обновления статуса",
  "p.status.a_plan": "План",
  "p.status.due_last": "Последнее обновление: {call}, {date}.",
  "p.status.due_never": "Обновлений пока нет.",
  "p.status.group_task": "Задачи",
  "p.status.group_risk": "Риски",
  "p.status.group_milestone": "Вехи",
  "p.status.group_issue": "Проблемы",
  "l.enum.risk_status.open": "Открыт",
  "l.enum.risk_status.mitigating": "Снижается",
  "l.enum.risk_status.closed": "Закрыт",
  "l.enum.risk_status.materialized": "Реализовался",
  "p.risk_edit.change": "Изменить",
  "p.risk_edit.change_named": "Изменить: {title}",
  "p.risk_edit.heading": "Изменить риск «{title}»",
  "p.risk_edit.says":
    "Вероятность и влияние — целые числа от {min} до {max}; уровень риска — их произведение. Мера снижения — не более {max_bytes} байт (в некоторых письменностях символов меньше). Дату пересмотра можно изменить, но нельзя удалить. Отправляется только то, что вы изменили.",
  "p.risk_edit.says_escalate":
    "Чтобы записать, что риск наступил, используйте «Эскалировать» в таблице: там записывается и время.",
  "p.risk_edit.problem_status": "Выберите статус.",
  "p.risk_edit.problem_score":
    "Вероятность и влияние должны быть целыми числами от {min} до {max}.",
  "p.risk_edit.problem_mitigation": "Мера снижения длиннее {max_bytes} байт.",
  "p.risk_edit.problem_review_cleared":
    "Установленную дату пересмотра удалить нельзя; выберите другую дату.",
  "p.risk_edit.problem_review_date": "Дата пересмотра не является датой.",
  "p.risk_edit.unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.risk_edit.saved": "Риск «{title}» сохранён.",
  "p.risk_edit.could_not_save": "Не удалось сохранить риск",
  "p.risk_edit.refused_gone":
    "Этого риска больше нет в плане; список перечитан.",
  "p.risk_edit.refused_invalid":
    "Отказано: сервис не принял статус, оценки или меру снижения. Ничего не изменено.",
  "p.risk_edit.f_status": "Статус риска",
  "p.risk_edit.f_probability": "Вероятность (от {min} до {max})",
  "p.risk_edit.f_impact": "Влияние (от {min} до {max})",
  "p.risk_edit.f_mitigation": "Мера снижения",
  "p.risk_edit.f_review_date": "Дата пересмотра",
  "p.risk_edit.save": "Сохранить риск",
  "p.risk_edit.close": "Закрыть форму",
  "p.plan_issues.refused_gone":
    "Этой проблемы больше нет в плане; список перечитан.",
  "p.plan_issues.refused_remove":
    "Отказано: вам нельзя удалять проблемы в этом плане. Ничего не удалено.",
  "p.plan_issues.refused_remove_admin":
    "Отказано: по рабочему процессу этого типа проблему может удалить только администратор. Ничего не удалено.",
  "p.plan_issues.remove_admin_only":
    "Удалить «{title}» может только администратор: переходами или полями этой проблемы управляет рабочий процесс её типа. Ничего не отправлено.",
  "p.plan_issues.confirm_remove":
    "Удалить проблему «{title}»? Она исчезнет из всех списков; запись о ней сохранится для аудита.",
  "p.plan_issues.removed": "Проблема «{title}» удалена.",
  "p.plan_issues.remove": "Удалить",
  "p.plan_issues.remove_named": "Удалить: {title}",
  "p.plan_issues.remove_says":
    "Удаление убирает проблему из всех списков и не может быть отменено здесь; аудит хранит запись. Если рабочий процесс управляет переходами или полями, удалить проблему может только администратор.",
  "p.plan_issues.raised_on_by": "Заведена {date}, автор: {who}.",
  "p.plan_issues.raised_on": "Заведена {date}.",
  "p.plan_issues.from_risk_named": "Заведена из риска «{title}».",
  "p.plan_issues.from_a_risk":
    "Заведена из риска, которого больше нет в списке.",
  "p.plan_issues.from_a_task":
    "Заведена из заблокированной задачи (откройте задачу)",
  "p.plan_issues.answered_by_change":
    "Решается запросом на изменение (см. запросы на изменение плана)",
  "p.plan_issues.resolved_on": "Решена {date}.",
  "p.task.issue_heading": "Проблема из этой задачи",
  "p.task.issue_says":
    "Проблемой можно завести только заблокированную задачу и только один раз, пока эта проблема открыта. Проблема — это блокер средней серьёзности, ответственный за который — исполнитель задачи; сама задача не меняется.",
  "p.task.issue_raised": "Проблема заведена.",
  "p.task.issue_open_list": "Открыть проблемы плана",
  "p.task.issue_exists":
    "Из этой задачи уже заведена открытая проблема: «{title}».",
  "p.task.issue_not_blocked":
    "Эта задача не заблокирована, поэтому завести из неё проблему нельзя. Сначала отметьте её заблокированной на доске.",
  "p.task.issue_raise": "Завести проблему из этой задачи",
  "p.task.refused_from_task_not_blocked":
    "Отказано: проблему можно завести только из заблокированной задачи, а эта уже не заблокирована. Задача перечитана.",
  "p.task.refused_from_task_exists":
    "Отказано: из этой задачи уже заведена открытая проблема. Проблемы перечитаны.",
  "p.task.refused_from_task_forbidden":
    "Отказано: вам нельзя заводить проблемы в этом плане.",
  "p.task.refused_from_task_gone": "Этой задачи больше нет в плане.",
  "p.plan_change_requests.refused_gone":
    "Этого запроса на изменение больше нет в плане; список перечитан.",
  "p.plan_change_requests.refused_links_conflict":
    "Отказано: запрос закрыт, или одна из проблем уже решается другим запросом на изменение. Запрос и проблемы плана перечитаны.",
  "p.plan_change_requests.refused_links_invalid":
    "Отказано: выбранной проблемы или риска больше нет в плане. Проблемы и риски плана перечитаны.",
  "p.plan_change_requests.refused_preview":
    "Отказано: в этом запросе нет правок «что если», которые может прочитать планировщик, поэтому прогноза нет.",
  "p.plan_change_requests.refused_baseline":
    "Отказано: по этому изменению базовый план уже зафиксирован, или оно больше не утверждено. Запрос перечитан.",
  "p.plan_change_requests.problem_links_too_many":
    "Выберите не более {max} проблем и {max} рисков.",
  "p.plan_change_requests.problem_links_elsewhere":
    "«{title}» уже решается другим запросом на изменение; снимите отметку.",
  "p.plan_change_requests.links_unchanged":
    "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.plan_change_requests.links_saved":
    "Проблемы и риски, которые решает это изменение, сохранены.",
  "p.plan_change_requests.confirm_baseline":
    "Зафиксировать базовый план расписания из-за «{title}»? Для этого изменения можно зафиксировать только один, и отменить это нельзя.",
  "p.plan_change_requests.baseline_taken":
    "Из-за этого изменения зафиксирована версия базового плана {version}.",
  "p.plan_change_requests.stated_impact":
    "Влияние, указанное автором запроса: {impact}.",
  "p.plan_change_requests.implemented_on": "Реализовано {date}.",
  "p.plan_change_requests.implement": "Отметить как реализованное",
  "p.plan_change_requests.implemented": "Изменение отмечено как реализованное.",
  "p.plan_change_requests.links_heading":
    "Проблемы и риски, которые решает это изменение",
  "p.plan_change_requests.links_none":
    "Оно пока не решает ни одной проблемы или риска.",
  "p.plan_change_requests.answers_issue": "Проблема: {title}",
  "p.plan_change_requests.answers_risk": "Риск: {title}",
  "p.plan_change_requests.an_issue": "проблема, которой больше нет в списке",
  "p.plan_change_requests.a_risk": "риск, которого больше нет в списке",
  "p.plan_change_requests.links_says":
    "Выбирается, пока запрос открыт (черновик, подан, на рассмотрении или утверждён). Проблему решает не более одного запроса на изменение; до {max} проблем и {max} рисков. Сохранение заменяет весь набор.",
  "p.plan_change_requests.links_issues": "Проблемы этого плана",
  "p.plan_change_requests.issue_elsewhere":
    "{title} (решается другим запросом на изменение)",
  "p.plan_change_requests.links_no_issues": "В этом плане нет проблем.",
  "p.plan_change_requests.links_risks": "Риски этого плана",
  "p.plan_change_requests.links_no_risks": "В этом плане нет рисков.",
  "p.plan_change_requests.links_save": "Сохранить, что оно решает",
  "p.plan_change_requests.links_closed":
    "Связи закрытого запроса изменить нельзя.",
  "p.plan_change_requests.forecast_heading": "Прогноз «что если»",
  "p.plan_change_requests.forecast_says":
    "Прогноз по текущему расписанию с применением правок «что если» из этого запроса. Чтение ничего не меняет; влияние, указанное выше, — собственная оценка автора запроса.",
  "p.plan_change_requests.forecast_show": "Показать прогноз",
  "p.plan_change_requests.forecast_label": "Прогноз — ничего не изменено",
  "p.plan_change_requests.forecast_finish":
    "Окончание плана: сейчас {before}, с изменением {after}.",
  "p.plan_change_requests.forecast_finish_unknown":
    "Окончание плана спрогнозировать нельзя.",
  "p.plan_change_requests.forecast_moved_one":
    "1 задача завершилась бы в другой день.",
  "p.plan_change_requests.forecast_moved_other":
    "Задач, которые завершились бы в другой день: {n}.",
  "p.plan_change_requests.forecast_later_one":
    "{title}: завершается на 1 рабочий день позже",
  "p.plan_change_requests.forecast_later_other":
    "{title}: завершается позже, рабочих дней — {n}",
  "p.plan_change_requests.forecast_earlier_one":
    "{title}: завершается на 1 рабочий день раньше",
  "p.plan_change_requests.forecast_earlier_other":
    "{title}: завершается раньше, рабочих дней — {n}",
  "p.plan_change_requests.a_task": "Задача",
  "p.plan_change_requests.forecast_labour":
    "Плановые затраты на труд изменились бы на {amount}.",
  "p.plan_change_requests.forecast_labour_hidden":
    "Прогноз затрат на труд показан только тем, кому разрешено видеть ставки стоимости.",
  "p.plan_change_requests.forecast_labour_none":
    "Нет работ со стоимостью, поэтому нет и прогноза затрат на труд.",
  "p.plan_change_requests.forecast_labour_unknown":
    "Прогноза затрат на труд нет.",
  "p.plan_change_requests.forecast_no_edits":
    "В этом запросе нет правок «что если», поэтому прогноза нет; указанное влияние — собственная оценка автора запроса.",
  "p.plan_change_requests.baseline_heading":
    "Базовый план из-за этого изменения",
  "p.plan_change_requests.baseline_done":
    "Из-за этого изменения зафиксирован базовый план расписания.",
  "p.plan_change_requests.baseline_says":
    "Зафиксировать его — выбор человека; автоматически базовый план не пересоздаётся. Один базовый план на изменение, причина в нём называет запрос, и отменить это нельзя.",
  "p.plan_change_requests.baseline_take": "Зафиксировать базовый план",
  "p.plan_change_requests.baseline_not_yet":
    "Базовый план можно зафиксировать, когда изменение утверждено.",
  "p.escalations.problem_outcome":
    "Выберите исход: решено, дано указание или отклонено.",
  "p.escalations.problem_response":
    "Напишите ответ от {min} до {max} символов.",
  "p.escalations.refused_ack_right":
    "Отказано: для принятия эскалации к сведению нужно право принимать эскалации к сведению (оно есть у администратора). Ничего не записано.",
  "p.escalations.refused_resolve_right":
    "Отказано: для разрешения эскалации нужно право разрешать эскалации (оно есть у администратора). Ничего не записано.",
  "p.escalations.refused_ack_state":
    "Отказано: эта эскалация тем временем разрешена или отозвана, поэтому принять её к сведению нельзя. Данные перечитаны.",
  "p.escalations.refused_resolve_state":
    "Отказано: эта эскалация отозвана или уже разрешена с другим исходом или ответом; разрешение никогда не перезаписывается. Данные перечитаны.",
  "p.escalations.refused_resolve_invalid":
    "Отказано: исход или ответ не приняты. Проверьте правила выше и повторите.",
  "p.escalations.refused_gone":
    "Этой эскалации больше нет, или она вам уже недоступна. Список перечитан.",
  "p.escalations.could_not_save": "Это не записано.",
  "p.escalations.by_unrecorded": "кто-то не записан",
  "p.escalations.ack_record":
    "Принято к сведению на уровне {level}: {by}, {at}.",
  "p.escalations.resolution_record": "Разрешено {at}: {by}; исход: {outcome}.",
  "p.escalations.outcome_unrecorded": "исход не записан",
  "p.escalations.records_heading": "Принятие к сведению и разрешение",
  "p.escalations.records_failed":
    "Не удалось прочитать принятие к сведению или разрешение; история ниже по-прежнему показывает, что произошло.",
  "p.escalations.ack_note_line": "Примечание: {note}",
  "p.escalations.ack_none": "На текущем уровне не принято к сведению.",
  "p.escalations.resolution_kept":
    "Разрешение — это запись: оно не изменяется и не перезаписывается.",
  "p.escalations.records_rules":
    "Для принятия к сведению нужно право принимать эскалации к сведению, для разрешения — право разрешать их; у администратора есть оба, а эта страница заранее не знает, есть ли они у вас. Принятие к сведению действует на текущем уровне: повышение уровня его снимает. Разрешение — это запись, и оно никогда не перезаписывается.",
  "p.escalations.ack_heading": "Принять эту эскалацию к сведению",
  "p.escalations.ack_note": "Примечание (необязательно)",
  "p.escalations.ack_done": "Принято к сведению; запись показана ниже.",
  "p.escalations.resolve_heading": "Разрешить эту эскалацию",
  "p.escalations.resolve_says":
    "Ответ — от {min} до {max} символов. Перед записью вас спросят, а после записи изменить его нельзя.",
  "p.escalations.confirm_resolve":
    "Записать это разрешение для «{title}»? После этого его нельзя изменить или перезаписать.",
  "p.escalations.resolution_done": "Разрешено; запись показана ниже.",
  "p.register_completeness.link": "Полнота реестра",
  "p.register_completeness.title": "Полнота реестра",
  "p.register_completeness.title_ppm": "Полнота реестра — PPM",
  "p.register_completeness.intro":
    "Насколько полон реестр каждой программы: из того, что должно быть записано по программе, какая доля есть, и каждое отсутствующее названо. Перечислены только программы (планы вида «Программа»), которые вам можно читать.",
  "p.register_completeness.rules":
    "Десять пунктов: старший ответственный владелец (его заменяет руководитель программы), партнёры, стадия, базовый план расписания, целевая дата, бюджет, источник финансирования в каждой строке бюджета, оценка уверенности в поставке, риски и зависимости. Партнёры требуются только там, где модель поставки предполагает партнёров, а источник финансирования — только там, где есть строка бюджета. Программа и подчинённые ей планы считаются вместе.",
  "p.register_completeness.switch_on":
    "Правило «нет реестра — нет денег» включено: выпуск строки бюджета или оформление твёрдого назначения в неполной программе отклоняется с указанием, чего не хватает.",
  "p.register_completeness.switch_off":
    "Правило «нет реестра — нет денег» выключено: выпуск строки бюджета или оформление твёрдого назначения в неполной программе проходит, а предупреждение сохраняется в журнале аудита.",
  "p.register_completeness.count_one": "Полна {complete} программа из {n}.",
  "p.register_completeness.count_other": "Полных программ: {complete} из {n}.",
  "p.register_completeness.none": "Программ, которые вам можно читать, нет.",
  "p.register_completeness.loading": "Загрузка…",
  "p.register_completeness.could_not_load":
    "Не удалось загрузить полноту реестра.",
  "p.register_completeness.table": "Программы и их реестры",
  "p.register_completeness.col_programme": "Программа",
  "p.register_completeness.col_state": "Состояние",
  "p.register_completeness.col_score": "Доля записанного",
  "p.register_completeness.col_on_record": "Записанные поля",
  "p.register_completeness.col_missing": "Отсутствует",
  "p.register_completeness.col_not_asked": "Не требуется",
  "p.register_completeness.of": "{present} из {applicable}",
  "p.register_completeness.state_complete": "полный",
  "p.register_completeness.state_incomplete": "неполный",
  "p.register_completeness.field_other": "другое поле",
  "p.register_completeness.none_listed": "нет",
  "p.meetings.title": "Совещания",
  "p.meetings.title_ppm": "Совещания — PPM",
  "p.meetings.link": "Совещания, протоколы и действия",
  "p.meetings.intro":
    "Совещание планируется с названием, датой и повесткой. Затем оно один раз записывается как проведённое, с протоколом и итогами (решениями, действиями и заметками), либо отменяется. Проведённое или отменённое совещание — это запись: протокол проведённого нельзя исправить, а отменённое нельзя восстановить. Участники не записываются.",
  "p.meetings.plan_heading": "Запланировать совещание",
  "p.meetings.plan_says":
    "Название до {title} символов и дата. В повестке до {items} пунктов по {chars} символов каждый; пункт может указывать на задачу, риск, веху или проблему этого плана.",
  "p.meetings.f_title": "Название (до {max} символов)",
  "p.meetings.f_date": "Дата, на которую оно запланировано",
  "p.meetings.agenda_item": "Пункт повестки {n}",
  "p.meetings.f_item_text": "О чём пункт повестки {n} (до {max} символов)",
  "p.meetings.f_item_link": "Запись, которой касается пункт повестки {n}",
  "p.meetings.no_record": "Без записи",
  "p.meetings.records_loading": "Загрузка записей плана…",
  "p.meetings.remove_item": "Удалить пункт повестки {n}",
  "p.meetings.add_item": "Добавить пункт повестки",
  "p.meetings.plan": "Запланировать совещание",
  "p.meetings.planned": "Запланировано: {title}, {date}.",
  "p.meetings.list_heading": "Совещания плана, сначала новые",
  "p.meetings.pdf_says":
    "Каждое совещание скачивается как документ PDF с тем, что показано на этой странице; его постоянные слова (заголовки и состояние) — на английском.",
  "p.meetings.loading": "Загрузка совещаний…",
  "p.meetings.none": "Совещаний пока нет.",
  "p.meetings.status_planned": "запланировано",
  "p.meetings.status_held": "проведено",
  "p.meetings.status_cancelled": "отменено",
  "p.meetings.held_on": "проведено {date}",
  "p.meetings.scheduled_on": "запланировано на {date}",
  "p.meetings.show": "Открыть совещание {title}",
  "p.meetings.hide": "Закрыть совещание {title}",
  "p.meetings.pdf": "Скачать как PDF",
  "p.meetings.pdf_named": "Скачать совещание {title} как PDF",
  "p.meetings.agenda_heading": "Повестка",
  "p.meetings.agenda_none": "Нет пунктов повестки.",
  "p.meetings.minutes_heading": "Протокол",
  "p.meetings.outcomes_heading": "Итоги",
  "p.meetings.outcomes_none": "Итоги не записаны.",
  "p.meetings.outcome_decision": "Решение",
  "p.meetings.outcome_action": "Действие",
  "p.meetings.outcome_note": "Заметка",
  "p.meetings.about": "о: {record}",
  "p.meetings.link_gone": "о записи, которой больше нет",
  "p.meetings.task_made": "задача, созданная из итога {n}",
  "p.meetings.held_kept":
    "Это совещание — запись: его протокол и итоги не изменяются.",
  "p.meetings.cancelled_kept":
    "Это совещание отменено. Это запись, и восстановить её нельзя.",
  "p.meetings.held_heading": "Записать {title} как проведённое",
  "p.meetings.held_says":
    "Протокол до {minutes} символов и до {items} итогов по {chars} символов каждый: каждый — решение, действие или заметка, и каждый может указывать на запись этого плана. Датой проведения считается плановая дата, если вы не укажете другую. Это делается один раз: после этого протокол исправить нельзя.",
  "p.meetings.f_minutes": "Протокол (до {max} символов)",
  "p.meetings.f_held_on": "Дата проведения (пусто — плановая дата)",
  "p.meetings.outcome_n": "Итог {n}",
  "p.meetings.f_outcome_kind": "Чем является итог {n}",
  "p.meetings.choose_kind": "Выберите…",
  "p.meetings.f_outcome_text": "Итог {n} словами (до {max} символов)",
  "p.meetings.f_outcome_link": "Запись, которой касается итог {n}",
  "p.meetings.remove_outcome": "Удалить итог {n}",
  "p.meetings.add_outcome": "Добавить итог",
  "p.meetings.record_held": "Записать как проведённое",
  "p.meetings.cancel_meeting": "Отменить совещание {title}",
  "p.meetings.confirm_held":
    "Записать {title} как проведённое? После этого протокол и итоги исправить нельзя.",
  "p.meetings.confirm_cancel":
    "Отменить {title}? Отменённое совещание восстановить нельзя.",
  "p.meetings.held_done": "Записано как проведённое: {title}.",
  "p.meetings.cancelled_done": "Отменено: {title}.",
  "p.meetings.problem_title": "Укажите название до {title} символов.",
  "p.meetings.problem_date": "Укажите дату.",
  "p.meetings.problem_agenda_count": "В повестке не более {items} пунктов.",
  "p.meetings.problem_outcome_count": "У совещания не более {items} итогов.",
  "p.meetings.problem_item_text": "Напишите не более {chars} символов.",
  "p.meetings.problem_minutes":
    "Напишите протокол не более чем в {minutes} символах.",
  "p.meetings.problem_outcome_kind": "Выберите решение, действие или заметку.",
  "p.meetings.problem_on_line": "Для {line}: {problem}",
  "p.meetings.could_not_load": "Не удалось загрузить совещания.",
  "p.meetings.could_not_save": "Совещание не изменено.",
  "p.meetings.refused_gone":
    "Отказано: этого совещания или плана больше нет. Список перечитан.",
  "p.meetings.refused_settled":
    "Отказано: это совещание уже проведено или отменено, поэтому это запись, и она не изменяется. Данные перечитаны.",
  "p.meetings.refused_link":
    "Отказано: запись, на которую указывает пункт, больше не относится к этому плану. Ничего не сохранено; выберите записи заново.",
  "p.meetings.refused_not_held":
    "Отказано: задачи предлагаются только по совещанию, которое проведено и имеет протокол.",
  "p.meetings.refused_commit_state":
    "Отказано: совещание не проведено, или по одному из этих действий тем временем появилась задача. Ничего не создано; совещание перечитано, поэтому проверьте снова.",
  "p.meetings.refused_commit_invalid":
    "Отказано: предложение больше не готово. Ничего не создано; проверьте предложения снова.",
  "p.meetings.refused_proposals":
    "Отказано: отправьте от 1 до {count} предложений.",
  "p.meetings.tasks_heading": "Задачи из действий",
  "p.meetings.tasks_says":
    "Для каждого действия, у которого ещё нет задачи, можно предложить её: название до {max} символов и фрагмент протокола, подтверждающий её, не короче {min} символов (регистр и пробелы не учитываются). Исполнитель необязателен и должен уже быть среди задач этого плана или в его матрице ответственности, в виде ссылки вроде person:…; срок необязателен, от {first} до {last}; баллы — от 0 до {points}. До {count} за раз. Проверка ничего не записывает; затем задачи создаются все или ни одной, и на действие приходится только одна задача.",
  "p.meetings.proposal_legend": "Действие {n}: {text}",
  "p.meetings.f_propose": "Предложить задачу для действия {n}",
  "p.meetings.f_task_title":
    "Название задачи для действия {n} (до {max} символов)",
  "p.meetings.f_excerpt":
    "Фрагмент протокола для действия {n} (не менее {min} символов)",
  "p.meetings.f_assignee":
    "Исполнитель для действия {n} (необязательная ссылка)",
  "p.meetings.f_due": "Срок для действия {n} (необязательно)",
  "p.meetings.f_points":
    "Баллы для действия {n} (необязательно, от 0 до {max})",
  "p.meetings.check": "Проверить предложения",
  "p.meetings.create_tasks": "Создать задачи",
  "p.meetings.none_chosen":
    "Ни одно предложение не выбрано, поэтому ничего не отправлено.",
  "p.meetings.checked_one":
    "Проверено, ничего не записано: готово предложений — {n}, не готово — {dropped}.",
  "p.meetings.checked_other":
    "Проверено, ничего не записано: готово предложений — {n}, не готово — {dropped}.",
  "p.meetings.ready": "Готово, без исполнителя.",
  "p.meetings.ready_for": "Готово, для {who}.",
  "p.meetings.not_all_ready":
    "Прежде чем создавать задачи, каждое выбранное предложение должно быть готово: измените или снимите отметку с неготовых и проверьте снова.",
  "p.meetings.check_again":
    "Предложения изменились после проверки: проверьте их снова.",
  "p.meetings.confirm_tasks_one":
    "Создать {n} задачу из действий {title}? На действие приходится только одна задача.",
  "p.meetings.confirm_tasks_other":
    "Создать задач из действий {title}: {n}? На действие приходится только одна задача.",
  "p.meetings.tasks_made_one":
    "Создана {n} задача; она связана под своим действием.",
  "p.meetings.tasks_made_other":
    "Создано задач: {n}; каждая связана под своим действием.",
  "p.meetings.could_not_check": "Не удалось проверить предложения.",
  "p.meetings.could_not_create": "Задачи не созданы.",
  "p.meetings.dropped_title_invalid":
    "Не готово: название пустое или слишком длинное.",
  "p.meetings.dropped_excerpt_not_in_minutes":
    "Не готово: фрагмент не найден в протоколе или слишком короткий.",
  "p.meetings.dropped_outcome_not_found":
    "Не готово: у совещания нет такого итога.",
  "p.meetings.dropped_outcome_not_an_action":
    "Не готово: этот итог не является действием.",
  "p.meetings.dropped_outcome_already_has_a_task":
    "Не готово: у этого действия уже есть задача.",
  "p.meetings.dropped_due_outside_window":
    "Не готово: срок вне допустимых дат.",
  "p.meetings.dropped_points_invalid":
    "Не готово: баллы вне допустимого диапазона.",
  "p.meetings.dropped_assignee_not_in_plan":
    "Не готово: исполнитель не входит в число людей этого плана (в его задачах или матрице ответственности).",
  "p.meetings.dropped_outcome_proposed_twice":
    "Не готово: одно и то же действие предложено дважды.",
  "p.meetings.dropped_other":
    "Не готово по причине, которую эта страница не знает.",
  "p.meetings.problem_proposals_count":
    "Предлагайте не более {count} задач за раз.",
  "p.meetings.problem_task_title": "Укажите название задачи до {max} символов.",
  "p.meetings.problem_excerpt":
    "Приведите фрагмент из протокола не короче {min} символов.",
  "p.meetings.problem_due":
    "Срок должен приходиться на период от недели до совещания до года после него.",
  "p.meetings.problem_points": "Баллы — целое число от 0 до {points}.",
  "p.lessons.title": "Извлечённые уроки",
  "p.lessons.title_ppm": "Извлечённые уроки — PPM",
  "p.lessons.link": "Извлечённые уроки",
  "p.lessons.intro":
    "Урок записывается на контрольной точке фазы или при закрытии плана. Он никогда не изменяется: его можно только отозвать, а отозванный урок больше не показывается (он остаётся в журнале аудита). Новому предложению, название которого похоже на название этого плана, показываются его уроки.",
  "p.lessons.record_heading": "Записать урок",
  "p.lessons.record_says":
    "Когда (контрольная точка или закрытие), что это, краткое описание до {max} символов и, по желанию, подробности до {bytes} байт (буква с диакритикой или другая письменность занимает больше одного) и задача, риск, веха или проблема этого плана, к которой он относится.",
  "p.lessons.f_occasion": "Когда",
  "p.lessons.choose_occasion": "Выберите контрольную точку или закрытие…",
  "p.lessons.closure": "Закрытие",
  "p.lessons.occasion_other": "Другой случай",
  "p.lessons.f_category": "Что это",
  "p.lessons.category_went_well": "Получилось хорошо",
  "p.lessons.category_went_badly": "Получилось плохо",
  "p.lessons.category_recommendation": "Рекомендация",
  "p.lessons.category_other": "Другой вид",
  "p.lessons.f_summary": "Краткое описание (до {max} символов)",
  "p.lessons.f_detail": "Подробности (необязательно, до {bytes} байт)",
  "p.lessons.f_link": "Запись, к которой относится (необязательно)",
  "p.lessons.record": "Записать урок",
  "p.lessons.recorded": "Записано: {summary}",
  "p.lessons.problem_occasion": "Выберите контрольную точку или закрытие.",
  "p.lessons.problem_category":
    "Выберите «получилось хорошо», «получилось плохо» или «рекомендация».",
  "p.lessons.problem_summary": "Напишите краткое описание до {max} символов.",
  "p.lessons.problem_detail": "Подробности длиннее {bytes} байт.",
  "p.lessons.list_heading": "Уроки плана, сначала старые",
  "p.lessons.loading": "Загрузка уроков…",
  "p.lessons.none": "Уроков пока нет.",
  "p.lessons.count_one": "{n} урок.",
  "p.lessons.count_other": "Уроков: {n}.",
  "p.lessons.recorded_on": "записано {date}",
  "p.lessons.recorded_on_by": "записано {date}, автор: {who}",
  "p.lessons.link_unlisted":
    "о записи, которой нет среди перечисленных здесь задач, рисков, вех и проблем плана",
  "p.lessons.withdraw": "Отозвать урок {summary}",
  "p.lessons.withdraw_short": "Отозвать",
  "p.lessons.confirm_withdraw":
    "Отозвать урок «{summary}»? Он больше не будет показываться ни здесь, ни новым предложениям; в журнале аудита он останется.",
  "p.lessons.withdrawn": "Отозвано: {summary}",
  "p.lessons.could_not_load": "Не удалось загрузить уроки.",
  "p.lessons.could_not_save": "Урок не изменён.",
  "p.lessons.refused_gone": "Отказано: плана больше нет.",
  "p.lessons.refused_withdrawn":
    "Отказано: этот урок уже отозван. Список перечитан.",
  "p.lessons.refused_link":
    "Отказано: выбранная запись больше не относится к этому плану. Ничего не записано; выберите её заново.",
  "p.lessons.refs_heading": "Что ссылается на запись",
  "p.lessons.refs_says":
    "Выберите задачу, риск, веху или проблему плана, чтобы вывести пункты совещаний и уроки, которые на неё указывают (до 100 каждого вида).",
  "p.lessons.f_target": "Запись",
  "p.lessons.choose_target": "Выберите запись…",
  "p.lessons.show_refs": "Показать, что на неё ссылается",
  "p.lessons.problem_target": "Сначала выберите запись.",
  "p.lessons.refs_count_one": "Ссылается {n} элемент.",
  "p.lessons.refs_count_other": "Ссылающихся элементов: {n}.",
  "p.lessons.refs_meetings": "Пункты совещаний",
  "p.lessons.refs_lessons": "Уроки, которые на неё указывают",
  "p.lessons.refs_agenda": "Повестка",
  "p.lessons.refused_no_record":
    "Эта запись больше не относится к этому плану, или вы не можете её читать.",
  "p.lessons.could_not_read_refs":
    "Не удалось прочитать, что на неё ссылается.",
  "p.lessons.similar": "Уроки похожих планов",
  "p.lessons.similar_named": "Уроки планов, похожих на {title}",
  "p.lessons.similar_heading": "Уроки планов, похожих на {title}",
  "p.lessons.similar_says":
    "Сопоставление только по названию со всеми планами, у которых есть уроки, закрытыми или нет, лучшее совпадение первым. Оценка и уверенность принадлежат средству сопоставления.",
  "p.lessons.similar_none": "У похожих планов нет уроков.",
  "p.lessons.match": "совпадение {score}, уверенность: {confidence}.",
  "p.lessons.match_title": "Названия похожи на {score}.",
  "p.lessons.confidence_high": "высокая",
  "p.lessons.confidence_medium": "средняя",
  "p.lessons.confidence_low": "низкая",
  "p.lessons.refused_proposal_gone":
    "Этого предложения больше нет, или вы не можете его читать.",
  "p.lessons.could_not_read_similar":
    "Не удалось прочитать уроки похожих планов.",
  "p.director_pack.open_by_severity": "Открытые по серьёзности: {list}",
  "p.director_pack.issues_overdue": "Просроченные проблемы: {n}",
  "p.director_pack.issues_in_window":
    "За период: заведено {raised}, решено {resolved}",
  "p.director_pack.oldest_issue":
    "Самая давняя открытая проблема: {title} (календарных дней открыта: {n})",
  "p.director_pack.approved_not_implemented":
    "Утверждено, но ещё не реализовано: {n}",
  "p.director_pack.decided_in_window": "Решено за период: {n}",
  "p.director_pack.changes_by_status": "По статусам: {list}",
  "p.director_pack.oldest_change":
    "Самый давний открытый запрос на изменение: {title} (календарных дней открыт: {n})",
  "p.live_pack.heading": "Показатели программ сейчас",
  "p.live_pack.says":
    "Читаются сейчас по планам, которые вам можно читать, и показываются так же, как в пакете для руководителя. Выпущенный пакет хранит цифры, с которыми он выпущен; предыдущая точка диаграммы-«термометра» берётся только из выпущенного пакета.",
  "p.live_pack.f_ac": "Фактическая стоимость для освоенного объёма",
  "p.live_pack.ac_budget_lines": "Строки бюджета (по умолчанию)",
  "p.live_pack.ac_labour": "Записанные затраты на труд",
  "p.live_pack.ac_both": "Строки бюджета и затраты на труд",
  "p.live_pack.f_window":
    "Проблемы и запросы на изменение за последние дни (от {min} до {max}; пусто — 30)",
  "p.live_pack.read_again": "Прочитать снова",
  "p.live_pack.problem_window":
    "Число дней должно быть целым от {min} до {max} или пустым (тогда 30).",
  "p.live_pack.could_not_read_fever": "Не удалось прочитать диаграмму буфера.",
  "p.live_pack.could_not_read_evm": "Не удалось прочитать освоенный объём.",
  "p.live_pack.could_not_read_benefits": "Не удалось прочитать выгоды.",
  "p.live_pack.could_not_read_issues":
    "Не удалось прочитать проблемы и запросы на изменение.",
  "p.live_pack.issues_heading": "Проблемы и запросы на изменение",
  "p.live_pack.issues_none":
    "Нет планов с проблемами или запросами на изменение, которые вам можно читать.",
  "p.live_pack.nothing": "Показывать нечего.",
  "p.forecast.heading": "Прогноз по завершении",
  "p.forecast.says":
    "Оценка по завершении — это фактическая стоимость на сегодня плюс оценка до завершения. Оценка до завершения — последняя итоговая цифра контроля проекта в валюте прогноза, а при её отсутствии — то, что ещё планирует последний базовый план. Читается сейчас; ничего не сохраняется.",
  "p.forecast.absent_no_currency":
    "Прогноза нет: у плана нет ни базового бюджета, ни итоговой цифры контроля проекта, поэтому нет валюты для прогноза.",
  "p.forecast.c_actual": "Фактическая стоимость на сегодня",
  "p.forecast.c_etc": "Оценка до завершения",
  "p.forecast.c_eac": "Оценка по завершении",
  "p.forecast.c_plans": "Планы",
  "p.forecast.not_known": "неизвестно",
  "p.forecast.etc_tpc": "по последней итоговой цифре контроля проекта",
  "p.forecast.etc_baseline": "по тому, что ещё планирует базовый план",
  "p.forecast.no_baseline_used": "Базовый бюджет не использовался.",
  "p.forecast.baseline_used": "Использован базовый бюджет версии {version}.",
  "p.forecast.as_of": "Прочитано {date}.",
  "p.forecast.other_currency":
    "Фактическая стоимость, записанная в валюте, отличной от {currency}, не добавляется.",
  "p.forecast.baselines_heading": "Базовые бюджеты",
  "p.forecast.baselines_says":
    "Базовый бюджет — утверждённый бюджет по периодам в одной валюте. После утверждения он заморожен и никогда не редактируется; повторное утверждение базового плана — это новая версия, и каждая версия после первой требует причины. Базовый бюджет содержит от 1 до {max} периодов, у каждого окончание не раньше начала, а плановая сумма не меньше нуля.",
  "p.forecast.no_baselines": "Ни один базовый бюджет не утверждён.",
  "p.forecast.version_line":
    "Версия {version} · {currency} · утверждена {date}",
  "p.forecast.period_line": "{start} — {end}: {amount}",
  "p.forecast.first_baseline_heading": "Утвердить первый базовый бюджет",
  "p.forecast.rebaseline_heading": "Утвердить версию {version}",
  "p.forecast.f_currency": "Валюта (три буквы)",
  "p.forecast.periods_legend": "Периоды",
  "p.forecast.f_start": "Начало периода {n}",
  "p.forecast.f_end": "Окончание периода {n}",
  "p.forecast.f_amount": "Плановая сумма периода {n}",
  "p.forecast.remove_period": "Удалить период {n}",
  "p.forecast.add_period": "Добавить период",
  "p.forecast.f_reason": "Причина (для первого необязательна)",
  "p.forecast.f_reason_needed":
    "Почему бюджет утверждается заново (обязательно)",
  "p.forecast.approve": "Утвердить базовый бюджет",
  "p.forecast.confirm_baseline":
    "Утвердить базовый бюджет версии {version}? После утверждения он заморожен, и его нельзя изменить или удалить.",
  "p.forecast.baseline_approved": "Версия {version} утверждена.",
  "p.forecast.problem_currency":
    "Валюта должна быть трёхбуквенным кодом, например GBP или EUR.",
  "p.forecast.problem_no_periods": "Укажите хотя бы один период.",
  "p.forecast.problem_too_many": "В базовом бюджете не более {max} периодов.",
  "p.forecast.problem_dates":
    "У каждого периода должны быть дата начала и дата окончания.",
  "p.forecast.problem_order":
    "Период не может закончиться раньше, чем начался.",
  "p.forecast.problem_amount":
    "Каждая плановая сумма должна быть числом не меньше нуля.",
  "p.forecast.problem_reason":
    "Для повторного утверждения базового бюджета нужна причина.",
  "p.forecast.problem_reason_long":
    "Причина слишком длинная: не более {bytes} байт.",
  "p.forecast.problem_same":
    "Это то же самое, что последняя версия; ничего не отправлено.",
  "p.forecast.refused_gone":
    "Этого плана больше нет, или вы не можете его читать.",
  "p.forecast.refused_baseline":
    "Базовый бюджет отклонён: тем временем могла быть утверждена другая версия, из-за чего нужна причина. Список перечитан.",
  "p.forecast.could_not_approve": "Не удалось утвердить базовый бюджет.",
  "p.forecast.could_not_read": "Не удалось прочитать прогноз.",
  "p.forecast.could_not_read_baselines":
    "Не удалось прочитать базовые бюджеты.",
  "p.forecast.could_not_read_lines": "Не удалось прочитать строки бюджета.",
  "p.forecast.tranches_heading":
    "Финансирование, удерживаемое до контрольной точки",
  "p.forecast.tranches_says":
    "Строка бюджета с контрольной точкой финансирования удерживается, пока стадия плана не достигнет этой точки; тогда её можно выпустить один раз, и только после этого по ней можно записывать фактическую стоимость. Выпуск нельзя отменить.",
  "p.forecast.stage_now": "Стадия плана: {stage}.",
  "p.forecast.stage_none": "План ещё не прошёл ни одной контрольной точки.",
  "p.forecast.no_tranches":
    "Ни одна строка бюджета не удерживается до контрольной точки.",
  "p.forecast.gate_line": "удерживается до: {gate}",
  "p.forecast.state_released": "выпущено {date}",
  "p.forecast.state_held":
    "всё ещё удерживается: контрольная точка не пройдена",
  "p.forecast.release": "Выпустить: {description}",
  "p.forecast.confirm_release":
    "Выпустить {description}, удерживаемое до {gate}? Выпуск нельзя отменить.",
  "p.forecast.released": "{description} выпущено.",
  "p.forecast.refused_line_gone": "Этой строки бюджета или плана больше нет.",
  "p.forecast.refused_register":
    "Отказано: реестр программы неполон. Сначала заполните его на странице полноты реестра.",
  "p.forecast.refused_release":
    "Отказано: строка больше не удерживается, или её контрольная точка не пройдена. Строки перечитаны.",
  "p.forecast.could_not_release": "Не удалось выпустить строку.",
  "p.forecast.rollup_heading": "Прогноз по завершении, сводный",
  "p.forecast.rollup_says":
    "От одного плана по подчинённым ему планам, до {max} уровней вглубь, одна строка на валюту: валюты никогда не складываются. Планы без валюты для прогноза учитываются в числе, но не прибавляются.",
  "p.forecast.f_plan": "План",
  "p.forecast.choose_plan": "Выберите план",
  "p.forecast.f_depth": "Уровней вглубь (от 1 до {max}; пусто — все)",
  "p.forecast.show_rollup": "Показать прогноз",
  "p.forecast.problem_plan": "Выберите план.",
  "p.forecast.problem_depth":
    "Число уровней должно быть целым от 1 до {max} или пустым.",
  "p.forecast.refused_plan_gone":
    "Этого плана больше нет, или вы не можете его читать.",
  "p.forecast.could_not_read_rollup": "Не удалось прочитать сводный прогноз.",
  "p.forecast.rollup_line_one": "От {name}: прочитан {n} план, {date}.",
  "p.forecast.rollup_line_other": "От {name}: прочитано планов — {n}, {date}.",
  "p.forecast.rollup_truncated":
    "Дерево больше, чем обходит сервис; цифры охватывают его часть.",
  "p.forecast.rollup_revisits_one":
    "{n} план достигнут дважды и учтён один раз.",
  "p.forecast.rollup_revisits_other":
    "Планов, достигнутых дважды и учтённых один раз: {n}.",
  "p.forecast.rollup_no_currency_one":
    "У {n} плана нет валюты для прогноза, и он не прибавляется.",
  "p.forecast.rollup_no_currency_other":
    "Планов без валюты для прогноза, которые не прибавляются: {n}.",
  "p.forecast.rollup_empty": "Ни у одного плана здесь нет прогноза.",
  "p.value.heading": "Реализованная ценность",
  "p.value.says":
    "Читается сейчас по записанной ценности; ничего не сохраняется. Рентабельность инвестиций соотносит каждый балл ценности с записанной фактической стоимостью и не указывается, если строки бюджета используют более одной валюты, так как валюты не пересчитываются. Показатель, который нельзя указать, сообщает причину, а не ноль.",
  "p.value.roi": "Рентабельность инвестиций: {pct}",
  "p.value.roi_absent": "Рентабельность инвестиций не указана: {reason}",
  "p.value.measured_share": "измеренные доказательства: {pct} баллов ценности",
  "p.value.ttv_one":
    "Время до получения ценности: медиана {p50} календарных дней, 85-й процентиль {p85} ({n} наблюдение)",
  "p.value.ttv_other":
    "Время до получения ценности: медиана {p50} календарных дней, 85-й процентиль {p85} (наблюдений: {n})",
  "p.value.ttv_absent": "Время до получения ценности не указано: {reason}",
  "p.value.adoption":
    "Внедрение: {pct} (активных пользователей {active} из {target} за {days} дн.)",
  "p.value.adoption_definition": "активный пользователь — это: {definition}",
  "p.value.adoption_absent": "Внедрение не указано: {reason}",
  "p.value.adoption_none": "Показаний по внедрению не записано.",
  "p.value.absent_unrealized": "ценность не записана",
  "p.value.absent_no_denominator":
    "не с чем соотносить (нет фактической стоимости или цели)",
  "p.value.absent_mixed_currency":
    "строки бюджета используют более одной валюты",
  "p.value.absent_no_baseline": "нет базового бюджета",
  "p.value.absent_no_responses": "никто не ответил",
  "p.value.absent_overflow": "значение слишком велико для вычисления",
  "p.value.absent_other": "сервис не назвал причину",
  "p.value.case_heading": "Цели бизнес-кейса",
  "p.value.no_targets": "Цели бизнес-кейса не записаны.",
  "p.value.target_line":
    "{metric}: с {baseline} до {target} · {source} · утверждено {date}",
  "p.value.source_charter": "устав",
  "p.value.source_gate_review": "обзор контрольной точки",
  "p.value.source_other": "другой источник",
  "p.value.to_costs": "Записать цель на странице затрат",
  "p.value.point_heading": "Записать наблюдаемую ценность",
  "p.value.point_says":
    "Ценность измерена, оценена или заявлена. С валютой это денежная сумма; без неё — целое число в собственной единице показателя. Записанный балл ценности здесь изменить нельзя. Один балл ценности на план может быть первой измеримой ценностью: он останавливает отсчёт времени до получения ценности, запущенный первой целью бизнес-кейса, и его нельзя переместить.",
  "p.value.f_value": "Ценность",
  "p.value.f_currency": "Валюта (три буквы или пусто)",
  "p.value.f_method": "Как получена",
  "p.value.method_measured": "измерена",
  "p.value.method_estimated": "оценена",
  "p.value.method_asserted": "заявлена",
  "p.value.f_evidence": "Доказательство (ссылка или указание, необязательно)",
  "p.value.f_first": "Это первая измеримая ценность",
  "p.value.record_point": "Записать ценность",
  "p.value.confirm_first":
    "Записать это как первую измеримую ценность плана? Это остановит отсчёт времени до получения ценности, и переместить её будет нельзя.",
  "p.value.point_recorded": "Ценность записана.",
  "p.value.problem_currency":
    "Валюта должна быть трёхбуквенным кодом или пустой.",
  "p.value.problem_amount": "Ценность должна быть денежной суммой.",
  "p.value.problem_whole": "Без валюты ценность должна быть целым числом.",
  "p.value.problem_method": "Выберите, как получена ценность.",
  "p.value.refused_first_taken":
    "Отказано: у этого плана уже есть первая измеримая ценность; отсчёт останавливается один раз.",
  "p.value.refused_value": "Ценность отклонена.",
  "p.value.refused_gone":
    "Этого плана больше нет, или вы не можете его читать.",
  "p.value.could_not_read": "Не удалось прочитать реализованную ценность.",
  "p.value.could_not_record_point": "Не удалось записать ценность.",
  "p.value.case_says":
    "Цель — это показатель, обещанный в уставе или на обзоре контрольной точки, с его значением при утверждении и обещанным значением, в виде целых чисел в единице показателя. Дата утверждения запускает отсчёт времени до получения ценности и не может быть изменена, а цель здесь нельзя изменить или удалить.",
  "p.value.to_benefits": "Реализованная ценность — на странице выгод.",
  "p.value.f_metric": "Показатель",
  "p.value.f_baseline": "Значение при утверждении",
  "p.value.f_target": "Обещанное значение",
  "p.value.f_unit": "Единица (необязательно)",
  "p.value.f_source": "Обещано в",
  "p.value.record_target": "Записать цель",
  "p.value.confirm_target":
    "Записать цель {metric}? Её нельзя изменить или удалить здесь.",
  "p.value.target_recorded": "Цель {metric} записана.",
  "p.value.problem_metric": "Назовите показатель.",
  "p.value.problem_target_values":
    "Значение при утверждении и обещанное значение должны быть целыми числами.",
  "p.value.problem_source": "Выберите, где была обещана цель.",
  "p.value.refused_target": "Цель отклонена.",
  "p.value.could_not_record_target": "Не удалось записать цель.",
  "p.scoring.rank_by": "Ранжировать по",
  "p.scoring.model_smart": "Smart Score (по доказательствам)",
  "p.scoring.model_wsjf": "WSJF (сначала самая короткая взвешенная работа)",
  "p.scoring.model_rice": "RICE (охват, влияние, уверенность, трудозатраты)",
  "p.scoring.models_say":
    "Smart Score строится по доказательствам, которые хранит сервис; WSJF и RICE оцениваются по оценкам человека, задаваемым для каждого плана кнопкой «Оценки». План, у которого нет всех нужных модели входных данных, остаётся без оценки и ранжируется последним, но не получает ноль. Полоса относится к Smart Score.",
  "p.scoring.of_plans_one":
    "{length} из {scored_of} плана, ранжирование по {model}",
  "p.scoring.of_plans_other":
    "{length} из {scored_of} планов, ранжирование по {model}",
  "p.scoring.ranked_by": "Планы, ранжированные по {model}",
  "p.scoring.waiting_for": "Ожидает",
  "p.scoring.open": "Оценки",
  "p.scoring.open_named": "Открыть оценки: {name}",
  "p.scoring.heading": "Оценки: {name}",
  "p.scoring.form": "Оценки: {name}",
  "p.scoring.says":
    "WSJF и RICE вычисляются по этим оценкам. Оценка, у которой нет какого-либо входного значения, не выдаётся и никогда не угадывается, а указанный ноль не считается отсутствующим значением. Сохранение заменяет все оценки: пустое поле очищает значение.",
  "p.scoring.last_set": "Последний раз задано {when}.",
  "p.scoring.none_set": "Оценки не заданы.",
  "p.scoring.wsjf_inputs":
    "Входные данные WSJF, каждое — целое число от {min} до {max}",
  "p.scoring.rice_inputs": "Входные данные RICE",
  "p.scoring.input_business_value": "Бизнес-ценность",
  "p.scoring.input_time_criticality": "Критичность по времени",
  "p.scoring.input_risk_reduction":
    "Снижение риска или возможность новых возможностей",
  "p.scoring.input_job_size": "Размер работы",
  "p.scoring.input_reach": "Охват (люди или события за период)",
  "p.scoring.input_impact": "Влияние",
  "p.scoring.input_confidence": "Уверенность (в процентах)",
  "p.scoring.input_effort": "Трудозатраты (человеко-месяцы)",
  "p.scoring.input_other": "другое входное значение",
  "p.scoring.component_cost_of_delay": "Стоимость задержки",
  "p.scoring.impact_minimal": "минимальное",
  "p.scoring.impact_low": "низкое",
  "p.scoring.impact_medium": "среднее",
  "p.scoring.impact_high": "высокое",
  "p.scoring.impact_massive": "огромное",
  "p.scoring.impact_value": "{value} (влияние: {word})",
  "p.scoring.not_estimated": "не оценено",
  "p.scoring.save": "Сохранить оценки",
  "p.scoring.close": "Закрыть оценки",
  "p.scoring.unchanged":
    "Оценки такие же, как сохранены, поэтому ничего не отправлено.",
  "p.scoring.saved": "Оценки сохранены: {name}.",
  "p.scoring.could_not_read": "Не удалось прочитать оценки.",
  "p.scoring.could_not_save": "Не удалось сохранить оценки.",
  "p.scoring.refused_gone":
    "Этого плана больше нет, или вы не можете его читать.",
  "p.scoring.refused_inputs":
    "Оценки отклонены: входное значение вне допустимого диапазона.",
  "p.scoring.problem_wsjf":
    "{input} должно быть целым числом от {min} до {max}.",
  "p.scoring.problem_reach": "{input} должно быть целым числом не меньше нуля.",
  "p.scoring.problem_impact":
    "{input} должно быть одним из значений: 0,25, 0,5, 1, 2 или 3.",
  "p.scoring.problem_confidence":
    "{input} должно быть целым числом от 0 до 100.",
  "p.scoring.problem_effort": "{input} должно быть больше нуля.",
  "p.scoring.problem_number": "{input} должно быть числом.",
  "p.scoring.formula_wsjf":
    "WSJF = (бизнес-ценность + критичность по времени + снижение риска или возможность новых возможностей) ÷ размер работы.",
  "p.scoring.formula_rice":
    "RICE = охват × влияние × уверенность ÷ трудозатраты.",
  "p.scoring.score_is": "Оценка: {score}",
  "p.scoring.no_score": "Оценки нет: она ожидает этих входных значений.",
  "p.scoring.needs_delay_part": "оно входит в стоимость задержки",
  "p.scoring.needs_divisor":
    "оно служит делителем: без него нет ценности на единицу работы",
  "p.scoring.needs_reach":
    "оно показывает, скольких людей работа затрагивает за период",
  "p.scoring.needs_impact":
    "оно показывает, насколько работа важна для каждого из охваченных",
  "p.scoring.needs_confidence":
    "оно уменьшает оценки в зависимости от степени уверенности",
  "p.scoring.needs_other": "оно нужно для оценки",
  "p.scoring.means_business_value":
    "относительная ценность для пользователя или бизнеса, от 1 до 20",
  "p.scoring.means_time_criticality":
    "как быстро ценность теряется при опоздании, от 1 до 20",
  "p.scoring.means_risk_reduction":
    "риск, который устраняется, или возможность, которая открывается, от 1 до 20",
  "p.scoring.means_cost_of_delay":
    "сумма трёх предыдущих: во сколько обходится задержка",
  "p.scoring.means_job_size":
    "относительный размер работы, от 1 до 20; делитель",
  "p.scoring.means_reach": "люди или события, охваченные за период",
  "p.scoring.means_impact":
    "0,25 минимальное, 0,5 низкое, 1 среднее, 2 высокое, 3 огромное",
  "p.scoring.means_confidence": "насколько уверены в оценках",
  "p.scoring.means_effort": "человеко-месяцы работы; делитель",
  "p.scoring.means_other": "составляющая, которую эта страница не знает",
  "p.frontier.heading": "Выбор портфеля",
  "p.frontier.says":
    "Генератор оценивает каждый доступный вам для чтения план по его Smart Score и определяет цену по строкам бюджета, затем берёт планы по убыванию оценки на единицу стоимости, пока не исчерпается лимит. Планы, которые обязательно включить, берутся первыми, сколько бы они ни стоили. План без Smart Score исключается; план без строк бюджета стоит ноль. Выбор не всегда лучший из возможных, поэтому лимит, который не приносит больше ценности, чем более дешёвый, помечается, а не скрывается.",
  "p.frontier.must_include":
    "Планы, которые каждый выбор обязан включать (до {max})",
  "p.frontier.plans_loading": "Чтение планов…",
  "p.frontier.plans_none": "Планов, которые вам можно читать, нет.",
  "p.frontier.frontier_heading": "Эффективная граница",
  "p.frontier.frontier_says":
    "Генератор запускается один раз для каждого лимита, и ничего не сохраняется. Равномерно распределите от {min} до {max} лимитов от нуля до стоимости всех планов, для которых можно определить цену (пусто — {fallback}), либо назовите до {list} лимитов сами, каждый — сумма не меньше нуля.",
  "p.frontier.f_currency": "Валюта затрат (три буквы)",
  "p.frontier.f_how": "Какие лимиты бюджета",
  "p.frontier.how_steps": "Распределить равномерно",
  "p.frontier.how_caps": "Лимиты, которые назову я",
  "p.frontier.f_steps": "Сколько лимитов (пусто — {fallback})",
  "p.frontier.f_caps": "Лимиты бюджета, по одному в строке (до {list})",
  "p.frontier.run": "Построить границу",
  "p.frontier.value_is": "Ценность — сумма Smart Score выбранных планов.",
  "p.frontier.table": "Граница по точкам",
  "p.frontier.col_cap": "Лимит бюджета",
  "p.frontier.col_cost": "Стоимость выбора",
  "p.frontier.col_value": "Ценность",
  "p.frontier.col_included": "Выбранные планы",
  "p.frontier.col_more": "Даёт больше ценности",
  "p.frontier.more_yes": "Да",
  "p.frontier.more_no": "Нет: столько же достигается при более низком лимите",
  "p.frontier.show_choice": "Показать выбор",
  "p.frontier.show_choice_named": "Показать выбор при {cap}",
  "p.frontier.save_point": "Сохранить как черновик",
  "p.frontier.save_point_named": "Сохранить выбор при {cap} как черновик",
  "p.frontier.point_name": "Граница при {cap}",
  "p.frontier.axis_cost": "Стоимость выбранных планов",
  "p.frontier.axis_value": "Ценность (сумма Smart Score)",
  "p.frontier.chart_points_one": "{n} лимит бюджета.",
  "p.frontier.chart_points_other": "Лимитов бюджета: {n}.",
  "p.frontier.chart_best": "Наибольшая ценность, {value}, стоит {cost}.",
  "p.frontier.chart_dominated_one":
    "{n} лимит не даёт больше ценности, чем более дешёвый.",
  "p.frontier.chart_dominated_other":
    "Лимитов, не дающих больше ценности, чем более дешёвый: {n}.",
  "p.frontier.legend_efficient": "даёт больше ценности",
  "p.frontier.legend_dominated":
    "не даёт больше ценности, чем более дешёвый лимит",
  "p.frontier.no_points": "У границы нет точек.",
  "p.frontier.generate_heading": "Создать черновик",
  "p.frontier.generate_says":
    "Сохраняется как черновик сценария, который можно оценить, сравнить и зафиксировать, как любой другой. Нужно название, не более {bytes} байт. С валютой цена каждого плана определяется в ней; лимиту нужна его валюта.",
  "p.frontier.g_name": "Название черновика",
  "p.frontier.g_cap": "Лимит бюджета (необязательно)",
  "p.frontier.g_currency": "Валюта (нужна вместе с лимитом)",
  "p.frontier.generate": "Создать черновик",
  "p.frontier.draft_saved": "Черновик {name} сохранён; он в списке выше.",
  "p.frontier.choice_at": "Что выбирает лимит {cap}",
  "p.frontier.choice_of": "Что выбрал {name}",
  "p.frontier.chosen_count_one": "Выбран {n} план из {of}.",
  "p.frontier.chosen_count_other": "Выбрано планов: {n} из {of}.",
  "p.frontier.col_plan": "План в выборе",
  "p.frontier.col_chosen": "Выбран или нет",
  "p.frontier.col_score": "Его Smart Score",
  "p.frontier.col_plan_cost": "Его стоимость",
  "p.frontier.col_why": "Почему исключён",
  "p.frontier.chosen": "выбран",
  "p.frontier.left_out": "исключён",
  "p.frontier.no_score": "нет Smart Score",
  "p.frontier.plan_unknown":
    "План, который вы не можете читать или которого больше нет",
  "p.frontier.reason_no_score":
    "у него нет Smart Score, а это не то же самое, что оценка ноль",
  "p.frontier.reason_foreign_currency":
    "его строки бюджета все в другой валюте, поэтому цену определить нельзя",
  "p.frontier.reason_over_cap": "лимит исчерпался раньше него",
  "p.frontier.reason_must_include_conflict":
    "его нужно включить, но это не план, который вы можете читать",
  "p.frontier.reason_other":
    "исключён по причине, которую эта страница не знает",
  "p.frontier.problem_currency": "Укажите валюту тремя буквами.",
  "p.frontier.problem_steps":
    "Число лимитов должно быть целым от {min} до {max}.",
  "p.frontier.problem_caps_none": "Назовите хотя бы один лимит.",
  "p.frontier.problem_caps_many": "Назовите не более {list} лимитов.",
  "p.frontier.problem_caps_amount":
    "Каждый лимит должен быть суммой не меньше нуля.",
  "p.frontier.problem_must_include":
    "Выберите для включения не более {list} планов.",
  "p.frontier.problem_name": "Дайте черновику название.",
  "p.frontier.problem_name_long": "Название длиннее {bytes} байт.",
  "p.frontier.problem_cap_currency": "Лимиту нужна его валюта.",
  "p.frontier.problem_cap_amount": "Лимит должен быть суммой не меньше нуля.",
  "p.frontier.refused_frontier": "Граница отклонена.",
  "p.frontier.refused_generate": "Черновик отклонён.",
  "p.frontier.could_not_read": "Не удалось построить границу.",
  "p.frontier.could_not_generate": "Не удалось создать черновик.",
  "p.frontier.rollback": "Откатить",
  "p.frontier.rollback_named": "Откатить {name}",
  "p.frontier.rollback_says":
    "Зафиксированный сценарий можно откатить до черновика после подтверждения; меняется только сценарий.",
  "p.frontier.confirm_rollback":
    "Откатить {name} до черновика? Он перестанет быть зафиксированным портфелем; фиксация сохранится в журнале аудита.",
  "p.frontier.rolled_back": "{name} снова черновик.",
  "p.frontier.refused_rollback_status":
    "{name} больше не зафиксирован, поэтому откатывать нечего. Список перечитан.",
  "p.frontier.refused_rollback_gone":
    "{name} больше нет, или вы не можете его читать. Список перечитан.",
  "p.frontier.could_not_roll_back": "Не удалось откатить сценарий.",
  "p.frontier.status_draft": "черновик",
  "p.frontier.status_committed": "зафиксирован",
  "p.frontier.violations_one": "{n} ограничение нарушено",
  "p.frontier.violations_other": "Нарушено ограничений: {n}",
  "p.frontier.compare_note":
    "B минус A, по одной валюте; валюты никогда не складываются.",
  "p.intake.edit": "Изменить",
  "p.intake.edit_named": "Изменить черновик {title}",
  "p.intake.edit_heading": "Изменение: {title}",
  "p.intake.edit_heading_loading": "Чтение черновика…",
  "p.intake.edit_form": "Поля черновика",
  "p.intake.edit_says":
    "Изменять можно только черновик; там же предложение помечается как внешняя просьба. Нужно название; название и краткое описание — не более {bytes} байт каждое; сумме нужна валюта; указанный компромисс — от {min} до {max} символов. Сохранение заменяет все поля.",
  "p.intake.save": "Сохранить черновик",
  "p.intake.saved": "{title} сохранено.",
  "p.intake.unchanged": "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.intake.could_not_read_draft": "Не удалось прочитать черновик.",
  "p.intake.problem_not_draft": "Изменять можно только черновик.",
  "p.intake.problem_title_long": "Название длиннее {bytes} байт.",
  "p.intake.problem_summary_long": "Краткое описание длиннее {bytes} байт.",
  "p.intake.refused_not_draft":
    "Изменение отклонено: предложение больше не черновик. Список перечитан.",
  "p.intake.refused_edit":
    "Изменение отклонено: поле вне того, что принимает сервис.",
  "p.intake.forecast_heading": "Сколько будет утверждено",
  "p.intake.forecast_says":
    "Строится моделированием по тому, сколько предложений было утверждено в каждом прошлом периоде; поступающие предложения на него не влияют. Для прогноза нужна история не менее чем за {needed} периодов.",
  "p.intake.f_periods": "Периодов вперёд (от 1 до {max}; пусто — {fallback})",
  "p.intake.f_period_days":
    "Дней в периоде (от 1 до {max}; пусто — {fallback})",
  "p.intake.f_history": "Периодов истории (от 1 до {max}; пусто — {fallback})",
  "p.intake.forecast_run": "Спрогнозировать",
  "p.intake.problem_periods":
    "Число периодов вперёд должно быть целым от 1 до {periods}.",
  "p.intake.problem_period_days":
    "Период должен быть целым числом дней от 1 до {days}.",
  "p.intake.problem_history":
    "История должна быть целым числом периодов от 1 до {history}.",
  "p.intake.refused_forecast": "Прогноз отклонён.",
  "p.intake.could_not_forecast": "Не удалось построить прогноз.",
  "p.intake.window": "История с {from} по {to}, периодами по {days} дн.",
  "p.intake.at_least_label_one": "В следующий {n} период ({days} дн.) не менее",
  "p.intake.at_least_label_other":
    "В следующие {n} периодов ({days} дн.) не менее",
  "p.intake.at_least_means":
    "с уверенностью 85%: цифра, на которую стоит рассчитывать",
  "p.intake.median_label": "Обычно",
  "p.intake.median_means":
    "в половине испытаний утверждается больше, в половине меньше: орёл или решка, а не обязательство",
  "p.intake.at_most_label": "Не более",
  "p.intake.at_most_means":
    "оптимистичный край: в 85% испытаний утверждается не больше",
  "p.intake.proposals_n_one": "{n} предложение",
  "p.intake.proposals_n_other": "Предложений: {n}",
  "p.intake.absent_history":
    "Прогноза нет: нужна история не менее чем за {needed} периодов, а есть {n}.",
  "p.intake.absent_other": "По этой истории построить прогноз не удалось.",
  "p.intake.approvals_history":
    "Утверждено за период, сначала самые давние: {list}",
  "p.intake.arrivals_history":
    "Поступило предложений за период, сначала самые давние (показано рядом; на прогноз не влияют): {list}",
  "p.planner.heading": "Запланированное время",
  "p.planner.says":
    "Зарезервируйте время на день для одной из ваших открытых задач. Интервал принадлежит только вам и никогда не меняет срок задачи; он заканчивается после начала и длится не более 12 часов. Интервалы могут пересекаться: пересечение показывается как конфликт, а интервал, который заканчивается, когда начинается следующий, не конфликт. Время указано в вашем часовом поясе, {tz}.",
  "p.planner.day_nav": "Выбор дня",
  "p.planner.previous_day": "Предыдущий день",
  "p.planner.back_to_today": "Вернуться к сегодняшнему дню",
  "p.planner.next_day": "Следующий день",
  "p.planner.f_day": "День",
  "p.planner.show_day": "Показать день",
  "p.planner.problem_day": "Укажите день в виде даты.",
  "p.planner.day_heading": "Запланировано на {day}",
  "p.planner.day_hours": "В этом дне {hours} ч из-за перевода часов.",
  "p.planner.nothing_planned": "На этот день ничего не запланировано.",
  "p.planner.clashes_one": "Конфликтует с {n} другим интервалом",
  "p.planner.clashes_other": "Конфликтует с другими интервалами: {n}",
  "p.planner.remove": "Удалить",
  "p.planner.remove_named": "Удалить интервал для {title}, {time}",
  "p.planner.confirm_remove":
    "Удалить интервал для {title}, {time}? Сама задача не меняется.",
  "p.planner.removed": "Интервал для {title}, {time}, удалён.",
  "p.planner.omitted_one":
    "{n} интервал по задаче, которую вам больше нельзя читать, не показан.",
  "p.planner.omitted_other":
    "Интервалы по задачам, которые вам больше нельзя читать, не показаны: {n}.",
  "p.planner.due_heading": "Срок в этот день",
  "p.planner.nothing_due": "В этот день нет сроков по назначенным вам задачам.",
  "p.planner.loading": "Чтение дня…",
  "p.planner.form": "Запланировать время для задачи",
  "p.planner.form_says":
    "Интервал приходится на {day} в вашем часовом поясе. Интервал, который переходит за полночь, планируется как два.",
  "p.planner.f_task": "Задача",
  "p.planner.choose_task": "Выберите одну из ваших открытых задач",
  "p.planner.loading_tasks": "Чтение ваших задач…",
  "p.planner.task_option": "{title}, в плане {plan}",
  "p.planner.no_tasks":
    "Вам ничего не назначено, поэтому нет задачи, для которой можно планировать время.",
  "p.planner.f_start": "Начало",
  "p.planner.f_end": "Окончание",
  "p.planner.plan_time": "Запланировать это время",
  "p.planner.planned": "Запланировано: {title}, {time}.",
  "p.planner.problem_task": "Выберите задачу.",
  "p.planner.problem_start": "Укажите время начала.",
  "p.planner.problem_end": "Укажите время окончания.",
  "p.planner.problem_start_skipped":
    "В этот день часы пропускают время начала; выберите другое.",
  "p.planner.problem_end_skipped":
    "В этот день часы пропускают время окончания; выберите другое.",
  "p.planner.problem_order": "Интервал должен заканчиваться после начала.",
  "p.planner.problem_too_long": "Интервал длится не более 12 часов.",
  "p.planner.refused_zone":
    "Часовой пояс вашего браузера не принят, поэтому показать день нельзя.",
  "p.planner.refused_slot":
    "Интервал не принят: он должен заканчиваться после начала и длиться не более 12 часов, а у человека не более {max} интервалов.",
  "p.planner.refused_task":
    "Этой задачи больше нет, или она в плане, который вам нельзя читать.",
  "p.planner.refused_gone": "Этого интервала уже нет. День перечитан.",
  "p.planner.could_not_load": "Не удалось прочитать день.",
  "p.planner.could_not_plan": "Не удалось запланировать время.",
  "p.planner.could_not_remove": "Не удалось удалить интервал.",
  "p.feed.heading": "Лента календаря",
  "p.feed.says":
    "Закрытый адрес, на который можно подписать приложение календаря: открытые задачи, назначенные вам и имеющие срок, вехи и спринты их планов, а также запланированное вами время. Второй адрес — лента ваших уведомлений для программы чтения лент. Любой, у кого есть адрес, может прочитать то, что он показывает, поэтому держите его при себе.",
  "p.feed.rules":
    "Адрес показывается один раз, здесь, и эта страница его не хранит; он действует 365 дней. Создание нового сразу прекращает действие любого прежнего.",
  "p.feed.issue": "Создать адрес ленты",
  "p.feed.confirm_issue":
    "Создать новый адрес ленты? Любой созданный ранее адрес сразу перестанет работать.",
  "p.feed.issued":
    "Ваши новые адреса ниже. Скопируйте их сейчас: повторно они не показываются.",
  "p.feed.calendar_address": "Адрес календаря (iCalendar)",
  "p.feed.atom_address": "Адрес ленты уведомлений (Atom)",
  "p.feed.revoke": "Прекратить действие всех адресов ленты",
  "p.feed.confirm_revoke":
    "Прекратить действие всех созданных вами адресов ленты? Приложение календаря, которое использует один из них, сразу перестанет обновляться.",
  "p.feed.revoked": "Все созданные вами адреса ленты перестали работать.",
  "p.feed.no_address":
    "В ответе не было адреса, который эта страница может показать, поэтому ничего не показано. Любой прежний адрес прекратил действие; при необходимости создайте другой.",
  "p.feed.refused_forbidden": "Вам нельзя управлять адресом ленты.",
  "p.feed.could_not_issue": "Не удалось создать адрес ленты.",
  "p.feed.could_not_revoke": "Не удалось прекратить действие адресов ленты.",
  "p.watching.heading": "За чем вы следите",
  "p.watching.says":
    "Вам сообщают, когда меняется то, за чем вы следите. За задачей вы следите, когда создаёте её, комментируете или получаете её, если вы не отключили это в настройках уведомлений. Кто следит, другим не сообщается. План показывается по названию; всё остальное — по виду и началу ссылки.",
  "p.watching.count_one": "Вы следите за {n} объектом.",
  "p.watching.count_other": "Объектов, за которыми вы следите: {n}.",
  "p.watching.capped": "Показано только {max} новейших.",
  "p.watching.none": "Сейчас вы ни за чем не следите.",
  "p.watching.named_plan": "План {name}",
  "p.watching.unnamed": "{kind}, ссылка {ref}",
  "p.watching.kind_plan": "План",
  "p.watching.kind_task": "Задача",
  "p.watching.kind_risk": "Риск",
  "p.watching.kind_proposal": "Предложение",
  "p.watching.kind_idea": "Идея",
  "p.watching.kind_canvas_item": "Элемент холста",
  "p.watching.kind_budget_line": "Строка бюджета",
  "p.watching.kind_scenario": "Сценарий",
  "p.watching.kind_document_version": "Версия документа",
  "p.watching.kind_other": "Что-то другое",
  "p.watching.stop": "Перестать следить",
  "p.watching.stop_named": "Перестать следить: {what}",
  "p.watching.confirm_stop":
    "Перестать следить за {what}? Вам больше не будут сообщать о его изменениях.",
  "p.watching.stopped": "Вы больше не следите за {what}.",
  "p.watching.refused": "Прекращение слежения не принято.",
  "p.watching.loading": "Чтение того, за чем вы следите…",
  "p.watching.could_not_load": "Не удалось прочитать, за чем вы следите.",
  "p.watching.could_not_stop": "Не удалось перестать следить.",
  "p.workprofile.heading": "Работа во времени",
  "p.workprofile.says":
    "Работа задач во всех планах, которые вам можно читать, распределённая по рабочим дням каждой задачи согласно её профилю, в целых минутах. Работа, которую нельзя разместить (задача, ещё не включённая в расписание, или без рабочего дня), считается отдельно и никогда не отбрасывается.",
  "p.workprofile.rules":
    "Недели идут с понедельника по воскресенье. Оставьте даты пустыми, чтобы охватить всю размещённую работу. Окно охватывает не более десяти лет и {max} периодов.",
  "p.workprofile.form": "Показать работу во времени",
  "p.workprofile.f_period": "Периоды",
  "p.workprofile.period_week": "Недели",
  "p.workprofile.period_day": "Дни",
  "p.workprofile.period_month": "Месяцы",
  "p.workprofile.f_from": "С (пусто — первый день с размещённой работой)",
  "p.workprofile.f_to": "По (пусто — последний день с размещённой работой)",
  "p.workprofile.show": "Показать работу",
  "p.workprofile.problem_from": "Укажите первый день в виде даты.",
  "p.workprofile.problem_to": "Укажите последний день в виде даты.",
  "p.workprofile.problem_order": "Первый день не должен быть позже последнего.",
  "p.workprofile.problem_too_long": "Окно охватывает не более десяти лет.",
  "p.workprofile.problem_periods":
    "В этом окне больше {max} периодов; выберите более длинные периоды или более короткое окно.",
  "p.workprofile.refused":
    "Это окно не принято: если работа ещё не размещена, укажите обе даты и уложитесь в число периодов, названное выше.",
  "p.workprofile.none":
    "Ни у одного плана, который вам можно читать, нет задачи с работой.",
  "p.workprofile.summary":
    "{range}: работы {work}, из них не размещено {unplaced}.",
  "p.workprofile.plans_table": "Работа по планам",
  "p.workprofile.periods_table": "Работа по периодам",
  "p.workprofile.col_plan": "План",
  "p.workprofile.col_work": "Работа",
  "p.workprofile.col_unplaced": "Не размещено",
  "p.workprofile.col_period": "Период",
  "p.workprofile.plan_unnamed": "План без названия",
  "p.workprofile.could_not_load": "Не удалось прочитать работу во времени.",
  "p.timeentry.heading": "Учёт времени",
  "p.timeentry.says":
    "Время записывается на вас в целых минутах, от 1 минуты до {max} за одну дату, в плане и, при желании, в одной из его задач. В утверждённую неделю время больше не принимается. Записанную запись нельзя изменить или удалить.",
  "p.timeentry.open": "Записать время",
  "p.timeentry.form": "Записать время в плане",
  "p.timeentry.f_plan": "План",
  "p.timeentry.choose_plan": "Выберите план",
  "p.timeentry.loading": "Чтение планов…",
  "p.timeentry.f_task": "Задача (необязательно)",
  "p.timeentry.no_task": "Без задачи: сам план",
  "p.timeentry.task_unnamed": "Задача, которой больше нет в этом плане",
  "p.timeentry.f_date": "Дата",
  "p.timeentry.f_duration": "Затраченное время",
  "p.timeentry.f_hours": "Часы",
  "p.timeentry.f_minutes": "Минуты",
  "p.timeentry.typed": "Это {time}.",
  "p.timeentry.f_category": "Категория",
  "p.timeentry.category_unclassified": "Не классифицировано",
  "p.timeentry.category_capex": "Капитальные расходы (capex)",
  "p.timeentry.category_opex": "Операционные расходы (opex)",
  "p.timeentry.f_billable": "Оплачиваемое",
  "p.timeentry.f_note": "Примечание (необязательно)",
  "p.timeentry.record": "Записать это время",
  "p.timeentry.confirm":
    "Записать {time} за {day} в плане {plan}? Записанную запись нельзя изменить или удалить.",
  "p.timeentry.recorded": "Записано {time} за {day} в плане {plan}.",
  "p.timeentry.problem_plan": "Выберите план.",
  "p.timeentry.problem_date": "Укажите дату, когда было затрачено время.",
  "p.timeentry.problem_locked":
    "Эта неделя утверждена и заблокирована, поэтому время в неё больше не принимается.",
  "p.timeentry.problem_duration":
    "Укажите затраченное время целыми часами (от 0 до 24) и минутами (от 0 до 59), всего от 1 минуты до {max} минут.",
  "p.timeentry.refused_locked":
    "Эта неделя утверждена и заблокирована, поэтому время в неё больше не принимается. Ничего не записано.",
  "p.timeentry.refused_invalid":
    "Запись не принята: её время или задача не подходят этому плану. Ничего не записано.",
  "p.timeentry.refused_plan":
    "Этого плана больше нет, или вы не можете его читать.",
  "p.timeentry.refused_forbidden": "Вам нельзя записывать время в этом плане.",
  "p.timeentry.could_not_load_plans": "Не удалось прочитать планы.",
  "p.timeentry.could_not_load_plan":
    "Не удалось прочитать задачи и время этого плана.",
  "p.timeentry.could_not_record": "Не удалось записать время.",
  "p.timeentry.yours_heading": "Ваше время в плане {plan}",
  "p.timeentry.yours_none": "В этом плане вы не записали времени.",
  "p.timeentry.yours_table":
    "Ваши записи времени в плане {plan}, сначала новые",
  "p.timeentry.col_date": "Дата",
  "p.timeentry.col_time": "Время",
  "p.timeentry.col_task": "Задача",
  "p.timeentry.col_category": "Категория",
  "p.timeentry.col_billable": "Оплачиваемое",
  "p.timeentry.col_note": "Примечание",
  "p.timeentry.billable_yes": "Оплачиваемое",
  "p.timeentry.billable_no": "Неоплачиваемое",
  "p.timereport.heading": "Отчёт о времени",
  "p.timereport.says":
    "Записанное время в планах, которые вам можно читать, сгруппированное по вашему выбору; оплачиваемое и неоплачиваемое всегда показаны отдельно. Оставьте даты пустыми, чтобы получить последние 90 дней до сегодняшнего. Человек задаётся ссылкой person: или worker:; оставьте пустым для всех.",
  "p.timereport.form": "Запросить отчёт о времени",
  "p.timereport.f_group_by": "Группировать по",
  "p.timereport.by_plan": "План",
  "p.timereport.by_person": "Человек",
  "p.timereport.by_category": "Категория",
  "p.timereport.by_billable": "Оплачиваемое или нет",
  "p.timereport.by_month": "Месяц",
  "p.timereport.f_from": "С",
  "p.timereport.f_to": "По",
  "p.timereport.f_plan": "План",
  "p.timereport.every_plan": "Все планы, которые вам можно читать",
  "p.timereport.loading_plans": "Чтение планов…",
  "p.timereport.f_person": "Человек (ссылка)",
  "p.timereport.show": "Показать отчёт о времени",
  "p.timereport.problem_from": "Укажите первый день в виде даты.",
  "p.timereport.problem_to": "Укажите последний день в виде даты.",
  "p.timereport.problem_order": "Первый день не должен быть позже последнего.",
  "p.timereport.problem_person":
    "Человек задаётся как person: или worker: и идентификатор, например person:6f1c2a3b-0000-4000-8000-000000000009.",
  "p.timereport.refused": "Отчёт о времени для этих параметров не принят.",
  "p.timereport.window": "Записанное время: {range}.",
  "p.timereport.no_entries": "В этих планах за этот период время не записано.",
  "p.timereport.table": "Время по группам",
  "p.timereport.col_entries": "Записей",
  "p.timereport.col_billable": "Оплачиваемое",
  "p.timereport.col_not_billable": "Неоплачиваемое",
  "p.timereport.col_total": "Итого",
  "p.timereport.total":
    "Всего: оплачиваемое {billable}, неоплачиваемое {not_billable}, вместе {total}.",
  "p.timereport.omitted_one":
    "{n} запись в плане, который вы не можете читать, пропущена.",
  "p.timereport.omitted_other":
    "Записей в планах, которые вы не можете читать, пропущено: {n}.",
  "p.timereport.truncated":
    "Прочитаны только первые {max} записей, поэтому отчёт неполный.",
  "p.timereport.csv": "Скачать этот отчёт о времени в формате CSV",
  "p.timereport.billable": "Оплачиваемое",
  "p.timereport.not_billable": "Неоплачиваемое",
  "p.timereport.plan_unnamed": "План без названия здесь ({ref})",
  "p.timereport.could_not_load": "Не удалось прочитать отчёт о времени.",
  "p.timereport.could_not_load_plans": "Не удалось прочитать планы для выбора.",
  "p.hammock.heading": "Гамаки: суммарные задачи, охватывающие другие",
  "p.hammock.says":
    "Гамак — это суммарная задача, даты которой берутся из связанных с ней задач этого плана, от самого раннего начала до самого позднего окончания. Он не входит в сеть зависимостей, поэтому никогда не меняет резерв времени ни одной задачи и окончание. Задача становится гамаком, только если у неё нет длительности, работы, ограничения и зависимости, а плановое начало очищено; превращение обратно в задачу удаляет все её связи. Гамак связывается с задачей или другим гамаком этого плана, но не с самим собой, не дважды и не по кругу.",
  "p.hammock.none": "В этом плане нет гамаков.",
  "p.hammock.spans": "Охватывает: {range}.",
  "p.hammock.no_links": "Дат пока нет: он не связан ни с одной задачей.",
  "p.hammock.members_unscheduled":
    "Дат пока нет: ни у одной связанной с ним задачи нет дат.",
  "p.hammock.untitled": "Задача без названия",
  "p.hammock.member_hammock": "{title} (гамак)",
  "p.hammock.unmake_named": "Сделать {title} снова задачей",
  "p.hammock.unlink": "Удалить связь",
  "p.hammock.unlink_named": "Удалить связь гамака {hammock} с {title}",
  "p.hammock.make_form": "Сделать задачу гамаком",
  "p.hammock.f_task": "Задача",
  "p.hammock.choose_task": "Выберите задачу",
  "p.hammock.make": "Сделать гамаком",
  "p.hammock.link_form": "Связать гамак с задачей",
  "p.hammock.f_hammock": "Гамак",
  "p.hammock.choose_hammock": "Выберите гамак",
  "p.hammock.f_member": "Задача, которую он охватывает",
  "p.hammock.choose_member": "Выберите задачу для охвата",
  "p.hammock.link": "Добавить связь",
  "p.hammock.confirm_make":
    "Сделать {title} гамаком? Его плановое начало будет очищено, а даты начнут браться из связанных с ним задач.",
  "p.hammock.confirm_unmake_one":
    "Сделать {title} снова задачей? Его {n} связь удаляется.",
  "p.hammock.confirm_unmake_other":
    "Сделать {title} снова задачей? Удаляется связей: {n}.",
  "p.hammock.confirm_unlink":
    "Удалить связь гамака {hammock} с {title}? Сама задача не меняется.",
  "p.hammock.made": "{title} теперь гамак.",
  "p.hammock.unmade": "{title} снова задача.",
  "p.hammock.linked": "Гамак {hammock} теперь охватывает {title}.",
  "p.hammock.unlinked": "Гамак {hammock} больше не охватывает {title}.",
  "p.hammock.problem_choose_task":
    "Выберите задачу, которую нужно сделать гамаком.",
  "p.hammock.problem_carries_work":
    "У этой задачи есть длительность, работа или ограничение: сначала очистите их в расписании.",
  "p.hammock.problem_has_dependencies":
    "У этой задачи есть зависимости: сначала удалите их, так как у гамака их нет.",
  "p.hammock.problem_choose_hammock": "Выберите гамак.",
  "p.hammock.problem_choose_member":
    "Выберите задачу, которую должен охватывать гамак.",
  "p.hammock.problem_self": "Гамак не может охватывать самого себя.",
  "p.hammock.problem_linked": "Этот гамак уже охватывает эту задачу.",
  "p.hammock.problem_loop":
    "Тот гамак уже охватывает этот, поэтому связь создала бы цикл.",
  "p.hammock.refused_make":
    "Сервис не сделал эту задачу гамаком: у неё всё ещё есть длительность, работа, ограничение или зависимость.",
  "p.hammock.refused_link":
    "Сервис не добавил связь: задачи могло не оказаться в этом плане, она могла быть уже охвачена или входить в цикл.",
  "p.hammock.refused_gone":
    "Задачи или гамака больше нет, или вы больше не можете их читать.",
  "p.hammock.refused_link_gone": "Этой связи уже нет.",
  "p.hammock.could_not_load": "Не удалось прочитать гамаки.",
  "p.hammock.could_not_save": "Не удалось изменить гамак.",
  "p.crossplan.heading": "Задачи связанных планов",
  "p.crossplan.says":
    "Это задачи других планов, которые вам можно читать и с которыми связаны задачи этого плана; они планируются по календарю их собственного плана. Штриховая полоса — задача вне критического пути; сплошная полоса и ромб отмечают задачу на нём.",
  "p.crossplan.network_finish":
    "С учётом связанных планов сеть заканчивается {date}.",
  "p.crossplan.truncated":
    "Показаны не все связанные планы: сеть достигла предела по числу планов или задач.",
  "p.crossplan.links_heading": "Связи, ведущие в другой план",
  "p.crossplan.lag": "запаздывание {days} рабочих дней",
  "p.crossplan.timeline": "Задачи плана {plan} на временной шкале",
  "p.crossplan.table": "Даты задач связанных планов",
  "p.crossplan.col_task": "Задача",
  "p.crossplan.col_plan": "План",
  "p.crossplan.col_start": "Раннее начало",
  "p.crossplan.col_finish": "Раннее окончание",
  "p.crossplan.col_critical": "На критическом пути",
  "p.crossplan.yes": "Да",
  "p.crossplan.no": "Нет",
  "p.crossplan.untitled": "Задача без названия",
  "p.crossplan.kind_fs": "окончание — начало",
  "p.crossplan.kind_ss": "начало — начало",
  "p.crossplan.kind_ff": "окончание — окончание",
  "p.crossplan.kind_sf": "начало — окончание",
  "p.crossplan.task_heading": "Зависимости в этом плане или другом",
  "p.crossplan.task_says":
    "Зависимость связывает эту задачу с задачей другого плана, который вам можно читать. Её вид — окончание — начало, если не выбран другой; запаздывание — целое число рабочих дней от минус {max} до {max}, отрицательное запаздывание — опережение. Зависимость не создаётся дважды одним и тем же способом, не по кругу между планами и не с гамаком. Зависимость, другой конец которой вам нельзя читать, учитывается в числе, но не называется и исключается из расписания.",
  "p.crossplan.no_links": "У этой задачи пока нет зависимостей.",
  "p.crossplan.this_before": "Эта задача предшествует: {other}",
  "p.crossplan.this_after": "Эта задача следует за: {other}",
  "p.crossplan.end_own": "{title}, в этом плане",
  "p.crossplan.end_external": "{title}, в плане {plan}",
  "p.crossplan.end_unnamed": "задача, не названная здесь ({ref})",
  "p.crossplan.remove": "Удалить",
  "p.crossplan.remove_named": "Удалить зависимость от {other}",
  "p.crossplan.restricted_one":
    "Ещё {n} зависимость связывает задачу, которую вам нельзя читать.",
  "p.crossplan.restricted_other":
    "Ещё зависимостей, связывающих задачи, которые вам нельзя читать: {n}.",
  "p.crossplan.see_network": "Открыть расписание по всем планам",
  "p.crossplan.form": "Добавить зависимость от задачи другого плана",
  "p.crossplan.f_order": "Порядок",
  "p.crossplan.order_before": "Эта задача идёт первой",
  "p.crossplan.order_after": "Другая задача идёт первой",
  "p.crossplan.f_plan": "Другой план",
  "p.crossplan.choose_plan": "Выберите план",
  "p.crossplan.f_task": "Задача в том плане",
  "p.crossplan.choose_task": "Выберите задачу",
  "p.crossplan.f_kind": "Как связаны две задачи",
  "p.crossplan.f_lag": "Запаздывание в рабочих днях (если пусто — 0)",
  "p.crossplan.add": "Добавить зависимость",
  "p.crossplan.added": "Добавлена зависимость от {title}.",
  "p.crossplan.removed": "Зависимость удалена.",
  "p.crossplan.confirm_remove_before":
    "Удалить зависимость: {task} предшествует {other}? Ни одна из задач не меняется.",
  "p.crossplan.confirm_remove_after":
    "Удалить зависимость: {task} следует за {other}? Ни одна из задач не меняется.",
  "p.crossplan.problem_plan": "Выберите другой план.",
  "p.crossplan.problem_task": "Выберите задачу в том плане.",
  "p.crossplan.problem_lag":
    "Запаздывание — целое число рабочих дней от минус {max} до {max}.",
  "p.crossplan.problem_self": "Задача не может зависеть от самой себя.",
  "p.crossplan.problem_exists": "Эта зависимость уже есть.",
  "p.crossplan.refused_unreadable":
    "Одной из двух задач больше нет, или вы не можете её читать.",
  "p.crossplan.refused_exists": "Эта зависимость уже есть.",
  "p.crossplan.refused_add":
    "Сервис не добавил зависимость: она, возможно, уже есть, образует цикл или связывает гамак.",
  "p.crossplan.refused_gone":
    "Этой зависимости уже нет, или вы больше не можете читать обе задачи.",
  "p.crossplan.could_not_load": "Не удалось прочитать зависимости этой задачи.",
  "p.crossplan.could_not_load_plans": "Не удалось прочитать планы для выбора.",
  "p.crossplan.could_not_load_tasks":
    "Не удалось прочитать задачи этого плана.",
  "p.crossplan.could_not_save": "Не удалось изменить зависимость.",
  "p.taskrollup.heading": "Сводка по подзадачам",
  "p.taskrollup.says":
    "Вычисляется каждый раз по подзадачам этой задачи и никогда не хранится: самый поздний срок среди них, их баллы в сумме, прогресс, взвешенный только по оценённым, и записанное по ним время.",
  "p.taskrollup.subtasks": "Подзадачи",
  "p.taskrollup.done_of_one": "Выполнено {done} из {n}",
  "p.taskrollup.done_of_other": "Выполнено {done} из {n}",
  "p.taskrollup.due": "Самый поздний срок",
  "p.taskrollup.points": "Сумма баллов",
  "p.taskrollup.progress": "Прогресс",
  "p.taskrollup.spent": "Записанное время",
  "p.taskrollup.no_children": "У этой задачи нет подзадач.",
  "p.taskrollup.no_dated_children": "Ни у одной из её подзадач нет срока.",
  "p.taskrollup.no_estimates": "Ни одна из её подзадач не оценена.",
  "p.taskrollup.absent": "Неизвестно.",
  "p.taskrollup.unestimated_one":
    "У {n} подзадачи нет оценки, поэтому она не входит в прогресс (не считается нулём):",
  "p.taskrollup.unestimated_other":
    "У подзадач без оценки ({n}) нет оценки, поэтому они не входят в прогресс (не считаются нулём):",
  "p.taskrollup.unnamed": "Подзадача, не названная здесь ({ref})",
  "p.taskrollup.refused_gone":
    "Этой задачи больше нет, или вы больше не можете её читать.",
  "p.taskrollup.could_not_load": "Не удалось прочитать сводку по подзадачам.",
  "p.readings.heading": "Подробнее об этом плане",
  "p.readings.says": "Каждое из этого читается только по вашему запросу.",
  "p.readings.read": "Прочитать",
  "p.readings.could_not_read": "Не удалось прочитать это.",
  "p.readings.not_measured": "не измерено",
  "p.readings.days": "{n} дн.",
  "p.readings.rollup_heading": "Поток по вложенным планам",
  "p.readings.rollup_says":
    "Задачи этого плана и всех планов под ним, посчитанные вместе, а не усреднённые; если поле пусто — до {max} уровней вниз.",
  "p.readings.f_depth": "Уровней вниз",
  "p.readings.rollup_plans_one": "{n} план.",
  "p.readings.rollup_plans_other": "Планов: {n}.",
  "p.readings.rollup_tasks_one":
    "{n} задача: завершено {finished}, в работе {wip}, не начато {not_started}.",
  "p.readings.rollup_tasks_other":
    "Задач: {n}: завершено {finished}, в работе {wip}, не начато {not_started}.",
  "p.readings.rollup_times":
    "Большая часть работы (85 из 100) завершалась в течение {cycle} после начала и {lead} после запроса; доля времени, затраченная на работу, а не на ожидание: {efficiency}.",
  "p.readings.rollup_truncated":
    "Учтены не все планы ниже: обход достиг предела.",
  "p.readings.rollup_revisits_one":
    "{n} план достигнут дважды и учтён один раз.",
  "p.readings.rollup_revisits_other":
    "Планов, достигнутых дважды и учтённых один раз: {n}.",
  "p.readings.rollup_table": "Поток по планам",
  "p.readings.col_plan": "План",
  "p.readings.col_depth": "Уровень",
  "p.readings.col_tasks": "Задачи",
  "p.readings.col_finished": "Завершено",
  "p.readings.col_wip": "В работе",
  "p.readings.col_efficiency": "Время работы, а не ожидания",
  "p.readings.effort_heading": "Записанное время",
  "p.readings.effort_says":
    "Время, записанное людьми по этому плану так, как они его указали; ничто его не проверяет.",
  "p.readings.read_effort": "Прочитать записанное время",
  "p.readings.effort_none": "По этому плану время не записано.",
  "p.readings.effort_total":
    "Записано {total}, из них оплачиваемое {billable}.",
  "p.readings.effort_split":
    "Капитальные расходы {capex}, операционные расходы {opex}, не классифицировано {unclassified}.",
  "p.readings.effort_by_person": "Время по людям",
  "p.readings.effort_by_task": "Время по задачам",
  "p.readings.col_person": "Человек",
  "p.readings.col_time": "Время",
  "p.readings.col_billable": "Оплачиваемое",
  "p.readings.col_task": "Задача",
  "p.readings.task_unnamed": "Задача, не названная здесь ({ref})",
  "p.readings.forecast_heading": "Прогноз поставки",
  "p.readings.forecast_says":
    "Десять тысяч прогонов, построенных по тому, сколько этот план завершал в каждом прошлом периоде: сколько может занять открытая работа и сколько может быть сделано за несколько периодов. Пустое поле означает значение по умолчанию; нужна история не менее чем за шесть периодов.",
  "p.readings.f_items": "Сколько элементов завершить (если пусто — открытые)",
  "p.readings.f_periods": "Периодов вперёд (если пусто — {n})",
  "p.readings.f_period_days": "Дней в периоде (если пусто — {n})",
  "p.readings.f_history": "Периодов истории (если пусто — {n})",
  "p.readings.read_forecast": "Прочитать прогноз",
  "p.readings.forecast_history_one": "Построено по {n} периоду в {days} дн.",
  "p.readings.forecast_history_other":
    "Построено по периодам ({n}) по {days} дн.",
  "p.readings.forecast_batch_one":
    "{n} элемент: половина прогонов завершается за {p50} дн., 85 из 100 — за {p85} дн. и 95 из 100 — за {p95} дн.",
  "p.readings.forecast_batch_other":
    "Элементов: {n}: половина прогонов завершается за {p50} дн., 85 из 100 — за {p85} дн. и 95 из 100 — за {p95} дн.",
  "p.readings.forecast_no_batch":
    "Сколько может занять работа, спрогнозировать нельзя: истории слишком мало или в ней ничего не завершено.",
  "p.readings.forecast_horizon":
    "В следующие {days} дн. может быть сделано около {median} элементов: не менее {least} в 85 прогонах из 100 и не более {most}.",
  "p.readings.forecast_no_horizon":
    "Сколько может быть сделано, спрогнозировать нельзя: истории слишком мало или в ней ничего не завершено.",
  "p.readings.performance_heading":
    "Расписание, стоимость и заинтересованные стороны",
  "p.readings.performance_says":
    "Показатель исполнения расписания — сделанная работа относительно работы, запланированной на сегодня, а показатель исполнения стоимости — сделанная работа относительно того, во сколько она обошлась; значение выше 100% лучше плана. Оценка заинтересованных сторон лежит от минус 100 до 100.",
  "p.readings.read_performance": "Прочитать показатели исполнения",
  "p.readings.schedule": "Исполнение расписания",
  "p.readings.cost": "Исполнение стоимости",
  "p.readings.stakeholders": "Оценка заинтересованных сторон",
  "p.readings.nps_one":
    "{score}, по {n} ответу ({promoters} сторонников, {detractors} критиков)",
  "p.readings.nps_other":
    "{score}, по ответам ({n}): {promoters} сторонников, {detractors} критиков",
  "p.readings.nps_none": "Ответов пока нет.",
  "p.readings.earned_value_link": "Открыть освоенный объём полностью",
  "p.readings.absent_no_baseline": "Не измерено: у плана нет базового бюджета.",
  "p.readings.absent_no_weighted_tasks":
    "Не измерено: ни у одной задачи нет веса.",
  "p.readings.absent_not_yet_defined":
    "Пока не измерено: на сегодня ничего не запланировано и не потрачено.",
  "p.readings.absent_several_currencies":
    "Не одна цифра: у плана бюджеты в нескольких валютах; см. освоенный объём.",
  "p.readings.absent_other": "Не измерено.",
  "p.readings.workflow_heading": "Рабочий процесс задач",
  "p.readings.read_workflow": "Прочитать рабочий процесс задач",
  "p.readings.workflow_plan": "У этого плана свой рабочий процесс задач.",
  "p.readings.workflow_built_in":
    "Этот план использует встроенный рабочий процесс задач.",
  "p.readings.workflow_constrained": "Разрешены только перечисленные переходы.",
  "p.readings.workflow_free":
    "Задача может перейти из любого состояния в любое другое.",
  "p.readings.cat_todo": "не начато",
  "p.readings.cat_active": "в работе",
  "p.readings.cat_waiting": "ожидание",
  "p.readings.cat_done": "завершено",
  "p.readings.cat_other": "другой вид",
  "p.readings.initial": "с чего задача начинается",
  "p.readings.terminal": "чем задача заканчивается",
  "p.readings.wip_limit": "не более {n} одновременно",
  "p.readings.anyone": "любой",
  "p.readings.roles": "роли: {roles}",
  "p.readings.problem_depth": "Уровней вниз — целое число от 1 до {max}.",
  "p.readings.problem_depth_zero": "Уровней вниз — целое число от 0 до {max}.",
  "p.readings.problem_items":
    "Сколько элементов завершить — целое число не меньше 1.",
  "p.readings.problem_periods": "Периодов вперёд — целое число от 1 до 520.",
  "p.readings.problem_period_days": "Дней в периоде — целое число от 1 до 90.",
  "p.readings.problem_history": "Периодов истории — целое число от 6 до 260.",
  "p.readings.problem_window": "Окно — целое число дней от 1 до 3650.",
  "p.readings.refused_query": "Сервис не принял этот запрос.",
  "p.readings.refused_module": "Эта часть плана отключена.",
  "p.readings.refused_gone":
    "Этого плана больше нет, или вы больше не можете его читать.",
  "p.readings.flow_feature": "Новые возможности",
  "p.readings.flow_defect": "Дефекты",
  "p.readings.flow_risk": "Работа с рисками",
  "p.readings.flow_debt": "Технический долг",
  "p.readings.flow_unclassified": "Не классифицировано",
  "p.readings.flow_other": "Другой вид",
  "p.flowmix.heading": "Состав завершённой работы",
  "p.flowmix.says":
    "Как работа, завершённая за период, делится на новые возможности, дефекты, работу с рисками и технический долг, в сравнении с составом, который план задумал, если он его объявил. Если оставить пустым, окно — {n} дн.",
  "p.flowmix.f_window": "Дней назад (если пусто — {n})",
  "p.flowmix.f_rollup": "Включить вложенные планы",
  "p.flowmix.f_depth": "Уровней вниз (если пусто — {max})",
  "p.flowmix.read": "Прочитать состав",
  "p.flowmix.window_one": "Работа, завершённая за последний {n} день.",
  "p.flowmix.window_other": "Работа, завершённая за последние дни ({n}).",
  "p.flowmix.plans_one": "Учтён {n} план.",
  "p.flowmix.plans_other": "Учтено планов: {n}.",
  "p.flowmix.truncated": "Учтены не все планы ниже: обход достиг предела.",
  "p.flowmix.none": "За этот период работа не завершена.",
  "p.flowmix.no_intent":
    "План не объявляет задуманного состава, поэтому сравнивать не с чем.",
  "p.flowmix.table": "Завершённая работа по видам",
  "p.flowmix.col_kind": "Вид работы",
  "p.flowmix.col_count": "Завершено",
  "p.flowmix.col_share": "Доля",
  "p.flowmix.col_intended": "Задуманная доля",
  "p.flowmix.col_gap": "Разрыв",
  "p.ccpm.heading": "Агрессивные и безопасные оценки",
  "p.ccpm.says":
    "Для критической цепи каждой задаче нужны агрессивная оценка (примерно равные шансы) и безопасная, в целых рабочих днях от 1 до {max}, причём агрессивная не больше безопасной. Показанная цифра — записанная, иначе из диапазона риска, иначе выведенная правилом развёртывания; каждая указывает, откуда она. Веха и задача без длительности не оцениваются. Очистка обеих удаляет записанное.",
  "p.ccpm.rule_explicit_only":
    "Правило: используются только заданные оценки; ничего не выводится.",
  "p.ccpm.rule_half_of_safe":
    "Правило: недостающая безопасная оценка — длительность задачи, а недостающая агрессивная — половина безопасной.",
  "p.ccpm.rule_other": "Правило: такое, которого эта страница не знает.",
  "p.ccpm.rule_unusable":
    "Настройку правила развёртывания прочитать не удалось, поэтому используются только заданные оценки.",
  "p.ccpm.attention_one": "У {n} задачи нужны оценки.",
  "p.ccpm.attention_other": "Задач, которым нужны оценки: {n}.",
  "p.ccpm.more": "Показаны первые {shown} из {total} задач.",
  "p.ccpm.no_tasks": "В этом плане нет задач для оценки.",
  "p.ccpm.table": "Оценки по задачам",
  "p.ccpm.col_task": "Задача",
  "p.ccpm.col_likely": "Длительность в рабочих днях",
  "p.ccpm.col_aggressive": "Агрессивная",
  "p.ccpm.col_safe": "Безопасная",
  "p.ccpm.col_state": "Положение",
  "p.ccpm.figure": "{days} дн., {source}",
  "p.ccpm.none": "нет",
  "p.ccpm.source_recorded": "записано",
  "p.ccpm.source_risk_range": "из диапазона риска",
  "p.ccpm.source_rule": "по правилу",
  "p.ccpm.state_complete": "Обе заданы",
  "p.ccpm.state_derived": "Частично выведена",
  "p.ccpm.state_missing": "Нет оценки",
  "p.ccpm.state_inconsistent": "Агрессивная выше безопасной",
  "p.ccpm.state_milestone": "Веха, не оценивается",
  "p.ccpm.state_no_duration": "Нет длительности, не оценивается",
  "p.ccpm.state_other": "Другое положение",
  "p.ccpm.form": "Записать оценки задачи",
  "p.ccpm.f_task": "Задача",
  "p.ccpm.choose_task": "Выберите задачу",
  "p.ccpm.f_aggressive": "Агрессивная, в рабочих днях",
  "p.ccpm.f_safe": "Безопасная, в рабочих днях",
  "p.ccpm.save": "Сохранить оценки",
  "p.ccpm.unchanged": "Ничего не изменилось, поэтому ничего не отправлено.",
  "p.ccpm.saved": "Оценки сохранены: {title}.",
  "p.ccpm.cleared": "Записанные оценки очищены: {title}.",
  "p.ccpm.confirm_clear":
    "Очистить записанные оценки для {title}? Цифра из диапазона риска или по правилу может по-прежнему показываться.",
  "p.ccpm.problem_task": "Выберите задачу.",
  "p.ccpm.problem_aggressive":
    "Агрессивная оценка — целое число рабочих дней от 1 до {max}.",
  "p.ccpm.problem_safe":
    "Безопасная оценка — целое число рабочих дней от 1 до {max}.",
  "p.ccpm.problem_order": "Агрессивная оценка не может быть выше безопасной.",
  "p.ccpm.refused_invalid":
    "Сервис не принял эти оценки: у задачи могла пропасть длительность, или она стала вехой.",
  "p.ccpm.refused_gone":
    "Этой задачи больше нет, или вы больше не можете её читать.",
  "p.ccpm.could_not_load": "Не удалось прочитать оценки.",
  "p.ccpm.could_not_save": "Не удалось сохранить оценки.",
  "p.frozen.heading": "Замороженный план буферов",
  "p.frozen.none": "План буферов ещё не заморожен.",
  "p.frozen.freeze_link": "Заморозить его в управлении буферами",
  "p.frozen.taken": "Заморожен {date}; {rule}",
  "p.frozen.table": "Буферы в замороженном виде",
  "p.frozen.col_buffer": "Буфер",
  "p.frozen.col_days": "Рабочие дни",
  "p.frozen.col_chain": "Цепь",
  "p.frozen.col_finish": "Плановое окончание",
  "p.frozen.project": "Проектный буфер",
  "p.frozen.feeding": "Питающий буфер в {joins}",
  "p.frozen.unsized": "размер не задан",
  "p.frozen.untitled": "Задача без названия",
  "p.frozen.history_one": "Заморожен {n} план буферов: {dates}.",
  "p.frozen.history_other":
    "Заморожено планов буферов — {n}, сначала новые: {dates}.",
  "p.frozen.could_not_load": "Не удалось прочитать замороженный план буферов.",
  "p.workingtime.heading": "Рабочий день и рабочая неделя",
  "p.workingtime.says":
    "Какова длительность рабочего дня (от 1 минуты до 24 часов) и сколько дней в неделю работают (от 1 до 7) для этого плана или как значение организации по умолчанию. Каждое задаётся новой записью, которую здесь нельзя изменить или удалить, поэтому вас спросят заранее; вторая запись для этого плана отклоняется.",
  "p.workingtime.form": "Задать рабочий день и неделю",
  "p.workingtime.f_scope": "Для",
  "p.workingtime.scope_plan": "Этого плана",
  "p.workingtime.scope_default": "Значения организации по умолчанию",
  "p.workingtime.f_hours": "Часов в дне",
  "p.workingtime.f_minutes": "И минут",
  "p.workingtime.f_days": "Дней в неделе",
  "p.workingtime.save": "Задать рабочий день",
  "p.workingtime.confirm_plan_one":
    "Задать рабочий день этого плана: {day}, {days} день в неделю? После этого здесь его нельзя изменить.",
  "p.workingtime.confirm_plan_other":
    "Задать рабочий день этого плана: {day}, дней в неделю — {days}? После этого здесь его нельзя изменить.",
  "p.workingtime.confirm_default_one":
    "Задать рабочий день организации по умолчанию: {day}, {days} день в неделю? После этого здесь его нельзя изменить.",
  "p.workingtime.confirm_default_other":
    "Задать рабочий день организации по умолчанию: {day}, дней в неделю — {days}? После этого здесь его нельзя изменить.",
  "p.workingtime.saved_one": "Задан рабочий день: {day}, {days} день в неделю.",
  "p.workingtime.saved_other":
    "Задан рабочий день: {day}, дней в неделю — {days}.",
  "p.workingtime.away_heading": "Время вне проектной работы",
  "p.workingtime.away_says":
    "Отпуск, праздник, учебный отпуск или обязанность вне проекта для одного человека с первого по последний день, чтобы это время не считалось простоем. Человек указывается ссылкой вроде person: и идентификатора. После записи её нельзя изменить или удалить, поэтому вас спросят заранее; пустое примечание не отправляется.",
  "p.workingtime.f_person": "Ссылка на человека",
  "p.workingtime.f_starts": "Первый день",
  "p.workingtime.f_ends": "Последний день",
  "p.workingtime.f_kind": "Вид",
  "p.workingtime.f_note": "Примечание (необязательно)",
  "p.workingtime.record": "Записать время вне работы",
  "p.workingtime.confirm_away":
    "Записать для {person}: {kind}, {range}? После этого запись нельзя изменить или удалить.",
  "p.workingtime.recorded": "Записано для {person}: {kind}, {range}.",
  "p.workingtime.kind_leave": "отпуск",
  "p.workingtime.kind_holiday": "праздник",
  "p.workingtime.kind_study_leave": "учебный отпуск",
  "p.workingtime.kind_non_project_duty": "обязанность вне проекта",
  "p.workingtime.problem_hours": "Часы — целое число от 0 до 24.",
  "p.workingtime.problem_minutes": "Минуты — целое число от 0 до 59.",
  "p.workingtime.problem_day": "Рабочий день — от 1 минуты до 24 часов.",
  "p.workingtime.problem_week": "Дней в неделе — целое число от 1 до 7.",
  "p.workingtime.problem_person":
    "Укажите человека как person: или worker: и идентификатор.",
  "p.workingtime.problem_starts": "Укажите первый день.",
  "p.workingtime.problem_ends": "Укажите последний день.",
  "p.workingtime.problem_order": "Последний день не может быть раньше первого.",
  "p.workingtime.refused_exists":
    "Для этого плана рабочий день уже задан, и заменить его здесь нельзя.",
  "p.workingtime.refused_working": "Сервис не принял этот рабочий день.",
  "p.workingtime.refused_non_working": "Сервис не принял это время вне работы.",
  "p.workingtime.refused_forbidden": "Вам нельзя это записывать.",
  "p.workingtime.could_not_save": "Не удалось это записать.",
  "p.task.is_hammock":
    "Эта задача — гамак: её даты берутся из охватываемых ею задач.",
  "p.task.hammock_link": "Открыть её охват на критическом пути",
  "p.adoption.could_not": "Не удалось записать внедрение.",
  "p.adoption.f_active": "Активные пользователи",
  "p.adoption.f_definition": "Что считалось активным пользователем",
  "p.adoption.f_target": "Для скольких пользователей",
  "p.adoption.f_window": "За сколько дней",
  "p.adoption.form": "Записать внедрение",
  "p.adoption.heading": "Сколько людей этим пользуются",
  "p.adoption.problem_active":
    "Укажите активных пользователей целым числом, 0 или больше.",
  "p.adoption.problem_definition":
    "Укажите, что считалось активным пользователем, чтобы последующие показания можно было сравнивать.",
  "p.adoption.problem_target":
    "Укажите, для скольких пользователей это предназначено, целым числом больше 0.",
  "p.adoption.problem_window": "Укажите окно целым числом дней, 1 или больше.",
  "p.adoption.record": "Записать внедрение",
  "p.adoption.refused": "Сервис не принял эти цифры.",
  "p.adoption.refused_forbidden":
    "Вам нельзя записывать внедрение для этого плана.",
  "p.adoption.refused_gone": "Этого плана больше нет.",
  "p.adoption.saved": "Внедрение записано.",
  "p.adoption.says":
    "Активные пользователи относительно тех, для кого это предназначено, за окно в несколько дней. То, что считалось активным, хранится рядом с цифрой, чтобы более позднее показание можно было с ним сравнить.",
  "p.bulk.apply": "Задать для выбранных задач",
  "p.bulk.choose_all": "Выбрать все задачи",
  "p.bulk.choose_none": "Снять выбор",
  "p.bulk.choose_plan": "Выберите план",
  "p.bulk.chosen_one": "Выбрана {n} задача",
  "p.bulk.chosen_other": "Выбрано задач: {n}",
  "p.bulk.confirm_delete_one":
    "Удалить {n} задачу? Она исчезнет с доски и из всех представлений плана.",
  "p.bulk.confirm_delete_other":
    "Удалить задач: {n}? Они исчезнут с доски и из всех представлений плана.",
  "p.bulk.confirm_move_one":
    "Переместить {n} задачу в план {plan}? Её спринт, компонент и родительская задача очищаются, а статус, которого нет в том плане, заменяется первым.",
  "p.bulk.confirm_move_other":
    "Переместить задач в план {plan}: {n}? Их спринт, компонент и родительская задача очищаются, а статус, которого нет в том плане, заменяется первым.",
  "p.bulk.copy": "Копировать в план",
  "p.bulk.could_not": "Не удалось изменить задачи.",
  "p.bulk.could_not_load_plans": "Не удалось прочитать планы.",
  "p.bulk.delete": "Удалить выбранные задачи",
  "p.bulk.delete_failed": "Не удалены: {titles}.",
  "p.bulk.deleted_gone_one": "{n} задачи уже не было.",
  "p.bulk.deleted_gone_other": "Задач, которых уже не было: {n}.",
  "p.bulk.deleted_one": "Удалена {n} задача.",
  "p.bulk.deleted_other": "Удалено задач: {n}.",
  "p.bulk.done_one": "Выполнено для {done} из {requested} задачи.",
  "p.bulk.done_other": "Выполнено для {done} из {requested} задач.",
  "p.bulk.edit_form": "Задать поля для выбранных задач",
  "p.bulk.f_assignee": "Исполнитель",
  "p.bulk.f_due": "Срок",
  "p.bulk.f_plan": "План",
  "p.bulk.f_start": "Дата начала",
  "p.bulk.f_value": "Новое значение: {field}",
  "p.bulk.heading": "Изменить много задач сразу",
  "p.bulk.keep_assignee": "Сохранить исполнителя при копировании",
  "p.bulk.mapped_one":
    "У {n} задачи статуса, которого нет в том плане, поэтому она начинается в первом состоянии того плана.",
  "p.bulk.mapped_other":
    "У задач ({n}) статуса, которого нет в том плане, поэтому они начинаются в первом состоянии того плана.",
  "p.bulk.mode_clear": "Очистить",
  "p.bulk.mode_keep": "Оставить как есть",
  "p.bulk.mode_set": "Задать",
  "p.bulk.move": "Переместить в план",
  "p.bulk.no_tasks": "В этом плане нет задач.",
  "p.bulk.problem_assignee":
    "Укажите исполнителя как person: или worker: и идентификатор.",
  "p.bulk.problem_due": "Укажите срок как существующую дату.",
  "p.bulk.problem_no_field":
    "Выберите хотя бы одно поле, которое нужно задать или очистить.",
  "p.bulk.problem_none": "Выберите хотя бы одну задачу.",
  "p.bulk.problem_plan": "Выберите план, в который копировать или перемещать.",
  "p.bulk.problem_same_plan":
    "Задачи уже в этом плане: выберите другой план, чтобы переместить их.",
  "p.bulk.problem_start":
    "Укажите дату начала как существующую дату от {min} до {latest}.",
  "p.bulk.problem_too_many": "Выбирайте не более {max} задач за раз.",
  "p.bulk.refused_edit": "Сервис не принял эти значения.",
  "p.bulk.refused_forbidden": "Вам нельзя изменять эти задачи.",
  "p.bulk.refused_plan_gone": "Этого плана больше нет, или вы его не видите.",
  "p.bulk.refused_titles":
    "Отказано по причине, которую сервис сообщает только по-английски: {titles}.",
  "p.bulk.refused_transfer":
    "Вы не видите этот план, или сервис не принял список.",
  "p.bulk.says":
    "Выберите задачи, затем задайте для всех одного исполнителя, срок или дату начала, скопируйте их в план или переместите туда либо удалите. Не более {max} за раз; дата начала — от {min} до {latest}. Статус изменяется на доске, по одной карточке. Копия начинается в первом состоянии того плана и сохраняет исполнителя только по запросу; перемещение очищает спринт, компонент и родительскую задачу, а задача с подзадачами не перемещается.",
  "p.bulk.this_plan": "{name} (этот план)",
  "p.bulk.transfer_form": "Копировать или переместить выбранные задачи",
  "p.bulk.unchanged":
    "У выбранных задач уже такие значения, поэтому ничего не отправлено.",
  "p.ceremony.choose_kind": "Выберите церемонию",
  "p.ceremony.could_not_load": "Не удалось прочитать церемонии спринта.",
  "p.ceremony.could_not_save": "Не удалось записать церемонию.",
  "p.ceremony.counts": "Проведённые церемонии",
  "p.ceremony.f_facilitator":
    "Фасилитатор (person: или worker: и идентификатор, необязательно)",
  "p.ceremony.f_kind": "Церемония",
  "p.ceremony.f_summary": "Что решено (необязательно)",
  "p.ceremony.facilitated_by": "фасилитатор: {ref}",
  "p.ceremony.form": "Записать церемонию",
  "p.ceremony.heading": "Церемонии: {name}",
  "p.ceremony.held_count_one": "{kind}: проведена {n} раз",
  "p.ceremony.held_count_other": "{kind}: проведена раз — {n}",
  "p.ceremony.held_saved": "Записано: {kind}.",
  "p.ceremony.kind_daily": "Ежедневная планёрка",
  "p.ceremony.kind_other": "Другая церемония",
  "p.ceremony.kind_planning": "Планирование",
  "p.ceremony.kind_retrospective": "Ретроспектива",
  "p.ceremony.kind_review": "Обзор",
  "p.ceremony.none_held": "{kind}: не проводилась",
  "p.ceremony.problem_facilitator":
    "Укажите фасилитатора как person: или worker: и идентификатор либо оставьте пустым.",
  "p.ceremony.problem_kind": "Выберите церемонию.",
  "p.ceremony.problem_once":
    "В этом спринте уже проведено: {kind}; повторное — это перепланирование, то есть новый спринт.",
  "p.ceremony.record": "Записать церемонию",
  "p.ceremony.refused_ceremony":
    "Сервис отказал: в этом спринте эта церемония, возможно, уже проведена.",
  "p.ceremony.refused_forbidden": "Вам нельзя записывать это для спринта.",
  "p.ceremony.refused_gone": "Этого спринта больше нет.",
  "p.ceremony.says":
    "В спринте проводятся одно планирование и один обзор и любое число ежедневных планёрок и ретроспектив. Считается каждый вид, в том числе не проведённые.",
  "p.checkins.heading": "Отметки хода: {title}",
  "p.checkins.none": "Отметок хода пока нет.",
  "p.checkins.row": "{date}: значение {value}",
  "p.commitment.added_one": "С тех пор добавлена {n} задача: {titles}.",
  "p.commitment.added_other": "С тех пор добавлено задач: {n}: {titles}.",
  "p.commitment.commit": "Зафиксировать задачи спринта",
  "p.commitment.confirm_one":
    "Зафиксировать сейчас {n} задачу в спринте {name}? Фиксация выполняется один раз и не может быть изменена; задачи, добавленные или удалённые позже, будут показаны как изменение.",
  "p.commitment.confirm_other":
    "Зафиксировать сейчас задачи в спринте {name}: {n}? Фиксация выполняется один раз и не может быть изменена; задачи, добавленные или удалённые позже, будут показаны как изменение.",
  "p.commitment.could_not": "Не удалось зафиксировать спринт.",
  "p.commitment.heading": "Что спринт взял на себя",
  "p.commitment.not_yet_one":
    "Ещё не зафиксировано; сейчас в спринте {n} задача.",
  "p.commitment.not_yet_other":
    "Ещё не зафиксировано; сейчас в спринте задач: {n}.",
  "p.commitment.problem_done":
    "Этот спринт уже зафиксирован: обязательство никогда не переписывается.",
  "p.commitment.problem_empty": "В этом спринте нет задач для фиксации.",
  "p.commitment.problem_unread": "Обязательство спринта ещё не прочитано.",
  "p.commitment.refused_commit":
    "Сервис отказал: этот спринт уже зафиксирован.",
  "p.commitment.removed_one": "С тех пор удалена {n} задача: {titles}.",
  "p.commitment.removed_other": "С тех пор удалено задач: {n}: {titles}.",
  "p.commitment.saved_one": "Зафиксирована {n} задача.",
  "p.commitment.saved_other": "Зафиксировано задач: {n}.",
  "p.commitment.says":
    "Задачи спринта такими, какими они были на момент фиксации, снимаются один раз, поэтому задача, добавленная или удалённая позже, читается как изменение, а не как план с самого начала.",
  "p.commitment.state":
    "Зафиксировано: {committed}; сейчас в спринте: {current}.",
  "p.commitment.unchanged": "С тех пор ничего не добавлено и не удалено.",
  "p.hours.col_actual": "Записано",
  "p.hours.col_placeholder": "Из них не назначено",
  "p.hours.col_plan": "План",
  "p.hours.col_planned": "Запланировано",
  "p.hours.col_variance": "Записано минус запланировано",
  "p.hours.could_not": "Не удалось прочитать часы.",
  "p.hours.f_from": "С",
  "p.hours.f_to": "По",
  "p.hours.form": "Запланированные и записанные часы",
  "p.hours.fte_says":
    "Эквиваленты полной занятости — это процент-дни, делённые на 100, умноженные на число дней в периоде; разрыв — предложение минус спрос (меньше нуля — нехватка). Предлагаемые бронирования — отдельный ряд.",
  "p.hours.h": "{h} ч",
  "p.hours.heading": "Запланированные и записанные часы",
  "p.hours.masked": "Люди не показаны: вам нельзя ими управлять.",
  "p.hours.none":
    "Ни у одного плана за этот период нет запланированных или записанных часов.",
  "p.hours.nothing_recorded": "ничего не записано",
  "p.hours.problem_from":
    "Укажите первый день как существующую дату или оставьте пустым.",
  "p.hours.problem_order": "Последний день не может быть раньше первого.",
  "p.hours.problem_span": "Окно может быть не более {max} дней.",
  "p.hours.problem_to":
    "Укажите последний день как существующую дату или оставьте пустым.",
  "p.hours.read": "Прочитать часы",
  "p.hours.refused": "Сервис не принял это окно.",
  "p.hours.says":
    "Запланировано — доля именного зафиксированного бронирования в каждом рабочем дне; записано — время, внесённое людьми, утверждённое или нет. Оставьте дни пустыми, чтобы получить четыре недели до сегодняшнего дня; не более {max} дней.",
  "p.hours.table": "Часы по планам",
  "p.hours.totals": "Запланировано {planned}; записано {actual}.",
  "p.hours.truncated":
    "Прочитаны только первые записи; итоги могут быть занижены.",
  "p.hours.unplanned": "{plan}: записано {hours}",
  "p.hours.unplanned_heading":
    "Время, записанное людьми без бронирования в плане",
  "p.hours.unplanned_people_one": "({n} человек)",
  "p.hours.unplanned_people_other": "(человек: {n})",
  "p.hours.window": "С {from} по {to}.",
  "p.kpireadings.col_standing": "Положение",
  "p.kpireadings.counted": "учтено",
  "p.kpireadings.replaced": "заменено более поздним показанием в тот же день",
  "p.kpireadings.says":
    "Все показания, сначала новые. Более позднее показание в тот же день заменяет более раннее, которое сохраняется.",
  "p.live.burndown_says":
    "Оставшиеся задачи по дням по записанным завершениям; идеальная линия не рисуется, и ничего не дополняется.",
  "p.live.event_comment_added": "добавлен комментарий",
  "p.live.event_other": "изменение",
  "p.live.event_raci_updated": "изменилось, кто что делает",
  "p.live.event_resync": "доска полностью перечитана",
  "p.live.event_task_created": "добавлена задача",
  "p.live.event_task_moved": "задача перемещена",
  "p.live.event_task_updated": "задача изменена",
  "p.live.follow": "Следить за изменениями по мере их появления",
  "p.live.heading": "Изменения по мере их появления",
  "p.live.last_change": "Последнее: {change}, {time}.",
  "p.live.says":
    "Пока слежение включено, доска перечитывается всякий раз, когда кто-то добавляет, перемещает или изменяет здесь задачу. Слежение прекращается, когда вы покидаете страницу.",
  "p.live.state_connecting": "Подключение…",
  "p.live.state_off": "Слежения нет.",
  "p.live.state_on": "Идёт слежение.",
  "p.live.state_retrying": "Соединение потеряно; повторная попытка.",
  "p.live.state_stopped": "Остановлено: сервис не стал присылать изменения.",
  "p.live.stop": "Перестать следить",
  "p.live.velocity_says":
    "Задачи и баллы, завершённые в каждом спринте, считаются по доске; это не мера потока.",
  "p.placeholder.choose": "Выберите заглушку",
  "p.placeholder.confirm_over":
    "Если назначить это человеку {person}, в некоторые дни он превысит загрузку. Всё равно сделать?",
  "p.placeholder.confirm_remove":
    "Удалить эту заглушку: {what}? Вместе с ней уйдёт спрос, который она обозначала.",
  "p.placeholder.could_not": "Не удалось изменить заглушку.",
  "p.placeholder.f_person": "Человек (person: или worker: и идентификатор)",
  "p.placeholder.f_placeholder": "Заглушка",
  "p.placeholder.form": "Назначить заглушку человеку",
  "p.placeholder.give": "Назначить человеку",
  "p.placeholder.given": "Назначено: {person}.",
  "p.placeholder.heading": "Назначить заглушку человеку или удалить её",
  "p.placeholder.not_given": "Не назначено: ничего не изменено.",
  "p.placeholder.open": "открыта",
  "p.placeholder.option": "{role} в плане {plan}, {percent}, с {from} по {to}",
  "p.placeholder.problem_choose": "Выберите заглушку.",
  "p.placeholder.problem_person":
    "Укажите человека как person: или worker: и идентификатор.",
  "p.placeholder.refused_forbidden":
    "Это может сделать только администратор или менеджер ресурсов.",
  "p.placeholder.refused_gone": "Этого бронирования больше нет.",
  "p.placeholder.refused_named":
    "В этом бронировании уже указан человек; список перечитан.",
  "p.placeholder.refused_person": "Сервис не принял этого человека.",
  "p.placeholder.remove": "Удалить заглушку",
  "p.placeholder.removed": "Заглушка удалена.",
  "p.placeholder.says":
    "Заглушка — это запрошенная роль без человека. Назначить её кому-либо — решение менеджера ресурсов; если это приведёт к превышению загрузки, вас сначала спросят.",
  "p.planokr.as_of": "вычислено {at}",
  "p.planokr.choose_plan": "Выберите план",
  "p.planokr.could_not": "Не удалось прочитать цели плана.",
  "p.planokr.could_not_load_plans": "Не удалось прочитать планы.",
  "p.planokr.currencies": "ключевые результаты в более чем одной валюте",
  "p.planokr.f_plan": "План",
  "p.planokr.form": "Оценка целей плана",
  "p.planokr.heading": "Оценка целей плана",
  "p.planokr.its_key_results": "Ключевые результаты: {title}",
  "p.planokr.key_results_one": "{n} ключевой результат",
  "p.planokr.key_results_other": "Ключевых результатов: {n}",
  "p.planokr.left_out_one": "{n} цель не измеряется и не входит в оценку.",
  "p.planokr.left_out_other":
    "Целей, которые не измеряются и не входят в оценку: {n}.",
  "p.planokr.no_objectives": "С этим планом не согласована ни одна цель.",
  "p.planokr.not_measured":
    "Не измерено: пока нельзя измерить ни одну согласованную с ним цель.",
  "p.planokr.objective_not_measured": "не измерено",
  "p.planokr.problem_plan": "Выберите план.",
  "p.planokr.read": "Прочитать оценку",
  "p.planokr.refused_gone": "Этого плана или цели больше нет.",
  "p.planokr.says":
    "Оценка каждой цели, согласованной с планом, взвешенная по тому, насколько сильно она согласована. Цель, которую нечем измерить, исключается и никогда не считается за 0%.",
  "p.planokr.score": "Оценка {score}",
  "p.planokr.unmeasurable_no_range": "нет прогресса: начало и цель совпадают",
  "p.planokr.unmeasurable_no_tolerance":
    "нет прогресса: значение удерживается без допуска",
  "p.planokr.unmeasurable_other": "прогресс вычислить нельзя",
  "p.planokr.unmeasurable_overflow": "нет прогресса: числа слишком велики",
  "p.planokr.unnamed": "Цель ({id})",
  "p.planokr.weight": "вес {weight}",
  "p.rate.answer_person":
    "{rate} в час на {date}, из собственной таблицы человека, действует с {from}.",
  "p.rate.answer_role":
    "{rate} в час на {date}, из таблицы его роли, действует с {from}.",
  "p.rate.could_not": "Не удалось прочитать ставку.",
  "p.rate.f_date": "На дату",
  "p.rate.f_person": "Человек",
  "p.rate.f_table": "Таблица",
  "p.rate.form": "Действующая ставка стоимости",
  "p.rate.heading": "Действующая ставка стоимости",
  "p.rate.none": "На {date} для этого человека нет действующей ставки.",
  "p.rate.problem_date": "Укажите день как существующую дату.",
  "p.rate.problem_person": "Укажите ссылку на человека.",
  "p.rate.read": "Прочитать ставку",
  "p.rate.refused_hidden":
    "Нет ставки, которую вам можно видеть: ставки показываются только финансовому отделу.",
  "p.rate.refused_query": "Сервис не принял этот запрос.",
  "p.rate.says":
    "Почасовая ставка, действующая для человека в день: из его собственной таблицы, иначе из таблицы его роли. Стандартная таблица, если не назвать другую.",
  "p.skillgap.choose_plan": "Выберите план",
  "p.skillgap.col_person": "Человек",
  "p.skillgap.col_skill": "Навык",
  "p.skillgap.col_status": "Положение",
  "p.skillgap.could_not": "Не удалось прочитать пробелы в навыках.",
  "p.skillgap.count_covered_one": "{n} навык есть;",
  "p.skillgap.count_covered_other": "Навыков есть: {n};",
  "p.skillgap.count_missing_one": "{n} навыка нет;",
  "p.skillgap.count_missing_other": "Навыков нет: {n};",
  "p.skillgap.count_unknown_one": "{n} неизвестно.",
  "p.skillgap.count_unknown_other": "Неизвестно: {n}.",
  "p.skillgap.f_plan": "План",
  "p.skillgap.form": "Пробелы в навыках плана",
  "p.skillgap.heading": "Навыки, которые требуют бронирования",
  "p.skillgap.none": "Ни одно бронирование в этом плане не требует навыка.",
  "p.skillgap.problem_plan": "Выберите план.",
  "p.skillgap.read": "Прочитать пробелы в навыках",
  "p.skillgap.says":
    "Каждый навык, которого требует бронирование в плане, в сравнении с тем, что есть у человека в сервисе кадров. «Неизвестно» означает, что его навыки прочитать не удалось; заглушка пока без человека и не учитывается.",
  "p.skillgap.status_covered": "есть",
  "p.skillgap.status_missing": "нет",
  "p.skillgap.status_unknown": "неизвестно",
  "p.skillgap.table": "Навыки по бронированиям",
  "p.util.by_person": "По людям",
  "p.util.by_team": "Команда",
  "p.util.col_available": "Доступно",
  "p.util.col_away": "Отсутствует",
  "p.util.col_effort": "Трудозатраты",
  "p.util.col_figure": "Использование мощности",
  "p.util.col_person": "Человек",
  "p.util.could_not": "Не удалось прочитать использование мощности.",
  "p.util.every_plan": "Все планы",
  "p.util.f_by": "Показать",
  "p.util.f_plan": "План",
  "p.util.f_window": "Дней назад от сегодняшнего",
  "p.util.form": "Трудозатраты и мощность",
  "p.util.heading": "Трудозатраты и мощность",
  "p.util.none":
    "За этот период никто не записал трудозатрат или времени отсутствия.",
  "p.util.over":
    "на пределе или выше мощности: это предупреждение, а не достижение",
  "p.util.people_counted_one": "Учтён {n} человек.",
  "p.util.people_counted_other": "Учтено людей: {n}.",
  "p.util.problem_window": "Укажите окно целым числом дней от 1 до {max}.",
  "p.util.read": "Прочитать использование мощности",
  "p.util.refused": "Сервис не принял этот запрос.",
  "p.util.says":
    "Записанные трудозатраты относительно заявленной мощности за вычетом времени отсутствия за {days} дн. до сегодняшнего дня, если не выбрать от 1 до {max}. На пределе или выше мощности — предупреждение об очереди. Малые группы и малые цифры скрываются.",
  "p.util.suppression":
    "Скрыто: группа меньше {people} человек или цифра меньше {hours}.",
  "p.util.table": "Использование мощности по людям",
  "p.util.team_detail": "{effort} из {available} доступных, отсутствует {away}",
  "p.util.unavailable_all_away": "отсутствует весь период, а не 0%",
  "p.util.unavailable_below_floor": "слишком мало, чтобы сообщать",
  "p.util.unavailable_no_capacity": "мощность не заявлена",
  "p.util.unavailable_other": "сообщать нельзя",
  "p.util.unavailable_small_group": "слишком мало людей для показа",
  "p.util.window_one": "За последний {n} день.",
  "p.util.window_other": "За последние дни: {n}.",
};
