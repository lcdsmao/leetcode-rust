use listnode::ListNode;
use p023_merge_k_sorted_lists::Solution;

fn main() {
    let res = Solution::merge_k_lists(vec![
        ListNode::from_vec(vec![1, 4, 5]),
        ListNode::from_vec(vec![1, 3, 4]),
        ListNode::from_vec(vec![2, 6]),
    ]);
    let ans = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(res, ans);
}
