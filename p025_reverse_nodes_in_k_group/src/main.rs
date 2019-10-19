use p025_reverse_nodes_in_k_group::Solution;
use listnode::ListNode;

fn main() {
    assert_eq!(
        Solution::reverse_k_group(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2),
        ListNode::from_vec(vec![2, 1, 4, 3, 5])
    );

    assert_eq!(
        Solution::reverse_k_group(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 3),
        ListNode::from_vec(vec![3, 2, 1, 4, 5])
    );
}
