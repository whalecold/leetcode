/**
 * [61] Rotate List
 *
 * Given a linked list, rotate the list to the right by k places, where k is non-negative.
 *
 * Example 1:
 *
 *
 * Input: 1->2->3->4->5->NULL, k = 2
 * Output: 4->5->1->2->3->NULL
 * Explanation:
 * rotate 1 steps to the right: 5->1->2->3->4->NULL
 * rotate 2 steps to the right: 4->5->1->2->3->NULL
 *
 *
 * Example 2:
 *
 *
 * Input: 0->1->2->NULL, k = 4
 * Output: 2->0->1->NULL
 * Explanation:
 * rotate 1 steps to the right: 2->0->1->NULL
 * rotate 2 steps to the right: 1->2->0->NULL
 * rotate 3 steps to the right: 0->1->2->NULL
 * rotate 4 steps to the right: 2->0->1->NULL
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
    // Ref: https://rust.cc/article?id=c9eb3807-fd86-4611-835a-57d3f8fe5a8f
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k <= 0 {
            return head;
        }
        let mut ptr = head.as_ref();
        let mut length = 0;
        while ptr.is_some() {
            ptr = ptr.unwrap().next.as_ref();
            length += 1;
        }
        let dis = length - k % length;
        if dis == length {
            return head;
        }
        let mut head = head;
        let mut ptr = head.as_mut().unwrap();
        for _i in 1..dis {
            ptr = ptr.next.as_mut().unwrap();
        }
        let mut new_head = ptr.next.take();
        let mut ptr = new_head.as_mut().unwrap();
        while ptr.next.as_mut().is_some() {
            ptr = ptr.next.as_mut().unwrap()
        }
        ptr.next = head;
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::utils::linked_list::to_list;
    use super::Solution;

    #[test]
    fn test_61() {
        assert_eq!(
            Solution::rotate_right(to_list(vec![1, 2]), 1),
            to_list(vec![2, 1])
        );
        assert_eq!(
            Solution::rotate_right(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![4, 5, 1, 2, 3])
        );
    }
}
