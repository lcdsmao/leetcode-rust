pub struct Solution {}

type SMap = [[bool; 9]; 9];

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_map = [[false; 9]; 9];
        let mut col_map = row_map.clone();
        let mut box_map = row_map.clone();
        for (i, chs) in board.iter().enumerate() {
            for (j, &ch) in chs.iter().enumerate() {
                if let Some(num) = ch.to_digit(10).map(|n| n as usize - 1) {
                    row_map[i][num] = true;
                    col_map[j][num] = true;
                    box_map[(i / 3) * 3 + j / 3][num] = true;
                }
            }
        }
        Self::backtrack(board, 0, &mut row_map, &mut col_map, &mut box_map);
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        index: usize,
        row_map: &mut SMap,
        col_map: &mut SMap,
        box_map: &mut SMap,
    ) -> bool {
        if index >= 81 {
            return true;
        }
        let row = index / 9;
        let col = index % 9;
        let b = (row / 3) * 3 + col / 3;
        if board[row][col].is_digit(10) {
            return Self::backtrack(board, index + 1, row_map, col_map, box_map);
        } else {
            for num in 0..=8 {
                let ch = std::char::from_digit(num as u32 + 1, 10).unwrap();
                if !row_map[row][num] && !col_map[col][num] && !box_map[b][num] {
                    row_map[row][num] = true;
                    col_map[col][num] = true;
                    box_map[b][num] = true;
                    board[row][col] = ch;
                    if Self::backtrack(board, index + 1, row_map, col_map, box_map) {
                        return true;
                    }
                    row_map[row][num] = false;
                    col_map[col][num] = false;
                    box_map[b][num] = false;
                    board[row][col] = '.';
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, res);
    }
}
