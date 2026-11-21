# Perbandingan

*Ringkasan hubungan perangkat lunak ini dengan produk lain.*

Produk di bawah ini melayani tujuan yang berbeda dan tidak diberi skor. Catatan ini didasarkan pada materi publik setiap proyek. **Tidak ada produk lain yang dipasang atau dijalankan**, dan masing-masing mungkin sudah berubah. Dokumentasi produk terkini perlu diperiksa sebelum keputusan diambil.

| Proyek | Apa itu | Perbedaan proyek ini |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Manajemen proyek bagi orang yang bukan manajer proyek | Proyek ini juga mencakup fungsi penjadwalan, biaya, dan tata kelola yang dipakai kantor portofolio |
| [Redmine](https://www.redmine.org/) | Pelacak serbaguna yang dapat dikonfigurasi, dengan jenis isu, status, dan peran yang ditentukan organisasi | Jenis isu, status, dan alur kerja ditentukan organisasi. Peran membawa izin isu, dipegang rencana demi rencana, dalam batas yang diizinkan login seseorang. Lihat [perbandingan Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (melalui forknya, Platform Collective) | Kerangka aplikasi bisnis di atas banyak layanan | Huly adalah platform multilayanan. Proyek ini satu layanan di atas PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Alternatif desktop untuk Microsoft Project, bagi satu manajer dan satu proyek | Proyek ini adalah layanan terhosting multipengguna dengan kontrol akses |

## Perbandingan produk

Halaman perbandingan satuan: [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperabilitas

- **Jira dan Asana:** impor CSV, dengan pratinjau dan alasan yang dinyatakan untuk semua yang dikecualikan.
- **Microsoft Project:** impor dan ekspor XML MSPDI, mencakup tugas, kerangka, tanggal, durasi, batasan, ketergantungan, dan kalender. Penerima tugas, sumber daya, dan garis dasar dikecualikan, dan berkas mencatat hal ini.

## Perbedaan desain

- **Alasan yang dinyatakan.** Angka yang hilang ditampilkan bersama alasan, misalnya "tidak ada perkiraan, karena…".
- **Deteksi duplikat yang dapat dijelaskan.** Pencocokan bekerja di semua jenis rencana dan menunjukkan buktinya.
- **Bawaan keamanan.** Login aktif secara bawaan, dan materi keamanan disertakan dalam repositori.
- **Durasi eksplisit.** Setiap durasi dinyatakan dalam hari kalender atau hari kerja.
- **Batasan tercatat.** Backlog mencatat batasan setiap butir.

## Keterbatasan yang diketahui

- Tidak ada aplikasi seluler atau aplikasi desktop.
- Tidak ada penyuntingan bersama waktu nyata.
- Satu pemelihara, riwayat produksi terbatas, dan tidak ada audit pihak ketiga.
- Uji beban yang dipublikasikan berjalan di satu mesin. Ia menunjukkan orde besaran dan bukan rencana kapasitas.
- Terjemahannya masih draf yang menunggu tinjauan.
- Tidak ada layar untuk menyusun atau memutuskan permintaan perubahan.
- Tidak ada ekosistem plugin atau pasar.

Perincian lebih lanjut ada di [catatan perbandingan](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Koreksi dapat diajukan sebagai isu beserta sumbernya.
