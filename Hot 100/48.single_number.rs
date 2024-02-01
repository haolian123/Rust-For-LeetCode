/*
    题目：136. 只出现一次的数字
    链接：https://leetcode.cn/problems/single-number/
 */
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
      let mut m = 0;
      for num in nums{
        m^=num;
      }
      m
    }
  }