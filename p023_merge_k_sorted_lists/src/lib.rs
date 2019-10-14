use listnode::ListNode;

pub struct Solution {}

use std::borrow::BorrowMut;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::mem;

#[derive(Eq, PartialEq, Debug)]
pub struct NodeWrapper {
    node: Option<Box<ListNode>>
}

impl Ord for NodeWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.node, &other.node) {
            (Some(s), Some(o)) => {
                s.val.cmp(&o.val).reverse()
            }
            (Some(_), None) => {
                Ordering::Greater
            }
            (None, Some(_)) => {
                Ordering::Less
            }
            (None, None) => {
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for NodeWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::with_capacity(lists.len());
        lists.into_iter().for_each(|n| heap.push(NodeWrapper { node: n }));
        let mut res = heap.pop()?.node;
        let mut lsmall = &mut res;
        while let Some(NodeWrapper { node: mut n @ Some(_) }) = heap.pop() {
            if lsmall.is_none() || lsmall.as_ref()?.val > n.as_ref()?.val {
                mem::swap(lsmall, &mut n);
            }
            if lsmall.is_some() {
                lsmall = lsmall.as_mut()?.next.borrow_mut();
            }
            heap.push(NodeWrapper { node: n });
        }
        res
    }
}
