use std::collections::{HashMap, HashSet, VecDeque};

type Point = (isize, isize);

struct Garden {
    map: HashMap<Point, char>,
}

impl Garden {
    fn get_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited = HashSet::new();

        for (point, plant) in &self.map {
            if visited.len() == self.map.len() {
                break;
            }

            if visited.contains(point) {
                continue;
            }

            // Collect plots in region
            let mut plots = HashSet::new();

            // Breadth first search
            let mut queue = VecDeque::new();
            queue.push_back(point.to_owned());

            while let Some(plot) = queue.pop_front() {
                visited.insert(plot);
                plots.insert(plot);

                vec![
                    (plot.0 - 1, plot.1),
                    (plot.0, plot.1 + 1),
                    (plot.0 + 1, plot.1),
                    (plot.0, plot.1 - 1),
                ]
                .into_iter()
                .filter(|p| match self.map.get(p) {
                    Some(c) => c == plant,
                    None => false,
                })
                .for_each(|f| {
                    if !visited.contains(&f) {
                        visited.insert(f);
                        queue.push_back(f);
                    }
                });
            }

            regions.push(Region { points: plots });
        }

        regions
    }
}

struct Region {
    points: HashSet<Point>,
}

impl Region {
    fn perimeter(&self) -> usize {
        self.points
            .iter()
            .flat_map(|point| {
                vec![
                    (point.0 - 1, point.1),
                    (point.0, point.1 + 1),
                    (point.0 + 1, point.1),
                    (point.0, point.1 - 1),
                ]
                .into_iter()
                .filter(|p| !self.points.contains(p))
                .map(|_| 1)
            })
            .sum()
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn cost(&self) -> usize {
        self.area() * self.perimeter()
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Garden {
    let mut map = HashMap::new();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            map.insert((row as isize, col as isize), c);
        }
    }

    Garden { map }
}

#[aoc(day12, part1)]
fn part1(input: &Garden) -> usize {
    input.get_regions().iter().map(|region| region.cost()).sum()
}

#[aoc(day12, part2)]
fn part2(_input: &Garden) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    const INPUT_1: &str = "
        AAAA
        BBCD
        BBCC
        EEEC
    ";

    const INPUT_2: &str = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    ";

    const INPUT_3: &str = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    ";

    #[test_case(INPUT_1, 140)]
    #[test_case(INPUT_2, 772)]
    #[test_case(INPUT_3, 1930)]
    fn part1_example(input: &str, want: usize) {
        assert_eq!(part1(&parse(input)), want);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(12));
        assert_eq!(part1(input), 1477762);
    }
}
