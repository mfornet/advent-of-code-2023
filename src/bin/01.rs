use std::collections::HashMap;

use aho_corasick::AhoCorasick;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut it = line
                    .chars()
                    .filter_map(|c| c.to_string().parse::<u32>().ok());
                let first = it.next().unwrap();
                let last = it.last().unwrap_or(first);
                first * 10 + last
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);

    let ac = AhoCorasick::new(digits.keys()).unwrap();

    Some(
        input
            .lines()
            .map(|line| {
                let mut it = ac
                    .find_overlapping_iter(line)
                    .map(|m| digits.get(&line[m.start()..m.end()]).copied().unwrap());

                let first = it.next().unwrap();
                let last = it.last().unwrap_or(first);

                first * 10 + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
