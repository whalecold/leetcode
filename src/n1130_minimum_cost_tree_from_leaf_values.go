import "math"

// 这还是个单调栈的问题，主要的难点在分析
func mctFromLeafValues(arr []int) int {
	stack := make([]int, 0, len(arr)+1)
	stack = append(stack, math.MaxInt32)
	var res int
	for _, a := range arr {
		for stack[len(stack)-1] <= a {
			last := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			res += last * min(stack[len(stack)-1], a)
		}
		stack = append(stack, a)
	}
	for len(stack) > 2 {
		last := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		res += last * stack[len(stack)-1]
	}
	return res
}

type pair struct {
	max   int
	value int
}

func mctFromLeafValues1(arr []int) int {
	dp := make([][]*pair, len(arr))
	for i := range dp {
		dp[i] = make([]*pair, len(arr))
		dp[i][i] = &pair{
			max: arr[i],
		}
	}
	for l := 1; l < len(arr); l++ {
		for i := l; i < len(arr); i++ {
			p, p1, p2 := &pair{}, dp[i-1][i-l], dp[i][i-l+1]
			value1, value2 := p1.value+p1.max*arr[i], p2.value+p2.max*arr[i-l]
			if value2 < value1 {
				p.value = value2
				p.max = max(p2.max, arr[i-l])
			} else {
				p.value = value1
				p.max = max(p1.max, arr[i])
			}
			dp[i][i-l] = p
			// for
			// fmt.Printf("i. %v  j. %v pair.. %v \n", i, i-l, p)
		}
	}
	return dp[len(arr)-1][0].value
}

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}

func min(i, j int) int {
	if i < j {
		return i
	}
	return j
}
