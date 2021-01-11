//1550. 存在连续三个奇数的数组
// 给你一个整数数组 arr，请你判断数组中是否存在连续三个元素都是奇数的情况：如果存在，请返回 true ；否则，返回 false 。
//
//  
//
// 示例 1：
//
// 输入：arr = [2,6,4,1]
// 输出：false
// 解释：不存在连续三个元素都是奇数的情况。
// 示例 2：
//
// 输入：arr = [1,2,34,3,4,5,7,23,12]
// 输出：true
// 解释：存在连续三个元素都是奇数的情况，即 [5,7,23] 。
//  
//
// 提示：
//
// 1 <= arr.length <= 1000
// 1 <= arr[i] <= 1000
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/three-consecutive-odds
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

//可以用滑动窗口算法，初始化第一个窗口，每次移动窗口更新统计数量
pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return false;
    }
    //初始化第一个窗口
    let mut count = 0;
    for i in 0..3 {
        if arr[i] % 2 == 1 {
            count += 1;
        }
    }
    if count == 3 { return true; }
    //滑动窗口更新
    for i in 1..arr.len() - 2 {
        if arr[i - 1] % 2 == 1 { count -= 1 }
        if arr[i + 2] % 2 == 1 { count += 1 }
        if count == 3 { return true; }
    }
    return false;
}

#[test]
fn test() {
    let res = three_consecutive_odds(vec![1, 1, 1]);
    assert!(res);
}
