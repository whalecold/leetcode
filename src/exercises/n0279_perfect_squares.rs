/**
 * [279] Perfect Squares
 *
 * Given a positive integer n,
 * find the least number of perfect square numbers (for example, 1, 4, 9, 16, ...) which sum to n.
 *
 * Example 1:
 *
 *
 * Input: n = 12
 * Output: 3
 * Explanation: 12 = 4 + 4 + 4.
 *
 * Example 2:
 *
 *
 * Input: n = 13
 * Output: 2
 * Explanation: 13 = 4 + 9.
 */
// TODO
#[allow(dead_code)]
static NEED_COMPILE: bool = true;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![std::i32::MAX; n + 1];
        let mut square  = 1;
        for i in 1..=n {
            if i == square * square {
                dp[i] = 1;
                square += 1;
            } else {
                for j in 1..=(square-1) {
                    dp[i] = i32::min(dp[j*j] + dp[i - j*j], dp[i]);
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_279() {
        assert_eq!(Solution::num_squares(2), 2);
        assert_eq!(Solution::num_squares(1), 1);
        assert_eq!(Solution::num_squares(8), 2);
        assert_eq!(Solution::num_squares(12), 3);
    }
}
