use advent_of_code::{Direction, NoCompare};
use itertools::Itertools;
use std::cmp::Reverse;

advent_of_code::solution!(17);

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct State {
    pos: (usize, usize),
    required: usize,
    remaining: usize,
    dir: Direction,
}

pub fn solve(board: &[Vec<u32>], minimum: usize, maximum: usize) -> u32 {
    let n = board.len();
    let m = board[0].len();

    let mut distance = vec![vec![vec![vec![u32::MAX; 4]; maximum + 1]; m]; n];
    let mut heap = std::collections::BinaryHeap::new();

    for dir in [Direction::Right, Direction::Down] {
        let state = State {
            pos: (0, 0),
            required: minimum,
            remaining: maximum,
            dir,
        };
        distance[0][0][maximum][dir.index()] = 0;
        heap.push(Reverse((0, NoCompare(state))));
    }

    while let Some(Reverse((d, NoCompare(state)))) = heap.pop() {
        let State {
            pos: (x, y),
            required,
            remaining,
            dir,
        } = state;

        if distance[x][y][remaining][dir.index()] < d {
            continue;
        }

        if x == n - 1 && y == m - 1 && required == 0 {
            return d;
        }

        for (ndir, (dx, dy)) in Direction::all() {
            if ndir == &dir.opposite() {
                continue;
            }

            if required > 0 && ndir != &dir {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            let nd = d + board[nx][ny];

            let n_state = if *ndir == dir {
                (remaining > 0).then(|| State {
                    pos: (nx, ny),
                    required: required.saturating_sub(1),
                    remaining: remaining - 1,
                    dir: *ndir,
                })
            } else {
                Some(State {
                    pos: (nx, ny),
                    required: minimum.saturating_sub(1),
                    remaining: maximum.saturating_sub(1),
                    dir: *ndir,
                })
            };

            if let Some(n_state) = n_state {
                if nd < distance[nx][ny][n_state.remaining][n_state.dir.index()] {
                    distance[nx][ny][n_state.remaining][n_state.dir.index()] = nd;
                    heap.push(Reverse((nd, NoCompare(n_state))));
                }
            }
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    Some(solve(&board, 0, 3))
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    Some(solve(&board, 4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(94));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
