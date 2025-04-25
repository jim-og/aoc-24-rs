use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

enum Operation {
    Disable,
    Enable,
    Mul(usize, usize),
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Operation> {
    Regex::new(r"mul\((\d+),(\d+)\)|(do)(\(\))|(don't)(\(\))")
        .expect("Unable to initialise regex")
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(entry, [first, second])| match entry {
            "do()" => Operation::Enable,
            "don't()" => Operation::Disable,
            _ => Operation::Mul(
                first.parse().expect("Unable to parse first number"),
                second.parse().expect("Unable to parse second number"),
            ),
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Operation]) -> usize {
    input
        .iter()
        .map(|operation| match operation {
            Operation::Mul(x, y) => x * y,
            _ => 0,
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Operation]) -> usize {
    let mut enabled = true;

    input
        .iter()
        .map(|op| match op {
            Operation::Disable => {
                enabled = false;
                0
            }
            Operation::Enable => {
                enabled = true;
                0
            }
            Operation::Mul(x, y) => {
                if enabled {
                    return x * y;
                }
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const PART_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const PART_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_2)), 48);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(3));
        assert_eq!(part1(input), 188116424);
        assert_eq!(part2(input), 104245808);
    }
}
