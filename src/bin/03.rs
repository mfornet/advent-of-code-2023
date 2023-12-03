use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let board = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let width = board[0].len();
    let height = board.len();

    assert!(board.iter().all(|line| line.len() == width));

    let number = Regex::new(r"\d+").unwrap();

    Some(
        board
            .iter()
            .enumerate()
            .flat_map(|(row, line)| number.find_iter(line).map(move |m| (row, m)))
            .filter_map(|(row, number)| {
                let x0 = row.saturating_sub(1);
                let x1 = std::cmp::min(height, row + 2);
                let y0 = number.start().saturating_sub(1);
                let y1 = std::cmp::min(width, number.end() + 1);

                for row in board.iter().take(x1).skip(x0) {
                    for y in y0..y1 {
                        let cell = row[y..y + 1].chars().next().unwrap();
                        if !matches!(cell, '.' | '0'..='9') {
                            let number = number.as_str().parse::<u32>().unwrap();
                            return Some(number);
                        }
                    }
                }

                None
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let width = board[0].len();
    let height = board.len();

    assert!(board.iter().all(|line| line.len() == width));

    let number = Regex::new(r"\d+").unwrap();

    Some(
        board
            .iter()
            .enumerate()
            .flat_map(|(row, line)| number.find_iter(line).map(move |m| (row, m)))
            .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (row, number)| {
                let x0 = row.saturating_sub(1);
                let x1 = std::cmp::min(height, row + 2);
                let y0 = number.start().saturating_sub(1);
                let y1 = std::cmp::min(width, number.end() + 1);

                for (x, row) in board.iter().enumerate().take(x1).skip(x0) {
                    for y in y0..y1 {
                        if &row[y..y + 1] == "*" {
                            acc.entry((x, y))
                                .or_default()
                                .push(number.as_str().parse::<u32>().unwrap());
                        }
                    }
                }

                acc
            })
            .values()
            .filter_map(|values| {
                if let [a, b] = values.as_slice() {
                    Some(*a * *b)
                } else {
                    None
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
