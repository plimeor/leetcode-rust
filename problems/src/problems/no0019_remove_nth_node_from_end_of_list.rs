// https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/

// 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
struct Solution;

use crate::common::ListNode;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        // 快指针指向最新的节点
        let mut fast_ptr = &head.clone();
        // 慢指针指向目标节点的前一个节点
        let mut slow_ptr = &mut head;

        let mut i = 0;
        while fast_ptr.as_ref().unwrap().next.is_some() {
            if i >= n {
                slow_ptr = &mut slow_ptr.as_mut().unwrap().next;
            }
            fast_ptr = &fast_ptr.as_ref().unwrap().next;
            i += 1;
        }

        let target_prt = &slow_ptr.as_mut().unwrap().next;
        slow_ptr.as_mut().unwrap().next = target_prt.as_ref().unwrap().next.clone();
        head.unwrap().next
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from(vec![1, 2, 3, 4, 5]), 2),
        ListNode::from(vec![1, 2, 3, 5])
    );

    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from(vec![1]), 1),
        None
    );

    assert_eq!(
        Solution::remove_nth_from_end(ListNode::from(vec![1, 2]), 1),
        ListNode::from(vec![1])
    );
}
