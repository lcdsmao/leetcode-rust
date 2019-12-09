pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut start = 0;
        let mut end = 0;
        while end < nums.len() - 1 {
            jumps += 1;
            let mut farthest = end as i32;
            for i in start..=end {
                if nums[i] + i as i32 > farthest {
                    farthest = nums[i] + i as i32;
                }
            }
            start = end + 1;
            end = farthest as usize;
        }
        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
