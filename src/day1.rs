use std::collections::{BinaryHeap, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

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
fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut list_1 = BinaryHeap::from(input.0.clone());
    let mut list_2 = BinaryHeap::from(input.1.clone());
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
fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let counts = input.1.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    input.0.iter().fold(0, |mut acc, num| {
        if let Some(count) = counts.get(num) {
            acc += num * count;
        }
        acc
    })
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
        assert_eq!(part2(&parse(TEST)), 31);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(1));
        assert_eq!(part1(input), 1530215);
        assert_eq!(part2(input), 26800609);
    }
}
