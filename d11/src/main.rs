use std::collections::HashMap;
use std::io::Error;
use std::{env, fs};

type Line = Vec<u64>;

fn split_num(number: u64) -> Option<(u64, u64)> {
    let digit_count = number.checked_ilog10().unwrap_or(0) + 1;

    if digit_count % 2 != 0 {
        return None;
    }

    let divisor = 10_u64.pow((digit_count / 2) as u32);
    let first_half = number / divisor;
    let second_half = number % divisor;

    Some((first_half, second_half))
}

fn recursive_solve(n: u64, times: u64, map: &mut HashMap<(u64, u64), usize>) -> usize {
    if times == 0 {
        return 1;
    }

    if let Some(r) = map.get(&(n, times)) {
        return *r;
    }

    let split = split_num(n);

    match n {
        0 => {
            let result = recursive_solve(1, times - 1, map);
            map.insert((n, times), result);
            result
        }
        _ if split.is_some() => {
            let (n1, n2) = split.unwrap();
            let result = recursive_solve(n1, times - 1, map) + recursive_solve(n2, times - 1, map);
            map.insert((n, times), result);
            result
        }
        _ => {
            let result = recursive_solve(n * 2024, times - 1, map);
            map.insert((n, times), result);
            result
        }
    }
}

fn get_result_2(line: &mut Line) -> usize {
    let mut total = 0;
    let mut map = HashMap::new();

    for n in line {
        total += recursive_solve(*n, 75, &mut map);
    }

    total as usize
}

fn get_result_1(line: &mut Line) -> usize {
    let mut total = 0;
    let mut map = HashMap::new();

    for n in line {
        total += recursive_solve(*n, 25, &mut map);
    }

    total as usize
}

fn parse_content(content: String) -> Line {
    content
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
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
            let mut line = parse_content(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(&mut line),
                    2 => get_result_2(&mut line),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
