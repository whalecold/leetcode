func uniquePathsWithObstacles(obstacleGrid [][]int) int {
    if len(obstacleGrid) == 0 || len(obstacleGrid[0]) == 0 {
        return 0
    }
    m, n := len(obstacleGrid), len(obstacleGrid[0])
    dp := make([][]int, m+1)
    for i := range dp {
        dp[i] = make([]int, n+1)
    }

    for i := m-1; i >= 0; i -= 1 {
        for j := n-1; j >= 0; j -= 1 {
            if i == m-1 && j == n-1 {
                if obstacleGrid[i][j] == 1 {
                    return 0
                }
                dp[i][j] = 1
                continue
            }
            if obstacleGrid[i][j] == 1 {
                continue
            }
            dp[i][j] = dp[i+1][j] + dp[i][j+1]
        }
    }
    return dp[0][0]
}
