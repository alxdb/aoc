use std::error::Error;
use std::ops::ControlFlow;
use std::str::{Chars, FromStr};

trait NegIndex<T> {
    fn get_neg(&self, i: i32) -> Option<&T>;
}

impl<T> NegIndex<T> for [T] {
    fn get_neg(&self, i: i32) -> Option<&T> {
        i.try_into().ok().and_then(|i: usize| self.get(i))
    }
}

struct Grid(Vec<Vec<char>>);

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split('\n')
            .map(str::chars)
            .map(Chars::collect::<Vec<_>>)
            .collect::<Vec<_>>();

        Ok(Grid(data))
    }
}

impl Grid {
    fn iter(&self) -> impl Iterator<Item = (i32, i32, char)> + '_ {
        self.0.iter().enumerate().flat_map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_i, &c)| (row_i as i32, col_i as i32, c))
        })
    }

    fn get(&self, row_i: i32, col_i: i32) -> Option<char> {
        self.0
            .get_neg(row_i)
            .and_then(|row| row.get_neg(col_i))
            .copied()
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    let grid = Grid::from_str(input)?;

    let mut count = 0;
    for (row_i, col_i, c) in grid.iter() {
        if c == XMAS[0] {
            for row_dir in -1..=1 {
                for col_dir in -1..=1 {
                    let is_xmas = (1..=3).try_for_each(|i| {
                        if let Some(c) = grid.get(row_i + row_dir * i, col_i + col_dir * i) {
                            if c == XMAS[i as usize] {
                                return ControlFlow::Continue(());
                            }
                        }
                        ControlFlow::Break(())
                    });
                    if is_xmas.is_continue() {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(EXAMPLE_INPUT)?, 18);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part1(&fetch_input(AocId {
                year: 2024,
                day: 4,
                part: 1
            })?)?,
            2633
        );
        Ok(())
    }
}
