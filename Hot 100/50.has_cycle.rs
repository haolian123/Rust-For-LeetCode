/*
    题目：141. 环形链表
    链接：https://leetcode.cn/problems/linked-list-cycle/
 */
impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut slow = &head;
        let mut fast = &head;

        while let Some(next_fast) = fast {
            if next_fast.next.is_none() {
                return false;
            }

            fast = &next_fast.next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;

            if fast.is_none() || slow.is_none() {
                return false;
            }

            if std::ptr::eq(
                fast.as_ref().unwrap().as_ref(),
                slow.as_ref().unwrap().as_ref(),
            ) {
                return true;
            }
        }

        false
    }
}