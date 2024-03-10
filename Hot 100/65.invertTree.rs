use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert(&root);
        root
    }
    fn invert(node: &Option<Rc<RefCell<TreeNode>>>){
      if let Some(n) = node{
        let mut n_borrow = n.borrow_mut();
        let left = n_borrow.left.take();
        let right = n_borrow.right.take();
        n_borrow.left = right;
        n_borrow.right = left;
        drop(n_borrow);
        Self::invert(&n.borrow().left);
        Self::invert(&n.borrow().right);
      }
    }
}