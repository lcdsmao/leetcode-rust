pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn combination_sum(candidate: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        for (i, &num) in candidate[start..].iter().enumerate() {
            match num.cmp(&target) {
                Ordering::Less => {
                    ans.push(num);
                    Self::backtrace(candidate, target - num, i + start, ans, res);
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
