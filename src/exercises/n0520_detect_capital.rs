/**
 * [520] Detect Capital
 *
 * Given a word, you need to judge whether the usage of capitals in it is right or not.
 *
 * We define the usage of capitals in a word to be right when one of the following cases holds:
 *
 * <ol>
 * 	All letters in this word are capitals, like "USA".
 * 	All letters in this word are not capitals, like "leetcode".
 * 	Only the first letter in this word is capital, like "Google".
 * </ol>
 * Otherwise, we define that this word doesn't use capitals in a right way.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: "USA"
 * Output: True
 *
 *
 *  
 *
 * Example 2:
 *
 *
 * Input: "FlaG"
 * Output: False
 *
 *
 *  
 *
 * Note: The input will be a non-empty word consisting of uppercase and lowercase latin letters.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn detect_capital_use(word: String) -> bool {
        let mut index = 0;
        let mut upper_num = 0;
        for w in word.as_bytes().iter() {
            if w.is_ascii_uppercase() {
                if index != 0 && index != upper_num {
                    return false;
                }
                upper_num += 1;
            } else if upper_num > 1 {
                return false;
            }
            index += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_520() {
        assert!(
            super::Solution::detect_capital_use(String::from("FFFa")),
            false
        )
    }
}
