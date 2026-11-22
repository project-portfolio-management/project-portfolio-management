//! Embeds `migrations/`: writes `$OUT_DIR/migrations.rs`, the version, name and text of every `.sql` file there in name
//! order, for `src/lib.rs` to build sqlx's migrator from. Reruns when a file is added or changed. A file whose name is
//! not `<version>_<description>.sql` with a numeric version fails the build, here, not at run time.

use std::error::Error;
use std::fmt::Write as _;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let dir = Path::new(&std::env::var("CARGO_MANIFEST_DIR")?).join("migrations");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut files = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "sql") {
            files.push(path);
        }
    }
    files.sort();
    let mut out = String::from("&[\n");
    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let name = path
            .file_name()
            .ok_or_else(|| format!("{} has no file name", path.display()))?
            .to_string_lossy();
        let (version, _) = name
            .split_once('_')
            .ok_or_else(|| format!("{name}: a migration file is <version>_<description>.sql"))?;
        let version: i64 = version
            .parse()
            .map_err(|e| format!("{name}: a migration version is a number ({e})"))?;
        writeln!(
            out,
            "    ({version}_i64, {name:?}, include_str!({:?})),",
            path.display().to_string()
        )?;
    }
    out.push_str("]\n");
    let target = Path::new(&std::env::var("OUT_DIR")?).join("migrations.rs");
    std::fs::write(target, out)?;
    Ok(())
}
