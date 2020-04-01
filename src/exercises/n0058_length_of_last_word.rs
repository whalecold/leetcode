/**
 * [58] Length of Last Word
 *
 * Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word (last word means the last appearing word if we loop from left to right) in the string.
 * If the last word does not exist, return 0.
 * Note: A word is defined as a maximal substring consisting of non-space characters only.
 * Example:
 *
 * Input: "Hello World"
 * Output: 5
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
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;
        for i in 0..s.len() {
            if s.chars().nth(s.len() - 1 - i).unwrap().eq(&' ') {
                if length != 0 {
                    break;
                }
                continue;
            }
            length += 1;
        }
        length
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_58() {
        assert_eq!(
            Solution::length_of_last_word(String::from("hello world")),
            5
        );
        assert_eq!(
            Solution::length_of_last_word(String::from("hello world   ")),
            5
        );
        assert_eq!(
            Solution::length_of_last_word(String::from("Today is a nice day")),
            3
        );
    }
}
