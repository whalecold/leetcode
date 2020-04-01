/**
 * [50] Pow(x, n)
 *
 * Implement <a href="http://www.cplusplus.com/reference/valarray/pow/" target="_blank">pow(x, n)</a>, which calculates x raised to the power n (x^<span style="font-size:10.8333px">n</span>).
 *
 * Example 1:
 *
 *
 * Input: 2.00000, 10
 * Output: 1024.00000
 *
 *
 * Example 2:
 *
 *
 * Input: 2.10000, 3
 * Output: 9.26100
 *
 *
 * Example 3:
 *
 *
 * Input: 2.00000, -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *
 * Note:
 *
 *
 * 	-100.0 < x < 100.0
 * 	n is a 32-bit signed integer, within the range [-2^31, 2^31 - 1]
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1 as f64;
        }
        let (mut x, mut n) = (x, n);
        if n < 0 {
            if n == std::i32::MIN {
                n += 2;
            }
            x = 1 as f64 / x;
            n = -n;
        }
        if n % 2 == 0 {
            Solution::my_pow(x * x, n / 2)
        } else {
            x * Solution::my_pow(x * x, n / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_50() {
        assert_eq!(Solution::my_pow(1 as f64, 0), 1 as f64);
        assert_eq!(Solution::my_pow(3.00000, -2147483648), 0.0);
    }
}
