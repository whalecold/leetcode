// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
// The robot can only move either down or right at any point in time. The robot is trying to reach
// the bottom-right corner of the grid (marked 'Finish' in the diagram below).
// How many possible unique paths are there?

/// ## Note: m and n will be at most 100.
/// #### Example 1:
// Input: m = 3, n = 2
// Output: 3
// Explanation:
// From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
// 1. Right -> Right -> Down
// 2. Right -> Down -> Right
// 3. Down -> Right -> Right

/// #### Example 2:
// Input: m = 7, n = 3
// Output: 28

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        if m == 0 || n == 0 {
            return 1;
        }
        let mut grids = vec![vec![0; n + 1]; m + 1];
        grids[0][1] = 1;
        for i in 1..=m {
            for j in 1..=n {
                grids[i][j] = grids[i - 1][j] + grids[i][j - 1];
            }
        }
        grids[m][n]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn test_62() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }
}
