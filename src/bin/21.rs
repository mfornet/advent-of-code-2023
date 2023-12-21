use std::collections::{HashSet, VecDeque};

use advent_of_code::RotateInPlace;
use itertools::Itertools;

advent_of_code::solution!(21);

fn find_start(board: &[Vec<char>]) -> (usize, usize) {
    let n = board.len();
    let m = board[0].len();
    let mut start = (n, m);

    'find_start: for i in 0..n {
        for j in 0..m {
            if board[i][j] == 'S' {
                start = (i, j);
                break 'find_start;
            }
        }
    }

    assert_ne!(start, (n, m));

    start
}

fn compute_distance(board: &[Vec<char>], source: (usize, usize)) -> Vec<Vec<usize>> {
    let (x, y) = source;
    let n = board.len();
    let m = board[0].len();
    let mut dist = vec![vec![usize::MAX; m]; n];
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    dist[x][y] = 0;

    while let Some((i, j)) = queue.pop_front() {
        let d = dist[i][j];

        for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni < 0 || ni >= n as isize || nj < 0 || nj >= m as isize {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if board[ni][nj] == '#' {
                continue;
            }

            if dist[ni][nj] > d + 1 {
                dist[ni][nj] = d + 1;
                queue.push_back((ni, nj));
            }
        }
    }

    dist
}

fn solve(input: &str, distance: usize) -> usize {
    let mut board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let n = board.len();
    let m = board[0].len();
    let (x, y) = find_start(&mut board);

    let dist = compute_distance(&board, (x, y));

    let mut answer = 0;
    for i in 0..n {
        for j in 0..m {
            if dist[i][j] <= distance && dist[i][j] % 2 == distance % 2 {
                answer += 1;
            }
        }
    }

    answer
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 64))
}

fn solve_corner_up_right(board: &[Vec<char>], distance: usize) -> usize {
    let n = board.len();
    let m = board[0].len();

    let src_to_corner = {
        let src = find_start(&board);
        let dist = compute_distance(&board, src);
        dist[0][m - 1]
    };

    // Distance to the corner is even, so parity doesn't change
    assert_eq!(src_to_corner % 2, 0);

    let dist_from_corner = compute_distance(&board, (n - 1, 0));

    // Distance between corners is manhattan distance
    assert_eq!(dist_from_corner[0][m - 1], n + m - 2);

    let mut count_up_to_dist = vec![0usize; n * m];
    for i in 0..n {
        for j in 0..m {
            if dist_from_corner[i][j] != usize::MAX {
                count_up_to_dist[dist_from_corner[i][j]] += 1;
            }
        }
    }
    while count_up_to_dist.last() == Some(&0) {
        count_up_to_dist.pop();
    }
    for i in 2..count_up_to_dist.len() {
        count_up_to_dist[i] += count_up_to_dist[i - 2];
    }

    let longest_distance = count_up_to_dist.len() - 1;

    let mut answer = 0;

    for delta in 1.. {
        let mut lo = 0;
        let mut hi = 1;

        let mut cur = 0;

        loop {
            let dist_to_farthest =
                src_to_corner + 2 + (delta - 1) * m + (hi - 1) * n + longest_distance;

            if dist_to_farthest <= distance {
                lo = hi;
                hi *= 2;
            } else {
                break;
            }
        }

        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            let dist_to_farthest =
                src_to_corner + 2 + (delta - 1) * m + (mid - 1) * n + longest_distance;

            if dist_to_farthest <= distance {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let len = count_up_to_dist.len();
        let (even, odd) = if len % 2 == 0 {
            (count_up_to_dist[len - 1], count_up_to_dist[len - 2])
        } else {
            (count_up_to_dist[len - 2], count_up_to_dist[len - 1])
        };

        let (covered_even, covered_odd) = if delta % 2 == distance % 2 {
            (lo - lo / 2, lo / 2)
        } else {
            (lo / 2, lo - lo / 2)
        };

        cur += even * covered_even + odd * covered_odd;

        loop {
            let dist_to_corner = src_to_corner + 2 + (delta - 1) * m + lo * n;
            if dist_to_corner <= distance {
                cur += count_up_to_dist[distance - dist_to_corner];
            } else {
                break;
            }
            lo += 1;
        }

        // println!(
        //     "delta={} even={} t_even={} odd={} t_odd={} t_cover={} tot={}",
        //     delta,
        //     even,
        //     covered_even,
        //     odd,
        //     covered_odd,
        //     even * covered_even + odd * covered_odd,
        //     cur
        // );

        if cur == 0 {
            break;
        }

        answer += cur;
    }

    // dbg!(answer);
    answer
}

fn solve_up_brute(board: &[Vec<char>], distance: usize) -> usize {
    let (x, y) = find_start(board);
    let n = board.len() as isize;
    let m = board[0].len() as isize;

    let mut queue = VecDeque::new();
    queue.push_back((x as isize, y as isize, 0));
    let mut seen = HashSet::new();
    seen.insert((x as isize, y as isize));
    let mut answer = 0;

    while let Some((x, y, d)) = queue.pop_front() {
        assert!(d <= distance);

        if d % 2 == distance % 2 && x < 0 {
            answer += 1;
        }

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= n || ny < 0 || ny >= m {
                continue;
            }

            let cx = ((nx % n + n) % n) as usize;
            let cy = ny as usize;

            if board[cx][cy] == '#' {
                continue;
            }

            if d < distance && seen.insert((nx, ny)) {
                queue.push_back((nx, ny, d + 1));
            }
        }
    }
    answer
}

