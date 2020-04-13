use std::collections::HashMap;
/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;
#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            if let Some(index) = m.get(&(target - *num)) {
                return vec![*index, i as i32];
            }
            m.insert(num, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
