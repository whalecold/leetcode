/**
 * [416] Partition Equal Subset Sum
 *
 * Given a non-empty array containing only positive integers, find if the array can be partitioned
 * into two subsets such that the sum of elements in both subsets is equal.
 *
 * Note:
 *
 * <ol>
 * 	Each of the array element will not exceed 100.
 * 	The array size will not exceed 200.
 * </ol>
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: [1, 5, 11, 5]
 *
 * Output: true
 *
 * Explanation: The array can be partitioned as [1, 5, 5] and [11].
 *
 *
 *  
 *
 * Example 2:
 *
 *
 * Input: [1, 2, 3, 5]
 *
 * Output: false
 *
 * Explanation: The array cannot be partitioned into equal sum subsets.
 *
 *
 *  
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        // 分成两份本质上就是看能不能达到总数的一半
        sum /= 2;
        let mut dp = vec![false; (sum + 1) as usize];
        dp[0] = true;
        for i in 1..nums.len() {
            // 这里从大到小是为了防止 nums[i] 是 j 的 1/2 这种情况发生，如果从小到达的话只要是倍数关系全部为 true
            for j in (1..=sum).rev() {
                if j >= nums[i] {
                    dp[j as usize] |= dp[(j - nums[i]) as usize];
                }
            }
        }
        dp[sum as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_416() {
        assert_eq!(Solution::can_partition(vec![1, 5, 5, 11]), true);
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
        assert_eq!(
            Solution::can_partition(vec![
                28, 63, 95, 30, 39, 16, 36, 44, 37, 100, 61, 73, 32, 71, 100, 2, 37, 60, 23, 71,
                53, 70, 69, 82, 97, 43, 16, 33, 29, 5, 97, 32, 29, 78, 93, 59, 37, 88, 89, 79, 75,
                9, 74, 32, 81, 12, 34, 13, 16, 15, 16, 40, 90, 70, 17, 78, 54, 81, 18, 92, 75, 74,
                59, 18, 66, 62, 55, 19, 2, 67, 30, 25, 64, 84, 25, 76, 98, 59, 74, 87, 5, 93, 97,
                68, 20, 58, 55, 73, 74, 97, 49, 71, 42, 26, 8, 87, 99, 1, 16, 79
            ]),
            true
        );
    }
}
