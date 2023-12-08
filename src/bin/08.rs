use std::collections::HashMap;

use num::integer::lcm;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Graph<'a> {
    instructions: Vec<usize>,
    next: Vec<[usize; 2]>,
    rev_name: HashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
    fn parse(s: &'a str) -> Self {
        let mut lines = s.lines();

        let instructions = lines.next().unwrap();
        let instructions = instructions
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            })
            .collect();

        lines.next().unwrap();

        let mut rev_name = HashMap::new();
        let mut next = vec![];

        for line in lines {
            let (src, dst) = line.split_once(" = ").unwrap();
            let (left, right) = dst
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();

            let len = rev_name.len();
            let src = *rev_name.entry(src).or_insert(len);

            let len = rev_name.len();
            let left = *rev_name.entry(left).or_insert(len);

            let len = rev_name.len();
            let right = *rev_name.entry(right).or_insert(len);

            if src >= next.len() {
                next.resize(src + 1, [usize::MAX; 2]);
            }

            next[src] = [left, right];
        }

        Self {
            instructions,
            next,
            rev_name,
        }
    }

    fn node(&self, name: &str) -> Node<'_> {
        let node = *self.rev_name.get(name).unwrap();

        Node {
            graph: self,
            instruction_id: 0,
            position: node,
        }
    }
}

#[derive(Clone)]
struct Node<'a> {
    graph: &'a Graph<'a>,
    instruction_id: usize,
    position: usize,
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.instruction_id == other.instruction_id && self.position == other.position
    }
}

impl<'a> Eq for Node<'a> {}

impl<'a> Node<'a> {
    fn step(&self) -> Self {
        let next_positions =
            self.graph.next[self.position][self.graph.instructions[self.instruction_id]];
        let next_instruction_id = (self.instruction_id + 1) % self.graph.instructions.len();

        Self {
            graph: self.graph,
            instruction_id: next_instruction_id,
            position: next_positions,
        }
    }

    #[allow(dead_code)]
    fn walk(&'a self) -> Path {
        let mut turtle = self.clone();
        let mut hare = self.clone();

        loop {
            turtle = turtle.step();
            hare = hare.step().step();

            if turtle == hare {
                break;
            }
        }

        let mut cycle = 0;
        loop {
            hare = hare.step();
            cycle += 1;
            if turtle == hare {
                break;
            }
        }

        turtle = self.clone();
        hare = self.clone();

        for _ in 0..cycle {
            hare = hare.step();
        }

        let mut head = 0;
        while turtle != hare {
            turtle = turtle.step();
            hare = hare.step();
            head += 1;
        }

        Path { head, cycle }
    }
}

#[derive(Debug)]
struct Path {
    #[allow(dead_code)]
    head: usize,
    #[allow(dead_code)]
    cycle: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = Graph::parse(input);

    let mut node = graph.node("AAA");
    let target = graph.node("ZZZ").position;

    let mut steps = 0;
    while node.position != target {
        node = node.step();
        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<usize> {
    let graph = Graph::parse(input);

    let mut end = vec![false; graph.rev_name.len()];

    for (key, value) in graph.rev_name.iter() {
        if key.ends_with('Z') {
            end[*value] = true;
        }
    }

    Some(
        graph
            .rev_name
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|start| {
                let mut node = graph.node(start);

                let mut steps = 0;
                while !end[node.position] {
                    node = node.step();
                    steps += 1;
                }

                // Required for solution with LCM to work
                // Run only on release mode, since example doesn't satisfy this constraint
                #[cfg(not(debug_assertions))]
                {
                    let path = node.walk();
                    assert_eq!(steps, path.cycle);
                }

                steps
            })
            .fold(1, lcm),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
