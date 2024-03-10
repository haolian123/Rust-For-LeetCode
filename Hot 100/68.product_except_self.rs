impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
      let n = nums.len();
      let mut res = vec![1;n];
      let mut right = 1;
      let mut left = 1; 
      for i in 0..n{
        res[i]*=left;
        res[n-i-1]*=right;
        right*=nums[n-i-1];
        left*= nums[i];
      }
      res 
    }
  }
  