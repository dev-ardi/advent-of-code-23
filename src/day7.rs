use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid: i64 = bid.parse().unwrap();
            let hand: Vec<usize> = hand
                .chars()
                .map(|c| "23456789TJQKA".find(c).unwrap())
                .collect();
            let counts: Vec<_> = hand
                .iter()
                .copied()
                .counts()
                .into_values()
                .sorted()
                .collect();
            let hand_kind = match counts.as_slice() {
                [1, 1, 1, 1, 1] => HandKind::HighCard,
                [1, 1, 1, 2] => HandKind::OnePair,
                [1, 2, 2] => HandKind::TwoPair,
                [1, 1, 3] => HandKind::ThreeOfAKind,
                [2, 3] => HandKind::FullHouse,
                [1, 4] => HandKind::FourOfAKind,
                [5] => HandKind::FiveOfAKind,
                _ => unreachable!(),
            };
            (hand_kind, hand, bid)
        })
        .sorted()
        .enumerate()
        .map(|(rank, (_, _, bid))| bid * (rank as i64 + 1))
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = 6440;

        assert_eq!(part1(input), output);
    }

    #[test]
    fn day7_part2() {
        let input = "";
        let output = "";

        assert_eq!(part2(input), output);
    }
}
