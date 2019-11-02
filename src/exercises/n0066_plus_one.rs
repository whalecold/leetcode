/**
 * [66] Plus One
 *
 * Given a non-empty array of digits representing a non-negative integer, plus one to the integer.
 *
 * The digits are stored such that the most significant digit is at the head of the list, and each element in the array contain a single digit.
 *
 * You may assume the integer does not contain any leading zero, except the number 0 itself.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 *
 *
 * Example 2:
 *
 *
 * Input: [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut taint = 0;
        for i in (0..digits.len()).rev() {
            digits[i] += 1;
            if digits[i] >= 10 {
                taint = 1;
                digits[i] = 0;
            } else {
                taint = 0;
                break;
            }
        }
        if taint == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_66() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
