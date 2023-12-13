use itertools::Itertools;

advent_of_code::solution!(13);

fn find_reflection(pattern: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
    let n = pattern.len();
    let m = pattern[0].len();

    let row = (1..n)
        .filter(|&row| {
            let size = std::cmp::min(row, n - row);
            (0..size).cartesian_product(0..m).all(|(x, y)| {
                let rx = row - x - 1;
                let cx = row + x;
                pattern[cx][y] == pattern[rx][y]
            })
        })
        .collect_vec();

    let col = (1..m)
        .filter(|&col| {
            let size = std::cmp::min(col, m - col);
            (0..n).cartesian_product(0..size).all(|(x, y)| {
                let ry = col - y - 1;
                let cy = col + y;
                pattern[x][cy] == pattern[x][ry]
            })
        })
        .collect_vec();

    (row, col)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| {
                let pattern = pattern
                    .lines()
                    .map(|line| line.chars().collect_vec())
                    .collect_vec();
                let (row, col) = find_reflection(&pattern);
                assert_eq!(row.len() + col.len(), 1);
                100 * row.get(0).copied().unwrap_or_default()
                    + col.get(0).copied().unwrap_or_default()
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| {
                let mut pattern = pattern
                    .lines()
                    .map(|line| line.chars().collect_vec())
                    .collect_vec();

                let (row, col) = find_reflection(&pattern);

                let n = pattern.len();
                let m = pattern[0].len();

                for (i, j) in (0..n).cartesian_product(0..m) {
                    match pattern[i][j] {
                        '#' => pattern[i][j] = '.',
                        '.' => pattern[i][j] = '#',
                        _ => unreachable!(),
                    }

                    let (n_row, n_col) = find_reflection(&pattern);

                    let n_row = n_row.into_iter().filter(|x| !row.contains(x)).collect_vec();
                    let n_col = n_col.into_iter().filter(|x| !col.contains(x)).collect_vec();

                    if !n_row.is_empty() || !n_col.is_empty() {
                        return 100 * n_row.get(0).copied().unwrap_or_default()
                            + n_col.get(0).copied().unwrap_or_default();
                    }

                    match pattern[i][j] {
                        '#' => pattern[i][j] = '.',
                        '.' => pattern[i][j] = '#',
                        _ => unreachable!(),
                    }
                }

                unreachable!();
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
