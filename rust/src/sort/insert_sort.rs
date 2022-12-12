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

        for i in 1..len {
            // do a[i] insert to a[i - 1], a[i - 2], a[i - 3]... among
            let mut j = i;
            while j > 0 && f(&self.arr[j], &self.arr[j - 1]) {
                self.arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
impl<T> From<Vec<T>> for InsertSort<T> {
    fn from(arr: Vec<T>) -> Self {
        Self { arr }
    }
}

impl<T: core::clone::Clone> From<&[T]> for InsertSort<T> {
    fn from(arr: &[T]) -> Self {
        Self { arr: arr.into() }
    }
}

impl<T: core::cmp::PartialOrd + Clone> Sort<T> for InsertSort<T> {
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
        let mut insert = InsertSort::from(vec![10, 9, 8, 6, 5, 4, 3, 2, 1]);
        println!("insert sort before: {insert:?}");
        insert.sort();
        println!("insert sort after: {insert:?}");
        assert!(insert.is_sort());
    }

    #[test]
    fn test_insert_sort_a_empty_arr() {
        let mut insert = InsertSort::from(Vec::<i32>::new());
        insert.sort();
        assert!(insert.is_sort());
    }
}
