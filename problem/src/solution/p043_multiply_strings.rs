pub struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut res = vec![0; num1.len() + num2.len()];
        for (i1, c1) in num1.chars().rev().enumerate() {
            for (i2, c2) in num2.chars().rev().enumerate() {
                let n1 = c1.to_digit(10).unwrap();
                let n2 = c2.to_digit(10).unwrap();
                res[i1 + i2] += n1 * n2;
                res[i1 + i2 + 1] += res[i1 + i2] / 10;
                res[i1 + i2] %= 10;
            }
        }
        while res.len() > 1 && Some(&0) == res.last() {
            res.pop();
        }
        res.into_iter().map(|n| n.to_string()).rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}
