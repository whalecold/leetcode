/**
 * [47] Permutations II
 *
 * Given a collection of numbers that might contain duplicates, return all possible unique permutations.
 *
 * Example:
 *
 *
 * Input: [1,1,2]
 * Output:
 * [
 *   [1,1,2],
 *   [1,2,1],
 *   [2,1,1]
 * ]
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut ret, temp, used) = (vec![], vec![], vec![false; nums.len()]);
        let mut nums = nums;
        nums.sort();
        Solution::backtrack(&nums, temp, used, &mut ret);
        ret
    }
    fn backtrack(nums: &Vec<i32>, temp: Vec<i32>, used: Vec<bool>, ret: &mut Vec<Vec<i32>>) {
        if temp.len() == nums.len() {
            ret.push(temp);
            return;
        }
        let (mut used, mut temp) = (used, temp);
        for i in 0..nums.len() {
            if used[i] || i > 0 && nums[i - 1] == nums[i] && !used[i - 1] {
                continue;
            }
            used[i] = true;
            temp.push(nums[i]);
            Solution::backtrack(nums, temp.clone(), used.clone(), ret);
            used[i] = false;
            temp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_47() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }
}
