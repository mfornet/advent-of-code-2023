advent_of_code::solution!(9);

pub fn solve(input: &str, first: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut base = line
                .split_ascii_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if first {
                base.reverse();
            }

            let mut answer = 0;

            while !base.iter().all(|&x| x == 0) {
                answer += *base.last().unwrap();
                base = base
                    .iter()
                    .zip(base.iter().skip(1))
                    .map(|(&x, &y)| y - x)
                    .collect();
            }

            answer
        })
        .sum::<i64>()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
