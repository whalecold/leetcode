/**
 * [202] Happy Number
 *
 * Write an algorithm to determine if a number is "happy".
 *
 * A happy number is a number defined by the following process: Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.
 *
 * Example:
 *
 *
 * Input: 19
 * Output: true
 * Explanation:
 * 1^2 + 9^2 = 82
 * 8^2 + 2^2 = 68
 * 6^2 + 8^2 = 100
 * 1^2 + 0^2 + 0^2 = 1
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_happy(n: i32) -> bool {
        let (mut fast, mut slow) = (n, n);
        let digit_square_sum = |x: i32| -> i32 {
            let (mut sum, mut x) = (0, x.clone());
            while x != 0 {
                sum += (x % 10) * (x % 10);
                x /= 10;
            }
            sum
        };
        loop {
            // 这里的主要思路就是怎么检测出有环，检测环的方法就是分一个慢指针，一个快指针，一个始终走两步，
            // 一个始终走一步，走到一样的就表示有环！！！
            slow = digit_square_sum(slow);
            fast = digit_square_sum(fast);
            fast = digit_square_sum(fast);
            if fast == slow {
                break;
            }
        }
        fast == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert_eq!(Solution::is_happy(0), false);
        assert_eq!(Solution::is_happy(2), false);
        assert_eq!(Solution::is_happy(9), false);
        assert_eq!(Solution::is_happy(1), true);
        assert_eq!(Solution::is_happy(19), true);
    }
}
