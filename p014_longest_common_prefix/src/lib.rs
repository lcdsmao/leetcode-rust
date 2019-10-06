pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let min_len = strs.iter()
            .min_by(|x, y| x.len().cmp(&y.len()))
            .map(|s| s.len())
            .unwrap_or(0);

        let mut i = 0;
        while i < min_len && strs.iter().all(|s| s.chars().nth(i) == strs[0].chars().nth(i)) {
            i = i + 1;
        }
        strs[0][..i].to_string()
    }
}
