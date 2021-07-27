pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut dir = 0;
        let mut x = 0;
        let mut y = 0;
        let d = [[0, 1], [1, 0], [0, -1], [-1, 0]];

        for i in 0..((n * n) as usize) {
            matrix[x as usize][y as usize] = i as i32 + 1;
            let nx = x + d[dir][0];
            let ny = y + d[dir][1];
            if nx < 0 || ny < 0 || nx >= n || ny >= n || matrix[nx as usize][ny as usize] != 0 {
                dir = (dir + 1) % 4;
            }
            x += d[dir][0];
            y += d[dir][1];
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        )
    }
}
