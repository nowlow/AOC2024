use regex::Regex;
use std::io::Error;
use std::{env, fs};

fn get_result_2(content: String) -> u32 {
    let regex = Regex::new(r"(mul|do|don't)\(([0-9]{1,3})?,?([0-9]{1,3})?\)").unwrap();
    let mut pairs: Vec<(u32, u32)> = vec![];
    let mut activated = true;

    for captures in regex.captures_iter(content.as_str()) {
        let command = captures.get(1).unwrap().as_str();

        match command {
            "do" => activated = true,
            "don't" => activated = false,
            "mul" => {
                if let (Some(n1), Some(n2)) = (captures.get(2), captures.get(3)) {
                    if activated {
                        pairs.push((
                            n1.as_str().parse::<u32>().unwrap(),
                            n2.as_str().parse::<u32>().unwrap(),
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    pairs.iter().fold(0, |acc, (a, b)| (acc + (a * b)))
}

fn get_result_1(content: String) -> u32 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut pairs: Vec<(u32, u32)> = vec![];

    for (_, [n1, n2]) in regex.captures_iter(content.as_str()).map(|c| c.extract()) {
        pairs.push((n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()));
    }

    pairs.iter().fold(0, |acc, (a, b)| (acc + (a * b)))
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
            println!(
                "{}",
                match part {
                    1 => get_result_1(content),
                    2 => get_result_2(content),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
