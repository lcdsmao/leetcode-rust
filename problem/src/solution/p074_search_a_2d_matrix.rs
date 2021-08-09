pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix.first().unwrap().len();
        let mut top: i32 = 0;
        let mut bottom: i32 = m as i32 - 1;
        let mut vmid: i32 = m as i32;

        while top <= bottom {
            vmid = top + (bottom - top) / 2;
            if target < matrix[vmid as usize][0] {
                bottom = vmid - 1;
            } else if target > matrix[vmid as usize][n - 1] {
                top = vmid + 1;
            } else {
                break;
            }
        }

        if vmid >= m as i32 {
            return false;
        }

        let mut left: i32 = 0;
        let mut right: i32 = n as i32 - 1;
        while left <= right {
            let hmid = left + (right - left) / 2;
            let v = matrix[vmid as usize][hmid as usize];
            if target > v {
                left = hmid + 1;
            } else if target < v {
                right = hmid - 1;
            } else if target == v {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3,
            ),
            true,
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13,
            ),
            false,
        );
    }
}
