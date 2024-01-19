/*
    题目：34. 在排序数组中查找元素的第一个和最后一个位置
    链接：https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/
*/

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let index1 = Self::find(&nums,target);
        let index2 = Self::find(&nums,target+1)-1;
        if (index1 as usize) <nums.len() && nums[index1 as usize]== target{
            return vec![index1,index2];
        }
        return vec![-1,-1];
    }
    pub fn find(nums:&[i32],target: i32) ->i32{
        let mut left =0;
        let mut right =(nums.len() as i32)-1;
        while(left<=right){
            let mid = left+(right-left)/2;
            if(nums[mid as usize]>=target){
                right = mid-1;
            }else{
                left = mid + 1;
            }
        }

        left as i32 
    }
}