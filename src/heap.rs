use alloc::vec::Vec;
use core::fmt::{Debug, Display};

fn parent(i: usize) -> usize {
    i / 2
}

fn left(i: usize) -> usize {
    ((i + 1) << 1) - 1
}

fn right(i: usize) -> usize {
    (i + 1) << 1
}

/// Heap
#[derive(Debug)]
pub struct Heap<T> {
    /// heap data
    data: Vec<T>,
    /// heap size
    size: usize,
}

impl<T: Clone + PartialOrd + Default + Display + Debug> Heap<T> {
    /// Creating a empty heap
    ///
    /// ```rust
    /// use algorithms_rs::Heap;
    ///
    /// let empty_heap = Heap::<i32>::new();
    ///
    /// assert_eq!(empty_heap.is_empty(), true);
    /// ```
    pub fn new() -> Self {
        Self {
            data: vec![],
            size: 0,
        }
    }

    /// Creating a heap from an array
    ///
    /// ```rust
    /// use algorithms_rs::Heap;
    ///
    /// let empty_heap = Heap::<i32>::from_vector(&vec![1]).unwrap();
    ///
    /// assert_eq!(empty_heap.is_empty(), true);
    /// ```
    pub fn from_vector(array: &[T]) -> anyhow::Result<Self> {
        if array.len() == 0 {
            return Err(anyhow::anyhow!("Can't create a empty heap"));
        }

        Ok(Self {
            data: array.into(),
            size: array.len() - 1,
        })
    }

    /// Length of the heap
    pub fn len(&self) -> usize {
        self.size
    }

    /// Determine if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the internal data of the heap
    pub fn inner_vec(&self) -> &[T] {
        &self.data
    }

    /// Big root heap adjustment Recursive algorithm implementation
    pub fn max_heapify(&mut self, index: usize) {
        // setting largest is index
        let mut largest = index;
        let left = left(index);
        let right = right(index);

        // if left > largest then larget = left
        if left <= self.len() && self.data.get(largest) < self.data.get(left) {
            largest = left;
        }

        // if right > largest then largest = right
        if right <= self.len() && self.data.get(largest) < self.data.get(right) {
            largest = right;
        }

        if largest != index {
            // swap vector index , largest value
            self.data.swap(index, largest);
            // rec call max_heapify
            self.max_heapify(largest);
        }
    }

    /// Small root heap adjustment Recursive algorithm implementation
    pub fn min_heapify(&mut self, index: usize) {
        // setting min is index
        let mut min = index;
        let left = left(index);
        let right = right(index);

        // if min > left then min = left
        if left <= self.len() && self.data.get(min) > self.data.get(left) {
            min = left;
        }

        // if min > right then min = right
        if right <= self.len() && self.data.get(min) > self.data.get(right) {
            min = right;
        }

        if min != index {
            // swap vector index, min value
            self.data.swap(index, min);
            // rec call min_heapify
            self.min_heapify(min);
        }
    }

    /// Small root heap upward adjustment Non-recursive algorithm implementation
    pub fn min_sift_up(&mut self, index: usize) {
        let mut cur_idx = index;
        loop {
            // if cur_idx is root idx will break
            if cur_idx == 0 {
                break;
            }

            // get parent index
            let parent_idx = parent(cur_idx);

            // when parent node <= child node will break
            if self.data[parent_idx] <= self.data[cur_idx] {
                break;
            }

            // swap parent node idx with child node idx
            self.data.swap(parent_idx, cur_idx);

            // now cur_idx is assign to it's parent idx
            cur_idx = parent_idx;
        }
    }

    /// Big root heap upward adjustment Non-recursive algorithm implementation
    pub fn max_sift_up(&mut self, index: usize) {
        let mut cur_idx = index;
        loop {
            // if cur_idx is root idx will break
            if cur_idx == 0 {
                break;
            }

            // get parent index
            let parent_idx = parent(cur_idx);

            // when child node <= parent node will break
            if self.data[cur_idx] <= self.data[parent_idx] {
                break;
            }

            // swap parent node idx with child node idx
            self.data.swap(parent_idx, cur_idx);

            // now cur_idx is assign to it's parent idx
            cur_idx = parent_idx;
        }
    }

