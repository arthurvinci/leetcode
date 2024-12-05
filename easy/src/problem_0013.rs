struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let mut prev_char = 'O';

        for char in s.chars() {
            match char {
                'I' => total += 1,
                'V' => {
                    if prev_char == 'I' {
                        total += 3;
                    } else {
                        total += 5;
                    }
                }
                'X' => {
                    if prev_char == 'I' {
                        total += 8;
                    } else {
                        total += 10;
                    }
                }
                'L' => {
                    if prev_char == 'X' {
                        total += 30;
                    } else {
                        total += 50;
                    }
                }
                'C' => {
                    if prev_char == 'X' {
                        total += 80;
                    } else {
                        total += 100;
                    }
                }
                'D' => {
                    if prev_char == 'C' {
                        total += 300;
                    } else {
                        total += 500;
                    }
                }
                'M' => {
                    if prev_char == 'C' {
                        total += 800;
                    } else {
                        total += 1000
                    }
                }
                _ => unreachable!(),
            }
            prev_char = char;
        }

        total
    }
}
