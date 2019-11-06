/**
 * [442] Find All Duplicates in an Array
 *
 * Given an array of integers, 1 &le; a[i] &le; n (n = size of array), some elements appear twice and others appear once.
 *
 * Find all the elements that appear twice in this array.
 *
 * Could you do it without extra space and in O(n) runtime?
 *
 * Example:<br/>
 *
 * Input:
 * [4,3,2,7,8,2,3,1]
 *
 * Output:
 * [2,3]
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = vec![];
        for i in 0..nums.len() {
            while nums[i] > 0
                && nums[i] as usize <= nums.len()
                && nums[i] != nums[nums[i] as usize - 1]
            {
                let j = nums[i].clone() as usize - 1;
                nums.swap(i, j);
            }
        }
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                ret.push(nums[i].clone());
            }
        }
        ret.sort();
        ret
    }
    #[allow(dead_code)]
    pub fn find_duplicates1(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = vec![];
        for i in 0..nums.len() {
            let val = nums[i].abs() - 1;
            if nums[val as usize] < 0 {
                ret.push(nums[i].abs());
            }
            nums[val as usize] = -nums[val as usize];
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_442() {
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
        assert_eq!(
            Solution::find_duplicates1(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
    }
}
