/*
给定一个整型数组，在数组中找出由三个数组成的最大乘积，并输出这个乘积。

示例 1:

输入: [1,2,3]
输出: 6
示例 2:

输入: [1,2,3,4]
输出: 24
注意:

给定的整型数组长度范围是[3,104]，数组中所有的元素范围是[-1000, 1000]。
输入的数组中任意三个数的乘积不会超出32位有符号整数的范围。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-product-of-three-numbers
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
结果有可能是：
1.两个负数*一个正数
2.3个正数相乘
两者取最大值即可

时间复杂度O(nlogn)
 */
pub fn maximum_product(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let len = nums.len();
    let mut nums = nums;
    nums.sort();
    let n1 = nums[0] * nums[1] * nums[len - 1];
    let n2 = nums[len - 3] * nums[len - 2] * nums[len - 1];
    return if n1 > n2 { n1 } else { n2 };
}

/*
时间复杂度O(n)，空间复杂度O(1)
 */
pub fn maximum_product_2(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    //保存最小的2两个数和最大3个的数
    let mut min1 = 1001;
    let mut min2 = 1001;
    let mut max1 = -1001;
    let mut max2 = -1001;
    let mut max3 = -1001;
    for num in nums.iter() {
        let num = *num;
        if num > max1 {
            max3 = max2;
            max2 = max1;
            max1 = num;
        } else if num > max2 {
            max3 = max2;
            max2 = num;
        } else if num > max3 {
            max3 = num;
        }
        if num < min1 {
            min2 = min1;
            min1 = num;
        } else if num < min2 {
            min2 = num;
        }
    }
    let n1 = min1 * min2 * max1;
    let n2 = max1 * max2 * max3;
    return if n1 > n2 { n1 } else { n2 };
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let res = maximum_product_2(nums);
    println!("res:{}", res);
    assert_eq!(6, res);
}

