pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut comb = vec![];
        fn dfs(start: i32, n: i32, k: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if comb.len() == k as usize {
                res.push(comb.to_vec());
                return;
            }
            for i in start..=n {
                comb.push(i);
                dfs(i + 1, n, k, comb, res);
                comb.pop();
            }
        }
        dfs(1, n, k, &mut comb, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
        );

        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }
}
