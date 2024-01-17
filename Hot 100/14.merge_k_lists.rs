/*
    题目：23. 合并 K 个升序链表
    链接：https://leetcode.cn/problems/merge-k-sorted-lists
 */

 use std::{cmp::Ordering, collections::BinaryHeap};

// 定义一个最小堆结构
#[derive(Eq, PartialEq)]
struct MinHeap(Box<ListNode>);

// 实现Ord，以便在二叉堆中正确比较
impl Ord for MinHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.val.cmp(&self.0.val)
    }
}

// 实现PartialOrd
impl PartialOrd for MinHeap {
    // 实现部分顺序比较
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // 二叉堆
        let mut heap = BinaryHeap::new();

        for node in lists {
            if let Some(node) = node {
                heap.push(MinHeap(node));
            }
        }

        let mut head = Box::new(ListNode::new(0));

        let mut tail = &mut head;

        while let Some(MinHeap(node)) = heap.pop() {
            // 将堆顶节点添加到当前链表的末端
            tail.next = Some(node.clone());
            // 更新tail指向链表的末端
            tail = tail.next.as_mut().unwrap();

            // 如果节点有下一个节点，将其加入堆中继续处理
            if let Some(next) = node.next {
                heap.push(MinHeap(next));
            }
        }

        head.next
    }
}
