/*
    题目：96. 不同的二叉搜索树
    链接：https://leetcode.cn/problems/unique-binary-search-trees/
 */
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 1..=i {
                dp[i] += dp[i - j] * dp[j - 1];
            }
        }

        dp[n]
    }
}
