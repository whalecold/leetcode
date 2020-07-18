/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
// 冒泡的时间复杂度是 O(N^2) 不符合要求
// 利用快排的思想去做的 之前有点思维定势了 每个节点都是可以拆出来的
// 这种方法理论上最优时间复杂度是 nlg(n), 但是数据不好的话很容易退化成 O(n^2)

// 看了下discuss 的解法是先利用 two point 把链表分成两部分，然后在分别对两部分进行
// 递归排序，得到结果后再进行 merge. 这种时间复杂度就比较稳定
func sortList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	cur := head.Next
	// 把比第一个节点小的和大的全部挑出来分类
	bigHead, smallHead := &ListNode{}, &ListNode{}
	bc, sc := bigHead, smallHead
	for cur != nil {
		temp := cur
		cur = cur.Next
		temp.Next = nil
		if temp.Val > head.Val {
			bc.Next = temp
			bc = temp
		} else {
			sc.Next = temp
			sc = temp
		}
	}
	// 对比自己大的继续进行排序
	head.Next = sortList(bigHead.Next)
	if smallHead.Next == nil {
		return head
	}
	// 对比自己小的进行排序
	smallHead.Next = sortList(smallHead.Next)
	lastBig := smallHead
	bc = smallHead
	for bc != nil {
		lastBig = bc
		bc = bc.Next
	}
	// 进行拼接
	lastBig.Next = head
	return smallHead.Next
}
