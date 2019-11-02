/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj) of elements in this subset satisfies:
 *
 * Si % Sj = 0 or Sj % Si = 0.
 *
 * If there are multiple solutions, return any subset is fine.
 *
 * Example 1:
 *
 * <div>
 *
 * Input: <span id="example-input-1-1">[1,2,3]</span>
 * Output: <span id="example-output-1">[1,2] </span>(of course, [1,3] will also be ok)
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,2,4,8]</span>
 * Output: <span id="example-output-2">[1,2,4,8]</span>
 *
 * </div>
 * </div>
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        nums.sort();
        let mut max_index = 0;
        let mut max = 0;

        let len = nums.len();
        let mut dp = vec![0; len];
        let mut index_array = vec![0; len];
        for i in (0..len).rev() {
            for j in i..len {
                if nums[j] % nums[i] == 0 && dp[i] < 1 + dp[j] {
                    dp[i] = dp[j] + 1;
                    index_array[i] = j;
                    if dp[i] > max {
                        max = dp[i];
                        max_index = i;
                    }
                }
            }
        }
        let mut ret = vec![];
        for _i in 0..max {
            ret.push(nums[max_index]);
            max_index = index_array[max_index];
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
            vec![1, 2, 4, 8]
        );
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 3]),
            vec![1, 2]
        );
    }
}
