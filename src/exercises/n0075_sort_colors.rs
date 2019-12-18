/**
 * [75] Sort Colors
 *
 * Given an array with n objects colored red, white or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white and blue.
 *
 * Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.
 *
 * Note: You are not suppose to use the library's sort function for this problem.
 *
 * Example:
 *
 *
 * Input: [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 *
 * Follow up:
 *
 *
 * 	A rather straight forward solution is a two-pass algorithm using counting sort.<br />
 * 	First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
 * 	Could you come up with a one-pass algorithm using only constant space?
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // two-pass or one-pass
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let (mut r, mut w, mut b) = (0 as usize, 0 as usize, nums.len() - 1);
        while w <= b {
            if nums[w] == 0 {
                nums.swap(r, w);
                w += 1;
                r += 1;
            } else if nums[w] == 1 {
                w += 1;
            } else {
                nums.swap(w, b);
                b -= 1;
            }
            if b == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_75() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);

        let mut v = vec![2];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![2]);

        let mut v = vec![2, 2];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![2, 2]);
    }
}
