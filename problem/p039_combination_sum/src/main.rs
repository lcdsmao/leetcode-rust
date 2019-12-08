use p039_combination_sum::Solution;

fn main() {
    let candidates = vec![2, 3, 6, 7];
    let ans = vec![vec![2, 2, 3], vec![7]];
    assert_eq!(Solution::combination_sum(candidates, 7), ans);
}
