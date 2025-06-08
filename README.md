# Advent of Code 2024
[![codecov](https://codecov.io/github/jim-og/aoc-24-rs/graph/badge.svg?token=UOKQR6JBFE)](https://codecov.io/github/jim-og/aoc-24-rs)

My solutions to 2024's [Advent of Code](https://adventofcode.com/2024).

## Notes
1. Day 1 has a good example of using `fold` to initialise a `HashMap`.
1. Day 2 part 2 is O(n^2) which is ok for the given input. I suspect O(n) could be achieved using two pointers but I'm going to move on.
1. Day 3 uses `regex`.
1. Day 5 has an example of sorting with a custom predicate.
1. Day 7 might be the most concise AoC solution I've done? The parse fn is a nice split, cast, and collect example. 
1. Day 10, I accidently solved Part 2 first before understanding the rules of Part 1. This day has a nice example of using `fold` to accumulate two results.
1. Day 11 has two different solutions for the same problem. The first, for Part 1, is influenced by the problem description and maintains a vector of pebbles which split and remain in order. Whilst easier to reason about conceptually, it's too inefficient for Part 2. The second solution ignores ordering, as it isn't relevant, and just keeps a count of how many pebbles there are for each value.
