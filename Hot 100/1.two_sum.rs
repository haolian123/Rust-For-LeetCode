/*
    题目：1.两数之和
    题号：1
    链接：https://leetcode.cn/problems/two-sum
 */

// 方法：哈希表
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new(); 

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&index) = map.get(&(target - num)) {
                // 检查当前数字的差（即 target - num）是否已经在 map 中
                // 如果是，返回这两个数字的索引
                return vec![index as i32, i as i32]; 
            }
            // 将当前数字及其索引存入 map
            map.insert(num, i); 
        }
        // 如果找不到符合条件的数字对，返回空
        vec![] 
    }
}

