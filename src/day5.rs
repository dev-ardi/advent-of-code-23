use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;
use utils::{line_vec, parse_line};

#[derive(Debug)]
struct Thing {
    to: u32,
    from: u32,
    range: u32,
}

struct Parsed {
    seeds: Vec<u32>,
    maps: Vec<Vec<Thing>>,
}

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines().filter(|x| !x.is_empty());
    let seeds = lines.next().unwrap().trim_start_matches("seeds: ");
    let seeds = line_vec(seeds);
    let mut maps = vec![];
    let mut curr = vec![];
    lines.next(); // skip first
    for line in lines {
        if line.contains("map") {
            maps.push(curr);
            curr = vec![];
            continue;
        }
        let mut parsed_line = parse_line(line);
        let thing = Thing {
            to: parsed_line.next().unwrap(),
            from: parsed_line.next().unwrap(),
            range: parsed_line.next().unwrap(),
        };
        curr.push(thing);
    }
    maps.push(curr);
    assert_eq!(maps.len(), 7);
    Parsed { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    let input = parse(input);
    input
        .seeds
        .iter()
        .map(|&seed| {
            input.maps.iter().fold(seed, |current, things| {
                things
                    .iter()
                    .find_map(|&Thing { to, from, range }| {
                        (current >= from && current < from + range).then(|| to + (current - from))
                    })
                    .unwrap_or(current)
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    let input = parse(input);
    input
        .seeds
        .into_par_iter()
        .chunks(2)
        .map(|a| (a[0]..a[0] + a[1]))
        .flatten()
        .map(|seed| {
            input.maps.iter().fold(seed, |current, things| {
                things
                    .iter()
                    .find_map(|&Thing { to, from, range }| {
                        (current >= from && current < from + range).then(|| to + (current - from))
                    })
                    .unwrap_or(current)
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = 35;

        assert_eq!(part1(&input), output);
    }

    #[test]
    fn part2_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = 46;

        assert_eq!(part2(&input), output);
    }
}
