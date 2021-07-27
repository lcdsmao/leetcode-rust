use listnode::ListNode;

pub struct Solution;

use std::borrow::BorrowMut;
use std::mem;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = l1;
        let mut l2 = l2;
        let mut lsmall = &mut result;
        let lbig = &mut l2;
        while lbig.is_some() {
            if lsmall.is_none() || lsmall.as_ref()?.val > lbig.as_ref()?.val {
                mem::swap(lsmall, lbig);
            }
            if lsmall.is_some() {
                lsmall = lsmall.as_mut()?.next.borrow_mut();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::merge_two_lists(
            ListNode::from_vec(vec![1, 2, 4]),
            ListNode::from_vec(vec![1, 3, 4]),
        );
        assert_eq!(res, ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]));
    }
}
