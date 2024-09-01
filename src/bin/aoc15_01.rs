use std::{iter::Map, str::Chars};

aoc::main!();

fn parse_input(input: &str) -> Map<Chars, impl FnMut(char) -> i32> {
    input.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => panic!("invalid char"),
    })
}

fn part1(input: &str) -> i32 {
    parse_input(input).sum()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .find(|&(_, x)| x == -1)
        .unwrap()
        .0
}
