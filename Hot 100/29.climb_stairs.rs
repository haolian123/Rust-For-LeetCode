/*
    题目：70. 爬楼梯
    链接：https://leetcode.cn/problems/climbing-stairs
 */

 impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        dp[2] = 2;
        
        for i in 3..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        
        dp[n as usize]
    }
}
