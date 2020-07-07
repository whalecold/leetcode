// dfs 2 means visited grid
// 时间复杂度和空间复杂度都是 O(m*n)
func closedIsland(grid [][]int) int {
	var sum int
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[i]); j++ {
			if grid[i][j] == 1 || grid[i][j] == 2 {
				continue
			}
			if helper(grid, i, j) {
				sum++
			}
		}
	}
	return sum
}

func helper(grid [][]int, row, col int) bool {
	if row < 0 || row >= len(grid) || col < 0 || col >= len(grid[row]) {
		return false
	}
	if grid[row][col] == 2 || grid[row][col] == 1 {
		return true
	}
	grid[row][col] = 2
	f1 := helper(grid, row+1, col)
	f2 := helper(grid, row, col+1)
	f3 := helper(grid, row-1, col)
	f4 := helper(grid, row, col-1)

	return f1 && f2 && f3 && f4
}
