const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    // println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let mut count = 0;
    let length = input.lines().next().unwrap().len();
    let mut gamma_vec = vec![];
    let mut epsilon_vec = vec![];
    loop {
        if count == length {
            break;
        }

        let mut seen_bits: Vec<u32> = vec![];
        for line in input.lines() {
            if let Some(bit) = line.chars().nth(count).map(|x| x.to_digit(2)).flatten() {
                seen_bits.push(bit);
            }
        }

        let ones = seen_bits.iter().filter(|&x| x.eq(&1)).count();
        let zeroes = seen_bits.iter().filter(|&x| x.eq(&0)).count();

        if (ones > zeroes) {
            gamma_vec.push('1');
            epsilon_vec.push('0');
        } else {
            gamma_vec.push('0');
            epsilon_vec.push('1');
        }

        count += 1;
    }

    let gamma = u32::from_str_radix(String::from_iter(gamma_vec).as_str(), 2).ok();
    let epsilon = u32::from_str_radix(String::from_iter(epsilon_vec).as_str(), 2).ok();
    if let (Some(g), Some(e)) = (gamma, epsilon) {
        return g * e;
    } else {
        0
    }
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
