// 和 153 相比主要是多了是否三个值一致的判断，如果三个值都一样的话
// 直接 l 和 r 都向中靠拢一格，所以时间复杂度相对于 153 会增加一点，
// 最差情况会退化到 O(N)

func findMin(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	l, r := 0, len(nums)-1
	for l < r {
		if nums[l] < nums[r] {
			return nums[l]
		}
		mid := (l + r) >> 1
		if nums[mid] == nums[l] && nums[mid] == nums[r] {
			l += 1
			r -= 1
			continue
		}
		if nums[mid] >= nums[l] {
			l = mid + 1
		} else {
			r = mid
		}
	}
	return nums[l]
}

// 这个是别人的解法，时间复杂度基本一致.
func findMinBeta(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	l, r := 0, len(nums)-1
	for l < r {
		mid := (l + r) >> 1
		if nums[mid] > nums[r] {
			l = mid + 1
		} else if nums[mid] < nums[r] {
			r = mid
		} else {
			r -= 1
		}
	}
	return nums[l]
}
