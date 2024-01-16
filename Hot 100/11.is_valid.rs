/*
    题目：20. 有效的括号
    链接：https://leetcode.cn/problems/valid-parentheses/
*/
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let mut brackets: HashMap<char,char> = HashMap::new();
        brackets.insert(')', '(');
        brackets.insert('}', '{');
        brackets.insert(']', '[');

        for c in s.chars(){
            match brackets.get(&c){
                Some(&opening_bracket) =>{
                    if stack.pop()!=Some(opening_bracket){
                        return false;
                    }
                }
                None =>{
                    stack.push(c);
                }
            }
        }

        return stack.is_empty();
    }
}