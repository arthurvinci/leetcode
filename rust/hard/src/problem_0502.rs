use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn find_maximized_capital(mut k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut cap_profits: Vec<(i32, i32)> =
            capital.into_iter().zip(profits.into_iter()).collect();
        cap_profits.sort();

        let mut current_capital = w;
        let mut cursor = 0;
        let mut heap = BinaryHeap::new();
        while k > 0 {
            while cursor < cap_profits.len() && cap_profits[cursor].0 <= current_capital {
                heap.push((cap_profits[cursor].1, cap_profits[cursor].0));
                cursor += 1;
            }

            if let Some((profit, min_cap)) = heap.pop() {
                if min_cap <= current_capital {
                    current_capital += profit;
                    k -= 1
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        current_capital
    }
}
