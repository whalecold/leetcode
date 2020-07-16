// sliding window 时间复杂度 O(lenA) 空间复杂度 O(K)
// 这里为了方便是空间复杂度是 O(lenA)
// 就是用队列把反转了 0 的 index 全部存起来，如果发现反转次数不够用了
// 则把最早入队列的 index 吐出去，然后移动 left.
func longestOnes(A []int, K int) int {
	if K == len(A) {
		return K
	}
	queue := make([]int, 0, len(A))
	var maxLength, left, right int
	for i, num := range A {
		if num == 0 {
			queue = append(queue, i)
			if len(queue) > K && len(queue) > 0 {
				left = queue[0] + 1
				queue = queue[1:]
			}
		}
		// fmt.Printf("left . %v right. %v\n", left, right)
		if right-left+1 > maxLength {
			maxLength = right - left + 1
		}
		right++
	}
	return maxLength
}

// 别人的解法..真的好厉害..差距太大了
func longestOnes(A []int, K int) int {
	var j, i int
	for ; j < len(A); j++ {
		if A[j] == 0 {
			K--
		}
		if K < 0 {
			if A[i] == 0 {
				K++
			}
			i++
		}
	}
	return j - i
}
