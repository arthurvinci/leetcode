struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let (mut map1, mut map2) = ([0; 26], [0; 26]);

        for char in word1.chars() {
            map1[char as usize - b'a' as usize] += 1;
        }

        for char in word2.chars() {
            map2[char as usize - b'a' as usize] += 1;
        }

        for i in 0..26 {
            if (map1[i] == 0 && map2[i] != 0) || (map1[i] != 0 && map2[i] == 0) {
                return false;
            }
        }

        map1.sort();
        map2.sort();

        map1 == map2
    }
}
