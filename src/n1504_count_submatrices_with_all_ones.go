func numSubmat(mat [][]int) int {
	if len(mat) == 0 {
		return 0
	}
	var res int
	dp := make([]int, len(mat[0]))
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[i]); j++ {
			if mat[i][j] == 0 {
				dp[j] = 0
			} else {
				dp[j] += 1
			}
		}
		res += stackHelper(dp)
	}
	return res
}

func stackHelper(dp []int) int {
	var res int
	stack := make([]int, 0, len(dp))
	sum := make([]int, len(dp))
	for i := 0; i < len(dp); i++ {
		for len(stack) != 0 && dp[stack[len(stack)-1]] >= dp[i] {
			stack = stack[:len(stack)-1]
		}
		if len(stack) == 0 {
			sum[i] = dp[i] * (i + 1)
		} else {
			preIndex := stack[len(stack)-1]
			sum[i] = sum[preIndex]
			sum[i] += dp[i] * (i - preIndex)
		}
		stack = append(stack, i)
	}

	for i := range sum {
		res += sum[i]
	}
	return res
}

//--------------------- 这里是分割线-----------------------------

// 这也是一个 dp 问题，时间复杂度是 O(M*N*N)
// discuss 里时间复杂度可以优化到 O(M*N) 用的是单调栈，空间复杂度优化到 0(M) tql!!!
func numSubmat1(mat [][]int) int {
	if len(mat) == 0 {
		return 0
	}
	dp := make([][]int, len(mat)+1)
	for i := range dp {
		dp[i] = make([]int, len(mat[0])+1)
	}
	var res int
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[i]); j++ {
			if mat[i][j] == 1 {
				dp[i+1][j+1] = dp[i+1][j] + 1
				res += countCol(dp, i+1, j+1, dp[i+1][j+1])
			}
		}
	}
	return res
}

func countCol(dp [][]int, row, col, max int) int {
	var res int
	for i := row; i >= 0 && dp[i][col] != 0; i-- {
		if dp[i][col] > max {
			res += max
		} else {
			res += dp[i][col]
			max = dp[i][col]
		}

	}
	return res
}
