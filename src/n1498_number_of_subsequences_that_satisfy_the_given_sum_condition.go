import "sort"

// 刚开始想到了要先排序 然后利用 2^(r-l) 去算，但还是有各种问题,后面去看了别人的解法
// 发现想错了，在应该是考虑在满足 nums[l] + nums[r] <= targer 的条件下，一定存在 nums[l]
// 的值，再利用剩下的长度做次方
// sliding window
func numSubseq(nums []int, target int) int {
	if len(nums) == 0 {
		return 0
	}
	sort.Ints(nums)
	power := make([]int, len(nums))
	mod := int(1e9 + 7)
	power[0] = 1
	for i := 1; i < len(nums); i++ {
		power[i] = power[i-1] * 2 % mod
	}
	l, r, res := 0, len(nums)-1, 0
	for l <= r {
		if nums[l]+nums[r] > target {
			r--
		} else {
			res = (res + power[r-l]) % mod
			l++
		}
	}
	return res
}
