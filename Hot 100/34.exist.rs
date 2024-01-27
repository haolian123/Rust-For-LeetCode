/*
    题目：79. 单词搜索
    链接：https://leetcode.cn/problems/word-search/
 */
impl Solution {
    fn back_trace(board: &Vec<Vec<char>>,word:&str,visited:&mut Vec<Vec<bool>>,cur_i:isize, cur_j:isize,index:usize)-> bool{
        if(board[cur_i as usize][cur_j as usize]!=word.chars().nth(index).unwrap()){
            return false;
        }
        if(index==word.len()-1){
            return true;
        }

        let arr = [[1,0],[0,1],[-1,0],[0,-1]];
        let m = board.len() as isize;
        let n = board[0].len() as isize;
       
        for i in 0..4{
            let new_i = cur_i+arr[i][0];
            let new_j = cur_j+arr[i][1];
           
            if new_i>=0&&new_i<m&&new_j>=0&&new_j<n&&!visited[new_i as usize][new_j as usize]{
                visited[new_i as usize][new_j as usize] = true;
                if Self::back_trace(board, word, visited, new_i, new_j,  index+1){
                    return true;
                }
                visited[new_i as usize][new_j as usize] = false;
            }
        }
        return false;
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();

        let mut visited:Vec<Vec<bool>> = vec![vec![false;n];m];
        for i in 0..m{
            for j in 0..n{
                visited[i][j]=true;
                if Self::back_trace(&board, word.as_str(), &mut visited, i as isize, j as isize, 0){
                    return true;
                }
                visited[i][j]=false;
                    
            }
        }
        return false;


    }
}