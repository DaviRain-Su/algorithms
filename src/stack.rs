use alloc::vec::Vec;

/// stack data structure
#[derive(Debug)]
pub struct Stack<T> {
    // data
    data: Vec<T>,
    // the stack top pointer, just the vector dynomic length
    top: usize,
}

impl<T: Clone> Stack<T> {
    /// Creating an empty stack
    ///
    /// ```rust
    /// use algorithms_rs::Stack;
    ///
    /// let stack = Stack::<i32>::new();
    ///
    /// assert_eq!(stack.is_empty(), true);
    /// ```
    pub fn new() -> Stack<T> {
        Self {
            data: Vec::new(),
            top: 0,
        }
    }

    /// Determine if stack is empty
    ///
    /// ```rust
    /// use algorithms_rs::Stack;
    ///
    /// let stack = Stack::<i32>::new();
    ///
    /// assert_eq!(stack.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        if self.top == 0 {
            true
        } else {
            false
        }
    }

    /// Put an element into the top of the stack of stack
    ///
    /// ```rust
    /// use algorithms_rs::Stack;
    ///
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(1);
    ///
    /// assert_eq!(stack.is_empty(), false);
    /// ```
    pub fn push(&mut self, element: T) {
        self.top += 1;
        self.data.push(element);
    }

    /// Remove an element from the top of the stack of stack
    ///
    /// ```rust
    /// use algorithms_rs::Stack;
    ///
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(1);
    /// let element = stack.pop().unwrap();
    ///
    /// assert_eq!(element, 1);
    /// assert_eq!(stack.is_empty(), true);
    /// ```
    pub fn pop(&mut self) -> anyhow::Result<T> {
        if self.is_empty() {
            return Err(anyhow::anyhow!("underflow"));
        } else {
            self.top -= 1;
            let element = self.data.remove(self.top);
            return Ok(element.clone());
        }
    }

    /// Return the top element of the stack
    ///
    /// ```rust
    /// use algorithms_rs::Stack;
    ///
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(1);
    /// stack.push(2);
    ///
    /// assert_eq!(stack.peek(), Some(&2));
    /// assert_eq!(stack.is_empty(), false);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.data.get(self.top - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty_success() {
        let empty_stack = Stack::<i32>::new();
        assert_eq!(true, empty_stack.is_empty());
    }

    #[test]
    fn test_is_empty_faild() {
        let mut empty_stack = Stack::<i32>::new();
        empty_stack.push(1);
        assert_eq!(false, empty_stack.is_empty());
    }

    #[test]
    fn test_pop_element_success() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        let ret = stack.pop().unwrap();
        assert_eq!(ret, 4);
        assert_eq!(false, stack.is_empty());
    }

    #[test]
    fn test_pop_element_faild() {
        let mut stack = Stack::<i32>::new();
        let ret = stack.pop();
        match ret {
            Ok(_) => todo!(),
            Err(err) => assert_eq!(err.to_string(), "underflow".to_string()),
        }
    }
}
