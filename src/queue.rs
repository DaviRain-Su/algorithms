use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

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
    pub fn en_queue(&mut self, element: T) -> anyhow::Result<()> {
        if self.head == (self.tail + 1) % self.len {
            return Err(anyhow::anyhow!("overflow"));
        }
        if let Some(value) = self.data.get_mut(self.tail) {
            *value = element;
        } else {
            return Err(anyhow::anyhow!(format!(
                "get index of {} element",
                self.tail
            )));
        }

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
    pub fn de_queue(&mut self) -> anyhow::Result<T> {
        if self.is_empty() {
            return Err(anyhow::anyhow!("underflow"));
        }
        let element = self.data.get(self.head);
        if self.head == (self.len - 1) {
            self.head = 0;
        } else {
            self.head += 1;
        }
        Ok(element.unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_result<T>(result: Result<T, anyhow::Error>) {
        match result {
            Ok(_value) => assert!(true, "en_queue successful!"),
            Err(err) => {
                if err.to_string() == "overflow".to_string() {
                    assert_eq!(err.to_string(), "overflow".to_string());
                } else if err.to_string() == "underflow".to_string() {
                    assert_eq!(err.to_string(), "underflow".to_string());
                }
            }
        }
    }

    #[test]
    fn test_is_empty_queue() {
        let empty_queue = Queue::<i32>::new(3);
        assert_eq!(true, empty_queue.is_empty());
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
