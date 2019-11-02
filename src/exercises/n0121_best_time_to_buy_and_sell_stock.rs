/// Say you have an array for which the ith element is the price of a given stock on day i.
/// If you were only permitted to complete at most one transaction
/// (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.
/// Note that you cannot sell a stock before you buy one.
/// ## Example 1:
/// Input: [7,1,5,3,6,4]
/// Output: 5
/// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//             Not 7-1 = 6, as selling price needs to be larger than buying price.

#[allow(dead_code)]
static NEED_COMPILE: bool = false;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let (mut buy, mut sell) = (0, 0);
        for i in 0..prices.len() {
            if prices[i] > prices[sell] {
                sell = i;
            }
            if prices[i] < prices[buy] {
                sell = i;
                buy = i;
            }
            if prices[sell] - prices[buy] > max_profit {
                max_profit = prices[sell] - prices[buy];
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![0, 1, 0, 3, 11]), 11);
        assert_eq!(Solution::max_profit(vec![1, 5, 0, 3, 11]), 11);
    }
}
