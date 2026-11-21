# Tutorial

*Langkah-langkah menjalankan demonstrasi di komputer lokal dan meninjau portofolio contoh.*

Demonstrasi ini adalah salinan lengkap perangkat lunak, berisi data untuk organisasi fiktif, Otoritas Transportasi Northgate. Semua orang, rencana, angka, dan tanggal bersifat fiktif, dan data dapat diubah dengan bebas.

**Catatan keamanan.** Demonstrasi mempercayai kunci uji yang dipublikasikan. Jangan dijalankan di jaringan bersama atau dipakai dengan data nyata.

## Persyaratan

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) dengan Compose

## Langkah 1: unduh kodenya

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Langkah 2: mulai demonstrasi

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Perintah ini membangun perangkat lunak, memulai basis data, memuat portofolio fiktif, dan memulai antarmuka. Pembangunan pertama memakan waktu lebih lama daripada berikutnya.

## Langkah 3: masuk

Buka <http://localhost:3000/id-my/signin>, biarkan alamat tetap `example@example.com`, lalu tekan **Sign in to the demonstration**. Tidak ada surel yang dikirim.

Halaman pertama adalah **tur demonstrasi**, yang mencantumkan setiap fitur menurut kelompok, dengan deskripsi dan tautan. Spanduk kuning di setiap halaman menyatakan bahwa datanya fiktif.

## Langkah 4: tinjau portofolio

Dari tur, buka **Plans** untuk melihat portofolio, program, dan proyeknya. Lalu buka **Portfolio dashboard**.

## Langkah 5: tinjau satu proyek

Buka proyek **North Line interlocking** melalui kelompok pelaksanaan pada tur. Tinjau bagan **Gantt**, **Critical path**, dan **Earned value**.

## Langkah 6: tinjau pekerjaan pribadi

Buka **My work** dan **My kanban board**. Sesi masuk sebagai Example Person, yang memiliki tugas terbuka.

## Langkah 7: tinjau kapasitas, kendali mulai, dan pelaporan

Buka **Capacity heatmap** untuk melihat sumber daya yang dipesan berlebih. Buka **Start queue**, tempat batas pekerjaan yang sedang berjalan telah menahan satu program. Buka **Director packs**, ringkasan bulanan yang dibekukan setelah diterbitkan.

## Langkah 8: hentikan demonstrasi

Perintah ini menghentikan semua layanan dan menghapus data demonstrasi:

```sh
podman compose --profile demo down -v
```

## Langkah berikutnya

- [Panduan instalasi](../../../../INSTALL.md), mencakup login dan penyedia identitas
- [Ringkasan peran](../../for/) untuk peran senior
- [Panduan demonstrasi](../../../../operations/demo.md)
- [Daftar fitur](../fitur/index.md)

Masalah dapat dilaporkan melalui [pelacak isu](https://github.com/project-portfolio-management/project-portfolio-management/issues).
