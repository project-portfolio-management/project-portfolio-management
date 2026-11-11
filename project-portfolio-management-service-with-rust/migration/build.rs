//! Embeds `migrations/`: writes `$OUT_DIR/migrations.rs`, the name and text of every `.sql` file there in name order,
//! for `src/lib.rs` to build sqlx's migrator from. Reruns when a file is added or changed.

use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let dir = Path::new(&std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
        .join("migrations");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("read migrations/")
        .map(|entry| entry.expect("a migrations/ entry").path())
        .filter(|path| path.extension().is_some_and(|e| e == "sql"))
        .collect();
    files.sort();
    let mut out = String::from("&[\n");
    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let name = path.file_name().expect("a file name").to_string_lossy();
        writeln!(
            out,
            "    ({name:?}, include_str!({:?})),",
            path.display().to_string()
        )
        .expect("write");
    }
    out.push_str("]\n");
    let target = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("migrations.rs");
    std::fs::write(target, out).expect("write migrations.rs");
}
