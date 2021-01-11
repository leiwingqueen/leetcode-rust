//给定一个整数数组和一个整数 k，判断数组中是否存在两个不同的索引 i 和 j，
// 使得 nums [i] = nums [j]，并且 i 和 j 的差的 绝对值 至多为 k。
//
//  
//
// 示例 1:
//
// 输入: nums = [1,2,3,1], k = 3
// 输出: true
// 示例 2:
//
// 输入: nums = [1,0,1,1], k = 1
// 输出: true
// 示例 3:
//
// 输入: nums = [1,2,3,1,2,3], k = 2
// 输出: false
// 通过次数74,746提交次数182,464
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/contains-duplicate-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


use std::collections::HashMap;

//使用一个hashmap保存最后的index，如果发现已经存在的key，则比较距离是否满足
//时间复杂度O(n),空间复杂度O(n)
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::new();
    //迭代器进行遍历
    for (i, num) in nums.iter().enumerate() {
        if map.contains_key(num) {
            let len = (i - map.get(num).unwrap() + 1) as i32;
            if len <= k {
                return true;
            }
        }
        //更新最后的index
        map.insert(num, i);
    }
    return true;
}

#[test]
fn test() {
    let res = contains_nearby_duplicate(vec![1, 2, 3, 1], 3);
    assert!(res);
}