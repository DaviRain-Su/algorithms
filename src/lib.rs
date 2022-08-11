#![cfg_attr(not(feature = "std"), no_std)]
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
extern crate alloc;

/// heap sort module
pub mod heap_sort;
/// queue struct module
pub mod queue;
/// stack struct module
pub mod stack;
/// Merkle Tree
pub mod merkle_tree;

#[cfg(test)]
mod tests;

pub use heap_sort::Heap;
pub use queue::Queue;
pub use stack::Stack;