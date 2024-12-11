use std::time::Instant;

const INPUT_FILE: &str = "input9.txt";

#[derive(Clone, PartialEq, Eq)]
enum BlockType {
    FILE,
    EMPTY,
}

#[derive(Clone)]
struct Block {
    block_type: BlockType,
    id: usize
}

impl Block {
    fn new(block_type: BlockType, id: usize) -> Self {
        Self {
            block_type,
            id,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum FileType {
    FILE,
    EMPTY,
}

#[derive(Clone)]
struct File {
    file_type: FileType,
    size: u64,
    id: usize
}

impl File {
    fn new(file_type: FileType, size: u64, id: usize) -> Self {
        Self {
            file_type,
            size,
            id,
        }
    }
}

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: String = aoc_utils::input::load_input_line(INPUT_FILE);
    let parsed_input: Vec<u64> = raw_input.chars().map(|s| s.to_string().parse::<u64>().unwrap()).collect();
    let mut disk_map: Vec<Block> = Vec::new();
    for (id, file_length) in parsed_input.iter().enumerate() {
        if id % 2 == 0 {
            for repetition in 0..*file_length {
                disk_map.push(Block::new(BlockType::FILE, id/2))
            }
        } else {
            for repetition in 0..*file_length {
                disk_map.push(Block::new(BlockType::EMPTY, id/2));
            }
        }
    }
    let mut file_map: Vec<File> = Vec::new();
    for (id, file_length) in parsed_input.iter().enumerate() {
        if id % 2 == 0 {
            file_map.push(File::new(FileType::FILE, *file_length, id/2))
        } else {
            file_map.push(File::new(FileType::EMPTY, *file_length, 0));
        }
    }

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    solve_part_one(&mut disk_map.clone());
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    solve_part_two(&mut file_map.clone());
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(disk_map: &mut Vec<Block>) {
    let mut solution = 0;
    for index in (0..disk_map.len()).rev() {
        let current_block = disk_map.get(index).unwrap();
        match current_block.block_type {
            BlockType::FILE => {
                for j in 0..index {
                    let test_block = disk_map.get(j).unwrap();
                    match test_block.block_type {
                        BlockType::EMPTY => {
                            disk_map.swap(index, j);
                            break;
                        },
                        _ => ()
                    }
                }
            },
            BlockType::EMPTY => continue,
        }
    }
    for (index, block) in disk_map.iter().enumerate() {
        match block.block_type {
            BlockType::FILE => solution += block.id * index,
            _ => ()
        }
    }
    println!("Day 9 Part 1: {}", solution);
}

fn solve_part_two(file_map: &mut Vec<File>) {
    let mut solution = 0;
    for index in (0..file_map.len()).rev() {
        let current_file = file_map.get(index).unwrap().clone();
        match current_file.file_type {
            FileType::FILE => {
                for j in 0..index {
                    let test_file = file_map.get(j).unwrap().clone();
                    match test_file.file_type {
                        FileType::EMPTY => {
                            if test_file.size == current_file.size {
                                file_map.swap(index, j);
                                break;
                            } else if test_file.size > current_file.size {
                                file_map.get_mut(j).unwrap().size = current_file.size;
                                file_map.swap(index, j);
                                file_map.insert(j + 1, File::new(FileType::EMPTY, test_file.size - current_file.size, test_file.id));
                                break;
                            }
                        },
                        _ => ()
                    }
                }
            },
            FileType::EMPTY => continue,
        }
    }
    let mut start = 0;
    for (_, file) in file_map.iter().enumerate() {
        match file.file_type {
            FileType::FILE => {
                for offset in 0..file.size {
                    solution += file.id * (start + offset as usize);
                }
                start += file.size as usize;
            },
            FileType::EMPTY => start += file.size as usize,
        }
    }
    println!("Day 9 Part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

}