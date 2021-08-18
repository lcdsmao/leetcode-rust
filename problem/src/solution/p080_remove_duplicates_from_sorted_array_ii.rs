pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        let mut i = 0;
        for j in 2..n {
            if nums[i] != nums[j] {
                nums[i + 2] = nums[j];
                i += 1;
            }
        }

        (i + 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn assert_it(input: &mut Vec<i32>, expect: Vec<i32>) {
            let n = Solution::remove_duplicates(input) as usize;
            assert_eq!(n, expect.len());
            assert_eq!(input.iter().take(n).cloned().collect::<Vec<i32>>(), expect);
        }

        assert_it(&mut vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3]);
        assert_it(
            &mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
            vec![0, 0, 1, 1, 2, 3, 3],
        );
    }
}
