use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<u32>,
}

impl Hand {
    fn new(cards: Vec<u32>) -> Self {
        let mut new_cards = cards.clone();
        new_cards.retain(|x| *x != 0);
        let n_zeros = cards.len() - new_cards.len();

        let mut groups = new_cards
            .into_iter()
            .sorted()
            .group_by(|x| *x)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .collect::<Vec<_>>();

        if let Some(last) = groups.last_mut() {
            *last += n_zeros;
        } else {
            groups.push(n_zeros);
        }

        let hand_type = match groups.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => unreachable!(),
        };

        Self { cards, hand_type }
    }
}

fn parse(s: &str, joker: bool) -> Hand {
    let jack = if joker { 0 } else { 11 };
    let cards = s
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => jack,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        })
        .collect();
    Hand::new(cards)
}

fn solve(input: &str, joker: bool) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let bid = bid.parse::<u64>().unwrap();
                (parse(hand, joker), bid)
            })
            .sorted_unstable_by(|a, b| a.0.cmp(&b.0))
            .enumerate()
            .map(|(i, (_, bid))| (i as u64 + 1) * bid)
            .sum::<u64>(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
