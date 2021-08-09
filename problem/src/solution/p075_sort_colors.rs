pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut zero_idx = 0;
        let mut one_idx = 0;
        let mut two_idx = nums.len() - 1;

        while one_idx <= two_idx {
            if nums[one_idx] == 2 {
                nums.swap(one_idx, two_idx);
                if two_idx == 0 {
                    break;
                }
                two_idx -= 1;
            } else if nums[one_idx] == 1 {
                one_idx += 1;
            } else if nums[one_idx] == 0 {
                nums.swap(one_idx, zero_idx);
                zero_idx += 1;
                one_idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);

        let mut nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);

        let mut nums = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}
