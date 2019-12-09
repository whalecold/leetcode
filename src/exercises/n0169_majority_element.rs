/**
 * [169] Majority Element
 *
 * Given an array of size n, find the majority element. The majority element is the element that appears more than &lfloor; n/2 &rfloor; times.
 *
 * You may assume that the array is non-empty and the majority element always exist in the array.
 *
 * Example 1:
 *
 *
 * Input: [3,2,3]
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: [2,2,1,1,1,2,2]
 * Output: 2
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // todo: empty nums or not exist beside the condition below
        // You may assume that the array is non-empty and the majority element always exist in the array.
        let (mut major, mut count) = (nums[0], 1);
        for i in 1..nums.len() {
            if count == 0 {
                count == 1;
                major = nums[i];
            } else if major == nums[i] {
                count += 1;
            } else {
                count -= 1;
            }
        }
        major
    }
    #[allow(dead_code)]
    pub fn majority_element1(nums: Vec<i32>) -> i32 {
        // todo: empty nums or not exist beside the condition below
        // You may assume that the array is non-empty and the majority element always exist in the array.
        let mut nums = nums;
        nums.sort();
        nums[nums.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_169() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
