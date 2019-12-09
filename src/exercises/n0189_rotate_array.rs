/**
 * [189] Rotate Array
 *
 * Given an array, rotate the array to the right by k steps, where k is non-negative.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3,4,5,6,7] and k = 3
 * Output: [5,6,7,1,2,3,4]
 * Explanation:
 * rotate 1 steps to the right: [7,1,2,3,4,5,6]
 * rotate 2 steps to the right: [6,7,1,2,3,4,5]
 * rotate 3 steps to the right: [5,6,7,1,2,3,4]
 *
 *
 * Example 2:
 *
 *
 * Input: [-1,-100,3,99] and k = 2
 * Output: [3,99,-1,-100]
 * Explanation:
 * rotate 1 steps to the right: [99,-1,-100,3]
 * rotate 2 steps to the right: [3,99,-1,-100]
 *
 *
 * Note:
 *
 *
 * 	Try to come up as many solutions as you can, there are at least 3 different ways to solve this problem.
 * 	Could you do it in-place with O(1) extra space?
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        nums.reverse();
        nums[0..k].reverse();
        nums[k..len].reverse();
    }

    #[allow(dead_code)]
    pub fn rotate1(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        if len == 0 || k == 0 {
            return;
        }
        let k = k as usize;
        let mut rotate_num = 0 as usize;
        let mut start = 0 as usize;
        let mut temp = nums[0];
        let mut cur = 0 as usize;

        while rotate_num < len {
            loop {
                let t = nums[(cur + k) % len];
                nums[(cur + k) % len] = temp;
                temp = t;

                cur = (cur + k) % len;
                rotate_num += 1;
                if cur == start || rotate_num == len {
                    break;
                }
            }
            start += 1;
            cur = start;
            temp = nums[cur];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_189() {
        let mut v1 = vec![1, 2, 3, 4, 5];
        Solution::rotate(&mut v1, 1);
        assert_eq!(v1, vec![5, 1, 2, 3, 4]);
        let mut v2 = vec![1, 2, 3, 4, 5, 6];
        Solution::rotate(&mut v2, 2);
        assert_eq!(v2, vec![5, 6, 1, 2, 3, 4]);

        let mut v11 = vec![1, 2, 3, 4, 5];
        Solution::rotate1(&mut v11, 1);
        assert_eq!(v11, vec![5, 1, 2, 3, 4]);
        let mut v12 = vec![1, 2, 3, 4, 5, 6];
        Solution::rotate1(&mut v12, 2);
        assert_eq!(v12, vec![5, 6, 1, 2, 3, 4]);
    }
}
