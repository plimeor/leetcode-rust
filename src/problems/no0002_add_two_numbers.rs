// https://leetcode-cn.com/problems/add-two-numbers/
struct Solution;

use crate::common::ListNode;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut ptr = &mut result;

        // l1, l2, sum, carry
        let mut status = (l1, l2, 0, 0);

        loop {
            status = match status {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    let sum = list.val + carry;
                    (list.next, None, sum % 10, sum / 10)
                }
                (Some(l1), Some(l2), _, carry) => {
                    let sum = l1.val + l2.val + carry;
                    (l1.next, l2.next, sum % 10, sum / 10)
                }
            };

            *ptr = Some(Box::new(ListNode::new(status.2)));
            // 对 rust 还不够熟悉，unwrap 需要注意一下
            // unwrap() 函数会转移变量的 owner，ptr 是一个可写引用，可写引用是不允许更改 owner 的
            //
            // Option.as_mut 方法，本质上是创建一个新的 Option，并保存之前内容的可写引用 即 Option<String> -> Option<&mut String>
            // 用在这里，会将 &mut Option<Box<ListNode>> 转换为 Option<&mut Box<ListNode>>
            // 摆脱了 borrow 关系，就可以使用 unwrap 方法了
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        result
    }
}
