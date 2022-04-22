// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: Clone = i32> {
    pub val: T,
    pub next: Option<Box<Self>>,
}

impl<T: Clone> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(list: Vec<T>) -> Option<Box<Self>> {
        if list.len() == 0 {
            return None;
        }

        let mut head: Option<Box<Self>> = None;
        let mut tail = &mut head;

        for i in 0..list.len() {
            let val = &list[i];
            *tail = Some(Box::new(Self::new(val.clone())));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}

#[test]
fn test() {
    let list = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3))),
        })),
    };

    assert_eq!(Box::new(list), ListNode::from(vec![1, 2, 3]).unwrap());

}
