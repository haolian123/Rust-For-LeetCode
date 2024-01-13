/*
    题目：2.两数相加
    链接：https://leetcode.cn/problems/add-two-numbers
 */
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(0));
        let mut cur = &mut res; // 使用 cur 来遍历链表
        let mut carry = 0; // 进位
        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = l1.as_ref().map_or(0, |node| node.val) + 
                      l2.as_ref().map_or(0, |node| node.val) +
                      carry;
            carry = sum / 10;
            // 创建新节点存储当前位的和
            cur.next = Some(Box::new(ListNode::new(sum % 10))); 
            // 移动到下一个节点
            cur = cur.next.as_mut().unwrap(); 

            // 移动 l1 和 l2 到下一个节点
            if let Some(node) = l1 {
                l1 = node.next;
            } else {
                l1 = None;
            }
            if let Some(node) = l2 {
                l2 = node.next;
            } else {
                l2 = None;
            }
        }

        res.next
    }
}
