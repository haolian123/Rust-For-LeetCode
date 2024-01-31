/*
    题目：128. 最长连续序列
    链接：https://leetcode.cn/problems/longest-consecutive-sequence/
 */
use std::collections::HashSet;
impl Solution {
  pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let Set:HashSet<i32> = nums.iter().cloned().collect();
    let mut res = 0;
    for &num in nums.iter(){
      if !Set.contains(&(num-1)){
        let mut count = 0;
        let mut index = num;
        while Set.contains(&index){
          count+=1;
          index+=1;
        }
        res = std::cmp::max(count,res);
      }
      
    }
    res
  }
}