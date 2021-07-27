use listnode::ListNode;

pub struct Solution {}

use core::mem;
use std::borrow::BorrowMut;

impl Solution {
    #[allow(unused_must_use)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut interval = 1;
        while interval < lists.len() {
            for i in (0..lists.len() - interval).step_by(interval * 2) {
                let m = Self::merge_two_lists(
                    lists.get_mut(i)?.take(),
                    lists.get_mut(i + interval)?.take(),
                );
                mem::replace(lists.get_mut(i)?, m);
            }
            interval *= 2;
        }
        lists.get_mut(0)?.take()
    }

    fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() || l2.is_none() {
            return l1.or(l2);
        }
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
        let res = Solution::merge_k_lists(vec![
            ListNode::from_vec(vec![1, 4, 5]),
            ListNode::from_vec(vec![1, 3, 4]),
            ListNode::from_vec(vec![2, 6]),
        ]);
        let ans = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(res, ans);
    }
}
