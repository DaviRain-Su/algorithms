use alloc::vec::Vec;

/// # stack data structure
/// 在栈中，被删除的是最近插入的元素： 栈的实现是一种后进先出策略。
/// 这里采用数组实现栈，还有别的方式也是可以实现栈的。
///
/// ## 栈的操作
/// 栈的操作有，栈上的INSERT操作称为压入PUSH,而无元素参数的DELETE操作称为弹出POP。
#[derive(Debug)]
pub struct Stack<T> {
    // data
    data: Vec<T>,
    // the stack top pointer, just the vector dynomic length
    top: usize,
}

impl<T: Clone> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
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
    /// let mut stack = Stack::<i32>::new();
    ///
    /// assert_eq!(stack.is_empty(), true);
    ///
    /// stack.push(2);
    ///
    /// assert_eq!(stack.is_empty(), false);
    /// ```
    /// ```no
    /// STACK-EMPTY(S)
    /// if S.top == 0
    ///     return true
    /// else return false
    /// ```
    pub fn is_empty(&self) -> bool {
        self.top == 0
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
    /// ```no
    /// PUSH(S, x)
    ///     S.top = S.top + 1
    ///     S[S.top] = x
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
    ///
    /// let element = stack.pop().unwrap();
    ///
    /// assert_eq!(element, 1);
    /// assert_eq!(stack.is_empty(), true);
    ///
    /// if let Err(err) = stack.pop() {
    ///  assert_eq!(err.to_string(), "underflow".to_string())
    /// }
    ///
    /// ```
    /// ```no
    /// POP(S)
    ///     if STACK-EMPTY(S)
    ///         error "underflow"
    ///     else
    ///         S.top = S.top - 1
    ///         return S[S.top + 1]
    /// ```
    pub fn pop(&mut self) -> anyhow::Result<T> {
        if self.is_empty() {
            Err(anyhow::anyhow!("underflow"))
        } else {
            self.top -= 1;
            Ok(self.data.remove(self.top))
        }
    }

    /// Return the top element of the stack
    ///
    /// ```rust
    /// use algorithms_rs::Stack;
    ///
    /// let mut stack = Stack::<i32>::new();
    ///
    /// assert_eq!(stack.peek(), None);
    ///
    /// stack.push(1);
    /// stack.push(2);
    ///
    /// assert_eq!(stack.peek(), Some(&2));
    /// assert_eq!(stack.is_empty(), false);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.data.get(self.top - 1)
    }

    /// the stack size
    /// ```rust
    /// use algorithms_rs::Stack;
    ///
    /// let stack = Stack::<i32>::new();
    ///
    /// assert_eq!(0, stack.size());
    /// ```
    pub fn size(&self) -> usize {
        self.top
    }
}
