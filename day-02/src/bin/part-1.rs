use day_02::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let result = process_part1(&file);
    println!("The result for part 1 is: {}", result)

}