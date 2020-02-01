use std::collections::VecDeque;

/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
 *
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 *
 * <img src="http://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png" />
 *
 * Example:
 *
 *
 * Input: "23"
 * Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 *
 *
 * Note:
 *
 * Although the above answer is in lexicographical order, your answer could be in any order you want.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ret = VecDeque::new();
        if digits.len() == 0 {
            return Vec::from(ret);
        }
        ret.push_back(String::new());
        let mapping = vec![
            "0", "1", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        for i in 0..digits.len() {
            while ret.front().unwrap().len() == i {
                let t = ret.pop_front().unwrap();
                let map_str = mapping
                    [digits.chars().nth(i).unwrap().to_digit(10).unwrap() as usize]
                    .to_owned();
                for j in 0..map_str.len() {
                    let mut t = t.clone();
                    t.push(map_str.chars().nth(j).unwrap());
                    ret.push_back(t);
                }
            }
        }
        Vec::from(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            vec![
                String::from("ad"),
                String::from("ae"),
                String::from("af"),
                String::from("bd"),
                String::from("be"),
                String::from("bf"),
                String::from("cd"),
                String::from("ce"),
                String::from("cf")
            ]
        );
    }
}
