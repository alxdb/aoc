use std::{collections::HashSet, iter::Map, str::Chars};

aoc::main!();

struct Dir(i8);

const INC: Dir = Dir(1);
const DEC: Dir = Dir(-1);
const NIL: Dir = Dir(0);

fn parse_input(input: &str) -> Map<Chars, impl FnMut(char) -> [Dir; 2]> {
    input.chars().map(|c| match c {
        '^' => [NIL, INC],
        'v' => [NIL, DEC],
        '>' => [INC, NIL],
        '<' => [DEC, NIL],
        _ => panic!("invalid char"),
    })
}

fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut current = [0, 0];
    visited.insert(current);
    for dirs in parse_input(input) {
        for (dim, dir) in current.iter_mut().zip(dirs) {
            *dim += dir.0;
        }
        visited.insert(current);
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut santa = [0, 0];
    visited.insert(santa);
    let mut robo_santa = [0, 0];
    let mut is_robo_santa = false;
    for dirs in parse_input(input) {
        let current = if is_robo_santa {
            &mut robo_santa
        } else {
            &mut santa
        };
        for (dim, dir) in current.iter_mut().zip(dirs) {
            *dim += dir.0;
        }
        visited.insert(*current);
        is_robo_santa = !is_robo_santa;
    }
    visited.len()
}
