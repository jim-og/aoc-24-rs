struct Equation {
    target: isize,
    numbers: Vec<isize>,
}

impl Equation {
    fn solve(&self, acc: isize, index: usize) -> bool {
        if index >= self.numbers.len() {
            return acc == self.target;
        }

        if let Some(number) = self.numbers.get(index) {
            return self.solve(acc + number, index + 1) || self.solve(acc * number, index + 1);
        }

        false
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split([':', ' '])
                .filter(|value| !value.is_empty())
                .map(|value| {
                    value
                        .parse::<isize>()
                        .expect("Error parsing str value to number")
                })
                .collect::<Vec<isize>>()
        })
        .map(|values| Equation {
            target: *values
                .first()
                .expect("Expected equation to have a target value"),
            numbers: values[1..].to_vec(),
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
