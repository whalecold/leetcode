/**
 * [338] Counting Bits
 *
 * Given a non negative integer number num. For every numbers i in the range 0 &le; i &le; num calculate the number of 1's in their binary representation and return them as an array.
 *
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">2</span>
 * Output: <span id="example-output-1">[0,1,1]</span>
 *
 * Example 2:
 *
 *
 * Input: <span id="example-input-1-1">5</span>
 * Output: [0,1,1,2,1,2]
 *
 *
 * Follow up:
 *
 *
 * 	It is very easy to come up with a solution with run time O(n*sizeof(integer)). But can you do it in linear time O(n) /possibly in a single pass?
 * 	Space complexity should be O(n).
 * 	Can you do it like a boss? Do it without using any builtin function like __builtin_popcount in c++ or in any other language.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn count_bits(num: i32) -> Vec<i32> {
        if num == 0 {
            return vec![0];
        }
        if num <= 1 {
            return vec![0, 1];
        }
        let num = num as usize;
        let mut dp = vec![0; num + 1];
        dp[1] = 1;
        let mut clean = 1;
        for i in 2..=num {
            if i == 2 * clean {
                dp[i] = 1;
                clean = i;
            } else {
                dp[i] = dp[clean] + dp[i - clean];
            }
        }
        dp
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_338() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
