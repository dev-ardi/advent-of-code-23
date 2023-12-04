use std::collections::{HashMap};

use aoc_runner_derive::{aoc};

struct State {
    num: Option<u32>,
    has_touched: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum Square {
    Num(u8),
    Symbol,
    Empty,
}

impl From<u8> for Square {
    fn from(value: u8) -> Self {
        let value = char::from(value);
        if value == '\n' {
            panic!("Newline is not a square");
        }
        if value.is_digit(10) {
            Square::Num(value.to_digit(10).unwrap() as u8)
        } else if value == '.' {
            Square::Empty
        } else {
            Square::Symbol
        }
    }
}

#[aoc(day3, part1)]
fn part1(input: &[u8]) -> u32 {
    let mut state = State {
        num: None,
        has_touched: false,
    };

    let line_width = (input.iter().position(|x| b'\n' == *x).unwrap() + 1) as isize; // +1 for the newline

    let rel = |offset_x: isize, offset_y: isize, x0: usize, y0: usize| {
        let x = x0 as isize + offset_x;
        let y = y0 as isize + offset_y;

        let idx = x + y * line_width;
        if x < 0 || x >= line_width || y < 0 || idx >= input.len() as isize || idx < 0 {
            return false;
        }

        let square: Square = input[idx as usize].into();
        matches!(square, Square::Symbol)
    };

    let input_iter = input
        .split(|a| b'\n' == *a)
        .map(|x| x.iter().map(|x| Square::from(*x)));

    let mut total = 0;
    for (y, line) in input_iter.into_iter().enumerate() {
        for (x, square) in line.into_iter().enumerate() {
            let rel = |offset_x, offset_y| rel(offset_x, offset_y, x, y);
            match state.num {
                Some(ref mut num) => {
                    // We're a number
                    match square {
                        Square::Num(square_n) => {
                            // Next one is a number
                            *num = *num * 10 + square_n as u32;

                            if !state.has_touched {
                                state.has_touched = rel(0, -1) || // above
                                rel(0, 1); // below
                                if state.has_touched {}
                            }
                        }
                        _ => {
                            // Not a number anymore
                            if !state.has_touched {
                                state.has_touched = rel(0, -1) || rel(0, 1) || rel(0, 0);
                                if state.has_touched {}
                            }
                            if state.has_touched {
                                total += *num;
                            }
                            state.num = None;
                            state.has_touched = false;
                        }
                    }
                }
                None => {
                    if let Square::Num(n) = square {
                        // We're becoming a number!
                        state.num = Some(n as u32);
                        state.has_touched = rel(-1,-1) || // top left
                        rel(-1,0)  || // left
                        rel(-1,1) || // bottom left
                        rel(0, 1)  || // below
                        rel(0, -1); // above
                    }
                }
            };
        }
    }
    total
}

#[aoc(day3, part2)]
fn part2(input: &[u8]) -> u32 {
    #[derive(Debug, PartialEq, Eq)]
    enum Square {
        Num(u8),
        Symbol,
        Empty,
    }

    impl From<u8> for Square {
        fn from(value: u8) -> Self {
            let value = char::from(value);
            if value == '\n' {
                panic!("Newline is not a square");
            }
            if value.is_digit(10) {
                Square::Num(value.to_digit(10).unwrap() as u8)
            } else if value == '*' {
                Square::Symbol
            } else {
                Square::Empty
            }
        }
    }

    struct State {
        num: Option<u32>,
        gear: Option<(usize, usize)>,
    }

    let mut state = State {
        num: None,
        gear: None,
    };

    let line_width = (input.iter().position(|x| b'\n' == *x).unwrap() + 1) as isize; // +1 for the newline

    let rel = |offset_x: isize, offset_y: isize, x0: usize, y0: usize| {
        let x = x0 as isize + offset_x;
        let y = y0 as isize + offset_y;

        let idx = x + y * line_width;
        if x < 0 || x >= line_width || y < 0 || idx >= input.len() as isize || idx < 0 {
            return false;
        }

        matches!(input[idx as usize].into(), Square::Symbol)
    };

    #[derive(Debug)]
    struct Pair {
        value: u32,
        occurrences: u8,
    }
    let mut gears = HashMap::<(usize, usize), Pair>::new();

    let input_iter = input
        .split(|a| b'\n' == *a)
        .map(|x| x.iter().map(|x| Square::from(*x)));

    for (y, line) in input_iter.into_iter().enumerate() {
        for (x, square) in line.into_iter().enumerate() {
            let rel = |offset_x, offset_y| rel(offset_x, offset_y, x, y);
            match state.num {
                Some(ref mut num) => {
                    // We're a number
                    match square {
                        Square::Num(square_n) => {
                            // Next one is a number
                            *num = *num * 10 + square_n as u32;

                            if state.gear.is_none() {
                                if rel(0, -1) {
                                    state.gear = Some((x, y - 1))
                                } else if rel(0, 1) {
                                    state.gear = Some((x, y + 1))
                                }
                            }
                        }
                        _ => {
                            // Not a number anymore
                            if state.gear.is_none() {
                                //state.has_touched = rel(0, -1) || rel(0, 1) || rel(0, 0);
                                if rel(0, -1) {
                                    state.gear = Some((x, y - 1))
                                } else if rel(0, 1) {
                                    state.gear = Some((x, y + 1))
                                } else if rel(0, 0) {
                                    state.gear = Some((x, y))
                                }
                            }
                            if let Some(gear) = state.gear {
                                gears
                                    .entry(gear)
                                    .and_modify(|Pair { value, occurrences }| {
                                        *value *= *num;
                                        *occurrences += 1;
                                    })
                                    .or_insert(Pair {
                                        value: *num,
                                        occurrences: 1,
                                    });
                            }
                            state.num = None;
                            state.gear = None;
                        }
                    }
                }
                None => {
                    if let Square::Num(n) = square {
                        // We're becoming a number!
                        state.num = Some(n as u32);

                        if rel(-1, -1) {
                            state.gear = Some((x - 1, y - 1))
                        } else if rel(-1, 0) {
                            state.gear = Some((x - 1, y))
                        } else if rel(-1, 1) {
                            state.gear = Some((x - 1, y + 1))
                        } else if rel(0, 1) {
                            state.gear = Some((x, y + 1))
                        } else if rel(0, -1) {
                            state.gear = Some((x, y - 1))
                        }
                    }
                }
            };
        }
    }
    gears
        .values()
        .filter_map(|Pair { occurrences, value }| (*occurrences != 1).then(|| value))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols() {
        let input = "467..114..*";
        let output = [
            Square::Num(4),
            Square::Num(6),
            Square::Num(7),
            Square::Empty,
            Square::Empty,
            Square::Num(1),
            Square::Num(1),
            Square::Num(4),
            Square::Empty,
            Square::Empty,
            Square::Symbol,
        ];
        assert_eq!(
            input
                .as_bytes()
                .iter()
                .map(|x| Square::from(*x))
                .zip(output.iter())
                .all(|(a, b)| a == *b),
            true
        );
    }

    #[test]
    fn part1_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = 4361;
        assert_eq!(part1(input.as_bytes()), output);
    }

    #[test]
    fn part2_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = 467835;
        assert_eq!(part2(input.as_bytes()), output);
    }
}
