struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut short_path = vec![];
        let tokens = path.split('/');

        for word in tokens {
            match word {
                ".." => {
                    short_path.pop();
                }
                "" | "." => {}
                _ => {
                    short_path.push(word);
                }
            }
        }

        let mut final_path = short_path
            .into_iter()
            .fold(String::from('/'), |mut acc, x| {
                acc.push_str(x);
                acc.push('/');
                acc
            });

        if final_path.len() > 1 {
            final_path.pop();
        }

        final_path
    }
}
