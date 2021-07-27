pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans: Vec<i32> = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            let except = target - num;
            if let Some(j) = map.get(&except) {
                ans.push(*j);
                ans.push(i as i32);
                return ans;
            }
            map.insert(num, i as i32);
        }
        panic!("No valid answer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let ans = Solution::two_sum(nums, target);
        assert_eq!(ans, vec![0, 1]);
    }
}
