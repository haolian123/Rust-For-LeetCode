/*
    题目：148. 排序链表
    链接：https://leetcode.cn/problems/sort-list/
 */
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (Some(mut n1), Some(mut n2)) => {
                    if n1.val < n2.val {
                        n1.next = merge(n1.next.take(), Some(n2));
                        Some(n1)
                    } else {
                        n2.next = merge(Some(n1), n2.next.take());
                        Some(n2)
                    }
                }
                (Some(n1), None) => Some(n1),
                (None, Some(n2)) => Some(n2),
                _ => None,
            }
        }

        fn merge_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() || head.as_ref().map(|x| x.next.is_none()).unwrap() {
                return head;
            }

            let mut fast = head.as_ref();
            let mut slow = head.as_ref();
            let mut pre = slow.clone();
            while fast.is_some() {
                pre = slow.clone();
                fast = fast.map(|x| x.next.as_ref()).flatten();
                if fast.is_none() {
                    break;
                }
                fast = fast.map(|x| x.next.as_ref()).flatten();
                slow = slow.map(|x| x.next.as_ref()).flatten();
            }

            let pre =
                unsafe { std::mem::transmute::<Option<&Box<ListNode>>, Option<&mut Box<ListNode>>>(pre) };
            let mid = pre.map(|x| x.next.take()).flatten();

            let newhead = merge_sort(head);
            let newmid = merge_sort(mid);
            merge(newhead, newmid)
        }

        merge_sort(head)
    }
}
