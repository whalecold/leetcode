/**
 * [150] Evaluate Reverse Polish Notation
 *
 * Evaluate the value of an arithmetic expression in <a href="http://en.wikipedia.org/wiki/Reverse_Polish_notation" target="_blank">Reverse Polish Notation</a>.
 *
 * Valid operators are +, -, *, /. Each operand may be an integer or another expression.
 *
 * Note:
 *
 *
 * 	Division between two integers should truncate toward zero.
 * 	The given RPN expression is always valid. That means the expression would always evaluate to a result and there won't be any divide by zero operation.
 *
 *
 * Example 1:
 *
 *
 * Input: ["2", "1", "+", "3", "*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 *
 *
 * Example 2:
 *
 *
 * Input: ["4", "13", "5", "/", "+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 *
 *
 * Example 3:
 *
 *
 * Input: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
 * Output: 22
 * Explanation:
 *   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut deque = VecDeque::new();
        for i in tokens.iter() {
            if !i.eq("+") && !i.eq("-") && !i.eq("*") && !i.eq("/") {
                deque.push_back(i.parse::<i32>().unwrap());
                continue;
            }

            let second = deque.pop_back().unwrap();
            let first = deque.pop_back().unwrap();
            if i.eq("+") {
                deque.push_back(first + second);
                continue;
            }
            if i.eq("-") {
                deque.push_back(first - second);
                continue;
            }
            if i.eq("*") {
                deque.push_back(first * second);
                continue;
            }
            if i.eq("/") {
                deque.push_back(first / second);
                continue;
            }
        }
        *deque.back().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_150() {
        assert_eq!(
            Solution::eval_rpn(vec![
                String::from("4"),
                String::from("13"),
                String::from("5"),
                String::from("/"),
                String::from("+"),
            ]),
            6
        );
    }
}
