/// Say you have an array for which the ith element is the price of a given stock on day i.
//
/// Design an algorithm to find the maximum profit. You may complete as many transactions as
/// you like (ie, buy one and sell one share of the stock multiple times) with the following restrictions:
//
/// You may not engage in multiple transactions at the same time(ie, you must sell the stock before you buy again).
/// After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)
/// Example:
//
/// Input: [1,2,3,0,2]
/// Output: 3
/// Explanation: transactions = [buy, sell, cooldown, buy, sell]

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    /// 根据状态及
    #[allow(dead_code)]
    pub fn max_profit_alpha(prices: Vec<i32>) -> i32 {
        let length = prices.len();
        if length <= 1 {
            return 0;
        }
        let mut sells = vec![0; length + 1];
        let mut buys = vec![0; length + 1];
        buys[1] = -prices[0];

        for i in 2..=length {
            let price = prices[i - 1];
            buys[i] = i32::max(buys[i - 1], sells[i - 2] - price);
            sells[i] = i32::max(buys[i - 1] + price, sells[i - 1]);
        }
        sells[length]
    }

    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut sell, mut pre_sell, mut buy) = (0, 0, std::i32::MIN);

        for i in 0..prices.len() {
            let price = prices[i];
            let pre_buy = buy;
            buy = i32::max(pre_sell - price, pre_buy);
            pre_sell = sell;
            sell = i32::max(pre_buy + price, pre_sell);
        }
        sell
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_309_alpha() {
        assert_eq!(Solution::max_profit_alpha(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit_alpha(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit_alpha(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(
            Solution::max_profit_alpha(vec![1, 9, 0, 9, 1, 1, 7, 1, 1, 5, 9, 9]),
            23
        );
        assert_eq!(
            Solution::max_profit_alpha(vec![1, 9, 13, 0, 1, 9, 1, 1, 7, 1, 1, 5, 9, 9]),
            34
        );
    }
    #[test]
    pub fn test_309() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(
            Solution::max_profit(vec![1, 9, 0, 9, 1, 1, 7, 1, 1, 5, 9, 9]),
            23
        );
        assert_eq!(
            Solution::max_profit(vec![1, 9, 13, 0, 1, 9, 1, 1, 7, 1, 1, 5, 9, 9]),
            34
        );
    }
}
