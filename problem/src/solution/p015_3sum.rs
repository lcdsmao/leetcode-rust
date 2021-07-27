pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut sorted = nums.to_vec();
        sorted.sort();

        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 0..(sorted.len() - 2) {
            if i > 0 && sorted[i] == sorted[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = sorted.len() - 1;
            while j < k {
                match (sorted[i] + sorted[j] + sorted[k]).cmp(&0) {
                    Ordering::Equal => {
                        ans.push(vec![sorted[i], sorted[j], sorted[k]]);
                        while j < k && sorted[j] == sorted[j + 1] {
                            j += 1;
                        }
                        while j < k && sorted[k] == sorted[k - 1] {
                            k -= 1;
                        }
                        j += 1;
                        k -= 1;
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
                    Ordering::Less => j += 1,
                };
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ans = Solution::three_sum(nums);
        assert_eq!(ans, vec![vec![-1, -1, 2], vec![-1, 0, 1],]);
    }
}
