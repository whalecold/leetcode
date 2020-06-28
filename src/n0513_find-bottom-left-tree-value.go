/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func findBottomLeftValue(root *TreeNode) int {
    if root == nil {
        return 0
    }
    queue := make([]*TreeNode, 0, 1000)
    ret := root.Val
    queue = append(queue, root)
    for len(queue) != 0 {
        size := len(queue)
        for i := 0; i < size; i += 1 {
            head := queue[0]
            queue = queue[1:]

            if i == 0 {
                ret = head.Val
            }

            if head.Left != nil {
                queue = append(queue, head.Left)
            }
            if head.Right != nil {
                queue = append(queue, head.Right)
            }
        }
    }
    return ret
}
