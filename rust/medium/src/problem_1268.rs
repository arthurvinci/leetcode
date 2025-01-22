struct Solution;

impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut result = Vec::with_capacity(search_word.len());
        products.sort_unstable();

        for prefix_len in 1..=search_word.len() {
            let prefix = &search_word[..prefix_len];
            let index = products.partition_point(|product| product.as_str() < prefix);

            let suggestions = products[index..]
                .iter()
                .take(3)
                .take_while(|product| product.starts_with(prefix))
                .cloned()
                .collect::<Vec<String>>();

            result.push(suggestions);
        }

        result
    }
}
