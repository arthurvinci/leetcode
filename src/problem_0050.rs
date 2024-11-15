struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Self::intern_pow(x, n as i64)
    }

    fn intern_pow(x: f64, n: i64) -> f64{
        if n == 0{
            return 1.0
        }

        if n == 1{
            return x
        }

        if n < 0 {
            let inverse = Self::intern_pow(x, -n);
            return 1.0/inverse;
        }

        let mut ret = Self::intern_pow(x, n/2);
        ret = ret*ret;

        if n%2 == 1{
            ret = ret * x;
        }

        ret
    }
}

#[cfg(test)]
mod tests{
    use crate::problem_0050::Solution;

    #[test]
    fn test_1(){
        let res = Solution::my_pow(1.0, -2147483648);
        assert_eq!(res, 1.0);
    }
}