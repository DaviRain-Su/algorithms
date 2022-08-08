use alloc::vec::Vec;

/// Stack struct
#[derive(Debug)]
pub struct Stack<T> {
    // data
    data: Vec<T>,
    // the stack top pointer
    top: usize,
}

impl<T: Clone> Stack<T> {
    /// construct stack
    pub fn new() -> Stack<T> {
        Self {
            data: Vec::new(),
            top: 0,
        }
    }

    /// the stack is empty
    pub fn is_empty(&self) -> bool {
        if self.top == 0 {
            true
        } else {
            false
        }
    }

    /// push one element to stack
    pub fn push(&mut self, element: T) {
        self.top += 1;
        self.data.push(element);
    }

    /// pop the top element from stack
    pub fn pop(&mut self) -> anyhow::Result<T> {
        if self.is_empty() {
            return Err(anyhow::anyhow!("underflow"));
        } else {
            self.top -= 1;
            let element = self.data.remove(self.top);
            return Ok(element.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

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
