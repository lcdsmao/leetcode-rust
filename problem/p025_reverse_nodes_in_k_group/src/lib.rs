use listnode::ListNode;

pub struct Solution {}

use std::borrow::BorrowMut;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 1 {
            return None;
        }
        let mut head = head;
        let mut tail = head.borrow_mut();
        for _ in 0..k {
            if let Some(tail_inner) = tail {
                tail = tail_inner.next.borrow_mut();
            } else {
                return head;
            }
        }
        let tail = Self::reverse_k_group(tail.take(), k);
        Self::reverse_between(head, tail)
    }

    // Input: head -> 1 -> 2 -> 3, tail
    // Output: 3 -> 2 -> 1 -> head -> tail
    fn reverse_between(
        head: Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev = tail;
        let mut curr = head;
        while let Some(curr_inner) = curr.borrow_mut() {
            let next = curr_inner.next.take();
            curr_inner.next = prev;
            prev = curr;
            curr = next;
        }
        prev
    }
}
