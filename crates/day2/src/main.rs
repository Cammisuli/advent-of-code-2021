const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let mut depth = 0;
    let mut forward = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let movement = split.next();
        let amount = split.next().and_then(|x| x.parse::<i32>().ok());
        match (movement, amount) {
            (Some("forward"), Some(count)) => forward += count,
            (Some("down"), Some(count)) => depth += count,
            (Some("up"), Some(count)) => depth -= count,
            _ => (),
        }
    }

    depth * forward
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let movement = split.next();
        let amount = split.next().and_then(|x| x.parse::<i32>().ok());
        match (movement, amount) {
            (Some("forward"), Some(movement)) => {
                forward += movement;
                depth += (movement * aim);
            }
            (Some("down"), Some(movement)) => aim += movement,
            (Some("up"), Some(movement)) => aim -= movement,
            _ => (),
        }
    }

    depth * forward
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
