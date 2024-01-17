/*
    题目：22. 括号生成
    链接：https://leetcode.cn/problems/generate-parentheses
 */

impl Solution{
    fn generate(left:i32,right:i32,n:i32,res:&mut Vec<String>,path:String){
        if left<right||left>n||right>n{
            return;
        }
        if right == n && left == n{
            res.push(path.clone());
            return;
        }
        Self::generate(left+1,right,n,res,path.clone()+"(");
        Self::generate(left,right+1,n,res,path.clone()+")");
    }
    fn generate_parenthesis(n:i32) -> Vec<String>{
        let mut res:Vec<String> = vec![];
        let path:String = String::new();
        Self::generate(0, 0, n, &mut res, path);
        res
    }
}