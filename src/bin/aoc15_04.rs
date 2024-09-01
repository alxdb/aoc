include!(concat!(env!("OUT_DIR"), "/aoc_prefix.rs"));

fn mine(input: &str, prefix: &str) -> u64 {
    let mut salt = 0;
    loop {
        let input = format!("{}{}", input.strip_suffix("\n").unwrap(), salt);
        let digest = md5::compute(input);
        if format!("{:x}", digest).starts_with(prefix) {
            return salt;
        }
        salt += 1;
    }
}

fn part1(input: &str) -> u64 {
    mine(input, "00000")
}

fn part2(input: &str) -> u64 {
    mine(input, "000000")
}
