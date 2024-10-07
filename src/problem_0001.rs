use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i  in 0..nums.len(){
            map.insert(*nums.get(i).unwrap(), i);
        }

        for i in 0..nums.len(){
            let i_value = *nums.get(i).unwrap();
            match map.get(&(target-i_value)){
                None => {}
                Some(index) => {
                    if *index != i{
                        return vec![i as i32, *index as i32]
                    }
                }
            }
        }

        panic!()
    }
}

#[cfg(test)]
mod test{
    use crate::problem_0001::Solution;

    #[test]
    fn test_2(){
        let res = Solution::two_sum(vec![3,2,4], 6);
        assert_eq!(res, vec![1,2])
    }

}