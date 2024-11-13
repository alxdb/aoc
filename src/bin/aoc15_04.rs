aoc::main!();

fn test_salt(mut input: String, salt: u64, mut leading_zeros: usize) -> bool {
    input.push_str(&salt.to_string());
    let digest = md5::compute(input);
    let mut current_byte = 0;
    while leading_zeros > 2 {
        if digest.0[current_byte] != 0 {
            return false;
        } else {
            leading_zeros -= 2;
            current_byte += 1;
        }
    }
    if leading_zeros == 2 {
        digest.0[current_byte] == 0
    } else {
        digest.0[current_byte] <= 15
    }
}

fn mine(input: &str, leading_zeros: usize) -> u64 {
    let input = input.strip_suffix("\n").unwrap().to_owned();
    let mut salt = 0;
    loop {
        if test_salt(input.clone(), salt, leading_zeros) {
            return salt;
        }
        salt += 1;
    }
}

fn part1(input: &str) -> u64 {
    mine(input, 5)
}

fn part2(input: &str) -> u64 {
    mine(input, 6)
}
