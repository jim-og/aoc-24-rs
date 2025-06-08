use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref STORE: Mutex<PebbleStore> = Mutex::new(PebbleStore::default());
}

type Pebbles = Vec<usize>;

#[derive(Default)]
struct PebbleStore {
    store: HashMap<usize, Pebbles>,
}

impl PebbleStore {
    fn get(&self, pebble: usize) -> Option<&Pebbles> {
        self.store.get(&pebble)
    }

    fn insert(&mut self, before: usize, after: Pebbles) {
        self.store.insert(before, after);
    }
}

trait Blinkable {
    fn blink(self) -> Pebbles;
}

impl Blinkable for usize {
    fn blink(self) -> Pebbles {
        if let Some(pebbles) = STORE.lock().unwrap().get(self) {
            return pebbles.to_vec();
        }

        let pebble_string = self.to_string();

        let pebbles = if self == 0 {
            vec![1]
        } else if pebble_string.len() % 2 == 0 {
            // Even number of digits
            let (left, right) = pebble_string.split_at(pebble_string.len() / 2);
            vec![
                left.parse().expect("Error parsing left pebble"),
                right.parse().expect("Error parsing right pebble"),
            ]
        } else {
            // Odd number of digits
            vec![self * 2024]
        };

        STORE.lock().unwrap().insert(self, pebbles.clone());
        pebbles
    }
}

impl Blinkable for Pebbles {
    fn blink(self) -> Pebbles {
        self.iter().flat_map(|pebble| pebble.blink()).collect()
    }
}

fn blink_n(input: &Pebbles, n: usize) -> usize {
    let mut pebbles = input.clone();
    for _ in 0..n {
        pebbles = pebbles.blink();
    }
    pebbles.len()
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Pebbles {
    input
        .split_whitespace()
        .map(|pebble| pebble.parse().expect("Error parsing numbers"))
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &Pebbles) -> usize {
    blink_n(input, 25)
}

#[aoc(day11, part2)]
fn part2(input: &Pebbles) -> usize {
    let mut pebbles: HashMap<usize, usize> = HashMap::new();
    input
        .iter()
        .for_each(|&num| *pebbles.entry(num).or_insert(0) += 1);

    for _ in 0..75 {
        let mut split_pebbles = HashMap::new();

        for (&pebble, &count) in pebbles.iter() {
            let pebble_string = pebble.to_string();
            if pebble == 0 {
                *split_pebbles.entry(1).or_insert(0) += count;
            } else if pebble_string.len() % 2 == 0 {
                // Even number of digits
                let (left, right) = pebble_string.split_at(pebble_string.len() / 2);
                *split_pebbles
                    .entry(left.parse().expect("Error parsing left pebble"))
                    .or_insert(0) += count;
                *split_pebbles
                    .entry(right.parse().expect("Error parsing left pebble"))
                    .or_insert(0) += count;
            } else {
                // Odd number of digits
                *split_pebbles.entry(pebble * 2024).or_insert(0) += count;
            }
        }

        pebbles = split_pebbles;
    }
    pebbles.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use test_case::test_case;

    const INPUT_1: &str = "
        0 1 10 99 999
    ";

    const INPUT_2: &str = "
        125 17
    ";

    #[test_case(INPUT_1, 1, "1 2024 1 0 9 9 2021976"; "input_1")]
    #[test_case(INPUT_2, 6, "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"; "input_2")]
    fn split_example(input: &str, blinks: usize, want: &str) {
        let mut pebbles = parse(input);
        for _ in 0..blinks {
            pebbles = pebbles.blink();
        }
        let mut result = format!("{:?}", pebbles).replace(",", "");
        result.pop();
        result.remove(0);

        assert_eq!(result, want);
    }

    #[test_case(INPUT_2, 55312; "input_2")]
    fn part1_example(input: &str, want: usize) {
        assert_eq!(part1(&parse(input)), want);
    }

    #[test]
    fn mainline() {
        let input = &parse(&parser::load_input(11));
        assert_eq!(part1(input), 228668);
        assert_eq!(part2(input), 270673834779359);
    }
}
