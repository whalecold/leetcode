/**
 * [258] Add Digits
 *
 * Given a non-negative integer num, repeatedly add all its digits until the result has only one digit.
 *
 * Example:
 *
 *
 * Input: 38
 * Output: 2
 * Explanation: The process is like: 3 + 8 = 11, 1 + 1 = 2.
 *              Since 2 has only one digit, return it.
 *
 *
 * Follow up:<br />
 * Could you do it without any loop/recursion in O(1) runtime?
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn add_digits(num: i32) -> i32 {
        1 + (num - 1) % 9
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_258() {
        assert_eq!(Solution::add_digits(38), 2);
        assert_eq!(Solution::add_digits(2), 2);
        assert_eq!(Solution::add_digits(18), 9);
        assert_eq!(Solution::add_digits(28), 1);
    }
}
