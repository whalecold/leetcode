// 用了 dfs 的方法解的
// 利用相邻的两个 1 之间会重叠部分边界的方法
func islandPerimeter(grid [][]int) int {
	if len(grid) == 0 {
		return 0
	}
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == 1 {
				return helper(grid, i, j)
			}
		}
	}
	return 0
}

func helper(grid [][]int, x, y int) int {
	if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[x]) || grid[x][y] == 0 {
		return 1
	}
	if grid[x][y] == 3 {
		return 0
	}
	grid[x][y] = 3
	return helper(grid, x, y+1) + helper(grid, x, y-1) + helper(grid, x+1, y) + helper(grid, x-1, y)
}
