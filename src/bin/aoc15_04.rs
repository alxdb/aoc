include!(concat!(env!("OUT_DIR"), "/aoc_prefix.rs"));

fn part1(input: &str) -> u64 {
    let mut salt = 0;
    loop {
        let input = format!("{}{}", input.strip_suffix("\n").unwrap(), salt);
        let digest = md5::compute(input);
        if format!("{:x}", digest).starts_with("00000") {
            return salt;
        }
        salt += 1;
    }
}

fn part2(input: &str) -> u64 {
    let mut salt = 0;
    loop {
        let input = format!("{}{}", input.strip_suffix("\n").unwrap(), salt);
        let digest = md5::compute(input);
        if format!("{:x}", digest).starts_with("000000") {
            return salt;
        }
        salt += 1;
    }
}
