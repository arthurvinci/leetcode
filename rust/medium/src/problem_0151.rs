struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ret: String = s
            .split(' ')
            .rev()
            .filter(|word| *word != "")
            .map(|word| format!("{} ", word))
            .collect();
        ret.pop();
        ret
    }
}
