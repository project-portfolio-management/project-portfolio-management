# Pengujian keamanan independen

*Untuk pembeli, penilai, atau penguji: apa yang perlu dicakup pengujian keamanan independen atas perangkat lunak ini, apa yang disediakan proyek untuk membantu, apa yang sudah dan belum dilakukan, dan apa yang diserahkan kepada Anda. Ini panduan, bukan laporan uji dan bukan nasihat hukum.*

## Posisi saat ini

**Belum ada uji keamanan independen yang dilakukan.** Tidak ada uji penetrasi, audit pihak ketiga, tinjauan kode independen, atau sertifikasi untuk perangkat lunak ini, dan proyek tidak mengklaim satu pun ([jalan menuju produksi](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). Pemindaian pasif berjalan di integrasi berkelanjutan proyek (aturan di `.zap/rules.tsv`), dan pengujian proyek sendiri memeriksa kendali yang tercantum di bawah; keduanya bukan pengganti penguji yang tidak menulis perangkat lunak ini.

Ini adalah butir 1 dan butir 9 dari [daftar periksa jaminan](../../../../governance/assurance-checklist.md). Ini milik penerap, dengan penguji bersertifikat, terhadap penerapan **Anda**: proyek tidak dapat menguji hosting, penyedia identitas, jaringan, atau kunci Anda.

## Apa yang diuji

Ujilah penerapan sebagaimana akan berjalan, dengan login ditegakkan dan penyedia identitas Anda di depannya. Cakupan yang disarankan, dari kendali yang diklaim perangkat lunak:

| Area | Yang diklaim perangkat lunak | Yang sebaiknya dicoba penguji |
|---|---|---|
| Autentikasi | Login aktif secara bawaan; layanan tidak akan dimulai tanpa kunci verifikasi atau, di produksi, dengan login dimatikan; token diverifikasi secara luring terhadap kunci yang dipublikasikan | Token palsu, kedaluwarsa, diputar ulang, dan salah tanda tangan; penerbit atau audiens yang salah; rotasi kunci; kumpulan kunci yang hilang |
| Otorisasi | Kebijakan berbasis atribut; aturan tingkat catatan | Eskalasi hak istimewa horizontal dan vertikal di setiap rute; mengubah pengenal di jalur dan isi; atribut yang seharusnya tidak dapat ditambahkan pemanggil |
| Rencana pribadi dan tanda penanganan | Rencana pribadi, dan rencana di atas izin akses pemanggil, tidak ada di daftar, hitungan, total, hasil pencarian, umpan, atau ekspor mana pun bagi pemanggil itu, termasuk administrator | Rute apa pun yang membocorkan keberadaan, nama, hitungan, atau total rencana yang tidak dapat dibaca pemanggil; pesan galat dan waktu yang berbeda antara "tersembunyi" dan "tidak ada" |
| Akses darurat | Hanya orang yang layak, alasan yang dinyatakan, paling lama empat jam, orang kedua diberi tahu | Penggunaan tanpa kelayakan; memperpanjang hak; bertindak tanpa orang kedua diberi tahu |
| Catatan beku dan hanya-tambah | Jejak audit, paket direktur yang diterbitkan, riwayat keyakinan pelaksanaan, riwayat status program, dan penyangga proyek menolak perubahan dan penghapusan di basis data | Memperbarui atau menghapus melalui API, melalui impor massal, dan melalui rute apa pun yang menyentuh baris yang sama |
| Pemisahan tugas | Peminta tidak dapat menyetujui permintaan perubahannya sendiri | Menyetujui sebagai peminta, sebagai administrator, dan dengan mengganti penyetuju setelah pengajuan |
| Unggahan dan lampiran | Isi harus sesuai dengan jenis yang dinyatakan; program ditolak; kaitan pemindai virus opsional gagal tertutup | Berkas poliglot, program yang diganti namanya, berkas terlalu besar dan cacat, trik jalur dalam nama, dan apakah pemindai Anda benar-benar dipanggil |
| Permintaan keluar | Tujuan yang diberikan pengguna dibatasi ke alamat `https` publik | Pemalsuan permintaan sisi server ke alamat privat, link-local, dan metadata, pengalihan, DNS rebinding |
| Sesi dan peramban | Batas waktu menganggur, batas keras, kuki `httpOnly`, Content Security Policy yang ketat, daftar izin CORS | Fiksasi dan penggunaan ulang sesi, pemalsuan permintaan lintas situs, injeksi ke antarmuka, clickjacking |
| Pembatasan laju | Batas per pemanggil | Menghindarinya dengan mengganti identitas, header, atau jalur; apakah batas bertahan di proksi Anda |
| Alat subjek data | Ekspor dan penyamaran identitas, hanya administrator, pratinjau terlebih dahulu | Penggunaan oleh non-administrator; kelengkapan terhadap [inventaris data pribadi](../../../../governance/personal-data-inventory.csv) |
| Cadangan | Enkripsi opsional, pemulihan yang diuji | Akses ke berkas cadangan dan kunci; pemulihan ke lingkungan bersih |

## Yang disediakan proyek

- Kode sumber, spesifikasi, dan pengujian, sehingga tinjauan kode dapat dimulai dari klaim.
- Deskripsi API yang dapat dibaca mesin (`/api-docs/openapi.json`), dengan skema permintaan dan respons untuk pekerjaan paket direktur; daftar rute diperiksa terhadap kode oleh pengujian.
- Konfigurasi sekali pakai dengan login dimatikan (`dev-insecure`, lihat [README](../../../../README.md)) **hanya untuk mesin milik penguji sendiri**, dan skrip yang menghasilkan kumpulan kunci uji (`scripts/smoke-keys.sh`) sehingga token dengan atribut pilihan dapat dibuat terhadap penerapan uji.
- [Diagram alur data](../../../../governance/data-flow.md), [buku panduan](../../../../operations/runbook.md), dan [sasaran pemulihan](../../../../operations/recovery-objectives.md).
- Cara pribadi untuk melaporkan temuan: [SECURITY.md](../../../../SECURITY.md).

## Aturan keterlibatan

Sepakati ini secara tertulis sebelum pengujian apa pun; proyek tidak dapat menetapkannya untuk lingkungan Anda.

- **Lingkungan.** Uji salinan dengan data yang realistis tetapi bukan data nyata. Jangan menguji produksi kecuali Anda memutuskannya, secara tertulis.
- **Otorisasi.** Penguji memiliki izin tertulis dari Anda, narahubung bernama, dan jendela waktu yang dinyatakan.
- **Di luar batas.** Penolakan layanan melalui volume, rekayasa sosial terhadap staf, dan layanan pihak ketiga (penyedia identitas Anda, host Anda) di luar cakupan kecuali pemilik masing-masing menyetujui. Cakupan proyek sendiri ada di [SECURITY](../../../../SECURITY.md).
- **Data.** Data pribadi nyata tidak dipakai, tidak disalin keluar, dan tidak dimasukkan ke dalam laporan.
- **Kunci.** Kunci uji terpisah dari kunci produksi dan dimusnahkan setelahnya.

## Pelaporan dan uji ulang

- Temuan tentang perangkat lunak ini disampaikan kepada pemelihara secara pribadi, seperti dinyatakan [SECURITY](../../../../SECURITY.md), dengan cara mereproduksi, versi atau commit, dan dampaknya. Target proyek adalah pengakuan dalam 5 hari kerja dan rencana dalam 15; keduanya target, bukan jaminan. Ada satu pemelihara dan tidak ada perjanjian tingkat layanan.
- Temuan tentang penerapan Anda disampaikan kepada pemilik Anda sendiri.
- Mintalah penguji menilai setiap temuan, menyebutkan apa yang **tidak** diuji, dan menguji ulang perbaikannya. Laporan yang hanya mencantumkan apa yang ditemukan tidak memberi tahu Anda apa yang terlewat.
- Simpan laporan, cakupan, tanggal, dan uji ulang bersama arsip Anda sendiri. Proyek tidak dapat menyimpannya untuk Anda.

## Batas dokumen ini

Dokumen ini mencantumkan area yang perlu dicakup, bukan rencana uji lengkap, dan klaim dalam tabel adalah klaim proyek sendiri. Ia tidak mensertifikasi apa pun, dan uji yang bersih adalah bukti tentang satu penerapan pada satu tanggal.

## Keputusan yang menjadi milik Anda

Siapa yang menguji, kapan, terhadap apa, dengan data apa dan dengan aturan apa; bagaimana temuan dinilai dan sampai kapan harus diperbaiki; dan apakah hasilnya dibagikan kepada penilai Anda.

## Baca selanjutnya

[Daftar periksa jaminan](../../../../governance/assurance-checklist.md) · [Jalan menuju produksi](../../path-to-production/index.md) · [Chief Security Officer](../chief-security-officer/index.md) · [Tata Kelola, Risiko, Kepatuhan](../governance-risk-compliance/index.md)
