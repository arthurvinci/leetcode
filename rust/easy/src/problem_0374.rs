struct Solution;

impl Solution {
    fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left < right {
            let pick = (right + left) / 2;
            match Self::guess(pick) {
                -1 => right = pick - 1,
                0 => return pick,
                1 => left = pick + 1,
                _ => unreachable!(),
            }
        }
        left
    }

    fn guess(n: i32) -> i32 {
        -1
    }
}
