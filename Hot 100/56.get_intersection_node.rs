/*
    题目：160. 相交链表
    链接：https://leetcode.cn/problems/intersection-of-two-linked-lists/
 */


 impl Solution {
    pub fn get_intersection_node(head_a: Option<Box<ListNode>>, head_b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut list1 = &head_a;
        let mut list2 = &head_b;

        while list1 as *const _ != list2 as *const _ {
            // 如果list1不为空，向后移动，否则跳转到headB
            list1 = if let Some(node) = list1 {
                &node.next
            } else {
                &head_b
            };

            // 如果list2不为空，向后移动，否则跳转到headA
            list2 = if let Some(node) = list2 {
                &node.next
            } else {
                &head_a
            };
        }

        list1.clone()
    }
}