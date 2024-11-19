struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut profit = 0;
        for price in prices {
            if price < buy {
                buy = price;
            } else if price - buy > profit {
                profit = price - buy;
            }
        }
        profit
    }
}
