/**
 * [127] Word Ladder
 *
 * Given two words (beginWord and endWord), and a dictionary's word list, find the length of shortest
 * transformation sequence from beginWord to endWord, such that:
 *
 * <ol>
 * 	Only one letter can be changed at a time.
 * 	Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
 * </ol>
 *
 * Note:
 *
 *
 * 	Return 0 if there is no such transformation sequence.
 * 	All words have the same length.
 * 	All words contain only lowercase alphabetic characters.
 * 	You may assume no duplicates in the word list.
 * 	You may assume beginWord and endWord are non-empty and are not the same.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * beginWord = "hit",
 * endWord = "cog",
 * wordList = ["hot","dot","dog","lot","log","cog"]
 *
 * Output: 5
 *
 * Explanation: As one shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog",
 * return its length 5.
 *
 *
 * Example 2:
 *
 *
 * Input:
 * beginWord = "hit"
 * endWord = "cog"
 * wordList = ["hot","dot","dog","lot","log"]
 *
 * Output: 0
 *
 * Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
 *
 *
 *
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let len = word_list.len();
        let target = word_list.iter().position(|x|x == &end_word);
        if target.is_none() {
            return 0
        }
        let target = target.unwrap();
        let mut deq = VecDeque::new();
        let mut dis = vec![0; len];
        let mut remain = (0..len).collect::<HashSet<_>>();
        remain.remove(&target);
        deq.push_back(target);
        while let Some(i) = deq.pop_front() {
            if Solution::is_ladder(&word_list[i], &begin_word) {
                return dis[i] + 2;
            }
            remain.retain(|&j|{
                if Solution::is_ladder(&word_list[i], &word_list[j]) {
                    deq.push_back(j);
                    dis[j] = dis[i] + 1;
                    false
                } else {
                    true
                }
            })
        }
        0
    }
    #[inline(always)]
    fn is_ladder(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() {
            return false
        }
        let mut iter1 = s1.chars().into_iter();
        let mut iter2 = s2.chars().into_iter();
        let mut diff = 0;
        while let (Some(c1), Some(c2)) = (iter1.next(), iter2.next()) {
            if c1 != c2 {
                diff += 1;
                if diff >= 2 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_127() {
        assert_eq!(
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                vec![
                    String::from("hot"),
                    String::from("dot"),
                    String::from("dog"),
                    String::from("lot"),
                    String::from("log"),
                    String::from("cog"),
                ]
            ),
            5
        );
    }
}
