// https://leetcode.com/problems/largest-rectangle-in-histogram/ 这个问题的变种
// 两种解法， 还有一种是 dp
// 这种的时间复杂度是 O(M*N)
func maximalRectangle(matrix [][]byte) int {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return 0
	}
	stack := make([]int, 0, len(matrix))
	dp := make([]int, len(matrix[0]))
	var max int
	for i := 0; i < len(matrix); i++ {
		stack = stack[:0]
		for j := 0; j <= len(matrix[i]); j++ {
			var min int
			if j < len(matrix[i]) {
				if matrix[i][j] == '0' {
					dp[j] = 0
				} else {
					dp[j] += 1
				}
				min = dp[j]
			}

			for len(stack) != 0 && dp[stack[len(stack)-1]] >= min {
				h := dp[stack[len(stack)-1]]
				stack = stack[:len(stack)-1]
				w := j
				if len(stack) != 0 {
					w = j - stack[len(stack)-1] - 1
				}
				if w*h > max {
					max = w * h
				}
			}
			stack = append(stack, j)
		}
	}
	return max
}

//https://leetcode.com/problems/maximal-rectangle/discuss/29054/Share-my-DP-solution 另外一种解法 在看了..
