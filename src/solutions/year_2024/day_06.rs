use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn cw(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn add(&self, dir: Dir) -> Coord {
        match dir {
            Dir::N => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Dir::E => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Dir::S => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Dir::W => Coord {
                row: self.row,
                col: self.col - 1,
            },
        }
    }

    fn in_bounds(&self, max_row: i32, max_col: i32) -> bool {
        self.row >= 0 && self.row < max_row && self.col >= 0 && self.col < max_col
    }
}

#[derive(Clone)]
enum Tile {
    Obstacle,
    Empty,
    Path(HashSet<Dir>),
}

struct Guard {
    coord: Coord,
    dir: Dir,
}

impl Guard {
    fn turn(&mut self) {
        self.dir = self.dir.cw();
    }

    fn facing(&self) -> Coord {
        self.coord.add(self.dir)
    }
}

struct Map {
    grid: HashMap<Coord, Tile>,
    max_row: i32,
    max_col: i32,
    guard: Guard,
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grid = HashMap::new();
        let mut guard = None;
        let max_row = value.lines().count() as i32;
        let max_col = value.lines().next().unwrap().len() as i32;

        for (row, line) in value.lines().enumerate() {
            for (col, char) in line.char_indices() {
                let coord = Coord {
                    row: row as i32,
                    col: col as i32,
                };
                let guard_dir = match char {
                    '.' => None,
                    '#' => {
                        grid.insert(coord, Tile::Obstacle);
                        None
                    }
                    '^' => Some(Dir::N),
                    '>' => Some(Dir::E),
                    'v' => Some(Dir::S),
                    '<' => Some(Dir::W),
                    _ => return Err("invalid char"),
                };
                if let Some(dir) = guard_dir {
                    if guard.is_none() {
                        guard = Some(Guard { coord, dir });
                        grid.insert(coord, Tile::Path(HashSet::from([dir])));
                    } else {
                        return Err("Multiple guards");
                    }
                }
            }
        }
        guard.ok_or("No guard found").map(|guard| Map {
            grid,
            max_row,
            max_col,
            guard,
        })
    }
}

impl From<Map> for String {
    fn from(map: Map) -> Self {
        let mut result = String::new();
        for row in 0..map.max_row {
            for col in 0..map.max_col {
                match map.get_tile(Coord { row, col }) {
                    Some(Tile::Obstacle) => result.push('#'),
                    Some(Tile::Empty) => result.push('.'),
                    Some(Tile::Path(dirs)) => {
                        if dirs.is_subset(&HashSet::from([Dir::N, Dir::S])) {
                            result.push('|');
                        } else if dirs.is_subset(&HashSet::from([Dir::E, Dir::W])) {
                            result.push('-');
                        } else {
                            result.push('+');
                        }
                        // [Dir::N] | [Dir::S] | [Dir::N, Dir::S] => result.push('|'),
                        // [Dir::E] | [Dir::W] | [Dir::E, Dir::W] => result.push('-'),
                        // _ => result.push('+'),
                    }
                    None => unreachable!(),
                }
            }
            result.push('\n');
        }
        result
    }
}

impl Map {
    fn get_tile(&self, coord: Coord) -> Option<Tile> {
        if let Some(tile) = self.grid.get(&coord) {
            Some(tile.clone())
        } else if !coord.in_bounds(self.max_row, self.max_col) {
            None
        } else {
            Some(Tile::Empty)
        }
    }

    fn turn_guard(&mut self) {
        self.guard.turn();
        match self.grid.get_mut(&self.guard.coord).unwrap() {
            Tile::Path(dirs) => dirs.insert(self.guard.dir),
            _ => unreachable!(),
        };
    }

    /// Return None if termination is uncertain
    /// Return Some(false) if path terminates
    /// Return Some(true) if path loops
    fn step_guard(&mut self) -> Option<bool> {
        let facing = self.guard.facing();
        match self.get_tile(facing) {
            None => Some(false),
            Some(tile) => match tile {
                Tile::Obstacle => {
                    self.guard.turn();
                    match self.grid.get_mut(&self.guard.coord).unwrap() {
                        Tile::Path(dirs) => {
                            if dirs.contains(&self.guard.dir) {
                                return Some(true);
                            } else {
                                dirs.insert(self.guard.dir);
                            }
                        }
                        _ => unreachable!(),
                    }
                    None
                }
                Tile::Empty => {
                    self.guard.coord = facing;
                    self.grid
                        .insert(facing, Tile::Path(HashSet::from([self.guard.dir])));
                    None
                }
                Tile::Path(mut dirs) => {
                    if dirs.contains(&self.guard.dir) {
                        Some(true)
                    } else {
                        self.guard.coord = facing;
                        dirs.insert(self.guard.dir);
                        self.grid.insert(facing, Tile::Path(dirs));
                        None
                    }
                }
            },
        }
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut map = Map::try_from(input)?;

    while let Some(tile) = map.get_tile(map.guard.facing()) {
        match tile {
            Tile::Obstacle => {
                map.guard.turn();
                match map.grid.get_mut(&map.guard.coord).unwrap() {
                    Tile::Path(dirs) => dirs.insert(map.guard.dir),
                    _ => unreachable!(),
                };
            }
            Tile::Empty => {
                map.guard.coord = map.guard.facing();
                map.grid
                    .insert(map.guard.coord, Tile::Path(HashSet::from([map.guard.dir])));
            }
            Tile::Path(mut dirs) => {
                map.guard.coord = map.guard.facing();
                dirs.insert(map.guard.dir);
                map.grid.insert(map.guard.coord, Tile::Path(dirs));
            }
        }
    }

    let tiles_visited = map
        .grid
        .values()
        .filter(|tile| matches!(tile, Tile::Path(_)))
        .count();

    println!("{}", Into::<String>::into(map));

    Ok(tiles_visited as u32)
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
