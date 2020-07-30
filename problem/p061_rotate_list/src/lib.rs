use listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() { return head; }

        let mut node = &head;
        let mut len = 0;
        while let Some(node_inner) = node.as_ref() {
            len += 1;
            node = &node_inner.next;
        }

        let k = k % len;
        if k <= 0 {
            return head;
        }

        let mut head = head;
        let mut node = &mut head;

        for _ in 0..(len - k - 1) {
            node = &mut node.as_mut().unwrap().next;
        }

        let mut new_head = node.as_mut().unwrap().next.take();
        let mut node = &mut new_head;

        for _ in 0..(k - 1) {
            node = &mut node.as_mut().unwrap().next;
        }
        node.as_mut().unwrap().next = head;

        return new_head;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let res = Solution::rotate_right(head, 2);
        assert_eq!(res, ListNode::from_vec(vec![4, 5, 1, 2, 3]));

        let head = ListNode::from_vec(vec![0, 1, 2]);
        let res = Solution::rotate_right(head, 4);
        assert_eq!(res, ListNode::from_vec(vec![2, 0, 1]));
    }
}
