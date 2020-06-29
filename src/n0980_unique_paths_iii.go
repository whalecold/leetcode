// backtrack 尝试下暴力解法..
// 真是丑陋的写法啊！！！勉强过了
func uniquePathsIII(grid [][]int) int {
	var end []int
	var steps, ret, x, y int

	for i := 0; i < len(grid); i += 1 {
		for j := 0; j < len(grid[i]); j += 1 {
			if grid[i][j] == 1 {
				x, y = i, j
			} else if grid[i][j] == 2 {
				end = []int{i, j}
			} else if grid[i][j] != -1 {
				steps += 1
			}
		}
	}
	steps += 1
	grid[x][y] = 3
	backtrack(grid, x+1, y, end, 1, steps, &ret)
	backtrack(grid, x, y+1, end, 1, steps, &ret)
	backtrack(grid, x-1, y, end, 1, steps, &ret)
	backtrack(grid, x, y-1, end, 1, steps, &ret)
	return ret
}

func backtrack(grid [][]int, x, y int, end []int, curStep, steps int, ret *int) {

	// 检查是否越界
	if x < 0 || y < 0 || x >= len(grid) || y >= len(grid[x]) {
		return
	}
	// 不能通过的地方也返回
	if grid[x][y] == -1 {
		return
	}
	// 检查是否已经走过
	if grid[x][y] == 3 {
		return
	}
	// 到达终点检查下是否走完了全部路程
	if x == end[0] && y == end[1] {
		if curStep == steps {
			*ret += 1
		}
		return
	}

	grid[x][y] = 3
	backtrack(grid, x+1, y, end, curStep+1, steps, ret)
	backtrack(grid, x, y+1, end, curStep+1, steps, ret)
	backtrack(grid, x-1, y, end, curStep+1, steps, ret)
	backtrack(grid, x, y-1, end, curStep+1, steps, ret)
	grid[x][y] = 0
}
