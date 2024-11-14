use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread::available_parallelism,
};

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
    // This may return a value that is not the smallest possible salt
    // May be aleviated with barriers and `fetch_min`, but barriers lead to hanging
    let input = input.strip_suffix("\n").unwrap().to_owned();
    std::thread::scope(|s| {
        let stop = Arc::new(AtomicBool::new(false));
        let result = Arc::new(AtomicU64::new(0));
        let n_workers = available_parallelism().unwrap().get();
        let step = n_workers as u64;
        for i in 0..step {
            let input = input.clone();
            let stop = stop.clone();
            let result = result.clone();
            s.spawn(move || {
                let mut salt = i;
                while !stop.load(Ordering::Relaxed) {
                    if test_salt(input.clone(), salt, leading_zeros) {
                        result.store(salt, Ordering::Relaxed);
                        stop.store(true, Ordering::Relaxed);
                    }
                    salt += step;
                }
            });
        }
        while !stop.load(Ordering::Relaxed) {
            std::hint::spin_loop();
        }
        result.load(Ordering::Relaxed)
    })
}

fn part1(input: &str) -> u64 {
    mine(input, 5)
}

fn part2(input: &str) -> u64 {
    mine(input, 6)
}
