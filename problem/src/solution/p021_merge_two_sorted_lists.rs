use listnode::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut curr = &mut dummy_head;
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        loop {
            let val = match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        l1 = n1.next.as_ref();
                        n1.val
                    } else {
                        l2 = n2.next.as_ref();
                        n2.val
                    }
                }
                (Some(n1), None) => {
                    l1 = n1.next.as_ref();
                    n1.val
                }
                (None, Some(n2)) => {
                    l2 = n2.next.as_ref();
                    n2.val
                }
                _ => {
                    break dummy_head.next;
                }
            };
            curr.next = Some(Box::new(ListNode::new(val)));
            curr = curr.next.as_mut().unwrap();
        }
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
