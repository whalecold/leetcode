/**
 * [32] Longest Valid Parentheses
 *
 * Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
 *
 * Example 1:
 *
 *
 * Input: "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()"
 *
 *
 * Example 2:
 *
 *
 * Input: ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()"
 *
 *
 */
use std::collections::VecDeque;
#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    // TODO optimize time O
    #[allow(dead_code)]
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut vector: VecDeque<char> = VecDeque::new();
        let mut slice = vec![0 as i32; s.len()];
        let mut longest = 0 as i32;
        let (left_parent, right_parent) = ('(', ')');
        for i in 0..s.len() {
            let ch = s.chars().nth(i).unwrap();
            if ch.eq(&right_parent) && !vector.is_empty() {
                let last = vector.back().unwrap();
                if last.eq(&left_parent) {
                    slice[i] = slice[i - 1] + 2;
                    if i > slice[i] as usize {
                        slice[i] += slice[i - slice[i] as usize];
                    }
                    if slice[i] > longest {
                        longest = slice[i]
                    }
                    vector.pop_back();
                    continue;
                }
            }
            vector.push_back(ch);
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_32() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("()()(")),
            4
        );
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("()(()(")),
            2
        );
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("())(()(")),
            2
        );
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("(((((((")),
            0
        );
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("()(())")),
            6
        );
    }
}
