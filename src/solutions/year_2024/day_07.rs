use std::error::Error;

struct Input(Vec<Equation>);

struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

impl TryFrom<&str> for Input {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .lines()
            .map(|line| -> Result<_, Self::Error> {
                let (target, numbers) = line.split_once(": ").ok_or("Invalid input")?;
                Ok(Equation {
                    target: target.parse()?,
                    numbers: numbers
                        .split(' ')
                        .map(|n| n.parse::<u64>())
                        .collect::<Result<Vec<_>, _>>()?,
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Input)
    }
}

fn concat(a: u64, b: u64) -> u64 {
    if b == 0 {
        a * 10
    } else {
        a * 10u64.pow(b.ilog10() + 1) + b
    }
}

impl Equation {
    fn is_valid(&self, use_concat: bool) -> bool {
        let mut partials = vec![self.numbers[0]];
        for number in &self.numbers[1..] {
            let mut new_partials = Vec::with_capacity(partials.len() * 3);
            for partial in partials {
                if partial <= self.target {
                    new_partials.push(partial + number);
                    new_partials.push(partial * number);
                    if use_concat {
                        new_partials.push(concat(partial, *number));
                    }
                }
            }
            partials = new_partials;
        }
        partials.contains(&self.target)
    }
}

pub fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let input = Input::try_from(input)?;
    let mut total_calibration_result = 0;
    for equation in input.0 {
        if equation.is_valid(false) {
            total_calibration_result += equation.target;
        }
    }
    Ok(total_calibration_result)
}

pub fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let input = Input::try_from(input)?;
    let mut total_calibration_result = 0;
    for equation in input.0 {
        if equation.is_valid(true) {
            total_calibration_result += equation.target;
        }
    }
    Ok(total_calibration_result)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 3749);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part1(&fetch_input(AocId {
                year: 2024,
                day: 7,
                part: 1
            })?)?,
            932137732557
        );
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 11387);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part2(&fetch_input(AocId {
                year: 2024,
                day: 7,
                part: 2
            })?)?,
            661823605105500
        );
        Ok(())
    }
}
