use p020_valid_parentheses::Solution;

fn main() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("(]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
}
