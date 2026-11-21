# Untuk Chief Risk Officer

*Chief Risk Officer: risiko yang dibantu perangkat lunak untuk Anda lihat, dan risiko menggunakannya.*

## Risiko yang dibantu dikelola

- **Risiko, isu, dan permintaan perubahan** dicatat per rencana; daftar isu lintas rencana yang dapat dibaca; permintaan perubahan dengan pemisahan tugas.
- **Keyakinan pelaksanaan dan pengecualian:** keyakinan yang dinyatakan dipisahkan dari RAG hasil hitung, ditandai bila keduanya berselisih dua tingkat, dengan riwayat yang hanya dapat ditambah, dan jam pengecualian 48 jam yang tidak diulang ketika peringkat memburuk dan hanya ditutup oleh keputusan tercatat.
- **Ketiadaan yang jujur:** data yang hilang ditampilkan sebagai "belum dinilai" atau null beserta alasan, tidak pernah sebagai hijau atau nol.
- **Paket bulanan yang dibekukan dan berversi,** sehingga apa yang disampaikan kepada direktur dapat ditunjukkan kemudian persis seperti saat diterbitkan.

## Risiko menggunakannya (daftar risiko proyek sendiri)

| Risiko | Keadaan |
|---|---|
| Tidak ada uji penetrasi, audit pihak ketiga, atau sertifikasi | Terbuka: pembeli yang memesannya ([daftar periksa jaminan](../../../../governance/assurance-checklist.md)) |
| Retensi hanya ditegakkan untuk sebagian data | Terbuka; pembersihan harus dijadwalkan |
| Alat subjek data tidak mencari teks bebas, lampiran, atau cadangan | Terbuka |
| Jejak audit dapat dikalahkan oleh administrator basis data; tidak ada rantai hash | Dinyatakan dalam [DPIA](../../../../governance/dpia.md) |
| Tanda penanganan menyembunyikan dan memberi label, tidak mengenkripsi | Dinyatakan |
| Pemindaian lampiran memerlukan pemindai yang dijalankan penerap | Terbuka |
| Satu pemelihara, tanpa SLA | Terbuka |
| Terjemahan masih draf | Terbuka |
| Hanya nonklinis; tidak ada kasus keselamatan klinis | Jangan memasukkan isi klinis |

## Kendali yang ditegakkan perangkat lunak

Login secara bawaan, pembatasan laju, header keamanan yang ketat, batas waktu sesi menganggur (30 menit) dan batas sesi keras, tujuan keluar dibatasi ke `https` publik, lampiran yang diperiksa tanda tangannya, baris audit yang tidak dapat diubah, akses darurat dengan audit. Masing-masing dicakup oleh pengujian yang dikutip di [jalan menuju produksi](../../path-to-production/index.md).

## Keputusan yang menjadi milik Anda

Selera risiko untuk uji coba; jendela pengecualian (bawaan 48 jam); tingkat penandaan mana yang dijamin untuk penerapan; siapa yang meninjau daftar akses dan seberapa sering.

## Baca selanjutnya

[Jalan menuju produksi](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Kepatuhan](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
