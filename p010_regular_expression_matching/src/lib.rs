pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for (j, chp) in p.chars().enumerate() {
            if chp == '*' {
                dp[0][j + 1] = dp[0][j - 1]
            }
        }

        for (i, chs) in s.chars().enumerate() {
            for (j, chp) in p.chars().enumerate() {
                dp[i + 1][j + 1] = match chp {
                    'a'..='z' => {
                        dp[i][j] && chs == chp
                    }
                    '.' => {
                        dp[i][j]
                    }
                    '*' => {
                        match p.chars().nth(j - 1) {
                            Some(c) if c == chs || c == '.' => {
                                dp[i][j + 1] || dp[i + 1][j - 1] || dp[i + 1][j]
                            }
                            Some(_) => {
                                dp[i + 1][j - 1]
                            }
                            None => {
                                unreachable!()
                            }
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }

        dp[s.len()][p.len()]
    }
}
