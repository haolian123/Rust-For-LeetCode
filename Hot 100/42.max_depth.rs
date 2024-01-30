/*
    题目：104. 二叉树的最大深度
    链接：https://leetcode.cn/problems/maximum-depth-of-binary-tree/
 */
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      match root {
          None => 0, // If the node is null, the depth is 0.
          Some(node) => {
              let left_depth = Solution::max_depth(node.borrow().left.clone()); // Recursively find the depth of the left subtree.
              let right_depth = Solution::max_depth(node.borrow().right.clone()); // Recursively find the depth of the right subtree.
              1 + std::cmp::max(left_depth, right_depth) // The depth of the tree is 1 (for the current node) plus the maximum of the depths of the left and right subtrees.
          }
      }
  }
}