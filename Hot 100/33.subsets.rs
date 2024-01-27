/*
    题目：78. 子集
    链接：https://leetcode.cn/problems/subsets
 */

 impl Solution {
    fn solve(res:&mut Vec<Vec<i32>>,path:&mut Vec<i32>,nums:&Vec<i32>,index:usize){
        res.push(path.clone());
        for i in index..nums.len(){
            path.push(nums[i]);
            Self::solve(res,path,nums,i+1);
            path.pop();
        }
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res:Vec<Vec<i32>> = Vec::new();
        let mut path:Vec<i32> = Vec::new();

        Self::solve(&mut res, &mut path, &nums, 0);

        return res;
    }
}