use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    get_numbers(input)
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    get_numbers(input)
        .windows(3)
        .map(|windows| windows.iter().sum::<i32>())
        .collect_vec()
        .windows(2)
        .filter(|windows| windows[0] < windows[1])
        .count()
}

fn get_numbers(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod test {
    use crate::{solve_problem1, solve_problem2};

    #[test]
    #[ignore = "day not available"]
    fn problem1() {
        let input = "";
        let expected = 0;
        let actual = solve_problem1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore = "day not available"]
    fn problem2() {
        let input = "";
        let expected = 0;
        let actual = solve_problem2(input);
        assert_eq!(expected, actual);
    }
}
