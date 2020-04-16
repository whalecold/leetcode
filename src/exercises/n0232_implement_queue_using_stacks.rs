/**
 * [232] Implement Queue using Stacks
 *
 * Implement the following operations of a queue using stacks.
 *
 *
 * 	push(x) -- Push element x to the back of queue.
 * 	pop() -- Removes the element from in front of queue.
 * 	peek() -- Get the front element.
 * 	empty() -- Return whether the queue is empty.
 *
 *
 * Example:
 *
 *
 * MyQueue queue = new MyQueue();
 *
 * queue.push(1);
 * queue.push(2);  
 * queue.peek();  // returns 1
 * queue.pop();   // returns 1
 * queue.empty(); // returns false
 *
 * Notes:
 *
 *
 * 	You must use only standard operations of a stack -- which means only push to top, peek/pop from top, size, and is empty operations are valid.
 * 	Depending on your language, stack may not be supported natively. You may simulate a stack by using a list or deque (double-ended queue), as long as you use only standard operations of a stack.
 * 	You may assume that all operations are valid (for example, no pop or peek operations will be called on an empty queue).
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::VecDeque;
struct MyQueue {
    input: VecDeque<i32>,
    output: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    #[allow(dead_code)]
    fn new() -> Self {
        MyQueue {
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    /** Push element x onto stack. */
    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.input.push_back(x)
    }

    /** Removes the element on top of the stack and returns that element. */
    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        if self.empty() {
            return 0;
        }
        if !self.output.is_empty() {
            return self.output.pop_back().unwrap();
        }
        while !self.input.is_empty() {
            self.output.push_back(self.input.pop_back().unwrap())
        }
        return self.output.pop_back().unwrap();
    }

    /** Get the peek element. */
    #[allow(dead_code)]
    fn peek(&mut self) -> i32 {
        if self.empty() {
            return 0;
        }
        if !self.output.is_empty() {
            return *self.output.back().unwrap();
        }
        while !self.input.is_empty() {
            self.output.push_back(self.input.pop_back().unwrap())
        }
        return *self.output.back().unwrap();
    }

    /** Returns whether the stack is empty. */
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        return self.input.is_empty() && self.output.is_empty();
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    #[test]
    fn test_232() {}
}
