use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        if m <= 0 {
            return 0;
        }
        let n = grid.first().unwrap().len();
        if n <= 0 {
            return 0;
        }

        let mut dp = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                dp[j] = match (i, j) {
                    (0, 0) => 0,
                    (0, _) => dp[j - 1],
                    (_, 0) => dp[j],
                    _ => min(dp[j], dp[j - 1]),
                } + grid[i][j];
            }
        }

        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7,
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12,
        );
    }
}
