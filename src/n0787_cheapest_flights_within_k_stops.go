import "container/heap"

// Dijkstra's Algorithm
func findCheapestPrice(n int, flights [][]int, src int, dst int, K int) int {
	flightMap := make(map[int][][]int, n)
	for _, f := range flights {
		flightMap[f[0]] = append(flightMap[f[0]], []int{f[1], f[2]})
	}

	h := &intHeap{}
	heap.Push(h, []int{src, 0, K + 1})

	for h.Len() != 0 {
		last := heap.Pop(h).([]int)

		stops, city, price := last[2], last[0], last[1]
		if city == dst {
			return price
		}

		if stops > 0 {
			fs := flightMap[city]
			for _, f := range fs {
				heap.Push(h, []int{f[0], f[1] + price, stops - 1})
			}
		}

	}
	return -1
}

type intHeap [][]int

func (h intHeap) Len() int           { return len(h) }
func (h intHeap) Less(i, j int) bool { return h[i][1] < h[j][1] }
func (h intHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *intHeap) Push(x interface{}) {
	*h = append(*h, x.([]int))
}
func (h *intHeap) Pop() interface{} {
	n := len(*h)
	x := (*h)[n-1]
	*h = (*h)[0 : n-1]
	return x
}

// https://www.youtube.com/watch?v=lAXZGERcDf4&list=PL1qIemuv_4SXqcH6-UUuKiVfBLiLZpXXH&index=3&t=12s
// 1、Dijkstra's Algorithm
// 2、dfs
// 3、bfs
// go 语言没有优先队列，要自己实现就很烦，这里先用 dfs 去解
func findCheapestPrice1(n int, flights [][]int, src int, dst int, K int) int {
	flightMap := make(map[int][][]int, len(flights))
	for _, v := range flights {
		flightMap[v[0]] = append(flightMap[v[0]], []int{v[1], v[2]})
	}
	// fmt.Printf("%v\n", flightMap)
	MAX := 1<<32 - 1
	min := MAX
	dfs(flightMap, 0, dst, src, K+1, &min)
	if min == MAX {
		return -1
	}
	return min
}

func dfs(fMap map[int][][]int, cost, dst, cur, K int, min *int) {
	// fmt.Printf("cur %v  dst %v K %v cost %v\n", cur, dst, K, cost)

	if K < 0 {
		return
	}
	if cur == dst {
		if cost < *min {
			*min = cost
		}
		return
	}

	for _, info := range fMap[cur] {
		if info[1]+cost >= *min {
			continue
		}
		dfs(fMap, cost+info[1], dst, info[0], K-1, min)
	}
	return
}

