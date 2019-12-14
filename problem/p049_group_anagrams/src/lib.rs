pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs {
            let mut vec_char: Vec<char> = s.chars().collect();
            vec_char.sort();
            map.entry(vec_char).or_insert(vec![]).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let mut res = Solution::group_anagrams(to_string_vec(strs));
        for v in res.iter_mut() {
            v.sort()
        }
        res.sort();
        assert_eq!(
            res,
            vec![
                to_string_vec(vec!["ate", "eat", "tea"]),
                to_string_vec(vec!["bat"]),
                to_string_vec(vec!["nat", "tan"]),
            ]
        );
    }

    fn to_string_vec(strs: Vec<&str>) -> Vec<String> {
        strs.into_iter().map(|s| s.to_string()).collect()
    }
}
