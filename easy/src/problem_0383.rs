use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_letters = HashMap::new();
        for char in magazine.chars() {
            match magazine_letters.get_mut(&char) {
                None => {
                    magazine_letters.insert(char, 1);
                }
                Some(amount) => {
                    *amount += 1;
                }
            }
        }

        for char in ransom_note.chars() {
            match magazine_letters.get_mut(&char) {
                None => return false,
                Some(amount) => {
                    if *amount > 0 {
                        *amount -= 1
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}
