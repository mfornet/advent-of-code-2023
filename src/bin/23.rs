use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(23);

type Graph = HashMap<(usize, usize), Vec<((usize, usize), u32)>>;

pub fn longest_path_acyclic(
    src: (usize, usize),
    target: (usize, usize),
    graph: &Graph,
    cache: &mut HashMap<(usize, usize), Option<u32>>,
) -> Option<u32> {
    if let Some(&result) = cache.get(&src) {
        return result;
    }

    let mut result = None;

    if src == target {
        result = Some(0);
    }

    for &(dst, d) in &graph[&src] {
        if let Some(n_dist) = longest_path_acyclic(dst, target, graph, cache) {
            result = result.max(Some(d + n_dist));
        }
    }

    cache.insert(src, result);
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    assert!(board[0][1] == '.');
    board[0][1] = 'v';
    let n = board.len();
    let m = board[0].len();
    assert_eq!(board[n - 1][m - 2], '.');
    board[n - 1][m - 2] = 'v';

    let mut graph = HashMap::new();
    graph.insert((n - 1, m - 2), vec![]);
    let mut highlights = vec![(0, 1)];

    while let Some((x, y)) = highlights.pop() {
        let mut neighbors = vec![];

        let mut visited = vec![vec![false; m]; n];
        let mut queue = VecDeque::new();

        visited[x][y] = true;

        match board[x][y] {
            'v' => {
                queue.push_back((x + 1, y, 1));
                visited[x + 1][y] = true;
            }
            '^' => {
                queue.push_back((x - 1, y, 1));
                visited[x - 1][y] = true;
            }
            '>' => {
                queue.push_back((x, y + 1, 1));
                visited[x][y + 1] = true;
            }
            '<' => {
                queue.push_back((x, y - 1, 1));
                visited[x][y - 1] = true;
            }
            _ => unreachable!(),
        }

        while let Some((x, y, d)) = queue.pop_front() {
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                match board[nx][ny] {
                    '#' => continue,
                    '.' => {
                        if !visited[nx][ny] {
                            visited[nx][ny] = true;
                            queue.push_back((nx, ny, d + 1));
                        }
                    }
                    'v' => {
                        if dx == 1 && dy == 0 {
                            neighbors.push(((nx, ny), d + 1));
                        }
                    }
                    '^' => {
                        if dx == -1 && dy == 0 {
                            neighbors.push(((nx, ny), d + 1));
                        }
                    }
                    '>' => {
                        if dx == 0 && dy == 1 {
                            neighbors.push(((nx, ny), d + 1));
                        }
                    }
                    '<' => {
                        if dx == 0 && dy == -1 {
                            neighbors.push(((nx, ny), d + 1));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        for (h, _) in &neighbors {
            if !graph.contains_key(h) {
                highlights.push(*h);
            }
        }

        graph.insert((x, y), neighbors);
    }

    let mut cache = HashMap::new();
    longest_path_acyclic((0, 1), (n - 1, m - 2), &graph, &mut cache)
        .unwrap()
        .into()
}

fn longest_path_cyclic(
    state: (usize, u64),
    target: usize,
    graph: &[Vec<(usize, u32)>],
    cache: &mut HashMap<(usize, u64), Option<u32>>,
) -> Option<u32> {
    if let Some(&result) = cache.get(&state) {
        return result;
    }

    let mut result = None;
    let (node, mask) = state;

    if node == target {
        result = Some(0);
    } else {
        for &(dst, d) in &graph[node] {
            if mask & (1 << dst) == 0 {
                if let Some(n_dist) =
                    longest_path_cyclic((dst, mask | (1 << dst)), target, graph, cache)
                {
                    result = result.max(Some(d + n_dist));
                }
            }
        }
    }

    cache.insert(state, result);
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let n = board.len();
    let m = board[0].len();

    let mut graph = HashMap::new();

    for x in 1..n - 1 {
        for y in 1..m - 1 {
            if board[x][y] == '#' {
                continue;
            }

            board[x][y] = '.';

            let mut count = 0;
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if board[nx][ny] != '#' {
                    count += 1;
                }
            }

            if count >= 3 {
                board[x][y] = 'x';
                graph.insert((x, y), vec![]);
            }
        }
    }
    board[n - 1][m - 2] = 'x';
    graph.insert((n - 1, m - 2), vec![]);

    board[0][1] = 'x';
    graph.insert((0, 1), vec![]);

    for (x, y) in graph.keys().copied().collect::<Vec<_>>().into_iter() {
        let mut neighbors = vec![];

        let mut visited = vec![vec![false; m]; n];
        let mut queue = VecDeque::new();

        visited[x][y] = true;
        queue.push_back((x, y, 0));

        while let Some((x, y, d)) = queue.pop_front() {
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if visited[nx][ny] {
                    continue;
                }
                visited[nx][ny] = true;

                match board[nx][ny] {
                    '#' => continue,
                    '.' => queue.push_back((nx, ny, d + 1)),
                    'x' => neighbors.push(((nx, ny), d + 1)),
                    _ => unreachable!(),
                }
            }
        }

        graph.insert((x, y), neighbors);
    }

    let mut remap = HashMap::new();
    for (id, key) in graph.keys().enumerate() {
        remap.insert(*key, id);
    }

    let mut n_graph = vec![vec![]; remap.len()];

    for (src, neighbors) in &graph {
        let src = remap[src];

        for (dst, d) in neighbors {
            let dst = remap[dst];
            n_graph[src].push((dst, *d));
        }
    }

    let source = remap[&(0, 1)];
    let target = remap[&(n - 1, m - 2)];

    let mut cache = HashMap::new();
    longest_path_cyclic((source, 1 << source), target, &n_graph, &mut cache)
        .unwrap()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
