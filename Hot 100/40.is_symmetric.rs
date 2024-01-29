/*
    题目：101. 对称二叉树
    链接：https://leetcode.cn/problems/symmetric-tree/
 */
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn check(left:&Option<Rc<RefCell<TreeNode>>>,right:&Option<Rc<RefCell<TreeNode>>>)->bool{
      match (left,right){
        (Some(l),Some(r))=>{
          let l = l.borrow();
          let r = r.borrow();
          l.val == r.val 
          && Self::check(&l.left, &r.right)&&Self::check(&l.right, &r.left)
        }
        (None,None)=> true,
        _ => false,

      }
      
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
      Self::check(&root, &root)
    }
}