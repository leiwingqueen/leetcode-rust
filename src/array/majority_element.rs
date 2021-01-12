//数组中占比超过一半的元素称之为主要元素。给定一个整数数组，找到它的主要元素。若没有，返回-1。
//
// 示例 1：
//
// 输入：[1,2,5,9,5,9,5,5,5]
// 输出：5
//  
//
// 示例 2：
//
// 输入：[3,2]
// 输出：-1
//  
//
// 示例 3：
//
// 输入：[2,2,1,1,1,2,2]
// 输出：2
//  
//
// 说明：
// 你有办法在时间复杂度为 O(N)，空间复杂度为 O(1) 内完成吗？
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/find-majority-element-lcci
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


use std::collections::HashMap;

///其实就是一个计数器，使用map实现即可
/// 时间复杂度O(n)，空间复杂度O(n)
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let l = (nums.len() / 2) as i32;
    for x in nums {
        let option = map.get(&x);
        let mut count = 0;
        if option.is_some() {
            //这种强转有点奇怪
            count = *option.unwrap();
        }
        count += 1;
        if count > l { return x; }
        map.insert(x, count);
    }
    return -1;
}

//TODO:还有一个摩尔投票法，没看懂

#[test]
fn test() {
    let nums = vec![1, 2, 5, 9, 5, 9, 5, 5, 5];
    let res = majority_element(nums);
    println!("res:{}", res);
    assert_eq!(5, res);
}

