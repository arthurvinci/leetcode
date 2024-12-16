struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut subsets = vec![vec![]];
        let mut index = 0;
        let mut last_value = -50;
        let mut last_start_index = 0;
        while index < nums.len() {
            let mut start_index = last_start_index;
            last_start_index = subsets.len();

            if nums[index] != last_value {
                start_index = 0;
            }

            last_value = nums[index];
            let mut new_subsets = vec![];
            for subset in subsets.iter().skip(start_index) {
                let mut new_subst = subset.clone();
                new_subst.push(last_value);
                new_subsets.push(new_subst);
            }
            subsets.append(&mut new_subsets);

            index += 1;
        }

        subsets
    }
}
