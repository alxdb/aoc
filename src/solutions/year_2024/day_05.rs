use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    ordering_rules: Vec<[u32; 2]>,
    updates: Vec<Vec<u32>>,
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("\n\n").ok_or("No separator".into()).and_then(
            |(ordering_rule_lines, update_lines)| {
                let ordering_rules = ordering_rule_lines
                    .lines()
                    .map(|line| {
                        line.split_once('|')
                            .ok_or::<Box<dyn Error>>("Invalid ordering rule".into())
                            .and_then(|(page_1, page_2)| {
                                Ok([u32::from_str(page_1)?, u32::from_str(page_2)?])
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let updates = update_lines
                    .lines()
                    .map(|line| line.split(',').map(u32::from_str).collect())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Input {
                    ordering_rules,
                    updates,
                })
            },
        )
    }
}

fn is_correctly_sorted(update: &[u32], pages_before_page: &HashMap<u32, HashSet<u32>>) -> bool {
    !update.iter().enumerate().any(|(i, page)| {
        update.iter().skip(i + 1).any(|page_after| {
            pages_before_page
                .get(page)
                .map_or(false, |pages_before| pages_before.contains(page_after))
        })
    })
}

pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let input = Input::from_str(input)?;

    let mut pages_before_page: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in input.ordering_rules {
        pages_before_page
            .entry(rule[1])
            .or_default()
            .insert(rule[0]);
    }

    let result = input
        .updates
        .iter()
        .filter(|update| is_correctly_sorted(update, &pages_before_page))
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum();

    Ok(result)
}

pub fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let input = Input::from_str(input)?;

    let mut pages_before_page: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in &input.ordering_rules {
        pages_before_page
            .entry(rule[1])
            .or_default()
            .insert(rule[0]);
    }
    let mut pages_after_page: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in &input.ordering_rules {
        pages_after_page.entry(rule[0]).or_default().insert(rule[1]);
    }

    let result = input
        .updates
        .iter()
        .filter(|update| !is_correctly_sorted(update, &pages_before_page))
        .map(|invalid_update| {
            let mut valid_update = invalid_update.clone();
            valid_update.sort_by(|a, b| {
                let is_before = pages_before_page
                    .get(a)
                    .map(|pages_before| pages_before.contains(b))
                    .and_then(|is_before| {
                        if is_before {
                            Some(Ordering::Less)
                        } else {
                            None
                        }
                    });
                let is_after = pages_after_page
                    .get(a)
                    .map(|pages_after| pages_after.contains(b))
                    .and_then(|is_before| {
                        if is_before {
                            Some(Ordering::Greater)
                        } else {
                            None
                        }
                    });
                is_before.or(is_after).unwrap_or(Ordering::Equal)
            });
            valid_update
        })
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 143);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part1(&fetch_input(AocId {
                year: 2024,
                day: 5,
                part: 1
            })?)?,
            4135
        );
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 123);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part2(&fetch_input(AocId {
                year: 2024,
                day: 5,
                part: 2
            })?)?,
            5285
        );
        Ok(())
    }
}
