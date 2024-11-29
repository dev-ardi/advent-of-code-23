use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day8)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> String {
    input.to_string()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#""#.to_string();
        let output = r#""#.to_string();

        let input = parse(input);
        assert_eq!(part1(input), output);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        let input = r#""#.to_string();
        let output = r#""#.to_string();

        let input = parse(input);
        assert_eq!(part2(input), output);
    }
}
use super::*;
