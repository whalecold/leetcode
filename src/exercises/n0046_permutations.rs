/**
 * [46] Permutations
 *
 * Given a collection of distinct integers, return all possible permutations.
 *
 * Example:
 *
 *
 * Input: [1,2,3]
 * Output:
 * [
 *   [1,2,3],
 *   [1,3,2],
 *   [2,1,3],
 *   [2,3,1],
 *   [3,1,2],
 *   [3,2,1]
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
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut temp = vec![];
        Solution::backtrack(&nums, &mut ret, temp);
        ret
    }
    fn backtrack(nums: &Vec<i32>, ret: &mut Vec<Vec<i32>>, temp: Vec<i32>) {
        if temp.len() == nums.len() {
            ret.push(temp);
            return;
        }
        let mut temp = temp;
        for i in 0..nums.len() {
            if temp.contains(&nums[i]) {
                continue;
            }
            temp.push(nums[i]);
            Solution::backtrack(nums, ret, temp.clone());
            temp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_46() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
