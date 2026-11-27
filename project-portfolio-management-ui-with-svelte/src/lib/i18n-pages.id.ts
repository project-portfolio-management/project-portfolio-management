// Indonesian (id-001) for the page and component catalogue (WW-1.1). Typed as a complete record of
// the English keys, so a missing string is a compile error.
//
// STATUS: MACHINE DRAFT. Written by Claude (the AI assistant) on 2026-11-21 and NOT reviewed by a
// speaker or a professional translator. The review is recorded in
// `governance/translation-reviews/id-001.md`; update the status there, not here.

import type { PageKey } from "./i18n-pages";

export const PAGES_ID: Record<PageKey, string> = {
  "c.CalendarMonth.next_month": "Bulan berikutnya",
  "p.calendar.day": "Hari",
  "p.calendar.agenda": "Agenda",
  "p.calendar.status": "Status",
  "p.calendar.status_done": "selesai",
  "p.calendar.status_open": "terbuka",
  "p.calendar.item": "Item pekerjaan",
  "p.calendar.showing": "Menampilkan {view}, {start} sampai {end}",
  "p.calendar.more": "+{n} lagi",
  "p.calendar.milestone": "Tonggak",
  "p.calendar.due": "Tenggat",
  "p.plans.pid.board.assignee": "Penanggung jawab",
  "p.plans.pid.board.moving": "Memindahkan {title} ke {column}…",
  "p.plans.pid.board.move_failed":
    "Tidak dapat memindahkan {title} ke {column}; kembali ke tempat semula.",
  "p.plans.pid.board.empty_column": "Tidak ada tugas",
  "p.plans.pid.board.search": "Cari di papan",
  "p.plans.pid.board.task": "Tugas",
  "p.plans.pid.board.status": "Status",
  "c.GanttView.holiday": "Hari libur",
  "c.GanttView.shaded":
    "Kolom yang diarsir adalah hari libur: akhir pekan dan hari libur dalam kalender kerja rencana ini.",
  "c.GanttView.kind": "Jenis",
  "c.GanttView.stage": "Tahap",
  "c.GanttView.task": "Tugas",
  "c.GanttView.start": "Mulai",
  "c.GanttView.end": "Selesai",
  "c.GanttView.predecessors": "Bergantung pada",
  "c.GanttView.depends_on": "Bergantung pada: {names}",
  "c.GanttView.critical": "{name} (jalur kritis)",
  "c.GanttView.zoom_in": "Skala waktu lebih halus",
  "c.GanttView.zoom_out": "Skala waktu lebih kasar",
  "c.GanttView.zoomed": "Skala waktu: {unit}",
  "c.GanttView.unit_day": "hari",
  "c.GanttView.unit_week": "minggu",
  "c.GanttView.unit_month": "bulan",
  "c.GanttView.export": "Ekspor jadwal",
  "c.GanttView.csv": "CSV",
  "c.GanttView.excel": "Excel",
  "c.GanttView.exported": "{n} tugas diekspor ke {file}",
  "c.CalendarMonth.week": "Minggu",
  "c.CalendarMonth.four_weeks": "4 minggu",
  "c.CalendarMonth.month": "Bulan",
  "c.CalendarMonth.today": "Hari ini",
  "c.CalendarMonth.previous": "{view} sebelumnya",
  "c.CalendarMonth.next": "{view} berikutnya",
  "c.CalendarMonth.working_calendar": "Kalender kerja",
  "c.CalendarMonth.previous_month": "Bulan sebelumnya",
  "c.CapacityMeter.days_over_capacity": "Hari melebihi kapasitas",
  "c.CostRiskView.adds": "{risk}: menambah {money} — {eventHitText}",
  "c.CostRiskView.already_spent": "Sudah dibelanjakan",
  "c.CostRiskView.assumptions": "Asumsi",
  "c.CostRiskView.budget": "anggaran",
  "c.CostRiskView.budget_2": "Anggaran",
  "c.CostRiskView.budget_sum_of_planned_amounts":
    "Anggaran (jumlah nilai terencana)",
  "c.CostRiskView.by_plan": "Per rencana",
  "c.CostRiskView.cost": "Biaya",
  "c.CostRiskView.cost_at_each_percentile":
    "Biaya dalam {currency} pada tiap persentil",
  "c.CostRiskView.cost_from": "Biaya dari",
  "c.CostRiskView.each_bar_is_a_cost":
    "Setiap batang adalah rentang biaya; makin tinggi berarti makin banyak simulasi berakhir di sana. Garis tegas adalah P50, P80, dan P90 (berlabel); garis putus-putus adalah anggaran.",
  "c.CostRiskView.each_plan_s_budget_and":
    "Anggaran dan rata-rata biaya simulasi tiap rencana dalam {currency}",
  "c.CostRiskView.estimate": "Perkiraan",
  "c.CostRiskView.independence": "Independensi.",
  "c.CostRiskView.left_out": "Tidak disertakan:",
  "c.CostRiskView.line": "Baris",
  "c.CostRiskView.low_planned_high": "Rendah / terencana / tinggi",
  "c.CostRiskView.mean_simulated_cost": "Rata-rata biaya simulasi",
  "c.CostRiskView.no_register_risk_was_used":
    "Tidak ada risiko dari register yang dipakai sebagai peristiwa biaya.",
  "c.CostRiskView.outcome": "Hasil",
  "c.CostRiskView.per_line_estimate_and_sensitivity":
    "Perkiraan dan sensitivitas per baris dalam {currency}",
  "c.CostRiskView.plan": "Rencana",
  "c.CostRiskView.risks_that_added_cost": "Risiko yang menambah biaya",
  "c.CostRiskView.seed_the_same_seed_gives":
    "Seed {seed}: seed yang sama memberi hasil yang sama. Hingga {max_iterations} iterasi.",
  "c.CostRiskView.sensitivity": "Sensitivitas",
  "c.CostRiskView.simulations": "Simulasi",
  "c.CostRiskView.spent": "Terpakai",
  "c.CostRiskView.the_same_figures_as_a": "Angka yang sama dalam bentuk tabel",
  "c.CostRiskView.to": "hingga",
  "c.CostRiskView.to_of": "{money} hingga {money2}: {count} dari {iterations}",
  "c.CostRiskView.what_drives_it": "Apa penyebabnya",
  "c.CpmGantt.cannot_meet_a_constraint_or":
    "tidak dapat memenuhi kendala atau tenggat (garis menandai batasnya)",
  "c.CpmGantt.critical_any_slip_moves_the":
    "kritis: keterlambatan apa pun menggeser penyelesaian",
  "c.CpmGantt.has_float_the_hatched_tail": "punya kelonggaran (ekor berarsir)",
  "c.CpmGantt.legend": "Legenda",
  "c.CpmGantt.near_critical": "hampir kritis",
  "c.CpmGantt.no_scheduled_tasks_to_draw":
    "Belum ada tugas terjadwal untuk digambar. Beri tugas durasi (dan tanggal mulai atau pendahulu).",
  "c.CpmGantt.the_outline_under_a_bar":
    "garis tepi di bawah batang adalah posisinya saat baseline ditetapkan",
  "c.CpmGantt.to_float_is_counted_in":
    "{from} hingga {to}. Kelonggaran dihitung dalam hari kerja rencana; batang digambar dalam hari kalender.",
  "c.CpmTable.calendar": "Kalender",
  "c.CpmTable.constraint": "Kendala",
  "c.CpmTable.deadline": "Tenggat",
  "c.CpmTable.finishes": "Selesai",
  "c.CpmTable.free_float": "Kelonggaran bebas",
  "c.CpmTable.overrides_its_predecessors": "(menimpa pendahulunya)",
  "c.CpmTable.plan": "rencana",
  "c.CpmTable.plan_assignee": "rencana + penanggung jawab",
  "c.CpmTable.starts": "Mulai",
  "c.CpmTable.state": "Keadaan",
  "c.CpmTable.task": "Tugas",
  "c.CpmTable.task_schedule_with_float_in":
    "Jadwal tugas dengan kelonggaran, dalam hari kerja rencana",
  "c.CpmTable.total_float": "Kelonggaran total",
  "c.CpmTable.untitled": "(tanpa judul)",
  "c.CumulativeFlow.cumulative_flow": "Alur kumulatif",
  "c.CumulativeFlow.day": "Hari",
  "c.CumulativeFlow.every_value_in_the_chart":
    "Setiap nilai dalam bagan. {note}",
  "c.CumulativeFlow.no_samples_in_this_window":
    "Tidak ada sampel di jendela ini.",
  "c.CumulativeFlow.the_vertical_gap_above_done":
    "jarak vertikal di atas “Selesai” adalah pekerjaan dalam proses; jarak horizontal ke sana kira-kira adalah waktu siklus",
  "c.CumulativeFlow.total": "Total",
  "c.CumulativeFlow.total_in_progress_done":
    "{total} total · {work_in_progress} dalam proses · {done} selesai",
  "c.EvBars.the_double_line_is_the":
    "Garis ganda adalah anggaran saat selesai: {money}.",
  "c.EvBars.unknown": "tidak diketahui",
  "c.KpiCard.add_a_reading": "Tambah pembacaan",
  "c.KpiCard.cadence": "Frekuensi",
  "c.KpiCard.daily": "Harian",
  "c.KpiCard.delete": "Hapus",
  "c.KpiCard.edit_or_delete": "Ubah atau hapus",
  "c.KpiCard.history": "Riwayat ({readings})",
  "c.KpiCard.loading": "Memuat…",
  "c.KpiCard.monthly": "Bulanan",
  "c.KpiCard.name": "Nama",
  "c.KpiCard.no_reading_yet": "Belum ada pembacaan.",
  "c.KpiCard.no_readings": "Tidak ada pembacaan.",
  "c.KpiCard.none": "Tidak ada",
  "c.KpiCard.note": "Catatan",
  "c.KpiCard.observed": "Teramati",
  "c.KpiCard.observed_on": "Teramati pada",
  "c.KpiCard.on": "{valueText} pada {observed_on}",
  "c.KpiCard.quarterly": "Kuartalan",
  "c.KpiCard.read": "· dibaca {cadence}",
  "c.KpiCard.record": "Catat",
  "c.KpiCard.red_line": "Garis merah",
  "c.KpiCard.save": "Simpan",
  "c.KpiCard.target": "Target",
  "c.KpiCard.unit": "Satuan",
  "c.KpiCard.value": "Nilai",
  "c.KpiCard.weekly": "Mingguan",
  "c.KpiCard.which_way_is_good_cannot":
    "Arah yang baik ({v}) tidak dapat diubah: hal itu akan mewarnai ulang setiap pembacaan sebelumnya.",
  "c.LilyGanttTable.critical_path": "jalur kritis",
  "c.LilyGanttTable.day": "Hari",
  "c.LilyGanttTable.month": "Bulan",
  "c.LilyGanttTable.no_dated_tasks_to_show":
    "Tidak ada tugas bertanggal untuk ditampilkan.",
  "c.LilyGanttTable.no_usable_dates": "Tidak ada tanggal yang dapat dipakai:",
  "c.LilyGanttTable.task": "Tugas",
  "c.LilyGanttTable.that_range_is_too_long":
    "Rentang itu terlalu panjang untuk satu kolom per {unit}; menampilkan satu per {unit2}.",
  "c.LilyGanttTable.time_scale": "Skala waktu",
  "c.LilyGanttTable.week": "Minggu",
  "c.LilyKanbanTable.move_to": "Pindahkan {title} ke",
  "c.LilyKanbanTable.no_tasks_yet": "Belum ada tugas.",
  "c.LilyKanbanTable.unknown_status": "Status tidak dikenal:",
  "c.LineRange.high": "Tinggi",
  "c.LineRange.low": "Rendah",
  "c.LineRange.planned_is_the_likely_cost":
    "Terencana adalah {money}, biaya yang mungkin. Kosongkan kolom untuk menghapusnya.",
  "c.LineRange.save_range": "Simpan rentang",
  "c.MilestoneCard.by": "Oleh",
  "c.MilestoneCard.by_hand_only": "Hanya manual",
  "c.MilestoneCard.complete": "Selesaikan",
  "c.MilestoneCard.completes": "Selesai saat",
  "c.MilestoneCard.done": "Selesai",
  "c.MilestoneCard.due": "Tenggat {due} · {kind}",
  "c.MilestoneCard.every_move_is_kept_in":
    "Setiap perpindahan disimpan dalam riwayat: tanggal lama, tanggal baru, alasan, dan siapa yang memindahkan.",
  "c.MilestoneCard.forecast": "· prakiraan {early_finish}",
  "c.MilestoneCard.from": "Dari",
  "c.MilestoneCard.loading": "Memuat…",
  "c.MilestoneCard.mark_done": "Tandai selesai",
  "c.MilestoneCard.move": "Pindahkan",
  "c.MilestoneCard.never_rescheduled": "Tidak pernah dijadwalkan ulang.",
  "c.MilestoneCard.new_due_date": "Tenggat baru",
  "c.MilestoneCard.no_reason_given": "tanpa alasan",
  "c.MilestoneCard.open": "Terbuka",
  "c.MilestoneCard.pick_the_tasks_that_must": "Pilih tugas yang harus selesai",
  "c.MilestoneCard.reason_required": "Alasan (wajib)",
  "c.MilestoneCard.reopen": "Buka kembali",
  "c.MilestoneCard.reschedule": "Jadwalkan ulang",
  "c.MilestoneCard.save": "Simpan",
  "c.MilestoneCard.slip_history": "Riwayat keterlambatan ({reschedules})",
  "c.MilestoneCard.tasks_it_waits_for": "Tugas yang ditunggunya",
  "c.MilestoneCard.tasks_this_milestone_waits_for":
    "Tugas yang ditunggu tonggak ini",
  "c.MilestoneCard.this_plan_has_no_tasks": "Rencana ini belum punya tugas.",
  "c.MilestoneCard.to": "Ke",
  "c.MilestoneCard.view_plan": "Lihat rencana",
  "c.MilestoneCard.when_all_those_tasks_are": "Saat semua tugas itu selesai",
  "c.MilestoneCard.why": "Mengapa",
  "c.NetworkDiagram.float": "{slice} → {slice2} · kelonggaran {total_float}",
  "c.NetworkDiagram.no_dependencies": "Tidak ada ketergantungan.",
  "c.NetworkDiagram.no_scheduled_tasks_to_draw":
    "Belum ada tugas terjadwal untuk digambar.",
  "c.NetworkDiagram.task_network_boxes_are_tasks":
    "Jaringan tugas: kotak adalah tugas, panah adalah ketergantungan",
  "c.NetworkDiagram.the_same_network_as_a":
    "Jaringan yang sama dalam bentuk daftar",
  "c.ObjectiveCard.add_a_key_result": "Tambah hasil kunci",
  "c.ObjectiveCard.add_key_result": "Tambah hasil kunci",
  "c.ObjectiveCard.check_in": "Pembaruan",
  "c.ObjectiveCard.confidence_0_100": "Keyakinan (0–100)",
  "c.ObjectiveCard.currency": "Mata uang",
  "c.ObjectiveCard.currency_e_g_gbp": "Mata uang (misalnya EUR, JPY, atau GBP)",
  "c.ObjectiveCard.decrease": "Turun",
  "c.CommandPalette.cmd_dashboard": "Dasbor",
  "c.CommandPalette.cmd_escalations": "Eskalasi",
  "c.CommandPalette.cmd_my_work": "Pekerjaan saya",
  "c.CommandPalette.cmd_new_plan": "Rencana baru",
  "c.CommandPalette.count_one": "{n} hasil",
  "c.CommandPalette.count_other": "{n} hasil",
  "c.CommandPalette.failed":
    "Pencarian rencana tidak tersedia saat ini. Perintah tetap berfungsi.",
  "c.CommandPalette.kind_command": "Perintah",
  "c.CommandPalette.kind_plan": "Rencana",
  "c.CommandPalette.kind_recent": "Terbaru",
  "c.CommandPalette.label": "Cari rencana dan perintah",
  "c.CommandPalette.none": "Tidak ada yang cocok",
  "c.CommandPalette.open": "Pencarian dan perintah",
  "c.CommandPalette.placeholder": "Ketik nama rencana atau perintah",
  "c.CommandPalette.title": "Pencarian dan perintah",
  "c.NavItems.portfolios": "Portofolio",
  "c.NavItems.projects": "Proyek",
  "c.NavItems.products": "Produk",
  "c.NavItems.programs": "Program",
  "c.NavItems.practices": "Praktik",
  "c.NavItems.processes": "Proses",
  "c.NavItems.purposes": "Tujuan",
  "c.NavItems.pathways": "Jalur",
  "c.PasteTasks.check": "Periksa dengan layanan",
  "c.PasteTasks.clear": "Bersihkan",
  "c.PasteTasks.col_due": "Tenggat",
  "c.PasteTasks.col_line": "Baris",
  "c.PasteTasks.col_notes": "Catatan",
  "c.PasteTasks.col_status": "Status",
  "c.PasteTasks.col_title": "Judul",
  "c.PasteTasks.create_one": "Buat 1 tugas",
  "c.PasteTasks.create_other": "Buat {n} tugas",
  "c.PasteTasks.done": "Selesai",
  "c.PasteTasks.failed": "Penempelan tidak berhasil.",
  "c.PasteTasks.help":
    "Salin sel dari lembar kerja dan tempel di sini: satu tugas per baris, dengan kolom Judul, Tenggat (yyyy-mm-dd), dan Catatan, dalam urutan itu. Tanggal menjadi tenggat tugas. Baris pertama yang menyebut nama kolom dilewati. Baris yang bermasalah ditampilkan beserta alasannya dan tidak disertakan.",
  "c.PasteTasks.label": "Baris yang ditempel",
  "c.PasteTasks.left_out": "Tidak disertakan: {why}",
  "c.PasteTasks.problem_beyond_limit": "lebih dari {max} baris",
  "c.PasteTasks.problem_date_bad":
    "“{value}” bukan tanggal; tulis dengan format yyyy-mm-dd",
  "c.PasteTasks.problem_no_title": "tanpa judul",
  "c.PasteTasks.problem_notes_long":
    "catatan lebih panjang dari {max} karakter",
  "c.PasteTasks.problem_same_as": "sama dengan baris {line}",
  "c.PasteTasks.problem_title_long": "judul lebih panjang dari {max} karakter",
  "c.PasteTasks.ready": "Siap",
  "c.PasteTasks.tally": "{good} siap, {left} tidak disertakan",
  "c.PasteTasks.title": "Tempel baris dari lembar kerja",
  "c.PasteTasks.would": "Yang akan terjadi",
  "c.ConfirmHost.cancel": "Batal",
  "c.ConfirmHost.ok": "OK",
  "c.ConfirmHost.title": "Harap konfirmasi",
  "c.ObjectiveCard.delete": "Hapus",
  "c.ObjectiveCard.direction": "Arah",
  "c.ObjectiveCard.edit_or_delete": "Ubah atau hapus",
  "c.ObjectiveCard.edit_or_delete_this_objective":
    "Ubah atau hapus sasaran ini",
  "c.ObjectiveCard.from": "{valueText} (dari {start_value})",
  "c.ObjectiveCard.increase": "Naik",
  "c.ObjectiveCard.it_has_no_measurable_key":
    "— belum punya hasil kunci yang terukur, jadi belum ada skor (bukan nol).",
  "c.ObjectiveCard.key_results_in_more_than":
    "Hasil kunci dalam lebih dari satu mata uang dinilai terpisah; mata uang tidak pernah dikonversi.",
  "c.ObjectiveCard.last_check_in": "· pembaruan terakhir {slice}",
  "c.ObjectiveCard.maintain": "Pertahankan",
  "c.ObjectiveCard.metric": "Metrik",
  "c.ObjectiveCard.no_key_results_yet": "Belum ada hasil kunci.",
  "c.ObjectiveCard.note": "Catatan",
  "c.ObjectiveCard.number": "Angka",
  "c.ObjectiveCard.percent": "Persen",
  "c.ObjectiveCard.period": "Periode",
  "c.ObjectiveCard.record": "Catat",
  "c.ObjectiveCard.save": "Simpan",
  "c.ObjectiveCard.start": "Mulai",
  "c.ObjectiveCard.target": "Target",
  "c.ObjectiveCard.the_baseline_metric_and_direction":
    "Baseline, metrik, dan arah tidak dapat diubah: kemajuan dari baseline yang bergeser bukanlah kemajuan.",
  "c.ObjectiveCard.title": "Judul",
  "c.ObjectiveCard.tolerance": "· toleransi ±{tolerance}",
  "c.ObjectiveCard.tolerance_2": "Toleransi",
  "c.ObjectiveCard.unit": "Satuan",
  "c.ObjectiveCard.value": "Nilai",
  "c.ObjectiveCard.yes_no": "Ya / tidak",
  "c.PlanForm.organization": "organisasi:…",
  "c.PlanForm.parent_plan_pid_uuid": "pid rencana induk (UUID)",
  "c.PlanForm.proj_2026": "PROJ-2026",
  "c.RaciMatrix.complete": "lengkap",
  "c.RaciMatrix.gaps": "Kesenjangan",
  "c.RaciMatrix.task": "Tugas",
  "c.RaciMatrix.who_does_what_raci_matrix":
    "Siapa melakukan apa (matriks RACI): siapa yang Responsible, Accountable, Consulted, dan Informed pada setiap tugas",
  "c.RiskInputs.adds_cost": "Menambah biaya",
  "c.RiskInputs.by_business_days": "sebesar (hari kerja)",
  "c.RiskInputs.currency": "Mata uang",
  "c.RiskInputs.delays_task": "Menunda tugas",
  "c.RiskInputs.gbp": "EUR",
  "c.RiskInputs.none": "Tidak ada",
  "c.RiskInputs.one_probability_covers_both_effects":
    "Satu probabilitas mencakup kedua dampak: risiko yang menunda tugas dan menambah biaya terjadi sekali dan melakukan keduanya. Kosongkan satu bagian untuk tidak menyertakannya.",
  "c.RiskInputs.probability": "Probabilitas (%)",
  "c.RiskInputs.save": "Simpan",
  "c.RiskInputs.saving": "Menyimpan…",
  "c.RiskInputs.simulation_inputs": "Masukan simulasi",
  "c.SCurve.actual": "Aktual",
  "c.SCurve.date": "Tanggal",
  "c.SCurve.earned": "Diperoleh",
  "c.SCurve.line_planned_value_to_earned":
    "Garis: nilai terencana ({from} hingga {to}). ● nilai yang diperoleh dan ■ biaya aktual, hanya pada hari yang tercatat ({snapshots} sejauh ini). Putus-putus: anggaran saat selesai, {money}.",
  "c.SCurve.no_curve_to_draw": "Tidak ada kurva untuk digambar.",
  "c.SCurve.not_recorded": "tidak tercatat",
  "c.SCurve.planned": "Terencana",
  "c.SCurve.s_curve_planned_value_over":
    "Kurva-S: nilai terencana dari waktu ke waktu, dengan nilai yang diperoleh dan biaya aktual yang tercatat",
  "c.SCurve.the_same_figures_as_a": "Angka yang sama dalam bentuk tabel",
  "c.VarianceTable.baselined": "Ber-baseline",
  "c.VarianceTable.duration": "Durasi Δ",
  "c.VarianceTable.finish": "Selesai Δ",
  "c.VarianceTable.now": "Sekarang",
  "c.VarianceTable.schedule_variance_against_the_baseli":
    "Selisih jadwal terhadap baseline, dalam hari kerja",
  "c.VarianceTable.standing": "Kedudukan",
  "c.VarianceTable.start": "Mulai Δ",
  "c.VarianceTable.task": "Tugas",
  "c.VarianceTable.the_plan_s_finish_is": "Penyelesaian rencana tidak berubah.",
  "c.VarianceTable.the_plan_s_own_finish":
    "Penyelesaian rencana itu sendiri tidak dapat dibandingkan.",
  "p.announcements.announcements": "Pengumuman",
  "p.announcements.announcements_ppm": "Pengumuman — PPM",
  "p.announcements.expires_optional": "Kedaluwarsa (opsional)",
  "p.announcements.important": "Penting",
  "p.announcements.info": "Info",
  "p.announcements.loading": "Memuat…",
  "p.announcements.message": "Pesan",
  "p.announcements.nothing_has_been_announced": "Belum ada yang diumumkan.",
  "p.announcements.pin_to_the_top": "Sematkan di atas",
  "p.announcements.pinned": "· Disematkan",
  "p.announcements.publish": "Terbitkan",
  "p.announcements.publish_an_announcement": "Terbitkan pengumuman",
  "p.announcements.publishing_needs_the_announcements_p":
    "Menerbitkan memerlukan izin pengumuman.",
  "p.announcements.retract": "Tarik kembali",
  "p.announcements.severity": "Tingkat keparahan",
  "p.announcements.title": "Judul",
  "p.announcements.urgent": "Mendesak",
  "p.auditor.action": "Tindakan",
  "p.auditor.actor": "Pelaku",
  "p.auditor.actor_less_recorded_actions": "Tindakan tercatat tanpa pelaku:",
  "p.auditor.audit_trail": "Jejak audit",
  "p.auditor.bearer_sub": "sub bearer",
  "p.auditor.detail": "Rincian",
  "p.auditor.e_g_merged": "mis. merged",
  "p.auditor.entity": "Entitas",
  "p.auditor.evidence_pack_csv": "Paket bukti (CSV)",
  "p.auditor.filter": "Filter",
  "p.auditor.findings": "Temuan",
  "p.auditor.no_audit_rows_match": "Tidak ada baris audit yang cocok.",
  "p.auditor.no_findings": "Tidak ada temuan.",
  "p.auditor.ppm": "{t} — PPM",
  "p.auditor.rows_distinct_actors_actor_less":
    "{returned} baris · {distinct_actors} pelaku berbeda · {actorless} tanpa pelaku",
  "p.auditor.rule": "Aturan",
  "p.auditor.submitted_without_a_verified_token":
    "(diajukan tanpa token terverifikasi)",
  "p.auditor.when": "Kapan",
  "p.automations.action": "tindakan",
  "p.automations.action_2": "Tindakan",
  "p.automations.add_label": "tambah label",
  "p.automations.any": "apa saja",
  "p.automations.assign": "tugaskan",
  "p.automations.assignee": "penanggung jawab",
  "p.automations.automation": "otomasi",
  "p.automations.automations_ppm": "Otomasi — PPM",
  "p.automations.blank_every_plan": "kosong = setiap rencana",
  "p.automations.cancel": "Batal",
  "p.automations.claim_based_so_a_deadline":
    "Berbasis klaim, sehingga tenggat dipicu tepat sekali meskipun penjadwal dan penyapuan manual berebut.",
  "p.automations.configure_a_rule_once_as":
    "Atur aturan sekali; saat item berpindah di papan, platform menerapkannya. Aturan tidak pernah menghalangi perpindahan yang memicunya — kegagalan dicatat sebagai eksekusi gagal, bukan pembatalan.",
  "p.automations.delete": "Hapus",
  "p.automations.detail": "Rincian",
  "p.automations.disable": "Nonaktifkan",
  "p.automations.due": "Tenggat",
  "p.automations.enable": "Aktifkan",
  "p.automations.every_plan": "setiap rencana",
  "p.automations.expire_review": "kedaluwarsakan tinjauan",
  "p.automations.from": "dari",
  "p.automations.in_calendar_days": "dalam hari kalender",
  "p.automations.label": "label",
  "p.automations.message": "pesan",
  "p.automations.name": "nama",
  "p.automations.name_2": "Nama",
  "p.automations.no_rules_configured": "Belum ada aturan yang diatur.",
  "p.automations.nothing_has_fired_yet": "Belum ada yang terpicu.",
  "p.automations.nothing_pending": "Tidak ada yang tertunda.",
  "p.automations.notify": "beri tahu",
  "p.automations.off": "mati",
  "p.automations.on": "hidup",
  "p.automations.operator": "operator",
  "p.automations.outcome": "Hasil",
  "p.automations.person_uuid": "person:<uuid>",
  "p.automations.plan_pid": "pid rencana",
  "p.automations.plan_stage_changed": "tahap rencana berubah",
  "p.automations.recipient": "penerima",
  "p.automations.review_submitted": "tinjauan diajukan",
  "p.automations.rules": "Aturan",
  "p.automations.runs": "Eksekusi",
  "p.automations.save_rule": "Simpan aturan",
  "p.automations.schedule_action": "jadwalkan tindakan",
  "p.automations.scope": "Cakupan",
  "p.automations.set_and_forget": "Atur lalu lupakan",
  "p.automations.set_task_status": "atur status tugas",
  "p.automations.source": "Sumber",
  "p.automations.state": "Keadaan",
  "p.automations.status": "status",
  "p.automations.subject": "Subjek",
  "p.automations.sweep_due_actions": "Sapu tindakan jatuh tempo",
  "p.automations.task_moved": "tugas dipindahkan",
  "p.automations.then": "maka",
  "p.automations.to": "ke",
  "p.automations.trigger": "pemicu",
  "p.automations.trigger_2": "Pemicu",
  "p.automations.when": "Kapan",
  "p.automations.workflow_automation": "Otomasi alur kerja",
  "p.board.amber": "kuning",
  "p.board.amount": "Jumlah",
  "p.board.benefit_realizations": "realisasi manfaat",
  "p.board.board_pack": "Paket dewan",
  "p.board.decision": "Keputusan",
  "p.board.decisions_in_window": "Keputusan dalam jendela",
  "p.board.green": "hijau",
  "p.board.investment_decisions": "Keputusan investasi",
  "p.board.kind": "Jenis",
  "p.board.milestones_completed": "tonggak selesai",
  "p.board.no_decisions_in_the_window":
    "Tidak ada keputusan dalam jendela ini.",
  "p.board.no_investment_decisions_recorded":
    "Belum ada keputusan investasi yang tercatat.",
  "p.board.no_snapshots_captured_yet": "Belum ada snapshot yang diambil.",
  "p.board.open_exposure": "Eksposur terbuka",
  "p.board.portfolios": "Portofolio",
  "p.board.ppm": "{t} — PPM",
  "p.board.realized_in_window": "Terealisasi dalam jendela:",
  "p.board.red_portfolios": "portofolio merah",
  "p.board.subject": "Subjek",
  "p.board.take_snapshot_now": "Ambil snapshot sekarang",
  "p.board.taken": "Diambil",
  "p.board.tranches_released": "tahap pencairan dirilis",
  "p.board.trends": "Tren",
  "p.board.unattributed_events":
    "({unattributed_events} peristiwa tanpa atribusi)",
  "p.board.what": "Apa",
  "p.board.when": "Kapan",
  "p.board.window": "jendela {from} → {to}",
  "p.board.work_items": "Item pekerjaan",
  "p.calendar.all": "(semua)",
  "p.calendar.done": "· selesai",
  "p.calendar.kind": "Jenis",
  "p.calendar.no_milestones_match": "Tidak ada tonggak yang cocok.",
  "p.calendar.ppm": "{t} — PPM",
  "p.capacity.capacity_ppm": "Kapasitas — PPM",
  "p.capacity.from": "dari",
  "p.capacity.hatched_extension_proposed_not_count":
    "perpanjangan berarsir: diusulkan, tidak dihitung",
  "p.capacity.legend": "Legenda",
  "p.capacity.over": "melebihi",
  "p.capacity.over_capacity_solid_bar_firm":
    "melebihi kapasitas (batang penuh: pemesanan pasti; garis: kapasitas yang berlaku)",
  "p.capacity.proposed": "Diusulkan",
  "p.capacity.to": "hingga",
  "p.capacity.window": "jendela: {from} → {to}",
  "p.capacity.within_capacity": "dalam kapasitas",
  "p.capacity.within_capacity_but_committing_the":
    "dalam kapasitas, tetapi menetapkan pemesanan yang diusulkan akan melampauinya",
  "p.ceo.all_announcements": "Semua pengumuman",
  "p.ceo.all_key_measures": "Semua ukuran utama",
  "p.ceo.all_milestones": "Semua tonggak",
  "p.ceo.and_more": "dan {more} lagi",
  "p.ceo.announcements": "Pengumuman",
  "p.ceo.ceo_dashboard": "Dasbor CEO",
  "p.ceo.ceo_dashboard_ppm": "Dasbor CEO — PPM",
  "p.ceo.decision_log": "Log keputusan",
  "p.ceo.escalated_risks": "risiko dieskalasi",
  "p.ceo.every_kpi_is_on_target": "● Semua KPI sesuai target.",
  "p.ceo.executive_detail": "Rincian eksekutif",
  "p.ceo.key_measures_needing_a_look": "Ukuran utama yang perlu dicermati",
  "p.ceo.loading": "Memuat…",
  "p.ceo.milestones_to_watch": "Tonggak yang perlu dipantau",
  "p.ceo.no_data_yet": "Belum ada data",
  "p.ceo.no_decisions_recorded_yet": "Belum ada keputusan yang tercatat.",
  "p.ceo.no_kpis_defined_yet": "Belum ada KPI yang ditetapkan.",
  "p.ceo.no_open_milestones": "Tidak ada tonggak terbuka.",
  "p.ceo.no_portfolio_is_over_budget":
    "● Tidak ada portofolio yang melebihi anggaran",
  "p.ceo.nothing_has_been_announced": "Belum ada yang diumumkan.",
  "p.ceo.over_budget": "▲ Melebihi anggaran: {join}",
  "p.ceo.overdue_milestones": "{overdueMilestones} tonggak terlambat",
  "p.ceo.portfolio_health": "Kesehatan portofolio",
  "p.ceo.recent_decisions": "Keputusan terbaru",
  "p.ceo.red_amber_green": "■ {red} merah · ▲ {amber} kuning · ● {green} hijau",
  "p.ceo.risk_and_money": "Risiko dan uang",
  "p.ceo.risk_register": "Register risiko",
  "p.compliance.compliance_risk_register": "Register risiko kepatuhan",
  "p.compliance.conformance_findings": "Temuan kesesuaian",
  "p.compliance.detail": "Rincian",
  "p.compliance.exposure": "Eksposur",
  "p.compliance.gate_review_recency_threshold_calend":
    "ambang kebaruan tinjauan gerbang: {review_days} hari kalender",
  "p.compliance.item": "Item",
  "p.compliance.no_compliance_risks_recorded":
    "Belum ada risiko kepatuhan yang tercatat.",
  "p.compliance.no_conformance_findings": "Tidak ada temuan kesesuaian.",
  "p.compliance.open_exposure": "{note} · eksposur terbuka {open_exposure}",
  "p.compliance.owner": "Pemilik",
  "p.compliance.ppm": "{t} — PPM",
  "p.compliance.risk": "Risiko",
  "p.compliance.rule": "Aturan",
  "p.compliance.status": "Status",
  "p.cost_rates.a_person": "Seorang individu",
  "p.cost_rates.a_role": "Sebuah peran",
  "p.cost_rates.add_a_rate_table": "Tambah tabel tarif",
  "p.cost_rates.add_rate": "Tambah tarif",
  "p.cost_rates.add_table": "Tambah tabel",
  "p.cost_rates.cost_rates": "Tarif biaya",
  "p.cost_rates.cost_rates_are_finance_data":
    "Tarif biaya adalah data keuangan. Anda tidak memiliki akses ke sana.",
  "p.cost_rates.cost_rates_ppm": "Tarif biaya — PPM",
  "p.cost_rates.currency": "Mata uang",
  "p.cost_rates.effective_from": "Berlaku mulai",
  "p.cost_rates.for": "Untuk",
  "p.cost_rates.from": "Dari",
  "p.cost_rates.name": "Nama",
  "p.cost_rates.no_rate_tables_yet": "Belum ada tabel tarif.",
  "p.cost_rates.no_rate_yet_work_by":
    "Belum ada tarif: pekerjaan oleh {v} ini tidak dihitung biayanya.",
  "p.cost_rates.per_hour": "Per jam",
  "p.cost_rates.person_person_reference": "Orang (referensi person:…)",
  "p.cost_rates.role": "Peran",
  "p.cost_risk.band_for_lines_with_no":
    "Pita untuk baris tanpa perkiraan (± %)",
  "p.cost_risk.distribution": "Distribusi",
  "p.cost_risk.iterations": "Iterasi",
  "p.cost_risk.pert": "PERT",
  "p.cost_risk.portfolio_cost_risk": "Risiko biaya portofolio",
  "p.cost_risk.portfolio_cost_risk_ppm": "Risiko biaya portofolio — PPM",
  "p.cost_risk.resources": "Sumber daya",
  "p.cost_risk.run": "Jalankan",
  "p.cost_risk.running": "Berjalan…",
  "p.cost_risk.seed": "Seed",
  "p.cost_risk.shows_people_instead": "menampilkan orang sebagai gantinya.",
  "p.cost_risk.triangular": "Segitiga",
  "p.cost_risk.what_all_your_plans_together":
    "Berapa biaya seluruh rencana Anda bersama-sama, mengingat rentang tiap baris anggaran dan risiko yang menambah biaya. Setiap mata uang berdiri sendiri; tidak pernah dijumlahkan.",
  "p.dashboard.dashboard_ppm": "Dasbor — PPM",
  "p.display.display": "Tampilan",
  "p.display.display_ppm": "Tampilan — PPM",
  "p.display.dyslexia_friendly_font": "Font ramah disleksia",
  "p.display.go_back_to_the_default": "Kembali ke bawaan untuk peran saya",
  "p.display.how_the_app_looks_and":
    "Bagaimana aplikasi terlihat dan terbaca. Ini tidak pernah mengubah apa yang dapat Anda lakukan.",
  "p.display.keeps_the_everyday_pages_in":
    "Menjaga halaman sehari-hari tetap di depan (Pekerjaan saya, Rencana, Papan, Kalender, Tur) dan menaruh sisanya di balik tautan “Lainnya”. Setiap halaman tetap ada dan berfungsi.",
  "p.display.sign_in_to_keep_these":
    "Masuk untuk menyimpan pengaturan ini di akun Anda, bukan hanya di perangkat ini.",
  "p.display.simple_view": "Tampilan sederhana",
  "p.display.this_is_the_default_for":
    "Ini adalah bawaan untuk peran Anda: {v}.",
  "p.display.use_simple_view": "Gunakan tampilan sederhana",
  "p.display.sections_title": "Bagian di bilah atas",
  "p.display.sections_help":
    "Hapus centang pada sebuah bagian untuk menyembunyikannya dari bilah atas. Bagian itu tetap dapat dijangkau lewat alamatnya, dan Anda dapat menampilkannya lagi di sini. Tidak ada yang disimpan sampai Anda menyembunyikan satu, sehingga bagian yang ditambahkan kemudian tetap sampai kepada Anda.",
  "p.display.sections_legend": "Tampilkan bagian ini",
  "p.display.sections_hidden_count_one": "{n} bagian disembunyikan",
  "p.display.sections_hidden_count_other": "{n} bagian disembunyikan",
  "p.display.sections_none_hidden": "Tidak ada bagian yang disembunyikan.",
  "p.display.sections_show_all": "Tampilkan semua bagian",
  "p.display.week_start_title": "Hari pertama dalam seminggu",
  "p.display.week_start_help":
    "Hari tempat kalender memulai tiap minggu. Secara bawaan mengikuti bahasa yang Anda pilih; nomor minggu tetap minggu ISO, yang dimulai pada hari Senin.",
  "p.display.week_start_label": "Minggu dimulai pada",
  "p.display.week_start_locale": "Hari yang lazim untuk bahasa ini",
  "p.display.week_start_mon": "Senin",
  "p.display.week_start_sun": "Minggu",
  "p.display.week_start_sat": "Sabtu",
  "p.display.use_the_dyslexia_friendly_font": "Gunakan font ramah disleksia",
  "p.display.uses_the_opendyslexic_typeface_with":
    "Memakai huruf OpenDyslexic, dengan jarak lebih lebar antarhuruf, antarkata, dan antarbaris.",
  "p.display.you_chose_this": "Anda memilih ini.",
  "p.documents.controlled_documents": "Dokumen terkendali",
  "p.documents.create_document": "Buat dokumen",
  "p.documents.document": "Dokumen",
  "p.documents.documents": "Dokumen",
  "p.documents.documents_ppm": "Dokumen — PPM",
  "p.documents.first_draft": "Draf pertama",
  "p.documents.in_force": "Berlaku",
  "p.documents.loading": "Memuat…",
  "p.documents.new_document": "Dokumen baru",
  "p.documents.no_documents_yet": "Belum ada dokumen.",
  "p.documents.pid.approver_s_id": "ID penyetuju",
  "p.documents.pid.date": "Tanggal",
  "p.documents.pid.document": "Dokumen",
  "p.documents.pid.documents": "← Dokumen",
  "p.documents.pid.look_up": "Cari",
  "p.documents.pid.nothing_was_in_force_on":
    "Tidak ada yang berlaku pada {on}.",
  "p.documents.pid.person_id": "ID orang",
  "p.documents.pid.save_draft": "Simpan draf",
  "p.documents.pid.send_back": "Kembalikan",
  "p.documents.pid.text": "Teks",
  "p.documents.pid.this_version_s_text_is":
    "Teks versi ini sudah final. Perubahan masuk ke draf berikutnya.",
  "p.documents.pid.version": "Versi {version}",
  "p.documents.pid.version_was_in_force_on":
    "Versi {version} berlaku pada {on}.",
  "p.documents.pid.versions": "Versi",
  "p.documents.pid.what_changed": "Apa yang berubah",
  "p.documents.pid.what_was_in_force_on": "Apa yang berlaku pada suatu tanggal",
  "p.documents.policies_and_procedures_a_new":
    "Kebijakan dan prosedur. Versi baru ditinjau dan disetujui sebelum berlaku, dan versi yang berlaku tidak pernah berubah.",
  "p.documents.title": "Judul",
  "p.documents.versions": "Versi",
  "p.engineering.assignee": "Penanggung jawab",
  "p.engineering.blocked_calendar_days": "Terhambat (hari kalender)",
  "p.engineering.blocked_work": "Pekerjaan terhambat",
  "p.engineering.declared_cause_failure_rate":
    "tingkat kegagalan berpenyebab dinyatakan",
  "p.engineering.delivery_links": "Tautan pengiriman",
  "p.engineering.deploys_mo": "deploy ({window_months} bln)",
  "p.engineering.devops_metrics": "Metrik DevOps",
  "p.engineering.environment": "Lingkungan",
  "p.engineering.incidents": "insiden",
  "p.engineering.incidents_unresolved_counted_never_t":
    "{unresolved_incidents} insiden belum terselesaikan (dihitung, tidak pernah diukur waktunya).",
  "p.engineering.item": "Item",
  "p.engineering.items_carry_no_moscow_tag":
    "{untagged} item tidak memiliki tag moscow.",
  "p.engineering.median_recovery_h": "median pemulihan (jam)",
  "p.engineering.moscow_scope": "Lingkup MoSCoW",
  "p.engineering.no_deploy_events_ingested_yet":
    "Belum ada peristiwa deploy yang diserap.",
  "p.engineering.no_externally_tracked_items":
    "Tidak ada item yang dilacak secara eksternal.",
  "p.engineering.nothing_blocked_ship_it":
    "Tidak ada yang terhambat. Rilis saja.",
  "p.engineering.ppm": "{t} — PPM",
  "p.engineering.releases": "Rilis",
  "p.engineering.task": "Tugas",
  "p.engineering.trackers": "Pelacak",
  "p.engineering.untracked": "Tidak dilacak:",
  "p.engineering.version": "Versi",
  "p.engineering.when": "Kapan",
  "p.executive.actor": "Pelaku",
  "p.executive.aligned": "Selaras",
  "p.executive.as_of": "per {as_of} · {derivation}",
  "p.executive.benefits": "Manfaat",
  "p.executive.benefits_realization": "Realisasi manfaat",
  "p.executive.calendar_days_since_update": "Hari kalender sejak pembaruan",
  "p.executive.collection": "Koleksi",
  "p.executive.currency": "Mata uang",
  "p.executive.decision": "Keputusan",
  "p.executive.decision_log": "Log keputusan",
  "p.executive.escalated_risks": "Risiko dieskalasi",
  "p.executive.kind": "Jenis",
  "p.executive.largest_unaligned_items": "Item tidak selaras terbesar",
  "p.executive.members": "Anggota",
  "p.executive.no_decisions_recorded_yet": "Belum ada keputusan yang tercatat.",
  "p.executive.no_target": "tanpa target",
  "p.executive.non_financial_only": "hanya nonkeuangan ({non_financial})",
  "p.executive.open_exposure": "Eksposur terbuka",
  "p.executive.overdue_milestones": "Tonggak terlambat",
  "p.executive.overrun_currencies": "Mata uang yang melampaui anggaran",
  "p.executive.planned_with_no_objective": "direncanakan tanpa sasaran",
  "p.executive.portfolio": "Portofolio",
  "p.executive.portfolio_health": "Kesehatan portofolio",
  "p.executive.ppm": "{t} — PPM",
  "p.executive.realization": "Realisasi",
  "p.executive.realized": "Terealisasi",
  "p.executive.status": "Status",
  "p.executive.strategic_alignment": "Keselarasan strategis",
  "p.executive.subject": "Subjek",
  "p.executive.target": "Target",
  "p.executive.total": "Total",
  "p.executive.unaligned": "Tidak selaras",
  "p.executive.unaligned_spend": "Belanja tidak selaras",
  "p.executive.when": "Kapan",
  "p.financials.actual": "Aktual",
  "p.financials.as_of": "per {as_of} · {note}",
  "p.financials.currency": "Mata uang",
  "p.financials.currency_exposure": "Eksposur mata uang",
  "p.financials.lines": "Baris",
  "p.financials.no_budget_lines_recorded_yet":
    "Belum ada baris anggaran yang tercatat.",
  "p.financials.planned": "Terencana",
  "p.financials.ppm": "{t} — PPM",
  "p.financials.remaining": "Sisa",
  "p.financials.work_items": "Item pekerjaan",
  "p.gantt.main_x": "{t} — Main X",
  "p.ideas.convert_target": "Target konversi",
  "p.ideas.ideas_ppm": "Ide — PPM",
  "p.kpis.all": "Semua",
  "p.kpis.blank_for_the_whole_portfolio": "kosong untuk seluruh portofolio",
  "p.kpis.daily": "Harian",
  "p.kpis.define": "Tetapkan",
  "p.kpis.define_a_key_measure_kpi": "Tetapkan ukuran utama (KPI)",
  "p.kpis.higher_is_better": "Lebih tinggi lebih baik",
  "p.kpis.key_measures_kpis": "Ukuran utama (KPI)",
  "p.kpis.kpis_by_status": "KPI menurut status",
  "p.kpis.kpis_ppm": "KPI — PPM",
  "p.kpis.loading": "Memuat…",
  "p.kpis.lower_is_better": "Lebih rendah lebih baik",
  "p.kpis.monthly": "Bulanan",
  "p.kpis.name": "Nama",
  "p.kpis.no_data_yet": "Belum ada data",
  "p.kpis.not_set": "Belum diatur",
  "p.kpis.off_target": "Di luar target",
  "p.kpis.on_target": "Sesuai target",
  "p.kpis.percent_defects_gbp": "persen, cacat, EUR…",
  "p.kpis.plan_optional_pid": "Rencana (pid opsional)",
  "p.kpis.quarterly": "Kuartalan",
  "p.kpis.read_how_often": "Seberapa sering dibaca",
  "p.kpis.red_line": "Garis merah",
  "p.kpis.show": "Tampilkan",
  "p.kpis.something_you_track_over_time":
    "Sesuatu yang Anda lacak dari waktu ke waktu terhadap target, sehingga Anda dapat melihat sejak dini apakah berada di jalur yang benar.",
  "p.kpis.standing_measures_green_is_at":
    "Ukuran berkelanjutan. Hijau berarti sama dengan atau lebih baik dari target; merah berarti sama dengan atau lebih buruk dari garis merah; kuning berada di antaranya. KPI yang belum pernah dibaca menampilkan “Belum ada data”.",
  "p.kpis.target": "Target",
  "p.kpis.the_red_line_must_be":
    "Garis merah harus berada di sisi yang lebih buruk dari target. Skalakan nilai menjadi bilangan bulat (basis poin, sen).",
  "p.kpis.unit": "Satuan",
  "p.kpis.watch": "Pantau",
  "p.kpis.weekly": "Mingguan",
  "p.kpis.which_way_is_good": "Arah yang baik",
  "p.layout.display": "Tampilan",
  "p.layout.more": "Lainnya",
  "p.layout.simple_view": "Tampilan sederhana: {v}",
  "p.lifecycle.as_of": "per {as_of}",
  "p.lifecycle.in_an_unrecognized_phase": "dalam fase yang tidak dikenal",
  "p.lifecycle.lifecycle": "Siklus hidup",
  "p.lifecycle.lifecycle_ppm": "Siklus hidup — PPM",
  "p.lifecycle.live": "Aktif",
  "p.lifecycle.live_items_one": "{total} item aktif · ambang macet {stall}",
  "p.lifecycle.live_items_other": "{total} item aktif · ambang macet {stall}",
  "p.lifecycle.phase": "Fase",
  "p.lifecycle.share": "Porsi",
  "p.lifecycle.stalled": "Macet",
  "p.lifecycle.where_everything_sits_across_the":
    "Di mana setiap hal berada dalam siklus hidup tantangan, sehingga fase berikutnya tidak pernah mengejutkan. Macet menghitung item yang berada dalam satu fase lebih lama dari ambang.",
  "p.me.calendar.add": "Tambah",
  "p.me.calendar.at_least_one_weekday_must":
    "Setidaknya satu hari kerja harus aktif.",
  "p.me.calendar.from": "Dari",
  "p.me.calendar.loading": "Memuat…",
  "p.me.calendar.most_i_can_commit": "Maksimum yang dapat saya tetapkan (%)",
  "p.me.calendar.my_calendar": "Kalender saya",
  "p.me.calendar.my_calendar_ppm": "Kalender saya — PPM",
  "p.me.calendar.my_days_off": "Hari libur saya",
  "p.me.calendar.my_work": "← Pekerjaan saya",
  "p.me.calendar.none_set": "Belum diatur.",
  "p.me.calendar.note": "Catatan",
  "p.me.calendar.part_time_spells_and_leave": "Masa paruh waktu dan cuti",
  "p.me.calendar.remove": "Hapus",
  "p.me.calendar.save": "Simpan",
  "p.me.calendar.saved": "Tersimpan.",
  "p.me.calendar.the_days_i_work": "Hari saya bekerja",
  "p.me.calendar.the_most_of_your_time":
    "Porsi waktu terbesar yang dapat Anda tetapkan di antara dua tanggal: 60 untuk masa paruh waktu, 0 untuk cuti. Di luar tanggal ini nilainya 100. Kapasitas dibaca hari demi hari, sehingga masa singkat tidak pernah hilang dalam rata-rata.",
  "p.me.calendar.to": "{from} hingga {to}: {max_percent}%{v}",
  "p.me.calendar.to_2": "Hingga",
  "p.me.due": "· tenggat {due_date}",
  "p.me.in_plans_you_cannot_read":
    "· {omitted_unreadable} dalam rencana yang tidak dapat Anda baca",
  "p.me.loading": "Memuat…",
  "p.me.my_calendar": "Kalender saya",
  "p.me.my_work": "Pekerjaan saya",
  "p.me.my_work_ppm": "Pekerjaan saya — PPM",
  "p.me.nothing_assigned_to_you_enjoy":
    "Tidak ada yang ditugaskan kepada Anda. Nikmatilah.",
  "p.me.open": "{today} ({tz}) · {total} terbuka",
  "p.me.showing_the_first_500": "· menampilkan 500 pertama",
  "p.me.the_days_you_work_so":
    "— hari Anda bekerja, sehingga tanggal Anda jujur.",
  "p.me.your_sign_in_identity_cannot":
    "Identitas masuk Anda tidak dapat dicocokkan dengan penugasan tugas, sehingga belum ada yang dapat didaftar di sini.",
  "p.milestones.a_milestone_is_a_dated":
    "Tonggak adalah janji bertanggal. Kaitkan dengan tugas yang ditunggunya dan tonggak tercapai saat tugas itu selesai, dengan prakiraan dari jadwal; biarkan tak terkait dan seseorang menandainya selesai.",
  "p.milestones.achieved": "Tercapai",
  "p.milestones.all": "Semua",
  "p.milestones.due_soon": "Segera jatuh tempo",
  "p.milestones.forecast": "Prakiraan:",
  "p.milestones.late": "Terlambat",
  "p.milestones.loading": "Memuat…",
  "p.milestones.milestones": "Tonggak",
  "p.milestones.milestones_by_status": "Tonggak menurut status",
  "p.milestones.milestones_ppm": "Tonggak — PPM",
  "p.milestones.show": "Tampilkan",
  "p.milestones.upcoming": "Akan datang",
  "p.objectives.objectives_ppm": "Sasaran — PPM",
  "p.objectives.period_e_g_2026_h2": "Periode (mis. 2026-H2)",
  "p.okrs.2026_q4_blank_for_all": "2026-Q4 (kosong untuk semua)",
  "p.okrs.a_goal_with_a_few":
    "Sebuah tujuan, dengan beberapa hasil terukur yang menunjukkan apakah tujuan itu tercapai.",
  "p.okrs.create": "Buat",
  "p.okrs.goals_and_key_results_okrs": "Tujuan dan hasil kunci (OKR)",
  "p.okrs.loading": "Memuat…",
  "p.okrs.new_objective": "Sasaran baru",
  "p.okrs.objectives_and_key_results_scores":
    "Sasaran dan hasil kunci. Skor dihitung dari pembaruan setiap kali Anda melihat; tidak ada yang disimpan yang dapat menjadi usang.",
  "p.okrs.okrs_ppm": "OKR — PPM",
  "p.okrs.period": "Periode",
  "p.okrs.show": "Tampilkan",
  "p.okrs.title": "Judul",
  "p.onboarding.a_role_by_role_first":
    'Panduan jam pertama untuk setiap peran. Setiap langkah menyebut halaman tujuannya. Ini urutan yang disarankan, bukan klaim terukur "Anda akan produktif dalam N menit" — belum ada yang mengukur waktunya.',
  "p.onboarding.onboarding_ppm": "Orientasi — PPM",
  "p.onboarding.operator_onboarding": "Orientasi operator",
  "p.plans.main_x": "{t} — Main X",
  "p.plans.merge.ppm": "{t} — PPM",
  "p.plans.new.main_x": "{t} — Main X",
  "p.plans.pid.board.action": "tindakan",
  "p.plans.pid.board.add_note": "Tambah catatan",
  "p.plans.pid.board.add_sprint": "Tambah sprint",
  "p.plans.pid.board.add_task": "Tambah tugas",
  "p.plans.pid.board.board": "Papan",
  "p.plans.pid.board.burndown": "Burndown — {name}",
  "p.plans.pid.board.convert_to_task": "Ubah menjadi tugas",
  "p.plans.pid.board.created_moves_blocked_now":
    "{length} dibuat · {length2} perpindahan · {length3} terhambat sekarang",
  "p.plans.pid.board.date": "Tanggal",
  "p.plans.pid.board.feedback": "umpan balik",
  "p.plans.pid.board.improve": "perbaiki",
  "p.plans.pid.board.new_task_title": "Judul tugas baru",
  "p.plans.pid.board.no_notes_for_this_sprint":
    "Belum ada catatan untuk sprint ini.",
  "p.plans.pid.board.none": "(tidak ada)",
  "p.plans.pid.board.note": "Catatan",
  "p.plans.pid.board.note_body": "Isi catatan",
  "p.plans.pid.board.note_category": "Kategori catatan",
  "p.plans.pid.board.points_done": "Poin selesai",
  "p.plans.pid.board.ppm": "{t} — PPM",
  "p.plans.pid.board.pts": "poin",
  "p.plans.pid.board.remaining_of": "Sisa dari {total_tasks}",
  "p.plans.pid.board.retro_feedback_notes": "Catatan retro / umpan balik",
  "p.plans.pid.board.sprint": "Sprint",
  "p.plans.pid.board.sprint_end": "Akhir sprint",
  "p.plans.pid.board.sprint_name": "Nama sprint",
  "p.plans.pid.board.sprint_start": "Awal sprint",
  "p.plans.pid.board.sprints": "Sprint",
  "p.plans.pid.board.standup_last_24h": "Standup (24 jam terakhir)",
  "p.plans.pid.board.story_points": "Poin cerita",
  "p.plans.pid.board.task_created": "→ tugas dibuat",
  "p.plans.pid.board.tasks_done": "Tugas selesai",
  "p.plans.pid.board.unpointed": "Tanpa poin",
  "p.plans.pid.board.velocity": "Velocity",
  "p.plans.pid.board.went_well": "berjalan baik",
  "p.plans.pid.calendar.at_least_one_weekday_must":
    "Setidaknya satu hari kerja harus aktif.",
  "p.plans.pid.calendar.critical_path": "← Jalur kritis",
  "p.plans.pid.calendar.holidays": "Hari libur",
  "p.plans.pid.calendar.loading": "Memuat…",
  "p.plans.pid.calendar.my_calendar": "Kalender saya",
  "p.plans.pid.calendar.save_calendar": "Simpan kalender",
  "p.plans.pid.calendar.saved": "Tersimpan.",
  "p.plans.pid.calendar.their_work_then_happens_only":
    "); pekerjaan mereka lalu hanya berlangsung pada hari kedua kalender bekerja.",
  "p.plans.pid.calendar.working_calendar": "Kalender kerja",
  "p.plans.pid.calendar.working_calendar_ppm": "Kalender kerja — PPM",
  "p.plans.pid.calendar.working_weekdays": "Hari kerja",
  "p.plans.pid.cost_risk.band_for_lines_with_no":
    "Pita untuk baris tanpa perkiraan (± %)",
  "p.plans.pid.cost_risk.cost_and_schedule_together":
    "Biaya dan jadwal bersama",
  "p.plans.pid.cost_risk.cost_risk": "Risiko biaya",
  "p.plans.pid.cost_risk.cost_risk_ppm": "Risiko biaya — PPM",
  "p.plans.pid.cost_risk.distribution": "Distribusi",
  "p.plans.pid.cost_risk.iterations": "Iterasi",
  "p.plans.pid.cost_risk.many_what_if_runs_of":
    "Banyak simulasi bagaimana-jika atas anggaran menunjukkan rentang biaya akhir dan seberapa mungkin masing-masing.",
  "p.plans.pid.cost_risk.pert": "PERT",
  "p.plans.pid.cost_risk.plan": "← Rencana",
  "p.plans.pid.cost_risk.run": "Jalankan",
  "p.plans.pid.cost_risk.running": "Berjalan…",
  "p.plans.pid.cost_risk.seed": "Seed",
  "p.plans.pid.cost_risk.triangular": "Segitiga",
  "p.plans.pid.cost_risk.whole_portfolio": "Seluruh portofolio",
  "p.plans.pid.costs.a_recorded_cost_keeps_the":
    "Biaya yang tercatat mempertahankan tarif saat dihargai; perubahan tarif kemudian tidak pernah menulis ulang biaya itu.",
  "p.plans.pid.costs.add_an_expense": "Tambah pengeluaran",
  "p.plans.pid.costs.add_expense": "Tambah pengeluaran",
  "p.plans.pid.costs.all_at_the_end": "Semua di akhir",
  "p.plans.pid.costs.all_at_the_start": "Semua di awal",
  "p.plans.pid.costs.amount": "Jumlah",
  "p.plans.pid.costs.an_expense_of_so_it":
    "▲ Pengeluaran sebesar {money} {reason}, sehingga tidak termasuk dalam periode.",
  "p.plans.pid.costs.choose": "Pilih…",
  "p.plans.pid.costs.cost_rates": "Tarif biaya",
  "p.plans.pid.costs.costs": "Biaya",
  "p.plans.pid.costs.costs_and_rates_are_finance":
    "Biaya dan tarif adalah data keuangan. Anda tidak memiliki akses ke sana.",
  "p.plans.pid.costs.costs_ppm": "Biaya — PPM",
  "p.plans.pid.costs.currency": "Mata uang",
  "p.plans.pid.costs.description": "Deskripsi",
  "p.plans.pid.costs.evenly_across_the_task": "Merata sepanjang tugas",
  "p.plans.pid.costs.falls": "Jatuh pada",
  "p.plans.pid.costs.fixed_cost": "Biaya tetap",
  "p.plans.pid.costs.kind": "Jenis",
  "p.plans.pid.costs.material": "Material",
  "p.plans.pid.costs.no_time_has_been_costed":
    "Belum ada waktu yang dihitung biayanya.",
  "p.plans.pid.costs.nothing_could_be_costed_yet":
    "Belum ada yang dapat dihitung biayanya: tambahkan tarif untuk orang yang mengerjakan, atau tambahkan pengeluaran.",
  "p.plans.pid.costs.per_period": "{currency}, per periode",
  "p.plans.pid.costs.plan": "← Rencana",
  "p.plans.pid.costs.planned_cost": "Biaya terencana",
  "p.plans.pid.costs.record_labour_costs": "Catat biaya tenaga kerja",
  "p.plans.pid.costs.recorded_labour_cost": "Biaya tenaga kerja tercatat",
  "p.plans.pid.costs.so_its_labour_is_not":
    "▲ {title} ({hoursText}): {reason}, sehingga tenaga kerjanya tidak dihitung biayanya.",
  "p.plans.pid.costs.task": "Tugas",
  "p.plans.pid.costs.time_entries_are_not_costed":
    "▲ {entries} entri waktu ({hoursText}) tidak dihitung biayanya: belum dicatat, atau tanpa tarif. Biayanya tidak diketahui, bukan nol.",
  "p.plans.pid.costs.total": "Total",
  "p.plans.pid.costs.work": "Pekerjaan",
  "p.plans.pid.critical_path.a_baseline_is_a_numbered":
    "Baseline adalah snapshot jadwal bernomor yang disimpan selamanya: baseline ulang membuat versi berikutnya dan tidak pernah menggantikan satu pun.",
  "p.plans.pid.critical_path.baselines": "Baseline",
  "p.plans.pid.critical_path.business_days": "hari kerja",
  "p.plans.pid.critical_path.compare_with": "Bandingkan dengan",
  "p.plans.pid.critical_path.constraint": "Kendala",
  "p.plans.pid.critical_path.constraint_date": "Tanggal kendala",
  "p.plans.pid.critical_path.critical_chains": "{count} rantai kritis{more}.",
  "p.plans.pid.critical_path.critical_path": "Jalur kritis",
  "p.plans.pid.critical_path.critical_path_ppm": "Jalur kritis — PPM",
  "p.plans.pid.critical_path.deadline_a_goal_it_shows":
    "Tenggat (sebuah sasaran: tampil sebagai kelonggaran negatif, tidak pernah menggeser tugas)",
  "p.plans.pid.critical_path.duration_business_days_0_is":
    "Durasi (hari kerja; 0 berarti tonggak)",
  "p.plans.pid.critical_path.finish_no_earlier_than":
    "Selesai tidak lebih awal dari",
  "p.plans.pid.critical_path.finish_no_later_than":
    "Selesai tidak lebih lambat dari",
  "p.plans.pid.critical_path.gantt": "Gantt",
  "p.plans.pid.critical_path.loading": "Memuat…",
  "p.plans.pid.critical_path.must_finish_on_overrides_logic":
    "Harus selesai pada (menimpa logika)",
  "p.plans.pid.critical_path.must_start_on_overrides_logic":
    "Harus mulai pada (menimpa logika)",
  "p.plans.pid.critical_path.near_critical_within": "Hampir kritis dalam",
  "p.plans.pid.critical_path.network": "Jaringan",
  "p.plans.pid.critical_path.no_baseline_yet_so_nothing":
    "Belum ada baseline, sehingga tidak ada yang dapat dikatakan tentang keterlambatan.",
  "p.plans.pid.critical_path.none_as_soon_as_possible":
    "Tidak ada (secepat mungkin)",
  "p.plans.pid.critical_path.not_scheduled": "Belum dijadwalkan",
  "p.plans.pid.critical_path.optimistic_business_days": "Optimis (hari kerja)",
  "p.plans.pid.critical_path.pessimistic_business_days": "Pesimis (hari kerja)",
  "p.plans.pid.critical_path.plan": "← Rencana",
  "p.plans.pid.critical_path.planned_start": "Mulai terencana",
  "p.plans.pid.critical_path.range_for_schedule_risk":
    "Rentang untuk risiko jadwal",
  "p.plans.pid.critical_path.range_saved": "Rentang tersimpan.",
  "p.plans.pid.critical_path.reason_required_this_overrides_the":
    "Alasan (wajib: ini menimpa logika jadwal)",
  "p.plans.pid.critical_path.save_range": "Simpan rentang",
  "p.plans.pid.critical_path.save_schedule": "Simpan jadwal",
  "p.plans.pid.critical_path.saved_moved": "Tersimpan. Bergeser:",
  "p.plans.pid.critical_path.saved_nothing_else_moved":
    "Tersimpan. Tidak ada yang lain bergeser.",
  "p.plans.pid.critical_path.saving": "Menyimpan…",
  "p.plans.pid.critical_path.show_the_baseline_under_each":
    "Tampilkan baseline di bawah tiap batang",
  "p.plans.pid.critical_path.start_no_earlier_than":
    "Mulai tidak lebih awal dari",
  "p.plans.pid.critical_path.start_no_later_than":
    "Mulai tidak lebih lambat dari",
  "p.plans.pid.critical_path.table": "Tabel",
  "p.plans.pid.critical_path.take_baseline": "Ambil baseline",
  "p.plans.pid.critical_path.task": "Tugas",
  "p.plans.pid.critical_path.the_duration_above_is_the":
    "{taskRangeText}. Durasi di atas adalah nilai yang paling mungkin.",
  "p.plans.pid.critical_path.the_longest_chain_of_tasks":
    "Rantai tugas terpanjang, yang menentukan tanggal selesai. Keterlambatan pada tugas mana pun di dalamnya menunda seluruh rencana.",
  "p.plans.pid.critical_path.these_assignees_share_no_business":
    "Para penanggung jawab ini tidak berbagi hari kerja dengan kalender rencana, sehingga kalender rencana dipakai untuk mereka: {join}.",
  "p.plans.pid.critical_path.this_network_has_tasks_too":
    "Jaringan ini memiliki {length} tugas, terlalu banyak untuk digambar dengan berguna. Tabel di bawah menyampaikan hal yang sama.",
  "p.plans.pid.critical_path.untitled": "(tanpa judul)",
  "p.plans.pid.critical_path.version": "Versi {version}: {reason} ({slice})",
  "p.plans.pid.critical_path.view": "Tampilan",
  "p.plans.pid.critical_path.why_a_baseline_now": "Mengapa baseline sekarang?",
  "p.plans.pid.critical_path.working_calendar": "Kalender kerja",
  "p.plans.pid.earned_value.a_percent_i_enter_flagged":
    "Persentase yang saya masukkan (ditandai)",
  "p.plans.pid.earned_value.actual_cost": "Biaya aktual",
  "p.plans.pid.earned_value.against_the_budget_vac": "Terhadap anggaran (VAC)",
  "p.plans.pid.earned_value.baseline_version":
    "baseline versi {baseline_version}",
  "p.plans.pid.earned_value.budget_at_completion": "Anggaran saat selesai",
  "p.plans.pid.earned_value.budget_lines_in_have_no":
    "Baris anggaran dalam {join} tidak punya baseline, sehingga tidak termasuk dalam angka di atas dan tidak pernah dijumlahkan ke mata uang lain.",
  "p.plans.pid.earned_value.by_story_points_when_finished":
    "menurut poin cerita, saat selesai",
  "p.plans.pid.earned_value.cancel": "Batal",
  "p.plans.pid.earned_value.change": "Ubah",
  "p.plans.pid.earned_value.complete": "{round}% selesai",
  "p.plans.pid.earned_value.cost_index": "Indeks biaya (",
  "p.plans.pid.earned_value.cost_is": "Biaya {word}.",
  "p.plans.pid.earned_value.cost_variance_ev_ac": "Selisih biaya (EV − AC)",
  "p.plans.pid.earned_value.count_a_task_another_way":
    "Hitung tugas dengan cara lain",
  "p.plans.pid.earned_value.count_tasks": "Hitung tugas",
  "p.plans.pid.earned_value.critical_path": "Jalur kritis",
  "p.plans.pid.earned_value.earned_value": "Nilai yang diperoleh",
  "p.plans.pid.earned_value.earned_value_figures":
    "Angka nilai yang diperoleh dalam {currency}",
  "p.plans.pid.earned_value.earned_value_ppm": "Nilai yang diperoleh — PPM",
  "p.plans.pid.earned_value.final_cost_eac": "Biaya akhir (EAC)",
  "p.plans.pid.earned_value.finished_only": "hanya yang selesai",
  "p.plans.pid.earned_value.forecast": "Prakiraan",
  "p.plans.pid.earned_value.forecast_final_cost":
    "· prakiraan biaya akhir {money}",
  "p.plans.pid.earned_value.forecasts_of_the_final_cost":
    "Prakiraan biaya akhir dalam {currency}, masing-masing dengan rumusnya sendiri",
  "p.plans.pid.earned_value.governance": "← Tata kelola",
  "p.plans.pid.earned_value.half_for_started": "setengah untuk yang dimulai",
  "p.plans.pid.earned_value.is_the_work_finished_worth":
    "Apakah pekerjaan yang selesai bernilai sebesar yang direncanakan hingga kini, dan berapa biayanya? Indeks di atas 1,00 lebih baik dari rencana.",
  "p.plans.pid.earned_value.leading_with_that_one_because":
    "Mengutamakan yang itu karena {why}. Yang lain ditampilkan di sampingnya: mana yang tepat bergantung pada apakah kelebihan biaya sebelumnya akan berlanjut.",
  "p.plans.pid.earned_value.loading": "Memuat…",
  "p.plans.pid.earned_value.manual": "manual",
  "p.plans.pid.earned_value.measured_as_of_percent_complete":
    "Diukur per {as_of}. Persentase selesai adalah pekerjaan yang rampung, bukan waktu yang berlalu.",
  "p.plans.pid.earned_value.method": "Metode",
  "p.plans.pid.earned_value.only_when_finished": "hanya saat selesai",
  "p.plans.pid.earned_value.only_when_finished_2": "Hanya saat selesai",
  "p.plans.pid.earned_value.over_time": "Dari waktu ke waktu",
  "p.plans.pid.earned_value.percent_complete": "Persentase selesai",
  "p.plans.pid.earned_value.planned_value": "Nilai terencana",
  "p.plans.pid.earned_value.plans": "{plans} rencana",
  "p.plans.pid.earned_value.progress_against_budget_earned_value":
    "Kemajuan terhadap anggaran (nilai yang diperoleh)",
  "p.plans.pid.earned_value.record_today_s_figures_now":
    "Catat angka hari ini sekarang",
  "p.plans.pid.earned_value.recorded": "Tercatat.",
  "p.plans.pid.earned_value.save": "Simpan",
  "p.plans.pid.earned_value.schedule_index": "Indeks jadwal (",
  "p.plans.pid.earned_value.schedule_index_by_time":
    "Indeks jadwal menurut waktu (",
  "p.plans.pid.earned_value.schedule_is": "Jadwal {word}.",
  "p.plans.pid.earned_value.schedule_variance_ev_pv":
    "Selisih jadwal (EV − PV)",
  "p.plans.pid.earned_value.spi": "SPI {ratioOrReason} ·",
  "p.plans.pid.earned_value.started_work_counts_half":
    "pekerjaan yang dimulai dihitung setengah",
  "p.plans.pid.earned_value.started_work_counts_half_2":
    "Pekerjaan yang dimulai dihitung setengah",
  "p.plans.pid.earned_value.still_to_spend_etc":
    "Masih akan dibelanjakan (ETC)",
  "p.plans.pid.earned_value.task": "(tugas)",
  "p.plans.pid.earned_value.task_2": "Tugas",
  "p.plans.pid.earned_value.the_one_to_lead_with": "— yang diutamakan",
  "p.plans.pid.earned_value.the_scheduler_also_records_them":
    "Penjadwal juga mencatatnya setiap hari.",
  "p.plans.pid.earned_value.this_plan_and_everything_beneath":
    "Rencana ini dan semua yang ada di bawahnya",
  "p.plans.pid.earned_value.unknown": "tidak diketahui",
  "p.plans.pid.earned_value.use_the_plan_s_method": "Gunakan metode rencana",
  "p.plans.pid.earned_value.where_it_is_heading": "Ke mana arahnya",
  "p.plans.pid.earned_value.why_required": "Mengapa (wajib)",
  "p.plans.pid.edit.main_x": "{t} — Main X",
  "p.plans.pid.flow.180_calendar_days": "180 hari kalender",
  "p.plans.pid.flow.30_calendar_days": "30 hari kalender",
  "p.plans.pid.flow.60_calendar_days": "60 hari kalender",
  "p.plans.pid.flow.90_calendar_days": "90 hari kalender",
  "p.plans.pid.flow.age": "Usia",
  "p.plans.pid.flow.aging_work_in_progress":
    "Pekerjaan dalam proses yang menua",
  "p.plans.pid.flow.arrival_rate": "Laju kedatangan (λ)",
  "p.plans.pid.flow.assignee": "Penanggung jawab",
  "p.plans.pid.flow.blocked": "Terhambat",
  "p.plans.pid.flow.calendar_day": "{v} / hari kalender",
  "p.plans.pid.flow.column": "Kolom",
  "p.plans.pid.flow.columns": "Kolom",
  "p.plans.pid.flow.concentrated": "terkonsentrasi",
  "p.plans.pid.flow.constraints": "Kendala",
  "p.plans.pid.flow.count": "Jumlah",
  "p.plans.pid.flow.cumulative_flow": "Alur kumulatif",
  "p.plans.pid.flow.cycle_time_is_work_in":
    "Waktu siklus adalah pekerjaan dalam proses dibagi throughput, sehingga menurunkan batas memperpendeknya tanpa ada yang bekerja lebih cepat.",
  "p.plans.pid.flow.detail": "Rincian",
  "p.plans.pid.flow.distributions": "Distribusi",
  "p.plans.pid.flow.first_pass_yield": "Hasil lolos pertama",
  "p.plans.pid.flow.flow": "Alur",
  "p.plans.pid.flow.flow_efficiency": "Efisiensi alur",
  "p.plans.pid.flow.flow_ppm": "Alur — PPM",
  "p.plans.pid.flow.how_long_work_takes": "Berapa lama pekerjaan berlangsung",
  "p.plans.pid.flow.how_long_work_takes_to":
    "Berapa lama pekerjaan selesai, dan berapa banyak yang sedang berjalan sekaligus.",
  "p.plans.pid.flow.implied_cycle_time": "Waktu siklus tersirat",
  "p.plans.pid.flow.item": "Item",
  "p.plans.pid.flow.limit": "Batas",
  "p.plans.pid.flow.max": "maks",
  "p.plans.pid.flow.mean": "rata-rata",
  "p.plans.pid.flow.measure": "Ukuran",
  "p.plans.pid.flow.min": "min",
  "p.plans.pid.flow.no_cap": "tanpa batas",
  "p.plans.pid.flow.no_expectation_yet": "belum ada harapan",
  "p.plans.pid.flow.no_findings_yet": "Belum ada temuan.",
  "p.plans.pid.flow.no_finished_work_to_measure":
    "Belum ada pekerjaan selesai untuk diukur.",
  "p.plans.pid.flow.not_enough_finished_items_yet":
    "Belum cukup item yang selesai.",
  "p.plans.pid.flow.not_started_finished_of":
    "{not_started} belum dimulai · {finished} selesai dari {tasks}",
  "p.plans.pid.flow.nothing_in_progress": "Tidak ada yang sedang berjalan.",
  "p.plans.pid.flow.observed_median": "Median teramati",
  "p.plans.pid.flow.of_the_calendar_time_these":
    "Dari waktu kalender yang dibutuhkan item-item ini, berapa banyak yang dipakai seseorang untuk mengerjakannya? Efisiensi alur dalam kerja pengetahuan biasanya berkisar 5–15%, sehingga angka rendah itu wajar — antrean adalah temuannya, bukan usahanya.",
  "p.plans.pid.flow.of_the_transitions_behind_these":
    "{percent} dari transisi di balik angka-angka ini disintesis saat log diperkenalkan, bukan diamati. Angka itu menjadi lebih kokoh seiring bertambahnya perpindahan papan yang nyata.",
  "p.plans.pid.flow.percentiles_are_nearest_rank_so":
    "Persentil memakai peringkat terdekat, sehingga setiap angka adalah item yang nyata. Waktu siklus adalah yang dikendalikan tim; waktu tunggu adalah yang dinanti pemohon, dan selisihnya adalah berapa lama pekerjaan berada di backlog.",
  "p.plans.pid.flow.recoverable": "Dapat dipulihkan",
  "p.plans.pid.flow.rework": "Pengerjaan ulang",
  "p.plans.pid.flow.rule": "Aturan",
  "p.plans.pid.flow.service_level_expectation": "Harapan tingkat layanan",
  "p.plans.pid.flow.status": "Status",
  "p.plans.pid.flow.strong_against_the_5_15": "Kuat dibandingkan norma 5–15%.",
  "p.plans.pid.flow.subject": "Subjek",
  "p.plans.pid.flow.the_mean_is_shown_for":
    "Rata-rata ditampilkan demi kelengkapan dan tidak menggambarkan item nyata mana pun — kutip p85.",
  "p.plans.pid.flow.the_plan_s_overall_ratio":
    ": rasio keseluruhan rencana ({percent}) jauh berbeda dari item yang lazim ({percent2}). Sebagian besar item mengalir; sebagian kecil macet. Itu perbaikan yang berbeda dari penyerahan yang lambat secara merata — temukan item-item itu daripada merombak proses.",
  "p.plans.pid.flow.the_waste_is": "Pemborosannya adalah",
  "p.plans.pid.flow.throughput": "Throughput (μ)",
  "p.plans.pid.flow.typical_median_item": "Lazim. Item median: {percent}.",
  "p.plans.pid.flow.unassigned": "belum ditugaskan",
  "p.plans.pid.flow.unusually_high_this_usually_means":
    "Luar biasa tinggi — ini biasanya berarti papan tidak dijaga tetap mutakhir, bukan antrean yang sudah hilang.",
  "p.plans.pid.flow.utilization": "Utilisasi (ρ)",
  "p.plans.pid.flow.vs_expectation": "dibanding harapan",
  "p.plans.pid.flow.window": "Jendela",
  "p.plans.pid.flow.work_in_progress": "Pekerjaan dalam proses",
  "p.plans.pid.flow.work_in_progress_2": "Pekerjaan dalam proses (κ)",
  "p.plans.pid.governance.actual": "Aktual",
  "p.plans.pid.governance.approved": "disetujui",
  "p.plans.pid.governance.approved_with_conditions": "disetujui dengan syarat",
  "p.plans.pid.governance.benefit": "Manfaat",
  "p.plans.pid.governance.benefit_category": "Kategori manfaat",
  "p.plans.pid.governance.capex": "capex",
  "p.plans.pid.governance.category": "Kategori",
  "p.plans.pid.governance.choose_objective": "pilih sasaran…",
  "p.plans.pid.governance.commit": "Tetapkan",
  "p.plans.pid.governance.commit_anyway": "Tetapkan tetap",
  "p.plans.pid.governance.committing_this_puts_them_over":
    "Menetapkan ini membuat mereka melebihi kapasitas.",
  "p.plans.pid.governance.compliance": "kepatuhan",
  "p.plans.pid.governance.cost_saving": "penghematan biaya",
  "p.plans.pid.governance.currency": "Mata uang",
  "p.plans.pid.governance.decision": "Keputusan",
  "p.plans.pid.governance.governance_ppm": "Tata kelola — PPM",
  "p.plans.pid.governance.hold": "ditahan",
  "p.plans.pid.governance.line": "Baris",
  "p.plans.pid.governance.materialized": "terjadi: {materialized}",
  "p.plans.pid.governance.max_exposure": "eksposur maksimum: {max_exposure}",
  "p.plans.pid.governance.new_benefit": "Manfaat baru",
  "p.plans.pid.governance.new_budget_line": "Baris anggaran baru",
  "p.plans.pid.governance.new_milestone": "Tonggak baru",
  "p.plans.pid.governance.new_milestone_due": "Tenggat tonggak baru",
  "p.plans.pid.governance.new_risk": "Risiko baru",
  "p.plans.pid.governance.next": "berikutnya: {next_gate}",
  "p.plans.pid.governance.objective": "Sasaran",
  "p.plans.pid.governance.open_risks": "risiko terbuka: {open}",
  "p.plans.pid.governance.opex": "opex",
  "p.plans.pid.governance.other": "lainnya",
  "p.plans.pid.governance.planned": "Terencana",
  "p.plans.pid.governance.planned_actual_variance":
    ": terencana {money} · aktual {money2} · selisih {money3}",
  "p.plans.pid.governance.planned_major": "Terencana (satuan utama)",
  "p.plans.pid.governance.proposed_does_not_count_until":
    "diusulkan (tidak dihitung sampai ditetapkan)",
  "p.plans.pid.governance.quality": "kualitas",
  "p.plans.pid.governance.range_for_cost_risk": "Rentang untuk risiko biaya",
  "p.plans.pid.governance.realized": "Terealisasi",
  "p.plans.pid.governance.record_actual": "Catat aktual",
  "p.plans.pid.governance.rejected": "ditolak",
  "p.plans.pid.governance.revenue": "pendapatan",
  "p.plans.pid.governance.review": "Tinjau",
  "p.plans.pid.governance.risk": "Risiko",
  "p.plans.pid.governance.risk_reduction": "pengurangan risiko",
  "p.plans.pid.governance.roi": "· ROI {toFixed}%",
  "p.plans.pid.governance.stage": "tahap: {v}",
  "p.plans.pid.governance.status": "Status",
  "p.plans.pid.governance.target": "Target",
  "p.plans.pid.governance.target_major": "Target (satuan utama)",
  "p.plans.pid.governance.target_realized_spend":
    ": target {money} · terealisasi {money2} · belanja {money3}",
  "p.plans.pid.governance.weight": "bobot",
  "p.plans.pid.governance.worker_uuid": "worker:<uuid>",
  "p.plans.pid.handover.discard": "Buang",
  "p.plans.pid.handover.from": "Dari",
  "p.plans.pid.handover.hand_over": "Serahkan",
  "p.plans.pid.handover.handover": "Serah terima",
  "p.plans.pid.handover.handover_ppm": "Serah terima — PPM",
  "p.plans.pid.handover.kind": "Jenis",
  "p.plans.pid.handover.onboarding_and_offboarding":
    "Orientasi dan pelepasan anggota",
  "p.plans.pid.handover.person": "person:…",
  "p.plans.pid.handover.person_empty_releases_the_work":
    "person:… (kosong berarti melepas pekerjaan)",
  "p.plans.pid.handover.plan": "← Rencana",
  "p.plans.pid.handover.preview": "Pratinjau",
  "p.plans.pid.handover.reason_recorded_with_each_item":
    "Alasan (dicatat bersama setiap item)",
  "p.plans.pid.handover.release": "Lepaskan",
  "p.plans.pid.handover.the_new_person_s_capacity":
    "Kapasitas orang baru tidak diperiksa di sini.",
  "p.plans.pid.handover.to": "Kepada",
  "p.plans.pid.handover.what": "Apa",
  "p.plans.pid.handover.what_holds_that_is_still":
    "Apa yang masih dipegang {from_person} dan masih aktif",
  "p.plans.pid.handover.who_is_handing_over": "Siapa yang menyerahkan?",
  "p.plans.pid.import.bring_tasks_in_from_jira":
    "Bawa tugas dari Jira, Asana, atau Microsoft Project. Anda melihat apa yang akan terjadi lebih dulu; tidak ada yang ditulis sampai Anda mengonfirmasi.",
  "p.plans.pid.import.choose_another_file": "Pilih berkas lain",
  "p.plans.pid.import.file": "Berkas",
  "p.plans.pid.import.import": "Impor",
  "p.plans.pid.import.import_ppm": "Impor — PPM",
  "p.plans.pid.import.imported": "Terimpor",
  "p.plans.pid.import.plan": "← Rencana",
  "p.plans.pid.import.preview": "Pratinjau",
  "p.plans.pid.import.tracker": "Pelacak",
  "p.plans.pid.import.where_from": "Dari mana",
  "p.plans.pid.joint_risk.a_late_project_usually_costs":
    "Proyek yang terlambat biasanya lebih mahal juga, sehingga ini menjalankan biaya dan jadwal bersama.",
  "p.plans.pid.joint_risk.assumptions": "Asumsi",
  "p.plans.pid.joint_risk.band_when_there_is_no":
    "Pita bila tidak ada perkiraan (± %)",
  "p.plans.pid.joint_risk.both_at_once": "Keduanya sekaligus",
  "p.plans.pid.joint_risk.budget": "anggaran",
  "p.plans.pid.joint_risk.budget_2": "Anggaran",
  "p.plans.pid.joint_risk.budget_lines_in_are_not":
    "Baris anggaran dalam {join} tidak termasuk dalam eksekusi ini; mata uang tidak pernah dijumlahkan.",
  "p.plans.pid.joint_risk.business_days_late":
    "terlambat {days} hari kerja, {money}",
  "p.plans.pid.joint_risk.business_days_late_2": "hari kerja terlambat →",
  "p.plans.pid.joint_risk.cost": "Biaya",
  "p.plans.pid.joint_risk.cost_2": "Biaya — {title}: {costSkippedText}",
  "p.plans.pid.joint_risk.cost_and_schedule_risk_ppm":
    "Risiko biaya dan jadwal — PPM",
  "p.plans.pid.joint_risk.cost_and_schedule_together":
    "Biaya dan jadwal bersama",
  "p.plans.pid.joint_risk.cost_lines": "Baris biaya",
  "p.plans.pid.joint_risk.cost_of_each_business_day":
    "Biaya setiap hari kerja keterlambatan (satuan utama)",
  "p.plans.pid.joint_risk.cost_risk": "Risiko biaya",
  "p.plans.pid.joint_risk.critical_path_finish_the_plan":
    "Penyelesaian jalur kritis (tanggal rencana itu sendiri)",
  "p.plans.pid.joint_risk.currency_if_the_plan_has":
    "Mata uang (jika rencana memiliki beberapa)",
  "p.plans.pid.joint_risk.distribution": "Distribusi",
  "p.plans.pid.joint_risk.each_dot_is_one_simulation":
    "Setiap titik adalah satu simulasi (sampel yang diperjarang). Di kanan garis tepat waktu berarti terlambat; di atas garis anggaran berarti melebihi anggaran. Tabel di atas memuat jumlah lengkapnya.",
  "p.plans.pid.joint_risk.estimate": "Perkiraan",
  "p.plans.pid.joint_risk.finish": "Selesai",
  "p.plans.pid.joint_risk.finish_date_at_each_percentile":
    "Tanggal selesai pada tiap persentil",
  "p.plans.pid.joint_risk.how_the_simulated_outcomes_fall":
    "Bagaimana hasil simulasi tersebar di sekitar anggaran dan penyelesaian jalur kritis",
  "p.plans.pid.joint_risk.independence": "Independensi.",
  "p.plans.pid.joint_risk.iterations": "Iterasi",
  "p.plans.pid.joint_risk.line": "Baris",
  "p.plans.pid.joint_risk.low_planned_high": "Rendah / terencana / tinggi",
  "p.plans.pid.joint_risk.mean_delay_cost_included":
    "Rata-rata (termasuk biaya keterlambatan)",
  "p.plans.pid.joint_risk.no_register_risk_was_used":
    "Tidak ada risiko dari register yang dipakai dalam eksekusi ini.",
  "p.plans.pid.joint_risk.not_enough_data": "data belum cukup",
  "p.plans.pid.joint_risk.of_which_delay_cost_on":
    "Di antaranya biaya keterlambatan, rata-rata",
  "p.plans.pid.joint_risk.on_time": "tepat waktu",
  "p.plans.pid.joint_risk.outcome": "Hasil",
  "p.plans.pid.joint_risk.per_line_estimate_in":
    "Perkiraan per baris dalam {currency}",
  "p.plans.pid.joint_risk.pert": "PERT",
  "p.plans.pid.joint_risk.plan": "← Rencana",
  "p.plans.pid.joint_risk.risks": "Risiko",
  "p.plans.pid.joint_risk.run": "Jalankan",
  "p.plans.pid.joint_risk.running": "Berjalan…",
  "p.plans.pid.joint_risk.scatter_of_simulated_total_cost":
    "Sebaran total biaya simulasi terhadap hari kerja keterlambatan, dengan anggaran dan garis tepat waktu",
  "p.plans.pid.joint_risk.schedule": "Jadwal — {title}: {skippedReasonText}",
  "p.plans.pid.joint_risk.schedule_risk": "Risiko jadwal",
  "p.plans.pid.joint_risk.seed": "Seed",
  "p.plans.pid.joint_risk.seed_the_same_seed_gives":
    "Seed {seed}: seed yang sama memberi hasil yang sama.",
  "p.plans.pid.joint_risk.share": "Porsi",
  "p.plans.pid.joint_risk.simulations": "Simulasi",
  "p.plans.pid.joint_risk.total_cost_delay_cost_included":
    "Total biaya, termasuk biaya keterlambatan",
  "p.plans.pid.joint_risk.triangular": "Segitiga",
  "p.plans.pid.leveling.allow_critical_tasks_to_move":
    "Izinkan tugas kritis berpindah (ini dapat menunda penyelesaian)",
  "p.plans.pid.leveling.apply": "Terapkan",
  "p.plans.pid.leveling.apply_the_ticked_moves":
    "Terapkan perpindahan yang dicentang",
  "p.plans.pid.leveling.balance_workloads_resource_leveling":
    "Seimbangkan beban kerja (perataan sumber daya)",
  "p.plans.pid.leveling.could_not_be_fixed": "Tidak dapat diperbaiki",
  "p.plans.pid.leveling.critical": "kritis",
  "p.plans.pid.leveling.each_start_date_the_proposal":
    "Setiap tanggal mulai yang akan diubah usulan",
  "p.plans.pid.leveling.finish": "Selesai",
  "p.plans.pid.leveling.float": "Kelonggaran",
  "p.plans.pid.leveling.i_have_read_the_proposal":
    "Saya telah membaca usulan dan ingin tanggal mulai ini diubah",
  "p.plans.pid.leveling.later_by": "Mundur sebesar",
  "p.plans.pid.leveling.load_per_task_of_a":
    "Beban per tugas (% dari hari seseorang)",
  "p.plans.pid.leveling.loading": "Memuat…",
  "p.plans.pid.leveling.no_start_date_would_change":
    "Tidak ada tanggal mulai yang akan berubah.",
  "p.plans.pid.leveling.nobody_is_over_allocated_so":
    "Tidak ada yang dialokasikan berlebih, sehingga tidak ada yang perlu diratakan.",
  "p.plans.pid.leveling.capacity_undeclared_h":
    "Tidak ada kapasitas yang dinyatakan",
  "p.plans.pid.leveling.capacity_undeclared_p":
    "Orang-orang ini memiliki pekerjaan pada hari tanpa ketersediaan yang dinyatakan; pada hari-hari itu mereka tidak melebihi dan tidak di dalam kapasitas:",
  "p.plans.pid.leveling.over_allocated_now": "Dialokasikan berlebih saat ini",
  "p.plans.pid.leveling.person": "Orang",
  "p.plans.pid.leveling.plan": "← Rencana",
  "p.plans.pid.leveling.propose_again": "Usulkan lagi",
  "p.plans.pid.leveling.proposed_start_dates": "Tanggal mulai yang diusulkan",
  "p.plans.pid.leveling.refused": "Ditolak: {note}",
  "p.plans.pid.leveling.resource_leveling_ppm": "Perataan sumber daya — PPM",
  "p.plans.pid.leveling.start": "Mulai",
  "p.plans.pid.leveling.start_finish_business_days":
    "{pid}: mulai {v}{start_days}, selesai {v2}{finish_days} hari kerja",
  "p.plans.pid.leveling.suggests_moving_tasks_so_that":
    "Menyarankan pemindahan tugas agar tidak ada yang dipesan melebihi kemampuannya. Tidak ada yang berubah sampai Anda mengonfirmasi.",
  "p.plans.pid.leveling.task": "Tugas",
  "p.plans.pid.leveling.the_search_stopped_at_its":
    "pencarian berhenti di batasnya",
  "p.plans.pid.leveling.this_is_advice_nothing_changes":
    "Ini hanya saran. Tidak ada yang berubah sampai Anda memilih perpindahan di bawah dan menerapkannya.",
  "p.plans.pid.leveling.why_kept_with_each_audit":
    "Mengapa (disimpan bersama setiap entri audit)",
  "p.plans.pid.leveling.working_calendar": "Kalender kerja",
  "p.plans.pid.more_tools": "Alat lainnya",
  "p.plans.pid.onboarding.access": "Akses",
  "p.plans.pid.onboarding.access_record": "Catatan akses",
  "p.plans.pid.onboarding.action": "Tindakan",
  "p.plans.pid.onboarding.cancel_checklist": "Batalkan daftar periksa",
  "p.plans.pid.onboarding.checklists": "Daftar periksa",
  "p.plans.pid.onboarding.complete_checklist": "Selesaikan daftar periksa",
  "p.plans.pid.onboarding.done": "Selesai",
  "p.plans.pid.onboarding.effective":
    "{status}{v} · berlaku {effective_date} · {progressText}",
  "p.plans.pid.onboarding.from":
    "{status} · mulai {effective_date} · {progressText}",
  "p.plans.pid.onboarding.granted": "Diberikan",
  "p.plans.pid.onboarding.hand_over_someone_s_work":
    "Serahkan pekerjaan seseorang",
  "p.plans.pid.onboarding.no_access_is_outstanding":
    "● Tidak ada akses yang tertunggak.",
  "p.plans.pid.onboarding.none_yet": "Belum ada.",
  "p.plans.pid.onboarding.onboarding_and_offboarding":
    "Orientasi dan pelepasan anggota",
  "p.plans.pid.onboarding.onboarding_and_offboarding_ppm":
    "Orientasi dan pelepasan anggota — PPM",
  "p.plans.pid.onboarding.person": "person:…",
  "p.plans.pid.onboarding.person_2": "Orang",
  "p.plans.pid.onboarding.plan": "← Rencana",
  "p.plans.pid.onboarding.ready_to_close": "Siap ditutup?",
  "p.plans.pid.onboarding.reason": "Alasan",
  "p.plans.pid.onboarding.reason_for_cancelling": "Alasan pembatalan",
  "p.plans.pid.onboarding.record": "Catat",
  "p.plans.pid.onboarding.record_access": "Catat akses",
  "p.plans.pid.onboarding.reopen": "Buka kembali",
  "p.plans.pid.onboarding.repo_write": "tulis repo",
  "p.plans.pid.onboarding.revoked": "Dicabut",
  "p.plans.pid.onboarding.save": "Simpan",
  "p.plans.pid.onboarding.since":
    "▲ {person_ref}: {access} (sejak {granted_on})",
  "p.plans.pid.onboarding.skip": "Lewati",
  "p.plans.pid.onboarding.start": "Mulai",
  "p.plans.pid.onboarding.start_a_checklist": "Mulai daftar periksa",
  "p.plans.pid.onboarding.takes_effect": "Berlaku",
  "p.plans.pid.onboarding.what": "Apa",
  "p.plans.pid.onboarding.who_holds_what_access_is":
    "Siapa memegang akses apa menyangkut orang, sehingga memerlukan izin untuk mengelola orang.",
  "p.plans.pid.onboarding.why_skip": "Mengapa dilewati?",
  "p.plans.pid.raci.add_a_person_person_or":
    "Tambah orang (person:… atau worker:…)",
  "p.plans.pid.raci.add_a_person_to_start":
    "Tambah orang untuk mulai menetapkan huruf.",
  "p.plans.pid.raci.add_column": "Tambah kolom",
  "p.plans.pid.raci.export_to_a_spreadsheet": "Ekspor ke lembar kerja",
  "p.plans.pid.raci.for_each_task_who_does":
    "Untuk setiap tugas: siapa yang mengerjakan, siapa yang mengesahkan, siapa yang dimintai pendapat, dan siapa yang diberi tahu.",
  "p.plans.pid.raci.have_nobody_accountable":
    "{missing_accountable} tidak punya penanggung jawab akhir.",
  "p.plans.pid.raci.have_nobody_responsible":
    "{missing_responsible} tidak punya pelaksana.",
  "p.plans.pid.raci.how_many_tasks_each_person":
    "Berapa tugas yang dipegang tiap orang untuk setiap huruf",
  "p.plans.pid.raci.legend": "Legenda",
  "p.plans.pid.raci.load_by_person": "Beban per orang",
  "p.plans.pid.raci.loading": "Memuat…",
  "p.plans.pid.raci.of_tasks_are_complete_one":
    "{complete} dari {tasks} tugas lengkap (satu penanggung jawab akhir, setidaknya satu pelaksana).",
  "p.plans.pid.raci.person": "Orang",
  "p.plans.pid.raci.plan": "← Rencana",
  "p.plans.pid.raci.raci_ppm": "RACI — PPM",
  "p.plans.pid.raci.this_plan_has_no_tasks": "Rencana ini belum punya tugas.",
  "p.plans.pid.raci.who_does_what_raci_matrix":
    "Siapa melakukan apa (matriks RACI)",
  "p.plans.pid.schedule.dependency_violated_successor_starts":
    "Ketergantungan dilanggar: penerus mulai {actual_start} padahal paling cepat boleh mulai {earliest_start}.",
  "p.plans.pid.schedule.schedule_ppm": "Jadwal — PPM",
  "p.plans.pid.schedule_health.as_of": "Per {as_of} ·",
  "p.plans.pid.schedule_health.baseline_version": "baseline versi {version}",
  "p.plans.pid.schedule_health.check": "Periksa",
  "p.plans.pid.schedule_health.checks_the_schedule_for_common":
    "Memeriksa jadwal untuk masalah umum, seperti tugas tanpa tautan atau tanpa perkiraan.",
  "p.plans.pid.schedule_health.critical_path": "Jalur kritis",
  "p.plans.pid.schedule_health.held_to": "Dipertahankan pada",
  "p.plans.pid.schedule_health.loading": "Memuat…",
  "p.plans.pid.schedule_health.no_baseline_taken": "belum ada baseline",
  "p.plans.pid.schedule_health.plan": "← Rencana",
  "p.plans.pid.schedule_health.re_run": "Jalankan ulang",
  "p.plans.pid.schedule_health.result": "Hasil",
  "p.plans.pid.schedule_health.schedule_health": "Kesehatan jadwal",
  "p.plans.pid.schedule_health.schedule_health_ppm": "Kesehatan jadwal — PPM",
  "p.plans.pid.schedule_health.see_the_note": "Lihat catatannya",
  "p.plans.pid.schedule_health.status_date": "Tanggal status",
  "p.plans.pid.schedule_health.the_fourteen_schedule_health_checks":
    "Empat belas pemeriksaan kesehatan jadwal",
  "p.plans.pid.schedule_health.to_fix": "Untuk diperbaiki",
  "p.plans.pid.schedule_health.what_was_found": "Apa yang ditemukan",
  "p.plans.pid.schedule_risk.assumptions": "Asumsi",
  "p.plans.pid.schedule_risk.band_for_tasks_with_no":
    "Pita untuk tugas tanpa perkiraan (± %)",
  "p.plans.pid.schedule_risk.critical_in": "Kritis dalam",
  "p.plans.pid.schedule_risk.critical_path": "Jalur kritis",
  "p.plans.pid.schedule_risk.critical_path_finish_the_plan":
    "Penyelesaian jalur kritis (tanggal rencana itu sendiri)",
  "p.plans.pid.schedule_risk.distribution": "Distribusi",
  "p.plans.pid.schedule_risk.each_bar_is_a_finish":
    "Setiap batang adalah tanggal selesai; makin tinggi berarti makin banyak simulasi berakhir di sana. Garis tegas adalah P50, P80, dan P90 (berlabel); garis putus-putus adalah tanggal rencana itu sendiri.",
  "p.plans.pid.schedule_risk.earliest_latest":
    "{join}. Paling awal {earliest}, paling akhir {latest}.",
  "p.plans.pid.schedule_risk.estimate": "Perkiraan",
  "p.plans.pid.schedule_risk.finish": "Selesai",
  "p.plans.pid.schedule_risk.finish_date": "Tanggal selesai",
  "p.plans.pid.schedule_risk.finish_date_at_each_percentile":
    "Tanggal selesai pada tiap persentil",
  "p.plans.pid.schedule_risk.histogram_of_simulated_finish_dates":
    "Histogram tanggal selesai simulasi, dengan P50, P80, dan P90 ditandai",
  "p.plans.pid.schedule_risk.independence": "Independensi.",
  "p.plans.pid.schedule_risk.iterations": "Iterasi",
  "p.plans.pid.schedule_risk.left_out": "Tidak disertakan:",
  "p.plans.pid.schedule_risk.many_what_if_runs_of":
    "Banyak simulasi bagaimana-jika atas jadwal menunjukkan rentang tanggal selesai dan seberapa mungkin masing-masing.",
  "p.plans.pid.schedule_risk.no_register_risk_was_used":
    "Tidak ada risiko dari register yang dipakai sebagai peristiwa jadwal.",
  "p.plans.pid.schedule_risk.not_enough_data": "data belum cukup",
  "p.plans.pid.schedule_risk.not_simulated_because_the_schedule":
    "Tidak disimulasikan, karena jadwal tidak dapat menempatkannya: {join}.",
  "p.plans.pid.schedule_risk.of": "{date}: {count} dari {iterations}",
  "p.plans.pid.schedule_risk.of_runs": "{criticalityText} dari eksekusi",
  "p.plans.pid.schedule_risk.optimistic_likely_pessimistic_busine":
    "Optimis / paling mungkin / pesimis (hari kerja)",
  "p.plans.pid.schedule_risk.outcome": "Hasil",
  "p.plans.pid.schedule_risk.per_task_estimate_criticality_and":
    "Perkiraan, kekritisan, dan sensitivitas per tugas",
  "p.plans.pid.schedule_risk.plan": "← Rencana",
  "p.plans.pid.schedule_risk.plan_2": "rencana",
  "p.plans.pid.schedule_risk.risks_used": "Risiko yang dipakai",
  "p.plans.pid.schedule_risk.run": "Jalankan",
  "p.plans.pid.schedule_risk.run_on_against_the_plan":
    "Dijalankan pada {ranOn} terhadap rencana apa adanya · seed {seed} · {iterations} iterasi · {distributionText} · kalender {calendar}",
  "p.plans.pid.schedule_risk.running": "Berjalan…",
  "p.plans.pid.schedule_risk.schedule_risk": "Risiko jadwal",
  "p.plans.pid.schedule_risk.schedule_risk_ppm": "Risiko jadwal — PPM",
  "p.plans.pid.schedule_risk.seed": "Seed",
  "p.plans.pid.schedule_risk.seed_the_same_seed_gives":
    "Seed {seed}: seed yang sama memberi hasil yang sama. Hingga {max_iterations} iterasi.",
  "p.plans.pid.schedule_risk.sensitivity": "Sensitivitas",
  "p.plans.pid.schedule_risk.share": "Porsi",
  "p.plans.pid.schedule_risk.simulations": "Simulasi",
  "p.plans.pid.schedule_risk.task": "Tugas",
  "p.plans.pid.schedule_risk.the_same_figures_as_a":
    "Angka yang sama dalam bentuk tabel",
  "p.plans.pid.schedule_risk.triangular": "Segitiga",
  "p.plans.pid.schedule_risk.weighted_towards_the_likely_value":
    "Dibobotkan ke nilai yang paling mungkin (PERT)",
  "p.plans.pid.schedule_risk.what_drives_it": "Apa penyebabnya",
  "p.plans.pid.schedule_risk.when_it_finishes": "Kapan selesai",
  "p.plans.pid.wbs.code": "Kode",
  "p.plans.pid.wbs.level": "Tingkat",
  "p.plans.pid.wbs.no_tasks_yet": "Belum ada tugas.",
  "p.plans.pid.wbs.plan": "← Rencana",
  "p.plans.pid.wbs.sub_tasks": "Subtugas",
  "p.plans.pid.wbs.task": "Tugas",
  "p.plans.pid.wbs.tasks_is_too_many_to":
    "{length} tugas terlalu banyak untuk disarangkan dengan berguna; tabel menampilkan semuanya.",
  "p.plans.pid.wbs.the_plan_s_tasks_each":
    "Tugas-tugas rencana, masing-masing dipecah menjadi subtugasnya. Kode pada tiap baris untuk dibaca: kode berubah saat pekerjaan ditambah atau dipindahkan.",
  "p.plans.pid.wbs.work_breakdown": "Struktur rincian kerja",
  "p.plans.pid.wbs.work_breakdown_as_a_table":
    "Struktur rincian kerja sebagai tabel",
  "p.plans.pid.wbs.work_breakdown_ppm": "Struktur rincian kerja — PPM",
  "p.plans.pid.wbs.work_breakdown_tree": "Pohon struktur rincian kerja",
  "p.plans.pid.what_if.after": "Sesudah",
  "p.plans.pid.what_if.before": "Sebelum",
  "p.plans.pid.what_if.business_days": "Hari kerja",
  "p.plans.pid.what_if.choose": "Pilih…",
  "p.plans.pid.what_if.commit_these_changes": "Tetapkan perubahan ini",
  "p.plans.pid.what_if.constraint": "Kendala",
  "p.plans.pid.what_if.critical": "■ kritis",
  "p.plans.pid.what_if.critical_path": "Jalur kritis",
  "p.plans.pid.what_if.date": "Tanggal",
  "p.plans.pid.what_if.discard": "Buang",
  "p.plans.pid.what_if.edit": "Ubah",
  "p.plans.pid.what_if.finish_to_finish": "Selesai ke selesai",
  "p.plans.pid.what_if.finish_to_start": "Selesai ke mulai",
  "p.plans.pid.what_if.float": "Kelonggaran",
  "p.plans.pid.what_if.follows": "{name} mengikuti {name2} ({toUpperCase})",
  "p.plans.pid.what_if.joins_the_critical_path":
    "Masuk ke jalur kritis: {join}.",
  "p.plans.pid.what_if.lag_business_days": "Jeda (hari kerja)",
  "p.plans.pid.what_if.leaves_it": "Keluar darinya: {join}.",
  "p.plans.pid.what_if.link": "Tautan",
  "p.plans.pid.what_if.new_start": "Mulai baru",
  "p.plans.pid.what_if.no_task_s_dates_or":
    "Tidak ada tanggal atau kelonggaran tugas yang berubah.",
  "p.plans.pid.what_if.none_clear_it": "Tidak ada (hapus)",
  "p.plans.pid.what_if.none_yet": "Belum ada.",
  "p.plans.pid.what_if.now_takes_of_the_day":
    "“{name}” kini memakai {units_percent}% dari hari untuk {hoursText} pekerjaan (dihitung dari aturan penjadwalannya).",
  "p.plans.pid.what_if.of_work_has_no_rate":
    "▲ {hoursText} pekerjaan tidak punya tarif, sehingga tidak dihitung biayanya.",
  "p.plans.pid.what_if.plan": "← Rencana",
  "p.plans.pid.what_if.planned_labour_cost":
    "Biaya tenaga kerja terencana: {deltaText} ({deltaText2} → {deltaText3}).",
  "p.plans.pid.what_if.predecessor": "Pendahulu",
  "p.plans.pid.what_if.preview": "Pratinjau",
  "p.plans.pid.what_if.reason": "Alasan",
  "p.plans.pid.what_if.reason_recorded_with_each_change":
    "Alasan (dicatat bersama setiap perubahan)",
  "p.plans.pid.what_if.remove": "Hapus",
  "p.plans.pid.what_if.stage_an_edit": "Siapkan perubahan",
  "p.plans.pid.what_if.stage_edit": "Siapkan perubahan",
  "p.plans.pid.what_if.staged_edits": "Perubahan yang disiapkan",
  "p.plans.pid.what_if.start_to_finish": "Mulai ke selesai",
  "p.plans.pid.what_if.start_to_start": "Mulai ke mulai",
  "p.plans.pid.what_if.successor": "Penerus",
  "p.plans.pid.what_if.task": "Tugas",
  "p.plans.pid.what_if.tasks_whose_dates_float_or":
    "Tugas yang tanggal, kelonggaran, atau status kritisnya berubah",
  "p.plans.pid.what_if.try_changes_on_a_copy":
    "Coba perubahan pada salinan. Tidak ada yang disimpan sampai Anda menetapkannya.",
  "p.plans.pid.what_if.type": "Tipe",
  "p.plans.pid.what_if.what_if": "Bagaimana-jika",
  "p.plans.pid.what_if.what_if_ppm": "Bagaimana-jika — PPM",
  "p.plans.pid.work.business_days_of_a_day":
    "{duration_days} hari kerja · {units_percent}% dari sehari · {hoursText} pekerjaan · {v}",
  "p.plans.pid.work.change": "Ubah",
  "p.plans.pid.work.costs": "Biaya",
  "p.plans.pid.work.day": "Hari",
  "p.plans.pid.work.duration_business_days": "Durasi (hari kerja)",
  "p.plans.pid.work.edit_assignment": "Ubah penugasan",
  "p.plans.pid.work.falls_outside_this_window":
    "{hoursText} berada di luar jendela ini.",
  "p.plans.pid.work.hours_per_each_row_adds":
    "Jam per {grain}; setiap baris dijumlahkan menjadi pekerjaan tugas itu, baris terakhir menjadi pekerjaan rencana.",
  "p.plans.pid.work.how_long_each_task_takes":
    "Berapa lama tiap tugas berlangsung, seberapa banyak hari seseorang terpakai, dan berapa banyak pekerjaan yang terkumpul.",
  "p.plans.pid.work.month": "Bulan",
  "p.plans.pid.work.no_assignment_yet_give_a":
    "Belum ada penugasan. Beri durasi dan satuan, lalu pekerjaan dihitung.",
  "p.plans.pid.work.no_task_carries_work_yet":
    "Belum ada tugas yang membawa pekerjaan, sehingga tidak ada yang disebar.",
  "p.plans.pid.work.not_placed": "(belum ditempatkan)",
  "p.plans.pid.work.percentages": "Persentase",
  "p.plans.pid.work.period": "Periode",
  "p.plans.pid.work.plan": "← Rencana",
  "p.plans.pid.work.plan_2": "Rencana",
  "p.plans.pid.work.rule": "Aturan",
  "p.plans.pid.work.save_assignment": "Simpan penugasan",
  "p.plans.pid.work.save_contour": "Simpan kontur",
  "p.plans.pid.work.so_its_work_is_not":
    "▲ {title} ({hoursText}) {reason}, sehingga pekerjaannya tidak masuk dalam kelompok.",
  "p.plans.pid.work.spread": "Sebaran",
  "p.plans.pid.work.task": "Tugas",
  "p.plans.pid.work.tasks": "Tugas",
  "p.plans.pid.work.this_plan_has_no_scheduled":
    "Rencana ini belum punya tugas terjadwal.",
  "p.plans.pid.work.total": "Total",
  "p.plans.pid.work.units": "Satuan (%)",
  "p.plans.pid.work.units_of_the_day": "Satuan (% dari hari)",
  "p.plans.pid.work.week": "Minggu",
  "p.plans.pid.work.work_and_assignments": "Pekerjaan dan penugasan",
  "p.plans.pid.work.work_and_assignments_ppm": "Pekerjaan dan penugasan — PPM",
  "p.plans.pid.work.work_hours": "Pekerjaan (jam)",
  "p.plans.pid.work.work_over_time": "Pekerjaan dari waktu ke waktu",
  "p.prioritization.all": "semua",
  "p.prioritization.apply": "Terapkan",
  "p.prioritization.band": "pita",
  "p.prioritization.band_2": "Pita",
  "p.prioritization.component": "Komponen",
  "p.prioritization.coverage": "cakupan",
  "p.prioritization.coverage_2": "Cakupan",
  "p.prioritization.explain": "Jelaskan",
  "p.prioritization.hide": "Sembunyikan",
  "p.prioritization.high": "tinggi",
  "p.prioritization.loading_the_breakdown": "Memuat rincian…",
  "p.prioritization.low": "rendah",
  "p.prioritization.medium": "sedang",
  "p.prioritization.missing_evidence": "Bukti yang kurang",
  "p.prioritization.no_evidence_for_any_component":
    "Tidak ada bukti untuk komponen mana pun — rencana ini belum dinilai.",
  "p.prioritization.no_plans_match_that_filter":
    "Tidak ada rencana yang cocok dengan filter itu.",
  "p.prioritization.none": "tidak ada",
  "p.prioritization.of_plans_one": "{length} dari {scored_of} rencana",
  "p.prioritization.of_plans_other": "{length} dari {scored_of} rencana",
  "p.prioritization.plan": "Rencana",
  "p.prioritization.points": "Poin",
  "p.prioritization.prioritization": "Prioritisasi",
  "p.prioritization.prioritization_ppm": "Prioritisasi — PPM",
  "p.prioritization.score": "Skor",
  "p.prioritization.shows_how_much_of_the":
    "menunjukkan seberapa banyak skor yang benar-benar didukung data.",
  "p.prioritization.smart_score_over_roi_strategic":
    "Smart Score atas ROI, keselarasan strategis, tinjauan pakar, risiko, permintaan, prioritas MoSCoW, dan momentum. Komponen tanpa bukti dibuang dan sisanya dinormalkan ulang —",
  "p.prioritization.unscored": "belum dinilai",
  "p.prioritization.value": "Nilai",
  "p.prioritization.weight": "Bobot",
  "p.proposals.actions": "Tindakan",
  "p.proposals.currency": "Mata uang",
  "p.proposals.live": "aktif:",
  "p.proposals.promote": "promosikan",
  "p.proposals.proposal": "usulan:",
  "p.proposals.proposals_ppm": "Usulan — PPM",
  "p.proposals.requested": "Diminta",
  "p.proposals.status": "Status",
  "p.proposals.status_filter": "Filter status",
  "p.proposals.target": "Target",
  "p.proposals.target_collection": "Koleksi target",
  "p.proposals.title": "Judul",
  "p.regulator.actual": "Aktual",
  "p.regulator.benefits": "Manfaat:",
  "p.regulator.currency": "Mata uang",
  "p.regulator.gate_decisions": "· Keputusan gerbang:",
  "p.regulator.members": "Anggota:",
  "p.regulator.names_withheld_abac_mask_obligation":
    "Nama disembunyikan (kewajiban penyamaran ABAC).",
  "p.regulator.no_budget_lines": "Tidak ada baris anggaran.",
  "p.regulator.no_portfolios_recorded": "Belum ada portofolio yang tercatat.",
  "p.regulator.none": "tidak ada",
  "p.regulator.planned": "Terencana",
  "p.regulator.ppm": "{t} — PPM",
  "p.regulator.stage": "tahap: {v}",
  "p.regulator.target_realized": "target {money} / terealisasi {money2}",
  "p.reports.collection": "Koleksi",
  "p.reports.csv": "CSV",
  "p.reports.fields_comma_separated": "bidang (dipisahkan koma)",
  "p.reports.name_contains": "nama mengandung…",
  "p.reports.reports_ppm": "Laporan — PPM",
  "p.resources.a_multi_skill_booking_counts":
    "Pemesanan multikeahlian dihitung penuh untuk setiap keahliannya, sehingga baris keahlian saling tumpang tindih dan tidak berjumlah sama dengan total.",
  "p.resources.by": "Menurut",
  "p.resources.demand_fte": "Permintaan FTE",
  "p.resources.demand_supply_and_gap_by":
    "Permintaan, pasokan, dan kesenjangan menurut periode dan {group_by}",
  "p.resources.every_figure_as_a_table": "Setiap angka dalam bentuk tabel",
  "p.resources.from": "Dari",
  "p.resources.gap_fte": "Kesenjangan FTE",
  "p.resources.gap_per_in_fte_supply":
    "Kesenjangan per {period}, dalam FTE (pasokan dikurangi permintaan)",
  "p.resources.loading": "Memuat…",
  "p.resources.month": "Bulan",
  "p.resources.no_demand_or_supply_in":
    "Tidak ada permintaan atau pasokan di jendela ini.",
  "p.resources.not_known": "tidak diketahui",
  "p.resources.of_which_unassigned": "Di antaranya belum ditugaskan",
  "p.resources.people": "Orang",
  "p.resources.per": "Per",
  "p.resources.per_by_scope":
    "{periodLabel} · per {period} · menurut {group_by} · cakupan: {kind}",
  "p.resources.period": "Periode",
  "p.resources.proposed_fte": "FTE diusulkan",
  "l.processes.name_problem":
    "Beri proses nama sepanjang 1 hingga 120 karakter.",
  "l.processes.steps_problem": "Sebuah proses memiliki 1 hingga {max} langkah.",
  "l.processes.title_problem":
    "Langkah {n}: beri judul sepanjang 1 hingga 200 karakter.",
  "l.processes.person_problem":
    "Langkah {n}: orangnya harus berupa referensi person: atau worker:.",
  "l.processes.kind_approve": "Setujui",
  "l.processes.kind_todo": "Dikerjakan",
  "l.processes.kind_notify": "Beri tahu",
  "l.processes.state_pending": "belum tercapai",
  "l.processes.state_waiting": "menunggu",
  "l.processes.state_completed": "selesai",
  "l.processes.state_skipped": "dilewati",
  "l.processes.state_rejected": "ditolak",
  "l.processes.state_failed": "gagal",
  "l.processes.run_running": "berjalan",
  "l.processes.run_completed": "selesai",
  "l.processes.run_rejected": "ditolak",
  "l.processes.run_cancelled": "dibatalkan",
  "l.processes.run_failed": "gagal",
  "p.processes.processes": "Proses persetujuan",
  "p.processes.processes_ppm": "Proses persetujuan — PPM",
  "p.processes.intro":
    "Proses adalah daftar langkah yang dijalankan satu per satu: meminta seseorang menyetujui, memberi seseorang tugas, atau memberi tahu seseorang. Penolakan mengakhiri eksekusi dan melewati langkah setelahnya. Mengubah proses tidak pernah memengaruhi eksekusi yang sedang berjalan.",
  "p.processes.definitions": "Proses",
  "p.processes.none_defined": "Belum ada proses yang ditetapkan.",
  "p.processes.version": "versi {v}",
  "p.processes.new_process": "Tetapkan proses",
  "p.processes.name": "Nama",
  "p.processes.step_n": "Langkah {n}",
  "p.processes.kind": "Fungsinya",
  "p.processes.person": "Orang (person:… atau worker:…)",
  "p.processes.step_title": "Isinya",
  "p.processes.add_step": "Tambah langkah",
  "p.processes.remove_step": "Hapus langkah {n}",
  "p.processes.create": "Tetapkan proses",
  "p.processes.defined": "{name} ditetapkan.",
  "p.processes.start_heading": "Mulai eksekusi {name}",
  "p.processes.subject_kind": "Tentang apa",
  "p.processes.subject_pid": "Pengenalnya",
  "p.processes.start": "Mulai eksekusi",
  "p.processes.started": "Eksekusi {name} dimulai.",
  "p.processes.runs": "Eksekusi Anda",
  "p.processes.none_runs": "Anda belum memulai eksekusi apa pun.",
  "p.processes.run_of": "{name} pada {kind} {pid}",
  "p.processes.status": "Status",
  "p.processes.show_run": "Tampilkan {name}",
  "p.processes.hide_run": "Sembunyikan {name}",
  "p.processes.steps": "Langkah",
  "p.processes.log": "Apa yang terjadi",
  "p.processes.complete_step": "Tandai langkah {n} selesai",
  "p.processes.cancel": "Batalkan eksekusi",
  "p.processes.resume": "Periksa persetujuan lagi",
  "p.processes.done": "Selesai.",
  "p.processes.could_not": "Tidak berhasil: {reason}",
  "p.processes.could_not_load": "Tidak dapat memuat proses",
  "l.tolerance.green": "dalam toleransi",
  "l.tolerance.amber": "mendekati batas",
  "l.tolerance.red": "melebihi toleransi",
  "l.tolerance.unknown": "tidak dapat diukur",
  "l.tolerance.not_set": "belum ada batas",
  "l.tolerance.time": "Waktu",
  "l.tolerance.schedule": "Jadwal terhadap baseline",
  "l.tolerance.cost": "Biaya",
  "l.tolerance.risk": "Risiko",
  "l.tolerance.resources": "Sumber daya",
  "l.tolerance.day_one": "{n} hari",
  "l.tolerance.day_other": "{n} hari",
  "l.tolerance.score": "skor {n}",
  "l.tolerance.points": "{n} poin di atas 100%",
  "l.tolerance.field_time": "Batas (hari kalender terlambat)",
  "l.tolerance.field_schedule":
    "Batas (hari kalender lebih lambat dari baseline)",
  "l.tolerance.field_cost": "Batas (persen di atas anggaran yang disetujui)",
  "l.tolerance.field_risk":
    "Batas (skor risiko tertinggi yang dapat diterima, 1 hingga 25)",
  "l.tolerance.field_resources": "Batas (poin persentase pemesanan berlebih)",
  "l.tolerance.no_limit":
    "Tidak ada batas yang ditetapkan di sini atau di atasnya.",
  "l.tolerance.could_not_measure": "Tidak dapat diukur.",
  "l.tolerance.zero_tolerance":
    "Toleransi nol: penyimpangan apa pun melebihi batas.",
  "l.tolerance.used": "{percent}% dari batas terpakai",
  "l.tolerance.own": "Ditetapkan pada rencana ini",
  "l.tolerance.inherited": "Diwarisi dari rencana di atasnya",
  "p.issue_settings.title": "Jenis isu, status, dan peran",
  "p.issue_settings.title_ppm": "Jenis isu, status, dan peran — PPM",
  "p.issue_settings.intro":
    "Kosakata isu milik organisasi. Hanya administrator yang dapat mengubahnya. Bila belum ada yang diatur, isu memakai jenis dan status bawaan.",
  "p.issue_settings.types_heading": "Jenis isu",
  "p.issue_settings.types_intro":
    "Jenis yang dipensiunkan tidak dapat dipilih lagi. Isu yang memilikinya tetap mempertahankannya.",
  "p.issue_settings.key": "Kunci",
  "p.issue_settings.label": "Nama",
  "p.issue_settings.state": "Keadaan",
  "p.issue_settings.in_use": "Dipakai",
  "p.issue_settings.retired": "Dipensiunkan",
  "p.issue_settings.retire": "Pensiunkan",
  "p.issue_settings.restore": "Pulihkan",
  "p.issue_settings.actions": "Tindakan",
  "p.issue_settings.add_type": "Tambah jenis",
  "p.issue_settings.roles_heading": "Peran",
  "p.issue_settings.roles_intro":
    "Peran yang diakui organisasi ini, dan apa yang diizinkan tiap peran pada rencana yang memiliki anggota peran. Peran hanya dapat mempersempit apa yang sudah diizinkan oleh login seseorang. Alur kerja hanya dapat menyebut peran yang terdaftar, sehingga perpindahan tidak dapat dibatasi pada peran yang salah eja.",
  "p.issue_settings.description": "Deskripsi",
  "p.issue_settings.add_role": "Tambah peran",
  "p.issue_settings.workflows_heading": "Status dan perpindahan",
  "p.issue_settings.workflows_intro":
    "Alur kerja menetapkan status sebuah jenis isu, siapa yang boleh melakukan tiap perpindahan, dan bidang apa yang diwajibkan atau dikunci tiap status. Alur kerja untuk satu jenis lebih diutamakan daripada yang untuk semua jenis. Isu dalam status yang Anda hapus tetap mempertahankannya.",
  "p.issue_settings.none_workflows":
    "Belum ada alur kerja isu yang terdaftar, sehingga isu memakai status bawaan.",
  "p.issue_settings.wf_name": "Nama alur kerja",
  "p.issue_settings.wf_type": "Berlaku untuk",
  "p.issue_settings.wf_all_types": "Semua jenis tanpa alur kerja sendiri",
  "p.issue_settings.wf_plan": "Satu rencana",
  "p.issue_settings.statuses": "Status",
  "p.issue_settings.category": "Arti",
  "p.issue_settings.initial": "Mulai di sini",
  "p.issue_settings.terminal": "Memerlukan resolusi",
  "p.issue_settings.add_status": "Tambah status",
  "p.issue_settings.moves": "Perpindahan yang diizinkan",
  "p.issue_settings.moves_intro":
    "Bila tidak ada perpindahan yang terdaftar, setiap perpindahan diizinkan. Setelah ada yang terdaftar, hanya yang terdaftar yang diizinkan.",
  "p.issue_settings.from": "Dari",
  "p.issue_settings.to": "Ke",
  "p.issue_settings.move_roles":
    "Peran (dipisahkan koma; kosong untuk siapa saja)",
  "p.issue_settings.add_move": "Tambah perpindahan",
  "p.issue_settings.remove": "Hapus",
  "p.issue_settings.register": "Daftarkan alur kerja",
  "p.issue_settings.withdraw": "Tarik",
  "p.issue_settings.saved": "Tersimpan.",
  "p.issue_settings.could_not_load": "Tidak dapat memuat pengaturan isu.",
  "p.issue_settings.could_not_save":
    "Tidak dapat menyimpan. Hanya administrator yang dapat mengubah pengaturan ini, dan setiap kunci harus baru serta berformat benar.",
  "p.issue_settings.cat_todo": "Belum dimulai",
  "p.issue_settings.cat_active": "Sedang dikerjakan",
  "p.issue_settings.cat_waiting": "Menunggu",
  "p.issue_settings.cat_done": "Selesai",
  "p.issue_settings.translations":
    "Terjemahan (bahasa=nama, dipisahkan titik koma)",
  "p.issue_settings.bad_translations":
    "Tulis setiap terjemahan sebagai bahasa=nama, misalnya cy=Risg.",
  "p.issue_settings.save": "Simpan",
  "p.issue_settings.permissions": "Izin",
  "p.issue_settings.perm_issue_add": "Mengajukan isu",
  "p.issue_settings.perm_issue_edit": "Mengubah isu",
  "p.issue_settings.perm_issue_move": "Memindahkan isu",
  "p.issue_settings.perm_issue_delete": "Menghapus isu",
  "p.issue_settings.edit": "Ubah",
  "p.issue_settings.save_workflow": "Simpan alur kerja",
  "p.issue_settings.cancel": "Batal",
  "p.issue_settings.editing": "Mengubah {name}",
  "p.issue_settings.rule_field": "Bidang",
  "p.issue_settings.rule_kind": "Aturan",
  "p.issue_settings.rule_required": "Wajib",
  "p.issue_settings.rule_read_only": "Hanya baca",
  "p.issue_settings.rule_roles":
    "Peran (dipisahkan koma; kosong untuk semua orang)",
  "p.issue_settings.add_rule": "Tambah aturan bidang",
  "p.issue_settings.field_title": "Judul",
  "p.issue_settings.field_description": "Deskripsi",
  "p.issue_settings.field_severity": "Tingkat keparahan",
  "p.issue_settings.field_priority": "Prioritas",
  "p.issue_settings.field_owner_ref": "Pemilik",
  "p.issue_settings.field_due_date": "Tenggat",
  "p.issue_settings.field_resolution": "Resolusi",
  "p.plan_settings.title": "Pengaturan isu untuk rencana ini",
  "p.plan_settings.title_ppm": "Pengaturan isu untuk rencana ini — PPM",
  "p.plan_settings.intro":
    "Siapa memegang peran apa di sini, jenis isu apa yang ditawarkan rencana ini, dan alur kerjanya sendiri. Administrator yang membuat perubahan ini.",
  "p.plan_settings.back": "Kembali ke isu rencana ini",
  "p.plan_settings.all_settings": "Pengaturan untuk seluruh organisasi",
  "p.plan_settings.members_heading": "Orang dan peran",
  "p.plan_settings.members_intro":
    "Bila rencana memiliki setidaknya satu orang yang terdaftar di sini, mengajukan, mengubah, memindahkan, dan menghapus isu padanya memerlukan peran yang mengizinkannya. Peran tidak dapat memberi seseorang lebih dari yang diizinkan login-nya, dan tidak membuka rencana privat.",
  "p.plan_settings.enforced": "Tindakan isu pada rencana ini memerlukan peran.",
  "p.plan_settings.not_enforced":
    "Siapa pun yang dapat menulis dapat bertindak pada isu di sini.",
  "p.plan_settings.person": "ID pengguna orang",
  "p.plan_settings.role": "Peran",
  "p.plan_settings.choose_role": "Pilih peran",
  "p.plan_settings.add_member": "Tambah orang",
  "p.plan_settings.types_heading": "Jenis isu pada rencana ini",
  "p.plan_settings.types_intro":
    "Centang jenis yang ditawarkan rencana ini. Bila tidak ada yang dicentang, rencana menawarkan semua jenis. Isu tetap mempertahankan jenis yang Anda keluarkan.",
  "p.plan_settings.save_types": "Simpan jenis",
  "p.plan_settings.workflows_intro":
    "Alur kerja di sini hanya berlaku untuk rencana ini, dan lebih diutamakan daripada milik organisasi. Alur kerja untuk satu jenis lebih diutamakan daripada yang untuk semua jenis. Isu dalam status yang Anda hapus tetap mempertahankannya.",
  "p.plan_issues.settings_link": "Pengaturan isu untuk rencana ini",
  "p.plan_issues.problem_fields_required": "Status ini memerlukan: {fields}.",
  "p.tolerances.tolerances": "Toleransi",
  "p.tolerances.tolerances_ppm": "Toleransi — PPM",
  "p.tolerances.plan_link": "Toleransi",
  "p.tolerances.intro":
    "Seberapa jauh tiap proyek boleh menyimpang dalam waktu, biaya, risiko, dan sumber daya sebelum harus diteruskan ke atas, dan seberapa banyak tiap batas terpakai. Dalam batas tetapi mendekatinya berwarna kuning; melampauinya merah; batas yang tidak dapat diukur ditampilkan sebagai tidak dapat diukur, tidak pernah sebagai hijau.",
  "p.tolerances.board_intro":
    "Setiap rencana yang dapat Anda baca dan memiliki toleransi, yang terburuk lebih dulu.",
  "p.tolerances.none": "Belum ada rencana yang memiliki toleransi.",
  "p.tolerances.none_match": "Tidak ada rencana yang cocok.",
  "p.tolerances.show": "Tampilkan",
  "p.tolerances.all_statuses": "Semua warna",
  "p.tolerances.plan": "Rencana",
  "p.tolerances.overall": "Keseluruhan",
  "p.tolerances.truncated":
    "Hanya {n} rencana pertama yang diukur, sehingga daftar ini mungkin tidak lengkap.",
  "p.tolerances.loading": "Memuat…",
  "p.tolerances.could_not_load": "Tidak dapat memuat toleransi",
  "p.tolerances.back": "← Rencana",
  "p.tolerances.board_link": "Semua toleransi",
  "p.tolerances.limit": "Batas",
  "p.tolerances.measured": "Terukur",
  "p.tolerances.how": "Cara pengukurannya",
  "p.tolerances.amber_from": "Kuning mulai dari (persen dari batas)",
  "p.tolerances.note": "Catatan",
  "p.tolerances.save": "Tetapkan batas",
  "p.tolerances.remove": "Hapus batas milik rencana ini",
  "p.tolerances.saved": "Batas {dimension} tersimpan.",
  "p.tolerances.removed": "Batas {dimension} dihapus.",
  "p.tolerances.could_not": "Tidak berhasil: {reason}",
  "p.tolerances.limit_problem": "Berikan batas sebagai angka.",
  "l.escalation.open": "terbuka",
  "l.escalation.acknowledged": "diakui",
  "l.escalation.resolved": "terselesaikan",
  "l.escalation.withdrawn": "ditarik",
  "l.escalation.reason_tolerance": "Toleransi terlampaui",
  "l.escalation.reason_risk": "Sebuah risiko",
  "l.escalation.reason_issue": "Sebuah isu",
  "l.escalation.reason_dependency": "Sebuah ketergantungan",
  "l.escalation.reason_decision": "Diperlukan keputusan",
  "l.escalation.reason_resource": "Sumber daya",
  "l.escalation.reason_other": "Hal lain",
  "l.escalation.outcome_decided": "Diputuskan",
  "l.escalation.outcome_guidance_given": "Arahan diberikan",
  "l.escalation.outcome_rejected": "Ditolak",
  "l.escalation.event_raised": "Diajukan",
  "l.escalation.event_acknowledged": "Diakui",
  "l.escalation.event_level_raised": "Dinaikkan satu tingkat",
  "l.escalation.event_resolved": "Diselesaikan",
  "l.escalation.event_withdrawn": "Ditarik",
  "l.escalation.title_problem": "Beri judul sepanjang 1 hingga 200 karakter.",
  "l.escalation.dimension_problem": "Sebutkan toleransi mana yang dimaksud.",
  "l.escalation.level_problem":
    "Tingkat adalah bilangan bulat dari 1 hingga 9.",
  "l.escalation.owner_problem":
    "Orangnya harus berupa referensi person: atau worker:.",
  "l.escalation.level_to": "Tingkat {level}: {to}",
  "l.escalation.level": "Tingkat {level}",
  "l.escalation.open_for": "Terbuka selama {n} hari",
  "l.escalation.took": "Memakan waktu {n} hari",
  "l.escalation.overdue": "terlambat",
  "l.escalation.automatic": "Diajukan otomatis",
  "p.escalations.escalations": "Eskalasi",
  "p.escalations.escalations_ppm": "Eskalasi — PPM",
  "p.escalations.plan_link": "Eskalasi",
  "p.task.task": "Tugas",
  "p.task.task_ppm": "Tugas — PPM",
  "p.task.back": "Kembali ke struktur rincian kerja",
  "p.task.could_not_load": "Tidak dapat memuat tugas.",
  "p.task.loading": "Memuat…",
  "p.task.status": "Status",
  "p.task.due": "Tenggat",
  "p.task.assignee": "Ditugaskan kepada",
  "p.task.description": "Deskripsi",
  "p.task.s_todo": "Akan dikerjakan",
  "p.task.s_in_progress": "Sedang berjalan",
  "p.task.s_in_review": "Dalam tinjauan",
  "p.task.s_done": "Selesai",
  "p.task.s_blocked": "Terhambat",
  "p.task.links_intro":
    "Di mana tugas ini berada di alat lain, seperti isu Jira atau Linear tempat tugas ini dilacak. Alamatnya disimpan dan ditampilkan; tidak pernah diambil dan tidak ada yang disinkronkan.",
  "p.task.back_board": "Kembali ke papan",
  "p.task.could_not_load_links":
    "Tidak dapat memuat tautan tugas ke tugas lain.",
  "p.task.could_not_load_tasks":
    "Tidak dapat memuat tugas rencana untuk ditautkan.",
  "p.task.edit_heading": "Ubah tugas",
  "p.task.edit_note":
    "Status berubah di papan. Sprint, komponen, dan tugas induk dibiarkan apa adanya.",
  "p.task.f_title": "Judul",
  "p.task.f_assignee":
    "Ditugaskan kepada (referensi person: atau worker:; kosongkan jika tidak ada)",
  "p.task.f_points":
    "Poin cerita (bilangan bulat dari 0 hingga 100; kosongkan jika tidak ada)",
  "p.task.f_due": "Tenggat (kosongkan jika tidak ada)",
  "p.task.save": "Simpan tugas",
  "p.task.saved": "Tugas tersimpan.",
  "p.task.problem_title_required": "Beri tugas sebuah judul.",
  "p.task.problem_title_too_long": "Judul terlalu panjang.",
  "p.task.problem_assignee":
    "Sebutkan penanggung jawab sebagai referensi person: atau worker:, atau kosongkan.",
  "p.task.problem_points_whole": "Poin cerita berupa bilangan bulat.",
  "p.task.problem_points_range": "Poin cerita harus dari 0 hingga 100.",
  "p.task.problem_due": "Tenggat bukan tanggal.",
  "p.task.relations_heading": "Tautan ke tugas lain",
  "p.task.relations_intro":
    "‘Menghambat’ mencegah tugas lain diselesaikan selama tugas ini terbuka. ‘Mendahului’ mengharuskan tugas lain jatuh tempo setelah tugas ini. Saat sebuah tugas selesai, tugas yang merupakan duplikatnya ikut selesai. ‘Terkait dengan’ dan ‘Disalin dari’ hanya untuk informasi.",
  "p.task.no_relations": "Tugas ini tidak memiliki tautan ke tugas lain.",
  "p.task.rel_related": "Terkait dengan",
  "p.task.rel_blocks": "Menghambat",
  "p.task.rel_blocked_by": "Dihambat oleh",
  "p.task.rel_precedes": "Mendahului",
  "p.task.rel_follows": "Mengikuti",
  "p.task.rel_duplicates": "Duplikat dari",
  "p.task.rel_duplicated_by": "Diduplikasi oleh",
  "p.task.rel_copied_from": "Disalin dari",
  "p.task.rel_copied_to": "Disalin ke",
  "p.task.other_state": "(kini {status})",
  "p.task.other_state_due": "(kini {status}, tenggat {date})",
  "p.task.remove_link": "Hapus",
  "p.task.remove_link_named": "Hapus tautan: {link} {title}",
  "p.task.hidden_links_one":
    "{n} tautan lagi menuju tugas di rencana yang tidak dapat Anda lihat.",
  "p.task.hidden_links_other":
    "{n} tautan lagi menuju tugas di rencana yang tidak dapat Anda lihat.",
  "p.task.add_link_heading": "Tautkan ke tugas lain",
  "p.task.f_kind": "Tugas ini",
  "p.task.f_other": "Tugas lain",
  "p.task.choose_task": "Pilih tugas",
  "p.task.add_link": "Tautkan tugas",
  "p.task.link_added": "Tautan ditambahkan.",
  "p.task.link_removed": "Tautan dihapus.",
  "p.task.problem_choose_task": "Pilih tugas lainnya.",
  "p.task.problem_self": "Tugas tidak dapat ditautkan ke dirinya sendiri.",
  "p.task.problem_exists": "Tugas ini sudah memiliki tautan itu.",
  "p.task.problem_cycle":
    "Tugas lain sudah memiliki tautan itu ke tugas ini; keduanya akan saling menunggu.",
  "p.task.problem_precedes_dates":
    "‘Mendahului’ mengharuskan tugas lain jatuh tempo setelah tugas ini: ubah tenggat dulu.",
  "p.task.refused_forbidden": "Anda tidak boleh mengubah tugas rencana ini.",
  "p.task.refused_gone":
    "Tugas ini sudah tidak ada, atau Anda tidak dapat melihatnya; data telah dibaca ulang.",
  "p.task.refused_link_gone":
    "Tautan itu sudah dihapus; tautan telah dibaca ulang.",
  "p.task.refused_link_exists":
    "Tautan itu sudah ada; tautan telah dibaca ulang.",
  "p.task.refused_edit_invalid":
    "Layanan tidak menerima perubahan: periksa bidangnya. Tenggat baru juga harus menjaga setiap tautan ‘Mendahului’ tetap berurutan.",
  "p.task.refused_link_invalid":
    "Layanan tidak menerima tautan: tugas lain mungkin sudah tidak ada, tautan ‘Menghambat’ atau ‘Mendahului’ tidak boleh berputar membentuk lingkaran, dan ‘Mendahului’ mengharuskan tugas lain jatuh tempo setelah tugas ini.",
  "p.me.notifications_link": "Notifikasi saya",
  "p.me.notifications_note":
    "— kotak masuk Anda, apa yang diberitahukan kepada Anda, dan ke mana salinannya dikirim.",
  "p.notifications.title_ppm": "Notifikasi saya — PPM",
  "p.notifications.heading": "Notifikasi saya",
  "p.notifications.back": "← Pekerjaan saya",
  "p.notifications.intro":
    "Semua yang ada di sini hanya milik Anda: tidak ada orang lain yang dapat membaca kotak masuk, pilihan, atau tujuan Anda.",
  "p.notifications.loading": "Memuat…",
  "p.notifications.inbox_heading": "Kotak masuk",
  "p.notifications.unread_only": "Tampilkan hanya yang belum dibaca",
  "p.notifications.unread_count_one": "{n} notifikasi belum dibaca.",
  "p.notifications.unread_count_other": "{n} notifikasi belum dibaca.",
  "p.notifications.inbox_tz": "Hari dihitung dalam zona waktu Anda, {tz}.",
  "p.notifications.inbox_empty": "Tidak ada apa-apa di kotak masuk Anda.",
  "p.notifications.inbox_unread_empty": "Tidak ada yang belum dibaca.",
  "p.notifications.status_unread": "Belum dibaca",
  "p.notifications.status_read": "Sudah dibaca",
  "p.notifications.digest_count_one": "ringkasan harian dari {n} notifikasi",
  "p.notifications.digest_count_other": "ringkasan harian dari {n} notifikasi",
  "p.notifications.open_subject": "Buka rencana",
  "p.notifications.mark_read": "Tandai sudah dibaca",
  "p.notifications.mark_read_named": "Tandai sudah dibaca: {message}",
  "p.notifications.marked_read": "Ditandai sudah dibaca.",
  "p.notifications.could_not_load_inbox":
    "Tidak dapat memuat kotak masuk Anda.",
  "p.notifications.could_not_mark":
    "Tidak dapat menandai notifikasi sudah dibaca.",
  "p.notifications.prefs_heading": "Apa yang diberitahukan kepada Anda",
  "p.notifications.prefs_intro":
    "Hidupkan atau matikan tiap jenis notifikasi untuk kotak masuk dan untuk tiap jenis tujuan. Salinan dikirim ke webhook, obrolan, atau email hanya bila Anda telah menambahkan tujuan jenis itu di bawah. Ringkasan harian mengumpulkan notifikasi satu hari tentang satu hal menjadi satu baris di kotak masuk; tidak ada yang ditahan atau dibuang.",
  "p.notifications.pref_daily": "Ringkasan harian di kotak masuk",
  "p.notifications.pref_saved": "Pilihan Anda tersimpan.",
  "p.notifications.pref_reset_done":
    "Kembali ke bawaan: hidup, sesuai kedatangannya.",
  "p.notifications.reset": "Kembali ke bawaan",
  "p.notifications.reset_named": "Kembali ke bawaan: {kind}",
  "p.notifications.could_not_load_prefs": "Tidak dapat memuat pilihan Anda.",
  "p.notifications.could_not_save_pref": "Tidak dapat menyimpan pilihan.",
  "p.notifications.sinks_heading": "Ke mana salinan dikirim",
  "p.notifications.sinks_intro":
    "Tujuan menerima salinan setiap notifikasi Anda, yang hanya memuat teks notifikasi itu sendiri. Alamatnya rahasia: setelah tersimpan hanya ditampilkan dalam bentuk singkat. Tujuan yang lambat atau rusak tidak pernah menahan kotak masuk; pengiriman yang gagal dicoba lagi empat kali dalam sekitar dua setengah jam, lalu dihentikan.",
  "p.notifications.no_sinks":
    "Anda tidak punya tujuan; notifikasi Anda hanya masuk ke kotak masuk.",
  "p.notifications.chat_named": "Obrolan {name}",
  "p.notifications.sink_sending": "Mengirim",
  "p.notifications.sink_paused": "Dijeda",
  "p.notifications.show_deliveries": "Pengiriman terbaru",
  "p.notifications.deliveries_named": "Pengiriman terbaru: {destination}",
  "p.notifications.remove_sink": "Hapus",
  "p.notifications.remove_sink_named": "Hapus: {destination}",
  "p.notifications.sink_removed": "Tujuan dihapus.",
  "p.notifications.deliveries_heading": "Pengiriman terbaru ke {destination}",
  "p.notifications.deliveries_note":
    "Alasan pengiriman gagal tidak ditampilkan di sini, karena alasan itu dapat mengutip alamat rahasia.",
  "p.notifications.no_deliveries": "Belum ada yang dikirim ke tujuan ini.",
  "p.notifications.attempts_one": "{n} percobaan",
  "p.notifications.attempts_other": "{n} percobaan",
  "p.notifications.delivered_at": "terkirim {date}",
  "p.notifications.next_attempt": "percobaan berikutnya {date}",
  "p.notifications.close_deliveries": "Tutup daftar pengiriman",
  "p.notifications.add_heading": "Tambah tujuan",
  "p.notifications.f_kind": "Jenis",
  "p.notifications.f_destination_email": "Alamat email",
  "p.notifications.f_destination_url": "Alamat web (https, ke host publik)",
  "p.notifications.f_chat_id":
    "Obrolan Telegram (nomornya, atau @ dan namanya)",
  "p.notifications.add_sink": "Tambah tujuan",
  "p.notifications.sink_added": "Tujuan ditambahkan: {destination}.",
  "p.notifications.could_not_load_sinks": "Tidak dapat memuat tujuan Anda.",
  "p.notifications.could_not_load_deliveries": "Tidak dapat memuat pengiriman.",
  "p.notifications.could_not_save_sink": "Tidak dapat menambah tujuan.",
  "p.notifications.could_not_remove_sink": "Tidak dapat menghapus tujuan.",
  "p.notifications.problem_too_many":
    "Anda sudah punya sepuluh tujuan, jumlah maksimum yang diizinkan: hapus satu dulu.",
  "p.notifications.problem_destination_required":
    "Berikan alamat tujuan pengiriman.",
  "p.notifications.problem_email_invalid": "Itu bukan alamat email.",
  "p.notifications.problem_url_invalid": "Itu bukan alamat web.",
  "p.notifications.problem_url_not_https": "Alamat web harus diawali https://.",
  "p.notifications.problem_url_credentials":
    "Alamat web tidak boleh memuat nama pengguna atau kata sandi.",
  "p.notifications.problem_url_private":
    "Alamat web harus berupa host publik, bukan host lokal, privat, atau internal.",
  "p.notifications.problem_url_too_long": "Alamat web terlalu panjang.",
  "p.notifications.problem_chat_id_required":
    "Tujuan Telegram memerlukan obrolan tempat memposting.",
  "p.notifications.refused_forbidden":
    "Anda tidak boleh mengubah pengaturan notifikasi ini.",
  "p.notifications.refused_note_gone":
    "Notifikasi itu sudah tidak ada, atau bukan milik Anda; kotak masuk telah dibaca ulang.",
  "p.notifications.refused_sink_gone":
    "Tujuan itu sudah dihapus; tujuan Anda telah dibaca ulang.",
  "p.notifications.refused_not_addressable":
    "Identitas masuk Anda bukan identitas yang dapat dituju notifikasi, sehingga tidak ada yang ditampilkan di sini.",
  "p.notifications.refused_pref_invalid":
    "Layanan tidak menerima pilihan itu; pilihan Anda telah dibaca ulang.",
  "p.notifications.refused_sink_invalid":
    "Layanan tidak menerima tujuan itu: harus berupa alamat email, atau alamat web https ke host publik; tujuan Telegram memerlukan obrolannya; dan maksimal sepuluh.",
  "l.enum.notify_kind.approval_decided":
    "Persetujuan yang Anda minta telah diputuskan",
  "l.enum.notify_kind.approval_requested": "Anda diminta menyetujui sesuatu",
  "l.enum.notify_kind.auto_watch":
    "Mengikuti apa yang Anda kerjakan, secara otomatis",
  "l.enum.notify_kind.automation": "Sebuah aturan memberi tahu Anda",
  "l.enum.notify_kind.comment": "Komentar",
  "l.enum.notify_kind.escalation": "Eskalasi",
  "l.enum.notify_kind.exceptions_digest": "Ringkasan pengecualian mingguan",
  "l.enum.notify_kind.reschedule_proposed": "Tanggal baru diusulkan",
  "l.enum.notify_kind.review_invited": "Anda diundang untuk meninjau",
  "l.enum.notify_kind.review_submitted": "Sebuah tinjauan diajukan",
  "l.enum.notify_kind.scheduled": "Pengingat terjadwal",
  "l.enum.notify_kind.status_update_due": "Pembaruan status jatuh tempo",
  "l.enum.notify_kind.task_assigned": "Sebuah tugas diberikan kepada Anda",
  "l.enum.notify_kind.watch_update": "Sesuatu yang Anda ikuti berubah",
  "l.enum.notify_kind.process_todo": "Langkah proses menjadi tugas Anda",
  "l.enum.notify_kind.process_notice": "Langkah proses yang Anda diberi tahu",
  "l.enum.notify_kind.break_glass":
    "Rencana privat dibuka dalam keadaan darurat",
  "l.enum.notify_channel.in_app": "Di kotak masuk",
  "l.enum.notify_channel.webhook": "Lewat webhook bertanda tangan",
  "l.enum.notify_channel.chat": "Di obrolan",
  "l.enum.notify_channel.email": "Lewat email",
  "l.enum.sink_kind.email": "Email",
  "l.enum.sink_kind.webhook": "Webhook bertanda tangan",
  "l.enum.delivery_status.pending": "Menunggu dikirim",
  "l.enum.delivery_status.sent": "Terkirim",
  "l.enum.delivery_status.failed": "Dihentikan",
  "l.enum.delivery_status.skipped":
    "Tidak dikirim: tujuan atau notifikasi sudah tidak ada",
  "l.link.kind_docs": "Dokumen",
  "l.link.kind_repo": "Repositori kode",
  "l.link.kind_ticket": "Tiket",
  "l.link.kind_design": "Desain",
  "l.link.kind_chat": "Obrolan",
  "l.link.kind_tracker": "Pelacak pekerjaan",
  "l.link.kind_other": "Lainnya",
  "l.link.system_other": "Alat lain",
  "l.fw.method_waterfall": "Waterfall",
  "l.fw.method_hybrid": "Hibrida",
  "l.fw.method_other": "Lainnya",
  "l.fw.model_sponsor": "Sponsor memutuskan",
  "l.fw.model_steering_committee": "Komite pengarah",
  "l.fw.model_product_owner": "Pemilik produk",
  "l.fw.model_delegated": "Didelegasikan",
  "l.fw.model_consensus": "Konsensus",
  "l.fw.model_other": "Lainnya",
  "p.frameworks.wow": "Cara kerja",
  "p.frameworks.wow_ppm": "Cara kerja — PPM",
  "p.frameworks.gov": "Kerangka tata kelola",
  "p.frameworks.gov_ppm": "Kerangka tata kelola — PPM",
  "p.frameworks.intro_wow":
    "Cara tim bekerja: metode, panjang iterasi, batas pekerjaan dalam proses. Rencana mengikuti satu, bersama atau miliknya sendiri; bila tidak ada, rencana mewarisi milik rencana terdekat di atasnya.",
  "p.frameworks.intro_gov":
    "Cara pekerjaan ditata kelola: siapa yang memutuskan, seberapa sering ditinjau, dan jalur eskalasi yang diikuti eskalasi. Rencana mengikuti satu, bersama atau miliknya sendiri; bila tidak ada, rencana mewarisi milik rencana terdekat di atasnya.",
  "p.frameworks.back": "Kembali ke rencana",
  "p.frameworks.loading": "Memuat…",
  "p.frameworks.could_not_load": "Tidak dapat memuat ini.",
  "p.frameworks.could_not": "Tidak dapat melakukannya: {reason}",
  "p.frameworks.in_force": "Berlaku untuk rencana ini",
  "p.frameworks.source_own": "diatur pada rencana ini",
  "p.frameworks.source_inherited": "diwarisi dari rencana di atasnya",
  "p.frameworks.source_none": "tidak ada",
  "p.frameworks.from_plan": "lihat rencana itu",
  "p.frameworks.scope_plan": "ditulis untuk satu rencana",
  "p.frameworks.scope_org": "dipakai bersama oleh organisasi",
  "p.frameworks.none": "Tidak ada yang berlaku.",
  "p.frameworks.edit": "Ubah",
  "p.frameworks.unfollow": "Berhenti mengikuti (mewarisi lagi)",
  "p.frameworks.follow_heading": "Ikuti yang berbeda",
  "p.frameworks.no_others": "Tidak ada yang bersama lainnya untuk diikuti.",
  "p.frameworks.choose": "Bersama atau yang ada",
  "p.frameworks.follow": "Ikuti ini",
  "p.frameworks.write_new": "Tulis yang baru untuk rencana ini",
  "p.frameworks.followed": "Kini mengikutinya.",
  "p.frameworks.unfollowed": "Berhenti mengikuti; rencana mewarisi lagi.",
  "p.frameworks.saved": "Tersimpan.",
  "p.frameworks.new_heading": "Yang baru untuk rencana ini",
  "p.frameworks.edit_heading": "Ubah yang ini",
  "p.frameworks.f_name": "Nama",
  "p.frameworks.f_description": "Deskripsi",
  "p.frameworks.save": "Simpan",
  "p.frameworks.cancel": "Batal",
  "p.frameworks.methodology": "Metode",
  "p.frameworks.cadence_days": "Panjang iterasi (hari)",
  "p.frameworks.wip_limit": "Batas pekerjaan dalam proses",
  "p.frameworks.ready": "Definisi siap",
  "p.frameworks.done": "Definisi selesai",
  "p.frameworks.ceremonies": "Upacara",
  "p.frameworks.decision_model": "Model keputusan",
  "p.frameworks.review_cadence": "Tinjau setiap (hari)",
  "p.frameworks.reporting_cadence": "Laporkan ke atas setiap (hari)",
  "p.frameworks.path": "Jalur eskalasi",
  "p.frameworks.rights": "Hak keputusan",
  "p.frameworks.forums": "Forum",
  "p.frameworks.days": "{n} hari",
  "p.frameworks.path_level": "Tingkat",
  "p.frameworks.path_body": "Siapa yang mendengarnya",
  "p.frameworks.path_trigger": "Apa yang mengirimnya ke sana",
  "p.frameworks.path_add": "Tambah tingkat",
  "p.frameworks.path_remove": "Hapus tingkat {n}",
  "p.frameworks.name_problem": "Beri nama sepanjang 1 hingga 120 karakter.",
  "p.frameworks.number_problem":
    "Hari berupa bilangan bulat dari 1 hingga 366 dan batas dari 1 hingga 1000.",
  "p.frameworks.path_problem":
    "Setiap tingkat eskalasi adalah bilangan bulat berbeda dari 1 hingga 9, dengan siapa yang mendengarnya dan apa yang mengirimnya ke sana.",
  "p.frameworks.confirm_shared_one":
    "Ini dipakai bersama: menyimpan akan mengubahnya juga untuk {n} rencana lain.",
  "p.frameworks.confirm_shared_other":
    "Ini dipakai bersama: menyimpan akan mengubahnya juga untuk {n} rencana lain.",
  "p.frameworks.confirm_problem_one":
    "Centang kotak untuk mengonfirmasi perubahan bagi {n} rencana lain.",
  "p.frameworks.confirm_problem_other":
    "Centang kotak untuk mengonfirmasi perubahan bagi {n} rencana lain.",
  "p.frameworks.deleted": "{name} dihapus.",
  "p.frameworks.delete": "Hapus {name} (tidak ada yang mengikutinya)",
  "p.frameworks.text_problem": "Definisi paling banyak 10000 karakter.",
  "p.frameworks.practice_problem":
    "Setiap upacara atau forum memerlukan nama dan seberapa sering berlangsung, dan paling banyak ada 20.",
  "p.frameworks.rights_problem":
    "Setiap hak keputusan memerlukan keputusannya dan siapa yang memutuskan, dan paling banyak ada 20.",
  "p.frameworks.practice_name": "Nama",
  "p.frameworks.practice_cadence": "Seberapa sering",
  "p.frameworks.practice_purpose": "Untuk apa",
  "p.frameworks.row_remove": "Hapus baris {n}",
  "p.frameworks.ceremony_add": "Tambah upacara",
  "p.frameworks.forum_add": "Tambah forum",
  "p.frameworks.right_decision": "Keputusan",
  "p.frameworks.right_decides": "Siapa yang memutuskan",
  "p.frameworks.right_consulted": "Siapa yang dimintai pendapat lebih dulu",
  "p.frameworks.right_add": "Tambah hak keputusan",
  "p.links.links": "Tautan",
  "p.links.links_ppm": "Tautan — PPM",
  "p.links.intro":
    "Di mana dokumen dan pekerjaan rencana ini berada, termasuk papan Jira, Trello, Asana, Linear, atau Monday miliknya. Alamatnya disimpan dan ditampilkan; tidak pernah diambil.",
  "p.links.back": "Kembali ke rencana",
  "p.links.loading": "Memuat…",
  "p.links.could_not_load": "Tidak dapat memuat tautan.",
  "p.links.could_not": "Tidak dapat melakukannya: {reason}",
  "p.links.none": "Belum ada tautan.",
  "p.links.opens_new": "terbuka di tab baru",
  "p.links.remove": "Hapus {label}",
  "p.links.added": "{label} ditambahkan.",
  "p.links.removed": "{label} dihapus.",
  "p.links.add_heading": "Tambah tautan",
  "p.links.add": "Tambah tautan",
  "p.links.f_label": "Label",
  "p.links.f_url": "Alamat (http atau https)",
  "p.links.f_kind": "Jenis",
  "p.links.f_system": "Alat eksternal",
  "p.links.f_id": "Pengenalnya di sana",
  "p.links.url_problem":
    "Alamat harus diawali http:// atau https:// dan menyebut host.",
  "p.links.label_problem": "Beri label.",
  "p.links.system_problem": "Tautan pelacak harus menyebut alatnya.",
  "p.links.system_kind_problem":
    "Hanya tautan pelacak atau tiket yang menyebut alat.",
  "p.links.id_problem": "Pengenal memerlukan alat dan tidak mengandung spasi.",
  "p.links.domain_problem": "Tautan alat itu ada di {domains}.",
  "p.escalations.intro":
    "Teruskan sesuatu ke atas saat proyek tidak dapat atau tidak seharusnya menyelesaikannya sendiri. Eskalasi diajukan pada satu tingkat jalur eskalasi dalam kerangka tata kelola proyek, diakui, dinaikkan satu tingkat bila tidak ada yang menjawab, dan berakhir terselesaikan atau ditarik. Orang yang disebut sebagai penanggung jawab dan orang yang mengajukannya diberi tahu di kotak masuk mereka (tidak ada email yang dikirim).",
  "p.escalations.register_intro":
    "Setiap eskalasi pada rencana yang dapat Anda baca.",
  "p.escalations.none": "Belum ada eskalasi.",
  "p.escalations.none_match": "Tidak ada eskalasi yang cocok.",
  "p.escalations.raise_heading": "Ajukan eskalasi",
  "p.escalations.f_title": "Judul",
  "p.escalations.f_reason": "Mengapa",
  "p.escalations.f_dimension": "Toleransi mana",
  "p.escalations.f_level": "Tingkat (kosong untuk tingkat pertama jalur)",
  "p.escalations.f_to": "Eskalasikan kepada (kosong untuk memakai jalur)",
  "p.escalations.f_owner": "Orang yang dituju (person:… atau worker:…)",
  "p.escalations.f_respond": "Jawaban diinginkan paling lambat",
  "p.escalations.f_description": "Apa dan mengapa",
  "p.escalations.raise": "Ajukan",
  "p.escalations.raised": "{title} diajukan.",
  "p.escalations.acknowledge": "Akui",
  "p.escalations.take_up": "Naikkan satu tingkat",
  "p.escalations.take_up_note": "Mengapa dinaikkan",
  "p.escalations.resolve": "Selesaikan",
  "p.escalations.outcome": "Hasil",
  "p.escalations.response": "Tanggapan",
  "p.escalations.withdraw": "Tarik",
  "p.escalations.done": "Selesai.",
  "p.escalations.could_not": "Tidak berhasil: {reason}",
  "p.error.status_401": "Anda belum masuk, atau sesi Anda telah berakhir.",
  "p.error.status_403": "Anda tidak diizinkan melakukan itu.",
  "p.error.status_404":
    "Itu tidak ditemukan, atau Anda tidak dapat melihatnya.",
  "p.error.status_409":
    "Itu bertentangan dengan keadaan saat ini: orang lain telah mengubah atau memutuskannya, atau sudah selesai.",
  "p.error.status_412":
    "Itu berubah sejak Anda memuatnya: muat ulang dan coba lagi.",
  "p.error.status_413": "Itu terlalu besar.",
  "p.error.status_422": "Sesuatu yang Anda masukkan tidak diterima.",
  "p.error.status_429":
    "Terlalu banyak permintaan: tunggu sebentar dan coba lagi.",
  "p.bridge.open_workforce": "Buka {name} di perencanaan tenaga kerja",
  "p.bridge.open_contacts": "Buka {name} di kontak",
  "p.error.client": "Permintaan ditolak.",
  "p.error.server": "Layanan mengalami masalah: coba lagi.",
  "p.escalations.note_problem": "Tulis setidaknya 10 karakter.",
  "p.escalations.show": "Tampilkan {title}",
  "p.escalations.hide": "Sembunyikan {title}",
  "p.escalations.history": "Riwayat",
  "p.escalations.status": "Status",
  "p.escalations.plan": "Rencana",
  "p.escalations.level_col": "Tingkat",
  "p.escalations.age": "Usia",
  "p.escalations.respond_by": "Jawaban diinginkan paling lambat",
  "p.escalations.filter": "Tampilkan",
  "p.escalations.all": "Semua",
  "p.escalations.live": "Masih terbuka",
  "p.escalations.overdue_count": "{n} terlambat",
  "p.escalations.loading": "Memuat…",
  "p.escalations.could_not_load": "Tidak dapat memuat eskalasi",
  "p.escalations.back": "← Rencana",
  "p.escalations.register_link": "Semua eskalasi",
  "p.tolerances.needs_escalation":
    "Ini melebihi toleransi dan belum ada yang mengeskalasikannya.",
  "p.tolerances.raise_one": "Ajukan eskalasi",
  "p.tolerances.escalate_all": "Eskalasikan setiap toleransi merah",
  "p.tolerances.escalate_all_intro":
    "Setiap dimensi yang melebihi toleransi dan belum dieskalasikan mendapat eskalasi pada tingkat pertama jalur eskalasi. Tidak ada yang dieskalasikan dengan sendirinya.",
  "p.tolerances.escalated_one": "{n} eskalasi diajukan.",
  "p.tolerances.escalated_other": "{n} eskalasi diajukan.",
  "p.tolerances.escalations_one": "{n} eskalasi terbuka",
  "p.tolerances.escalations_other": "{n} eskalasi terbuka",
  "p.resources.rbs.by_role": "Menurut peran",
  "p.resources.rbs.show_tasks": "Tampilkan tugas terbuka {role}",
  "p.resources.rbs.hide_tasks": "Sembunyikan tugas terbuka {role}",
  "p.resources.rbs.no_open_tasks": "tidak ada tugas terbuka",
  "p.resources.rbs.tasks_in_plans_you_cannot_read":
    "{count} dalam rencana yang tidak dapat Anda baca tidak ditampilkan.",
  "p.resources.rbs.only_the_first_500_tasks":
    "Hanya 500 tugas pertama yang dibaca, sehingga daftar ini mungkin tidak lengkap.",
  "p.resources.rbs.could_not_load_tasks": "Tidak dapat memuat tugas: {reason}",
  "p.resources.rbs.by_skill": "Menurut keahlian",
  "p.resources.rbs.demand_by_skill": "Permintaan menurut keahlian",
  "p.resources.rbs.demand_fte": "Permintaan FTE",
  "p.resources.rbs.no_roles_in_this_window": "Tidak ada peran di jendela ini.",
  "p.resources.rbs.no_skill_demand_in_this":
    "Tidak ada permintaan keahlian di jendela ini.",
  "p.resources.rbs.not_known": "tidak diketahui",
  "p.resources.rbs.people": "Orang",
  "p.resources.rbs.resource_breakdown": "Rincian sumber daya",
  "p.resources.rbs.resource_breakdown_ppm": "Rincian sumber daya — PPM",
  "p.resources.rbs.role": "Peran",
  "p.resources.rbs.roles_with_people_demand_and":
    "Peran beserta orang, permintaan, dan pasokan",
  "p.resources.rbs.skill": "Keahlian",
  "p.resources.rbs.supply_fte": "Pasokan FTE",
  "p.resources.rbs.you_can_see_headcounts_only":
    "Anda hanya dapat melihat jumlah orang, bukan nama.",
  "p.resources.resources": "Sumber daya",
  "p.resources.resources_ppm": "Sumber daya — PPM",
  "p.resources.role": "Peran",
  "p.resources.roles_and_totals_only": "hanya peran dan total",
  "p.resources.roles_asked_for_with_no": "Peran yang diminta tanpa orang",
  "p.resources.short_spare_balanced_supply_not":
    "▼ kurang · ▲ lebih · ● seimbang · ? pasokan tidak diketahui. Arsiran hanya menunjukkan seberapa besar kesenjangannya.",
  "p.resources.show": "Tampilkan",
  "p.resources.skill": "Keahlian",
  "p.resources.skill_demand_only": "Keahlian (hanya permintaan)",
  "p.resources.supply_fte": "Pasokan FTE",
  "p.resources.to": "Hingga",
  "p.resources.unclassified": "tidak terklasifikasi",
  "p.resources.week": "Minggu",
  "p.resources.what_the_plans_ask_for":
    "Apa yang diminta rencana dibandingkan dengan yang dapat diberikan orang di tiap peran, dalam ekuivalen penuh waktu. Pemesanan yang diusulkan dipisahkan dan tidak pernah dihitung.",
  "p.reviews.accept": "Terima",
  "p.reviews.accepted": "diterima",
  "p.reviews.actions": "Tindakan",
  "p.reviews.advance": "Majukan",
  "p.reviews.all": "semua",
  "p.reviews.collaborative_review": "Tinjauan kolaboratif",
  "p.reviews.consensus": "Konsensus",
  "p.reviews.consensus_for_this_subject": "Konsensus untuk subjek ini",
  "p.reviews.decline": "Tolak",
  "p.reviews.declined": "ditolak",
  "p.reviews.delegate": "Delegasikan",
  "p.reviews.delegate_a_specific_item_to":
    "Delegasikan item tertentu kepada pakar yang tepat. Hanya peninjau yang telah menerima yang dapat mengajukan putusan, sehingga undangan yang tidak dijawab tidak pernah menjadi bukti.",
  "p.reviews.due": "tenggat",
  "p.reviews.e_g_clinical_safety": "mis. keselamatan klinis",
  "p.reviews.expertise": "keahlian",
  "p.reviews.expertise_2": "Keahlian",
  "p.reviews.expired": "kedaluwarsa",
  "p.reviews.external": "eksternal",
  "p.reviews.hold": "Tahan",
  "p.reviews.idea": "ide",
  "p.reviews.internal": "internal",
  "p.reviews.invited": "diundang",
  "p.reviews.no_review_invitations_yet": "Belum ada undangan tinjauan.",
  "p.reviews.no_score": "tanpa skor",
  "p.reviews.person_uuid": "person:<uuid>",
  "p.reviews.plan": "rencana",
  "p.reviews.proposal": "usulan",
  "p.reviews.reject": "Tolak",
  "p.reviews.reviewer": "peninjau",
  "p.reviews.reviewer_2": "Peninjau",
  "p.reviews.reviews_ppm": "Tinjauan — PPM",
  "p.reviews.scope": "cakupan",
  "p.reviews.scope_2": "Cakupan",
  "p.reviews.status": "status",
  "p.reviews.status_2": "Status",
  "p.reviews.subject": "subjek",
  "p.reviews.subject_2": "Subjek",
  "p.reviews.subject_pid": "pid subjek",
  "p.reviews.submitted": "dikirim",
  "p.reviews.submitted_outstanding_declined_exter":
    "{submitted} dikirim · {outstanding} belum selesai · {declined} ditolak · {external_submitted} eksternal",
  "p.reviews.uuid": "uuid",
  "p.reviews.verdict": "Putusan",
  "p.reviews.withdraw": "Tarik",
  "p.risk.category": "Kategori",
  "p.risk.concentration": "Konsentrasi",
  "p.risk.escalated": "Dieskalasi",
  "p.risk.exposure": "Eksposur",
  "p.risk.exposure_25_of_estate":
    "— eksposur {exposure} (≥ 25% dari seluruh aset)",
  "p.risk.heatmap": "Peta panas",
  "p.risk.item": "Butir",
  "p.risk.materialized": "Terjadi",
  "p.risk.max_estate_max_single_risk":
    "— maksimum keseluruhan {v}, maksimum satu risiko {v2}",
  "p.risk.no_open_risks": "Tidak ada risiko terbuka.",
  "p.risk.open_exposure": "Eksposur terbuka",
  "p.risk.open_risks_estate_open_exposure":
    "risiko terbuka · eksposur terbuka keseluruhan",
  "p.risk.overdue_reviews": "Tinjauan terlambat",
  "p.risk.portfolio": "Portofolio",
  "p.risk.posture_by_portfolio": "Postur per portofolio",
  "p.risk.ppm": "{t} — PPM",
  "p.risk.probability_impact": "probabilitas ↓ / dampak →",
  "p.risk.review_due": "(tinjauan jatuh tempo {review_date})",
  "p.risk.risk": "Risiko",
  "p.risk.top_risks": "Risiko teratas",
  "p.risk.unassigned": "(belum ditugaskan)",
  "p.scenarios.alignment": "Keselarasan",
  "p.scenarios.cap": "Batas",
  "p.scenarios.compare": "Bandingkan",
  "p.scenarios.currency": "Mata uang",
  "p.scenarios.delta_b_a": "Selisih (b − a)",
  "p.scenarios.evaluation": "Evaluasi",
  "p.scenarios.feasible": "Layak",
  "p.scenarios.no": "tidak",
  "p.scenarios.open_exposure": "Eksposur terbuka",
  "p.scenarios.pick": "pilih…",
  "p.scenarios.planned": "Direncanakan {currency}",
  "p.scenarios.scenario": "Skenario",
  "p.scenarios.scenarios_ppm": "Skenario — PPM",
  "p.scenarios.status": "Status",
  "p.scenarios.work_item_pids_space_comma":
    "pid item pekerjaan (dipisahkan spasi/koma)",
  "p.scenarios.yes": "ya",
  "p.security.exposure": "Eksposur",
  "p.security.item": "Butir",
  "p.security.late_stage_items_with_no":
    "Item tahap akhir tanpa risiko keamanan tercatat",
  "p.security.no_security_risks_recorded":
    "Tidak ada risiko keamanan tercatat.",
  "p.security.none_every_late_stage_item":
    "Tidak ada — setiap item tahap akhir memiliki risiko keamanan yang tercatat.",
  "p.security.open_exposure": "{note} · eksposur terbuka {open_exposure}",
  "p.security.owner": "Pemilik",
  "p.security.ppm": "{t} — PPM",
  "p.security.risk": "Risiko",
  "p.security.security_risk_register": "Registri risiko keamanan",
  "p.security.status": "Status",
  "p.signin.check_your_email_for_a": "Periksa email Anda untuk tautan masuk.",
  "p.signin.could_not_send_the_sign":
    "Tautan masuk tidak dapat dikirim. Silakan coba lagi.",
  "p.signin.email": "Surel",
  "p.signin.send_magic_link": "Kirim tautan ajaib",
  "p.signin.sign_in": "Masuk",
  "p.technology.as_of": "per {as_of} · {convention}",
  "p.technology.as_of_edges": "per {as_of} · {edges} sisi",
  "p.technology.calendar_days": "{median_lead_days} hari kalender",
  "p.technology.completed_before_timing_existed_unti":
    "· {undated_completions} selesai sebelum pencatatan waktu ada (tanpa waktu)",
  "p.technology.cross_portfolio_edges": "Sisi lintas portofolio",
  "p.technology.delivery_flow": "Alur pengiriman",
  "p.technology.dependency_risk": "Risiko ketergantungan",
  "p.technology.dependents": "Yang bergantung",
  "p.technology.edges_importing_risk_from_a":
    "Sisi yang membawa risiko dari pendahulu berstatus merah",
  "p.technology.exposure": "Eksposur",
  "p.technology.item": "Butir",
  "p.technology.kind": "Jenis",
  "p.technology.median_lead": "Median waktu tunggu:",
  "p.technology.milestones_completed": "Pencapaian selesai",
  "p.technology.month": "Bulan",
  "p.technology.most_depended_on_single_point":
    "Paling banyak diandalkan (kandidat titik kegagalan tunggal)",
  "p.technology.no_dependencies_recorded_yet":
    "Belum ada ketergantungan tercatat.",
  "p.technology.no_tech_debt_risks_recorded":
    "Tidak ada risiko tech_debt tercatat.",
  "p.technology.no_tech_tags_recorded_yet": "Belum ada tag `tech:*` tercatat.",
  "p.technology.no_timed_completions": "tidak ada penyelesaian berwaktu",
  "p.technology.no_timed_completions_in_the":
    "Tidak ada penyelesaian berwaktu dalam rentang ini.",
  "p.technology.none_all_edges_stay_within":
    "Tidak ada — semua sisi berada dalam satu portofolio.",
  "p.technology.none_no_red_predecessors":
    "Tidak ada — tidak ada pendahulu berstatus merah.",
  "p.technology.open_exposure": "{note} · eksposur terbuka {open_exposure}",
  "p.technology.over_timed_completions":
    "dari {timed_completions} penyelesaian berwaktu",
  "p.technology.owner": "Pemilik",
  "p.technology.ppm": "{t} — PPM",
  "p.technology.predecessor": "Pendahulu",
  "p.technology.predecessor_red": "Pendahulu (merah)",
  "p.technology.ring_votes": "({ring_votes} suara cincin)",
  "p.technology.risk": "Risiko",
  "p.technology.status": "Status (",
  "p.technology.status_2": "Status",
  "p.technology.successor": "Penerus",
  "p.technology.successor_at_risk": "Penerus berisiko",
  "p.technology.technical_debt_register": "Registri utang teknis",
  "p.technology.technology_radar": "Radar teknologi",
  "p.verify.request_a_new_link": "Minta tautan baru",
  "p.verify.sign_in_link": "Tautan masuk",
  "p.milestones.milestone_by_due_date_one":
    "{length} pencapaian, menurut tanggal jatuh tempo",
  "p.milestones.milestone_by_due_date_other":
    "{length} pencapaian, menurut tanggal jatuh tempo",
  "p.okrs.objective_least_progressed_first_one":
    "{length} sasaran, yang paling lambat maju lebih dahulu",
  "p.okrs.objective_least_progressed_first_other":
    "{length} sasaran, yang paling lambat maju lebih dahulu",
  "p.plans.pid.board.blocked_calendar_day_one":
    "terhambat {blocked_days} hari kalender",
  "p.plans.pid.board.blocked_calendar_day_other":
    "terhambat {blocked_days} hari kalender",
  "p.plans.pid.calendar.holiday_one": "{length} hari libur: {join}",
  "p.plans.pid.calendar.holiday_other": "{length} hari libur: {join}",
  "p.plans.pid.costs.for_across_one":
    "{money} untuk {hoursText} dalam {entries} entri.",
  "p.plans.pid.costs.for_across_other":
    "{money} untuk {hoursText} dalam {entries} entri.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_one":
    "{plans_without_figures} rencana di bawah belum memiliki angka (tanpa baseline atau tidak ada yang dapat diukur) dan tidak ditebak.",
  "p.plans.pid.earned_value.plan_below_have_no_figures_other":
    "{plans_without_figures} rencana di bawah belum memiliki angka (tanpa baseline atau tidak ada yang dapat diukur) dan tidak ditebak.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_one":
    "selesai tanpa pernah bergerak mundur ({rework_count} gerakan mundur). Bacalah bersama throughput: throughput yang naik dengan yield yang turun berarti pekerjaan dikembalikan ke dirinya sendiri.",
  "p.plans.pid.flow.finished_without_ever_moving_backwar_other":
    "selesai tanpa pernah bergerak mundur ({rework_count} gerakan mundur). Bacalah bersama throughput: throughput yang naik dengan yield yang turun berarti pekerjaan dikembalikan ke dirinya sendiri.",
  "p.plans.pid.flow.of_items_finish_within_this_one":
    "{percent} item selesai dalam waktu ini, dari {sample} item yang selesai",
  "p.plans.pid.flow.of_items_finish_within_this_other":
    "{percent} item selesai dalam waktu ini, dari {sample} item yang selesai",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_one":
    "{length} risiko memengaruhi jadwal dan biaya sekaligus: masing-masing terjadi sekali dan memengaruhi keduanya.",
  "p.plans.pid.joint_risk.risk_affect_both_the_schedule_other":
    "{length} risiko memengaruhi jadwal dan biaya sekaligus: masing-masing terjadi sekali dan memengaruhi keduanya.",
  "p.plans.pid.leveling.apply_move_one": "Terapkan {length} pemindahan",
  "p.plans.pid.leveling.apply_move_other": "Terapkan {length} pemindahan",
  "p.plans.pid.leveling.business_day_one": "{delay_days} hari kerja",
  "p.plans.pid.leveling.business_day_other": "{delay_days} hari kerja",
  "p.plans.pid.leveling.other_task_would_be_pushed_one":
    "{length} tugas lain akan terdorong sebagai akibatnya",
  "p.plans.pid.leveling.other_task_would_be_pushed_other":
    "{length} tugas lain akan terdorong sebagai akibatnya",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_one":
    "{hidden} daftar periksa tentang orang tidak ditampilkan kepada Anda.",
  "p.plans.pid.onboarding.checklist_about_people_not_shown_other":
    "{hidden} daftar periksa tentang orang tidak ditampilkan kepada Anda.",
  "p.plans.pid.schedule_health.preset_thresholds_task_one":
    "· preset {preset} (ambang batas: {source}) · {tasks} tugas",
  "p.plans.pid.schedule_health.preset_thresholds_task_other":
    "· preset {preset} (ambang batas: {source}) · {tasks} tugas",
  "p.plans.pid.schedule_health.task_one": "{length} tugas",
  "p.plans.pid.schedule_health.task_other": "{length} tugas",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_one":
    "{title}: peluang {probability_percent}% menambah {impact_days} hari kerja",
  "p.plans.pid.schedule_risk.chance_of_adding_business_day_other":
    "{title}: peluang {probability_percent}% menambah {impact_days} hari kerja",
  "p.plans.pid.wbs.move_up": "Pindahkan {title} ke atas",
  "p.plans.pid.wbs.move_down": "Pindahkan {title} ke bawah",
  "p.plans.pid.wbs.moved": "{title} dipindahkan.",
  "p.plans.pid.wbs.truncated":
    "Menampilkan {shown} dari {total} tugas: rencana ini terlalu besar untuk didaftar seluruhnya, sehingga kerangka di bawah tidak lengkap.",
  "p.plans.pid.critical_path.too_large":
    "Rencana ini terlalu besar untuk dijadwalkan seluruhnya dalam satu jawaban.",
  "p.plans.pid.critical_path.show_critical_only":
    "Tampilkan hanya bagian kritis",
  "p.plans.pid.critical_path.critical_only_notice":
    "Hanya menampilkan tugas kritis dan nyaris kritis; sisa rencana tidak ditampilkan.",
  "p.plans.pid.earned_value.names_truncated":
    "Rencana ini sangat besar: angka mencakup setiap tugas, tetapi beberapa nama tugas di bawah mungkin hilang.",
  "p.plans.pid.wbs.could_not_move": "Tugas tidak dapat dipindahkan: {reason}",
  "p.plans.pid.wbs.sub_task_one": "({children} subtugas)",
  "p.plans.pid.wbs.sub_task_other": "({children} subtugas)",
  "p.plans.pid.wbs.links": "Tautan",
  "p.plans.pid.wbs.link_one": "· {links} tautan",
  "p.plans.pid.wbs.link_other": "· {links} tautan",
  "p.common.on": "aktif",
  "p.common.off": "nonaktif",
  "p.common.pre_gate": "pra-gerbang",
  "p.common.role_word": "peran",
  "p.common.person_word": "orang",
  "c.NetworkDiagram.task_fallback": "tugas",
  "c.NetworkDiagram.task_summary":
    "{title}, {early_start} hingga {early_finish}, kelonggaran {total_float} hari kerja",
  "c.NetworkDiagram.task_summary_critical":
    "{title}, {early_start} hingga {early_finish}, kelonggaran {total_float} hari kerja, berada di jalur kritis",
  "c.NetworkDiagram.lag_one": ", jeda {lag_days} hari kerja",
  "c.NetworkDiagram.lag_other": ", jeda {lag_days} hari kerja",
  "c.KpiCard.direction_higher": "lebih tinggi",
  "c.KpiCard.direction_lower": "lebih rendah",
  "c.CalendarMonth.legend":
    "– bukan hari kerja   ✕ hari libur   A absensi tercatat",
  "c.CalendarMonth.legend_editable":
    " · pilih hari kerja untuk menandai atau menghapus hari libur",
  "c.CostRiskView.run_summary":
    "seed {seed} · {iterations} iterasi · {distribution}",
  "c.CostRiskView.dist_pert": "PERT",
  "c.CostRiskView.dist_triangular": "Segitiga",
  "c.CostRiskView.risks_without_cost_one":
    "{count} risiko di registri tidak memiliki kolom biaya dan tidak termasuk di sini.",
  "c.CostRiskView.risks_without_cost_other":
    "{count} risiko di registri tidak memiliki kolom biaya dan tidak termasuk di sini.",
  "c.CumulativeFlow.hide_the_numbers": "Sembunyikan angka",
  "c.CumulativeFlow.show_the_numbers": "Tampilkan angka",
  "c.CostRiskView.scope_all_one": "Semua {plans} rencana yang dapat Anda baca",
  "c.CostRiskView.scope_all_other":
    "Semua {plans} rencana yang dapat Anda baca",
  "c.CostRiskView.scope_this_plan": "Rencana ini",
  "c.MilestoneCard.reason": "Alasan",
  "c.MilestoneCard.reason_required_open":
    " (wajib: tugas yang ditunggu masih terbuka)",
  "c.MilestoneCard.reason_optional": " (opsional)",
  "c.VarianceTable.matched_of_baselined":
    "{matched} dari {baseline_tasks} tugas ber-baseline masih ada dalam rencana:",
  "c.VarianceTable.slipped_pulled_on":
    "{slipped} tertunda, {pulled_in} dimajukan, {on_baseline} sesuai baseline{unmeasurable}.",
  "c.VarianceTable.cannot_be_measured": ", {unmeasurable} tidak dapat diukur",
  "c.VarianceTable.added_removed":
    "{added} ditambahkan dan {removed} dihapus sejak itu.",
  "c.VarianceTable.finish_later_one":
    "Penyelesaian rencana {days} hari kerja lebih lambat dari baseline{note}.",
  "c.VarianceTable.finish_later_other":
    "Penyelesaian rencana {days} hari kerja lebih lambat dari baseline{note}.",
  "c.VarianceTable.finish_earlier_one":
    "Penyelesaian rencana {days} hari kerja lebih cepat dari baseline{note}.",
  "c.VarianceTable.finish_earlier_other":
    "Penyelesaian rencana {days} hari kerja lebih cepat dari baseline{note}.",
  "c.VarianceTable.check_removed":
    " (periksa tugas yang dihapus: menghapus pekerjaan bukanlah perbaikan)",
  "p.plans.pid.critical_path.finishes": "Selesai {date}.",
  "p.plans.pid.critical_path.nothing_scheduled": "Belum ada yang dijadwalkan.",
  "p.plans.pid.critical_path.calendar_label": "Kalender: {calendar}.",
  "p.plans.pid.critical_path.calendar_plan": "milik rencana ini",
  "p.plans.pid.critical_path.calendar_default":
    "bawaan (setiap hari adalah hari kerja)",
  "p.plans.pid.critical_path.more_not_shown": " (selebihnya tidak ditampilkan)",
  "p.plans.pid.critical_path.pushed_later":
    "Pendahulunya mendorongnya {days} hari kerja lebih lambat dari rencana.",
  "p.plans.pid.earned_value.manual_percent_banner":
    "Sebagian angka ini bertumpu pada persentase yang dimasukkan orang, bukan pada pekerjaan yang selesai: {tasks}.",
  "p.plans.pid.earned_value.a_task": "sebuah tugas",
  "p.plans.pid.earned_value.overrun": " (melebihi)",
  "p.plans.pid.earned_value.target_budget": "anggaran",
  "p.plans.pid.earned_value.target_forecast": "perkiraan",
  "p.plans.pid.earned_value.lights_use":
    "Lampu memakai {which}: {values} (basis poin).",
  "p.plans.pid.earned_value.thresholds_default": "ambang batas bawaan",
  "p.plans.pid.earned_value.thresholds_deployment":
    "ambang batas penerapan ini",
  "p.plans.pid.earned_value.rollup_money":
    "Anggaran {bac}; direncanakan {pv}; diperoleh {ev}; dibelanjakan {ac}{unknown}.",
  "p.plans.pid.earned_value.cost_unknown_for":
    " (biaya tidak diketahui untuk {plans} rencana)",
  "p.plans.pid.earned_value.cpi_value": "CPI {value}",
  "p.plans.pid.earned_value.cpi_not_stated":
    "tidak dinyatakan: biaya tidak diketahui untuk beberapa rencana",
  "p.plans.pid.earned_value.weight_points": ", {weight} poin",
  "p.plans.pid.critical_path.moved_line":
    "{title}: {moved}{finish}{start} hari kerja",
  "p.plans.pid.critical_path.moved_both": "mulai dan selesai",
  "p.plans.pid.critical_path.moved_start": "mulai",
  "p.plans.pid.critical_path.moved_finish": "selesai",
  "p.plans.pid.critical_path.moved_finish_by": " selesai {days}",
  "p.plans.pid.critical_path.moved_start_by": " mulai {days}",
  "p.plans.pid.flow.overdue_one":
    "{count} item sudah melewati harapan. Item yang menua adalah satu-satunya hal di halaman ini yang hasilnya masih dapat Anda ubah.",
  "p.plans.pid.flow.overdue_other":
    "{count} item sudah melewati harapan. Item yang menua adalah satu-satunya hal di halaman ini yang hasilnya masih dapat Anda ubah.",
  "p.plans.pid.flow.classified_with": "Diklasifikasikan dengan {map}:",
  "p.plans.pid.flow.map_override": "penimpaan penerapan",
  "p.plans.pid.flow.map_default": "peta bawaan",
  "p.plans.pid.governance.proposed_not_counted":
    "diusulkan{requested} — tidak dihitung",
  "p.plans.pid.governance.requested": ", diminta",
  "p.plans.pid.joint_risk.run_summary":
    "{currency} · seed {seed} · {iterations} iterasi · {distribution} · kalender {calendar}",
  "p.plans.pid.joint_risk.dist_pert": "PERT",
  "p.plans.pid.joint_risk.dist_triangular": "Segitiga",
  "p.plans.pid.leveling.applied_one":
    "Menerapkan {ok} dari {requested} pemindahan{refused}.",
  "p.plans.pid.leveling.applied_other":
    "Menerapkan {ok} dari {requested} pemindahan{refused}.",
  "p.plans.pid.leveling.applied_refused": "; {refused} ditolak",
  "p.plans.pid.leveling.proposal_summary":
    "Kalender: {calendar} · beban {load}% per tugas · tugas kritis {critical} berpindah",
  "p.plans.pid.leveling.may": "boleh",
  "p.plans.pid.leveling.may_not": "tidak boleh",
  "p.plans.pid.onboarding.due": ", jatuh tempo {date}",
  "p.plans.pid.schedule_risk.risks_without_schedule_one":
    "{count} risiko di registri tidak memiliki kolom jadwal dan tidak termasuk di sini.",
  "p.plans.pid.schedule_risk.risks_without_schedule_other":
    "{count} risiko di registri tidak memiliki kolom jadwal dan tidak termasuk di sini.",
  "p.resources.from_date": "dari {date}",
  "p.resources.to_date": "hingga {date}",
  "p.kpis.none": "Belum ada KPI.",
  "p.kpis.none_with_status": "Belum ada KPI dengan status itu.",
  "p.milestones.none":
    "Belum ada pencapaian. Tambahkan dari halaman tata kelola rencana.",
  "p.milestones.none_with_status":
    "Belum ada pencapaian dengan status itu. Tambahkan dari halaman tata kelola rencana.",
  "p.okrs.none": "Belum ada sasaran.",
  "p.okrs.none_for_period": "Belum ada sasaran untuk {period}.",
  "p.me.calendar.own_set": "Kalender pribadi Anda sudah diatur.",
  "p.me.calendar.plan_applies":
    "Tidak ada kalender pribadi: kalender rencana berlaku untuk Anda.",
  "p.me.calendar.leave_counts":
    "Cuti yang tercatat untuk Anda sudah dihitung sebagai hari libur.",
  "p.documents.pid.page_title": "{title} — PPM",
  "p.documents.pid.document_fallback": "Dokumen",
  "p.plans.pid.page_title": "{name} — Main X",
  "p.plans.pid.plan_fallback": "Rencana",
  "p.plans.pid.custom_scheme": "Kustom({name})",
  "p.plans.pid.calendar.own_calendar":
    "Rencana ini memiliki kalendernya sendiri.",
  "p.plans.pid.calendar.no_calendar":
    "Kalender belum diatur: setiap hari adalah hari kerja.",
  "p.plans.pid.calendar.each_person_can_set":
    "Setiap orang juga dapat mengatur kalender pribadinya (",
  "p.reviews.consensus_line":
    "skor rata-rata: {mean} · mayoritas: {majority} · {state}",
  "p.reviews.no_majority": "tidak ada mayoritas",
  "p.reviews.everyone_answered": "semua sudah menjawab",
  "p.reviews.still_waiting": "masih menunggu",
  "l.announcement_view.just_now": "baru saja",
  "l.announcement_view.min_ago": "{minutes} mnt lalu",
  "l.announcement_view.h_ago": "{hours} j lalu",
  "l.announcement_view.calendar_days_ago": "{days} hari kalender lalu",
  "l.announcement_view.scheduled": "dijadwalkan",
  "l.announcement_view.expired": "Kedaluwarsa",
  "l.api_ppm.task": "tugas {uid}",
  "l.api_ppm.the_project_calendar_comes_across":
    " Kalender proyek ikut terbawa.",
  "l.api_tba.calendar_days": "{toFixed} hari kalender",
  "l.api_tba.work_in_progress_is_growing": "Pekerjaan dalam proses bertambah",
  "l.api_tba.steady_state": "Keadaan stabil",
  "l.api_tba.queue_draining": "Antrean berkurang",
  "l.api_tba.not_enough_data": "Data tidak cukup",
  "l.capacity_view.committed_against_a_capacity_as":
    "{committed}% dikomitmenkan terhadap kapasitas serendah {capacity}%",
  "l.capacity_view.committed": "{committed}% dikomitmenkan",
  "l.capacity_view.to": " hingga {to}",
  "l.capacity_view.over_capacity_from_asked_available":
    "melebihi kapasitas dari {from}{v} ({demand}% diminta, {capacity}% tersedia)",
  "l.capacity_view.capacity_undeclared":
    "tidak ada kapasitas yang dinyatakan pada beberapa hari kerja: tidak melebihi dan tidak di dalam kapasitas",
  "l.capacity_view.over_capacity_from_asked_undeclared":
    "melebihi kapasitas sejak {from}{v} ({demand} diminta, lebih dari kapasitas mana pun)",
  "l.capacity_view.asked_undeclared":
    "{from}{v}: {demand} diminta, lebih dari kapasitas mana pun",
  "l.capacity_view.over_capacity": "melebihi kapasitas",
  "l.capacity_view.committing_it_would_put_them":
    "; mengomitmenkannya akan membuat mereka melebihi kapasitas",
  "l.capacity_view.proposed_not_counted":
    "{percent}% diusulkan, tidak dihitung{v}",
  "l.capacity_view.asked_available":
    "{from}{v}: {demand}% diminta, {capacity}% tersedia",
  "l.ceo_view.no_portfolios_to_report_on":
    "Belum ada portofolio untuk dilaporkan.",
  "l.ceo_view.portfolios_exist_but_hold_no":
    "Portofolio ada tetapi belum memuat rencana.",
  "l.ceo_view.of_plans_are_red_amber":
    "{red} dari {n} rencana berstatus merah, {amber} kuning, {green} hijau.",
  "c.CalendarMonth.not_a_business_day": "bukan hari kerja",
  "c.CalendarMonth.business_day": "hari kerja",
  "c.KpiCard.that_did_not_work": "Itu tidak berhasil",
  "c.KpiCard.could_not_load_the_history": "Riwayat tidak dapat dimuat",
  "c.KpiCard.delete_the_kpi_its_readings":
    "Hapus KPI “{name}”? Pembacaannya tetap disimpan.",
  "c.LineRange.could_not_save": "Tidak dapat menyimpan",
  "c.MilestoneCard.that_did_not_work": "Itu tidak berhasil",
  "c.MilestoneCard.could_not_load_the_plan": "Tugas rencana tidak dapat dimuat",
  "c.MilestoneCard.could_not_load_the_history": "Riwayat tidak dapat dimuat",
  "c.ObjectiveCard.that_did_not_work": "Itu tidak berhasil",
  "c.ObjectiveCard.delete_the_objective_and_its":
    "Hapus sasaran “{title}” beserta hasil kuncinya?",
  "c.ObjectiveCard.delete_the_key_result": "Hapus hasil kunci “{title}”?",
  "c.RaciMatrix.accountable_moved_from_to_a":
    "Penanggung jawab akhir dipindah dari {shortRef} ke {shortRef2}: satu tugas hanya memiliki satu.",
  "c.RiskInputs.could_not_load_the_plan": "Tugas rencana tidak dapat dimuat",
  "c.RiskInputs.could_not_save": "Tidak dapat menyimpan",
  "c.RiskInputs.chance": "peluang {probability_percent}%",
  "c.RiskInputs.adds_business_days_to_a":
    "menambah {schedule_impact_days} hari kerja pada sebuah tugas",
  "c.RiskInputs.adds": "menambah {toFixed} {cost_currency}",
  "c.RiskInputs.not_used_in_the_simulations": "Tidak dipakai dalam simulasi",
  "l.cpm_view.cannot_meet_its_constraint_or":
    "Tidak dapat memenuhi batasan atau tenggatnya: kurang {plural}",
  "l.cpm_view.on_the_critical_path_any":
    "Di jalur kritis: setiap keterlambatan menggeser penyelesaian",
  "l.cpm_view.of_float_near_critical": "kelonggaran {d} (nyaris kritis)",
  "l.cpm_view.of_float": "kelonggaran {d}",
  "l.cpm_view.baselined_to": ". Di-baseline {start} hingga {finish}",
  "l.cpm_view.to": "{v}: {early_start} hingga {early_finish}. {floatLabel}{v2}",
  "l.cpm_view.has_no_duration_yet": "belum memiliki durasi",
  "l.cpm_view.has_no_start_date_and":
    "belum memiliki tanggal mulai dan tidak ada yang sebelumnya dijadwalkan",
  "l.cpm_view.has_a_date_outside_the":
    "memiliki tanggal di luar rentang yang didukung",
  "l.cpm_view.slipped": "Tertunda",
  "l.cpm_view.finishes_later_than_baselined":
    "selesai {d} lebih lambat dari baseline",
  "l.cpm_view.pulled_in": "Dimajukan",
  "l.cpm_view.finishes_earlier_than_baselined":
    "selesai {d} lebih cepat dari baseline",
  "l.cpm_view.on_baseline": "Sesuai baseline",
  "l.cpm_view.finishes_when_it_was_baselined":
    "selesai pada waktu yang ditetapkan dalam baseline",
  "l.cpm_view.added": "Ditambahkan",
  "l.cpm_view.not_in_the_baseline_so":
    "tidak ada dalam baseline, jadi tidak ada penundaan",
  "l.cpm_view.removed": "Dihapus",
  "l.cpm_view.was_in_the_baseline_and":
    "ada dalam baseline dan sudah hilang; tanggal baseline-nya tetap disimpan",
  "l.cpm_view.cannot_say": "Tidak dapat ditentukan",
  "l.cpm_view.it_had_no_schedule_when":
    "belum memiliki jadwal saat di-baseline",
  "l.cpm_view.it_has_no_schedule_now": "sekarang tidak memiliki jadwal",
  "l.document_view.version": "Versi {version} — {v}",
  "l.document_view.since": "{base} sejak {effective_from}",
  "l.document_view.by_version": " oleh versi {version}",
  "l.document_view.on": "{base} pada {superseded_on}{v}",
  "l.document_view.nothing_in_force_yet": "Belum ada yang berlaku",
  "l.document_view.version_in_force": "Versi {in_force_today} berlaku",
  "l.ev_view.nothing_was_planned_to_be":
    "belum ada yang direncanakan untuk dikerjakan",
  "l.ev_view.no_cost_has_been_recorded": "belum ada biaya yang tercatat",
  "l.ev_view.the_baseline_plans_no_value":
    "baseline tidak merencanakan nilai apa pun",
  "l.ev_view.not_available": "tidak tersedia",
  "l.ev_view.schedule_no_ratio": "Jadwal: tanpa rasio, {absentText}.",
  "l.ev_view.ahead_of_schedule": "Lebih cepat dari jadwal",
  "l.ev_view.behind_schedule": "Terlambat dari jadwal",
  "l.ev_view.on_schedule": "Sesuai jadwal",
  "l.ev_view.spi": "{word}: SPI {ratioText}, {gap}.",
  "l.ev_view.cost_no_ratio": "Biaya: tanpa rasio, {absentText}.",
  "l.ev_view.under_budget": "Di bawah anggaran",
  "l.ev_view.over_budget": "Melebihi anggaran",
  "l.ev_view.on_budget": "Sesuai anggaran",
  "l.ev_view.earned_schedule_no_ratio":
    "Jadwal yang diperoleh: tanpa rasio, {absentText}.",
  "l.ev_view.by_earned_schedule_the_work":
    "Menurut jadwal yang diperoleh, pekerjaan {word}: telah memperoleh dalam {days} apa yang diharapkan rencana setelah {days2} (SPI(t) {ratioText}).",
  "l.ev_view.finished_work_counts_for_its":
    "pekerjaan selesai dihitung menurut poin ceritanya",
  "l.ev_view.only_finished_work_counts":
    "hanya pekerjaan selesai yang dihitung",
  "l.ev_view.by_story_points": "menurut poin cerita",
  "l.ev_view.counted_by_a_person_entered":
    "{length} dihitung dari persentase yang dimasukkan orang (manual)",
  "l.ev_view.with_no_points_carry_no":
    "{length} tanpa poin tidak memiliki bobot",
  "l.ev_view.no_actual_cost_is_recorded":
    "belum ada biaya aktual yang tercatat",
  "l.ev_view.the_ratio_it_divides_by": "rasio pembaginya belum terdefinisi",
  "l.ev_view.no_estimate_to_complete_has":
    "belum ada perkiraan hingga selesai yang dimasukkan",
  "l.ev_view.the_budget_is_already_spent": "anggaran sudah habis dibelanjakan",
  "l.ev_view.no_cost_remains": "tidak ada biaya tersisa",
  "l.ev_view.against_the_money_is_already":
    "Terhadap {target}: dana sudah habis dibelanjakan, sehingga tidak ada efisiensi yang dapat menyelesaikan sisa pekerjaan di dalamnya.",
  "l.ev_view.against": "Terhadap {target}: {amountAbsentText}.",
  "l.ev_view.against_the_remaining_work_must":
    "Terhadap {target}: sisa pekerjaan harus dikerjakan {ratioText} kali seefisien pekerjaan sejauh ini (TCPI {ratioText2}), sehingga membutuhkan peningkatan efisiensi.",
  "l.ev_view.against_the_remaining_work_can":
    "Terhadap {target}: sisa pekerjaan dapat dikerjakan dengan {ratioText} dari efisiensi biaya pekerjaan sejauh ini (TCPI {ratioText2}).",
  "l.ev_view.earned": "{date}: diperoleh {ev_minor}",
  "l.ev_view.spent": "{date}: dibelanjakan {ac_minor}",
  "l.import_view.would_create": "Akan membuat",
  "l.import_view.created": "Dibuat",
  "l.import_view.and": "{verb} {plural} dan {upd} {tasks_updated}{tail}.",
  "l.import_view.preview_the_import": "Pratinjau impor",
  "l.import_view.import": "Impor {plural}",
  "l.kpi_view.target_or_more_red_at":
    "Target {target_value} atau lebih; merah pada {red_value} atau kurang{u}",
  "l.kpi_view.target_or_less_red_at":
    "Target {target_value} atau kurang; merah pada {red_value} atau lebih{u}",
  "l.kpi_view.no_reading_within_twice_its":
    "Tidak ada pembacaan dalam dua kali kadens {cadence}",
  "l.kpi_view.no_reading_yet": "belum ada pembacaan",
  "l.milestone_view.there_is_no_forecast": "Tidak ada perkiraan.",
  "l.milestone_view.some_open_tasks_cannot_be":
    " (beberapa tugas terbuka tidak dapat ditempatkan, jadi ini mungkin terlalu awal)",
  "l.milestone_view.forecast_to_finish_after_it":
    "Diperkirakan selesai {forecast}, {businessDays} setelah jatuh tempo ({due}){partial}.",
  "l.milestone_view.forecast_to_finish_on_the":
    "Diperkirakan selesai pada tanggal jatuh tempo, {due}{partial}.",
  "l.milestone_view.forecast_to_finish_before_it":
    "Diperkirakan selesai {forecast}, {businessDays} sebelum jatuh tempo ({due}){partial}.",
  "l.milestone_view.of_them_later": ", {later_moves} di antaranya mundur",
  "l.milestone_view.back_on_its_original_date": "kembali ke tanggal semula",
  "l.milestone_view.later_than_the_original":
    "{calendarDays} lebih lambat dari tanggal semula {original_due}",
  "l.milestone_view.earlier_than_the_original":
    "{calendarDays} lebih cepat dari tanggal semula {original_due}",
  "l.milestone_view.originally_due_now":
    "Semula jatuh tempo {original_due}; {times}{later}; kini {net}.",
  "l.milestone_view.on": " pada {achieved_on}",
  "l.milestone_view.achieved_every_task_it_waits":
    "Tercapai{on}: setiap tugas yang ditunggu sudah selesai.",
  "l.milestone_view.reason": " Alasan: {done_reason}",
  "l.milestone_view.marked_done": "Ditandai selesai{on}.{why}",
  "l.milestone_view.due": "{name}: {toLowerCase}, jatuh tempo {due}{state}",
  "l.my_work_view.this_is_already_late_for":
    "Ini sudah terlambat bagi rencana ({days} tertinggal).",
  "l.my_work_view.on_the_critical_path": "Di jalur kritis",
  "l.my_work_view.of_float": "kelonggaran {days}",
  "l.okr_view.not_measured": "Tidak diukur",
  "l.okr_view.not_measurable": "Tidak dapat diukur ({reason}).",
  "l.okr_view.of": "{current} dari {target}{u}",
  "l.okr_view.confidence_not_part_of_the":
    "Keyakinan {confidence}% (bukan bagian dari skor)",
  "l.onboarding_view.give_the_date_it_takes": "Berikan tanggal berlakunya.",
  "l.onboarding_view.give_the_person_as_person":
    "Tulis orangnya sebagai person:… (atau worker:…).",
  "l.onboarding_view.say_what_access_in_words":
    "Sebutkan akses apa, dengan kata-kata.",
  "l.onboarding_view.choose_granted_or_revoked":
    "Pilih diberikan atau dicabut.",
  "l.onboarding_view.give_who_is_handing_over":
    "Tulis siapa yang menyerahkan, sebagai person:…",
  "l.onboarding_view.give_who_takes_over_as":
    "Tulis siapa yang mengambil alih sebagai person:… (atau kosongkan untuk melepaskan pekerjaan).",
  "l.onboarding_view.hand_over_to_someone_else": "Serahkan kepada orang lain.",
  "l.onboarding_view.no_items": "Tidak ada item",
  "l.onboarding_view.of_done": "{done} dari {total} selesai",
  "l.onboarding_view.skipped": "{skipped} dilewati",
  "l.onboarding_view.overdue": "{overdue} terlambat",
  "l.onboarding_view.skipped_2": "Dilewati",
  "l.onboarding_view.overdue_2": "Terlambat",
  "l.onboarding_view.nothing_is_left_open_this":
    "Tidak ada yang tersisa terbuka. Rencana ini siap ditutup.",
  "l.onboarding_view.not_ready_to_close": "Belum siap ditutup: {join}.",
  "l.onboarding_view.this_person_holds_nothing_live":
    "Orang ini tidak memegang apa pun yang aktif dalam rencana ini.",
  "l.raci_view.nobody_is_accountable": "Tidak ada yang bertanggung jawab akhir",
  "l.raci_view.nobody_is_responsible":
    "Tidak ada yang bertanggung jawab melaksanakan",
  "l.raci_view.more_than_one_person_is":
    "Lebih dari satu orang bertanggung jawab akhir",
  "l.raci_view.has_no_role": "{task}: {who} tidak memiliki peran",
  "l.raci_view.and": " dan ",
  "l.raci_view.is": "{task}: {who} adalah {join}",
  "l.rbs.a_booking_that_needs_several":
    "Pemesanan yang membutuhkan beberapa keterampilan dihitung penuh untuk masing-masing, sehingga keterampilan tumpang tindih dan tidak dijumlahkan. Pasokan dan orang tidak diketahui per keterampilan.",
  "l.rbs.named": "{length} bernama",
  "l.rbs.supply_not_known": "pasokan tidak diketahui",
  "l.rbs.supply_fte": "pasokan {supply_fte} FTE",
  "l.rbs.demand_fte": "{role}: {people}, permintaan {demand_fte} FTE, {supply}",
  "l.scheduling_view.pass": "Lulus",
  "l.scheduling_view.fail": "Gagal",
  "l.scheduling_view.not_computable": "Tidak dapat dihitung",
  "l.scheduling_view.not_enough_data": "Data tidak cukup.",
  "l.scheduling_view.not_enough_data_2": "Data tidak cukup: {reason}",
  "l.scheduling_view.must_pass_a_business_day":
    "Harus lulus: keterlambatan {v} hari kerja harus menggeser penyelesaian",
  "l.scheduling_view.at_least_business_days": "Setidaknya {limit} hari kerja",
  "l.scheduling_view.at_most": "paling banyak",
  "l.scheduling_view.at_least": "setidaknya",
  "l.scheduling_view.no_figure": "Tidak ada angka",
  "l.scheduling_view.finish_moved_of_business_days":
    "Penyelesaian bergeser {count} dari {of} hari kerja",
  "l.scheduling_view.of": "{toFixed} ({count} dari {of})",
  "l.scheduling_view.of_2": "{value}% ({count} dari {of} {of_what})",
  "l.scheduling_view.passed_failed_not_computable":
    "{passed} lulus, {failed} gagal, {not_computable} tidak dapat dihitung",
  "l.scheduling_view.not_resolved": "Tidak terselesaikan.",
  "l.scheduling_view.it_is_on_the_critical":
    " Berada di jalur kritis, sehingga penyelesaian proyek bergeser.",
  "l.scheduling_view.starts_later_finishes_instead_of":
    "{pid} ({who}) mulai {from_start} → {scheduled_start}, {days} lebih lambat; selesai {to_finish} dan bukan {from_finish}.{crit}",
  "l.scheduling_view.to": "{from} hingga {to}",
  "l.scheduling_view.is_over_capacity_on":
    "{who} melebihi kapasitas pada {span}.",
  "l.scheduling_view.is_asked_for_on_and":
    "{who} diminta {demand}% pada {span} dan hanya dapat memberi {capacity}%.",
  "l.scheduling_view.no_project_finish_can_be":
    "Penyelesaian proyek tidak dapat dihitung.",
  "l.scheduling_view.the_project_finish_stays":
    "Penyelesaian proyek tetap {before}.",
  "l.scheduling_view.the_project_finish_moves_from":
    "Penyelesaian proyek bergeser dari {before} ke {after} ({days} lebih lambat).",
  "l.scheduling_view.no_demand_or_supply": "Tidak ada permintaan atau pasokan",
  "l.scheduling_view.demand_fte_supply_not_known":
    "Permintaan {demand_fte} FTE; pasokan tidak diketahui",
  "l.scheduling_view.short_fte": "Kurang {abs} FTE",
  "l.scheduling_view.spare_fte": "Lebih {gap_fte} FTE",
  "l.scheduling_view.balanced": "Seimbang",
  "l.scheduling_view.the_critical_path_finish": "penyelesaian jalur kritis",
  "l.scheduling_view.of_the_simulated_outcomes_finish":
    "{pct}% hasil simulasi selesai pada atau sebelum {date}.",
  "l.scheduling_view.no_variation": "Tanpa variasi",
  "l.scheduling_view.no_estimate_using_the_fallback":
    "Tanpa perkiraan: memakai rentang cadangan",
  "l.scheduling_view.milestone_no_spread": "Pencapaian: tanpa sebaran",
  "l.scheduling_view.estimated_widened_to_include_the":
    "Diperkirakan (dilebarkan agar mencakup nilai yang paling mungkin)",
  "l.scheduling_view.estimated": "Diperkirakan",
  "l.scheduling_view.pert_favours_the_likely_value":
    "PERT (condong ke nilai yang paling mungkin)",
  "l.scheduling_view.of_the_simulated_outcomes_cost":
    "{round}% hasil simulasi berbiaya tidak lebih dari anggaran ({budgetText}).",
  "l.scheduling_view.estimated_widened_to_include_the_2":
    "Diperkirakan (dilebarkan agar mencakup jumlah yang direncanakan)",
  "l.scheduling_view.stated_struck_in_of_runs":
    "dinyatakan {round}%, terjadi pada {toFixed}% dari proses",
  "l.scheduling_view.within_budget_and_on_time":
    "Dalam anggaran dan tepat waktu",
  "l.scheduling_view.within_budget_but_late": "Dalam anggaran tetapi terlambat",
  "l.scheduling_view.on_time_but_over_budget":
    "Tepat waktu tetapi melebihi anggaran",
  "l.scheduling_view.over_budget_and_late": "Melebihi anggaran dan terlambat",
  "l.scheduling_view.both_within_budget_and_on":
    "Dalam anggaran sekaligus tepat waktu: {pct} (dalam anggaran saja {pct2}; tepat waktu saja {pct3}).",
  "l.scheduling_view.a_late_finish_costs_nothing":
    "Penyelesaian terlambat tidak menambah biaya dalam proses ini (biaya keterlambatan per hari kerja adalah 0).",
  "l.scheduling_view.each_business_day_late_adds":
    "Setiap hari kerja terlambat menambah {format}; selesai lebih awal tidak mengembalikan apa pun.",
  "l.simulation_inputs.the_probability_must_be_a":
    "Probabilitas harus berupa bilangan bulat dari 0 hingga 100.",
  "l.simulation_inputs.name_the_task_and_the":
    "Sebutkan tugas dan hari kerja yang ditambahkannya sekaligus, atau kosongkan keduanya.",
  "l.simulation_inputs.the_business_days_a_risk":
    "Hari kerja yang ditambahkan risiko harus berupa bilangan bulat dari 1 hingga 365.",
  "l.simulation_inputs.give_the_cost_and_its":
    "Isi biaya dan mata uangnya sekaligus, atau kosongkan keduanya.",
  "l.simulation_inputs.the_cost_must_be_an":
    "Biaya harus berupa jumlah di atas nol.",
  "l.simulation_inputs.the_currency_must_be_a":
    "Mata uang harus berupa kode 3 huruf, misalnya EUR, JPY atau GBP.",
  "l.simulation_inputs.a_risk_that_changes_the":
    "Risiko yang mengubah jadwal atau biaya memerlukan probabilitas.",
  "l.simulation_inputs.amounts_must_be_numbers_zero":
    "Jumlah harus berupa angka, nol atau lebih.",
  "l.simulation_inputs.the_low_amount_cannot_be":
    "Jumlah rendah tidak boleh di atas jumlah yang direncanakan.",
  "l.simulation_inputs.the_high_amount_cannot_be":
    "Jumlah tinggi tidak boleh di bawah jumlah yang direncanakan.",
  "l.simulation_inputs.business_days_must_be_whole":
    "Hari kerja harus berupa bilangan bulat dari 0 hingga 3650.",
  "l.simulation_inputs.give_the_task_a_duration":
    "Beri tugas durasi terlebih dahulu: itulah nilai yang paling mungkin.",
  "l.simulation_inputs.optimistic_cannot_be_above_the":
    "Optimistis tidak boleh di atas durasi yang paling mungkin ({likely}).",
  "l.simulation_inputs.pessimistic_cannot_be_below_the":
    "Pesimistis tidak boleh di bawah durasi yang paling mungkin ({likely}).",
  "l.simulation_inputs.no_range_the_simulation_uses":
    "Tanpa rentang: simulasi memakai rentang cadangan",
  "l.simulation_inputs.to": "{fmt} hingga {fmt2}",
  "l.wbs.restricted_item": "Item terbatas",
  "l.whatif_view.choose_a_task": "Pilih tugas.",
  "l.whatif_view.give_the_new_start_date": "Berikan tanggal mulai baru.",
  "l.whatif_view.duration_must_be_a_whole":
    "Durasi harus berupa bilangan bulat hari kerja dari 0 hingga 3650.",
  "l.whatif_view.choose_a_constraint": "Pilih batasan.",
  "l.whatif_view.a_constraint_needs_a_date": "Batasan memerlukan tanggal.",
  "l.whatif_view.a_hard_constraint_needs_a": "Batasan keras memerlukan alasan.",
  "l.whatif_view.choose_both_tasks": "Pilih kedua tugas.",
  "l.whatif_view.a_task_cannot_depend_on":
    "Tugas tidak dapat bergantung pada dirinya sendiri.",
  "l.whatif_view.lag_must_be_a_whole":
    "Jeda harus berupa bilangan bulat hari kerja (negatif berarti mendahului).",
  "l.whatif_view.choose_a_link": "Pilih tautan.",
  "l.whatif_view.start_on": "Mulai “{name}” pada {start_date}",
  "l.whatif_view.make_a_milestone": "Jadikan “{name}” sebuah pencapaian",
  "l.whatif_view.for": "{constraint_kind} {constraint_date} untuk “{name}”",
  "l.whatif_view.clear_the_constraint_on": "Hapus batasan pada “{name}”",
  "l.whatif_view.lag": ", jeda {lag_days}",
  "l.whatif_view.follows": "“{name}” mengikuti “{name2}” ({toUpperCase}{v})",
  "l.whatif_view.remove_the_link": "Hapus tautan {slice}",
  "l.whatif_view.the_plan_has_no_finish":
    "Rencana belum memiliki tanggal selesai.",
  "l.whatif_view.the_finish_stays": "Tanggal selesai tetap {after}.",
  "l.whatif_view.the_finish_becomes": "Tanggal selesai menjadi {after}.",
  "l.whatif_view.the_plan_no_longer_has":
    "Rencana tidak lagi memiliki tanggal selesai (sebelumnya {before}).",
  "l.whatif_view.the_finish_moves_from_to":
    "Tanggal selesai bergeser dari {before} ke {after}.",
  "l.whatif_view.cost_impact_is_finance_data":
    "Dampak biaya adalah data keuangan, sehingga tidak ditampilkan kepada Anda.",
  "l.whatif_view.no_task_with_work_has":
    "Tidak ada tugas berpekerjaan yang memiliki tarif, sehingga tidak ada biaya untuk dibandingkan.",
  "l.work_view.work_must_be_a_number":
    "Pekerjaan harus berupa jumlah jam di atas nol.",
  "l.work_view.duration_must_be_a_whole":
    "Durasi harus berupa bilangan bulat hari kerja dari 1 hingga 3650.",
  "l.work_view.units_must_be_a_whole":
    "Unit harus berupa persen bulat dari satu hari, dari 1 hingga 1000.",
  "l.work_view.give_the_units_too": "Isi juga unitnya.",
  "l.work_view.business_days_at_is":
    "{duration_days} hari kerja pada {units_percent}% adalah {hoursText}",
  "l.work_view.the_duration_was_worked_out": "Durasi telah dihitung.",
  "l.work_view.the_share_of_the_day": "Porsi hari telah dihitung.",
  "l.work_view.the_work_was_worked_out": "Pekerjaan telah dihitung.",
  "l.work_view.rounded_the_three_do_not":
    " Dibulatkan: ketiganya tidak cocok hingga ke menit.",
  "l.work_view.capacity_not_checked_the_task":
    "Kapasitas tidak diperiksa: tugas belum memiliki tanggal.",
  "l.work_view.capacity_undeclared":
    "Tidak ada kapasitas yang dinyatakan untuk beberapa hari ini, jadi ini tidak melebihi dan tidak di dalam kapasitas orang tersebut.",
  "l.work_view.within_the_person_s_capacity": "Dalam kapasitas orang tersebut.",
  "l.work_view.give_1_to_40_whole":
    "Isi 1 hingga 40 persentase bulat, dipisahkan koma.",
  "l.work_view.the_percentages_add_up_to":
    "Persentase berjumlah {sum}, bukan 100.",
  "p.home.main_x_plans": "Main X · Rencana",
  "p.announcements.could_not_load_the_announcements":
    "Pengumuman tidak dapat dimuat",
  "p.announcements.could_not_publish": "Tidak dapat memublikasikan",
  "p.announcements.could_not_retract": "Tidak dapat menarik kembali",
  "p.auditor.auditor_ppm": "Auditor — PPM",
  "p.automations.could_not_load_automations": "Otomasi tidak dapat dimuat",
  "p.automations.rule_saved_it_fires_from":
    "Aturan disimpan — berjalan mulai dari perpindahan berikutnya yang cocok",
  "p.automations.could_not_save_the_rule": "Aturan tidak dapat disimpan",
  "p.automations.that_action_was_refused": "Tindakan itu ditolak",
  "p.automations.capped_at_run_again_for":
    " (dibatasi {cap} — jalankan lagi untuk sisanya)",
  "p.automations.swept_fired_already_claimed":
    "Disapu: {fired} dijalankan, {skipped_already_claimed} sudah diklaim{v}",
  "p.automations.the_sweep_failed": "Penyapuan gagal",
  "p.board.board_ppm": "Papan — PPM",
  "p.calendar.calendar_ppm": "Kalender — PPM",
  "p.ceo.could_not_load": "Tidak dapat memuat",
  "p.compliance.compliance_ppm": "Kepatuhan — PPM",
  "p.cost_rates.could_not_load_the_rate": "Tabel tarif tidak dapat dimuat",
  "p.cost_rates.rate_table_added": "Tabel tarif ditambahkan.",
  "p.cost_rates.could_not_add_the_table": "Tabel tidak dapat ditambahkan",
  "p.cost_rates.give_a_date_and_an":
    "Isi tanggal dan tarif per jam nol atau lebih.",
  "p.cost_rates.rate_added": "Tarif ditambahkan.",
  "p.cost_rates.could_not_add_the_rate": "Tarif tidak dapat ditambahkan",
  "p.cost_risk.could_not_run_the_simulation": "Simulasi tidak dapat dijalankan",
  "p.documents.could_not_load_the_documents": "Dokumen tidak dapat dimuat",
  "p.documents.could_not_create_the_document": "Dokumen tidak dapat dibuat",
  "p.documents.pid.could_not_load_the_document": "Dokumen tidak dapat dimuat",
  "p.documents.pid.that_did_not_work": "Itu tidak berhasil",
  "p.documents.pid.enter_the_approver_s_id":
    "Masukkan id penyetuju (id orang, bukan diri Anda sendiri).",
  "p.documents.pid.sent_for_review": "Dikirim untuk ditinjau.",
  "p.documents.pid.needs_changes": "Perlu perubahan",
  "p.documents.pid.approved": "Disetujui.",
  "p.documents.pid.sent_back_to_draft": "Dikembalikan ke draf.",
  "p.documents.pid.in_force_from_today": "Berlaku mulai hari ini.",
  "p.documents.pid.next_draft_started": "Draf berikutnya dimulai.",
  "p.documents.pid.could_not_look_that_up": "Tidak dapat mencarinya",
  "p.engineering.engineering_ppm": "Rekayasa — PPM",
  "p.executive.executive_ppm": "Eksekutif — PPM",
  "p.financials.financials_ppm": "Keuangan — PPM",
  "p.gantt.gantt_main_x": "Gantt — Main X",
  "p.gantt.calendar_failed":
    "Kalender kerja tidak dapat dimuat, sehingga hari tidak kerja tidak diberi bayangan.",
  "p.kpis.could_not_load_the_kpis": "KPI tidak dapat dimuat",
  "p.kpis.could_not_define_the_kpi": "KPI tidak dapat didefinisikan",
  "p.lifecycle.could_not_load_the_funnel": "Corong tidak dapat dimuat",
  "p.me.could_not_load_your_tasks": "Tugas Anda tidak dapat dimuat",
  "p.me.calendar.could_not_load_your_calendar":
    "Kalender Anda tidak dapat dimuat",
  "p.me.calendar.could_not_save": "Tidak dapat menyimpan",
  "p.me.calendar.could_not_remove": "Tidak dapat menghapus",
  "p.milestones.could_not_load_the_milestones": "Pencapaian tidak dapat dimuat",
  "p.objectives.create_failed": "pembuatan gagal",
  "p.objectives.alignment_failed": "penyelarasan gagal",
  "p.okrs.could_not_load_the_okrs": "OKR tidak dapat dimuat",
  "p.okrs.could_not_create_the_objective": "Sasaran tidak dapat dibuat",
  "p.plans.plans_main_x": "Rencana — Main X",
  "p.plans.pid.main_x": "{v} — Main X",
  "p.plans.pid.board.pt": " · {points} poin",
  "p.plans.pid.board.board_ppm": "Papan — PPM",
  "p.plans.pid.calendar.could_not_load_the_calendar":
    "Kalender tidak dapat dimuat",
  "p.plans.pid.calendar.could_not_save": "Tidak dapat menyimpan",
  "p.plans.pid.cost_risk.could_not_run_the_simulation":
    "Simulasi tidak dapat dijalankan",
  "p.plans.pid.costs.could_not_load_the_costs": "Biaya tidak dapat dimuat",
  "p.plans.pid.costs.choose_a_task": "Pilih tugas.",
  "p.plans.pid.costs.the_amount_must_be_above": "Jumlah harus di atas nol.",
  "p.plans.pid.costs.the_currency_must_be_a":
    "Mata uang harus berupa kode 3 huruf, misalnya EUR, JPY atau GBP.",
  "p.money.currency_hint": "misalnya EUR, JPY atau GBP",
  "p.money.amount_or_currency":
    "Isi jumlah sebagai angka dan mata uang sebagai kode 3 huruf, misalnya EUR, JPY atau GBP.",
  "p.plans.pid.costs.expense_added": "Pengeluaran ditambahkan.",
  "p.plans.pid.costs.could_not_add_the_expense":
    "Pengeluaran tidak dapat ditambahkan",
  "p.plans.pid.costs.could_not_record": "Tidak dapat mencatat",
  "p.plans.pid.critical_path.could_not_load_the_baselines":
    "Baseline tidak dapat dimuat",
  "p.plans.pid.critical_path.could_not_take_the_baseline":
    "Baseline tidak dapat dibuat",
  "p.plans.pid.critical_path.could_not_load_the_schedule":
    "Jadwal tidak dapat dimuat",
  "p.plans.pid.critical_path.could_not_save_the_estimate":
    "Perkiraan tidak dapat disimpan",
  "p.plans.pid.critical_path.could_not_save": "Tidak dapat menyimpan",
  "p.plans.pid.earned_value.could_not_load_earned_value":
    "Nilai yang diperoleh tidak dapat dimuat",
  "p.plans.pid.earned_value.could_not_save": "Tidak dapat menyimpan",
  "p.plans.pid.earned_value.could_not_record": "Tidak dapat mencatat",
  "p.plans.pid.edit.edit_case_main_x": "Ubah kasus — Main X",
  "p.plans.pid.flow.could_not_load_flow_data": "Data alur tidak dapat dimuat.",
  "p.plans.pid.handover.handover_is_about_people_so":
    "Serah terima menyangkut orang, sehingga memerlukan izin untuk mengelola orang.",
  "p.plans.pid.handover.could_not_preview": "Tidak dapat menampilkan pratinjau",
  "p.plans.pid.handover.what_this_person_holds_changed":
    "Apa yang dipegang orang ini berubah sejak pratinjau, sehingga tidak ada yang diubah. Pratinjau lagi, lalu terapkan.",
  "p.plans.pid.handover.could_not_commit": "Tidak dapat menerapkan",
  "p.plans.pid.handover.discarded_nothing_was_changed":
    "Dibuang. Tidak ada yang diubah.",
  "p.plans.pid.import.choose_the_file_first": "Pilih berkas terlebih dahulu.",
  "p.plans.pid.import.the_import_did_not_work": "Impor tidak berhasil",
  "p.plans.pid.joint_risk.could_not_run_the_simulation":
    "Simulasi tidak dapat dijalankan",
  "p.plans.pid.leveling.could_not_propose_a_leveling":
    "Perataan tidak dapat diusulkan",
  "p.plans.pid.leveling.could_not_apply_the_moves":
    "Pemindahan tidak dapat diterapkan",
  "p.plans.pid.onboarding.could_not_load": "Tidak dapat memuat",
  "p.plans.pid.onboarding.could_not_open_the_checklist":
    "Daftar periksa tidak dapat dibuka",
  "p.plans.pid.onboarding.checklist_started": "Daftar periksa dimulai.",
  "p.plans.pid.onboarding.could_not_start": "Tidak dapat memulai",
  "p.plans.pid.onboarding.could_not_save": "Tidak dapat menyimpan",
  "p.plans.pid.onboarding.checklist_completed": "Daftar periksa selesai.",
  "p.plans.pid.onboarding.could_not_complete": "Tidak dapat menyelesaikan",
  "p.plans.pid.onboarding.say_why_you_are_cancelling":
    "Sebutkan alasan Anda membatalkan.",
  "p.plans.pid.onboarding.checklist_cancelled": "Daftar periksa dibatalkan.",
  "p.plans.pid.onboarding.could_not_cancel": "Tidak dapat membatalkan",
  "p.plans.pid.onboarding.could_not_record": "Tidak dapat mencatat",
  "p.plans.pid.raci.could_not_load_who_does":
    "Pembagian siapa mengerjakan apa tidak dapat dimuat",
  "p.plans.pid.raci.could_not_save": "Tidak dapat menyimpan",
  "p.plans.pid.raci.use_a_person_or_worker":
    "Gunakan rujukan person: atau worker: dengan UUID.",
  "p.plans.pid.schedule_health.could_not_run_the_schedule":
    "Pemeriksaan kesehatan jadwal tidak dapat dijalankan",
  "p.plans.pid.schedule_risk.could_not_run_the_simulation":
    "Simulasi tidak dapat dijalankan",
  "p.plans.pid.wbs.could_not_load_the_tasks": "Tugas tidak dapat dimuat",
  "p.plans.pid.wbs.link_counts_failed":
    "Jumlah tautan setiap tugas tidak dapat dimuat, sehingga jumlah tidak ditampilkan.",
  "p.plans.pid.what_if.could_not_load_the_plan": "Rencana tidak dapat dimuat",
  "p.plans.pid.what_if.at_most_edits_at_a":
    "Paling banyak {MAX_EDITS} perubahan sekaligus.",
  "p.plans.pid.what_if.edit_staged_preview_to_see":
    "Perubahan disiapkan. Pratinjau untuk melihat dampaknya.",
  "p.plans.pid.what_if.discarded_nothing_was_changed":
    "Dibuang. Tidak ada yang diubah.",
  "p.plans.pid.what_if.could_not_preview": "Tidak dapat menampilkan pratinjau",
  "p.plans.pid.what_if.the_plan_changed_since_this":
    "Rencana berubah sejak pratinjau ini, sehingga tidak ada yang diterapkan. Pratinjau lagi, lalu terapkan.",
  "p.plans.pid.what_if.could_not_commit": "Tidak dapat menerapkan",
  "p.plans.pid.what_if.to": "{early_start} hingga {early_finish}",
  "p.plans.pid.what_if.not_scheduled": "tidak dijadwalkan",
  "p.plans.pid.work.could_not_load_the_plan": "Rencana tidak dapat dimuat",
  "p.plans.pid.work.could_not_load_the_work":
    "Profil pekerjaan tidak dapat dimuat",
  "p.plans.pid.work.could_not_save": "Tidak dapat menyimpan",
  "p.plans.pid.work.contour_saved": "Kontur disimpan.",
  "p.plans.merge.merge_plans_ppm": "Gabungkan rencana — PPM",
  "p.plans.new.new_case_main_x": "Kasus baru — Main X",
  "p.prioritization.could_not_load_the_queue": "Antrean tidak dapat dimuat",
  "p.prioritization.could_not_load_the_score": "Skor tidak dapat dimuat",
  "p.proposals.duplicate_check_failed": "pemeriksaan duplikat gagal",
  "p.regulator.regulator_ppm": "Regulator — PPM",
  "p.reports.save_failed": "penyimpanan gagal",
  "p.reports.run_failed": "proses gagal",
  "p.resources.could_not_load_the_resource":
    "Rencana sumber daya tidak dapat dimuat",
  "p.resources.rbs.could_not_load_resources": "Sumber daya tidak dapat dimuat",
  "p.reviews.could_not_load_reviews": "Tinjauan tidak dapat dimuat",
  "p.reviews.invitation_sent": "Undangan dikirim",
  "p.reviews.could_not_delegate": "Tidak dapat mendelegasikan",
  "p.reviews.score_0_100_leave_blank": "Skor 0–100 (kosongkan jika tidak ada)",
  "p.reviews.score_must_be_between_0": "Skor harus antara 0 dan 100",
  "p.reviews.that_action_was_refused": "Tindakan itu ditolak",
  "p.reviews.could_not_load_consensus": "Konsensus tidak dapat dimuat",
  "p.risk.risk_ppm": "Risiko — PPM",
  "p.scenarios.evaluate_failed": "evaluasi gagal",
  "p.security.security_ppm": "Keamanan — PPM",
  "p.technology.technology_ppm": "Teknologi — PPM",
  "p.tour.tour_main_x_plans": "Tur · Rencana Main X",
  "p.verify.this_sign_in_link_is": "Tautan masuk ini tidak memiliki token.",
  "p.verify.we_could_not_reach_the":
    "Kami tidak dapat menjangkau layanan masuk. Silakan coba lagi sebentar lagi.",
  "p.verify.this_sign_in_link_is_2":
    "Tautan masuk ini tidak valid atau sudah kedaluwarsa.",
  "l.ceo_view.gate_review": "Tinjauan gerbang",
  "l.ceo_view.scenario_committed": "Skenario diterapkan",
  "l.ceo_view.proposal": "Usulan",
  "l.ceo_view.plans_merged": "Rencana digabungkan",
  "c.CumulativeFlow.in_review": "Dalam tinjauan",
  "c.CumulativeFlow.in_progress": "Sedang dikerjakan",
  "c.CumulativeFlow.todo": "Akan dikerjakan",
  "l.document_view.draft": "Draf",
  "l.document_view.in_review": "Dalam tinjauan",
  "l.document_view.approved_not_yet_in_force": "Disetujui, belum berlaku",
  "l.document_view.replaced": "Digantikan",
  "l.document_view.send_for_review": "Kirim untuk ditinjau",
  "l.document_view.approve": "Setujui",
  "l.document_view.bring_into_force_today": "Berlakukan hari ini",
  "l.document_view.start_the_next_draft": "Mulai draf berikutnya",
  "l.document_view.waiting_for_the_approver": "Menunggu penyetuju",
  "l.document_view.approved": "Disetujui",
  "l.document_view.change_this_document": "Ubah dokumen ini",
  "l.document_view.someone_else_approves_it_you":
    "Orang lain yang menyetujuinya: Anda tidak dapat menyetujui dokumen Anda sendiri.",
  "l.document_view.only_the_person_asked_can":
    "Hanya orang yang diminta yang dapat memutuskan.",
  "l.document_view.it_comes_into_force_today":
    "Berlaku mulai hari ini dan menggantikan versi yang kini berlaku.",
  "l.document_view.the_next_draft_starts_as":
    "Draf berikutnya dimulai sebagai salinan teks terbaru.",
  "l.ev_view.on_track": "sesuai jalur",
  "l.ev_view.off_plan": "menyimpang dari rencana",
  "l.ev_view.not_yet_known": "belum diketahui",
  "l.glossary.schedule_performance_index_work_done":
    "Indeks kinerja jadwal: pekerjaan yang selesai dibandingkan dengan pekerjaan yang direncanakan sampai saat ini. Di atas 1,00 berarti lebih cepat dari rencana.",
  "l.glossary.cost_performance_index_value_of":
    "Indeks kinerja biaya: nilai pekerjaan yang selesai dibandingkan dengan biayanya. Di atas 1,00 berarti di bawah anggaran.",
  "l.glossary.schedule_index_by_time_how":
    "Indeks jadwal menurut waktu: sejauh mana rencana berjalan dalam waktu, yang tetap benar meskipun proyek selesai terlambat.",
  "l.glossary.earned_value_the_budget_for":
    "Nilai yang diperoleh: anggaran untuk pekerjaan yang benar-benar selesai.",
  "l.glossary.planned_value_the_budget_for":
    "Nilai terencana: anggaran untuk pekerjaan yang seharusnya sudah selesai sampai saat ini.",
  "l.glossary.actual_cost_what_has_really":
    "Biaya aktual: apa yang sebenarnya sudah dibelanjakan.",
  "l.glossary.estimate_at_completion_the_expected":
    "Perkiraan saat selesai: biaya akhir yang diharapkan.",
  "l.glossary.estimate_to_complete_what_is":
    "Perkiraan hingga selesai: apa yang masih diharapkan akan dibelanjakan.",
  "l.glossary.variance_at_completion_the_expected":
    "Selisih saat selesai: biaya akhir yang diharapkan dibandingkan dengan anggaran.",
  "l.glossary.red_amber_or_green_a": "Merah, kuning, atau hijau: lampu status.",
  "l.glossary.responsible_accountable_consulted_in":
    "Responsible, Accountable, Consulted, Informed: siapa yang mengerjakan, siapa yang mengesahkan, siapa yang dimintai pendapat, dan siapa yang diberi tahu.",
  "l.glossary.a_way_of_weighting_the":
    "Cara memberi bobot lebih pada perkiraan yang paling mungkin dibandingkan kasus terbaik dan terburuk.",
  "l.glossary.key_performance_indicator_a_measure":
    "Indikator kinerja utama: ukuran yang dipantau dari waktu ke waktu terhadap target.",
  "l.glossary.key_performance_indicators_measures_":
    "Indikator kinerja utama: ukuran yang dipantau dari waktu ke waktu terhadap target.",
  "l.glossary.objective_and_key_results_a":
    "Sasaran dan hasil kunci: satu tujuan dengan beberapa hasil yang dapat diukur.",
  "l.glossary.objectives_and_key_results_goals":
    "Sasaran dan hasil kunci: tujuan-tujuan dengan beberapa hasil yang dapat diukur.",
  "l.import_view.jira_csv_export": "Jira (ekspor CSV)",
  "l.import_view.asana_csv_export": "Asana (ekspor CSV)",
  "l.import_view.microsoft_project_xml_file": "Microsoft Project (berkas XML)",
  "l.import_view.columns_not_imported": "Kolom tidak diimpor",
  "l.import_view.rows_skipped": "Baris dilewati",
  "l.import_view.links_not_imported": "Tautan tidak diimpor",
  "l.import_view.parents_not_used": "Induk tidak dipakai",
  "l.import_view.dates_not_understood": "Tanggal tidak dipahami",
  "l.import_view.fields_shortened": "Kolom dipersingkat",
  "l.import_view.status_notes": "Catatan status",
  "l.import_view.resources_not_matched_to_people":
    "Sumber daya tidak dicocokkan dengan orang",
  "l.import_view.costs_not_imported": "Biaya tidak diimpor",
  "l.import_view.schedules_not_imported": "Jadwal tidak diimpor",
  "l.import_view.constraints_not_imported": "Batasan tidak diimpor",
  "l.import_view.calendar_notes": "Catatan kalender",
  "l.kpi_view.improving": "Membaik",
  "l.kpi_view.getting_worse": "Memburuk",
  "l.kpi_view.no_change": "Tidak berubah",
  "l.milestone_view.on_track": "Sesuai jalur",
  "l.milestone_view.at_risk": "Berisiko",
  "l.milestone_view.forecast_late": "Diperkirakan terlambat",
  "l.milestone_view.no_forecast": "Tanpa perkiraan",
  "l.milestone_view.it_is_not_linked_to":
    "Tidak terhubung ke tugas mana pun, sehingga tidak ada dasar untuk membuat perkiraan.",
  "l.milestone_view.its_open_tasks_have_no":
    "Tugas terbukanya tidak memiliki tanggal yang dapat ditempatkan jadwal (beri durasi dan tanggal mulai atau pendahulu).",
  "l.milestone_view.the_plan_s_dependencies_contain":
    "Ketergantungan rencana mengandung siklus, sehingga perkiraan tidak dapat dibuat.",
  "l.okr_view.not_measurable_the_start_and":
    "Tidak dapat diukur: titik awal dan target sama, sehingga tidak ada jarak untuk ditempuh.",
  "l.okr_view.not_measurable_a_maintain_key":
    "Tidak dapat diukur: hasil kunci jenis pertahankan memerlukan rentang toleransi.",
  "l.okr_view.not_measurable_the_numbers_are":
    "Tidak dapat diukur: angkanya terlalu besar untuk dinilai.",
  "l.onboarding_view.someone_joins_this_plan":
    "Seseorang bergabung dengan rencana ini",
  "l.onboarding_view.someone_leaves_this_plan":
    "Seseorang meninggalkan rencana ini",
  "l.onboarding_view.take_this_plan_in": "Terima rencana ini",
  "l.onboarding_view.close_this_plan_out": "Tutup rencana ini",
  "l.onboarding_view.booking": "Pemesanan",
  "l.onboarding.executive": "Eksekutif",
  "l.onboarding.outcomes_and_exposure_across_the":
    "Hasil dan eksposur di seluruh aset, bukan satu rencana saja.",
  "l.onboarding.read_the_executive_summary": "Baca ringkasan eksekutif",
  "l.onboarding.health_decisions_benefits_realized_a":
    "Kesehatan, keputusan, manfaat yang terwujud, dan keselarasan strategis dalam satu tampilan.",
  "l.onboarding.open_the_at_a_glance": "Buka dasbor sekilas",
  "l.onboarding.the_whole_estate_s_rag":
    "Status RAG seluruh aset dan pelanggaran jadwal sekaligus.",
  "l.onboarding.review_the_board_pack": "Tinjau paket dewan",
  "l.onboarding.the_pack_a_board_meeting":
    "Paket yang akan diserahkan dalam rapat dewan: investasi dan tren.",
  "l.onboarding.check_budget_variance_and_exposure":
    "Periksa selisih anggaran dan eksposur",
  "l.onboarding.where_spend_is_diverging_from":
    "Di mana pengeluaran menyimpang dari rencana, per mata uang — mata uang tidak pernah dijumlahkan.",
  "l.onboarding.compare_two_candidate_portfolios":
    "Bandingkan dua portofolio kandidat",
  "l.onboarding.what_if_scenarios_side_by":
    "Skenario andaikan berdampingan, sebelum mengikat dana ke salah satunya.",
  "l.onboarding.see_what_s_ranked_highest":
    "Lihat apa yang berperingkat tertinggi, dan alasannya",
  "l.onboarding.the_smart_score_queue_every":
    "Antrean Smart Score — setiap skor menunjukkan bukti di baliknya, atau menyatakan bahwa tidak ada.",
  "l.onboarding.pmo_portfolio_manager": "PMO / manajer portofolio",
  "l.onboarding.intake_governance_and_the_schedule":
    "Penerimaan, tata kelola, dan jadwal di seluruh rencana.",
  "l.onboarding.open_the_intake_pipeline": "Buka alur penerimaan",
  "l.onboarding.every_proposal_s_stage_from":
    "Tahap setiap usulan, dari draf hingga persetujuan.",
  "l.onboarding.register_a_plan": "Daftarkan rencana",
  "l.onboarding.the_first_plan_you_create":
    "Rencana pertama yang Anda buat adalah cara yang baik untuk melihat seluruh bentuk muatan.",
  "l.onboarding.browse_the_plan_registry": "Telusuri registri rencana",
  "l.onboarding.one_recursive_collection_kind_is":
    "Satu koleksi rekursif — `kind` adalah label, bukan gerbang.",
  "l.onboarding.open_a_plan_s_governance":
    "Buka tab tata kelola sebuah rencana",
  "l.onboarding.gate_reviews_risks_and_budget":
    "Tinjauan gerbang, risiko, dan pos anggaran untuk satu rencana itu.",
  "l.onboarding.look_at_the_cross_plan": "Lihat jadwal lintas rencana",
  "l.onboarding.dependencies_the_critical_path_and":
    "Ketergantungan, jalur kritis, dan penerus yang tertunda.",
  "l.onboarding.check_the_risk_register": "Periksa registri risiko",
  "l.onboarding.everything_open_or_mitigating_ranked":
    "Semua yang terbuka atau sedang dimitigasi, diurutkan menurut eksposur.",
  "l.onboarding.review_the_compliance_register": "Tinjau registri kepatuhan",
  "l.onboarding.standing_findings_across_the_estate":
    "Temuan yang masih berlaku di seluruh aset, bukan hanya satu rencana.",
  "l.onboarding.run_a_saved_report": "Jalankan laporan tersimpan",
  "l.onboarding.a_filter_field_projection_saved":
    "Filter dan proyeksi kolom yang disimpan untuk dipakai ulang, dijalankan sesuai permintaan.",
  "l.onboarding.resource_manager": "Manajer sumber daya",
  "l.onboarding.who_is_allocated_where_and":
    "Siapa dialokasikan di mana, dan bagaimana pengiriman benar-benar mengalir.",
  "l.onboarding.check_the_capacity_rollup": "Periksa rekapitulasi kapasitas",
  "l.onboarding.who_is_over_allocated_this":
    "Siapa yang teralokasi berlebih pada jendela ini, dijumlahkan dari semua rencana yang mereka ikuti.",
  "l.onboarding.open_a_plan_s_task": "Buka papan tugas sebuah rencana",
  "l.onboarding.the_kanban_board_that_task":
    "Papan Kanban tempat pekerjaan tingkat tugas benar-benar bergerak.",
  "l.onboarding.read_that_plan_s_flow": "Baca angka alur rencana itu",
  "l.onboarding.cycle_time_lead_time_and":
    "Waktu siklus, waktu tunggu, dan efisiensi alur — diturunkan dari riwayat papan itu sendiri.",
  "l.onboarding.scan_blocked_items_and_the":
    "Pindai item terhambat dan bauran pengiriman",
  "l.onboarding.what_s_stuck_the_moscow":
    "Apa yang macet, bauran MoSCoW, dan kalender pencapaian dalam satu tampilan.",
  "l.onboarding.look_at_the_milestone_calendar": "Lihat kalender pencapaian",
  "l.onboarding.every_milestone_across_every_plan":
    "Setiap pencapaian di setiap rencana, termasuk tanggal jatuh tempo.",
  "l.onboarding.see_what_automations_are_configured":
    "Lihat otomasi apa yang dikonfigurasi",
  "l.onboarding.what_fires_on_a_field":
    "Apa yang berjalan saat sebuah kolom berubah, dan apa yang dilakukannya.",
  "l.onboarding.open_collaborative_reviews": "Buka tinjauan kolaboratif",
  "l.onboarding.where_a_plan_gets_a":
    "Tempat sebuah rencana mendapat pendapat kedua sebelum keputusan diambil.",
  "l.raci_view.responsible": "Pelaksana",
  "l.raci_view.accountable": "Penanggung jawab akhir",
  "l.raci_view.consulted": "Dikonsultasi",
  "l.raci_view.informed": "Diberi tahu",
  "l.raci_view.does_the_work": "mengerjakan pekerjaan",
  "l.raci_view.answers_for_it_and_signs":
    "bertanggung jawab atasnya dan mengesahkannya (satu per tugas)",
  "l.raci_view.is_asked_before": "dimintai pendapat sebelumnya",
  "l.raci_view.is_told_after": "diberi tahu sesudahnya",
  "l.scheduling_view.no_schedule_baseline_has_been":
    "Belum ada baseline jadwal yang dibuat untuk rencana ini.",
  "l.scheduling_view.nothing_was_baselined_to_finish":
    "Tidak ada yang di-baseline untuk selesai pada tanggal status.",
  "l.scheduling_view.the_plan_s_dependencies_contain":
    "Ketergantungan rencana mengandung siklus.",
  "l.scheduling_view.there_are_no_incomplete_tasks":
    "Tidak ada tugas yang belum selesai untuk diukur.",
  "l.scheduling_view.there_are_no_relationships_between":
    "Tidak ada hubungan antartugas untuk diukur.",
  "l.scheduling_view.no_incomplete_task_has_dates":
    "Tidak ada tugas belum selesai yang memiliki tanggal yang dapat ditempatkan jadwal.",
  "l.scheduling_view.no_incomplete_task_has_a":
    "Tidak ada tugas belum selesai yang memiliki durasi.",
  "l.scheduling_view.the_plan_has_no_tasks": "Rencana tidak memiliki tugas.",
  "l.scheduling_view.there_is_no_critical_task":
    "Tidak ada tugas kritis untuk diuji.",
  "l.scheduling_view.no_path_remains_to_measure":
    "Tidak ada jalur tersisa untuk diukur.",
  "l.scheduling_view.every_task_involved_has_already":
    "Setiap tugas yang terlibat sudah dimulai, sehingga tidak ada yang boleh dipindahkan.",
  "l.scheduling_view.the_tasks_involved_are_critical":
    "Tugas yang terlibat bersifat kritis; memindahkan salah satunya akan menggeser penyelesaian. Izinkan tugas kritis untuk melihat biayanya.",
  "l.scheduling_view.delaying_a_task_enough_would":
    "Menunda tugas cukup lama akan merusak jadwal (kelonggaran negatif atau penyelesaian lebih lambat).",
  "l.scheduling_view.a_task_involved_has_a":
    "Sebuah tugas yang terlibat memiliki batasan keras dan tidak dapat dipindahkan.",
  "l.scheduling_view.the_schedule_would_not_let":
    "Jadwal tidak mengizinkan tugas dimulai lebih lambat lagi.",
  "l.scheduling_view.the_search_stopped_at_its":
    "Pencarian berhenti pada batasnya sebelum menyelesaikan ini.",
  "l.scheduling_view.no_probability": "tanpa probabilitas",
  "l.scheduling_view.no_task_named": "tidak ada tugas yang disebut",
  "l.scheduling_view.no_impact_in_business_days":
    "tanpa dampak dalam hari kerja",
  "l.scheduling_view.the_risk_is_not_open": "risikonya tidak terbuka",
  "l.scheduling_view.its_task_is_not_on": "tugasnya tidak ada dalam jadwal",
  "l.scheduling_view.the_plan_has_no_budget":
    "rencana tidak memiliki pos anggaran dalam mata uangnya",
  "l.whatif_view.move_a_task_set_its":
    "Pindahkan tugas (atur tanggal mulainya)",
  "l.whatif_view.change_a_task_s_duration": "Ubah durasi tugas",
  "l.whatif_view.change_a_task_s_constraint": "Ubah batasan tugas",
  "l.whatif_view.add_a_link": "Tambah tautan",
  "l.whatif_view.remove_a_link": "Hapus tautan",
  "l.whatif_view.must_start_on_hard": "Harus mulai pada (keras)",
  "l.whatif_view.must_finish_on_hard": "Harus selesai pada (keras)",
  "l.work_view.fixed_units": "Unit tetap",
  "l.work_view.fixed_duration": "Durasi tetap",
  "l.work_view.fixed_work": "Pekerjaan tetap",
  "l.work_view.changing_the_work_changes_how":
    "Mengubah pekerjaan mengubah lamanya; porsi hari tetap.",
  "l.work_view.changing_the_work_changes_the":
    "Mengubah pekerjaan mengubah porsi hari; durasi tetap.",
  "l.work_view.changing_the_duration_or_the":
    "Mengubah durasi atau porsi hari mengubah yang lain; pekerjaan tetap.",
  "l.work_view.flat_the_same_each_day": "Datar: sama setiap hari",
  "l.work_view.front_loaded_more_at_the":
    "Berat di depan: lebih banyak di awal",
  "l.work_view.back_loaded_more_at_the":
    "Berat di belakang: lebih banyak di akhir",
  "l.work_view.bell_most_in_the_middle": "Lonceng: paling banyak di tengah",
  "l.work_view.turtle_ramps_up_holds_ramps": "Kura-kura: naik, bertahan, turun",
  "l.work_view.custom_percentages": "Persentase kustom",
  "l.work_view.has_no_dates_yet": "belum memiliki tanggal",
  "l.work_view.has_no_business_day_to":
    "tidak memiliki hari kerja untuk ditempati",
  "l.work_view.has_a_contour_that_gives":
    "memiliki kontur yang tidak memberinya bobot",
  "l.work_view.no_rate_applies": "tidak ada tarif yang berlaku",
  "l.work_view.no_rate_applied_on_some":
    "tidak ada tarif yang berlaku pada beberapa hari",
  "l.work_view.has_nobody_assigned": "tidak memiliki orang yang ditugaskan",
  "p._layout_svelte.reviews": "Tinjauan",
  "p._layout_svelte.automations": "Otomasi",
  "p._layout_svelte.goals_okrs": "Tujuan (OKR)",
  "p._layout_svelte.onboarding": "Orientasi",
  "p.ceo.red": "Merah",
  "p.ceo.amber": "Kuning",
  "p.ceo.green": "Hijau",
  "p.display.saved_on_this_device": "Disimpan di perangkat ini.",
  "p.display.saved_to_your_account_so":
    "Disimpan ke akun Anda, sehingga mengikuti Anda ke peramban lain.",
  "p.display.could_not_save_to_your":
    "Tidak dapat menyimpan ke akun Anda saat ini; disimpan di perangkat ini.",
  "p.me.today": "Hari ini",
  "p.me.this_week": "Minggu ini",
  "p.me.later": "Nanti",
  "p.me.no_due_date": "Tanpa tanggal jatuh tempo",
  "p.plans.pid.leveling": "Perataan",
  "p.plans.pid.cost_and_schedule_risk": "Risiko biaya dan jadwal",
  "p.plans.pid.board.todo": "Akan dikerjakan",
  "p.plans.pid.board.in_progress": "Sedang dikerjakan",
  "p.plans.pid.board.in_review": "Dalam tinjauan",
  "p.common.business_day_one": "{n} hari kerja",
  "p.common.business_day_other": "{n} hari kerja",
  "p.common.calendar_day_one": "{n} hari kalender",
  "p.common.calendar_day_other": "{n} hari kalender",
  "l.milestone_view.rescheduled_one": "dijadwalkan ulang {count} kali",
  "l.milestone_view.rescheduled_other": "dijadwalkan ulang {count} kali",
  "l.milestone_view.open_of_linked_one":
    "{open_tasks} dari {linked_tasks} tugas masih terbuka",
  "l.milestone_view.open_of_linked_other":
    "{open_tasks} dari {linked_tasks} tugas masih terbuka",
  "l.cpm_view.restricted_one":
    "{n} ketergantungan ke rencana yang tidak dapat Anda lihat tidak disertakan dalam tanggal ini.",
  "l.cpm_view.restricted_other":
    "{n} ketergantungan ke rencana yang tidak dapat Anda lihat tidak disertakan dalam tanggal ini.",
  "l.cpm_view.monday": "Senin",
  "l.cpm_view.tuesday": "Selasa",
  "l.cpm_view.wednesday": "Rabu",
  "l.cpm_view.thursday": "Kamis",
  "l.cpm_view.friday": "Jumat",
  "l.cpm_view.saturday": "Sabtu",
  "l.cpm_view.sunday": "Minggu",
  "l.ev_view.gap_more":
    "nilai yang diperoleh {money} lebih banyak dari yang direncanakan sampai saat ini",
  "l.ev_view.gap_less":
    "nilai yang diperoleh {money} lebih sedikit dari yang direncanakan sampai saat ini",
  "l.ev_view.cost_line_less":
    "{word}: CPI {ratio}, {money} lebih sedikit dibelanjakan daripada yang diperoleh pekerjaan.",
  "l.ev_view.cost_line_more":
    "{word}: CPI {ratio}, {money} lebih banyak dibelanjakan daripada yang diperoleh pekerjaan.",
  "l.ev_view.tasks_counted_one":
    "{count} tugas dihitung, dibobot {weighting}; {how}",
  "l.ev_view.tasks_counted_other":
    "{count} tugas dihitung, dibobot {weighting}; {how}",
  "l.ev_view.equally": "sama rata",
  "l.onboarding_view.would_move_one": "{n} item akan dipindahkan ke {to}.",
  "l.onboarding_view.would_move_other": "{n} item akan dipindahkan ke {to}.",
  "l.onboarding_view.would_release_one":
    "{n} item akan dilepaskan: tugas dan risiko tidak lagi ditugaskan, pemesanan dan peran dihapus.",
  "l.onboarding_view.would_release_other":
    "{n} item akan dilepaskan: tugas dan risiko tidak lagi ditugaskan, pemesanan dan peran dihapus.",
  "l.whatif_view.set_duration_one":
    "Atur “{name}” menjadi {duration_days} hari kerja",
  "l.whatif_view.set_duration_other":
    "Atur “{name}” menjadi {duration_days} hari kerja",
  "l.work_view.over_capacity_one":
    "Melebihi kapasitas selama {days_over} hari kalender ({from} hingga {to}): ini meminta lebih dari yang dapat diberikan orang tersebut.",
  "l.work_view.over_capacity_other":
    "Melebihi kapasitas selama {days_over} hari kalender ({from} hingga {to}): ini meminta lebih dari yang dapat diberikan orang tersebut.",
  "p.plans.pid.handover.done_one":
    "Selesai: {count} item diserahkan dan dicatat.",
  "p.plans.pid.handover.done_other":
    "Selesai: {count} item diserahkan dan dicatat.",
  "p.plans.pid.what_if.committed_one": "Diterapkan: {count} perubahan dicatat.",
  "p.plans.pid.what_if.committed_other":
    "Diterapkan: {count} perubahan dicatat.",
  "l.import_view.mspdi_hint":
    "Di Microsoft Project gunakan File → Save As → XML Format. Berkas .mpp tidak dapat dibaca.",
  "c.CalendarMonth.january": "Januari",
  "c.CalendarMonth.february": "Februari",
  "c.CalendarMonth.march": "Maret",
  "c.CalendarMonth.april": "April",
  "c.CalendarMonth.may": "Mei",
  "c.CalendarMonth.june": "Juni",
  "c.CalendarMonth.july": "Juli",
  "c.CalendarMonth.august": "Agustus",
  "c.CalendarMonth.september": "September",
  "c.CalendarMonth.october": "Oktober",
  "c.CalendarMonth.november": "November",
  "c.CalendarMonth.december": "Desember",
  "c.LilyGanttTable.schedule": "Jadwal",
  "c.LilyKanbanTable.board": "Papan",
  "l.announcement_view.yesterday": "kemarin",
  "c.CalendarMonth.holiday_state": "hari libur",
  "c.CalendarMonth.absent_state": "absen",
  "l.ev_view.word_ahead": "lebih cepat",
  "l.ev_view.word_behind": "tertinggal",
  "l.ev_view.slipping": "tertunda",
  "l.import_view.task_one": "{n} tugas",
  "l.import_view.task_other": "{n} tugas",
  "l.import_view.dependency_one": "{n} ketergantungan",
  "l.import_view.dependency_other": "{n} ketergantungan",
  "l.import_view.word_update": "perbarui",
  "l.import_view.word_updated": "diperbarui",
  "l.rbs.headcount_one": "{headcount} orang",
  "l.rbs.headcount_other": "{headcount} orang",
  "l.scheduling_view.exactly": "tepat",
  "l.scheduling_view.strength_strong": "kuat",
  "l.scheduling_view.strength_moderate": "sedang",
  "l.scheduling_view.strength_weak": "lemah",
  "l.simulation_inputs.planned_word": "direncanakan",
  "p.plans.pid.board.unassigned": "belum ditugaskan",
  "p.plans.pid.costs.recorded_one":
    "Mencatat {recorded} entri waktu; {already_recorded} sudah tercatat sebelumnya; {length} tidak memiliki tarif.",
  "p.plans.pid.costs.recorded_other":
    "Mencatat {recorded} entri waktu; {already_recorded} sudah tercatat sebelumnya; {length} tidak memiliki tarif.",
  "c.CostRiskView.histogram_label":
    "Histogram biaya {currency} hasil simulasi, dengan P50, P80, P90 dan anggaran ditandai",
  "c.CumulativeFlow.chart_label":
    "Alur kumulatif selama {days} hari kalender, puncak {peak} tugas",
  "p.plans.pid.onboarding.owner_of": "Pemilik {title}",
  "p.plans.pid.onboarding.due_date_of": "Tanggal jatuh tempo {title}",
  "p.plans.pid.onboarding.why_skip_title": "Alasan melewati {title}",
  "p.plans.pid.what_if.remove_edit": "Hapus perubahan {n}",
  "c.LilyGanttTable.cell_active": "{name}: {start} hingga {end}{ends}",
  "c.LilyGanttTable.ends_here": " (berakhir di sini)",
  "c.LilyGanttTable.cell_not_scheduled": "{name}: tidak dijadwalkan",
  "l.simulation_inputs.range_business_days":
    "{optimistic} / {likely} / {pessimistic} hari kerja (optimistis / paling mungkin / pesimistis)",
  "l.enum.task_status.todo": "akan dikerjakan",
  "l.enum.task_status.in_progress": "sedang dikerjakan",
  "l.enum.task_status.in_review": "dalam tinjauan",
  "l.enum.task_status.done": "selesai",
  "l.enum.task_status.blocked": "terhambat",
  "l.enum.kind.Portfolio": "Portofolio",
  "l.enum.kind.Project": "Proyek",
  "l.enum.kind.Product": "Produk",
  "l.enum.kind.Program": "Program",
  "l.enum.kind.Practice": "Praktik",
  "l.enum.kind.Process": "Proses",
  "l.enum.kind.Purpose": "Tujuan",
  "l.enum.kind.Pathway": "Jalur",
  "l.enum.kind.Proposal": "Usulan",
  "l.enum.plan_status.Proposed": "Diusulkan",
  "l.enum.plan_status.Active": "Aktif",
  "l.enum.plan_status.OnHold": "Ditahan",
  "l.enum.plan_status.Completed": "Selesai",
  "l.enum.plan_status.Cancelled": "Dibatalkan",
  "l.enum.collection.portfolios": "portofolio",
  "l.enum.collection.projects": "proyek",
  "l.enum.collection.products": "produk",
  "l.enum.collection.programs": "program",
  "l.enum.collection.practices": "praktik",
  "l.enum.collection.processes": "proses",
  "l.enum.collection.purposes": "tujuan",
  "l.enum.collection.pathways": "jalur",
  "l.enum.proposal_status.draft": "draf",
  "l.enum.proposal_status.submitted": "diajukan",
  "l.enum.proposal_status.in_review": "dalam tinjauan",
  "l.enum.proposal_status.approved": "disetujui",
  "l.enum.proposal_status.rejected": "ditolak",
  "l.enum.proposal_status.promoted": "dipromosikan",
  "l.enum.band.high": "tinggi",
  "l.enum.band.medium": "sedang",
  "l.enum.band.low": "rendah",
  "l.enum.band.unscored": "belum dinilai",
  "l.enum.api.unauthorized": "tidak terotorisasi",
  "l.enum.api.forbidden": "dilarang",
  "l.enum.api.not_found": "tidak ditemukan",
  "l.enum.api.internal_error": "Kesalahan Internal",
  "l.enum.api.rate_limited": "dibatasi laju",
  "l.enum.api.http_status": "HTTP {status}",
  "p.error.not_found": "Halaman itu tidak ada.",
  "p.error.not_allowed": "Anda tidak diizinkan melihat halaman itu.",
  "p.error.went_wrong": "Terjadi kesalahan. Silakan coba lagi.",
  "p.error.title": "Kesalahan {status} — PPM",
  "p.error.back": "Kembali ke awal",
  "p.issues.issues_ppm": "Registri isu — PPM",
  "p.issues.issues": "Registri isu",
  "p.issues.intro":
    "Isu adalah sesuatu yang sudah terjadi dan perlu dikelola; risiko adalah sesuatu yang mungkin terjadi. Yang paling serius lebih dahulu, lalu yang terlama. Usia dihitung dalam hari kalender sejak isu diajukan.",
  "p.issues.could_not_load": "Isu tidak dapat dimuat",
  "p.issues.by_severity": "Isu menurut tingkat keparahan",
  "p.issues.overdue_count": "Terlambat",
  "p.issues.oldest_open": "Terbuka terlama: {title} ({days} hari kalender)",
  "p.issues.show": "Tampilkan",
  "p.issues.all_statuses": "Semua status",
  "p.issues.open_only": "Hanya yang terbuka dan sedang berjalan",
  "p.issues.severity_filter": "Tingkat keparahan",
  "p.issues.all_severities": "Semua tingkat keparahan",
  "p.issues.older_than": "Berusia setidaknya sekian hari kalender",
  "p.issues.none": "Tidak ada isu tercatat.",
  "p.issues.none_match": "Tidak ada isu yang cocok.",
  "p.issues.loading": "Memuat…",
  "p.issues.count_one": "{n} isu, yang paling serius lebih dahulu",
  "p.issues.count_other": "{n} isu, yang paling serius lebih dahulu",
  "p.issues.truncated":
    "Menampilkan {n} pertama; persempit filter untuk melihat sisanya.",
  "p.issues.title_col": "Isu",
  "p.issues.kind_col": "Jenis",
  "p.issues.severity_col": "Tingkat keparahan",
  "p.issues.status_col": "Status",
  "p.issues.owner_col": "Pemilik",
  "p.issues.age_col": "Usia (hari kalender)",
  "p.issues.due_col": "Jatuh tempo",
  "p.issues.overdue_flag": "Terlambat",
  "p.issues.no_owner": "Tanpa pemilik",
  "p.issues.no_due": "Tanpa tanggal jatuh tempo",
  "p.issues.plan_link": "Buka isu rencana",
  "p.issues.plan_issues_link": "Isu",
  "p.plan_issues.title_ppm": "Isu rencana — PPM",
  "p.plan_issues.title": "Isu rencana",
  "p.plan_issues.intro":
    "Isu yang diajukan pada rencana ini, yang paling serius lebih dahulu. Isu adalah sesuatu yang sudah terjadi; risiko adalah sesuatu yang mungkin terjadi. Ajukan isu, ubah isu, atau ajukan isu dari risiko yang sudah terjadi.",
  "p.plan_issues.back": "Kembali ke rencana",
  "p.plan_issues.register_link": "Isu di semua rencana",
  "p.plan_issues.could_not_load": "Isu rencana tidak dapat dimuat",
  "p.plan_issues.could_not_save": "Isu tidak dapat disimpan",
  "p.plan_issues.counts":
    "{total} secara keseluruhan, {open} terbuka atau sedang berjalan, {overdue} terlambat",
  "p.plan_issues.actions_col": "Tindakan",
  "p.plan_issues.edit": "Ubah",
  "p.plan_issues.raise_heading": "Ajukan isu",
  "p.plan_issues.edit_heading": "Ubah isu",
  "p.plan_issues.f_title": "Judul",
  "p.plan_issues.f_description": "Deskripsi",
  "p.plan_issues.f_owner": "Pemilik (rujukan person: atau worker:)",
  "p.plan_issues.f_raised_on": "Diajukan pada (kosongkan untuk hari ini)",
  "p.plan_issues.f_resolution": "Penyelesaian (diperlukan untuk menutup isu)",
  "p.plan_issues.raise": "Ajukan isu",
  "p.plan_issues.save": "Simpan perubahan",
  "p.plan_issues.cancel_edit": "Batalkan perubahan",
  "p.plan_issues.raised": "Isu telah diajukan.",
  "p.plan_issues.saved": "Isu telah disimpan.",
  "p.plan_issues.problem_title_required": "Beri isu sebuah judul.",
  "p.plan_issues.problem_resolution_required":
    "Isu yang ditutup memerlukan penyelesaian: jelaskan bagaimana isu itu diselesaikan.",
  "p.plan_issues.problem_due_before_raised":
    "Tanggal jatuh tempo tidak boleh sebelum hari isu diajukan.",
  "p.plan_issues.from_risk_heading": "Ajukan isu dari risiko",
  "p.plan_issues.from_risk_intro":
    "Untuk risiko yang sudah terjadi. Isu mengambil judul, deskripsi, dan pemilik risiko serta tingkat keparahan dari eksposurnya; risiko itu sendiri tidak diubah. Satu risiko dapat memiliki satu isu terbuka pada satu waktu.",
  "p.plan_issues.f_risk": "Risiko",
  "p.plan_issues.choose_risk": "Pilih risiko",
  "p.plan_issues.raise_from_risk": "Ajukan isu dari risiko ini",
  "p.plan_issues.raised_from_risk": "Isu diajukan dari risiko.",
  "l.enum.severity.low": "rendah",
  "l.enum.severity.med": "sedang",
  "l.enum.severity.high": "tinggi",
  "l.enum.severity.critical": "kritis",
  "l.enum.issue_status.open": "terbuka",
  "l.enum.issue_status.in_progress": "sedang berjalan",
  "l.enum.issue_status.resolved": "terselesaikan",
  "l.enum.issue_status.closed": "ditutup",
  "l.enum.issue_kind.bug": "bug",
  "l.enum.issue_kind.risk": "risiko",
  "l.enum.issue_kind.blocker": "penghambat",
  "l.enum.issue_kind.question": "pertanyaan",
  "l.enum.issue_kind.improvement": "perbaikan",
  "l.enum.issue_kind.concern": "kekhawatiran",
  "p.signin.oidc_idle":
    "Anda dikeluarkan karena tidak aktif selama beberapa waktu. Silakan masuk lagi.",
  "p.plans.pid.board.only_first_tasks":
    "Rencana ini memiliki {total} tugas; papan menampilkan {shown} pertama. Persempit menurut komponen, atau baca sisanya melalui API.",
  "p.plan_modules.title": "Alat pada rencana ini",
  "p.plan_modules.title_ppm": "Alat pada rencana ini — PPM",
  "p.plan_modules.back": "← Kembali ke rencana",
  "p.plan_modules.intro":
    "Pilih alat yang ditampilkan rencana ini. Alat yang nonaktif disembunyikan dari halaman rencana, dan halamannya menjawab “tidak ditemukan”. Tidak ada yang dihapus: mengaktifkan alat lagi menampilkan apa yang dimuatnya.",
  "p.plan_modules.source_plan": "Rencana ini memiliki pilihan alatnya sendiri.",
  "p.plan_modules.source_default": "Rencana ini memakai alat bawaan penerapan.",
  "p.plan_modules.legend": "Alat yang ditampilkan rencana ini",
  "p.plan_modules.module_tasks": "Tugas, papan, dan analisis jadwal",
  "p.plan_modules.module_time": "Pencatatan waktu",
  "p.plan_modules.module_risks": "Risiko dan isu",
  "p.plan_modules.module_financials":
    "Anggaran, biaya, dan nilai yang diperoleh",
  "p.plan_modules.module_milestones": "Pencapaian, jadwal, dan kalender",
  "p.plan_modules.module_governance":
    "Tata kelola, gerbang, dan permintaan perubahan",
  "p.plan_modules.module_roadmap": "Peta jalan produk publik",
  "p.plan_modules.none_chosen":
    "Tanpa alat yang dipilih, halaman rencana menampilkan catatannya dan tanpa alat.",
  "p.plan_modules.save": "Simpan alat",
  "p.plan_modules.presets_heading": "Pilihan siap pakai",
  "p.plan_modules.personal_is":
    "Set pribadi, untuk pekerjaan seseorang sendiri: {names}.",
  "p.plan_modules.use_personal": "Pakai set pribadi",
  "p.plan_modules.default_is": "Bawaan penerapan: {names}.",
  "p.plan_modules.use_default": "Ikuti bawaan penerapan",
  "p.plan_modules.no_tools": "tanpa alat",
  "p.plan_modules.unchanged":
    "Tidak ada yang disimpan: rencana ini sudah mengikuti bawaan, yaitu pilihan ini.",
  "p.plan_modules.saved": "Disimpan.",
  "p.plan_modules.could_not_load": "Alat rencana ini tidak dapat dimuat.",
  "p.plan_modules.could_not_save": "Perubahan tidak disimpan.",
  "p.canvas.title": "Kanvas strategi",
  "p.canvas.title_ppm": "Kanvas strategi — PPM",
  "p.canvas.intro":
    "Kanvas adalah papan satu halaman berisi kotak-kotak, seperti Lean Canvas atau analisis SWOT. Sebuah butir di dalamnya dapat menjadi gagasan, sasaran, atau, pada kanvas rencana, risiko.",
  "p.canvas.plan_title": "Kanvas pada rencana ini",
  "p.canvas.plan_title_ppm": "Kanvas pada rencana ini — PPM",
  "p.canvas.plan_intro":
    "Kanvas strategi pada rencana ini. Sebuah butir di salah satunya dapat menjadi risiko pada registri rencana ini.",
  "p.canvas.one_title": "Kanvas strategi",
  "p.canvas.one_title_ppm": "Kanvas strategi — PPM",
  "p.canvas.back": "← Kembali ke kanvas",
  "p.canvas.list_heading": "Kanvas",
  "p.canvas.filter": "Tampilkan",
  "p.canvas.filter_all": "Semua jenis",
  "p.canvas.none_yet": "Belum ada kanvas.",
  "p.canvas.none_of_kind": "Tidak ada kanvas jenis ini.",
  "p.canvas.free_standing": "tidak pada rencana",
  "p.canvas.on_plan": "pada rencana",
  "p.canvas.new_heading": "Mulai kanvas",
  "p.canvas.new_on_plan": "Kanvas baru berada pada rencana ini.",
  "p.canvas.new_free_standing":
    "Kanvas baru tidak berada pada rencana mana pun, sehingga butirnya dapat menjadi gagasan atau sasaran tetapi bukan risiko.",
  "p.canvas.f_template": "Jenis kanvas",
  "p.canvas.f_title": "Judul",
  "p.canvas.f_description": "Deskripsi (opsional)",
  "p.canvas.create": "Mulai kanvas",
  "p.canvas.could_not_load_list": "Kanvas tidak dapat dimuat.",
  "p.canvas.could_not_load_templates":
    "Jenis kanvas tidak dapat dimuat, sehingga kanvas baru tidak dapat dimulai.",
  "p.canvas.could_not_create": "Kanvas tidak dimulai.",
  "p.canvas.problem_title_blank": "Beri kanvas sebuah judul.",
  "p.canvas.problem_title_long": "Judul paling banyak 200 karakter.",
  "p.canvas.problem_description_long":
    "Deskripsi paling banyak 2.000 karakter.",
  "p.canvas.template_lean": "Lean Canvas",
  "p.canvas.template_business_model": "Business Model Canvas",
  "p.canvas.template_swot": "Analisis SWOT",
  "p.canvas.template_empathy": "Peta empati",
  "p.canvas.template_logic_model": "Model logika",
  "p.canvas.template_risk": "Kanvas risiko",
  "p.canvas.template_goal": "Kanvas tujuan",
  "p.canvas.box_problem": "Masalah",
  "p.canvas.box_solution": "Solusi",
  "p.canvas.box_key_metrics": "Metrik kunci",
  "p.canvas.box_unique_value_proposition": "Proposisi nilai unik",
  "p.canvas.box_unfair_advantage": "Keunggulan tak tertandingi",
  "p.canvas.box_channels": "Saluran",
  "p.canvas.box_customer_segments": "Segmen pelanggan",
  "p.canvas.box_cost_structure": "Struktur biaya",
  "p.canvas.box_revenue_streams": "Aliran pendapatan",
  "p.canvas.box_key_partners": "Mitra kunci",
  "p.canvas.box_key_activities": "Aktivitas kunci",
  "p.canvas.box_key_resources": "Sumber daya kunci",
  "p.canvas.box_value_propositions": "Proposisi nilai",
  "p.canvas.box_customer_relationships": "Hubungan pelanggan",
  "p.canvas.box_strengths": "Kekuatan",
  "p.canvas.box_weaknesses": "Kelemahan",
  "p.canvas.box_opportunities": "Peluang",
  "p.canvas.box_threats": "Ancaman",
  "p.canvas.box_says": "Mengatakan",
  "p.canvas.box_thinks": "Memikirkan",
  "p.canvas.box_does": "Melakukan",
  "p.canvas.box_feels": "Merasakan",
  "p.canvas.box_pains": "Kesulitan",
  "p.canvas.box_gains": "Keuntungan",
  "p.canvas.box_inputs": "Masukan",
  "p.canvas.box_activities": "Aktivitas",
  "p.canvas.box_outputs": "Keluaran",
  "p.canvas.box_outcomes": "Hasil",
  "p.canvas.box_impact": "Dampak",
  "p.canvas.box_vulnerabilities": "Kerentanan",
  "p.canvas.box_existing_controls": "Kendali yang ada",
  "p.canvas.box_residual_risks": "Risiko residual",
  "p.canvas.box_goal": "Tujuan",
  "p.canvas.box_success_measures": "Ukuran keberhasilan",
  "p.canvas.box_actions": "Tindakan",
  "p.canvas.box_obstacles": "Hambatan",
  "p.canvas.box_resources": "Sumber daya",
  "p.canvas.box_empty": "Belum ada isinya.",
  "p.canvas.f_new_item": "Butir baru di {box}",
  "p.canvas.add": "Tambah",
  "p.canvas.added": "Ditambahkan.",
  "p.canvas.f_text": "Teks",
  "p.canvas.f_box": "Kotak",
  "p.canvas.save": "Simpan",
  "p.canvas.cancel": "Batal",
  "p.canvas.saved": "Disimpan.",
  "p.canvas.unchanged": "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.canvas.edit": "Ubah",
  "p.canvas.remove": "Hapus",
  "p.canvas.removed": "Dihapus.",
  "p.canvas.confirm_remove_item": "Hapus “{text}” dari kanvas?",
  "p.canvas.promote": "Jadikan…",
  "p.canvas.promote_legend": "Jadikan butir ini",
  "p.canvas.target_idea": "gagasan",
  "p.canvas.target_objective": "sasaran",
  "p.canvas.target_risk": "risiko pada rencana ini",
  "p.canvas.promoted": "Dijadikan {kind}.",
  "p.canvas.promote_once":
    "Sebuah butir hanya dapat dijadikan sesuatu satu kali.",
  "p.canvas.promote_go": "Jadikan ini",
  "p.canvas.no_risk_free_standing":
    "Kanvas ini tidak berada pada rencana mana pun, sehingga butir tidak dapat menjadi risiko.",
  "p.canvas.f_probability": "Probabilitas (1 sampai 5)",
  "p.canvas.f_impact": "Dampak (1 sampai 5)",
  "p.canvas.problem_risk_scores":
    "Risiko memerlukan probabilitas dan dampak, masing-masing bilangan bulat dari 1 sampai 5.",
  "p.canvas.problem_text_blank": "Tulis teks butir terlebih dahulu.",
  "p.canvas.problem_text_long": "Sebuah butir paling banyak 2.000 karakter.",
  "p.canvas.became_idea": "Menjadi gagasan",
  "p.canvas.became_risk": "Menjadi risiko pada rencana ini",
  "p.canvas.became_objective": "Menjadi sasaran",
  "p.canvas.owner": "Pemilik: {owner}",
  "p.canvas.full":
    "Kanvas ini memuat butir sebanyak yang dapat ditampung ({max}); hapus satu untuk menambah yang lain.",
  "p.canvas.could_not_load": "Kanvas ini tidak dapat dimuat.",
  "p.canvas.could_not_add": "Butir tidak ditambahkan.",
  "p.canvas.could_not_save": "Perubahan tidak disimpan.",
  "p.canvas.could_not_remove": "Butir tidak dihapus.",
  "p.canvas.could_not_promote": "Butir tidak dijadikan apa pun.",
  "p.canvas.remove_canvas_heading": "Hapus kanvas",
  "p.canvas.remove_canvas_says":
    "Menghapus kanvas menghapus butirnya. Gagasan, sasaran, dan risiko yang dihasilkan butirnya tetap ada.",
  "p.canvas.remove_canvas": "Hapus kanvas ini",
  "p.canvas.confirm_remove_canvas": "Hapus kanvas “{title}” beserta butirnya?",
  "p.canvas.could_not_remove_canvas": "Kanvas tidak dihapus.",
  "p.plan_templates.title": "Templat rencana",
  "p.plan_templates.title_ppm": "Templat rencana — PPM",
  "p.plan_templates.intro":
    "Templat adalah sebuah rencana beserta rencana di bawahnya, disimpan tanpa orang dan dengan setiap tanggal berupa jumlah hari setelah tanggal mulai, sehingga rencana baru dapat dimulai darinya.",
  "p.plan_templates.saved_heading": "Templat tersimpan",
  "p.plan_templates.starters_heading": "Pemula bawaan",
  "p.plan_templates.none_saved":
    "Belum ada templat yang disimpan. Simpan satu dari halaman sebuah rencana.",
  "p.plan_templates.none_starters": "Tidak ada pemula bawaan.",
  "p.plan_templates.plans_count_one": "{count} rencana",
  "p.plan_templates.plans_count_other": "{count} rencana",
  "p.plan_templates.needs_start": "memerlukan tanggal mulai",
  "p.plan_templates.could_not_load_list": "Templat tidak dapat dimuat.",
  "p.plan_templates.one_title": "Templat rencana",
  "p.plan_templates.one_title_ppm": "Templat rencana — PPM",
  "p.plan_templates.back": "← Kembali ke templat",
  "p.plan_templates.is_starter":
    "Pemula bawaan. Tidak dapat diubah atau dihapus; mulai rencana darinya dan simpan dengan nama Anda sendiri.",
  "p.plan_templates.is_saved": "Templat tersimpan, versi {version}.",
  "p.plan_templates.holds_heading": "Isinya",
  "p.plan_templates.holds_says":
    "Setiap rencana, jumlah tugasnya, dan pencapaiannya sebagai hari setelah tanggal mulai. Tidak ada orang yang disimpan.",
  "p.plan_templates.tasks_count_one": "{count} tugas",
  "p.plan_templates.tasks_count_other": "{count} tugas",
  "p.plan_templates.milestone_at": "{name}: hari ke-{days}",
  "p.plan_templates.start_heading": "Mulai rencana darinya",
  "p.plan_templates.start_says":
    "Rencana baru dibuat dari templat, dengan tanggal dihitung dari tanggal mulai. Tugas dimulai pada langkah pertamanya.",
  "p.plan_templates.f_plan_name": "Nama rencana teratas (opsional)",
  "p.plan_templates.f_start": "Tanggal mulai",
  "p.plan_templates.no_dates":
    "Templat ini tidak memuat tanggal, sehingga tanggal mulai tidak diperlukan.",
  "p.plan_templates.f_parent": "Letakkan di bawah",
  "p.plan_templates.no_parent":
    "Tidak ada rencana: rencana tingkat atas yang baru",
  "p.plan_templates.start": "Mulai rencana",
  "p.plan_templates.started":
    "Dibuat. Rencana: {plans}; tugas: {tasks}; pencapaian: {milestones}.",
  "p.plan_templates.open_new": "Buka rencana baru",
  "p.plan_templates.could_not_load": "Templat ini tidak dapat dimuat.",
  "p.plan_templates.could_not_start": "Rencana tidak dimulai.",
  "p.plan_templates.problem_plan_name_long":
    "Nama rencana paling banyak 1.024 karakter.",
  "p.plan_templates.problem_start_needed":
    "Pilih tanggal mulai: templat ini memuat tanggal.",
  "p.plan_templates.problem_start_bad":
    "Tulis tanggal mulai sebagai hari, misalnya 2027-01-04.",
  "p.plan_templates.remove_heading": "Hapus templat",
  "p.plan_templates.remove_says":
    "Menghapus templat tidak mengubah rencana yang sudah dibuat darinya.",
  "p.plan_templates.remove": "Hapus templat ini",
  "p.plan_templates.confirm_remove": "Hapus templat “{name}”?",
  "p.plan_templates.could_not_remove": "Templat tidak dihapus.",
  "p.plan_templates.save_title": "Simpan sebagai templat",
  "p.plan_templates.save_title_ppm": "Simpan sebagai templat — PPM",
  "p.plan_templates.save_says":
    "Menyimpan rencana ini dan rencana di bawahnya yang dapat Anda baca, beserta tugas, komponen, pencapaian, dan alur kerjanya. Tidak ada orang yang disimpan, dan setiap tanggal menjadi jumlah hari setelah tanggal mulai. Mengubah rencana di kemudian hari tidak mengubah templat.",
  "p.plan_templates.f_name": "Nama templat",
  "p.plan_templates.f_description": "Deskripsi (opsional)",
  "p.plan_templates.save": "Simpan templat",
  "p.plan_templates.saved":
    "Disimpan. Rencana: {plans}; tugas: {tasks}; pencapaian: {milestones}.",
  "p.plan_templates.omitted_one":
    "{count} rencana di bawah rencana ini dilewati, karena Anda tidak dapat membacanya.",
  "p.plan_templates.omitted_other":
    "{count} rencana di bawah rencana ini dilewati, karena Anda tidak dapat membacanya.",
  "p.plan_templates.open_template": "Buka templat",
  "p.plan_templates.name_taken":
    "Templat dengan nama ini sudah ada. Pilih nama lain.",
  "p.plan_templates.could_not_save": "Templat tidak disimpan.",
  "p.plan_templates.problem_name_blank": "Beri templat sebuah nama.",
  "p.plan_templates.problem_name_long":
    "Nama templat paling banyak 120 karakter.",
  "p.plan_export.title": "Ekspor ke Microsoft Project",
  "p.plan_export.title_ppm": "Ekspor ke Microsoft Project — PPM",
  "p.plan_export.intro":
    "Simpan tugas rencana ini sebagai berkas XML MSPDI, format yang dibuka Microsoft Project dan alat penjadwalan lainnya.",
  "p.plan_export.carried_heading": "Isi berkas",
  "p.plan_export.carried":
    "Setiap tugas beserta kerangkanya di bawah induknya, tanggal mulai, durasi dalam hari kerja, pencapaian, batasan, tenggat, dan catatan; apakah sudah selesai; tautan antartugas beserta jenis dan jedanya; dan kalender kerja rencana.",
  "p.plan_export.not_carried_heading": "Yang tidak disertakan",
  "p.plan_export.not_carried":
    "Orang yang ditugaskan dan sumber daya; status apa pun selain selesai (tugas selesai 0 atau 100 persen); sprint dan komponen; usaha; kolom kustom; komentar; baseline; tautan antartugas selain “mendahului”; dan jeda dalam waktu berlalu (setiap jeda dalam hari kerja). Berkas juga menyatakan hal ini.",
  "p.plan_export.limit":
    "Rencana dengan lebih dari 5.000 tugas tidak dapat diekspor.",
  "p.plan_export.not_checked":
    "Berkas telah dibaca kembali oleh pengimpor layanan ini sendiri, tetapi belum dibuka di Microsoft Project itu sendiri.",
  "p.plan_export.download": "Unduh berkas MSPDI",
  "p.plan_export.to_import": "Sebaliknya, masukkan berkas MSPDI ke rencana ini",
  "p.confidence.assess_heading": "Tambah penilaian",
  "p.confidence.assess_says":
    "Nilai yang dinyatakan adalah pandangan seseorang tentang apakah rencana akan terlaksana. Nilai ini ditambahkan ke riwayat dan tidak pernah menggantikan yang sebelumnya.",
  "p.confidence.choose_value": "Pilih nilai",
  "p.confidence.could_not_load":
    "Keyakinan pelaksanaan rencana ini tidak dapat dimuat.",
  "p.confidence.could_not_save": "Penilaian tidak ditambahkan.",
  "p.confidence.f_assessed_on": "Dinilai pada (opsional; hari ini jika kosong)",
  "p.confidence.f_kind": "Siapa yang menyatakan",
  "p.confidence.f_rationale": "Mengapa (setidaknya {min} karakter)",
  "p.confidence.f_value": "Keyakinan pelaksanaan",
  "p.confidence.history_heading": "Riwayat, terbaru dahulu",
  "p.confidence.intro":
    "Pandangan seseorang yang dinyatakan tentang apakah rencana ini akan terlaksana, disimpan di samping RAG yang dihitung dari data rencana dan tidak pernah dicampur dengannya.",
  "p.confidence.kind_independent": "Penilaian independen",
  "p.confidence.kind_self": "Tim rencana itu sendiri (penilaian mandiri)",
  "p.confidence.no_history": "Belum ada penilaian yang dinyatakan.",
  "p.confidence.none_yet":
    "Belum dinilai: belum ada yang menyatakan pandangan. Itu tidak sama dengan hijau.",
  "p.confidence.opens_exception":
    "Kuning/merah atau merah membuka pengecualian untuk rencana ini, atau memperburuk yang sudah terbuka. Seseorang kemudian harus melengkapinya.",
  "p.confidence.problem_future": "Tanggal penilaian tidak boleh di masa depan.",
  "p.confidence.problem_kind": "Pilih siapa yang menyatakan.",
  "p.confidence.problem_rationale":
    "Jelaskan alasannya, dalam setidaknya {min} karakter.",
  "p.confidence.problem_value": "Pilih nilai.",
  "p.confidence.refused_independent":
    "Penilaian independen memerlukan hak penilai independen. Anda dapat menambahkan penilaian mandiri.",
  "p.confidence.save": "Tambah penilaian",
  "p.confidence.saved_exception_one":
    "Ditambahkan. Rencana ini kini memiliki pengecualian terbuka: harus dilengkapi dalam {hours} jam.",
  "p.confidence.saved_exception_other":
    "Ditambahkan. Rencana ini kini memiliki pengecualian terbuka: harus dilengkapi dalam {hours} jam.",
  "p.confidence.saved": "Ditambahkan.",
  "p.confidence.shown_heading": "Yang ditampilkan peta portofolio",
  "p.confidence.shown_independent": "Penilaian independen pada {when}.",
  "p.confidence.shown_self":
    "Penilaian mandiri pada {when}; penilaian independen akan ditampilkan sebagai gantinya.",
  "p.confidence.stale": "usang: lebih lama dari bulan ini",
  "p.confidence.title_ppm": "Keyakinan pelaksanaan — PPM",
  "p.confidence.title": "Keyakinan pelaksanaan",
  "p.exceptions.a_plan": "Sebuah rencana",
  "p.exceptions.breached_one":
    "Dilanggar: tidak dilengkapi dalam {hours} jam sejak dibuka.",
  "p.exceptions.breached_other":
    "Dilanggar: tidak dilengkapi dalam {hours} jam sejak dibuka.",
  "p.exceptions.cancel": "Batal",
  "p.thresholds.could_not_load": "Triase dan ambang batas tidak dapat dimuat.",
  "p.thresholds.could_not_save": "Perubahan tidak disimpan.",
  "p.thresholds.could_not_sweep": "Ambang batas tidak dapat diperiksa.",
  "p.thresholds.decide_says":
    "Mencatat keputusan menutup pengecualian. Ini mencatat apa yang diputuskan; ini tidak menjeda, mengatur ulang, atau menghentikan program, yang dilakukan seseorang melalui status program.",
  "p.thresholds.decide": "Catat keputusan",
  "p.thresholds.decided_done": "Keputusan dicatat: {action}.",
  "p.thresholds.f_action": "Keputusan",
  "p.thresholds.f_amount": "Jumlah batas",
  "p.thresholds.f_count":
    "Paket terbit berstatus merah berturut-turut ({min} sampai {max})",
  "p.thresholds.f_currency": "Mata uang (tiga huruf)",
  "p.thresholds.f_date": "Tanggal selesai perkiraan yang tidak boleh dilewati",
  "p.thresholds.f_expiry":
    "Penimpaan hingga (di masa depan, paling lama {days} hari ke depan)",
  "p.thresholds.f_outcome": "Hasil",
  "p.thresholds.f_reason": "Alasan (setidaknya {min} karakter)",
  "p.thresholds.history_item": "{date}: {outcome}. {reason}",
  "p.thresholds.history": "Triase sebelumnya",
  "p.thresholds.not_set": "Belum diatur",
  "p.thresholds.now_cap": "Terlampaui jika biaya P80 di atas {amount}.",
  "p.thresholds.now_forecast":
    "Terlampaui jika perkiraan tanggal selesai setelah {date}.",
  "p.thresholds.now_red_packs_one":
    "Terlampaui jika keyakinan merah pada {n} paket terbit berturut-turut.",
  "p.thresholds.now_red_packs_other":
    "Terlampaui jika keyakinan merah pada {n} paket terbit berturut-turut.",
  "p.thresholds.opened_by_threshold":
    "Dibuka karena ambang batas program terlampaui.",
  "p.thresholds.problem_action":
    "Pilih jeda, atur ulang, hentikan, atau penimpaan.",
  "p.thresholds.problem_cap": "Batas harus berupa jumlah di atas nol.",
  "p.thresholds.problem_count":
    "Jumlah paket harus berupa bilangan bulat dari {min} sampai {max}.",
  "p.thresholds.problem_currency":
    "Mata uang harus berupa kode ISO 4217 tiga huruf.",
  "p.thresholds.problem_date": "Pilih tanggal.",
  "p.thresholds.problem_expiry_long":
    "Penimpaan dapat berlaku paling lama {days} hari.",
  "p.thresholds.problem_expiry_needed":
    "Penimpaan memerlukan tanggal berlakunya.",
  "p.thresholds.problem_expiry_past": "Tanggal penimpaan harus di masa depan.",
  "p.thresholds.problem_outcome":
    "Pilih selesaikan, perbaiki, jeda, atau hentikan.",
  "p.thresholds.problem_reason": "Alasan harus setidaknya {min} karakter.",
  "p.thresholds.remove_threshold": "Hapus ambang batas",
  "p.thresholds.save_threshold": "Simpan ambang batas",
  "p.thresholds.save_triage": "Catat triase",
  "p.thresholds.sweep_done_one":
    "{checked} ambang batas diperiksa; {n} pengecualian dibuka.",
  "p.thresholds.sweep_done_other":
    "{checked} ambang batas diperiksa; {n} pengecualian dibuka.",
  "p.thresholds.sweep_item": "{programme}: {kind} — {outcome}",
  "p.thresholds.sweep_list": "Ambang batas yang terlampaui",
  "p.thresholds.sweep_says":
    "Pemeriksaan membuka pengecualian pelaksanaan untuk setiap ambang batas yang terlampaui, atau menambah yang sudah terbuka; ini tidak menjeda, mengatur ulang, atau menghentikan apa pun.",
  "p.thresholds.sweep": "Periksa ambang batas sekarang",
  "p.thresholds.threshold_removed": "Ambang batas dihapus: {kind}.",
  "p.thresholds.threshold_saved": "Ambang batas disimpan: {kind}.",
  "p.thresholds.thresholds_heading": "Ambang batas",
  "p.thresholds.thresholds_says":
    "Melampaui ambang batas membuka pengecualian pelaksanaan, yang dijawab seseorang dengan jeda, atur ulang, hentikan, atau penimpaan dengan masa berlaku. Tidak ada yang dijeda, diatur ulang, atau dihentikan secara otomatis.",
  "p.thresholds.triage_heading": "Triase",
  "p.thresholds.triage_latest": "Triase terbaru, {date}: {outcome}. {reason}",
  "p.thresholds.triage_none": "Belum ada triase yang dicatat.",
  "p.thresholds.triage_saved": "Triase dicatat.",
  "p.thresholds.triage_says":
    "Triase adalah pandangan bertanggal tentang ke mana program seharusnya menuju, beserta alasannya. Ini mencatat pandangan; tidak mengubah apa pun tentang program.",
  "p.thresholds.unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.thresholds.action_pause": "Jeda",
  "p.thresholds.action_reset": "Atur ulang",
  "p.thresholds.action_stop": "Hentikan",
  "p.thresholds.action_override": "Timpa",
  "p.thresholds.kind_confidence_red_packs":
    "Keyakinan merah selama beberapa paket",
  "p.thresholds.kind_forecast_past": "Perkiraan selesai melewati suatu tanggal",
  "p.thresholds.kind_p80_cap": "Biaya P80 di atas batas",
  "p.thresholds.outcome_finish": "Selesaikan",
  "p.thresholds.outcome_fix": "Perbaiki",
  "p.thresholds.outcome_pause": "Jeda",
  "p.thresholds.outcome_stop": "Hentikan",
  "p.thresholds.sweep_opened": "pengecualian dibuka",
  "p.thresholds.sweep_already_open": "pengecualian sudah terbuka",
  "p.thresholds.sweep_attached_to_open_exception":
    "ditambahkan ke pengecualian yang terbuka",
  "p.thresholds.sweep_suppressed_by_override":
    "tidak dibuka: penimpaan sedang berlaku",
  "p.exceptions.closed_done": "Keputusan dicatat; pengecualian ditutup.",
  "p.exceptions.complete_it": "Lengkapi",
  "p.exceptions.complete_says":
    "Pengecualian lengkap jika penyebab, keputusan yang diperlukan, pemilik, dan tanggal batas keputusan semuanya ada. Bagian yang dikosongkan mempertahankan isinya semula.",
  "p.exceptions.complete":
    "Lengkap: memiliki penyebab, keputusan yang diperlukan, pemilik, dan tanggal batas keputusan.",
  "p.exceptions.conflict_closed": "Pengecualian ini sudah ditutup.",
  "p.exceptions.could_not_load": "Pengecualian tidak dapat dimuat.",
  "p.exceptions.could_not_save": "Perubahan tidak disimpan.",
  "p.exceptions.decide_it": "Catat keputusan",
  "p.exceptions.decide_says":
    "Mencatat keputusan menutup pengecualian. Meningkatkan keyakinan tidak pernah menutupnya dengan sendirinya.",
  "p.exceptions.decide": "Catat keputusan dan tutup",
  "p.exceptions.f_breached": "Hanya yang dilanggar",
  "p.exceptions.f_decision": "Keputusan (setidaknya {min} karakter)",
  "p.exceptions.f_owner_ref": "Pemilik (rujukan orang)",
  "p.exceptions.f_owner_role": "Peran pemilik (yang ditampilkan paket)",
  "p.exceptions.f_status": "Tampilkan",
  "p.exceptions.intro_one":
    "Pengecualian dibuka ketika keyakinan pelaksanaan sebuah rencana mencapai kuning/merah atau merah. Harus dilengkapi dalam {hours} jam dan hanya ditutup oleh keputusan yang dicatat.",
  "p.exceptions.intro_other":
    "Pengecualian dibuka ketika keyakinan pelaksanaan sebuah rencana mencapai kuning/merah atau merah. Harus dilengkapi dalam {hours} jam dan hanya ditutup oleh keputusan yang dicatat.",
  "p.exceptions.is_closed": "ditutup {when}",
  "p.exceptions.is_open": "terbuka",
  "p.exceptions.missing": "Masih diperlukan: {parts}.",
  "p.exceptions.none": "Tidak ada pengecualian yang cocok.",
  "p.exceptions.opened": "Dibuka {when}",
  "p.exceptions.part_cause": "Penyebab",
  "p.exceptions.part_decide_by": "Putuskan sebelum",
  "p.exceptions.part_decision_needed": "Keputusan yang diperlukan",
  "p.exceptions.part_decision": "Keputusan",
  "p.exceptions.part_owner": "Pemilik",
  "p.exceptions.problem_decision":
    "Tulis keputusan dalam setidaknya {min} karakter.",
  "p.exceptions.problem_part_short":
    "Penyebab, keputusan yang diperlukan, atau pemilik memerlukan setidaknya {min} karakter agar dihitung.",
  "p.exceptions.refused_threshold":
    "Pengecualian ini dibuka oleh ambang batas, sehingga ditutup dengan jeda, atur ulang, hentikan, atau penimpaan yang dicatat, bukan keputusan di sini.",
  "p.exceptions.save": "Simpan",
  "p.exceptions.saved": "Disimpan.",
  "p.exceptions.see_register": "Buka registri pengecualian",
  "p.exceptions.status_all": "Semua pengecualian",
  "p.exceptions.status_closed": "Ditutup",
  "p.exceptions.status_open": "Terbuka",
  "p.exceptions.title_ppm": "Pengecualian — PPM",
  "p.exceptions.title": "Pengecualian",
  "p.exceptions.unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.exceptions.worsened_one": "Memburuk {count} kali sejak dibuka",
  "p.exceptions.worsened_other": "Memburuk {count} kali sejak dibuka",
  "p.exceptions.worst": "Keyakinan terburuk: {value}",
  "p.aims.aims_heading": "Tujuan strategis",
  "p.aims.aims_legend": "Sasaran yang merupakan tujuan strategis",
  "p.aims.aims_says":
    "Tujuan strategis adalah sasaran yang menjadi dasar pengorganisasian portofolio. Membatalkan tandanya juga menghentikannya menjadi tujuan utama program mana pun.",
  "p.aims.choose_aim": "Pilih tujuan",
  "p.aims.choose_programme": "Pilih program",
  "p.aims.could_not_load": "Peta portofolio tidak dapat dimuat.",
  "p.aims.could_not_save": "Perubahan tidak disimpan.",
  "p.aims.f_aim": "Tujuan utama",
  "p.aims.f_period": "Bulan (kosong untuk bulan ini)",
  "p.aims.f_programme": "Program",
  "p.aims.intro":
    "Program yang dapat Anda baca, masing-masing sekali di bawah tujuan strategis utamanya. Keyakinan pelaksanaan adalah nilai yang dinyatakan, ditampilkan di samping RAG yang dihitung dari data, tidak pernah dicampur dengannya; belum dinilai bukan hijau.",
  "p.aims.map_heading": "Peta portofolio, {period}",
  "p.aims.marked": "Ditandai sebagai tujuan strategis.",
  "p.aims.no_aims":
    "Belum ada sasaran yang menjadi tujuan strategis; tandai satu di atas terlebih dahulu.",
  "p.aims.no_objectives": "Belum ada sasaran.",
  "p.aims.no_programmes":
    "Tidak ada program di peta: program adalah rencana berjenis Program.",
  "p.aims.primary_heading": "Tujuan utama sebuah program",
  "p.aims.primary_says":
    "Sebuah program muncul di peta di bawah satu tujuan utamanya; tujuan lain yang dilayaninya disebutkan di sampingnya. Memilih tujuan utama baru menjadikan yang sebelumnya tautan biasa.",
  "p.aims.primary_set": "Tujuan utama ditetapkan.",
  "p.aims.problem_already_primary":
    "Itu sudah menjadi tujuan utama program ini.",
  "p.aims.problem_choose": "Pilih program dan tujuan.",
  "p.aims.problem_period":
    "Pilih bulan sebagai YYYY-MM, atau kosongkan untuk bulan ini.",
  "p.aims.set_primary": "Tetapkan tujuan utama",
  "p.aims.show": "Tampilkan",
  "p.aims.title_ppm": "Peta portofolio — PPM",
  "p.aims.title": "Peta portofolio",
  "p.aims.to_objectives": "Buka sasaran",
  "p.aims.unmarked": "Bukan lagi tujuan strategis.",
  "p.programme.cited_exception":
    "Ini berasal dari pengecualian pelaksanaan program ini.",
  "p.programme.cited_review": "Ini berasal dari tinjauan gerbang program ini.",
  "p.programme.cites_none": "Tidak keduanya",
  "p.programme.conflict_status":
    "Program sudah berstatus itu, atau sudah dihentikan dan tetap dihentikan.",
  "p.programme.could_not_decide": "Keputusan tidak dicatat.",
  "p.programme.could_not_load":
    "Gerbang dan status rencana ini tidak dapat dimuat.",
  "p.programme.could_not_save_gates":
    "Tidak semua tanggal gerbang disimpan; tanggal yang ditampilkan adalah yang dimiliki layanan.",
  "p.programme.decide": "Catat keputusan",
  "p.programme.decided": "Dicatat: program kini {status}.",
  "p.programme.do_pause": "Jeda program",
  "p.programme.do_resume": "Lanjutkan program",
  "p.programme.do_stop": "Hentikan program",
  "p.programme.f_change": "Ubah program menjadi",
  "p.programme.f_exception": "Berasal dari pengecualian (opsional)",
  "p.programme.f_reason": "Alasan (setidaknya {min} karakter)",
  "p.programme.f_review": "Berasal dari tinjauan gerbang (opsional)",
  "p.programme.gates_heading": "Tanggal gerbang yang direncanakan",
  "p.programme.gates_says":
    "Tanggal rencana ini mengharapkan tinjauannya di setiap gerbang. Tanggal tidak dapat dikosongkan setelah ditetapkan, hanya dipindahkan.",
  "p.programme.gates_saved": "Tanggal gerbang disimpan.",
  "p.programme.gates_unchanged":
    "Tidak ada tanggal gerbang yang berubah, jadi tidak ada yang dikirim.",
  "p.programme.held_allocations_one":
    "{count} alokasi ditandai, tidak dihapus.",
  "p.programme.held_allocations_other":
    "{count} alokasi ditandai, tidak dihapus.",
  "p.programme.held_benefits_one": "{count} manfaat tetap dicatat.",
  "p.programme.held_benefits_other": "{count} manfaat tetap dicatat.",
  "p.programme.held_heading": "Yang masih dipegang program",
  "p.programme.held_money": "Uang: direncanakan {planned}, aktual {actual}.",
  "p.programme.history_heading": "Keputusan, terbaru dahulu",
  "p.programme.no_history": "Belum ada keputusan: program aktif.",
  "p.programme.not_a_programme":
    "Hanya rencana berjenis Program yang memiliki status program. Tanggal gerbangnya tetap dapat direncanakan di atas.",
  "p.programme.problem_reason":
    "Berikan alasan dalam setidaknya {min} karakter.",
  "p.programme.problem_status": "Pilih perubahannya.",
  "p.programme.refused_decider":
    "Menjeda, melanjutkan, atau menghentikan program memerlukan hak pengambil keputusan program.",
  "p.programme.refused_not_programme":
    "Hanya rencana berjenis Program yang memiliki status, dan tinjauan atau pengecualian yang dikutip harus milik program ini.",
  "p.programme.save_gates": "Simpan tanggal gerbang",
  "p.programme.status_heading": "Status program",
  "p.programme.status_is": "Status: {status}",
  "p.programme.status_says":
    "Program aktif sampai keputusan yang dicatat menjeda atau menghentikannya. Jeda menandai alokasinya dan tidak menghapus satu pun.",
  "p.programme.stop_warns":
    "Penghentian bersifat final: program tetap dihentikan dan menyimpan catatannya.",
  "p.programme.stopped_final":
    "Program ini dihentikan. Itu final; catatannya disimpan.",
  "p.programme.title_ppm": "Gerbang dan status program — PPM",
  "p.programme.title": "Gerbang dan status program",
  "p.programme.buffer_by_days": "Sejumlah hari kerja",
  "p.programme.buffer_by_rule":
    "Setengah dari pekerjaan kritis ber-baseline, dibulatkan ke atas",
  "p.programme.buffer_declared_one": "Dinyatakan: penyangga {days} hari kerja.",
  "p.programme.buffer_declared_other":
    "Dinyatakan: penyangga {days} hari kerja.",
  "p.programme.buffer_heading": "Penyangga proyek",
  "p.programme.buffer_says":
    "Waktu yang disisihkan di akhir program untuk menyerap keterlambatan. Bagan menunjukkan berapa banyak yang terpakai dibandingkan berapa banyak pekerjaan kritis yang selesai. Pernyataan tidak pernah menggantikan yang sebelumnya; yang terbaru adalah penyangganya. Rencana penyangga rantai kritis, jika ada, dipakai sebagai gantinya.",
  "p.programme.buffer_history": "Pernyataan, terbaru dahulu",
  "p.programme.buffer_was_days_one": "{days} hari kerja, dinyatakan",
  "p.programme.buffer_was_days_other": "{days} hari kerja, dinyatakan",
  "p.programme.buffer_was_rule_one": "{days} hari kerja, menurut aturan",
  "p.programme.buffer_was_rule_other": "{days} hari kerja, menurut aturan",
  "p.programme.could_not_declare_buffer": "Penyangga tidak dinyatakan.",
  "p.programme.could_not_load_buffer": "Penyangga proyek tidak dapat dibaca.",
  "p.programme.declare_buffer": "Nyatakan penyangga",
  "p.programme.f_buffer_days": "Hari kerja (1 sampai {max})",
  "p.programme.f_buffer_how": "Berapa panjang penyangga",
  "p.programme.f_buffer_note": "Catatan (opsional)",
  "p.programme.problem_buffer_days":
    "Tulis penyangga sebagai bilangan bulat hari kerja dari 1 sampai 2000.",
  "p.programme.refused_buffer_rule":
    "Aturan memerlukan baseline jadwal dengan pekerjaan kritis yang memiliki durasi; tetapkan baseline terlebih dahulu, atau berikan jumlah hari.",
  "p.programme.refused_not_programme_buffer":
    "Hanya rencana berjenis Program yang memiliki penyangga proyek.",
  "p.programme.profile_heading": "Model pelaksanaan dan pelaporan",
  "p.programme.f_model": "Model pelaksanaan",
  "p.programme.f_owner": "Organisasi pemilik",
  "p.programme.no_owner": "Tanpa organisasi pemilik",
  "p.programme.model_says":
    "Model pelaksanaan menyatakan siapa yang membangun dan siapa yang menerapkan; paket direktur menampilkannya di samping program. Model dan pemilik disimpan bersama.",
  "p.programme.save_model": "Simpan model pelaksanaan",
  "p.programme.model_saved": "Model pelaksanaan disimpan.",
  "p.programme.f_method": "Cara program melaporkan kemajuan",
  "p.programme.method_evm": "Nilai yang diperoleh (kinerja biaya dan jadwal)",
  "p.programme.method_product": "Burn-up dan alur (program produk)",
  "p.programme.save_method": "Simpan metode pelaporan",
  "p.programme.method_saved_evm":
    "Program kini melaporkan menurut nilai yang diperoleh.",
  "p.programme.method_saved_product":
    "Program kini melaporkan menurut burn-up dan alur.",
  "p.programme.profile_unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.programme.could_not_load_profile": "Model pelaksanaan tidak dapat dimuat.",
  "p.programme.could_not_save_profile": "Perubahan tidak disimpan.",
  "p.benefits.classified": "Klasifikasi disimpan.",
  "p.benefits.classify_says":
    "Menyimpan mengirim seluruh klasifikasi: pilihan yang dibiarkan tidak dinyatakan akan menghapusnya.",
  "p.benefits.could_not_load": "Manfaat rencana ini tidak dapat dimuat.",
  "p.benefits.could_not_save": "Klasifikasi tidak disimpan.",
  "p.benefits.could_not_validate": "Validasi tidak dicatat.",
  "p.benefits.f_annual": "Jumlah tahunan ({currency})",
  "p.benefits.f_classification": "Klasifikasi",
  "p.benefits.f_kpi": "KPI yang mengukurnya",
  "p.benefits.f_note": "Catatan (opsional)",
  "p.benefits.f_recurrence": "Berulang",
  "p.benefits.f_validated": "Jumlah tervalidasi sampai saat ini ({currency})",
  "p.benefits.intro":
    "Jumlah yang terwujud adalah klaim. Ia hanya divalidasi oleh validasi tercatat dari seseorang di luar program; masing-masing menyatakan jumlah tervalidasi sampai saat ini, dan yang terbaru yang dihitung.",
  "p.benefits.measured_by_kpi":
    "Manfaat nonkeuangan diukur oleh KPI-nya, bukan divalidasi sebagai jumlah.",
  "p.benefits.money_line": "Target {target}; terwujud (diklaim) {realized}.",
  "p.benefits.no_kpi": "Tanpa KPI",
  "p.benefits.non_financial": "Manfaat nonkeuangan.",
  "p.benefits.none": "Rencana ini belum memiliki manfaat.",
  "p.benefits.problem_annual_amount":
    "Tulis jumlah tahunan sebagai angka, tidak di bawah 0.",
  "p.benefits.problem_annual_financial":
    "Hanya manfaat keuangan, yang memiliki mata uang, yang memiliki jumlah tahunan.",
  "p.benefits.problem_annual_recurring":
    "Hanya manfaat berulang yang memiliki jumlah tahunan: pilih berulang terlebih dahulu.",
  "p.benefits.problem_validated_amount":
    "Tulis jumlah tervalidasi sampai saat ini sebagai angka, tidak di bawah 0.",
  "p.benefits.refused_member":
    "Validator tidak boleh menjadi anggota program: minta seseorang di luar program untuk memvalidasi.",
  "p.benefits.save": "Simpan klasifikasi",
  "p.benefits.standing_claimed": "Diklaim, belum divalidasi",
  "p.benefits.standing_not_claimed": "Belum ada yang diklaim",
  "p.benefits.standing_partly_validated": "Sebagian tervalidasi",
  "p.benefits.standing_validated": "Tervalidasi",
  "p.benefits.title_ppm": "Manfaat — PPM",
  "p.benefits.title": "Manfaat: klasifikasi dan validasi",
  "p.benefits.validate": "Catat validasi",
  "p.benefits.validated_done": "Validasi dicatat.",
  "p.benefits.validated_to_date": "{amount} tervalidasi sampai saat ini",
  "p.benefits.validation_heading": "Validasi",
  "p.benefits.validation_says":
    "Validasi disimpan sebagaimana dicatat; validasi baru menyatakan jumlah tervalidasi sampai saat ini dan tidak menggantikan satu pun.",
  "p.funding.add": "Tambah sumber",
  "p.funding.added": "Sumber pendanaan ditambahkan.",
  "p.funding.choose_plan": "Pilih rencana",
  "p.funding.could_not_load_lines":
    "Pos anggaran rencana ini tidak dapat dimuat.",
  "p.funding.could_not_load_money":
    "Dana menurut sumber pendanaan tidak dapat dimuat.",
  "p.funding.could_not_load": "Sumber pendanaan tidak dapat dimuat.",
  "p.funding.could_not_save": "Perubahan tidak disimpan.",
  "p.funding.f_combine": "Gabungkan belanja modal dan operasional",
  "p.funding.f_fy": "Tahun keuangan (tahun awalnya; kosong untuk tahun ini)",
  "p.funding.f_name": "Sumber pendanaan baru",
  "p.funding.f_plan": "Rencana",
  "p.funding.f_source": "Sumber pendanaan",
  "p.funding.intro":
    "Dari mana dana setiap program berasal. Pos anggaran tanpa sumber pendanaan dihitung dan didaftar, tidak pernah disembunyikan.",
  "p.funding.is_active": "aktif",
  "p.funding.is_retired": "dipensiunkan",
  "p.funding.line_cleared": "Pos tidak lagi memiliki sumber pendanaan.",
  "p.funding.line_set": "Sumber pendanaan pos ditetapkan.",
  "p.funding.lines_heading": "Pos anggaran dan sumber pendanaannya",
  "p.funding.lines_says":
    "Pilih rencana untuk melihat pos anggarannya dan beri masing-masing sumber pendanaan.",
  "p.funding.money_heading": "Dana menurut sumber pendanaan",
  "p.funding.no_lines": "Rencana ini tidak memiliki pos anggaran.",
  "p.funding.no_source": "Tanpa sumber pendanaan",
  "p.funding.no_sources": "Belum ada sumber pendanaan yang ditambahkan.",
  "p.funding.problem_fy":
    "Tulis tahun keuangan sebagai tahun awalnya, dari 2000 sampai 2100.",
  "p.funding.problem_name_taken": "Sumber pendanaan dengan nama itu sudah ada.",
  "p.funding.problem_name":
    "Nama sumber pendanaan terdiri dari 2 sampai 120 karakter.",
  "p.funding.refused_inactive":
    "Sumber pendanaan itu sudah dipensiunkan, sehingga tidak dapat diberikan ke pos.",
  "p.funding.restore": "Pulihkan",
  "p.funding.restored": "Sumber pendanaan dipulihkan.",
  "p.funding.retire_says":
    "Sumber yang dipensiunkan tidak dapat lagi diberikan ke pos, tetapi pos yang sudah memilikinya tetap menyimpannya.",
  "p.funding.retire": "Pensiunkan",
  "p.funding.retired_option": "{name} (dipensiunkan)",
  "p.funding.retired": "Sumber pendanaan dipensiunkan.",
  "p.funding.show": "Tampilkan",
  "p.funding.sources_heading": "Sumber pendanaan",
  "p.funding.title_ppm": "Pendanaan — PPM",
  "p.funding.title": "Pendanaan",
  "p.director_pack.actions_heading": "Tinjau, terbitkan, atau terbitkan ulang",
  "p.director_pack.aud_board": "Dewan",
  "p.director_pack.aud_escalation_board": "Dewan eskalasi",
  "p.director_pack.aud_executive": "Eksekutif",
  "p.director_pack.aud_org": "Organisasi: {name}",
  "p.director_pack.aud_partner": "Mitra",
  "p.director_pack.choose_audience": "Pilih audiens",
  "p.director_pack.conflict_draft":
    "Periode ini sudah memiliki paket untuk audiens itu. Bukalah: terbitkan drafnya, atau terbitkan ulang dengan catatan perubahan.",
  "p.director_pack.conflict_issue":
    "Paket ini tidak dapat diterbitkan sekarang: sudah diterbitkan, atau merupakan draf terjadwal yang belum ditinjau kantor portofolio.",
  "p.director_pack.conflict_reissue":
    "Hanya versi terbitan terbaru yang diterbitkan ulang, dan versi yang lebih baru sudah ada.",
  "p.director_pack.conflict_review":
    "Draf ini sudah ditinjau, atau bukan lagi draf.",
  "p.director_pack.could_not_act": "Perubahan tidak dilakukan.",
  "p.director_pack.could_not_draft": "Paket tidak dibuat drafnya.",
  "p.director_pack.cut_off_says":
    "Kosongkan untuk memakai sekarang. Paket membaca rencana sebagaimana adanya pada batas waktu; tidak boleh di masa depan atau lebih dari 400 hari lalu.",
  "p.director_pack.draft_heading": "Buat draf paket",
  "p.director_pack.draft_says":
    "Pembuatan draf membaca rencana yang boleh Anda bagikan sebagaimana adanya pada batas waktu dan menyimpan hasilnya sebagai draf. Tidak ada yang dikirim ke audiens sampai seseorang menerbitkannya.",
  "p.director_pack.draft": "Buat draf paket",
  "p.director_pack.f_audience": "Audiens",
  "p.director_pack.f_change_note": "Catatan perubahan",
  "p.director_pack.f_cut_off": "Batas waktu (opsional)",
  "p.director_pack.f_period": "Periode (bulan)",
  "p.director_pack.from_schedule": "dibuat oleh jadwal bulanan",
  "p.director_pack.is_reviewed": "Ditinjau oleh kantor portofolio.",
  "p.director_pack.is_stale": "usang: batas waktunya sebelum akhir periodenya",
  "p.director_pack.issue_says":
    "Penerbitan membekukan paket dan memberi tahu audiensnya. Tidak dapat dibatalkan; perubahan berikutnya adalah penerbitan ulang.",
  "p.director_pack.issue_waits":
    "Jadwal bulanan membuat draf ini, sehingga kantor portofolio meninjaunya sebelum dapat diterbitkan.",
  "p.director_pack.issue": "Terbitkan paket",
  "p.director_pack.issued_done": "Diterbitkan.",
  "p.director_pack.not_reviewed": "Belum ditinjau.",
  "p.director_pack.problem_audience": "Pilih audiens.",
  "p.director_pack.problem_cut_off_bad":
    "Tulis batas waktu sebagai tanggal dan jam.",
  "p.director_pack.problem_cut_off_future":
    "Batas waktu tidak boleh di masa depan.",
  "p.director_pack.problem_cut_off_old":
    "Batas waktu tidak boleh lebih dari 400 hari lalu.",
  "p.director_pack.problem_note":
    "Jelaskan mengapa ada versi baru, dalam setidaknya {min} karakter.",
  "p.director_pack.problem_period": "Pilih periode sebagai bulan.",
  "p.director_pack.refused_edit":
    "Membuat draf, meninjau, dan menerbitkan ulang paket memerlukan akses editor paket direktur.",
  "p.director_pack.refused_issue":
    "Menerbitkan paket memerlukan akses penerbit paket direktur.",
  "p.director_pack.reissue_says":
    "Penerbitan ulang membuat draf versi berikutnya dengan catatan Anda; versi ini tetap dapat dibaca apa adanya. Tulis setidaknya {min} karakter.",
  "p.director_pack.reissue": "Terbitkan ulang dengan catatan perubahan",
  "p.director_pack.review": "Tandai sudah ditinjau",
  "p.director_pack.reviewed_done": "Ditandai sudah ditinjau.",
  "p.director_pack.add_schedule_heading": "Tambah jadwal",
  "p.director_pack.add_schedule": "Tambah jadwal",
  "p.director_pack.conflict_schedule":
    "Audiens itu sudah memiliki jadwal; ubah harinya saja.",
  "p.director_pack.could_not_change_schedule": "Jadwal tidak diubah.",
  "p.director_pack.could_not_load_schedules": "Jadwal tidak dapat dimuat.",
  "p.director_pack.could_not_load_subscriptions":
    "Paket mana yang diberitahukan kepada Anda tidak dapat dimuat.",
  "p.director_pack.could_not_subscribe": "Perubahan tidak disimpan.",
  "p.director_pack.day_says":
    "Tanggal dari 1 sampai 28, agar setiap bulan memilikinya. Proses pertama adalah tanggal tersebut berikutnya.",
  "p.director_pack.f_day": "Tanggal dalam bulan",
  "p.director_pack.last_drafted":
    "Proses terakhir {when}: membuat draf paket untuk {period}.",
  "p.director_pack.last_run": "Proses terakhir {when}.",
  "p.director_pack.last_skipped":
    "Proses terakhir {when}: melewati {period}, yang sudah memiliki paket.",
  "p.director_pack.move_day": "Pindahkan ke tanggal ini",
  "p.director_pack.no_schedules":
    "Belum ada paket yang dibuat drafnya menurut jadwal.",
  "p.director_pack.problem_day": "Pilih tanggal dalam bulan dari 1 sampai 28.",
  "p.director_pack.refused_subscribe":
    "Anda hanya dapat diberi tahu tentang audiens yang paketnya boleh Anda baca.",
  "p.director_pack.schedule_added": "Jadwal ditambahkan.",
  "p.director_pack.schedule_off":
    "Dinonaktifkan. Akan membuat draf pada tanggal {day} setiap bulan.",
  "p.director_pack.schedule_on":
    "Membuat draf pada tanggal {day} setiap bulan; berikutnya pada {next}.",
  "p.director_pack.schedule_saved": "Disimpan.",
  "p.director_pack.schedule_switched_off": "Dinonaktifkan.",
  "p.director_pack.schedule_switched_on": "Diaktifkan.",
  "p.director_pack.schedule_unchanged":
    "Itu sudah tanggalnya, jadi tidak ada yang dikirim.",
  "p.director_pack.schedules_intro":
    "Setiap jadwal membuat draf paket sebuah audiens pada tanggal tertentu setiap bulan. Jadwal tidak pernah menerbitkannya: kantor portofolio meninjau draf dan seseorang menerbitkannya.",
  "p.director_pack.schedules_link": "Jadwal bulanan",
  "p.director_pack.schedules_title_ppm": "Jadwal bulanan — PPM",
  "p.director_pack.schedules_title": "Jadwal bulanan",
  "p.director_pack.subs_heading": "Beri tahu saya saat paket diterbitkan",
  "p.director_pack.subs_legend": "Paket yang ingin diketahui",
  "p.director_pack.subs_says":
    "Anda diberi tahu di layanan, dengan tautan, saat paket untuk audiens pilihan Anda diterbitkan. Pemberitahuan tidak pernah memuat paketnya sendiri.",
  "p.director_pack.subscribed": "Anda akan diberi tahu.",
  "p.director_pack.sweep_heading": "Jalankan yang jatuh tempo sekarang",
  "p.director_pack.sweep_result":
    "Dibuat draf: {drafted}; dilewati karena paket sudah ada: {skipped}.",
  "p.director_pack.sweep_says":
    "Membuat draf setiap paket yang harinya sudah tiba dan belum memilikinya. Tidak ada yang diterbitkan.",
  "p.director_pack.sweep": "Jalankan jadwal yang jatuh tempo",
  "p.director_pack.switch_off": "Nonaktifkan",
  "p.director_pack.switch_on": "Aktifkan",
  "p.director_pack.unsubscribed": "Anda tidak akan diberi tahu lagi.",
  "p.plans.pid.modules_failed":
    "Alat mana yang dipakai rencana ini tidak dapat diperiksa, sehingga semua alat didaftar.",
  "p._layout_svelte.handling_mark": "Tanda penanganan: {mark}",
  "p._layout_svelte.handling_mark_unknown":
    "Tanda penanganan: tidak dapat diperiksa. Anggap rencana ini bertanda sampai dapat diperiksa.",
  "l.enum.cr_status.draft": "draf",
  "l.enum.cr_status.submitted": "diajukan",
  "l.enum.cr_status.under_review": "dalam tinjauan",
  "l.enum.cr_status.approved": "disetujui",
  "l.enum.cr_status.rejected": "ditolak",
  "l.enum.cr_status.implemented": "diimplementasikan",
  "l.enum.cr_status.withdrawn": "ditarik",
  "l.enum.cr_kind.scope": "lingkup",
  "l.enum.cr_kind.schedule": "jadwal",
  "l.enum.cr_kind.cost": "biaya",
  "l.enum.cr_kind.quality": "kualitas",
  "l.enum.cr_kind.other": "lainnya",
  "p.plan_change_requests.title_ppm": "Permintaan perubahan rencana — PPM",
  "p.plan_change_requests.title": "Permintaan perubahan rencana",
  "p.plan_change_requests.intro":
    "Permintaan perubahan pada rencana ini, terbaru dahulu. Buat drafnya dan ubah selama masih draf; ajukan kepada penyetuju bernama, yang bukan Anda; penyetuju meninjaunya dan mencatat keputusan beserta alasan. Permintaan terbuka dapat ditarik.",
  "p.plan_change_requests.register_link":
    "Permintaan perubahan di semua rencana",
  "p.plan_change_requests.could_not_load":
    "Permintaan perubahan rencana tidak dapat dimuat",
  "p.plan_change_requests.could_not_save":
    "Permintaan perubahan tidak dapat disimpan",
  "p.plan_change_requests.steps": "Langkah",
  "p.plan_change_requests.draft_heading": "Buat draf permintaan perubahan",
  "p.plan_change_requests.edit_heading": "Ubah draf",
  "p.plan_change_requests.f_description": "Deskripsi (apa yang akan diubah)",
  "p.plan_change_requests.f_reason": "Alasan (mengapa perubahan diperlukan)",
  "p.plan_change_requests.f_days":
    "Dampak jadwal dalam hari kerja (negatif memajukan penyelesaian; kosongkan jika belum dinilai)",
  "p.plan_change_requests.f_cost":
    "Dampak biaya (kosongkan jika belum dinilai)",
  "p.plan_change_requests.f_currency":
    "Mata uang biaya (tiga huruf, misalnya EUR)",
  "p.plan_change_requests.f_summary": "Ringkasan dampak",
  "p.plan_change_requests.save_draft": "Simpan draf",
  "p.plan_change_requests.drafted": "Draf permintaan perubahan dibuat.",
  "p.plan_change_requests.saved": "Draf disimpan.",
  "p.plan_change_requests.problem_title_required":
    "Beri permintaan perubahan sebuah judul.",
  "p.plan_change_requests.problem_description_required":
    "Jelaskan apa yang akan diubah.",
  "p.plan_change_requests.problem_reason_required":
    "Jelaskan mengapa perubahan diperlukan.",
  "p.plan_change_requests.problem_days_not_whole":
    "Dampak jadwal berupa bilangan bulat hari kerja.",
  "p.plan_change_requests.problem_days_out_of_range":
    "Dampak jadwal harus dari -3650 sampai 3650 hari kerja.",
  "p.plan_change_requests.problem_cost_not_amount":
    "Dampak biaya bukan sebuah jumlah.",
  "p.plan_change_requests.problem_cost_needs_currency":
    "Dampak biaya memerlukan mata uangnya.",
  "p.plan_change_requests.problem_currency_needs_cost":
    "Mata uang memerlukan dampak biaya.",
  "p.plan_change_requests.problem_currency_invalid":
    "Mata uang berupa kode tiga huruf, misalnya EUR, JPY atau GBP.",
  "p.plan_change_requests.steps_heading": "Langkah untuk {title}",
  "p.plan_change_requests.status_now": "Kini berstatus {status}.",
  "p.plan_change_requests.decided": "Diputuskan oleh {who} pada {on}: {reason}",
  "p.plan_change_requests.no_steps":
    "Tidak ada langkah lebih lanjut yang dapat diambil pada permintaan perubahan ini.",
  "p.plan_change_requests.who_note":
    "Layanan menentukan siapa yang boleh mengambil setiap langkah: hanya orang yang membuat drafnya yang mengubah, mengajukan, atau menariknya (administrator juga dapat menariknya), dan hanya penyetuju bernama yang meninjau atau memutuskan.",
  "p.plan_change_requests.submit_heading": "Ajukan untuk diputuskan",
  "p.plan_change_requests.f_approver":
    "Penyetuju (rujukan person: atau worker:, bukan orang yang membuat draf)",
  "p.plan_change_requests.submit": "Ajukan",
  "p.plan_change_requests.submitted": "Permintaan perubahan diajukan.",
  "p.plan_change_requests.problem_approver_not_reference":
    "Sebutkan penyetuju sebagai rujukan person: atau worker:.",
  "p.plan_change_requests.problem_approver_is_requester":
    "Penyetuju tidak boleh orang yang meminta perubahan.",
  "p.plan_change_requests.review": "Mulai tinjauan",
  "p.plan_change_requests.review_started": "Tinjauan dimulai.",
  "p.plan_change_requests.decide_heading": "Catat keputusan",
  "p.plan_change_requests.f_decision": "Keputusan",
  "p.plan_change_requests.approve": "Setujui",
  "p.plan_change_requests.reject": "Tolak",
  "p.plan_change_requests.f_decision_reason":
    "Alasan keputusan (setidaknya 10 karakter)",
  "p.plan_change_requests.decide": "Catat keputusan",
  "p.plan_change_requests.decision_recorded": "Keputusan dicatat.",
  "p.plan_change_requests.problem_decision_reason":
    "Keputusan memerlukan alasan setidaknya 10 karakter.",
  "p.plan_change_requests.withdraw": "Tarik permintaan perubahan",
  "p.plan_change_requests.withdrawn": "Permintaan perubahan ditarik.",
  "p.plan_change_requests.close_steps": "Tutup langkah",
  "p.plan_change_requests.refused_requester":
    "Hanya orang yang membuat draf permintaan perubahan ini yang dapat melakukannya.",
  "p.plan_change_requests.refused_withdraw":
    "Hanya orang yang membuat draf permintaan perubahan ini, atau administrator, yang dapat menariknya.",
  "p.plan_change_requests.refused_approver":
    "Hanya penyetuju bernama yang dapat meninjau atau memutuskan permintaan perubahan ini.",
  "p.plan_change_requests.refused_state":
    "Permintaan perubahan ini sudah berlanjut dan langkah itu tidak lagi terbuka untuknya; daftar telah dibaca ulang.",
  "p.plan_change_requests.refused_invalid":
    "Layanan tidak menerima yang dimasukkan: periksa kolom dan coba lagi.",
  "p.change_requests.change_requests_ppm": "Permintaan perubahan — PPM",
  "p.change_requests.change_requests": "Permintaan perubahan",
  "p.change_requests.intro":
    "Permintaan perubahan adalah usulan terkendali untuk mengubah baseline yang telah disetujui. Satu penyetuju bernama, yang bukan peminta, memutuskan dan memberi alasan. Dampak adalah yang dinyatakan peminta; perkiraan hanya ditampilkan jika ada perubahan andaikan yang dilampirkan.",
  "p.change_requests.could_not_load": "Permintaan perubahan tidak dapat dimuat",
  "p.change_requests.by_status": "Permintaan perubahan menurut status",
  "p.change_requests.awaiting": "Menunggu keputusan",
  "p.change_requests.oldest_open":
    "Terbuka terlama: {title} ({days} hari kalender)",
  "p.change_requests.show": "Tampilkan",
  "p.change_requests.all_statuses": "Semua status",
  "p.change_requests.open_only": "Hanya yang terbuka",
  "p.change_requests.none": "Tidak ada permintaan perubahan tercatat.",
  "p.change_requests.none_match": "Tidak ada permintaan perubahan yang cocok.",
  "p.change_requests.loading": "Memuat…",
  "p.change_requests.count_one": "{n} permintaan perubahan, terbaru dahulu",
  "p.change_requests.count_other": "{n} permintaan perubahan, terbaru dahulu",
  "p.change_requests.title_col": "Permintaan perubahan",
  "p.change_requests.kind_col": "Jenis",
  "p.change_requests.status_col": "Status",
  "p.change_requests.impact_col": "Dampak yang dinyatakan",
  "p.change_requests.age_col": "Usia (hari kalender)",
  "p.change_requests.approver_col": "Penyetuju",
  "p.change_requests.no_approver": "Belum ditentukan",
  "p.change_requests.no_impact": "Tidak ada dampak yang dinyatakan",
  "p.change_requests.impact_days": "{days} hari kerja",
  "p.change_requests.plan_link": "Buka permintaan perubahan rencana",
  "p.director_pack.director_packs_ppm": "Paket direktur — PPM",
  "p.director_pack.director_packs": "Paket direktur",
  "p.director_pack.intro":
    "Paket direktur adalah laporan bulanan kepada direktur, dibuat drafnya sekali, diterbitkan oleh orang tertentu, lalu dibekukan: paket terbitan tidak pernah berubah, dan koreksi adalah versi baru dengan catatan. Setiap angka di bawah dibaca dari paket terbitan; tidak ada yang dihitung ulang di sini.",
  "p.director_pack.could_not_load": "Paket direktur tidak dapat dimuat",
  "p.director_pack.could_not_load_one": "Paket direktur ini tidak dapat dimuat",
  "p.director_pack.loading": "Memuat…",
  "p.director_pack.none": "Tidak ada paket direktur yang tersedia bagi Anda.",
  "p.director_pack.col_period": "Periode",
  "p.director_pack.col_audience": "Audiens",
  "p.director_pack.col_version": "Versi",
  "p.director_pack.col_status": "Status",
  "p.director_pack.col_issued": "Diterbitkan",
  "p.director_pack.status_draft": "draf",
  "p.director_pack.status_issued": "diterbitkan",
  "p.director_pack.open_pack":
    "Buka paket {period}, {audience}, versi {version}",
  "p.director_pack.all_packs": "Semua paket direktur",
  "p.director_pack.pack_heading": "Paket direktur {period} — {audience}",
  "p.director_pack.version_line":
    "Versi {version} · {status} · batas waktu {cutoff}",
  "p.director_pack.issued_on": "Diterbitkan {when}",
  "p.director_pack.draft_note":
    "Ini adalah draf. Belum diterbitkan, sehingga belum dibekukan.",
  "p.director_pack.change_note": "Catatan perubahan: {note}",
  "p.director_pack.previous_pack":
    "Dibandingkan dengan paket {period}, versi {version}.",
  "p.director_pack.download_pdf": "Unduh sebagai PDF",
  "p.director_pack.pdf_note":
    "PDF memuat angka yang sama tetapi kata-kata tetapnya berbahasa Inggris. Untuk menyimpan dalam bahasa lain, cetak halaman ini.",
  "p.director_pack.basis":
    "Catatan sebagaimana adanya saat paket dibuat drafnya, bertanggal pada atau sebelum batas waktu. Isu dan permintaan perubahan tidak menyimpan riwayat status, jadi yang ditutup setelah batas waktu tetapi sebelum pembuatan draf tampil sebagai ditutup. Bagian tanpa isi menyatakannya; ketiadaan tidak pernah ditampilkan sebagai nol atau hijau.",
  "p.director_pack.s_quality": "Kualitas data",
  "p.director_pack.s_changed": "Apa yang berubah",
  "p.director_pack.s_map": "Peta portofolio",
  "p.director_pack.s_exceptions": "Pengecualian",
  "p.director_pack.s_gates": "Keputusan gerbang",
  "p.director_pack.s_funding": "Pendanaan",
  "p.director_pack.s_fever": "Bagan demam penyangga",
  "p.director_pack.s_evm": "Nilai yang diperoleh",
  "p.director_pack.s_benefits": "Manfaat",
  "p.director_pack.s_capacity": "Kapasitas",
  "p.director_pack.s_issues": "Isu",
  "p.director_pack.s_changes": "Permintaan perubahan",
  "p.director_pack.plans_included": "Rencana yang disertakan: {n}",
  "p.director_pack.plans_left_out":
    "Rencana yang dilewati karena bersifat pribadi atau bertanda: {n}",
  "p.director_pack.sections_missing": "Bagian yang belum dibuat: {list}",
  "p.director_pack.all_sections": "Setiap bagian paket tersedia.",
  "p.director_pack.nothing_here": "Tidak ada yang ditampilkan di bagian ini.",
  "p.director_pack.changed_issues": "Isu terbuka: {now}, {change}",
  "p.director_pack.changed_changes":
    "Permintaan perubahan menunggu keputusan: {now}, {change}",
  "p.director_pack.changed_was": "sebelumnya {before}",
  "p.director_pack.changed_unchanged": "tidak berubah",
  "p.director_pack.changed_first":
    "tidak ada paket sebelumnya untuk dibandingkan",
  "p.director_pack.c_programme": "Program",
  "p.director_pack.c_aim": "Tujuan strategis",
  "p.director_pack.c_status": "Status",
  "p.director_pack.c_stage": "Tahap",
  "p.director_pack.c_phase": "Fase",
  "p.director_pack.c_confidence": "Keyakinan pelaksanaan",
  "p.director_pack.c_trend": "Tren",
  "p.director_pack.self_assessed": "penilaian mandiri",
  "p.director_pack.stale": "usang",
  "p.director_pack.no_aim": "Tanpa tujuan strategis",
  "p.director_pack.c_plan": "Rencana",
  "p.director_pack.c_worst": "Peringkat terburuk",
  "p.director_pack.c_need": "Keputusan yang diperlukan",
  "p.director_pack.c_owner_role": "Pemilik (peran)",
  "p.director_pack.c_decide_by": "Putuskan sebelum",
  "p.director_pack.c_hours_open": "Jam terbuka",
  "p.director_pack.ex_window":
    "Pengecualian yang tidak dilengkapi dalam {hours} jam sejak dibuka dianggap dilanggar.",
  "p.director_pack.ex_open": "Terbuka: {n}",
  "p.director_pack.ex_breached": "Dilanggar: {n}",
  "p.director_pack.ex_closed": "Ditutup dalam periode: {n}",
  "p.director_pack.g_passed": "Lulus: {n}",
  "p.director_pack.g_failed": "Gagal: {n}",
  "p.director_pack.g_deferred": "Ditunda: {n}",
  "p.director_pack.g_paused": "Program dijeda: {n}",
  "p.director_pack.g_stopped": "Program dihentikan: {n}",
  "p.director_pack.c_gate": "Gerbang",
  "p.director_pack.c_decided": "Diputuskan",
  "p.director_pack.c_planned": "Direncanakan",
  "p.director_pack.c_reason": "Alasan",
  "p.director_pack.c_days_late": "Hari terlambat",
  "p.director_pack.c_since": "Sejak",
  "p.director_pack.c_allocs": "Alokasi ditandai",
  "p.director_pack.reason_hold": "ditinjau sebagai tahan",
  "p.director_pack.reason_no_review":
    "tanggal rencana terlewati tanpa tinjauan",
  "p.director_pack.fy_line":
    "Tahun keuangan {label}, {start} hingga {end}. Belanja modal dan operasional ditampilkan terpisah, dan mata uang tidak pernah dijumlahkan.",
  "p.director_pack.c_source": "Sumber pendanaan",
  "p.director_pack.c_class": "Kelas",
  "p.director_pack.c_budget": "Anggaran",
  "p.director_pack.c_spend": "Pengeluaran sampai saat ini",
  "p.director_pack.c_variance": "Selisih",
  "p.director_pack.no_funding_source": "tanpa sumber pendanaan",
  "p.director_pack.forecast_line":
    "Perkiraan seumur hidup dalam {currency}: aktual {actual}, perkiraan saat selesai {eac}",
  "p.director_pack.unfunded_lines":
    "Pos tanpa sumber pendanaan: {n}. Pos tanpa tanggal: {dated}.",
  "p.director_pack.c_chain": "Rantai selesai (%)",
  "p.director_pack.c_buffer": "Penyangga terpakai (%)",
  "p.director_pack.c_zone": "Zona",
  "p.director_pack.c_slip": "Penundaan (hari)",
  "p.director_pack.c_buffer_days": "Penyangga (hari)",
  "p.director_pack.c_previous": "Paket sebelumnya",
  "p.director_pack.edge_below": "pemulihan: digambar di tepi bawah",
  "p.director_pack.edge_above": "melebihi: digambar di tepi atas",
  "p.director_pack.fever_none_no_buffer":
    "tidak ada titik bagan demam: belum ada penyangga proyek yang dinyatakan",
  "p.director_pack.fever_none_no_baseline":
    "tidak ada titik bagan demam: tidak ada baseline jadwal",
  "p.director_pack.fever_none_no_chain":
    "tidak ada titik bagan demam: tidak ada pekerjaan kritis berdurasi untuk diukur",
  "p.director_pack.fever_none_no_forecast":
    "tidak ada titik bagan demam: jadwal tidak dapat diperkirakan sekarang",
  "p.director_pack.fever_lines":
    "Garis zona (penyangga terpakai % pada 0 % dan pada 100 % rantai selesai): hijau ke kuning {ga0} hingga {ga1}; kuning ke merah {ar0} hingga {ar1}.",
  "p.director_pack.c_bac": "Anggaran saat selesai",
  "p.director_pack.c_ev": "Nilai yang diperoleh",
  "p.director_pack.c_ac": "Biaya aktual",
  "p.director_pack.c_spi": "SPI",
  "p.director_pack.c_cpi": "CPI",
  "p.director_pack.below_threshold":
    "Di bawah ambang batas nilai yang diperoleh: anggaran saat selesai {bac}",
  "p.director_pack.no_baseline": "tanpa baseline",
  "p.director_pack.evm_ratio_note":
    "SPI dan CPI ditampilkan hingga dua angka desimal; kosong berarti tidak terdefinisi, bukan nol.",
  "p.director_pack.c_recurrence": "Perulangan",
  "p.director_pack.c_classification": "Klasifikasi",
  "p.director_pack.c_claimed": "Diklaim",
  "p.director_pack.c_validated": "Tervalidasi",
  "p.director_pack.c_unvalidated": "Diklaim, belum divalidasi",
  "p.director_pack.benefits_note":
    "Terwujud tetapi belum tervalidasi adalah klaim, bukan manfaat tervalidasi. Jumlah berulang bersifat tahunan dan tidak pernah dijumlahkan dengan jumlah sekali saja.",
  "p.director_pack.c_pool": "Kolam keterampilan",
  "p.director_pack.c_demand": "Permintaan (FTE)",
  "p.director_pack.c_supply": "Pasokan (FTE)",
  "p.director_pack.c_gap": "Kesenjangan (FTE)",
  "p.director_pack.c_month": "Bulan",
  "p.director_pack.c_reading": "Pembacaan",
  "p.director_pack.no_pool": "tidak ada kolam yang ditugaskan",
  "p.director_pack.capacity_note":
    "Setiap sel memiliki kata dan simbol selain warna. Tidak ada pasokan tercatat bukan berarti kapasitas nol.",
  "p.director_pack.partners_short": "pelaksanaan mitra di bawah komitmen",
  "p.director_pack.open_issues": "Isu terbuka: {n}",
  "p.director_pack.awaiting": "Permintaan perubahan menunggu keputusan: {n}",
  "p.director_pack.dash": "tidak dicatat",
  "l.enum.pk_conf.green": "hijau",
  "l.enum.pk_conf.amber_green": "kuning/hijau",
  "l.enum.pk_conf.amber": "kuning",
  "l.enum.pk_conf.amber_red": "kuning/merah",
  "l.enum.pk_conf.red": "merah",
  "l.enum.pk_conf.not_assessed": "belum dinilai",
  "l.enum.pk_trend.up": "membaik",
  "l.enum.pk_trend.down": "memburuk",
  "l.enum.pk_trend.same": "tidak berubah",
  "l.enum.pk_trend.new": "baru dinilai",
  "l.enum.pk_trend.not_assessed": "belum dinilai",
  "l.enum.pk_trend.no_earlier_pack": "tidak ada paket sebelumnya",
  "l.enum.pk_status.active": "aktif",
  "l.enum.pk_status.paused": "dijeda",
  "l.enum.pk_status.stopped": "dihentikan",
  "l.enum.pk_zone.green": "hijau",
  "l.enum.pk_zone.amber": "kuning",
  "l.enum.pk_zone.red": "merah",
  "l.enum.pk_heat.shortfall": "kekurangan",
  "l.enum.pk_heat.tight": "ketat",
  "l.enum.pk_heat.enough": "cukup",
  "l.enum.pk_heat.no_supply_recorded": "tidak ada pasokan tercatat",
  "l.enum.pk_heat.no_demand": "tidak ada permintaan",
  "l.enum.pk_heat.nothing_recorded": "tidak ada yang tercatat",
  "l.enum.pk_class.capex": "modal",
  "l.enum.pk_class.opex": "operasional",
  "l.enum.pk_class.all": "modal dan operasional digabung",
  "l.enum.pk_recur.recurring": "berulang",
  "l.enum.pk_recur.one_off": "sekali saja",
  "l.enum.pk_recur.not_stated": "tidak dinyatakan",
  "l.enum.pk_cls.cash_releasing": "melepas kas",
  "l.enum.pk_cls.non_cash_releasing": "tidak melepas kas",
  "l.enum.pk_cls.societal": "kemasyarakatan",
  "l.enum.pk_cls.not_stated": "tidak dinyatakan",
  "l.enum.pk_gate.g0_concept": "G0 konsep",
  "l.enum.pk_gate.g1_feasibility": "G1 kelayakan",
  "l.enum.pk_gate.g2_definition": "G2 definisi",
  "l.enum.pk_gate.g3_delivery": "G3 pelaksanaan",
  "l.enum.pk_gate.g4_launch": "G4 peluncuran",
  "l.enum.pk_gate.g5_benefits": "G5 manfaat",
  "p.director_pack.c_currency": "Mata uang",
  "p.critical_chain.chain_ppm": "Rantai kritis dan penyangga — PPM",
  "p.critical_chain.buffers_ppm": "Manajemen penyangga — PPM",
  "p.critical_chain.pipeline_ppm": "Drum dan saluran — PPM",
  "p.critical_chain.loading": "Memuat…",
  "p.critical_chain.yes": "ya",
  "p.critical_chain.no": "tidak",
  "p.critical_chain.zone_green": "hijau",
  "p.critical_chain.zone_amber": "kuning",
  "p.critical_chain.zone_red": "merah",
  "p.critical_chain.zone_none": "tidak ada pembacaan",
  "p.critical_chain.outcome_as_planned": "mempertahankan tanggalnya",
  "p.critical_chain.outcome_staggered": "mulai lebih lambat",
  "p.critical_chain.outcome_exceeds_drum":
    "meminta lebih dari yang dimiliki drum",
  "p.critical_chain.outcome_no_slot": "tidak ada awal yang cocok",
  "p.critical_chain.advice":
    "Ini adalah saran. Tidak ada yang diubah, dan seseorang yang memutuskan apa yang dilakukan.",
  "p.critical_chain.link_plan": "Rencana",
  "p.critical_chain.link_buffers": "Manajemen penyangga",
  "p.critical_chain.link_chain": "Rantai kritis",
  "p.critical_chain.link_critical_path": "Jalur kritis",
  "p.critical_chain.nav_pipeline": "Drum dan saluran",
  "p.critical_chain.chain_h1": "Rantai kritis dan penyangga",
  "p.critical_chain.chain_lede":
    "Rantai kritis adalah rangkaian tugas terpanjang setelah waktu orang diperhitungkan: mengikuti ketergantungan dan penantian terhadap orang yang sibuk pada tugas lain. Ditampilkan di samping jalur kritis, yang hanya mengikuti ketergantungan; keduanya tidak pernah dicampur.",
  "p.critical_chain.chain_h2": "Rantai kritis",
  "p.critical_chain.no_chain": "Tidak ada rantai kritis: {reason}",
  "p.critical_chain.reason_no_assignments":
    "tidak ada orang yang ditugaskan pada tugas yang belum selesai, sehingga tidak ada penantian terhadap orang untuk diikuti",
  "p.critical_chain.reason_no_scheduled_tasks":
    "tidak ada tugas yang memiliki jadwal",
  "p.critical_chain.chain_work": "Hari kerja pekerjaan pada rantai: {days}",
  "p.critical_chain.chain_tasks_count": "Tugas pada rantai: {tasks}",
  "p.critical_chain.chain_finish":
    "Setelah waktu orang diperhitungkan, rencana selesai pada {finish}.",
  "p.critical_chain.chain_delay_none":
    "Itu tidak lebih lambat dari penyelesaian jalur kritis.",
  "p.critical_chain.chain_delay":
    "Hari kerja lebih lambat dari penyelesaian jalur kritis: {days}",
  "p.critical_chain.crosses_yes":
    "Rantai berpindah antarjalur: ada orang yang dipakai bersama.",
  "p.critical_chain.crosses_no":
    "Rantai tetap pada satu jalur: tidak ada orang yang dipakai bersama.",
  "p.critical_chain.tied":
    "Rantai lain sama panjangnya; yang ini ditampilkan karena lebih dahulu dalam rencana.",
  "p.critical_chain.timeline_label": "Rantai pada lini masa",
  "p.critical_chain.timeline_caption":
    "Setiap batang penuh adalah tugas pada rantai. Batang putus-putus adalah penyangga proyek. Tabel di bawah memberikan angka yang sama.",
  "p.critical_chain.col_task": "Tugas",
  "p.critical_chain.col_days": "Hari kerja",
  "p.critical_chain.col_start": "Mulai",
  "p.critical_chain.col_finish": "Selesai",
  "p.critical_chain.col_held_by": "Ditahan oleh",
  "p.critical_chain.col_on_path": "Di jalur kritis",
  "p.critical_chain.held_first": "pertama pada rantai",
  "p.critical_chain.held_dep": "sebuah ketergantungan",
  "p.critical_chain.held_person": "seseorang: {person}",
  "p.critical_chain.held_person_hidden": "seseorang (disembunyikan)",
  "p.critical_chain.cp_h2": "Jalur kritis di sampingnya",
  "p.critical_chain.cp_finish": "Jalur kritis selesai pada {finish}.",
  "p.critical_chain.cp_none": "Tidak ada jalur kritis untuk ditampilkan.",
  "p.critical_chain.buffers_h2": "Penyangga",
  "p.critical_chain.buffers_lede":
    "Tugas dijadwalkan pada perkiraan agresifnya; pengaman yang diambil dari setiap tugas dikumpulkan menjadi penyangga. Ukuran dalam hari kerja. Aturannya: {rule}.",
  "p.critical_chain.rule_cut_and_paste":
    "separuh pengaman yang dihilangkan (potong dan tempel)",
  "p.critical_chain.rule_root_sum_squares":
    "akar dari jumlah kuadrat pengaman yang dihilangkan",
  "p.critical_chain.no_buffers":
    "Tidak ada penyangga untuk ditampilkan: {reason}",
  "p.critical_chain.reason_estimates_missing":
    "beberapa tugas pada rantai belum memiliki perkiraan agresif dan aman. Catat melalui layanan perkiraan; belum ada layar untuk itu.",
  "p.critical_chain.missing_h3": "Tugas pada rantai tanpa kedua perkiraan",
  "p.critical_chain.project_buffer":
    "Penyangga proyek: {days} hari kerja, melindungi rantai yang selesai pada {finish}. Dengan penyangga, tanggalnya {with}.",
  "p.critical_chain.project_safety":
    "Pengaman yang dihilangkan dari tugas rantai: {days} hari kerja.",
  "p.critical_chain.project_buffer_bar": "Penyangga proyek",
  "p.critical_chain.feeding_h3": "Penyangga pengumpan",
  "p.critical_chain.feeding_none":
    "Tidak ada rantai tugas lain yang bergabung ke rantai kritis.",
  "p.critical_chain.col_feeding_tasks": "Tugas pengumpan",
  "p.critical_chain.col_joins": "Bergabung ke",
  "p.critical_chain.col_buffer": "Penyangga (hari kerja)",
  "p.critical_chain.col_room": "Ruang sebelum bergabung (hari kerja)",
  "p.critical_chain.col_push": "Akan mendorong rantai",
  "p.critical_chain.push_yes": "ya, sebanyak {days} hari kerja",
  "p.critical_chain.unsized": "belum berukuran: perkiraan hilang",
  "p.critical_chain.resource_h3": "Penyangga sumber daya (peringatan kesiapan)",
  "p.critical_chain.resource_lede":
    "Penyangga sumber daya bukan waktu. Ini pengingat bahwa seseorang harus siap sebelum tugasnya pada rantai dimulai, {lead} hari kerja sebelumnya.",
  "p.critical_chain.col_person": "Orang",
  "p.critical_chain.col_alert_on": "Siap paling lambat",
  "p.critical_chain.col_task_starts": "Tugas dimulai",
  "p.critical_chain.hidden": "(disembunyikan)",
  "p.critical_chain.likely_h3":
    "Dijadwalkan pada durasi paling mungkin (tanpa perkiraan agresif)",
  "p.critical_chain.could_not_load_chain": "Rantai kritis tidak dapat dimuat",
  "p.critical_chain.buffers_h1": "Manajemen penyangga",
  "p.critical_chain.buffers_lede2":
    "Setiap penyangga dibaca terhadap jadwal terkini: seberapa banyak rantai selesai dan seberapa banyak penyangga terpakai oleh perkiraan penundaan. Penyangga di zona merah memerlukan tindakan pemulihan, yang disepakati dan dicatat oleh seseorang.",
  "p.critical_chain.no_plan":
    "Belum ada rencana penyangga yang dibekukan untuk rencana ini. Bekukan setelah rantai dan penyangga disepakati; pembacaan berikutnya diukur terhadapnya.",
  "p.critical_chain.freeze_confirm":
    "Saya sudah membaca rantai dan penyangga dan ingin membekukannya sebagai rencana",
  "p.critical_chain.freeze_button": "Bekukan rencana penyangga",
  "p.critical_chain.frozen_on":
    "Diukur terhadap rencana penyangga yang dibekukan pada {date}.",
  "p.critical_chain.alerts_h2": "Peringatan",
  "p.critical_chain.alerts_none": "Tidak ada penyangga di zona merah.",
  "p.critical_chain.alert_project":
    "Penyangga proyek di zona merah: {consumed}% terpakai. Rantai: {chain}.",
  "p.critical_chain.alert_feeding":
    "Penyangga pengumpan di zona merah: {consumed}% terpakai. Rantai: {chain}, bergabung ke {joins}.",
  "p.critical_chain.needs_action":
    "Tidak ada tindakan pemulihan terbuka yang dicatat untuk: {buffers}.",
  "p.critical_chain.charts_h2": "Bagan demam",
  "p.critical_chain.project_label": "Penyangga proyek",
  "p.critical_chain.feeding_label": "Penyangga pengumpan: {chain}",
  "p.critical_chain.axis_x": "Rantai selesai (%)",
  "p.critical_chain.axis_y": "Penyangga terpakai (%)",
  "p.critical_chain.chart_caption":
    "{buffer}: {zone}; {consumed}% penyangga terpakai; {done}% rantai selesai.",
  "p.critical_chain.beyond_edge":
    "Angka sebenarnya berada di luar bagan, sehingga titik digambar di tepi.",
  "p.critical_chain.table_h2": "Angka yang sama sebagai tabel",
  "p.critical_chain.col_size": "Penyangga (hari kerja)",
  "p.critical_chain.col_done": "Rantai selesai (%)",
  "p.critical_chain.col_used": "Penyangga terpakai (%)",
  "p.critical_chain.col_slip": "Perkiraan penundaan (hari kerja)",
  "p.critical_chain.col_zone": "Zona",
  "p.critical_chain.no_point": "Tidak ada pembacaan: {reason}",
  "p.critical_chain.actions_h2": "Tindakan pemulihan",
  "p.critical_chain.actions_none": "Belum ada tindakan pemulihan yang dicatat.",
  "p.critical_chain.col_buffer_name": "Penyangga",
  "p.critical_chain.col_action": "Tindakan",
  "p.critical_chain.col_owner": "Pemilik",
  "p.critical_chain.col_due": "Jatuh tempo",
  "p.critical_chain.col_status": "Status",
  "p.critical_chain.status_open": "terbuka",
  "p.critical_chain.status_done": "selesai",
  "p.critical_chain.status_dropped": "dibatalkan",
  "p.critical_chain.mark_done": "Tandai selesai",
  "p.critical_chain.drop_action": "Batalkan",
  "p.critical_chain.add_action_h3": "Catat tindakan pemulihan",
  "p.critical_chain.f_buffer": "Penyangga",
  "p.critical_chain.f_action": "Apa yang akan dilakukan",
  "p.critical_chain.f_owner": "Pemilik (rujukan orang)",
  "p.critical_chain.f_due": "Jatuh tempo (opsional)",
  "p.critical_chain.add_button": "Catat tindakan",
  "p.critical_chain.snap_h2": "Riwayat",
  "p.critical_chain.snap_button": "Catat rekaman sekarang",
  "p.critical_chain.snap_none": "Belum ada rekaman yang dicatat.",
  "p.critical_chain.col_snap_date": "Dicatat pada",
  "p.critical_chain.could_not_load_buffers":
    "Manajemen penyangga tidak dapat dimuat",
  "p.critical_chain.could_not_save": "Tidak dapat menyimpannya",
  "p.critical_chain.pipeline_h1": "Drum dan saluran",
  "p.critical_chain.pipeline_lede":
    "Drum adalah kolam keterampilan yang paling diandalkan portofolio. Proyek yang membutuhkannya pada waktu yang sama diberi jeda bertahap: layanan mengusulkan awal yang lebih lambat, dengan penyangga kapasitas antarproyek, dan menjelaskan masing-masing. Layanan tidak pernah memindahkan tanggal.",
  "p.critical_chain.drum_h2": "Drum",
  "p.critical_chain.drum_named":
    "Drumnya adalah {name}: ditetapkan oleh kantor portofolio.",
  "p.critical_chain.drum_overloaded":
    "Drumnya adalah {name}: tidak ada kolam yang ditetapkan, dan yang ini memiliki kekurangan terbesar.",
  "p.critical_chain.no_drum": "Tidak ada drum: {why}",
  "p.critical_chain.set_h3": "Tetapkan drum",
  "p.critical_chain.pick_pool": "Kolam keterampilan",
  "p.critical_chain.set_button": "Tetapkan sebagai drum",
  "p.critical_chain.clear_button": "Hapus penetapan",
  "p.critical_chain.capacity":
    "Kapasitas drum: {fte} FTE. Penyangga kapasitas antarproyek: {days} hari.",
  "p.critical_chain.no_capacity":
    "Drum tidak memiliki pasokan tercatat dalam rentang ini, sehingga tidak ada yang dapat dijadikan acuan penjedaan.",
  "p.critical_chain.projects_h2": "Awal yang diusulkan",
  "p.critical_chain.projects_none":
    "Tidak ada proyek yang membutuhkan drum dalam rentang ini.",
  "p.critical_chain.pipeline_timeline":
    "Pemakaian drum sekarang dan sesuai usulan",
  "p.critical_chain.pipeline_timeline_caption":
    "Untuk setiap proyek, batang atas adalah saat proyek memakai drum sekarang dan batang bawah adalah saat memakainya sesuai usulan. Tabel di bawah memberikan tanggal yang sama.",
  "p.critical_chain.bar_now": "sekarang",
  "p.critical_chain.bar_proposed": "diusulkan",
  "p.critical_chain.col_project": "Proyek",
  "p.critical_chain.col_needs": "Membutuhkan drum",
  "p.critical_chain.col_peak": "Puncak (FTE)",
  "p.critical_chain.col_proposal": "Usulan",
  "p.critical_chain.col_proposed": "Pemakaian pertama yang diusulkan",
  "p.critical_chain.col_delay": "Lebih lambat (hari)",
  "p.critical_chain.col_why": "Mengapa",
  "p.critical_chain.could_not_load_pipeline": "Saluran tidak dapat dimuat",
  "p.start_queue.nav": "Antrean mulai",
  "p.start_queue.title": "Antrean mulai — PPM",
  "p.start_queue.h1": "Antrean mulai",
  "p.start_queue.lede":
    "Program yang meminta untuk memulai pelaksanaan dan ditolak, menurut urutan permintaan. Program ditolak ketika batas pekerjaan dalam proses penuh atau ketika kolam kendala kekurangan pasokan pada bulan-bulan mendatang. Urutannya tidak pernah berubah, dan sebuah awal tetap dapat dilanjutkan dengan penimpaan yang dicatat beserta alasan.",
  "p.start_queue.loading": "Memuat antrean…",
  "p.start_queue.none": "Tidak ada program yang menunggu untuk dimulai.",
  "p.start_queue.col_pos": "Urutan",
  "p.start_queue.col_prog": "Program",
  "p.start_queue.col_asked": "Diminta pada",
  "p.start_queue.col_why": "Alasan ditolak",
  "p.start_queue.col_act": "Tindakan",
  "p.start_queue.limit":
    "Semua {limit} tempat terisi: {in_delivery} program sedang dilaksanakan.",
  "p.start_queue.pool":
    "Kolam kendala {pool} kekurangan pasokan pada: {months}.",
  "p.start_queue.withdraw": "Tarik {name} dari antrean",
  "p.start_queue.withdraw_short": "Tarik",
  "p.start_queue.could_not_load": "Antrean mulai tidak dapat dimuat",
  "p.start_queue.could_not_withdraw": "Tidak dapat menarik dari antrean",
  "p.table.search": "Saring baris",
  "p.table.export": "Ekspor",
  "p.table.find": "Cari",
  "p.table.moved": "{header} dipindahkan ke posisi {position} dari {count}",
  "p.table.copy": "Salin",
  "p.table.copied": "{rows} baris disalin ke papan klip",
  "p.table.copy_failed": "Tidak dapat menyalin ke papan klip",
  "p.table.matches": "{matches} dari {total} ditampilkan",
  "p.table.search_in": "Cari di",
  "p.table.all_columns": "Semua kolom",
  "p.table.pages": "Halaman",
  "p.table.export_csv": "CSV",
  "p.table.export_tsv": "TSV",
  "p.table.export_excel": "Excel",
  "p.table.exported": "{rows} baris diekspor ke {file}",
  "p.table.columns": "Kolom",
  "p.table.show_column": "Tampilkan {header}",
  "p.table.resize": "Ubah ukuran kolom {header}",
  "p.table.select_all": "Pilih semua baris",
  "p.table.select_col": "Pilih",
  "p.table.select_row": "Pilih {row}",
  "p.table.prev": "Halaman sebelumnya",
  "p.table.next": "Halaman berikutnya",
  "p.table.page_status": "Halaman {page} dari {pages} ({rows} baris)",
  "p.table.page_announce": "Halaman {page} dari {pages}",
  "p.table.sorted": "{header} diurutkan {direction}",
  "p.table.asc": "menaik",
  "p.table.desc": "menurun",
  "p.table.none": "tidak diurutkan",
  "p.table.filtered": "{matches} dari {total} baris cocok",
  "p.table.selected": "{n} baris dipilih",
  "p.table.empty": "Tidak ada yang ditampilkan.",
  "p.demo.tour_title": "Tur demonstrasi — PPM",
  "p.demo.tour_h1": "Tur demonstrasi",
  "p.demo.tour_lede":
    "Anda masuk sebagai Example Person. Semua yang ada di sini fiktif: Northgate Transport Authority, orang-orangnya, uangnya, dan tanggalnya dibuat untuk demonstrasi ini. Pilih fitur mana pun di bawah dan cobalah: Anda dapat mengubah hal-hal, dan tidak ada yang nyata.",
  "p.demo.tour_story_h2": "Ceritanya",
  "p.demo.tour_story":
    "Northgate Transport Authority sedang memodernisasi jaringan keretanya: persinyalan baru, stasiun tanpa tangga, dan depo digital. Sepuluh rencana, delapan orang, empat organisasi, dan beberapa hal yang sengaja dibuat bermasalah, agar layar memiliki sesuatu untuk ditampilkan.",
  "p.demo.tour_open": "Buka",
  "p.demo.tour_no_plan":
    "Rencana North Line tidak ditemukan, sehingga tautan ke halamannya disembunyikan. Jalankan seed lagi.",
  "p.demo.tour_toc": "Fitur",
  "p.demo.g_overview": "Portofolio sekilas",
  "p.demo.g_delivery": "Melaksanakan satu proyek (interlocking North Line)",
  "p.demo.g_risk": "Risiko, isu, dan perubahan",
  "p.demo.g_money": "Keuangan",
  "p.demo.g_people": "Orang dan kapasitas",
  "p.demo.g_strategy": "Strategi dan penerimaan",
  "p.demo.g_escalation": "Tata kelola dan eskalasi",
  "p.demo.g_chain": "Rantai kritis",
  "p.demo.g_tools": "Alat",
  "p.demo.f_kanban_t": "Papan kanban saya",
  "p.demo.f_kanban_d":
    "Tugas terbuka Anda sendiri, dalam kolom menurut status, dari setiap proyek. Pindahkan satu dengan mouse atau menunya; status tugas berubah untuk semua orang.",
  "p.demo.f_me_t": "Pekerjaan saya",
  "p.demo.f_me_d":
    "Tugas yang sama menurut tanggal jatuh tempo dalam zona waktu Anda sendiri: terlambat, hari ini, minggu ini, nanti.",
  "p.demo.f_plans_t": "Rencana",
  "p.demo.f_plans_d":
    "Sepuluh rencana fiktif: satu portofolio, tiga program, dan enam proyek untuk Northgate Transport Authority. Buka salah satunya.",
  "p.demo.f_dashboard_t": "Dasbor portofolio",
  "p.demo.f_dashboard_d":
    "Status menurut jenis dan tahap, dengan jumlah merah, kuning, dan hijau.",
  "p.demo.f_gantt_t": "Gantt",
  "p.demo.f_gantt_d": "Jadwal setiap proyek pada satu lini masa.",
  "p.demo.f_milestones_t": "Pencapaian",
  "p.demo.f_milestones_d":
    "Desain disetujui, peralatan dikirim, dan mulai beroperasi untuk setiap proyek.",
  "p.demo.f_calendar_t": "Kalender",
  "p.demo.f_calendar_d": "Pencapaian dan tanggal jatuh tempo menurut bulan.",
  "p.demo.f_board_t": "Papan tugas",
  "p.demo.f_board_d":
    "Delapan tugas dari survei hingga serah terima, dalam kolom menurut status. Satu terhambat oleh jaminan keselamatan.",
  "p.demo.f_critical_path_t": "Jalur kritis",
  "p.demo.f_critical_path_d":
    "Rantai tugas yang saling bergantung yang menentukan tanggal selesai.",
  "p.demo.f_earned_value_t": "Nilai yang diperoleh",
  "p.demo.f_earned_value_d":
    "Nilai terencana, nilai yang diperoleh, dan biaya, dengan indeks jadwal dan biaya.",
  "p.demo.f_schedule_health_t": "Kesehatan jadwal",
  "p.demo.f_schedule_health_d":
    "Pemeriksaan logika rencana: tautan hilang, tugas panjang, batasan.",
  "p.demo.f_wbs_t": "Rincian pekerjaan",
  "p.demo.f_wbs_d": "Proyek yang dipecah menjadi paket pekerjaan.",
  "p.demo.f_risk_t": "Registri risiko",
  "p.demo.f_risk_d":
    "Tiga risiko pada setiap proyek, dinilai menurut probabilitas dan dampak.",
  "p.demo.f_issues_t": "Isu",
  "p.demo.f_issues_d":
    "Jalur kabel yang terhambat, rig uji yang dipesan ganda, dan izin yang tertunda.",
  "p.demo.f_change_requests_t": "Permintaan perubahan",
  "p.demo.f_change_requests_d":
    "Permintaan memundurkan mulai beroperasi dua minggu, dengan dampak jadwal dan biayanya, menunggu direktur program.",
  "p.demo.f_joint_risk_t": "Risiko biaya dan jadwal bersama",
  "p.demo.f_joint_risk_d":
    "Apa yang dilakukan risiko terhadap tanggal selesai dan biaya, secara bersamaan.",
  "p.demo.f_financials_t": "Keuangan",
  "p.demo.f_financials_d":
    "Anggaran, pengeluaran aktual, dan selisih di seluruh portofolio.",
  "p.demo.f_cost_risk_t": "Risiko biaya",
  "p.demo.f_cost_risk_d": "Sebaran biaya yang mungkin, dengan P50 dan P80.",
  "p.demo.f_ledger_t": "Buku besar pendanaan",
  "p.demo.f_ledger_d":
    "Hibah dan cadangan, ditelusuri ke pos anggaran yang memakainya. Dana yang belum direncanakan pada pos ditampilkan sebagai tidak tertelusuri.",
  "p.demo.f_costs_t": "Biaya pada proyek",
  "p.demo.f_costs_d":
    "Biaya tenaga kerja dari tabel tarif, dan biaya lain menurut tugas.",
  "p.demo.f_resources_t": "Sumber daya",
  "p.demo.f_resources_d":
    "Delapan orang fiktif, peran mereka, dan pekerjaan yang dipesankan untuk mereka.",
  "p.demo.f_capacity_t": "Peta panas kapasitas",
  "p.demo.f_capacity_d":
    "Pasokan dan permintaan menurut kolam keterampilan dan bulan. Bulan merah menunjukkan di mana insinyur persinyalan kekurangan.",
  "p.demo.f_key_persons_t": "Orang kunci",
  "p.demo.f_key_persons_d":
    "Titik kegagalan tunggal: satu insinyur yang belum dapat digantikan siapa pun, dan satu peran dengan penerus yang siap.",
  "p.demo.f_objectives_t": "Sasaran",
  "p.demo.f_objectives_d":
    "Tiga sasaran, masing-masing terkait dengan rencana yang melayaninya.",
  "p.demo.f_okrs_t": "Tujuan dan hasil kunci",
  "p.demo.f_okrs_d": "Hasil kunci dengan pembaruan berkala dan keyakinan.",
  "p.demo.f_kpis_t": "Ukuran kunci",
  "p.demo.f_kpis_d":
    "Menit keterlambatan, kepuasan penumpang, dan pengeluaran terhadap rencana, dengan satu pembacaan setiap bulan.",
  "p.demo.f_proposals_t": "Penerimaan pekerjaan",
  "p.demo.f_proposals_d":
    "Lima usulan pada tahap berbeda, dari draf hingga disetujui dan ditolak.",
  "p.demo.f_ideas_t": "Gagasan",
  "p.demo.f_ideas_d":
    "Empat gagasan dengan suara. Ubah salah satunya menjadi usulan.",
  "p.demo.f_scenarios_t": "Skenario",
  "p.demo.f_scenarios_d":
    "Apa yang muat di bawah batas anggaran jika pekerjaan persinyalan didahulukan.",
  "p.demo.f_director_packs_t": "Paket direktur",
  "p.demo.f_director_packs_d":
    "Paket eksekutif dan dewan untuk September, disusun dari data langsung. Buka satu untuk melihat setiap bagian.",
  "p.demo.f_organizations_t": "Organisasi mitra",
  "p.demo.f_organizations_d":
    "Otoritas, firma pelaksananya, seorang mitra, dan seorang pemasok.",
  "p.demo.f_milestone_signoffs_t": "Pengesahan pencapaian",
  "p.demo.f_milestone_signoffs_d":
    "Mulai beroperasi memerlukan tanda tangan mitra dan pemasok. Pemasok sudah; mitra belum, sehingga mulai beroperasi diserahkan tetapi tidak terpenuhi.",
  "p.demo.f_cutover_t": "Go/no-go peralihan",
  "p.demo.f_cutover_d":
    "Tiga kriteria berhenti, satu terpenuhi, satu tanda tangan bersama masuk. Go akan ditolak, dan layar menjelaskan alasannya.",
  "p.demo.f_deployment_moves_t": "Pemindahan penerapan",
  "p.demo.f_deployment_moves_d":
    "Permintaan memundurkan awal pemasangan dua minggu, dengan dampak lanjutan pada tugas lain dan biaya tenaga kerjanya, menunggu persetujuan.",
  "p.demo.f_start_queue_t": "Antrean mulai",
  "p.demo.f_start_queue_d":
    "Dua program sedang dilaksanakan dan batasnya dua, sehingga yang ketiga menunggu dalam antrean, berurutan.",
  "p.demo.f_critical_chain_t": "Rantai kritis",
  "p.demo.f_critical_chain_d":
    "Rantai melalui tugas yang saling bergantung dan orang yang mereka bagi, dengan perkiraan agresif dan aman.",
  "p.demo.f_buffers_t": "Manajemen penyangga",
  "p.demo.f_buffers_d":
    "Rencana penyangga yang dibekukan, bagan demam, dan satu tindakan pemulihan.",
  "p.demo.f_pipeline_t": "Drum dan saluran",
  "p.demo.f_pipeline_d":
    "Kolam keterampilan yang paling diandalkan portofolio, dan usulan awal bertahap.",
  "p.demo.f_search_t": "Pencarian",
  "p.demo.f_search_d":
    "Ketik pada kaca pembesar di header. Tautan ini mencari “signal”.",
  "p.demo.f_email_deliveries_t": "Pengiriman email",
  "p.demo.f_email_deliveries_d":
    "Setiap email yang dicoba dikirim layanan. Tidak ada yang dikirim dari demonstrasi, sehingga daftar ini kosong.",
  "p.demo.f_display_t": "Pengaturan tampilan",
  "p.demo.f_display_d": "Ukuran teks, tema, tampilan sederhana, dan bahasa.",
  "p.kanban.nav": "Kanban saya",
  "p.kanban.title": "Papan kanban saya — PPM",
  "p.kanban.h1": "Papan kanban saya",
  "p.kanban.lede":
    "Tugas terbuka Anda di setiap rencana, dalam kolom menurut status. Pindahkan kartu dengan mouse, atau tekan Enter pada kartu untuk memilih kolom dari menu. Pemindahan mengubah status tugas untuk semua orang.",
  "p.kanban.board_label": "Tugas Anda menurut status",
  "p.kanban.loading": "Memuat papan Anda…",
  "p.kanban.none": "Anda tidak memiliki tugas terbuka.",
  "p.kanban.todo": "Akan dikerjakan",
  "p.kanban.in_progress": "Sedang dikerjakan",
  "p.kanban.in_review": "Dalam tinjauan",
  "p.kanban.blocked": "Terhambat",
  "p.kanban.done": "Selesai",
  "p.kanban.card_count": "{n} kartu",
  "p.kanban.move_button": "Pindahkan {title}",
  "p.kanban.move_menu": "Pindah ke kolom",
  "p.kanban.moved": "{title} dipindahkan ke {column}",
  "p.kanban.due": "jatuh tempo {date}",
  "p.kanban.points": "{n} poin",
  "p.kanban.sign_in": "Masuk untuk melihat papan Anda.",
  "p.kanban.could_not_load": "Papan Anda tidak dapat dimuat",
  "p.kanban.could_not_move":
    "Tugas tidak dapat dipindahkan; papan tetap seperti semula.",
  "p.kanban.hidden":
    "{n} tugas tidak ditampilkan karena rencananya tidak dapat Anda baca.",
  "p.dashboard.my_home": "Beranda Saya",
  "p.demo.banner":
    "Ini adalah demonstrasi dengan data fiktif. Tidak ada yang nyata di sini.",
  "p.demo.banner_tour": "Ikuti tur",
  "p.demo.signin_h2": "Demonstrasi",
  "p.demo.signin_lede":
    "Situs ini adalah demonstrasi. Masuk dengan example@example.com untuk melihat setiap fitur dengan data fiktif. Tidak ada email yang dikirim.",
  "p.demo.signin_button": "Masuk ke demonstrasi",
  "p.demo.signin_wrong": "Demonstrasi hanya menerima example@example.com.",
  "p.search.title": "Pencarian — PPM",
  "p.search.h1": "Cari rencana",
  "p.search.lede":
    "Rencana yang namanya, pengenalnya, kata kuncinya, atau tagnya cocok dengan pencarian Anda. Rencana yang tidak boleh Anda baca tidak didaftar.",
  "p.search.none_asked": "Ketik sesuatu di kotak pencarian pada header.",
  "p.search.searching": "Mencari…",
  "p.search.results": "{n} rencana cocok dengan “{q}”.",
  "p.search.no_results": "Tidak ada rencana yang cocok dengan “{q}”.",
  "p.search.could_not_search": "Tidak dapat mencari",
  "p.layout.link_picker": "Halaman",
  "p.layout.search_picker": "Cari rencana",
  "p.layout.search_input": "Kata pencarian",
  "p.layout.search_submit": "Cari",
  "p.layout.lp_home": "Beranda",
  "p.layout.lp_plans": "Rencana",
  "p.layout.lp_dashboard": "Dasbor",
  "p.layout.lp_milestones": "Pencapaian",
  "p.layout.lp_proposals": "Usulan",
  "p.cutover.link": "Go/no-go peralihan",
  "p.cutover.title": "Go/no-go peralihan — PPM",
  "p.cutover.h1": "Go/no-go peralihan",
  "p.cutover.lede":
    "Keputusan untuk mengalihkan sistem yang kritis bagi keselamatan. Kriteria berhenti ditulis dan diterbitkan lebih dahulu, lalu dibekukan: mengubahnya membuat versi baru. Go memerlukan setiap kriteria terpenuhi dan setiap tanda tangan bersama yang diwajibkan. No-go, atau penundaan demi keselamatan, adalah keputusan yang diambil berdasarkan kriteria; itu bukan keterlambatan.",
  "p.cutover.loading": "Memuat keputusan…",
  "p.cutover.none": "Belum ada keputusan peralihan yang dibuat drafnya.",
  "p.cutover.new_h2": "Buat draf keputusan peralihan",
  "p.cutover.new_title": "Judul",
  "p.cutover.new_criteria":
    "Kriteria berhenti, satu per baris (masing-masing 5 sampai 500 karakter)",
  "p.cutover.draft": "Buat draf keputusan",
  "p.cutover.version": "versi {n}",
  "p.cutover.status_draft": "draf",
  "p.cutover.status_published": "diterbitkan, kriteria dibekukan",
  "p.cutover.status_open": "terbuka untuk diputuskan",
  "p.cutover.status_decided": "diputuskan",
  "p.cutover.status_superseded": "digantikan",
  "p.cutover.outcome_go": "go",
  "p.cutover.outcome_no_go": "no-go",
  "p.cutover.outcome_delay_for_safety": "tunda demi keselamatan",
  "p.cutover.decided_line": "Diputuskan pada {on}: {outcome}. {why}",
  "p.cutover.criteria_h3": "Kriteria berhenti",
  "p.cutover.col_n": "No.",
  "p.cutover.col_criterion": "Kriteria",
  "p.cutover.col_met": "Terpenuhi",
  "p.cutover.col_evidence": "Bukti",
  "p.cutover.col_mark": "Tandai",
  "p.cutover.met": "terpenuhi",
  "p.cutover.not_met": "tidak terpenuhi",
  "p.cutover.evidence": "Bukti",
  "p.cutover.mark_met": "Tandai terpenuhi: {name}",
  "p.cutover.mark_unmet": "Tandai tidak terpenuhi: {name}",
  "p.cutover.mark_met_short": "Terpenuhi",
  "p.cutover.mark_unmet_short": "Tidak terpenuhi",
  "p.cutover.cosign_h3": "Tanda tangan bersama",
  "p.cutover.cosign_signed": "Sudah menandatangani: {roles}.",
  "p.cutover.cosign_missing": "Masih diperlukan: {roles}.",
  "p.cutover.cosign_stale":
    "Ditandatangani untuk bukti sebelumnya, sehingga perlu ditandatangani lagi: {roles}.",
  "p.cutover.cosign_none": "tidak ada",
  "p.cutover.cosign_role":
    "Peran untuk menandatangani (token Anda harus membawanya)",
  "p.cutover.cosign": "Tanda tangan bersama",
  "p.cutover.refused_h3": "Go akan ditolak karena",
  "p.cutover.publish": "Terbitkan dan bekukan kriteria",
  "p.cutover.open": "Buka keputusan",
  "p.cutover.new_version": "Mulai versi baru untuk mengubah kriteria",
  "p.cutover.decide_h3": "Putuskan",
  "p.cutover.outcome": "Hasil",
  "p.cutover.reason": "Alasan",
  "p.cutover.decide": "Catat keputusan",
  "p.cutover.could_not_load": "Keputusan tidak dapat dimuat",
  "p.cutover.could_not_save": "Tidak dapat menyimpan",
  "p.cutover.cancel_edit": "Berhenti mengubah",
  "p.cutover.criteria_frozen":
    "Keputusan ini sudah diterbitkan, sehingga kriterianya dibekukan: buat versi baru untuk mengubahnya.",
  "p.cutover.criteria_rules":
    "Satu kriteria berhenti per baris: 1 sampai 30, masing-masing 5 sampai 500 karakter. Dapat diubah sampai keputusan diterbitkan.",
  "p.cutover.criteria_saved": "Kriteria berhenti disimpan.",
  "p.cutover.criteria_unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.cutover.edit_criteria_label": "Kriteria berhenti, satu per baris",
  "p.cutover.edit_criteria": "Ubah kriteria berhenti",
  "p.cutover.problem_count":
    "Keputusan memerlukan 1 sampai 30 kriteria berhenti.",
  "p.cutover.problem_length":
    "Kriteria berhenti {line} harus 5 sampai 500 karakter.",
  "p.cutover.save_criteria": "Simpan kriteria berhenti",
  "p.moves.link": "Pemindahan penerapan",
  "p.moves.title": "Pemindahan penerapan — PPM",
  "p.moves.h1": "Pemindahan penerapan",
  "p.moves.lede":
    "Permintaan untuk memindahkan awal sebuah tugas. Tidak ada yang berubah sampai penyetuju menyetujuinya. Dampak lanjutan pada tugas lain, dan biaya tenaga kerjanya, dihitung saat pemindahan diminta, dengan menjalankan jadwal memakai tanggal yang dipindahkan.",
  "p.moves.loading": "Memuat pemindahan…",
  "p.moves.none": "Belum ada pemindahan yang diminta.",
  "p.moves.request_h2": "Minta pemindahan",
  "p.moves.task": "Tugas",
  "p.moves.new_start": "Awal baru",
  "p.moves.reason": "Alasan",
  "p.moves.approver": "Penyetuju (person:<id> atau role:<nama>, opsional)",
  "p.moves.request": "Minta pemindahan",
  "p.moves.col_task": "Tugas",
  "p.moves.col_dates": "Mulai",
  "p.moves.col_reason": "Alasan",
  "p.moves.col_by": "Diminta oleh",
  "p.moves.col_approver": "Penyetuju",
  "p.moves.col_status": "Status",
  "p.moves.col_knock": "Dampak lanjutan",
  "p.moves.col_act": "Putuskan",
  "p.moves.status_requested": "menunggu penyetuju",
  "p.moves.status_approved": "disetujui",
  "p.moves.status_refused": "ditolak",
  "p.moves.knock": "{n} tugas lain berpindah; biaya tenaga kerja {cost}",
  "p.moves.no_cost": "tidak ada perubahan biaya tenaga kerja",
  "p.moves.decision_reason": "Alasan keputusan",
  "p.moves.approve": "Setujui",
  "p.moves.refuse": "Tolak",
  "p.moves.approve_for": "Setujui pemindahan {name}",
  "p.moves.refuse_for": "Tolak pemindahan {name}",
  "p.moves.decided": "{by} pada {on}: {why}",
  "p.moves.by_requester_h2": "Biaya dampak lanjutan menurut peminta",
  "p.moves.requester_row": "{who}: {n} permintaan; tenaga kerja {cost}",
  "p.moves.could_not_load": "Pemindahan tidak dapat dimuat",
  "p.moves.could_not_save": "Tidak dapat menyimpan",
  "p.ledger.nav": "Buku besar pendanaan",
  "p.ledger.title": "Buku besar pendanaan — PPM",
  "p.ledger.h1": "Buku besar pendanaan",
  "p.ledger.lede":
    "Setiap alokasi pendanaan, ditelusuri ke pos anggaran yang memakainya, gerbangnya, pencapaian yang terpenuhi, dan manfaat yang divalidasi. Dana yang dialokasikan tetapi tidak direncanakan pada pos mana pun yang boleh Anda baca bersifat tidak tertelusuri, dan ditampilkan beserta jumlahnya.",
  "p.ledger.scope":
    "Membuat alokasi, memetakan kode biaya, dan merekonsiliasi ekspor keuangan dilakukan melalui API; layar ini membaca buku besar.",
  "p.ledger.loading": "Memuat buku besar…",
  "p.ledger.none": "Belum ada alokasi pendanaan yang dicatat.",
  "p.ledger.col_source": "Sumber pendanaan",
  "p.ledger.col_ref": "Rujukan",
  "p.ledger.col_on": "Dialokasikan pada",
  "p.ledger.col_amount": "Dialokasikan",
  "p.ledger.col_planned": "Direncanakan pada pos",
  "p.ledger.col_untraced": "Tidak tertelusuri",
  "p.ledger.col_lines": "Pos",
  "p.ledger.col_act": "Telusuri",
  "p.ledger.trace": "Telusuri {name}",
  "p.ledger.trace_short": "Telusuri",
  "p.ledger.trace_h2": "Apa yang dibeli {name}",
  "p.ledger.money":
    "Dialokasikan {allocated}, direncanakan pada pos {planned}, dibelanjakan {spent}, tidak tertelusuri {untraced} ({currency}).",
  "p.ledger.gates": "Pos dengan gerbang: {gated}. Dirilis: {released}.",
  "p.ledger.milestones_met": "Pencapaian terpenuhi: {met} dari {total}.",
  "p.ledger.benefits": "Manfaat tercatat: {n}.",
  "p.ledger.validated": "Tervalidasi dalam {currency}: {amount}",
  "p.ledger.lines_h3": "Pos anggaran",
  "p.ledger.line_plan": "Program",
  "p.ledger.line_desc": "Deskripsi",
  "p.ledger.line_planned": "Direncanakan",
  "p.ledger.line_actual": "Dibelanjakan",
  "p.ledger.line_gate": "Gerbang",
  "p.ledger.released": "dirilis",
  "p.ledger.held": "ditahan",
  "p.ledger.no_gate": "tanpa gerbang",
  "p.ledger.could_not_load": "Buku besar tidak dapat dimuat",
  "p.ledger.could_not_trace": "Alokasi tidak dapat ditelusuri",
  "p.ledger.a_line":
    "sebuah pos anggaran (pilih rencananya di atas untuk melihatnya)",
  "p.ledger.choose_line": "Pilih pos",
  "p.ledger.choose_plan_first": "Pilih rencana di atas terlebih dahulu",
  "p.ledger.choose_plan": "Pilih rencana",
  "p.ledger.choose_source": "Pilih sumber pendanaan",
  "p.ledger.code_mapped": "Kode biaya dipetakan.",
  "p.ledger.codes_heading": "Kode biaya",
  "p.ledger.codes_says":
    "Setiap kode biaya keuangan dipetakan ke satu pos anggaran, sehingga ekspor keuangan dapat disandingkan dengan registri. Memetakan kode yang sudah dipetakan memindahkannya.",
  "p.ledger.could_not_load_lines":
    "Pos anggaran rencana ini tidak dapat dimuat.",
  "p.ledger.could_not_reconcile": "Ekspor tidak dapat direkonsiliasi.",
  "p.ledger.could_not_record": "Alokasi tidak dicatat.",
  "p.ledger.could_not_save": "Perubahan tidak disimpan.",
  "p.ledger.f_allocation": "Alokasi yang dipakainya",
  "p.ledger.f_amount": "Jumlah",
  "p.ledger.f_code_line": "Pos anggaran (dari rencana di atas)",
  "p.ledger.f_code": "Kode biaya",
  "p.ledger.f_csv": "Atau tempel ekspor (CSV)",
  "p.ledger.f_currency": "Mata uang",
  "p.ledger.f_file": "Berkas ekspor keuangan (CSV)",
  "p.ledger.f_on": "Dialokasikan pada",
  "p.ledger.f_plan": "Rencana",
  "p.ledger.f_reference": "Rujukan (opsional)",
  "p.ledger.f_source": "Sumber pendanaan",
  "p.ledger.line_detached": "Pos tidak lagi memakai alokasi.",
  "p.ledger.line_drawn": "Pos kini memakai alokasi itu.",
  "p.ledger.lines_heading": "Pos anggaran dan alokasi yang dipakainya",
  "p.ledger.lines_says":
    "Pos memakai alokasi dari sumber pendanaan dan mata uangnya sendiri; tetapkan sumbernya di halaman Pendanaan terlebih dahulu.",
  "p.ledger.map_code": "Petakan kode biaya",
  "p.ledger.matched_heading": "Kode biaya yang cocok",
  "p.ledger.matched_line":
    "dalam ekspor {ledger}, dalam registri {register}, selisih {difference}",
  "p.ledger.mismatch_heading": "Baris dengan mata uang berbeda dari posnya",
  "p.ledger.mismatch_line":
    "baris dalam {row}, pos dalam {line}: {amount}, tidak dijumlahkan",
  "p.ledger.needs_source": "memerlukan sumber pendanaan terlebih dahulu",
  "p.ledger.no_allocation": "Tidak memakai alokasi",
  "p.ledger.no_codes": "Belum ada kode biaya yang dipetakan.",
  "p.ledger.no_lines": "Rencana ini tidak memiliki pos anggaran.",
  "p.ledger.no_matching_allocation":
    "Belum ada alokasi dari sumber dan mata uang pos ini",
  "p.ledger.no_source": "tanpa sumber pendanaan",
  "p.ledger.not_reconciled":
    "Tidak terekonsiliasi: ada yang tidak cocok. Tidak ada yang ditulis.",
  "p.ledger.problem_amount": "Tulis jumlah sebagai angka di atas 0.",
  "p.ledger.problem_code_line": "Pilih pos anggaran untuk kode tersebut.",
  "p.ledger.problem_code": "Kode biaya terdiri dari 1 sampai 60 karakter.",
  "p.ledger.problem_currency":
    "Tulis mata uang sebagai tiga huruf, seperti GBP.",
  "p.ledger.problem_date_future": "Tanggal alokasi tidak boleh di masa depan.",
  "p.ledger.problem_date": "Pilih tanggal alokasi.",
  "p.ledger.problem_export_empty":
    "Pilih berkas atau tempel ekspor terlebih dahulu.",
  "p.ledger.problem_export_header":
    "Baris pertama ekspor harus menyebut cost_code, amount_minor dan currency.",
  "p.ledger.problem_export_large": "Ekspor lebih dari 1 MB: pisahkan.",
  "p.ledger.problem_reference": "Rujukan paling banyak 120 karakter.",
  "p.ledger.problem_source": "Pilih sumber pendanaan.",
  "p.ledger.reconcile_heading": "Rekonsiliasi ekspor keuangan",
  "p.ledger.reconcile_says":
    "Kode biaya setiap baris dicari dalam peta dan jumlahnya dijumlahkan di samping apa yang dicatat registri sebagai terbelanjakan pada pos. Tidak ada yang ditulis.",
  "p.ledger.reconcile": "Rekonsiliasi",
  "p.ledger.reconciled":
    "Terekonsiliasi: setiap baris cocok dengan sebuah pos, dalam mata uangnya, tanpa selisih.",
  "p.ledger.record_heading": "Catat alokasi",
  "p.ledger.record_says":
    "Dana yang dialokasikan sebuah sumber pendanaan. Alokasi disimpan sebagaimana dicatat dan tidak pernah diubah.",
  "p.ledger.record": "Catat alokasi",
  "p.ledger.recorded": "Alokasi dicatat.",
  "p.ledger.refused_export":
    "Ekspor tidak dapat dibaca: setiap baris memerlukan kode biaya, jumlah bulat dalam satuan terkecil, dan mata uang, tidak melebihi yang diterima layanan.",
  "p.ledger.refused_line":
    "Pos tidak dapat memakai alokasi itu: keduanya harus memiliki sumber pendanaan dan mata uang yang sama.",
  "p.ledger.rows_read_one": "{n} baris dibaca.",
  "p.ledger.rows_read_other": "{n} baris dibaca.",
  "p.ledger.to_funding": "Tambah sumber pendanaan di halaman Pendanaan",
  "p.ledger.unmatched_heading":
    "Kode biaya yang tidak dipetakan ke pos yang dapat Anda baca",
  "p.signoffs.nav": "Pengesahan pencapaian",
  "p.signoffs.title": "Pengesahan pencapaian — PPM",
  "p.signoffs.h1": "Pengesahan pencapaian",
  "p.signoffs.lede":
    "Beberapa pencapaian hanya terpenuhi jika organisasi mitra yang disebutkan telah mengesahkannya. Diserahkan berarti organisasi pelaksana menyatakan sudah selesai; terpenuhi berarti diserahkan dengan setiap pengesahan berlaku. Pengesahan yang ditarik dihitung sebagai belum terpenuhi lagi.",
  "p.signoffs.loading": "Memuat pengesahan…",
  "p.signoffs.none": "Tidak ada pencapaian yang memerlukan pengesahan mitra.",
  "p.signoffs.totals":
    "Memerlukan pengesahan: {n}. Diserahkan: {delivered}. Terpenuhi: {met}. Diserahkan tetapi tidak terpenuhi: {gap}. Terpenuhi, sebagai bagian dari yang diserahkan: {share}.",
  "p.signoffs.no_share": "tidak diukur",
  "p.signoffs.col_milestone": "Pencapaian",
  "p.signoffs.col_due": "Jatuh tempo",
  "p.signoffs.col_state": "Keadaan",
  "p.signoffs.col_delivered": "Diserahkan pada",
  "p.signoffs.col_met": "Terpenuhi pada",
  "p.signoffs.col_outstanding": "Menunggu",
  "p.signoffs.col_act": "Tanda tangan atau tarik",
  "p.signoffs.state_met": "terpenuhi",
  "p.signoffs.state_delivered_not_met": "diserahkan, tidak terpenuhi",
  "p.signoffs.state_not_delivered": "belum diserahkan",
  "p.signoffs.organization": "Organisasi",
  "p.signoffs.reason": "Alasan (untuk penarikan)",
  "p.signoffs.sign": "Tanda tangan",
  "p.signoffs.withdraw": "Tarik",
  "p.signoffs.sign_for": "Tanda tangani {name} sebagai organisasi terpilih",
  "p.signoffs.withdraw_for": "Tarik pengesahan untuk {name}",
  "p.signoffs.not_built":
    "Organisasi mana yang diperlukan sebuah pencapaian dipilih di halaman tata kelola setiap rencana; layar ini menampilkan dan mencatat pengesahan.",
  "p.signoffs.problem_inactive":
    "{name} tidak lagi aktif, sehingga tidak dapat diwajibkan: hapus centangnya untuk menyimpan.",
  "p.signoffs.problem_too_many":
    "Sebuah pencapaian dapat mewajibkan paling banyak {max} organisasi.",
  "p.signoffs.required_cancel": "Batal",
  "p.signoffs.required_change_for":
    "Ubah siapa yang harus menandatangani {name}",
  "p.signoffs.required_change": "Ubah siapa yang harus menandatangani",
  "p.signoffs.required_cleared":
    "{name} tidak lagi memerlukan pengesahan mitra.",
  "p.signoffs.required_heading": "Pengesahan mitra",
  "p.signoffs.required_inactive": "{name} (tidak lagi aktif)",
  "p.signoffs.required_legend":
    "Organisasi yang harus mengesahkan {name} (paling banyak {max})",
  "p.signoffs.required_link": "Catat pengesahan",
  "p.signoffs.required_no_orgs": "Tidak ada organisasi aktif untuk dipilih.",
  "p.signoffs.required_none": "tidak perlu pengesahan mitra",
  "p.signoffs.required_save": "Simpan siapa yang harus menandatangani",
  "p.signoffs.required_saved":
    "Siapa yang harus menandatangani {name} disimpan.",
  "p.signoffs.required_says":
    "Pencapaian dengan pengesahan mitra hanya terpenuhi jika setiap organisasi di bawah telah menandatangani; diserahkan dan terpenuhi adalah tanggal yang terpisah.",
  "p.signoffs.required_signed": "{name} menandatangani pada {date}",
  "p.signoffs.required_unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.signoffs.required_waiting": "{name} belum menandatangani",
  "p.signoffs.could_not_load": "Pengesahan tidak dapat dimuat",
  "p.signoffs.could_not_save": "Pengesahan tidak dapat dicatat",
  "p.key_persons.nav": "Orang kunci",
  "p.key_persons.title": "Orang kunci — PPM",
  "p.key_persons.h1": "Orang kunci dan titik kegagalan tunggal",
  "p.key_persons.lede":
    "Orang dan peran yang tidak dapat hilang dari portofolio tanpa kerugian, siapa yang akan mengambil alih, dan seberapa siap mereka. Hanya manajer sumber daya yang dapat membaca ini, karena menyangkut orang bernama.",
  "p.key_persons.loading": "Memuat…",
  "p.key_persons.none": "Belum ada orang atau peran kunci yang dicatat.",
  "p.key_persons.not_allowed":
    "Hanya manajer sumber daya yang dapat membaca orang kunci.",
  "p.key_persons.add_h2": "Catat orang atau peran kunci",
  "p.key_persons.subject_kind": "Orang atau peran",
  "p.key_persons.kind_person": "orang",
  "p.key_persons.kind_role": "peran",
  "p.key_persons.subject": "Nama atau peran",
  "p.key_persons.successor": "Penerus",
  "p.key_persons.readiness": "Kesiapan penerus",
  "p.key_persons.ready_ready": "siap",
  "p.key_persons.ready_partly": "sebagian siap",
  "p.key_persons.ready_none": "belum siap",
  "p.key_persons.note": "Catatan",
  "p.key_persons.add": "Catat",
  "p.key_persons.col_subject": "Orang atau peran",
  "p.key_persons.col_successor": "Penerus",
  "p.key_persons.col_ready": "Kesiapan",
  "p.key_persons.col_note": "Catatan",
  "p.key_persons.col_act": "Tindakan",
  "p.key_persons.remove": "Hapus {name}",
  "p.key_persons.remove_short": "Hapus",
  "p.key_persons.no_successor": "belum ada penerus yang ditunjuk",
  "p.key_persons.contingent_h2": "Pekerja kontingen",
  "p.key_persons.contingent_note":
    "Orang yang ditandai sebagai tenaga kontingen, yang dapat pergi dengan pemberitahuan singkat.",
  "p.key_persons.contingent_none": "Tidak ada yang ditandai sebagai kontingen.",
  "p.key_persons.contingent_person": "Rujukan orang",
  "p.key_persons.contingent_unmark": "Hapus tanda {name}",
  "p.key_persons.mark": "Tandai",
  "p.key_persons.unmark": "Hapus tanda",
  "p.key_persons.could_not_load": "Orang kunci tidak dapat dimuat",
  "p.key_persons.could_not_save": "Tidak dapat menyimpan",
  "p.organizations.nav": "Organisasi",
  "p.organizations.title": "Organisasi — PPM",
  "p.organizations.h1": "Organisasi mitra",
  "p.organizations.lede":
    "Organisasi yang melaksanakan, mensponsori, memasok, atau bermitra pada portofolio. Paket dapat ditujukan kepada sebuah organisasi, pencapaian dapat memerlukan pengesahannya, dan kapasitas dapat dikomitmenkan olehnya. Mempensiunkan organisasi menyimpan riwayatnya.",
  "p.organizations.loading": "Memuat organisasi…",
  "p.organizations.none": "Belum ada organisasi yang ditambahkan.",
  "p.organizations.add_h2": "Tambah organisasi",
  "p.organizations.name": "Nama",
  "p.organizations.kind": "Jenis",
  "p.organizations.add": "Tambah",
  "p.organizations.col_name": "Nama",
  "p.organizations.col_kind": "Jenis",
  "p.organizations.col_state": "Keadaan",
  "p.organizations.col_audience": "Audiens paket",
  "p.organizations.col_act": "Tindakan",
  "p.organizations.kind_delivery": "pelaksana",
  "p.organizations.kind_partner": "mitra",
  "p.organizations.kind_sponsor": "sponsor",
  "p.organizations.kind_supplier": "pemasok",
  "p.organizations.active": "aktif",
  "p.organizations.retired": "dipensiunkan",
  "p.organizations.retire": "Pensiunkan {name}",
  "p.organizations.restore": "Pulihkan {name}",
  "p.organizations.retire_short": "Pensiunkan",
  "p.organizations.restore_short": "Pulihkan",
  "p.organizations.could_not_load": "Organisasi tidak dapat dimuat",
  "p.organizations.could_not_save": "Organisasi tidak dapat disimpan",
  "p.email_deliveries.nav": "Pengiriman email",
  "p.email_deliveries.title": "Pengiriman email — PPM",
  "p.email_deliveries.h1": "Pengiriman email",
  "p.email_deliveries.lede":
    "Setiap email yang dicoba dikirim layanan, satu baris untuk setiap penerima: apakah terkirim, menunggu dicoba lagi, atau gagal permanen, berapa kali percobaan, dan kesalahan terakhir. Hanya administrator yang dapat membaca ini, karena memuat alamat email.",
  "p.email_deliveries.loading": "Memuat pengiriman…",
  "p.email_deliveries.none": "Belum ada email yang dikirim atau dicoba.",
  "p.email_deliveries.counts":
    "Terkirim: {sent}. Menunggu dicoba lagi: {retry}. Gagal permanen: {failed}.",
  "p.email_deliveries.filter": "Tampilkan",
  "p.email_deliveries.all": "Semua",
  "p.email_deliveries.col_recipient": "Penerima",
  "p.email_deliveries.col_subject": "Subjek",
  "p.email_deliveries.col_status": "Status",
  "p.email_deliveries.col_attempts": "Percobaan",
  "p.email_deliveries.col_error": "Kesalahan terakhir",
  "p.email_deliveries.col_next": "Percobaan berikutnya",
  "p.email_deliveries.col_sent": "Dikirim pada",
  "p.email_deliveries.status_queued": "mengantre",
  "p.email_deliveries.status_retry": "akan dicoba lagi",
  "p.email_deliveries.status_sent": "terkirim",
  "p.email_deliveries.status_failed": "gagal",
  "p.email_deliveries.not_allowed":
    "Hanya administrator yang dapat membaca pengiriman email.",
  "p.email_deliveries.could_not_load": "Pengiriman email tidak dapat dimuat",
  "p.director_pack.s_signoffs": "Pengesahan pencapaian",
  "p.director_pack.signoffs_note":
    "Beberapa pencapaian hanya terpenuhi jika organisasi mitra yang disebutkan telah mengesahkannya. Diserahkan berarti organisasi pelaksana menyatakan sudah selesai; terpenuhi berarti diserahkan dengan setiap pengesahan berlaku. Pengesahan yang ditarik dihitung sebagai belum terpenuhi.",
  "p.director_pack.signoffs_totals":
    "Memerlukan pengesahan: {n}. Diserahkan: {delivered}. Terpenuhi: {met}. Diserahkan tetapi tidak terpenuhi: {gap}. Terpenuhi, sebagai bagian dari yang diserahkan: {share}.",
  "p.director_pack.so_met": "terpenuhi",
  "p.director_pack.so_delivered_not_met": "diserahkan, tidak terpenuhi",
  "p.director_pack.so_not_delivered": "belum diserahkan",
  "p.director_pack.so_no_share": "belum ada yang diserahkan",
  "p.director_pack.c_milestone": "Pencapaian",
  "p.director_pack.c_due": "Jatuh tempo",
  "p.director_pack.c_delivered_on": "Diserahkan pada",
  "p.director_pack.c_met_on": "Terpenuhi pada",
  "p.director_pack.c_outstanding": "Pengesahan belum diterima dari",
  "p.director_pack.c_delivery": "Pelaksanaan",
  "p.director_pack.dm_not_stated": "tidak dinyatakan",
  "p.director_pack.dm_central_product": "produk terpusat",
  "p.director_pack.dm_central_platform_local_deployment":
    "platform terpusat, penerapan lokal",
  "p.director_pack.dm_local_programme_central_standards":
    "program lokal, standar terpusat",
  "p.director_pack.dm_owner": "pemilik: {name}",
  "p.director_pack.dm_not_ours": "tidak dilaksanakan oleh organisasi",
  "p.director_pack.s_flow": "Burn-up dan alur (program produk)",
  "p.director_pack.c_scope": "Tugas dibuat",
  "p.director_pack.c_finished": "Tugas selesai",
  "p.director_pack.c_open": "Tugas terbuka",
  "p.director_pack.c_throughput": "Selesai per minggu (empat minggu terakhir)",
  "p.director_pack.c_burnup": "Selesai, minggu demi minggu (terlama dahulu)",
  "p.director_pack.no_flow":
    "{name}: program produk yang belum memiliki tugas, sehingga tidak ada burn-up atau alur.",
  "p.director_pack.s_asks": "Permintaan eksternal dan imbal baliknya",
  "p.director_pack.asks_note":
    "Permintaan dari sponsor tidak dapat disetujui sampai menyebutkan apa yang digantikannya. Ini mendaftar permintaan yang diajukan dalam periode.",
  "p.director_pack.asks_count":
    "Permintaan diajukan dalam periode: {n}. Belum ada imbal balik yang disebutkan: {without}.",
  "p.director_pack.ask_none": "belum ada imbal balik yang disebutkan",
  "p.director_pack.c_ask": "Permintaan",
  "p.director_pack.c_tradeoff": "Apa yang digantikannya",
  "p.director_pack.register_line":
    "Program dengan registri lengkap: {complete} dari {n}.",
  "p.director_pack.register_enforced":
    "Pos anggaran tidak dirilis, dan tidak ada alokasi tetap yang dibukukan, pada program dengan registri tidak lengkap.",
  "p.director_pack.register_not_enforced":
    "Registri tidak lengkap adalah peringatan, bukan penghalang: dana tetap dapat bergerak, dan peringatan diaudit.",
  "p.director_pack.s_register": "Program dengan registri tidak lengkap",
  "p.director_pack.c_score": "Lengkap",
  "p.director_pack.c_missing": "Hilang",
  "p.director_pack.rf_sro": "pemilik penanggung jawab senior",
  "p.director_pack.rf_partners": "mitra",
  "p.director_pack.rf_stage": "tahap",
  "p.director_pack.rf_baseline": "baseline jadwal",
  "p.director_pack.rf_target_date": "tanggal target",
  "p.director_pack.rf_budget": "anggaran",
  "p.director_pack.rf_funding_source": "sumber pendanaan",
  "p.director_pack.rf_confidence": "keyakinan pelaksanaan",
  "p.director_pack.rf_top_risks": "risiko teratas",
  "p.director_pack.rf_dependencies": "ketergantungan",
  "p.director_pack.s_watermelon": "Ukuran semangka",
  "p.director_pack.wm_note":
    "Hijau di luar, merah di dalam. Ini dibaca dari paket ini dan paket terbitan sebelumnya sebagaimana tersimpan, tidak dihitung ulang. Ukuran tanpa sesuatu untuk diukur menyebutkan alasannya; bukan nol.",
  "p.director_pack.wm_gap":
    "Keyakinan penilaian mandiri dan independen berbeda",
  "p.director_pack.wm_late":
    "Penemuan terlambat: merah atau kuning-merah sekarang, hijau pada paket sebelumnya",
  "p.director_pack.wm_red": "Merah tanpa keputusan tercatat dalam periode",
  "p.director_pack.wm_signoff":
    "Pencapaian terpenuhi dengan pengesahan, dari yang diserahkan",
  "p.director_pack.wm_signoff_of":
    "{met} terpenuhi dari {delivered} diserahkan",
  "p.director_pack.wm_not_measured": "tidak diukur",
  "p.director_pack.c_measure": "Ukuran",
  "p.director_pack.c_programmes": "Program, atau alasan tidak diukur",
  "p.director_pack.wm_count": "Program: {n}",
  "p.director_pack.s_keypeople": "Orang kunci dan tenaga kontingen",
  "p.director_pack.kp_note":
    "Titik kegagalan tunggal dihitung di sini dan tidak pernah disebutkan namanya; registri itu sendiri untuk manajer sumber daya.",
  "p.director_pack.kp_counts":
    "Titik kegagalan tunggal: {n} ({people} orang, {roles} peran). Tanpa penerus: {nosucc}. Penerus siap: {ready}; sebagian siap: {partly}; belum siap: {none}.",
  "p.director_pack.kp_no_people": "tidak ada orang di kolam ini",
  "p.director_pack.c_headcount": "Orang",
  "p.director_pack.c_contingent": "Kontingen",
  "p.director_pack.c_contingent_share": "Porsi kontingen",
  "p.director_pack.s_triage": "Triase dan ambang batas",
  "p.director_pack.tr_note":
    "Triase terbaru setiap program ditampilkan. Ambang batas yang terlampaui membuka pengecualian yang memerlukan jeda, atur ulang, hentikan, atau penimpaan dengan masa berlaku; tidak ada yang dijeda atau dihentikan secara otomatis.",
  "p.director_pack.tr_decisions":
    "Keputusan periode ini: jeda {pause}, atur ulang {reset}, hentikan {stop}, timpa {override}.",
  "p.director_pack.tr_open":
    "Pengecualian ambang batas menunggu keputusan: {n}.",
  "p.director_pack.tr_override_line":
    "Penimpaan berlaku untuk {programme} hingga {expiry}.",
  "p.director_pack.tr_finish": "selesaikan",
  "p.director_pack.tr_fix": "perbaiki",
  "p.director_pack.tr_pause": "jeda",
  "p.director_pack.tr_stop": "hentikan",
  "p.director_pack.c_outcome": "Triase",
  "p.director_pack.c_triaged_on": "Ditriase pada",
  "l.enum.pk_conf.threshold": "ambang batas terlampaui",
  "p.director_pack.s_cutovers": "Keputusan peralihan",
  "p.director_pack.co_note":
    "Peralihan diputuskan berdasarkan kriteria berhenti yang diterbitkan sebelum keputusan. No-go atau penundaan demi keselamatan adalah keputusan yang diambil berdasarkan kriteria tersebut, dan tidak dihitung sebagai keterlambatan.",
  "p.director_pack.co_counts":
    "Diputuskan dalam periode: go {go}, no-go {nogo}, tunda demi keselamatan {delay}.",
  "p.director_pack.co_go": "lanjut",
  "p.director_pack.co_no_go": "no-go",
  "p.director_pack.co_delay_for_safety": "tunda demi keselamatan",
  "p.director_pack.c_cutover": "Peralihan",
  "p.director_pack.c_decision": "Keputusan",
  "p.director_pack.c_decided_on": "Diputuskan pada",
  "p.director_pack.s_outcomes": "Hasil dan penurunan eskalasi",
  "p.director_pack.oc_note":
    "Tingkat eskalasi adalah yang ditetapkan sponsor, dan tidak pernah dihitung di sini. Rangkaian di bawah menghitung kuartal selesai yang setiap ukuran kartu skor dan setiap kriterianya terpenuhi; ini ditawarkan kepada orang yang memutuskan.",
  "p.director_pack.oc_level":
    "Tingkat eskalasi: {level}, ditetapkan pada {on}.",
  "p.director_pack.oc_no_level": "Belum ada tingkat eskalasi yang dicatat.",
  "p.director_pack.oc_run":
    "Kuartal berturut-turut yang terpenuhi: {n}. Aturan meminta {need}.",
  "p.director_pack.oc_rule_yes":
    "Aturan terpenuhi. Apakah turun satu tingkat adalah keputusan sponsor.",
  "p.director_pack.oc_rule_no": "Aturan belum terpenuhi.",
  "p.director_pack.oc_met": "terpenuhi",
  "p.director_pack.oc_not_met": "tidak terpenuhi",
  "p.director_pack.oc_missing": "hilang",
  "p.director_pack.c_quarter": "Kuartal",
  "p.director_pack.c_why": "Mengapa",
  "p.balance.balance": "Keseimbangan portofolio",
  "p.balance.balance_ppm": "Keseimbangan portofolio — PPM",
  "p.balance.intro":
    "Posisi biaya terencana, nilai manfaat target, dan risiko terbuka setiap portofolio terhadap yang lain, dan ketidakseimbangan yang timbul dari angka-angka itu.",
  "p.balance.loading": "Memuat…",
  "p.balance.could_not_load": "Keseimbangan portofolio tidak dapat dimuat",
  "p.balance.none":
    "Belum ada biaya, nilai manfaat, atau risiko yang dicatat terhadap portofolio.",
  "p.balance.currency": "Mata uang",
  "p.balance.unassigned": "Tanpa portofolio",
  "p.balance.caption":
    "Setiap gelembung adalah satu portofolio. Mendatar: biaya terencana dalam {currency}. Menaik: nilai manfaat target dalam {currency}. Luas: eksposur risiko terbuka.",
  "p.balance.axis_cost": "Biaya terencana ({currency})",
  "p.balance.axis_value": "Nilai manfaat target ({currency})",
  "p.balance.imbalances": "Ketidakseimbangan",
  "p.balance.no_imbalance":
    "Tidak ada aturan di bawah ini yang menandai ketidakseimbangan pada angka-angka ini.",
  "p.balance.rules":
    "Tanda diberikan ketika sebuah portofolio menguasai {concentration}% atau lebih dari biaya terencana atau risiko terbuka (dengan sedikitnya dua portofolio), ketika porsi biaya terencana atau risikonya lebih tinggi {gap} poin persentase atau lebih daripada porsi nilai targetnya, atau ketika portofolio memiliki biaya terencana tetapi tanpa nilai target dalam mata uang yang dipilih. Tanda hanyalah ajakan untuk memeriksa, bukan putusan.",
  "p.balance.flag_cost_concentration":
    "{name} menguasai {percent}% dari biaya terencana.",
  "p.balance.flag_risk_concentration":
    "{name} menguasai {percent}% dari eksposur risiko terbuka.",
  "p.balance.flag_cost_ahead_of_value":
    "{name} memiliki porsi biaya terencana yang jauh di atas porsi nilai targetnya sebesar {percent}%.",
  "p.balance.flag_risk_ahead_of_value":
    "{name} memiliki porsi risiko terbuka yang jauh di atas porsi nilai targetnya sebesar {percent}%.",
  "p.balance.flag_no_value":
    "{name} memiliki biaya terencana tetapi tidak ada nilai target yang dicatat dalam {currency}.",
  "p.balance.table": "Angka yang sama dalam bentuk tabel",
  "p.balance.portfolio": "Portofolio",
  "p.balance.cost": "Biaya terencana ({currency})",
  "p.balance.value": "Nilai target ({currency})",
  "p.balance.risk": "Eksposur risiko terbuka",
  "p.balance.cost_share": "Porsi biaya",
  "p.balance.value_share": "Porsi nilai",
  "p.balance.risk_share": "Porsi risiko",
  "p.balance.flags": "Tanda",
  "p.balance.no_flag": "tidak ada",
  "p.balance.notes":
    "Dikelompokkan menurut portofolio, bukan menurut kategori atau tujuan strategis: portofolio adalah satu-satunya pengelompokan yang menjadi dasar pelaporan biaya, nilai manfaat, dan risiko. Biaya adalah jumlah terencana pada baris anggaran, nilai adalah target dari manfaat yang dicatat, dan risiko adalah skor eksposur terbuka; uang dalam mata uang berbeda tidak pernah dijumlahkan.",
  "p.roadmap.title":
    "Peta jalan — dampak, upaya, dan Sekarang / Berikutnya / Nanti",
  "p.roadmap.h1":
    "Peta jalan: dampak, upaya, dan Sekarang / Berikutnya / Nanti",
  "p.roadmap.lede":
    "Gagasan dan usulan terbuka yang masih diputuskan, ditempatkan menurut seberapa penting (dampak) dan seberapa besar pekerjaan yang diperlukan (upaya). Layanan tidak menyimpan dampak atau upaya untuk gagasan atau usulan, jadi Anda menilai masing-masing di sini; penilaian Anda hanya disimpan di peramban ini dan tidak dibagikan atau dikirim ke layanan. Butir tanpa kedua penilaian tidak ditempatkan.",
  "p.roadmap.loading": "Memuat gagasan dan usulan…",
  "p.roadmap.matrix_h2": "Matriks dampak dan upaya",
  "p.roadmap.matrix_caption":
    "Dampak pada baris, upaya pada kolom. Setiap sel mencantumkan butir yang dinilai demikian.",
  "p.roadmap.impact": "Dampak",
  "p.roadmap.effort": "Upaya",
  "p.roadmap.impact_level": "Dampak {level}",
  "p.roadmap.effort_level": "Upaya {level}",
  "p.roadmap.level_low": "Rendah",
  "p.roadmap.level_medium": "Sedang",
  "p.roadmap.level_high": "Tinggi",
  "p.roadmap.rate_h2": "Nilai butir-butir",
  "p.roadmap.rate_hint":
    "Tabel ini adalah cara lewat papan ketik menuju matriks: pilih dampak dan upaya untuk setiap butir, lalu butir itu berpindah di matriks dan di daftar Sekarang / Berikutnya / Nanti.",
  "p.roadmap.col_item": "Butir",
  "p.roadmap.col_type": "Jenis",
  "p.roadmap.col_horizon": "Kapan",
  "p.roadmap.kind_idea": "Gagasan",
  "p.roadmap.kind_proposal": "Usulan",
  "p.roadmap.not_rated": "Belum dinilai",
  "p.roadmap.rate_impact": "Dampak {title}",
  "p.roadmap.rate_effort": "Upaya {title}",
  "p.roadmap.no_items": "Tidak ada gagasan atau usulan terbuka untuk dinilai.",
  "p.roadmap.horizons_h2": "Sekarang / Berikutnya / Nanti",
  "p.roadmap.horizon_rule":
    "Sekarang: dampak lebih tinggi daripada upaya. Berikutnya: dampak sama dengan upaya dan setidaknya sedang. Nanti: semua yang lain yang sudah dinilai. Belum dinilai: dampak atau upaya belum dipilih.",
  "p.roadmap.h_now": "Sekarang",
  "p.roadmap.h_next": "Berikutnya",
  "p.roadmap.h_later": "Nanti",
  "p.roadmap.h_unrated": "Belum dinilai",
  "p.roadmap.nothing_here": "Tidak ada apa-apa di sini.",
  "p.roadmap.see_ideas": "Lihat gagasan dan usulan pada peta jalan",
  "p.people_impact.title": "Dampak pada orang (dari layanan tenaga kerja)",
  "p.people_impact.intro":
    "Hanya jumlah, dari layanan tenaga kerja (WPM), diminta dengan login Anda sendiri. Tidak ada orang yang disebutkan namanya dan tidak ada yang dijumlahkan.",
  "p.people_impact.ask": "Tanya layanan tenaga kerja",
  "p.people_impact.asking": "Bertanya…",
  "p.people_impact.failed": "Dampak pada orang tidak dapat dimuat.",
  "p.people_impact.not_known": "Tidak diketahui.",
  "p.people_impact.source": "Dari layanan tenaga kerja, per {date}.",
  "p.people_impact.unknown": "Tidak diketahui: {reason}",
  "p.people_impact.end_sent":
    "Tanggal akhir dikirim: {date} (tanggal target rencana ini).",
  "p.people_impact.committed": "Orang yang dikomitmenkan, menurut kolam",
  "p.people_impact.ending":
    "Keterlibatan berjangka tetap dan kontrak yang berakhir sebelum akhir",
  "p.people_impact.in_no_pool": "Tanpa kolam",
  "p.people_impact.end_not_recorded":
    "Akhir tidak dicatat (tidak dihitung sebagai berakhir)",
  "p.people_impact.hidden": "disembunyikan (kurang dari {floor})",
  "p.people_impact.no_pools": "Tidak ada kolam untuk ditampilkan.",
  "p.people_impact.part_unknown": "Tidak diketahui: {reason}",
  "p.people_impact.over": "Bulan-kolam yang melebihi komitmen",
  "p.people_impact.over_counts":
    "{over} dari {considered} bulan-kolam melebihi komitmen; {unknown} tidak dapat ditentukan.",
  "p.people_impact.no_total":
    "Tidak ada total yang diberikan: satu orang dapat dihitung di lebih dari satu kolam.",
  "l.enum.pk_heat.supply_undeclared": "ketersediaan tidak dinyatakan",
  "l.enum.pk_heat.supply_unknown": "pasokan tidak diketahui",
  "p.capacity_heatmap.nav": "Peta panas kapasitas",
  "p.capacity_heatmap.title": "Peta panas kapasitas — PPM",
  "p.capacity_heatmap.h1": "Kapasitas menurut kolam keterampilan dan bulan",
  "p.capacity_heatmap.lede":
    "Setiap kotak membandingkan pekerjaan yang diminta dari sebuah kolam keterampilan dalam satu bulan dengan orang yang tersedia untuknya, dalam ekuivalen waktu penuh (FTE).",
  "p.capacity_heatmap.bridge_note":
    "Kolam yang ditautkan ke layanan tenaga kerja mengambil pasokannya dari sana. Apa yang dimasukkan di sini untuk kolam itu ditampilkan di sampingnya dan tidak pernah ditambahkan padanya. Bulan tanpa angka dari layanan tenaga kerja ditampilkan sebagai tidak diketahui, bukan nol.",
  "p.capacity_heatmap.from": "Bulan pertama",
  "p.capacity_heatmap.months": "Jumlah bulan",
  "p.capacity_heatmap.show": "Tampilkan",
  "p.capacity_heatmap.loading": "Memuat peta panas…",
  "p.capacity_heatmap.could_not_load":
    "Peta panas kapasitas tidak dapat dimuat.",
  "p.capacity_heatmap.none":
    "Tidak ada kolam keterampilan dan tidak ada permintaan untuk ditampilkan.",
  "p.capacity_heatmap.caption":
    "Permintaan dan pasokan dalam ekuivalen waktu penuh, menurut kolam keterampilan dan bulan",
  "p.capacity_heatmap.col_pool": "Kolam keterampilan",
  "p.capacity_heatmap.linked_to": "Ditautkan ke layanan tenaga kerja",
  "p.capacity_heatmap.demand": "Permintaan: {fte}",
  "p.capacity_heatmap.supply": "Pasokan: {fte}",
  "p.capacity_heatmap.gap": "Selisih: {fte}",
  "p.capacity_heatmap.unknown": "tidak diketahui",
  "p.capacity_heatmap.not_declared": "tidak dinyatakan",
  "p.capacity_heatmap.none_recorded": "tidak ada yang dicatat",
  "p.capacity_heatmap.source_wpm": "Pasokan dari layanan tenaga kerja",
  "p.capacity_heatmap.source_here": "Pasokan dimasukkan di sini",
  "p.capacity_heatmap.peer_live": "Langsung, diambil {when}.",
  "p.capacity_heatmap.peer_live_no_time": "Langsung.",
  "p.capacity_heatmap.peer_last":
    "Terakhir diterima {age} (diambil {when}); tidak langsung sekarang: {reason}",
  "p.capacity_heatmap.peer_hidden":
    "Disembunyikan: layanan tenaga kerja menyembunyikan bulan yang terdiri dari pekerja lebih sedikit daripada batas minimumnya.",
  "p.capacity_heatmap.peer_unknown": "Tidak diketahui: {reason}",
  "p.capacity_heatmap.no_reason":
    "tidak ada angka yang diterima dari layanan tenaga kerja",
  "p.capacity_heatmap.entered_here":
    "Dimasukkan di sini: {fte} (ditampilkan di samping, tidak ditambahkan)",
  "p.capacity_heatmap.entered_here_none": "Dimasukkan di sini: tidak ada",
  "p.capacity_heatmap.partners_short":
    "mitra menyerahkan lebih sedikit daripada yang dikomitmenkan",
  "p.capacity_heatmap.no_pool": "Tidak ada kolam yang ditetapkan",
  "p.capacity_heatmap.key":
    "▲ kekurangan · ◆ ketat · ● cukup · ○ tanpa permintaan · – tanpa pasokan tercatat · ? pasokan tidak diketahui. Kata-katanya selalu ditulis juga.",
  "p.capacity_heatmap.every_figure": "Setiap angka dalam bentuk tabel",
  "p.capacity_heatmap.table_label":
    "Kapasitas menurut kolam keterampilan dan bulan",
  "p.capacity_heatmap.c_pool": "Kolam keterampilan",
  "p.capacity_heatmap.c_month": "Bulan",
  "p.capacity_heatmap.c_reading": "Pembacaan",
  "p.capacity_heatmap.c_demand": "Permintaan (ekuivalen waktu penuh)",
  "p.capacity_heatmap.c_supply": "Pasokan (ekuivalen waktu penuh)",
  "p.capacity_heatmap.c_source": "Pasokan dari",
  "p.capacity_heatmap.c_status": "Pembacaan layanan tenaga kerja",
  "p.capacity_heatmap.c_entered_here": "Dimasukkan di sini",
  "p.capacity_heatmap.see_pools":
    "Tautkan kolam keterampilan ke layanan tenaga kerja",
  "p.skill_pools.nav": "Kolam keterampilan",
  "p.skill_pools.title": "Kolam keterampilan — PPM",
  "p.skill_pools.h1": "Kolam keterampilan",
  "p.skill_pools.lede":
    "Kolam keterampilan mengelompokkan peran dan keterampilan yang dapat mengerjakan pekerjaan yang sama. Di sini Anda dapat menautkan sebuah kolam ke kolam yang sesuai di layanan tenaga kerja.",
  "p.skill_pools.rule":
    "Tautan harus ditetapkan di kedua sisi, di sini dan di layanan tenaga kerja, oleh orang yang dapat melihat keduanya. Tautan tidak pernah dicocokkan berdasarkan nama. Selama kolam ditautkan, pasokannya pada peta panas kapasitas berasal dari layanan tenaga kerja dan permintaannya dikirim ke sana.",
  "p.skill_pools.loading": "Memuat kolam keterampilan…",
  "p.skill_pools.could_not_load": "Kolam keterampilan tidak dapat dimuat.",
  "p.skill_pools.none": "Belum ada kolam keterampilan yang dibuat.",
  "p.skill_pools.col_name": "Nama",
  "p.skill_pools.col_roles": "Peran",
  "p.skill_pools.col_skills": "Keterampilan",
  "p.skill_pools.col_state": "Status",
  "p.skill_pools.col_link": "Kolam layanan tenaga kerja",
  "p.skill_pools.col_act": "Ubah",
  "p.skill_pools.active": "aktif",
  "p.skill_pools.retired": "dipensiunkan",
  "p.skill_pools.not_linked": "tidak ditautkan",
  "p.skill_pools.change_short": "Ubah tautan",
  "p.skill_pools.change": "Ubah tautan untuk {name}",
  "p.skill_pools.edit_h2": "Tautkan {name} ke layanan tenaga kerja",
  "p.skill_pools.ref_label": "Id kolam layanan tenaga kerja",
  "p.skill_pools.ref_hint":
    "Salin dari kolam di layanan tenaga kerja: 36 karakter, huruf kecil, dengan tanda hubung, seperti 3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b.",
  "p.skill_pools.ref_invalid":
    "Itu bukan id kolam layanan tenaga kerja: harus 36 karakter, huruf kecil, dengan tanda hubung.",
  "p.skill_pools.save": "Simpan tautan",
  "p.skill_pools.clear": "Hapus tautan",
  "p.skill_pools.cancel": "Batal",
  "p.skill_pools.confirm_clear":
    "Hapus tautan antara {name} dan layanan tenaga kerja? Pasokannya pada peta panas akan kembali menjadi apa yang dimasukkan di sini. Setiap angka permintaan yang sudah dikirim ke layanan tenaga kerja untuk kolam ini akan ditarik di sana (dikirim sebagai nol).",
  "p.skill_pools.saved":
    "{name} sekarang ditautkan ke kolam layanan tenaga kerja {ref}.",
  "p.skill_pools.cleared":
    "{name} tidak lagi ditautkan ke layanan tenaga kerja.",
  "p.skill_pools.taken":
    "Kolam lain di sini sudah ditautkan ke kolam layanan tenaga kerja itu. Hapus tautan tersebut lebih dahulu: satu kolam di sini untuk satu kolam di sana.",
  "p.skill_pools.invalid":
    "Layanan tidak menerima id itu. Id harus berupa id kolam layanan tenaga kerja: 36 karakter, huruf kecil, dengan tanda hubung.",
  "p.skill_pools.missing":
    "Kolam itu sudah tidak ada. Daftar telah dimuat ulang.",
  "p.skill_pools.could_not_save": "Tautan tidak dapat diubah.",
  "p.skill_pools.see_heatmap": "Lihat peta panas kapasitas",
  "p.skill_pools.withdrawing":
    "Angka permintaan sedang ditarik dari kolam layanan tenaga kerja yang tadinya ditautkan: {n}.",
  "p.skill_pools.add_heading": "Tambah kolam",
  "p.skill_pools.add_says":
    "Kolam mengelompokkan peran sumber daya dan keterampilan, sehingga peta panas kapasitas menghitung permintaan dan pasokannya satu kali. Sebuah peran atau keterampilan berada di paling banyak satu kolam.",
  "p.skill_pools.add": "Tambah kolam",
  "p.skill_pools.added": "Kolam {name} ditambahkan.",
  "p.skill_pools.choose_pool": "Pilih kolam",
  "p.skill_pools.clash_item": "{value} (di {pool})",
  "p.skill_pools.conflict_members":
    "Kolam dengan nama itu sudah ada, atau salah satu peran atau keterampilan ini sudah ada di kolam lain.",
  "p.skill_pools.could_not_add": "Kolam tidak ditambahkan.",
  "p.skill_pools.f_name": "Nama kolam",
  "p.skill_pools.f_pool": "Kolam",
  "p.skill_pools.f_roles": "Peran",
  "p.skill_pools.f_skills": "Keterampilan",
  "p.skill_pools.list_says":
    "Pisahkan peran atau keterampilan dengan koma atau baris baru; pengulangan hanya disimpan sekali.",
  "p.skill_pools.members_heading": "Ubah kolam",
  "p.skill_pools.members_saved":
    "Peran dan keterampilan {name} telah disimpan.",
  "p.skill_pools.problem_clash": "Sudah ada di kolam lain: {list}.",
  "p.skill_pools.problem_name_taken": "Kolam dengan nama itu sudah ada.",
  "p.skill_pools.problem_name":
    "Nama kolam terdiri dari 2 sampai 120 karakter.",
  "p.skill_pools.restore": "Pulihkan kolam",
  "p.skill_pools.restored": "Kolam {name} dipulihkan.",
  "p.skill_pools.retire_says":
    "Kolam yang dipensiunkan tetap ada dalam catatan dan mempertahankan peran serta keterampilannya; memulihkannya berarti dihitung kembali.",
  "p.skill_pools.retire": "Pensiunkan kolam",
  "p.skill_pools.retired_done": "Kolam {name} dipensiunkan.",
  "p.skill_pools.save_members": "Simpan peran dan keterampilan",
  "p.skill_pools.unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.capacity_heatmap.set_commitment": "Tetapkan komitmen kapasitas",
  "p.commitments.title": "Komitmen kapasitas — PPM",
  "p.commitments.h1": "Komitmen kapasitas",
  "p.commitments.lede":
    "Catat kapasitas yang dijanjikan kepada sebuah program (pasokan) atau pekerjaan yang dibutuhkan program dari sebuah kolam keterampilan (permintaan), untuk satu bulan, dalam ekuivalen waktu penuh.",
  "p.commitments.bridge_note":
    "Untuk kolam yang ditautkan ke layanan tenaga kerja, komitmen permintaan juga mengirim total permintaan program untuk kolam dan bulan itu ke sana. Total nol menarik angka tersebut di sana. Tidak ada yang dicoba ulang: simpan lagi untuk mengirim lagi.",
  "p.commitments.see_heatmap": "Lihat peta panas kapasitas",
  "p.commitments.from_plan": "Komitmen kapasitas untuk program ini",
  "p.commitments.form_h2": "Tetapkan komitmen",
  "p.commitments.choose": "Pilih…",
  "p.commitments.pool": "Kolam keterampilan",
  "p.commitments.pool_linked": "{name} (ditautkan ke layanan tenaga kerja)",
  "p.commitments.pool_hint": "Hanya kolam aktif yang tercantum.",
  "p.commitments.pool_missing": "Pilih kolam keterampilan.",
  "p.commitments.plan": "Program",
  "p.commitments.plan_hint":
    "Rencana harus berupa program (rencana berjenis Program).",
  "p.commitments.plan_missing": "Pilih program.",
  "p.commitments.kind": "Apa yang dikomitmenkan",
  "p.commitments.kind_demand":
    "Permintaan: pekerjaan yang dibutuhkan program dari kolam",
  "p.commitments.kind_supply":
    "Pasokan: kapasitas yang dijanjikan kepada program",
  "p.commitments.organization": "Organisasi mitra (opsional)",
  "p.commitments.organization_hint":
    "Biarkan kosong untuk organisasi Anda sendiri.",
  "p.commitments.month": "Bulan",
  "p.commitments.month_hint": "Satu bulan: tahun dan bulan.",
  "p.commitments.month_invalid": "Berikan bulan sebagai tahun dan bulan.",
  "p.commitments.committed": "Dikomitmenkan (ekuivalen waktu penuh)",
  "p.commitments.committed_hint":
    "Angka dari 0 sampai 1000, ditulis seperti Anda menulis angka. Nol menarik angka permintaan yang dikirim ke layanan tenaga kerja.",
  "p.commitments.delivered": "Diserahkan (ekuivalen waktu penuh, opsional)",
  "p.commitments.delivered_hint":
    "Apa yang sebenarnya diserahkan, jika diketahui.",
  "p.commitments.fte_invalid":
    "Angka dari 0 sampai 1000, ditulis seperti Anda menulis angka.",
  "p.commitments.note": "Catatan (opsional)",
  "p.commitments.save": "Simpan komitmen",
  "p.commitments.saved": "Tersimpan: {pool}, {plan}, {month}.",
  "p.commitments.not_found":
    "Program itu tidak ditemukan, atau Anda tidak dapat membacanya.",
  "p.commitments.refused":
    "Layanan tidak menerimanya. Rencana harus berupa program, kolam harus yang aktif, dan setiap angka dari 0 sampai 1000.",
  "p.commitments.could_not_save": "Komitmen tidak dapat disimpan.",
  "p.commitments.could_not_load": "Komitmen kapasitas tidak dapat dimuat.",
  "p.commitments.loading": "Memuat pengiriman terbaru…",
  "p.commitments.sends_h2": "Pengiriman terbaru ke layanan tenaga kerja",
  "p.commitments.sends_note":
    "Pengiriman terakhir untuk setiap program, kolam, dan bulan, terbaru lebih dahulu. Pengiriman yang sudah dimulai menampilkan jawabannya di sini setelah tiba: segarkan untuk melihatnya.",
  "p.commitments.refresh": "Segarkan",
  "p.commitments.no_sends": "Belum ada yang dikirim ke layanan tenaga kerja.",
  "p.commitments.c_pool": "Kolam keterampilan",
  "p.commitments.c_programme": "Program",
  "p.commitments.c_month": "Bulan",
  "p.commitments.c_sent": "Dikirim",
  "p.commitments.c_outcome": "Apa yang terjadi",
  "p.commitments.c_at": "Kapan",
  "p.commitments.sent_fte": "{fte} ekuivalen waktu penuh",
  "p.commitments.sent_withdrawal": "penarikan (nol)",
  "p.commitments.sent_nothing": "tidak ada",
  "p.commitments.out_not_sent_at_all":
    "Komitmen ini tidak dikirim ke layanan tenaga kerja: hanya permintaan pada kolam yang ditautkan ke sana yang dikirim.",
  "p.commitments.out_none_yet":
    "Belum ada yang dikirim ke layanan tenaga kerja untuk program, kolam, dan bulan ini.",
  "p.commitments.out_started":
    "Mengirim {fte} ekuivalen waktu penuh (total program untuk kolam dan bulan ini) ke layanan tenaga kerja. Jawabannya muncul di bawah Pengiriman terbaru.",
  "p.commitments.out_started_zero":
    "Menarik angka program ini untuk kolam dan bulan ini dari layanan tenaga kerja, karena totalnya sekarang nol. Jawabannya muncul di bawah Pengiriman terbaru.",
  "p.commitments.out_sent":
    "Layanan tenaga kerja menerima {fte} ekuivalen waktu penuh.",
  "p.commitments.out_sent_zero":
    "Layanan tenaga kerja menarik angkanya untuk program, kolam, dan bulan ini.",
  "p.commitments.out_duplicate":
    "Layanan tenaga kerja sudah memiliki angka ini; tidak ada yang berubah di sana.",
  "p.commitments.out_refused":
    "Layanan tenaga kerja menolaknya. Tidak ada yang dicoba ulang; simpan lagi untuk mengirim lagi.",
  "p.commitments.out_unreachable":
    "Layanan tenaga kerja tidak dapat dihubungi, atau tidak menjawab tepat waktu. Tidak ada yang dicoba ulang; simpan lagi untuk mengirim lagi.",
  "p.commitments.out_not_configured":
    "Tidak dikirim: layanan ini tidak diatur untuk menghubungi layanan tenaga kerja.",
  "p.commitments.out_no_token":
    "Tidak dikirim: tidak ada login untuk diteruskan ke layanan tenaga kerja.",
  "p.commitments.out_too_large":
    "Tidak dikirim: layanan tenaga kerja menerima paling banyak 10 000 ekuivalen waktu penuh.",
  "p.commitments.out_old_zero":
    "Tidak dikirim: nol yang dicatat sebelum nol dikirim, sehingga layanan tenaga kerja mempertahankan angka terakhirnya. Simpan lagi untuk menariknya.",
  "p.commitments.out_other": "Layanan mencatat: {outcome}.",
  "p.discussion.heading": "Diskusi",
  "p.discussion.loading": "Memuat…",
  "p.discussion.could_not_load_comments": "Komentar tidak dapat dimuat.",
  "p.discussion.could_not_load_reactions": "Reaksi tidak dapat dimuat.",
  "p.discussion.could_not_load_files": "Berkas tidak dapat dimuat.",
  "p.discussion.could_not_load_watch":
    "Status apakah Anda mengikuti ini tidak dapat dibaca.",
  "p.discussion.sign_in_to_follow":
    "Masuk untuk mengikuti ini dan diberi tahu tentang komentar dan perubahan baru.",
  "p.discussion.you_follow":
    "Anda mengikuti ini: Anda diberi tahu tentang komentar dan perubahan baru.",
  "p.discussion.you_do_not_follow": "Anda tidak mengikuti ini.",
  "p.discussion.followers_one": "{n} orang mengikutinya.",
  "p.discussion.followers_other": "{n} orang mengikutinya.",
  "p.discussion.follow": "Ikuti",
  "p.discussion.unfollow": "Berhenti mengikuti",
  "p.discussion.followed": "Sekarang Anda mengikuti ini.",
  "p.discussion.unfollowed": "Anda tidak lagi mengikuti ini.",
  "p.discussion.reactions_label": "Reaksi",
  "p.discussion.reaction_named_one": "{reaction}: {n} orang",
  "p.discussion.reaction_named_other": "{reaction}: {n} orang",
  "p.discussion.r_thumbs_up": "Jempol",
  "p.discussion.r_heart": "Hati",
  "p.discussion.r_celebrate": "Rayakan",
  "p.discussion.r_eyes": "Sedang melihat",
  "p.discussion.r_question": "Pertanyaan",
  "p.discussion.comments_heading": "Komentar",
  "p.discussion.no_comments": "Belum ada komentar.",
  "p.discussion.no_author": "seseorang yang belum masuk",
  "p.discussion.by_at": "Oleh {who}, {when}",
  "p.discussion.edited_at": "diubah {when}",
  "p.discussion.edit": "Ubah",
  "p.discussion.edit_named": "Ubah komentar oleh {who}, {when}",
  "p.discussion.delete": "Hapus",
  "p.discussion.delete_named": "Hapus komentar oleh {who}, {when}",
  "p.discussion.confirm_delete_comment":
    "Hapus komentar ini? Versi sebelumnya tetap disimpan, tetapi tidak ada yang melihatnya lagi di halaman.",
  "p.discussion.earlier": "Versi sebelumnya",
  "p.discussion.no_earlier": "Tidak ada versi sebelumnya yang disimpan.",
  "p.discussion.replaced_at": "Diganti {when}",
  "p.discussion.f_edit": "Ubah komentar",
  "p.discussion.save_edit": "Simpan komentar",
  "p.discussion.cancel": "Batal",
  "p.discussion.f_comment": "Komentar Anda",
  "p.discussion.chars_left": "Tersisa {n} dari {max} karakter.",
  "p.discussion.chars_over": "Melebihi batas {max} sebanyak {n} karakter.",
  "p.discussion.post": "Kirim komentar",
  "p.discussion.posted": "Komentar telah dikirim.",
  "p.discussion.edited": "Komentar diubah; versi sebelumnya tetap disimpan.",
  "p.discussion.deleted": "Komentar telah dihapus.",
  "p.discussion.problem_blank": "Tulis komentar terlebih dahulu.",
  "p.discussion.problem_too_long":
    "Komentar dapat terdiri dari paling banyak 4000 karakter.",
  "p.discussion.files_heading": "Berkas",
  "p.discussion.files_intro":
    "Berkas yang dilampirkan di sini disimpan ke komputer Anda saat dibuka; berkas tidak pernah ditampilkan di dalam halaman ini.",
  "p.discussion.no_files": "Tidak ada berkas terlampir.",
  "p.discussion.file_facts": "{type}, {size}, ditambahkan oleh {who}, {when}",
  "p.discussion.remove_file": "Hapus",
  "p.discussion.remove_file_named": "Hapus berkas {name}",
  "p.discussion.confirm_remove_file":
    "Hapus berkas {name}? Tidak ada yang dapat mengunduhnya lagi dari sini.",
  "p.discussion.removed_file": "Berkas {name} telah dihapus.",
  "p.discussion.f_file": "Berkas untuk dilampirkan",
  "p.discussion.files_rules":
    "File PDF, gambar (PNG, JPEG, GIF, atau WebP), teks biasa, CSV, Markdown, JSON, atau file Word, Excel, atau PowerPoint, paling besar 10 MB. Halaman web, gambar SVG, arsip, dan program ditolak.",
  "p.discussion.attach": "Lampirkan file",
  "p.discussion.attached": "File {name} telah dilampirkan.",
  "p.discussion.problem_choose_file": "Pilih file terlebih dahulu.",
  "p.discussion.problem_empty": "File itu kosong.",
  "p.discussion.problem_too_large":
    "File itu lebih dari 10 MB, batas untuk satu file.",
  "p.discussion.problem_type":
    "Jenis file itu tidak dapat dilampirkan. Daftar di bawah menyebutkan jenis yang dapat dilampirkan.",
  "p.discussion.type_pdf": "Dokumen PDF",
  "p.discussion.type_png": "Gambar PNG",
  "p.discussion.type_jpeg": "Gambar JPEG",
  "p.discussion.type_gif": "Gambar GIF",
  "p.discussion.type_webp": "Gambar WebP",
  "p.discussion.type_text": "teks biasa",
  "p.discussion.type_csv": "Tabel CSV",
  "p.discussion.type_markdown": "Teks Markdown",
  "p.discussion.type_json": "Data JSON",
  "p.discussion.type_word": "Dokumen Word",
  "p.discussion.type_excel": "Buku kerja Excel",
  "p.discussion.type_powerpoint": "Slide PowerPoint",
  "p.discussion.size_b": "{n} byte",
  "p.discussion.size_kb": "{n} KB",
  "p.discussion.size_mb": "{n} MB",
  "p.discussion.refused_sign_in":
    "Masuk terlebih dahulu: layanan tidak mengetahui siapa Anda.",
  "p.discussion.refused_not_author":
    "Hanya orang yang menulis komentar yang boleh mengubah atau menghapusnya.",
  "p.discussion.refused_not_uploader":
    "Hanya orang yang melampirkan file, atau administrator, yang boleh menghapusnya.",
  "p.discussion.refused_forbidden":
    "Anda tidak boleh membaca atau mengubah ini.",
  "p.discussion.refused_comment_gone":
    "Komentar itu sudah tidak ada; komentar telah dibaca ulang.",
  "p.discussion.refused_file_gone":
    "File itu sudah dihapus; daftar file telah dibaca ulang.",
  "p.discussion.refused_gone":
    "Ini tidak dapat ditemukan: mungkin sudah dihapus, atau Anda tidak dapat melihatnya.",
  "p.discussion.refused_comment_invalid":
    "Layanan tidak menerima komentar itu: komentar tidak boleh kosong, dan paling banyak 4000 karakter.",
  "p.discussion.refused_file_invalid":
    "Layanan tidak menerima file itu: jenis, ukuran, atau isinya tidak lolos pemeriksaan (file harus sesuai dengan jenisnya, dan tidak ada program yang diterima).",
  "p.discussion.refused_too_large": "File terlalu besar untuk dikirim.",
  "p.discussion.refused_scan_unavailable":
    "File tidak dilampirkan: pemindai malware tidak menjawab. Coba lagi nanti.",
  "p.me.timesheet_link": "Lembar waktu saya",
  "p.me.timesheet_note":
    "— waktu Anda menurut hari dan rencana, minggu demi minggu, dan pengiriman satu minggu untuk persetujuan.",
  "p.timesheet.title_ppm": "Lembar waktu saya — PPM",
  "p.timesheet.heading": "Lembar waktu saya",
  "p.timesheet.back": "← Pekerjaan saya",
  "p.timesheet.intro":
    "Waktu yang Anda catat untuk satu minggu, menurut hari dan rencana, dalam jam dan menit, dengan waktu yang dapat ditagih dan yang tidak dapat ditagih selalu ditampilkan terpisah. Kirim minggu yang sudah selesai untuk persetujuan; setelah disetujui, minggu itu final dan tidak ada waktu lagi yang dapat dicatat di dalamnya.",
  "p.timesheet.loading": "Memuat…",
  "p.timesheet.week_nav": "Pilih minggu",
  "p.timesheet.previous_week": "Minggu sebelumnya",
  "p.timesheet.this_week": "Minggu ini",
  "p.timesheet.next_week": "Minggu berikutnya",
  "p.timesheet.f_week": "Minggu, misalnya 2026-W46",
  "p.timesheet.show_week": "Tampilkan minggu",
  "p.timesheet.week_hint":
    "Minggu berjalan dari Senin sampai Minggu dan diberi nomor menurut standar internasional, apa pun hari pertama minggu Anda. Hari ini adalah {today} di zona waktu Anda, {tz}.",
  "p.timesheet.problem_week":
    "Itu bukan minggu: tulis tahun, ‘-W’, dan nomor minggu, misalnya 2026-W46.",
  "p.timesheet.mine_heading": "Minggu Anda {week}",
  "p.timesheet.mine_caption":
    "Waktu Anda pada minggu {week}, menurut rencana dan hari",
  "p.timesheet.status_open": "Terbuka: belum dikirim untuk persetujuan",
  "p.timesheet.status_submitted":
    "Dikirim untuk persetujuan: menunggu keputusan",
  "p.timesheet.status_approved":
    "Disetujui: final, dan terkunci terhadap waktu baru",
  "p.timesheet.status_rejected": "Dikembalikan: ubah lalu kirim lagi",
  "p.timesheet.rejected_because": "penyetuju berkata: {comment}",
  "p.timesheet.decision_comment": "dengan catatan: {comment}",
  "p.timesheet.no_entries":
    "Tidak ada waktu yang dicatat pada minggu ini, jadi tidak ada total.",
  "p.timesheet.col_plan": "Nama rencana",
  "p.timesheet.col_total": "Jumlah",
  "p.timesheet.col_billable": "Dapat ditagih",
  "p.timesheet.col_non_billable": "Tidak dapat ditagih",
  "p.timesheet.row_day_totals": "Setiap hari",
  "p.timesheet.plan_unnamed": "Rencana {id}… (nama tidak ditampilkan)",
  "p.timesheet.split_line":
    "Dapat ditagih {billable}, tidak dapat ditagih {non_billable}: total {total}.",
  "p.timesheet.omitted":
    "Entri yang dikecualikan karena berada di rencana yang tidak dapat Anda baca: {n}.",
  "p.timesheet.hours_one": "{n} jam",
  "p.timesheet.hours_other": "{n} jam",
  "p.timesheet.minutes_one": "{n} mnt",
  "p.timesheet.minutes_other": "{n} mnt",
  "p.timesheet.duration_hm": "{h} {m}",
  "p.timesheet.submit": "Kirim minggu ini untuk persetujuan",
  "p.timesheet.submit_again": "Kirim minggu ini lagi untuk persetujuan",
  "p.timesheet.submitted_notice":
    "Minggu {week} telah dikirim untuk persetujuan.",
  "p.timesheet.nothing_to_submit":
    "Minggu tanpa waktu yang dicatat tidak dapat dikirim untuk persetujuan.",
  "p.timesheet.waiting": "Minggu ini menunggu keputusan penyetuju.",
  "p.timesheet.locked":
    "Minggu yang disetujui bersifat final: tidak ada waktu lagi yang dapat dicatat di dalamnya.",
  "p.timesheet.could_not_load": "Minggu Anda tidak dapat dimuat.",
  "p.timesheet.could_not_submit":
    "Minggu tidak dapat dikirim untuk persetujuan.",
  "p.timesheet.other_heading": "Minggu orang lain, untuk penyetuju",
  "p.timesheet.other_intro":
    "Jika Anda boleh menyetujui lembar waktu, berikan rujukan orang itu untuk membaca minggunya pada minggu yang dipilih di atas, lalu setujui atau kembalikan. Tidak ada yang memutuskan minggunya sendiri, pengembalian memerlukan alasan, dan persetujuan bersifat final.",
  "p.timesheet.f_person": "Orang: ‘person:’ atau ‘worker:’ dan id-nya",
  "p.timesheet.show_person": "Tampilkan minggunya",
  "p.timesheet.problem_person":
    "Berikan orang sebagai ‘person:’ atau ‘worker:’ diikuti id-nya, tanpa tambahan lain.",
  "p.timesheet.other_week_heading": "Minggu {week} milik {person}",
  "p.timesheet.other_caption":
    "Waktu {person} pada minggu {week}, menurut rencana dan hari",
  "p.timesheet.decision_legend": "Keputusan Anda",
  "p.timesheet.approve": "Setujui",
  "p.timesheet.reject": "Kembalikan",
  "p.timesheet.f_comment": "Catatan, jika Anda mau",
  "p.timesheet.f_comment_required": "Mengapa dikembalikan (wajib)",
  "p.timesheet.approve_final":
    "Persetujuan bersifat final: minggu itu kemudian terkunci, dan tidak ada yang dapat mengembalikannya.",
  "p.timesheet.record_decision": "Catat keputusan",
  "p.timesheet.approved_notice": "Minggu {week} milik {person} disetujui.",
  "p.timesheet.rejected_notice": "Minggu {week} milik {person} dikembalikan.",
  "p.timesheet.other_not_submitted":
    "Minggu ini belum dikirim untuk persetujuan, jadi belum ada yang perlu diputuskan.",
  "p.timesheet.other_rejected":
    "Minggu ini dikembalikan; dapat diputuskan lagi setelah orang itu mengirimnya lagi.",
  "p.timesheet.problem_own_week":
    "Itu minggu Anda sendiri: tidak ada yang memutuskan lembar waktunya sendiri.",
  "p.timesheet.problem_comment": "Sebutkan mengapa minggu itu dikembalikan.",
  "p.timesheet.could_not_load_other": "Minggu orang itu tidak dapat dimuat.",
  "p.timesheet.could_not_decide": "Keputusan tidak dapat dicatat.",
  "p.timesheet.refused_read_forbidden":
    "Hanya orang itu sendiri atau penyetuju lembar waktu yang boleh membaca minggu itu.",
  "p.timesheet.refused_submit_forbidden":
    "Hanya orang pemilik minggu itu yang boleh mengirimnya untuk persetujuan.",
  "p.timesheet.refused_decide_forbidden":
    "Anda tidak boleh memutuskan minggu ini: diperlukan hak penyetuju lembar waktu, dan tidak ada yang memutuskan minggunya sendiri.",
  "p.timesheet.refused_submit_conflict":
    "Minggu ini sudah dikirim untuk persetujuan atau sudah disetujui; minggu telah dibaca ulang.",
  "p.timesheet.refused_decide_conflict":
    "Minggu ini tidak lagi menunggu keputusan; minggu telah dibaca ulang.",
  "p.timesheet.refused_not_addressable":
    "Identitas login Anda tidak memiliki rujukan orang, jadi tidak ada lembar waktu untuk ditampilkan.",
  "p.timesheet.refused_read_invalid":
    "Layanan tidak menerima minggu atau orang itu.",
  "p.timesheet.refused_submit_invalid":
    "Minggu tidak dikirim: tidak ada waktu yang dicatat di dalamnya.",
  "p.timesheet.refused_decide_invalid":
    "Layanan tidak menerima keputusan itu: mengembalikan minggu memerlukan alasan.",
  "p.security.access_link": "Tinjauan akses dan akses darurat",
  "p.security.access_note":
    "— siapa memegang akses apa, untuk sertifikasi ulang, dan akses darurat terbatas waktu ke rencana pribadi, yang dicatat dan diaudit.",
  "p.access.title_ppm": "Tinjauan akses dan akses darurat — PPM",
  "p.access.back": "← Keamanan",
  "p.access.heading": "Tinjauan akses dan akses darurat",
  "p.access.intro":
    "Tinjauan akses mencantumkan akses yang dicatat orang sebagai telah diberikan, untuk sertifikasi ulang. Akses darurat membuka rencana pribadi bagi Anda untuk waktu terbatas ketika tidak ada cara lain; setiap penggunaan dicatat dan diaudit.",
  "p.access.review_heading": "Tinjauan akses",
  "p.access.review_intro":
    "Akses yang dicatat sebagai diberikan dalam buku besar akses dan belum dicabut, yang tertua lebih dahulu, beserta siapa yang memberikannya dan kapan. Ini menunjukkan apa yang dicatat orang, bukan apa yang diberlakukan layanan login: bandingkan keduanya. Hanya untuk administrator; setiap pembacaan dicatat dalam jejak audit, sehingga tinjauan hanya dibaca ketika Anda memintanya.",
  "p.access.f_review_plan":
    "Id rencana (kosongkan untuk setiap rencana yang dapat Anda baca)",
  "p.access.f_review_days": "Jatuh tempo tinjauan setelah sekian hari kalender",
  "p.access.show_review": "Tampilkan tinjauan",
  "p.access.problem_review_plan":
    "Itu bukan id rencana: tempelkan id rencana, atau kosongkan kotak untuk setiap rencana.",
  "p.access.problem_review_days":
    "Tulis jumlah hari kalender sebagai bilangan bulat, 0 atau lebih.",
  "p.access.could_not_load_review": "Tinjauan akses tidak dapat dimuat.",
  "p.access.review_totals":
    "Per {as_of}. Pemberian akses yang masih berlaku: {standing}. Jatuh tempo tinjauan, berusia {days} hari kalender atau lebih: {due}.",
  "p.access.review_note":
    "Usia dihitung dalam hari kalender dari hari pemberian akses dicatat sampai hari ini menurut penerapan.",
  "p.access.download_review":
    "Unduh tinjauan sebagai spreadsheet, dengan kolom untuk keputusan Anda mempertahankan atau mencabut",
  "p.access.review_caption":
    "Akses yang berlaku hari ini, yang tertua lebih dahulu",
  "p.access.review_empty":
    "Tidak ada akses yang dicatat sebagai diberikan yang masih berlaku di rencana yang dapat Anda baca.",
  "p.access.col_person": "Orang",
  "p.access.col_access": "Akses",
  "p.access.col_plan": "Nama rencana",
  "p.access.col_granted_on": "Diberikan pada",
  "p.access.col_granted_by": "Diberikan oleh",
  "p.access.col_age": "Usia dalam hari kalender",
  "p.access.col_review": "Tinjauan",
  "p.access.col_note": "Catatan",
  "p.access.due": "Jatuh tempo tinjauan",
  "p.access.not_due": "Belum jatuh tempo",
  "p.access.plan_unnamed": "Rencana {id}… (nama tidak ditampilkan)",
  "p.access.bg_heading": "Akses darurat ke rencana pribadi (break-glass)",
  "p.access.bg_audited_heading":
    "Setiap penggunaan akses darurat dicatat dan diaudit.",
  "p.access.bg_audited_open":
    "Membuka rencana mencatat siapa Anda, rencananya, alasan Anda, berapa lama, dan orang yang Anda sebut, dalam jejak audit.",
  "p.access.bg_audited_told":
    "Orang yang Anda sebut langsung diberi tahu. Jika tidak dapat diberi tahu, rencana tidak dibuka.",
  "p.access.bg_audited_each":
    "Setiap permintaan yang hanya diloloskan oleh akses ini dicatat dalam jejak audit sebagai akses darurat.",
  "p.access.bg_audited_visibility":
    "Akses ini memungkinkan Anda melihat rencana. Apa yang boleh Anda ubah tetap ditentukan oleh hak Anda sendiri.",
  "p.access.bg_audited_ends":
    "Akses berakhir dengan sendirinya pada waktu yang ditampilkan, atau lebih cepat jika Anda mengakhirinya. Paling banyak lima sehari, dan satu per satu pada setiap rencana.",
  "p.access.bg_intro":
    "Gunakan hanya ketika anggota rencana tidak dapat dihubungi dan pekerjaan tidak dapat menunggu. Diperlukan kelayakan break-glass atau akses administrator; layanan yang memutuskan.",
  "p.access.problems_heading": "Rencana tidak dibuka:",
  "p.access.problem_plan": "Berikan id rencana pribadi.",
  "p.access.problem_reason_short":
    "Sebutkan mengapa Anda memerlukan akses, minimal {reason_min} karakter.",
  "p.access.problem_reason_long":
    "Alasan lebih panjang dari {reason_max} karakter.",
  "p.access.problem_minutes":
    "Berikan lama waktu dalam menit bulat, dari {minutes_min} sampai {minutes_max}.",
  "p.access.problem_notify":
    "Sebutkan orang yang akan diberi tahu sebagai ‘person:’ atau ‘worker:’ diikuti id-nya, tanpa tambahan lain.",
  "p.access.problem_acknowledged":
    "Centang kotak untuk menyatakan bahwa Anda memahami bahwa ini dicatat dan diaudit dan bahwa orang itu diberi tahu.",
  "p.access.could_not_open": "Rencana tidak dapat dibuka.",
  "p.access.opened_notice":
    "Rencana terbuka bagi Anda sampai {until} (zona waktu Anda, {tz}). {person} telah diberi tahu.",
  "p.access.go_to_plan": "Buka rencana",
  "p.access.f_plan": "Id rencana pribadi",
  "p.access.f_reason":
    "Mengapa Anda memerlukan akses (wajib, minimal {min} karakter)",
  "p.access.reason_count": "{n} karakter; minimal {min} dan maksimal {max}.",
  "p.access.f_minutes": "Berapa lama, dalam menit ({min} sampai {max})",
  "p.access.f_notify":
    "Orang yang diberi tahu: ‘person:’ atau ‘worker:’ dan id-nya (bukan Anda)",
  "p.access.f_acknowledge":
    "Saya memahami bahwa ini dicatat dan diaudit, dan bahwa orang yang saya sebut akan diberi tahu.",
  "p.access.open": "Buka rencana dengan akses darurat",
  "p.access.grants_heading": "Pemberian akses darurat",
  "p.access.grants_intro":
    "Milik Anda, yang terbaru lebih dahulu; administrator melihat milik semua orang. Pemberian akses tidak pernah dihapus. Waktu ditampilkan dalam zona waktu Anda, {tz}.",
  "p.access.grants_caption":
    "Pemberian akses darurat, yang terbaru lebih dahulu",
  "p.access.grants_empty": "Belum ada akses darurat yang dibuka.",
  "p.access.loading": "Memuat…",
  "p.access.could_not_load_grants":
    "Pemberian akses darurat tidak dapat dimuat.",
  "p.access.could_not_revoke": "Akses darurat tidak dapat diakhiri.",
  "p.access.revoked_notice": "Akses darurat ke {plan} telah berakhir.",
  "p.access.revoke": "Akhiri sekarang",
  "p.access.revoke_for": "Akhiri sekarang: akses darurat ke {plan}",
  "p.access.state_live": "Berlaku",
  "p.access.state_revoked": "Diakhiri lebih awal",
  "p.access.state_expired": "Habis",
  "p.access.ended_by": "{when}, diakhiri oleh {who}",
  "p.access.col_state": "Status",
  "p.access.col_holder": "Dipegang oleh",
  "p.access.col_opened": "Dibuka",
  "p.access.col_ends": "Berakhir atau telah berakhir",
  "p.access.col_reason": "Alasan yang diberikan",
  "p.access.col_told": "Orang yang diberi tahu",
  "p.access.col_act": "Tindakan",
  "p.access.refused_sign_in": "Masuk terlebih dahulu.",
  "p.access.refused_review_forbidden":
    "Tinjauan akses hanya untuk administrator.",
  "p.access.refused_review_invalid":
    "Layanan tidak menerima rencana atau jumlah hari itu.",
  "p.access.refused_open_forbidden":
    "Anda tidak boleh menggunakan akses darurat: diperlukan kelayakan break-glass atau akses administrator.",
  "p.access.refused_open_not_found": "Tidak ada rencana dengan id itu.",
  "p.access.refused_open_conflict":
    "Anda sudah memegang akses darurat ke rencana ini; daftar di bawah telah dibaca ulang.",
  "p.access.refused_open_invalid":
    "Rencana tidak dibuka. Rencana itu tidak pribadi sehingga tidak ada yang perlu dibuka, atau orang yang diberi tahu adalah Anda sendiri, atau orang itu tidak dapat diberi tahu. Tidak ada akses yang diberikan.",
  "p.access.refused_open_limit":
    "Anda telah membuka akses darurat sebanyak batas yang diizinkan dalam sehari (lima, dihitung selama 24 jam terakhir).",
  "p.access.refused_revoke_not_found":
    "Pemberian akses itu tidak ditemukan, atau bukan hak Anda untuk mengakhirinya.",
  "p.deescalation.nav": "Kartu skor dan de-eskalasi",
  "p.deescalation.title": "Kartu skor dan de-eskalasi — PPM",
  "p.deescalation.h1": "Kartu skor dan de-eskalasi",
  "p.deescalation.lede":
    "Tingkat eskalasi sebagaimana ditetapkan sponsor, kriteria untuk menurunkannya beserta buktinya setiap kuartal, dan kartu skor yang ukuran kuncinya dibaca menurut kuartal yang telah selesai.",
  "p.deescalation.tracker_heading": "Pelacak de-eskalasi",
  "p.deescalation.tracker_says":
    "Tingkat dicatat sebagaimana ditetapkan sponsor. Tingkat tidak pernah dihitung di sini, dan tidak ada yang di halaman ini yang mengubahnya: rangkaian kuartal yang terpenuhi ditawarkan kepada orang yang memutuskan.",
  "p.deescalation.loading": "Memuat…",
  "p.deescalation.could_not_load": "Pelacak de-eskalasi tidak dapat dimuat.",
  "p.deescalation.could_not_save": "Perubahan tidak disimpan.",
  "p.deescalation.level_heading": "Tingkat eskalasi",
  "p.deescalation.level_now": "Tingkat {level}, ditetapkan pada {date}.",
  "p.deescalation.level_note": "Catatan sponsor: {note}",
  "p.deescalation.level_none": "Belum ada tingkat eskalasi yang dicatat.",
  "p.deescalation.level_history": "Tingkat sebelumnya",
  "p.deescalation.level_history_item": "{level}, ditetapkan pada {date}",
  "p.deescalation.run_heading": "Kuartal terpenuhi berturut-turut",
  "p.deescalation.run_one":
    "{n} kuartal selesai terpenuhi berturut-turut, dari {need} yang diminta aturan.",
  "p.deescalation.run_other":
    "{n} kuartal selesai terpenuhi berturut-turut, dari {need} yang diminta aturan.",
  "p.deescalation.rule_satisfied":
    "Aturan terpenuhi. Ini untuk dipertimbangkan sponsor; tingkat tidak berubah dengan sendirinya.",
  "p.deescalation.rule_not_satisfied": "Aturan belum terpenuhi.",
  "p.deescalation.setting_unusable":
    "Pengaturan penerapan untuk jumlah kuartal yang diminta aturan tidak dapat digunakan, sehingga bawaan {n} berlaku.",
  "p.deescalation.reads_scorecard": "Pelacak membaca kartu skor {name}.",
  "p.deescalation.reads_no_scorecard":
    "Tidak ada kartu skor yang dibaca pelacak, jadi hanya kriteria yang dihitung. Tandai salah satunya saat Anda menambahkannya, di bawah.",
  "p.deescalation.hidden_kpis_one":
    "{n} ukuran kunci pada kartu skor ini milik rencana yang tidak boleh Anda baca: tidak ditampilkan dan tidak dihitung di sini.",
  "p.deescalation.hidden_kpis_other":
    "{n} ukuran kunci pada kartu skor ini milik rencana yang tidak boleh Anda baca: tidak ditampilkan dan tidak dihitung di sini.",
  "p.deescalation.quarters_label": "Setiap kuartal yang telah selesai",
  "p.deescalation.col_quarter": "Kuartal",
  "p.deescalation.col_state": "Kedudukan",
  "p.deescalation.col_reasons": "Alasan",
  "p.deescalation.state_met": "Terpenuhi",
  "p.deescalation.state_not_met": "Tidak terpenuhi",
  "p.deescalation.state_missing": "Tidak ada yang dicatat",
  "p.deescalation.state_with_value": "{state} ({value})",
  "p.deescalation.reason_all_met":
    "Setiap ukuran kunci dan setiap kriteria terpenuhi.",
  "p.deescalation.reason_nothing":
    "Tidak ada yang dapat diukur: tidak ada ukuran kunci dan tidak ada kriteria.",
  "p.deescalation.reason_no_kpis":
    "Kartu skor tidak memiliki ukuran kunci yang dapat Anda baca.",
  "p.deescalation.reason_not_met": "Tidak terpenuhi: {list}.",
  "p.deescalation.reason_missing": "Tidak ada yang dicatat untuk: {list}.",
  "p.deescalation.quarter": "K{q} {year}",
  "p.deescalation.quarter_standing": "{quarter}: {state}. {reasons}",
  "p.deescalation.tracker_kpis_label": "Kartu skor pelacak, menurut kuartal",
  "p.deescalation.set_level_heading": "Catat tingkat eskalasi",
  "p.deescalation.set_level_says":
    "Hanya sponsor (seseorang dengan peran sponsor) atau administrator yang dapat mencatat tingkat, sebagaimana yang telah mereka tetapkan. Tingkat terdiri dari {min} sampai {max} karakter; catatan bersifat opsional.",
  "p.deescalation.f_level": "Tingkat",
  "p.deescalation.f_level_note": "Catatan (opsional)",
  "p.deescalation.save_level": "Catat tingkat",
  "p.deescalation.level_saved": "Tingkat {level} dicatat.",
  "p.deescalation.unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.deescalation.problem_level":
    "Tingkat terdiri dari {min} sampai {max} karakter.",
  "p.deescalation.refused_level_forbidden":
    "Ditolak: hanya sponsor atau administrator yang dapat mencatat tingkat eskalasi.",
  "p.deescalation.criteria_heading": "Kriteria untuk menurunkan tingkat",
  "p.deescalation.criteria_says":
    "Kriteria ditambahkan dan tidak pernah diedit atau dipensiunkan. Buktinya dicatat untuk kuartal yang telah selesai, dan catatan terbaru untuk kuartal itu yang dihitung.",
  "p.deescalation.criteria_none": "Belum ada kriteria.",
  "p.deescalation.evidence_of": "Bukti untuk: {criterion}",
  "p.deescalation.evidence_link": "Bukti",
  "p.deescalation.add_criterion_heading": "Tambah kriteria",
  "p.deescalation.f_criterion": "Kriteria ({min} sampai {max} karakter)",
  "p.deescalation.add_criterion": "Tambah kriteria",
  "p.deescalation.criterion_added": "Kriteria ditambahkan.",
  "p.deescalation.problem_criterion":
    "Kriteria terdiri dari {min} sampai {max} karakter.",
  "p.deescalation.evidence_heading": "Catat bukti",
  "p.deescalation.evidence_says":
    "Pilih kriteria dan kuartal yang telah selesai (kuartal yang masih berjalan tidak dapat dicatat), sebutkan apakah terpenuhi, dan berikan catatan minimal {min} karakter. Tautan bersifat opsional: alamat http atau https tanpa spasi, hingga {max} karakter.",
  "p.deescalation.f_ev_criterion": "Kriteria",
  "p.deescalation.f_ev_quarter": "Kuartal yang telah selesai",
  "p.deescalation.f_ev_met": "Apakah terpenuhi?",
  "p.deescalation.f_ev_note": "Catatan (minimal {min} karakter)",
  "p.deescalation.f_ev_link": "Tautan ke bukti (opsional)",
  "p.deescalation.save_evidence": "Catat bukti",
  "p.deescalation.evidence_saved": "Bukti dicatat untuk {quarter}.",
  "p.deescalation.problem_criterion_choose":
    "Tambahkan kriteria terlebih dahulu, lalu catat buktinya.",
  "p.deescalation.problem_quarter": "Pilih kuartal yang telah selesai.",
  "p.deescalation.problem_quarter_unfinished":
    "Kuartal itu belum selesai, jadi belum dapat dicatat.",
  "p.deescalation.problem_met": "Sebutkan apakah kriteria terpenuhi.",
  "p.deescalation.problem_note": "Diperlukan catatan minimal {min} karakter.",
  "p.deescalation.problem_link":
    "Tautan adalah alamat http atau https tanpa spasi, hingga {max} karakter.",
  "p.deescalation.refused_evidence":
    "Ditolak: kuartal harus sudah selesai, catatan minimal lima karakter, dan tautan harus berupa alamat http atau https tanpa spasi.",
  "p.deescalation.refused_gone":
    "Catatan itu sudah tidak ada. Muat ulang halaman.",
  "p.deescalation.scorecards_heading": "Kartu skor",
  "p.deescalation.scorecards_says":
    "Kartu skor adalah kumpulan ukuran kunci bernama, dibaca menurut kuartal yang telah selesai. Ukuran kunci terpenuhi dalam suatu kuartal ketika pembacaan terbarunya pada kuartal itu sama dengan atau lebih baik daripada targetnya; kuartal tanpa pembacaan dianggap hilang, yang tidak sama dengan tidak terpenuhi.",
  "p.deescalation.scorecards_none": "Belum ada kartu skor.",
  "p.deescalation.could_not_load_scorecards": "Kartu skor tidak dapat dimuat.",
  "p.deescalation.col_scorecard": "Kartu skor",
  "p.deescalation.col_tracker": "Dibaca oleh pelacak",
  "p.deescalation.is_tracker": "Ya, yang dibaca pelacak",
  "p.deescalation.not_tracker": "Tidak",
  "p.deescalation.add_scorecard_heading": "Tambah kartu skor",
  "p.deescalation.add_scorecard_says":
    "Nama sepanjang {min} sampai {max} karakter, belum dipakai (dalam huruf besar atau kecil apa pun). Paling banyak satu kartu skor yang dibaca pelacak.",
  "p.deescalation.f_name": "Nama",
  "p.deescalation.f_for_tracker": "Yang dibaca pelacak",
  "p.deescalation.add_scorecard": "Tambah kartu skor",
  "p.deescalation.scorecard_added": "Kartu skor {name} ditambahkan.",
  "p.deescalation.problem_name":
    "Nama kartu skor terdiri dari {min} sampai {max} karakter.",
  "p.deescalation.problem_name_taken":
    "Sebuah kartu skor sudah memiliki nama itu.",
  "p.deescalation.problem_tracker_taken":
    "Kartu skor {name} sudah menjadi yang dibaca pelacak.",
  "p.deescalation.scorecard_heading": "Kartu skor menurut kuartal",
  "p.deescalation.f_scorecard": "Kartu skor",
  "p.deescalation.choose_scorecard": "Pilih kartu skor",
  "p.deescalation.could_not_load_scorecard": "Kartu skor tidak dapat dimuat.",
  "p.deescalation.scorecard_says":
    "Setiap ukuran kunci menurut kuartal yang telah selesai: terpenuhi, tidak terpenuhi, atau tidak ada yang dicatat.",
  "p.deescalation.scorecard_kpis_label": "Ukuran kunci {name}, menurut kuartal",
  "p.deescalation.scorecard_quarters_label": "Setiap kuartal kartu skor",
  "p.deescalation.col_kpi": "Ukuran kunci",
  "p.deescalation.col_target": "Nilai target",
  "p.deescalation.target_at_least": "minimal {value} {unit}",
  "p.deescalation.target_at_most": "maksimal {value} {unit}",
  "p.deescalation.no_kpis":
    "Kartu skor ini tidak memiliki ukuran kunci yang dapat Anda baca.",
  "p.deescalation.unit_percent": "persen",
  "p.deescalation.unit_count": "jumlah",
  "p.deescalation.kpis_heading": "Pilih ukuran kuncinya (KPI)",
  "p.deescalation.kpis_says":
    "Centang ukuran kunci yang dimuat kartu skor ini, paling banyak {max}. Menyimpan akan mengganti seluruh kumpulan; ukuran kunci pada rencana yang tidak boleh Anda baca tetap berada di kartu.",
  "p.deescalation.f_kpis": "Ukuran kunci pada kartu skor",
  "p.deescalation.kpis_none":
    "Tidak ada ukuran kunci untuk dipilih. Tambahkan di halaman ukuran kunci, atau dari templat di bawah.",
  "p.deescalation.save_kpis": "Simpan ukuran kunci",
  "p.deescalation.kpis_saved_one":
    "Kartu skor sekarang memuat {n} ukuran kunci.",
  "p.deescalation.kpis_saved_other":
    "Kartu skor sekarang memuat {n} ukuran kunci.",
  "p.deescalation.problem_kpis_max":
    "Kartu skor memuat paling banyak {max} ukuran kunci.",
  "p.deescalation.refused_kpis":
    "Ditolak: masing-masing harus berupa ukuran kunci yang dapat Anda baca, dan kartu skor memuat paling banyak 60.",
  "p.deescalation.template_heading": "Tambah ukuran dari templat",
  "p.deescalation.template_says":
    "Kumpulan awal, ditawarkan dan tidak diterapkan. Tidak memiliki target: Anda menetapkan setiap target dan garis merah, dan ukuran yang Anda pilih menjadi ukuran kunci pada kartu skor ini.",
  "p.deescalation.template_rule":
    "Jika lebih tinggi lebih baik, garis merah harus di bawah target; jika lebih rendah lebih baik, di atasnya. Target dan garis merah berupa bilangan bulat.",
  "p.deescalation.could_not_load_template": "Templat tidak dapat dimuat.",
  "p.deescalation.higher_better": "Lebih tinggi lebih baik",
  "p.deescalation.lower_better": "Lebih rendah lebih baik",
  "p.deescalation.unit": "diukur sebagai {unit}",
  "p.deescalation.f_pick": "Tambah {measure}",
  "p.deescalation.f_target": "Nilai target",
  "p.deescalation.f_red": "Garis merah",
  "p.deescalation.add_measures": "Tambah ukuran",
  "p.deescalation.measures_added_one": "{n} ukuran ditambahkan ke kartu skor.",
  "p.deescalation.measures_added_other":
    "{n} ukuran ditambahkan ke kartu skor.",
  "p.deescalation.problem_pick_one": "Pilih setidaknya satu ukuran.",
  "p.deescalation.problem_whole":
    "{measure}: target dan garis merah berupa bilangan bulat.",
  "p.deescalation.problem_red_higher":
    "{measure}: lebih tinggi lebih baik, jadi garis merah harus di bawah target.",
  "p.deescalation.problem_red_lower":
    "{measure}: lebih rendah lebih baik, jadi garis merah harus di atas target.",
  "p.deescalation.problem_not_in_template":
    "{measure} tidak ada dalam templat.",
  "p.deescalation.refused_template":
    "Ditolak: periksa setiap target dan garis merah terhadap aturan arah.",
  "p.deescalation.measure_programmes_with_independent_assurance":
    "Program dengan jaminan independen pada kuartal terakhir",
  "p.deescalation.measure_milestones_met_on_time":
    "Pencapaian yang terpenuhi pada atau sebelum tanggalnya",
  "p.deescalation.measure_benefits_validated":
    "Manfaat terencana yang divalidasi oleh pihak di luar program",
  "p.deescalation.measure_red_without_decision":
    "Program berstatus merah tanpa keputusan yang dicatat",
  "p.deescalation.measure_open_critical_issues":
    "Isu terbuka dengan tingkat keparahan kritis",
  "p.deescalation.measure_change_requests_waiting":
    "Permintaan perubahan yang menunggu keputusan lebih dari 30 hari",
  "p.phase.name_initiating": "Inisiasi",
  "p.phase.name_planning": "Perencanaan",
  "p.phase.name_executing": "Pelaksanaan",
  "p.phase.name_controlling": "Pengendalian",
  "p.phase.name_closing": "Penutupan",
  "p.phase.heading": "Fase",
  "p.phase.heading_programme": "Fase dan awal",
  "p.phase.says":
    "Rencana maju satu fase pada satu waktu; rencana dapat mundur ke fase sebelumnya dengan alasan, yang dicatat bersama perpindahannya. Fase tidak pernah menghentikan pencatatan pekerjaan.",
  "p.phase.could_not_load": "Fase rencana tidak dapat dimuat",
  "p.phase.now": "Rencana berada pada fase {phase}.",
  "p.phase.none":
    "Belum ada fase yang dicatat: perpindahan pertama adalah ke Inisiasi.",
  "p.phase.time_heading": "Waktu di setiap fase",
  "p.phase.days_in_one": "{phase}: {days} hari",
  "p.phase.days_in_other": "{phase}: {days} hari",
  "p.phase.visits_one": "dimasuki {visits} kali",
  "p.phase.visits_other": "dimasuki {visits} kali",
  "p.phase.current": "sekarang",
  "p.phase.check_heading": "Pemeriksaan awal",
  "p.phase.check_says":
    "Program dimulai ketika fasenya berpindah ke Pelaksanaan. Permulaan ditolak jika semua tempat untuk program dalam pelaksanaan aktif sudah terisi (program yang dijeda dan dihentikan tidak memakai tempat) atau jika kolam kendala kekurangan pasokan dalam salah satu dari enam bulan ke depan. Program yang ditolak masuk antrean awal dan mempertahankan tempatnya; program dapat tetap dimulai dengan alasan penggantian sepanjang {min} karakter atau lebih, yang dicatat dan diaudit. Membaca pemeriksaan tidak memindahkan apa pun.",
  "p.phase.could_not_check": "Pemeriksaan awal tidak dapat dibaca",
  "p.phase.places":
    "{in_delivery} dari {limit} tempat untuk program dalam pelaksanaan aktif sudah terisi.",
  "p.phase.no_limit":
    "Tidak ada batas program dalam pelaksanaan aktif yang ditetapkan.",
  "p.phase.limit_unusable":
    "Batas penerapan untuk program dalam pelaksanaan tidak dapat digunakan, jadi tidak ada batas yang berlaku.",
  "p.phase.pool_is": "Kolam kendala adalah {pool}.",
  "p.phase.no_pool": "Tidak ada kolam kendala yang ditentukan.",
  "p.phase.would_refuse": "Permulaan sekarang akan ditolak:",
  "p.phase.would_start": "Permulaan sekarang akan dilanjutkan.",
  "p.phase.in_queue": "Program ini berada di urutan {n} dalam antrean awal.",
  "p.phase.see_queue": "Lihat antrean awal",
  "p.phase.refusal_limit":
    "Seluruh {limit} tempat untuk program dalam pelaksanaan aktif sudah terisi ({in_delivery} dalam pelaksanaan).",
  "p.phase.refusal_pool":
    "Kolam kendala {pool} kekurangan pasokan pada {months}.",
  "p.phase.demand_heading": "Permintaan pasti program ini",
  "p.phase.demand_caption":
    "Ekuivalen waktu penuh menurut kolam keterampilan dan bulan, dari alokasi pasti pada program ini dan rencana di bawahnya",
  "p.phase.no_demand":
    "Tidak ada permintaan pasti pada program ini di kolam keterampilan dalam enam bulan ke depan.",
  "p.phase.col_pool": "Kolam keterampilan",
  "p.phase.check_again": "Periksa lagi",
  "p.phase.move_heading": "Ubah fase",
  "p.phase.f_to": "Pindah ke",
  "p.phase.forward_to": "Maju ke {phase}",
  "p.phase.back_to": "Mundur ke {phase}",
  "p.phase.back_needs_reason": "Perpindahan mundur memerlukan alasan.",
  "p.phase.f_reason":
    "Alasan (diperlukan untuk mundur; dicatat bersama perpindahan)",
  "p.phase.start_may_queue":
    "Pindah ke Pelaksanaan adalah awal program ini: jika pemeriksaan tidak menemukan ruang saat dikirim, permulaan ditolak dan program masuk antrean awal.",
  "p.phase.start_would_queue":
    "Permulaan ini akan ditolak. Jika dikirim tanpa alasan penggantian, program masuk antrean awal (atau mempertahankan tempatnya di sana).",
  "p.phase.f_override":
    "Alasan penggantian, untuk tetap memulai ({min} karakter atau lebih; dicatat dan diaudit)",
  "p.phase.refused_why": "Mengapa permulaan ditolak",
  "p.phase.move": "Pindahkan fase",
  "p.phase.moved": "Rencana sekarang berada pada fase {phase}.",
  "p.phase.started_override":
    "Program dimulai dengan penggantian: sekarang berada pada fase {phase}, dan penggantian dicatat.",
  "p.phase.start_refused_place":
    "Permulaan ditolak, jadi program berada di urutan {n} dalam antrean awal. Untuk tetap memulainya, berikan alasan penggantian.",
  "p.phase.start_refused":
    "Permulaan ditolak, dan program masuk antrean awal. Untuk tetap memulainya, berikan alasan penggantian.",
  "p.phase.could_not_move": "Fase tidak dapat dipindahkan",
  "p.phase.problem_choose": "Pilih fase tujuan.",
  "p.phase.problem_same":
    "Rencana sudah berada pada fase itu, jadi tidak ada yang dikirim.",
  "p.phase.problem_skip":
    "Perpindahan maju dilakukan satu fase pada satu waktu.",
  "p.phase.problem_reason": "Perpindahan mundur memerlukan alasan.",
  "p.phase.problem_override":
    "Alasan penggantian terdiri dari {min} karakter atau lebih.",
  "p.phase.refused_move":
    "Ditolak: perpindahan maju dilakukan satu fase pada satu waktu, perpindahan mundur memerlukan alasan, dan rencana tidak boleh sudah berada pada fase itu.",
  "p.phase.refused_gone": "Rencana itu sudah tidak ada.",
  "p.proposals.act_submit": "Ajukan",
  "p.proposals.act_review": "Mulai tinjauan",
  "p.proposals.act_approve": "Setujui",
  "p.proposals.act_reject": "Tolak",
  "p.proposals.act_submit_named": "Ajukan {title}",
  "p.proposals.act_review_named": "Mulai tinjauan {title}",
  "p.proposals.act_approve_named": "Setujui {title}",
  "p.proposals.act_reject_named": "Tolak {title}",
  "p.proposals.ask_rule":
    "Permintaan eksternal dari sponsor harus menyebutkan apa yang digantikannya (pertukaran sepanjang {min} karakter atau lebih) sebelum dapat disetujui.",
  "p.proposals.new_form": "Usulan baru",
  "p.proposals.f_external_ask": "Permintaan eksternal dari sponsor",
  "p.proposals.f_new_tradeoff":
    "Apa yang digantikannya (pertukaran, {min} sampai {max} karakter; dapat disebutkan nanti, sebelum persetujuan)",
  "p.proposals.col_tradeoff": "Permintaan eksternal dan pertukaran",
  "p.proposals.not_ask": "Bukan permintaan eksternal",
  "p.proposals.ask_with": "Permintaan eksternal. Pertukaran: {tradeoff}",
  "p.proposals.ask_without":
    "Permintaan eksternal. Belum ada pertukaran yang disebutkan.",
  "p.proposals.tradeoff_name": "Sebutkan pertukaran",
  "p.proposals.tradeoff_change": "Ubah pertukaran",
  "p.proposals.tradeoff_name_named": "Sebutkan pertukaran {title}",
  "p.proposals.tradeoff_change_named": "Ubah pertukaran {title}",
  "p.proposals.tradeoff_heading": "Pertukaran {title}",
  "p.proposals.tradeoff_says":
    "Sebutkan apa yang digantikan permintaan ini, dalam {min} sampai {max} karakter. Dapat disebutkan atau diubah sampai usulan disetujui atau ditolak, dan setiap perubahan dicatat.",
  "p.proposals.f_tradeoff":
    "Apa yang digantikannya ({min} sampai {max} karakter)",
  "p.proposals.tradeoff_save": "Simpan pertukaran",
  "p.proposals.tradeoff_close": "Tutup formulir pertukaran",
  "p.proposals.tradeoff_unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.proposals.tradeoff_saved": "Pertukaran disimpan untuk {title}.",
  "p.proposals.problem_title": "Usulan memerlukan judul.",
  "p.proposals.problem_tradeoff":
    "Pertukaran terdiri dari {min} sampai {max} karakter.",
  "p.proposals.problem_not_ask":
    "Hanya permintaan eksternal yang memiliki pertukaran.",
  "p.proposals.problem_decided":
    "Usulan sudah diputuskan, jadi pertukarannya tidak dapat diubah lagi.",
  "p.proposals.problem_approve":
    "{title} adalah permintaan eksternal: sebutkan pertukarannya ({min} karakter atau lebih) sebelum menyetujuinya.",
  "p.proposals.refused_create":
    "Ditolak: usulan memerlukan judul, jumlah uang memerlukan mata uang, dan pertukaran paling banyak {max} karakter.",
  "p.proposals.refused_tradeoff":
    "Ditolak: pertukaran terdiri dari {min} sampai {max} karakter, pada permintaan eksternal yang belum diputuskan.",
  "p.proposals.refused_step":
    "Ditolak: langkah itu tidak sesuai dengan status usulan.",
  "p.proposals.refused_gone": "Usulan itu sudah tidak ada.",
  "p.thresholds.opened_by_kind":
    "Dibuka karena program melewati ambangnya: {kind}.",
  "p.thresholds.joined_by_kind":
    "Selama ini terbuka, program juga melewati ambang: {kind}.",
  "p.deescalation.refused_tracker_taken":
    "Ditolak: kartu skor lain sudah menjadi yang dibaca pelacak.",
  "p.deescalation.level_now_by":
    "Tingkat {level}, ditetapkan pada {date} oleh {who}.",
  "p.deescalation.level_history_item_by":
    "{level}, ditetapkan pada {date} oleh {who}",
  "p.deescalation.evidence_recorded_by": "dicatat oleh {who} pada {date}",
  "p.deescalation.evidence_recorded_on": "dicatat pada {date}",
  "p.proposals.refused_approve_status":
    "Ditolak: hanya usulan yang sedang ditinjau yang dapat disetujui.",
  "p.proposals.refused_tradeoff_required":
    "Ditolak: permintaan eksternal memerlukan pertukaran bernama sepanjang {min} karakter atau lebih sebelum dapat disetujui.",
  "p.ledger.untraced_no_readable_line":
    "Tidak ada baris anggaran yang boleh Anda baca yang bersumber dari alokasi ini, jadi ke mana uangnya mengalir tidak dapat ditelusuri di sini.",
  "p.ledger.untraced_not_on_a_line":
    "Sebagian alokasi ini belum direncanakan pada baris anggaran mana pun.",
  "p.ledger.untraced_lines_over_plan":
    "Baris anggaran merencanakan lebih dari yang dimuat alokasi ini.",
  "p.ledger.untraced_other":
    "Sebagian uang alokasi ini tidak dapat ditelusuri.",
  "p.status.title_ppm": "Pembaruan status — PPM",
  "p.status.title": "Pembaruan status",
  "p.status.link": "Pembaruan status dan buktinya",
  "p.status.intro":
    "Pembaruan status adalah penilaian seseorang tentang bagaimana rencana berjalan (sesuai jalur, berisiko, atau di luar jalur), dengan apa yang terjadi dalam kata-kata. Tidak ada yang di sini menghitung skor.",
  "p.status.could_not_load": "Status rencana tidak dapat dimuat",
  "p.status.now_heading": "Posisi rencana",
  "p.status.no_update": "Belum ada pembaruan status yang diposting.",
  "p.status.cadence_is_one": "Pembaruan diharapkan setiap hari.",
  "p.status.cadence_is_other": "Pembaruan diharapkan setiap {n} hari.",
  "p.status.standing_overdue_one":
    "Pembaruan berikutnya jatuh tempo pada {date}: terlambat {n} hari.",
  "p.status.standing_overdue_other":
    "Pembaruan berikutnya jatuh tempo pada {date}: terlambat {n} hari.",
  "p.status.standing_due": "Pembaruan jatuh tempo hari ini, {date}.",
  "p.status.standing_next": "Pembaruan berikutnya jatuh tempo pada {date}.",
  "p.status.no_cadence":
    "Tidak ada irama yang ditetapkan, jadi pembaruan tidak pernah jatuh tempo atau terlambat.",
  "p.status.as_of": "Per {date}, dalam zona waktu penerapan.",
  "p.status.loading": "Memuat…",
  "p.status.posted_on_by": "diposting pada {date} oleh {who}",
  "p.status.posted_on": "diposting pada {date}",
  "p.status.claims_of": "Klaim dalam pembaruan {date}",
  "p.status.source_metric": "{metric}: sekarang {value}",
  "p.status.source_metric_unknown": "{metric}: tidak diketahui (bukan nol)",
  "p.status.source_unavailable": "Sumber yang tidak tersedia lagi",
  "p.status.kind_task": "Tugas: {title}",
  "p.status.kind_risk": "Risiko: {title}",
  "p.status.kind_milestone": "Pencapaian: {title}",
  "p.status.kind_issue": "Isu: {title}",
  "p.status.kind_other": "Catatan: {title}",
  "p.status.metric_overdue": "Tugas terlambat",
  "p.status.metric_due_soon": "Tugas segera jatuh tempo",
  "p.status.metric_blocked": "Tugas terblokir",
  "p.status.metric_no_due_date": "Tugas tanpa tenggat",
  "p.status.metric_def_overdue":
    "Tugas terbuka yang tenggatnya sebelum hari ini. Tugas yang jatuh tempo hari ini tidak terlambat. Tidak diketahui jika ada tugas terbuka dan tidak satu pun memiliki tenggat.",
  "p.status.metric_def_due_soon":
    "Tugas terbuka yang jatuh tempo hari ini atau dalam {days} hari ke depan. Tidak diketahui jika ada tugas terbuka dan tidak satu pun memiliki tenggat.",
  "p.status.metric_def_blocked": "Tugas terbuka yang statusnya terblokir.",
  "p.status.metric_def_no_due_date":
    "Tugas terbuka tanpa tenggat: tugas yang tidak dapat dilihat oleh angka lain.",
  "p.status.metrics_heading": "Cara angka dihitung",
  "p.status.metrics_says":
    "Atas tugas terbuka rencana (tugas yang statusnya belum selesai), dihitung saat pembaruan dibaca, bukan saat diposting.",
  "p.status.cadence_heading": "Seberapa sering pembaruan diharapkan",
  "p.status.cadence_says":
    "Setiap {min} sampai {max} hari. Menetapkannya memulai ulang jam dari hari ini, sehingga irama yang sama tidak dikirim lagi. Irama tidak dapat dihapus setelah ditetapkan.",
  "p.status.cadence_set_by": "Ditetapkan pada {date} oleh {who}.",
  "p.status.cadence_set_on": "Ditetapkan pada {date}.",
  "p.status.f_cadence": "Hari antar pembaruan ({min} sampai {max})",
  "p.status.save_cadence": "Simpan irama",
  "p.status.unchanged": "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.status.problem_cadence":
    "Irama adalah bilangan bulat hari dari {min} sampai {max}.",
  "p.status.cadence_saved_one":
    "Pembaruan sekarang diharapkan setiap hari; jam mulai hari ini.",
  "p.status.cadence_saved_other":
    "Pembaruan sekarang diharapkan setiap {n} hari; jam mulai hari ini.",
  "p.status.could_not_save": "Irama tidak dapat disimpan",
  "p.status.refused_gone": "Rencana itu sudah tidak ada.",
  "p.status.history_heading": "Setiap pembaruan, yang terbaru lebih dahulu",
  "p.status.history_says":
    "Pembaruan tidak pernah diubah atau dihapus. Setiap sumber ditampilkan sebagaimana adanya sekarang: catatan yang sudah hilang tidak tersedia lagi, dan setiap angka adalah nilainya hari ini.",
  "p.status.history_none": "Belum ada pembaruan.",
  "p.status.post_heading": "Posting pembaruan",
  "p.status.post_says":
    "Pilih penilaian dan jelaskan apa yang terjadi (hingga {max} karakter). Pembaruan tidak dapat diubah atau dihapus setelah diposting, dan memostingnya memulai ulang jam irama.",
  "p.status.claims_rule":
    "Anda dapat menambahkan hingga {claims} klaim, masing-masing satu kalimat hingga {chars} karakter yang mengutip hingga {sources} sumber: angka di bawah, atau tugas, risiko, pencapaian, dan isu rencana. Setiap sumber diperiksa saat pembaruan diposting.",
  "p.status.f_call": "Penilaian",
  "p.status.call_on_track": "Sesuai jalur",
  "p.status.call_at_risk": "Berisiko",
  "p.status.call_off_track": "Di luar jalur",
  "p.status.f_narrative": "Apa yang terjadi (hingga {max} karakter)",
  "p.status.claim_legend": "Klaim {n}",
  "p.status.f_claim_text": "Klaim (hingga {chars} karakter)",
  "p.status.f_metrics": "Angka yang dikutipnya",
  "p.status.f_record": "Catatan rencana yang dikutip klaim {n}",
  "p.status.choose_record": "Pilih catatan",
  "p.status.add_record": "Tambah catatan ke klaim {n}",
  "p.status.claim_records": "Catatan yang dikutip klaim {n}",
  "p.status.remove": "Hapus",
  "p.status.remove_source": "Hapus {source} dari klaim {n}",
  "p.status.remove_claim": "Hapus klaim {n}",
  "p.status.add_claim": "Tambah klaim",
  "p.status.post": "Posting pembaruan",
  "p.status.posted": "Pembaruan diposting: {call}.",
  "p.status.could_not_post": "Pembaruan tidak dapat diposting",
  "p.status.problem_call": "Pilih sesuai jalur, berisiko, atau di luar jalur.",
  "p.status.problem_narrative":
    "Jelaskan apa yang terjadi, dalam hingga {max} karakter.",
  "p.status.problem_claims": "Pembaruan memuat paling banyak {claims} klaim.",
  "p.status.problem_claim_text":
    "Klaim adalah satu kalimat hingga {chars} karakter.",
  "p.status.problem_sources": "Klaim mengutip paling banyak {sources} sumber.",
  "p.status.problem_in_claim": "Klaim {n}: {problem}",
  "p.status.refused_update":
    "Ditolak: sumber mungkin bukan lagi catatan rencana ini. Tidak ada yang diposting; periksa klaim dan posting lagi.",
  "p.status.due_heading": "Pembaruan status yang jatuh tempo",
  "p.status.due_none":
    "Tidak ada rencana yang dapat Anda baca yang pembaruan statusnya jatuh tempo.",
  "p.status.due_could_not_load":
    "Pembaruan status yang jatuh tempo tidak dapat dimuat",
  "p.status.a_plan": "Sebuah rencana",
  "p.status.due_last": "Pembaruan terakhir: {call}, pada {date}.",
  "p.status.due_never": "Belum ada pembaruan.",
  "p.status.group_task": "Tugas",
  "p.status.group_risk": "Risiko",
  "p.status.group_milestone": "Pencapaian",
  "p.status.group_issue": "Isu",
  "l.enum.risk_status.open": "Terbuka",
  "l.enum.risk_status.mitigating": "Dimitigasi",
  "l.enum.risk_status.closed": "Ditutup",
  "l.enum.risk_status.materialized": "Terjadi",
  "p.risk_edit.change": "Ubah",
  "p.risk_edit.change_named": "Ubah: {title}",
  "p.risk_edit.heading": "Ubah risiko “{title}”",
  "p.risk_edit.says":
    "Kemungkinan dan dampak berupa bilangan bulat dari {min} sampai {max}; eksposur adalah hasil kalinya. Mitigasi paling banyak {max_bytes} byte (lebih sedikit karakter pada beberapa aksara). Tanggal tinjauan dapat diubah tetapi tidak dapat dihapus. Hanya yang Anda ubah yang dikirim.",
  "p.risk_edit.says_escalate":
    "Untuk mencatat bahwa risiko telah terjadi, gunakan Eskalasi di tabel, yang juga mencatat kapan.",
  "p.risk_edit.problem_status": "Pilih status.",
  "p.risk_edit.problem_score":
    "Kemungkinan dan dampak masing-masing harus berupa bilangan bulat dari {min} sampai {max}.",
  "p.risk_edit.problem_mitigation":
    "Mitigasi lebih panjang dari {max_bytes} byte.",
  "p.risk_edit.problem_review_cleared":
    "Tanggal tinjauan tidak dapat dihapus setelah ditetapkan; pilih tanggal lain.",
  "p.risk_edit.problem_review_date": "Tanggal tinjauan bukan tanggal.",
  "p.risk_edit.unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.risk_edit.saved": "Risiko “{title}” disimpan.",
  "p.risk_edit.could_not_save": "Risiko tidak dapat disimpan",
  "p.risk_edit.refused_gone":
    "Risiko itu sudah tidak ada di rencana ini; daftar telah dibaca ulang.",
  "p.risk_edit.refused_invalid":
    "Ditolak: layanan tidak menerima status, skor, atau mitigasi. Tidak ada yang diubah.",
  "p.risk_edit.f_status": "Status risiko",
  "p.risk_edit.f_probability": "Kemungkinan ({min} sampai {max})",
  "p.risk_edit.f_impact": "Dampak ({min} sampai {max})",
  "p.risk_edit.f_mitigation": "Mitigasi",
  "p.risk_edit.f_review_date": "Tanggal tinjauan",
  "p.risk_edit.save": "Simpan risiko",
  "p.risk_edit.close": "Tutup formulir",
  "p.risk_edit.f_reason": "Mengapa skor berubah (opsional)",
  "p.risk_edit.history_h": "Riwayat skor",
  "p.risk_edit.history_row":
    "{date}: kemungkinan {probability} × dampak {impact} = {score}.",
  "p.risk_edit.history_reason": "Alasan: {reason}",
  "p.risk_edit.history_first":
    "Skor pertama yang diketahui; riwayat dimulai di sini.",
  "p.plan_issues.refused_gone":
    "Isu itu sudah tidak ada di rencana ini; daftar telah dibaca ulang.",
  "p.plan_issues.refused_remove":
    "Ditolak: Anda tidak boleh menghapus isu pada rencana ini. Tidak ada yang dihapus.",
  "p.plan_issues.refused_remove_admin":
    "Ditolak: hanya administrator yang dapat menghapus isu di bawah alur kerja jenis ini. Tidak ada yang dihapus.",
  "p.plan_issues.remove_admin_only":
    "Hanya administrator yang dapat menghapus “{title}”: alur kerja jenisnya mengatur perpindahan atau kolomnya. Tidak ada yang dikirim.",
  "p.plan_issues.confirm_remove":
    "Hapus isu “{title}”? Isu keluar dari setiap daftar; catatannya tetap disimpan untuk audit.",
  "p.plan_issues.removed": "Isu “{title}” telah dihapus.",
  "p.plan_issues.remove": "Hapus",
  "p.plan_issues.remove_named": "Hapus: {title}",
  "p.plan_issues.remove_says":
    "Menghapus isu mengeluarkannya dari setiap daftar dan tidak dapat dibatalkan di sini; audit menyimpan catatannya. Di bawah alur kerja yang mengatur perpindahan atau kolom, hanya administrator yang dapat menghapusnya.",
  "p.plan_issues.raised_on_by": "Diangkat pada {date} oleh {who}.",
  "p.plan_issues.raised_on": "Diangkat pada {date}.",
  "p.plan_issues.from_risk_named": "Diangkat dari risiko “{title}”.",
  "p.plan_issues.from_a_risk":
    "Diangkat dari risiko yang tidak tercantum lagi.",
  "p.plan_issues.from_a_task": "Diangkat dari tugas terblokir (buka tugas)",
  "p.plan_issues.answered_by_change":
    "Dijawab oleh permintaan perubahan (lihat permintaan perubahan rencana)",
  "p.plan_issues.resolved_on": "Diselesaikan pada {date}.",
  "p.task.issue_heading": "Isu dari tugas ini",
  "p.task.issue_says":
    "Hanya tugas terblokir yang dapat diangkat sebagai isu, dan hanya sekali selama isu itu terbuka. Isu ini adalah penghambat dengan tingkat keparahan sedang, dimiliki oleh penerima tugas; tugasnya sendiri tidak diubah.",
  "p.task.issue_raised": "Isu telah diangkat.",
  "p.task.issue_open_list": "Lihat isu rencana",
  "p.task.issue_exists":
    "Isu terbuka telah diangkat dari tugas ini: “{title}”.",
  "p.task.issue_not_blocked":
    "Tugas ini tidak terblokir, jadi tidak dapat diangkat sebagai isu. Tandai terblokir di papan terlebih dahulu.",
  "p.task.issue_raise": "Angkat isu dari tugas ini",
  "p.task.refused_from_task_not_blocked":
    "Ditolak: hanya tugas terblokir yang dapat diangkat sebagai isu, dan tugas ini sudah tidak terblokir. Tugas telah dibaca ulang.",
  "p.task.refused_from_task_exists":
    "Ditolak: isu terbuka sudah diangkat dari tugas ini. Isu telah dibaca ulang.",
  "p.task.refused_from_task_forbidden":
    "Ditolak: Anda tidak boleh mengangkat isu pada rencana ini.",
  "p.task.refused_from_task_gone": "Tugas itu sudah tidak ada di rencana ini.",
  "p.plan_change_requests.refused_gone":
    "Permintaan perubahan itu sudah tidak ada di rencana ini; daftar telah dibaca ulang.",
  "p.plan_change_requests.refused_links_conflict":
    "Ditolak: permintaan telah ditutup, atau salah satu isu sekarang dijawab oleh permintaan perubahan lain. Permintaan dan isu rencana telah dibaca ulang.",
  "p.plan_change_requests.refused_links_invalid":
    "Ditolak: isu atau risiko yang dipilih sudah tidak ada di rencana ini. Isu dan risiko rencana telah dibaca ulang.",
  "p.plan_change_requests.refused_preview":
    "Ditolak: permintaan ini tidak membawa suntingan bagaimana-jika yang dapat dibaca penjadwal, jadi tidak ada ramalan.",
  "p.plan_change_requests.refused_baseline":
    "Ditolak: garis dasar sudah diambil untuk perubahan ini, atau perubahan tidak lagi disetujui. Permintaan telah dibaca ulang.",
  "p.plan_change_requests.problem_links_too_many":
    "Pilih paling banyak {max} isu dan {max} risiko.",
  "p.plan_change_requests.problem_links_elsewhere":
    "“{title}” sudah dijawab oleh permintaan perubahan lain; hapus centangnya.",
  "p.plan_change_requests.links_unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.plan_change_requests.links_saved":
    "Isu dan risiko yang dijawab perubahan ini telah disimpan.",
  "p.plan_change_requests.confirm_baseline":
    "Ambil garis dasar jadwal karena “{title}”? Hanya satu yang dapat diambil untuk perubahan ini, dan tidak dapat dibatalkan.",
  "p.plan_change_requests.baseline_taken":
    "Garis dasar versi {version} diambil karena perubahan ini.",
  "p.plan_change_requests.stated_impact":
    "Dampak yang dinyatakan pemohon: {impact}.",
  "p.plan_change_requests.implemented_on": "Diimplementasikan pada {date}.",
  "p.plan_change_requests.implement": "Tandai sebagai diimplementasikan",
  "p.plan_change_requests.implemented":
    "Perubahan ditandai sebagai diimplementasikan.",
  "p.plan_change_requests.links_heading":
    "Isu dan risiko yang dijawab perubahan ini",
  "p.plan_change_requests.links_none":
    "Belum menjawab isu atau risiko apa pun.",
  "p.plan_change_requests.answers_issue": "Isu: {title}",
  "p.plan_change_requests.answers_risk": "Risiko: {title}",
  "p.plan_change_requests.an_issue": "isu yang tidak tercantum lagi",
  "p.plan_change_requests.a_risk": "risiko yang tidak tercantum lagi",
  "p.plan_change_requests.links_says":
    "Dipilih selama permintaan terbuka (draf, diajukan, ditinjau, atau disetujui). Satu isu menjawab paling banyak satu permintaan perubahan; hingga {max} isu dan {max} risiko. Menyimpan mengganti seluruh kumpulan.",
  "p.plan_change_requests.links_issues": "Isu rencana ini",
  "p.plan_change_requests.issue_elsewhere":
    "{title} (menjawab permintaan perubahan lain)",
  "p.plan_change_requests.links_no_issues": "Rencana ini tidak memiliki isu.",
  "p.plan_change_requests.links_risks": "Risiko rencana ini",
  "p.plan_change_requests.links_no_risks": "Rencana ini tidak memiliki risiko.",
  "p.plan_change_requests.links_save": "Simpan yang dijawabnya",
  "p.plan_change_requests.links_closed":
    "Permintaan yang sudah ditutup tidak dapat ditautkan ulang.",
  "p.plan_change_requests.forecast_heading": "Ramalan bagaimana-jika",
  "p.plan_change_requests.forecast_says":
    "Ramalan dari jadwal saat ini dengan suntingan bagaimana-jika permintaan ini diterapkan. Membacanya tidak mengubah apa pun; dampak yang dinyatakan di atas adalah angka pemohon sendiri.",
  "p.plan_change_requests.forecast_show": "Tampilkan ramalan",
  "p.plan_change_requests.forecast_label": "Ramalan — tidak ada yang diubah",
  "p.plan_change_requests.forecast_finish":
    "Selesai rencana: {before} sekarang, {after} dengan perubahan.",
  "p.plan_change_requests.forecast_finish_unknown":
    "Waktu selesai rencana tidak dapat diramalkan.",
  "p.plan_change_requests.forecast_moved_one":
    "1 tugas akan selesai pada hari yang berbeda.",
  "p.plan_change_requests.forecast_moved_other":
    "{n} tugas akan selesai pada hari yang berbeda.",
  "p.plan_change_requests.forecast_later_one":
    "{title}: selesai 1 hari kerja lebih lambat",
  "p.plan_change_requests.forecast_later_other":
    "{title}: selesai {n} hari kerja lebih lambat",
  "p.plan_change_requests.forecast_earlier_one":
    "{title}: selesai 1 hari kerja lebih awal",
  "p.plan_change_requests.forecast_earlier_other":
    "{title}: selesai {n} hari kerja lebih awal",
  "p.plan_change_requests.a_task": "Sebuah tugas",
  "p.plan_change_requests.forecast_labour":
    "Biaya tenaga kerja terencana akan berubah sebesar {amount}.",
  "p.plan_change_requests.forecast_labour_hidden":
    "Ramalan biaya tenaga kerja hanya ditampilkan kepada orang yang boleh melihat tarif biaya.",
  "p.plan_change_requests.forecast_labour_none":
    "Tidak ada pekerjaan yang dibiayai, jadi tidak ada ramalan biaya tenaga kerja.",
  "p.plan_change_requests.forecast_labour_unknown":
    "Tidak ada ramalan biaya tenaga kerja.",
  "p.plan_change_requests.forecast_no_edits":
    "Permintaan ini tidak membawa suntingan bagaimana-jika, jadi tidak ada ramalan; dampak yang dinyatakannya adalah angka pemohon sendiri.",
  "p.plan_change_requests.baseline_heading": "Garis dasar karena perubahan ini",
  "p.plan_change_requests.baseline_done":
    "Garis dasar jadwal telah diambil karena perubahan ini.",
  "p.plan_change_requests.baseline_says":
    "Mengambilnya adalah pilihan seseorang; tidak ada yang diubah garis dasarnya secara otomatis. Satu garis dasar per perubahan, alasannya menyebut permintaan, dan tidak dapat dibatalkan.",
  "p.plan_change_requests.baseline_take": "Ambil garis dasar",
  "p.plan_change_requests.baseline_not_yet":
    "Garis dasar dapat diambil setelah perubahan disetujui.",
  "p.escalations.problem_outcome":
    "Pilih hasil: diputuskan, panduan diberikan, atau ditolak.",
  "p.escalations.problem_response":
    "Tulis tanggapan sepanjang {min} sampai {max} karakter.",
  "p.escalations.refused_ack_right":
    "Ditolak: mengakui eskalasi memerlukan hak untuk mengakui eskalasi (administrator memegangnya). Tidak ada yang dicatat.",
  "p.escalations.refused_resolve_right":
    "Ditolak: menyelesaikan eskalasi memerlukan hak untuk menyelesaikan eskalasi (administrator memegangnya). Tidak ada yang dicatat.",
  "p.escalations.refused_ack_state":
    "Ditolak: eskalasi ini telah diselesaikan atau ditarik sementara itu, jadi tidak dapat diakui. Eskalasi telah dibaca ulang.",
  "p.escalations.refused_resolve_state":
    "Ditolak: eskalasi ini telah ditarik, atau sudah diselesaikan dengan hasil atau tanggapan yang berbeda; penyelesaian tidak pernah ditimpa. Eskalasi telah dibaca ulang.",
  "p.escalations.refused_resolve_invalid":
    "Ditolak: hasil atau tanggapan tidak diterima. Periksa aturan di atas dan coba lagi.",
  "p.escalations.refused_gone":
    "Eskalasi ini sudah tidak ada, atau Anda tidak dapat melihatnya lagi. Daftar telah dibaca ulang.",
  "p.escalations.could_not_save": "Itu tidak dicatat.",
  "p.escalations.by_unrecorded": "seseorang yang tidak tercatat",
  "p.escalations.ack_record": "Diakui pada tingkat {level} oleh {by}, {at}.",
  "p.escalations.resolution_record": "Diselesaikan {at} oleh {by}: {outcome}.",
  "p.escalations.outcome_unrecorded": "tidak ada hasil yang dicatat",
  "p.escalations.records_heading": "Pengakuan dan penyelesaian",
  "p.escalations.records_failed":
    "Pengakuan atau penyelesaian tidak dapat dibaca; riwayat di bawah tetap menunjukkan apa yang terjadi.",
  "p.escalations.ack_note_line": "Catatan: {note}",
  "p.escalations.ack_none": "Belum diakui pada tingkatnya saat ini.",
  "p.escalations.resolution_kept":
    "Penyelesaian adalah catatan: tidak diubah atau ditimpa.",
  "p.escalations.records_rules":
    "Mengakui memerlukan hak untuk mengakui eskalasi, dan menyelesaikan memerlukan hak untuk menyelesaikannya; administrator memegang keduanya, dan halaman ini tidak dapat mengetahui sebelumnya apakah Anda memilikinya. Pengakuan berlaku pada tingkat saat ini: menaikkannya satu tingkat menghapusnya. Penyelesaian adalah catatan dan tidak pernah ditimpa.",
  "p.escalations.ack_heading": "Akui eskalasi ini",
  "p.escalations.ack_note": "Catatan (opsional)",
  "p.escalations.ack_done": "Diakui; catatannya ditampilkan di bawah.",
  "p.escalations.resolve_heading": "Selesaikan eskalasi ini",
  "p.escalations.resolve_says":
    "Tanggapan terdiri dari {min} sampai {max} karakter. Anda ditanya sebelum dicatat, dan tidak dapat diubah sesudahnya.",
  "p.escalations.confirm_resolve":
    "Catat penyelesaian “{title}” ini? Tidak dapat diubah atau ditimpa sesudahnya.",
  "p.escalations.resolution_done":
    "Diselesaikan; catatannya ditampilkan di bawah.",
  "p.register_completeness.link": "Kelengkapan registri",
  "p.register_completeness.title": "Kelengkapan registri",
  "p.register_completeness.title_ppm": "Kelengkapan registri — PPM",
  "p.register_completeness.intro":
    "Seberapa lengkap registri setiap program: dari hal-hal yang seharusnya tercatat pada sebuah program, porsi yang dimilikinya, dan setiap yang hilang disebutkan. Hanya program (rencana berjenis Program) yang dapat Anda baca yang tercantum.",
  "p.register_completeness.rules":
    "Kesepuluhnya: pemilik penanggung jawab senior (pimpinan program dianggap mewakilinya), mitra, tahap, garis dasar jadwal, tanggal target, anggaran, sumber pendanaan pada setiap baris anggaran, penilaian keyakinan pelaksanaan, risiko, dan ketergantungan. Mitra hanya diminta di mana model pelaksanaan memiliki mitra, dan sumber pendanaan hanya di mana ada baris anggaran. Program dan rencana di bawahnya dihitung bersama.",
  "p.register_completeness.switch_on":
    "“Tanpa registri, tanpa uang” aktif: melepaskan baris anggaran atau memesan alokasi pasti pada program yang tidak lengkap ditolak, dengan menyebut apa yang hilang.",
  "p.register_completeness.switch_off":
    "“Tanpa registri, tanpa uang” nonaktif: melepaskan baris anggaran atau memesan alokasi pasti pada program yang tidak lengkap dilanjutkan, dan peringatannya disimpan dalam jejak audit.",
  "p.register_completeness.count_one": "{complete} dari {n} program lengkap.",
  "p.register_completeness.count_other": "{complete} dari {n} program lengkap.",
  "p.register_completeness.none": "Tidak ada program yang dapat Anda baca.",
  "p.register_completeness.loading": "Memuat…",
  "p.register_completeness.could_not_load":
    "Kelengkapan registri tidak dapat dimuat.",
  "p.register_completeness.table": "Program dan registrinya",
  "p.register_completeness.col_programme": "Program",
  "p.register_completeness.col_state": "Status",
  "p.register_completeness.col_score": "Porsi yang tercatat",
  "p.register_completeness.col_on_record": "Kolom yang tercatat",
  "p.register_completeness.col_missing": "Hilang",
  "p.register_completeness.col_not_asked": "Tidak diminta",
  "p.register_completeness.of": "{present} dari {applicable}",
  "p.register_completeness.state_complete": "lengkap",
  "p.register_completeness.state_incomplete": "tidak lengkap",
  "p.register_completeness.field_other": "kolom lain",
  "p.register_completeness.none_listed": "tidak ada",
  "p.meetings.title": "Rapat",
  "p.meetings.title_ppm": "Rapat — PPM",
  "p.meetings.link": "Rapat, notulen, dan tindakan",
  "p.meetings.intro":
    "Rapat direncanakan dengan judul, tanggal, dan agenda. Setelah itu dicatat satu kali sebagai telah berlangsung, dengan notulen dan hasilnya (keputusan, tindakan, dan catatan), atau dibatalkan. Rapat yang telah berlangsung atau dibatalkan adalah catatan: notulen yang telah dicatat tidak dapat dikoreksi dan rapat yang dibatalkan tidak dapat dipulihkan. Peserta tidak dicatat.",
  "p.meetings.plan_heading": "Rencanakan rapat",
  "p.meetings.plan_says":
    "Judul hingga {title} karakter dan tanggal. Agenda memuat hingga {items} butir masing-masing hingga {chars} karakter; sebuah butir dapat menunjuk ke tugas, risiko, pencapaian, atau isu rencana ini.",
  "p.meetings.f_title": "Judul (hingga {max} karakter)",
  "p.meetings.f_date": "Tanggal yang direncanakan",
  "p.meetings.agenda_item": "Butir agenda {n}",
  "p.meetings.f_item_text":
    "Tentang apa butir agenda {n} (hingga {max} karakter)",
  "p.meetings.f_item_link": "Catatan yang menjadi perhatian butir agenda {n}",
  "p.meetings.no_record": "Tanpa catatan",
  "p.meetings.records_loading": "Memuat catatan rencana…",
  "p.meetings.remove_item": "Hapus butir agenda {n}",
  "p.meetings.add_item": "Tambah butir agenda",
  "p.meetings.plan": "Rencanakan rapat",
  "p.meetings.planned": "Direncanakan: {title}, pada {date}.",
  "p.meetings.list_heading": "Rapat rencana, yang terbaru lebih dahulu",
  "p.meetings.pdf_says":
    "Setiap rapat diunduh sebagai dokumen PDF yang memuat apa yang ditampilkan halaman ini; kata-kata tetapnya (judul dan status) dalam bahasa Inggris.",
  "p.meetings.loading": "Memuat rapat…",
  "p.meetings.none": "Belum ada rapat.",
  "p.meetings.status_planned": "direncanakan",
  "p.meetings.status_held": "telah berlangsung",
  "p.meetings.status_cancelled": "dibatalkan",
  "p.meetings.held_on": "berlangsung pada {date}",
  "p.meetings.scheduled_on": "direncanakan untuk {date}",
  "p.meetings.show": "Buka rapat {title}",
  "p.meetings.hide": "Tutup rapat {title}",
  "p.meetings.pdf": "Unduh sebagai PDF",
  "p.meetings.pdf_named": "Unduh rapat {title} sebagai PDF",
  "p.meetings.agenda_heading": "Agenda rapat",
  "p.meetings.agenda_none": "Tidak ada butir agenda.",
  "p.meetings.minutes_heading": "Notulen",
  "p.meetings.outcomes_heading": "Hasil",
  "p.meetings.outcomes_none": "Tidak ada hasil yang dicatat.",
  "p.meetings.outcome_decision": "Keputusan",
  "p.meetings.outcome_action": "Tindakan",
  "p.meetings.outcome_note": "Catatan",
  "p.meetings.about": "tentang {record}",
  "p.meetings.link_gone": "tentang catatan yang sudah tidak ada",
  "p.meetings.task_made": "tugas yang dibuat dari hasil {n}",
  "p.meetings.held_kept":
    "Rapat ini adalah catatan: notulen dan hasilnya tidak diubah.",
  "p.meetings.cancelled_kept":
    "Rapat ini dibatalkan. Ini adalah catatan dan tidak dapat dipulihkan.",
  "p.meetings.held_heading": "Catat {title} sebagai telah berlangsung",
  "p.meetings.held_says":
    "Notulen hingga {minutes} karakter, dan hingga {items} hasil masing-masing hingga {chars} karakter: masing-masing berupa keputusan, tindakan, atau catatan, dan masing-masing dapat menunjuk ke catatan rencana ini. Tanggal berlangsung adalah tanggal yang direncanakan kecuali Anda memberikan yang lain. Ini dilakukan satu kali: notulen tidak dapat dikoreksi sesudahnya.",
  "p.meetings.f_minutes": "Notulen (hingga {max} karakter)",
  "p.meetings.f_held_on":
    "Tanggal berlangsung (kosongkan untuk tanggal yang direncanakan)",
  "p.meetings.outcome_n": "Hasil {n}",
  "p.meetings.f_outcome_kind": "Hasil {n} adalah",
  "p.meetings.choose_kind": "Pilih…",
  "p.meetings.f_outcome_text":
    "Hasil {n} dalam kata-kata (hingga {max} karakter)",
  "p.meetings.f_outcome_link": "Catatan yang menjadi perhatian hasil {n}",
  "p.meetings.remove_outcome": "Hapus hasil {n}",
  "p.meetings.add_outcome": "Tambah hasil",
  "p.meetings.record_held": "Catat sebagai telah berlangsung",
  "p.meetings.cancel_meeting": "Batalkan rapat {title}",
  "p.meetings.confirm_held":
    "Catat {title} sebagai telah berlangsung? Notulen dan hasilnya tidak dapat dikoreksi sesudahnya.",
  "p.meetings.confirm_cancel":
    "Batalkan {title}? Rapat yang dibatalkan tidak dapat dipulihkan.",
  "p.meetings.held_done": "Dicatat sebagai telah berlangsung: {title}.",
  "p.meetings.cancelled_done": "Dibatalkan: {title}.",
  "p.meetings.problem_title": "Berikan judul hingga {title} karakter.",
  "p.meetings.problem_date": "Berikan tanggal.",
  "p.meetings.problem_agenda_count":
    "Agenda memuat paling banyak {items} butir.",
  "p.meetings.problem_outcome_count":
    "Rapat memuat paling banyak {items} hasil.",
  "p.meetings.problem_item_text": "Tulis dalam hingga {chars} karakter.",
  "p.meetings.problem_minutes":
    "Tulis notulen, dalam hingga {minutes} karakter.",
  "p.meetings.problem_outcome_kind": "Pilih keputusan, tindakan, atau catatan.",
  "p.meetings.problem_on_line": "Untuk {line}: {problem}",
  "p.meetings.could_not_load": "Rapat tidak dapat dimuat.",
  "p.meetings.could_not_save": "Rapat tidak diubah.",
  "p.meetings.refused_gone":
    "Ditolak: rapat ini, atau rencananya, sudah tidak ada. Daftar telah dibaca ulang.",
  "p.meetings.refused_settled":
    "Ditolak: rapat ini sudah berlangsung atau dibatalkan, jadi merupakan catatan dan tidak diubah. Rapat telah dibaca ulang.",
  "p.meetings.refused_link":
    "Ditolak: catatan yang ditunjuk sebuah butir bukan lagi catatan rencana ini. Tidak ada yang disimpan; pilih catatan lagi.",
  "p.meetings.refused_not_held":
    "Ditolak: tugas hanya diusulkan dari rapat yang telah berlangsung dan memiliki notulen.",
  "p.meetings.refused_commit_state":
    "Ditolak: rapat belum berlangsung, atau salah satu tindakan ini sementara itu sudah memiliki tugas. Tidak ada yang dibuat; rapat telah dibaca ulang, jadi periksa lagi.",
  "p.meetings.refused_commit_invalid":
    "Ditolak: sebuah usulan tidak lagi siap. Tidak ada yang dibuat; periksa usulan lagi.",
  "p.meetings.refused_proposals": "Ditolak: kirim antara 1 dan {count} usulan.",
  "p.meetings.tasks_heading": "Tugas dari tindakan",
  "p.meetings.tasks_says":
    "Untuk setiap tindakan yang belum memiliki tugas Anda dapat mengusulkan satu: judul hingga {max} karakter dan kutipan notulen yang mendukungnya, minimal {min} karakter (huruf besar dan spasi diabaikan). Penerima tugas bersifat opsional dan harus sudah ada di tugas rencana ini atau bagan tanggung jawabnya, sebagai rujukan seperti person:…; tenggat bersifat opsional, dari {first} sampai {last}; poin berkisar dari 0 sampai {points}. Hingga {count} sekaligus. Pemeriksaan tidak menulis apa pun; tugas kemudian dibuat semuanya atau tidak sama sekali, dan satu tindakan hanya mendapat satu tugas.",
  "p.meetings.proposal_legend": "Tindakan {n}: {text}",
  "p.meetings.f_propose": "Usulkan tugas untuk tindakan {n}",
  "p.meetings.f_task_title":
    "Judul tugas untuk tindakan {n} (hingga {max} karakter)",
  "p.meetings.f_excerpt":
    "Kutipan notulen untuk tindakan {n} (minimal {min} karakter)",
  "p.meetings.f_assignee":
    "Penerima tugas untuk tindakan {n} (rujukan opsional)",
  "p.meetings.f_due": "Tenggat untuk tindakan {n} (opsional)",
  "p.meetings.f_points": "Poin untuk tindakan {n} (opsional, 0 sampai {max})",
  "p.meetings.check": "Periksa usulan",
  "p.meetings.create_tasks": "Buat tugas",
  "p.meetings.none_chosen":
    "Tidak ada usulan yang dipilih, jadi tidak ada yang dikirim.",
  "p.meetings.checked_one":
    "Diperiksa, tidak ada yang ditulis: {n} usulan siap, {dropped} tidak.",
  "p.meetings.checked_other":
    "Diperiksa, tidak ada yang ditulis: {n} usulan siap, {dropped} tidak.",
  "p.meetings.ready": "Siap, tanpa penerima tugas.",
  "p.meetings.ready_for": "Siap, untuk {who}.",
  "p.meetings.not_all_ready":
    "Setiap usulan yang dipilih harus siap sebelum tugas dapat dibuat: ubah atau hapus centang yang belum siap, lalu periksa lagi.",
  "p.meetings.check_again": "Usulan berubah sejak diperiksa: periksa lagi.",
  "p.meetings.confirm_tasks_one":
    "Buat {n} tugas dari tindakan {title}? Satu tindakan hanya mendapat satu tugas.",
  "p.meetings.confirm_tasks_other":
    "Buat {n} tugas dari tindakan {title}? Satu tindakan hanya mendapat satu tugas.",
  "p.meetings.tasks_made_one":
    "{n} tugas dibuat; ditautkan di bawah tindakannya.",
  "p.meetings.tasks_made_other":
    "{n} tugas dibuat; masing-masing ditautkan di bawah tindakannya.",
  "p.meetings.could_not_check": "Usulan tidak dapat diperiksa.",
  "p.meetings.could_not_create": "Tugas tidak dibuat.",
  "p.meetings.dropped_title_invalid":
    "Belum siap: judul kosong atau terlalu panjang.",
  "p.meetings.dropped_excerpt_not_in_minutes":
    "Belum siap: kutipan tidak ditemukan dalam notulen, atau terlalu pendek.",
  "p.meetings.dropped_outcome_not_found":
    "Belum siap: rapat tidak memiliki hasil seperti itu.",
  "p.meetings.dropped_outcome_not_an_action":
    "Belum siap: hasil itu bukan tindakan.",
  "p.meetings.dropped_outcome_already_has_a_task":
    "Belum siap: tindakan itu sudah memiliki tugas.",
  "p.meetings.dropped_due_outside_window":
    "Belum siap: tenggat berada di luar tanggal yang diizinkan.",
  "p.meetings.dropped_points_invalid":
    "Belum siap: poin berada di luar rentang yang diizinkan.",
  "p.meetings.dropped_assignee_not_in_plan":
    "Belum siap: penerima tugas bukan salah satu orang rencana ini (pada tugas atau bagan tanggung jawabnya).",
  "p.meetings.dropped_outcome_proposed_twice":
    "Belum siap: tindakan yang sama diusulkan dua kali.",
  "p.meetings.dropped_other":
    "Belum siap, karena alasan yang tidak diketahui halaman ini.",
  "p.meetings.problem_proposals_count":
    "Usulkan paling banyak {count} tugas sekaligus.",
  "p.meetings.problem_task_title": "Berikan tugas judul hingga {max} karakter.",
  "p.meetings.problem_excerpt":
    "Kutip bagian minimal {min} karakter yang ada dalam notulen.",
  "p.meetings.problem_due":
    "Tenggat harus jatuh dari seminggu sebelum rapat sampai setahun sesudahnya.",
  "p.meetings.problem_points":
    "Poin adalah bilangan bulat dari 0 sampai {points}.",
  "p.lessons.title": "Pelajaran yang dipetik",
  "p.lessons.title_ppm": "Pelajaran yang dipetik — PPM",
  "p.lessons.link": "Pelajaran yang dipetik",
  "p.lessons.intro":
    "Pelajaran dicatat pada gerbang fase atau pada penutupan rencana. Pelajaran tidak pernah diubah: hanya dapat ditarik, dan pelajaran yang ditarik tidak ditampilkan lagi (tetap ada dalam jejak audit). Usulan baru yang judulnya mirip dengan rencana ini akan menampilkan pelajarannya.",
  "p.lessons.record_heading": "Catat pelajaran",
  "p.lessons.record_says":
    "Kapan (gerbang atau penutupan), apa itu, ringkasan hingga {max} karakter dan, jika Anda mau, rincian hingga {bytes} byte (huruf beraksen atau aksara lain memakai lebih dari satu) serta tugas, risiko, pencapaian, atau isu rencana ini yang menjadi pokoknya.",
  "p.lessons.f_occasion": "Kapan",
  "p.lessons.choose_occasion": "Pilih gerbang atau penutupan…",
  "p.lessons.closure": "Penutupan",
  "p.lessons.occasion_other": "Kesempatan lain",
  "p.lessons.f_category": "Apa itu",
  "p.lessons.category_went_well": "Berjalan baik",
  "p.lessons.category_went_badly": "Berjalan buruk",
  "p.lessons.category_recommendation": "Rekomendasi",
  "p.lessons.category_other": "Jenis lain",
  "p.lessons.f_summary": "Ringkasan (hingga {max} karakter)",
  "p.lessons.f_detail": "Rincian (opsional, hingga {bytes} byte)",
  "p.lessons.f_link": "Catatan yang menjadi pokoknya (opsional)",
  "p.lessons.record": "Catat pelajaran",
  "p.lessons.recorded": "Dicatat: {summary}",
  "p.lessons.problem_occasion": "Pilih gerbang atau penutupan.",
  "p.lessons.problem_category":
    "Pilih berjalan baik, berjalan buruk, atau rekomendasi.",
  "p.lessons.problem_summary": "Tulis ringkasan hingga {max} karakter.",
  "p.lessons.problem_detail": "Rincian lebih panjang dari {bytes} byte.",
  "p.lessons.list_heading": "Pelajaran rencana, yang tertua lebih dahulu",
  "p.lessons.loading": "Memuat pelajaran…",
  "p.lessons.none": "Belum ada pelajaran.",
  "p.lessons.count_one": "{n} pelajaran.",
  "p.lessons.count_other": "{n} pelajaran.",
  "p.lessons.recorded_on": "dicatat pada {date}",
  "p.lessons.recorded_on_by": "dicatat pada {date} oleh {who}",
  "p.lessons.link_unlisted":
    "tentang catatan yang tidak termasuk dalam tugas, risiko, pencapaian, dan isu rencana yang tercantum di sini",
  "p.lessons.withdraw": "Tarik pelajaran {summary}",
  "p.lessons.withdraw_short": "Tarik",
  "p.lessons.confirm_withdraw":
    "Tarik pelajaran “{summary}”? Pelajaran tidak akan ditampilkan lagi, di sini maupun pada usulan baru; tetap ada dalam jejak audit.",
  "p.lessons.withdrawn": "Ditarik: {summary}",
  "p.lessons.could_not_load": "Pelajaran tidak dapat dimuat.",
  "p.lessons.could_not_save": "Pelajaran tidak diubah.",
  "p.lessons.refused_gone": "Ditolak: rencana sudah tidak ada.",
  "p.lessons.refused_withdrawn":
    "Ditolak: pelajaran itu sudah ditarik. Daftar telah dibaca ulang.",
  "p.lessons.refused_link":
    "Ditolak: catatan yang dipilih bukan lagi catatan rencana ini. Tidak ada yang dicatat; pilih lagi.",
  "p.lessons.refs_heading": "Apa yang merujuk pada suatu catatan",
  "p.lessons.refs_says":
    "Pilih salah satu tugas, risiko, pencapaian, atau isu rencana untuk mencantumkan butir rapat dan pelajaran yang menunjuk padanya (hingga 100 untuk masing-masing).",
  "p.lessons.f_target": "Catatan",
  "p.lessons.choose_target": "Pilih catatan…",
  "p.lessons.show_refs": "Tampilkan apa yang merujuk padanya",
  "p.lessons.problem_target": "Pilih catatan terlebih dahulu.",
  "p.lessons.refs_count_one": "{n} butir merujuk padanya.",
  "p.lessons.refs_count_other": "{n} butir merujuk padanya.",
  "p.lessons.refs_meetings": "Butir rapat",
  "p.lessons.refs_lessons": "Pelajaran yang menunjuk padanya",
  "p.lessons.refs_agenda": "Agenda rapat",
  "p.lessons.refused_no_record":
    "Catatan itu bukan lagi catatan rencana ini, atau Anda tidak dapat membacanya.",
  "p.lessons.could_not_read_refs":
    "Apa yang merujuk padanya tidak dapat dibaca.",
  "p.lessons.similar": "Pelajaran dari rencana serupa",
  "p.lessons.similar_named": "Pelajaran dari rencana seperti {title}",
  "p.lessons.similar_heading": "Pelajaran dari rencana seperti {title}",
  "p.lessons.similar_says":
    "Dicocokkan hanya pada judul, terhadap setiap rencana yang memiliki pelajaran, ditutup atau tidak, kecocokan terbaik lebih dahulu. Skor dan tingkat keyakinan berasal dari pencocok.",
  "p.lessons.similar_none": "Tidak ada rencana serupa yang memiliki pelajaran.",
  "p.lessons.match": "kecocokan {score}, keyakinan {confidence}.",
  "p.lessons.match_title": "Judul mirip {score}.",
  "p.lessons.confidence_high": "tinggi",
  "p.lessons.confidence_medium": "sedang",
  "p.lessons.confidence_low": "rendah",
  "p.lessons.refused_proposal_gone":
    "Usulan itu sudah tidak ada, atau Anda tidak dapat membacanya.",
  "p.lessons.could_not_read_similar":
    "Pelajaran dari rencana serupa tidak dapat dibaca.",
  "p.director_pack.open_by_severity":
    "Terbuka menurut tingkat keparahan: {list}",
  "p.director_pack.issues_overdue": "Isu terlambat: {n}",
  "p.director_pack.issues_in_window":
    "Dalam periode: {raised} diangkat, {resolved} diselesaikan",
  "p.director_pack.oldest_issue":
    "Isu terbuka tertua: {title} (hari kalender terbuka: {n})",
  "p.director_pack.approved_not_implemented":
    "Disetujui tetapi belum diimplementasikan: {n}",
  "p.director_pack.decided_in_window": "Diputuskan dalam periode: {n}",
  "p.director_pack.changes_by_status": "Menurut status: {list}",
  "p.director_pack.oldest_change":
    "Permintaan perubahan terbuka tertua: {title} (hari kalender terbuka: {n})",
  "p.live_pack.heading": "Pembacaan program, sekarang",
  "p.live_pack.says":
    "Dibaca sekarang dari rencana yang dapat Anda baca, digambar seperti paket direktur menggambarnya. Paket yang diterbitkan mempertahankan angka saat diterbitkan; titik sebelumnya pada bagan demam hanya berasal dari paket yang diterbitkan.",
  "p.live_pack.f_ac": "Biaya aktual untuk nilai yang diperoleh",
  "p.live_pack.ac_budget_lines": "Baris anggaran (bawaan)",
  "p.live_pack.ac_labour": "Biaya tenaga kerja yang dicatat",
  "p.live_pack.ac_both": "Baris anggaran dan biaya tenaga kerja",
  "p.live_pack.f_window":
    "Isu dan permintaan perubahan selama beberapa hari terakhir ({min} sampai {max}; kosong untuk 30)",
  "p.live_pack.read_again": "Baca ulang",
  "p.live_pack.problem_window":
    "Jumlah hari harus berupa bilangan bulat dari {min} sampai {max}, atau kosong untuk 30.",
  "p.live_pack.could_not_read_fever":
    "Bagan demam penyangga tidak dapat dibaca.",
  "p.live_pack.could_not_read_evm": "Nilai yang diperoleh tidak dapat dibaca.",
  "p.live_pack.could_not_read_benefits": "Manfaat tidak dapat dibaca.",
  "p.live_pack.could_not_read_issues":
    "Isu dan permintaan perubahan tidak dapat dibaca.",
  "p.live_pack.issues_heading": "Isu dan permintaan perubahan",
  "p.live_pack.issues_none":
    "Anda tidak dapat membaca rencana yang memiliki isu atau permintaan perubahan.",
  "p.live_pack.nothing": "Tidak ada yang ditampilkan.",
  "p.forecast.heading": "Ramalan saat selesai",
  "p.forecast.says":
    "Perkiraan saat selesai adalah biaya aktual sejauh ini ditambah perkiraan untuk menyelesaikan. Perkiraan untuk menyelesaikan adalah angka kendali total proyek terbaru dalam mata uang ramalan, atau jika tidak ada, apa yang masih direncanakan garis dasar terbaru. Dibaca sekarang; tidak ada yang disimpan.",
  "p.forecast.absent_no_currency":
    "Tidak ada ramalan: rencana tidak memiliki garis dasar anggaran dan angka kendali total proyek, jadi tidak ada mata uang untuk meramalkan.",
  "p.forecast.c_actual": "Biaya aktual sejauh ini",
  "p.forecast.c_etc": "Perkiraan untuk menyelesaikan",
  "p.forecast.c_eac": "Perkiraan saat selesai",
  "p.forecast.c_plans": "Rencana",
  "p.forecast.not_known": "tidak diketahui",
  "p.forecast.etc_tpc": "dari angka kendali total proyek terbaru",
  "p.forecast.etc_baseline": "dari apa yang masih direncanakan garis dasar",
  "p.forecast.no_baseline_used":
    "Tidak ada garis dasar anggaran yang digunakan.",
  "p.forecast.baseline_used":
    "Menggunakan garis dasar anggaran versi {version}.",
  "p.forecast.as_of": "Dibaca {date}.",
  "p.forecast.other_currency":
    "Biaya aktual yang dicatat dalam mata uang selain {currency} tidak ditambahkan.",
  "p.forecast.baselines_heading": "Garis dasar anggaran",
  "p.forecast.baselines_says":
    "Garis dasar adalah anggaran yang disetujui menurut periode, dalam satu mata uang. Setelah disetujui, garis dasar dibekukan dan tidak pernah diedit; penetapan ulang garis dasar adalah versi baru, dan setiap versi setelah yang pertama memerlukan alasan. Garis dasar memuat 1 sampai {max} periode, masing-masing dengan akhir tidak sebelum awalnya dan jumlah terencana tidak di bawah nol.",
  "p.forecast.no_baselines": "Belum ada garis dasar anggaran yang disetujui.",
  "p.forecast.version_line": "Versi {version} · {currency} · disetujui {date}",
  "p.forecast.period_line": "{start} sampai {end}: {amount}",
  "p.forecast.first_baseline_heading": "Setujui garis dasar pertama",
  "p.forecast.rebaseline_heading": "Setujui versi {version}",
  "p.forecast.f_currency": "Mata uang (tiga huruf)",
  "p.forecast.periods_legend": "Periode",
  "p.forecast.f_start": "Periode {n} dimulai",
  "p.forecast.f_end": "Periode {n} berakhir",
  "p.forecast.f_amount": "Jumlah terencana periode {n}",
  "p.forecast.remove_period": "Hapus periode {n}",
  "p.forecast.add_period": "Tambah periode",
  "p.forecast.f_reason": "Alasan (opsional untuk yang pertama)",
  "p.forecast.f_reason_needed":
    "Mengapa anggaran ditetapkan ulang garis dasarnya (wajib)",
  "p.forecast.approve": "Setujui garis dasar",
  "p.forecast.confirm_baseline":
    "Setujui garis dasar anggaran versi {version}? Setelah disetujui, dibekukan dan tidak dapat diubah atau dihapus.",
  "p.forecast.baseline_approved": "Versi {version} disetujui.",
  "p.forecast.problem_currency":
    "Mata uang harus berupa kode tiga huruf, seperti GBP atau EUR.",
  "p.forecast.problem_no_periods": "Berikan setidaknya satu periode.",
  "p.forecast.problem_too_many":
    "Garis dasar memuat paling banyak {max} periode.",
  "p.forecast.problem_dates":
    "Setiap periode memerlukan tanggal mulai dan tanggal berakhir.",
  "p.forecast.problem_order": "Periode tidak dapat berakhir sebelum dimulai.",
  "p.forecast.problem_amount":
    "Setiap jumlah terencana harus berupa angka, tidak di bawah nol.",
  "p.forecast.problem_reason": "Penetapan ulang garis dasar memerlukan alasan.",
  "p.forecast.problem_reason_long":
    "Alasan terlalu panjang: paling banyak {bytes} byte.",
  "p.forecast.problem_same":
    "Ini sama dengan versi terbaru; tidak ada yang dikirim.",
  "p.forecast.refused_gone":
    "Rencana ini sudah tidak ada, atau Anda tidak dapat membacanya.",
  "p.forecast.refused_baseline":
    "Garis dasar ditolak: versi lain mungkin telah disetujui sementara itu, sehingga alasan diperlukan. Daftar telah dibaca ulang.",
  "p.forecast.could_not_approve": "Garis dasar tidak dapat disetujui.",
  "p.forecast.could_not_read": "Ramalan tidak dapat dibaca.",
  "p.forecast.could_not_read_baselines":
    "Garis dasar anggaran tidak dapat dibaca.",
  "p.forecast.could_not_read_lines": "Baris anggaran tidak dapat dibaca.",
  "p.forecast.tranches_heading": "Dana yang ditahan sampai suatu gerbang",
  "p.forecast.tranches_says":
    "Baris anggaran dengan gerbang pendanaan ditahan sampai tahap rencana mencapai gerbang itu; setelah itu dapat dilepaskan, satu kali, dan baru kemudian biaya aktual dapat dicatat terhadapnya. Pelepasan tidak dapat dibatalkan.",
  "p.forecast.stage_now": "Tahap rencana: {stage}.",
  "p.forecast.stage_none": "Rencana belum melewati gerbang apa pun.",
  "p.forecast.no_tranches":
    "Tidak ada baris anggaran yang ditahan sampai suatu gerbang.",
  "p.forecast.gate_line": "ditahan sampai {gate}",
  "p.forecast.state_released": "dilepaskan {date}",
  "p.forecast.state_held": "masih ditahan: gerbang belum dilewati",
  "p.forecast.release": "Lepaskan {description}",
  "p.forecast.confirm_release":
    "Lepaskan {description}, yang ditahan sampai {gate}? Pelepasan tidak dapat dibatalkan.",
  "p.forecast.released": "{description} dilepaskan.",
  "p.forecast.refused_line_gone":
    "Baris anggaran atau rencana itu sudah tidak ada.",
  "p.forecast.refused_register":
    "Ditolak: registri program tidak lengkap. Lengkapi dahulu di halaman kelengkapan registri.",
  "p.forecast.refused_release":
    "Ditolak: baris itu tidak lagi ditahan, atau gerbangnya belum dilewati. Baris telah dibaca ulang.",
  "p.forecast.could_not_release": "Baris tidak dapat dilepaskan.",
  "p.forecast.rollup_heading": "Ramalan saat selesai, diringkas",
  "p.forecast.rollup_says":
    "Dari satu rencana atas rencana di bawahnya, hingga {max} tingkat ke dalam, satu baris per mata uang: mata uang tidak pernah dijumlahkan. Rencana tanpa mata uang untuk meramalkan dihitung, tidak ditambahkan.",
  "p.forecast.f_plan": "Rencana",
  "p.forecast.choose_plan": "Pilih rencana",
  "p.forecast.f_depth": "Tingkat ke dalam (1 sampai {max}; kosong untuk semua)",
  "p.forecast.show_rollup": "Tampilkan ramalan",
  "p.forecast.problem_plan": "Pilih rencana.",
  "p.forecast.problem_depth":
    "Tingkat ke dalam harus berupa bilangan bulat dari 1 sampai {max}, atau kosong.",
  "p.forecast.refused_plan_gone":
    "Rencana itu sudah tidak ada, atau Anda tidak dapat membacanya.",
  "p.forecast.could_not_read_rollup":
    "Ramalan yang diringkas tidak dapat dibaca.",
  "p.forecast.rollup_line_one": "Dari {name}: {n} rencana dibaca, {date}.",
  "p.forecast.rollup_line_other": "Dari {name}: {n} rencana dibaca, {date}.",
  "p.forecast.rollup_truncated":
    "Pohon lebih besar daripada yang dijelajahi layanan; angka hanya mencakup sebagian.",
  "p.forecast.rollup_revisits_one":
    "{n} rencana dicapai dua kali dan dihitung sekali.",
  "p.forecast.rollup_revisits_other":
    "{n} rencana dicapai dua kali dan dihitung sekali.",
  "p.forecast.rollup_no_currency_one":
    "{n} rencana tidak memiliki mata uang untuk meramalkan dan tidak ditambahkan.",
  "p.forecast.rollup_no_currency_other":
    "{n} rencana tidak memiliki mata uang untuk meramalkan dan tidak ditambahkan.",
  "p.forecast.rollup_empty": "Tidak ada rencana di sini yang memiliki ramalan.",
  "p.value.heading": "Nilai yang terealisasi",
  "p.value.says":
    "Dibaca sekarang dari nilai yang dicatat; tidak ada yang disimpan. Imbal hasil investasi membandingkan setiap titik nilai dengan biaya aktual yang dicatat, dan ditahan jika baris anggaran memakai lebih dari satu mata uang, karena tidak ada mata uang yang dikonversi. Angka yang tidak dapat dinyatakan menyebutkan alasannya, bukan nol.",
  "p.value.roi": "Imbal hasil investasi: {pct}",
  "p.value.roi_absent": "Imbal hasil investasi tidak dinyatakan: {reason}",
  "p.value.measured_share": "bukti terukur: {pct} dari titik nilai",
  "p.value.ttv_one":
    "Waktu menuju nilai: median {p50} hari kalender, persentil ke-85 {p85} ({n} pengamatan)",
  "p.value.ttv_other":
    "Waktu menuju nilai: median {p50} hari kalender, persentil ke-85 {p85} ({n} pengamatan)",
  "p.value.ttv_absent": "Waktu menuju nilai tidak dinyatakan: {reason}",
  "p.value.adoption":
    "Adopsi: {pct} ({active} dari {target} pengguna aktif selama {days} hari)",
  "p.value.adoption_definition": "pengguna aktif adalah: {definition}",
  "p.value.adoption_absent": "Adopsi tidak dinyatakan: {reason}",
  "p.value.adoption_none": "Tidak ada pembacaan adopsi yang dicatat.",
  "p.value.absent_unrealized": "belum ada nilai yang dicatat",
  "p.value.absent_no_denominator":
    "tidak ada pembanding (tidak ada biaya aktual, atau tidak ada target)",
  "p.value.absent_mixed_currency":
    "baris anggaran memakai lebih dari satu mata uang",
  "p.value.absent_no_baseline": "tidak ada garis dasar anggaran",
  "p.value.absent_no_responses": "belum ada yang menanggapi",
  "p.value.absent_overflow": "angka terlalu besar untuk dihitung",
  "p.value.absent_other": "layanan tidak menyebutkan alasannya",
  "p.value.case_heading": "Target kasus bisnis",
  "p.value.no_targets": "Tidak ada target kasus bisnis yang dicatat.",
  "p.value.target_line":
    "{metric}: dari {baseline} ke {target} · {source} · disetujui {date}",
  "p.value.source_charter": "piagam",
  "p.value.source_gate_review": "tinjauan gerbang",
  "p.value.source_other": "sumber lain",
  "p.value.to_costs": "Catat target di halaman biaya",
  "p.value.point_heading": "Catat nilai yang teramati",
  "p.value.point_says":
    "Nilai bersifat terukur, diperkirakan, atau ditegaskan. Dengan mata uang, nilai adalah jumlah uang; tanpa mata uang, bilangan bulat dalam satuan ukuran itu sendiri. Titik nilai tidak dapat diubah di sini setelah dicatat. Satu titik nilai per rencana dapat menjadi nilai terukur pertama: titik itu menghentikan jam waktu menuju nilai, yang dimulai oleh target kasus bisnis pertama, dan tidak dapat dipindahkan.",
  "p.value.f_value": "Nilai",
  "p.value.f_currency": "Mata uang (tiga huruf, atau kosong)",
  "p.value.f_method": "Bagaimana nilai diperoleh",
  "p.value.method_measured": "terukur",
  "p.value.method_estimated": "diperkirakan",
  "p.value.method_asserted": "ditegaskan",
  "p.value.f_evidence": "Bukti (tautan atau rujukan, opsional)",
  "p.value.f_first": "Ini adalah nilai terukur pertama",
  "p.value.record_point": "Catat nilai",
  "p.value.confirm_first":
    "Catat ini sebagai nilai terukur pertama rencana? Ini menghentikan jam waktu menuju nilai dan tidak dapat dipindahkan.",
  "p.value.point_recorded": "Nilai dicatat.",
  "p.value.problem_currency":
    "Mata uang harus berupa kode tiga huruf, atau kosong.",
  "p.value.problem_amount": "Nilai harus berupa jumlah uang.",
  "p.value.problem_whole":
    "Tanpa mata uang, nilai harus berupa bilangan bulat.",
  "p.value.problem_method": "Pilih bagaimana nilai diperoleh.",
  "p.value.refused_first_taken":
    "Ditolak: rencana ini sudah memiliki nilai terukur pertama; jam berhenti satu kali.",
  "p.value.refused_value": "Nilai ditolak.",
  "p.value.refused_gone":
    "Rencana ini sudah tidak ada, atau Anda tidak dapat membacanya.",
  "p.value.could_not_read": "Nilai yang terealisasi tidak dapat dibaca.",
  "p.value.could_not_record_point": "Nilai tidak dapat dicatat.",
  "p.value.case_says":
    "Target adalah ukuran yang dijanjikan dalam piagam atau pada tinjauan gerbang, dengan nilainya saat persetujuan dan nilai yang dijanjikan, sebagai bilangan bulat dalam satuan ukuran itu. Tanggal persetujuannya memulai jam waktu menuju nilai dan tidak dapat diubah, dan target tidak dapat diubah atau dihapus di sini.",
  "p.value.to_benefits": "Nilai yang terealisasi ada di halaman manfaat.",
  "p.value.f_metric": "Ukuran",
  "p.value.f_baseline": "Nilai saat persetujuan",
  "p.value.f_target": "Nilai yang dijanjikan",
  "p.value.f_unit": "Satuan (opsional)",
  "p.value.f_source": "Dijanjikan dalam",
  "p.value.record_target": "Catat target",
  "p.value.confirm_target":
    "Catat target {metric}? Tidak dapat diubah atau dihapus di sini.",
  "p.value.target_recorded": "Target {metric} dicatat.",
  "p.value.problem_metric": "Sebutkan ukurannya.",
  "p.value.problem_target_values":
    "Nilai saat persetujuan dan nilai yang dijanjikan harus berupa bilangan bulat.",
  "p.value.problem_source": "Pilih di mana target dijanjikan.",
  "p.value.refused_target": "Target ditolak.",
  "p.value.could_not_record_target": "Target tidak dapat dicatat.",
  "p.scoring.rank_by": "Urutkan menurut",
  "p.scoring.model_smart": "Smart Score (dari bukti)",
  "p.scoring.model_wsjf": "WSJF (pekerjaan terpendek berbobot lebih dahulu)",
  "p.scoring.model_rice": "RICE (jangkauan, dampak, keyakinan, upaya)",
  "p.scoring.models_say":
    "Smart Score diambil dari bukti yang disimpan layanan; WSJF dan RICE diberi skor dari perkiraan seseorang, yang ditetapkan untuk setiap rencana di bawah tombol Perkiraannya. Rencana tanpa semua masukan yang dibutuhkan model tidak diberi skor dan diurutkan paling akhir, tidak pernah diberi skor nol. Pita adalah milik Smart Score.",
  "p.scoring.of_plans_one":
    "{length} dari {scored_of} rencana, diurutkan menurut {model}",
  "p.scoring.of_plans_other":
    "{length} dari {scored_of} rencana, diurutkan menurut {model}",
  "p.scoring.ranked_by": "Rencana diurutkan menurut {model}",
  "p.scoring.waiting_for": "Menunggu",
  "p.scoring.open": "Perkiraan",
  "p.scoring.open_named": "Buka perkiraan {name}",
  "p.scoring.heading": "Perkiraan untuk {name}",
  "p.scoring.form": "Perkiraan {name}",
  "p.scoring.says":
    "WSJF dan RICE diberi skor dari perkiraan ini. Skor dengan masukan yang hilang tidak diberikan, tidak pernah ditebak, dan nol yang dinyatakan bukan masukan yang hilang. Menyimpan mengganti semua perkiraan: kotak kosong menghapusnya.",
  "p.scoring.last_set": "Terakhir ditetapkan {when}.",
  "p.scoring.none_set": "Belum ada perkiraan yang ditetapkan.",
  "p.scoring.wsjf_inputs":
    "Masukan WSJF, masing-masing bilangan bulat dari {min} sampai {max}",
  "p.scoring.rice_inputs": "Masukan RICE",
  "p.scoring.input_business_value": "Nilai bisnis",
  "p.scoring.input_time_criticality": "Kekritisan waktu",
  "p.scoring.input_risk_reduction":
    "Pengurangan risiko atau pemungkinan peluang",
  "p.scoring.input_job_size": "Ukuran pekerjaan",
  "p.scoring.input_reach": "Jangkauan (orang atau kejadian dalam satu periode)",
  "p.scoring.input_impact": "Dampak",
  "p.scoring.input_confidence": "Keyakinan (persen)",
  "p.scoring.input_effort": "Upaya (orang-bulan)",
  "p.scoring.input_other": "masukan lain",
  "p.scoring.component_cost_of_delay": "Biaya keterlambatan",
  "p.scoring.impact_minimal": "minim",
  "p.scoring.impact_low": "rendah",
  "p.scoring.impact_medium": "sedang",
  "p.scoring.impact_high": "tinggi",
  "p.scoring.impact_massive": "sangat besar",
  "p.scoring.impact_value": "{value} (dampak {word})",
  "p.scoring.not_estimated": "belum diperkirakan",
  "p.scoring.save": "Simpan perkiraan",
  "p.scoring.close": "Tutup perkiraan",
  "p.scoring.unchanged":
    "Perkiraan sama seperti yang tersimpan, jadi tidak ada yang dikirim.",
  "p.scoring.saved": "Perkiraan untuk {name} disimpan.",
  "p.scoring.could_not_read": "Perkiraan tidak dapat dibaca.",
  "p.scoring.could_not_save": "Perkiraan tidak dapat disimpan.",
  "p.scoring.refused_gone":
    "Rencana itu sudah tidak ada, atau Anda tidak dapat membacanya.",
  "p.scoring.refused_inputs":
    "Perkiraan ditolak: sebuah masukan berada di luar rentangnya.",
  "p.scoring.problem_wsjf":
    "{input} harus berupa bilangan bulat dari {min} sampai {max}.",
  "p.scoring.problem_reach":
    "{input} harus berupa bilangan bulat, tidak di bawah nol.",
  "p.scoring.problem_impact":
    "{input} harus salah satu dari 0,25, 0,5, 1, 2, atau 3.",
  "p.scoring.problem_confidence":
    "{input} harus berupa bilangan bulat dari 0 sampai 100.",
  "p.scoring.problem_effort": "{input} harus di atas nol.",
  "p.scoring.problem_number": "{input} harus berupa angka.",
  "p.scoring.formula_wsjf":
    "WSJF adalah (nilai bisnis + kekritisan waktu + pengurangan risiko atau pemungkinan peluang) ÷ ukuran pekerjaan.",
  "p.scoring.formula_rice":
    "RICE adalah jangkauan × dampak × keyakinan ÷ upaya.",
  "p.scoring.score_is": "Skor: {score}",
  "p.scoring.no_score": "Tidak ada skor: menunggu masukan ini.",
  "p.scoring.needs_delay_part": "merupakan bagian dari biaya keterlambatan",
  "p.scoring.needs_divisor":
    "merupakan pembagi: tanpanya tidak ada nilai per satuan pekerjaan",
  "p.scoring.needs_reach":
    "menyatakan berapa banyak orang yang disentuh pekerjaan dalam satu periode",
  "p.scoring.needs_impact":
    "menyatakan seberapa penting pekerjaan bagi setiap orang yang terjangkau",
  "p.scoring.needs_confidence":
    "mendiskon perkiraan menurut seberapa yakin perkiraan itu",
  "p.scoring.needs_other": "skor memerlukannya",
  "p.scoring.means_business_value":
    "nilai relatif bagi pengguna atau bisnis, 1 sampai 20",
  "p.scoring.means_time_criticality":
    "seberapa cepat nilai menurun jika terlambat, 1 sampai 20",
  "p.scoring.means_risk_reduction":
    "risiko yang dihilangkannya atau peluang yang dibukanya, 1 sampai 20",
  "p.scoring.means_cost_of_delay":
    "jumlah dari tiga di atas: berapa biaya keterlambatan",
  "p.scoring.means_job_size": "ukuran relatif pekerjaan, 1 sampai 20; pembagi",
  "p.scoring.means_reach":
    "orang atau kejadian yang terjangkau dalam satu periode",
  "p.scoring.means_impact":
    "0,25 minim, 0,5 rendah, 1 sedang, 2 tinggi, 3 sangat besar",
  "p.scoring.means_confidence": "seberapa yakin perkiraan itu",
  "p.scoring.means_effort": "orang-bulan pekerjaan; pembagi",
  "p.scoring.means_other": "komponen yang tidak dikenal halaman ini",
  "p.frontier.heading": "Memilih portofolio",
  "p.frontier.says":
    "Pembangkit memberi skor setiap rencana yang dapat Anda baca menurut Smart Score-nya dan menetapkan harganya menurut baris anggarannya, lalu mengambil rencana menurut skor per satuan biaya sampai batas habis. Rencana yang harus disertakan diambil lebih dahulu, berapa pun biayanya. Rencana tanpa Smart Score dikecualikan; rencana tanpa baris anggaran tidak berbiaya. Pilihan tidak selalu yang terbaik, jadi batas yang tidak membeli nilai lebih banyak daripada yang lebih murah ditandai, tidak disembunyikan.",
  "p.frontier.must_include":
    "Rencana yang harus disertakan setiap pilihan (hingga {max})",
  "p.frontier.plans_loading": "Membaca rencana…",
  "p.frontier.plans_none": "Tidak ada rencana yang dapat Anda baca.",
  "p.frontier.frontier_heading": "Batas efisien",
  "p.frontier.frontier_says":
    "Pembangkit dijalankan satu kali pada setiap batas, dan tidak ada yang disimpan. Sebarkan {min} sampai {max} batas secara merata dari nol sampai biaya semua rencana yang dapat dihargai (kosong untuk {fallback}), atau sebutkan sendiri hingga {list} batas, masing-masing berupa jumlah tidak di bawah nol.",
  "p.frontier.f_currency": "Mata uang biaya (tiga huruf)",
  "p.frontier.f_how": "Batas anggaran yang mana",
  "p.frontier.how_steps": "Sebarkan merata",
  "p.frontier.how_caps": "Batas yang saya sebutkan",
  "p.frontier.f_steps": "Berapa banyak batas (kosong untuk {fallback})",
  "p.frontier.f_caps": "Batas anggaran, satu per baris (hingga {list})",
  "p.frontier.run": "Gambar batas efisien",
  "p.frontier.value_is":
    "Nilai adalah jumlah Smart Score rencana yang dipilih.",
  "p.frontier.table": "Batas efisien, titik demi titik",
  "p.frontier.col_cap": "Batas anggaran",
  "p.frontier.col_cost": "Biaya pilihan",
  "p.frontier.col_value": "Nilai",
  "p.frontier.col_included": "Rencana yang dipilih",
  "p.frontier.col_more": "Membeli nilai lebih banyak",
  "p.frontier.more_yes": "Ya",
  "p.frontier.more_no": "Tidak: batas yang lebih murah mencapai sama banyaknya",
  "p.frontier.show_choice": "Tampilkan pilihan",
  "p.frontier.show_choice_named": "Tampilkan pilihan pada {cap}",
  "p.frontier.save_point": "Simpan sebagai draf",
  "p.frontier.save_point_named": "Simpan pilihan pada {cap} sebagai draf",
  "p.frontier.point_name": "Batas efisien pada {cap}",
  "p.frontier.axis_cost": "Biaya rencana yang dipilih",
  "p.frontier.axis_value": "Nilai (jumlah Smart Score)",
  "p.frontier.chart_points_one": "{n} batas anggaran.",
  "p.frontier.chart_points_other": "{n} batas anggaran.",
  "p.frontier.chart_best": "Nilai terbanyak, {value}, berbiaya {cost}.",
  "p.frontier.chart_dominated_one":
    "{n} batas tidak membeli nilai lebih banyak daripada yang lebih murah.",
  "p.frontier.chart_dominated_other":
    "{n} batas tidak membeli nilai lebih banyak daripada yang lebih murah.",
  "p.frontier.legend_efficient": "membeli nilai lebih banyak",
  "p.frontier.legend_dominated":
    "tidak membeli nilai lebih banyak daripada batas yang lebih murah",
  "p.frontier.no_points": "Batas efisien tidak memiliki titik.",
  "p.frontier.generate_heading": "Hasilkan draf",
  "p.frontier.generate_says":
    "Disimpan sebagai skenario draf, untuk dievaluasi, dibandingkan, dan dikomitmenkan seperti yang lain. Nama diperlukan, paling banyak {bytes} byte. Dengan mata uang, setiap rencana dihargai di dalamnya; batas memerlukan mata uangnya.",
  "p.frontier.g_name": "Nama draf",
  "p.frontier.g_cap": "Batas anggaran (opsional)",
  "p.frontier.g_currency": "Mata uang (diperlukan dengan batas)",
  "p.frontier.generate": "Hasilkan draf",
  "p.frontier.draft_saved": "Draf {name} disimpan; ada di daftar di atas.",
  "p.frontier.choice_at": "Apa yang dipilih oleh batas {cap}",
  "p.frontier.choice_of": "Apa yang dipilih {name}",
  "p.frontier.chosen_count_one": "{n} rencana dipilih dari {of}.",
  "p.frontier.chosen_count_other": "{n} rencana dipilih dari {of}.",
  "p.frontier.col_plan": "Rencana dalam pilihan",
  "p.frontier.col_chosen": "Dipilih atau tidak",
  "p.frontier.col_score": "Smart Score-nya",
  "p.frontier.col_plan_cost": "Biayanya",
  "p.frontier.col_why": "Mengapa dikecualikan",
  "p.frontier.chosen": "dipilih",
  "p.frontier.left_out": "dikecualikan",
  "p.frontier.no_score": "tanpa Smart Score",
  "p.frontier.plan_unknown":
    "Rencana yang tidak dapat Anda baca, atau yang sudah tidak ada",
  "p.frontier.reason_no_score":
    "tidak memiliki Smart Score, yang bukan skor nol",
  "p.frontier.reason_foreign_currency":
    "baris anggarannya semua dalam mata uang lain, sehingga tidak dapat dihargai",
  "p.frontier.reason_over_cap": "batas habis sebelum rencana ini",
  "p.frontier.reason_must_include_conflict":
    "harus disertakan, tetapi bukan rencana yang dapat Anda baca",
  "p.frontier.reason_other":
    "dikecualikan karena alasan yang tidak diketahui halaman ini",
  "p.frontier.problem_currency": "Berikan mata uang sebagai tiga huruf.",
  "p.frontier.problem_steps":
    "Jumlah batas harus berupa bilangan bulat dari {min} sampai {max}.",
  "p.frontier.problem_caps_none": "Sebutkan setidaknya satu batas.",
  "p.frontier.problem_caps_many": "Sebutkan paling banyak {list} batas.",
  "p.frontier.problem_caps_amount":
    "Setiap batas harus berupa jumlah tidak di bawah nol.",
  "p.frontier.problem_must_include":
    "Pilih paling banyak {list} rencana untuk disertakan.",
  "p.frontier.problem_name": "Beri draf sebuah nama.",
  "p.frontier.problem_name_long": "Nama lebih panjang dari {bytes} byte.",
  "p.frontier.problem_cap_currency": "Batas memerlukan mata uangnya.",
  "p.frontier.problem_cap_amount":
    "Batas harus berupa jumlah tidak di bawah nol.",
  "p.frontier.refused_frontier": "Batas efisien ditolak.",
  "p.frontier.refused_generate": "Draf ditolak.",
  "p.frontier.could_not_read": "Batas efisien tidak dapat digambar.",
  "p.frontier.could_not_generate": "Draf tidak dapat dihasilkan.",
  "p.frontier.rollback": "Gulung balik",
  "p.frontier.rollback_named": "Gulung balik {name}",
  "p.frontier.rollback_says":
    "Skenario yang telah dikomitmenkan dapat digulung balik menjadi draf, setelah ada pertanyaan; tidak ada yang berubah selain skenarionya.",
  "p.frontier.confirm_rollback":
    "Gulung balik {name} menjadi draf? Skenario tidak lagi menjadi portofolio yang dikomitmenkan; komitmen tetap ada dalam jejak audit.",
  "p.frontier.rolled_back": "{name} kembali menjadi draf.",
  "p.frontier.refused_rollback_status":
    "{name} tidak lagi dikomitmenkan, jadi tidak ada yang digulung balik. Daftar telah dibaca ulang.",
  "p.frontier.refused_rollback_gone":
    "{name} sudah tidak ada, atau Anda tidak dapat membacanya. Daftar telah dibaca ulang.",
  "p.frontier.could_not_roll_back": "Skenario tidak dapat digulung balik.",
  "p.frontier.status_draft": "draf",
  "p.frontier.status_committed": "dikomitmenkan",
  "p.frontier.violations_one": "{n} kendala dilanggar",
  "p.frontier.violations_other": "{n} kendala dilanggar",
  "p.frontier.compare_note":
    "B dikurangi A, satu mata uang pada satu waktu; mata uang tidak pernah dijumlahkan.",
  "p.intake.edit": "Ubah",
  "p.intake.edit_named": "Ubah draf {title}",
  "p.intake.edit_heading": "Ubah {title}",
  "p.intake.edit_heading_loading": "Membaca draf…",
  "p.intake.edit_form": "Kolom draf",
  "p.intake.edit_says":
    "Hanya draf yang dapat diubah, di sinilah usulan juga ditandai sebagai permintaan eksternal. Judul diperlukan; judul dan ringkasan masing-masing paling banyak {bytes} byte; jumlah uang memerlukan mata uangnya; pertukaran yang disebutkan terdiri dari {min} sampai {max} karakter. Menyimpan mengganti setiap kolom.",
  "p.intake.save": "Simpan draf",
  "p.intake.saved": "{title} disimpan.",
  "p.intake.unchanged": "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.intake.could_not_read_draft": "Draf tidak dapat dibaca.",
  "p.intake.problem_not_draft": "Hanya draf yang dapat diubah.",
  "p.intake.problem_title_long": "Judul lebih panjang dari {bytes} byte.",
  "p.intake.problem_summary_long": "Ringkasan lebih panjang dari {bytes} byte.",
  "p.intake.refused_not_draft":
    "Perubahan ditolak: usulan bukan lagi draf. Daftar telah dibaca ulang.",
  "p.intake.refused_edit":
    "Perubahan ditolak: sebuah kolom berada di luar yang diterima layanan.",
  "p.intake.forecast_heading": "Berapa banyak yang akan disetujui",
  "p.intake.forecast_says":
    "Dihasilkan lewat simulasi dari berapa banyak usulan yang disetujui pada setiap periode lampau; usulan yang masuk tidak memengaruhinya. Ramalan memerlukan riwayat sedikitnya {needed} periode.",
  "p.intake.f_periods":
    "Periode ke depan (1 sampai {max}; kosong untuk {fallback})",
  "p.intake.f_period_days":
    "Hari dalam satu periode (1 sampai {max}; kosong untuk {fallback})",
  "p.intake.f_history":
    "Periode riwayat (1 sampai {max}; kosong untuk {fallback})",
  "p.intake.forecast_run": "Ramalkan",
  "p.intake.problem_periods":
    "Periode ke depan harus berupa bilangan bulat dari 1 sampai {periods}.",
  "p.intake.problem_period_days":
    "Satu periode harus berupa bilangan bulat hari dari 1 sampai {days}.",
  "p.intake.problem_history":
    "Riwayat harus berupa bilangan bulat periode dari 1 sampai {history}.",
  "p.intake.refused_forecast": "Ramalan ditolak.",
  "p.intake.could_not_forecast": "Ramalan tidak dapat dibuat.",
  "p.intake.window":
    "Riwayat dari {from} sampai {to}, dalam periode {days} hari.",
  "p.intake.at_least_label_one":
    "Dalam {n} periode berikutnya ({days} hari), setidaknya",
  "p.intake.at_least_label_other":
    "Dalam {n} periode berikutnya ({days} hari), setidaknya",
  "p.intake.at_least_means":
    "dengan keyakinan 85%: angka untuk dijadikan dasar rencana",
  "p.intake.median_label": "Biasanya",
  "p.intake.median_means":
    "separuh percobaan menyetujui lebih banyak, separuh lebih sedikit: tebakan lempar koin, bukan komitmen",
  "p.intake.at_most_label": "Paling banyak",
  "p.intake.at_most_means":
    "ujung yang optimistis: 85% percobaan menyetujui tidak lebih dari ini",
  "p.intake.proposals_n_one": "{n} usulan",
  "p.intake.proposals_n_other": "{n} usulan",
  "p.intake.absent_history":
    "Tidak ada ramalan: diperlukan riwayat sedikitnya {needed} periode, dan yang ada {n}.",
  "p.intake.absent_other":
    "Tidak ada ramalan yang dapat dibuat dari riwayat ini.",
  "p.intake.approvals_history":
    "Disetujui per periode, yang tertua lebih dahulu: {list}",
  "p.intake.arrivals_history":
    "Usulan masuk per periode, yang tertua lebih dahulu (ditampilkan di sampingnya; tidak memengaruhi ramalan): {list}",
  "p.planner.heading": "Waktu terencana",
  "p.planner.says":
    "Blokir waktu untuk salah satu tugas terbuka Anda pada suatu hari. Slot hanya milik Anda dan tidak pernah mengubah tenggat tugas; slot berakhir setelah dimulai dan berlangsung paling lama 12 jam. Slot boleh tumpang tindih: tumpang tindih ditampilkan sebagai bentrokan, dan slot yang berakhir saat slot berikutnya dimulai bukan bentrokan. Waktu dalam zona waktu Anda, {tz}.",
  "p.planner.day_nav": "Pilih hari",
  "p.planner.previous_day": "Hari sebelumnya",
  "p.planner.back_to_today": "Kembali ke hari ini",
  "p.planner.next_day": "Hari berikutnya",
  "p.planner.f_day": "Hari",
  "p.planner.show_day": "Tampilkan hari",
  "p.planner.problem_day": "Berikan hari sebagai tanggal.",
  "p.planner.day_heading": "Direncanakan pada {day}",
  "p.planner.day_hours":
    "Hari ini berlangsung {hours} jam, karena perubahan jam.",
  "p.planner.nothing_planned": "Tidak ada yang direncanakan pada hari ini.",
  "p.planner.clashes_one": "Bentrok dengan {n} slot lain",
  "p.planner.clashes_other": "Bentrok dengan {n} slot lain",
  "p.planner.remove": "Hapus",
  "p.planner.remove_named": "Hapus slot untuk {title}, {time}",
  "p.planner.confirm_remove":
    "Hapus slot untuk {title}, {time}? Tugasnya sendiri tidak diubah.",
  "p.planner.removed": "Slot untuk {title}, {time} dihapus.",
  "p.planner.omitted_one":
    "{n} slot pada tugas yang tidak dapat Anda baca lagi tidak ditampilkan.",
  "p.planner.omitted_other":
    "{n} slot pada tugas yang tidak dapat Anda baca lagi tidak ditampilkan.",
  "p.planner.due_heading": "Jatuh tempo pada hari ini",
  "p.planner.nothing_due":
    "Tidak ada yang ditugaskan kepada Anda yang jatuh tempo pada hari ini.",
  "p.planner.loading": "Membaca hari…",
  "p.planner.form": "Rencanakan waktu untuk tugas",
  "p.planner.form_says":
    "Slot berada pada {day}, dalam zona waktu Anda. Slot yang melewati tengah malam direncanakan sebagai dua.",
  "p.planner.f_task": "Tugas",
  "p.planner.choose_task": "Pilih salah satu tugas terbuka Anda",
  "p.planner.loading_tasks": "Membaca tugas Anda…",
  "p.planner.task_option": "{title}, di {plan}",
  "p.planner.no_tasks":
    "Tidak ada yang ditugaskan kepada Anda, jadi tidak ada tugas untuk direncanakan waktunya.",
  "p.planner.f_start": "Mulai",
  "p.planner.f_end": "Berakhir",
  "p.planner.plan_time": "Rencanakan waktu ini",
  "p.planner.planned": "Direncanakan {title}, {time}.",
  "p.planner.problem_task": "Pilih tugas.",
  "p.planner.problem_start": "Berikan waktu mulainya.",
  "p.planner.problem_end": "Berikan waktu berakhirnya.",
  "p.planner.problem_start_skipped":
    "Jam melewati waktu mulai pada hari ini; pilih yang lain.",
  "p.planner.problem_end_skipped":
    "Jam melewati waktu berakhir pada hari ini; pilih yang lain.",
  "p.planner.problem_order": "Slot harus berakhir setelah dimulai.",
  "p.planner.problem_too_long": "Slot berlangsung paling lama 12 jam.",
  "p.planner.refused_zone":
    "Zona waktu peramban Anda tidak diterima, jadi hari tidak dapat ditampilkan.",
  "p.planner.refused_slot":
    "Slot tidak diterima: harus berakhir setelah dimulai dan berlangsung paling lama 12 jam, dan satu orang memegang paling banyak {max} slot.",
  "p.planner.refused_task":
    "Tugas itu sudah tidak ada, atau berada di rencana yang tidak dapat Anda baca.",
  "p.planner.refused_gone":
    "Slot itu sudah tidak ada. Hari telah dibaca ulang.",
  "p.planner.could_not_load": "Hari tidak dapat dibaca.",
  "p.planner.could_not_plan": "Waktu tidak dapat direncanakan.",
  "p.planner.could_not_remove": "Slot tidak dapat dihapus.",
  "p.feed.heading": "Umpan kalender",
  "p.feed.says":
    "Alamat pribadi yang dapat dilanggan aplikasi kalender: tugas terbuka yang ditugaskan kepada Anda yang memiliki tenggat, pencapaian dan sprint rencananya, dan waktu yang Anda rencanakan. Alamat kedua adalah umpan notifikasi Anda untuk pembaca umpan. Siapa pun yang memiliki alamat dapat membaca apa yang ditampilkannya, jadi simpanlah untuk diri Anda sendiri.",
  "p.feed.rules":
    "Alamat ditampilkan sekali, di sini, dan halaman ini tidak menyimpannya; berlaku selama 365 hari. Membuat yang baru langsung menghentikan yang sebelumnya.",
  "p.feed.issue": "Buat alamat umpan",
  "p.feed.confirm_issue":
    "Buat alamat umpan baru? Alamat yang pernah Anda buat sebelumnya langsung berhenti berfungsi.",
  "p.feed.issued":
    "Alamat baru Anda ada di bawah. Salin sekarang: alamat tidak ditampilkan lagi.",
  "p.feed.calendar_address": "Alamat kalender (iCalendar)",
  "p.feed.atom_address": "Alamat umpan notifikasi (Atom)",
  "p.feed.revoke": "Hentikan setiap alamat umpan",
  "p.feed.confirm_revoke":
    "Hentikan setiap alamat umpan yang Anda buat? Aplikasi kalender yang memakai salah satunya langsung berhenti memperbarui.",
  "p.feed.revoked":
    "Setiap alamat umpan yang Anda buat telah berhenti berfungsi.",
  "p.feed.no_address":
    "Jawaban tidak memuat alamat yang dapat ditampilkan halaman ini, jadi tidak ada yang ditampilkan. Alamat sebelumnya telah dihentikan; buat yang lain jika Anda memerlukannya.",
  "p.feed.refused_forbidden": "Anda tidak boleh mengelola alamat umpan.",
  "p.feed.could_not_issue": "Alamat umpan tidak dapat dibuat.",
  "p.feed.could_not_revoke": "Alamat umpan tidak dapat dihentikan.",
  "p.watching.heading": "Yang Anda ikuti",
  "p.watching.says":
    "Anda diberi tahu ketika sesuatu yang Anda ikuti berubah. Anda mengikuti tugas ketika Anda membuatnya, mengomentarinya, atau diberi tugas itu, kecuali Anda mematikannya dalam pilihan notifikasi Anda. Tidak ada orang lain yang diberi tahu siapa yang mengikuti. Rencana disebut dengan namanya; yang lain ditampilkan menurut jenis dan awal rujukannya.",
  "p.watching.count_one": "Anda mengikuti {n} hal.",
  "p.watching.count_other": "Anda mengikuti {n} hal.",
  "p.watching.capped": "Hanya {max} yang terbaru ditampilkan.",
  "p.watching.none": "Saat ini Anda tidak mengikuti apa pun.",
  "p.watching.named_plan": "Rencana {name}",
  "p.watching.unnamed": "{kind}, rujukan {ref}",
  "p.watching.kind_plan": "Rencana",
  "p.watching.kind_task": "Tugas",
  "p.watching.kind_risk": "Risiko",
  "p.watching.kind_proposal": "Usulan",
  "p.watching.kind_idea": "Gagasan",
  "p.watching.kind_canvas_item": "Butir kanvas",
  "p.watching.kind_budget_line": "Baris anggaran",
  "p.watching.kind_scenario": "Skenario",
  "p.watching.kind_document_version": "Versi dokumen",
  "p.watching.kind_other": "Hal lain",
  "p.watching.stop": "Berhenti mengikuti",
  "p.watching.stop_named": "Berhenti mengikuti {what}",
  "p.watching.confirm_stop":
    "Berhenti mengikuti {what}? Anda tidak akan diberi tahu lagi ketika berubah.",
  "p.watching.stopped": "Anda tidak lagi mengikuti {what}.",
  "p.watching.refused": "Penghentian mengikuti itu tidak diterima.",
  "p.watching.loading": "Membaca yang Anda ikuti…",
  "p.watching.could_not_load": "Yang Anda ikuti tidak dapat dibaca.",
  "p.watching.could_not_stop": "Tidak dapat berhenti mengikutinya.",
  "p.workprofile.heading": "Pekerjaan dari waktu ke waktu",
  "p.workprofile.says":
    "Pekerjaan yang dibawa tugas di setiap rencana yang dapat Anda baca, disebar pada hari kerja setiap tugas menurut konturnya, dalam menit bulat. Pekerjaan yang tidak dapat ditempatkan (tugas yang belum dijadwalkan, atau yang tanpa hari kerja) dihitung terpisah, tidak pernah dibuang.",
  "p.workprofile.rules":
    "Minggu berjalan dari Senin sampai Minggu. Kosongkan tanggal untuk mencakup semua pekerjaan yang ditempatkan. Jendela mencakup paling lama sepuluh tahun dan {max} periode.",
  "p.workprofile.form": "Tampilkan pekerjaan dari waktu ke waktu",
  "p.workprofile.f_period": "Periode",
  "p.workprofile.period_week": "Minggu",
  "p.workprofile.period_day": "Hari",
  "p.workprofile.period_month": "Bulan",
  "p.workprofile.f_from": "Dari (kosong untuk hari pertama yang ditempatkan)",
  "p.workprofile.f_to": "Sampai (kosong untuk hari terakhir yang ditempatkan)",
  "p.workprofile.show": "Tampilkan pekerjaan",
  "p.workprofile.problem_from": "Berikan hari pertama sebagai tanggal.",
  "p.workprofile.problem_to": "Berikan hari terakhir sebagai tanggal.",
  "p.workprofile.problem_order":
    "Hari pertama tidak boleh setelah hari terakhir.",
  "p.workprofile.problem_too_long":
    "Jendela mencakup paling lama sepuluh tahun.",
  "p.workprofile.problem_periods":
    "Jendela itu memuat lebih dari {max} periode; pilih periode yang lebih panjang atau jendela yang lebih pendek.",
  "p.workprofile.refused":
    "Jendela itu tidak diterima: jika belum ada pekerjaan yang ditempatkan, berikan kedua tanggal, dan tetap dalam batas periode yang disebutkan di atas.",
  "p.workprofile.none":
    "Tidak ada rencana yang dapat Anda baca yang memiliki tugas dengan pekerjaan.",
  "p.workprofile.summary":
    "{range}: {work} pekerjaan, {unplaced} di antaranya tidak ditempatkan.",
  "p.workprofile.plans_table": "Pekerjaan menurut rencana",
  "p.workprofile.periods_table": "Pekerjaan menurut periode",
  "p.workprofile.col_plan": "Rencana",
  "p.workprofile.col_work": "Pekerjaan",
  "p.workprofile.col_unplaced": "Tidak ditempatkan",
  "p.workprofile.col_period": "Periode",
  "p.workprofile.plan_unnamed": "Rencana tanpa nama",
  "p.workprofile.could_not_load":
    "Pekerjaan dari waktu ke waktu tidak dapat dibaca.",
  "p.timeentry.heading": "Catat waktu",
  "p.timeentry.says":
    "Waktu dicatat atas nama Anda, dalam menit bulat, dari 1 menit sampai {max} pada satu tanggal, dalam sebuah rencana dan, jika Anda mau, salah satu tugasnya. Minggu yang disetujui tidak menerima waktu lagi. Entri tidak dapat diubah atau dihapus setelah dicatat.",
  "p.timeentry.open": "Catat sedikit waktu",
  "p.timeentry.form": "Catat waktu dalam rencana",
  "p.timeentry.f_plan": "Rencana",
  "p.timeentry.choose_plan": "Pilih rencana",
  "p.timeentry.loading": "Membaca rencana…",
  "p.timeentry.f_task": "Tugas (opsional)",
  "p.timeentry.no_task": "Tanpa tugas: rencana itu sendiri",
  "p.timeentry.task_unnamed": "Tugas yang tidak ada lagi di rencana ini",
  "p.timeentry.f_date": "Tanggal",
  "p.timeentry.f_duration": "Waktu yang dihabiskan",
  "p.timeentry.f_hours": "Jam",
  "p.timeentry.f_minutes": "Menit",
  "p.timeentry.typed": "Itu {time}.",
  "p.timeentry.f_category": "Kategori",
  "p.timeentry.category_unclassified": "Tidak diklasifikasikan",
  "p.timeentry.category_capex": "Belanja modal (capex)",
  "p.timeentry.category_opex": "Belanja operasional (opex)",
  "p.timeentry.f_billable": "Dapat ditagih",
  "p.timeentry.f_note": "Catatan (opsional)",
  "p.timeentry.record": "Catat waktu ini",
  "p.timeentry.confirm":
    "Catat {time} pada {day} di {plan}? Entri tidak dapat diubah atau dihapus setelah dicatat.",
  "p.timeentry.recorded": "Dicatat {time} pada {day} di {plan}.",
  "p.timeentry.problem_plan": "Pilih rencana.",
  "p.timeentry.problem_date": "Berikan tanggal waktu dihabiskan.",
  "p.timeentry.problem_locked":
    "Minggu itu disetujui dan terkunci, jadi tidak menerima waktu lagi.",
  "p.timeentry.problem_duration":
    "Berikan waktu yang dihabiskan sebagai jam bulat (0 sampai 24) dan menit (0 sampai 59), dari 1 menit sampai {max} menit seluruhnya.",
  "p.timeentry.refused_locked":
    "Minggu itu disetujui dan terkunci, jadi tidak menerima waktu lagi. Tidak ada yang dicatat.",
  "p.timeentry.refused_invalid":
    "Entri tidak diterima: waktunya, atau tugasnya, bukan yang diterima rencana. Tidak ada yang dicatat.",
  "p.timeentry.refused_plan":
    "Rencana itu sudah tidak ada, atau Anda tidak dapat membacanya.",
  "p.timeentry.refused_forbidden":
    "Anda tidak boleh mencatat waktu dalam rencana ini.",
  "p.timeentry.could_not_load_plans": "Rencana tidak dapat dibaca.",
  "p.timeentry.could_not_load_plan":
    "Tugas dan waktu rencana itu tidak dapat dibaca.",
  "p.timeentry.could_not_record": "Waktu tidak dapat dicatat.",
  "p.timeentry.yours_heading": "Waktu Anda di {plan}",
  "p.timeentry.yours_none": "Anda belum mencatat waktu apa pun di rencana ini.",
  "p.timeentry.yours_table":
    "Entri waktu Anda di {plan}, yang terbaru lebih dahulu",
  "p.timeentry.col_date": "Tanggal",
  "p.timeentry.col_time": "Waktu",
  "p.timeentry.col_task": "Tugas",
  "p.timeentry.col_category": "Kategori",
  "p.timeentry.col_billable": "Dapat ditagih",
  "p.timeentry.col_note": "Catatan",
  "p.timeentry.billable_yes": "Dapat ditagih",
  "p.timeentry.billable_no": "Tidak dapat ditagih",
  "p.timereport.heading": "Laporan waktu",
  "p.timereport.says":
    "Waktu yang dicatat dalam rencana yang dapat Anda baca, dikelompokkan sesuai pilihan Anda, dengan yang dapat ditagih dan tidak dapat ditagih selalu terpisah. Kosongkan tanggal untuk 90 hari terakhir sampai hari ini. Orang adalah rujukan person: atau worker:; kosongkan untuk semua orang.",
  "p.timereport.form": "Minta laporan waktu",
  "p.timereport.f_group_by": "Kelompokkan menurut",
  "p.timereport.by_plan": "Rencana",
  "p.timereport.by_person": "Orang",
  "p.timereport.by_category": "Kategori",
  "p.timereport.by_billable": "Dapat ditagih atau tidak",
  "p.timereport.by_month": "Bulan",
  "p.timereport.f_from": "Dari",
  "p.timereport.f_to": "Sampai",
  "p.timereport.f_plan": "Rencana",
  "p.timereport.every_plan": "Setiap rencana yang dapat Anda baca",
  "p.timereport.loading_plans": "Membaca rencana…",
  "p.timereport.f_person": "Orang (sebuah rujukan)",
  "p.timereport.show": "Tampilkan laporan waktu",
  "p.timereport.problem_from": "Berikan hari pertama sebagai tanggal.",
  "p.timereport.problem_to": "Berikan hari terakhir sebagai tanggal.",
  "p.timereport.problem_order":
    "Hari pertama tidak boleh setelah hari terakhir.",
  "p.timereport.problem_person":
    "Orang adalah person: atau worker: dan sebuah pengenal, seperti person:6f1c2a3b-0000-4000-8000-000000000009.",
  "p.timereport.refused": "Laporan waktu tidak diterima untuk pilihan itu.",
  "p.timereport.window": "Waktu yang dicatat {range}.",
  "p.timereport.no_entries":
    "Tidak ada waktu yang dicatat di rencana ini pada jangka itu.",
  "p.timereport.table": "Waktu menurut kelompok",
  "p.timereport.col_entries": "Entri",
  "p.timereport.col_billable": "Dapat ditagih",
  "p.timereport.col_not_billable": "Tidak dapat ditagih",
  "p.timereport.col_total": "Jumlah",
  "p.timereport.total":
    "Seluruhnya: {billable} dapat ditagih, {not_billable} tidak dapat ditagih, {total} bersama-sama.",
  "p.timereport.omitted_one":
    "{n} entri di rencana yang tidak dapat Anda baca dikecualikan.",
  "p.timereport.omitted_other":
    "{n} entri di rencana yang tidak dapat Anda baca dikecualikan.",
  "p.timereport.truncated":
    "Hanya {max} entri pertama yang dibaca, jadi laporan tidak lengkap.",
  "p.timereport.csv": "Unduh laporan waktu ini sebagai CSV",
  "p.timereport.billable": "Dapat ditagih",
  "p.timereport.not_billable": "Tidak dapat ditagih",
  "p.timereport.plan_unnamed": "Rencana yang tidak disebutkan di sini ({ref})",
  "p.timereport.could_not_load": "Laporan waktu tidak dapat dibaca.",
  "p.timereport.could_not_load_plans":
    "Rencana untuk dipilih tidak dapat dibaca.",
  "p.hammock.heading": "Hammock: tugas ringkasan yang mencakup tugas lain",
  "p.hammock.says":
    "Hammock adalah ringkasan yang tanggalnya berasal dari tugas rencana ini yang ditautkannya, dari mulai paling awal sampai selesai paling akhir. Hammock tidak ada dalam jaringan ketergantungan, jadi tidak pernah mengubah float tugas mana pun atau waktu selesai. Tugas menjadi hammock hanya jika tidak memiliki durasi, pekerjaan, kendala, atau ketergantungan, dan mulai terencananya dikosongkan; menjadikannya tugas lagi menghapus setiap tautannya. Hammock menaut ke tugas atau hammock lain pada rencana ini, bukan ke dirinya sendiri, tidak dua kali, dan tidak dalam putaran.",
  "p.hammock.none": "Rencana ini tidak memiliki hammock.",
  "p.hammock.spans": "Mencakup {range}.",
  "p.hammock.no_links": "Belum ada tanggal: tidak menaut ke tugas mana pun.",
  "p.hammock.members_unscheduled":
    "Belum ada tanggal: tidak satu pun tugas yang ditautkannya memiliki tanggal.",
  "p.hammock.untitled": "Tugas tanpa judul",
  "p.hammock.member_hammock": "{title} (sebuah hammock)",
  "p.hammock.unmake_named": "Jadikan {title} tugas lagi",
  "p.hammock.unlink": "Hapus tautan",
  "p.hammock.unlink_named": "Hapus tautan dari {hammock} ke {title}",
  "p.hammock.make_form": "Jadikan tugas sebagai hammock",
  "p.hammock.f_task": "Tugas",
  "p.hammock.choose_task": "Pilih tugas",
  "p.hammock.make": "Jadikan hammock",
  "p.hammock.link_form": "Tautkan hammock ke tugas",
  "p.hammock.f_hammock": "Tugas hammock",
  "p.hammock.choose_hammock": "Pilih hammock",
  "p.hammock.f_member": "Tugas yang dicakupnya",
  "p.hammock.choose_member": "Pilih tugas untuk dicakup",
  "p.hammock.link": "Tambah tautan",
  "p.hammock.confirm_make":
    "Jadikan {title} sebuah hammock? Mulai terencananya dikosongkan, dan tanggalnya kemudian berasal dari tugas yang ditautkannya.",
  "p.hammock.confirm_unmake_one":
    "Jadikan {title} tugas lagi? {n} tautannya hilang.",
  "p.hammock.confirm_unmake_other":
    "Jadikan {title} tugas lagi? {n} tautannya hilang.",
  "p.hammock.confirm_unlink":
    "Hapus tautan dari {hammock} ke {title}? Tugasnya sendiri tidak diubah.",
  "p.hammock.made": "{title} sekarang adalah hammock.",
  "p.hammock.unmade": "{title} adalah tugas lagi.",
  "p.hammock.linked": "{hammock} sekarang mencakup {title}.",
  "p.hammock.unlinked": "{hammock} tidak lagi mencakup {title}.",
  "p.hammock.problem_choose_task": "Pilih tugas untuk dijadikan hammock.",
  "p.hammock.problem_carries_work":
    "Tugas ini memiliki durasi, pekerjaan, atau kendala: kosongkan dahulu pada jadwal.",
  "p.hammock.problem_has_dependencies":
    "Tugas ini memiliki ketergantungan: hapus dahulu, karena hammock tidak menerimanya.",
  "p.hammock.problem_choose_hammock": "Pilih hammock.",
  "p.hammock.problem_choose_member": "Pilih tugas untuk dicakup hammock.",
  "p.hammock.problem_self": "Hammock tidak dapat mencakup dirinya sendiri.",
  "p.hammock.problem_linked": "Hammock ini sudah mencakup tugas itu.",
  "p.hammock.problem_loop":
    "Hammock itu sudah mencakup yang ini, jadi tautan akan membuat putaran.",
  "p.hammock.refused_make":
    "Layanan tidak mau menjadikan tugas ini hammock: tugas masih memiliki durasi, pekerjaan, kendala, atau ketergantungan.",
  "p.hammock.refused_link":
    "Layanan tidak mau menambah tautan: tugas mungkin sudah tidak ada di rencana ini, sudah dicakup, atau bagian dari putaran.",
  "p.hammock.refused_gone":
    "Tugas atau hammock sudah tidak ada, atau Anda tidak dapat membacanya lagi.",
  "p.hammock.refused_link_gone": "Tautan itu sudah tidak ada.",
  "p.hammock.could_not_load": "Hammock tidak dapat dibaca.",
  "p.hammock.could_not_save": "Hammock tidak dapat diubah.",
  "p.crossplan.heading": "Tugas dari rencana yang ditautkan",
  "p.crossplan.says":
    "Ini adalah tugas dari rencana lain yang dapat Anda baca yang ditautkan oleh tugas rencana ini, dijadwalkan dengan kalender rencananya sendiri. Batang putus-putus adalah tugas di luar jalur kritis; batang penuh dan belah ketupat menandai tugas yang berada di jalur itu.",
  "p.crossplan.network_finish":
    "Dengan rencana yang ditautkan, jaringan selesai {date}.",
  "p.crossplan.truncated":
    "Tidak setiap rencana yang ditautkan ditampilkan: jaringan mencapai batas rencana atau tugasnya.",
  "p.crossplan.links_heading": "Tautan yang melintas ke rencana lain",
  "p.crossplan.lag": "jeda {days} hari kerja",
  "p.crossplan.timeline": "Tugas {plan} pada lini masa",
  "p.crossplan.table": "Tanggal tugas dari rencana yang ditautkan",
  "p.crossplan.col_task": "Tugas",
  "p.crossplan.col_plan": "Rencana",
  "p.crossplan.col_start": "Mulai paling awal",
  "p.crossplan.col_finish": "Selesai paling awal",
  "p.crossplan.col_critical": "Di jalur kritis",
  "p.crossplan.yes": "Ya",
  "p.crossplan.no": "Tidak",
  "p.crossplan.untitled": "Tugas tanpa judul",
  "p.crossplan.kind_fs": "selesai ke mulai",
  "p.crossplan.kind_ss": "mulai ke mulai",
  "p.crossplan.kind_ff": "selesai ke selesai",
  "p.crossplan.kind_sf": "mulai ke selesai",
  "p.crossplan.task_heading":
    "Ketergantungan, di rencana ini atau rencana lain",
  "p.crossplan.task_says":
    "Ketergantungan menghubungkan tugas ini dengan tugas di rencana lain yang dapat Anda baca. Jenisnya selesai ke mulai kecuali dipilih lain; jedanya adalah hari kerja bulat dari minus {max} sampai {max}, jeda negatif adalah mendahului. Ketergantungan tidak dibuat dua kali dengan cara yang sama, tidak dalam putaran antarrencana, dan tidak dengan hammock. Ketergantungan yang ujung lainnya tidak dapat Anda baca dihitung, tidak pernah disebutkan namanya, dan dikecualikan dari jadwal.",
  "p.crossplan.no_links": "Tugas ini belum memiliki ketergantungan.",
  "p.crossplan.this_before": "Tugas ini mendahului {other}",
  "p.crossplan.this_after": "Tugas ini menyusul {other}",
  "p.crossplan.end_own": "{title}, di rencana ini",
  "p.crossplan.end_external": "{title}, di {plan}",
  "p.crossplan.end_unnamed": "tugas yang tidak disebutkan di sini ({ref})",
  "p.crossplan.remove": "Hapus",
  "p.crossplan.remove_named": "Hapus ketergantungan pada {other}",
  "p.crossplan.restricted_one":
    "{n} ketergantungan lagi menghubungkan tugas yang tidak dapat Anda baca.",
  "p.crossplan.restricted_other":
    "{n} ketergantungan lagi menghubungkan tugas yang tidak dapat Anda baca.",
  "p.crossplan.see_network": "Lihat jadwal lintas rencana",
  "p.crossplan.form": "Tambah ketergantungan pada tugas rencana lain",
  "p.crossplan.f_order": "Urutan",
  "p.crossplan.order_before": "Tugas ini lebih dahulu",
  "p.crossplan.order_after": "Tugas yang lain lebih dahulu",
  "p.crossplan.f_plan": "Rencana lain",
  "p.crossplan.choose_plan": "Pilih rencana",
  "p.crossplan.f_task": "Tugas di rencana itu",
  "p.crossplan.choose_task": "Pilih tugas",
  "p.crossplan.f_kind": "Bagaimana keduanya terhubung",
  "p.crossplan.f_lag": "Jeda dalam hari kerja (0 jika kosong)",
  "p.crossplan.add": "Tambah ketergantungan",
  "p.crossplan.added": "Ketergantungan pada {title} ditambahkan.",
  "p.crossplan.removed": "Ketergantungan dihapus.",
  "p.crossplan.confirm_remove_before":
    "Hapus ketergantungan: {task} mendahului {other}? Tidak ada tugas yang diubah.",
  "p.crossplan.confirm_remove_after":
    "Hapus ketergantungan: {task} menyusul {other}? Tidak ada tugas yang diubah.",
  "p.crossplan.problem_plan": "Pilih rencana lain.",
  "p.crossplan.problem_task": "Pilih tugas di rencana itu.",
  "p.crossplan.problem_lag":
    "Jeda adalah bilangan bulat hari kerja dari minus {max} sampai {max}.",
  "p.crossplan.problem_self":
    "Tugas tidak dapat bergantung pada dirinya sendiri.",
  "p.crossplan.problem_exists": "Ketergantungan ini sudah ada.",
  "p.crossplan.refused_unreadable":
    "Salah satu dari kedua tugas sudah tidak ada, atau Anda tidak dapat membacanya.",
  "p.crossplan.refused_exists": "Ketergantungan ini sudah ada.",
  "p.crossplan.refused_add":
    "Layanan tidak mau menambah ketergantungan: mungkin sudah ada, membentuk putaran, atau menghubungkan hammock.",
  "p.crossplan.refused_gone":
    "Ketergantungan itu sudah tidak ada, atau Anda tidak dapat lagi membaca kedua tugas.",
  "p.crossplan.could_not_load": "Ketergantungan tugas ini tidak dapat dibaca.",
  "p.crossplan.could_not_load_plans":
    "Rencana untuk dipilih tidak dapat dibaca.",
  "p.crossplan.could_not_load_tasks": "Tugas rencana itu tidak dapat dibaca.",
  "p.crossplan.could_not_save": "Ketergantungan tidak dapat diubah.",
  "p.taskrollup.heading": "Subtugas yang diringkas",
  "p.taskrollup.says":
    "Dihitung setiap kali dari subtugas tugas ini, tidak pernah disimpan: tenggat terlambat di antaranya, poinnya dijumlahkan, kemajuan berbobot hanya dari yang diperkirakan, dan waktu yang dicatat pada semuanya.",
  "p.taskrollup.subtasks": "Subtugas",
  "p.taskrollup.done_of_one": "{done} dari {n} selesai",
  "p.taskrollup.done_of_other": "{done} dari {n} selesai",
  "p.taskrollup.due": "Tenggat terakhir",
  "p.taskrollup.points": "Poin dijumlahkan",
  "p.taskrollup.progress": "Kemajuan",
  "p.taskrollup.spent": "Waktu yang dicatat",
  "p.taskrollup.no_children": "Tugas ini tidak memiliki subtugas.",
  "p.taskrollup.no_dated_children":
    "Tidak ada subtugasnya yang memiliki tenggat.",
  "p.taskrollup.no_estimates": "Tidak ada subtugasnya yang diperkirakan.",
  "p.taskrollup.absent": "Tidak diketahui.",
  "p.taskrollup.unestimated_one":
    "{n} subtugas tidak memiliki perkiraan, jadi dikecualikan dari kemajuan (tidak dihitung sebagai nol):",
  "p.taskrollup.unestimated_other":
    "{n} subtugas tidak memiliki perkiraan, jadi dikecualikan dari kemajuan (tidak dihitung sebagai nol):",
  "p.taskrollup.unnamed": "Subtugas yang tidak disebutkan di sini ({ref})",
  "p.taskrollup.refused_gone":
    "Tugas ini sudah tidak ada, atau Anda tidak dapat lagi membacanya.",
  "p.taskrollup.could_not_load": "Subtugas yang diringkas tidak dapat dibaca.",
  "p.readings.heading": "Lebih lanjut tentang rencana ini",
  "p.readings.says": "Masing-masing dibaca hanya ketika Anda memintanya.",
  "p.readings.read": "Baca",
  "p.readings.could_not_read": "Ini tidak dapat dibaca.",
  "p.readings.not_measured": "tidak diukur",
  "p.readings.days": "{n} hari",
  "p.readings.rollup_heading": "Alur di seluruh rencana yang dimuatnya",
  "p.readings.rollup_says":
    "Tugas rencana ini dan setiap rencana di bawahnya, dihitung bersama, tidak dirata-ratakan; hingga {max} tingkat ke bawah jika dikosongkan.",
  "p.readings.f_depth": "Tingkat ke bawah",
  "p.readings.rollup_plans_one": "{n} rencana.",
  "p.readings.rollup_plans_other": "{n} rencana.",
  "p.readings.rollup_tasks_one":
    "{n} tugas: {finished} selesai, {wip} sedang dikerjakan, {not_started} belum dimulai.",
  "p.readings.rollup_tasks_other":
    "{n} tugas: {finished} selesai, {wip} sedang dikerjakan, {not_started} belum dimulai.",
  "p.readings.rollup_times":
    "Sebagian besar pekerjaan (85 dari 100) selesai dalam {cycle} sejak dimulai dan {lead} sejak diminta; waktu yang dipakai untuk bekerja, bukan menunggu: {efficiency}.",
  "p.readings.rollup_truncated":
    "Tidak setiap rencana di bawahnya dihitung: penelusuran mencapai batasnya.",
  "p.readings.rollup_revisits_one":
    "{n} rencana dicapai dua kali dan dihitung sekali.",
  "p.readings.rollup_revisits_other":
    "{n} rencana dicapai dua kali dan dihitung sekali.",
  "p.readings.rollup_table": "Alur menurut rencana",
  "p.readings.col_plan": "Rencana",
  "p.readings.col_depth": "Tingkat",
  "p.readings.col_tasks": "Tugas",
  "p.readings.col_finished": "Selesai",
  "p.readings.col_wip": "Sedang dikerjakan",
  "p.readings.col_efficiency": "Waktu bekerja, bukan menunggu",
  "p.readings.effort_heading": "Waktu yang dicatat",
  "p.readings.effort_says":
    "Waktu yang dicatat orang terhadap rencana ini, sebagaimana mereka menyatakannya; tidak ada yang memeriksanya.",
  "p.readings.read_effort": "Baca waktu yang dicatat",
  "p.readings.effort_none":
    "Tidak ada waktu yang dicatat terhadap rencana ini.",
  "p.readings.effort_total":
    "{total} dicatat, di antaranya {billable} dapat ditagih.",
  "p.readings.effort_split":
    "Belanja modal {capex}, belanja operasional {opex}, tidak diklasifikasikan {unclassified}.",
  "p.readings.effort_by_person": "Waktu menurut orang",
  "p.readings.effort_by_task": "Waktu menurut tugas",
  "p.readings.col_person": "Orang",
  "p.readings.col_time": "Waktu",
  "p.readings.col_billable": "Dapat ditagih",
  "p.readings.col_task": "Tugas",
  "p.readings.task_unnamed": "Tugas yang tidak disebutkan di sini ({ref})",
  "p.readings.forecast_heading": "Ramalan pelaksanaan",
  "p.readings.forecast_says":
    "Sepuluh ribu putaran dihasilkan dari berapa banyak yang diselesaikan rencana ini pada setiap periode lampau: berapa lama pekerjaan terbuka dapat berlangsung, dan berapa banyak yang dapat dikerjakan dalam beberapa periode. Kotak kosong adalah bawaan; diperlukan riwayat sedikitnya enam periode.",
  "p.readings.f_items": "Butir untuk diselesaikan (yang terbuka jika kosong)",
  "p.readings.f_periods": "Periode ke depan ({n} jika kosong)",
  "p.readings.f_period_days": "Hari dalam satu periode ({n} jika kosong)",
  "p.readings.f_history": "Periode riwayat ({n} jika kosong)",
  "p.readings.read_forecast": "Baca ramalan",
  "p.readings.forecast_history_one":
    "Dihasilkan dari {n} periode sepanjang {days} hari.",
  "p.readings.forecast_history_other":
    "Dihasilkan dari {n} periode sepanjang {days} hari.",
  "p.readings.forecast_batch_one":
    "{n} butir: separuh putaran selesai dalam {p50} hari, 85 dari 100 dalam {p85} hari, dan 95 dari 100 dalam {p95} hari.",
  "p.readings.forecast_batch_other":
    "{n} butir: separuh putaran selesai dalam {p50} hari, 85 dari 100 dalam {p85} hari, dan 95 dari 100 dalam {p95} hari.",
  "p.readings.forecast_no_batch":
    "Berapa lama pekerjaan dapat berlangsung tidak dapat diramalkan: riwayat terlalu sedikit, atau tidak ada yang selesai di dalamnya.",
  "p.readings.forecast_horizon":
    "Dalam {days} hari ke depan, sekitar {median} butir dapat dikerjakan: setidaknya {least} dalam 85 dari 100 putaran, dan paling banyak {most}.",
  "p.readings.forecast_no_horizon":
    "Berapa banyak yang dapat dikerjakan tidak dapat diramalkan: riwayat terlalu sedikit, atau tidak ada yang selesai di dalamnya.",
  "p.readings.performance_heading": "Jadwal, biaya, dan pemangku kepentingan",
  "p.readings.performance_says":
    "Kinerja jadwal adalah pekerjaan yang dikerjakan terhadap pekerjaan yang direncanakan sejauh ini, dan kinerja biaya adalah pekerjaan yang dikerjakan terhadap biayanya; di atas 100% lebih baik daripada rencana. Skor pemangku kepentingan berkisar dari minus 100 sampai 100.",
  "p.readings.read_performance": "Baca kinerja",
  "p.readings.schedule": "Kinerja jadwal",
  "p.readings.cost": "Kinerja biaya",
  "p.readings.stakeholders": "Skor pemangku kepentingan",
  "p.readings.nps_one":
    "{score}, dari {n} jawaban ({promoters} mendukung, {detractors} menjatuhkan)",
  "p.readings.nps_other":
    "{score}, dari {n} jawaban ({promoters} mendukung, {detractors} menjatuhkan)",
  "p.readings.nps_none": "Belum ada jawaban.",
  "p.readings.earned_value_link": "Lihat nilai yang diperoleh selengkapnya",
  "p.readings.absent_no_baseline":
    "Tidak diukur: rencana tidak memiliki garis dasar anggaran.",
  "p.readings.absent_no_weighted_tasks":
    "Tidak diukur: tidak ada tugas yang memiliki bobot.",
  "p.readings.absent_not_yet_defined":
    "Belum diukur: sejauh ini belum ada yang direncanakan atau dibelanjakan.",
  "p.readings.absent_several_currencies":
    "Bukan satu angka: rencana memiliki anggaran dalam beberapa mata uang; lihat nilai yang diperoleh.",
  "p.readings.absent_other": "Tidak diukur.",
  "p.readings.workflow_heading": "Alur kerja tugas",
  "p.readings.read_workflow": "Baca alur kerja tugas",
  "p.readings.workflow_plan":
    "Rencana ini memiliki alur kerja tugasnya sendiri.",
  "p.readings.workflow_built_in":
    "Rencana ini memakai alur kerja tugas bawaan.",
  "p.readings.workflow_constrained":
    "Hanya perpindahan yang tercantum yang diizinkan.",
  "p.readings.workflow_free":
    "Tugas boleh berpindah dari status apa pun ke status lain mana pun.",
  "p.readings.cat_todo": "belum dimulai",
  "p.readings.cat_active": "sedang dikerjakan",
  "p.readings.cat_waiting": "menunggu",
  "p.readings.cat_done": "selesai",
  "p.readings.cat_other": "jenis lain",
  "p.readings.initial": "tempat tugas dimulai",
  "p.readings.terminal": "tempat tugas berakhir",
  "p.readings.wip_limit": "paling banyak {n} sekaligus",
  "p.readings.anyone": "siapa saja",
  "p.readings.roles": "peran: {roles}",
  "p.readings.problem_depth":
    "Tingkat ke bawah adalah bilangan bulat dari 1 sampai {max}.",
  "p.readings.problem_depth_zero":
    "Tingkat ke bawah adalah bilangan bulat dari 0 sampai {max}.",
  "p.readings.problem_items":
    "Butir untuk diselesaikan adalah bilangan bulat minimal 1.",
  "p.readings.problem_periods":
    "Periode ke depan adalah bilangan bulat dari 1 sampai 520.",
  "p.readings.problem_period_days":
    "Hari dalam satu periode adalah bilangan bulat dari 1 sampai 90.",
  "p.readings.problem_history":
    "Periode riwayat adalah bilangan bulat dari 6 sampai 260.",
  "p.readings.problem_window":
    "Jendela adalah bilangan bulat hari dari 1 sampai 3650.",
  "p.readings.refused_query": "Layanan tidak mau menerima pertanyaan itu.",
  "p.readings.refused_module": "Bagian rencana ini dimatikan.",
  "p.readings.refused_gone":
    "Rencana ini sudah tidak ada, atau Anda tidak dapat lagi membacanya.",
  "p.readings.flow_feature": "Fitur baru",
  "p.readings.flow_defect": "Cacat",
  "p.readings.flow_risk": "Pekerjaan risiko",
  "p.readings.flow_debt": "Utang teknis",
  "p.readings.flow_unclassified": "Tidak diklasifikasikan",
  "p.readings.flow_other": "Jenis lain",
  "p.flowmix.heading": "Campuran pekerjaan yang selesai",
  "p.flowmix.says":
    "Bagaimana pekerjaan yang selesai dalam suatu jendela terbagi menjadi fitur baru, cacat, pekerjaan risiko, dan utang teknis, dibandingkan dengan campuran yang dimaksudkan rencana jika menyatakannya. Jendela adalah {n} hari jika dikosongkan.",
  "p.flowmix.f_window": "Hari ke belakang ({n} jika kosong)",
  "p.flowmix.f_rollup": "Sertakan rencana yang dimuatnya",
  "p.flowmix.f_depth": "Tingkat ke bawah ({max} jika kosong)",
  "p.flowmix.read": "Baca campuran",
  "p.flowmix.window_one": "Pekerjaan yang selesai dalam {n} hari terakhir.",
  "p.flowmix.window_other": "Pekerjaan yang selesai dalam {n} hari terakhir.",
  "p.flowmix.plans_one": "{n} rencana dihitung.",
  "p.flowmix.plans_other": "{n} rencana dihitung.",
  "p.flowmix.truncated":
    "Tidak setiap rencana di bawahnya dihitung: penelusuran mencapai batasnya.",
  "p.flowmix.none": "Tidak ada pekerjaan yang selesai dalam jendela itu.",
  "p.flowmix.no_intent":
    "Rencana tidak menyatakan campuran yang dimaksudkan, jadi tidak ada yang dibandingkan.",
  "p.flowmix.table": "Pekerjaan yang selesai menurut jenis",
  "p.flowmix.col_kind": "Jenis pekerjaan",
  "p.flowmix.col_count": "Selesai",
  "p.flowmix.col_share": "Porsi",
  "p.flowmix.col_intended": "Porsi yang dimaksudkan",
  "p.flowmix.col_gap": "Selisih",
  "p.ccpm.heading": "Perkiraan agresif dan aman",
  "p.ccpm.says":
    "Untuk rantai kritis, setiap tugas memerlukan perkiraan agresif (peluang kira-kira seimbang) dan perkiraan aman, dalam hari kerja bulat dari 1 sampai {max}, yang agresif tidak di atas yang aman. Angka yang ditampilkan adalah yang dicatat, atau jika tidak ada, milik rentang risiko, atau jika tidak ada, yang diturunkan aturan penerapan, dan masing-masing menyebutkan asalnya. Pencapaian dan tugas tanpa durasi tidak diperkirakan. Mengosongkan keduanya menghapus apa yang dicatat.",
  "p.ccpm.rule_explicit_only":
    "Aturan: hanya perkiraan yang diberikan yang dipakai; tidak ada yang diturunkan.",
  "p.ccpm.rule_half_of_safe":
    "Aturan: perkiraan aman yang hilang adalah durasi tugas, dan perkiraan agresif yang hilang adalah separuh dari yang aman.",
  "p.ccpm.rule_other": "Aturan: satu yang tidak dikenal halaman ini.",
  "p.ccpm.rule_unusable":
    "Pengaturan aturan penerapan tidak dapat dibaca, jadi hanya perkiraan yang diberikan yang dipakai.",
  "p.ccpm.attention_one": "{n} tugas memerlukan perkiraannya.",
  "p.ccpm.attention_other": "{n} tugas memerlukan perkiraannya.",
  "p.ccpm.more": "{shown} pertama dari {total} tugas ditampilkan.",
  "p.ccpm.no_tasks": "Rencana ini tidak memiliki tugas untuk diperkirakan.",
  "p.ccpm.table": "Perkiraan menurut tugas",
  "p.ccpm.col_task": "Tugas",
  "p.ccpm.col_likely": "Durasi dalam hari kerja",
  "p.ccpm.col_aggressive": "Agresif",
  "p.ccpm.col_safe": "Aman",
  "p.ccpm.col_state": "Kedudukan",
  "p.ccpm.figure": "{days} hari, {source}",
  "p.ccpm.none": "tidak ada",
  "p.ccpm.source_recorded": "dicatat",
  "p.ccpm.source_risk_range": "dari rentang risiko",
  "p.ccpm.source_rule": "menurut aturan",
  "p.ccpm.state_complete": "Keduanya diberikan",
  "p.ccpm.state_derived": "Sebagian diturunkan",
  "p.ccpm.state_missing": "Perkiraan hilang",
  "p.ccpm.state_inconsistent": "Agresif di atas aman",
  "p.ccpm.state_milestone": "Pencapaian, tidak diperkirakan",
  "p.ccpm.state_no_duration": "Tanpa durasi, tidak diperkirakan",
  "p.ccpm.state_other": "Kedudukan lain",
  "p.ccpm.form": "Catat perkiraan sebuah tugas",
  "p.ccpm.f_task": "Tugas",
  "p.ccpm.choose_task": "Pilih tugas",
  "p.ccpm.f_aggressive": "Agresif, dalam hari kerja",
  "p.ccpm.f_safe": "Aman, dalam hari kerja",
  "p.ccpm.save": "Simpan perkiraan",
  "p.ccpm.unchanged": "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.ccpm.saved": "Perkiraan untuk {title} disimpan.",
  "p.ccpm.cleared": "Perkiraan yang dicatat untuk {title} dikosongkan.",
  "p.ccpm.confirm_clear":
    "Kosongkan perkiraan yang dicatat untuk {title}? Angka dari rentang risiko atau aturan mungkin masih ditampilkan.",
  "p.ccpm.problem_task": "Pilih tugas.",
  "p.ccpm.problem_aggressive":
    "Perkiraan agresif adalah bilangan bulat hari kerja dari 1 sampai {max}.",
  "p.ccpm.problem_safe":
    "Perkiraan aman adalah bilangan bulat hari kerja dari 1 sampai {max}.",
  "p.ccpm.problem_order":
    "Perkiraan agresif tidak dapat di atas perkiraan aman.",
  "p.ccpm.refused_invalid":
    "Layanan tidak mau menerima perkiraan ini: tugas mungkin kehilangan durasinya atau menjadi pencapaian.",
  "p.ccpm.refused_gone":
    "Tugas ini sudah tidak ada, atau Anda tidak dapat lagi membacanya.",
  "p.ccpm.could_not_load": "Perkiraan tidak dapat dibaca.",
  "p.ccpm.could_not_save": "Perkiraan tidak dapat disimpan.",
  "p.frozen.heading": "Rencana penyangga yang dibekukan",
  "p.frozen.none": "Belum ada rencana penyangga yang dibekukan.",
  "p.frozen.freeze_link": "Bekukan satu pada manajemen penyangga",
  "p.frozen.taken": "Dibekukan pada {date}; {rule}",
  "p.frozen.table": "Penyangga sebagaimana dibekukan",
  "p.frozen.col_buffer": "Penyangga",
  "p.frozen.col_days": "Hari kerja",
  "p.frozen.col_chain": "Rantai",
  "p.frozen.col_finish": "Selesai terencana",
  "p.frozen.project": "Penyangga proyek",
  "p.frozen.feeding": "Penyangga pengumpan ke {joins}",
  "p.frozen.unsized": "belum diukur ukurannya",
  "p.frozen.untitled": "Tugas tanpa judul",
  "p.frozen.history_one": "{n} rencana penyangga dibekukan: {dates}.",
  "p.frozen.history_other":
    "{n} rencana penyangga dibekukan, yang terbaru lebih dahulu: {dates}.",
  "p.frozen.could_not_load":
    "Rencana penyangga yang dibekukan tidak dapat dibaca.",
  "p.workingtime.heading": "Hari dan minggu kerja",
  "p.workingtime.says":
    "Berapa lama satu hari kerja (1 menit sampai 24 jam) dan berapa hari seminggu yang dikerjakan (1 sampai 7), untuk rencana ini atau sebagai bawaan organisasi. Masing-masing adalah catatan baru yang tidak dapat diubah atau dihapus di sini, jadi Anda ditanya dahulu; yang kedua untuk rencana ini ditolak.",
  "p.workingtime.form": "Tetapkan hari dan minggu kerja",
  "p.workingtime.f_scope": "Untuk",
  "p.workingtime.scope_plan": "Rencana ini",
  "p.workingtime.scope_default": "Bawaan organisasi",
  "p.workingtime.f_hours": "Jam dalam sehari",
  "p.workingtime.f_minutes": "Dan menit",
  "p.workingtime.f_days": "Hari dalam seminggu",
  "p.workingtime.save": "Tetapkan hari kerja",
  "p.workingtime.confirm_plan_one":
    "Tetapkan hari kerja rencana ini menjadi {day}, {days} hari seminggu? Tidak dapat diubah di sini sesudahnya.",
  "p.workingtime.confirm_plan_other":
    "Tetapkan hari kerja rencana ini menjadi {day}, {days} hari seminggu? Tidak dapat diubah di sini sesudahnya.",
  "p.workingtime.confirm_default_one":
    "Tetapkan hari kerja bawaan organisasi menjadi {day}, {days} hari seminggu? Tidak dapat diubah di sini sesudahnya.",
  "p.workingtime.confirm_default_other":
    "Tetapkan hari kerja bawaan organisasi menjadi {day}, {days} hari seminggu? Tidak dapat diubah di sini sesudahnya.",
  "p.workingtime.saved_one":
    "Hari kerja ditetapkan menjadi {day}, {days} hari seminggu.",
  "p.workingtime.saved_other":
    "Hari kerja ditetapkan menjadi {day}, {days} hari seminggu.",
  "p.workingtime.away_heading": "Waktu di luar pekerjaan proyek",
  "p.workingtime.away_says":
    "Cuti, hari libur, cuti belajar, atau tugas di luar proyek, untuk satu orang dari hari pertama sampai hari terakhir, agar waktu itu tidak dibaca sebagai menganggur. Orang adalah rujukan seperti person: dan sebuah id. Tidak dapat diubah atau dihapus setelah dicatat, jadi Anda ditanya dahulu; catatan kosong tidak dikirim.",
  "p.workingtime.f_person": "Rujukan orang",
  "p.workingtime.f_starts": "Hari pertama",
  "p.workingtime.f_ends": "Hari terakhir",
  "p.workingtime.f_kind": "Jenis",
  "p.workingtime.f_note": "Catatan (opsional)",
  "p.workingtime.record": "Catat waktu di luar",
  "p.workingtime.confirm_away":
    "Catat {kind} untuk {person}, {range}? Tidak dapat diubah atau dihapus sesudahnya.",
  "p.workingtime.recorded": "Dicatat {kind} untuk {person}, {range}.",
  "p.workingtime.kind_leave": "cuti",
  "p.workingtime.kind_holiday": "hari libur",
  "p.workingtime.kind_study_leave": "cuti belajar",
  "p.workingtime.kind_non_project_duty": "tugas di luar proyek",
  "p.workingtime.problem_hours": "Jam adalah bilangan bulat dari 0 sampai 24.",
  "p.workingtime.problem_minutes":
    "Menit adalah bilangan bulat dari 0 sampai 59.",
  "p.workingtime.problem_day":
    "Hari kerja berlangsung dari 1 menit sampai 24 jam.",
  "p.workingtime.problem_week":
    "Hari dalam seminggu adalah bilangan bulat dari 1 sampai 7.",
  "p.workingtime.problem_person":
    "Berikan orang sebagai person: atau worker: dan sebuah id.",
  "p.workingtime.problem_starts": "Berikan hari pertama.",
  "p.workingtime.problem_ends": "Berikan hari terakhir.",
  "p.workingtime.problem_order":
    "Hari terakhir tidak boleh sebelum hari pertama.",
  "p.workingtime.refused_exists":
    "Rencana ini sudah memiliki hari kerja yang ditetapkan, dan tidak dapat diganti di sini.",
  "p.workingtime.refused_working": "Layanan tidak mau menerima hari kerja ini.",
  "p.workingtime.refused_non_working":
    "Layanan tidak mau menerima waktu di luar ini.",
  "p.workingtime.refused_forbidden": "Anda tidak boleh mencatat ini.",
  "p.workingtime.could_not_save": "Ini tidak dapat dicatat.",
  "p.task.is_hammock":
    "Tugas ini adalah hammock: tanggalnya berasal dari tugas yang dicakupnya.",
  "p.task.hammock_link": "Lihat cakupannya pada jalur kritis",
  "p.adoption.could_not": "Adopsi tidak dapat dicatat.",
  "p.adoption.f_active": "Pengguna aktif",
  "p.adoption.f_definition": "Apa yang dihitung sebagai pengguna aktif",
  "p.adoption.f_target": "Pengguna yang dituju",
  "p.adoption.f_window": "Selama berapa hari",
  "p.adoption.form": "Catat adopsi",
  "p.adoption.heading": "Berapa banyak orang yang memakainya",
  "p.adoption.problem_active":
    "Berikan pengguna aktif sebagai bilangan bulat, 0 atau lebih.",
  "p.adoption.problem_definition":
    "Sebutkan apa yang dihitung sebagai pengguna aktif, agar pembacaan berikutnya dapat dibandingkan.",
  "p.adoption.problem_target":
    "Berikan pengguna yang dituju sebagai bilangan bulat di atas 0.",
  "p.adoption.problem_window":
    "Berikan jendela sebagai bilangan bulat hari, 1 atau lebih.",
  "p.adoption.record": "Catat adopsi",
  "p.adoption.refused": "Layanan tidak menerima angka-angka ini.",
  "p.adoption.refused_forbidden":
    "Anda tidak boleh mencatat adopsi untuk rencana ini.",
  "p.adoption.refused_gone": "Rencana ini sudah tidak ada.",
  "p.adoption.saved": "Adopsi dicatat.",
  "p.adoption.says":
    "Pengguna aktif terhadap pengguna yang dituju, selama jendela hari. Apa yang dihitung sebagai aktif disimpan di samping angkanya, sehingga pembacaan berikutnya dapat dibandingkan.",
  "p.bulk.apply": "Terapkan pada tugas yang dipilih",
  "p.bulk.choose_all": "Pilih semua tugas",
  "p.bulk.choose_none": "Jangan pilih apa pun",
  "p.bulk.choose_plan": "Pilih rencana",
  "p.bulk.chosen_one": "{n} tugas dipilih",
  "p.bulk.chosen_other": "{n} tugas dipilih",
  "p.bulk.confirm_delete_one":
    "Hapus {n} tugas? Tugas keluar dari papan dan setiap tampilan rencana.",
  "p.bulk.confirm_delete_other":
    "Hapus {n} tugas? Tugas keluar dari papan dan setiap tampilan rencana.",
  "p.bulk.confirm_move_one":
    "Pindahkan {n} tugas ke {plan}? Sprint, komponen, dan tugas induknya dikosongkan, dan status yang tidak dimiliki rencana itu menjadi status pertamanya.",
  "p.bulk.confirm_move_other":
    "Pindahkan {n} tugas ke {plan}? Sprint, komponen, dan tugas induknya dikosongkan, dan status yang tidak dimiliki rencana itu menjadi status pertamanya.",
  "p.bulk.copy": "Salin ke rencana",
  "p.bulk.could_not": "Tugas tidak dapat diubah.",
  "p.bulk.could_not_load_plans": "Rencana tidak dapat dibaca.",
  "p.bulk.delete": "Hapus tugas yang dipilih",
  "p.bulk.delete_failed": "Tidak dihapus: {titles}.",
  "p.bulk.deleted_gone_one": "{n} tugas sudah tidak ada.",
  "p.bulk.deleted_gone_other": "{n} tugas sudah tidak ada.",
  "p.bulk.deleted_one": "{n} tugas dihapus.",
  "p.bulk.deleted_other": "{n} tugas dihapus.",
  "p.bulk.done_one": "Selesai untuk {done} dari {requested} tugas.",
  "p.bulk.done_other": "Selesai untuk {done} dari {requested} tugas.",
  "p.bulk.edit_form": "Tetapkan kolom pada tugas yang dipilih",
  "p.bulk.f_assignee": "Penerima tugas",
  "p.bulk.f_due": "Tenggat",
  "p.bulk.f_plan": "Rencana",
  "p.bulk.f_start": "Tanggal mulai",
  "p.bulk.f_value": "{field} baru",
  "p.bulk.heading": "Ubah banyak tugas sekaligus",
  "p.bulk.keep_assignee": "Pertahankan penerima tugas saat menyalin",
  "p.bulk.mapped_one":
    "{n} tugas memiliki status yang tidak dimiliki rencana itu, jadi dimulai pada status pertama rencana itu.",
  "p.bulk.mapped_other":
    "{n} tugas memiliki status yang tidak dimiliki rencana itu, jadi dimulai pada status pertama rencana itu.",
  "p.bulk.mode_clear": "Kosongkan",
  "p.bulk.mode_keep": "Biarkan apa adanya",
  "p.bulk.mode_set": "Tetapkan ke",
  "p.bulk.move": "Pindahkan ke rencana",
  "p.bulk.no_tasks": "Rencana ini tidak memiliki tugas.",
  "p.bulk.problem_assignee":
    "Berikan penerima tugas sebagai person: atau worker: dan sebuah id.",
  "p.bulk.problem_due": "Berikan tenggat sebagai tanggal yang nyata.",
  "p.bulk.problem_no_field":
    "Pilih setidaknya satu kolom untuk ditetapkan atau dikosongkan.",
  "p.bulk.problem_none": "Pilih setidaknya satu tugas.",
  "p.bulk.problem_plan": "Pilih rencana tujuan salin atau pindah.",
  "p.bulk.problem_same_plan":
    "Tugas sudah ada di rencana ini: pilih rencana lain untuk memindahkannya.",
  "p.bulk.problem_start":
    "Berikan tanggal mulai sebagai tanggal nyata dari {min} sampai {latest}.",
  "p.bulk.problem_too_many": "Pilih paling banyak {max} tugas sekaligus.",
  "p.bulk.refused_edit": "Layanan tidak menerima nilai ini.",
  "p.bulk.refused_forbidden": "Anda tidak boleh mengubah tugas ini.",
  "p.bulk.refused_plan_gone":
    "Rencana itu sudah tidak ada atau bukan rencana yang dapat Anda lihat.",
  "p.bulk.refused_titles":
    "Ditolak, dengan alasan yang hanya diberikan layanan dalam bahasa Inggris: {titles}.",
  "p.bulk.refused_transfer":
    "Rencana itu bukan rencana yang dapat Anda lihat, atau layanan tidak menerima daftarnya.",
  "p.bulk.says":
    "Pilih tugas, lalu tetapkan penerima tugas, tenggat, atau tanggal mulai yang sama pada semuanya, salin ke rencana atau pindahkan ke sana, atau hapus. Paling banyak {max} sekaligus; tanggal mulai berkisar dari {min} sampai {latest}. Status diubah di papan, satu kartu pada satu waktu. Salinan dimulai pada status pertama rencana itu dan mempertahankan penerima tugas hanya jika diminta; pemindahan mengosongkan sprint, komponen, dan tugas induk, dan tugas yang memiliki subtugas tidak dipindahkan.",
  "p.bulk.this_plan": "{name} (rencana ini)",
  "p.bulk.transfer_form": "Salin atau pindahkan tugas yang dipilih",
  "p.bulk.unchanged":
    "Tugas yang dipilih sudah memiliki nilai ini, jadi tidak ada yang dikirim.",
  "p.ceremony.choose_kind": "Pilih seremoni",
  "p.ceremony.could_not_load": "Seremoni sprint tidak dapat dibaca.",
  "p.ceremony.could_not_save": "Seremoni tidak dapat dicatat.",
  "p.ceremony.counts": "Seremoni yang telah berlangsung",
  "p.ceremony.f_facilitator":
    "Fasilitator (person: atau worker: dan sebuah id, opsional)",
  "p.ceremony.f_kind": "Seremoni",
  "p.ceremony.f_summary": "Apa yang diputuskan (opsional)",
  "p.ceremony.facilitated_by": "difasilitasi oleh {ref}",
  "p.ceremony.form": "Catat seremoni",
  "p.ceremony.heading": "Seremoni {name}",
  "p.ceremony.held_count_one": "{kind}: berlangsung {n} kali",
  "p.ceremony.held_count_other": "{kind}: berlangsung {n} kali",
  "p.ceremony.held_saved": "Dicatat: {kind}.",
  "p.ceremony.kind_daily": "Stand-up harian",
  "p.ceremony.kind_other": "Seremoni lain",
  "p.ceremony.kind_planning": "Perencanaan",
  "p.ceremony.kind_retrospective": "Retrospektif",
  "p.ceremony.kind_review": "Tinjauan",
  "p.ceremony.none_held": "{kind}: belum berlangsung",
  "p.ceremony.problem_facilitator":
    "Berikan fasilitator sebagai person: atau worker: dan sebuah id, atau kosongkan.",
  "p.ceremony.problem_kind": "Pilih seremoni.",
  "p.ceremony.problem_once":
    "Sprint ini sudah menyelenggarakan {kind}: yang kedua adalah perencanaan ulang, yang merupakan sprint baru.",
  "p.ceremony.record": "Catat seremoni",
  "p.ceremony.refused_ceremony":
    "Layanan menolaknya: sprint ini mungkin sudah menyelenggarakan seremoni itu.",
  "p.ceremony.refused_forbidden": "Anda tidak boleh mencatat ini untuk sprint.",
  "p.ceremony.refused_gone": "Sprint ini sudah tidak ada.",
  "p.ceremony.says":
    "Sprint memiliki satu perencanaan dan satu tinjauan, dan sejumlah stand-up harian dan retrospektif. Setiap jenis dihitung, termasuk yang belum berlangsung.",
  "p.checkins.heading": "Check-in untuk {title}",
  "p.checkins.none": "Belum ada check-in.",
  "p.checkins.row": "{date}: nilai {value}",
  "p.commitment.added_one": "{n} tugas ditambahkan sejak itu: {titles}.",
  "p.commitment.added_other": "{n} tugas ditambahkan sejak itu: {titles}.",
  "p.commitment.commit": "Komitmenkan tugas sprint",
  "p.commitment.confirm_one":
    "Komitmenkan {n} tugas di {name} sekarang? Dilakukan sekali dan tidak dapat diubah; tugas yang ditambahkan atau dihapus kemudian ditampilkan sebagai perubahan.",
  "p.commitment.confirm_other":
    "Komitmenkan {n} tugas di {name} sekarang? Dilakukan sekali dan tidak dapat diubah; tugas yang ditambahkan atau dihapus kemudian ditampilkan sebagai perubahan.",
  "p.commitment.could_not": "Sprint tidak dapat dikomitmenkan.",
  "p.commitment.heading": "Apa yang dikomitmenkan sprint",
  "p.commitment.not_yet_one":
    "Belum dikomitmenkan; {n} tugas ada di sprint sekarang.",
  "p.commitment.not_yet_other":
    "Belum dikomitmenkan; {n} tugas ada di sprint sekarang.",
  "p.commitment.problem_done":
    "Sprint ini sudah dikomitmenkan: komitmen tidak pernah ditulis ulang.",
  "p.commitment.problem_empty":
    "Sprint ini tidak memiliki tugas untuk dikomitmenkan.",
  "p.commitment.problem_unread": "Komitmen sprint belum dibaca.",
  "p.commitment.refused_commit":
    "Layanan menolaknya: sprint ini sudah dikomitmenkan.",
  "p.commitment.removed_one": "{n} tugas dihapus sejak itu: {titles}.",
  "p.commitment.removed_other": "{n} tugas dihapus sejak itu: {titles}.",
  "p.commitment.saved_one": "{n} tugas dikomitmenkan.",
  "p.commitment.saved_other": "{n} tugas dikomitmenkan.",
  "p.commitment.says":
    "Tugas sprint sebagaimana adanya saat dikomitmenkan, diambil sekali, sehingga tugas yang ditambahkan atau dihapus kemudian terbaca sebagai perubahan dan bukan sebagai rencana sejak awal.",
  "p.commitment.state":
    "{committed} dikomitmenkan; {current} ada di sprint sekarang.",
  "p.commitment.unchanged":
    "Tidak ada yang ditambahkan atau dihapus sejak itu.",
  "p.hours.col_actual": "Dicatat",
  "p.hours.col_placeholder": "Di antaranya belum ditugaskan",
  "p.hours.col_plan": "Rencana",
  "p.hours.col_planned": "Direncanakan",
  "p.hours.col_variance": "Dicatat dikurangi direncanakan",
  "p.hours.could_not": "Jam tidak dapat dibaca.",
  "p.hours.f_from": "Dari",
  "p.hours.f_to": "Sampai",
  "p.hours.form": "Jam yang direncanakan terhadap yang dicatat",
  "p.hours.fte_says":
    "Ekuivalen waktu penuh adalah persen-hari dibagi 100 kali jumlah hari dalam periode; selisih adalah pasokan dikurangi permintaan (di bawah nol berarti kurang). Pemesanan yang diusulkan adalah rangkaian terpisah.",
  "p.hours.h": "{h} jam",
  "p.hours.heading": "Jam yang direncanakan terhadap yang dicatat",
  "p.hours.masked":
    "Orang tidak ditampilkan: Anda tidak boleh mengelola mereka.",
  "p.hours.none":
    "Tidak ada rencana yang memiliki jam terencana atau tercatat dalam jendela ini.",
  "p.hours.nothing_recorded": "tidak ada yang dicatat",
  "p.hours.problem_from":
    "Berikan hari pertama sebagai tanggal nyata, atau kosongkan.",
  "p.hours.problem_order": "Hari terakhir tidak boleh sebelum hari pertama.",
  "p.hours.problem_span": "Jendela paling lama {max} hari.",
  "p.hours.problem_to":
    "Berikan hari terakhir sebagai tanggal nyata, atau kosongkan.",
  "p.hours.read": "Baca jam",
  "p.hours.refused": "Layanan tidak menerima jendela ini.",
  "p.hours.says":
    "Terencana adalah porsi pemesanan bernama yang dikomitmenkan pada setiap hari kerja; tercatat adalah waktu yang dimasukkan orang, disetujui atau tidak. Kosongkan hari untuk empat minggu sampai hari ini; paling banyak {max} hari.",
  "p.hours.table": "Jam menurut rencana",
  "p.hours.totals": "Terencana {planned}; tercatat {actual}.",
  "p.hours.truncated":
    "Hanya entri pertama yang dibaca; jumlah mungkin rendah.",
  "p.hours.unplanned": "{plan}: {hours} dicatat",
  "p.hours.unplanned_heading":
    "Waktu yang dicatat oleh orang tanpa pemesanan pada rencana",
  "p.hours.unplanned_people_one": "({n} orang)",
  "p.hours.unplanned_people_other": "({n} orang)",
  "p.hours.window": "Dari {from} sampai {to}.",
  "p.kpireadings.col_standing": "Kedudukan",
  "p.kpireadings.counted": "dihitung",
  "p.kpireadings.replaced":
    "diganti oleh pembacaan yang lebih baru pada hari itu",
  "p.kpireadings.says":
    "Setiap pembacaan, yang terbaru lebih dahulu. Pembacaan yang lebih baru pada hari yang sama menggantikan yang lebih awal, yang tetap disimpan.",
  "p.live.burndown_says":
    "Tugas yang tersisa setiap hari, dari penyelesaian yang dicatat; tidak ada garis ideal yang digambar dan tidak ada yang diisi.",
  "p.live.event_comment_added": "komentar ditambahkan",
  "p.live.event_other": "sebuah perubahan",
  "p.live.event_raci_updated": "siapa mengerjakan apa berubah",
  "p.live.event_resync": "papan dibaca ulang seluruhnya",
  "p.live.event_task_created": "tugas ditambahkan",
  "p.live.event_task_moved": "tugas dipindahkan",
  "p.live.event_task_updated": "tugas diubah",
  "p.live.follow": "Ikuti perubahan saat terjadi",
  "p.live.heading": "Perubahan saat terjadi",
  "p.live.last_change": "Terakhir: {change} pada {time}.",
  "p.live.says":
    "Selama diikuti, papan dibaca ulang setiap kali seseorang menambah, memindahkan, atau mengubah tugas di sini. Berhenti saat Anda meninggalkan halaman.",
  "p.live.state_connecting": "Menghubungkan…",
  "p.live.state_off": "Tidak mengikuti.",
  "p.live.state_on": "Mengikuti.",
  "p.live.state_retrying": "Koneksi terputus; mencoba lagi.",
  "p.live.state_stopped": "Berhenti: layanan tidak mau mengirim perubahan.",
  "p.live.stop": "Berhenti mengikuti",
  "p.live.velocity_says":
    "Tugas dan poin yang selesai pada setiap sprint, dihitung dari papan; ini bukan ukuran alur.",
  "p.placeholder.choose": "Pilih posisi sementara",
  "p.placeholder.confirm_over":
    "Memberikan ini kepada {person} akan membuatnya melebihi kapasitas pada beberapa hari. Tetap lakukan?",
  "p.placeholder.confirm_remove":
    "Hapus posisi sementara ini: {what}? Permintaan yang diwakilinya ikut hilang.",
  "p.placeholder.could_not": "Posisi sementara tidak dapat diubah.",
  "p.placeholder.f_person": "Orang (person: atau worker: dan sebuah id)",
  "p.placeholder.f_placeholder": "Posisi sementara",
  "p.placeholder.form": "Berikan posisi sementara kepada seseorang",
  "p.placeholder.give": "Berikan kepada orang itu",
  "p.placeholder.given": "Diberikan kepada {person}.",
  "p.placeholder.heading":
    "Berikan posisi sementara kepada seseorang, atau hapus",
  "p.placeholder.not_given": "Tidak diberikan: tidak ada yang diubah.",
  "p.placeholder.open": "terbuka",
  "p.placeholder.option": "{role} di {plan}, {percent}, {from} sampai {to}",
  "p.placeholder.problem_choose": "Pilih posisi sementara.",
  "p.placeholder.problem_person":
    "Berikan orang sebagai person: atau worker: dan sebuah id.",
  "p.placeholder.refused_forbidden":
    "Hanya administrator atau manajer sumber daya yang dapat melakukan ini.",
  "p.placeholder.refused_gone": "Pemesanan itu sudah tidak ada.",
  "p.placeholder.refused_named":
    "Pemesanan ini sudah menyebut seseorang; daftar telah dibaca ulang.",
  "p.placeholder.refused_person": "Layanan tidak menerima orang itu.",
  "p.placeholder.remove": "Hapus posisi sementara",
  "p.placeholder.removed": "Posisi sementara dihapus.",
  "p.placeholder.says":
    "Posisi sementara adalah peran yang diminta tanpa orang tertentu. Memberikannya kepada seseorang adalah keputusan manajer sumber daya; jika akan membuat orang itu melebihi kapasitas, Anda ditanya dahulu.",
  "p.planokr.as_of": "dihitung {at}",
  "p.planokr.choose_plan": "Pilih rencana",
  "p.planokr.could_not": "Sasaran rencana tidak dapat dibaca.",
  "p.planokr.could_not_load_plans": "Rencana tidak dapat dibaca.",
  "p.planokr.currencies": "hasil kunci dalam lebih dari satu mata uang",
  "p.planokr.f_plan": "Rencana",
  "p.planokr.form": "Skor sasaran rencana",
  "p.planokr.heading": "Skor sasaran rencana",
  "p.planokr.its_key_results": "Hasil kunci {title}",
  "p.planokr.key_results_one": "{n} hasil kunci",
  "p.planokr.key_results_other": "{n} hasil kunci",
  "p.planokr.left_out_one":
    "{n} tujuan tidak diukur dan dikecualikan dari skor.",
  "p.planokr.left_out_other":
    "{n} tujuan tidak diukur dan dikecualikan dari skor.",
  "p.planokr.no_objectives":
    "Tidak ada tujuan yang diselaraskan dengan rencana ini.",
  "p.planokr.not_measured":
    "Tidak diukur: belum ada tujuan yang diselaraskan dengannya yang dapat diukur.",
  "p.planokr.objective_not_measured": "tidak diukur",
  "p.planokr.problem_plan": "Pilih rencana.",
  "p.planokr.read": "Baca skor",
  "p.planokr.refused_gone": "Rencana atau tujuan itu sudah tidak ada.",
  "p.planokr.says":
    "Skor setiap tujuan yang diselaraskan dengan rencana, dibobot menurut seberapa kuat keselarasannya. Tujuan yang tidak dapat diukur oleh apa pun dikecualikan, tidak pernah dihitung sebagai 0%.",
  "p.planokr.score": "Skor {score}",
  "p.planokr.unmeasurable_no_range":
    "tanpa kemajuan: titik awal dan targetnya sama",
  "p.planokr.unmeasurable_no_tolerance":
    "tanpa kemajuan: dijaga tetap tanpa toleransi",
  "p.planokr.unmeasurable_other": "kemajuan tidak dapat dihitung",
  "p.planokr.unmeasurable_overflow": "tanpa kemajuan: angkanya terlalu besar",
  "p.planokr.unnamed": "Sebuah tujuan ({id})",
  "p.planokr.weight": "bobot {weight}",
  "p.rate.answer_person":
    "{rate} per jam pada {date}, dari tabel milik orang itu sendiri, berlaku sejak {from}.",
  "p.rate.answer_role":
    "{rate} per jam pada {date}, dari tabel peran mereka, berlaku sejak {from}.",
  "p.rate.could_not": "Tarif tidak dapat dibaca.",
  "p.rate.f_date": "Pada",
  "p.rate.f_person": "Orang",
  "p.rate.f_table": "Tabel",
  "p.rate.form": "Tarif biaya yang berlaku",
  "p.rate.heading": "Tarif biaya yang berlaku",
  "p.rate.none": "Tidak ada tarif yang berlaku untuk orang ini pada {date}.",
  "p.rate.problem_date": "Berikan hari sebagai tanggal nyata.",
  "p.rate.problem_person": "Berikan rujukan orang.",
  "p.rate.read": "Baca tarif",
  "p.rate.refused_hidden":
    "Tidak ada tarif yang dapat Anda lihat: tarif hanya ditampilkan kepada keuangan.",
  "p.rate.refused_query": "Layanan tidak menerima pertanyaan ini.",
  "p.rate.says":
    "Tarif per jam yang berlaku bagi seseorang pada suatu hari: milik tabelnya sendiri, atau jika tidak ada, milik perannya. Tabel standar kecuali Anda menyebut yang lain.",
  "p.skillgap.choose_plan": "Pilih rencana",
  "p.skillgap.col_person": "Orang",
  "p.skillgap.col_skill": "Keterampilan",
  "p.skillgap.col_status": "Kedudukan",
  "p.skillgap.could_not": "Kesenjangan keterampilan tidak dapat dibaca.",
  "p.skillgap.count_covered_one": "{n} keterampilan dimiliki;",
  "p.skillgap.count_covered_other": "{n} keterampilan dimiliki;",
  "p.skillgap.count_missing_one": "{n} keterampilan kurang;",
  "p.skillgap.count_missing_other": "{n} keterampilan kurang;",
  "p.skillgap.count_unknown_one": "{n} tidak diketahui.",
  "p.skillgap.count_unknown_other": "{n} tidak diketahui.",
  "p.skillgap.f_plan": "Rencana",
  "p.skillgap.form": "Kesenjangan keterampilan rencana",
  "p.skillgap.heading": "Keterampilan yang diminta pemesanan",
  "p.skillgap.none":
    "Tidak ada pemesanan pada rencana ini yang meminta keterampilan.",
  "p.skillgap.problem_plan": "Pilih rencana.",
  "p.skillgap.read": "Baca kesenjangan keterampilan",
  "p.skillgap.says":
    "Setiap keterampilan yang diminta pemesanan pada rencana, terhadap apa yang dimiliki orangnya di layanan tenaga kerja. Tidak diketahui berarti keterampilan mereka tidak dapat dibaca; posisi sementara belum memiliki orang dan dikecualikan.",
  "p.skillgap.status_covered": "dimiliki",
  "p.skillgap.status_missing": "kurang",
  "p.skillgap.status_unknown": "tidak diketahui",
  "p.skillgap.table": "Keterampilan menurut pemesanan",
  "p.util.by_person": "Menurut orang",
  "p.util.by_team": "Tim",
  "p.util.col_available": "Tersedia",
  "p.util.col_away": "Tidak hadir",
  "p.util.col_effort": "Upaya",
  "p.util.col_figure": "Penggunaan kapasitas",
  "p.util.col_person": "Orang",
  "p.util.could_not": "Penggunaan kapasitas tidak dapat dibaca.",
  "p.util.every_plan": "Setiap rencana",
  "p.util.f_by": "Tampilkan",
  "p.util.f_plan": "Rencana",
  "p.util.f_window": "Hari ke belakang dari hari ini",
  "p.util.form": "Upaya terhadap kapasitas",
  "p.util.heading": "Upaya terhadap kapasitas",
  "p.util.none":
    "Tidak ada yang mencatat upaya atau waktu tidak hadir dalam jendela ini.",
  "p.util.over": "pada atau melebihi kapasitas: peringatan, bukan pencapaian",
  "p.util.people_counted_one": "{n} orang dihitung.",
  "p.util.people_counted_other": "{n} orang dihitung.",
  "p.util.problem_window":
    "Berikan jendela sebagai bilangan bulat hari dari 1 sampai {max}.",
  "p.util.read": "Baca penggunaan kapasitas",
  "p.util.refused": "Layanan tidak menerima pertanyaan ini.",
  "p.util.says":
    "Upaya yang dicatat terhadap kapasitas yang dinyatakan, dikurangi waktu tidak hadir, selama {days} hari sampai hari ini kecuali Anda memilih 1 sampai {max}. Pada atau melebihi kapasitas adalah peringatan tentang antrean. Kelompok kecil dan angka kecil ditahan.",
  "p.util.suppression":
    "Ditahan: kelompok di bawah {people} orang, atau angka di bawah {hours}.",
  "p.util.table": "Penggunaan kapasitas menurut orang",
  "p.util.team_detail":
    "{effort} dari {available} tersedia, {away} tidak hadir",
  "p.util.unavailable_all_away": "tidak hadir sepanjang jendela, bukan 0%",
  "p.util.unavailable_below_floor": "terlalu sedikit untuk dilaporkan",
  "p.util.unavailable_no_capacity": "tidak ada kapasitas yang dinyatakan",
  "p.util.unavailable_other": "tidak dapat dilaporkan",
  "p.util.unavailable_small_group": "terlalu sedikit orang untuk ditampilkan",
  "p.util.window_one": "Selama {n} hari terakhir.",
  "p.util.window_other": "Selama {n} hari terakhir.",
  "p.jobs.title": "Impor dan ekspor",
  "p.jobs.title_ppm": "Impor dan ekspor — PPM",
  "p.jobs.intro":
    "Impor dan ekspor massal rencana, yang terbaru lebih dulu. Pekerjaan berjalan di latar belakang: baca daftar lagi untuk melihat kemajuannya.",
  "p.jobs.loading": "Memuat…",
  "p.jobs.could_not_load": "Tidak dapat memuat pekerjaan.",
  "p.jobs.none": "Tidak ada pekerjaan yang dapat Anda lihat.",
  "p.jobs.refresh": "Baca lagi",
  "p.jobs.filter_kind": "Jenis",
  "p.jobs.filter_status": "Status",
  "p.jobs.any": "apa saja",
  "p.jobs.kind_import": "impor",
  "p.jobs.kind_export": "ekspor",
  "p.jobs.fmt_jsonl": "baris JSON",
  "p.jobs.status_queued": "dalam antrean",
  "p.jobs.status_running": "berjalan",
  "p.jobs.status_completed": "selesai",
  "p.jobs.status_completed_with_errors": "selesai dengan galat",
  "p.jobs.status_failed": "gagal",
  "p.jobs.status_cancelled": "dibatalkan",
  "p.jobs.status_unknown": "status tidak diketahui",
  "p.jobs.started": "Dimulai {at}",
  "p.jobs.not_started": "Belum dimulai",
  "p.jobs.finished": "selesai {at}",
  "p.jobs.progress": "{percent} selesai",
  "p.jobs.rows":
    "{total} baris: {created} dibuat, {updated} diperbarui, {unchanged} tidak berubah, {review} ditahan untuk ditinjau, {failed} gagal.",
  "p.jobs.not_reconciled": "Hitungan ini tidak berjumlah total.",
  "p.jobs.held":
    "Baris yang ditahan untuk ditinjau tercantum di bawah pada “Ditahan untuk ditinjau”.",
  "p.jobs.stopping":
    "Diminta berhenti: pekerjaan menyelesaikan baris saat ini dan menyimpan yang sudah diproses.",
  "p.jobs.errors": "Laporan galat (CSV)",
  "p.jobs.download": "Unduh ekspor",
  "p.jobs.cancel": "Batalkan",
  "p.jobs.cancel_named": "Batalkan pekerjaan {kind}",
  "p.jobs.cancel_ask":
    "Batalkan pekerjaan ini? Pekerjaan dalam antrean tidak dimulai; yang sedang berjalan menyelesaikan baris saat ini dan menyimpan yang sudah diproses.",
  "p.jobs.nothing_to_cancel":
    "Tidak ada yang dibatalkan: pekerjaan sudah selesai atau sudah diminta berhenti.",
  "p.jobs.could_not_cancel": "Tidak dapat membatalkan pekerjaan.",
  "p.jobs.gone": "Pekerjaan itu sudah tidak ada.",
  "p.jobs.export_h": "Ekspor rencana",
  "p.jobs.export_rules":
    "Hanya rencana yang dapat Anda baca, paling banyak 100.000. Rujukan orang disamarkan kecuali Anda meminta ekspor penuh, yang memerlukan hak destruktif.",
  "p.jobs.format": "Format berkas",
  "p.jobs.full": "Ekspor penuh, dengan rujukan orang",
  "p.jobs.kind_filter": "Jenis rencana (opsional)",
  "p.jobs.name_filter": "Nama berisi (opsional)",
  "p.jobs.start_export": "Mulai ekspor",
  "p.jobs.export_forbidden": "Ekspor penuh memerlukan hak destruktif.",
  "p.jobs.export_refused":
    "Layanan menolak ekspor: persempit filter (paling banyak 100.000 rencana).",
  "p.jobs.could_not_export": "Tidak dapat memulai ekspor.",
  "p.jobs.import_h": "Impor rencana",
  "p.jobs.import_rules":
    "Satu rencana per baris (baris JSON) atau per baris CSV, paling banyak 100.000 baris; instalasi membatasi ukuran berkas. Mengirim berkas yang sama lagi tidak memulai apa pun yang baru.",
  "p.jobs.file": "Berkas",
  "p.jobs.keyless":
    "Baris tanpa pengenal yang tampak seperti rencana yang sudah ada",
  "p.jobs.keyless_review": "ditahan untuk ditinjau",
  "p.jobs.keyless_create": "tetap dibuat",
  "p.jobs.start_import": "Mulai impor",
  "p.jobs.no_file": "Pilih berkas lebih dulu.",
  "p.jobs.too_big": "Berkas melebihi batas ukuran instalasi.",
  "p.jobs.import_refused":
    "Layanan menolak berkas: kosong, terlalu besar, atau tidak dalam format yang dipilih.",
  "p.jobs.could_not_import": "Tidak dapat memulai impor.",
  "p.jobs.queued_ok": "Dimulai: pekerjaan ada di atas daftar.",
  "p.jobs.replayed": "Berkas ini sudah dikirim; pekerjaannya ada di daftar.",
  "p.jobs.review_h": "Ditahan untuk ditinjau",
  "p.jobs.review_intro":
    "Baris yang tampak seperti rencana yang sudah ada ditahan, tidak dibuat. Putuskan masing-masing: buat sebagai rencana baru, atau buang sebagai duplikat. Menggabungkannya ke rencana yang mirip tidak ditawarkan di sini: buang, lalu gabungkan rencana dari daftar rencana.",
  "p.jobs.review_none": "Tidak ada yang ditahan untuk ditinjau.",
  "p.jobs.could_not_load_review":
    "Tidak dapat memuat baris yang ditahan untuk ditinjau.",
  "p.jobs.review_line": "Baris {line} berkas",
  "p.jobs.review_matches": "Mirip dengan:",
  "p.jobs.review_match": "{score} mirip, keyakinan {confidence}",
  "p.jobs.confidence_high": "tinggi",
  "p.jobs.confidence_medium": "sedang",
  "p.jobs.confidence_low": "rendah",
  "p.jobs.confidence_unknown": "tidak diketahui",
  "p.jobs.review_unnamed": "(tanpa nama)",
  "p.jobs.review_create": "Ini baru: buat",
  "p.jobs.review_discard": "Ini duplikat: buang",
  "p.jobs.review_create_ask": "Buat “{name}” sebagai rencana baru?",
  "p.jobs.review_discard_ask": "Buang “{name}”? Tidak ada yang dibuat darinya.",
  "p.jobs.review_created": "“{name}” dibuat.",
  "p.jobs.review_discarded": "“{name}” dibuang.",
  "p.jobs.review_already": "Baris itu sudah diputuskan.",
  "p.jobs.review_invalid":
    "Baris yang ditahan bukan lagi rencana yang sah, jadi tidak dapat dibuat.",
  "p.jobs.review_gone": "Baris itu sudah tidak ada.",
  "p.jobs.could_not_decide": "Tidak dapat mencatat keputusan.",
  "p.jobs.review_show": "Tampilkan baris yang ditahannya",
  "p.jobs.review_all": "Tampilkan semua baris yang ditahan",
  "p.jobs.review_one_job": "Menampilkan baris yang ditahan oleh satu impor.",
  "p.jobs.review_more": "Hanya 500 yang pertama yang dicantumkan.",
  "p.plan_export.record_heading": "Catatan rencana (JSON)",
  "p.plan_export.record_intro":
    "Semua yang tersimpan tentang rencana, sebagai JSON, untuk permintaan catatan atau akses. Bila akses Anda menyamarkan orang, siapa yang memimpin dan memilikinya disamarkan dan berkas menyatakannya. Setiap ekspor seperti ini dicatat di jejak audit rencana.",
  "p.plan_export.record_download": "Unduh catatan rencana",
  "p.template_sync.link": "Perubahan templat",
  "p.template_sync.title": "Perubahan templat",
  "p.template_sync.title_ppm": "Perubahan templat — PPM",
  "p.template_sync.loading": "Memuat…",
  "p.template_sync.could_not_load": "Tidak dapat memuat perubahan templat.",
  "p.template_sync.not_from_template":
    "Rencana ini tidak dibuat dari templat yang disimpan, jadi tidak ada yang perlu diperbarui.",
  "p.template_sync.made_from": "Dibuat dari templat “{name}”.",
  "p.template_sync.versions":
    "Diperbarui ke versi {synced}; templat ada di versi {version}.",
  "p.template_sync.intro":
    "Setiap perubahan di bawah berasal dari templat sejak rencana ini terakhir diperbarui. Centang yang akan diambil; sisanya tetap menunggu. Perubahan pada tugas yang telah diubah atau dihapus oleh orang-orang rencana ini hanya diterapkan jika Anda juga mencentang “Timpa suntingan rencana ini”.",
  "p.template_sync.up_to_date":
    "Mutakhir: tidak ada perubahan yang belum diambil rencana ini.",
  "p.template_sync.kind_add": "ditambahkan",
  "p.template_sync.kind_change": "diubah",
  "p.template_sync.kind_remove": "dihapus",
  "p.template_sync.kind_unknown": "perubahan tidak diketahui",
  "p.template_sync.take": "Ambil perubahan ini",
  "p.template_sync.edited":
    "Orang-orang rencana ini telah mengubah atau menghapus tugas ini.",
  "p.template_sync.overwrite": "Timpa suntingan rencana ini",
  "p.template_sync.diff_title": "Judul: {from} → {to}",
  "p.template_sync.diff_description": "Deskripsi: {from} → {to}",
  "p.template_sync.diff_points": "Poin: {from} → {to}",
  "p.template_sync.none_value": "tidak ada",
  "p.template_sync.apply": "Terapkan perubahan yang dipilih",
  "p.template_sync.apply_ask":
    "Terapkan perubahan yang dipilih ke rencana ini? Tugas ditambahkan, diubah, atau dihapus seperti tercantum, dan perubahan dicatat.",
  "p.template_sync.choose_first":
    "Centang setidaknya satu perubahan lebih dulu.",
  "p.template_sync.applied": "Diterapkan: {n}.",
  "p.template_sync.overwritten":
    "Dari itu, menimpa suntingan rencana ini: {n}.",
  "p.template_sync.remaining": "Masih menunggu: {n}.",
  "p.template_sync.refused_edited":
    "“{title}” tidak diterapkan: rencana ini telah mengubahnya; centang “Timpa suntingan rencana ini” untuk tetap menerapkannya.",
  "p.template_sync.refused_removed":
    "“{title}” tidak diterapkan: rencana ini telah menghapus tugas itu, jadi tidak ada yang diubah.",
  "p.template_sync.could_not_apply": "Tidak dapat menerapkan perubahan.",
  "p.template_sync.apply_refused":
    "Layanan menolak perubahan: templat mungkin telah berubah sementara itu. Baca perubahan lagi.",
  "p.template_sync.plans_h": "Rencana yang dibuat darinya",
  "p.template_sync.plans_none":
    "Tidak ada rencana yang dapat Anda baca yang dibuat darinya.",
  "p.template_sync.plans_row": "versi {synced} dari {version}",
  "p.template_sync.waiting": "perubahan menunggu: {n}",
  "p.template_sync.plan_up_to_date": "mutakhir",
  "p.template_sync.open_changes": "Lihat perubahan",
  "p.template_sync.could_not_load_plans":
    "Tidak dapat memuat rencana yang dibuat darinya.",
  "p.template_sync.revise_h": "Revisi dari sebuah rencana",
  "p.template_sync.revise_says":
    "Tugas-tugas pohon rencana yang dipilih menjadi versi berikutnya dari templat. Pohon harus mempertahankan bentuk templat: rencana yang sama di tempat yang sama, dengan rencana baru hanya di akhir. Rencana yang dibuat dari templat tidak diubah; masing-masing melihat perbedaannya di halaman Perubahan templat.",
  "p.template_sync.revise_plan": "Pohon rencana",
  "p.template_sync.revise_choose": "Pilih rencana",
  "p.template_sync.revise": "Revisi templat",
  "p.template_sync.revise_ask":
    "Jadikan tugas-tugas “{name}” versi berikutnya dari “{template}”?",
  "p.template_sync.revised": "Sekarang versi {version}.",
  "p.template_sync.revised_omitted":
    "Rencana di pohon yang tidak dapat Anda baca dilewati: {n}.",
  "p.template_sync.revise_shape":
    "Pohon rencana tidak lagi berbentuk seperti templat: revisi dapat mengubah tugas dan menambah rencana di akhir, bukan memindahkan atau mengganti nama rencana.",
  "p.template_sync.could_not_revise": "Tidak dapat merevisi templat.",
  "p.template_sync.revise_pick": "Pilih pohon rencana lebih dulu.",
  "p.mspdi_batch.title": "Impor beberapa berkas MS Project",
  "p.mspdi_batch.title_ppm": "Impor beberapa berkas MS Project — PPM",
  "p.mspdi_batch.intro":
    "Masukkan hingga sepuluh berkas MS Project (MSPDI XML) sekaligus. Masing-masing masuk ke rencana yang Anda pilih atau ke rencana baru. Langkah pertama selalu pratinjau yang tidak menulis apa pun; tombol yang sama lalu mengimpor.",
  "p.mspdi_batch.files": "Berkas (XML MS Project, paling banyak sepuluh)",
  "p.mspdi_batch.too_many": "Pilih paling banyak sepuluh berkas.",
  "p.mspdi_batch.none_chosen": "Pilih setidaknya satu berkas lebih dulu.",
  "p.mspdi_batch.target": "Ke",
  "p.mspdi_batch.target_new": "rencana baru",
  "p.mspdi_batch.target_existing": "rencana yang sudah ada",
  "p.mspdi_batch.new_name":
    "Nama rencana baru (opsional: nama proyek dalam berkas)",
  "p.mspdi_batch.existing_plan": "Rencana",
  "p.mspdi_batch.choose_plan": "Pilih rencana",
  "p.mspdi_batch.choose_plan_first": "Pilih rencana untuk “{file}” lebih dulu.",
  "p.mspdi_batch.parent": "Letakkan rencana baru di bawah (opsional)",
  "p.mspdi_batch.no_parent": "tidak ada rencana",
  "p.mspdi_batch.preview": "Pratinjau impor",
  "p.mspdi_batch.import": "Impor",
  "p.mspdi_batch.import_ask":
    "Impor berkas sekarang? Rencana dibuat dan tugas ditulis seperti pratinjau. Ini bukan satu transaksi: proyek yang gagal menghentikan sisanya, dan rencana yang sudah dibuat tetap ada.",
  "p.mspdi_batch.start_over": "Pilih berkas lain",
  "p.mspdi_batch.result_preview_h": "Pratinjau",
  "p.mspdi_batch.result_done_h": "Diimpor",
  "p.mspdi_batch.outcome_would_import": "akan diimpor",
  "p.mspdi_batch.outcome_imported": "diimpor",
  "p.mspdi_batch.outcome_failed": "gagal",
  "p.mspdi_batch.outcome_not_attempted": "tidak dicoba",
  "p.mspdi_batch.outcome_unknown": "hasil tidak diketahui",
  "p.mspdi_batch.new_plan_note": "rencana baru akan dibuat",
  "p.mspdi_batch.plan_created": "rencana baru dibuat",
  "p.mspdi_batch.failed_note":
    "Proyek ini gagal, jadi yang sesudahnya tidak dicoba. Perbaiki berkas dan impor lagi: rencana yang ada diperbarui, tidak diduplikasi.",
  "p.mspdi_batch.pool_h": "Orang dan sumber daya dalam berkas",
  "p.mspdi_batch.pool_note":
    "Sumber daya dicantumkan sekali untuk semua berkas, dengan jumlah penugasan masing-masing. Mereka tidak dicocokkan dengan orang: nama bukan identitas.",
  "p.mspdi_batch.pool_row": "{name} — penugasan: {assignments}",
  "p.mspdi_batch.links_h": "Tautan antarberkas",
  "p.mspdi_batch.links_would": "Tautan antarberkas yang akan dibuat: {n}.",
  "p.mspdi_batch.links_made": "Tautan dibuat: {n}; sudah ada: {m}.",
  "p.mspdi_batch.links_unresolved":
    "Tautan yang menunjuk ke luar berkas atau ke tugas yang tidak ada, tidak dibuat: {n}.",
  "p.mspdi_batch.links_after_failure":
    "Tidak ada tautan antarberkas yang dibuat, karena sebuah proyek gagal.",
  "p.mspdi_batch.not_offered":
    "Tidak ditawarkan di sini: memetakan bidang khusus berkas, mencocokkan sumber daya dengan orang, dan mengimpor tarif mereka.",
  "p.mspdi_batch.could_not": "Impor tidak berhasil.",
  "p.mspdi_batch.refused":
    "Layanan menolak berkas: salah satunya bukan XML MS Project, terlalu besar, atau menyebut rencana yang tidak dapat Anda lihat.",
  "p.mspdi_batch.link_from_jobs": "Impor beberapa berkas MS Project",
  "p.mspdi_batch.read_failed": "Tidak dapat membaca “{file}”.",
  "p.components.link": "Komponen",
  "p.components.title": "Komponen",
  "p.components.title_ppm": "Komponen — PPM",
  "p.components.intro":
    "Komponen mengelompokkan tugas rencana menurut bagian pekerjaan, misalnya API atau aplikasi seluler. Satu tugas termasuk paling banyak satu komponen. Setiap tugas dihitung: yang tidak di komponen mana pun dicantumkan sebagai belum ditetapkan.",
  "p.components.loading": "Memuat…",
  "p.components.could_not_load": "Tidak dapat memuat komponen.",
  "p.components.none": "Rencana ini belum memiliki komponen.",
  "p.components.unassigned": "Belum ditetapkan",
  "p.components.counts": "{open} terbuka, {done} selesai, {total} seluruhnya",
  "p.components.lead": "Pemimpin: {lead}",
  "p.components.no_lead": "tanpa pemimpin",
  "p.components.edit": "Ubah",
  "p.components.save": "Simpan",
  "p.components.cancel": "Batal",
  "p.components.remove": "Hapus",
  "p.components.remove_ask":
    "Hapus komponen “{name}”? Tidak ada tugas di dalamnya, jadi tidak ada yang lain berubah.",
  "p.components.in_use":
    "Tugas yang masih termasuk: {n}. Pindahkan ke komponen lain sebelum menghapusnya.",
  "p.components.in_use_refused":
    "Masih ada tugas di komponen ini. Pindahkan ke komponen lain lebih dulu.",
  "p.components.add_h": "Tambah komponen",
  "p.components.name": "Nama (hingga 120 karakter)",
  "p.components.lead_field":
    "Pemimpin (opsional: rujukan person: atau worker:)",
  "p.components.add": "Tambahkan komponen",
  "p.components.name_required": "Nama wajib diisi.",
  "p.components.name_too_long": "Nama lebih dari 120 karakter.",
  "p.components.lead_bad":
    "Pemimpin harus berupa rujukan person: atau worker:.",
  "p.components.taken": "Komponen dengan nama ini sudah ada di rencana.",
  "p.components.too_many": "Satu rencana menampung paling banyak 200 komponen.",
  "p.components.could_not_save": "Tidak dapat menyimpan komponen.",
  "p.components.could_not_remove": "Tidak dapat menghapus komponen.",
  "p.components.removed": "“{name}” dihapus.",
  "p.components.saved": "“{name}” disimpan.",
  "p.components.added": "“{name}” ditambahkan.",
  "p.resource_roles.heading": "Peran utama orang",
  "p.resource_roles.says":
    "Peran utama seseorang menentukan baris mana di atas yang menghitung kapasitas yang dinyatakannya sebagai pasokan. Menetapkannya memerlukan peran manajer sumber daya atau administrator; orang lain hanya melihat milik mereka sendiri.",
  "p.resource_roles.none": "Belum ada yang memiliki peran utama.",
  "p.resource_roles.person": "Orang (rujukan person: atau worker:)",
  "p.resource_roles.role": "Peran (hingga 60 karakter; kosong menghapusnya)",
  "p.resource_roles.set": "Tetapkan peran",
  "p.resource_roles.form": "Tetapkan peran utama seseorang",
  "p.resource_roles.person_bad":
    "Orang harus berupa rujukan person: atau worker:.",
  "p.resource_roles.role_too_long": "Peran lebih dari 60 karakter.",
  "p.resource_roles.forbidden":
    "Menetapkan peran memerlukan peran manajer sumber daya atau administrator.",
  "p.resource_roles.could_not_load": "Tidak dapat memuat peran.",
  "p.resource_roles.could_not_set": "Tidak dapat menetapkan peran.",
  "p.resource_roles.set_ok": "{person} kini memiliki peran utama “{role}”.",
  "p.resource_roles.cleared_ok": "{person} tidak lagi memiliki peran utama.",
  "p.goals.link": "Sasaran",
  "p.goals.title": "Sasaran",
  "p.goals.title_ppm": "Sasaran — PPM",
  "p.goals.intro":
    "Apa yang ingin dicapai rencana ini, secara berurutan. Sasaran dikenali dari tempatnya dalam daftar, jadi perubahan ditolak jika orang lain mengubah sasaran setelah halaman ini membacanya.",
  "p.goals.loading": "Memuat sasaran…",
  "p.goals.could_not_load": "Tidak dapat memuat sasaran.",
  "p.goals.none": "Rencana ini belum memiliki sasaran.",
  "p.goals.target": "Tanggal target: {date}",
  "p.goals.status": "Keadaan: {status}",
  "p.goals.st.NotStarted": "Belum dimulai",
  "p.goals.st.InProgress": "Sedang berjalan",
  "p.goals.st.Achieved": "Tercapai",
  "p.goals.st.Missed": "Tidak tercapai",
  "p.goals.edit": "Ubah",
  "p.goals.save": "Simpan sasaran",
  "p.goals.cancel": "Batal",
  "p.goals.remove": "Hapus",
  "p.goals.remove_ask":
    "Hapus sasaran “{title}”? Sasaran sesudahnya naik satu tempat.",
  "p.goals.add_h": "Tambah sasaran",
  "p.goals.title_field": "Judul (hingga 512 karakter)",
  "p.goals.description_field": "Deskripsi (opsional)",
  "p.goals.target_field": "Tanggal target (opsional)",
  "p.goals.status_field": "Keadaan",
  "p.goals.status_none": "Tanpa keadaan",
  "p.goals.status_custom": "Keadaan lain, dinamai di bawah",
  "p.goals.custom_field": "Nama keadaan",
  "p.goals.add": "Tambahkan sasaran",
  "p.goals.title_required": "Judul wajib diisi.",
  "p.goals.title_too_long": "Judul lebih dari 512 karakter.",
  "p.goals.date_bad": "Tanggal target harus hari yang nyata.",
  "p.goals.custom_required": "Beri nama keadaan, atau pilih yang lain.",
  "p.goals.full": "Satu rencana memuat paling banyak 256 sasaran.",
  "p.goals.changed":
    "Sasaran berubah setelah halaman ini membacanya, jadi tidak ada yang disimpan. Sasaran telah dibaca ulang: periksa, lalu coba lagi.",
  "p.goals.gone": "Sasaran itu sudah tidak ada. Sasaran telah dibaca ulang.",
  "p.goals.could_not_save": "Tidak dapat menyimpan sasaran.",
  "p.goals.could_not_remove": "Tidak dapat menghapus sasaran.",
  "p.goals.added": "“{title}” ditambahkan.",
  "p.goals.saved": "“{title}” disimpan.",
  "p.goals.removed": "“{title}” dihapus.",
  "p.custom_fields.title": "Bidang khusus",
  "p.custom_fields.title_ppm": "Bidang khusus — PPM",
  "p.custom_fields.intro":
    "Bidang yang ditambahkan administrator ke setiap rencana atau tugas, masing-masing dengan jenis yang menjadi acuan pemeriksaan nilainya. Nilai disimpan terpisah dari rencana, sehingga tidak pernah memengaruhi pencocokan.",
  "p.custom_fields.loading": "Memuat bidang khusus…",
  "p.custom_fields.could_not_load": "Tidak dapat memuat bidang khusus.",
  "p.custom_fields.for.plan": "Bidang pada rencana",
  "p.custom_fields.for.task": "Bidang pada tugas",
  "p.custom_fields.none": "Belum ada yang dideklarasikan.",
  "p.custom_fields.type.text": "Teks",
  "p.custom_fields.type.date": "Tanggal",
  "p.custom_fields.type.bool": "Ya atau tidak",
  "p.custom_fields.type.integer": "Bilangan bulat",
  "p.custom_fields.type.list": "Satu pilihan dari daftar",
  "p.custom_fields.type.multi_list": "Beberapa pilihan dari daftar",
  "p.custom_fields.is_required": "wajib",
  "p.custom_fields.kinds_only": "hanya untuk rencana berjenis: {kinds}",
  "p.custom_fields.choices": "pilihan: {options}",
  "p.custom_fields.withdraw": "Tarik",
  "p.custom_fields.withdraw_ask":
    "Tarik bidang “{label}”? Nilai yang sudah tersimpan tetap dapat dibaca, ditandai sebagai ditarik, dan nilai baru tidak dapat diisi.",
  "p.custom_fields.withdrawn": "“{label}” ditarik.",
  "p.custom_fields.could_not_withdraw": "Tidak dapat menarik bidang.",
  "p.custom_fields.declare_h": "Deklarasikan bidang",
  "p.custom_fields.key_field":
    "Kunci (huruf kecil, angka, dan garis bawah, diawali huruf)",
  "p.custom_fields.label_field": "Label (hingga 100 karakter)",
  "p.custom_fields.type_field": "Jenis nilai",
  "p.custom_fields.applies_field": "Berlaku untuk",
  "p.custom_fields.applies.plan": "Rencana",
  "p.custom_fields.applies.task": "Tugas",
  "p.custom_fields.kinds_field":
    "Hanya untuk jenis rencana ini (opsional, dipisahkan koma)",
  "p.custom_fields.options_field":
    "Pilihan, satu per baris (hingga 100, masing-masing hingga 100 karakter)",
  "p.custom_fields.required_field":
    "Setiap rencana atau tugas harus memiliki nilai",
  "p.custom_fields.declare": "Deklarasikan bidang ini",
  "p.custom_fields.key_bad":
    "Kunci harus berupa huruf kecil, angka, dan garis bawah, diawali huruf, dan paling banyak 100 karakter.",
  "p.custom_fields.label_bad": "Label wajib diisi, paling banyak 100 karakter.",
  "p.custom_fields.options_none":
    "Bidang daftar memerlukan paling sedikit satu pilihan.",
  "p.custom_fields.options_long":
    "Daftar memuat paling banyak 100 pilihan, masing-masing paling banyak 100 karakter.",
  "p.custom_fields.options_twice": "Setiap pilihan hanya boleh muncul sekali.",
  "p.custom_fields.key_taken":
    "Bidang dengan kunci ini sudah ada untuk jenis subjek tersebut.",
  "p.custom_fields.could_not_declare": "Tidak dapat mendeklarasikan bidang.",
  "p.custom_fields.declared": "“{label}” dideklarasikan.",
  "p.custom_fields.search_h": "Cari menurut nilai bidang",
  "p.custom_fields.field_field": "Bidang",
  "p.custom_fields.choose": "Pilih…",
  "p.custom_fields.value_field": "Nilai",
  "p.custom_fields.yes": "Ya",
  "p.custom_fields.no": "Tidak",
  "p.custom_fields.search": "Cari",
  "p.custom_fields.found.plan": "Rencana cocok yang dapat Anda lihat: {n}",
  "p.custom_fields.found.task": "Tugas cocok yang dapat Anda lihat: {n}",
  "p.custom_fields.truncated":
    "Ada lebih dari 5.000 nilai tersimpan untuk bidang ini; hanya 5.000 pertama yang diperiksa.",
  "p.custom_fields.field_required": "Pilih bidang.",
  "p.custom_fields.value_required": "Berikan nilai yang dicari.",
  "p.custom_fields.value_integer": "Nilai harus bilangan bulat.",
  "p.custom_fields.value_date": "Nilai harus hari yang nyata.",
  "p.custom_fields.could_not_search": "Tidak dapat mencari pada bidang.",
  "p.custom_fields.link": "Bidang khusus",
  "p.custom_fields.values_title": "Bidang khusus rencana ini",
  "p.custom_fields.values_title_ppm": "Bidang khusus rencana — PPM",
  "p.custom_fields.values_intro":
    "Nilai bidang yang dideklarasikan administrator untuk rencana berjenis ini. Penyimpanan hanya mengirim yang berubah; jika satu nilai salah, tidak ada yang disimpan.",
  "p.custom_fields.values_could_not_load":
    "Tidak dapat memuat bidang khusus rencana ini.",
  "p.custom_fields.values_missing":
    "Bidang wajib yang belum bernilai: {fields}",
  "p.custom_fields.values_none":
    "Belum ada bidang khusus untuk jenis rencana ini. Bidang dideklarasikan di halaman",
  "p.custom_fields.values_h": "Nilai-nilai",
  "p.custom_fields.values_required_label": "{label} (wajib)",
  "p.custom_fields.values_unset": "Tanpa nilai",
  "p.custom_fields.values_save": "Simpan nilai",
  "p.custom_fields.values_unchanged":
    "Tidak ada yang berubah, jadi tidak ada yang dikirim.",
  "p.custom_fields.values_saved": "Nilai disimpan.",
  "p.custom_fields.values_could_not_save": "Tidak dapat menyimpan nilai.",
  "p.custom_fields.values_withdrawn_h":
    "Nilai bidang yang ditarik (hanya baca)",
  "p.custom_fields.v_integer": "{label}: nilai harus bilangan bulat.",
  "p.custom_fields.v_date": "{label}: nilai harus hari yang nyata.",
  "p.custom_fields.v_choice": "{label}: pilih salah satu pilihan bidang.",
  "p.custom_fields.v_too_long": "{label}: teks lebih dari 1.000 karakter.",
  "p.custom_fields.v_required": "{label} wajib diisi dan tidak boleh kosong.",
  "p.onboarding_templates.title": "Templat daftar periksa",
  "p.onboarding_templates.title_ppm": "Templat daftar periksa — PPM",
  "p.onboarding_templates.intro":
    "Daftar periksa awal setiap kasus orientasi atau keluar, dengan kapan tiap butir jatuh tempo dari tanggal berlakunya. Kasus yang sudah dimulai tetap memakai butirnya.",
  "p.onboarding_templates.loading": "Memuat daftar periksa…",
  "p.onboarding_templates.could_not_load": "Tidak dapat memuat daftar periksa.",
  "p.onboarding_templates.is_custom":
    "Jenis ini memakai daftar periksanya sendiri.",
  "p.onboarding_templates.is_builtin":
    "Jenis ini memakai daftar periksa bawaan; butirnya berasal dari layanan, dalam bahasa Inggris.",
  "p.onboarding_templates.before": "hari sebelum tanggal: {n}",
  "p.onboarding_templates.after": "hari sesudah tanggal: {n}",
  "p.onboarding_templates.on_the_day": "pada tanggal",
  "p.onboarding_templates.edit": "Ubah",
  "p.onboarding_templates.reset": "Kembali ke daftar periksa bawaan",
  "p.onboarding_templates.reset_ask":
    "Kembali ke daftar periksa bawaan untuk “{kind}”? Daftar khusus dihapus; kasus yang sudah dimulai tetap memakai butirnya.",
  "p.onboarding_templates.reset_done":
    "“{kind}” kembali memakai daftar periksa bawaan.",
  "p.onboarding_templates.could_not_reset":
    "Tidak dapat kembali ke daftar periksa bawaan.",
  "p.onboarding_templates.item_title":
    "Butir {n}: yang harus dilakukan (hingga 200 karakter)",
  "p.onboarding_templates.item_offset":
    "Butir {n}: hari dari tanggal (−365 hingga 365; negatif berarti sebelum)",
  "p.onboarding_templates.up": "Naikkan butir {n}",
  "p.onboarding_templates.down": "Turunkan butir {n}",
  "p.onboarding_templates.remove": "Hapus butir {n}",
  "p.onboarding_templates.add": "Tambah butir",
  "p.onboarding_templates.save": "Simpan daftar periksa",
  "p.onboarding_templates.cancel": "Batal",
  "p.onboarding_templates.count_bad":
    "Daftar periksa memuat 1 hingga {max} butir.",
  "p.onboarding_templates.title_bad":
    "Butir {n}: judul wajib diisi, hingga 200 karakter.",
  "p.onboarding_templates.offset_bad":
    "Butir {n}: jumlah hari harus bilangan bulat dari −365 hingga 365.",
  "p.onboarding_templates.saved": "Daftar periksa untuk “{kind}” disimpan.",
  "p.onboarding_templates.could_not_save":
    "Tidak dapat menyimpan daftar periksa.",
  "p.onboarding_templates.forbidden":
    "Mengubah daftar periksa memerlukan izin untuk mengelola orang.",
};
