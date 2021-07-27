pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n {
            0 => 1_f64,
            1 => x,
            -1 => 1_f64 / x,
            _ => {
                let half = Self::my_pow(x, n / 2);
                let remainder = n % 2;
                half * half * Self::my_pow(x, remainder)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_pow(2.00000_f64, 10), 2.00000_f64.powi(10));
        assert_eq!(Solution::my_pow(2.10000_f64, 3), 2.10000_f64.powi(3));
        assert_eq!(Solution::my_pow(2.00000_f64, -2), 2.00000_f64.powi(-2));
    }
}
