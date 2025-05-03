use std::collections::{HashMap, HashSet};

struct Input {
    rules: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

impl Input {
    fn solve_part1(&self) -> usize {
        let mut res = 0;

        for update in &self.updates {
            let mut valid = true;

            for (i, val) in update.iter().enumerate() {
                let slices = update.split_at(i + 1);
                let mut left = Vec::from(slices.0);
                left.pop()
                    .expect("Should have a value to pop in left array");
                let right = Vec::from(slices.1);

                if let Some(rule) = self.rules.get(val) {
                    if rule.contains_any(right) {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                let (left, _) = update.split_at(update.len() / 2 + 1);
                res += left.last().expect("Should have a last value in left")
            }
        }

        res
    }

    fn solve_part2(&self) -> usize {
        123
    }
}

trait ContainsAny {
    fn contains_any(&self, values: Vec<usize>) -> bool;
}

impl ContainsAny for HashSet<usize> {
    fn contains_any(&self, values: Vec<usize>) -> bool {
        for val in values {
            if self.contains(&val) {
                return true;
            }
        }
        false
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in input.trim().lines().map(|line| line.trim()) {
        if line.contains('|') {
            let nums = line
                .split('|')
                .map(|c| {
                    c.trim()
                        .parse::<usize>()
                        .expect("Should be able to convert next to number")
                })
                .collect::<Vec<usize>>();

            let left = nums.first().expect("Should have a number in nums");
            let right = nums.last().expect("Should have a number in nums");

            rules
                .entry(*right)
                .or_insert_with(HashSet::new)
                .insert(*left);
        } else if line.contains(',') {
            updates.push(
                line.split(',')
                    .map(|n| {
                        n.parse()
                            .expect("Should be able to parse updates as numbers")
                    })
                    .collect(),
            );
        }
    }

    Input { rules, updates }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    input.solve_part1()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> usize {
    input.solve_part2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 123);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(5));
        assert_eq!(part1(input), 5329);
    }
}
