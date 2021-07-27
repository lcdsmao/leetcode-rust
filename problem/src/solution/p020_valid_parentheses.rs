pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            match ch {
                ')' | ']' | '}' => {
                    if stack.last() == Some(&ch) {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                '(' => {
                    stack.push(')');
                }
                '[' => {
                    stack.push(']');
                }
                '{' => {
                    stack.push('}');
                }
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }
}
