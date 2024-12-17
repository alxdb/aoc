use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

#[allow(dead_code)]
fn print_antinodes(input: &str, antinodes: &HashSet<[usize; 2]>) {
    let mut antinode_map = String::new();
    for (row_i, row) in input.lines().enumerate() {
        for (col_i, c) in row.char_indices() {
            if antinodes.contains(&[row_i, col_i]) {
                antinode_map.push('#');
            } else {
                antinode_map.push(c);
            }
        }
        antinode_map.push('\n');
    }
    println!("{}", antinode_map);
}

pub fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut node_map = HashMap::new();
    let size = [input.lines().count(), input.lines().next().unwrap().len()];
    for (row_i, row) in input.lines().enumerate() {
        for (col_i, c) in row.char_indices().filter(|(_, c)| *c != '.') {
            node_map
                .entry(c)
                .and_modify(|v: &mut HashSet<[usize; 2]>| {
                    v.insert([row_i, col_i]);
                })
                .or_insert(HashSet::from([[row_i, col_i]]));
        }
    }
    let mut antinodes = HashSet::new();
    for nodes in node_map.values() {
        for focus_node in nodes {
            for other_node in nodes.iter().filter(|n| *n != focus_node) {
                let mut antinode = *focus_node;
                for (x, y) in antinode.iter_mut().zip(other_node) {
                    *x += *x - y;
                }
                if (0..size[0]).contains(&antinode[0]) && (0..size[1]).contains(&antinode[1]) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    Ok(antinodes.len() as u64)
}

pub fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut node_map = HashMap::new();
    let size = [input.lines().count(), input.lines().next().unwrap().len()];
    for (row_i, row) in input.lines().enumerate() {
        for (col_i, c) in row.char_indices().filter(|(_, c)| *c != '.') {
            node_map
                .entry(c)
                .and_modify(|v: &mut HashSet<[usize; 2]>| {
                    v.insert([row_i, col_i]);
                })
                .or_insert(HashSet::from([[row_i, col_i]]));
        }
    }
    let mut antinodes = HashSet::new();
    for nodes in node_map.values() {
        for focus_node in nodes {
            for other_node in nodes.iter().filter(|n| *n != focus_node) {
                antinodes.insert(*other_node);

                let distance = {
                    let mut distance = *focus_node;
                    for (x, y) in distance.iter_mut().zip(other_node) {
                        *x -= y;
                    }
                    distance
                };

                let mut antinode = *focus_node;
                loop {
                    for (x, y) in antinode.iter_mut().zip(distance) {
                        *x += y;
                    }
                    if (0..size[0]).contains(&antinode[0]) && (0..size[1]).contains(&antinode[1]) {
                        antinodes.insert(antinode);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    Ok(antinodes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 14);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part1(&fetch_input(AocId {
                year: 2024,
                day: 8,
                part: 1
            })?)?,
            332
        );
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 34);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part2(&fetch_input(AocId {
                year: 2024,
                day: 8,
                part: 2
            })?)?,
            1174
        );
        Ok(())
    }
}
