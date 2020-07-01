// 现在的时间复杂度是 O(len(num) * k)
// 这有个优点就是 in-place
func removeKdigits1(num string, k int) string {
	if len(num) == k {
		return "0"
	}
	nums := []byte(num)

	for i := 0; i < k; {
		if nums[0] == byte('0') {
			nums = nums[1:]
			continue
		}
		var j int
		for ; j < len(nums)-1; j += 1 {
			if nums[j] > nums[j+1] {
				break
			}
		}
		nums = append(nums[:j], nums[j+1:len(nums)]...)
		i += 1
	}

	// check leader zero
	for len(nums) != 0 && nums[0] == byte('0') {
		nums = nums[1:]
	}
	if len(nums) == 0 {
		return "0"
	}

	return string(nums)
}

// 利用单调递增栈 O(len(num))
func removeKdigits(num string, k int) string {
	if len(num) == k {
		return "0"
	}
	var remove int
	stack := make([]byte, 0, len(num))
	for i := 0; i < len(num); i += 1 {
		for len(stack) != 0 && stack[len(stack)-1] > num[i] && remove < k {
			stack = stack[:len(stack)-1]
			remove++
		}
		stack = append(stack, num[i])
	}
	// 需要检查是否已经移除 k 个字段
	for remove < k {
		stack = stack[:len(stack)-1]
		remove++
	}

	// check leader zero
	for len(stack) != 0 && stack[0] == byte('0') {
		stack = stack[1:]
	}
	if len(stack) == 0 {
		return "0"
	}

	return string(stack)
}
