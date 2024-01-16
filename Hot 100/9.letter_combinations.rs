/*
    题目：17. 电话号码的字母组合
    链接：https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
*/

impl Solution{
    fn generate_combinations(words:&Vec<&str>, res:&mut Vec<String>, path: &mut String, digits:&str,index:usize){
        if index>= digits.len(){
            res.push(path.clone());
            return;
        }
        let number = digits.as_bytes()[index] - b'0';
        let word = words[number as usize];

        for ch in word.chars(){
            path.push(ch);
            Self::generate_combinations(words,res,path,digits,index+1);
            path.pop();
        }
    }
    pub fn letter_combinations(digits:String) -> Vec<String>{
        let words = vec![" ", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut res = Vec::new();
        let mut path = String::new();
        if !digits.is_empty(){
            Self::generate_combinations(&words,&mut res,&mut path,&digits,0)
        }
        res
    }
} 