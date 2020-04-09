/**
 * [876] Middle of the Linked List
 *
 * Given a non-empty, singly linked list with head node head, return a middle node of linked list.
 *
 * If there are two middle nodes, return the second middle node.
 *
 *  
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[1,2,3,4,5]</span>
 * Output: Node 3 from this list (Serialization: <span id="example-output-1">[3,4,5]</span>)
 * The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
 * Note that we returned a ListNode object ans, such that:
 * ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,2,3,4,5,6]</span>
 * Output: Node 4 from this list (Serialization: <span id="example-output-2">[4,5,6]</span>)
 * Since the list has two middle nodes with values 3 and 4, we return the second one.
 *
 *
 *  
 *
 * Note:
 *
 *
 * 	The number of nodes in the given list will be between 1 and 100.
 *
 * </div>
 * </div>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}
use super::super::utils::linked_list::ListNode;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let temp = head.clone();
        if head.is_none() {
            return head;
        }
        let (mut slow, mut fast) = (head.unwrap(), temp.as_ref());
        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.next.unwrap();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        Some(slow)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_876() {}
}
