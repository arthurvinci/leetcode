struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            0
        } else {
            let mut min_coins = vec![None; amount as usize + 1];
            min_coins[0] = Some(0);
            Self::intern_coin_change(&coins, amount, &mut min_coins);
            min_coins[amount as usize].unwrap_or(-1)
        }
    }

    fn intern_coin_change(coins: &[i32], amount: i32, min_coins: &mut [Option<i32>]) {
        let mut min = i32::MAX;
        for coin in coins {
            if amount >= *coin {
                let index = amount as usize - *coin as usize;
                if min_coins[index].is_none() {
                    Self::intern_coin_change(coins, amount - *coin, min_coins);
                }
                let prev_min = min_coins[index].unwrap();
                if prev_min >= 0 {
                    min = min.min(1 + min_coins[index].unwrap())
                }
            }
        }
        min_coins[amount as usize] = if min == i32::MAX { Some(-1) } else { Some(min) }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0322::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::coin_change(vec![1], 0), -1)
    }
}
