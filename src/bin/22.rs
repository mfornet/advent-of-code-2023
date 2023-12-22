use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(22);

struct Brick {
    low: (usize, usize, usize),
    high: (usize, usize, usize),
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (low, high) = value
            .split('~')
            .map(|point| {
                point
                    .split(',')
                    .map(|value| value.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        Brick { low, high }
    }
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.low.2 == other.low.2
    }
}

impl Eq for Brick {}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.low.2.cmp(&other.low.2))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn build_graph(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut bricks = input.lines().map(Into::<Brick>::into).collect_vec();
    bricks.sort_unstable();

    let max_x = bricks.iter().map(|brick| brick.high.0).max().unwrap();
    let max_y = bricks.iter().map(|brick| brick.high.1).max().unwrap();

    let mut board = vec![vec![(usize::MAX, 0); max_y + 1]; max_x + 1];
    let mut graph = vec![vec![]; bricks.len()];
    let mut rev_graph = vec![vec![]; bricks.len()];

    for (id, brick) in bricks.into_iter().enumerate() {
        let mut highest = 0;
        let mut indexes = HashSet::<usize>::new();

        for row in board.iter().take(brick.high.0 + 1).skip(brick.low.0) {
            for &(id, height) in row.iter().take(brick.high.1 + 1).skip(brick.low.1) {
                if height > highest {
                    highest = height;
                    indexes.clear();
                }

                if highest == height && id != usize::MAX {
                    indexes.insert(id);
                }
            }
        }

        assert!(brick.low.2 > highest);

        for index in indexes {
            graph[index].push(id);
            rev_graph[id].push(index);
        }

        for row in board.iter_mut().take(brick.high.0 + 1).skip(brick.low.0) {
            for cell in row.iter_mut().take(brick.high.1 + 1).skip(brick.low.1) {
                *cell = (id, brick.high.2 - brick.low.2 + 1 + highest);
            }
        }
    }

    (graph, rev_graph)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, rev_graph) = build_graph(input);

    graph
        .into_iter()
        .map(|node| node.into_iter().all(|id| rev_graph[id].len() > 1) as u32)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (graph, rev_graph) = build_graph(input);

    let rev_graph = rev_graph
        .into_iter()
        .map(|node| node.into_iter().collect::<HashSet<_>>())
        .collect_vec();

    (0..graph.len())
        .map(|i| {
            let mut rev_graph = rev_graph.clone();
            let mut answer = 0;
            let mut events = vec![i];

            while let Some(node) = events.pop() {
                for &next in &graph[node] {
                    if rev_graph[next].remove(&node) && rev_graph[next].is_empty() {
                        events.push(next);
                        answer += 1;
                    }
                }
            }
            answer
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
