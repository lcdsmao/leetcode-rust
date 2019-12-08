use p040_combination_sum_ii::Solution;

fn main() {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let res = vec![
        vec![1, 1, 6],
        vec![1, 2, 5],
        vec![1, 7],
        vec![2, 6],
    ];
    assert_eq!(Solution::combination_sum2(candidates, 8), res);
}
