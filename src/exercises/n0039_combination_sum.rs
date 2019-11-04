/**
 * [39] Combination Sum
 *
 * Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * The same repeated number may be chosen from candidates unlimited number of times.
 *
 * Note:
 *
 *
 * 	All numbers (including target) will be positive integers.
 * 	The solution set must not contain duplicate combinations.
 *
 *
 * Example 1:
 *
 *
 * Input: candidates = [2,3,6,7], target = 7,
 * A solution set is:
 * [
 *   [7],
 *   [2,2,3]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,3,5], target = 8,
 * A solution set is:
 * [
 *   [2,2,2,2],
 *   [2,3,3],
 *   [3,5]
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
    pub fn combination_sum(candidates: &Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let candidates = candidates.clone();
        let mut result = vec![];
        let mut combination = vec![];
        Solution::combination_sum1(&candidates, target, &mut result, &mut combination, 0);
        result
    }
    fn combination_sum1(
        candidates: &Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
        combination: &mut Vec<i32>,
        begin: usize,
    ) {
        if target == 0 {
            result.push(combination.to_vec());
            return;
        }
        let mut i = begin;
        while i < candidates.len() && target > candidates[i] {
            combination.push(candidates[i]);
            Solution::combination_sum1(candidates, target - candidates[i], result, combination, i);
            combination.pop();
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(&vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![2, 2, 3]]
        );
    }
}
