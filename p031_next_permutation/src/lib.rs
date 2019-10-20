pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (0..nums.len()).rev() {
            let j = i - 1;
            match (nums.get(j), nums.get(i)) {
                (Some(&n), Some(&m)) if n < m => {
                    let k = nums.iter().enumerate()
                        .skip(i)
                        .rev()
                        .find(|(_, &v)| v > n)
                        .unwrap().0;
                    nums.swap(j, k);
                    nums[i..].reverse();
                    break;
                }
                (None, Some(_)) => {
                    nums.reverse();
                }
                _ => {}
            }
        }
    }
}
