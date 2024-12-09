use std::error::Error;

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    todo!()
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    const EXAMPLE_INPUT_2: &str = "###
..#
^##";

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 41);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part1(&fetch_input(AocId {
                year: 2024,
                day: 6,
                part: 1
            })?)?,
            5453
        );
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 6);
        Ok(())
    }

    #[test]
    fn test_part2_example_2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE_INPUT_2)?, 0);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part2(&fetch_input(AocId {
                year: 2024,
                day: 6,
                part: 2
            })?)?,
            2188
        );
        Ok(())
    }
}
