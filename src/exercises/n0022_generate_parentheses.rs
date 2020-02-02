/**
 * [22] Generate Parentheses
 *
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 *
 *
 * For example, given n = 3, a solution set is:
 *
 *
 * [
 *   "((()))",
 *   "(()())",
 *   "(())()",
 *   "()(())",
 *   "()()()"
 * ]
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    fn dfs(ret: &mut Vec<String>, mut cur: String, open: i32, close: i32, max: i32) {
        if cur.len() as i32 == max * 2 {
            ret.push(cur);
            return;
        }
        if open < max {
            let mut temp = cur.clone();
            temp.push('(');
            Solution::dfs(ret, temp, open + 1, close, max);
        }
        if close < open {
            cur.push(')');
            Solution::dfs(ret, cur, open, close + 1, max);
        }
    }
    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = vec![];
        Solution::dfs(&mut ret, String::new(), 0, 0, n);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()")
            ]
        );
    }
}
