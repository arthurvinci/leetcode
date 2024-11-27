struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut min = 1;
        let mut max  = piles.iter().max().unwrap()+1;

        while min < max {
            let mid = (min+max)/2;
            let nb_hours = piles.iter().fold(0, |acc, x| acc + 1 + (x-1+mid)/mid);
            if nb_hours > h {
                min = mid + 1;
            }
            else{
                max = mid;
            }
        }

        min
    }
}
