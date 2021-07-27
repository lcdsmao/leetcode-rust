pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }
        let mut res: Vec<Vec<i32>> = Vec::new();

        let mut nums = nums.clone();
        nums.sort();

        for (i1, &n1) in nums[0..(nums.len() - 3)].iter().enumerate() {
            if i1 > 0 && n1 == nums[i1 - 1] {
                continue;
            }

            for i2 in (i1 + 1)..(nums.len() - 2) {
                let n2 = nums[i2];
                if i2 > i1 + 1 && n2 == nums[i2 - 1] {
                    continue;
                }

                let mut i3 = i2 + 1;
                let mut i4 = nums.len() - 1;

                while i3 < i4 {
                    let n3 = nums[i3];
                    let n4 = nums[i4];

                    if i3 > i2 + 1 && n3 == nums[i3 - 1] {
                        i3 += 1;
                        continue;
                    }
                    if i4 < nums.len() - 1 && n4 == nums[i4 + 1] {
                        i4 -= 1;
                        continue;
                    }

                    match (n1 + n2 + n3 + n4).cmp(&target) {
                        Ordering::Equal => {
                            let v = vec![n1, n2, n3, n4];
                            res.push(v);
                            i3 += 1;
                            i4 -= 1;
                        }
                        Ordering::Less => {
                            i3 += 1;
                        }
                        Ordering::Greater => {
                            i4 -= 1;
                        }
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        let mut ans = vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]];
        ans.sort();
        assert_eq!(res, ans);
    }
}
