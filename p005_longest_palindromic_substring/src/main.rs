use p005_longest_palindromic_substring::Solution;

fn main() {
    let s = Solution::longest_palindrome(String::from("babad"));
    assert_eq!(s, String::from("bab"));

    let s = Solution::longest_palindrome(String::from("cbbd"));
    assert_eq!(s, String::from("bb"));
}
