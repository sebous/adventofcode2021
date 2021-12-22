use std::{
    cmp::{max, min},
    collections::HashSet,
};

use parse_display::{Display, FromStr};

use crate::lib::input::load_input;

#[derive(Display, Debug, FromStr)]
#[display("{0}..{1}")]
struct AxisRange(isize, isize);

#[derive(Display, Debug, FromStr)]
#[display("{mode} x={x},y={y},z={z}")]
struct Instr {
    mode: String,
    x: AxisRange,
    y: AxisRange,
    z: AxisRange,
}

fn parse() -> Vec<Instr> {
    load_input("22")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn process(instr: &Vec<Instr>) -> usize {
    let mut memory = HashSet::new();

    for line in instr {
        for x in max(-50, line.x.0)..=min(50, line.x.1) {
            for y in max(-50, line.y.0)..=min(50, line.y.1) {
                for z in max(-50, line.z.0)..=min(50, line.z.1) {
                    match line.mode.as_str() {
                        "on" => {
                            memory.insert((x, y, z));
                        }
                        "off" => {
                            memory.remove(&(x, y, z));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    memory.len()
}

pub fn run() {
    let instr = parse();
    let p1 = process(&instr);
    println!("part 1: {}", p1);
}
