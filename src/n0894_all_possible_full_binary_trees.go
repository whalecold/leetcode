/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

// 这种思路其实就是暴力解法，比较蠢，不断的进行分治然后把可能的结果返回来
// 这里因为所有的 val 都为 0, 所以这里是可以优化的，按照 left 和 right 的长度进行 cache.
// 还可以进一步优化，因为 val 都是 0 了，所以 buildHelper 这个函数可以忽略掉 mid, left, right 了，
// 直接用 right-left 的长度作为参数，这样也可以优化一些。有点懒得优化了- -！
// 下面这种方式有一个好处，就是如果 val 需要有值，就比较方便了。
func allPossibleFBT(N int) []*TreeNode {
	ret := make([]*TreeNode, 0, 100)
	cache := make(map[int][]*TreeNode)
	for i := 0; i < N; i++ {
		ret = append(ret, buildHelper(i, 0, N-1, cache)...)
	}
	return ret
}

func buildHelper(mid int, left, right int, cache map[int][]*TreeNode) []*TreeNode {
	key := (mid-left)*1000 + (right - left)
	if nodes, ok := cache[key]; ok {
		return nodes
	}
	if mid == left && mid == right {
		return []*TreeNode{&TreeNode{}}
	}
	if (right-mid)%2 == 0 || (mid-left)%2 == 0 {
		return nil
	}

	var lnodes, rnodes []*TreeNode
	for i := left; i < mid; i++ {
		lnodes = append(lnodes, buildHelper(i, left, mid-1, cache)...)
	}
	for i := mid + 1; i <= right; i++ {
		rnodes = append(rnodes, buildHelper(i, mid+1, right, cache)...)
	}
	res := make([]*TreeNode, 0, len(lnodes)*len(rnodes))
	for _, l := range lnodes {
		for _, r := range rnodes {
			node := &TreeNode{}
			node.Left = l
			node.Right = r
			res = append(res, node)
		}
	}
	cache[key] = res
	return res
}
