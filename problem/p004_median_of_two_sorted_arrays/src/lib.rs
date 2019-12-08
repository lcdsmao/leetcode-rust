pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len() + nums2.len();
        if n % 2 == 0 {
            let ans1 = Self::find_k_th_max(&nums1, 0, &nums2, 0, n / 2);
            let ans2 = Self::find_k_th_max(&nums1, 0, &nums2, 0, n / 2 + 1);
            (ans1 as f64 + ans2 as f64) / 2.0
        } else {
            Self::find_k_th_max(&nums1, 0, &nums2, 0, n / 2 + 1) as f64
        }
    }

    fn find_k_th_max(
        num1: &Vec<i32>,
        start1: usize,
        num2: &Vec<i32>,
        start2: usize,
        k: usize,
    ) -> i32 {
        if start1 >= num1.len() {
            return num2[start2 + k - 1];
        }
        if start2 >= num2.len() {
            return num1[start1 + k - 1];
        }

        if k == 1 {
            return num1[start1].min(num2[start2]);
        }

        let max1 = num1.get(start1 + k / 2 - 1).unwrap_or(&i32::max_value());
        let max2 = num2.get(start2 + k / 2 - 1).unwrap_or(&i32::max_value());

        if max1 < max2 {
            Self::find_k_th_max(num1, start1 + k / 2, num2, start2, k - k / 2)
        } else {
            Self::find_k_th_max(num1, start1, num2, start2 + k / 2, k - k / 2)
        }
    }
}
