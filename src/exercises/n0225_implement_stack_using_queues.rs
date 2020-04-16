/**
 * [225] Implement Stack using Queues
 *
 * Implement the following operations of a stack using queues.
 *
 *
 * 	push(x) -- Push element x onto stack.
 * 	pop() -- Removes the element on top of the stack.
 * 	top() -- Get the top element.
 * 	empty() -- Return whether the stack is empty.
 *
 *
 * Example:
 *
 *
 * MyStack stack = new MyStack();
 *
 * stack.push(1);
 * stack.push(2);  
 * stack.top();   // returns 2
 * stack.pop();   // returns 2
 * stack.empty(); // returns false
 *
 * Notes:
 *
 *
 * 	You must use only standard operations of a queue -- which means only push to back, peek/pop from front, size, and is empty operations are valid.
 * 	Depending on your language, queue may not be supported natively. You may simulate a queue by using a list or deque (double-ended queue), as long as you use only standard operations of a queue.
 * 	You may assume that all operations are valid (for example, no pop or top operations will be called on an empty stack).
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}
use std::collections::VecDeque;
struct MyStack {
    stack: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    #[allow(dead_code)]
    fn new() -> Self {
        MyStack {
            stack: VecDeque::new(),
        }
    }

    /** Push element x onto stack. */
    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.stack.push_back(x);
        let size = self.stack.len() - 1;
        for _i in 0..size {
            let front = self.stack.pop_front().unwrap();
            self.stack.push_back(front);
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        self.stack.pop_front().unwrap()
    }

    /** Get the top element. */
    #[allow(dead_code)]
    fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    /** Returns whether the stack is empty. */
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {

    #[test]
    fn test_225() {}
}
