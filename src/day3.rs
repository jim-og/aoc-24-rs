use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Unable to initialise regex");
    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        res.push((
            first.parse().expect("Unable to parse first number"),
            second.parse().expect("Unable to parse second number"),
        ));
    }

    res
}

#[aoc(day3, part1)]
fn part1(input: &Vec<(usize, usize)>) -> usize {
    input.iter().map(|(first, second)| first * second).sum()
}

#[aoc(day3, part2)]
fn part2(_input: &Vec<(usize, usize)>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 161);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(3));
        assert_eq!(part1(input), 188116424);
    }
}
