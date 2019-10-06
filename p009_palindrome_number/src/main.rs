use p009_palindrome_number::Solution;

fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
}
