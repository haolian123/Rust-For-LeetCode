/*
    题目：85. 最大矩形
    链接：https://leetcode.cn/problems/maximal-rectangle/
 */
impl Solution {
    fn solve(heights:&Vec<i32>)->i32{
        let mut res = 0;
        let mut stack =Vec::new();

        for i in 0..heights.len(){
            while let Some(&top) = stack.last(){
                if heights[top as usize]<=heights[i]{break;}
                stack.pop();
                let height = heights[top];
                let width = if stack.is_empty(){i} else{i-stack.last().unwrap()-1};
                res = std::cmp::max(res,height as usize*width);
                
            }
            stack.push(i);
        }
        return res as i32;
    }
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m =matrix.len();
        let n = matrix[0].len()+2;
        let mut res = 0;

        let mut heights = vec![0;n];
        for i in 0..m{
            for j in 1..n-1{
                if matrix[i][j-1]=='1'{
                    heights[j]+=1;
                }else{
                    heights[j]=0;
                }
            }
            res = std::cmp::max(res,Self::solve(&heights));
        }

        res



    }
}
