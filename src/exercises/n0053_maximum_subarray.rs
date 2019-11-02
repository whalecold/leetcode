/// Given an integer array nums, find the contiguous subarray (containing at least one number)
/// which has the largest sum and return its sum.
///
/// ## Example:
/// Input: [-2,1,-3,4,-1,2,1,-5,4],
/// Output: 6
/// Explanation: [4,-1,2,1] has the largest sum = 6.

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = std::i32::MIN;
        let mut cur = 0;
        for i in 0..nums.len() {
            cur += nums[i];
            max = i32::max(max, cur);
            if cur <= 0 {
                cur = 0;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_53() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 6]),
            7
        );
        assert_eq!(Solution::max_sub_array(vec![-2, -1, -4]), -1);
        assert_eq!(Solution::max_sub_array(vec![-3]), -3);
    }
}
