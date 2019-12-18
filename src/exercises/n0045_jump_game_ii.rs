/**
 * [45] Jump Game II
 *
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 *
 * Each element in the array represents your maximum jump length at that position.
 *
 * Your goal is to reach the last index in the minimum number of jumps.
 *
 * Example:
 *
 *
 * Input: [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2.
 *     Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * Note:
 *
 * You can assume that you can always reach the last index.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let mut solutions = vec![std::usize::MAX; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize + 1 >= nums.len() {
                solutions[i] = 1;
                continue;
            }
            for j in (1..=nums[i]).rev() {
                let j = j as usize + i;
                if solutions[j] < solutions[i] - 1 {
                    solutions[i] = solutions[j] + 1;
                }
            }
        }
        solutions[0] as i32
    }
    #[allow(dead_code)]
    pub fn jump1(nums: Vec<i32>) -> i32 {
        let (mut cur_end, mut jumps, mut farthest) = (0 as usize, 0, 0 as usize);
        for i in 0..(nums.len() - 1) {
            farthest = usize::max(farthest, i + nums[i] as usize);
            if i == cur_end {
                jumps += 1;
                cur_end = farthest;
            }
        }
        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_45() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![0]), 0);
        assert_eq!(Solution::jump(vec![1]), 0);

        assert_eq!(Solution::jump1(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump1(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(Solution::jump1(vec![0]), 0);
        assert_eq!(Solution::jump1(vec![1]), 0);
    }
}
