use std::collections::VecDeque;

use advent_of_code::Direction;
use itertools::Itertools;

advent_of_code::solution!(16);

struct BoundingBox {
    n: isize,
    m: isize,
}

impl BoundingBox {
    fn new(n: usize, m: usize) -> Self {
        Self {
            n: n as isize,
            m: m as isize,
        }
    }

    fn next(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let (x, y) = pos;
        let x = x as isize;
        let y = y as isize;
        let (x, y) = match dir {
            Direction::Up => (x - 1, y),
            Direction::Left => (x, y - 1),
            Direction::Down => (x + 1, y),
            Direction::Right => (x, y + 1),
        };

        if x < 0 || y < 0 || x >= self.n || y >= self.m {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }
}

fn next_dirs(cell: char, dir: Direction) -> Vec<Direction> {
    match (cell, dir) {
        ('.', dir) => vec![dir],
        ('|', Direction::Down | Direction::Up) => vec![dir],
        ('|', Direction::Left | Direction::Right) => vec![Direction::Up, Direction::Down],
        ('-', Direction::Down | Direction::Up) => vec![Direction::Right, Direction::Left],
        ('-', Direction::Left | Direction::Right) => vec![dir],
        ('\\', Direction::Down) => vec![Direction::Right],
        ('\\', Direction::Up) => vec![Direction::Left],
        ('\\', Direction::Left) => vec![Direction::Up],
        ('\\', Direction::Right) => vec![Direction::Down],
        ('/', Direction::Down) => vec![Direction::Left],
        ('/', Direction::Up) => vec![Direction::Right],
        ('/', Direction::Left) => vec![Direction::Down],
        ('/', Direction::Right) => vec![Direction::Up],
        _ => unreachable!(),
    }
}

fn covered(board: &[Vec<char>], start: ((usize, usize), Direction)) -> usize {
    let n = board.len();
    let m = board[0].len();

    let bbox = BoundingBox::new(n, m);

    let mut seen = vec![vec![vec![false; 4]; m]; n];
    let mut queue = VecDeque::new();

    let ((x, y), dir) = start;

    for dir in next_dirs(board[x][y], dir) {
        seen[x][y][dir.index()] = true;
        queue.push_back(((x, y), dir));
    }

    while let Some(((x, y), dir)) = queue.pop_front() {
        if let Some((nx, ny)) = bbox.next((x, y), dir) {
            for ndir in next_dirs(board[nx][ny], dir) {
                if !seen[nx][ny][ndir.index()] {
                    seen[nx][ny][ndir.index()] = true;
                    queue.push_back(((nx, ny), ndir));
                }
            }
        }
    }

    seen.into_iter()
        .flatten()
        .filter(|x| x.iter().any(|x| *x))
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    Some(covered(&board, ((0, 0), Direction::Right)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let n = board.len();
    let m = board[0].len();
    let mut answer = 0;

    for i in 0..n {
        let left = covered(&board, ((i, 0), Direction::Right));
        answer = std::cmp::max(answer, left);

        let right = covered(&board, ((i, m - 1), Direction::Left));
        answer = std::cmp::max(answer, right);
    }

    for j in 0..m {
        let up = covered(&board, ((0, j), Direction::Down));
        answer = std::cmp::max(answer, up);

        let down = covered(&board, ((n - 1, j), Direction::Up));
        answer = std::cmp::max(answer, down);
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
