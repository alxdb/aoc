use std::error::Error;

use bit_set::BitSet;

#[derive(Clone, Copy)]
enum Axis {
    N,
    E,
    S,
    W,
}

impl Axis {
    fn cw(&self) -> Axis {
        match self {
            Axis::N => Axis::E,
            Axis::E => Axis::S,
            Axis::S => Axis::W,
            Axis::W => Axis::N,
        }
    }
}

impl TryFrom<&char> for Axis {
    type Error = &'static str;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Axis::N),
            '>' => Ok(Axis::E),
            'v' => Ok(Axis::S),
            '<' => Ok(Axis::W),
            _ => Err("invalid input"),
        }
    }
}

#[derive(Clone)]
struct Map {
    visited: [BitSet; 4],
    obstacles: BitSet,
    size: [usize; 2],
    guard_coords: [usize; 2],
    guard_axis: Axis,
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Map, Self::Error> {
        let contents: Vec<Vec<char>> = value
            .lines()
            .map(str::chars)
            .map(Iterator::collect)
            .collect();

        let size = [contents.len(), contents[0].len()];
        let set_size = size[0] * size[1];
        let mut visited = [
            BitSet::with_capacity(set_size),
            BitSet::with_capacity(set_size),
            BitSet::with_capacity(set_size),
            BitSet::with_capacity(set_size),
        ];
        let mut obstacles = BitSet::with_capacity(set_size);
        let mut guard_coords = [usize::MAX, usize::MAX];
        let mut guard_axis = Axis::N;

        for (row, line) in contents.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                let index = row * size[0] + col;
                match c {
                    '.' => continue,
                    '#' => {
                        obstacles.insert(index);
                    }
                    _ => {
                        guard_axis = c.try_into()?;
                        guard_coords = [row, col];
                        visited[guard_axis as usize].insert(index);
                    }
                }
            }
        }

        Ok(Map {
            visited,
            obstacles,
            size,
            guard_coords,
            guard_axis,
        })
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size[0] {
            for col in 0..self.size[1] {
                let coord = [row, col];
                let index = self.get_index(&coord);
                if self.guard_coords == coord {
                    match self.guard_axis {
                        Axis::N => write!(f, "^")?,
                        Axis::E => write!(f, ">")?,
                        Axis::S => write!(f, "v")?,
                        Axis::W => write!(f, "<")?,
                    }
                } else if self.obstacles.contains(index) {
                    write!(f, "#")?;
                } else {
                    let mut axes = BitSet::with_capacity(4);
                    if self.visited[Axis::N as usize].contains(index) {
                        axes.insert(Axis::N as usize);
                    }
                    if self.visited[Axis::E as usize].contains(index) {
                        axes.insert(Axis::E as usize);
                    }
                    if self.visited[Axis::S as usize].contains(index) {
                        axes.insert(Axis::S as usize);
                    }
                    if self.visited[Axis::W as usize].contains(index) {
                        axes.insert(Axis::W as usize);
                    }
                    if axes.is_empty() {
                        write!(f, ".")?;
                    } else if axes.is_subset(&BitSet::from_bytes(&[0b10100000])) {
                        write!(f, "|")?;
                    } else if axes.is_subset(&BitSet::from_bytes(&[0b01010000])) {
                        write!(f, "-")?;
                    } else {
                        write!(f, "+")?;
                    }
                }
            }
            if row < self.size[0] - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum End {
    Loop,
    Edge,
}

enum Hit {
    Obstacle,
    End(End),
    Empty([usize; 2]),
}

impl Map {
    fn get_index(&self, coords: &[usize; 2]) -> usize {
        coords[0] * self.size[1] + coords[1]
    }

    fn move_by(&self, coords: &[usize; 2], axis: Axis) -> Option<[usize; 2]> {
        match axis {
            Axis::N => {
                if coords[0] == 0 {
                    None
                } else {
                    Some([coords[0] - 1, coords[1]])
                }
            }
            Axis::E => {
                if coords[1] == (self.size[1] - 1) {
                    None
                } else {
                    Some([coords[0], coords[1] + 1])
                }
            }
            Axis::S => {
                if coords[0] == (self.size[0] - 1) {
                    None
                } else {
                    Some([coords[0] + 1, coords[1]])
                }
            }
            Axis::W => {
                if coords[1] == 0 {
                    None
                } else {
                    Some([coords[0], coords[1] - 1])
                }
            }
        }
    }

    fn guard_facing(&self) -> Hit {
        if let Some(coords) = self.move_by(&self.guard_coords, self.guard_axis) {
            let index = self.get_index(&coords);
            if self.obstacles.contains(index) {
                Hit::Obstacle
            } else if self.visited[self.guard_axis as usize].contains(index) {
                Hit::End(End::Loop)
            } else {
                Hit::Empty(coords)
            }
        } else {
            Hit::End(End::Edge)
        }
    }

    fn step_guard(&mut self, hit: &Hit) -> Option<End> {
        match hit {
            Hit::End(end) => Some(*end),
            Hit::Obstacle => {
                self.guard_axis = self.guard_axis.cw();
                if !self.visited[self.guard_axis as usize]
                    .insert(self.get_index(&self.guard_coords))
                {
                    Some(End::Loop)
                } else {
                    None
                }
            }
            Hit::Empty(coords) => {
                self.guard_coords = *coords;
                self.visited[self.guard_axis as usize].insert(self.get_index(coords));
                None
            }
        }
    }

    fn visited_any(&self, coord: &[usize; 2]) -> bool {
        let index = self.get_index(coord);
        for i in 0..=3 {
            if self.visited[i].contains(index) {
                return true;
            }
        }
        false
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut map: Map = input.try_into()?;
    loop {
        if map.step_guard(&map.guard_facing()).is_some() {
            break;
        }
    }
    let mut visited = 0;
    for row in 0..map.size[0] {
        for col in 0..map.size[1] {
            if map.visited_any(&[row, col]) {
                visited += 1;
            }
        }
    }
    Ok(visited)
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut map: Map = input.try_into()?;
    let mut loops = 0;
    loop {
        let hit = map.guard_facing();
        match hit {
            Hit::End(_) => break,
            Hit::Obstacle => (),
            Hit::Empty(coord) => {
                if !map.visited_any(&coord) {
                    let mut alt_map = map.clone();
                    alt_map.obstacles.insert(alt_map.get_index(&coord));
                    loop {
                        match alt_map.step_guard(&alt_map.guard_facing()) {
                            Some(End::Loop) => {
                                loops += 1;
                                break;
                            }
                            Some(End::Edge) => {
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        map.step_guard(&hit);
    }
    Ok(loops)
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
