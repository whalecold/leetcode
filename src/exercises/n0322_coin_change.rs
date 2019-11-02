/**
 * [322] Coin Change
 *
 * You are given coins of different denominations and a total amount of money amount.
 * Write a function to compute the fewest number of coins that you need to make up that amount.
 * If that amount of money cannot be made up by any combination of the coins, return -1.
 *
 * Example 1:
 *
 *
 * Input: coins = [1, 2, 5], amount = 11
 * Output: 3
 * Explanation: 11 = 5 + 5 + 1
 *
 * Example 2:
 *
 *
 * Input: coins = [2], amount = 3
 * Output: -1
 *
 *
 * Note:<br />
 * You may assume that you have an infinite number of each kind of coin.
 *
 */

#[allow(dead_code)]
static NEED_COMPILE: bool = true;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 {
            return -1;
        }
        let amount = amount as usize;
        let mut dp = vec![std::i32::MAX; amount + 1];
        dp[0] = 0;
        for i in 1..=amount {
            let mut min = std::i32::MAX;
            for coin in coins.iter() {
                let coin = coin.to_owned() as usize;
                if i >= coin && dp[i - coin] != std::i32::MAX {
                    min = i32::min(dp[i - coin] + 1, min);
                }
            }
            dp[i] = min
        }
        if dp[amount] == std::i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_322() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![474, 83, 404, 3], 264), 8);
        assert_eq!(Solution::coin_change(vec![2], 11), -1);
    }
}
