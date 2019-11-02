/**
 * [376] Wiggle Subsequence
 *
 * A sequence of numbers is called a wiggle sequence if the differences between successive numbers strictly alternate between positive and negative. The first difference (if one exists) may be either positive or negative. A sequence with fewer than two elements is trivially a wiggle sequence.
 *
 * For example, [1,7,4,9,2,5] is a wiggle sequence because the differences (6,-3,5,-7,3) are alternately positive and negative. In contrast, [1,4,7,2,5] and [1,7,4,5,5] are not wiggle sequences, the first because its first two differences are positive and the second because its last difference is zero.
 *
 * Given a sequence of integers, return the length of the longest subsequence that is a wiggle sequence. A subsequence is obtained by deleting some number of elements (eventually, also zero) from the original sequence, leaving the remaining elements in their original order.
 *
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[1,7,4,9,2,5]</span>
 * Output: <span id="example-output-1">6
 * Explanation: </span>The entire sequence is a wiggle sequence.
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,17,5,10,13,15,10,5,16,8]</span>
 * Output: <span id="example-output-2">7
 * </span><span id="example-output-1">Explanation: </span>There are several subsequences that achieve this length. One is [1,17,10,13,10,16,8].
 *
 * <div>
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">[1,2,3,4,5,6,7,8,9]</span>
 * Output: <span id="example-output-3">2</span>
 *
 * Follow up:<br />
 * Can you do it in O(n) time?
 * </div>
 * </div>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // O(n*n)
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut dp= vec![0; len + 1];
        let mut store = vec![0; len + 1];
        let mut max = 1;
        dp[1] = 1;
        for i in 2..=len {
            let mut max_index = 1;
            for j in 1..i {
                if nums[i - 1] < nums[j - 1] && (store[j] > 0 || j == 1)
                    || nums[i - 1] > nums[j - 1] && (store[j] < 0 || j == 1)
                {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        max_index = j;
                    }
                }
            }
            store[i] = nums[i - 1] - nums[max_index - 1];
            max = i32::max(dp[i], max);
        }
        max
    }
    #[allow(dead_code)]
    pub fn wiggle_max_length_on(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut up = vec![0; len];
        let mut down = vec![0; len];
        up[0] = 1;
        down[0] = 1;
        for i in 1..len {
            if nums[i] > nums[i - 1] {
                up[i] = down[i - 1] + 1;
                down[i] = down[i - 1];
            } else if nums[i] < nums[i - 1] {
                up[i] = up[i - 1];
                down[i] = up[i - 1] + 1;
            } else {
                up[i] = up[i - 1];
                down[i] = down[i - 1];
            }
        }
        i32::max(up[len - 1], down[len - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_376() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 2, 3]), 2);
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(Solution::wiggle_max_length(vec![3, 2, 5]), 3);
    }
    #[test]
    fn test_376_on() {
        assert_eq!(Solution::wiggle_max_length_on(vec![1, 2, 3]), 2);
        assert_eq!(
            Solution::wiggle_max_length_on(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
        assert_eq!(Solution::wiggle_max_length_on(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(Solution::wiggle_max_length_on(vec![3, 2, 5]), 3);
        assert_eq!(Solution::wiggle_max_length_on(vec![3, 3, 3, 2, 5]), 3);
        assert_eq!(Solution::wiggle_max_length_on(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(Solution::wiggle_max_length_on(vec![0, 0]), 1);
    }
}
