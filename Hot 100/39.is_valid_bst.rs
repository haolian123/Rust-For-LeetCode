/*
    题目：98. 验证二叉搜索树
    链接：https://leetcode.cn/problems/validate-binary-search-tree/
 */
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  fn check(node:&Option<Rc<RefCell<TreeNode>>>,pre:&mut Option<Rc<RefCell<TreeNode>>>)->bool{
    if let Some(n) = node{
      let n_borrow = n.borrow();
      if !Self::check(&n_borrow.left, pre){
        return false;
      }
      if let Some(p) = pre{
        let p_borrow = p.borrow();
        if p_borrow.val >= n_borrow.val{
          return false;
        } 
      }
      *pre = Some(n.clone());
      if !Self::check(&n_borrow.right, pre){
        return false;
      }
    }
    true
  }
  pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut pre = None;
    Self::check(&root,&mut pre)
  }
}