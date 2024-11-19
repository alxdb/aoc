#![feature(iter_array_chunks)]

use std::collections::HashSet;

aoc::main!();

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let vowels = ['a', 'e', 'i', 'o', 'u'];
            let banned_patterns = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

            let mut vowel_count = 0;
            let mut prev_letter = None;
            let mut found_repeat = false;
            for char in line.chars() {
                if vowel_count < 3 && vowels.contains(&char) {
                    vowel_count += 1;
                }
                if let Some(prev) = prev_letter {
                    if !found_repeat && char == prev {
                        found_repeat = true;
                    }
                    if banned_patterns.contains(&(prev, char)) {
                        eprintln!("line={} banned prev={} char={}", line, prev, char);
                        return false;
                    }
                }
                prev_letter = Some(char);
            }
            eprintln!(
                "line={}, found_repeat={}, vowel_count={}",
                line, found_repeat, vowel_count
            );
            found_repeat && vowel_count >= 3
        })
        .count()
}

fn has_repeating_chunk(chunks: impl Iterator<Item = [char; 2]>) -> bool {
    let mut found_chunks: HashSet<[char; 2]> = HashSet::new();
    for chunk in chunks {
        if found_chunks.contains(&chunk) {
            eprintln!("chunk={:?}", chunk);
            return true;
        } else {
            found_chunks.insert(chunk);
        }
    }
    false
}

fn has_between_chunk(chunks: impl Iterator<Item = [char; 3]>) -> bool {
    for chunk in chunks {
        if chunk[0] == chunk[2] {
            eprintln!("chunk={:?}", chunk);
            return true;
        }
    }
    false
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let has_repeating_chunk = has_repeating_chunk(line.chars().array_chunks::<2>())
                || has_repeating_chunk(line.chars().skip(1).array_chunks::<2>());
            let has_between_chunk = has_between_chunk(line.chars().array_chunks::<3>())
                || has_between_chunk(line.chars().skip(1).array_chunks::<3>())
                || has_between_chunk(line.chars().skip(2).array_chunks::<3>());
            eprintln!(
                "line={} has_repeating_chunk={}, has_between_chunk={}",
                line, has_repeating_chunk, has_between_chunk
            );
            has_repeating_chunk && has_between_chunk
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part2("xxyxx"), 1);
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }
}
