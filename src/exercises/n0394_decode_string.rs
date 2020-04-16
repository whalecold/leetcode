/**
 * [394] Decode String
 *
 * Given an encoded string, return its decoded string.
 * The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
 * You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.
 * Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].
 * Examples:
 *
 * s = "3[a]2[bc]", return "aaabcbc".
 * s = "3[a2[c]]", return "accaccacc".
 * s = "2[abc]3[cd]ef", return "abcabccdcdcdef".
 *
 *  
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn decode_string(s: String) -> String {
        let (mut count_deque, mut str_deque) = (VecDeque::new(), VecDeque::new());
        let mut str = String::new();
        let length = s.len();
        let mut index = 0 as usize;
        let sss = s.as_bytes();
        while index < length {
            let mut ch = sss[index] as char;
            if ch.is_digit(10) {
                let mut count = 0;
                loop {
                    count = count * 10 + ch.to_digit(10).unwrap();
                    index += 1;
                    ch = sss[index] as char;
                    if !ch.is_digit(10) {
                        break;
                    }
                }
                count_deque.push_back(count);
            } else if ch.eq(&'[') {
                str_deque.push_back(str);
                str = String::new();
                index += 1;
            } else if ch.eq(&']') {
                let mut temp = str_deque.pop_back().unwrap();
                let times = count_deque.pop_back().unwrap();
                for _i in 0..times {
                    temp += str.as_str();
                }
                str = temp;
                index += 1;
            } else {
                str.push(ch);
                index += 1;
            }
        }
        str
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_394() {
        assert_eq!(
            Solution::decode_string(String::from("3[a]2[bc]")),
            String::from("aaabcbc")
        );
    }
}
