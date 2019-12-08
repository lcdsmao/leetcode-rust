use p022_generate_parentheses::Solution;

fn main() {
    let mut res = Solution::generate_parenthesis(3);
    let mut ans = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
    res.sort();
    ans.sort();
    assert_eq!(res, ans);
}
