# Tata Kelola, Risiko, Kepatuhan

*Untuk orang yang menjalankan tata kelola, risiko, dan kepatuhan: apa yang dibantu perangkat lunak untuk Anda buktikan, apa yang ditegakkannya, dan apa yang harus Anda sediakan. Bukan nasihat hukum.*

## Tata kelola

- **Keputusan dicatat.** Permintaan perubahan memerlukan penyetuju bernama yang bukan peminta, alasan yang dinyatakan sedikitnya 10 karakter, dan diaudit menurut pelaku; basis data menolak persetujuan tanpa pemutus dan alasan.
- **Paket direktur bulanan yang dibekukan.** Disusun satu kali, diterbitkan oleh seseorang, tidak dapat diubah setelah terbit; koreksi berupa penerbitan ulang dengan catatan. Akses baca menurut audiens.
- **Keyakinan pelaksanaan dan pengecualian.** Keyakinan yang dinyatakan hanya dapat ditambah dan ditampilkan di samping status hasil hitung, tidak pernah dicampur. Peringkat merah atau kuning-merah membuka pengecualian dengan jam 48 jam (dapat dikonfigurasi) yang hanya ditutup oleh keputusan tercatat.
- **Tata kelola proyek perangkat lunak itu sendiri:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [kendali perubahan](../../../../spec/18-change-control.md), [pernyataan AI](../../../../AI_STATEMENT.md).

## Risiko

- Risiko, isu, dan permintaan perubahan per rencana; daftar isu lintas rencana yang dapat dibaca.
- Data yang hilang ditampilkan sebagai "belum dinilai", tidak pernah sebagai hijau.
- Risiko terbuka milik proyek sendiri (tanpa uji penetrasi, retensi sebagian, satu pemelihara, terjemahan draf) ada di halaman [jalan menuju produksi](../../path-to-production/index.md), butir demi butir dengan perkiraan untuk masing-masing.

## Kepatuhan

| Area | Di mana | Keadaan |
|---|---|---|
| Pelindungan data (hukum yang berlaku; [§12.2a](../../../../spec/12-compliance.md) memetakan GDPR, UK GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA, dan lainnya) | [DPIA](../../../../governance/dpia.md), [catatan pemrosesan](../../../../governance/record-of-processing.md), [inventaris data pribadi](../../../../governance/personal-data-inventory.csv), [jadwal retensi](../../../../governance/retention-schedule.md), [alur data](../../../../governance/data-flow.md) | Terisi awal; penanda `‹buyer›` milik Anda; retensi baru sebagian ditegakkan |
| DTAC NHS Inggris (satu skema nasional, contoh kerja; skema lain mulai dari [daftar periksa jaminan](../../../../governance/assurance-checklist.md)) | [Daftar periksa DTAC](../../../../governance/dtac-checklist.md) | Dijawab bagian demi bagian; baris pembeli dan pemasok terbuka |
| Aksesibilitas | [pernyataan](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA otomatis; tanpa uji manual |
| Kewajiban bahasa | [pernyataan bahasa](../../../../governance/language-statement.md) | Terjemahan masih draf |
| ISO/IEC 27001 | [spesifikasi kepatuhan](../../../../spec/12-compliance.md) | Kendali sisi penerapan; tidak tersertifikasi |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [spesifikasi kepatuhan](../../../../spec/12-compliance.md) §12.4 | Tidak berlaku (tidak ada data pasien); dinyatakan, tidak dibuat-buat |

## Bukti dan jaminan

[Daftar periksa jaminan](../../../../governance/assurance-checklist.md) mencantumkan 14 butir yang akan diminta penilai, dengan pemilik dan keadaan. Sebagian besar terbuka dan milik penerap: uji penetrasi, audit aksesibilitas, latihan pemulihan bencana, jaminan hosting, konfigurasi penyedia identitas, sertifikasi ulang akses, proses insiden, manajemen arsip. Jejak audit menolak perubahan dan penghapusan di basis data, tetapi administrator basis data dapat mengalahkannya; tidak ada rantai hash.

## Keputusan yang menjadi milik Anda

Dasar hukum dan periode retensi; jendela pengecualian; tingkat penandaan untuk penerapan; siapa yang meninjau akses dan seberapa sering; apakah prosedur penahanan hukum diperlukan (belum ada yang dibangun).

## Baca selanjutnya

[Indeks paket tata kelola](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Spesifikasi kepatuhan](../../../../spec/12-compliance.md)
