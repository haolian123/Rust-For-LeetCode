/*
    题目：19. 删除链表的倒数第 N 个结点
    链接：https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
*/

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut dummy;
        let mut fast = &slow.clone();

        // 移动快指针
        for _ in 0..=n {
            if let Some(next) = fast {
                fast = &next.next;
            } else {
                return None; // n 大于链表长度
            }
        }

        // 同时移动快慢指针
        while let Some(next) = fast {
            slow = &mut slow.as_mut().unwrap().next;
            fast = &next.next;
        }

        // 删除节点
        let next_node = slow.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = next_node.and_then(|node| node.next);

        dummy.unwrap().next
    }
}