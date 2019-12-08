use p001_two_sum::Solution;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ans = Solution::two_sum(nums, target);
    assert_eq!(ans, vec![0, 1]);
}
