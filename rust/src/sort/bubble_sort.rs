// Sort the array using bubble sort. The idea behind
// bubble sort is to look for adjacent indexes which
// are out of place and interchange their elements
// until the entire array is sorted.
use super::Sort;
use core::cmp::PartialOrd;

/// bubble sort
#[derive(Debug)]
pub struct BubbleSort<T> {
    arr: Vec<T>,
}

impl<T> From<Vec<T>> for BubbleSort<T> {
    fn from(arr: Vec<T>) -> Self {
        Self { arr }
    }
}

impl<T: core::clone::Clone> From<&[T]> for BubbleSort<T> {
    fn from(arr: &[T]) -> Self {
        Self { arr: arr.into() }
    }
}

impl<T: Clone + PartialOrd> Sort<T> for BubbleSort<T> {
    fn inner(&self) -> Vec<T> {
        self.arr.clone()
    }

    fn sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        let _ = self.bubble_sort_by(f);
    }
}

impl<T: PartialOrd> BubbleSort<T> {
    /// bubble sort can customize the comparison logic based on the comparison function passed in
    pub fn bubble_sort_by<F>(&mut self, cmp_fun: F) -> bool
    where
        F: FnOnce(&T, &T) -> bool + Copy,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut bubble = BubbleSort::from(vec![10, 4, 6, 8, 13, 2, 3]);
        bubble.sort();
        println!("{bubble:?}");
        assert!(bubble.is_sort());
    }

    #[test]
    fn test_bubble_sort_a_empty_arr() {
        let mut bubble = BubbleSort::from(Vec::<i32>::new());
        bubble.sort();
        assert!(bubble.is_sort());
    }
}
