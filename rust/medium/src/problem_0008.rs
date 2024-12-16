struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut number: i32 = 0;
        let mut positive = true;
        let mut start_reading = false;

        for char in s.chars() {
            match char {
                '+' => {
                    if start_reading {
                        break;
                    } else {
                        positive = true;
                        start_reading = true;
                    }
                }
                '-' => {
                    if start_reading {
                        break;
                    } else {
                        positive = false;
                        start_reading = true;
                    }
                }
                ' ' => {
                    if start_reading {
                        break;
                    }
                }
                _ => {
                    start_reading = true;
                    match char.to_digit(10) {
                        None => break,
                        Some(digit) => {
                            if positive {
                                let new_number = (number as u64) * 10 + digit as u64;
                                number = new_number.min(i32::MAX as u64) as i32
                            } else {
                                let new_number = (number as i64) * 10 - digit as i64;
                                number = new_number.max(i32::MIN as i64) as i32;
                            }
                        }
                    }
                }
            }
        }
        number
    }
}
