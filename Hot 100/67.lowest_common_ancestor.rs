use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match(&root,&p,&q){
          (Some(node),Some(p_node),Some(q_node))=>{
            let p_val = p_node.borrow().val;
            let q_val = q_node.borrow().val;

            if node.borrow().val == p_val || node.borrow().val == q_val{
              return Some(node.clone());
            }

            let left = Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(),q.clone());
            let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());

            match (left,right){
              (Some(_),Some(_)) => Some(node.clone()),
              (Some(left_node),None) => Some(left_node),
              (None, Some(right_node)) => Some(right_node),
              (None,None) => None,
            }
          },
          _ => None,
        }
    }
}
