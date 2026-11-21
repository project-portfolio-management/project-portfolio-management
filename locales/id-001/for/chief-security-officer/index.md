# Untuk Chief Security Officer

*Chief Security Officer: apa yang ditegakkan perangkat lunak, apa yang sudah diverifikasi proyek, dan apa yang harus Anda uji dan operasikan sendiri.*

## Yang ditegakkan perangkat lunak

| Area | Perilaku |
|---|---|
| Autentikasi | Login aktif secara bawaan. Layanan tidak akan dimulai tanpa kunci verifikasi token, dan tidak akan dimulai di produksi dengan login dimatikan. Masuk melalui penyedia identitas seperti Microsoft Entra ID; autentikasi multifaktor tetap di tangan penyedia ([panduan masuk](../../../../operations/sign-in-with-entra.md)) |
| Otorisasi | Kebijakan berbasis atribut pada atribut token. Rencana pribadi dikeluarkan dari setiap agregat bagi semua orang; tanda penanganan menyembunyikan rencana di atas izin akses pemanggil, termasuk administrator |
| Akses darurat | Akses ke rencana pribadi memerlukan kelayakan, alasan yang dinyatakan, paling lama empat jam, dan orang kedua diberi tahu; setiap permintaan diaudit |
| Sesi | Batas waktu menganggur (bawaan 30 menit) dan batas sesi keras |
| Transpor dan peramban | Header keamanan, Content Security Policy yang ketat, daftar izin CORS, pembatasan laju per pemanggil |
| Keluar | Tujuan yang diberikan pengguna dibatasi ke alamat `https` publik |
| Unggahan | Isi harus sesuai dengan jenis yang dinyatakan; program ditolak; pemindaian `clamd` opsional gagal tertutup |
| Audit | Jejak audit menolak perubahan dan penghapusan di basis data |
| Catatan beku | Paket direktur yang diterbitkan dan penilaian keyakinan pelaksanaan tidak dapat diubah atau hanya dapat ditambah melalui pemicu basis data |
| Rantai pasok | `cargo deny`, daftar bahan perangkat lunak, asal-usul build saat rilis, [kebijakan keamanan](../../../../SECURITY.md) |

## Yang belum dilakukan

- Tidak ada uji penetrasi, audit pihak ketiga, tinjauan kode independen, atau sertifikasi.
- Tidak ada pemindai malware yang disertakan; kaitan `clamd` baru diuji terhadap pengganti.
- Jejak audit dapat dikalahkan oleh administrator basis data; tidak ada rantai hash.
- Tanda penanganan menyembunyikan dan memberi label; tidak mengenkripsi, dan tanda tidak dicetak ke berkas yang dihasilkan.
- Sesi disimpan di memori kecuali Redis dikonfigurasi.
- Alat subjek data menemukan referensi, bukan nama yang diketik dalam teks bebas, berkas yang diunggah, atau cadangan.

## Mengoperasikannya

Cadangan memiliki pemulihan yang diuji, enkripsi opsional, dan target pemulihan yang dinyatakan ([paket operasi](../../../../operations/README.md)). [Buku panduan](../../../../operations/runbook.md) memberi langkah teknis insiden; proses insiden dan pelanggaran adalah milik Anda. Laporkan kerentanan secara pribadi seperti dijelaskan [SECURITY.md](../../../../SECURITY.md); targetnya adalah pengakuan dalam 5 hari kerja, bukan jaminan.

## Keputusan yang menjadi milik Anda

Siapa yang melakukan uji penetrasi dan kapan; akses bersyarat dan pemetaan peran penyedia identitas; tingkat penandaan yang dijamin untuk penerapan ini; pemindai di balik kaitan lampiran; seberapa sering akses disertifikasi ulang.

## Baca selanjutnya

[Daftar periksa jaminan](../../../../governance/assurance-checklist.md) · [Jalan menuju produksi](../../path-to-production/index.md) · [Alur data](../../../../governance/data-flow.md)
