use day_03::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Result part 1: {:?}", process_part1(&file));
}
