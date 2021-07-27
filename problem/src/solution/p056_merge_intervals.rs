pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        let mut intervals = intervals;
        intervals.sort_by_key(|v| v[0]);
        let mut merged = vec![];
        let mut temp = intervals[0].to_owned();
        for v in intervals {
            if v[0] <= temp[1] {
                temp[1] = v[1].max(temp[1]);
            } else {
                merged.push(temp);
                temp = v.to_owned();
            }
        }
        merged.push(temp);
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18],]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18],]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5],]),
            vec![vec![1, 5],]
        );
    }
}
