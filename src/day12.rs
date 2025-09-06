use std::collections::{HashMap, HashSet, VecDeque};

type Point = (isize, isize);

struct Garden {
    map: HashMap<Point, char>,
}

impl Garden {
    fn get_regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited = HashSet::new();
        let mut min_bound = (isize::MAX, isize::MAX);
        let mut max_bound = (isize::MIN, isize::MIN);

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
            visited.insert(point.to_owned());
            queue.push_back(point.to_owned());

            while let Some(plot) = queue.pop_front() {
                plots.insert(plot);

                // Determine the bounding box of the region
                min_bound.0 = min_bound.0.min(plot.0);
                min_bound.1 = min_bound.1.min(plot.1);
                max_bound.0 = max_bound.0.max(plot.0);
                max_bound.1 = max_bound.1.max(plot.1);

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

            regions.push(Region {
                points: plots,
                min_bound,
                max_bound,
            });
        }

        regions
    }
}

struct Region {
    points: HashSet<Point>,
    min_bound: Point,
    max_bound: Point,
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

    fn sides(&self) -> usize {
        let mut sides = 0;

        // Horizontal scan
        for row in self.min_bound.0..=self.max_bound.0 {
            let mut t_edge = None;
            let mut b_edge = None;

            for col in self.min_bound.1..=self.max_bound.1 {
                if let Some(point) = self.points.get(&(row, col)) {
                    t_edge = match self.points.get(&(row - 1, col)) {
                        Some(_) => None,
                        None => {
                            // It's a top edge
                            if t_edge.is_none() {
                                sides += 1;
                            }
                            Some(point)
                        }
                    };
                    b_edge = match self.points.get(&(row + 1, col)) {
                        Some(_) => None,
                        None => {
                            // It's a bottom edge
                            if b_edge.is_none() {
                                sides += 1;
                            }
                            Some(point)
                        }
                    }
                } else {
                    t_edge = None;
                    b_edge = None;
                }
            }
        }

        // Vertical scan
        for col in self.min_bound.1..=self.max_bound.1 {
            let mut l_edge = None;
            let mut r_edge = None;

            for row in self.min_bound.0..=self.max_bound.0 {
                if let Some(point) = self.points.get(&(row, col)) {
                    l_edge = match self.points.get(&(row, col - 1)) {
                        Some(_) => None,
                        None => {
                            // It's a left edge
                            if l_edge.is_none() {
                                sides += 1;
                            }
                            Some(point)
                        }
                    };
                    r_edge = match self.points.get(&(row, col + 1)) {
                        Some(_) => None,
                        None => {
                            // It's a right edge
                            if r_edge.is_none() {
                                sides += 1;
                            }
                            Some(point)
                        }
                    }
                } else {
                    r_edge = None;
                    l_edge = None;
                }
            }
        }

        sides
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn cost(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn discount_cost(&self) -> usize {
        self.area() * self.sides()
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
fn part2(input: &Garden) -> usize {
    input
        .get_regions()
        .iter()
        .map(|region| region.discount_cost())
        .sum()
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

    const INPUT_4: &str = "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE
    ";

    const INPUT_5: &str = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
    ";

    #[test_case(INPUT_1, 140)]
    #[test_case(INPUT_2, 772)]
    #[test_case(INPUT_3, 1930)]
    fn part1_example(input: &str, want: usize) {
        assert_eq!(part1(&parse(input)), want);
    }

    #[test_case(INPUT_1, 80)]
    #[test_case(INPUT_2, 436)]
    #[test_case(INPUT_3, 1206)]
    #[test_case(INPUT_4, 236)]
    #[test_case(INPUT_5, 368)]
    fn part2_example(input: &str, want: usize) {
        assert_eq!(part2(&parse(input)), want);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(12));
        assert_eq!(part1(input), 1477762);
        assert_eq!(part2(input), 923480);
    }
}
