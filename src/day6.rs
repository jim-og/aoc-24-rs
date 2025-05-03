use std::collections::HashMap;

type Point = (isize, isize);

trait Movable {
    fn progress(&self, direction: &Direction) -> Point;
}

impl Movable for Point {
    fn progress(&self, direction: &Direction) -> Point {
        match direction {
            Direction::North => (self.0 - 1, self.1),
            Direction::East => (self.0, self.1 + 1),
            Direction::South => (self.0 + 1, self.1),
            Direction::West => (self.0, self.1 - 1),
        }
    }
}

type Map = HashMap<Point, Position>;

trait Traversable {
    fn visited(&self) -> usize;
}

impl Traversable for Map {
    fn visited(&self) -> usize {
        self.iter().filter(|(_, position)| position.visited).count()
    }
}

#[derive(Clone)]
struct Position {
    visited: bool,
    obstructed: bool,
}

impl Position {
    pub fn new(obstructed: bool) -> Self {
        Self {
            visited: false,
            obstructed,
        }
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

trait Turnable {
    fn turn(&self) -> Direction;
}

impl Turnable for Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (Point, Map) {
    let mut start = (0, 0);
    let mut map = HashMap::new();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            let position = match c {
                '^' => {
                    start = (row as isize, col as isize);
                    Position {
                        visited: true,
                        obstructed: false,
                    }
                }
                '#' => Position::new(true),
                _ => Position::new(false),
            };
            map.insert((row as isize, col as isize), position);
        }
    }

    (start, map)
}

#[aoc(day6, part1)]
fn part1(input: &(Point, Map)) -> usize {
    let mut point = input.0;
    let mut direction = Direction::North;
    let mut map = input.1.clone();

    loop {
        let next_point = point.progress(&direction);
        match map.get_mut(&next_point) {
            Some(position) => {
                if position.obstructed {
                    // Turn if obstructed
                    direction = direction.turn();
                } else {
                    // Otherwise, progress in the current direction
                    position.visit();
                    point = point.progress(&direction);
                }
            }
            None => break,
        }
    }

    map.visited()
}

#[aoc(day6, part2)]
fn part2(_input: &(Point, Map)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST: &str = "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST)), 41);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(TEST)), 6);
    // }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(6));
        assert_eq!(part1(input), 5208);
    }
}
