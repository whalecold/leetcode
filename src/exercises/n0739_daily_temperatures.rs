/**
 * [739] Daily Temperatures
 *
 *
 * Given a list of daily temperatures T, return a list such that, for each day in the input, tells you how many days you would have to wait until a warmer temperature.  If there is no future day for which this is possible, put 0 instead.
 *
 * For example, given the list of temperatures T = [73, 74, 75, 71, 69, 72, 76, 73], your output should be [1, 1, 4, 2, 1, 1, 0, 0].
 *
 *
 * Note:
 * The length of temperatures will be in the range [1, 30000].
 * Each temperature will be an integer in the range [30, 100].
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut ret: Vec<i32> = vec![0; t.len()];
        for i in 0..t.len() {
            if deque.is_empty() || t[*deque.back().unwrap()] >= t[i] {
                deque.push_back(i);
                continue;
            }
            while !deque.is_empty() {
                if t[*deque.back().unwrap()] >= t[i] {
                    break;
                }
                let last = deque.pop_back().unwrap();
                ret[last] = (i - last) as i32;
            }
            deque.push_back(i);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_739() {
        assert_eq!(
            Solution::daily_temperatures(vec![75, 71, 69, 72, 76]),
            vec![4, 2, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75]),
            vec![1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
