use super::Infite;
use super::Sort;
use core::fmt::Debug;

/// Merge Sort
#[derive(Debug)]
pub struct MergeSort<T> {
    array: Vec<T>,
}

impl<T> MergeSort<T>
where
    T: Copy + Default + Infite + Debug,
{
    fn merge_sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        let start = 0;
        let end = self.array.len() - 1;
        inner_merge_sort(&mut self.array, start, end, f);
    }
}

impl<T> From<Vec<T>> for MergeSort<T> {
    fn from(arr: Vec<T>) -> Self {
        Self { array: arr }
    }
}

impl<T> Sort<T> for MergeSort<T>
where
    T: core::cmp::PartialOrd + Default + Copy + Infite + Debug,
{
    fn inner(&self) -> Vec<T> {
        self.array.clone()
    }

    fn sort_by<F>(&mut self, f: F)
    where
        F: FnOnce(&T, &T) -> bool + core::marker::Copy,
    {
        self.merge_sort_by(f)
    }
}

fn inner_merge_sort<T, F>(array: &mut Vec<T>, p: usize, r: usize, f: F)
where
    T: Default + Copy + Infite + Debug,
    F: FnOnce(&T, &T) -> bool + core::marker::Copy,
{
    if p < r {
        let q = (p + r) / 2;
        inner_merge_sort(array, p, q, f);
        inner_merge_sort(array, q + 1, r, f);
        inner_merge(array, p, q + 1, r + 1, f); // this is sick
    }
}

fn inner_merge<T, F>(arr: &mut Vec<T>, p: usize, q: usize, r: usize, f: F)
where
    T: Default + Copy + Infite + Debug,
    F: FnOnce(&T, &T) -> bool + core::marker::Copy,
{
    log::info!("p = {}, q = {}, r = {}", p, q, r);
    let n1 = q - p;
    let n2 = r - q;

    // let L[1..n1 + 1] and R[1..n2+ 1] be new arrayss
    let max_value = T::max_value();
    let mut l_arr = vec![max_value; n1 + 1];
    let mut r_arr = vec![max_value; n2 + 1];

    for i in 0..n1 {
        if let Some(v) = arr.get(p + i) {
            if let Some(t) = l_arr.get_mut(i) {
                *t = *v;
            }
        }
    }

    log::info!("l_arr = {:?}", l_arr);

    for j in 0..n2 {
        if let Some(v) = arr.get(q + j) {
            if let Some(t) = r_arr.get_mut(j) {
                *t = *v;
            }
        }
    }
    log::info!("r_arr = {:?}", r_arr);

    let mut i = 0usize;
    let mut j = 0usize;

    for k in p..r {
        if f(&l_arr[i], &r_arr[j]) {
            if let Some(v) = arr.get_mut(k) {
                *v = l_arr[i];
            }
            i = i + 1;
        } else {
            if let Some(v) = arr.get_mut(k) {
                *v = r_arr[j];
            }
            j = j + 1;
        }
    }
}

#[test]
fn test_merge_sort() {
    impl Infite for i32 {
        fn max_value() -> Self {
            i32::MAX
        }
        fn min_value() -> Self {
            i32::MIN
        }
    }
    let array = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    let mut merge_sort = MergeSort::from(array);
    println!("merge_sort: {:?}", merge_sort);
    merge_sort.sort();
    assert!(merge_sort.is_sort());
    println!("merge_sort: {:?}", merge_sort);
}
