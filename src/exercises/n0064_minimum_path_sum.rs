/// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom
/// right which minimizes the sum of all numbers along its path.
/// Note: You can only move either down or right at any point in time.
/// ## Example:
/// ```
/// Input:
/// [
///   [1,3,1],
///   [1,5,1],
///   [4,2,1]
/// ]
/// Output: 7
/// Explanation: Because the path 1→3→1→1→1 minimizes the sum.

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![std::i32::MAX; n + 1]; m + 1];
        dp[0][1] = 0;
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i][j - 1]) + grid[i - 1][j - 1];
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_64() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        )
    }
}
