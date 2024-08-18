use std::{env::var_os, error::Error, ffi::OsString, fs, path::Path};

fn get_env(key: &str) -> Result<OsString, String> {
    var_os(key).ok_or(format!("Could not find environment variable '{}'", key))
}

const AOC_PREFIX: &str = stringify!(
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Read;
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        println!("{}", part1(&input));
        println!("{}", part2(&input));
        Ok(())
    }
);

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = get_env("OUT_DIR")?;

    let bin_prefix_path = Path::new(&out_dir).join("aoc_prefix.rs");
    fs::write(&bin_prefix_path, AOC_PREFIX)?;
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
