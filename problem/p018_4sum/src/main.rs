use p018_4sum::Solution;

fn main() {
    let res = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
    let mut ans = vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]];
    ans.sort();
    assert_eq!(res, ans);
}
