pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix.first().unwrap().is_empty() {
            return;
        }
        let row_last = matrix.len() - 1;
        let col_last = matrix.first().unwrap().len() - 1;
        for i in 0..row_last {
            for j in i..row_last - i {
                let mut prev = *matrix.get(i).unwrap().get(j).unwrap();
                prev = Self::replace_matrix_element(matrix, j, row_last - i, prev);
                prev = Self::replace_matrix_element(matrix, row_last - i, col_last - j, prev);
                prev = Self::replace_matrix_element(matrix, col_last - j, i, prev);
                Self::replace_matrix_element(matrix, i, j, prev);
            }
        }
    }

    fn replace_matrix_element(
        matrix: &mut Vec<Vec<i32>>,
        row: usize,
        col: usize,
        value: i32,
    ) -> i32 {
        let target = matrix.get_mut(row).and_then(|v| v.get_mut(col)).unwrap();
        std::mem::replace(target, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);

        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ]
        );
    }
}
