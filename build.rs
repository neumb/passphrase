use std::env;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wordlist.rs");

    let mut out = BufWriter::new(fs::File::create(dest_path)?);
    writeln!(out, "static WORDLIST: &[&str] = &[")?;

    let words = BufReader::new(fs::File::open("eff_short_wordlist_1.txt")?);
    for word in words.lines() {
        writeln!(out, "  \"{}\",", word?)?;
    }
    writeln!(out, "];")?;
    out.flush()?;

    println!("cargo::rerun-if-changed=build.rs");
    Ok(())
}
