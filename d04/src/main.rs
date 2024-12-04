use std::io::Error;
use std::{env, fs};

fn get_result_2(content: String) -> u32 {
    let mut count = 0;
    let lines = content.lines().collect::<Vec<&str>>();

    for y in 0..lines.len() {
        let line = lines.get(y).unwrap();

        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();

            if c == 'A' {
                if x > 0 && y > 0 && x < line.len() - 1 && y < lines.len() - 1 {
                    let top_line = lines.get(y - 1).unwrap();
                    let bottom_line = lines.get(y + 1).unwrap();

                    let top_left = top_line.chars().nth(x - 1).unwrap();
                    let top_right = top_line.chars().nth(x + 1).unwrap();
                    let bottom_left = bottom_line.chars().nth(x - 1).unwrap();
                    let bottom_right = bottom_line.chars().nth(x + 1).unwrap();

                    let diagonal_fw_valid = (top_left == 'M' && bottom_right == 'S')
                        || (top_left == 'S' && bottom_right == 'M');
                    let diagonal_bw_valid = (top_right == 'M' && bottom_left == 'S')
                        || (top_right == 'S' && bottom_left == 'M');

                    if diagonal_fw_valid && diagonal_bw_valid {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn get_result_1(content: String) -> u32 {
    let mut count = 0;
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let lines = content.lines().collect::<Vec<&str>>();

    for y in 0..lines.len() {
        let line = lines.get(y).unwrap();

        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();

            if c == 'X' {
                for (dir_x, dir_y) in directions.clone() {
                    let mut new_x = x;
                    let mut new_y = y;
                    let mut xmas = "XMAS".to_string();

                    while line.chars().nth(new_x).is_some() && lines.get(new_y).is_some() {
                        let new_line = lines.get(new_y).unwrap();
                        let new_char = new_line.chars().nth(new_x).unwrap();

                        if new_char == xmas.chars().nth(0).unwrap() {
                            xmas.remove(0);
                        } else {
                            break;
                        }

                        if xmas.len() == 0 {
                            count += 1;
                            // print!("({x}, {y}) to ({new_x}, {new_y})");
                            break;
                        }

                        if dir_x != -1 || new_x > 0 {
                            match dir_x {
                                1 => new_x += 1,
                                -1 => new_x -= 1,
                                _ => {}
                            }
                        } else if dir_x == -1 && new_x == 0 {
                            break;
                        }

                        if dir_y != -1 || new_y > 0 {
                            match dir_y {
                                1 => new_y += 1,
                                -1 => new_y -= 1,
                                _ => {}
                            }
                        } else if dir_y == -1 && new_y == 0 {
                            break;
                        }
                    }
                }
            }
        }
    }

    count
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
