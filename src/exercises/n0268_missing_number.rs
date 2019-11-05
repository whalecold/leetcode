/**
 * [268] Missing Number
 *
 * Given an array containing n distinct numbers taken from 0, 1, 2, ..., n, find the one that is missing from the array.
 *
 * Example 1:
 *
 *
 * Input: [3,0,1]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: [9,6,4,2,3,5,7,0,1]
 * Output: 8
 *
 *
 * Note:<br />
 * Your algorithm should run in linear runtime complexity. Could you implement it using only constant extra space complexity?
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ret = nums.len() as i32;
        for i in 0..nums.len() {
            ret = ret ^ i as i32 ^ nums[i];
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_268() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }
}
