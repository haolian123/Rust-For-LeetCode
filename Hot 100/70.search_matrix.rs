impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row = m-1;
        let mut col = 0;
        while row>=0&&col<n {
          if matrix[row][col]==target{
            return true;
          }else if matrix[row][col]<target{
            col+=1;
          }else{
              if row==0{
                  break;
              }
              row-=1;
          }
          
        }
        return false;
    }
  }