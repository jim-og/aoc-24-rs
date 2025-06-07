use std::collections::{HashMap, HashSet};

type Point = (isize, isize);

#[derive(Eq, PartialEq, Hash)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

trait Offsetable {
    fn offset(&self, offset: isize, direction: &Direction) -> Point;
}

impl Offsetable for Point {
    fn offset(&self, offset: isize, direction: &Direction) -> Point {
        match direction {
            Direction::North => (self.0 - offset, self.1),
            Direction::NorthEast => (self.0 - offset, self.1 + offset),
            Direction::East => (self.0, self.1 + offset),
            Direction::SouthEast => (self.0 + offset, self.1 + offset),
            Direction::South => (self.0 + offset, self.1),
            Direction::SouthWest => (self.0 + offset, self.1 - offset),
            Direction::West => (self.0, self.1 - offset),
            Direction::NorthWest => (self.0 - offset, self.1 - offset),
        }
    }
}

#[derive(Default)]
struct WordSearch {
    map: HashMap<Point, char>,
    max: (isize, isize),
}

impl WordSearch {
    fn search_for_xword(&self) -> usize {
        let combinations = HashSet::from(["MSSM", "MMSS", "SMMS", "SSMM"]);
        let mut res = 0;

        for row in 0..=self.max.0 {
            for col in 0..=self.max.1 {
                let c = self
                    .map
                    .get(&(row, col))
                    .expect("Should be a letter at this position");

                if *c != 'A' {
                    continue;
                }

                let directions = [
                    Direction::NorthEast,
                    Direction::SouthEast,
                    Direction::SouthWest,
                    Direction::NorthWest,
                ];

                let cross = directions
                    .iter()
                    .map(|direction| {
                        self.map
                            .get(&(row, col).offset(1, direction))
                            .unwrap_or(&'.')
                    })
                    .copied()
                    .collect::<String>();

                if combinations.contains(&cross.as_str()) {
                    res += 1;
                }
            }
        }

        res
    }

    fn search_for_word(&self, word: &str) -> usize {
        let mut res = 0;

        for row in 0..=self.max.0 {
            for col in 0..=self.max.1 {
                let mut invalid = HashSet::new();
                let directions = [
                    Direction::North,
                    Direction::NorthEast,
                    Direction::East,
                    Direction::SouthEast,
                    Direction::South,
                    Direction::SouthWest,
                    Direction::West,
                    Direction::NorthWest,
                ];

                for (i, c) in word.chars().enumerate() {
                    for direction in &directions {
                        if invalid.contains(direction) {
                            continue;
                        }
                        if let Some(next_c) =
                            self.map.get(&(row, col).offset(i as isize, direction))
                        {
                            if *next_c != c {
                                invalid.insert(direction);
                            }
                        } else {
                            invalid.insert(direction);
                        }
                    }
                }

                res += directions.len() - invalid.len();
            }
        }

        res
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> WordSearch {
    let mut word_search = WordSearch::default();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            word_search.map.insert((row as isize, col as isize), c);
        }
    }

    word_search.max = word_search.map.keys().cloned().max().unwrap_or((0, 0));
    word_search
}

#[aoc(day4, part1)]
fn part1(input: &WordSearch) -> usize {
    input.search_for_word("XMAS")
}

#[aoc(day4, part2)]
fn part2(input: &WordSearch) -> usize {
    input.search_for_xword()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 9);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(4));
        assert_eq!(part1(input), 2646);
        assert_eq!(part2(input), 2000);
    }
}
