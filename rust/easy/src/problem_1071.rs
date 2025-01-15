struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (mut s1, mut s2) = (str1.clone(), str2.clone());
        s1 += &str2;
        s2 += &str1;
        if s1 != s2 {
            return "".to_string();
        }
        let n = Self::gcd(str1.len(), str2.len());
        str1.get(0..n).unwrap().to_string()
    }
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
