// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
// The robot can only move either down or right at any point in time. The robot is trying to reach
// the bottom-right corner of the grid (marked 'Finish' in the diagram below).
// Now consider if some obstacles are added to the grids. How many unique paths would there be?

/// ## Example 1:
//
//  Input:
//  [
//    [0,0,0],
//    [0,1,0],
//    [0,0,0]
//  ]
//  Output: 2
//  Explanation:
//  There is one obstacle in the middle of the 3x3 grid above.
//  There are two ways to reach the bottom-right corner:
//  1. Right -> Right -> Down -> Down
//  2. Down -> Down -> Right -> Right

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut grids = vec![vec![0; n + 1]; m + 1];
        grids[1][0] = 1;
        for i in 1..=m {
            for j in 1..=n {
                if obstacle_grid[i - 1][j - 1] != 1 {
                    grids[i][j] = grids[i][j - 1] + grids[i - 1][j];
                }
            }
        }
        grids[m][n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_62() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![0, 0, 0]
            ]),
            1
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![1, 0]]),
            0
        );
    }
}
