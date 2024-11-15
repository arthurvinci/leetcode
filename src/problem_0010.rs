struct Solution;

#[derive(Clone)]
enum Token{
    AnyCharacter,
    ZeroOrMore(Box<Token>),
    Character(char),
}

impl Token{
    pub fn match_internal_char(&self, ch: char) -> bool {
        match self{
            Token::AnyCharacter => true,
            Token::ZeroOrMore(tk) => {
                tk.match_internal_char(ch)
            }
            Token::Character(ch_p) => *ch_p == ch
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let tokens = Self::parse_pattern(p);
        Self::internal_is_match(s.chars().collect(), tokens)
    }

    fn internal_is_match(mut s: Vec<char>, mut tokens: Vec<Token>) -> bool {
        let last_char = s.pop();
        let last_token = tokens.pop();
        match (last_char, last_token) {
            (None, None) => true,
            (None, Some(token)) => {
                match token {
                    Token::ZeroOrMore(_) => Self::internal_is_match(s, tokens),
                    _ => false
                }
            }
            (Some(_), None) => false,
                (Some(ch), Some(token)) => {
                match token{
                    Token::AnyCharacter => {
                        Self::internal_is_match(s, tokens)
                    }
                    Token::ZeroOrMore(ch_p) => {
                        if ch_p.match_internal_char(ch){
                            let continue_without = Self::internal_is_match(s.clone(), tokens.clone());
                            if !continue_without{
                                let mut new_s = s.clone();
                                new_s.push(ch);

                                let zero_char = Self::internal_is_match(new_s, tokens.clone());

                                if !zero_char{
                                    tokens.push(Token::ZeroOrMore(ch_p));
                                    let last_try = Self::internal_is_match(s, tokens);
                                    last_try
                                }
                                else{
                                    true
                                }
                            }
                            else {
                                true
                            }
                        }
                        else{
                            s.push(ch);
                            Self::internal_is_match(s, tokens)
                        }
                    }
                    Token::Character(ch_p) => {
                        if ch_p != ch{
                            false
                        }
                        else {
                            Self::internal_is_match(s, tokens)
                        }
                    }
                }

            }
        }
    }

    pub fn parse_pattern(p: String) -> Vec<Token>{
        let mut tokens = vec![];
        for ch in p.chars() {
            match ch {
                '.' => tokens.push(Token::AnyCharacter),
                '*' => {
                    let prev_token = tokens.pop().unwrap();
                    tokens.push(Token::ZeroOrMore(Box::new(prev_token)));
                }
                _ => {
                    tokens.push(Token::Character(ch))
                }
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests{
    use crate::problem_0010::Solution;

    #[test]
    fn test_1(){
        let ok = Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string());
        assert!(ok);
    }

    #[test]
    fn test_2(){
        let ok = Solution::is_match("a".to_string(), "ab*a".to_string());
        assert!(!ok);
    }

    #[test]
    fn test_3(){
        let ok = Solution::is_match("aa".to_string(), "ab*a*".to_string());
        assert!(ok);
    }

    #[test]
    fn test_4(){
        let ok = Solution::is_match("a".to_string(), "aa*".to_string());
        assert!(ok);
    }

    #[test]
    fn test_5(){
        let ok = Solution::is_match("aaa".to_string(), "ab*a".to_string());
        assert!(!ok);
    }

    #[test]
    fn test_6(){
        let ok = Solution::is_match("aaa".to_string(), ".*".to_string());
        assert!(ok);
    }
}