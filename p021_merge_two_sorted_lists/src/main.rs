use p021_merge_two_sorted_lists::Solution;
use listnode::ListNode;

fn main() {
    let res = Solution::merge_two_lists(
        ListNode::from_vec(vec![1, 2, 4]),
        ListNode::from_vec(vec![1, 3, 4]),
    );
    assert_eq!(res, ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]));
}
