struct Solution {}

pub enum Token{
    Add,
    Mul,
    Sub,
    Div,
    Number(i32)
}


impl Token{
    pub fn from(input_string: String) -> Vec<Self> {
        let mut nodes = vec![];
        let mut current_str = "".to_string();

        for char in input_string.chars() {
            match char{
                '-' => {
                    nodes.push(Token::Number(str::parse(&current_str).unwrap()));
                    nodes.push(Token::Sub);
                    current_str = "".to_string();
                }

                '+' =>{
                    nodes.push(Token::Number(str::parse(&current_str).unwrap()));
                    nodes.push(Token::Add);
                    current_str = "".to_string();
                }

                '*'=> {
                    nodes.push(Token::Number(str::parse(&current_str).unwrap()));
                    nodes.push(Token::Mul);
                    current_str = "".to_string();
                }
                '/' => {
                    nodes.push(Token::Number(str::parse(&current_str).unwrap()));
                    nodes.push(Token::Div);
                    current_str = "".to_string();
                }
                _ =>{
                    current_str.push(char);
                }
            }
        }

        if !current_str.is_empty() {
            nodes.push(Token::Number(str::parse(&current_str).unwrap()));
        }

        nodes
    }

    pub fn is_operator(&self) -> bool {
        !matches!(self, Token::Number(_))
    }

    pub fn apply_opp(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Token::Add => lhs + rhs,
            Token::Div => lhs / rhs,
            Token::Mul => lhs * rhs,
            Token::Sub => lhs - rhs,
            Token::Number(_) => panic!("This token is not an operator!")
        }
    }
}

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32>{
        let tokens = Token::from(expression);
        Self::diff_ways_to_compute_tokens(&tokens)
    }

    pub fn diff_ways_to_compute_tokens(tokens: &[Token]) -> Vec<i32> {
        if tokens.is_empty() {
            vec![0]
        } else if tokens.len() == 1 {
            match tokens.first().unwrap() {
                Token::Number(nb) => vec![*nb],
                _ => panic!("Parsing error!")
            }
        } else {
            let mut values: Vec<i32> = vec![];
            for (index, token) in tokens.iter().enumerate() {
                if token.is_operator(){
                    let left_values = Self::diff_ways_to_compute_tokens( &tokens[0..index]);
                    let right_values = Self::diff_ways_to_compute_tokens(&tokens[index+1..]);
                    for lhs in left_values {
                        for rhs in &right_values {
                            values.push(token.apply_opp(lhs, *rhs))
                        }
                    }
                }
            }

            values
        }
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0241::Solution;

    #[test]
    fn test_1() {
        let mut ok = Solution::diff_ways_to_compute("2-1-1".to_string());
        ok.sort();
        assert_eq!(ok, vec![0,2])
    }

    #[test]
    fn test_2() {
        let mut ok = Solution::diff_ways_to_compute("2*3-4*5".to_string());
        ok.sort();
        assert_eq!(ok, vec![-34,-14,-10,-10,10])
    }
}