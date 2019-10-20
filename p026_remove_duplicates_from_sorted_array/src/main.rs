use p026_remove_duplicates_from_sorted_array::Solution;
use std::borrow::BorrowMut;

fn main() {
    let mut v = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(v.borrow_mut()), 2);
    assert_eq!(&v[0..2], vec![1, 2].as_slice());
}
