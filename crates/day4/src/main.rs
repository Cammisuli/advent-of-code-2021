mod board;

use board::Board;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

fn solve_problem1(input: &str) -> u32 {
    let (draws, mut boards) = get_draws_and_boards(input);
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

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    let (draws, mut boards) = get_draws_and_boards(input);
    for number in draws {
        if let [mut board] = *boards {
            // there's only one board left at this point :sweat_smile:
            board.mark_number(number);
            return board.score(number);
        }

        boards = boards
            .into_iter()
            .map(|mut board| {
                board.mark_number(number);
                board
            })
            .filter(|x| !x.is_winner())
            .collect();
    }

    0
}

fn get_draws_and_boards(input: &str) -> (Vec<u32>, Vec<Board>) {
    let (draws, boards) = input.split_once("\n\n").unwrap();
    let draws: Vec<u32> = draws.split(',').filter_map(|x| x.parse().ok()).collect();

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
            Board::new(numbers)
        })
        .collect();

    (draws, boards)
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
