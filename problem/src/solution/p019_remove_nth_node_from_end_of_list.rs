// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    // Two pointers algorithm seems impossible in Rust unless using unsafe
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode { val: 0, next: head });

        let mut len = 0;

        let mut p = &dummy_head;
        while let Some(next) = p.next.as_ref() {
            p = next;
            len += 1;
        }

        let mut p = &mut dummy_head;
        while len > n {
            p = p.next.as_mut().unwrap();
            len -= 1;
        }
        p.next = p.next.take().and_then(|n| n.next);
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let head = create_list_node(vec![1, 2, 3, 4, 5]);
        let res = Solution::remove_nth_from_end(head, 2);
        let ans = create_list_node(vec![1, 2, 3, 5]);
        assert_eq!(res, ans);
    }

    fn create_list_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut nodes: Vec<_> = vec.iter().map(|&n| Box::new(ListNode::new(n))).collect();

        for i in (1..nodes.len()).rev() {
            let last = nodes.remove(i);
            nodes[i - 1].next = Some(last);
        }

        nodes.pop()
    }
}
