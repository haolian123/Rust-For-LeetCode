/*
    题目：33. 搜索旋转排序数组
    链接：https://leetcode.cn/problems/search-in-rotated-sorted-array
 */


 impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // 初始化左右指针
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            // 检查中间位置的元素是否是目标元素
            if nums[mid] == target {
                return mid as i32;
            }

            // 判断中间元素与左侧元素的关系，以确定旋转的位置
            if nums[mid] >= nums[left] {
                // 如果中间元素大于等于左侧元素，则左半部分是有序的
                // 检查目标元素是否在左侧有序部分
                if target >= nums[left] && target <= nums[mid] {
                    // 如果是，调整右指针，缩小搜索范围到左半部分
                    right = mid - 1;
                } else {
                    // 否则调整左指针，搜索右半部分
                    left = mid + 1;
                }
            } else {
                // 否则，右半部分是有序的
                // 检查目标元素是否在右侧有序部分
                if target >= nums[mid] && target <= nums[right] {
                    // 如果是，调整左指针，缩小搜索范围到右半部分
                    left = mid + 1;
                } else {
                    // 否则调整右指针，搜索左半部分
                    right = mid - 1;
                }
            }
        }
        // 如果没有找到目标，返回-1
        -1
    }
}