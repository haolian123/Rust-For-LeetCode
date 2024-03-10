use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut window: VecDeque<usize> = VecDeque::new();
        let mut res: Vec<i32> = Vec::new();
        let n = nums.len();
        let k = k as usize;
        for i in 0..n{
          while let Some(&first) = window.front(){
            if i - first>=k{
              window.pop_front();
            }else{
              break;
            }
          }
          while let Some(&last) = window.back(){
            if nums[last] < nums[i]{
              window.pop_back();
            }else{
              break;
            }
          }
          window.push_back(i);
          if i>=k-1{
            res.push(nums[*window.front().unwrap()])
          }
        }
        res 
    }
}
