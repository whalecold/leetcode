/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
 *
 * Example:
 *
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
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
impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut l3 = &mut head;
        let (mut l1, mut l2) = (l1, l2);
        while l1.is_some() && l2.is_some() {
            let next = if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let (origin, next) = Solution::take_head(l1);
                l1 = origin;
                next
            } else {
                let (origin, next) = Solution::take_head(l2);
                l2 = origin;
                next
            };
            l3.as_mut().unwrap().next = next;
            l3= &mut l3.as_mut().unwrap().next;
        }
        if l1.is_some() {
            l3.as_mut().unwrap().next = l1;
        }
        if l2.is_some() {
            l3.as_mut().unwrap().next = l2;
        }
        head.unwrap().next
    }
    #[inline(always)]
    fn take_head(mut l: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let l_next = l.as_mut().unwrap().next.take();
        let next = l.take();
        l = l_next;
        (l, next)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::utils::linked_list::to_list;
    use super::Solution;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
