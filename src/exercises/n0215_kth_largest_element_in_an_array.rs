/**
 * [215] Kth Largest Element in an Array
 *
 * Find the kth largest element in an unsorted array. Note that it is the kth largest element in the sorted order, not the kth distinct element.
 *
 * Example 1:
 *
 *
 * Input: [3,2,1,5,6,4] and k = 2
 * Output: 5
 *
 *
 * Example 2:
 *
 *
 * Input: [3,2,3,1,2,4,5,5,6] and k = 4
 * Output: 4
 *
 * Note: <br />
 * You may assume k is always valid, 1 &le; k &le; array's length.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    #[allow(dead_code)]
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums.iter() {
            if heap.len() < k as usize {
                heap.push(Reverse(num));
                continue;
            }
            if *heap.peek().unwrap().0 > *num {
                continue;
            }
            heap.pop();
            heap.push(Reverse(num));
        }
        *heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_215() {
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 3, 4], 1), 4);
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 3, 4], 3), 3);
        assert_eq!(
            Solution::find_kth_largest(vec![9, 2, 3, 3, 4, 1, 0, 1], 5),
            2
        );
    }
}
