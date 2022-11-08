// Sort the array using bubble sort. The idea behind
// bubble sort is to look for adjacent indexes which
// are out of place and interchange their elements
// until the entire array is sorted.
use super::Sort;

#[derive(Debug)]
pub struct BubbleSort<T> {
    arr: Vec<T>,
}

impl<T: Clone + core::cmp::PartialOrd> Sort<T> for BubbleSort<T> {
    fn from_vec(arr: Vec<T>) -> Self {
        Self { arr }
    }

    fn inner(&self) -> Vec<T> {
        self.arr.clone()
    }

    fn sort(&mut self) {
        let _ = self.bubble_sort();
    }

    fn sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        let _ = self.bubble_sort_by(f);
    }
}

impl<T: core::cmp::PartialOrd> BubbleSort<T> {
    /// bubble sort can customize the comparison logic based on the comparison function passed in
    pub fn bubble_sort_by<F>(&mut self, cmp_fun: F) -> bool
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        if self.arr.is_empty() {
            return true;
        }

        let mut sorted = false;
        let len = self.arr.len();

        while !sorted {
            sorted = true;
            (0..(len - 1)).for_each(|idx| {
                if cmp_fun(&self.arr[idx + 1], &self.arr[idx]) {
                    self.arr.swap(idx, idx + 1);
                    sorted = false;
                }
            })
        }

        sorted
    }

    /// The ascending sort algorithm for bubble sort
    pub fn bubble_sort(&mut self) -> bool {
        self.bubble_sort_by(|v1, v2| v1 < v2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut bubble = BubbleSort::from_vec(vec![10, 4, 6, 8, 13, 2, 3]);
        bubble.sort();
        assert!(bubble.is_sort());
    }

    #[test]
    fn test_bubble_sort_a_empty_arr() {
        let mut bubble = BubbleSort::from_vec(Vec::<i32>::new());
        bubble.sort();
        assert!(bubble.is_sort());
    }
}
