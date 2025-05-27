use num_integer::Integer;

#[derive(Clone)]
enum Block {
    File(usize),
    Empty,
}

trait Disk {
    fn checksum(&self) -> usize;
    fn fragment(&mut self) -> &mut Vec<Block>;
}

impl Disk for Vec<Block> {
    fn checksum(&self) -> usize {
        self.iter()
            .enumerate()
            .map(|(position, block)| match block {
                Block::File(id) => position * id,
                Block::Empty => 0,
            })
            .sum()
    }

    fn fragment(&mut self) -> &mut Self {
        let mut l = 0_usize;
        let mut r = self.len() - 1;

        while l < r {
            let left = self.get(l).expect("Expected a left block");
            let right = self.get(r).expect("Expected a right block");

            if let Block::Empty = right {
                r -= 1;
                continue;
            }

            if let Block::Empty = left {
                self.swap(l, r);
                r -= 1;
            }

            l += 1;
        }

        self
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Block> {
    let mut id = 0;

    input
        .trim()
        .chars()
        .enumerate()
        .map(|(index, c)| (index, c.to_digit(10).expect("Failed to cast char to usize")))
        .flat_map(|(index, n)| {
            std::iter::repeat(if index.is_even() {
                id += 1;
                Block::File(id - 1)
            } else {
                Block::Empty
            })
            .take(n as usize)
            .collect::<Vec<Block>>()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Block]) -> usize {
    input.to_owned().fragment().checksum()
}

#[aoc(day9, part2)]
fn part2(input: &[Block]) -> usize {
    input.to_owned().fragment().checksum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    const INPUT_1: &str = "12345";
    const INPUT_2: &str = "2333133121414131402";

    #[test_case(INPUT_1, 60; "input_1")]
    #[test_case(INPUT_2, 1928; "input_2")]
    fn part1_example(input: &str, want: usize) {
        assert_eq!(part1(&parse(input)), want);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(9));
        assert_eq!(part1(input), 6356833654075);
    }
}
