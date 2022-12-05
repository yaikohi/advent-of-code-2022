use std::str::Split;

pub fn process_part2(input: &str) -> u32 {
    let mut result: Vec<u32> = count_calories(split_text(input));
    result.sort_by(|a,b| b.cmp(a));
    let sum = result.iter().take(3).sum::<u32>();
    sum
}

pub fn process_part1(input: &str) -> u32 {
    let result_split_text = split_text(input);
    let result_grouped_calories = count_calories(result_split_text);
    let result = count_max_calories(result_grouped_calories);
    result
}

pub fn split_text(input: &str) -> Split<&str> {
    let result = input.split("\n\n");
    result
}

pub fn count_calories(input: Split<&str>) -> Vec<u32> {
    let result = input
        .map(|elf_calories| {
            elf_calories
                .lines()
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    result
}

pub fn count_max_calories(input: Vec<u32>) -> u32 {
    let binding = input.to_owned();
    let result = binding.iter().max().expect("??").to_owned();
    result
}