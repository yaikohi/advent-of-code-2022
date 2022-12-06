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

// pub fn process_part2(input: &str) -> String {
//     let mut result: u16 = 0;
//     result.to_string()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn p1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "7");
    }

    // #[test]
    // fn p2_works() {
    //     let result = process_part2(TEST_INPUT);
    //     assert_eq!(result, "");
    // }
}

// 1829 is too low
// 1833 - correct!