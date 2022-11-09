use super::Sort;

/// select sort
#[derive(Debug)]
pub struct SelectSort<T> {
    arr: Vec<T>,
}

impl<T> SelectSort<T> {
    pub fn select_sort<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        let len = self.arr.len();
        for i in 0..len {
            // 将a[i]和a[i+1..len]最小的元素交换
            let mut min = i; // 最小元素的索引
            for j in i + 1..len {
                if f(&self.arr[j], &self.arr[min]) {
                    min = j;
                }
            }
            self.arr.swap(i, min);
        }
    }
}

impl<T: core::cmp::PartialOrd + Clone> Sort<T> for SelectSort<T> {
    fn from_vec(arr: Vec<T>) -> Self {
        Self { arr }
    }

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
        let mut select = SelectSort::from_vec(vec![10, 9, 8, 6, 5, 4, 3, 2, 1]);
        select.sort();
        assert!(select.is_sort());
    }

    #[test]
    fn test_select_sort_a_empty_arr() {
        let mut select = SelectSort::from_vec(Vec::<i32>::new());
        select.sort();
        assert!(select.is_sort());
    }
}
