/**
 * [155] Min Stack
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
 *
 *
 * 	push(x) -- Push element x onto stack.
 * 	pop() -- Removes the element on top of the stack.
 * 	top() -- Get the top element.
 * 	getMin() -- Retrieve the minimum element in the stack.
 *
 *
 *  
 *
 * Example:
 *
 *
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin();   --> Returns -3.
 * minStack.pop();
 * minStack.top();      --> Returns 0.
 * minStack.getMin();   --> Returns -2.
 *
 *
 *  
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

struct MinStack {
    vec: Vec<i32>,
    min: i32,
}

// 可以使用两个数组或者一个数组加一个 min 值
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            vec: Vec::new(),
            min: i32::max_value(),
        }
    }

    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.vec.push(self.min);
            self.min = x;
        }
        self.vec.push(x);
    }

    fn pop(&mut self) {
        if self.vec.pop().unwrap() == self.min {
            self.min = self.vec.pop().unwrap();
        }
    }

    fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // --> Returns -3.
        min_stack.pop();
        assert_eq!(min_stack.top(), 0); // --> Returns 0.
        assert_eq!(min_stack.get_min(), -2); // --> Returns -2.[]
    }
}
