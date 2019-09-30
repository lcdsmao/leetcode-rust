use p004_median_of_two_sorted_arrays::Solution;

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let ans = Solution::find_median_sorted_arrays(nums1, nums2);
    assert_eq!(ans, 2.0);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let ans = Solution::find_median_sorted_arrays(nums1, nums2);
    assert_eq!(ans, 2.5);
}
