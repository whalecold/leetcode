/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * If the target is not found in the array, return [-1, -1].
 *
 * Example 1:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 *
 * Example 2:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = vec![-1, -1];
        if nums.len() == 0 || nums.first().unwrap() > &target || nums.last().unwrap() < &target {
            return ret;
        }
        let (mut left, mut right) = (0 as usize, nums.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] >= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        if nums[left] != target {
            return ret;
        }
        ret[0] = left as i32;

        let (mut left, mut right) = (0 as usize, nums.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        println!("left {}", left);
        if nums[left] == target {
            ret[1] = left as i32;
        } else {
            ret[1] = left as i32 - 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_34() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
    }
}
