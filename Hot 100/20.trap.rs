/*
    题目：42. 接雨水
    链接：https://leetcode.cn/problems/trapping-rain-water/
*/

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut leftMax:i32 = 0;
        let mut rightMax:i32 = 0;
        let mut left = 0 as usize;
        let mut right = height.len()-1;
        let mut res = 0 as i32;
        while left < right{
            leftMax = std::cmp::max(leftMax, height[left]);
            rightMax = std::cmp::max(rightMax, height[right]);

            if(height[left]<=height[right]){
                res+= leftMax-height[left];
                left+=1;
            }else{
                res+= rightMax-height[right];
                right-=1;
            }
        }
        res 
    }
}