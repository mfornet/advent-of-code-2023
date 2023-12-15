use itertools::Itertools;

advent_of_code::solution!(15);

fn custom_hash(s: &str) -> u32 {
    s.as_bytes()
        .iter()
        .fold(0, |hash, &c| (hash + (c as u32)) * 17 % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end_matches('\n')
            .split(',')
            .map(custom_hash)
            .sum::<u32>(),
    )
}

#[derive(Debug)]
struct PoorManHashmap<'a> {
    data: Vec<Vec<(&'a str, u32)>>,
}

impl<'a> PoorManHashmap<'a> {
    fn new() -> Self {
        Self {
            data: vec![vec![]; 256],
        }
    }

    fn insert(&mut self, key: &'a str, value: u32) {
        let key_hash = custom_hash(key) as usize;
        if let Some(slot) = self.data[key_hash].iter_mut().find(|(k, _)| *k == key) {
            slot.1 = value;
        } else {
            self.data[key_hash].push((key, value));
        }
    }

    fn remove_entry(&mut self, key: &'a str) {
        let key_hash = custom_hash(key) as usize;
        if let Some((index, _)) = self.data[key_hash]
            .iter()
            .find_position(|&&(k, _)| k == key)
        {
            self.data[key_hash].remove(index);
        }
    }

    fn focus_power(&self) -> u32 {
        self.data
            .iter()
            .enumerate()
            .map(|(index, unit)| {
                (index as u32 + 1)
                    * unit
                        .iter()
                        .enumerate()
                        .map(|(slot, &(_, value))| (slot as u32 + 1) * value)
                        .sum::<u32>()
            })
            .sum::<u32>()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end_matches('\n')
            .split(',')
            .fold(PoorManHashmap::new(), |mut hashmap, s| {
                if s.contains('-') {
                    let key = s.trim_end_matches('-');
                    hashmap.remove_entry(key);
                } else {
                    let (key, value) = s.split_once('=').unwrap();
                    hashmap.insert(key, value.parse::<u32>().unwrap());
                }
                hashmap
            })
            .focus_power(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
