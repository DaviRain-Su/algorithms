mod bubble_sort;
pub use bubble_sort::*;

trait Sort<T> {
    fn from_vec(array: Vec<T>) -> Self;

    fn into_inner(&self) -> Vec<T>;

    fn sort(&mut self);

    fn is_sort<F>(&self, f: F) -> bool
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        let array = self.into_inner();
        
        if array.is_empty() { return true; }

        for idx in 0..array.len() - 1 {
            if !f(&array[idx], &array[idx + 1]) {
                return false;
            }
        }
        true
    }
}
