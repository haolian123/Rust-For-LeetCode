/*
    题目：48. 旋转图像
    链接：https://leetcode.cn/problems/rotate-image/
 */
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // 翻转矩阵的行
        let m = matrix.len();
        for i in 0..(m / 2) {
            for j in 0..m {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[m - i - 1][j];
                matrix[m - i - 1][j] = temp;
            }
        }

        // 交换对角线元素
        for i in 0..m {
            for j in i..m {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
}
