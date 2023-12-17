mod day;
pub mod template;

pub use day::*;

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
