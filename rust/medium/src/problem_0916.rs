struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];

        let words1_nb: Vec<u128> = words1
            .iter()
            .map(|word| {
                word.chars().fold(1, |prod, x| {
                    prod * primes[(x as u8 - b'a') as usize] as u128
                })
            })
            .collect();

        let words2_nb: Vec<u128> = words2
            .iter()
            .map(|word| {
                word.chars().fold(1, |prod, x| {
                    prod * primes[(x as u8 - b'a') as usize] as u128
                })
            })
            .collect();

        let words2_lcm = Self::lcm_of_numbers(&words2_nb);

        let mut universal_words = vec![];
        for (index, word) in words1_nb.iter().enumerate() {
            if *word % words2_lcm == 0 {
                universal_words.push(words1[index].clone());
            }
        }

        universal_words
    }

    fn gcd(a: u128, b: u128) -> u128 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    // Function to compute the LCM of two numbers
    fn lcm(a: u128, b: u128) -> u128 {
        if a == 0 || b == 0 {
            0
        } else {
            (a / Self::gcd(a, b)) * b
        }
    }

    // Function to compute the LCM of a list of numbers
    fn lcm_of_numbers(numbers: &[u128]) -> u128 {
        numbers.iter().cloned().reduce(Self::lcm).unwrap_or(0)
    }
}
