/**
 * [30] Substring with Concatenation of All Words
 *
 * You are given a string, s, and a list of words, words, that are all of the same length. Find all starting indices of substring(s) in s that is a concatenation of each word in words exactly once and without any intervening characters.
 *  
 * Example 1:
 *
 * Input:
 *   s = "barfoothefoobarman",
 *   words = ["foo","bar"]
 * Output: [0,9]
 * Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
 * The output order does not matter, returning [9,0] is fine too.
 *
 * Example 2:
 *
 * Input:
 *   s = "wordgoodgoodgoodbestword",
 *   words = ["word","good","best","word"]
 * Output: []
 *
 */
use std::collections::HashMap;

#[allow(dead_code)]
static NEED_COMPILE: bool = false;
#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // TODO fix https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/13658/Easy-Two-Map-Solution-(C%2B%2BJava)
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ret = vec![];
        if words.len() == 0 || words.len() == 0 || s.len() < words.len() * words[0].len() {
            return ret;
        }
        let (num, length) = (words.len(), words[0].len());
        let mut counts = HashMap::new();
        for word in words.iter() {
            match counts.get_mut(word.as_str()) {
                Some(value) => {
                    *value += 1;
                }
                _ => {
                    counts.insert(word.as_str(), 1);
                }
            }
        }
        let s = s.as_str();
        for i in 0..=(s.len() - num * length) {
            let mut seen = HashMap::new();
            let mut j = 0 as usize;
            while j < num {
                let (sub, mut v) = (&s[i + j * length..i + (j + 1) * length], 1);
                match seen.get_mut(sub) {
                    Some(value) => {
                        *value += 1;
                        v = *value;
                    }
                    _ => {
                        seen.insert(sub, 1);
                    }
                }
                let c = *counts.get(sub).or(Some(&0)).unwrap();
                if c == 0 || v > c {
                    break;
                }
                j += 1;
            }
            if j == num {
                ret.push(i as i32);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_30() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoothefoobarman"),
                vec![String::from("bar"), String::from("foo")]
            ),
            vec![0, 9]
        );
    }
}
