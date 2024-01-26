/*
    题目：64. 最小路径和
    链接：https://leetcode.cn/problems/minimum-path-sum
 */
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = grid.clone();

        for i in 1..m {
            dp[i][0] += dp[i - 1][0];
        }
        for j in 1..n {
            dp[0][j] += dp[0][j - 1];
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i][j] + std::cmp::min(dp[i - 1][j], dp[i][j - 1]);
            }
        }

        dp[m - 1][n - 1]
    }
}
