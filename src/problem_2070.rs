use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut maximum_beauties = HashMap::new();
        let mut prices = HashSet::new();
        for item in items {
            let current_max = maximum_beauties.get(&item[0]).unwrap_or(&0);
            if item[1] > *current_max {
                maximum_beauties.insert(item[0], item[1]);
            }
            prices.insert(item[0]);
        }

        let sorted_price = Self::sort_array(prices.into_iter().collect());
        let sorted_queries = Self::sort_array(queries.clone());

        let mut i = 0;
        let mut max_beauty= 0;
        let mut beauty_map = HashMap::new();
        for query in sorted_queries {
            while i < sorted_price.len() && sorted_price[i] <= query{
                let beauty = *maximum_beauties.get(&sorted_price[i]).unwrap();
                if beauty > max_beauty{
                    max_beauty = beauty
                }
                i += 1;
            }

            beauty_map.insert(query, max_beauty);
        }

        queries.iter().map(|query| {
            *beauty_map.get(query).unwrap()
        }).collect()
    }

    fn sort_array(items: Vec<i32>) -> Vec<i32> {
        if items.len() == 0 {
            return vec![]
        }

        if items.len() == 1 {
            return items
        }

        let size = items.len() / 2;
        Self::merge_arrays(&Self::sort_array(items[0..size].to_vec()), &Self::sort_array(items[size..].to_vec()))

    }

    fn merge_arrays(array_1: &Vec<i32>, array_2: &Vec<i32>) ->  Vec<i32> {
        let mut i = 0;
        let mut j = 0;
        let mut ret = vec![];

        while i < array_1.len() && j < array_2.len(){
            let elem_1 = array_1[i].clone();
            let elem_2 = array_2[j].clone();
            if elem_1 < elem_2 {
                ret.push(elem_1);
                i += 1;
            }
            else {
                ret.push(elem_2);
                j += 1;
            }
        }

        while i < array_1.len() {
            ret.push(array_1[i].clone());
            i += 1;
        }

        while j < array_2.len() {
            ret.push(array_2[j].clone());
            j += 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests{
    use crate::problem_2070::Solution;

    #[test]
    fn test_1(){
        let ok = Solution::maximum_beauty(vec![vec![1,2],vec![3,2],vec![2,4],vec![5,6],vec![3,5]], vec![1,2,3,4,5,6]);
        assert_eq!(ok, vec![2,4,5,5,6,6])
    }
}

