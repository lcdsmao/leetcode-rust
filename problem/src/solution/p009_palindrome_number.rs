pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || x % 10 == 0 && x != 0 {
            return false;
        }

        let mut reverse_num = 0;
        let mut num = x;
        while num > reverse_num {
            reverse_num = reverse_num * 10 + num % 10;
            num = num / 10;
        }

        num == reverse_num || num == reverse_num / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
    }
}
