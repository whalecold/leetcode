/**
 * [42] Trapping Rain Water
 *
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png" style="width: 412px; height: 161px;" /><br />
 * <small>The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped. Thanks Marcos for contributing this image!</small>
 *
 * Example:
 *
 *
 * Input: [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        let (mut left, mut right) = (0 as usize, height.len() - 1);
        let (mut max_left, mut max_right) = (0 as usize, 0 as usize);
        let mut sum = 0;
        while left < right {
            let (height_left, height_right) = (height[left] as usize, height[right] as usize);
            max_left = usize::max(height_left, max_left);
            max_right = usize::max(height_right, max_right);
            // 这里保证了下面 max - [left/right] 是肯定成立的，只要有一方的 max 大于另外一方。
            if max_left < max_right {
                sum += (max_left - height_left) as i32;
                left += 1;
            } else {
                sum += (max_right - height_right) as i32;
                right -= 1;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_42() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
