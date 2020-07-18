func permute(nums []int) [][]int {
	ret := make([][]int, 0, len(nums))
	cur := make([]int, 0, len(nums))
	cache := make(map[int]bool)
	backtrack(&ret, nums, cur, cache)
	return ret
}

func backtrack(ret *[][]int, nums, cur []int, cache map[int]bool) {
	if len(cur) == len(nums) {
		*ret = append(*ret, copy(cur))
		return
	}

	for i := 0; i < len(nums); i++ {
		if !cache[i] {
			cache[i] = true
			cur = append(cur, nums[i])
			backtrack(ret, nums, cur, cache)
			cur = cur[:len(cur)-1]
			delete(cache, i)
		}
	}
}

func copy(in []int) []int {
	ret := make([]int, len(in))
	for i, num := range in {
		ret[i] = num
	}
	return ret
}
