use std::fs;
use day_04::process_part2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("Result part 1: {:?}", process_part2(&input));
}