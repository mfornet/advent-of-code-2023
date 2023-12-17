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
}
