/// Given a non-negative integer n, count all numbers with unique digits, x, where 0 ≤ x < 10n.
//
/// Example:
///
/// Input: 2
/// Output: 91
/// Explanation: The answer should be the total numbers in the range of 0 ≤ x < 100,
//             excluding 11,22,33,44,55,66,77,88,99

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        } else if n == 1 {
            return 10;
        }
        let mut count = 9;
        for i in 1..n {
            count = count * (10 - i)
        }
        Solution::count_numbers_with_unique_digits(n - 1) + count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_357() {
        assert_eq!(Solution::count_numbers_with_unique_digits(1), 10);
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
        assert_eq!(Solution::count_numbers_with_unique_digits(3), 739);
        assert_eq!(Solution::count_numbers_with_unique_digits(4), 5275);
        assert_eq!(Solution::count_numbers_with_unique_digits(100), 8877691);
    }
}
