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

// #[derive(Clone)]
// enum Tile {
//     Obstacle,
//     Path(HashSet<Dir>),
// }

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
    obstacles: HashSet<Coord>,
    path: HashMap<Coord, HashSet<Dir>>,
    max_row: i32,
    max_col: i32,
    guard: Guard,
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut obstacles = HashSet::new();
        let mut path = HashMap::new();
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
                        obstacles.insert(coord);
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
                        path.insert(coord, HashSet::from([dir]));
                    } else {
                        return Err("Multiple guards");
                    }
                }
            }
        }
        guard.ok_or("No guard found").map(|guard| Map {
            obstacles,
            path,
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
                let coord = Coord { row, col };
                if map.obstacles.get(&coord).is_some() {
                    result.push('#');
                } else if let Some(dirs) = map.path.get(&coord) {
                    if dirs.is_subset(&HashSet::from([Dir::N, Dir::S])) {
                        result.push('|');
                    } else if dirs.is_subset(&HashSet::from([Dir::E, Dir::W])) {
                        result.push('-');
                    } else {
                        result.push('+');
                    }
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        result
    }
}

impl Map {
    /// Returns whether or not the path will loop
    /// Returns None if this is not yet known
    fn step_guard(&mut self) -> Option<bool> {
        if !self.guard.facing().in_bounds(self.max_row, self.max_col) {
            Some(false)
        } else if self.obstacles.contains(&self.guard.facing()) {
            self.guard.turn();

            let dirs = self.path.get_mut(&self.guard.coord).unwrap();
            if dirs.contains(&self.guard.dir) {
                Some(true)
            } else {
                dirs.insert(self.guard.dir);
                None
            }
        } else if let Some(dirs) = self.path.get_mut(&self.guard.facing()) {
            if dirs.contains(&self.guard.dir) {
                Some(true)
            } else {
                dirs.insert(self.guard.dir);
                self.guard.coord = self.guard.facing();
                None
            }
        } else {
            self.path
                .insert(self.guard.facing(), HashSet::from([self.guard.dir]));
            self.guard.coord = self.guard.facing();
            None
        }
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut map = Map::try_from(input)?;

    loop {
        if let Some(loops) = map.step_guard() {
            if loops {
                panic!("part 1 loops")
            } else {
                break;
            }
        }
    }
    let tiles_visited = map.path.keys().count();

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
