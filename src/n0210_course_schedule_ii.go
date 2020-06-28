// 思路一样的解法，因为课程序号都已经固定了，所以直接用数组做散列表就好了
// 依旧是先遍历入度为 0 的课程
func findOrder(numCourses int, prerequisites [][]int) []int {
    graphVec := make([]int, numCourses)
    graphMap := make(map[int][]int, numCourses)
    ret := make([]int, 0, numCourses)
    for _, p := range prerequisites {
        graphMap[p[1]] = append(graphMap[p[1]], p[0])
        graphVec[p[0]] += 1
    }

    l := list.New()
    for index, v := range graphVec {
        if v == 0 {
            l.PushBack(index)
        }
    }

    for l.Len() != 0 {
        front := l.Front()
        val := front.Value.(int)
        l.Remove(front)

        ret = append(ret, val)
        numCourses -= 1

        for _, v := range graphMap[val] {
            graphVec[v] -= 1
            if graphVec[v] == 0 {
                l.PushBack(v)
            }
        }
    }
    if numCourses == 0 {
        return ret
    }
    // 如果有循环依赖的话 numCourses 不会被减到 0.
    return nil
}
