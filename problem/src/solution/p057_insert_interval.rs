pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let mut res = vec![];

        let mut interval_iter = intervals.into_iter();
        let mut temp = Some(new_interval);
        while let Some(v) = interval_iter.next() {
            let t = temp.as_mut().unwrap();
            if v[1] < t[0] {
                res.push(v);
            } else if v[0] > t[1] {
                res.push(temp.take().unwrap());
                res.push(v);
                break;
            } else {
                t[0] = t[0].min(v[0]);
                t[1] = t[1].max(v[1]);
            }
        }
        if let Some(t) = temp {
            res.push(t);
        }
        res.extend(interval_iter);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        assert_eq!(
            Solution::insert(intervals, new_interval),
            vec![vec![1, 5], vec![6, 9]]
        );

        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        assert_eq!(
            Solution::insert(intervals, new_interval),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
