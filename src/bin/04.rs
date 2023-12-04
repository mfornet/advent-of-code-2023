use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(4);

struct ScratchPad(u32);

impl FromStr for ScratchPad {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, data) = s.split_once(':').unwrap();

        let (target, given) = data.split_once('|').unwrap();

        let target = target
            .split(' ')
            .filter_map(|number| number.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let total = given
            .split(' ')
            .filter_map(|number| number.parse::<u32>().ok())
            .filter(|number| target.contains(number))
            .count() as u32;

        Ok(Self(total))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let total = line.parse::<ScratchPad>().unwrap().0;
                if total == 0 {
                    0
                } else {
                    1 << (total - 1)
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut interval_end = vec![0];

    Some(
        input
            .lines()
            .enumerate()
            .fold((0, 1), |(mut total, mut copies), (i, line)| {
                let cur = line.parse::<ScratchPad>().unwrap().0 as usize;
                total += copies;

                if interval_end.len() <= i + cur {
                    interval_end.resize(i + cur + 1, 0);
                }

                if cur > 0 {
                    interval_end[i + cur] += copies;
                    copies += copies;
                }
                copies -= interval_end[i];

                (total, copies)
            })
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
