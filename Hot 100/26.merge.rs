/*
    题目：56. 合并区间
    链接：https://leetcode.cn/problems/merge-intervals/
 */
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res:Vec<Vec<i32>> = Vec::new();
        
        intervals.sort_by(|a,b|a[0].cmp(&b[0]));

        for it in intervals.into_iter(){
            if(res.is_empty()||res.last().unwrap()[1]<it[0]){
                res.push(it);
            }else{
                let last = res.last_mut().unwrap();
                last[1] = std::cmp::max(last[1], it[1]);
            }
        }
        res
    }
}