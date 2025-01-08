struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut presum = vec![0; s.len() + 1];

        for shift in shifts {
            let mut add = 1;
            if shift[2] == 0 {
                add = -1;
            }
            presum[shift[0] as usize] += add;
            presum[shift[1] as usize + 1] -= add;
        }

        let mut new_s: Vec<char> = s.chars().collect();
        let mut shift = 0;

        for i in 0..s.len() {
            shift += presum[i];
            let mut offset = ((new_s[i] as u8 - b'a') as i32 + shift) % 26;
            if offset < 0 {
                offset += 26;
            }
            new_s[i] = (b'a' + offset as u8) as char;
        }

        new_s.iter().collect()
    }
}
