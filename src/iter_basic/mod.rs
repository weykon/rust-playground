pub mod fibonacci;
pub mod adapter_code_mode;

// iterator 基础
fn procedure_fn() {
    let nums = [1, 2, 3, 4];
    for i in nums {
        // print!(i);
    }
}

//  自定义迭代器
mod tree_order_iterator {
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        l: Option<Box<TreeNode>>,
        r: Option<Box<TreeNode>>,
    }
    struct InorderIterator<'a> {
        stack: Vec<&'a TreeNode>,
        current: Option<&'a TreeNode>,
    }
    impl<'a> InorderIterator<'a> {
        fn new(root: &'a TreeNode) -> Self {
            let mut iter = InorderIterator {
                stack: Vec::new(),
                current: Some(root),
            };
            while let Some(node) =
                iter.current
            {
                iter.stack.push(node);
                iter.current = node
                    .l
                    .as_ref()
                    .map(|n| n.as_ref());
            }
            iter
        }
    }
    impl<'a> Iterator for InorderIterator<'a> {
        type Item = i32;

        fn next(
            &mut self,
        ) -> Option<Self::Item> {
            if let Some(node) =
                self.stack.pop()
            {
                // 如果右子树存在，将右子树的所有左节点入栈
                self.current = node
                    .r
                    .as_ref()
                    .map(|n| n.as_ref());
                while let Some(curr) =
                    self.current
                {
                    self.stack.push(curr);
                    self.current = curr
                        .l
                        .as_ref()
                        .map(|n| n.as_ref());
                }
                Some(node.value)
            } else {
                None
            }
        }
    }
}
