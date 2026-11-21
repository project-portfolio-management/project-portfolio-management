# Untuk Chief Information Officer

*Chief Information Officer: integrasi, identitas, data, hosting, dan informasi yang akan diminta dari Anda.*

## Identitas dan akses

- Login **ditegakkan secara bawaan**; layanan menolak dimulai tanpa kunci verifikasi token, dan di produksi dengan login dimatikan.
- Masuk melalui penyedia identitas (termasuk Microsoft Entra ID); autentikasi multifaktor tetap di tangan penyedia ([panduan masuk](../../../../operations/sign-in-with-entra.md)).
- Otorisasi memakai atribut token (misalnya `access`, `directorpack`, `confidence`) dengan kebijakan atribut. Rencana pribadi disembunyikan dari setiap agregat; tanda penanganan menyembunyikan rencana di atas izin akses pemanggil, termasuk administrator.
- Akses darurat ke rencana pribadi memerlukan kelayakan, alasan yang dinyatakan, paling lama empat jam, dan orang kedua diberi tahu.
- Administrator dapat mendaftar hak akses yang berlaku (alat bantu sertifikasi ulang) dan mengekspor atau menyamarkan data satu orang.

## Data dan integrasi

| Topik | Posisi |
|---|---|
| Data yang disimpan | Catatan rencana, tugas, risiko, anggaran, pencapaian, waktu, dan biaya; orang berupa referensi buram, bukan nama. Tidak ada data pasien, tidak ada data kategori khusus secara desain |
| Inventaris | Setiap kolom data pribadi tercantum dalam [inventaris data pribadi](../../../../governance/personal-data-inventory.csv); sebuah pengujian menjaganya sesuai dengan skema |
| Antarmuka | REST dengan dokumen OpenAPI turunan; impor dan ekspor massal; umpan kalender; peristiwa keluar dan webhook hanya ke tujuan `https` publik |
| Cadangan | Pencadangan berskrip dengan pemulihan yang diuji, enkripsi opsional, target pemulihan yang dinyatakan ([pencadangan dan pemulihan](../../../../operations/backup-and-restore.md)) |
| Hosting | Citra kontainer dan `compose.yaml`; perangkat lunak tidak melakukan transfer sendiri, sehingga lokasi data adalah pilihan hosting Anda |

## Celah yang harus Anda tutup

- Uji penetrasi, tinjauan kode independen, dan sertifikasi: belum ada yang dilakukan.
- Retensi baru sebagian ditegakkan; pembersihan harus dijadwalkan (lihat [jadwal retensi](../../../../governance/retention-schedule.md)).
- Alat subjek data menemukan referensi, bukan nama yang diketik dalam teks bebas atau berkas yang diunggah.
- Jejak audit tidak kebal terhadap pengubahan oleh administrator basis data.

## Baca selanjutnya

[Diagram alur data](../../../../governance/data-flow.md) · [Daftar periksa jaminan](../../../../governance/assurance-checklist.md) · [Penerapan](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
