/**
 * [67] Add Binary
 *
 * Given two binary strings, return their sum (also a binary string).
 *
 * The input strings are both non-empty and contains only characters 1 or 0.
 *
 * Example 1:
 *
 *
 * Input: a = "11", b = "1"
 * Output: "100"
 *
 * Example 2:
 *
 *
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *  
 * Constraints:
 *
 * 	Each string consists only of '0' or '1' characters.
 * 	1 <= a.length, b.length <= 10^4
 * 	Each string is either "0" or doesn't contain any leading zero.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn add_binary(a: String, b: String) -> String {
        let (mut len_a, mut len_b) = ((a.len() - 1) as i32, (b.len() - 1) as i32);
        let (mut str, mut remain) = (String::new(), 0);
        while len_a >= 0 || len_b >= 0 {
            let mut sum = remain;
            if len_a >= 0 {
                sum += a.chars().nth(len_a as usize).unwrap().to_digit(10).unwrap();
                len_a -= 1;
            }
            if len_b >= 0 {
                sum += b.chars().nth(len_b as usize).unwrap().to_digit(10).unwrap();
                len_b -= 1;
            }
            remain = sum / 2;
            let mut ch = '0';
            if sum % 2 == 1 {
                ch = '1';
            }
            str.push(ch);
        }
        if remain == 1 {
            str.push('1');
        }
        str.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_67() {
        assert_eq!(
            Solution::add_binary(String::from("11"), String::from("1")),
            String::from("100")
        );
        assert_eq!(
            Solution::add_binary(String::from("1010"), String::from("1011")),
            String::from("10101")
        );
    }
}
