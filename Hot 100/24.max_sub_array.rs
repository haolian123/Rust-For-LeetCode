/*
    题目：53. 最大子数组和
    链接：https://leetcode.cn/problems/maximum-subarray/
 */
impl Solution {
    fn max_sub(nums:&mut Vec<i32>,left:usize,right:usize)->i32{
        if left==right{
            return nums[left];
        }

        let mid = left+(right-left)/2 ;
        let leftMax = Self::max_sub(nums, left, mid);
        let rightMax = Self::max_sub(nums, mid+1,right);
        let mut leftMaxCross = nums[mid];
        let mut rightMaxCross = nums[mid+1];
        let mut temp = 0;
        for i in (left..=mid ).rev(){
            temp+=nums[i ];
            leftMaxCross=std::cmp::max(leftMaxCross,temp);
        }
        temp = 0;
        for i in mid+1..=right{
            temp+=nums[i ];
            rightMaxCross=std::cmp::max(rightMaxCross,temp);
        }
        return std::cmp::max(std::cmp::max(leftMax, rightMax),leftMaxCross+rightMaxCross);
        
    }
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut nums =  nums;
        let len = nums.len();
        Self::max_sub(&mut nums,0,len-1)
    }
}