use itertools::Itertools;

advent_of_code::solution!(11);

struct FenwickTree {
    tree: Vec<usize>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn add(&mut self, index: usize, value: usize) {
        let mut i = index + 1;
        while i < self.tree.len() {
            self.tree[i] += value;
            i += i & (!i + 1);
        }
    }

    fn sum(&self, index: usize) -> usize {
        let mut i = index + 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & (!i + 1);
        }
        sum
    }
}

pub fn solve(input: &str, expansion: usize) -> usize {
    let galaxy_map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let n = galaxy_map.len();
    let m = galaxy_map[0].len();

    let mut rows = FenwickTree::new(n);
    let mut cols = FenwickTree::new(m);

    for i in 0..n {
        if !galaxy_map[i].iter().any(|&x| x) {
            rows.add(i, 1);
        }
    }

    for j in 0..m {
        if !galaxy_map.iter().map(|row| row[j]).any(|x| x) {
            cols.add(j, 1);
        }
    }

    let mut locations = vec![];

    for i in 0..n {
        for j in 0..m {
            if galaxy_map[i][j] {
                locations.push((i, j));
            }
        }
    }

    let mut answer = 0;

    for (i, &(x0, y0)) in locations.iter().enumerate() {
        for &(x1, y1) in locations[0..i].iter() {
            let xh = std::cmp::max(x0, x1);
            let xl = std::cmp::min(x0, x1);
            let yh = std::cmp::max(y0, y1);
            let yl = std::cmp::min(y0, y1);

            answer += xh - xl + yh - yl
                + (rows.sum(xh) - rows.sum(xl) + cols.sum(yh) - cols.sum(yl)) * (expansion - 1);
        }
    }
    answer
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 1000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 1030);
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, 8410);
    }
}
