# Untuk Chief Strategy Officer

*Chief Strategy Officer: bagaimana perangkat lunak menghubungkan strategi dengan pekerjaan, apa yang ditunjukkannya kepada Anda, dan apa yang belum dapat dilakukannya.*

## Yang dilakukannya untuk strategi

- **Satu registri untuk semua yang didanai.** Portofolio, program, proyek, dan produk adalah satu catatan rekursif, sehingga seluruh aset dapat dibaca sebagai pohon ([tujuan dan visi](../../../../spec/01-purpose-and-vision.md)).
- **Deteksi duplikat.** Pencocokan yang dapat dijelaskan menunjukkan kapan dua bagian organisasi menyusun piagam untuk inisiatif yang sama, sebelum keduanya didanai.
- **Tujuan strategis dan peta portofolio.** Anda menandai sasaran sebagai tujuan strategis dan menamai satu tujuan utama untuk setiap program. `GET /api/portfolio-map` dan paket direktur bulanan mengelompokkan program menurut tujuan utama. Sebuah program muncul sekali, dengan tujuan lainnya disebutkan; program tanpa tujuan utama dicantumkan di bawah "tanpa tujuan strategis", dan total tidak pernah dihitung ganda.
- **Keyakinan pelaksanaan di samping status hasil hitung.** Orang menyatakan keyakinan mereka atas pelaksanaan; perangkat lunak menampilkannya di sebelah merah/kuning/hijau hasil hitung dan menandai selisih dua tingkat. Rencana yang belum dinilai siapa pun menampilkan "belum dinilai", tidak pernah hijau.
- **Paket bulanan yang dibekukan.** Apa yang disampaikan kepada direktur disimpan persis seperti saat diterbitkan, dengan tren yang diukur terhadap paket terbit sebelumnya.

## Yang belum dilakukannya

- Paket belum memuat keputusan gerbang, pendanaan menurut tahun anggaran, bagan demam penyangga, nilai yang diperoleh, manfaat tervalidasi, atau peta panas kapasitas (`DP-5` sampai `DP-10` di [`tasks.md`](../../../../tasks.md)). Paket menyebutkan bagian mana yang belum dibangun.
- "Program" berarti rencana yang jenisnya `Program`, karena data tidak memiliki definisi lain; tujuan "tingkat atas" adalah apa pun yang Anda tandai, karena sasaran tidak memiliki hierarki.
- Tidak ada perencanaan skenario, dan belum ada layar untuk peta atau paket: keduanya tersedia sebagai data.
- Tidak tersertifikasi, dan hanya memiliki satu pemelihara ([jalan menuju produksi](../../path-to-production/index.md)).

## Keputusan yang menjadi milik Anda

Sasaran mana yang menjadi tujuan strategis; tujuan utama untuk setiap program; siapa yang menyatakan dan siapa yang menilai secara independen keyakinan pelaksanaan; berapa bulan tren yang sebaiknya dilihat direktur.

## Baca selanjutnya

[Peta jalan](../../../../spec/15-roadmap.md) · [Pertanyaan terbuka](../../../../spec/16-open-questions.md) · [Cakupan](../../../../spec/02-scope.md)
