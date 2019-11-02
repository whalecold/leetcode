use std::collections::HashMap;
/**
 * [523] Continuous Subarray Sum
 *
 * Given a list of non-negative numbers and a target integer k, write a function to check if the array has a continuous subarray of size at least 2 that sums up to a multiple of k, that is, sums up to n*k where n is also an integer.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: [23, 2, 4, 6, 7],  k=6
 * Output: True
 * Explanation: Because [2, 4] is a continuous subarray of size 2 and sums up to 6.
 *
 *
 * Example 2:
 *
 *
 * Input: [23, 2, 6, 4, 7],  k=6
 * Output: True
 * Explanation: Because [23, 2, 6, 4, 7] is an continuous subarray of size 5 and sums up to 42.
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * 	The length of the array won't exceed 10,000.
 * 	You may assume the sum of all the numbers is in the range of a signed 32-bit integer.
 * </ol>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut m: HashMap<i32, i32> = HashMap::new();
        m.insert(0, -1);
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if k != 0 {
                sum %= k;
            }
            if i >= 1 && nums[i] == nums[i-1] && k == 0 && nums[i] == 0 {
                return true
            }
            println!("[{:?}: sum : {}]", m, sum);
            if let Some(prev) = m.get(&sum) {
                if i != 0 && i as i32 - *prev > 1 {
                    return true;
                }
            }
            m.insert(sum, i as i32);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_523() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
        assert_eq!(Solution::check_subarray_sum(vec![0], 0), false);
        assert_eq!(Solution::check_subarray_sum(vec![0, 0], 0), true);
        assert_eq!(Solution::check_subarray_sum(vec![0, 0], -1), true);
        assert_eq!(Solution::check_subarray_sum(vec![0, 1, 0], 0), false);
    }
}
