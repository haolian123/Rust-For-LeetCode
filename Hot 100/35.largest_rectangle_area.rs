/*
    题目：84. 柱状图中最大的矩形
    链接：https://leetcode.cn/problems/largest-rectangle-in-histogram/
 */
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let mut tmp = vec![0];

        tmp.extend_from_slice(&heights);

        tmp.push(0);

        let mut stack = Vec::new();
        let mut res = 0;
        for i in 0..tmp.len(){
            while let Some(&top) = stack.last(){
                if tmp[top]<tmp[i]{break;}
                stack.pop();
                let height = tmp[top];
                let width = if stack.is_empty(){i} else{i-stack.last().unwrap()-1};
                res = std::cmp::max(res,height as usize*width);
            }
            stack.push(i);
        }
        res as i32
    }
}
