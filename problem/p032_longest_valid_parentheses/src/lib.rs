pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_len: i32 = 0;
        let mut stack: Vec<i32> = vec![];
        stack.push(-1);
        for (i, ch) in s.chars().enumerate() {
            let i = i as i32;
            if ch == '(' {
                stack.push(i);
            } else {
                stack.pop();
                if let Some(&j) = stack.last() {
                    max_len = max_len.max(i - j);
                } else {
                    stack.push(i);
                }
            }
        }
        max_len
    }
}
