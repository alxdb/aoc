use std::{collections::HashSet, error::Error};

#[derive(Debug)]
enum InvalidInput {
    InvalidChar(char),
    EmptyInput,
    UnequalLineLength,
    MultipleGuards,
}

impl std::fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidInput::InvalidChar(c) => write!(f, "invalid char '{}'", c),
            InvalidInput::EmptyInput => write!(f, "input is empty"),
            InvalidInput::UnequalLineLength => write!(f, "input lines of differing length"),
            InvalidInput::MultipleGuards => write!(f, "multiple guards in input"),
        }
    }
}

impl Error for InvalidInput {}

enum Direction {
    N,
    E,
    S,
    W,
}

impl TryFrom<char> for Direction {
    type Error = InvalidInput;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::N),
            '>' => Ok(Direction::E),
            'v' => Ok(Direction::S),
            '<' => Ok(Direction::W),
            _ => Err(InvalidInput::InvalidChar(value)),
        }
    }
}

enum Tile {
    Obstruction,
    Empty { visited: bool },
}

struct Grid {
    tiles: Vec<Tile>,
    n_cols: usize,
    n_rows: usize,
    guard_coords: [usize; 2],
    guard_direction: Direction,
}

impl TryFrom<&str> for Grid {
    type Error = InvalidInput;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(InvalidInput::EmptyInput);
        }

        let mut tiles: Vec<Tile> = Vec::new();
        let mut n_cols: Option<usize> = None;
        let mut guard_index: Option<[usize; 2]> = None;
        let mut guard_direction: Option<Direction> = None;

        for (row_n, line) in value.lines().enumerate() {
            if let Some(n_cols) = n_cols {
                if line.len() != n_cols {
                    return Err(InvalidInput::UnequalLineLength);
                }
            } else {
                if line.is_empty() {
                    return Err(InvalidInput::EmptyInput);
                }
                n_cols = Some(line.len())
            }
            for (col_n, c) in line.char_indices() {
                match c {
                    '.' => tiles.push(Tile::Empty { visited: false }),
                    '#' => tiles.push(Tile::Obstruction),
                    _ => {
                        match guard_direction {
                            None => guard_direction = Some(Direction::try_from(c)?),
                            Some(_) => return Err(InvalidInput::MultipleGuards),
                        };
                        guard_index = Some([row_n, col_n]);
                        tiles.push(Tile::Empty { visited: true });
                    }
                }
            }
        }

        let n_rows = tiles.len() / n_cols.unwrap();
        Ok(Grid {
            tiles,
            n_cols: n_cols.unwrap(),
            n_rows,
            guard_coords: guard_index.unwrap(),
            guard_direction: guard_direction.unwrap(),
        })
    }
}

impl Grid {
    fn at(&self, row: usize, col: usize) -> Option<&Tile> {
        if col >= self.n_cols {
            None
        } else {
            self.tiles.get(row * self.n_cols + col)
        }
    }

    fn at_mut(&mut self, row: usize, col: usize) -> Option<&mut Tile> {
        if col >= self.n_cols {
            None
        } else {
            self.tiles.get_mut(row * self.n_cols + col)
        }
    }

    fn guard_is_facing(&mut self) -> Option<(&mut Tile, [usize; 2])> {
        let facing_coords = match self.guard_direction {
            Direction::N => [self.guard_coords[0] - 1, self.guard_coords[1]],
            Direction::E => [self.guard_coords[0], self.guard_coords[1] + 1],
            Direction::S => [self.guard_coords[0] + 1, self.guard_coords[1]],
            Direction::W => [self.guard_coords[0], self.guard_coords[1] - 1],
        };
        self.at_mut(facing_coords[0], facing_coords[1])
            .map(|t| (t, facing_coords))
    }

    fn turn_guard(&mut self) {
        self.guard_direction = match self.guard_direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut grid = Grid::try_from(input)?;

    while let Some((tile, coords)) = grid.guard_is_facing() {
        match tile {
            Tile::Empty { visited } => {
                *visited = true;
                grid.guard_coords = coords;
            }
            Tile::Obstruction => grid.turn_guard(),
        }
    }

    Ok(grid
        .tiles
        .into_iter()
        .filter(|t| matches!(t, Tile::Empty { visited: true }))
        .count()
        .try_into()?)
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

    // #[test]
    // fn test_part2_example() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2(EXAMPLE_INPUT)?, 123);
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(
    //         part2(&fetch_input(AocId {
    //             year: 2024,
    //             day: 6,
    //             part: 2
    //         })?)?,
    //         5285
    //     );
    //     Ok(())
    // }
}
