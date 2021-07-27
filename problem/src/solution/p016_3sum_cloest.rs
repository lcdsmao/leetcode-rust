pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut sorted = nums.to_vec();
        sorted.sort();

        let mut diff: Option<i32> = None;
        let mut ans: Option<i32> = None;
        for i in 0..(sorted.len() - 2) {
            let mut j = i + 1;
            let mut k = sorted.len() - 1;
            while j < k {
                let sum = sorted[i] + sorted[j] + sorted[k];
                let d = (sum - target).abs();
                if d < diff.unwrap_or(i32::max_value()) {
                    ans = Some(sum);
                    diff = Some(d);
                }
                if sum == target {
                    return target;
                } else if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        ans.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1, 2, 1, -4];
        let ans = Solution::three_sum_closest(nums, 1);
        assert_eq!(ans, 2);
    }
}
