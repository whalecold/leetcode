/**
 * [343] Integer Break
 *
 * Given a positive integer n, break it into the sum of at least two positive integers and maximize the product of those integers. Return the maximum product you can get.
 *
 * Example 1:
 *
 * <div>
 *
 * Input: <span id="example-input-1-1">2</span>
 * Output: <span id="example-output-1">1</span>
 * Explanation: 2 = 1 + 1, 1 &times; 1 = 1.
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">10</span>
 * Output: <span id="example-output-2">36</span>
 * Explanation: 10 = 3 + 3 + 4, 3 &times; 3 &times; 4 = 36.
 *
 * Note: You may assume that n is not less than 2 and not larger than 58.
 * </div>
 * </div>
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn integer_break(n: i32) -> i32 {
        if n <= 2 {
            return 1;
        }
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[2] = 1;
        dp[1] = 1;
        for i in 3..=n {
            for j in 2..=i / 2 {
                dp[i] = i32::max(j as i32 * i32::max(dp[i - j], (i - j) as i32), dp[i]);
            }
        }
        dp[n]
    }
    // 这个还有写特殊解法，链接如下：https://leetcode.com/problems/integer-break/discuss/80721/Why-factor-2-or-3-The-math-behind-this-problem.
    // 这个解法不属于常规解法，就不写在这里了。
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_343() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
