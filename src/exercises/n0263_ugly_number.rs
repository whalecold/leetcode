/**
 * [263] Ugly Number
 *
 * Write a program to check whether a given number is an ugly number.
 *
 * Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.
 *
 * Example 1:
 *
 *
 * Input: 6
 * Output: true
 * Explanation: 6 = 2 &times; 3
 *
 * Example 2:
 *
 *
 * Input: 8
 * Output: true
 * Explanation: 8 = 2 &times; 2 &times; 2
 *
 *
 * Example 3:
 *
 *
 * Input: 14
 * Output: false
 * Explanation: 14 is not ugly since it includes another prime factor 7.
 *
 *
 * Note:
 *
 * <ol>
 * 	1 is typically treated as an ugly number.
 * 	Input is within the 32-bit signed integer range: [-2^31,  2^31 - 1].
 * </ol>
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_ugly(num: i32) -> bool {
        if num <= 0 {
            return false;
        }
        let mut num = num;
        for i in [2, 3, 5].iter() {
            while num % *i == 0 {
                num /= *i;
            }
        }
        num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_263() {
        assert_eq!(Solution::is_ugly(8), true);
        assert_eq!(Solution::is_ugly(1), true);
        assert_eq!(Solution::is_ugly(0), false);
        assert_eq!(Solution::is_ugly(63), false);
        assert_eq!(Solution::is_ugly(64), true);
        assert_eq!(Solution::is_ugly(14), false);
    }
}