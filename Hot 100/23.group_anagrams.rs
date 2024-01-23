/*
    题目：49. 字母异位词分组
    链接：https://leetcode.cn/problems/group-anagrams/
 */

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 创建一个 HashMap 来存储排序后的字符串和原始字符串列表的映射
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        // 遍历字符串列表
        for str in strs {
            // 将字符串转换为字符数组
            let mut chars: Vec<char> = str.chars().collect();
            // 对字符数组进行排序
            chars.sort_unstable();
            // 将排序后的字符数组转换回字符串
            let sorted_str = chars.into_iter().collect::<String>();

            // 将原始字符串插入到 HashMap 中
            // 如果排序后的字符串作为键不存在，将创建新的字符串列表
            map.entry(sorted_str).or_insert_with(Vec::new).push(str);
        }

        // 收集 HashMap 中所有值（字符串列表）并返回
        map.into_iter().map(|(_, v)| v).collect()
    }
}
