pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut permutations = vec![];
        Self::backtrace(&mut nums, 0, &mut permutations);
        permutations
    }

    fn backtrace(nums: &mut Vec<i32>, start: usize, permutation: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            permutation.push(nums.clone());
            return;
        }

        let mut set: HashSet<i32> = HashSet::new();
        for i in start..nums.len() {
            if set.insert(nums[i]) {
                nums.swap(i, start);
                Self::backtrace(nums, start + 1, permutation);
                nums.swap(i, start);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]
        );
    }
}
