/*
    题目：121. 买卖股票的最佳时机
    链接：https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/
 */
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut right_max : Vec<i32> = vec![0;prices.len()];
        let mut max = 0;
        let mut res = 0;
        for i in (0..=prices.len()-1).rev(){
          right_max[i]=max;
          max=std::cmp::max(prices[i],max);
        }
        for i in 0..prices.len(){
          let sub = right_max[i]-prices[i];
          res = std::cmp::max(sub,res);
        }
  
        res 
    }
  }