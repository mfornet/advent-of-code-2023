use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy)]
struct Point2d {
    x: f64,
    y: f64,
}

impl std::ops::Add for Point2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for Point2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    position: Point2d,
    direction: Point2d,
}

impl Hailstone {
    fn same_direction(&self, point: &Point2d) -> bool {
        let delta = point.clone() - self.position;
        dot(&delta, &self.direction) >= EPS
    }
}

const EPS: f64 = 1e-9;

fn dot(a: &Point2d, b: &Point2d) -> f64 {
    a.x * b.x + a.y * b.y
}

fn cross(a: &Point2d, b: &Point2d) -> f64 {
    a.x * b.y - a.y * b.x
}

fn crosspoint(l: &Hailstone, m: &Hailstone) -> Option<Point2d> {
    let a = cross(&l.direction, &m.direction);
    let b = cross(&l.direction, &(l.position + l.direction - m.position));

    if a.abs() < EPS && b.abs() < EPS {
        unreachable!()
    }

    if a.abs() < EPS {
        return None;
    }

    Some(m.position + m.direction * (b / a))
}

fn solve_part_one(input: &str, low: f64, high: f64) -> u32 {
    let hailstones = input
        .lines()
        .map(|line| {
            line.split(" @ ")
                .map(|point| {
                    point
                        .split(",")
                        .map(|coord| coord.trim_start_matches(' ').parse::<i64>().unwrap() as f64)
                        .collect_tuple::<(_, _, _)>()
                        .unwrap()
                })
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .map(|hailstone| Hailstone {
            position: Point2d {
                x: hailstone.0 .0,
                y: hailstone.0 .1,
            },
            direction: Point2d {
                x: hailstone.1 .0,
                y: hailstone.1 .1,
            },
        })
        .collect_vec();

    let n = hailstones.len();
    let mut total = 0;

    let mut smallest_x = high;
    let mut smallest_y = high;
    let mut largest_x = low;
    let mut largest_y = low;

    for i in 0..n {
        for j in i + 1..n {
            if let Some(point) = crosspoint(&hailstones[i], &hailstones[j]) {
                if low <= point.x
                    && point.x <= high
                    && low <= point.y
                    && point.y <= high
                    && hailstones[i].same_direction(&point)
                    && hailstones[j].same_direction(&point)
                {
                    smallest_x = smallest_x.min(point.x);
                    smallest_y = smallest_y.min(point.y);
                    largest_x = largest_x.max(point.x);
                    largest_y = largest_y.max(point.y);

                    total += 1;
                }
            }
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve_part_one(input, 200000000000000.0, 400000000000000.0))
}

#[derive(Default, Clone, Copy, Debug)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl std::ops::Add for Point3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Point3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Point3d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Point3d {
    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn norm_squared(&self) -> f64 {
        self.dot(self)
    }
}

impl From<(f64, f64, f64)> for Point3d {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Default, Debug)]
struct State {
    positions: Vec<Point3d>,
    directions: Vec<Point3d>,
}

impl State {
    fn init_variable(&self) -> Variables {
        Variables {
            position: Point3d::default(),
            direction: Point3d::default(),
            delta: vec![0.0; self.positions.len()],
        }
    }

    fn eval(&self, var: &Variables) -> f64 {
        let n = self.positions.len() as f64;
        let p = var.position;
        let d = var.direction;

        self.positions
            .iter()
            .zip(self.directions.iter())
            .zip(var.delta.iter())
            .map(|((&pi, &di), &ai)| (p - pi + (d - di) * ai).norm_squared())
            .sum::<f64>()
            / n
    }

    fn diff(&self, var: &Variables) -> Variables {
        let mut diff = Variables::new_like(var);
        let n = self.positions.len() as f64;
        let p = var.position;
        let d = var.direction;

        let mut acc_p = Point3d::default();
        let mut acc_d = Point3d::default();

        for (i, ((&pi, &di), &ai)) in self
            .positions
            .iter()
            .zip(self.directions.iter())
            .zip(var.delta.iter())
            .enumerate()
        {
            let p = p - pi;
            let d = d - di;
            diff.delta[i] = (2. * p.dot(&d) + 2. * ai * d.norm_squared()) / n;

            acc_p = acc_p + p * 2. + d * 2. * ai;
            acc_d = acc_d + p * ai * 2. + d * 2. * ai * ai;
        }

        acc_p = acc_p * (1. / n);
        acc_d = acc_d * (1. / n);

        diff.position = acc_p;
        diff.direction = acc_d;

        diff
    }
}

#[derive(Debug)]
struct Variables {
    position: Point3d,
    direction: Point3d,
    delta: Vec<f64>,
}

impl Variables {
    fn new_like(var: &Variables) -> Variables {
        Variables {
            position: var.position,
            direction: var.direction,
            delta: vec![0.0; var.delta.len()],
        }
    }

    fn update(&mut self, diff: &Variables, lr: f64) {
        self.position = self.position - diff.position * lr;
        self.direction = self.direction - diff.direction * lr;
        for (delta, &d) in self.delta.iter_mut().zip(diff.delta.iter()) {
            *delta -= d * lr;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // TODO: Solved using python. Rewrite in Rust.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(
            &advent_of_code::template::read_file("examples", DAY),
            7.0,
            27.0,
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
