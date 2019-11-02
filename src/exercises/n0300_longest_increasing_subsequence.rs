/**
 * [300] Longest Increasing Subsequence
 *
 * Given an unsorted array of integers, find the length of longest increasing subsequence.
 *
 * Example:
 *
 *
 * Input: [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
 *
 * Note:
 *
 *
 * 	There may be more than one LIS combination, it is only necessary for you to return the length.
 * 	Your algorithm should run in O(n^2) complexity.
 *
 *
 * Follow up: Could you improve it to O(n log n) time complexity?
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Solution::length_of_lis_sub(&nums, std::i32::MIN, 0)
    }

    #[allow(dead_code)]
    pub fn length_of_lis_sub(nums: &Vec<i32>, prev: i32, cur: usize) -> i32 {
        if cur == nums.len() {
            return 0;
        }
        let mut taken = 0;
        if nums[cur] > prev {
            taken = 1 + Solution::length_of_lis_sub(nums, nums[cur], cur + 1);
        }
        let not_taken = Solution::length_of_lis_sub(nums, prev, cur + 1);
        i32::max(taken, not_taken)
    }
    #[allow(dead_code)]
    pub fn length_of_lis1(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut max = 1;
        let mut dp = vec![1; len];
        for i in 1..len {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = i32::max(dp[j] + 1, dp[i]);
                }
            }
            max = i32::max(max, dp[i]);
        }
        max
    }
    #[allow(dead_code)]
    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        for num in nums.iter() {
            let index = match dp.binary_search(num) {
                Ok(index) => index,
                Err(e) => e,
            };
            if index == dp.len() {
                dp.push(num.to_owned());
            } else {
                dp[index] = num.to_owned();
            }
        }
        dp.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_300() {
        assert_eq!(
            Solution::length_of_lis2(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
        assert_eq!(
            Solution::length_of_lis1(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
    }
}
