use std::{error::Error, fmt::Debug};

use bit_set::BitSet;

#[derive(Clone, Copy, Debug)]
enum Block {
    File { id: usize, size: usize },
    Free { size: usize },
}

impl Block {
    fn size(&self) -> &usize {
        match self {
            Self::File { id: _, size } => size,
            Self::Free { size } => size,
        }
    }

    fn size_mut(&mut self) -> &mut usize {
        match self {
            Self::File { id: _, size } => size,
            Self::Free { size } => size,
        }
    }
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
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .map(|(index, value)| {
                    let is_file = index % 2 == 0;
                    let file_id = index / 2;
                    let size = value;
                    if is_file {
                        Block::File { id: file_id, size }
                    } else {
                        Block::Free { size }
                    }
                })
                .collect(),
        }
    }

    fn flatten(&mut self) {
        let mut blocks = self
            .blocks
            .iter()
            .flat_map(|&block| {
                let mut new_block = block;
                *new_block.size_mut() = 1;
                vec![new_block; *block.size()]
            })
            .collect();
        std::mem::swap(&mut blocks, &mut self.blocks);
    }

    fn checksum(&self) -> usize {
        let mut position = 0;
        let mut checksum = 0;
        for block in &self.blocks {
            match block {
                Block::File { id, size } => {
                    for i in 0..*size {
                        checksum += id * (i + position);
                    }
                    position += size;
                }
                Block::Free { size } => {
                    position += size;
                }
            }
        }
        checksum
    }

    // Replace block at index with free space.
    fn take_block(&mut self, index: usize) -> Block {
        let element = self.blocks.remove(index);
        self.blocks.insert(
            index,
            Block::Free {
                size: *element.size(),
            },
        );
        element
    }

    fn defrag(&mut self) {
        let mut moved = BitSet::with_capacity(self.blocks.len());

        loop {
            let Some((file_index, file_id, file_size)) = self
                .blocks
                .iter()
                .enumerate()
                .filter_map(|(i, &block)| match block {
                    Block::File { id, size } => {
                        if moved.contains(i) {
                            None
                        } else {
                            Some((i, id, size))
                        }
                    }
                    Block::Free { size: _ } => None,
                })
                .next_back()
            else {
                break;
            };
            // dbg!((file_index, file_id, file_size));

            if let Some(free_index) = self
                .blocks
                .iter()
                .enumerate()
                .find(|&(i, &block)| {
                    if i >= file_index {
                        return false;
                    }
                    match block {
                        Block::File { id: _, size: _ } => false,
                        Block::Free { size } => size >= file_size,
                    }
                })
                .map(|(i, _)| i)
            {
                // dbg!(free_index);
                let file_block = self.take_block(file_index);

                if *self.blocks[free_index].size() == file_size {
                    self.blocks.remove(free_index);
                } else {
                    *self.blocks[free_index].size_mut() -= file_size;
                }
                self.blocks.insert(free_index, file_block);
            }
            moved.insert(file_index);
            // dbg!(&moved);
        }
    }
}

impl Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            match block {
                Block::File { id, size } => {
                    for _ in 0..*size {
                        write!(f, "{}", id)?
                    }
                }
                Block::Free { size } => {
                    for _ in 0..*size {
                        write!(f, ".")?
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut disk = Disk::from_disk_map(input);
    disk.flatten();
    disk.defrag();
    Ok(disk.checksum())
}

pub fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut disk = Disk::from_disk_map(input);
    disk.defrag();
    Ok(disk.checksum())
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

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE_INPUT)?, 2858);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            part2(&fetch_input(AocId {
                year: 2024,
                day: 9,
                part: 2
            })?)?,
            6423258376982
        );
        Ok(())
    }
}
