/*
    题目：32. 最长有效括号
    链接：https://leetcode.cn/problems/longest-valid-parentheses
 */

 impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut res = 0;
        let len = s.len();
        let mut dp = vec![0; len];
        let s_bytes = s.as_bytes(); // Convert string to byte array for easier access

        for i in 1..len {
            if s_bytes[i] == b')' {
                if s_bytes[i - 1] == b'(' {
                    dp[i] = if i >= 2 { dp[i - 2] + 2 } else { 2 };
                } else if i as i32 - dp[i - 1] as i32 - 1 >= 0 && s_bytes[(i as i32 - dp[i - 1] as i32 - 1) as usize] == b'(' {
                    dp[i] = dp[i - 1] + 2;
                    if i as i32 - dp[i - 1] as i32 - 2 >= 0 {
                        dp[i] += dp[(i as i32 - dp[i - 1] as i32 - 2) as usize];
                    }
                }
                res = std::cmp::max(res, dp[i]);
            }
        }

        res
    }
}
