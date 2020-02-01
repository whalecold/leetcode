use std::collections::HashMap;

/**
 * [13] Roman to Integer
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 *
 * For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 *
 * 	I can be placed before V (5) and X (10) to make 4 and 9.
 * 	X can be placed before L (50) and C (100) to make 40 and 90.
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 *
 * Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
 *
 * Example 1:
 *
 *
 * Input: "III"
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: "IV"
 * Output: 4
 *
 * Example 3:
 *
 *
 * Input: "IX"
 * Output: 9
 *
 * Example 4:
 *
 *
 * Input: "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 *
 *
 * Example 5:
 *
 *
 * Input: "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut numeral_map = HashMap::new();
        numeral_map.insert('I' as char, 1);
        numeral_map.insert('V' as char, 5);
        numeral_map.insert('X' as char, 10);
        numeral_map.insert('L' as char, 50);
        numeral_map.insert('C' as char, 100);
        numeral_map.insert('D' as char, 500);
        numeral_map.insert('M' as char, 1000);
        let mut ret = 0;
        for i in 0..s.len() - 1 {
            let index = numeral_map.get(&s.chars().nth(i).unwrap()).unwrap();
            let next = numeral_map.get(&s.chars().nth(i + 1).unwrap()).unwrap();
            if *index < *next {
                ret -= *index;
            } else {
                ret += *index;
            }
        }
        let i = s.chars().nth(s.len() - 1).unwrap();
        ret + numeral_map.get(&i).unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_13() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }
}
