struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = candies.iter().max().unwrap().clone();
        candies
            .into_iter()
            .map(|candy_amount| candy_amount + extra_candies >= max_candies)
            .collect()
    }
}
