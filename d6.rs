pub fn parse_t(line: &str) -> impl Iterator<Item = f32> + '_ {
    line.split(' ').filter_map(|x| x.parse().ok())
}

pub fn run(input: &str) -> u32 {
    let mut lines = input.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let line1 = parse_t(line1);
    let line2 = parse_t(line2);
    line1
        .zip(line2)
        .map(|(time, record_distance)| {
            // let a = 1;
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
