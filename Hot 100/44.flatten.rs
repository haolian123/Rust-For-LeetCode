/*
    题目：114. 二叉树展开为链表
    链接：https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/
 */
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn find_right(node:Option<&Rc<RefCell<TreeNode>>>)->Option<Rc<RefCell<TreeNode>>>{
      node.and_then(|n|{
        let mut n_borrow = n.borrow_mut();
        if n_borrow.right.is_some(){
          let right_node = n_borrow.right.take();
          let result = Self::find_right(right_node.as_ref());
          n_borrow.right = right_node;
          result
        }else{
          Some(Rc::clone(n))
        }
      })
    }
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root{
          let left_node = node.borrow_mut().left.take();
          let right_node = node.borrow_mut().right.take();

          if let Some(left) = left_node{
            let left_right_most = Self::find_right(Some(&left));
            node.borrow_mut().right = Some(left);
            if let Some(lrm_node) = left_right_most{
              lrm_node.borrow_mut().right = right_node;
            }
          }else{
            node.borrow_mut().right = right_node;
          }
          Self::flatten(&mut node.borrow_mut().right);
        }
    }
}