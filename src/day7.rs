struct Equation {
    target: isize,
    numbers: Vec<isize>,
}

impl Equation {
    fn solve(&self, acc: isize, index: usize, concat_enabled: bool) -> bool {
        match self.numbers.get(index) {
            Some(number) => {
                if self.solve(acc + number, index + 1, concat_enabled)
                    || self.solve(acc * number, index + 1, concat_enabled)
                {
                    true
                } else if concat_enabled {
                    let combined = format!("{}{}", acc, number)
                        .parse::<isize>()
                        .expect("Error concatenating numbers");
                    self.solve(combined, index + 1, concat_enabled)
                } else {
                    false
                }
            }
            // All numbers used, have we got the right answer?
            None => acc == self.target,
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (target, numbers) = line
                .trim()
                .split_once(": ")
                .expect("Error splitting input line");
            Equation {
                target: target.parse().expect("Error parsing target"),
                numbers: numbers
                    .split_whitespace()
                    .map(|number| number.parse().expect("Error parsing numbers"))
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> isize {
    input
        .iter()
        .filter(|equation| equation.solve(0, 0, false))
        .map(|equation| equation.target)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> isize {
    input
        .iter()
        .filter(|equation| equation.solve(0, 0, true))
        .map(|equation| equation.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 11387);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(7));
        assert_eq!(part1(input), 3119088655389);
        assert_eq!(part2(input), 264184041398847);
    }
}
