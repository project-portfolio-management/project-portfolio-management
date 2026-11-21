# Jalan menuju produksi

*Apa arti "produksi" bagi perangkat lunak ini, dan seberapa jauh jaraknya, butir demi butir.*

"Produksi" di sini berarti sebuah organisasi menjalankan perangkat lunak ini untuk pekerjaan nyata. Rencana nyata, nama orang nyata, dan keputusan nyata bergantung padanya, gangguan atau kebocoran membawa akibat, dan ada pihak yang bertanggung jawab. Di badan kesehatan besar atau kementerian pemerintah, produksi juga berarti jaminan mutu internal organisasi sudah dilalui: uji keamanan, penilaian dampak pelindungan data, audit aksesibilitas, dan, di National Health Service Inggris, Digital Technology Assessment Criteria (DTAC).

## Posisi saat ini

- Tidak ada sertifikasi, persetujuan, atau audit yang diklaim. Halaman ini adalah perkiraan proyek sendiri.
- "Medis" berarti alat portofolio dan proyek **nonklinis**: tanpa rekam medis pasien dan tanpa dukungan keputusan klinis.
- Proyek menyarankan memulai dengan uji coba pada data nonklinis dan tidak rahasia, dijalankan oleh organisasi yang menyelesaikan jaminan mutunya sendiri.

## Kesiapan, butir demi butir

Setiap angka adalah perkiraan proyek pada 13 November 2026, bukan hasil pengukuran. Setiap penggeser dimulai pada perkiraan itu. Menggeser penggeser mencatat pandangan Anda sendiri selama membaca; tidak ada yang disimpan atau dikirim.

- **Jaminan keamanan independen** — 0%. Tidak ada uji penetrasi, audit pihak ketiga, atau sertifikasi. Pembeli atau pemasok harus memesannya (RG-13).
- **Aksesibilitas** — 40%. Audit WCAG 2.2 AA otomatis berjalan di setiap rute pada setiap perubahan. Belum ada uji manual, dan belum ada uji dengan teknologi bantu atau dengan penyandang disabilitas (RG-13).
- **Permintaan subjek data** — 50%. Administrator dapat mengekspor dan menyamarkan identitas seseorang di semua referensi dalam skema, dengan pratinjau terlebih dahulu (RG-6). Nama yang diketik dalam judul, komentar, atau dokumen, berkas yang diunggah, rekaman audit, dan cadangan tidak ditemukan secara otomatis.
- **Retensi** — 35%. Pembersihan, yang secara bawaan berupa uji coba, mencakup notifikasi, log pengiriman, kunci idempotensi, serta rekaman tren dan adopsi, dengan periode yang ditetapkan per penerapan (RG-4). Rencana, tugas, catatan waktu, dokumen, dan jejak audit disimpan sampai operator menghapusnya.
- **Tanda penanganan** — 60%. Klasifikasi yang dapat dikonfigurasi menyembunyikan rencana di atas izin akses pemanggil, termasuk administrator, dan memberi label pada setiap respons (RG-7). Tanda tidak dicetak ke berkas yang dihasilkan, dan izin akses hanya sebaik atribut penyedia identitas.
- **Keamanan lampiran** — 50%. Isi harus sesuai dengan jenis yang dinyatakan, program ditolak, dan pemindaian `clamd` opsional gagal tertutup (RG-8). Tidak ada pemindai yang disertakan, dan kaitan itu baru diuji terhadap pengganti.
- **Tinjauan akses dan akses darurat** — 45%. Laporan hak akses yang berlaku mendukung sertifikasi ulang (RG-9), dan akses darurat dibatasi waktu serta memerlukan alasan yang dinyatakan (RG-10). Laporan menunjukkan apa yang tercatat, bukan apa yang ditegakkan penyedia identitas; tidak ada rute yang mencatat keputusan peninjau; dan pemberitahuan akses darurat dapat menyebut seseorang yang tidak menerima apa pun sementara hak aksesnya masih berlaku.
- **Kinerja dan kapasitas** — 40%. Uji beban dan ketahanan di satu mesin ada di [`BENCHMARKS.md`](../../../BENCHMARKS.md): permintaan biasa dijawab dalam puluhan milidetik, rencana dengan sekitar 12.700 tugas lambat dijadwalkan, dan proses tumbuh hingga sekitar 1 GB. Tidak ada rencana kapasitas, uji dengan banyak pengguna, atau uji di beberapa mesin.
- **Sesi di beberapa server front-end** — 70%. Sesi disimpan di memori kecuali Redis dikonfigurasi (RG-12); dengan Redis, proses masuk bergantung padanya.
- **Integritas jejak audit** — 50%. Penulisan diaudit, dan pemicu basis data menolak perubahan dan penghapusan, dengan pengecualian yang dinyatakan (RG-3). Administrator basis data dapat menghapus pemicu itu, dan tidak ada rantai hash.
- **Pencadangan dan pemulihan** — 80%. Pemulihan diuji dalam integrasi berkelanjutan, polos dan terenkripsi, terhadap target pemulihan yang dinyatakan (RG-5). Enkripsi bersifat opsional, dan latihan berjalan di CI milik proyek sendiri, bukan di hosting penerap.
- **Dukungan dan kesinambungan** — 10%. Satu pemelihara dan tanpa perjanjian tingkat layanan ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Terjemahan** — 20%. Teks dalam bahasa selain Inggris ditulis mesin sampai penutur meninjaunya, dan kewajiban bahasa menurut undang-undang memerlukan teks yang sudah ditinjau ([pernyataan bahasa](../../../governance/language-statement.md)).
- **Penggunaan klinis** — 0%. Belum ada kasus keselamatan klinis yang ditulis, dan penggunaan klinis di luar cakupan.

## Yang harus dilakukan penerapan pertama

1. Selesaikan [daftar periksa jaminan](../../../governance/assurance-checklist.md), dimulai dengan uji penetrasi dan audit aksesibilitas.
2. Isi setiap baris "Buyer to complete" dalam [daftar periksa DTAC](../../../governance/dtac-checklist.md) dan penanda dalam [DPIA](../../../governance/dpia.md).
3. Sepakati periode retensi dengan manajer arsip dan jadwalkan pembersihan.
4. Aktifkan pencadangan terenkripsi dan jalankan latihan pemulihan di hosting milik penerapan itu sendiri.
5. Jaga agar data tetap nonklinis dan di bawah tingkat penandaan yang dijamin untuk penerapan itu.
6. Rencanakan celah di atas, atau danai penghapusannya.

Bukti untuk setiap butir ada di [penilaian](../../../governance/production-readiness.md).
