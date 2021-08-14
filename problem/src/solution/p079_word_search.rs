use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let all_chars: HashSet<char> = board.iter().flatten().cloned().collect();
        let word: Vec<char> = word.chars().collect();
        if word.iter().any(|ch| !all_chars.contains(ch)) {
            return false;
        }

        let m = board.len();
        let n = board.first().unwrap().len();
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if Self::backtrack(&board, &word, &mut visited, i as i32, j as i32, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn backtrack(
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
        i: i32,
        j: i32,
        k: usize,
    ) -> bool {
        if k == word.len() {
            return true;
        }
        let m = board.len() as i32;
        let n = board.first().unwrap().len() as i32;
        if i < 0
            || i >= m
            || j < 0
            || j >= n
            || visited[i as usize][j as usize]
            || board[i as usize][j as usize] != word[k]
        {
            return false;
        }

        visited[i as usize][j as usize] = true;
        for dir in Self::DIRS.iter() {
            if Self::backtrack(board, word, visited, i + dir[0], j + dir[1], k + 1) {
                return true;
            }
        }
        visited[i as usize][j as usize] = false;

        false
    }

    const DIRS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true,
        );

        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            ),
            true,
        );

        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ),
            false,
        );
    }
}
