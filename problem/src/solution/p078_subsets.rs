pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut curr = vec![];
        fn dfs(nums: &Vec<i32>, start: usize, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            res.push(curr.to_vec());
            if start >= nums.len() {
                return;
            }
            for i in start..nums.len() {
                curr.push(nums[i]);
                dfs(nums, i + 1, curr, res);
                curr.pop();
            }
        }
        dfs(&nums, 0, &mut curr, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ],
        );
        assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]],);
    }
}
