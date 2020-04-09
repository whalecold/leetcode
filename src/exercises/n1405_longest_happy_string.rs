/**
 * [1405] Longest Happy String
 *
 * A string is called happy if it does not have any of the strings 'aaa', 'bbb' or 'ccc' as a substring.
 * Given three integers a, b and c, return any string s, which satisfies following conditions:
 *
 * 	s is happy and longest possible.
 * 	s contains at most a occurrences of the letter 'a', at most b occurrences of the letter 'b' and at most c occurrences of the letter 'c'.
 * 	s will only contain 'a', 'b' and 'c' letters.
 *
 * If there is no such string s return the empty string "".
 *  
 * Example 1:
 *
 * Input: a = 1, b = 1, c = 7
 * Output: "ccaccbcc"
 * Explanation: "ccbccacc" would also be a correct answer.
 *
 * Example 2:
 *
 * Input: a = 2, b = 2, c = 1
 * Output: "aabbc"
 *
 * Example 3:
 *
 * Input: a = 7, b = 1, c = 0
 * Output: "aabaa"
 * Explanation: It's the only correct answer in this case.
 *
 *  
 * Constraints:
 *
 * 	0 <= a, b, c <= 100
 * 	a + b + c > 0
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    // TODO
    // Ref: https://leetcode.com/problems/longest-happy-string/discuss/564277/C%2B%2BJava-a-greater-b-greater-c
    #[allow(dead_code)]
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut ret = String::new();
        let (mut t_a, mut t_b, mut t_c, sum) = (0, 0, 0, a + b + c);
        let (mut a, mut b, mut c) = (a, b, c);
        for _i in 0..sum {
            if (a >= b && a >= c && t_a != 2) || (t_b == 2 && a > 0) || (t_c == 2 && a > 0) {
                ret.push('a');
                t_a += 1;
                a -= 1;
                t_b = 0;
                t_c = 0;
            } else if (b >= a && b >= c && t_b != 2) || (t_a == 2 && b > 0) || (t_c == 2 && b > 0) {
                ret.push('b');
                t_b += 1;
                b -= 1;
                t_a = 0;
                t_c = 0;
            } else if (c >= a && c >= b && t_c != 2) || (t_a == 2 && c > 0) || (t_b == 2 && c > 0) {
                ret.push('c');
                t_c += 1;
                c -= 1;
                t_a = 0;
                t_b = 0;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1405() {
        assert_eq!(
            Solution::longest_diverse_string(1, 1, 7),
            String::from("ccaccbcc")
        );
        assert_eq!(
            Solution::longest_diverse_string(2, 2, 1),
            String::from("ababc")
        );
        assert_eq!(
            Solution::longest_diverse_string(7, 1, 0),
            String::from("aabaa")
        );
    }
}
