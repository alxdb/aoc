use std::{error::Error, fmt::Debug};

#[derive(Clone, Copy)]
enum Block {
    File { id: u64 },
    Free,
}

struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    fn from_disk_map(disk_map: &str) -> Self {
        Self {
            blocks: disk_map
                .chars()
                .filter(|c| *c != '\n')
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .flat_map(|(index, value)| {
                    let is_file = index % 2 == 0;
                    let file_id = (index / 2) as u64;
                    let length = value as usize;
                    vec![
                        if is_file {
                            Block::File { id: file_id }
                        } else {
                            Block::Free
                        };
                        length
                    ]
                })
                .collect(),
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(index, block)| match block {
                Block::Free => None,
                Block::File { id } => Some((index as u64, id)),
            })
            .map(|(index, id)| index * id)
            .sum()
    }
}

impl Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            match block {
                Block::File { id } => write!(f, "{}", id)?,
                Block::Free => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut disk = Disk::from_disk_map(input);
    let mut free_index = 0;
    let mut file_index = disk
        .blocks
        .iter()
        .enumerate()
        .rev()
        .find(|(_, b)| matches!(b, Block::File { .. }))
        .map(|(i, _)| i)
        .unwrap();

    loop {
        match disk.blocks[free_index] {
            Block::Free => {
                disk.blocks.swap(free_index, file_index);
                while matches!(disk.blocks[file_index], Block::Free) {
                    file_index -= 1;
                }
            }
            Block::File { .. } => {
                free_index += 1;
            }
        }
        if free_index == file_index {
            break;
        }
    }
    Ok(disk.checksum())
}

pub fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::{fetch_input, AocId};
    use std::error::Error;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE_INPUT)?, 1928);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part1(&fetch_input(AocId {
                year: 2024,
                day: 9,
                part: 1
            })?)?,
            6386640365805
        );
        Ok(())
    }

    // #[test]
    // fn test_part2_example() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2(EXAMPLE_INPUT)?, 0);
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(
    //         part2(&fetch_input(AocId {
    //             year: 2024,
    //             day: 9,
    //             part: 2
    //         })?)?,
    //         0
    //     );
    //     Ok(())
    // }
}
