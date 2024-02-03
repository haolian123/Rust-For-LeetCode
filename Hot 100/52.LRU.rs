/*
    题目：146. LRU 缓存
    链接：https://leetcode.cn/problems/lru-cache/
 */
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    val: i32,
    key: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val,
            key,
            next: None,
            prev: None,
        }))
    }
}

struct LRUCache {
    capacity: usize,
    size: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let head = Node::new(0, 0);
        head.borrow_mut().next = Some(Rc::clone(&head));
        head.borrow_mut().prev = Some(Rc::clone(&head));

        LRUCache {
            capacity,
            size: 0,
            map: HashMap::new(),
            head,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            let val = node.borrow().val;
            self.move_to_head(&node);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let maybe_node = self.map.get(&key).cloned();
        match maybe_node {
            Some(node) => {
                node.borrow_mut().val = value;
                self.move_to_head(&node);
            },
            None => {
                if self.size >= self.capacity {
                    let tail_node = self.head.borrow_mut().prev.take().unwrap();
                    let prev_node = tail_node.borrow_mut().prev.take().unwrap();
                    prev_node.borrow_mut().next = Some(Rc::clone(&self.head));
                    self.head.borrow_mut().prev = Some(Rc::clone(&prev_node));

                    self.map.remove(&tail_node.borrow().key);
                    self.size -= 1;
                }
                let new_node = Node::new(key, value);
                new_node.borrow_mut().next = Some(self.head.borrow().next.as_ref().unwrap().clone());
                new_node.borrow_mut().prev = Some(Rc::clone(&self.head));

                let next_node = self.head.borrow_mut().next.take().unwrap();
                next_node.borrow_mut().prev = Some(Rc::clone(&new_node));

                self.head.borrow_mut().next = Some(Rc::clone(&new_node));
                self.map.insert(key, Rc::clone(&new_node));
                self.size += 1;
            }
        }
    }

    fn move_to_head(&mut self, node: &Rc<RefCell<Node>>) {
        let prev_node = node.borrow_mut().prev.take().unwrap();
        let next_node = node.borrow_mut().next.take().unwrap();

        prev_node.borrow_mut().next = Some(Rc::clone(&next_node));
        next_node.borrow_mut().prev = Some(Rc::clone(&prev_node));

        node.borrow_mut().next = Some(self.head.borrow().next.as_ref().unwrap().clone());
        node.borrow_mut().prev = Some(Rc::clone(&self.head));

        let head_next_node = self.head.borrow_mut().next.take().unwrap();
        head_next_node.borrow_mut().prev = Some(Rc::clone(node));
        self.head.borrow_mut().next = Some(Rc::clone(node));
    }
}
