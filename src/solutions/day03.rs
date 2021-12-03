use crate::lib::input::load_input;

fn get_oposite_bits(char_vec: &Vec<char>) -> Vec<char> {
    char_vec
        .iter()
        .map(|ch| if *ch == '0' { '1' } else { '0' })
        .collect()
}

fn get_nth_column_bit(input: &Vec<Vec<char>>, n: usize) -> char {
    let mut sum = 0;
    for y in 0..input.len() {
        sum += input[y][n].to_digit(10).unwrap() as usize;
    }

    if sum as f32 >= (input.len() as f32) / 2.0 {
        return '1';
    } else {
        return '0';
    }
}

fn filter_by_criteria(input: &Vec<Vec<char>>, criteria: bool) -> usize {
    let first_line = input.iter().next().unwrap();

    let mut result = input.clone();

    for x in 0..first_line.len() {
        if result.len() == 1 {
            break;
        }

        let bit = get_nth_column_bit(&result, x);
        result = result
            .into_iter()
            .filter(|line| {
                if criteria {
                    return line[x] == bit;
                } else {
                    return line[x] != bit;
                }
            })
            .collect();
    }

    let result_string: String = result.iter().next().unwrap().to_owned().iter().collect();
    usize::from_str_radix(&result_string, 2).unwrap()
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    let first_line = input.iter().next().unwrap();
    let mut most_common_bits: Vec<char> = vec![];

    for x in 0..first_line.len() {
        let bit = get_nth_column_bit(&input, x);
        most_common_bits.push(bit);
    }

    let least_common: String = get_oposite_bits(&most_common_bits).iter().collect();
    let least_common = usize::from_str_radix(&least_common, 2).unwrap();

    let most_common_bits: String = most_common_bits.iter().collect();
    let most_common_bits = usize::from_str_radix(&most_common_bits, 2).unwrap();

    most_common_bits * least_common
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    let oxygen = filter_by_criteria(input, true);
    let co2 = filter_by_criteria(input, false);
    co2 * oxygen
}

#[allow(dead_code)]
pub fn run() {
    let input: Vec<Vec<char>> = load_input("03")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let p1 = part_one(&input);
    println!("part 1: {}", p1);

    let p2 = part_two(&input);
    println!("part 2: {}", p2);
}
