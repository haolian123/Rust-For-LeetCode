impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut res = 0;
        let mut dp = vec![std::i32::MAX;(n+1) as usize];
        dp[0] = 0;
        for i in 1..=n{
            if i*i>n{
                break;
            }
            let mut tmp = i*i;
            for j in tmp..=n{
              if(dp[(j-tmp) as usize]!=std::i32::MAX){
                dp[j as usize] = std::cmp::min(dp[j as usize], dp[(j-tmp) as usize]+1);
              }
            }  
        }
        dp[n as usize]
    }
  }