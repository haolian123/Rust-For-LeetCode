/*
    题目：139. 单词拆分
    链接：https://leetcode.cn/problems/word-break/
 */
use std::collections::HashSet;
impl Solution {
  pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    let mut dp:Vec<bool> = vec![false;n+1];
    let word_set:HashSet<String> = word_dict.iter().cloned().collect();
    dp[0]=true;
    for i in 1..=n{
      for j in 1..=i{
        let tmp = &s[j-1..i];
        if word_set.contains(tmp){
          dp[i]=dp[j-1]|dp[i];
          break;
        }
      }
    }
    return dp[n];
  }
}