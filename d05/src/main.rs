use std::io::Error;
use std::{env, fs};

type Order = (u32, u32);
type Update = Vec<u32>;

#[derive(Debug)]
struct Input {
    orders: Vec<Order>,
    updates: Vec<Update>,
}

fn get_unsorted_idx(update: &Update, orders: &Vec<Order>) -> Option<(usize, usize)> {
    for i in 0..(update.len() - 1) {
        let page_number = update[i];
        let rest = &update[i + 1..];

        for other_page_number in rest {
            let pos = orders
                .iter()
                .position(|pair| *pair == (*other_page_number, page_number));

            if pos.is_some() {
                let position = update.iter().position(|n| n == other_page_number).unwrap();

                return Some((i, position));
            }
        }
    }
    None
}

fn is_update_sorted(update: &Update, orders: &Vec<Order>) -> bool {
    get_unsorted_idx(update, orders).is_none()
}

fn get_sorted_update(update: &Update, orders: &Vec<Order>) -> Update {
    let mut new_update = update.clone();

    while let Some((wrong, swap)) = get_unsorted_idx(&new_update, orders) {
        new_update.swap(wrong, swap);
    }

    new_update
}

fn get_result_2(input: Input) -> u32 {
    let mut sum: u32 = 0;

    for update in input.updates {
        if !is_update_sorted(&update, &input.orders) {
            sum += get_sorted_update(&update, &input.orders)[update.len() / 2];
        }
    }
    sum
}

fn get_result_1(input: Input) -> u32 {
    let mut sum: u32 = 0;

    for update in input.updates {
        if is_update_sorted(&update, &input.orders) {
            // println!("update {update:?} is sorted");
            sum += update[update.len() / 2];
        }
    }
    sum
}

fn parse_content(content: String) -> Input {
    let lines = content.lines();
    let mut orders: Vec<Order> = vec![];
    let mut updates: Vec<Update> = vec![];
    let mut is_in_orders_section = true;

    for line in lines {
        if line.len() == 0 {
            is_in_orders_section = false;
        } else {
            if is_in_orders_section {
                let splits = line.split("|").collect::<Vec<&str>>();
                orders.push((
                    splits[0].parse::<u32>().unwrap(),
                    splits[1].parse::<u32>().unwrap(),
                ));
            } else {
                let splits = line.split(",");
                updates.push(splits.map(|n| n.parse::<u32>().unwrap()).collect())
            }
        }
    }

    Input { orders, updates }
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
            let input = parse_content(content);

            println!(
                "{}",
                match part {
                    1 => get_result_1(input),
                    2 => get_result_2(input),
                    _ => 0,
                }
            )
        }
        Err(error) => println!("{:?}", error),
    };
}
