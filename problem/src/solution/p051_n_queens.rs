pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![];
        let n = n as usize;
        let mut board = vec![vec![b'.'; n]; n];
        Self::backtrace(
            &mut board,
            0,
            &mut vec![false; n],
            &mut vec![false; 2 * n - 1],
            &mut vec![false; 2 * n - 1],
            &mut res,
        );
        res
    }

    fn backtrace(
        board: &mut Vec<Vec<u8>>,
        row: usize,
        m90: &mut Vec<bool>,
        m45: &mut Vec<bool>,
        m135: &mut Vec<bool>,
        res: &mut Vec<Vec<String>>,
    ) {
        let n = board.len();
        if row == n {
            let ans = board
                .iter()
                .map(|v| String::from_utf8(v.clone()).unwrap())
                .collect();
            res.push(ans);
            return;
        }
        for col in 0..n {
            if !m90[col] && !m45[n + row - col - 1] && !m135[row + col] {
                m90[col] = true;
                m45[n + row - col - 1] = true;
                m135[row + col] = true;
                board[row][col] = b'Q';
                Self::backtrace(board, row + 1, m90, m45, m135, res);
                board[row][col] = b'.';
                m90[col] = false;
                m45[n + row - col - 1] = false;
                m135[row + col] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ],
            ]
        );
    }
}
