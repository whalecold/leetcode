import "sort"

func rangeSum(nums []int, n int, left int, right int) int {
	mod := int(1e9 + 7)
	ret := make([]int, 0, n*(n+1)/2)
	for i := 0; i < len(nums); i++ {
		ret = append(ret, nums[i])
		last := nums[i]
		for j := i + 1; j < len(nums); j++ {
			last += nums[j]
			ret = append(ret, last)
		}
	}
	var res int
	sort.Ints(ret)
	for i := left - 1; i < right; i++ {
		res += ret[i]
	}
	return res % mod
}
