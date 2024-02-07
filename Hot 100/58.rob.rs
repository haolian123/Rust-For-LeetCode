/*
    题目：198. 打家劫舍
    链接：https://leetcode.cn/problems/house-robber/
 */
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            0 => return 0, // 处理空数组的情况
            1 => return nums[0], // 只有一个元素时直接返回这个元素
            _ => {} // 对于长度大于1的情况，不需要特殊处理
        }

        let mut dp = vec![0; len];
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]); 

        for i in 2..len {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
        }

        return dp[len - 1]; 
    }
}
