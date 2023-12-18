use advent_of_code::{cross, Direction, Point};
use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Default, Debug)]
struct State {
    last_point: Point,
    corners: i64,
    perimeter: i64,
    area: i64,
}

impl State {
    fn area(&self) -> i64 {
        -self.area / 2 + self.perimeter / 2 - self.corners / 2 + 1
    }
}

fn parse_direction(s: &str) -> Direction {
    match s {
        "U" => Direction::Up,
        "L" => Direction::Left,
        "D" => Direction::Down,
        "R" => Direction::Right,
        _ => panic!("Invalid direction"),
    }
}

fn apply_state(mut state: State, step: (Direction, i64)) -> State {
    let (direction, distance) = step;
    let dir = direction.vector() * distance;

    let n_point = state.last_point + dir;
    let partial_area = cross(&n_point, &state.last_point);

    state.area += partial_area;
    state.last_point = n_point;
    state.perimeter += distance + 1;
    state.corners += 1;

    state
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (direction, distance, _) = line.split_whitespace().collect_tuple().unwrap();
                let direction = parse_direction(direction);
                let distance = distance.parse::<i64>().unwrap();
                (direction, distance)
            })
            .fold(State::default(), apply_state)
            .area(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, _, color) = line.split_whitespace().collect_tuple().unwrap();

                let (distance, direction) = color
                    .chars()
                    .skip(2)
                    .take(6)
                    .fold((0, 0), |(acc, last), c| {
                        (acc * 16 + last, c.to_digit(16).unwrap() as i64)
                    });

                let direction = match direction {
                    0 => Direction::Right,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Up,
                    _ => panic!("Invalid direction"),
                };

                (direction, distance)
            })
            .fold(State::default(), apply_state)
            .area(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, 1
            )),
            Some(4)
        );
        assert_eq!(
            part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, 2
            )),
            Some(9)
        );
        assert_eq!(
            part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, 3
            )),
            Some(21)
        );

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
