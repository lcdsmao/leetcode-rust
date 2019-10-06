use p016_3sum_cloest::Solution;

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let ans = Solution::three_sum_closest(nums, 1);
    assert_eq!(ans, 2);
}
