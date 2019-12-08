pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = if let Some(l) = words.first() {
            l.len()
        } else {
            return vec![];
        };
        let all_len = word_len * words.len();
        let word_map = Self::convert_to_hash_map(words);
        s.as_bytes()
            .windows(all_len)
            .map(|w| {
                let ws: Vec<String> = w
                    .chunks(word_len)
                    .map(|s| String::from_utf8_lossy(s).into())
                    .collect();
                Self::convert_to_hash_map(ws)
            })
            .enumerate()
            .filter_map(|(i, m)| if m == word_map { Some(i as i32) } else { None })
            .collect()
    }

    fn convert_to_hash_map(strs: Vec<String>) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for s in strs {
            *map.entry(s).or_insert(0) += 1;
        }
        map
    }
}
