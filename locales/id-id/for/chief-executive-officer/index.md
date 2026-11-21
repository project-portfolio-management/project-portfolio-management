# Untuk Chief Executive Officer

*Chief Executive Officer: apa perangkat lunak ini, apa yang diberikannya kepada Anda, apa yang tidak, dan keputusan yang menjadi milik Anda.*

## Apa itu, dalam satu paragraf

Registri setiap **rencana** yang didanai organisasi (portofolio, program, proyek, produk, dan seterusnya, satu catatan rekursif), dengan ruang kerja manajemen proyek di bawahnya (tugas, jadwal, biaya, sumber daya, risiko, isu, permintaan perubahan, tata kelola). Ia menemukan inisiatif duplikat lewat pencocokan yang dapat dijelaskan, sehingga dua departemen yang menyusun piagam untuk pekerjaan yang sama terlihat sebelum keduanya didanai. Lihat [tujuan dan visi](../../../../spec/01-purpose-and-vision.md).

## Yang dapat Anda andalkan

- **Satu tempat untuk bertanya "apa yang sedang kita kerjakan?"** Rencana bersarang pada kedalaman berapa pun; rencana pribadi tidak masuk ke total dan hitungan mana pun, bagi siapa pun, berdasarkan aturan yang diperiksa pengujian.
- **Paket direktur bulanan** yang disusun satu kali, diterbitkan oleh orang bernama, lalu **dibekukan oleh basis data**: paket yang sudah terbit tidak dapat disunting; koreksi adalah versi baru yang diterbitkan ulang dengan catatan. Saat ini paket memuat peta portofolio menurut tujuan strategis, keyakinan pelaksanaan, pengecualian, isu, dan permintaan perubahan.
- **Keyakinan pelaksanaan yang dinyatakan manusia, ditampilkan di samping status hasil hitung, tidak pernah dicampur.** Rencana yang belum dinilai siapa pun menampilkan "belum dinilai", tidak pernah hijau. Peringkat kuning-merah atau merah membuka pengecualian dengan jam 48 jam (dapat dikonfigurasi); jam yang terlewat ditampilkan dalam paket.
- **Keputusan meninggalkan jejak.** Setiap perubahan diaudit, jejak audit tidak dapat diubah melalui layanan, dan permintaan perubahan tidak dapat disetujui oleh orang yang mengajukannya.

## Yang belum diberikannya

- Paket **belum** memuat keputusan gerbang, pendanaan dan uang tahun anggaran, bagan demam penyangga, nilai yang diperoleh, manfaat tervalidasi, atau peta panas kapasitas. Semuanya tercantum sebagai `DP-5` sampai `DP-10` di [`tasks.md`](../../../../tasks.md), dan paket menyebutkan bagian mana yang belum dibangun, bukan membiarkannya kosong.
- Tidak ada PDF atau layar untuk paket: paket disampaikan sebagai data (`DP-11`).
- Perangkat lunak ini **tidak tersertifikasi atau diaudit** oleh siapa pun. Apa saja yang diperlukan untuk penggunaan produksi, butir demi butir dengan perkiraan untuk masing-masing, ada di halaman [jalan menuju produksi](../../path-to-production/index.md).
- Satu pemelihara, tanpa perjanjian tingkat layanan ([tata kelola](../../../../GOVERNANCE.md)).

## Keputusan yang menjadi milik Anda

1. Apakah akan melakukan uji coba, dan pada bagian portofolio yang mana.
2. Siapa **sponsor** untuk aturan pengecualian 48 jam dan siapa yang boleh menerbitkan paket bulanan.
3. Apakah akan mendanai celah di atas (penilaian, uji penetrasi, pengaturan dukungan) atau menerimanya.

## Baca selanjutnya

[Jalan menuju produksi](../../path-to-production/index.md) · [Peta jalan](../../../../spec/15-roadmap.md) · [Pertanyaan terbuka](../../../../spec/16-open-questions.md)
