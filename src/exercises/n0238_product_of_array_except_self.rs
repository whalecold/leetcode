/**
 * [238] Product of Array Except Self
 *
 * Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].
 *
 * Example:
 *
 *
 * Input:  [1,2,3,4]
 * Output: [24,12,8,6]
 *
 *
 * Note: Please solve it without division and in O(n).
 *
 * Follow up:<br />
 * Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 1;
        let mut zero_num = 0;
        let mut ret = vec![];
        for i in 0..nums.len() {
            if nums[i] != 0 {
                sum *= nums[i];
            } else {
                zero_num += 1;
            }
        }
        for i in 0..nums.len() {
            if zero_num > 1 || (zero_num == 1 && nums[i] != 0) {
                ret.push(0);
                continue;
            }
            if nums[i] == 0 {
                ret.push(sum);
            } else {
                ret.push(sum / nums[i]);
            }
        }
        ret
    }
    #[allow(dead_code)]
    pub fn product_except_self1(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![1; nums.len()];
        let (mut left, mut right) = (1, 1);
        for i in 0..nums.len() {
            ret[i] *= left;
            left *= nums[i];
        }
        for i in (0..nums.len()).rev() {
            ret[i] *= right;
            right *= nums[i];
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_238() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(Solution::product_except_self(vec![0, 0]), vec![0, 0]);
        assert_eq!(Solution::product_except_self(vec![1, 0]), vec![0, 1]);
        assert_eq!(Solution::product_except_self(vec![1, 1]), vec![1, 1]);
        assert_eq!(
            Solution::product_except_self(vec![0, 1, 0, 3, 0]),
            vec![0, 0, 0, 0, 0]
        );

        assert_eq!(
            Solution::product_except_self1(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(Solution::product_except_self1(vec![0, 0]), vec![0, 0]);
        assert_eq!(Solution::product_except_self1(vec![1, 0]), vec![0, 1]);
        assert_eq!(Solution::product_except_self1(vec![1, 1]), vec![1, 1]);
        assert_eq!(
            Solution::product_except_self1(vec![0, 1, 0, 3, 0]),
            vec![0, 0, 0, 0, 0]
        );
    }
}
