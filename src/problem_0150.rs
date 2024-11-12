struct Solution {}

enum Token {
    Number(i32),
    Add,
    Sub,
    Mul,
    Div
}

impl Token {
    pub fn from_string(string: &str) -> Token {
        match string{
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            _ => Token::Number(string.parse::<i32>().unwrap())
        }
    }
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let tokenized: Vec<Token> = tokens.iter().map(|x| {Token::from_string(x)}).collect();
        let mut stack =vec![];

        for token in tokenized {
            match token {
                Token::Number(nb) => {
                    stack.push(nb);
                }
                Token::Add => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs+rhs);
                }
                Token::Sub => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs-rhs);
                }
                Token::Mul => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs*rhs);
                }
                Token::Div => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs/rhs);
                }
            }
        }

        *stack.get(0).unwrap()
    }


}

#[cfg(test)]
mod tests{
    use crate::problem_0150::Solution;

    #[test]
    fn test_1(){
        let ok = Solution::eval_rpn(vec!["1".to_string(),"2".to_string(),"+".to_string(),"3".to_string(),"*".to_string(),"4".to_string(),"-".to_string()]);
        assert_eq!(ok, 5)
    }

    #[test]
    fn test_2(){
        let ok = Solution::eval_rpn(vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()]);
        assert_eq!(ok, 9)
    }

    #[test]
    fn test_3(){
        let ok = Solution::eval_rpn(vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()]);
        assert_eq!(ok, 6)
    }

    #[test]
    fn test_4(){
        let ok = Solution::eval_rpn(vec!["10".to_string(),"6".to_string(),"9".to_string(),"3".to_string(),"+".to_string(),"-11".to_string(),"*".to_string(),"/".to_string(),"*".to_string(),"17".to_string(),"+".to_string(),"5".to_string(),"+".to_string()]);
        assert_eq!(ok, 22)
    }
}