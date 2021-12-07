use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let coords = gather_coords(input);
    let lines = coords
        .into_iter()
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .collect_vec();
    get_intersections(lines)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let coords = gather_coords(input);
    get_intersections(coords)
}

fn get_intersections(coords: Vec<(i32, i32, i32, i32)>) -> usize {
    let mut points = HashMap::new();
    for (mut x1, mut y1, x2, y2) in coords {
        // check to see if the deltas will be +1 or -1
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        while (x1, y1) != (x2 + dx, y2 + dy) {
            *points.entry((x1, y1)).or_insert(0) += 1;
            x1 += dx;
            y1 += dy;
        }
    }
    points.values().filter(|&&n| n > 1).count()
}

fn gather_coords(input: &str) -> Vec<(i32, i32, i32, i32)> {
    input
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .map(|i| i.split(','))
                .flatten()
                .flat_map(|i| i.parse().ok())
                .collect_tuple()
        })
        .collect()
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
