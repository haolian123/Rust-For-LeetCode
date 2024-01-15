/*
    题目： 10.正则表达式匹配
    链接：https://leetcode.cn/problems/regular-expression-matching
 */

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect(); 
        let p_chars: Vec<char> = p.chars().collect();
        let m = s_chars.len();
        let n = p_chars.len();
        // dp[i][j]表示s的前i个字符和p的前j个字符的匹配情况
        let mut dp = vec![vec![false; n + 1]; m + 1]; 

        dp[0][0] = true; // 两个空字符串是匹配的

        for i in 0..=m {
            for j in 1..=n {
                if p_chars[j - 1] == '*' {
                    // 处理 '*' 字符
                    // 不使用 '*' 字符
                    let flag1 = dp[i][j - 2]; 
                    // 使用 '*' 字符：检查前一个字符是否匹配
                    let flag2 = i > 0 && dp[i - 1][j] && (s_chars[i - 1] == p_chars[j - 2] || p_chars[j - 2] == '.');
                    dp[i][j] = flag1 || flag2;
                } else {
                    // 处理普通字符和 '.' 字符
                    dp[i][j] = i > 0 && dp[i - 1][j - 1] && (s_chars[i - 1] == p_chars[j - 1] || p_chars[j - 1] == '.');
                }
            }
        }

        dp[m][n] 
    }
}
