/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
// 简单的递归调用 bottom-up
func findFrequentTreeSum(root *TreeNode) []int {
    if root == nil {
        return nil
    }
    numsMap := make(map[int]int)
    helper(root, numsMap)
    ret := make([]int, 0, len(numsMap))
    max := - 1 << 31
    for k, v := range numsMap {
        if v > max {
            max = v
            ret = append(ret[:0], k)
        } else if v == max {
            ret = append(ret, k)
        }
    }
    return ret
}

func helper(root *TreeNode, numsMap map[int]int) int {
    if root == nil {
        return 0
    }
    left := helper(root.Left, numsMap)
    right := helper(root.Right, numsMap)
    if left == 0 && right == 0 {
        numsMap[root.Val] += 1
    } else {
        numsMap[root.Val + left + right] += 1
    }
    return root.Val + left + right
}
