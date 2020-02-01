use std::collections::{HashMap, VecDeque};

/**
 * [20] Valid Parentheses
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 * <ol>
 * 	Open brackets must be closed by the same type of brackets.
 * 	Open brackets must be closed in the correct order.
 * </ol>
 *
 * Note that an empty string is also considered valid.
 *
 * Example 1:
 *
 *
 * Input: "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: "(]"
 * Output: false
 *
 *
 * Example 4:
 *
 *
 * Input: "([)]"
 * Output: false
 *
 *
 * Example 5:
 *
 *
 * Input: "{[]}"
 * Output: true
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let mut parentheses = HashMap::new();
        parentheses.insert(']', '[');
        parentheses.insert(')', '(');
        parentheses.insert('}', '{');
        let mut stack: VecDeque<char> = VecDeque::new();
        for i in 0..s.len() {
            let c1 = s.chars().nth(i).unwrap();
            if let Some(c3) = parentheses.get(&c1) {
                if stack.len() == 0 {
                    return false
                }
                let c2 = stack.pop_back().unwrap();
                if !c2.eq(c3) {
                    return false;
                }
            } else {
                stack.push_back(c1.to_owned());
            }
        }

        if stack.len() == 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("")), true);
        assert_eq!(Solution::is_valid(String::from("{]")), false);
        assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    }
}
