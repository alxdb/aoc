use std::error::Error;
use std::str::FromStr;

struct Input(Vec<Vec<u32>>);

impl FromStr for Input {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.split_whitespace()
                    .map(u32::from_str)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Input)
    }
}

fn report_is_safe(report: &[u32]) -> bool {
    let diffs = report
        .windows(2)
        .map(|x| x[0] as i32 - x[1] as i32)
        .collect::<Vec<_>>();
    let increasing = diffs.iter().all(|&x| x > 0);
    let decreasing = diffs.iter().all(|&x| x < 0);
    let abs_diff = diffs.iter().map(|x| x.abs()).collect::<Vec<_>>();

    !diffs.is_empty()
        && (increasing || decreasing)
        && *abs_diff.iter().max().unwrap() <= 3
        && *abs_diff.iter().min().unwrap() >= 1
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    Input::from_str(input)?
        .0
        .iter()
        .filter(|report| report_is_safe(report))
        .count()
        .try_into()
        .map_err(|e| -> Box<dyn Error> { Box::new(e) })
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    Input::from_str(input)?
        .0
        .into_iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let mut dampened = report.clone();
                dampened.remove(i);
                report_is_safe(&dampened)
            })
        })
        .count()
        .try_into()
        .map_err(|e| -> Box<dyn Error> { Box::new(e) })
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let input = &fetch_input(AocId {
            year: 2024,
            day: 2,
            part: 1,
        })?;
        assert_eq!(part1(input)?, 252);
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 4);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let input = &fetch_input(AocId {
            year: 2024,
            day: 2,
            part: 2,
        })?;
        assert_eq!(part2(input)?, 324);
        Ok(())
    }
}
