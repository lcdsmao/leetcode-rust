pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let mut left = 0usize;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return true;
            }
            if nums[left] < nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else if nums[mid] < nums[left] {
                if nums[mid] < target && target < nums[left] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                left = left + 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
        assert_eq!(Solution::search(vec![1], 1), true);
        assert_eq!(
            Solution::search(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
                2
            ),
            true
        );
    }
}
