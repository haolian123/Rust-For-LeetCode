/*
    题目：4. 寻找两个正序数组的中位数
    链接：https://leetcode.cn/problems/median-of-two-sorted-arrays
*/

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let left = (m + n + 1) / 2;
        let right = (m + n + 2) / 2;

        // 计算两个数组的中位数
        let result1 = Self::find_kth(&nums1, 0, &nums2, 0, left);
        let result2 = Self::find_kth(&nums1, 0, &nums2, 0, right);
        (result1 + result2) as f64 / 2.0
    }

    fn find_kth(nums1: &[i32], i: usize, nums2: &[i32], j: usize, k: usize) -> i32 {
        // 如果nums1为空，则直接从nums2中取值
        if i >= nums1.len() {
            return nums2[j + k - 1];
        }
        // 如果nums2为空，则直接从nums1中取值
        if j >= nums2.len() {
            return nums1[i + k - 1];
        }
        // 如果k为1，取两个数组当前元素的最小值
        if k == 1 {
            return std::cmp::min(nums1[i], nums2[j]);
        }

        // 在两个数组中分别查找第k/2小的数
        let mid_val1 = if i + k / 2 - 1 < nums1.len() {
            nums1[i + k / 2 - 1]
        } else {
            i32::MAX
        };
        let mid_val2 = if j + k / 2 - 1 < nums2.len() {
            nums2[j + k / 2 - 1]
        } else {
            i32::MAX
        };

        if mid_val1 < mid_val2 {
            Self::find_kth(nums1, i + k / 2, nums2, j, k - k / 2)
        } else {
            Self::find_kth(nums1, i, nums2, j + k / 2, k - k / 2)
        }
    }
}
