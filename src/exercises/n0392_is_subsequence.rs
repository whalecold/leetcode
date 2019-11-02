/**
 * [392] Is Subsequence
 *
 *
 * Given a string s and a string t, check if s is subsequence of t.
 *
 *
 *
 * You may assume that there is only lower case English letters in both s and t. t is potentially a very long (length ~= 500,000) string, and s is a short string (<=100).
 *
 *
 *
 * A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).
 *
 *
 * Example 1:<br />
 * s = "abc", t = "ahbgdc"
 *
 *
 * Return true.
 *
 *
 * Example 2:<br />
 * s = "axc", t = "ahbgdc"
 *
 *
 * Return false.
 *
 *
 * Follow up:<br />
 * If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?
 *
 * Credits:<br />Special thanks to <a href="https://leetcode.com/pbrother/">@pbrother</a> for adding this problem and creating all test cases.
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        let sub = s.as_bytes();
        let mut index = 0 as usize;
        for i in t.as_bytes() {
            if index == sub.len() {
                break;
            }
            if *i == sub[index] {
                index += 1;
            }
        }
        return index == sub.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_392() {
        assert_eq!(
            Solution::is_subsequence(String::from("abc"), String::from("abhdgyfc")),
            true
        );
        assert_eq!(
            Solution::is_subsequence(String::from("abc"), String::from("ahdgyf")),
            false
        );
    }
}
