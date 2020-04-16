/**
 * [678] Valid Parenthesis String
 *
 *
 * Given a string containing only three types of characters: '(', ')' and '*', write a function to check whether this string is valid. We define the validity of a string by these rules:
 * <ol>
 * Any left parenthesis '(' must have a corresponding right parenthesis ')'.
 * Any right parenthesis ')' must have a corresponding left parenthesis '('.
 * Left parenthesis '(' must go before the corresponding right parenthesis ')'.
 * '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string.
 * An empty string is also valid.
 * </ol>
 *
 *
 * Example 1:<br />
 *
 * Input: "()"
 * Output: True
 *
 *
 *
 * Example 2:<br />
 *
 * Input: "(*)"
 * Output: True
 *
 *
 *
 * Example 3:<br />
 *
 * Input: "(*))"
 * Output: True
 *
 *
 *
 * Note:<br>
 * <ol>
 * The string size will be in the range [1, 100].
 * </ol>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn check_valid_string(s: String) -> bool {
        let mut any_num = 0;
        let mut brackets_num = 0;
        let mut left_asterisk = 0;
        let s = s.as_bytes();
        for i in 0..s.len() {
            let ch = s[i] as char;
            if ch.eq(&')') {
                if brackets_num == 0 && any_num == 0 {
                    return false;
                }
                if brackets_num == 0 {
                    any_num -= 1;
                } else {
                    brackets_num -= 1;
                    if left_asterisk > brackets_num {
                        left_asterisk = brackets_num;
                    }
                }
            } else if ch.eq(&'*') {
                any_num += 1;
                if left_asterisk < brackets_num {
                    left_asterisk += 1;
                }
            } else {
                brackets_num += 1;
            }
        }
        left_asterisk >= brackets_num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_678() {
        assert_eq!(
            Solution::check_valid_string(String::from("((**********(")),
            false
        );
        assert_eq!(
            Solution::check_valid_string(String::from("(*(*)(((*)()")),
            false
        );
        assert_eq!(Solution::check_valid_string(String::from("(**)((*")), false);
        assert_eq!(Solution::check_valid_string(String::from(")***")), false);
        assert_eq!(Solution::check_valid_string(String::from("***(")), false);
        assert_eq!(Solution::check_valid_string(String::from("(*)")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*))")), true);
        assert_eq!(Solution::check_valid_string(String::from("((*)")), true);
        assert_eq!(Solution::check_valid_string(String::from("(((*)")), false);
        assert_eq!(Solution::check_valid_string(String::from("(*)))")), false);
    }
}
