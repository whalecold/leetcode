/**
 * [131] Palindrome Partitioning
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome.
 *
 * Return all possible palindrome partitioning of s.
 *
 * Example:
 *
 *
 * Input: "aab"
 * Output:
 * [
 *   ["aa","b"],
 *   ["a","a","b"]
 * ]
 *
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // TODO optimize time complexity
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let (mut ret, temp) = (vec![], vec![]);
        Solution::backtrack(&s, &mut ret, 0, temp);
        ret
    }
    fn backtrack(s: &String, ret: &mut Vec<Vec<String>>, begin: usize, temp: Vec<String>) {
        if begin == s.len() {
            ret.push(temp);
            return;
        }
        let mut cur = String::new();
        let mut temp = temp;
        for i in begin..s.len() {
            cur.push(s.chars().nth(i).unwrap());
            if !Solution::is_palindrome(&cur) {
                continue;
            }
            temp.push(cur.clone());
            Solution::backtrack(s, ret, i + 1, temp.clone());
            temp.pop();
        }
    }
    fn is_palindrome(s: &String) -> bool {
        let (mut left, mut right) = (0 as usize, s.len() - 1);
        while left < right {
            if !s.as_bytes()[left].eq(&s.as_bytes()[right]) {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_131() {
        assert_eq!(
            Solution::partition(String::from("cbc")),
            vec![
                vec![String::from("c"), String::from("b"), String::from("c")],
                vec![String::from("cbc")]
            ]
        );
        assert_eq!(
            Solution::partition(String::from("bc")),
            vec![vec![String::from("b"), String::from("c")]]
        );
        assert_eq!(
            Solution::partition(String::from("aabc")),
            vec![
                vec![
                    String::from("a"),
                    String::from("a"),
                    String::from("b"),
                    String::from("c")
                ],
                vec![String::from("aa"), String::from("b"), String::from("c")],
            ]
        )
    }
}
