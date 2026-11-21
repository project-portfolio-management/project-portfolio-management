# Untuk Chief Operating Officer

*Chief Operating Officer: apa yang Anda jalankan setiap hari, apa yang dilakukan perangkat lunak untuk pelaksanaan, dan apa yang diperlukan untuk mengoperasikannya.*

## Yang dilakukannya untuk operasi

- **Perencanaan dan kendali pelaksanaan:** rencana, tugas, ketergantungan, perhitungan jalur kritis, pencapaian lintas rencana, sprint dan fase, burndown, dan metrik alur.
- **Kapasitas:** alokasi, ketersediaan, cuti dan periode tidak bekerja, peran sumber daya, beban kerja dan utilisasi turunan. (Peta panas kapasitas menurut kolam keterampilan dan bulan untuk paket direktur belum dibangun: `DP-10`.)
- **Waktu dan biaya:** lembar waktu, waktu tercatat, tarif biaya (dibatasi untuk keuangan), anggaran, dan biaya turunan.
- **Alur tata kelola:** daftar isu, permintaan perubahan dengan pemisahan tugas (peminta tidak dapat menyetujui), persetujuan, tinjauan, kasus anggota baru/pindah/keluar.
- **Pengecualian:** keyakinan pelaksanaan merah atau kuning-merah membuka pengecualian yang memerlukan penyebab, keputusan yang dibutuhkan, pemilik bernama, dan tanggal batas keputusan dalam 48 jam (jendela dapat dikonfigurasi, 1 sampai 720 jam). Pelanggaran muncul di paket direktur dan di `GET /api/exceptions`.

## Yang diperlukan untuk menjalankannya

| Kebutuhan | Rincian |
|---|---|
| Layanan | Satu layanan Rust, PostgreSQL 18, Redis opsional (cache produksi, sesi bersama opsional), UI SvelteKit |
| Buku panduan | [paket operasi](../../../../operations/README.md): penerapan, buku panduan, pencadangan dan pemulihan, sasaran pemulihan, peningkatan dan migrasi |
| Pekerjaan terjadwal | Tidak ada yang menjadwalkan dirinya sendiri. Pembersihan retensi, pemberitahuan pelanggaran pengecualian, dan tugas serupa harus dipasang pada penjadwal oleh operator Anda |
| Dukungan | Satu pemelihara, tanpa SLA; anggarkan dukungan lini pertama Anda sendiri |

## Ketahui batasnya

- Rencana dengan beberapa ribu tugas lambat dijadwalkan; jawaban penuh ditolak di atas 5.000 tugas kecuali diminta (`?full=true` atau `?only=critical`). Angka dan catatan ada di [`BENCHMARKS.md`](../../../../BENCHMARKS.md); itu orde besaran dari satu mesin, bukan kapasitas.
- Layar tersedia untuk sebagian besar ruang kerja tetapi **belum** untuk permintaan perubahan, isu, paket, atau keyakinan pelaksanaan; saat ini semuanya hanya melalui API.

## Keputusan yang menjadi milik Anda

Siapa pemilik setiap pengecualian, siapa yang menerbitkan paket bulanan (irama dan pengingat adalah `DP-12`, belum dibangun), dan irama tinjauan apa yang diberi masukan oleh aturan pengecualian.

## Baca selanjutnya

[Paket operasi](../../../../operations/README.md) · [Buku panduan](../../../../operations/runbook.md) · [Sasaran pemulihan](../../../../operations/recovery-objectives.md)
