/*
    题目：31. 下一个排列
    链接：https://leetcode.cn/problems/next-permutation
 */
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32-2;
        while i>=0 && nums[i as usize]>=nums[(i+1) as usize]{
            i-=1;
        }
        if i>=0{
            let mut j = nums.len() as i32-1;
            while j>i && nums[j as usize]<=nums[i as usize]{
                j-=1;
            }
            nums.swap(i as usize, j as usize);
        }
        nums[(i+1) as usize..].reverse();
    }
}