use aoc_runner_derive::aoc;

enum Balls {
    Red(u64),
    Blue(u64),
    Green(u64),
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u64 {
    let input = input.lines().map(|l| {
        l.split(';').map(|input| {
            input.split(", ").map(|str| {
                let mut split = str.trim().split(' ');
                let num = split.next().unwrap().parse::<u64>().unwrap();
                match split.next().unwrap() {
                    "red" => Balls::Red(num),
                    "green" => Balls::Green(num),
                    "blue" => Balls::Blue(num),
                    _ => panic!("unexpected color name"),
                }
            })
        })
    });

    let mut sum = 0;
    'game: for (game_num, game) in input.into_iter().enumerate() {
        for set in game {
            for balls in set {
                match balls {
                    Balls::Red(n) => {
                        if n > 12 {
                            continue 'game;
                        }
                    }
                    Balls::Green(n) => {
                        if n > 13 {
                            continue 'game;
                        }
                    }
                    Balls::Blue(n) => {
                        if n > 14 {
                            continue 'game;
                        }
                    }
                }
            }
        }
        sum += game_num + 1;
    }
    sum as u64
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u64 {
    let input = input.lines().enumerate().map(|(n, l)| {
        l.split(';').map(|input| {
            input.split(", ").map(|str| {
                let mut split = str.trim().split(' ');
                let num = split.next().unwrap().parse::<u64>().unwrap();
                match split.next().unwrap() {
                    "red" => Balls::Red(num),
                    "green" => Balls::Green(num),
                    "blue" => Balls::Blue(num),
                    _ => panic!("unexpected color name"),
                }
            })
        })
    });

    let mut powers = 0u64;

    for (game_num, game) in input.into_iter().enumerate() {
        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        for set in game {
            for balls in set {
                match balls {
                    Balls::Red(n) => {
                        red_min = red_min.max(n);
                    }
                    Balls::Green(n) => {
                        green_min = green_min.max(n);
                    }
                    Balls::Blue(n) => {
                        blue_min = blue_min.max(n);
                    }
                }
            }
        }
        powers += red_min * green_min * blue_min;
    }
    powers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part1(&input);
        assert_eq!(output, 8);
    }

    #[test]
    fn part2_example() {
        let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part2(&input);
        assert_eq!(output, 2286);
    }
}
