/*
*
*
*
*
*
*
*/

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        if s.len() == 0 {
            return 1;
        }
        if s[0] == 48 {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }
        let num = String::from_utf8_lossy(&s[0..=1]);
        let num = num.to_uppercase().parse::<i32>().unwrap();
        let s1 = String::from_utf8_lossy(&s[1..s.len()]);
        let s2 = String::from_utf8_lossy(&s[2..s.len()]);
        if num >= 10 && num <= 26 {
            Solution::num_decodings(s1.to_uppercase()) + Solution::num_decodings(s2.to_uppercase())
        } else {
            Solution::num_decodings(s1.to_uppercase())
        }
    }
    #[allow(dead_code)]
    pub fn num_decodings1(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        if len == 0 || s[0] == 48 {
            return 0;
        }
        let mut counts = vec![0; len + 2];
        counts[len] = 1;
        for i in (0..len).rev() {
            if i == len - 1 || s[i] == 48 {
                if s[i] != 48 {
                    counts[i] = 1;
                }
                continue;
            }
            let s = String::from_utf8_lossy(&s[i..=i + 1]);
            let num = s.to_uppercase().parse::<i32>().unwrap();
            counts[i] = counts[i + 1];
            if num >= 10 && num <= 26 {
                counts[i] += counts[i + 2];
            }
        }
        counts[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(Solution::num_decodings("32".to_owned()), 1);
        assert_eq!(Solution::num_decodings("0".to_owned()), 0);
        assert_eq!(Solution::num_decodings("10".to_owned()), 1);
        assert_eq!(Solution::num_decodings("101".to_owned()), 1);
        assert_eq!(Solution::num_decodings("226".to_owned()), 3);
        assert_eq!(Solution::num_decodings("12".to_owned()), 2);
        assert_eq!(Solution::num_decodings("012".to_owned()), 0);
        //        assert_eq!(Solution::num_decodings("1002".to_owned()), 0);
        assert_eq!(Solution::num_decodings("110".to_owned()), 1);
        assert_eq!(
            Solution::num_decodings("9371597631128776948387197132267188677349946742344217846154932859125134924241649584251978418763151253".to_owned()),
            3981312
        );
    }

    #[test]
    fn test_91_1() {
        assert_eq!(Solution::num_decodings1("32".to_owned()), 1);
        assert_eq!(Solution::num_decodings1("0".to_owned()), 0);
        assert_eq!(Solution::num_decodings1("10".to_owned()), 1);
        assert_eq!(Solution::num_decodings1("101".to_owned()), 1);
        assert_eq!(Solution::num_decodings1("226".to_owned()), 3);
        assert_eq!(Solution::num_decodings1("12".to_owned()), 2);
        assert_eq!(Solution::num_decodings1("012".to_owned()), 0);
        assert_eq!(Solution::num_decodings1("110".to_owned()), 1);
        assert_eq!(
            Solution::num_decodings1("9371597631128776948387197132267188677349946742344217846154932859125134924241649584251978418763151253".to_owned()),
            3981312
        );
    }

}
