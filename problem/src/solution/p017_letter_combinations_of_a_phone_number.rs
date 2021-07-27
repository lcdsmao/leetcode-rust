pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut phone_map: HashMap<u32, &str> = HashMap::new();
        phone_map.insert(2, "abc");
        phone_map.insert(3, "def");
        phone_map.insert(4, "ghi");
        phone_map.insert(5, "jkl");
        phone_map.insert(6, "mno");
        phone_map.insert(7, "pqrs");
        phone_map.insert(8, "tuv");
        phone_map.insert(9, "wxyz");

        Self::dfs(&phone_map, digits.as_str())
    }

    fn dfs(phone_map: &HashMap<u32, &str>, digits: &str) -> Vec<String> {
        let mut res = Vec::new();
        if let Some(head) = digits.chars().next() {
            let chars = head
                .to_digit(10)
                .and_then(|num| phone_map.get(&num))
                .unwrap();

            for c in chars.chars() {
                let mut r = Self::dfs(phone_map, &digits[1..]);
                if r.is_empty() {
                    r.push("".to_string());
                }
                r.iter_mut().for_each(|s| s.insert(0, c));
                res.append(&mut r);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::letter_combinations("23".to_string());
        assert_eq!(
            res,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );

        let res = Solution::letter_combinations("".to_string());
        assert_eq!(res, Vec::<String>::new());
    }
}
