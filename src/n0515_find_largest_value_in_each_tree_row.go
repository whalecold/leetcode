/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
// 简单的层级遍历
func largestValues(root *TreeNode) []int {
    if root == nil {
        return nil
    }
    queue := make([]*TreeNode, 0, 1000)
    ret := make([]int, 0, 30)
    queue = append(queue, root)
    for len(queue) != 0 {
        size := len(queue)
        max := - 1 << 31
        for i := 0; i < size; i += 1 {
            head := queue[0]
            queue = queue[1:]
            if head.Val > max {
                max = head.Val
            }
            if head.Left != nil {
                queue = append(queue, head.Left)
            }
            if head.Right != nil {
                queue = append(queue, head.Right)
            }
        }
        ret = append(ret, max)
    }
    return ret
}
