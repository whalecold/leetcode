/**
 * [40] Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * Each number in candidates may only be used once in the combination.
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
 * Input: candidates = [10,1,2,7,6,1,5], target = 8,
 * A solution set is:
 * [
 *   [1, 7],
 *   [1, 2, 5],
 *   [2, 6],
 *   [1, 1, 6]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,5,2,1,2], target = 5,
 * A solution set is:
 * [
 *   [1,2,2],
 *   [5]
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
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut ret = vec![];
        let mut one = vec![];
        candidates.sort();
        Solution::combination(&candidates, 0, target, one, &mut ret);
        ret
    }

    fn combination(
        candidates: &Vec<i32>,
        index: usize,
        target: i32,
        one: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(one);
            return;
        }
        if target < 0 {
            return;
        }
        let mut one = one;
        for i in index..candidates.len() {
            if i > index && candidates[i] == candidates[i - 1] {
                continue;
            }
            one.push(candidates[i]);
            Solution::combination(
                candidates,
                i + 1,
                target - candidates[i],
                one.clone(),
                result,
            );
            one.remove(one.len() - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_40() {
        assert_eq!(
            Solution::combination_sum2(vec![1, 2, 3, 7], 8),
            vec![vec![1, 7]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
