use std::{fmt::Display, time::Instant};

use clap::{Args, Parser, Subcommand};
use regex::Regex;

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
    /// Test all solutions with previously determined answers
    TestSolutions,
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            let input_path = cache::get_input_path(&aoc_id, &cli.aoc_token)?;
            let solution_start = Instant::now();
            let solution_answers = solution_runner::run_solution(&aoc_id, &input_path)?;
            let solution_elapsed = solution_start.elapsed();
            println!("Solution executed in {:.2?}", solution_elapsed);
            println!(
                "Solution result is: part1={} part2={}",
                solution_answers[0], solution_answers[1]
            );

            let cached_answers = cache::get_answers(&aoc_id)?;
            let yes_re = Regex::new(r"^(y|Y)$")?;
            for (i, (cached_answer, solution_answer)) in cached_answers
                .into_iter()
                .zip(solution_answers.into_iter())
                .enumerate()
            {
                let part_num = i + 1;
                if let Some(cached_answer) = cached_answer {
                    if cached_answer == solution_answer {
                        println!("part{} is correct!", part_num);
                    } else {
                        println!(
                            "part{} is incorrect: {}!={}",
                            part_num, solution_answer, cached_answer
                        );
                    }
                } else {
                    let response =
                        rprompt::prompt_reply(format!("Is part{} correct? (y/N) ", part_num))?;
                    if yes_re.is_match(&response) {
                        cache::store_answer(&aoc_id, part_num, &solution_answer)?;
                        println!("Stored answer");
                    } else {
                        println!("Not storing answer");
                    }
                }
            }
        }
        Commands::TestSolutions => {
            let answers = cache::get_all_answers()?;
            for (aoc_id, answers) in answers.iter() {
                println!("Running solution: {aoc_id}");
                let input_path = cache::get_input_path(aoc_id, &cli.aoc_token)?;
                let solution_start = Instant::now();
                let solution_answers = solution_runner::run_solution(aoc_id, &input_path)?;
                let solution_elapsed = solution_start.elapsed();
                println!("Solution executed in {:.2?}", solution_elapsed);
                println!(
                    "Solution result is: part1={} part2={}",
                    solution_answers[0], solution_answers[1]
                );
                for (i, answer) in answers.iter().enumerate() {
                    if let Some(answer) = answer {
                        if *answer != solution_answers[i] {
                            println!("Solution for part{} is incorrect!", i + 1);
                            println!("Correct solution is: {}", answer);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

mod cache {
    use crate::AocId;
    use anyhow::anyhow;
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::{self, OpenOptions};
    use std::io::{ErrorKind, Write};
    use std::path::PathBuf;

    fn cache_path() -> anyhow::Result<PathBuf> {
        dirs::cache_dir()
            .ok_or(anyhow!("Cannot determine cache dir"))
            .map(|p| p.join("aoc"))
    }

    fn ignore_already_exists(e: std::io::Error) -> anyhow::Result<()> {
        match e.kind() {
            ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e.into()),
        }
    }

    pub fn get_input_path(aoc_id: &AocId, aoc_token: &str) -> anyhow::Result<PathBuf> {
        let input_path = cache_path()?.join(format!("input_{:02}_{:02}", aoc_id.year, aoc_id.day));

        if !input_path.try_exists()? {
            let input = reqwest::blocking::Client::new()
                .get(format!(
                    "https://adventofcode.com/20{}/day/{}/input",
                    aoc_id.year, aoc_id.day
                ))
                .header("COOKIE", format!("session={aoc_token}"))
                .send()?
                .error_for_status()?
                .bytes()?;
            fs::create_dir_all(input_path.parent().unwrap()).or_else(ignore_already_exists)?;
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&input_path)
            {
                Ok(mut f) => f.write_all(&input)?,
                Err(e) => ignore_already_exists(e)?,
            }
        }
        Ok(input_path)
    }

    fn get_answer_path(aoc_id: &AocId) -> anyhow::Result<PathBuf> {
        let answer_path =
            cache_path()?.join(format!("answers_{:02}_{:02}", aoc_id.year, aoc_id.day));
        if !answer_path.try_exists()? {
            fs::create_dir_all(&answer_path).or_else(ignore_already_exists)?;
        }
        Ok(answer_path)
    }

    pub fn store_answer(aoc_id: &AocId, part_num: usize, answer: &str) -> anyhow::Result<()> {
        let answer_path = get_answer_path(aoc_id)?;
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(answer_path.join(format!("part_{}", part_num)))
        {
            Ok(mut f) => Ok(write!(f, "{}", answer)?),
            Err(e) => ignore_already_exists(e),
        }
    }

    fn read_answers(answer_path: &PathBuf) -> anyhow::Result<[Option<String>; 2]> {
        let mut result = [None, None];
        for entry in fs::read_dir(answer_path)? {
            let entry = entry?;
            if let Some(idx) = match entry.file_name() {
                f if f == "part_1" => Some(0),
                f if f == "part_2" => Some(1),
                _ => None,
            } {
                result[idx] = Some(fs::read_to_string(entry.path())?);
            }
        }
        Ok(result)
    }

    pub fn get_answers(aoc_id: &AocId) -> anyhow::Result<[Option<String>; 2]> {
        read_answers(&get_answer_path(aoc_id)?)
    }

    pub fn get_all_answers() -> anyhow::Result<HashMap<AocId, [Option<String>; 2]>> {
        thread_local! {
            static RE: Regex = Regex::new(r"^answers_(?<year>[0-9]{2})_(?<day>[0-9]{2})$").unwrap();
        }

        let mut answers = HashMap::new();
        for entry in fs::read_dir(cache_path()?)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(caps) = entry
                    .file_name()
                    .to_str()
                    .and_then(|s| RE.with(|re| re.captures(s)))
                {
                    let aoc_id = AocId {
                        year: caps["year"].parse()?,
                        day: caps["day"].parse()?,
                    };
                    answers.insert(aoc_id, read_answers(&entry.path())?);
                }
            }
        }
        Ok(answers)
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

    pub fn run_solution(aoc_id: &AocId, input_path: &PathBuf) -> anyhow::Result<[String; 2]> {
        let solution_exe = format!("aoc{:02}_{:02}", aoc_id.year, aoc_id.day);
        let input_file = File::open(input_path)?;
        let solution_output = Command::new(&solution_exe)
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
        solution_output
            .stdout
            .split(|x| *x == b'\n')
            .take(2)
            .map(|x| Ok(from_utf8(x)?.to_owned()))
            .collect::<anyhow::Result<Vec<String>>>()?
            .try_into()
            .map_err(|_| anyhow!("Solution returned less than 2 results"))
    }
}
