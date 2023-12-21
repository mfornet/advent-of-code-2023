mod day;
pub mod template;

pub use day::*;
use num_enum::IntoPrimitive;

pub struct NoCompare<T>(pub T);

impl<T> std::ops::Deref for NoCompare<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for NoCompare<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> PartialEq for NoCompare<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> Eq for NoCompare<T> {}

impl<T> PartialOrd for NoCompare<T> {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl<T> Ord for NoCompare<T> {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, IntoPrimitive)]
#[repr(u8)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }

    pub fn all() -> &'static [(Self, (isize, isize))] {
        &[
            (Self::Up, (-1, 0)),
            (Self::Left, (0, -1)),
            (Self::Down, (1, 0)),
            (Self::Right, (0, 1)),
        ]
    }

    pub fn index(&self) -> usize {
        Into::<u8>::into(*self) as usize
    }

    pub fn vector(&self) -> Point {
        match self {
            Self::Up => Point { x: 0, y: -1 },
            Self::Left => Point { x: -1, y: 0 },
            Self::Down => Point { x: 0, y: 1 },
            Self::Right => Point { x: 1, y: 0 },
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub fn cross(a: &Point, b: &Point) -> i64 {
    a.x * b.y - a.y * b.x
}

impl std::ops::Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct RotateInPlace<T> {
    data: Vec<Vec<T>>,
    tmp: Vec<Vec<T>>,
}

impl<T: Clone + Default> RotateInPlace<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let n = data.len();
        let m = data[0].len();
        Self {
            data,
            tmp: vec![vec![T::default(); n]; m],
        }
    }

    pub fn rotate(&mut self) {
        let n = self.data.len();

        for (i, row) in self.data.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                self.tmp[j][n - i - 1] = c.clone();
            }
        }

        std::mem::swap(&mut self.data, &mut self.tmp);
    }
}

impl<T> std::ops::Deref for RotateInPlace<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for RotateInPlace<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
