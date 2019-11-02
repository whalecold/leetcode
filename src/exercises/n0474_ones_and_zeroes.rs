/**
 * [474] Ones and Zeroes
 *
 * In the computer world, use restricted resource you have to generate maximum benefit is what we always want to pursue.
 *
 * For now, suppose you are a dominator of m 0s and n 1s respectively. On the other hand, there is an array with strings consisting of only 0s and 1s.
 *
 * Now your task is to find the maximum number of strings that you can form with given m 0s and n 1s. Each 0 and 1 can be used at most once.
 *
 * Note:
 *
 * <ol>
 * 	The given numbers of 0s and 1s will both not exceed 100
 * 	The size of given string array won't exceed 600.
 * </ol>
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: Array = {"10", "0001", "111001", "1", "0"}, m = 5, n = 3
 * Output: 4
 *
 * Explanation: This are totally 4 strings can be formed by the using of 5 0s and 3 1s, which are &ldquo;10,&rdquo;0001&rdquo;,&rdquo;1&rdquo;,&rdquo;0&rdquo;
 *
 *
 *  
 *
 * Example 2:
 *
 *
 * Input: Array = {"10", "0", "1"}, m = 1, n = 1
 * Output: 2
 *
 * Explanation: You could form "10", but then you'd have nothing left. Better form "0" and "1".
 *
 *
 *  
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for str in strs.iter() {
            let mut ones = 0;
            let mut zeros = 0;
            for s in str.bytes() {
                if s == '0' as u8 {
                    zeros += 1;
                }
                if s == '1' as u8 {
                    ones += 1;
                }
            }
            println!("str {} zeros {} ones {}", str, zeros, ones);
            for i in (zeros..=m).rev() {
                for j in (ones..=n).rev() {
                    dp[i][j] = i32::max(dp[i][j], dp[i - zeros][j - ones] + 1);
                }
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_474() {
        assert_eq!(
            Solution::find_max_form(
                vec![
                    String::from("10"),
                    String::from("0001"),
                    String::from("111001"),
                    String::from("1"),
                    String::from("0")
                ],
                5,
                3
            ),
            4
        );
    }
}
