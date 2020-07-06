// 优化后的版本 空间复杂度是 O(N)
func countSquares(matrix [][]int) int {
	if len(matrix) == 0 {
		return 0
	}
	col := len(matrix[0])
	dp := make([]int, col+1)
	var total int
	for i := 0; i < len(matrix); i++ {
		var last int
		for j := 0; j < col; j++ {
			if matrix[i][j] == 1 {
				temp := dp[j+1]
				dp[j+1] = min(min(dp[j+1], dp[j]), last) + 1
				last = temp
				total += dp[j+1]
			} else {
				// 对于空间做优化的一定要注意，需要清空数据 不能像没有优化一样就放着不管
				dp[j+1] = 0
			}
		}
	}
	return total
}

// 这个是最原始的版本，空间复杂度是可以优化的。
func countSquares1(matrix [][]int) int {
	row := len(matrix)
	col := len(matrix[0])
	dp := make([][]int, row+1)
	for i := range dp {
		dp[i] = make([]int, col+1)
	}
	var total int
	for i := 1; i <= row; i += 1 {
		for j := 1; j <= col; j += 1 {
			if matrix[i-1][j-1] == 1 {
				dp[i][j] = min(min(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1]) + 1
				// fmt.Printf("dp[i][j] %v  i %v j %v\n", dp[i][j], i, j)
				total += dp[i][j]
			}
		}
	}
	return total
}

func min(i, j int) int {
	if i < j {
		return i
	}
	return j
}
