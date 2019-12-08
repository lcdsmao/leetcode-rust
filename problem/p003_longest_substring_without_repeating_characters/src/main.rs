use p003_longest_substring_without_repeating_characters::Solution;

fn main() {
    let len = Solution::length_of_longest_substring(String::from("pwwkew"));
    assert_eq!(len, 3);

    let len = Solution::length_of_longest_substring(String::from("tmmzuxt"));
    assert_eq!(len, 5);
}
