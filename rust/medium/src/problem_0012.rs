struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let roman_string = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut output = String::new();
        let mut index = 0;
        while index < values.len() {
            if num >= values[index] {
                output.push_str(roman_string[index]);
                num -= values[index];
            } else {
                index += 1;
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0012::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX".to_string())
    }
}
