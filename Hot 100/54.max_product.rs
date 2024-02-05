/*
    题目：152. 乘积最大子数组
    链接：https://leetcode.cn/problems/maximum-product-subarray/
 */

 impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut cur_min = 1;
        let mut cur_max = 1;
        for num in nums{
          if num<0{
            std::mem::swap(&mut cur_max, &mut cur_min);
          }
          cur_max = std::cmp::max(cur_max*num,num);
          cur_min = std::cmp::min(cur_min*num,num);
          res = std::cmp::max(cur_max,res);
        }
  
        res 
    }
  }
  