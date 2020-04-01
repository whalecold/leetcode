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

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // TODO fix https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/13658/Easy-Two-Map-Solution-(C%2B%2BJava)
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.len() == 0 {
            return vec![];
        }
        let (mut ret, used, length) = (vec![], vec![false; words.len()], words[0].len());
        Solution::backtrack(&s, &words, 0, &mut ret, used, &length);
        ret
    }
    fn backtrack(
        s: &String,
        words: &Vec<String>,
        begin: usize,
        ret: &mut Vec<i32>,
        used: Vec<bool>,
        length: &usize,
    ) {
        let mut i = begin;
        let mut used = used;
        while i + *length <= s.len() {
            let ss = &s[i..i + *length];
            println!("ss {} i {} words {:?}", ss, i, words);
            if let Ok(index) = words.binary_search(&String::from(ss)) {
                println!("index {} ss {} i {}", index, ss, i);
                if used[index] {
                    continue;
                }
                used[index] = true;
                if !used.contains(&false) {
                    Solution::clean_used(&mut used);
                    let n = (i - (words.len() - 1) * (*length)) as i32;
                    if !ret.contains(&n) {
                        ret.push(n);
                    }
                }
                Solution::backtrack(s, words, i + *length, ret, used.clone(), length);
                used[index] = false;
            }
            Solution::clean_used(&mut used);
            i += 1;
        }
    }

    fn clean_used(used: &mut Vec<bool>) {
        for i in 0..used.len() {
            used[i] = false;
        }
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
