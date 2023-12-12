use itertools::Itertools;

advent_of_code::solution!(12);

fn solve(spring: &str, counts: impl Iterator<Item = usize>) -> usize {
    let counts = counts.collect_vec();

    let spring = format!(".{}", spring.trim_end_matches('.'));
    let spring = spring.chars().collect_vec();

    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for count in counts {
        let mut n_dp = vec![0; spring.len() + 1];
        let mut chunk = 0;

        for (i, &c) in spring.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            if chunk >= count && spring[i - count] != '#' {
                n_dp[i + 1] += dp[i - count];
            }
        }

        dp = n_dp;
    }

    *dp.last().unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let (spring, counts) = line.split_once(' ').unwrap();
                let counts = counts
                    .split(',')
                    .map(|number| number.parse::<usize>().unwrap());
                solve(spring, counts)
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let (spring, counts) = line.split_once(' ').unwrap();

                let spring = std::iter::once(spring).cycle().take(5).join("?");

                let counts = counts
                    .split(',')
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect_vec();
                let n = counts.len();

                solve(&spring, counts.into_iter().cycle().take(5 * n))
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
