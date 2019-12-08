use p030_substring_with_concatenation_of_all_words::Solution;

fn main() {
    let res = Solution::find_substring(
        "barfoothefoobarman".to_string(),
        vec!["foo".to_string(), "bar".to_string()],
    );
    assert_eq!(res, vec![0, 9]);
}
