use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut asteroid_stack = vec![];
        for asteroid in asteroids {
            loop {
                let last = asteroid_stack.last().unwrap_or(&asteroid);
                if *last > 0 && *last * asteroid < 0 {
                    match asteroid.abs().cmp(&last.abs()) {
                        Ordering::Less => {
                            break;
                        }
                        Ordering::Equal => {
                            asteroid_stack.pop();
                            break;
                        }
                        Ordering::Greater => {
                            asteroid_stack.pop();
                        }
                    }
                } else {
                    asteroid_stack.push(asteroid);
                    break;
                }
            }
        }

        asteroid_stack
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0735::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10])
    }
}
