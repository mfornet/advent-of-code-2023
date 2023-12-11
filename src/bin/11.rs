use itertools::Itertools;

advent_of_code::solution!(11);

pub fn solve(input: &str, expansion: usize) -> usize {
    let galaxy_map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let n = galaxy_map.len();
    let m = galaxy_map[0].len();

    let mut rows = vec![1; n];
    let mut cols = vec![1; m];

    let mut locations = vec![];

    for i in 0..n {
        for j in 0..m {
            if galaxy_map[i][j] {
                locations.push((i, j));
                rows[i] = 0;
                cols[j] = 0;
            }
        }
    }

    for i in 1..n {
        rows[i] += rows[i - 1];
    }

    for i in 1..m {
        cols[i] += cols[i - 1];
    }

    let mut answer = 0;

    for (i, &(x0, y0)) in locations.iter().enumerate() {
        for &(x1, y1) in locations[0..i].iter() {
            let xh = std::cmp::max(x0, x1);
            let xl = std::cmp::min(x0, x1);
            let yh = std::cmp::max(y0, y1);
            let yl = std::cmp::min(y0, y1);

            answer +=
                xh - xl + yh - yl + (rows[xh] - rows[xl] + cols[yh] - cols[yl]) * (expansion - 1);
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
