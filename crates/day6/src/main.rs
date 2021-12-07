const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u64 {
    simulate(input, 80)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u64 {
    simulate(input, 256)
}

fn simulate(input: &str, days: i32) -> u64 {
    let mut fishes: Vec<u64> = input.split(',').flat_map(|f| f.parse().ok()).collect();

    for _ in 0..days {
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                fishes.append(&mut vec![8]);
                fishes[i] = 6;
            } else {
                fishes[i] -= 1;
            }
        }
    }

    fishes.len() as u64
}

fn potentially_faster(input: &str, days: i32) -> u64 {
    // hold the total amount of fish with days left in an array
    // index 0 is fish that need to give birth
    // index 1..8 is the amount of fish with days remaining each
    // [0, 1, 2, 3, 4, 5, 6, 7, 8]
    let mut fish: [u64; 9] = [0; 9];

    input
        .split(',')
        .flat_map(|f| f.parse::<usize>().ok())
        .for_each(|n| fish[n] += 1);

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish.iter().sum()
}

// #[cfg(test)]
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
