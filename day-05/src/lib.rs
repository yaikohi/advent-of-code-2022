use std::collections::VecDeque;
use std::iter::FromIterator;

pub fn process_part1(input: &str) -> String {
    let mut result: u16 = 0;
    // let parsed_input = input.lines().collect::<Vec<&str>>();
    let stack_1: VecDeque<&str> = VecDeque::from_iter(["G", "D", "V", "Z", "J", "S", "B"]);
    let stack_2: VecDeque<&str> = VecDeque::from_iter(["Z", "S", "M", "G", "V", "P"]);
    let stack_3: VecDeque<&str> = VecDeque::from_iter(["C", "L", "B", "S", "W", "T", "Q", "F"]);
    let stack_4: VecDeque<&str> = VecDeque::from_iter(["H", "J", "G", "W", "M", "R", "V", "Q"]);
    let stack_5: VecDeque<&str> = VecDeque::from_iter(["C", "L", "S", "N", "F", "M", "D"]);
    let stack_6: VecDeque<&str> = VecDeque::from_iter(["R", "G", "C", "D"]);
    let stack_7: VecDeque<&str> = VecDeque::from_iter(["H", "G", "T", "R", "J", "D", "S", "Q"]);
    let stack_8: VecDeque<&str> = VecDeque::from_iter(["P", "F", "V"]);
    let stack_9: VecDeque<&str> = VecDeque::from_iter(["D", "R", "S", "T", "J"]);
    // dbg!(stack_1);
    // dbg!(stack_2);
    // dbg!(stack_3);
    // dbg!(stack_4);
    // dbg!(stack_5);
    // dbg!(stack_6);
    // dbg!(stack_7);
    // dbg!(stack_8);
    // dbg!(stack_9);

    let formatted_input = input.split("\n").collect::<Vec<&str>>();
    let parsed_input = &formatted_input[10..];

    for line in parsed_input {
        let res = line.split(" ").collect::<Vec<&str>>();
        let amount_to_move = res[1].parse::<u16>().unwrap();
        let from_stack = res[3].parse::<u16>().unwrap();
        let to_stack = res[5].parse::<u16>().unwrap();
        dbg!(amount_to_move);
        dbg!(from_stack);
        dbg!(to_stack);
        // dbg!(&res);
    }

    // dbg!(&parsed_input);

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result: u16 = 0;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn p1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "");
    }

    // #[test]
    // fn p2_works() {
    //     let result = process_part2(TEST_INPUT);
    //     assert_eq!(result, "");
    // }
}

// 955 = too high
// 526 incorrect
// 917 correct
