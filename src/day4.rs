use aoc_runner_derive::{aoc, aoc_generator};

enum State {
    Card,
    First(u8),
    Second,
}

struct Counter<const T: usize, I> {
    state: State,
    iter: I,
    count: u8,
    lines: Vec<u8>,
    vec: [[u8; 2]; T],
}

impl<const T: usize, I: Iterator<Item = u8>> Counter<T, I> {
    pub fn new(iter: I) -> Self {
        Self {
            state: State::Card,
            iter,
            count: 0,
            lines: Vec::new(),
            vec: [[0u8, 0u8]; T],
        }
    }
    pub fn compute(self) -> Vec<u8> {
        let Self {
            mut state,
            mut iter,
            mut count,
            mut lines,
            mut vec,
        } = self;
        while let Some(c) = iter.next() {
            match state {
                State::Card => {
                    if c == b':' {
                        iter.next();
                        state = State::First(0);
                    }
                }
                State::First(n) => {
                    debug_assert!(n != b' ');
                    vec[n as usize][0] = c;
                    vec[n as usize][1] = iter.next().unwrap();
                    iter.next();
                    state = State::First(n + 1);

                    if n + 1 == T as u8 {
                        assert_eq!(iter.nth(0).unwrap() as char, '|');
                        assert_eq!(iter.nth(0).unwrap() as char, ' ');
                        state = State::Second;
                    }
                }
                State::Second => {
                    let pat = [c, iter.next().unwrap()];
                    let whitespace = iter.next();
                    if vec.iter().any(|x| *x == pat) {
                        count += 1
                    };
                    if whitespace.unwrap_or(b'\n') == b'\n' {
                        vec = [[0u8, 0u8]; T];
                        state = State::Card;
                        lines.push(count);
                        count = 0;
                        continue;
                    }
                }
            }
        }
        lines
    }
}

const MAX: usize = 10;

// #[aoc_generator(day4)]
// fn parse(input: &[u8]) -> Vec<u8> {
//     Counter::<MAX, _>::new(input.iter().copied()).compute()
// }

#[aoc(day4, part1)]
fn part1(input: &[u8]) -> u32 {
    let input = Counter::<MAX, _>::new(input.iter().copied()).compute();
    let lines = input.iter().map(|x| *x as u32);

    lines
        .map(|res| if res == 0 { 0 } else { 1 << res - 1 })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[u8]) -> u32 {
    let input = Counter::<MAX, _>::new(input.iter().copied()).compute();
    let mut copies = vec![1u32; input.len()];
    for (n, results) in input.into_iter().enumerate() {
        for i in 1..results + 1 {
            copies[n + i as usize] += 1 * copies[n];
        }
    }
    copies.iter().sum()
}

// #[cfg(test)]
//
// mod tests {
//     use super::*;
//
//     fn make_input() -> Vec<u8> {
//         let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
//             .as_bytes();
//         Counter::<5, _>::new(input.iter().copied()).compute()
//     }
//     #[test]
//     fn counter() {
//         let output = vec![4, 2, 2, 1, 0, 0];
//         assert_eq!(make_input(), output);
//     }
//
//     #[test]
//     fn part1_example() {
//         let output = 13;
//         assert_eq!(part1(&make_input()), output);
//     }
//
//     #[test]
//     fn part2_example() {
//         let output = 30;
//         assert_eq!(part2(&make_input()), output);
//     }
// }
