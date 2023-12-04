use std::rc::Rc;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line.split(':').nth(1).unwrap();
            let mut it = line.split('|').map(|x| {
                x.trim()
                    .split(' ')
                    .map(|x| x.trim())
                    .filter(|x| !x.is_empty())
            });
            let first = it.next().unwrap();
            let second = it.next().unwrap();

            let res = second
                .filter(|num| first.clone().any(|x| x == *num))
                .count() as u32;
            if res == 0 {
                0
            } else {
                1 << res - 1
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let res = input
        .lines()
        .map(|line| {
            let line = line.split(':').nth(1).unwrap();
            let mut it = line.split('|').map(|x| {
                x.trim()
                    .split(' ')
                    .map(|x| x.trim())
                    .filter(|x| !x.is_empty())
            });
            let first = it.next().unwrap();
            let second = it.next().unwrap();

            second
                .filter(|num| first.clone().any(|x| x == *num))
                .count() as usize
        })
        .collect::<Vec<_>>();

    let mut copies = vec![1; res.len()];
    for (n, results) in res.iter().enumerate() {
        for i in 1..*results + 1 {
            copies[n + i] += 1 * copies[n];
        }
    }
    copies.iter().sum()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = 13;
        assert_eq!(part1(input), output);
 
    }

    #[test]
    fn part2_example() {
let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
let output = 30;

        assert_eq!(part2(input), output);
    }
}
