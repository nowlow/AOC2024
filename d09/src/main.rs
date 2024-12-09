use std::io::Error;
use std::{env, fs};

fn block_starts_at(i: usize, diskmap: &Vec<u64>) -> u64 {
    let mut total = 0;
    for j in 0..i {
        total += diskmap[j];
    }
    total
}

fn get_result_2(diskmap: &Vec<u64>) -> u64 {
    let mut total = 0;
    let mut free_blocks: Vec<(usize, u64)> = vec![];

    for i in (1..diskmap.len()).step_by(2) {
        free_blocks.push((i, diskmap[i]));
    }

    for i in (0..diskmap.len()).rev().step_by(2) {
        let file_id = i / 2;
        let file_size = diskmap[i];
        let mut has_been_included = false;

        for (block_index, block_size) in free_blocks.iter_mut() {
            if *block_index > i {
                break;
            }

            if *block_size >= file_size {
                for j in 0..file_size {
                    let new_index = block_starts_at(*block_index, diskmap)
                        + (diskmap[*block_index] - *block_size)
                        + j;
                    total += new_index * file_id as u64;
                }
                *block_size -= file_size;
                has_been_included = true;
                break;
            }
        }

        if !has_been_included {
            for j in 0..file_size {
                let new_index = block_starts_at(i, diskmap) + j;
                total += new_index * file_id as u64;
            }
        }
    }

    total
}

fn expand_diskmap(diskmap: &Vec<u64>) -> Vec<Option<u64>> {
    let mut result: Vec<Option<u64>> = vec![];

    for i in 0..diskmap.len() {
        let n = diskmap[i];

        for _ in 0..n {
            if i % 2 == 0 {
                result.push(Some((i / 2) as u64));
            } else {
                result.push(None);
            }
        }
    }

    result
}

#[allow(dead_code)]
fn print_diskmap(indexes: &Vec<Option<u64>>) {
    for index in indexes {
        print!(
            "{}",
            match index {
                Some(index) => index.to_string(),
                None => ".".to_string(),
            }
        )
    }
    println!();
}

fn create_checksum(diskmap: &Vec<u64>) -> u64 {
    let mut expanded = expand_diskmap(diskmap);
    let mut total = 0;

    for i in 0..expanded.len() {
        if expanded[i..]
            .iter()
            .find(|p| match p {
                Some(_) => true,
                None => false,
            })
            .is_none()
        {
            break;
        }

        if let Some(file) = expanded[i] {
            total += (i as u64) * file;
        } else {
            let last_file = expanded
                .iter()
                .enumerate()
                .rev()
                .find(|(_, x)| x.is_some())
                .map(|(i, _)| i)
                .unwrap();

            if let Some(file) = expanded[last_file] {
                total += (i as u64) * file;
            } else {
                unreachable!();
            }
            expanded.swap(i, last_file);
        }
    }

    total
}

fn get_result_1(diskmap: &Vec<u64>) -> u64 {
    create_checksum(diskmap)
}

fn parse_content(content: String) -> Vec<u64> {
    let split = content.split("").collect::<Vec<&str>>();
    let mut result = vec![];

    for c in split {
        if let Ok(n) = c.parse::<u64>() {
            result.push(n);
        }
    }

    result
}

fn read_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args[1].parse::<u64>().unwrap();
    let input = args[2].clone();

    match read_file(input.as_str()) {
        Ok(content) => {
            let diskmap = parse_content(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(&diskmap),
                    2 => get_result_2(&diskmap),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
