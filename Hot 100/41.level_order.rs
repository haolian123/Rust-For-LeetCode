/*
    题目：102. 二叉树的层序遍历
    链接：https://leetcode.cn/problems/binary-tree-level-order-traversal/
 */
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
      if root.is_none(){
        return vec![];
      }
      let mut res = vec![];
      let mut queue = VecDeque::new();

      if let Some(node) = root{
        queue.push_back(node);
      }
      while !queue.is_empty(){
        let level_size = queue.len();
        let mut path = Vec::with_capacity(level_size);
        for _ in 0..level_size{
          let node = queue.pop_front().unwrap();
          let node_ref = node.borrow();
          path.push(node_ref.val);
          if let Some(left) = &node_ref.left{
            queue.push_back(left.clone());
          }
          if let Some(right) = &node_ref.right{
            queue.push_back(right.clone());
          }
          
        }
        res.push(path);
        
      }
      res
    }
}