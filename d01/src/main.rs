use std::convert::TryFrom;
use std::io::Error;
use std::{env, fs};

fn get_result_1(input: (Vec<u32>, Vec<u32>)) -> u32 {
    let mut sum = 0;

    let mut col_1 = input.0.clone();
    let mut col_2 = input.1.clone();

    col_1.sort();
    col_2.sort();

    for i in 0..col_1.len() {
        let a = col_1[i];
        let b = col_2[i];

        sum += a.abs_diff(b);
    }

    sum
}

fn get_result_2((col_1, col_2): (Vec<u32>, Vec<u32>)) -> u32 {
    let mut sum = 0;

    for i in 0..col_1.len() {
        let a = col_1[i];
        let times = u32::try_from(col_2.iter().filter(|&n| *n == a).count()).unwrap();

        sum += a * times;
    }

    sum
}

fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines();
    let mut col_1: Vec<u32> = vec![];
    let mut col_2: Vec<u32> = vec![];

    for line in lines {
        let splited_line = line.split("   ").collect::<Vec<&str>>();
        col_1.push(splited_line[0].parse::<u32>().unwrap());
        col_2.push(splited_line[1].parse::<u32>().unwrap());
    }

    return (col_1, col_2);
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
