use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split(' ')
                .map(|n| n.parse().expect("Error parsing level"))
                .collect()
        })
        .collect()
}

trait Report {
    fn safe(&self, min_diff: usize, max_diff: usize) -> bool;
}

impl Report for Vec<usize> {
    fn safe(&self, min_diff: usize, max_diff: usize) -> bool {
        let mut is_positive = None;
        for w in self.windows(2) {
            let diff = w[1] as isize - w[0] as isize;
            if diff.unsigned_abs() < min_diff || diff.unsigned_abs() > max_diff {
                return false;
            }
            if let Some(positive) = is_positive {
                if positive != diff.is_positive() {
                    return false;
                }
            } else {
                is_positive = Some(diff.is_positive());
            }
        }
        return true;
    }
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|report| if report.safe(1, 3) { 1 } else { 0 })
        .sum()
}

#[aoc(day2, part2)]
fn part2(_input: &Vec<Vec<usize>>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 4);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(2));
        assert_eq!(part1(input), 314);
    }
}
