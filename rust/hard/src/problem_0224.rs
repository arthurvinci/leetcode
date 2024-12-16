use std::str::Chars;

struct Solution;
impl Solution {
    pub fn calculate(s: String) -> i32 {
        Self::calc(&mut s.chars())
    }

    pub fn calc(chars: &mut Chars) -> i32 {
        let mut answer = 0;
        let mut current_number = 0;
        let mut sig = 1;
        let mut has_num = false;

        loop {
            let ch = chars.next().unwrap_or(')');
            if let Some(d) = ch.to_digit(10) {
                current_number = current_number * 10 + (d as i32);
                has_num = true;
            } else {
                if has_num {
                    answer += current_number * sig;
                    sig = 1;
                    has_num = false;
                    current_number = 0;
                }
                match ch {
                    '(' => {
                        answer += Self::calc(chars) * sig;
                        sig = 1;
                    }
                    ')' => return answer,
                    '-' => sig *= -1,
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0224::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::calculate("2-1 + 2".to_string()), 3)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::calculate("-1 - -1".to_string()), 0)
    }

    #[test]
    fn test_4_5() {
        assert_eq!(Solution::calculate("1+(4+5+2)-3".to_string()), 9)
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23)
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::calculate("1-( -2)".to_string()), 3)
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::calculate(
                "1-(3+5-2+(3+19-(3-1-4+(9-4-(4-(1+(3)-2)-5)+8-(3-5)-1)-4)-5)-4+3-9)-4-(3+2-5)-10"
                    .to_string()
            ),
            -15
        )
    }
}
