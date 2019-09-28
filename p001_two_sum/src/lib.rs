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
