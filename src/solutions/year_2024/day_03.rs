use std::error::Error;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut chars = input.chars().peekable();
    let mut total = 0;
    while let Some(c) = chars.next() {
        if c == 'm'
            && chars.next() == Some('u')
            && chars.next() == Some('l')
            && chars.next() == Some('(')
        {
            let mut parse_digit = |end_char| {
                let mut digits = String::new();
                while let Some(c) = chars.peek() {
                    if digits.len() < 4 && c.is_ascii_digit() {
                        digits.push(chars.next().unwrap());
                    } else if !digits.is_empty() && *c == end_char {
                        chars.next().unwrap();
                        return Some(u32::from_str(&digits).unwrap());
                    } else {
                        return None;
                    }
                }
                None
            };
            if let Some(a) = parse_digit(',') {
                if let Some(b) = parse_digit(')') {
                    total += a * b;
                }
            }
        }
    }
    Ok(total)
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut total = 0;
    let mut start = 0;
    loop {
        let end = input[start..]
            .find("don't()")
            .map(|end| start + end)
            .unwrap_or(input.len());
        total += part1(&input[start..end])?;
        if end == input.len() {
            break;
        } else if let Some(new_start) = input[end..].find("do()") {
            start = end + new_start;
            if start >= input.len() {
                break;
            }
        } else {
            break;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        const EXAMPLE_INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(EXAMPLE_INPUT)?, 161);
        Ok(())
    }

    #[test]
    fn test_part1_nested_mul() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1("mul(mul(1,2)")?, 2);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        let input = &fetch_input(AocId {
            year: 2024,
            day: 3,
            part: 1,
        })?;
        assert_eq!(part1(input)?, 171183089);
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        const EXAMPLE_INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(EXAMPLE_INPUT)?, 48);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        let input = &fetch_input(AocId {
            year: 2024,
            day: 3,
            part: 2,
        })?;
        assert_eq!(part2(input)?, 63866497);
        Ok(())
    }
}
