/**
 * [19] Remove Nth Node From End of List
 *
 * Given a linked list, remove the n-th node from the end of list and return its head.
 *
 * Example:
 *
 *
 * Given linked list: 1->2->3->4->5, and n = 2.
 *
 * After removing the second node from the end, the linked list becomes 1->2->3->5.
 *
 *
 * Note:
 *
 * Given n will always be valid.
 *
 * Follow up:
 *
 * Could you do this in one pass?
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

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
use super::super::utils::linked_list::ListNode;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 主要是利用了双下标的方法
        let clone = Some(Box::new(ListNode { val: 0, next: head.clone() }));
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));

        let (mut fast, mut slow) = (clone.as_ref(), dummy_head.as_mut());
        for _ in 0..n {
            fast = fast.unwrap().next.as_ref();
        }
        while fast.unwrap().next.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.unwrap().next.as_mut();
        }
        let next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = next;
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::utils::linked_list::to_list;
    use super::Solution;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}
