pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let p = p.into_bytes();
        let s = s.into_bytes();

        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;

        let mut penum = p.iter().enumerate();
        while let Some((i, b'*')) = penum.next() {
            dp[0][i + 1] = dp[0][i];
        }

        for (i, &chs) in s.iter().enumerate() {
            for (j, &chp) in p.iter().enumerate() {
                dp[i + 1][j + 1] = match chp {
                    b'*' => dp[i][j + 1] || dp[i + 1][j],
                    b'?' => dp[i][j],
                    b'a'..=b'z' => dp[i][j] && chs == chp,
                    _ => unreachable!(),
                };
            }
        }

        dp[s.len()][p.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
        assert_eq!(
            Solution::is_match("cb".to_string(), "?a".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("adceb".to_string(), "*a*b".to_string()),
            true
        );
    }
}
