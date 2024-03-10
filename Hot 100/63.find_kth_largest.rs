impl Solution {
    fn find_kth(nums: &Vec<i32>, k: usize) -> i32 {
        let mid = (nums.len() - 1) / 2;
        let pivot = nums[mid];
        let mut small: Vec<i32> = Vec::new();
        let mut equal: Vec<i32> = Vec::new();
        let mut great: Vec<i32> = Vec::new();
        
        for num in nums {
            if *num < pivot {
                small.push(*num);
            } else if *num == pivot {
                equal.push(*num);
            } else {
                great.push(*num);
            }
        }
        
        if small.len() >= k {
            return Self::find_kth(&small, k);
        } else if small.len() + equal.len() >= k {
            return equal[0];
        } else {
            return Self::find_kth(&great, k - (small.len() + equal.len()));
        }
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        return Self::find_kth(&nums, nums.len() - (k as usize) + 1);
    }
}
