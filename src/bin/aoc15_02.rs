include!(concat!(env!("OUT_DIR"), "/aoc_prefix.rs"));

fn parse_line(line: &str) -> [i64; 3] {
    line.split('x')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|x| {
            let side_areas = [x[0] * x[1], x[1] * x[2], x[0] * x[2]];
            side_areas.into_iter().map(|x| x * 2).sum::<i64>()
                + side_areas.into_iter().min().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|mut x| {
            x.sort();
            x[0] * 2 + x[1] * 2 + x.into_iter().product::<i64>()
        })
        .sum()
}
