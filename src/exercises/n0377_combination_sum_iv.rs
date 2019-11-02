/**
 * [377] Combination Sum IV
 *
 * Given an integer array with all positive numbers and no duplicates, find the number of possible
 * combinations that add up to a positive integer target.
 *
 * Example:
 *
 *
 * nums = [1, 2, 3]
 * target = 4
 *
 * The possible combination ways are:
 * (1, 1, 1, 1)
 * (1, 1, 2)
 * (1, 2, 1)
 * (1, 3)
 * (2, 1, 1)
 * (2, 2)
 * (3, 1)
 *
 * Note that different sequences are counted as different combinations.
 *
 * Therefore the output is 7.
 *
 *
 *  
 *
 * Follow up:<br />
 * What if negative numbers are allowed in the given array?<br />
 * How does it change the problem?<br />
 * What limitation we need to add to the question to allow negative numbers?
 *
 * Credits:<br />
 * Special thanks to <a href="https://leetcode.com/pbrother/">@pbrother</a> for adding this problem and creating all test cases.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target{
            for num in nums.iter() {
                if *num <= i {
                    dp[i as usize] = dp[(i - *num) as usize] + dp[i as usize];
                }
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_377() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }
}