    /// Small root heap downward adjustment Non-recursive algorithm implementation
    pub fn min_sift_down(&mut self, heap_len: usize) {
        let mut cur_idx = 0usize;
        loop {
            // get cur_idx has left child idx
            let mut child_idx = 2 * cur_idx + 1;

            if cur_idx > heap_len || child_idx > heap_len {
                break;
            }

            // child is the left child of cur_idx
            // find left child and right child lesser child
            if child_idx + 1 <= heap_len {
                // right_child_idx is the right child of cur_idx
                if self.data[child_idx + 1] < self.data[child_idx] {
                    child_idx += 1;
                }
            }

            // child is the lesser child of cur_idx
            // if child's parent (cur_idx) <= child will break
            if self.data[cur_idx] <= self.data[child_idx] {
                break;
            }

            // otherwise swap lesser child idx with cur_idx(parent idx)
            self.data.swap(child_idx, cur_idx);

            // assign cur_idx with lesser child idx
            cur_idx = child_idx;
        }
    }

    /// Big root heap downward adjustment Non-recursive algorithm implementation
    pub fn max_sift_down(&mut self, heap_len: usize) {
        let mut cur_idx = 0usize;
        loop {
            // get cur_idx has left child idx
            let mut child_idx = 2 * cur_idx + 1;

            if cur_idx > heap_len || child_idx > heap_len {
                break;
            }

            // child is the left child of cur_idx
            // find left child and right child bigger child
            if child_idx + 1 <= heap_len {
                if self.data[child_idx + 1] > self.data[child_idx] {
                    child_idx += 1;
                }
            }

            // child is the lesser child of cur_idx
            // if child's parent (cur_idx) > child will break
            if self.data[cur_idx] > self.data[child_idx] {
                break;
            }

            // otherwise swap lesser child idx with cur_idx(parent idx)
            self.data.swap(child_idx, cur_idx);

            // assign cur_idx with lesser child idx
            cur_idx = child_idx;
        }
    }

