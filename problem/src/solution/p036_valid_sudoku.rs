pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_map = [[false; 9]; 9];
        let mut col_map = row_map.clone();
        let mut box_map = row_map.clone();
        for (i, chs) in board.iter().enumerate() {
            for (j, &ch) in chs.iter().enumerate() {
                match ch.to_digit(10).map(|n| n as usize - 1) {
                    Some(index) => {
                        if !row_map[index][i] {
                            row_map[index][i] = true
                        } else {
                            return false;
                        }
                        if !col_map[index][j] {
                            col_map[index][j] = true
                        } else {
                            return false;
                        }
                        let k = (i / 3) * 3 + j / 3;
                        if !box_map[index][k] {
                            box_map[index][k] = true
                        } else {
                            return false;
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = vec![
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
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }
}
