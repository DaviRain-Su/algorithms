#![allow(clippy::borrowed_box)]
#![allow(clippy::only_used_in_recursion)]

//! # Introduction to algorithms
//! > thrid edition implement by rust programming
//!
//! ## Now Implement
//!    - heap sort algorithm
//!        - Max Heap
//!             - Asc sort by Max Heap,`asc_sort_with_max_sift`, `heap_sort_by_max_heap`
//!        - Min Heap
//!             - Dec sort by Min Heap, `dec_sort_with_min_sift`, `heap_sort_by_min_heap`
//!    - stack
//!         - push top element
//!         - pop top element
//!    - queue
//!         - push queue tail element
//!         - pop queue head element
//!

/// heap sort module
pub mod heap;
/// queue struct module
pub mod queue;
/// stack struct module
pub mod stack;

/// sort algorithm
pub mod sort;

/// search algorithm
pub mod search;

/// data struct
pub mod datastruct;

#[cfg(test)]
mod tests;
pub mod utils;

pub mod chapter4;

pub use heap::Heap;
pub use queue::Queue;
pub use stack::Stack;
