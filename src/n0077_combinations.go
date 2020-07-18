// 回溯算法
func combine(n int, k int) [][]int {
	ret := make([][]int, 0, n)
	cur := make([]int, 0, n)
	backtrack(&ret, cur, 1, k, n)
	return ret
}

func backtrack(ret *[][]int, current []int, index, k, max int) {
	if k == 0 {
		*ret = append(*ret, copy(current))
		return
	}
	if index > max {
		return
	}
	for i := index; i <= max; i++ {
		// backtrack(ret, current, i+1, k, max)
		current = append(current, i)
		backtrack(ret, current, i+1, k-1, max)
		current = current[:len(current)-1]
	}
}

func copy(in []int) []int {
	ret := make([]int, len(in))
	for i, num := range in {
		ret[i] = num
	}
	return ret
}
