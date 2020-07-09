/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

// 主要思路就是把所有的节点进行编号，根节点为 n 的话那么左子树就是 2 * n，右子树为 2 * n + 1，
// 按照层级放到 map 后把所有节点进行下排序，用最大的编号减去最小的编号加 1 就是最后的结果了。
// 这里的 map 可以优化下，每次存的时候就存个最大值和最小值就好了。
func widthOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}
	dMap := make(map[int][]int)
	helper(root, 0, 1, dMap)
	var maxWidth int
	for _, vals := range dMap {
		// sort.Ints(vals)
		width := 1
		if len(vals) >= 2 {
			width = vals[len(vals)-1] - vals[0] + 1
		}
		if width > maxWidth {
			maxWidth = width
		}
	}
	return maxWidth
}

func helper(root *TreeNode, deepth, x int, dMap map[int][]int) {
	if root == nil {
		return
	}
	// 这段代码太丑陋了！
	if val, ok := dMap[deepth]; ok {
		if len(val) == 1 {
			if x > val[0] {
				val = append(val, x)
			} else {
				val = append([]int{x}, val...)
			}
			dMap[deepth] = val
		} else {
			if x > val[1] {
				val[1] = x
			} else if x < val[0] {
				val[0] = x
			}
		}
	} else {
		dMap[deepth] = []int{x}
	}
	// dMap[deepth] = append(dMap[deepth], x)
	helper(root.Left, deepth+1, 2*x, dMap)
	helper(root.Right, deepth+1, 2*x+1, dMap)
}
