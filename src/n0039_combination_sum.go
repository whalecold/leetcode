// backtrack
func combinationSum(candidates []int, target int) [][]int {
	ret := make([][]int, 0, len(candidates))
	cur := make([]int, 0, len(candidates))
	backtrack(&ret, candidates, cur, 0, target, 0)
	return ret
}

func backtrack(ret *[][]int, candidates, cur []int, curNum, target, index int) {
	if curNum == target {
		*ret = append(*ret, copy(cur))
		return
	}
	if curNum > target {
		return
	}
	for i := index; i < len(candidates); i++ {
		cur = append(cur, candidates[i])
		backtrack(ret, candidates, cur, curNum+candidates[i], target, i)
		cur = cur[:len(cur)-1]
	}
}

func copy(in []int) []int {
	ret := make([]int, len(in))
	for i, num := range in {
		ret[i] = num
	}
	return ret
}
