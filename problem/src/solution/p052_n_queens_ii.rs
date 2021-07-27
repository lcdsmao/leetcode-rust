pub struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        Self::backtrace(
            n,
            0,
            &mut vec![false; n],
            &mut vec![false; 2 * n - 1],
            &mut vec![false; 2 * n - 1],
        )
    }

    fn backtrace(
        n: usize,
        row: usize,
        m90: &mut Vec<bool>,
        m45: &mut Vec<bool>,
        m135: &mut Vec<bool>,
    ) -> i32 {
        if row == n {
            return 1;
        }
        let mut count = 0;
        for col in 0..n {
            if !m90[col] && !m45[n + row - col - 1] && !m135[row + col] {
                m90[col] = true;
                m45[n + row - col - 1] = true;
                m135[row + col] = true;
                count += Self::backtrace(n, row + 1, m90, m45, m135);
                m90[col] = false;
                m45[n + row - col - 1] = false;
                m135[row + col] = false;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }
}
