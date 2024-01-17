/*
    题目：21. 合并两个有序链表
    链接：https://leetcode.cn/problems/merge-two-sorted-lists
 */
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 将两个链表转为可变引用
        let mut list1 = list1;
        let mut list2 = list2;

        // 创建一个虚拟头节点
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        // 遍历两个链表，按顺序合并
        while list1.is_some() && list2.is_some() {
            let next = if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                list1.take().map(|mut node| { list1 = node.next.take(); node })
            } else {
                list2.take().map(|mut node| { list2 = node.next.take(); node })
            };
            tail.next = next;
            tail = tail.next.as_mut().unwrap();
        }

        // 连接剩余的部分
        tail.next = if list1.is_some() { list1 } else { list2 };

        // 返回合并后的链表
        dummy.next
    }
}