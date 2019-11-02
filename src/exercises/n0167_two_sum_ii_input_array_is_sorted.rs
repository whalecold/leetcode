/**
 * [167] Two Sum II - Input array is sorted
 *
 * Given an array of integers that is already sorted in ascending order, find two numbers such that they add up to a specific target number.
 * 
 * The function twoSum should return indices of the two numbers such that they add up to the target, where index1 must be less than index2.
 * 
 * Note:
 * 
 * 
 * 	Your returned answers (both index1 and index2) are not zero-based.
 * 	You may assume that each input would have exactly one solution and you may not use the same element twice.
 * 
 * 
 * Example:
 * 
 * 
 * Input: numbers = [2,7,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
 * 
 */

use std::collections::HashMap;
#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::with_capacity(numbers.len());
        for i in 0..numbers.len() {

            if let Some(index) = m.get(&(target - numbers[i])) {
                return vec![*index + 1, i as i32 + 1]
            }
            m.insert(numbers[i],i as i32);
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_167() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
