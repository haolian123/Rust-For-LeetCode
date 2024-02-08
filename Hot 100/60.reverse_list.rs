/*
    题目：206. 反转链表
    链接：https://leetcode.cn/problems/reverse-linked-list/
 */
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut pre = None;
      let mut head = head;
      while let Some(mut node) = head{
        let mut tmp = node.next.take();
        node.next = pre;
        pre = Some(node);
        head=tmp;
      }
      pre
    }
  }