// https://leetcode-cn.com/problems/middle-of-the-linked-list/

// 给定一个头结点为 head 的非空单链表，返回链表的中间结点。
// 如果有两个中间结点，则返回第二个中间结点。
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_ptr = &head;
        let mut slow_ptr = &head;

        // Option.as_ref()，可以将 &Option<T> 转换为 Option<&T>
        // 这样就可以在不更改所有权的情况下使用 unwrap 了。
        while fast_ptr.is_some() && fast_ptr.as_ref().unwrap().next.is_some() {
            fast_ptr = &fast_ptr.as_ref().unwrap().next;
            fast_ptr = &fast_ptr.as_ref().unwrap().next;
            slow_ptr = &slow_ptr.as_ref().unwrap().next;
        }
        slow_ptr.clone()
    }
}
