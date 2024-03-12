impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut index = 0;
        for i in 0..n{
            if nums[i]!=0{
                nums[index]=nums[i];
                index+=1;
            }
        }
        for i in index..n{
            nums[i] = 0 as i32;
        }
    }
}