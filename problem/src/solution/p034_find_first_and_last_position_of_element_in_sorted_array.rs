pub struct Solution {}

use std::borrow::Borrow;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = Self::binary_search(nums.borrow(), target, true);
        if left == -1 {
            return vec![-1, -1];
        }

        let right = Self::binary_search(nums.borrow(), target, false);
        vec![left, right]
    }

    fn binary_search(nums: &Vec<i32>, target: i32, left_most: bool) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        let factor = if left_most { 0 } else { 1 };
        while l < r {
            let mid = l + (r - l + factor) / 2;
            if nums[mid] < target {
                l = mid + 1 - factor;
            } else if nums[mid] > target {
                r = mid - factor;
            } else if nums[mid] == target {
                if left_most {
                    r = mid;
                } else {
                    l = mid;
                }
            }
        }
        if left_most && nums[l] == target {
            l as i32
        } else if !left_most && nums[r] == target {
            r as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );

        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }
}
