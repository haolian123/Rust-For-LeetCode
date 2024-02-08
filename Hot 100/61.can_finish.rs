/*
    题目：207. 课程表
    链接：https://leetcode.cn/problems/course-schedule/
 */
use std::collections::VecDeque;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph = vec![vec![false;n];n];
        let mut degree = vec![0;n];

        for prerequisites in prerequisites{
          let a = prerequisites[1] as usize;
          let b = prerequisites[0] as usize;
          if !graph[a][b]{
            graph[a][b] = true;
            degree[b]+=1;
          }
        }
        
        let mut queue = VecDeque::new();
        for i in 0..n{
          if degree[i]==0{
            queue.push_back(i);
          }
        }

        let mut count = 0;
        
        while let Some(front) = queue.pop_front(){
          count+=1;
          for i in 0..n{
            if graph[front][i]{
              degree[i]-=1;
              if degree[i] == 0{
                queue.push_back(i);
              }
            }
          }
        }

        return count == n;
    }
}