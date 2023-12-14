use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(14);

fn tilt_north(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    let m = board[0].len();

    for j in 0..m {
        let mut free = 0;
        for i in 0..n {
            match board[i][j] {
                'O' => {
                    board[i][j] = '.';
                    board[free][j] = 'O';
                    free += 1;
                }
                '#' => {
                    free = i + 1;
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn rotate_inplace(board: &mut Vec<Vec<char>>, tmp: &mut Vec<Vec<char>>) {
    let n = board.len();

    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            tmp[j][n - i - 1] = *c;
        }
    }

    std::mem::swap(board, tmp);
}

fn eval(board: &[Vec<char>]) -> usize {
    let n = board.len();
    board
        .iter()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|&&c| c == 'O').count() * (n - i))
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    tilt_north(&mut board);
    Some(eval(&board))
}

pub fn hash_board(board: &[Vec<char>]) -> usize {
    const MOD: usize = 1_000_000_007;
    const BASE: usize = 3;

    board
        .iter()
        .flatten()
        .fold((0, 1), |(mut hash, power), f| {
            if *f == 'O' {
                hash = (hash + power) % MOD;
            }
            (hash, power * BASE % MOD)
        })
        .0
}

pub fn part_two(input: &str) -> Option<usize> {
    const TARGET: usize = 1_000_000_000;

    let mut board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut tmp = vec![vec!['.'; board.len()]; board[0].len()];
    let mut seen = HashMap::new();
    let mut step = 0;

    while step < TARGET {
        for _ in 0..4 {
            tilt_north(&mut board);
            rotate_inplace(&mut board, &mut tmp);
        }

        step += 1;
        let hash = hash_board(&board);
        let prev = *seen.entry(hash).or_insert(step);

        if prev < step {
            let cycle = step - prev;
            let full_cycles = (TARGET - step) / cycle;
            step += full_cycles * cycle;
        }
    }

    Some(eval(&board))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
