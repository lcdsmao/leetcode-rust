pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        if digits.is_empty() {
            return vec![];
        }
        let n = digits.len();
        let mut result = vec![0; n + 1];
        let mut acc = 1;
        for i in (0..n).rev() {
            let v = digits[i] + acc;
            result[i + 1] = if v == 10 {
                acc = 1;
                0
            } else {
                acc = 0;
                v
            }
        }

        if acc == 1 {
            result[0] = 1;
            result
        } else {
            result.into_iter().skip(1).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4],);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2],);
        assert_eq!(Solution::plus_one(vec![0]), vec![1],);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 9]), vec![4, 3, 3, 0],);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0],);
    }
}
