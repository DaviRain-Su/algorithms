use super::Sort;

/// Insert sort
#[derive(Debug)]
pub struct InsertSort<T> {
    arr: Vec<T>,
}

impl<T> InsertSort<T> {
    fn insert_sort<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        let len = self.arr.len();

        for i in 0..len {
            // 将a[i]插入到a[i - 1], a[i - 2], a[i - 3]...之中
            let mut j = i;
            loop {
                if j > 0 && f(&self.arr[j], &self.arr[j - 1]) {
                    self.arr.swap(j, j - 1);
                    j -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

impl<T: core::cmp::PartialOrd + Clone> Sort<T> for InsertSort<T> {
    fn from_vec(array: Vec<T>) -> Self {
        Self { arr: array }
    }

    fn inner(&self) -> Vec<T> {
        self.arr.clone()
    }

    fn sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        self.insert_sort(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_sort_ok() {
        let mut insert = InsertSort::from_vec(vec![10, 9, 8, 6, 5, 4, 3, 2, 1]);
        insert.sort();
        assert!(insert.is_sort());
    }

    #[test]
    fn test_insert_sort_a_empty_arr() {
        let mut insert = InsertSort::from_vec(Vec::<i32>::new());
        insert.sort();
        assert!(insert.is_sort());
    }
}
