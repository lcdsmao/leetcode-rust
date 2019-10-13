// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<Self>> {
        let mut nodes: Vec<_> = vec.iter().map(|&n| Box::new(ListNode::new(n))).collect();

        for i in (1..nodes.len()).rev() {
            let last = nodes.remove(i);
            nodes[i - 1].next = Some(last);
        }

        nodes.pop()
    }
}
