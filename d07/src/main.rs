use std::io::Error;
use std::{env, fs};

type Operation = (i64, Vec<i64>);

fn evaluate_expression(numbers: &[i64], ops: &[char]) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in ops.iter().enumerate() {
        let next = numbers[i + 1];
        result = match op {
            '+' => result + next,
            '-' => result - next,
            '*' => result * next,
            '|' => {
                (result * (10_i64.pow(TryInto::try_into(next.to_string().len()).unwrap()))) + next
            }
            '/' => {
                if next == 0 || result % next != 0 {
                    return i64::MIN; // Invalid operation
                }
                result / next
            }
            _ => unreachable!(),
        };
    }
    result
}

fn can_make_target(numbers: &[i64], target: i64, operators: &Vec<char>) -> Option<Vec<char>> {
    fn dp(
        pos: usize,
        ops: &mut Vec<char>,
        numbers: &[i64],
        target: i64,
        operators: &Vec<char>,
    ) -> bool {
        if pos == numbers.len() - 1 {
            return evaluate_expression(numbers, ops) == target;
        }

        for &op in operators {
            ops.push(op);
            if dp(pos + 1, ops, numbers, target, operators) {
                return true;
            }
            ops.pop();
        }
        false
    }

    let mut ops = Vec::new();
    if dp(0, &mut ops, numbers, target, operators) {
        Some(ops)
    } else {
        None
    }
}

fn get_result_2(operations: &Vec<Operation>) -> i64 {
    let mut total = 0;

    for operation in operations {
        if let Some(_op) = can_make_target(&operation.1, operation.0, &vec!['*', '+', '|']) {
            total += operation.0;
        }
    }
    total
}

fn get_result_1(operations: &Vec<Operation>) -> i64 {
    let mut total = 0;

    for operation in operations {
        if let Some(_op) = can_make_target(&operation.1, operation.0, &vec!['*', '+']) {
            // for _ in 0..operation.1.len() - 1 {
            //     print!("(");
            // }

            // for i in 0..operation.1.len() {
            //     if i < operation.1.len() - 1 {
            //         print!("{}) {} ", operation.1[i], op[i]);
            //     } else {
            //         println!("{} == {}", operation.1[i], operation.0);
            //     }
            // }
            total += operation.0;
        }
    }
    total
}

fn parse_content(content: String) -> Vec<Operation> {
    content
        .lines()
        .map(|line| {
            let splits = line.split(":").collect::<Vec<&str>>();

            (
                splits[0].parse::<i64>().unwrap(),
                splits[1]
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect()
}

fn read_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let part = args[1].parse::<i64>().unwrap();
    let input = args[2].clone();

    match read_file(input.as_str()) {
        Ok(content) => {
            let operations = parse_content(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(&operations),
                    2 => get_result_2(&operations),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
