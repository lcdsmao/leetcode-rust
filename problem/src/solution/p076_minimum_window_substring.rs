use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }

        let mut t_map = t.chars().fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        });

        let s_vec: Vec<char> = s.chars().collect();
        let (mut slow, mut fast) = (0, 0);
        let (mut min_slow, mut min_fast) = (0, s.len());
        let mut valid_count = 0;

        while fast < s.len() {
            t_map.entry(s_vec[fast]).and_modify(|e| {
                if *e > 0 {
                    valid_count += 1;
                }
                *e -= 1;
            });
            while valid_count == t.len() {
                if min_fast - min_slow > fast - slow {
                    min_fast = fast;
                    min_slow = slow;
                }

                if let Some(e) = t_map.get_mut(&s_vec[slow]) {
                    *e += 1;
                    if *e > 0 {
                        valid_count -= 1;
                    }
                }
                slow += 1;
            }
            fast += 1;
        }

        if min_fast >= s.len() {
            "".to_string()
        } else {
            s_vec[min_slow..=min_fast].into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string(),
        );

        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string(),
        );

        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string(),
        );
    }
}
