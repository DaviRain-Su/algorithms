use super::Infite;
use super::Sort;
use core::clone::Clone;
use core::cmp::PartialOrd;
use core::fmt::Debug;

/// Merge Sort
#[derive(Debug)]
pub struct MergeSort<T> {
    arr: Vec<T>,
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
        let end = self.arr.len() - 1;
        inner_merge_sort(&mut self.arr, start, end, f);
    }
}

impl<T> From<Vec<T>> for MergeSort<T> {
    fn from(arr: Vec<T>) -> Self {
        Self { arr }
    }
}

impl<T: Clone> From<&[T]> for MergeSort<T> {
    fn from(arr: &[T]) -> Self {
        Self { arr: arr.into() }
    }
}

impl<T> Sort<T> for MergeSort<T>
where
    T: PartialOrd + Default + Copy + Infite + Debug,
{
    fn inner(&self) -> Vec<T> {
        self.arr.clone()
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
    F: FnOnce(&T, &T) -> bool + Copy,
{
    if p < r {
        let q = (p + r) / 2;
        inner_merge_sort(array, p, q, f);
        inner_merge_sort(array, q + 1, r, f);
        inner_merge(array, p, q + 1, r + 1, f); // this is sick
    }
}

fn inner_merge<T, F>(arr: &mut [T], p: usize, q: usize, r: usize, f: F)
where
    T: Default + Copy + Infite + Debug,
    F: FnOnce(&T, &T) -> bool + Copy,
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
            i += 1;
        } else {
            if let Some(v) = arr.get_mut(k) {
                *v = r_arr[j];
            }
            j += 1;
        }
    }
}

#[test]
fn test_merge_sort() {
    let array = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    let mut merge_sort = MergeSort::from(array);
    println!("merge_sort: {merge_sort:?}");
    merge_sort.sort();
    assert!(merge_sort.is_sort());
    println!("merge_sort: {merge_sort:?}");
}

type Link<T> = Option<Box<ListNode<T>>>;
/// link node for sort list merge sort
#[derive(Debug, PartialEq)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Link<T>,
}

#[allow(dead_code)]
fn merge_two_lists_recu<T: Ord + Copy>(list1: Link<T>, list2: Link<T>) -> Link<T> {
    match (list1, list2) {
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
        (Some(l), Some(r)) => {
            if l.val <= r.val {
                Some(Box::new(ListNode {
                    next: merge_two_lists(l.next, Some(r)),
                    val: l.val,
                }))
            } else {
                Some(Box::new(ListNode {
                    next: merge_two_lists(Some(l), r.next),
                    val: r.val,
                }))
            }
        }
    }
}

#[allow(dead_code)]
fn merge_two_lists_no_recu<T: Ord + Copy>(list1: Link<T>, list2: Link<T>) -> Link<T> {
    let mut output = None;

    let mut next_node_pos = &mut output;
    let mut l1_opt = list1;
    let mut l2_opt = list2;
    loop {
        let mut l1 = match l1_opt {
            Some(l1) => l1,
            None => {
                *next_node_pos = l2_opt;
                break;
            }
        };
        let mut l2 = match l2_opt {
            Some(l2) => l2,
            None => {
                *next_node_pos = Some(l1);
                break;
            }
        };

        if l1.val < l2.val {
            l1_opt = l1.next.take();
            l2_opt = Some(l2);
            *next_node_pos = Some(l1);
        } else {
            l2_opt = l2.next.take();
            l1_opt = Some(l1);
            *next_node_pos = Some(l2);
        }

        next_node_pos = &mut next_node_pos.as_mut().unwrap().next;
    }

    output
}

// list merge sort
pub fn merge_two_lists<T: Ord>(mut list1: Link<T>, mut list2: Link<T>) -> Link<T> {
    let mut head = None;
    let mut tail = &mut head;

    loop {
        match (list1, list2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    list1 = l1.next.take();
                    list2 = Some(l2);
                    tail = &mut tail.insert(l1).next;
                } else {
                    list1 = Some(l1);
                    list2 = l2.next.take();
                    tail = &mut tail.insert(l2).next;
                }
            }
            (l1, l2) => break *tail = l1.or(l2),
        }
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! list {
        () => { None };
        ($head:expr $(, $val:expr)* $(,)?) => {
            Some(Box::new(ListNode {
                val: $head,
                next: list!($($val),*),
            }))
        };
    }

    #[test]
    fn test_merge_list() {
        let list1 = list!(1, 3);
        println!("list1: {:#?}", list1);
        let list2 = list!(2, 4);
        println!("list2: {:#?}", list2);

        let result = list!(1, 2, 3, 4);
        assert_eq!(result, merge_two_lists(list1, list2));
    }
}
