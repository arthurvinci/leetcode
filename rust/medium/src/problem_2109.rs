struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut new_string = String::new();
        let mut spaces_index = 0;

        for (index, char) in s.chars().enumerate() {
            if spaces_index < spaces.len() && spaces[spaces_index] == index as i32 {
                new_string.push(' ');
                spaces_index += 1;
            }

            new_string.push(char)
        }
        new_string
    }
}
