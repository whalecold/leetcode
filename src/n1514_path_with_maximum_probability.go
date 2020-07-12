import "container/heap"

// 用了 dfs TLE 了 然后看别人用 bfs 没有超时...
// 最好的办法 Dijkstra 如果想拿点分数还是把图的所有算法都过一遍了... 方法后续补充
// https://www.youtube.com/watch?v=lAXZGERcDf4&t=15s 视频
func maxProbability(n int, edges [][]int, succProb []float64, start int, end int) float64 {
	m := make(map[int][][]float64, n)
	for i, edge := range edges {
		m[edge[0]] = append(m[edge[0]], []float64{succProb[i], float64(edge[1])})
		m[edge[1]] = append(m[edge[1]], []float64{succProb[i], float64(edge[0])})
	}
	visit, dis := make([]int, n), make([]float64, n)
	dis[start] = 1
	h := &floatHeap{}
	heap.Push(h, []float64{float64(start), 1})

	for h.Len() != 0 {
		last := heap.Pop(h).([]float64)
		if visit[int(last[0])] == 1 {
			continue
		}
		visit[int(last[0])] = 1

		data := m[int(last[0])]
		for _, d := range data {
			if visit[int(d[1])] == 1 {
				continue
			}
			if last[1]*d[0] > dis[int(d[1])] {
				dis[int(d[1])] = last[1] * d[0]
				heap.Push(h, []float64{d[1], last[1] * d[0]})
			}
		}
	}

	return dis[end]
}

type floatHeap [][]float64

func (h floatHeap) Len() int           { return len(h) }
func (h floatHeap) Less(i, j int) bool { return h[i][1] > h[j][1] }
func (h floatHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *floatHeap) Push(x interface{}) {
	*h = append(*h, x.([]float64))
}
func (h *floatHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

// dfs TLE
// func maxProbability(n int, edges [][]int, succProb []float64, start int, end int) float64 {
//     graph := make([][]float64, n)
//     for i := range graph {
//         graph[i] = make([]float64, n)
//     }
//     for i := 0; i < len(edges); i++ {
//         edge := edges[i]
//         graph[edge[0]][edge[1]] = succProb[i]
//         graph[edge[1]][edge[0]] = succProb[i]
//     }
//     fmt.Printf("graph %v\n", graph)
//     cache := make(map[int]bool)
//     max := float64(0)
//     dfs(graph, start, end, 1, &max, cache)
//     return max
// }

// func dfs(graph [][]float64, next, end int, cur float64, max *float64, cache map[int]bool) {
//     // fmt.Printf("next %v cur %v end %v  max %v cache %v\n", next, cur, end, *max, cache)
//     if cache[next] {
//         return
//     }
//     if next == end {
//         // fmt.Printf("---  --  next %v cur %v end %v  max %v\n", next, cur, end, *max)
//         if cur > *max {
//             *max = cur
//         }
//         return
//     }
//     cache[next] = true
//     for i := 0; i < len(graph[next]); i++ {
//         if graph[next][i] != 0 {
//             dfs(graph, i, end, cur * graph[next][i], max, cache)
//         }
//     }
//     cache[next] = false

// }
