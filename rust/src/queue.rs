use std::vec;
use std::vec::Vec;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Custom Error(`0`)")]
    Custom(String),
    #[error("Queue overflow")]
    Overflow,
    #[error("Queue underflow")]
    Underflow,
    #[error("failed get value index on {0}")]
    FailedGetValue(usize),
}

/// Queue data structure
#[derive(Debug)]
pub struct Queue<T> {
    // data
    data: Vec<T>,
    // the queue head pointer
    head: usize,
    // the queue tail pointer
    tail: usize,
    // the queue size
    len: usize,
}

impl<T: Clone + Default> Queue<T> {
    /// Create an empty queue of fixed size
    ///
    /// ```rust
    /// use algorithms_rs::Queue;
    ///
    /// let empty_queue = Queue::<i32>::new(1);
    ///
    /// assert_eq!(empty_queue.is_empty(), true);
    /// ```
    pub fn new(length: usize) -> Self {
        let data = vec![T::default(); length];
        Self {
            data,
            head: 0,
            tail: 0,
            len: length,
        }
    }

    /// Determine if queue is empty
    ///
    /// ```rust
    /// use algorithms_rs::Queue;
    ///
    /// let empty_queue = Queue::<i32>::new(1);
    ///
    /// assert_eq!(empty_queue.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Enter the queue from the end of the queue
    ///
    /// ```rust
    /// use algorithms_rs::Queue;
    ///
    /// let mut queue = Queue::<i32>::new(3);
    ///
    /// queue.en_queue(1);
    /// queue.en_queue(2);
    ///
    /// assert_eq!(queue.is_empty(), false);
    /// ```
    pub fn en_queue(&mut self, element: T) -> Result<(), QueueError> {
        if self.head == (self.tail + 1) % self.len {
            return Err(QueueError::Overflow);
        }

        *self
            .data
            .get_mut(self.tail)
            .ok_or(QueueError::FailedGetValue(self.tail))? = element;

        if self.tail == (self.len - 1) {
            self.tail = 0;
        } else {
            self.tail += 1;
        }

        Ok(())
    }

    /// From the head of the queue Out of the queue
    ///
    /// ```rust
    /// use algorithms_rs::Queue;
    ///
    /// let mut queue = Queue::<i32>::new(3);
    ///
    /// queue.en_queue(1);
    /// queue.en_queue(2);
    ///
    /// queue.de_queue();
    /// queue.de_queue();
    ///
    /// assert_eq!(queue.is_empty(), true);
    /// ```
    pub fn de_queue(&mut self) -> Result<T, QueueError> {
        if self.is_empty() {
            return Err(QueueError::Underflow);
        }

        if self.head == (self.len - 1) {
            self.head = 0;
        } else {
            self.head += 1;
        }

        self.data
            .get(self.head)
            .ok_or(QueueError::FailedGetValue(self.head))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_result<T>(result: Result<T, QueueError>) {
        match result {
            Ok(_value) => {}
            Err(err) => match err {
                QueueError::Overflow => {
                    assert_eq!(err.to_string(), "Queue overflow".to_string());
                }
                QueueError::Underflow => {
                    assert_eq!(err.to_string(), "Queue underflow".to_string());
                }
                e => {
                    panic!("{e:?}")
                }
            },
        }
    }

    #[test]
    fn test_is_empty_queue() {
        let empty_queue = Queue::<i32>::new(3);
        assert!(empty_queue.is_empty());
    }

    #[test]
    fn test_is_enqueue_success() {
        let mut queue = Queue::<i32>::new(3);
        let ret = queue.en_queue(1);
        process_result(ret);
        let ret = queue.en_queue(2);
        process_result(ret);
    }

    #[test]
    fn test_is_enqueue_overflow() {
        let mut queue = Queue::<i32>::new(3);
        let ret = queue.en_queue(1);
        process_result(ret);
        let ret = queue.en_queue(2);
        process_result(ret);
        let ret = queue.en_queue(3);
        process_result(ret);
    }

    #[test]
    fn test_is_dequeue_success() {
        let mut queue = Queue::<i32>::new(3);
        let ret = queue.en_queue(1);
        process_result(ret);
        let ret = queue.en_queue(2);
        process_result(ret);
        let ret = queue.de_queue();
        process_result(ret);
        let ret = queue.de_queue();
        process_result(ret);
        let ret = queue.en_queue(4);
        process_result(ret);
        let ret = queue.de_queue();
        process_result(ret);
        let ret = queue.en_queue(5);
        process_result(ret);
        let ret = queue.en_queue(6);
        process_result(ret);
        let ret = queue.de_queue();
        process_result(ret);
        let ret = queue.de_queue();
        process_result(ret);
        let ret = queue.en_queue(7);
        process_result(ret);
        let ret = queue.en_queue(8);
        process_result(ret);
    }
}
