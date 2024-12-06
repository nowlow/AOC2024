use std::io::Error;
use std::{env, fs};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Clone)]
enum Next {
    Obstacle,
    Path,
    Out,
}

#[derive(Debug, Clone)]
struct Map {
    x: usize,
    y: usize,
    direction: Direction,
    lines: Vec<String>,
    width: usize,
    height: usize,
}

fn what_next(map: &Map) -> Next {
    match map.direction {
        Direction::Up => {
            if map.y == 0 {
                return Next::Out;
            } else if map.lines[map.y - 1].chars().nth(map.x).unwrap() != '.' {
                return Next::Obstacle;
            }

            Next::Path
        }
        Direction::Right => {
            if map.x == map.width - 1 {
                return Next::Out;
            } else if map.lines[map.y].chars().nth(map.x + 1).unwrap() != '.' {
                return Next::Obstacle;
            }

            Next::Path
        }
        Direction::Down => {
            if map.y == map.height - 1 {
                return Next::Out;
            } else if map.lines[map.y + 1].chars().nth(map.x).unwrap() != '.' {
                return Next::Obstacle;
            }
            Next::Path
        }
        Direction::Left => {
            if map.x == 0 {
                return Next::Out;
            } else if map.lines[map.y].chars().nth(map.x - 1).unwrap() != '.' {
                return Next::Obstacle;
            }

            Next::Path
        }
    }
}

fn move_guard(map: &mut Map) -> Next {
    let next = what_next(map);

    if next != Next::Out {
        match map.direction {
            Direction::Up => {
                if next == Next::Obstacle {
                    map.direction = Direction::Right;
                } else {
                    map.y -= 1;
                }
            }
            Direction::Right => {
                if next == Next::Obstacle {
                    map.direction = Direction::Down;
                } else {
                    map.x += 1;
                }
            }
            Direction::Down => {
                if next == Next::Obstacle {
                    map.direction = Direction::Left;
                } else {
                    map.y += 1;
                }
            }
            Direction::Left => {
                if next == Next::Obstacle {
                    map.direction = Direction::Up;
                } else {
                    map.x -= 1;
                }
            }
        }
    }

    next
}

fn test_loop(map: &mut Map) -> bool {
    let mut visited: Vec<(usize, usize, Direction)> = vec![(map.x, map.y, map.direction.clone())];

    while move_guard(map) != Next::Out {
        if visited
            .iter()
            .find(|p| **p == (map.x, map.y, map.direction.clone()))
            .is_some()
        {
            return true;
        } else {
            visited.push((map.x, map.y, map.direction.clone()));
        }
    }

    // println!("test not conclued with {pos:?}");
    false
}

fn get_guard_visits(map: &mut Map) -> Vec<(usize, usize)> {
    let mut visited: Vec<(usize, usize)> = vec![(map.x, map.y)];

    while move_guard(map) != Next::Out {
        if visited.iter().find(|p| **p == (map.x, map.y)).is_none() {
            visited.push((map.x, map.y));
        }
    }

    visited
}

fn get_result_2(map: &Map) -> usize {
    let mut total = 0;

    let visited = get_guard_visits(&mut map.clone());

    for y in 0..map.height {
        for x in 0..map.width {
            if visited.iter().find(|p| **p == (x, y)).is_none() {
                continue;
            }
            if x == map.x && y == map.y || map.lines[y].chars().nth(x).unwrap() != '.' {
                continue;
            }
            let mut new_map = map.clone();
            new_map.lines[y].replace_range(x..x + 1, "O");

            if test_loop(&mut new_map) {
                total += 1;
            }
        }
    }

    total
}

fn get_result_1(map: &mut Map) -> usize {
    get_guard_visits(map).len()
}

fn parse_content(content: String) -> Map {
    let (x, y, _) = content.chars().fold((0, 0, 0), |mut pos, c| {
        if pos.2 == 0 {
            if c == '\n' {
                pos.0 = 0;
                pos.1 += 1;
            } else if c != '^' {
                pos.0 += 1;
            } else {
                pos.2 = 1;
            }
        }
        pos
    });

    let mut lines = content
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    lines[y] = lines[y].replace("^", ".");

    Map {
        x,
        y,
        direction: Direction::Up,
        lines: lines.clone(),
        width: lines[0].len(),
        height: lines.len(),
    }
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
            let mut map = parse_content(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(&mut map),
                    2 => get_result_2(&mut map),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
