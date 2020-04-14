/**
 * [525] Contiguous Array
 *
 * Given a binary array, find the maximum length of a contiguous subarray with equal number of 0 and 1.
 *
 *
 * Example 1:<br />
 *
 * Input: [0,1]
 * Output: 2
 * Explanation: [0, 1] is the longest contiguous subarray with equal number of 0 and 1.
 *
 *
 *
 * Example 2:<br />
 *
 * Input: [0,1,0]
 * Output: 2
 * Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
 *
 *
 *
 * Note:
 * The length of the given binary array will not exceed 50,000.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut calc: Vec<i32> = vec![0; nums.len() + 1];
        let mut value_map: HashMap<i32, (i32, i32)> = HashMap::new();
        value_map.insert(0, (0, -1));
        for i in 1..=nums.len() {
            if nums[i - 1] == 1 {
                calc[i] = calc[i - 1] + 1;
            } else {
                calc[i] = calc[i - 1] - 1;
            }
            match value_map.get_mut(&calc[i]) {
                Some(value) => {
                    value.1 = i as i32;
                }
                _ => {
                    value_map.insert(calc[i], (i as i32, -1));
                }
            }
        }
        let mut max = 0;
        for (_, v) in value_map {
            if v.1 == -1 {
                continue;
            }
            if v.1 - v.0 > max {
                max = v.1 - v.0;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_525() {
        assert_eq!(Solution::find_max_length(vec![1, 0, 1, 0]), 4);
    }
}
