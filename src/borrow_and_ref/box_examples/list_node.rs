use std::{borrow::BorrowMut, f32::INFINITY, ops::DerefMut};

// 首先裁决一下为什么这里是需要使用Box
// 如果使用直接引用的，
// next: Option<&ListNode>,
// 那么
// struct ListNode {
//     value: i32,
//     next: Option<ListNode>, // 错误：递归类型 `ListNode` has infinite size
// }
// 这里有一个循环的问题，因为next是一个ListNode，而ListNode里面又有一个next，这样就会无限循环下去
// 也就是永远不知道它的大小
#[derive(Debug)]
struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(value: i32) -> Self {
        ListNode { value, next: None }
    }

    fn append(&mut self, value: i32) {
        // 递归
        // match &mut self.next {
        //     Some(next) => next.append(value),
        //     None => self.next = Some(Box::new(ListNode::new(value))),
        // }
        // 迭代
        // 这里其实可以使用那个 mem::replace
        let mut current = Box::new(self);
        loop {
            match current.next {
                Some(ref mut next) => current = Box::new(next.as_mut()),
                None => {
                    current.next = Some(Box::new(ListNode::new(value)));
                    break;
                }
            }
        }
    }
}

mod tests {
    use crate::borrow_and_ref::box_examples::list_node::ListNode;

    #[test]
    fn test1234() {
        let mut list = ListNode::new(1);
        list.append(2);
        list.append(3);
        list.append(4);
        println!("here result: {:?}", list);
    }
}