struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut nb_chunks = 0;
        let mut max_in_stack = 0;
        let mut nb_in_stack = 0;
        for num in arr {
            max_in_stack = max_in_stack.max(num);
            nb_in_stack += 1;

            if nb_in_stack == max_in_stack + 1 {
                nb_chunks += 1;
            }
        }

        nb_chunks
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0769::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![2, 0, 1]), 1);
    }
}
