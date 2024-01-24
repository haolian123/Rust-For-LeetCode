/*
    题目：55. 跳跃游戏
    链接：https://leetcode.cn/problems/jump-game
 */
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut index = 0;
        let mut max_len = nums[0];
        while index<nums.len()-1{
            max_len=std::cmp::max(max_len,nums[index]+index as i32);
            if(index>=max_len as usize){
                return false;
            }
                
            index+=1;
        }
        return true;
    }
}