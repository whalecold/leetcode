/*
*
*
*
*
*
*
*/

use std::collections::HashSet;

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        let mut words = HashSet::new();
        for word in word_dict.iter() {
            words.insert(word);
        }

        for i in 1..=s.len() {
            for j in i..=s.len() {
                if dp[i - 1] {
                    // 这里有很多重复的检查 下面那种有 break 但是这里 break 去掉会有问题 多个角度想想办法
                    if words.contains(&s[i - 1..=j - 1].to_string()) {
                        dp[j] = true;
                    }
                }
            }
        }
        dp[s.len()]
    }
    #[allow(dead_code)]
    pub fn word_break1(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        let mut words = HashSet::new();
        for word in word_dict.iter() {
            words.insert(word);
        }

        for i in 1..=s.len() {
            for j in (0..i).rev() {
                if dp[j] {
                    if words.contains(&s[j..i].to_string()) {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::word_break(
                "leetcode".to_owned(),
                vec!["leet".to_owned(), "code".to_owned()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_owned(),
                vec!["apple".to_owned(), "pen".to_owned()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                vec![
                    "cats".to_owned(),
                    "dog".to_owned(),
                    "sand".to_owned(),
                    "and".to_owned(),
                    "cat".to_owned()
                ]
            ),
            false
        );

        assert_eq!(
            Solution::word_break1(
                "leetcode".to_owned(),
                vec!["leet".to_owned(), "code".to_owned()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break1(
                "applepenapple".to_owned(),
                vec!["apple".to_owned(), "pen".to_owned()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break1(
                "catsandog".to_owned(),
                vec![
                    "cats".to_owned(),
                    "dog".to_owned(),
                    "sand".to_owned(),
                    "and".to_owned(),
                    "cat".to_owned()
                ]
            ),
            false
        )
    }
}
