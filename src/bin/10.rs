use std::{collections::HashSet, vec};

use itertools::Itertools;

advent_of_code::solution!(10);

type Graph = Vec<Vec<Vec<(usize, usize)>>>;

fn go(
    sx: usize,
    sy: usize,
    tx: usize,
    ty: usize,
    graph: &mut [Vec<Vec<(usize, usize)>>],
) -> Option<Vec<(usize, usize)>> {
    let mut x = sx;
    let mut y = sy;
    let mut px = tx;
    let mut py = ty;

    let mut the_loop = vec![(tx, ty)];

    while (x, y) != (tx, ty) {
        the_loop.push((x, y));

        let mut found = false;

        for &(nx, ny) in &graph[x][y] {
            if (nx, ny) == (px, py) {
                continue;
            }

            if (nx, ny) == (tx, ty) || graph[nx][ny].contains(&(x, y)) {
                found = true;
                px = x;
                py = y;
                x = nx;
                y = ny;
                break;
            }
        }

        if !found {
            return None;
        }
    }

    graph[tx][ty].push((sx, sy));
    graph[tx][ty].push((px, py));

    Some(the_loop)
}

fn build_graph(input: &str) -> (Graph, (usize, usize)) {
    let board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let n = board.len();
    let m = board[0].len();

    let mut graph = vec![vec![vec![]; m]; n];
    let mut start = None;

    for (i, j) in (0..n).cartesian_product(0..m) {
        if i + 1 < n && matches!(board[i][j], '|' | '7' | 'F') {
            graph[i][j].push((i + 1, j));
        }

        if i > 0 && matches!(board[i][j], '|' | 'L' | 'J') {
            graph[i][j].push((i - 1, j));
        }

        if j + 1 < m && matches!(board[i][j], '-' | 'L' | 'F') {
            graph[i][j].push((i, j + 1));
        }

        if j > 0 && matches!(board[i][j], '-' | 'J' | '7') {
            graph[i][j].push((i, j - 1));
        }

        if board[i][j] == 'S' {
            assert!(start.is_none());
            start = Some((i, j));
        }
    }

    (graph, start.unwrap())
}

fn find_loop(
    graph: &mut Vec<Vec<Vec<(usize, usize)>>>,
    sx: usize,
    sy: usize,
) -> Vec<(usize, usize)> {
    let n = graph.len();
    let m = graph[0].len();

    let mut the_loop = None;

    for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nx = sx as i32 + dx;
        let ny = sy as i32 + dy;

        if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if !graph[nx][ny].contains(&(sx, sy)) {
            continue;
        }

        if let Some(found_loop) = go(nx, ny, sx, sy, graph) {
            the_loop = Some(found_loop);
            break;
        }
    }

    the_loop.unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut graph, (sx, sy)) = build_graph(input);
    Some(find_loop(&mut graph, sx, sy).len() / 2)
}

enum WindDir {
    Up,
    Down,
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut graph, (sx, sy)) = build_graph(input);
    let the_loop = find_loop(&mut graph, sx, sy)
        .into_iter()
        .collect::<HashSet<_>>();

    let mut answer = 0;
    let n = graph.len();
    let m = graph[0].len();

    let has_up = |x: usize, y: usize| -> bool { x > 0 && graph[x][y].contains(&(x - 1, y)) };
    let has_down = |x: usize, y: usize| -> bool { x + 1 < n && graph[x][y].contains(&(x + 1, y)) };

    for x in 0..n {
        let mut wind = 0;
        let mut last = None;

        for y in 0..m {
            if the_loop.contains(&(x, y)) {
                let up = has_up(x, y);
                let down = has_down(x, y);

                match (up, down, last) {
                    (true, true, None) => {
                        last = None;
                        wind ^= 1;
                    }
                    (false, true, None) => last = Some(WindDir::Down),
                    (false, true, Some(WindDir::Up)) => {
                        wind ^= 1;
                        last = None;
                    }
                    (false, true, Some(WindDir::Down)) => last = None,
                    (true, false, None) => last = Some(WindDir::Up),
                    (true, false, Some(WindDir::Down)) => {
                        wind ^= 1;
                        last = None;
                    }
                    (true, false, Some(WindDir::Up)) => last = None,
                    (false, false, prev) => last = prev,
                    _ => unreachable!(),
                }
            } else {
                answer += wind;
            }
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let tests = &[(1, 4), (2, 4), (3, 8), (4, 8)];

        for (part, expected) in tests {
            let result = part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, *part,
            ));
            assert_eq!(result, Some(*expected));
        }
    }

    #[test]
    fn test_part_two() {
        let tests = &[(5, 4), (6, 4), (7, 8), (8, 10)];

        for (part, expected) in tests {
            let result = part_two(&advent_of_code::template::read_file_part(
                "examples", DAY, *part,
            ));
            assert_eq!(result, Some(*expected));
        }
    }
}
