/**
 * [494] Target Sum
 *
 *
 * You are given a list of non-dp integers, a1, a2, ..., an, and a target, S. Now you have 2 symbols + and -. For each integer, you should choose one from + and - as its new symbol.
 *  
 *
 * Find out how many ways to assign symbols to make sum of integers equal to target S.  
 *
 *
 * Example 1:<br />
 *
 * Input: nums is [1, 1, 1, 1, 1], S is 3.
 * Output: 5
 * Explanation:
 *
 * -1+1+1+1+1 = 3
 * +1-1+1+1+1 = 3
 * +1+1-1+1+1 = 3
 * +1+1+1-1+1 = 3
 * +1+1+1+1-1 = 3
 *
 * There are 5 ways to assign symbols to make the sum of nums be target 3.
 *
 *
 *
 * Note:<br>
 * <ol>
 * The length of the given array is positive and will not exceed 20.
 * The sum of elements in the given array will not exceed 1000.
 * Your output answer is guaranteed to be fitted in a 32-bit integer.
 * </ol>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut all: i32 = nums.iter().sum();
        if all < s {
            return 0;
        }
        all += s;
        if all % 2 == 1 {
            return 0;
        }
        let all = all >> 1;
        let len = (all + 1) as usize;
        let mut dp = vec![0; len];
        dp[0] = 1;
        for num in nums.iter() {
            for sum in (*num..=all).rev() {
                if sum >= *num {
                    dp[sum as usize] += dp[(sum - *num) as usize];
                }
            }
        }
        dp[all as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_494() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1], 1), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1], 2), 1);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1], 3), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1], 1), 3);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(
            Solution::find_target_sum_ways(vec![1, 2, 7, 9, 981], 1000000000),
            0
        );
    }
}
