/*
    题目：72. 编辑距离
    链接：https://leetcode.cn/problems/edit-distance/
 */
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let char1 = word1.as_bytes();
        let char2 = word2.as_bytes();
        let mut dp:Vec<Vec<i32>> = vec![vec![0;n+1];m+1];

        for i in 1..=m as usize{
            dp[i][0]=i as i32;
        }
        for i in 1..=n as usize{
            dp[0][i]=i as i32;
        }

        for i in 1..=m as usize{
            for j in 1..=n as usize{
                if char1[i-1] == char2[j-1]{
                    dp[i][j] = dp[i-1][j-1];
                }else{
                    dp[i][j]=std::cmp::min(dp[i-1][j],std::cmp::min(dp[i][j-1],dp[i-1][j-1]))+1;
                } 
            }
        }


        return dp[m][n];
    }
}