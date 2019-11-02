/**
 * [264] Ugly Number II
 *
 * Write a program to find the n-th ugly number.
 *
 * Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.
 *
 * Example:
 *
 *
 * Input: n = 10
 * Output: 12
 * Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.
 *
 * Note:  
 *
 * <ol>
 * 	1 is typically treated as an ugly number.
 * 	n does not exceed 1690.
 * </ol>
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }
        let n = n as usize;
        let mut dp = vec![0; n];
        dp[0] = 1;
        let (mut t2, mut t3, mut t5) = (0, 0, 0);
        for i in 1..n {
            dp[i] = i32::min(dp[t5] * 5, i32::min(dp[t2] * 2, dp[t3] * 3));
            if dp[i] == dp[t2] * 2 {
                t2 += 1;
            }
            if dp[i] == dp[t3] * 3 {
                t3 += 1;
            }
            if dp[i] == dp[t5] * 5 {
                t5 += 1;
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        assert_eq!(Solution::nth_ugly_number(1), 1);
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(19), 32);
        assert_eq!(Solution::nth_ugly_number(23), 48);
        assert_eq!(Solution::nth_ugly_number(25), 54);
    }
}
