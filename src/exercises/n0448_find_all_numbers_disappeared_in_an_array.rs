/**
 * [448] Find All Numbers Disappeared in an Array
 *
 * Given an array of integers where 1 &le; a[i] &le; n (n = size of array), some elements appear twice and others appear once.
 *
 * Find all the elements of [1, n] inclusive that do not appear in this array.
 *
 * Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.
 *
 * Example:
 *
 * Input:
 * [4,3,2,7,8,2,3,1]
 *
 * Output:
 * [5,6]
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            while nums[i] > 0
                && nums[i] as usize <= nums.len()
                && nums[i] != nums[nums[i] as usize - 1]
            {
                let j = nums[i].clone() as usize - 1;
                nums.swap(i, j);
            }
        }
        let mut ret = vec![];
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                ret.push(i as i32 + 1);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_448() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![1, 1, 3, 3, 5]),
            vec![2, 4]
        );
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }
}
