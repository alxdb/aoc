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
            let input_path = input_cache::get_input_path(&aoc_id, &cli.aoc_token)?;
            let [part1, part2] = solution_runner::run_solution(&aoc_id, &input_path)?;
            println!("Solution result is: part1={part1} part2={part2}");
        }
    }
    Ok(())
}

mod input_cache {
    use crate::AocId;
    use std::fs::{self, OpenOptions};
    use std::io::{ErrorKind, Write};
    use std::path::PathBuf;

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

    pub fn get_input_path(aoc_id: &AocId, aoc_token: &str) -> anyhow::Result<PathBuf> {
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
        Ok(input_path)
    }
}

mod solution_runner {
    use crate::AocId;
    use anyhow::anyhow;
    use std::fs::File;
    use std::io::ErrorKind;
    use std::path::PathBuf;
    use std::process::Command;
    use std::str::from_utf8;

    pub fn run_solution(aoc_id: &AocId, input_path: &PathBuf) -> anyhow::Result<[u64; 2]> {
        let solution_exe = format!("aoc{:02}_{:02}", aoc_id.year, aoc_id.day);
        let input_file = File::open(input_path)?;
        let solution_output = Command::new(solution_exe.clone())
            .stdin(input_file)
            .output()
            .map_err(|e| match e.kind() {
                ErrorKind::NotFound => {
                    anyhow!("Could not find solution `{}`", solution_exe)
                }
                _ => e.into(),
            })?;
        if !solution_output.status.success() {
            let solution_error = from_utf8(&solution_output.stderr)?;
            Err(anyhow!(
                "Failed to execute solution: status=`{}` stderr follows:\n{}",
                solution_output.status,
                solution_error
            ))?
        }
        let solutions = solution_output
            .stdout
            .split(|x| *x == b'\n')
            .take(2)
            .map(|x| Ok(from_utf8(x)?.parse()?))
            .collect::<anyhow::Result<Vec<u64>>>()?;
        Ok([solutions[0], solutions[1]])
    }
}
