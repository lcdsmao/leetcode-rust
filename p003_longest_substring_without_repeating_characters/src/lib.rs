pub struct Solution {}

use std::collections::HashMap;
use std::cmp::max;

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