struct Solution;

use std::iter;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        n == 0
            || flowerbed
                .iter()
                .chain(iter::once(&0))
                .fold((1, 0), |(consecutive_zeros, open_pots), &pot| {
                    if pot == 1 {
                        (0, open_pots)
                    } else if consecutive_zeros == 2 {
                        (1, open_pots + 1)
                    } else {
                        (consecutive_zeros + 1, open_pots)
                    }
                })
                .1
                >= n
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0605::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 1], 1))
    }
}
