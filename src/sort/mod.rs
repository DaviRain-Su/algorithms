mod bubble_sort;
pub use bubble_sort::*;
mod select_sort;
pub use select_sort::*;
mod insert_sort;
pub use insert_sort::*;

trait Sort<T: core::cmp::PartialOrd + Clone> {
    fn from_vec(array: Vec<T>) -> Self;

    fn inner(&self) -> Vec<T>;

    fn sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy;

    fn sort(&mut self) {
        self.sort_by(|v1, v2| v1 < v2);
    }

    fn is_sort(&self) -> bool {
        self.is_sort_by(|v1, v2| v1 < v2)
    }

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
