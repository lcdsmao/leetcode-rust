pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutation = vec![];
        let mut is_visited = vec![false; nums.len()];
        Self::backtrace(&nums, &mut is_visited, &mut vec![], &mut permutation);
        permutation
    }

    fn backtrace(
        nums: &Vec<i32>,
        is_visited: &mut Vec<bool>,
        candidate: &mut Vec<i32>,
        permutation: &mut Vec<Vec<i32>>,
    ) {
        if candidate.len() == nums.len() {
            permutation.push(candidate.clone());
            return;
        }
        for i in 0..nums.len() {
            if !is_visited[i] {
                is_visited[i] = true;
                candidate.push(nums[i]);
                Self::backtrace(nums, is_visited, candidate, permutation);
                candidate.pop();
                is_visited[i] = false;
            }
        }
    }

    pub fn permute2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrace(first: usize, nums: &mut Vec<i32>, permutation: &mut Vec<Vec<i32>>) {
            if first == nums.len() {
                permutation.push(nums.clone());
                return;
            }
            for i in first..nums.len() {
                nums.swap(i, first);
                backtrace(first + 1, nums, permutation);
                nums.swap(i, first);
            }
        }

        let mut permutation = vec![];
        backtrace(0, &mut nums.clone(), &mut permutation);
        permutation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expect = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        let mut res = Solution::permute(vec![1, 2, 3]);
        res.sort();
        assert_eq!(res, expect);

        let mut res = Solution::permute2(vec![1, 2, 3]);
        res.sort();
        assert_eq!(res, expect);
    }
}
