/// Given a string s, find the longest palindromic substring in s.
/// You may assume that the maximum length of s is 1000.
///
/// ### Example 1:
/// `
/// Input: "babad"
/// Output: "bab"
/// Note: "aba" is also a valid answer.
/// `
/// ### Example 2:
/// `
/// Input: "cbbd";
/// Output: "bb"

/// `

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    /// solved by dp
    #[allow(dead_code)]
    pub fn longest_palindrome_1(s: String) -> String {
        let seq: Vec<u8> = s.bytes().collect();
        let len = seq.len();
        if len <= 1 {
            return s;
        }

        let (mut max_len, mut max_start) = (0, 0);
        let mut lp = vec![vec![false; len]; len];

        for i in 0..len {
            for j in (0..=i).rev() {
                if i - j < 2 {
                    lp[j][i] = seq[i] == seq[j];
                } else {
                    lp[j][i] = (seq[i] == seq[j]) && lp[j + 1][i - 1];
                }
                if lp[j][i] && i - j + 1 > max_len {
                    max_len = i - j + 1;
                    max_start = j;
                }
            }
        }
        s[max_start..max_start + max_len].to_owned()
    }
    /// speed from mid
    #[allow(dead_code)]
    pub fn longest_palindrome_2(s: String) -> String {
        let seq: Vec<u8> = s.bytes().collect();
        let len = seq.len();
        if len <= 1 {
            return s;
        }
        let (mut max_len, mut max_start) = (0, 0);
        for idx in 0..len {
            let (mut start, mut end) = (idx, idx);
            while start > 0 && seq[start - 1] == seq[idx] {
                start -= 1;
            }
            while end < len - 1 && seq[end + 1] == seq[idx] {
                end += 1;
            }
            while start > 0 && end < len - 1 && seq[start - 1] == seq[end + 1] {
                start -= 1;
                end += 1;
            }
            if end - start + 1 > max_len {
                max_len = end - start + 1;
                max_start = start;
            }
        }
        s[max_start..max_start + max_len].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_5_1() {
        assert_eq!(Solution::longest_palindrome_1("babad".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome_1("cbbd".to_owned()), "bb");
        assert_eq!(
            Solution::longest_palindrome_1("aaaaaa".to_owned()),
            "aaaaaa"
        );
        assert_eq!(Solution::longest_palindrome_1("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome_1("a".to_owned()), "a");
        assert_eq!(Solution::longest_palindrome_1("abcda".to_owned()), "a");
    }

    #[test]
    fn test_5_2() {
        assert_eq!(Solution::longest_palindrome_2("babad".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome_2("cbbd".to_owned()), "bb");
        assert_eq!(
            Solution::longest_palindrome_2("aaaaaa".to_owned()),
            "aaaaaa"
        );
        assert_eq!(Solution::longest_palindrome_2("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome_2("a".to_owned()), "a");
        assert_eq!(Solution::longest_palindrome_2("abcda".to_owned()), "a");
    }
}