fn solve_up(board: &[Vec<char>], distance: usize) -> usize {
    let n = board.len();
    let m = board[0].len();

    let dist = compute_distance(board, (n - 1, m / 2));

    let mut even = 0;
    let mut odd = 0;

    for i in 0..n {
        for j in 0..m {
            if dist[i][j] != usize::MAX {
                if dist[i][j] % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
        }
    }

    let first_center = n / 2 + 1;

    let (mut cur_par, mut next_par) = if distance % 2 == first_center % 2 {
        (even, odd)
    } else {
        (odd, even)
    };

    let farthest = dist
        .iter()
        .flatten()
        .filter(|&&d| d != usize::MAX)
        .max()
        .copied()
        .unwrap();

    let mut answer = 0;
    for delta in 1.. {
        let center = n / 2 + 1 + (delta - 1) * n;
        if center > distance {
            break;
        }

        let mut cur = 0;

        if center + farthest <= distance {
            cur += cur_par;
        } else {
            for i in 0..n {
                for j in 0..m {
                    if dist[i][j] != usize::MAX {
                        let d = dist[i][j] + center;
                        if d <= distance && distance % 2 == d % 2 {
                            cur += 1;
                        }
                    }
                }
            }
        }
        answer += cur;
        std::mem::swap(&mut cur_par, &mut next_par);
    }

    // dbg!(answer)
    answer
}

fn solve_up_right(board: &[Vec<char>], distance: usize, fast: bool) -> usize {
    let up_right = solve_corner_up_right(board, distance);

    let up = if fast {
        solve_up(board, distance)
    } else {
        solve_up_brute(board, distance)
    };
    up_right + up
}

fn solve_part_two(input: &str, distance: usize) -> usize {
    let board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut board = RotateInPlace::new(board);
    let mut answer = 0;
    let mut good = true;

    // Compute distance in the cell (0, 0)
    {
        let src = find_start(&board);
        let dist = compute_distance(&board, src);
        let n = board.len();
        let m = board[0].len();

        for i in 0..n {
            for j in 0..m {
                if dist[i][j] <= distance && dist[i][j] % 2 == distance % 2 {
                    answer += 1;
                }

                if 2 * i + 1 == n || 2 * j + 1 == m {
                    good &= board[i][j] != '#';
                }
            }
        }
    }

    if !good {
        println!("Using slow algorithm");
    }

    for _ in 0..4 {
        answer += solve_up_right(&board, distance, good);
        board.rotate();
    }

    answer
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve_part_two(input, 26501365))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_solve_up() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let board = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let mut board = RotateInPlace::new(board);
        for d in 1.. {
            for _ in 0..4 {
                let found = solve_up(&board, d);
                let expected = solve_up_brute(&board, d);
                println!("{} {} {}", d, found, expected);
                assert_eq!(found, expected, "d = {}", d);
                board.rotate();
            }
        }
    }

    #[test]
    fn test_solve_up_data() {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let board = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let mut board = RotateInPlace::new(board);
        for d in 30001.. {
            for _ in 0..4 {
                let found = solve_up(&board, d);
                let expected = solve_up_brute(&board, d);
                println!("{} {} {}", d, found, expected);
                assert_eq!(found, expected, "d = {}", d);
                board.rotate();
            }
        }
    }

    #[test]
    fn test_part_two() {
        for (dist, expected) in [
            (6, 16),
            (10, 50),
            (50, 1594),
            (100, 6536),
            (500, 167004),
            (1000, 668697),
            (5000, 16733044),
        ] {
            assert_eq!(
                solve_part_two(&advent_of_code::template::read_file("examples", DAY), dist),
                expected,
                "dist = {}",
                dist
            );
        }
    }
}
