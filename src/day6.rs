use std::collections::{HashMap, HashSet};

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

struct Input {
    start: Point,
    map: Map,
    max: Point,
}

trait Traversable {
    fn traverse(&mut self, start: Point, start_direction: Direction) -> bool;
    fn visited(&self) -> usize;
}

impl Traversable for Map {
    fn traverse(&mut self, start: Point, start_direction: Direction) -> bool {
        let mut point = start;
        let mut direction = start_direction;

        loop {
            let next_point = point.progress(&direction);
            match self.get_mut(&next_point) {
                Some(position) => {
                    if position.visited.contains(&direction) {
                        // We're in a loop
                        return false;
                    } else if position.obstructed {
                        // Turn if obstructed
                        direction = direction.turn();
                    } else {
                        // Otherwise, progress in the current direction
                        position.visit(&direction);
                        point = next_point;
                    }
                }
                None => break,
            }
        }

        true
    }

    fn visited(&self) -> usize {
        self.iter()
            .filter(|(_, position)| !position.visited.is_empty())
            .count()
    }
}

#[derive(Clone)]
struct Position {
    visited: HashSet<Direction>,
    obstructed: bool,
}

impl Position {
    pub fn new(obstructed: bool) -> Self {
        Self {
            visited: HashSet::new(),
            obstructed,
        }
    }

    pub fn visit(&mut self, direction: &Direction) {
        self.visited.insert(direction.clone());
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
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
fn parse(input: &str) -> Input {
    let mut start = (0, 0);
    let mut map = HashMap::new();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            let position = match c {
                '^' => {
                    start = (row as isize, col as isize);
                    Position {
                        visited: HashSet::from([Direction::North]),
                        obstructed: false,
                    }
                }
                '#' => Position::new(true),
                _ => Position::new(false),
            };
            map.insert((row as isize, col as isize), position);
        }
    }

    let max = *map.keys().max().expect("Map should have a maximum point");

    Input { start, map, max }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    let mut map = input.map.clone();
    match map.traverse(input.start, Direction::North) {
        true => map.visited(),
        false => panic!("Did not traverse map correctly"),
    }
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> usize {
    // brute force, take the input map and iterate through each point adding an obstacle
    // if an obstacle exists already, or it's the start, don't add an obstacle
    // detect loops and return
    // improvement to determine candidates for obstacles
    // will be all the next_point locations
    // could do an initial pass to find these
    // input.1.clone().traverse(input.0, Direction::North).loops()
    let mut res = 0;

    for row in 0..=input.max.0 {
        for col in 0..=input.max.1 {
            if (row, col) == input.start {
                continue;
            }

            let mut map = input.map.clone();

            if let Some(position) = map.get_mut(&(row, col)) {
                if position.obstructed {
                    continue;
                }
                position.obstructed = true;
                if !map.traverse(input.start, Direction::North) {
                    res += 1;
                }
            }
        }
    }

    res
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST)), 6);
        // 2268
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(6));
        assert_eq!(part1(input), 5208);
        assert_eq!(part2(input), 1972);
    }
}
