use std::fmt::{Debug, Display};

#[allow(dead_code)]
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

    pub fn min_siftup(&mut self, index: usize) {
         let mut i = index;
         loop {
            // if i is root idx will break
            if i == 0 {
                break;
            }

            // get parent index
            let p = parent(i);

            // when parent node <= child node will break
            if self.data[p] <= self.data[i] {
                break;
            }

            // swap parent node idx with child node idx
            self.data.swap(p,i);
            
            // now i is assign to has parent idx
            i = p;
         }    
    }


    pub fn max_siftup(&mut self, index: usize) {
        let mut i = index;
        loop {
           // if i is root idx will break
           if i == 0 {
               break;
           }

           // get parent index
           let p = parent(i);

           // when child node <= parent node will break
           if self.data[i] <= self.data[p] {
               break;
           }

           // swap parent node idx with child node idx
           self.data.swap(p,i);
           
           // now i is assign to has parent idx
           i = p;
        }    
   }

    /// build Max Heap
    pub fn build_max_heap(&mut self) {
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
    pub fn build_min_heap(&mut self) {
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
        self.build_max_heap();
        for index in (1..self.data.len()).rev() {
            self.data.swap(0, index);
            self.size -= 1;
            self.max_heapify(0);
        }
    }

    /// dec sort by Min Heap
    pub fn heap_sort_by_min_heap(&mut self) {
        self.build_min_heap();
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
    println!("vector = {:?}", vec_temp);
}

#[test]
fn test_build_max_heap() {
    let mut temp_heap = Heap::from_vector(&vec![5, 3, 7, 9, 10, 23, 45, 23, 12, 23, 0, 12, 32]);
    println!("temp Heap = {:?}", temp_heap);

    temp_heap.heap_sort();

    println!("temp Heap = {:?}", temp_heap);
}

#[test]
fn test_build_min_heap() {
    let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 0, 23, 34, 56, 11, 230, 12]);

    println!("min_heap = {:?}", min_heap);

    min_heap.build_min_heap();

    min_heap.heap_sort_by_min_heap();

    println!("min_heap = {:?}", min_heap);
}

#[test]
fn test_siftup_min_heap() {
    let mut min_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]);
    println!("min_heap = {:?}", min_heap);

    min_heap.build_min_heap_by_siftup();

    println!("min_heap = {:?}", min_heap);
}

#[test]
fn test_siftup_max_heap() {
    let mut max_heap = Heap::from_vector(&vec![3, 2, 1, 4, 5]);

    println!("max_heap = {:?}", max_heap);


    max_heap.build_max_heap_by_shift_up();

    println!("min_heap = {:?}", max_heap);
}
