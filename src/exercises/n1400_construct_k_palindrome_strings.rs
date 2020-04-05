/**
 * [1400] Construct K Palindrome Strings
 *
 * Given a string s and an integer k. You should construct k non-empty palindrome strings using all the characters in s.
 *
 * Return True if you can use all the characters in s to construct k palindrome strings or False otherwise.
 *
 *  
 * Example 1:
 *
 *
 * Input: s = "annabelle", k = 2
 * Output: true
 * Explanation: You can construct two palindromes using all characters in s.
 * Some possible constructions "anna" + "elble", "anbna" + "elle", "anellena" + "b"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "leetcode", k = 3
 * Output: false
 * Explanation: It is impossible to construct 3 palindromes using all the characters of s.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "true", k = 4
 * Output: true
 * Explanation: The only possible solution is to put each character in a separate string.
 *
 *
 * Example 4:
 *
 *
 * Input: s = "yzyzyzyzyzyzyzy", k = 2
 * Output: true
 * Explanation: Simply you can put all z's in one string and all y's in the other string. Both strings will be palindrome.
 *
 *
 * Example 5:
 *
 *
 * Input: s = "cr", k = 7
 * Output: false
 * Explanation: We don't have enough characters in s to construct 7 palindromes.
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= s.length <= 10^5
 * 	All characters in s are lower-case English letters.
 * 	1 <= k <= 10^5
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        let mut slice = vec![0; 26];
        let chars = s.as_bytes();
        for i in 0..chars.len() {
            let index = (chars[i] - 97) as usize;
            slice[index] += 1;
        }
        let mut odd_number = 0;
        for i in 0..slice.len() {
            slice[i] %= 2;
            if slice[i] == 1 {
                odd_number += 1;
            }
        }
        odd_number <= k
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1400() {}
}
