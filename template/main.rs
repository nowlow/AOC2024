use std::io::Error;
use std::{env, fs};

fn get_result_2(content: String) -> u32 {
    todo!()
}

fn get_result_1(content: String) -> u32 {
    todo!()
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
