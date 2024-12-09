use std::cmp::Ordering;

struct Solution;

// Start, value, end
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Job(i32, i32, i32);

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let sorted_jobs: Vec<Job> = Self::get_jobs(start_time, end_time, profit);

        let mut cache = vec![-1; sorted_jobs.len()];

        Self::internal_job_max(&sorted_jobs, &mut cache, 0);
        cache[0]
    }

    fn get_jobs(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> Vec<Job> {
        let mut jobs = vec![];
        for i in 0..start_time.len() {
            jobs.push(Job(start_time[i], profit[i], end_time[i]))
        }
        jobs.sort();

        jobs
    }

    fn internal_job_max(sorted_jobs: &Vec<Job>, cache: &mut Vec<i32>, index: usize) {
        if index == sorted_jobs.len() - 1 {
            cache[index] = sorted_jobs[index].1;
        } else {
            let value_to_find = sorted_jobs[index].2;
            let mut left_pointer = index;
            let mut right_pointer = sorted_jobs.len();
            while left_pointer < right_pointer {
                let mid_pointer = (left_pointer + right_pointer) / 2;
                match sorted_jobs[mid_pointer].0.cmp(&value_to_find) {
                    Ordering::Less => {
                        left_pointer = mid_pointer + 1;
                    }
                    Ordering::Equal => {
                        left_pointer = mid_pointer;
                        break;
                    }
                    Ordering::Greater => {
                        right_pointer = mid_pointer;
                    }
                }
            }

            if cache[index + 1] == -1 {
                Self::internal_job_max(sorted_jobs, cache, index + 1);
            }

            if left_pointer < sorted_jobs.len() {
                if cache[left_pointer] == -1 {
                    Self::internal_job_max(sorted_jobs, cache, left_pointer);
                }
                cache[index] = (sorted_jobs[index].1 + cache[left_pointer]).max(cache[index + 1]);
            } else {
                cache[index] = sorted_jobs[index].1.max(cache[index + 1]);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::problem_1235::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        )
    }
}
