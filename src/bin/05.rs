use std::{collections::BTreeMap, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct MapRange {
    updated_ranges: BTreeMap<u64, (u64, u64)>,
}

impl MapRange {
    fn convert(&self, value: u64) -> u64 {
        if let Some((&src, &(dst, len))) = self.updated_ranges.range(..=value).next_back() {
            assert!(src <= value);
            if value < src + len {
                return dst + (value - src);
            }
        }
        value
    }

    fn convert_range(&self, mut range: (u64, u64)) -> Vec<(u64, u64)> {
        let mut answer = vec![];
        for (&src, &(dst, len)) in self.updated_ranges.iter() {
            if range.0 == range.1 {
                break;
            }

            if range.0 < src {
                let end = std::cmp::min(src, range.1);
                answer.push((range.0, end));
                range.0 = end;
            }

            if range.0 == range.1 {
                break;
            }

            assert!(src <= range.0);

            if range.0 < src + len {
                let end = std::cmp::min(src + len, range.1);
                answer.push((dst + (range.0 - src), dst + (end - src)));
                range.0 = end;
            }
        }

        if range.0 < range.1 {
            answer.push(range);
        }

        answer
    }
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, data) = s.split_once(':').unwrap();

        Ok(Self {
            updated_ranges: data
                .trim_start_matches('\n')
                .split('\n')
                .map(|ranges| {
                    let (dst, src, len) = ranges
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .next_tuple()
                        .unwrap();

                    (src, (dst, len))
                })
                .collect::<BTreeMap<_, _>>(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.trim_end_matches('\n');
    let mut it = input.split("\n\n");
    let seeds = it.next().unwrap();
    let seeds = seeds
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    it.fold(seeds, |seeds, line| {
        let ranges = line.parse::<MapRange>().unwrap();
        seeds
            .into_iter()
            .map(|seed| ranges.convert(seed))
            .collect::<Vec<_>>()
    })
    .into_iter()
    .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.trim_end_matches('\n');
    let mut it = input.split("\n\n");
    let seeds = it.next().unwrap();
    let seeds = seeds
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut c| {
            let (start, len) = c.next_tuple().unwrap();
            (start, start + len)
        })
        .collect::<Vec<_>>();

    it.fold(seeds, |seeds, line| {
        let ranges = line.parse::<MapRange>().unwrap();
        seeds
            .into_iter()
            .flat_map(|seed| ranges.convert_range(seed))
            .collect::<Vec<_>>()
    })
    .into_iter()
    .map(|(start, _)| start)
    .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
