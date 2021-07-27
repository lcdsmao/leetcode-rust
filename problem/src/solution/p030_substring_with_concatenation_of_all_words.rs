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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()],
        );
        assert_eq!(res, vec![0, 9]);
    }
}
