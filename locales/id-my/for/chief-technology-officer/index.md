# Untuk Chief Technology Officer

*Chief Technology Officer: arsitektur, kualitas, cara memperluasnya, dan di mana rekayasanya masih tipis.*

## Arsitektur

Tiga subproyek, satu entitas ([arsitektur](../../../../spec/08-architecture.md)):

| Bagian | Teknologi |
|---|---|
| Pustaka pencocok | Rust; pencocokan berpasangan deterministik dan probabilistik yang dapat dijelaskan; disematkan oleh layanan |
| Layanan | Rust, axa di atas axum, sqlx, PostgreSQL 18; REST di bawah `/api`, OpenAPI diturunkan dari rute |
| UI | SvelteKit 3, SPA Svelte 5 dengan backend-for-frontend; katalog i18n |

Rantai alat dikunci di [`rust-toolchain.toml`](../../../../rust-toolchain.toml). Persistensi ada di [spesifikasi 10](../../../../spec/10-persistence.md); API di [spesifikasi 09](../../../../spec/09-api-surface.md).

## Kekuatan rekayasa yang perlu diketahui

- **Invarian berada di basis data tempat yang penting:** log audit menolak perubahan dan penghapusan; paket direktur yang diterbitkan dan penilaian keyakinan pelaksanaan tidak dapat diubah atau hanya dapat ditambah melalui pemicu; batasan CHECK menopang aturan layanan.
- **Pengujian penjaga:** pengujian sensus menggagalkan build ketika rute agregat baru melewati penyaring rencana pribadi, rute daftar tanpa paginasi, kolom data pribadi tidak ada di inventaris, atau rute tidak ada di OpenAPI. Semuanya diperbaiki dengan mengklasifikasikan hal baru itu, bukan dengan melemahkan pengujian.
- **Pengujian token asli:** otorisasi diuji dari ujung ke ujung dengan token PASETO bertanda tangan (setiap pengujian dalam prosesnya sendiri, karena pengaturan autentikasi dibaca sekali per proses).
- **Rantai pasok:** `cargo deny`, daftar bahan perangkat lunak, asal-usul build saat rilis, kebijakan keamanan.
- **CI** membangun dan menguji semuanya pada setiap push; lihat `.github/workflows/`.

## Di mana masih tipis

- Satu pemelihara, sehingga faktor bus adalah satu ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Kinerja diketahui secara kasar; cache respons hanya mencakup rute jalur kritis; rencana yang sangat besar lambat.
- Sesi di memori kecuali Redis dikonfigurasi.
- Beberapa bagian paket direktur dan semua layar paket belum dibangun ([`tasks.md`](../../../../tasks.md)).
- Alat AI dipakai untuk membangunnya; produk tidak mengandung model. Lihat [pernyataan AI](../../../../AI_STATEMENT.md).

## Keputusan yang menjadi milik Anda

Topologi hosting dan Redis, apakah akan melakukan fork atau mengikuti hulu, siapa yang meninjau kode secara independen, dan bagaimana layanan cocok dengan penyedia identitas Anda (kumpulan kunci PASETO, atau OIDC melalui Keycloak atau Entra).

## Baca selanjutnya

[Arsitektur](../../../../spec/08-architecture.md) · [Strategi pengujian](../../../../spec/11-testing-strategy.md) · [Status implementasi](../../../../spec/14-implementation-status.md) · [Tolok ukur](../../../../BENCHMARKS.md)
