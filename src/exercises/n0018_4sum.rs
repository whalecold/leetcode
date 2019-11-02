/**
 * [18] 4Sum
 *
 * Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
 *
 * Note:
 *
 * The solution set must not contain duplicate quadruplets.
 *
 * Example:
 *
 *
 * Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
 *
 * A solution set is:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        if nums.len() < 4 {
            return ret;
        }
        let mut nums = nums;
        nums.sort();
        for low in 0..nums.len() - 3 {
            if low != 0 && nums[low] == nums[low - 1] {
                continue;
            }
            for hi in low + 1..nums.len() - 2 {
                if nums[hi] == nums[hi - 1] && hi != low + 1 {
                    continue;
                }
                let (mut l, mut r) = (hi + 1, nums.len() - 1);
                while l < r {
                    let sum = nums[low] + nums[hi] + nums[l] + nums[r];
                    if sum > target {
                        r -= 1;
                    } else if sum < target {
                        l += 1;
                    } else {
                        ret.push(vec![nums[low], nums[hi], nums[l], nums[r]]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![0, 0, 0, 0], 0),
            vec![vec![0, 0, 0, 0]]
        );
    }
}
