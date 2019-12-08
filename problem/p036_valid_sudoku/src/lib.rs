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
