use std::collections::{HashMap, HashSet};

type Point = (isize, isize);

struct Facility {
    map: HashMap<Point, usize>,
    trailheads: Vec<Point>,
}

impl Facility {
    fn next_steps(&self, point: &Point) -> Vec<Point> {
        let height = self.map.get(point).expect("Expected point to exist in map");
        vec![
            (point.0 - 1, point.1),
            (point.0, point.1 + 1),
            (point.0 + 1, point.1),
            (point.0, point.1 - 1),
        ]
        .into_iter()
        .filter(|point| match self.map.get(point) {
            // The question states that each step must increment by exactly 1 to be valid.
            Some(value) => *value == (height + 1),
            None => false,
        })
        .collect()
    }

    fn hike(&self, point: &Point) -> Vec<Point> {
        let height = self.map.get(point).expect("Expected point to exist in map");
        if *height == 9 {
            Vec::from([*point])
        } else {
            self.next_steps(point)
                .iter()
                .flat_map(|next_point| self.hike(next_point))
                .collect()
        }
    }

    fn set_off(&self) -> (usize, usize) {
        self.trailheads.iter().map(|point| self.hike(point)).fold(
            (0, 0),
            |(acc_score, acc_rating), summits| {
                // Part 1 is how many unique SUMMITS a trailhead can reach.
                // Part 2 is how many unique TRAILS from a trailhead lead to a summit.
                let summits_set: HashSet<_> = summits.iter().collect();
                (acc_score + summits_set.len(), acc_rating + summits.len())
            },
        )
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Facility {
    let mut map = HashMap::new();
    let mut trailheads = Vec::new();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            let n = if c == '.' {
                // Set to an arbitrary number higher than the summit.
                10
            } else {
                c.to_digit(10).expect("Failed to convert char")
            } as usize;

            if n == 0 {
                trailheads.push((row as isize, col as isize));
            }

            map.insert((row as isize, col as isize), n);
        }
    }

    Facility { map, trailheads }
}

#[aoc(day10, part1)]
fn part1(input: &Facility) -> usize {
    input.set_off().0
}

#[aoc(day10, part2)]
fn part2(input: &Facility) -> usize {
    input.set_off().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    const INPUT_0: &str = "
        0123
        1234
        8765
        9876
    ";

    const INPUT_1: &str = "
        ...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9
    ";

    const INPUT_2: &str = "
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
    ";

    const INPUT_3: &str = "
        10..9..
        2...8..
        3...7..
        4567654
        ...8..3
        ...9..2
        .....01
    ";

    const INPUT_4: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    ";

    const INPUT_5: &str = "
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....
    ";

    const INPUT_6: &str = "
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
    ";

    const INPUT_7: &str = "
        012345
        123456
        234567
        345678
        4.6789
        56789.
    ";

    #[test_case(INPUT_0, 1; "input_0")]
    #[test_case(INPUT_1, 2; "input_1")]
    #[test_case(INPUT_2, 4; "input_2")]
    #[test_case(INPUT_3, 3; "input_3")]
    #[test_case(INPUT_4, 36; "input_4")]
    fn part1_example(input: &str, want: usize) {
        assert_eq!(part1(&parse(input)), want);
    }

    #[test_case(INPUT_5, 3; "input_5")]
    #[test_case(INPUT_6, 13; "input_6")]
    #[test_case(INPUT_7, 227; "input_7")]
    #[test_case(INPUT_4, 81; "input_4")]
    fn part2_example(input: &str, want: usize) {
        assert_eq!(part2(&parse(input)), want);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(10));
        assert_eq!(part1(input), 841);
        assert_eq!(part2(input), 1875);
    }
}
