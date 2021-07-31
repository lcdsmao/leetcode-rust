pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        if m <= 0 {
            return 0;
        };
        let n = obstacle_grid.first().unwrap().len();
        if n <= 0 {
            return 0;
        };
        let mut dp = vec![1; n + 1];
        dp[0] = 0;
        for i in 0..m {
            for j in 0..n {
                match i {
                    0 => {
                        if obstacle_grid[i][j] == 1 || (j > 0 && dp[j] == 0) {
                            dp[j + 1] = 0;
                        }
                    }
                    _ => {
                        dp[j + 1] = if obstacle_grid[i][j] == 1 {
                            0
                        } else {
                            dp[j + 1] + dp[j]
                        }
                    }
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
            ]),
            2,
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0],]),
            1,
        );
    }
}
