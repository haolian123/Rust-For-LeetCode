/*
    题目：15.三数之和
    链接：https://leetcode.cn/problems/3sum/description/
 */
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![]; 
        let n = nums.len(); 
        // 排序
        nums.sort(); 
        for i in 0..n { 
            // 跳过重复的元素，以避免重复的三元组
            if i > 0 && nums[i] == nums[i - 1] {
                continue; 
            }
            let mut left = i + 1;
            let mut right = n - 1;

            while left < right { 
                let sum = nums[i] + nums[left] + nums[right]; 

                if sum > 0 { 
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1; //跳过重复的元素
                    }
                    right -= 1; 
                } else if sum < 0 { 
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1; //跳过重复的元素
                    }
                    left += 1; 
                } else { 
                    res.push(vec![nums[i], nums[left], nums[right]]); 
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1; 
                    }
                    right -= 1; 
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1; 
                    }
                    left += 1; 
                }
            }
        }
        res 
    }
}
