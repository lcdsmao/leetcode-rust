use p032_longest_valid_parentheses::Solution;

fn main() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
}
