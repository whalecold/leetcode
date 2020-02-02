/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head.
 *
 * You may not modify the values in the list's nodes, only nodes itself may be changed.
 *
 *  
 *
 * Example:
 *
 *
 * Given 1->2->3->4, you should return the list as 2->1->4->3.
 *
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

#[allow(dead_code)]
impl Solution {
    #[allow(dead_code)]
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();
        while head.is_some() {
            let mut lhs = head.as_mut().unwrap().next.take();
            if lhs.is_none() {
                break
            }
            let mut rhs= lhs.as_mut().unwrap().next.take();
            if rhs.is_none() {
                head.as_mut().unwrap().next = lhs;
                break
            }
            let next = rhs.as_mut().unwrap().next.take();
            lhs.as_mut().unwrap().next = next;
            rhs.as_mut().unwrap().next = lhs;
            head.as_mut().unwrap().next = rhs;
            head = head.unwrap().next.as_mut().unwrap().next.as_mut();
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::utils::linked_list::to_list;
    use super::Solution;

    #[test]
    fn test_24() {
        assert_eq!(Solution::swap_pairs(to_list(vec![1, 2, 3, 4])), to_list(vec![2, 1, 4, 3]));
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
        assert_eq!(Solution::swap_pairs(to_list(vec![1, 2, 3])), to_list(vec![2, 1, 3]));
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}
