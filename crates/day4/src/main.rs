use itertools::Itertools;
use vectrix::Matrix;

const INPUT: &str = include_str!("input.txt");

type BoardMatrix = Matrix<(u32, bool), 5, 5>;

struct Board {
    numbers: BoardMatrix,
}
impl Board {
    fn mark_number(&mut self, called_number: u32) {
        self.numbers = self
            .numbers
            .iter()
            .map(|x| {
                if x.0 == called_number {
                    return (x.0, true);
                }
                *x
            })
            .collect()
    }

    fn is_winner(&self) -> bool {
        let row_full = self.numbers.iter_rows().any(|row| row.iter().all(|x| x.1));

        let col_full = self
            .numbers
            .iter_columns()
            .any(|col| col.iter().all(|x| x.1));

        row_full || col_full
    }

    fn score(&self, called_number: u32) -> u32 {
        self.numbers
            .into_iter()
            .filter(|x| !x.1)
            .map(|x| x.0)
            .sum::<u32>()
            * called_number
    }
}

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn solve_problem1(input: &str) -> u32 {
    let (draws, mut boards) = get_inputs(input);
    for number in draws {
        for board in boards.iter_mut() {
            board.mark_number(number);

            if board.is_winner() {
                return board.score(number);
            }
        }
    }

    0
}

fn get_inputs(input: &str) -> (Vec<u32>, Vec<Board>) {
    let (draws, boards) = input.split_once("\n\n").unwrap();
    let draws: Vec<u32> = draws
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect_vec();

    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|board| {
            let numbers = board
                .lines()
                .flat_map(|line| {
                    line.split_whitespace()
                        .flat_map(|x| x.parse::<u32>().ok())
                        .map(|n| (n, false))
                })
                .collect();
            Board { numbers }
        })
        .collect();

    (draws, boards)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    todo!()
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
