use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|input| {
            let x = input.chars().find_map(|x| char::to_digit(x, 10)).unwrap();
            let y = input
                .chars()
                .rev()
                .find_map(|x| char::to_digit(x, 10))
                .unwrap();
            x * 10 + y
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    fn find<'a>(input: &'a str, numbers: &[&'a str]) -> &'a str {
        for count in 0..input.len() {
            for n in numbers {
                if count + n.len() > input.len() {
                    continue;
                }

                let sl1 = &input[count..count + n.len()];
                if sl1 == *n {
                    return n;
                }
            }
        }
        panic!("Invalid input");
    }

    const NUMBERS: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    const REVERSE_NUMS: [&str; 18] = [
        "enin", "thgie", "neves", "xis", "evif", "ruof", "eerht", "owt", "eno", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    input
        .lines()
        .map(|input| {
            let n0 = find(input, &NUMBERS);

            let input_rev = input.chars().rev().collect::<String>();
            let n1 = find(&input_rev, &REVERSE_NUMS);

            let n0 = match n0 {
                "one" | "1" => 1,
                "two" | "2" => 2,
                "three" | "3" => 3,
                "four" | "4" => 4,
                "five" | "5" => 5,
                "six" | "6" => 6,
                "seven" | "7" => 7,
                "eight" | "8" => 8,
                "nine" | "9" => 9,
                _ => panic!(),
            };

            let n1 = match n1 {
                "enin" | "9" => 9,
                "thgie" | "8" => 8,
                "neves" | "7" => 7,
                "xis" | "6" => 6,
                "evif" | "5" => 5,
                "ruof" | "4" => 4,
                "eerht" | "3" => 3,
                "owt" | "2" => 2,
                "eno" | "1" => 1,
                _ => panic!(),
            };
            n0 * 10 + n1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn part2_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }
}
