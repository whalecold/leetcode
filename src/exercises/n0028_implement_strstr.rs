/**
 * [28] Implement strStr()
 *
 * Implement <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strStr()</a>.
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 *
 * Example 1:
 *
 *
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 *
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strstr()</a> and Java's <a href="https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)" target="_blank">indexOf()</a>.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn str_str1(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if needle.len() > haystack.len() {
            return -1;
        }
        for i in 0..=(haystack.len() - needle.len()) {
            let ln = needle.len();
            let mut j = 0;
            while j < ln {
                if !haystack.as_bytes()[i + j].eq(&needle.as_bytes()[j]) {
                    break;
                }
                j += 1;
            }
            if j == ln {
                return i as i32;
            }
        }
        -1
    }
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        let kmp = Solution::kmpProcess(&needle);
        let mut i = 0;
        while i <= haystack.len() - needle.len() {
            let mut j = 0;
            while j < needle.len() {
                if !haystack.as_bytes()[i + j].eq(&needle.as_bytes()[j]) {
                    break;
                }
                j += 1;
            }
            if j == needle.len() {
                return i as i32;
            }
            if j == 0 {
                i += 1;
            } else {
                i = i + j - kmp[j - 1];
            }
        }
        -1
    }
    fn kmpProcess(needle: &String) -> Vec<usize> {
        let mut ret = vec![0; needle.len()];
        let (mut i, mut len) = (1 as usize, 0 as usize);
        while i < needle.len() {
            if needle.as_bytes()[i].eq(&needle.as_bytes()[len]) {
                len += 1;
                ret[i] = len;
                i += 1;
            } else if len > 0 {
                len = ret[len - 1];
            } else {
                ret[i] = 0;
                i += 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_28() {
        assert_eq!(
            Solution::str_str(String::from("hello"), String::from("llo")),
            2
        );
        assert_eq!(
            Solution::str_str(String::from("aaaaa"), String::from("bba")),
            -1
        );
        assert_eq!(Solution::str_str(String::from("a"), String::from("a")), 0);

        assert_eq!(
            Solution::str_str(String::from("mississippi"), String::from("issip")),
            4
        );
    }
}
