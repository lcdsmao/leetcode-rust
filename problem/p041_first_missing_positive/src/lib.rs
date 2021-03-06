pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        // LeetCode rust version is old
        // let valid_range = 1..=(nums.len() as i32);
        for i in 0..nums.len() {
            let mut n = nums[i];
            while n >= 1
                && n < (nums.len() as i32)
                && n != nums[(n - 1) as usize]
                && n != i as i32 + 1
            {
                nums.swap(i, (n - 1) as usize);
                n = nums[i]
            }
        }

        for (i, &n) in nums.iter().enumerate() {
            if n != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
