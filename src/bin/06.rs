use itertools::Itertools;

advent_of_code::solution!(6);

fn parse(line: &str) -> impl Iterator<Item = u64> + '_ {
    let (_, numbers) = line.split_once(':').unwrap();
    numbers
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
}

fn solve(time: u64, distance: u64) -> u64 {
    let tf = time as f64;
    let df = distance as f64;

    let x0 = (tf - (tf * tf - 4.0 * df).sqrt()) / 2.0;
    let x1 = (tf + (tf * tf - 4.0 * df).sqrt()) / 2.0;
    let mut x0 = x0.ceil() as u64;
    let mut x1 = x1.floor() as u64;

    if (time - x0) * x0 <= distance {
        x0 += 1;
    }

    if (time - x1) * x1 <= distance {
        x1 -= 1;
    }

    x1 - x0 + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let (time, distance) = input.lines().map(parse).next_tuple().unwrap();

    Some(
        time.zip(distance)
            .map(|(t, d)| solve(t, d))
            .product::<u64>(),
    )
}

fn parse_two(line: &str) -> u64 {
    let (_, numbers) = line.split_once(':').unwrap();
    numbers.replace(' ', "").parse::<u64>().unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (t, d) = input.lines().map(parse_two).next_tuple().unwrap();
    Some(solve(t, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
