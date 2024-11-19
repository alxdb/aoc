use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
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
    let input = input.strip_suffix("\n").unwrap().to_owned();
    let result = Arc::new(AtomicU64::new(u64::MAX));
    let n_workers = available_parallelism().unwrap().get();
    let step = n_workers as u64;

    std::thread::scope(|s| {
        let workers: Vec<_> = (0..step)
            .map(|i| {
                let input = input.clone();
                let result = result.clone();
                s.spawn(move || {
                    let mut salt = i;
                    while result.load(Ordering::Acquire) > salt {
                        if test_salt(input.clone(), salt, leading_zeros) {
                            result.fetch_min(salt, Ordering::AcqRel);
                        }
                        salt += step;
                    }
                })
            })
            .collect();

        for worker in workers {
            worker.join().unwrap();
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
