pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn combination_sum2(candidate: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidate = candidate.clone();
        candidate.sort();
        let mut res = vec![];
        Self::backtrace(&candidate, target, 0, &mut vec![], &mut res);
        return res;
    }

    fn backtrace(
        candidate: &Vec<i32>,
        target: i32,
        start: usize,
        ans: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if start >= candidate.len() {
            return;
        }
        for (i, &num) in candidate.iter().enumerate().skip(start) {
            if i > start && candidate[i - 1] == num {
                continue;
            }
            match num.cmp(&target) {
                Ordering::Less => {
                    ans.push(num);
                    Self::backtrace(candidate, target - num, i + 1, ans, res);
                    ans.pop();
                }
                Ordering::Equal => {
                    let mut v = ans.clone();
                    v.push(num);
                    res.push(v);
                    break;
                }
                Ordering::Greater => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let res = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        assert_eq!(Solution::combination_sum2(candidates, 8), res);
    }
}
