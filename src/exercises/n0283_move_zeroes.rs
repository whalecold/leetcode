/**
 * [283] Move Zeroes
 *
 * Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.
 *
 * Example:
 *
 *
 * Input: [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 *
 * Note:
 *
 * <ol>
 * 	You must do this in-place without making a copy of the array.
 * 	Minimize the total number of operations.
 * </ol>
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // two-pointer
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut begin = -1;
        for i in 0..nums.len() {
            if nums[i] != 0 && begin != -1 {
                nums.swap(i, begin as usize);
                begin += 1;
            }
            if nums[i] == 0 {
                if begin == -1 {
                    begin = i as i32;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_283() {
        let nums = vec![0, 1, 0, 3, 12];
        let mut second = nums.clone();
        Solution::move_zeroes(&mut second);
        assert_eq!(second, vec![1, 3, 12, 0, 0]);
    }
}
