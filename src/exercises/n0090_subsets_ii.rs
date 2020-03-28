/**
 * [90] Subsets II
 *
 * Given a collection of integers that might contain duplicates, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 *
 * Input: [1,2,2]
 * Output:
 * [
 *   [2],
 *   [1],
 *   [1,2,2],
 *   [2,2],
 *   [1,2],
 *   []
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
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ret = vec![];
        let temp = vec![];
        Solution::backtrack(&nums, 0, &mut ret, temp);
        ret
    }
    fn backtrack(nums: &Vec<i32>, begin: usize, ret: &mut Vec<Vec<i32>>, temp: Vec<i32>) {
        ret.push(temp.clone());
        let mut temp = temp;
        for i in begin..nums.len() {
            // remove duplicate
            if i > begin && nums[i - 1] == nums[i] {
                continue;
            }
            temp.push(nums[i]);
            // select current element
            Solution::backtrack(nums, i + 1, ret, temp.clone());
            // remove current element
            temp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_90() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        )
    }
}
