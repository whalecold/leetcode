/**
 * [41] First Missing Positive
 *
 * Given an unsorted integer array, find the smallest missing positive integer.
 *
 * Example 1:
 *
 *
 * Input: [1,2,0]
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: [3,4,-1,1]
 * Output: 2
 *
 *
 * Example 3:
 *
 *
 * Input: [7,8,9,11,12]
 * Output: 1
 *
 *
 * Note:
 *
 * Your algorithm should run in O(n) time and uses constant extra space.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        for i in 0..nums.len() {
            while nums[i] > 0
                && nums[i] <= nums.len() as i32
                && nums[nums[i] as usize - 1] != nums[i]
            {
                let j = nums[i].clone() as usize - 1;
                nums.swap(i, j);
            }
        }
        println!("{:?}", nums);
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_41() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 10]), 1);
    }
}
