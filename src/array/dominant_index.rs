//在一个给定的数组nums中，总是存在一个最大元素 。
//
// 查找数组中的最大元素是否至少是数组中每个其他数字的两倍。
//
// 如果是，则返回最大元素的索引，否则返回-1。
//
// 示例 1:
//
// 输入: nums = [3, 6, 1, 0]
// 输出: 1
// 解释: 6是最大的整数, 对于数组中的其他整数,
// 6大于数组中其他元素的两倍。6的索引是1, 所以我们返回1.
//  
//
// 示例 2:
//
// 输入: nums = [1, 2, 3, 4]
// 输出: -1
// 解释: 4没有超过3的两倍大, 所以我们返回 -1.
//  
//
// 提示:
//
// nums 的长度范围在[1, 50].
// 每个 nums[i] 的整数范围在 [0, 100].
//
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/largest-number-at-least-twice-of-others
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

///扫描两遍
pub fn dominant_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return -1; }
    //找到最大数所在的索引
    let mut idx = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[idx] {
            idx = i;
        }
    }
    //第二遍扫描比较
    for i in 0..nums.len() {
        if i != idx {
            if nums[idx] < nums[i] * 2 {
                return -1;
            }
        }
    }
    return idx as i32;
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let res = dominant_index(nums);
    print!("{}", res);
    assert_eq!(-1, res);
}