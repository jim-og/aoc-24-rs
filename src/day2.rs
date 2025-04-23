use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<isize>> {
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
    fn safe(&self, min_diff: isize, max_diff: isize) -> bool;
}

impl Report for Vec<isize> {
    fn safe(&self, min_diff: isize, max_diff: isize) -> bool {
        let mut prev_diff = self[1] - self[0];

        for w in self.windows(2) {
            let diff = w[1] - w[0];
            if diff.abs() < min_diff || diff.abs() > max_diff || diff.signum() != prev_diff.signum()
            {
                return false;
            }
            prev_diff = diff;
        }
        true
    }
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Vec<isize>>) -> usize {
    input.iter().filter(|report| report.safe(1, 3)).count()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Vec<isize>>) -> usize {
    let mut count = 0;

    for report in input {
        if report.safe(1, 3) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut r = report.clone();
                r.remove(i);
                if r.safe(1, 3) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
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
        assert_eq!(part2(input), 373);
    }
}
