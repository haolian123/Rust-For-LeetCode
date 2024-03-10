impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
      let m = matrix.len();
      let n = matrix[0].len();
      let mut res = 0;
      let mut dp = vec![vec![0;n+1];m+1];
      for i in 1..=m{
        for j in 1..=n{
          if(matrix[i-1][j-1]=='1'){
            dp[i][j]=std::cmp::min(std::cmp::min(dp[i-1][j], dp[i][j-1]),dp[i-1][j-1])+1;
            res = std::cmp::max(dp[i][j], res);
          }
        }
      }
      return res*res;
    }
  }