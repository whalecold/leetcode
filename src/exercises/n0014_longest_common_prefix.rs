use std::ops::Index;

/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 * Example 1:
 *
 *
 * Input: ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 * Note:
 *
 * All given inputs are in lowercase letters a-z.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        let mut cs = strs[0].to_owned();
        for index in 1..strs.len() {
            for i in 0..cs.len() {
                if i >= strs[index].len() || strs[index].as_bytes()[i] != cs.as_bytes()[i] {
                    cs.split_off(i);
                    break;
                }
            }
        }
        cs
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("car"),
                String::from("dog")
            ]),
            ""
        );
    }
}
