use std::io::Error;
use std::{env, fs};

use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl ClawMachine {
    fn is_reachable(&self, prize: &Point) -> Option<i64> {
        let determinant = self.button_a.x * self.button_b.y - self.button_b.x * self.button_a.y;
        if determinant == 0 {
            return None;
        }

        let n_numerator = prize.x * self.button_b.y - self.button_b.x * prize.y;
        let m_numerator = self.button_a.x * prize.y - prize.x * self.button_a.y;

        if n_numerator % determinant != 0 || m_numerator % determinant != 0 {
            return None;
        }

        let n = n_numerator / determinant;
        let m = m_numerator / determinant;

        if n >= 0 && m >= 0 {
            Some(3 * n + m)
        } else {
            None
        }
    }
}

fn get_result_2(machines: &Vec<ClawMachine>) -> i64 {
    let mut total = 0;
    let offset = 10000000000000;

    for machine in machines[..].iter() {
        if let Some(credits) = machine.is_reachable(&Point {
            x: machine.prize.x + offset,
            y: machine.prize.y + offset,
        }) {
            total += credits;
        }
    }

    total
}

fn get_result_1(machines: &Vec<ClawMachine>) -> i64 {
    let mut total = 0;

    for machine in machines[..].iter() {
        if let Some(credits) = machine.is_reachable(&machine.prize) {
            total += credits;
        }
    }

    total
}

fn parse_content(content: String) -> Vec<ClawMachine> {
    let mut machines: Vec<ClawMachine> = Vec::new();
    let raw_machines = content.split("\n\n");

    for raw_machine in raw_machines {
        let lines = raw_machine.lines().collect::<Vec<&str>>();

        let line_a = lines[0];
        let line_b = lines[1];
        let line_prize = lines[2];

        let regex = Regex::new(r"X(\+|-|=)([0-9]+), Y(\+|-|=)([0-9]+)").unwrap();

        let captures_a = regex.captures(line_a).unwrap();
        let captures_b = regex.captures(line_b).unwrap();
        let captures_prize = regex.captures(line_prize).unwrap();

        machines.push(ClawMachine {
            button_a: Point {
                x: captures_a.get(2).unwrap().as_str().parse().unwrap(),
                y: captures_a.get(4).unwrap().as_str().parse().unwrap(),
            },
            button_b: Point {
                x: captures_b.get(2).unwrap().as_str().parse().unwrap(),
                y: captures_b.get(4).unwrap().as_str().parse().unwrap(),
            },
            prize: Point {
                x: captures_prize.get(2).unwrap().as_str().parse().unwrap(),
                y: captures_prize.get(4).unwrap().as_str().parse().unwrap(),
            },
        })
    }

    machines
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
            let machines = parse_content(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(&machines),
                    2 => get_result_2(&machines),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
