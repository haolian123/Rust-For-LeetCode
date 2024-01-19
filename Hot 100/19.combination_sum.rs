/*
    题目：39. 组合总和
    链接：https://leetcode.cn/problems/combination-sum/
*/

impl Solution {
    pub fn solve(nums: &[i32], target: i32, index: usize, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
        if target == 0 {
            res.push(path.clone());
            return;
        }
        if target < 0 {
            return;
        }
        for i in index..nums.len() {
            path.push(nums[i]);
            // 递归调用，注意这里传递 `nums` 的引用
            Self::solve(nums, target - nums[i], i, res, path);
            path.pop();
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        // 调用 `solve` 时传递 `candidates` 的引用
        Self::solve(&candidates, target, 0, &mut res, &mut path);
        res
    }
}
