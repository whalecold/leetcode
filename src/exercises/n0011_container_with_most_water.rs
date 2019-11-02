/**
 * [11] Container With Most Water
 *
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.
 *
 * Note: You may not slant the container and n is at least 2.
 *
 *  
 *
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg" style="width: 600px; height: 287px;" />
 *
 * <small>The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49. </small>
 *
 *  
 *
 * Example:
 *
 *
 * Input: [1,8,6,2,5,4,8,3,7]
 * Output: 49
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max = 0;
        loop {
            if left == right {
                break;
            }
            max = i32::max(
                max,
                (right - left) as i32 * i32::min(height[left], height[right]),
            );
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
    }
}
