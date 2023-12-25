use std::{collections::HashMap, vec};

advent_of_code::solution!(25);

#[derive(Clone)]
struct Edge {
    dst: usize,
    flow: usize,
}

impl Edge {
    fn new(dst: usize) -> Self {
        Edge { dst, flow: 0 }
    }
}

fn augmenting_path(
    src: usize,
    sink: usize,
    graph: &mut Vec<Vec<Edge>>,
    visited: &mut Vec<bool>,
) -> bool {
    if visited[src] {
        return false;
    }
    visited[src] = true;

    if src == sink {
        return true;
    }

    let n = graph[src].len();
    for i in 0..n {
        if graph[src][i].flow == 1 {
            continue;
        }

        if augmenting_path(graph[src][i].dst, sink, graph, visited) {
            graph[src][i].flow += 1;
            return true;
        }
    }

    false
}

fn max_flow(mut graph: Vec<Vec<Edge>>, src: usize, sink: usize) -> usize {
    let mut mf = 0;

    while mf <= 3 {
        let mut visited = vec![false; graph.len()];
        if !augmenting_path(src, sink, &mut graph, &mut visited) {
            break;
        }
        mf += 1;
    }

    mf
}

fn find_size(graph: &Vec<Vec<Edge>>) -> usize {
    let mut visited = vec![false; graph.len()];
    let mut size = 0;
    let mut queue = vec![0];
    visited[0] = true;

    while let Some(u) = queue.pop() {
        size += 1;
        for e in &graph[u] {
            let v = e.dst;
            if !visited[v] {
                visited[v] = true;
                queue.push(v);
            }
        }
    }
    size
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let mut graph = vec![];

    for line in input.lines() {
        let (head, tail) = line.split_once(": ").unwrap();
        let len = cache.len();
        let head = *cache.entry(head).or_insert(len);

        for nxt in tail.split(' ') {
            let len = cache.len();
            let nxt = *cache.entry(nxt).or_insert(len);

            if cache.len() > graph.len() {
                graph.resize_with(cache.len(), || vec![]);
            }

            graph[head].push(Edge::new(nxt));
            graph[nxt].push(Edge::new(head));
        }
    }

    let mut edges = vec![];

    for i in 0..graph.len() {
        for e in &graph[i] {
            if i < e.dst {
                let mf = max_flow(graph.clone(), i, e.dst);
                if mf <= 3 {
                    edges.push((i, e.dst));
                }
            }
        }
    }

    for (u, v) in edges {
        graph[u].retain(|e| e.dst != v);
        graph[v].retain(|e| e.dst != u);
    }

    let left = find_size(&graph);
    let right = cache.len() - left;

    Some(left * right)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
