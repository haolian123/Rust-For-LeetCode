/*
    题目：最长回文子串
    链接：https://leetcode.cn/problems/longest-palindromic-substring/
 */
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut max_len = 1;
        let mut start = 0;
        let mut dp = vec![vec![false; n]; n];

        // 从字符串末尾开始，向前遍历
        for i in (0..n).rev() {
            for j in i..n {
                // 检查子串是否为回文
                dp[i][j] = bytes[i] == bytes[j] && (j - i <= 1 || dp[i + 1][j - 1]);

                // 如果是回文，并且长度大于当前最大长度，则更新最大长度和起始位置
                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    start = i;
                }
            }
        }

        s[start..start + max_len].to_string()
    }
}