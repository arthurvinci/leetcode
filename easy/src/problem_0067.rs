struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (lena, lenb) = (a.len(), b.len());
        let maxlen = lena.max(lenb);
        let mut ans = String::new();
        let mut carry = false;

        // Pad the shorter string with '0's
        let a = format!("{:0>width$}", a, width = maxlen);
        let b = format!("{:0>width$}", b, width = maxlen);

        for i in (0..maxlen).rev() {
            let (c1, c2) = (a.as_bytes()[i] as char, b.as_bytes()[i] as char);
            match (c1, c2, carry) {
                ('1', '1', false) | ('1', '0', true) | ('0', '1', true) => {
                    ans.push('0');
                    carry = true;
                }
                ('1', '0', false) | ('0', '1', false) | ('0', '0', true) => {
                    ans.push('1');
                    carry = false;
                }
                ('0', '0', false) => {
                    ans.push('0');
                }
                ('1', '1', true) => {
                    ans.push('1');
                }
                _ => unreachable!(),
            }
        }
        if carry {
            ans.push('1');
        }
        ans.chars().rev().collect()
    }
}
