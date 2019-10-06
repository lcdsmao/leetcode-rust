use p010_regular_expression_matching::Solution;

fn main() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);

    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
}
