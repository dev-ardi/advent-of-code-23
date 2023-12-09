// use aoc_runner_derive::{aoc, aoc_generator};
// #[aoc_generator(day6)]
// fn parse(input: &str) -> String {
//    todo!()
//}

use aoc_runner_derive::aoc;
use itertools::Itertools;
use utils::{parse_line, parse_t};

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let line1 = parse_t::<f32>(line1);
    let line2 = parse_t::<f32>(line2);
    line1
        .zip(line2)
        .map(|(time, record_distance)| {
            let b = -time;
            let c = record_distance;

            let r = (b * b - 4.0 * c).sqrt();
            let big = (-b + r) / 2.0;
            let small = (-b - r) / 2.0;

            let x = big.ceil();
            let y = small.floor();

            let res = (x - y).abs() as u32;

            if x.fract() == 0.0 {
                res - 1
            } else {
                res
            }
        })
        .fold(1, |acc, e| acc * e)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = 288;

        assert_eq!(part1(input), output);
    }

    #[test]
    fn part2_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = 0;

        assert_eq!(part2(input), output);
    }
}
