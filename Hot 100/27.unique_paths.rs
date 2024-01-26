/*
    题目：62. 不同路径
    链接：https://leetcode.cn/problems/unique-paths/
 */

 impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![1; n as usize]; m as usize];

        for i in 1..m as usize {
            for j in 1..n as usize {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m as usize - 1][n as usize - 1]
    }
}
