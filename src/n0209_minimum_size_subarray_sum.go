func minSubArrayLen(s int, nums []int) int {
	var left, right, sum int
	minimal := 1<<31 - 1
	for ; right < len(nums); right++ {
		sum += nums[right]

		for left < len(nums) && sum-nums[left] >= s {
			sum -= nums[left]
			left++
		}
		// fmt.Printf("left .. %v right .. %v sum .. %v\n", left, right, sum)

		if sum >= s && right-left+1 < minimal {
			minimal = right - left + 1
		}
	}
	if minimal == 1<<31-1 {
		return 0
	}
	return minimal
}
