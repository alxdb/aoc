use std::fmt::Display;

use clap::{Args, Parser, Subcommand};

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
    RunSolution(AocId),
}

#[derive(Args, Debug, Clone, Copy)]
struct AocId {
    year: u32,
    day: u32,
}

impl Display for AocId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Year 20{}, Day {}", self.year, self.day)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::RunSolution(aoc_id) => {
            println!("Running solution: {aoc_id}");
            let input = input_cache::get_input(&aoc_id, &cli.aoc_token)?;
            println!("Fetched input: len={}", input.len());
        }
    }
    Ok(())
}

mod input_cache {
    use crate::AocId;
    use std::fs::{self, OpenOptions};
    use std::io::{ErrorKind, Write};

    fn fetch_input(aoc_id: &AocId, aoc_token: &str) -> anyhow::Result<Vec<u8>> {
        let response = reqwest::blocking::Client::new()
            .get(format!(
                "https://adventofcode.com/20{}/day/{}/input",
                aoc_id.year, aoc_id.day
            ))
            .header("COOKIE", format!("session={aoc_token}"))
            .send()?
            .error_for_status()?;
        Ok(response.bytes()?.into())
    }

    pub fn get_input(aoc_id: &AocId, aoc_token: &str) -> anyhow::Result<String> {
        let input_path = dirs::cache_dir()
            .unwrap()
            .join("aoc")
            .join(format!("input_{:02}_{:02}", aoc_id.year, aoc_id.day));
        fs::create_dir_all(input_path.parent().unwrap()).or_else(|e| match e.kind() {
            ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e),
        })?;
        if !input_path.exists() {
            let input = fetch_input(aoc_id, aoc_token)?;
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&input_path)
            {
                Ok(mut f) => f.write_all(&input)?,
                Err(e) => match e.kind() {
                    ErrorKind::AlreadyExists => (),
                    _ => return Err(anyhow::Error::from(e)),
                },
            }
        }

        Ok(fs::read_to_string(&input_path)?)
    }
}
