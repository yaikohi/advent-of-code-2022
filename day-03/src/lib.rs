use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let compartment_a = &line[0..sack_length];
            let compartment_b = &line[sack_length..(sack_length * 2)];

            let common_char = compartment_a
                .chars()
                .find(|c| compartment_b.contains(*c))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let scores: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect();

    let result: usize = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let a = lines[0];
            let b = lines[1];
            let c = lines[2];

            dbg!(a, b, c);
            let common_character = a
                .chars()
                .find(|chr| b.contains(*chr) && c.contains(*chr))
                .unwrap();
            dbg!(&common_character);
            dbg!(scores.get(&common_character).unwrap());
            scores.get(&common_character).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn p1_works() {
        let res = process_part1(INPUT);
        assert_eq!(res, "157")
    }

    #[test]
    fn p2_works() {
        let res = process_part2(INPUT);
        assert_eq!(res, "70")
    }
}
