use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    let mut result: u16 = 0;
    let res = input
        .split("")
        .filter(|c| !(c.is_empty()))
        .collect::<Vec<&str>>();
    for char in 0..(res.len() - 4) {
        let one = res[char];
        let two = res[char + 1];
        let three = res[char + 2];
        let four = res[char + 3];

        let original_slice = [one, two, three, four];
        let unique_slice = [one, two, three, four]
            .into_iter()
            .unique()
            .collect::<Vec<&str>>();

        if original_slice.len() == unique_slice.len() {
            result = (char as u16) + 4;
            break;
        }
    }
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result: u16 = 0;
    let res = input
        .split("")
        .filter(|c| !(c.is_empty()))
        .collect::<Vec<&str>>();
    for char in 0..(res.len() - 14) {
        let one = res[char];
        let two = res[char + 1];
        let three = res[char + 2];
        let four = res[char + 3];
        let five = res[char + 4];
        let six = res[char + 5];
        let seven = res[char + 6];
        let eight = res[char + 7];
        let nine = res[char + 8];
        let ten = res[char + 9];
        let eleven = res[char + 10];
        let twelve = res[char + 11];
        let thirteen = res[char + 12];
        let fourteen = res[char + 13];

        let original_slice = [
            one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve, thirteen,
            fourteen,
        ];
        let unique_slice = [
            one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve, thirteen,
            fourteen,
        ]
        .into_iter()
        .unique()
        .collect::<Vec<&str>>();

        if original_slice.len() == unique_slice.len() {
            result = (char as u16) + 14;
            break;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    #[test]
    fn p1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn p2_works() {
        let result = process_part2(TEST_INPUT);
        assert_eq!(result, "19");
    }
}

// p1: 1829 is too low
// p1: 1833 - correct!

// p2: 3425 - correct!