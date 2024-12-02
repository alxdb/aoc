use std::{error::Error, str::FromStr};

struct Input(Vec<u32>, Vec<u32>);

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut input = Input(vec![], vec![]);
        for line in s.split('\n') {
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

    todo!()
}
