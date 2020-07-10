/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Prev *Node
 *     Next *Node
 *     Child *Node
 * }
 */

// 遍历的版本，空间复杂度可以优化为链表的层级
// 时间击败了 100% 空间击败了 98.11%
func flatten(root *Node) *Node {
	if root == nil {
		return nil
	}
	dummy := &Node{}
	cur := dummy
	stack := make([]*Node, 0, 10)
	for root != nil || len(stack) != 0 {
		if root == nil {
			root = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		child := root.Child
		root.Child = nil
		cur.Next = root
		root.Prev = cur
		cur = root

		if child == nil {
			root = root.Next
			continue
		}
		if root.Next != nil {
			stack = append(stack, root.Next)
		}
		root = child
	}

	if dummy.Next != nil {
		dummy.Next.Prev = nil
	}
	return dummy.Next
}

// 优化下，遍历的时候就重置指针
// 时间复杂度 O(N) 空间复杂度因为存在调用栈的关系，也为 O(N)
func flatten2(root *Node) *Node {
	if root == nil {
		return nil
	}
	dummy := &Node{}
	cur := dummy
	dfs(root, &cur)

	if dummy.Next != nil {
		dummy.Next.Prev = nil
	}
	return dummy.Next
}

func dfs(node *Node, cur **Node) {
	if node == nil {
		return
	}
	// 因为这两个值在接下来的过程中可能会变，所以先做缓存
	child := node.Child
	next := node.Next

	(**cur).Next = node
	node.Prev = *cur
	node.Child = nil
	*cur = node

	dfs(child, cur)
	dfs(next, cur)
}

// 这是最简单的方法，用 dfs 先遍历出所有的节点，然后重置指针
// 时间复杂度 O(N) 空间复杂度 O(N)
func flatten1(root *Node) *Node {
	if root == nil {
		return nil
	}
	stack := make([]*Node, 0, 30)
	flattenHelper(root, &stack)
	dummy := &Node{}
	cur := dummy
	for _, node := range stack {
		node.Child = nil
		cur.Next = node
		node.Prev = cur
		cur = node
	}
	if dummy.Next != nil {
		dummy.Next.Prev = nil
	}
	return dummy.Next
}

func flattenHelper(root *Node, stack *[]*Node) {
	if root == nil {
		return
	}
	*stack = append(*stack, root)
	flattenHelper(root.Child, stack)
	flattenHelper(root.Next, stack)
}
