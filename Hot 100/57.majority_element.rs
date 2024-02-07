/*
    题目：169. 多数元素
    链接：https://leetcode.cn/problems/majority-element/
 */
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut vote = 0;
        for i in nums{
          if vote ==0{
            res = i;
            vote =1;
          }else{
            if res == i{
              vote+=1;
            }else{
              vote-=1;
            }
          }
        }
        res
    }
  }