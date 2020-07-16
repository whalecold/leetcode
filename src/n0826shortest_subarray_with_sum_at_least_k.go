// 分析问题本质：有负数
// 利用单调队列的特性去解
func shortestSubarray(A []int, K int) int {
	sums := make([]int64, len(A)+1)
	for i := 1; i < len(sums); i++ {
		sums[i] = sums[i-1] + int64(A[i-1])
	}
	minimal := 1<<31 - 1
	queue := make([]int, 0, len(A))
	for i := 0; i < len(sums); i++ {
		for len(queue) != 0 && sums[i]-sums[queue[0]] >= int64(K) {
			if i-queue[0] < minimal {
				minimal = i - queue[0]
			}
			queue = queue[1:]
		}
		for len(queue) != 0 && sums[queue[len(queue)-1]] >= sums[i] {
			queue = queue[:len(queue)-1]
		}
		queue = append(queue, i)
		// fmt.Printf("queue .. %v\n", queue)
	}
	if minimal == 1<<31-1 {
		return -1
	}
	return minimal
}

// TLE
func shortestSubarray2(A []int, K int) int {
	sums := make([]int64, len(A)+1)
	for i := 1; i < len(sums); i++ {
		sums[i] = sums[i-1] + int64(A[i-1])
	}
	minimal := 1<<31 - 1
	for i := 0; i <= len(A); i++ {
		for j := i + 1; j <= len(A); j++ {
			if sums[j]-sums[i] >= int64(K) && j-i < minimal {
				minimal = j - i
			}
		}
	}
	if minimal == 1<<31-1 {
		return -1
	}
	return minimal
}

// 这种方式不适合有负数的
func shortestSubarray1(A []int, K int) int {
	var left, right, sum int
	minimal := 1<<31 - 1
	for ; right < len(A); right++ {
		sum += A[right]
		for sum >= K {
			if sum >= K && right-left+1 < minimal {
				minimal = right - left + 1
			}
			sum -= A[left]
			left++
		}
	}
	if minimal == 1<<31-1 {
		return -1
	}
	return minimal
}
