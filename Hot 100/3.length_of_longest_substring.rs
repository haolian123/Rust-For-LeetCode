/*
    题目：3. 无重复字符的最长子串
    链接：https://leetcode.cn/problems/longest-substring-without-repeating-characters
 */
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 使用 HashMap 存储字符及其最新的索引
        let mut char_indices = HashMap::new();
        let mut max_length = 0;  // 存储最长子串的长度
        let mut start = -1i32;   // 子串的起始位置

        // 遍历字符串中的每个字符及其索引
        for (i, ch) in s.chars().enumerate() {
            // 如果字符已经存在于 HashMap 中，则更新起始位置
            if let Some(&index) = char_indices.get(&ch) {
                start = std::cmp::max(start, index);
            }
            // 更新最长子串的长度
            max_length = std::cmp::max(max_length, i as i32 - start);
            // 将当前字符及其索引添加到 HashMap 中
            char_indices.insert(ch, i as i32);
        }

        max_length
    }
}
