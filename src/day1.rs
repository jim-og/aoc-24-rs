use std::collections::BinaryHeap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> (BinaryHeap<usize>, BinaryHeap<usize>) {
    let mut list_1 = BinaryHeap::new();
    let mut list_2 = BinaryHeap::new();

    for line in input.trim().lines() {
        let split = line.trim().split("   ").collect::<Vec<&str>>();
        list_1.push(
            split
                .first()
                .expect("Expected split to contain a first value")
                .parse::<usize>()
                .expect("Error parsing list 1 entry"),
        );
        list_2.push(
            split
                .last()
                .expect("Expected split to contain a last value")
                .parse::<usize>()
                .expect("Error parsing list 2 entry"),
        );
    }

    (list_1, list_2)
}

#[aoc(day1, part1)]
fn part1(input: &(BinaryHeap<usize>, BinaryHeap<usize>)) -> usize {
    let mut list_1 = input.0.clone();
    let mut list_2 = input.1.clone();
    let mut res = 0;

    while let Some(l1) = list_1.pop() {
        let l2 = list_2
            .pop()
            .expect("Expected list 2 to have a value to pop");
        let delta = (l1 as isize - l2 as isize).unsigned_abs();
        res += delta;
    }

    res
}

#[aoc(day1, part2)]
fn part2(input: &(BinaryHeap<usize>, BinaryHeap<usize>)) -> usize {
    let mut list_1 = input.0.clone();
    let mut list_2 = input.1.clone();
    let mut res = 0;

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 0);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(1));
        assert_eq!(part1(input), 1530215);
    }
}
