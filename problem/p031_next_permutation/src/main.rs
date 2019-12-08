use p031_next_permutation::Solution;
use std::borrow::BorrowMut;

fn main() {
    let mut v = vec![1, 2, 3];
    Solution::next_permutation(v.borrow_mut());
    assert_eq!(v, vec![1, 3, 2].borrow_mut());
}
