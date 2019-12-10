pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut slow = 0;
        for fast in 0..nums.len() {
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }
        slow as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut v), 2);
        assert_eq!(&v[0..2], vec![1, 2].as_slice());
    }
}
