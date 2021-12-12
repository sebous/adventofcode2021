use itertools::Itertools;

use crate::lib::input::load_input;

fn parse_input() -> Vec<Vec<String>> {
    load_input("12")
        .lines()
        .map(|l| l.split("-").map(|s| s.to_owned()).collect_vec())
        .collect_vec()
}

fn find_paths_recursively(
    start_at: &str,
    path_buffer: &Vec<String>,
    subpaths: &Vec<Vec<String>>,
    small_visited_limit: usize,
    small_visited: Option<&str>,
) -> Option<Vec<Vec<String>>> {
    let possible_next_locs = subpaths
        .iter()
        .filter(|p| p.contains(&start_at.to_string()))
        .filter_map(|p| p.iter().find(|x| *x != start_at));

    let mut path_buffer = path_buffer.clone();
    path_buffer.push(start_at.to_owned());

    let mut possibilities: Vec<Vec<String>> = vec![];

    if start_at == "end" {
        return Some(vec![path_buffer]);
    }

    for next_loc in possible_next_locs {
        if next_loc == "start" {
            continue;
        }
        let next_loc_count = path_buffer.iter().filter(|p| p == &next_loc).count();

        match next_loc.chars().all(|ch| ch.is_lowercase()) {
            // is lowercase
            true => match next_loc_count > 0 {
                // loc was already visited
                true => match small_visited.is_none() && next_loc_count < small_visited_limit {
                    true => {
                        find_paths_recursively(
                            next_loc,
                            &path_buffer,
                            subpaths,
                            small_visited_limit,
                            Some(next_loc),
                        )
                        .map(|x| possibilities.extend(x));
                    }
                    false => (),
                },
                // not visited
                false => {
                    find_paths_recursively(
                        next_loc,
                        &path_buffer,
                        subpaths,
                        small_visited_limit,
                        small_visited,
                    )
                    .map(|x| possibilities.extend(x));
                }
            },
            // uppercase
            false => {
                find_paths_recursively(
                    next_loc,
                    &path_buffer,
                    subpaths,
                    small_visited_limit,
                    small_visited,
                )
                .map(|x| possibilities.extend(x));
            }
        }
    }

    Some(possibilities)
}

pub fn run() {
    let input = parse_input();
    let paths_1 = find_paths_recursively("start", &vec![], &input, 1, None).unwrap();
    println!("part 1: {:?}", paths_1.len());

    let paths_2 = find_paths_recursively("start", &vec![], &input, 2, None).unwrap();
    println!("part 2: {:?}", paths_2.len());
}
