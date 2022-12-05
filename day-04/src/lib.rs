pub fn process_part1(input: &str) -> String {
    let mut result = 0;
    let formatted_input = input.lines().collect::<Vec<&str>>().into_iter();

    for line in formatted_input {
        let lines = line
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|slice| {
                let res = slice
                    .split("-")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|val| {
                        let res = val.parse::<usize>().unwrap();
                        res
                    })
                    .collect::<Vec<usize>>();
                res
            })
            .collect::<Vec<_>>();

        if (lines[0][0] <= lines[1][0] && lines[0][1] >= lines[1][1])
            | (lines[0][0] >= lines[1][0] && lines[0][1] <= lines[1][1])
        {
            result += 1
        }
    }

    dbg!(result);

    result.to_string()
}

pub fn process_part2(input: &str) -> &str {
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn p1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "2");
    }

    // #[test]
    // fn p2_works() {
    //     let result = process_part2(TEST_INPUT);
    //     // assert_eq!(result, 4);
    // }
}
