use p019_remove_nth_node_from_end_of_list::{ListNode, Solution};

fn main() {
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
