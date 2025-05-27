use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

type Point = (isize, isize);
type Antennas = HashMap<char, Vec<Point>>;

struct Input {
    antennas: Antennas,
    max_point: Point,
}

trait Constrainable {
    fn valid(&self, max_point: &Point) -> bool;
}

impl Constrainable for Point {
    fn valid(&self, max_point: &Point) -> bool {
        self.0 >= 0 && self.0 <= max_point.0 && self.1 >= 0 && self.1 <= max_point.1
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let mut antennas = HashMap::new();
    let mut max_point = Point::default();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            let point = (row as isize, col as isize);
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push(point);
            }
            max_point = max(max_point, point);
        }
    }

    Input {
        antennas,
        max_point,
    }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let mut antinodes = HashSet::new();

    for (_, antennas) in input.antennas.iter() {
        for (i, a1) in antennas.iter().enumerate() {
            for antenna in antennas.iter().skip(i + 1) {
                let a2 = antenna;
                let drow = a1.0 - a2.0;
                let dcol = a1.1 - a2.1;
                let an1 = (a1.0 + drow, a1.1 + dcol);
                let an2 = (a2.0 - drow, a2.1 - dcol);

                for an in [an1, an2] {
                    if an.valid(&input.max_point) {
                        antinodes.insert(an);
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    let mut antinodes = HashSet::new();

    for (_, antennas) in input.antennas.iter() {
        for (i, a1) in antennas.iter().enumerate() {
            for a2 in antennas.iter().skip(i + 1) {
                let drow = a1.0 - a2.0;
                let dcol = a1.1 - a2.1;

                let mut an = *a1;
                while an.valid(&input.max_point) {
                    antinodes.insert(an);
                    an = (an.0 + drow, an.1 + dcol)
                }

                an = *a2;
                while an.valid(&input.max_point) {
                    antinodes.insert(an);
                    an = (an.0 - drow, an.1 - dcol)
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    const TEST1: &str = "
        ..........
        ..........
        ..........
        ....a.....
        ..........
        .....a....
        ..........
        ..........
        ..........
        ..........
    ";

    const TEST2: &str = "
        ..........
        ..........
        ..........
        ....a.....
        ........a.
        .....a....
        ..........
        ......A...
        ..........
        ..........
    ";

    const TEST3: &str = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    ";

    const TEST4: &str = "
        T.........
        ...T......
        .T........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........  
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST1)), 2);
        assert_eq!(part1(&parse(TEST2)), 4);
        assert_eq!(part1(&parse(TEST3)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST4)), 9);
        assert_eq!(part2(&parse(TEST3)), 34);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(8));
        assert_eq!(part1(input), 240);
        assert_eq!(part2(input), 955);
    }
}
