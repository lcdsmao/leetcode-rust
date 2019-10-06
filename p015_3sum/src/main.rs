use p015_3sum::Solution;

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let ans = Solution::three_sum(nums);
    assert_eq!(ans, vec![
        vec![-1, -1, 2],
        vec![-1, 0, 1],
    ]);
}
