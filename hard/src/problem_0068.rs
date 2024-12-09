use std::iter::repeat;

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut index = 0;
        let mut justified_text = vec![];
        while index < words.len() {
            let mut new_line = vec![];
            let mut words_length = 0;
            while index < words.len() && words_length + words[index].len() as i32 <= max_width {
                new_line.push(words[index].clone());
                words_length += words[index].len() as i32 + 1;
                index += 1;
            }
            justified_text.push(Self::justified_line(
                new_line,
                max_width,
                words_length,
                index >= words.len(),
            ))
        }
        justified_text
    }

    fn justified_line(
        words: Vec<String>,
        max_width: i32,
        mut words_length: i32,
        is_last_line: bool,
    ) -> String {
        let mut new_words: Vec<String> = vec![];

        words[..words.len() - 1].into_iter().for_each(|word| {
            let mut new_word = word.clone();
            new_word.push(' ');
            new_words.push(new_word)
        });

        words_length -= 1;

        if is_last_line || words.len() == 1 {
            new_words.push(words.last().unwrap().clone());

            let filled = repeat(' ')
                .take((max_width - words_length) as usize)
                .collect::<String>();
            new_words.push(filled);
        } else {
            let average_spaces = (max_width as usize - words_length as usize) / (words.len() - 1);
            let mut remaining_space =
                (max_width as usize - words_length as usize) % (words.len() - 1);

            for word in &mut new_words {
                let mut spaces_amount = average_spaces;

                if remaining_space > 0 {
                    spaces_amount += 1;
                    remaining_space -= 1;
                }

                let spaces: String = repeat(' ').take(spaces_amount).collect();
                word.push_str(&spaces)
            }

            new_words.push(words.last().unwrap().clone());
        }

        new_words.concat()
    }
}
