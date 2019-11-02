/**
 * [33] Search in Rotated Sorted Array
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 *
 * You are given a target value to search. If found in the array return its index, otherwise return -1.
 *
 * You may assume no duplicate exists in the array.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * Example 1:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        if nums[0] == target {
            return 0;
        }
        if nums[0] < nums[nums.len() - 1] {
            if let Ok(index) = nums.binary_search(&target) {
                return index as i32;
            } else {
                return -1;
            }
        }
        let mut pivot = 0 as usize;
        let mut start = 0 as usize;
        let mut end = nums.len() - 1;
        while start < end {
            let mid = (start + end) / 2;
            if nums[mid] > nums[end] {
                start = mid;
                if nums[mid + 1] <= nums[nums.len() - 1] {
                    pivot = mid + 1;
                    break;
                }
            } else {
                end = mid;
            }
        }
        let sub_nums;
        let mut predict = 0;
        if nums[nums.len() - 1] >= target {
            sub_nums = &nums[pivot..nums.len()];
            predict = pivot as i32;
        } else {
            sub_nums = &nums[0..pivot];
        }
        if let Ok(index) = sub_nums.binary_search(&target) {
            return index as i32 + predict;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_33() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 12), -1);
        assert_eq!(Solution::search(vec![3, 1], 0), -1);
        assert_eq!(Solution::search(vec![3, 1], 1), 1);
    }
}
