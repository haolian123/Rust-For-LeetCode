/*
    题目：94. 二叉树的中序遍历
    链接：https://leetcode.cn/problems/binary-tree-inorder-traversal/
 */
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = VecDeque::new();
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur {
                stack.push_back(node.clone());
                cur = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop_back() {
                res.push(node.borrow().val);
                cur = node.borrow().right.clone();
            }
        }

        res
    }
}
