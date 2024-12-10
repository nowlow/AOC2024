use std::collections::HashSet;
use std::io::Error;
use std::{env, fs};

fn count_paths_from(
    (x, y): (usize, usize),
    map: &Vec<Vec<u8>>,
    explored: Vec<(usize, usize)>,
    found: &mut HashSet<(usize, usize)>,
) -> u32 {
    if explored.iter().find(|p| **p == (x, y)).is_some() {
        return 0;
    }

    let current = map[y][x];

    if current == 9 {
        found.insert((x, y));
        return 1;
    }

    let mut total = 0;
    let mut self_explored = explored.clone();

    self_explored.push((x, y));

    if x > 0 && map[y][x - 1] as i64 - current as i64 == 1 {
        total += count_paths_from((x - 1, y), map, self_explored.clone(), found);
    }

    if x < map[0].len() - 1 && map[y][x + 1] as i64 - current as i64 == 1 {
        total += count_paths_from((x + 1, y), map, self_explored.clone(), found);
    }

    if y > 0 && map[y - 1][x] as i64 - current as i64 == 1 {
        total += count_paths_from((x, y - 1), map, self_explored.clone(), found);
    }

    if y < map.len() - 1 && map[y + 1][x] as i64 - current as i64 == 1 {
        total += count_paths_from((x, y + 1), map, self_explored.clone(), found);
    }

    total
}

fn get_result_2(map: &Vec<Vec<u8>>) -> u32 {
    let mut total = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let mut found = HashSet::new();
                total += count_paths_from((x, y), map, Vec::new(), &mut found);
            }
        }
    }

    total
}

fn get_result_1(map: &Vec<Vec<u8>>) -> u32 {
    let mut total = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let mut found = HashSet::new();
                count_paths_from((x, y), map, Vec::new(), &mut found);

                total += found.len() as u32;
            }
        }
    }

    total
}

fn parse_content(content: String) -> Vec<Vec<u8>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
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
            let map = parse_content(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(&map),
                    2 => get_result_2(&map),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
