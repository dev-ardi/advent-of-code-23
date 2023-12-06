use aoc_runner_derive::{aoc, aoc_generator};

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

fn parse_line(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.split(' ').filter_map(|x| x.parse().ok())
}
fn line_vec(line: &str) -> Vec<u32> {
    parse_line(line).collect()
}

#[aoc_generator(day5)]
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
fn part1(input: &Parsed) -> u32 {
    input
        .seeds
        .iter()
        .copied()
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

#[aoc(day5, part2)]
fn part2(input: &Parsed) -> u32 {
    todo!()
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

        let input = parse(input);
        assert_eq!(part1(&input), output);
    }

    // #[test]
    // fn part2_example() {
    //     let input = "";
    //     let output = 0;
    //
    //     let input = parse(input);
    //     assert_eq!(part2(&input), output);
    // }
}
