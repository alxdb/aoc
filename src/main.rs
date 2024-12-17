use aoc_rust::solutions;
use aoc_rust::{fetch_input, AocId};
use clap::Parser;
use std::error::Error;
use std::fmt::Display;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Year of solution to run
    year: u16,
    /// Day of solution to run
    day: u8,
    /// Part of solution to run
    part: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let aoc_id = AocId {
        year: cli.year,
        day: cli.day,
        part: cli.part,
    };

    let input = fetch_input(aoc_id)?;
    let answer: Box<dyn Display> = match aoc_id {
        AocId {
            year: 2024,
            day: 1,
            part: 1,
        } => Box::new(solutions::year_2024::day_01::part1(&input)?),
        AocId {
            year: 2024,
            day: 1,
            part: 2,
        } => Box::new(solutions::year_2024::day_01::part2(&input)?),
        AocId {
            year: 2024,
            day: 2,
            part: 1,
        } => Box::new(solutions::year_2024::day_02::part1(&input)?),
        AocId {
            year: 2024,
            day: 2,
            part: 2,
        } => Box::new(solutions::year_2024::day_02::part2(&input)?),
        AocId {
            year: 2024,
            day: 3,
            part: 1,
        } => Box::new(solutions::year_2024::day_03::part1(&input)?),
        AocId {
            year: 2024,
            day: 3,
            part: 2,
        } => Box::new(solutions::year_2024::day_03::part2(&input)?),
        AocId {
            year: 2024,
            day: 4,
            part: 1,
        } => Box::new(solutions::year_2024::day_04::part1(&input)?),
        AocId {
            year: 2024,
            day: 4,
            part: 2,
        } => Box::new(solutions::year_2024::day_04::part2(&input)?),
        AocId {
            year: 2024,
            day: 5,
            part: 1,
        } => Box::new(solutions::year_2024::day_05::part1(&input)?),
        AocId {
            year: 2024,
            day: 5,
            part: 2,
        } => Box::new(solutions::year_2024::day_05::part2(&input)?),
        AocId {
            year: 2024,
            day: 6,
            part: 1,
        } => Box::new(solutions::year_2024::day_06::part1(&input)?),
        AocId {
            year: 2024,
            day: 6,
            part: 2,
        } => Box::new(solutions::year_2024::day_06::part2(&input)?),
        AocId {
            year: 2024,
            day: 7,
            part: 1,
        } => Box::new(solutions::year_2024::day_07::part1(&input)?),
        AocId {
            year: 2024,
            day: 7,
            part: 2,
        } => Box::new(solutions::year_2024::day_07::part2(&input)?),
        AocId {
            year: 2024,
            day: 8,
            part: 1,
        } => Box::new(solutions::year_2024::day_08::part1(&input)?),
        AocId {
            year: 2024,
            day: 8,
            part: 2,
        } => Box::new(solutions::year_2024::day_08::part2(&input)?),
        AocId {
            year: 2024,
            day: 9,
            part: 1,
        } => Box::new(solutions::year_2024::day_09::part1(&input)?),
        AocId {
            year: 2024,
            day: 9,
            part: 2,
        } => Box::new(solutions::year_2024::day_09::part2(&input)?),
        _ => todo!(),
    };
    println!("answer={}", answer);
    Ok(())
}
