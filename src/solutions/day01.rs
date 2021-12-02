use crate::lib::input::load_input;

fn parse_input() -> Vec<usize> {
    load_input("01")
        .lines()
        .map(|f| f.parse().unwrap())
        .collect()
}

fn part_one(input: &Vec<usize>) -> usize {
    let mut counter = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            counter += 1;
        }
    }
    counter
}

fn part_two(input: &Vec<usize>) -> usize {
    let mut counter = 0;
    for i in 1..input.len() - 2 {
        let a = input[i - 1] + input[i] + input[i + 1];
        let b = input[i] + input[i + 1] + input[i + 2];

        if b > a {
            counter += 1;
        }
    }
    counter
}

pub fn run() {
    let input = parse_input();

    let p1 = part_one(&input);
    println!("part 1: {}", p1);

    let p2 = part_two(&input);
    println!("part 2: {}", p2);
}

#[test]
fn part_two_test() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let result = part_two(&input);
    assert_eq!(result, 5);
}
