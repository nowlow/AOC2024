use std::io::Error;
use std::{env, fs};

fn remove_index<T: Clone>(arr: &[T], index: usize) -> Vec<T> {
    let mut new_vec = arr.to_vec();
    new_vec.remove(index);
    new_vec
}

fn is_line_safe(line: Vec<u32>) -> bool {
    let direction = if line[0] > line[1] { -1 } else { 1 };
    let mut safe = true;

    for i in 0..(line.len() - 1) {
        if safe {
            let abs_diff = line[i].abs_diff(line[i + 1]);

            if abs_diff < 1 || abs_diff > 3 {
                safe = false;
            }

            if (direction == -1 && line[i] < line[i + 1])
                || (direction == 1 && line[i] > line[i + 1])
            {
                safe = false;
            }
        }
    }

    safe
}

fn dampener_solver(line: Vec<u32>) -> bool {
    for index in 0..line.len() {
        if is_line_safe(remove_index(&line, index)) {
            return true;
        }
    }

    false
}

fn get_result_2(input: Vec<Vec<u32>>) -> u32 {
    let mut safes = 0;
    for line in input {
        if is_line_safe(line.clone()) {
            safes += 1;
        } else if dampener_solver(line) {
            safes += 1;
        }
    }
    return safes;
}

fn get_result_1(input: Vec<Vec<u32>>) -> u32 {
    let mut safes = 0;
    for line in input {
        if is_line_safe(line) {
            safes += 1;
        }
    }
    return safes;
}

fn parse_input(content: String) -> Vec<Vec<u32>> {
    let lines = content.lines();

    return lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
}

fn read_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args[1].parse::<u32>().unwrap();
    let input = args[2].clone();

    match read_file(input.as_str()) {
        Ok(content) => {
            let parsed = parse_input(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(parsed),
                    2 => get_result_2(parsed),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
