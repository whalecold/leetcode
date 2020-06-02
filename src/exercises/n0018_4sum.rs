use console::colors_enabled;
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
use std::collections::{HashMap, VecDeque};
use std::ops::Index;

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
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
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut deque = vec![VecDeque::new(); 5];
        let mut set = HashMap::new();
        let mut ret = 0;
        set.insert('c', 0 as usize);
        set.insert('r', 1 as usize);
        set.insert('o', 2 as usize);
        set.insert('a', 3 as usize);
        set.insert('k', 4 as usize);
        for i in 0..croak_of_frogs.len() {
            let ch = croak_of_frogs.chars().nth(i).unwrap();
            let index = set.get(&ch).unwrap_or(&5);
            if *index >= deque.len() {
                continue;
            }
            deque[*index].push_back(i);
        }
        let num = deque[0].len();
        for i in 1..deque.len() {
            if deque[i].len() != num {
                return -1;
            }
        }
        for i in 0..num {
            for digit in 1..deque.len() {
                if deque[digit].index(i).le(deque[digit - 1].index(i)) {
                    return -1;
                }
            }
        }
        num as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_18() {
        assert_eq!(Solution::min_number_of_frogs(String::from("croakcroak")), 1);
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
