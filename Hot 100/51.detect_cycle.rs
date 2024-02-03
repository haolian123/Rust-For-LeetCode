/*
    题目：142. 环形链表 II
    链接：https://leetcode.cn/problems/linked-list-cycle-ii/
 */
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        if head.is_none() || head.as_ref()?.borrow().next.is_none() {
            return None;
        }

        let mut slow = head.clone();
        let mut fast = head.clone();
        
        while let (Some(s), Some(f)) = (&slow, &fast) {
            let f_next = f.borrow().next.clone();
            fast = f_next.and_then(|next| next.borrow().next.clone());
            slow = s.borrow().next.clone();

            if slow == fast {
                break;
            }
        }

        if fast.is_none() {
            return None;
        }

        fast = head;
        while slow != fast {
            slow = slow.unwrap().borrow().next.clone();
            fast = fast.unwrap().borrow().next.clone();
        }
        fast
    }
}
