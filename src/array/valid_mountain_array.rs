//给定一个整数数组 A，如果它是有效的山脉数组就返回 true，否则返回 false。
//
// 让我们回顾一下，如果 A 满足下述条件，那么它是一个山脉数组：
//
// A.length >= 3
// 在 0 < i < A.length - 1 条件下，存在 i 使得：
// A[0] < A[1] < ... A[i-1] < A[i]
// A[i] > A[i+1] > ... > A[A.length - 1]
//  
//
//
//
//  
//
// 示例 1：
//
// 输入：[2,1]
// 输出：false
// 示例 2：
//
// 输入：[3,5,5]
// 输出：false
// 示例 3：
//
// 输入：[0,3,2,1]
// 输出：true
//  
//
// 提示：
//
// 0 <= A.length <= 10000
// 0 <= A[i] <= 10000 
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/valid-mountain-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

///双指针解法，左右指针分别找到山峰的位置，检查山峰的位置是否在同一个点即可
pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let len = arr.len();
    if arr.len() < 3 { return false; }
    let mut l = 1;
    let mut r = len - 2;
    //找到山峰的左侧
    while l < len && arr[l] > arr[l - 1] {
        l += 1;
    }
    if l == 1 || l == len { return false; }
    l -= 1;
    //println!("l:{}", l);
    //找到山峰的右侧
    while arr[r] > arr[r + 1] && r >= l {
        r -= 1;
    }
    r += 1;
    //println!("r:{}", r);
    return r == l && r < len - 1;
}

#[test]
fn test() {
    let res = valid_mountain_array(vec![0, 3, 2, 1]);
    println!("{}", res);
    assert!(res);
}

#[test]
fn test2() {
    let array = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res = valid_mountain_array(array);
    println!("{}", res);
    assert!(!res);
}
