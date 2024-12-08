use std::collections::{HashMap, HashSet};
use std::io::Error;
use std::{env, fs};

type Point = (i32, i32);
type Antennas = HashMap<char, Vec<Point>>;

#[derive(Debug)]
struct Map {
    antennas: Antennas,
    width: i32,
    height: i32,
}

fn multiply_vector(p1: &Point, p2: &Point, multiplier: i32, map: &Map) -> Option<Point> {
    let end = (
        ((1 - multiplier) * p1.0) + (multiplier * p2.0),
        ((1 - multiplier) * p1.1) + (multiplier * p2.1),
    );

    if (0..map.width).contains(&end.0) && (0..map.height).contains(&end.1) {
        return Some(end);
    }
    None
}

#[allow(unused)]
fn add_to_vector(p1: &Point, p2: &Point, n: f32, map: &Map) -> Option<Point> {
    let length = (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f32).sqrt();

    let x1 = p1.0 as f32;
    let y1 = p1.1 as f32;
    let x2 = p2.0 as f32;
    let y2 = p2.1 as f32;

    let end: Point = (
        (x1 - n * (x2 - x1) / length).round() as i32,
        (y1 - n * (y2 - x1) / length).round() as i32,
    );

    if (0..map.width).contains(&end.0) && (0..map.height).contains(&end.1) {
        return Some(end);
    }
    None
}

fn get_result_2(map: &Map) -> usize {
    let mut set: HashSet<Point> = HashSet::new();

    for key in map.antennas.keys() {
        let entries = map.antennas.get(key).unwrap();

        for node in entries {
            for other_node in entries {
                if other_node == node {
                    continue;
                }

                let mut current_multiplier = 0;

                while let Some(anti_node) =
                    multiply_vector(node, other_node, current_multiplier, map)
                {
                    current_multiplier += 1;
                    set.insert(anti_node);
                }
            }
        }
    }

    set.iter().len()
}

fn get_result_1(map: &Map) -> usize {
    let mut set: HashSet<Point> = HashSet::new();

    for key in map.antennas.keys() {
        let entries = map.antennas.get(key).unwrap();

        for node in entries {
            for other_node in entries {
                if other_node == node {
                    continue;
                }

                if let Some(anti_node) = multiply_vector(node, other_node, 2, map) {
                    set.insert(anti_node);
                }
            }
        }
    }

    set.iter().len()
}

fn parse_content(content: String) -> Map {
    let lines = content.lines().collect::<Vec<&str>>();
    let mut map = Map {
        antennas: HashMap::new(),
        width: TryFrom::try_from(lines[0].len()).unwrap(),
        height: TryFrom::try_from(lines.len()).unwrap(),
    };

    for y in 0..lines.len() {
        let line = lines[y];

        for x in 0..line.len() {
            let c = line.chars().to_owned().nth(x).unwrap();

            if c != '.' {
                let point: Point = (TryFrom::try_from(x).unwrap(), TryFrom::try_from(y).unwrap());
                if let Some(result) = map.antennas.get_mut(&c) {
                    result.push(point);
                } else {
                    map.antennas.insert(c, vec![point]);
                }
            }
        }
    }

    map
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
