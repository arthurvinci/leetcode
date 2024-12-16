use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;

        while i + j + 1 > 0 && i >= 0 && j >= 0 {
            let pointer = i as usize + j as usize + 1;
            match nums1[i as usize].cmp(&nums2[j as usize]) {
                Ordering::Less | Ordering::Equal => {
                    nums1[pointer] = nums2[j as usize];
                    j -= 1;
                }
                Ordering::Greater => {
                    nums1[pointer] = nums1[i as usize];
                    i -= 1;
                }
            }
        }

        while j >= 0 {
            nums1[j as usize] = nums2[j as usize];
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0088::Solution;

    #[test]
    fn test_1() {
        let mut nums1 = vec![0];
        Solution::merge(&mut nums1, 0, &mut vec![1], 1);
        assert_eq!(nums1, vec![1])
    }
}
