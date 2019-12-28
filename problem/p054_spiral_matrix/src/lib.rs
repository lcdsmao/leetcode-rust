pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        if matrix.is_empty() {
            return ans;
        }

        let mut r_s = 0 as i32;
        let mut r_e = (matrix.len() - 1) as i32;
        let mut c_s = 0 as i32;
        let mut c_e = (matrix[0].len() - 1) as i32;

        while r_s <= r_e && c_s <= c_e {
            for i in c_s..=c_e {
                ans.push(matrix[r_s as usize][i as usize])
            }
            r_s += 1;

            for i in r_s..=r_e {
                ans.push(matrix[i as usize][c_e as usize]);
            }
            c_e -= 1;

            if r_s <= r_e && c_s <= c_e {
                for i in (c_s..=c_e).rev() {
                    ans.push(matrix[r_e as usize][i as usize]);
                }
                r_e -= 1;

                for i in (r_s..=r_e).rev() {
                    ans.push(matrix[i as usize][c_s as usize])
                }
                c_s += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        );

        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        );
    }
}
