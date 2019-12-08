use p041_first_missing_positive::Solution;

fn main() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}
