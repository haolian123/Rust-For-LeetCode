/*
    题目：155. 最小栈
    链接：https://leetcode.cn/problems/min-stack/
 */

 struct MinStack {
    min_stack:Vec<i32>,
    stack:Vec<i32>,
  }

impl MinStack {

    fn new() -> Self {
    MinStack{
        stack:Vec::new(),
        min_stack:vec![std::i32::MAX],
    }
    
    }
    
    fn push(&mut self, val: i32) {
    self.stack.push(val);
    let  push_val =std::cmp::min(*self.min_stack.last().unwrap(), val);
    
    self.min_stack.push(push_val);
    }
    
    fn pop(&mut self) {
    self.stack.pop();
    self.min_stack.pop();
    }
    
    fn top(&self) -> i32 {
    return *self.stack.last().unwrap();
    }
    
    fn get_min(&self) -> i32 {
    return *self.min_stack.last().unwrap();
    }
}