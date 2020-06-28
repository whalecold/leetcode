// 二分法
func findMin(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	l, r := 0, len(nums)-1
	if nums[l] < nums[r] {
		return nums[0]
	}
	for l < r {
		mid := (l + r) >> 1
		// 当中间值大于两端 那么 pivot 必然位于 mid 的右边
		// 这里要注意下 mid 可能会等于 l 的边界情况
		if nums[mid] >= nums[l] && nums[mid] > nums[r] {
			l = mid + 1
		} else if nums[mid] < nums[r] {
			// 如果中间值比 r 小 那么 r 肯定不是最小值，所以把 r 移动到 mid
			r = mid
		}
	}
	return nums[l]
}
