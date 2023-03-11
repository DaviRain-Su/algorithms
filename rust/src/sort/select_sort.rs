use super::Sort;
use core::clone::Clone;
use core::cmp::PartialOrd;

/// select sort
#[derive(Debug)]
pub struct SelectSort<T> {
    arr: Vec<T>,
}

impl<T> From<Vec<T>> for SelectSort<T> {
    fn from(arr: Vec<T>) -> Self {
        Self { arr }
    }
}

impl<T: Clone> From<&[T]> for SelectSort<T> {
    fn from(arr: &[T]) -> Self {
        Self { arr: arr.into() }
    }
}

impl<T> SelectSort<T> {
    pub fn select_sort<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + Copy,
    {
        for i in 0..self.arr.len() {
            // 将a[i]和a[i+1..len]最小的元素交换
            let mut min = i; // 最小元素的索引
            for j in (i + 1)..self.arr.len() {
                if f(&self.arr[j], &self.arr[min]) {
                    min = j;
                }
            }
            self.arr.swap(i, min);
        }
    }

    pub fn select_sort_by_find_smallest<F>(&mut self, f: F) -> Vec<T>
    where
        F: FnOnce(Option<&T>, Option<&T>) -> bool + Copy,
    {
        let mut result = vec![];
        for _ in 0..self.arr.len() {
            let smallest = self.find_smallest(f);
            result.push(self.arr.swap_remove(smallest));
        }
        result
    }

    pub fn find_smallest<F>(&self, f: F) -> usize
    where
        F: FnOnce(Option<&T>, Option<&T>) -> bool + Copy,
    {
        let mut smallest = self.arr.get(0);
        let mut smallest_index = 0usize;
        for i in 1..self.arr.len() {
            if f(self.arr.get(i), smallest) {
                smallest = self.arr.get(i);
                smallest_index = i;
            }
        }
        smallest_index
    }
}

impl<T: PartialOrd + Clone> Sort<T> for SelectSort<T> {
    fn inner(&self) -> Vec<T> {
        self.arr.clone()
    }

    fn sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        self.select_sort(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_sort_ok() {
        let mut select = SelectSort::from(vec![10, 9, 8, 6, 5, 4, 3, 2, 1]);
        select.sort();
        assert!(select.is_sort());
    }

    #[test]
    fn test_select_sort_a_empty_arr() {
        let mut select = SelectSort::from(Vec::<i32>::new());
        select.sort();
        assert!(select.is_sort());
    }

    #[test]
    fn test_select_sort_by_find_smallest() {
        let mut select = SelectSort::from(vec![10, 9, 8, 6, 5, 4, 3, 2, 1]);
        let result = select.select_sort_by_find_smallest(|v1, v2| v1 < v2);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 8, 9, 10]);
    }
}
