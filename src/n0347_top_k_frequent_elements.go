
// bucket sort!
func topKFrequent(nums []int, k int) []int {
	if len(nums) == 0 {
		return nil
	}
	bucket := make([][]int, len(nums)+1)
	numMap := make(map[int]int)
	for _, num := range nums {
		numMap[num]++
	}
	for k, v := range numMap {
		bucket[v] = append(bucket[v], k)
	}
	ret := make([]int, 0, k)
	for i := len(bucket) - 1; i >= 0 && k > 0; i-- {
		if len(bucket[i]) != 0 {
			ret = append(ret, bucket[i]...)
			k -= len(bucket[i])
		}
	}

	return ret
}
