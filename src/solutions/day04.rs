use regex::Regex;

use crate::lib::input::load_input;

type Board = Vec<Vec<usize>>;

fn parse_input() -> (Vec<usize>, Vec<Board>) {
    let input = load_input("04");
    let mut lines = input.lines();

    let re = Regex::new(r"\d+").unwrap();

    let game_numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];

    let mut board_index = 0;

    for (i, line) in lines.enumerate() {
        if line == "" {
            boards.push(Vec::new());
            if i != 0 {
                board_index += 1;
            }
            continue;
        }

        let numbers_matches: Vec<usize> = re
            .captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        boards[board_index].push(numbers_matches);
    }

    (game_numbers, boards)
}

fn check_rows(board: &Board, played_numbers: &Vec<usize>) -> bool {
    for row in board {
        if row.iter().all(|n| played_numbers.contains(n)) {
            return true;
        }
    }
    false
}

fn check_columns(board: &Board, played_numbers: &Vec<usize>) -> bool {
    for i in 0..board.len() {
        if board
            .iter()
            .map(|row| row.iter().nth(i).unwrap())
            .all(|n| played_numbers.contains(n))
        {
            return true;
        }
    }
    false
}

fn get_result(i: usize, boards: &Vec<Board>, played_numbers: &Vec<usize>) -> usize {
    let unmarked_numbers_sum = boards
        .iter()
        .nth(i)
        .unwrap()
        .iter()
        .flatten()
        .filter(|num| !played_numbers.contains(*num))
        .sum::<usize>();

    unmarked_numbers_sum * played_numbers.last().unwrap()
}

fn part_one(game_numbers: &Vec<usize>, boards: &Vec<Board>) {
    let mut played_numbers = vec![];
    let mut winner_board_i: usize = 0;

    'outer: for num in game_numbers.iter() {
        played_numbers.push(num.to_owned());

        for (i, board) in boards.into_iter().enumerate() {
            let row_win = check_rows(board, &played_numbers);
            let col_win = check_columns(board, &played_numbers);

            if row_win || col_win {
                winner_board_i = i;
                break 'outer;
            }
        }
    }

    let result = get_result(winner_board_i, boards, &played_numbers);
    println!("part 1: {}", result);
}

fn part_two(game_numbers: &Vec<usize>, boards: &Vec<Board>) {
    let mut played_numbers = vec![];
    let mut winner_boards = vec![];
    let mut last_won: usize = 0;

    'outer: for num in game_numbers.iter() {
        played_numbers.push(num.to_owned());

        for (i, board) in boards.into_iter().enumerate() {
            if winner_boards.contains(&i) {
                continue;
            }

            let row_win = check_rows(board, &played_numbers);
            let col_win = check_columns(board, &played_numbers);

            if row_win || col_win {
                winner_boards.push(i);
                last_won = i;
            }

            if winner_boards.len() == boards.len() {
                break 'outer;
            }
        }
    }

    let result = get_result(last_won, boards, &played_numbers);
    println!("part 2: {}", result);
}

#[allow(dead_code)]
pub fn run() {
    let (game_numbers, boards) = parse_input();
    part_one(&game_numbers, &boards);
    part_two(&game_numbers, &boards);
}
