use std::ops::Add;

//867. 转置矩阵
//给定一个矩阵 A， 返回 A 的转置矩阵。
//
// 矩阵的转置是指将矩阵的主对角线翻转，交换矩阵的行索引与列索引。
//
//  
//
// 示例 1：
//
// 输入：[[1,2,3],[4,5,6],[7,8,9]]
// 输出：[[1,4,7],[2,5,8],[3,6,9]]
// 示例 2：
//
// 输入：[[1,2,3],[4,5,6]]
// 输出：[[1,4],[2,5],[3,6]]
//  
//
// 提示：
//
// 1 <= A.length <= 1000
// 1 <= A[0].length <= 1000
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/transpose-matrix
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if a.len() == 0 {
        return vec![];
    }
    let row = a.len();
    let col = a[0].len();
    let mut res = vec![];
    for i in 0..col {
        //每一行的数据
        let mut x: Vec<i32> = vec![];
        for j in 0..row {
            x.push(a[j][i]);
        }
        res.push(x);
    }
    return res;
}

///[[1,2,3],[4,5,6],[7,8,9]]
#[test]
fn test() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let res = transpose(a);
    for row in res {
        let mut s = String::from("");
        for x in row {
            s.push_str(x.to_string().add(",").as_str());
        }
        println!("[{}]", s);
    }
}