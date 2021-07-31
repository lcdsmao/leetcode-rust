use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];

        dp[0][0] = 0;
        for i in 0..m {
            dp[i + 1][0] = i as i32 + 1;
        }
        for i in 0..n {
            dp[0][i + 1] = i as i32 + 1;
        }

        for i in 0..m {
            for j in 0..n {
                dp[i + 1][j + 1] = if word1[i] == word2[j] {
                    dp[i][j]
                } else {
                    min(dp[i][j], min(dp[i + 1][j], dp[i][j + 1])) + 1
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3,
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5,
        );
    }
}
