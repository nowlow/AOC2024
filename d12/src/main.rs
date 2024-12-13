use std::io::Error;
use std::{env, fs};

type Map = Vec<Vec<char>>;
type Position = (usize, usize);
type Side = Vec<Position>;

#[derive(Debug)]
struct Region {
    plant_type: char,
    plants: Vec<Position>,
    area: usize,
    perimeter: usize,
    outline: Side,

    horizontal_sides: Vec<Vec<Position>>,
    vertical_sides: Vec<Vec<Position>>,
}

impl Region {
    fn has(&self, plant: &Position) -> bool {
        self.plants.contains(&plant)
    }

    fn push(&mut self, plant: &Position) {
        self.plants.push(*plant)
    }

    fn start_horizontal_side(&mut self, at: Position) {
        self.horizontal_sides.push(vec![at]);
    }

    fn start_vertical_side(&mut self, at: Position) {
        self.vertical_sides.push(vec![at]);
    }

    fn find_horizontal_side(&mut self, (x, y): Position) -> Option<&mut Vec<Position>> {
        let side = self.horizontal_sides
            .iter_mut()
            .find(|side| if x > 0 { side.contains(&(x - 1, y)) } else { false } || side.contains(&(x + 1, y)));

        // println!("[Horizontal] For {:?} found {:?}", (x, y), side);

        side
    }

    fn find_vertical_side(&mut self, (x, y): Position) -> Option<&mut Vec<Position>> {
        let side = self.vertical_sides
            .iter_mut()
            .find(|side| if y > 0 { side.contains(&(x, y - 1)) } else { false } || side.contains(&(x, y + 1)));

        // println!("[Vertical] For {   :?} found {:?}", (x, y), side);

        side
    }

    fn insert_on_horizontal_side(&mut self, (x, y): Position) {
        if let Some(side) = self.find_horizontal_side((x, y)) {
            side.push((x, y));
        } else {
            self.start_horizontal_side((x, y));
        }
    }

    fn insert_on_vertical_side(&mut self, (x, y): Position) {
        if let Some(side) = self.find_vertical_side((x, y)) {
            side.push((x, y));
        } else {
            self.start_vertical_side((x, y));
        }
    }
}

fn explore_region(map: &Map, region: &mut Region, (x, y): Position) {
    // println!("Exploring {:?}...", (x, y));

    if x > 0 && map[y][x - 1] == region.plant_type && !region.has(&(x - 1, y)) {
        region.push(&(x - 1, y));
        region.area += 1;
        explore_region(map, region, (x - 1, y));
    } else if x == 0 || map[y][x - 1] != region.plant_type {
        region.perimeter += 1;
        region.outline.push((x, y));
        region.insert_on_vertical_side((x, y));
    }

    if x < map[y].len() - 1 && map[y][x + 1] == region.plant_type && !region.has(&(x + 1, y)) {
        region.push(&(x + 1, y));
        region.area += 1;
        explore_region(map, region, (x + 1, y));
    } else if x == map[y].len() - 1 || map[y][x + 1] != region.plant_type {
        region.perimeter += 1;
        region.outline.push((x, y));
        region.insert_on_vertical_side((x, y));
    }

    if y > 0 && map[y - 1][x] == region.plant_type && !region.has(&(x, y - 1)) {
        region.push(&(x, y - 1));
        region.area += 1;
        explore_region(map, region, (x, y - 1));
    } else if y == 0 || map[y - 1][x] != region.plant_type {
        region.perimeter += 1;
        region.outline.push((x, y));
        region.insert_on_horizontal_side((x, y));
    }

    if y < map.len() - 1 && map[y + 1][x] == region.plant_type && !region.has(&(x, y + 1)) {
        region.push(&(x, y + 1));
        region.area += 1;
        explore_region(map, region, (x, y + 1));
    } else if y == map.len() - 1 || map[y + 1][x] != region.plant_type {
        region.perimeter += 1;
        region.outline.push((x, y));
        region.insert_on_horizontal_side((x, y));
    }
}

fn get_region(map: &Map, start: Position) -> Region {
    let mut region = Region {
        plant_type: map[start.1][start.0],
        plants: Vec::new(),
        area: 1,
        perimeter: 0,
        outline: Vec::new(),
        horizontal_sides: Vec::new(),
        vertical_sides: Vec::new(),
    };

    region.push(&start);

    explore_region(map, &mut region, start);

    region
}

fn build_regions(map: &Map) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];

    for y in 0..map.len() {
        let line = &map[y];
        for x in 0..line.len() {
            if regions.iter().find(|region| region.has(&(x, y))).is_none() {
                regions.push(get_region(map, (x, y)));
            }
        }
    }

    regions
}

fn get_result_2(map: &Map) -> usize {
    let regions = build_regions(map);
    let mut total = 0;

    for region in regions {
        let sides = region.vertical_sides.len() + region.horizontal_sides.len();

        println!("vertical: {:?}", region.vertical_sides);
        println!("horizontal: {:?}", region.horizontal_sides);

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if region.outline.iter().find(|&p| *p == (x, y)).is_some() {
                    print!("{}", region.plant_type);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();

        total += sides * region.area;
    }

    total
}

fn get_result_1(map: &Map) -> usize {
    build_regions(map)
        .iter()
        .fold(0, |acc, region| acc + (region.area * region.perimeter))
}

fn parse_content(content: String) -> Map {
    content.lines().map(|l| l.chars().collect()).collect()
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
