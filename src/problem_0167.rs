struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut answer = vec![];
        for (index, number) in numbers.iter().enumerate() {
            match Self::is_value_in_sorted_array(target - *number, &numbers[index+1..]) {
                None => {}
                Some(ind) => {
                    answer =  vec![1 + index as i32, 2 + index as i32 + ind as i32];
                    break;
                }
            }
        }
        answer
    }

    pub fn is_value_in_sorted_array(value: i32, array: &[i32]) -> Option<usize> {
        let mut left_index = 0usize;
        let mut right_index = array.len()-1;

        while left_index < right_index {
            let middle_index = (left_index+right_index+1)/2;

            if array[middle_index] > value{
                if middle_index == right_index{
                    break;
                }
                right_index = middle_index;
            }
            else{
                if middle_index == left_index{
                    break;
                }
                left_index = middle_index;
            }
        }

        if array[left_index] == value {
            Some(left_index)
        }
        else{
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0167::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::two_sum(vec![2,7,11,15], 9),
            vec![1,2]
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::two_sum(vec![2,3,4], 6),
            vec![1,3]
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::two_sum(vec![-1,0], -1),
            vec![1,2]
        )
    }

}
