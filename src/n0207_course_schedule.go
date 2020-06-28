// 检查是否是 DAG
// 另外一道类似的题目 https://leetcode.com/problems/course-schedule-ii/
// 这里用的是 bfs 思路，把所有入度为 0 的挑出来，然后遍历这些入度为 0 的出度，把对应出度的入度个数减 1，以此往复，
// 一直到找不到入度为 0 的节点为止，再判断是否所有节点都被遍历过了。
func canFinish(numCourses int, prerequisites [][]int) bool {
	graph := make(map[int][]int)
	inDegree := make(map[int]int)
	for _, p := range prerequisites {
		graph[p[0]] = append(graph[p[0]], p[1])
		inDegree[p[1]] += 1

		if _, ok := inDegree[p[0]]; !ok {
			inDegree[p[0]] = 0
		}
	}

	for len(inDegree) != 0 {
		needDelete := make([]int, 0, len(inDegree))
		for k, v := range inDegree {
			if v == 0 {
				needDelete = append(needDelete, k)
			}
		}

		if len(needDelete) == 0 && len(inDegree) != 0 {
			return false
		}

		for _, d := range needDelete {
			delete(inDegree, d)
			for _, dd := range graph[d] {
				inDegree[dd] -= 1
			}
		}
	}
	return true
}
