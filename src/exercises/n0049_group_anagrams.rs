/**
 * [49] Group Anagrams
 *
 * Given an array of strings, group anagrams together.
 *
 * Example:
 *
 *
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 *   ["ate","eat","tea"],
 *   ["nat","tan"],
 *   ["bat"]
 * ]
 *
 * Note:
 *
 *
 * 	All inputs will be in lowercase.
 * 	The order of your output does not matter.
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m_temp: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for i in 0..strs.len() {
            let hash = Solution::str_sort(&strs[i]);
            match m_temp.get_mut(&hash) {
                Some(slice) => {
                    slice.push(strs[i].clone());
                    true
                }
                _ => {
                    m_temp.insert(hash.clone(), vec![strs[i].clone()]);
                    false
                }
            };
        }
        let mut ret = vec![];
        for (_, v) in &m_temp {
            ret.push(v.clone());
        }
        ret.sort();
        ret
    }

    fn str_sort(str: &String) -> Vec<i32> {
        let mut ret = vec![0; 26];
        for i in 0..str.len() {
            ret[str.bytes().nth(i).unwrap() as usize - 97 as usize] += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_49() {
        assert_eq!(
            Solution::group_anagrams(vec![
                String::from("eat"),
                String::from("tea"),
                String::from("tan"),
                String::from("ate"),
                String::from("nat"),
                String::from("bat")
            ]),
            vec![
                vec![String::from("bat")],
                vec![
                    String::from("eat"),
                    String::from("tea"),
                    String::from("ate")
                ],
                vec![String::from("tan"), String::from("nat")]
            ]
        );
    }
}
