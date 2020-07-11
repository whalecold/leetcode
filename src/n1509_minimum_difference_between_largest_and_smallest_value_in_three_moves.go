import "sort"

// 啊啊啊啊啊啊 好气啊 题目没有审清楚没有在规定时间内做出来！！！
// 不然这次可以加好多分了！！！
// 用排序的方法其实很简单，先排序，然后遍历三次，每次遍历的时候
// 比较左右两遍的差值，然后移动左右的位置。
func minDifference(nums []int) int {
	if len(nums) <= 4 {
		return 0
	}
	sort.Ints(nums)
	left, right := 0, len(nums)-1
	for i := 3; i > 0; i-- {
		if nums[right]-nums[right-i] > nums[left+i]-nums[left] {
			right--
		} else {
			left++
		}
	}
	return nums[right] - nums[left]

}

