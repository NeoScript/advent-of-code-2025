use std::{fs, path::Path};

pub mod models;

use models::{Dial, RotationInstruction};

fn main() {
    println!("Hello, world!");

    let result_one = part_one();
    println!("result from part one: {}", result_one);

    let result_two = part_two();
    println!("result from part two: {}", result_two);
}

fn input_one(path: &Path) -> Vec<RotationInstruction> {
    let file = fs::read_to_string(path).expect("should be able to read file");

    let lines = file.split("\n");
    let instructions: Vec<RotationInstruction> =
        lines.filter_map(|row| row.try_into().ok()).collect();
    instructions
}

fn part_one() -> u64 {
    let path = Path::new("input1.txt");
    let mut dial = Dial::default();
    let instructions = input_one(path);

    let mut zero_counts = 0;
    for i in instructions {
        // println!("{:?} -> apply({:?})", dial, i);
        dial.turn(&i);
        if dial.at_zero() {
            zero_counts += 1
        }
    }
    zero_counts
}

fn part_two() -> u64 {
    let path = Path::new("input1.txt");
    let mut dial = Dial::default();
    let instructions = input_one(path);

    let mut zero_counts = 0;
    for i in instructions {
        // println!("{:?} -> apply({:?})", dial, i);
        let counted = dial.turn_and_count_zeros(&i);
        zero_counts += counted;
    }
    zero_counts
}
