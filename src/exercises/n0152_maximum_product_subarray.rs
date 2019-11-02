/*
*
*
*
*
*
*
*/

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut max = std::i32::MIN;
        for i in 0..len {
            let mut dp = vec![std::i32::MIN; len];
            for j in i..len {
                if i == j {
                    dp[j] = nums[i];
                    max = i32::max(dp[j], max);
                    continue;
                }
                dp[j] = nums[j] * dp[j - 1];
                max = i32::max(dp[j], max);
            }
        }
        max
    }
    #[allow(dead_code)]
    pub fn max_product1(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let (mut p_max, mut p_min) = (max, max);
        for i in 1..nums.len() {
            let i_max = i32::max(i32::max(nums[i], p_max * nums[i]), nums[i] * p_min);
            let i_min = i32::min(i32::min(nums[i], p_min * nums[i]), nums[i] * p_max);
            max = i32::max(max, i_max);
            p_max = i_max;
            p_min = i_min;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-1, -2, 0, -1, -8]), 8);
        assert_eq!(Solution::max_product(vec![2, 3, -1, 4]), 6);
        assert_eq!(Solution::max_product(vec![2, 3, -1, 4, -1]), 24);
    }
    #[test]
    fn test_example1() {
        assert_eq!(Solution::max_product1(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product1(vec![-1, -2, 0, -1, -8]), 8);
        assert_eq!(Solution::max_product1(vec![2, 3, -1, 4]), 6);
        assert_eq!(Solution::max_product1(vec![2, 3, -1, 4, -1]), 24);
        assert_eq!(Solution::max_product1(vec![2, 3, -1, 4, 8]), 32);
        assert_eq!(Solution::max_product1(vec![0, 2]), 2);
        assert_eq!(Solution::max_product1(vec![2, -5, -2, -4, 3]), 24);
    }
}
