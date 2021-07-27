pub struct Solution {}

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut max_len = 0;
        let mut map: HashMap<char, usize> = HashMap::with_capacity(s.len());
        for (i, ch) in s.chars().enumerate() {
            match map.get(&ch) {
                Some(j) if *j >= start => {
                    start = *j + 1;
                }
                _ => {
                    max_len = max(max_len, i - start + 1);
                }
            }
            map.insert(ch, i);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let len = Solution::length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(len, 3);

        let len = Solution::length_of_longest_substring(String::from("tmmzuxt"));
        assert_eq!(len, 5);
    }
}
