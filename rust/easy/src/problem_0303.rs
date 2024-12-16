struct NumArray {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![nums[0]];
        for num in nums.iter().skip(1) {
            sums.push(*num + *sums.last().unwrap())
        }
        Self { sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = self.sums[right as usize];
        if left.is_positive() {
            sum -= self.sums[left as usize - 1];
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0303::NumArray;

    #[test]
    fn test_1() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3)
    }
}
