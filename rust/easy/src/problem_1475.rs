struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut final_prices = prices.clone();
        let mut waiting_prices = vec![];

        for (index, price) in prices.into_iter().enumerate() {
            while !waiting_prices.is_empty() {
                let (prev_index, prev_price) = waiting_prices.pop().unwrap();
                if prev_price >= price {
                    final_prices[prev_index] -= price;
                } else {
                    waiting_prices.push((prev_index, prev_price));
                    break;
                }
            }

            waiting_prices.push((index, price));
        }

        final_prices
    }
}
