use listnode::ListNode;

pub struct Solution {}

use std::borrow::BorrowMut;
use std::mem;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode { val: 0, next: head });
        let mut curr = dummy_head.next.borrow_mut();
        while curr.is_some() && curr.as_ref()?.next.is_some() {
            let mut next = curr.as_mut()?.next.take();
            mem::swap(curr, next.borrow_mut());
            for _ in 0..2 {
                curr = curr.as_mut()?.next.borrow_mut();
                mem::swap(curr, next.borrow_mut());
            }
        }
        dummy_head.next
    }
}
