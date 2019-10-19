use p024_swap_nodes_in_pairs::Solution;
use listnode::ListNode;

fn main() {
    let res = Solution::swap_pairs(ListNode::from_vec(vec![1, 2, 3, 4]));
    assert_eq!(res, ListNode::from_vec(vec![2, 1, 4, 3]));
}
