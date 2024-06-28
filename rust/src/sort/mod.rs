mod bubble_sort;
pub use bubble_sort::*;
mod select_sort;
pub use select_sort::*;
mod insert_sort;
pub use insert_sort::*;
mod merge_sort;
pub use merge_sort::*;
mod quick_sort;
pub use quick_sort::*;

/// Generic interface to sorting algorithms
pub trait Sort<T: core::cmp::PartialOrd + Clone> {
    /// Get the internal data
    fn inner(&self) -> Vec<T>;

    /// Customizable comparison logic based on closures
    fn sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy;

    /// Sort by ascending order
    fn sort(&mut self) {
        self.sort_by(|v1, v2| v1 <= v2);
    }

    /// Determine if the sort is ascending
    fn is_sort(&self) -> bool {
        self.is_sort_by(|v1, v2| v1 <= v2)
    }

    /// Customized judgments are ordered
    fn is_sort_by<F>(&self, f: F) -> bool
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        let array = self.inner();

        if array.is_empty() {
            return true;
        }

        for idx in 0..array.len() - 1 {
            if !f(&array[idx], &array[idx + 1]) {
                return false;
            }
        }
        true
    }
}

pub trait Infite {
    fn max_value() -> Self;

    fn min_value() -> Self;
}

impl Infite for i32 {
    fn max_value() -> Self {
        i32::MAX
    }
    fn min_value() -> Self {
        i32::MIN
    }
}
