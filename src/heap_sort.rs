use std::fmt::{Debug, Display};

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
    /// create a heap from vector
    pub fn from_vector(array: &[T]) -> Self {
        Self {
            data: array.into(),
            size: array.len() - 1,
        }
    }

    /// the present heap size
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// max heap heapify
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

    /// the min heap heapify
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

    /// the min heap siftup
    pub fn min_siftup(&mut self, index: usize) {
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

    /// the max heap siftup
    pub fn max_siftup(&mut self, index: usize) {
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

    /// the min shift down
    #[allow(dead_code)]
    fn min_sift_down(&mut self, heap_len: usize) {
        let mut cur_idx = 0usize;
        loop {
            // get cur_idx has left child idx
            let mut child_idx = left(cur_idx);

            if cur_idx > heap_len || child_idx >= heap_len {
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

    /// build Max Heap
    pub fn build_max_heap_by_max_heapify(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.max_heapify(index);
        }
    }

    /// build Max Heap
    pub fn build_max_heap_by_shift_up(&mut self) {
        for index in (0..self.data.len()).rev() {
            self.max_siftup(index);
        }
    }

    /// build Min Heap
    pub fn build_min_heap_by_min_heapify(&mut self) {
        for index in (0..(self.len() / 2)).rev() {
            self.min_heapify(index);
        }
    }

    pub fn build_min_heap_by_siftup(&mut self) {
        for index in (0..self.data.len()).rev() {
            self.min_siftup(index);
        }
    }

    /// asc sort by Max Heap
    pub fn heap_sort(&mut self) {
        self.build_max_heap_by_max_heapify();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size -= 1;
            self.max_heapify(0);
        }
    }

    /// dec sort by Min Heap
    pub fn heap_sort_by_min_heap(&mut self) {
        self.build_min_heap_by_min_heapify();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size -= 1;
            self.min_heapify(0);
        }
    }
}

#[test]
fn test_replace() {
    let mut vec_temp = vec![1, 2, 3];
    vec_temp.swap(0, 1);
    assert_eq!(vec_temp, vec![2, 1, 3]);
}

#[test]
fn test_build_max_heap() {
    let mut temp_heap = Heap::from_vector(&vec![5, 3, 7, 9, 10, 23, 45, 23, 12, 23, 0, 12, 32]);
    temp_heap.heap_sort();
    assert_eq!(temp_heap.data, vec![0, 3, 5, 7, 9, 10, 12, 12, 23, 23, 23, 32, 45]);
}

#[test]
fn test_build_min_heap() {
    let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 0, 23, 34, 56, 11, 230, 12]);
    min_heap.heap_sort_by_min_heap();
    assert_eq!(min_heap.data, vec![230, 56, 34, 23, 12, 11, 3, 2, 1, 0]);
}

#[test]
fn test_siftup_min_heap() {
    let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]);
    min_heap.build_min_heap_by_siftup();
    assert_eq!(min_heap.data, vec![1, 3, 2, 4, 5]);
}

#[test]
fn test_siftup_max_heap() {
    let mut max_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]);
    max_heap.build_max_heap_by_shift_up();
    assert_eq!(max_heap.data, vec![5, 4, 2, 3, 1])
}