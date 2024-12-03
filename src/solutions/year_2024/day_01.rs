use std::collections::HashMap;
use std::{error::Error, str::FromStr};

struct Input(Vec<u32>, Vec<u32>);

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut input = Input(vec![], vec![]);
        for line in s.split('\n') {
            if line.is_empty() {
                break;
            }

            let mut nums = line.split("   ");
            let num1 = nums.next().ok_or("Lists are not of equal length")?;
            input.0.push(u32::from_str(num1)?);
            let num2 = nums.next().ok_or("Lists are not of equal length")?;
            input.1.push(u32::from_str(num2)?);
        }
        Ok(input)
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut input = Input::from_str(input)?;
    input.0.sort_unstable();
    input.1.sort_unstable();

    let mut total_distance = 0;
    for item in input.0.into_iter().zip(input.1) {
        if item.0 > item.1 {
            total_distance += item.0 - item.1;
        } else {
            total_distance += item.1 - item.0;
        }
    }

    Ok(total_distance)
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let input = Input::from_str(input)?;

    let mut occurrences = HashMap::new();
    for num in input.1.into_iter() {
        occurrences.entry(num).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut similarity_score = 0;
    for num in input.0.into_iter() {
        similarity_score += num * occurrences.get(&num).unwrap_or(&0);
    }

    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let answer = part1(&fetch_input(AocId {
            year: 2024,
            day: 1,
            part: 1,
        })?)?;
        assert_eq!(answer, 2264607);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let answer = part2(&fetch_input(AocId {
            year: 2024,
            day: 1,
            part: 2,
        })?)?;
        assert_eq!(answer, 19457120);
        Ok(())
    }
}
