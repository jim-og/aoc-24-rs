struct Equation {
    target: isize,
    numbers: Vec<isize>,
}

impl Equation {
    fn solve(&self, acc: isize, index: usize) -> bool {
        match self.numbers.get(index) {
            Some(number) => {
                self.solve(acc + number, index + 1) || self.solve(acc * number, index + 1)
            }
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
fn part1(input: &Vec<Equation>) -> isize {
    input
        .iter()
        .filter(|equation| equation.solve(0, 0))
        .map(|equation| equation.target)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Equation>) -> String {
    todo!()
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
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(7));
        assert_eq!(part1(input), 3119088655389);
    }
}
