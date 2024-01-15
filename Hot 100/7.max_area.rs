/*
    题目：盛最多水的容器
    题解：https://leetcode.cn/problems/container-with-most-water/
 */
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res:i32 = 0;
        let mut left=0;
        let mut right = height.len()-1;
        while left<right{
            let h = std::cmp::min(height[left], height[right]);
            let wid:i32 = (right-left) as i32;
            res=std::cmp::max(res, h*wid);
            if(height[left]==h){
                left+=1;
            }else{
                right-=1;
            }
        }
        return res;
    }
}