/*
304. 二维区域和检索 - 矩阵不可变

给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (row1, col1) ，右下角为 (row2, col2) 。


上图子矩阵左上角 (row1, col1) = (2, 1) ，右下角(row2, col2) = (4, 3)，该子矩形内元素的总和为 8。

 

示例：

给定 matrix = [
  [3, 0, 1, 4, 2],
  [5, 6, 3, 2, 1],
  [1, 2, 0, 1, 5],
  [4, 1, 0, 1, 7],
  [1, 0, 3, 0, 5]
]

sumRegion(2, 1, 4, 3) -> 8
sumRegion(1, 1, 2, 2) -> 11
sumRegion(1, 2, 2, 4) -> 12
 

提示：

你可以假设矩阵不可变。
会多次调用 sumRegion 方法。
你可以假设 row1 ≤ row2 且 col1 ≤ col2 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/range-sum-query-2d-immutable
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
struct NumMatrix {
    prefixSum: Vec<Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    /*
    前缀和，空间换时间
     */
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (row, col) = (matrix.len(), if matrix.len() == 0 { 0 } else { matrix[0].len() });
        let mut prefixSum = vec![vec![0; col + 1]; row + 1];
        //初始化前缀和
        for i in 1..row + 1 {
            for j in 1..col + 1 {
                prefixSum[i][j] = prefixSum[i][j - 1] + prefixSum[i - 1][j] - prefixSum[i - 1][j - 1] + matrix[i - 1][j - 1];
            }
        }
        let nm = NumMatrix { prefixSum };
        return nm;
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        return self.prefixSum[row2 + 1][col2 + 1] - self.prefixSum[row1][col2 + 1]
            - self.prefixSum[row2 + 1][col1] + self.prefixSum[row1][col1];
    }
}

#[test]
fn test() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5]
    ];
    let num_matrix = NumMatrix::new(matrix);
    let res = num_matrix.sum_region(2, 1, 4, 3);
    println!("{}", res);
    assert_eq!(8, res);
}