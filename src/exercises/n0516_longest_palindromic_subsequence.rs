/**
 * [516] Longest Palindromic Subsequence
 *
 * 
 * Given a string s, find the longest palindromic subsequence's length in s. You may assume that the maximum length of s is 1000.
 * 
 * 
 * Example 1:<br>
 * Input: 
 * 
 * "bbbab"
 * 
 * Output: 
 * 
 * 4
 * 
 * One possible longest palindromic subsequence is "bbbb".
 * 
 * 
 * Example 2:<br>
 * Input:
 * 
 * "cbbd"
 * 
 * Output:
 * 
 * 2
 * 
 * One possible longest palindromic subsequence is "bb".
 * 
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len() as usize;
        let mut dp = vec![vec![0; len]; len];

        for i in (0..len).rev() {
            dp[i][i] = 1;
            for j in i+1..len {
                if s[i] == s[j] {
                    dp[i][j] = dp[i+1][j-1] + 2;
                } else {
                    dp[i][j] = i32::max(dp[i+1][j], dp[i][j-1])
                }
            }
        }
        dp[0][len-1]
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_516() {
        assert_eq!(Solution::longest_palindrome_subseq(String::from("abcb")), 3);
        assert_eq!(Solution::longest_palindrome_subseq(String::from("abcbbb")), 4);
    }
}
