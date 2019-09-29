// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut curr = &mut dummy_head;
        let mut p = l1.as_ref();
        let mut q = l2.as_ref();
        let mut carry = 0;
        while p.is_some() || q.is_some() {
            let p_val = p.map_or(0, |n| n.val);
            let q_val = q.map_or(0, |n| n.val);
            let sum = p_val + q_val + carry;
            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
            p = p.and_then(|n| n.next.as_ref());
            q = q.and_then(|n| n.next.as_ref());
        }
        if carry > 0 {
            curr.next = Some(Box::new(ListNode::new(carry)));
        }
        dummy_head.next
    }
}
