/*
    题目：105. 从前序与中序遍历序列构造二叉树
    链接：https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
 */
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn solve(preorder: &Vec<i32>, inorder: &Vec<i32>, root: usize, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        let mut index = start;
        while index <= end && inorder[index] != preorder[root] {
            index += 1;
        }
        let node = Rc::new(RefCell::new(TreeNode::new(preorder[root])));
        if start < index {
            node.borrow_mut().left = Self::solve(preorder, inorder, root + 1, start, index - 1);
        }
        if index < end {
            node.borrow_mut().right = Self::solve(preorder, inorder, root + index - start + 1, index + 1, end);
        }
        Some(node)
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() || preorder.len() != inorder.len() {
            return None;
        }
        Self::solve(&preorder, &inorder, 0, 0, preorder.len() - 1)
    }
}