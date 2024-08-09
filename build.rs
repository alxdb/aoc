use std::{env::var_os, error::Error, ffi::OsString, fs, path::Path};

fn get_env(key: &str) -> Result<OsString, String> {
    var_os(key).ok_or(format!("Could not find environment variable '{}'", key))
}

const AOC_PREFIX: &str = stringify!(
    fn main() {
        println!("answer to part 1: {}", part1());
        println!("answer to part 2: {}", part2());
    }
);

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = get_env("OUT_DIR")?;

    let bin_prefix_path = Path::new(&out_dir).join("aoc_prefix.rs");
    fs::write(&bin_prefix_path, AOC_PREFIX)?;
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
