use crate::lib::input::load_input;

fn part_one(input: &Vec<(String, usize)>) -> usize {
    let mut depth = 0;
    let mut distance = 0;

    for (command, size) in input.iter() {
        match command.as_str() {
            "forward" => distance += size,
            "down" => depth += size,
            "up" => depth -= size,
            _ => unimplemented!(),
        }
    }

    depth * distance
}

fn part_two(input: &Vec<(String, usize)>) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    for (command, size) in input.iter() {
        match command.as_str() {
            "forward" => {
                distance += size;
                depth += aim * size;
            }
            "down" => aim += size,
            "up" => aim -= size,
            _ => unimplemented!(),
        }
    }

    distance * depth
}

pub fn run() {
    let input: Vec<(String, usize)> = load_input("02")
        .lines()
        .map(|l| {
            let (command, size) = l.split_once(" ").unwrap();
            (command.to_owned(), size.parse().unwrap())
        })
        .collect();

    let one = part_one(&input);
    println!("Part one: {}", one);

    let two = part_two(&input);
    println!("Part two: {}", two);
}