    /// Constructing a big root heap by recursive adjustment algorithm of big root heap
    pub fn build_max_heap_by_max_heapify(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.max_heapify(index);
        }
    }

    /// Construction of large root heap by non-recursive adjustment algorithm of large root heap
    ///
    /// ```rust
    /// use algorithms_rs::Heap;
    ///
    /// let mut max_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]).unwrap();
    ///
    /// max_heap.build_max_heap_by_shift_up();
    ///
    /// assert_eq!(max_heap.inner_vec().to_vec(), vec![5, 4, 2, 3, 1])
    /// ```
    pub fn build_max_heap_by_shift_up(&mut self) {
        // for i = [2; n]
        // invariant : heap(1, i - 1)
        // max_sift_up(i)
        // heap(1, i)
        for index in (0..self.data.len()).rev() {
            self.max_sift_up(index);
        }
    }

    /// Constructing rootlet heap by recursive adjustment algorithm of rootlet heap
    pub fn build_min_heap_by_min_heapify(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.min_heapify(index);
        }
    }

    /// Construction of rootlet heap by non-recursive adjustment algorithm of rootlet heap
    ///
    /// ```rust
    ///  use algorithms_rs::Heap;
    ///
    ///  let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]).unwrap();
    ///
    ///  min_heap.build_min_heap_by_siftup();
    ///
    ///  assert_eq!(min_heap.inner_vec().to_vec(), vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn build_min_heap_by_siftup(&mut self) {
        // for i = [2; n]
        // invariant : heap(1, i - 1)
        // min_sift_up(i)
        // heap(1, i)
        for index in 0..self.data.len() {
            self.min_sift_up(index);
        }
    }

    /// Ascending sort implementation based on recursive implementation of the big root heap
    ///
    /// ```rust
    /// use algorithms_rs::Heap;
    ///
    /// let mut max_heap = Heap::from_vector(&vec![5, 3, 7, 9, 10, 23, 45, 23, 12, 23, 0, 12, 32]).unwrap();
    ///
    /// max_heap.heap_sort_by_max_heap();
    ///
    /// assert_eq!(
    ///    max_heap.inner_vec().to_vec(),
    ///    vec![0, 3, 5, 7, 9, 10, 12, 12, 23, 23, 23, 32, 45]
    /// );
    /// ```
    pub fn heap_sort_by_max_heap(&mut self) {
        self.build_max_heap_by_max_heapify();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size -= 1;
            self.max_heapify(0);
        }
    }

    /// Descending sort implementation based on recursive implementation of small root heap
    ///
    /// ```rust
    /// use algorithms_rs::Heap;
    ///
    /// let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 0, 23, 34, 56, 11, 230, 12]).unwrap();
    ///
    /// min_heap.heap_sort_by_min_heap();
    ///
    /// assert_eq!(min_heap.inner_vec().to_vec(), vec![230, 56, 34, 23, 12, 11, 3, 2, 1, 0]);
    /// ```
    pub fn heap_sort_by_min_heap(&mut self) {
        self.build_min_heap_by_min_heapify();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size -= 1;
            self.min_heapify(0);
        }
    }

    /// Descending sort implementation based on non-recursive implementation of small root heap
    ///
    /// ```rust
    ///  use algorithms_rs::Heap;
    ///
    ///  let mut min_heap =
    ///  Heap::from_vector(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 13, 14]).unwrap();
    ///  min_heap.dec_sort_with_min_sift();
    ///  assert_eq!(
    ///        min_heap.inner_vec().to_vec(),
    ///         vec![14, 13, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    ///  );
    /// ```
    pub fn dec_sort_with_min_sift(&mut self) {
        // for (i = n; i >= 2; i --)
        // heap(1, i) && sorted(i + 1, n) && x[1..i] <= x[i+1..n]
        // swap(1, i)
        // heap(2, i - 1) && sorted(i, n) && x[1..i-1] <= x[i..n]
        // sift_down(i - 1)
        // heap(1, i - 1) && sorted(i, n) && x[1..i - 1] <= x[i..n]
        // build Min Heap by min siftup
        self.build_min_heap_by_siftup();
        for idx in (1..self.data.len()).rev() {
            self.data.swap(0, idx);
            self.min_sift_down(idx - 1);
        }
    }

    /// Non-recursive implementation of ascending sort based on large root heap
    ///
    /// ```rust
    ///  use algorithms_rs::Heap;
    ///
    ///  let mut max_heap = Heap::from_vector(&vec![9, 8, 7, 6, 5, 5, 4, 3, 2, 1, 0]).unwrap();
    ///
    ///  max_heap.asc_sort_with_max_sift();
    ///
    ///  assert_eq!(max_heap.inner_vec().to_vec(), vec![0, 1, 2, 3, 4, 5, 5, 6, 7, 8, 9]);
    /// ```
    pub fn asc_sort_with_max_sift(&mut self) {
        // for (i = n; i >= 2; i --)
        // heap(1, i) && sorted(i + 1, n) && x[1..i] <= x[i+1..n]
        // swap(1, i)
        // heap(2, i - 1) && sorted(i, n) && x[1..i-1] <= x[i..n]
        // sift_down(i - 1)
        // heap(1, i - 1) && sorted(i, n) && x[1..i - 1] <= x[i..n]
        // build Max heap by max shiftup
        self.build_max_heap_by_shift_up();
        for idx in (1..self.data.len()).rev() {
            self.data.swap(0, idx);
            self.max_sift_down(idx - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let mut vec_temp = vec![1, 2, 3];
        vec_temp.swap(0, 1);
        assert_eq!(vec_temp, vec![2, 1, 3]);
    }

    #[test]
    fn test_build_max_heap() {
        let mut max_heap =
            Heap::from_vector(&vec![5, 3, 7, 9, 10, 23, 45, 23, 12, 23, 0, 12, 32]).unwrap();
        max_heap.heap_sort_by_max_heap();
        assert_eq!(
            max_heap.data,
            vec![0, 3, 5, 7, 9, 10, 12, 12, 23, 23, 23, 32, 45]
        );
    }

    #[test]
    fn test_build_min_heap() {
        let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 0, 23, 34, 56, 11, 230, 12]).unwrap();
        min_heap.heap_sort_by_min_heap();
        assert_eq!(min_heap.data, vec![230, 56, 34, 23, 12, 11, 3, 2, 1, 0]);
    }

    #[test]
    fn test_siftup_min_heap() {
        let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]).unwrap();
        min_heap.build_min_heap_by_siftup();
        assert_eq!(min_heap.data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_siftup_max_heap() {
        let mut max_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]).unwrap();
        max_heap.build_max_heap_by_shift_up();
        assert_eq!(max_heap.data, vec![5, 4, 2, 3, 1])
    }

    #[test]
    fn test_siftup_dec_sort() {
        let mut min_heap =
            Heap::from_vector(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 13, 14]).unwrap();
        min_heap.dec_sort_with_min_sift();
        assert_eq!(
            min_heap.data,
            vec![14, 13, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
    }

    #[test]
    fn test_siftup_asc_sort() {
        let mut max_heap = Heap::from_vector(&vec![9, 8, 7, 6, 5, 5, 4, 3, 2, 1, 0]).unwrap();
        max_heap.asc_sort_with_max_sift();
        assert_eq!(max_heap.data, vec![0, 1, 2, 3, 4, 5, 5, 6, 7, 8, 9]);
    }
}
