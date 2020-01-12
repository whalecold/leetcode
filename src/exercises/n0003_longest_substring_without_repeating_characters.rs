/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">"abcabcbb"</span>
 * Output: <span id="example-output-1">3
 * Explanation:</span> The answer is "abc", with the length of 3.
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">"bbbbb"</span>
 * Output: <span id="example-output-2">1
 * </span><span id="example-output-1">Explanation: T</span>he answer is "b", with the length of 1.
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">"pwwkew"</span>
 * Output: <span id="example-output-3">3
 * </span><span id="example-output-1">Explanation: </span>The answer is "wke", with the length of 3.
 *              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 * </div>
 * </div>
 * </div>
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 这种方法比较粗糙一点 还有一种方式是用 map 存储了每次字段的下表，就不再列了。
        if s.len() == 0 {
            return 0
        }
        let mut ret = 1 as i32;
        let mut l = 0 as i32;
        let bytes = s.as_bytes();
        for i in 1..bytes.len() {
            let mut index = i as i32- 1;
            while index >= l  {
                ret = if i as i32 - l  > ret { i as i32 - l } else { ret };
                if bytes[index as usize] == bytes[i] {
                    l = index as i32 + 1;
                    break;
                }
                index -= 1;
            }
        }
        ret = if bytes.len() as i32 - l  > ret { bytes.len() as i32 - l } else { ret };
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("au")),
            2
        );
    }
}
