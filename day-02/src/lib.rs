// Points:
// - 1 for Rock,
// - 2 for Paper,
// - and 3 for Scissors.
//
// - 6 for winning,
// - 3 for a draw
// - 0 for losing
// A - X = draw, both 3
// A - Y = p2 wins 6, p1 loses 0
// A - Z = p2 loses 0, p1 wins 6

// enum Move {
//     Rock = 1,
//     Paper = 2,
//     Scizzors = 3,
// }


pub fn process_part1(input: &str) -> String {
    let res = get_results_ver_1(input);
    res
}

/**
 * Converts the array to an array with chunks imitating a 2d array
 */
pub fn to_2d_arr(arr: Vec<&str>) -> Vec<Vec<&str>> {
    let result = arr
        .chunks(2)
        .map(|comp| {
            let res = comp.to_owned();
            res
        })
        .collect::<Vec<Vec<_>>>();
    result
}

/**
 * Removes whitespace & linebreaks
 */
pub fn format_input(input: &str) -> Vec<Vec<&str>> {
    let input_whitespace_removed: Vec<&str> = input.split_whitespace().collect::<Vec<&str>>();

    let result_array = input_whitespace_removed
        .iter()
        .map(|line| {
            let res = line.split("\n").collect::<Vec<&str>>();
            res[0]
        })
        .collect::<Vec<&str>>();

    return to_2d_arr(result_array);
}

pub fn get_results_ver_1(input: &str) -> String {
    let result = format_input(input);

    // let mut p1score = 0;
    let mut p2score: u8 = 0;

    let iterable_result = result.into_iter();

    for game in iterable_result {
        match game[..] {
            ["A", "Y"] => {
                // p1score += 1;
                p2score += 8
            }
            ["A", "Z"] => {
                // p1score += 7;
                p2score += 3
            }
            ["A", "X"] => {
                // p1score += 4;
                p2score += 4
            }
            ["B", "Z"] => {
                // p1score += 2;
                p2score += 9
            }
            ["B", "X"] => {
                // p1score += 8;
                p2score += 1
            }
            ["B", "Y"] => {
                // p1score += 5;
                p2score += 5
            }
            ["C", "Z"] => {
                // p1score += 6;
                p2score += 6
            }
            ["C", "X"] => {
                // p1score += 3;
                p2score += 7
            }
            ["C", "Y"] => {
                // p1score += 9;
                p2score += 2
            }
            _ => println!("HELP"),
        }
    }
    p2score.to_string()
}

// TODO:
// 1. -> end result = points?
// 2. -> to achieve win, what move? -> what points?
// 3. add both of these for every game
pub fn process_part2(input: &str) -> String {
    let mut res: usize = 0;
    let filtered_input = input
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    for game in filtered_input {
        match game[..] {
            ["A", "X"] => res += 3, // 0
            ["A", "Y"] => res += 4, // 3
            ["A", "Z"] => res += 8, // 6
            ["B", "X"] => res += 1, // 0
            ["B", "Y"] => res += 5, // 3
            ["B", "Z"] => res += 9, // 6
            ["C", "X"] => res += 2, // 0
            ["C", "Y"] => res += 6, // 3
            ["C", "Z"] => res += 7, // 6
            _ => println!("Error!"),
        }
        dbg!(game);
        dbg!(res);
    }
    res.to_string()
}
#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1_works() {
        let res = process_part1(INPUT);
        assert_eq!(res, "15");
    }

    #[test]
    fn part2_works() {
        let res = process_part2(INPUT);
        assert_eq!(res, "12");
    }
}
