/*
    题目：124. 二叉树中的最大路径和
    链接：https://leetcode.cn/problems/binary-tree-maximum-path-sum/
 */
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

impl Solution {
    fn solve(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let left = max(0, Solution::solve(&node.left, res));
            let right = max(0, Solution::solve(&node.right, res));
            
            *res = max(*res, left + right + node.val);
            max(left, right) + node.val
        } else {
            0
        }
    }
    
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = std::i32::MIN;
        Solution::solve(&root, &mut res);
        res
    }
}