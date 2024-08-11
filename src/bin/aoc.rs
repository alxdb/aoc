use anyhow::anyhow;
use clap::{Parser, Subcommand};
use regex::Regex;
use std::sync::OnceLock;

/// Run and test AOC solutions
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(env("AOC_TOKEN"), hide_env_values(true))]
    aoc_token: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a solution
    RunSolution { solution: String },
}

#[derive(Debug, Clone, Copy)]
struct AocId {
    year: u32,
    day: u32,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::RunSolution { solution } => {
            let aoc_id = parse_solution_name(&solution)?;
            println!("Running solution: {aoc_id:?}");
        }
    }
    Ok(())
}

pub fn parse_solution_name(solution_name: &str) -> anyhow::Result<AocId> {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"aoc(\d{2})_(\d{2})$").unwrap())
        .captures(solution_name)
        .map(|caps| caps.extract())
        .ok_or(anyhow!("Solution name {} invalid", solution_name))
        .map(|(_, [year, day])| AocId {
            year: year.parse().unwrap(),
            day: day.parse().unwrap(),
        })
}

mod input_cache {
    use crate::AocId;
    use anyhow::anyhow;
    use std::fs::OpenOptions;
    use std::io::{ErrorKind, Write};
    use std::path::{Path, PathBuf};

    fn input_path(aoc_id: &AocId) -> PathBuf {
        dirs::cache_dir()
            .unwrap()
            .join("aoc")
            .join(format!("input_{}_{}", aoc_id.year, aoc_id.day))
    }

    fn fetch_input(aoc_id: &AocId, aoc_token: &str) -> Vec<u8> {
        todo!()
    }

    fn initialize_input(aoc_id: &AocId, aoc_token: &str) -> anyhow::Result<()> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(input_path(aoc_id))
            .and_then(|mut f| f.write_all(&fetch_input(aoc_id, aoc_token)))
            .or_else(|e| match e.kind() {
                ErrorKind::AlreadyExists => Ok(()),
                _ => Err(e),
            })
            .map_err(anyhow::Error::from)
    }

    fn read_input(aoc_id: &AocId) -> anyhow::Result<String> {
        todo!()
    }

    fn get_input(aoc_id: &AocId, aoc_token: &str) -> anyhow::Result<String> {
        initialize_input(aoc_id, aoc_token)?;
        todo!()
    }
}
