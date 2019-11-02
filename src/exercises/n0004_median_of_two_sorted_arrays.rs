/**
 * [4] Median of Two Sorted Arrays
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 *
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 *
 * You may assume nums1 and nums2 cannot be both empty.
 *
 * Example 1:
 *
 *
 * nums1 = [1, 3]
 * nums2 = [2]
 *
 * The median is 2.0
 *
 *
 * Example 2:
 *
 *
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 *
 * The median is (2 + 3)/2 = 2.5
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut nums1, mut nums2) = (&nums1, &nums2);
        if nums1.len() > nums2.len() {
            let temp = nums2;
            nums2 = nums1;
            nums1 = temp;
        }
        let (m, n) = (nums1.len(), nums2.len());
        let (mut min, mut max) = (0, m);
        let half = (m + n + 1) / 2;
        while min <= max {
            let i = (min + max) / 2;
            let j = half - i;
            if i > min && nums1[i - 1] >= nums2[j] {
                max = i - 1;
            } else if i < max && nums2[j - 1] > nums1[i] {
                min = i + 1;
            } else {
                let max_left: f64;
                if i == 0 {
                    max_left = nums2[j - 1] as f64;
                } else if j == 0 {
                    max_left = nums1[i - 1] as f64;
                } else {
                    max_left = i32::max(nums2[j - 1], nums1[i - 1]) as f64;
                }
                if (m + n) % 2 == 1 {
                    return max_left;
                }
                let min_right: f64;
                if i == m {
                    min_right = nums2[j] as f64;
                } else if j == n {
                    min_right = nums1[i] as f64;
                } else {
                    min_right = i32::min(nums1[i], nums2[j]) as f64;
                }
                return (min_right + max_left) / 2 as f64;
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
//        assert_eq!(
//            Solution::find_median_sorted_arrays(vec![1], vec![2, 3]),
//            2.0
//        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
