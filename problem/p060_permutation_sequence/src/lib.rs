pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut nums: Vec<_> = (1..=n).collect();
        let mut factors: Vec<_> = nums
            .iter()
            .scan(1, |state, &x| {
                *state = *state * x;
                Some(*state)
            })
            .collect();

        let mut k = k - 1;
        let mut res = vec![];

        factors.pop();
        factors.reverse();
        for f in factors {
            let j = k / f;
            res.push(nums.remove(j as usize).to_string());
            k = k % f;
        }
        res.push(nums[0].to_string());
        res.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
    }
}
