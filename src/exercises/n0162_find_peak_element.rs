/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is greater than its neighbors.
 *
 * Given an input array nums, where nums[i] &ne; nums[i+1], find a peak element and return its index.
 *
 * The array may contain multiple peaks, in that case return the index to any one of the peaks is fine.
 *
 * You may imagine that nums[-1] = nums[n] = -&infin;.
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 1 or 5
 * Explanation: Your function can return either index number 1 where the peak element is 2,
 *              or index number 5 where the peak element is 6.
 *
 *
 * Note:
 *
 * Your solution should be in logarithmic complexity.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid1 = (low + high) / 2;
            let mid2 = mid1 + 1;
            if nums[mid1] < nums[mid2] {
                low = mid2;
            } else {
                high = mid1;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_162() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4, 5, 6]), 5);
        assert_eq!(Solution::find_peak_element(vec![8, 7, 6, 5, 1]), 0);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
