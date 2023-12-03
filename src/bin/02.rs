use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let top = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    Some(
        input
            .lines()
            .filter_map(|line| {
                let (head, rounds) = line.split_once(':').unwrap();

                let game_id = head.split(' ').last().unwrap().parse::<u32>().unwrap();
                rounds
                    .split(';')
                    .flat_map(|round| round.split(','))
                    .all(|round| {
                        let [amount, color] = TryInto::<[&str; 2]>::try_into(
                            round.trim_start_matches(' ').split(' ').collect::<Vec<_>>(),
                        )
                        .unwrap();
                        let amount = amount.parse::<u32>().unwrap();
                        amount <= *top.get(color).unwrap()
                    })
                    .then_some(game_id)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let rounds = line.split(':').last().unwrap();
                rounds
                    .split(';')
                    .flat_map(|round| round.split(','))
                    .fold(HashMap::new(), |mut acc, round| {
                        let (amount, color) = round.trim().split_once(' ').unwrap();
                        let amount = amount.parse::<u32>().unwrap();
                        acc.entry(color)
                            .and_modify(|e| *e = std::cmp::max(*e, amount))
                            .or_insert(amount);
                        acc
                    })
                    .values()
                    .product::<u32>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
