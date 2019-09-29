use p002_add_two_numbers::{ListNode, Solution};

fn main() {
    // 2 -> 4 -> 3
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            })),
        })),
    }));

    // 5 -> 6 -> 4
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));

    let ans = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 8,
                next: None,
            })),
        })),
    }));

    assert_eq!(Solution::add_two_numbers(l1, l2), ans);
}
