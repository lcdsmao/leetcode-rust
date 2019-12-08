pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n <= 0 {
            return vec![];
        }
        Self::dfs(n, 0)
    }

    fn dfs(open: i32, close: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if open == 0 && close == 0 {
            res.push("".to_string());
        }
        if close > 0 {
            let v = Self::dfs(open, close - 1);
            res.extend(v.into_iter().map(|s| [")", s.as_str()].concat()));
        }
        if open > 0 {
            let v = Self::dfs(open - 1, close + 1);
            res.extend(v.into_iter().map(|s| ["(", s.as_str()].concat()));
        }
        res
    }
}
