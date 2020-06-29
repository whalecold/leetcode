// dp
func uniquePaths(m int, n int) int {
    dp := make([][]int, n)
    for i := range dp {
        dp[i] = make([]int, m)
    }
    dp[n-1][m-1] = 1
    for i := n-1; i >= 0; i -= 1 {
        for j := m-1; j >= 0; j -= 1 {
            if i == n-1 && j == m-1 {
                continue
            }
            if i == n-1 {
                dp[i][j] = dp[i][j+1]
            } else if j == m-1 {
                dp[i][j] = dp[i+1][j]
            } else {
                dp[i][j] = dp[i][j+1] + dp[i+1][j]
            }
        }
    }
    return dp[0][0]
}
