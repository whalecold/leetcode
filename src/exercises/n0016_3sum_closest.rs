/**
 * [16] 3Sum Closest
 *
 * Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 2, 1, -4], and target = 1.
 *
 * The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if i32::abs(target - sum) < i32::abs(target - closest) {
                    closest = sum;
                }
                if sum > target {
                    r -= 1;
                } else if sum < target {
                    l += 1;
                } else {
                    return target;
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![-3, -2, -5, 3, -4], -1), -2);
    }
}
