# Untuk Chief Legal Officer

*Chief Legal Officer: lisensi, pelindungan data, aksesibilitas, kewajiban bahasa, dan hal-hal yang tidak dijamin proyek. Ini bukan nasihat hukum.*

## Lisensi

Ditawarkan di bawah salah satu dari MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, atau GPL-3.0-only, atau ketentuan khusus berdasarkan kesepakatan. Pilihan dan teksnya ada di [`LICENSE.md`](../../../../LICENSE.md) dan `LICENSE/`; sebuah pengujian menjaga setiap manifes konsisten dengannya.

## Pelindungan data (hukum yang berlaku bagi Anda)

Ditulis untuk hukum pelindungan data apa pun: [spesifikasi kepatuhan §12.2a](../../../../spec/12-compliance.md) memetakan hukum yang umum menurut wilayah (GDPR dan UK GDPR, LGPD, PIPL, DPDP Act, PIPEDA, CCPA, dan lainnya) pada apa yang didorong masing-masing di sini, berdasarkan pengetahuan umum dan tidak diperiksa terhadap teks setiap undang-undang. Dokumen terisi awal yang Anda sesuaikan; setiap penanda `‹buyer›` adalah keputusan yang menjadi milik Anda (pengendali, dasar hukum, periode retensi, pemroses, pengesahan):

- [Templat DPIA](../../../../governance/dpia.md) dan [catatan pemrosesan](../../../../governance/record-of-processing.md)
- [Inventaris data pribadi](../../../../governance/personal-data-inventory.csv): setiap kolom yang dapat memuat data pribadi
- [Jadwal retensi](../../../../governance/retention-schedule.md): usulan, dengan apa yang ditegakkan kode saat ini
- [Diagram alur data](../../../../governance/data-flow.md)

Perangkat lunak ini menyimpan orang sebagai referensi buram ditambah waktu tercatat, ketersediaan, dan tarif biaya, yang berdekatan dengan pemantauan sehingga memerlukan DPIA. Ia menawarkan ekspor dan penghapusan subjek data melalui penyamaran identitas atau, jika penerapan memilihnya, penghapusan (hanya administrator, pratinjau terlebih dahulu; WW-1.8). Paket direktur yang dibekukan dan riwayat keyakinan yang hanya dapat ditambah dikosongkan dari orang yang menyusun atau menilai, tidak pernah dihapus, sehingga permintaan penghapusan dapat dipenuhi tanpa merusak catatan. Ia **tidak** menemukan nama yang diketik dalam teks bebas, dan cadangan menyimpan catatan yang sudah dihapus aplikasi.

## Kerangka kerja

HIPAA dan rezim sertifikasi data kesehatan AS **tidak berlaku** (tidak ada data pasien); rezim perangkat klinis tidak berlaku. [Daftar periksa DTAC](../../../../governance/dtac-checklist.md) menjawab Digital Technology Assessment Criteria NHS Inggris, satu skema nasional yang disimpan sebagai contoh kerja; untuk skema negara dan sektor lain (ISO/IEC 27001, SOC 2, skema cloud dan sektor publik nasional) mulailah dari [daftar periksa jaminan](../../../../governance/assurance-checklist.md). Tidak ada yang tersertifikasi.

## Aksesibilitas dan bahasa

WCAG 2.2 AA diuji otomatis di setiap rute; belum ada uji manual atau uji teknologi bantu ([pernyataan](../../../../governance/accessibility-statement.md)). Terjemahan bahasa Wales dan lainnya masih draf yang menunggu penerjemah profesional ([pernyataan bahasa](../../../../governance/language-statement.md)); relevan jika Anda memiliki kewajiban bahasa menurut undang-undang.

## Uji tuntas pemasok

Satu pemelihara, tanpa dukungan kontraktual, tanpa klaim jaminan, alat AI dipakai dalam pengembangan dan diungkapkan dalam [pernyataan AI](../../../../AI_STATEMENT.md). Produk tidak mengandung model AI. Pelaporan kerentanan: [SECURITY.md](../../../../SECURITY.md).

## Keputusan yang menjadi milik Anda

Dasar hukum dan periode retensi; apakah Anda memerlukan prosedur penahanan hukum (belum ada yang dibangun); ketentuan kontrak dan dukungan dengan pemelihara.

## Baca selanjutnya

[Daftar periksa jaminan](../../../../governance/assurance-checklist.md) · [Indeks paket tata kelola](../../../../governance/README.md)
