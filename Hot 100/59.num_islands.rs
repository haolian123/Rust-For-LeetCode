/*
    题目：200. 岛屿数量
    链接：https://leetcode.cn/problems/number-of-islands/
 */

 impl Solution {
  fn dfs(grid:&mut Vec<Vec<char>>,i:usize,j:usize){
    let m = grid.len();
    let n = grid[0].len();
    grid[i][j]='0';
    if i+1<m&&grid[i+1][j]=='1'{
      Self::dfs(grid,i+1,j);
    }
    if i>0&&grid[i-1][j]=='1'{
      Self::dfs(grid,i-1,j);
    }
    if j+1<n&&grid[i][j+1]=='1'{
      Self::dfs(grid,i,j+1);
    }
    if j>0&&grid[i][j-1]=='1'{
      Self::dfs(grid,i,j-1);
    }
  }
  pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
      let mut res = 0;
      let m = grid.len();
      let n = grid[0].len();

      let mut visited = vec![vec![false;n];m];

      for i in 0..m{
        for j in 0..n{
          if grid[i][j]=='1'&&!visited[i][j]{
            res+=1;
            Self::dfs(&mut grid,i,j);
          }
        }
      }
      res
  }
}
